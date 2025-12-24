# Data Model: SCCP Constant Folding Optimizer

**Feature**: Constant Folding Optimizer with SCCP  
**Branch**: `020-sccp-constant-folding`  
**Date**: 2025-12-05  
**Status**: Phase 1 Complete

## Overview

This document defines the complete data model for the Sparse Conditional Constant Propagation optimizer, including all entity structures, their fields, relationships, validation rules, and state transitions. The model follows the modular architecture defined in research.md with clear separation between lattice representation, constant evaluation, propagation algorithm, and IR rewriting.

## Core Entities

### 1. LatticeValue

**Purpose**: Represents the abstract interpretation state of an SSA value during SCCP analysis.

**Location**: `src/ir/optimizer/constant_folding/lattice.rs`

**Definition**:

```rust
/// Lattice value for abstract interpretation in SCCP.
/// 
/// Lattice ordering: Bottom ≤ Constant ≤ Top
/// - Bottom (⊥): Unreachable or uninitialized value
/// - Constant(v): Proven compile-time constant with value v
/// - Top (⊤): Overdefined value that may vary at runtime
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    /// Bottom of lattice: unreachable or uninitialized
    Bottom,
    
    /// Middle of lattice: proven constant value
    Constant(ConstantValue),
    
    /// Top of lattice: overdefined (runtime-varying)
    Top,
}
```

**Fields**:

- `Bottom`: No associated data (unit variant)
- `Constant(ConstantValue)`: Embedded constant value (see ConstantValue entity)
- `Top`: No associated data (unit variant)

**Validation Rules**:

- Lattice ordering must be preserved: ⊥ ≤ Constant ≤ ⊤
- Meet operation must be monotonic (never decrease in lattice)
- Constant variants must contain valid ConstantValue instances

**State Transitions**:

```text
Initial State: Bottom (for locals) or Top (for parameters/globals)
    ↓
Transition: Value proven constant through evaluation
    ↓
Constant(v)
    ↓
Transition: Conflicting constants meet (phi node with different constants)
    ↓
Top (terminal state - never transitions away)
```

**Operations**:

```rust
impl LatticeValue {
    /// Compute lattice meet (greatest lower bound) of two values.
    /// 
    /// Meet operation semantics:
    /// - Bottom ⊓ x = x (Bottom is identity)
    /// - Top ⊓ x = Top (Top absorbs)
    /// - Constant(v1) ⊓ Constant(v2) = Constant(v1) if v1 == v2, else Top
    /// 
    /// # Examples
    /// ```
    /// use jsavrs::ir::optimizer::constant_folding::*;
    /// 
    /// let bottom = LatticeValue::Bottom;
    /// let const_42 = LatticeValue::Constant(ConstantValue::I32(42));
    /// assert_eq!(bottom.meet(&const_42), const_42);
    /// ```
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // Bottom is identity
            (LatticeValue::Bottom, x) | (x, LatticeValue::Bottom) => x.clone(),
            
            // Top absorbs everything
            (LatticeValue::Top, _) | (_, LatticeValue::Top) => LatticeValue::Top,
            
            // Constants meet to same constant or Top
            (LatticeValue::Constant(v1), LatticeValue::Constant(v2)) => {
                if v1 == v2 {
                    LatticeValue::Constant(v1.clone())
                } else {
                    LatticeValue::Top
                }
            }
        }
    }
    
    /// Check if value is a constant.
    pub fn is_constant(&self) -> bool {
        matches!(self, LatticeValue::Constant(_))
    }
    
    /// Extract constant value if present.
    pub fn as_constant(&self) -> Option<&ConstantValue> {
        if let LatticeValue::Constant(v) = self {
            Some(v)
        } else {
            None
        }
    }
    
    /// Check if value is bottom (unreachable).
    pub fn is_bottom(&self) -> bool {
        matches!(self, LatticeValue::Bottom)
    }
    
    /// Check if value is top (overdefined).
    pub fn is_top(&self) -> bool {
        matches!(self, LatticeValue::Top)
    }
}
```

**Relationships**:

- Contains one `ConstantValue` when in Constant state
- Stored in `LatticeState` HashMap keyed by `ValueId`
- Updated by `SCCPropagator` during worklist processing

---

### 2. ConstantValue

**Purpose**: Type-safe representation of compile-time constant values for all supported IR types.

**Location**: `src/ir/optimizer/constant_folding/lattice.rs`

**Definition**:

```rust
/// Type-safe constant value representation.
/// 
/// Supports all IR primitive types with native Rust representations
/// maintaining correct semantics for arithmetic and comparisons.
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    // Signed integers
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    
    // Unsigned integers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    
    // Floating-point (IEEE 754)
    F32(f32),
    F64(f64),
    
    // Boolean
    Bool(bool),
    
    // Unicode character
    Char(char),
}
```

**Fields**: Each variant contains the native Rust type for that constant kind.

**Validation Rules**:

- Integer values must be within type range (enforced by Rust type system)
- Floating-point values may be NaN, Infinity, or -Infinity (all valid IEEE 754)
- Char values must be valid Unicode scalar values (enforced by Rust `char` type)
- No invalid bit patterns allowed (Rust type safety)

**Operations**:

```rust
impl ConstantValue {
    /// Get the IR type of this constant value.
    pub fn get_type(&self) -> IRType {
        match self {
            ConstantValue::I8(_) => IRType::I8,
            ConstantValue::I16(_) => IRType::I16,
            ConstantValue::I32(_) => IRType::I32,
            ConstantValue::I64(_) => IRType::I64,
            ConstantValue::U8(_) => IRType::U8,
            ConstantValue::U16(_) => IRType::U16,
            ConstantValue::U32(_) => IRType::U32,
            ConstantValue::U64(_) => IRType::U64,
            ConstantValue::F32(_) => IRType::F32,
            ConstantValue::F64(_) => IRType::F64,
            ConstantValue::Bool(_) => IRType::Bool,
            ConstantValue::Char(_) => IRType::Char,
        }
    }
    
    /// Check if two constant values have compatible types for operations.
    pub fn types_match(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
    
    /// Convert to boolean if possible (for branch conditions).
    pub fn as_bool(&self) -> Option<bool> {
        if let ConstantValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
}
```

**Special Cases**:

- **Floating-Point Equality**: Uses `f32::to_bits()` / `f64::to_bits()` for exact bit-level comparison (distinguishes NaN, -0.0 vs +0.0)
- **NaN Handling**: NaN values are valid constants but never equal to any value (including themselves per IEEE 754)
- **Infinity**: Positive and negative infinity are valid constants

**Relationships**:

- Embedded in `LatticeValue::Constant` variant
- Produced by `ConstantEvaluator`
- Converted to IR constants by `IRRewriter`

---

### 3. LatticeState

**Purpose**: Global mapping from SSA values to their lattice values during SCCP analysis.

**Location**: `src/ir/optimizer/constant_folding/propagator.rs`

**Definition**:

```rust
/// Global lattice state for all SSA values in a function.
/// 
/// Maintains the current abstract interpretation state for each value
/// during SCCP propagation.
#[derive(Debug)]
pub struct LatticeState {
    /// Mapping from SSA value IDs to their lattice values
    values: HashMap<ValueId, LatticeValue>,
}
```

**Fields**:

- `values: HashMap<ValueId, LatticeValue>`: Main state storage

**Validation Rules**:

- All SSA values in scope must have entries (or default to Bottom)
- Lattice values must never decrease in ordering (monotonicity)
- Updates must preserve SSA def-use relationships

**Operations**:

```rust
impl LatticeState {
    /// Create new empty lattice state with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: HashMap::with_capacity(capacity),
        }
    }
    
    /// Get lattice value for an SSA value (defaults to Bottom if not present).
    pub fn get(&self, value_id: ValueId) -> LatticeValue {
        self.values.get(&value_id).cloned().unwrap_or(LatticeValue::Bottom)
    }
    
    /// Update lattice value if it changes (returns true if updated).
    /// 
    /// Enforces monotonicity: new value must be ≥ old value in lattice ordering.
    pub fn update(&mut self, value_id: ValueId, new_value: LatticeValue) -> bool {
        let old_value = self.get(value_id);
        let merged = old_value.meet(&new_value);
        
        if merged != old_value {
            self.values.insert(value_id, merged);
            true
        } else {
            false
        }
    }
    
    /// Initialize value to specific lattice value (for function parameters).
    pub fn initialize(&mut self, value_id: ValueId, value: LatticeValue) {
        self.values.insert(value_id, value);
    }
}
```

**Relationships**:

- Owned by `SCCPropagator`
- Queried by `ConstantEvaluator` for operand values
- Updated during worklist processing

---

### 4. ConstantEvaluator

**Purpose**: Evaluates constant expressions for all IR types with correct overflow and edge case handling.

**Location**: `src/ir/optimizer/constant_folding/evaluator.rs`

**Definition**:

```rust
/// Constant expression evaluator for SCCP.
/// 
/// Evaluates binary and unary operations on constant operands,
/// producing constant results or Top (overdefined) on edge cases.
#[derive(Debug)]
pub struct ConstantEvaluator {
    /// Diagnostic emitter for warnings (division by zero, etc.)
    diagnostics: DiagnosticEmitter,
}
```

**Fields**:

- `diagnostics: DiagnosticEmitter`: For emitting warnings on edge cases

**Operations**:

```rust
impl ConstantEvaluator {
    /// Evaluate binary operation on constant operands.
    /// 
    /// Returns:
    /// - LatticeValue::Constant(result) if evaluation succeeds
    /// - LatticeValue::Top if overflow, type mismatch, or edge case
    /// 
    /// Emits warnings for division by zero (but not overflow per spec).
    pub fn evaluate_binary_op(
        &mut self,
        op: BinaryOp,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> LatticeValue {
        // Type checking
        if !left.types_match(right) {
            return LatticeValue::Top;
        }
        
        match (op, left, right) {
            // I32 arithmetic
            (BinaryOp::Add, ConstantValue::I32(l), ConstantValue::I32(r)) => {
                l.checked_add(*r)
                    .map(|v| LatticeValue::Constant(ConstantValue::I32(v)))
                    .unwrap_or(LatticeValue::Top)
            }
            (BinaryOp::Sub, ConstantValue::I32(l), ConstantValue::I32(r)) => {
                l.checked_sub(*r)
                    .map(|v| LatticeValue::Constant(ConstantValue::I32(v)))
                    .unwrap_or(LatticeValue::Top)
            }
            (BinaryOp::Mul, ConstantValue::I32(l), ConstantValue::I32(r)) => {
                l.checked_mul(*r)
                    .map(|v| LatticeValue::Constant(ConstantValue::I32(v)))
                    .unwrap_or(LatticeValue::Top)
            }
            (BinaryOp::Div, ConstantValue::I32(l), ConstantValue::I32(r)) => {
                if *r == 0 {
                    self.diagnostics.warn("Division by zero in constant expression");
                    LatticeValue::Top
                } else {
                    l.checked_div(*r)
                        .map(|v| LatticeValue::Constant(ConstantValue::I32(v)))
                        .unwrap_or(LatticeValue::Top)
                }
            }
            // ... similar for all other type/op combinations
            
            // Floating-point arithmetic (IEEE 754 semantics)
            (BinaryOp::Add, ConstantValue::F32(l), ConstantValue::F32(r)) => {
                LatticeValue::Constant(ConstantValue::F32(l + r))
            }
            // F32 division by zero produces Infinity (valid per IEEE 754, no warning)
            (BinaryOp::Div, ConstantValue::F32(l), ConstantValue::F32(r)) => {
                LatticeValue::Constant(ConstantValue::F32(l / r))
            }
            
            // Boolean logic
            (BinaryOp::And, ConstantValue::Bool(l), ConstantValue::Bool(r)) => {
                LatticeValue::Constant(ConstantValue::Bool(*l && *r))
            }
            (BinaryOp::Or, ConstantValue::Bool(l), ConstantValue::Bool(r)) => {
                LatticeValue::Constant(ConstantValue::Bool(*l || *r))
            }
            
            _ => LatticeValue::Top, // Unsupported operation
        }
    }
    
    /// Evaluate unary operation on constant operand.
    pub fn evaluate_unary_op(
        &self,
        op: UnaryOp,
        operand: &ConstantValue,
    ) -> LatticeValue {
        match (op, operand) {
            (UnaryOp::Neg, ConstantValue::I32(v)) => {
                v.checked_neg()
                    .map(|n| LatticeValue::Constant(ConstantValue::I32(n)))
                    .unwrap_or(LatticeValue::Top)
            }
            (UnaryOp::Not, ConstantValue::Bool(v)) => {
                LatticeValue::Constant(ConstantValue::Bool(!*v))
            }
            // ... similar for other types
            _ => LatticeValue::Top,
        }
    }
}
```

**Edge Case Handling**:

- **Integer Overflow**: Returns `LatticeValue::Top` (no warning per spec)
- **Division by Zero (Integer)**: Returns `LatticeValue::Top` + emits warning
- **Division by Zero (Float)**: Returns Infinity (valid IEEE 754, no warning)
- **NaN Propagation**: Preserves NaN through arithmetic (IEEE 754)
- **Type Mismatch**: Returns `LatticeValue::Top`

**Relationships**:

- Used by `SCCPropagator` to evaluate instructions
- Receives operand lattice values from `LatticeState`
- Produces `LatticeValue` results for SSA def-use propagation

---

### 5. CFGEdge

**Purpose**: Represents a control flow edge from one basic block to another.

**Location**: `src/ir/optimizer/constant_folding/propagator.rs`

**Definition**:

```rust
/// Control flow graph edge (from predecessor to successor block).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CFGEdge {
    pub from: BlockId,
    pub to: BlockId,
}
```

**Fields**:

- `from: BlockId`: Source basic block identifier
- `to: BlockId`: Destination basic block identifier

**Validation Rules**:

- Both `from` and `to` must be valid block IDs in the function's CFG
- Self-edges (loops) are valid
- Edges must correspond to actual terminator branches in IR

**State Transitions**:

```text
Initial State: All edges unmarked (except entry block successors → executable)
    ↓
Transition: Block with this edge as predecessor becomes executable
           AND terminator evaluation permits this edge
    ↓
Marked Executable (terminal state - never unmarked)
```

**Relationships**:

- Tracked in `ExecutableEdgeSet`
- Determined by terminator evaluation in `SCCPropagator`
- Used to filter phi node predecessors

---

### 6. ExecutableEdgeSet

**Purpose**: Tracks which CFG edges have been proven executable during SCCP analysis.

**Location**: `src/ir/optimizer/constant_folding/propagator.rs`

**Definition**:

```rust
/// Set of CFG edges proven to be executable.
/// 
/// Initially contains only edges from the entry block.
/// Grows monotonically as control flow is proven reachable.
#[derive(Debug)]
pub struct ExecutableEdgeSet {
    edges: HashSet<CFGEdge>,
}
```

**Fields**:

- `edges: HashSet<CFGEdge>`: Set of executable edges

**Operations**:

```rust
impl ExecutableEdgeSet {
    /// Create empty set with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            edges: HashSet::with_capacity(capacity),
        }
    }
    
    /// Mark an edge as executable (returns true if newly added).
    pub fn mark_executable(&mut self, edge: CFGEdge) -> bool {
        self.edges.insert(edge)
    }
    
    /// Check if an edge is executable.
    pub fn is_executable(&self, edge: CFGEdge) -> bool {
        self.edges.contains(&edge)
    }
    
    /// Check if any incoming edge to a block is executable.
    pub fn has_executable_predecessor(&self, block: BlockId) -> bool {
        self.edges.iter().any(|edge| edge.to == block)
    }
    
    /// Get all executable predecessors of a block.
    pub fn executable_predecessors(&self, block: BlockId) -> impl Iterator<Item = BlockId> + '_ {
        self.edges.iter()
            .filter(move |edge| edge.to == block)
            .map(|edge| edge.from)
    }
}
```

**Validation Rules**:

- Edges are added monotonically (never removed)
- Entry block successors are always executable
- Edge marking must be consistent with terminator evaluation

**Relationships**:

- Owned by `SCCPropagator`
- Updated when processing CFG worklist items
- Queried when evaluating phi nodes

---

### 7. Worklist (Generic)

**Purpose**: Generic FIFO queue with deduplication for work items during SCCP propagation.

**Location**: `src/ir/optimizer/constant_folding/propagator.rs`

**Definition**:

```rust
/// FIFO worklist with automatic deduplication.
/// 
/// Ensures each item is processed at most once per enqueue operation,
/// preventing redundant work during propagation.
#[derive(Debug)]
pub struct Worklist<T: Hash + Eq + Clone> {
    queue: VecDeque<T>,
    pending: HashSet<T>,
}
```

**Fields**:

- `queue: VecDeque<T>`: FIFO queue of work items
- `pending: HashSet<T>`: Set of items currently in queue (for deduplication)

**Type Parameters**:

- `T`: Work item type (must be `Hash + Eq + Clone`)

**Operations**:

```rust
impl<T: Hash + Eq + Clone> Worklist<T> {
    /// Create empty worklist with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            pending: HashSet::with_capacity(capacity),
        }
    }
    
    /// Add item to worklist (ignored if already pending).
    pub fn push(&mut self, item: T) {
        if self.pending.insert(item.clone()) {
            self.queue.push_back(item);
        }
    }
    
    /// Remove and return next item from worklist.
    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front().map(|item| {
            self.pending.remove(&item);
            item
        })
    }
    
    /// Check if worklist is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Get number of items in worklist.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
```

**Validation Rules**:

- `pending` set must always contain exactly the items in `queue`
- No duplicate items in `queue`

**Relationships**:

- Instantiated as `Worklist<CFGEdge>` for CFG edge worklist
- Instantiated as `Worklist<(ValueId, InstructionId)>` for SSA edge worklist
- Owned by `SCCPropagator`

---

### 8. SCCPropagator

**Purpose**: Implements the core SCCP worklist algorithm for constant propagation and unreachable code discovery.

**Location**: `src/ir/optimizer/constant_folding/propagator.rs`

**Definition**:

```rust
/// Sparse Conditional Constant Propagator.
/// 
/// Implements the Wegman-Zadeck algorithm for discovering constant values
/// and unreachable code through simultaneous SSA and CFG edge propagation.
#[derive(Debug)]
pub struct SCCPropagator {
    /// Lattice values for all SSA values
    lattice: LatticeState,
    
    /// Set of executable CFG edges
    executable_edges: ExecutableEdgeSet,
    
    /// Worklist of SSA edges to process (value-to-use)
    ssa_worklist: Worklist<(ValueId, InstructionId)>,
    
    /// Worklist of CFG edges to process (block-to-block)
    cfg_worklist: Worklist<CFGEdge>,
    
    /// Constant evaluator for instruction evaluation
    evaluator: ConstantEvaluator,
    
    /// Configuration options
    config: SCCPConfig,
    
    /// Iteration counter for convergence tracking
    iteration_count: usize,
}
```

**Fields**:

- `lattice`: Global lattice state (see LatticeState)
- `executable_edges`: Set of executable CFG edges (see ExecutableEdgeSet)
- `ssa_worklist`: Queue of SSA def-use edges requiring processing
- `cfg_worklist`: Queue of CFG edges requiring processing
- `evaluator`: Constant expression evaluator (see ConstantEvaluator)
- `config`: Configuration (see SCCPConfig)
- `iteration_count`: Tracks iterations for convergence metrics

**Operations**:

```rust
impl SCCPropagator {
    /// Create new propagator for a function.
    pub fn new_for_function(function: &Function, config: SCCPConfig) -> Self {
        let num_instructions = function.count_instructions();
        let num_blocks = function.basic_blocks().len();
        
        Self {
            lattice: LatticeState::with_capacity(num_instructions * 3 / 2),
            executable_edges: ExecutableEdgeSet::with_capacity(num_blocks * 2),
            ssa_worklist: Worklist::with_capacity(num_instructions / 2),
            cfg_worklist: Worklist::with_capacity(num_blocks),
            evaluator: ConstantEvaluator::new(),
            config,
            iteration_count: 0,
        }
    }
    
    /// Run SCCP analysis to convergence (or max iterations).
    pub fn propagate(&mut self, function: &Function) -> Result<(), SCCPError> {
        // Initialize: mark entry block edges executable
        for succ in function.entry_block().successors() {
            let edge = CFGEdge {
                from: function.entry_block().id(),
                to: succ,
            };
            self.cfg_worklist.push(edge);
        }
        
        // Initialize: set parameters to Top, locals to Bottom
        for param in function.parameters() {
            self.lattice.initialize(param.value_id(), LatticeValue::Top);
        }
        
        // Main propagation loop
        while !self.cfg_worklist.is_empty() || !self.ssa_worklist.is_empty() {
            self.iteration_count += 1;
            
            if self.iteration_count > self.config.max_iterations {
                return Err(SCCPError::MaxIterationsExceeded);
            }
            
            // Process CFG edges
            while let Some(edge) = self.cfg_worklist.pop() {
                self.visit_cfg_edge(edge, function)?;
            }
            
            // Process SSA edges
            while let Some((value, use_instr)) = self.ssa_worklist.pop() {
                self.visit_ssa_edge(value, use_instr, function)?;
            }
        }
        
        Ok(())
    }
    
    /// Visit a CFG edge (mark executable and process destination block).
    fn visit_cfg_edge(&mut self, edge: CFGEdge, function: &Function) -> Result<(), SCCPError> {
        // Mark edge executable
        let newly_executable = self.executable_edges.mark_executable(edge);
        
        if newly_executable {
            let block = function.get_block(edge.to)?;
            
            // Visit all phi nodes
            for phi in block.phi_nodes() {
                self.visit_phi(phi, block.id())?;
            }
            
            // Visit all instructions
            for instr in block.instructions() {
                self.visit_instruction(instr)?;
            }
            
            // Visit terminator
            self.visit_terminator(block.terminator(), block.id())?;
        }
        
        Ok(())
    }
    
    /// Visit an SSA edge (re-evaluate use instruction).
    fn visit_ssa_edge(
        &mut self,
        value: ValueId,
        use_instr: InstructionId,
        function: &Function,
    ) -> Result<(), SCCPError> {
        let instr = function.get_instruction(use_instr)?;
        self.visit_instruction(instr)
    }
    
    /// Evaluate phi node and update lattice.
    fn visit_phi(&mut self, phi: &PhiNode, block_id: BlockId) -> Result<(), SCCPError> {
        let mut result = LatticeValue::Bottom;
        
        for (pred_block, value) in phi.incoming_values() {
            let edge = CFGEdge { from: pred_block, to: block_id };
            
            if self.executable_edges.is_executable(edge) {
                let pred_value = self.lattice.get(value);
                result = result.meet(&pred_value);
            }
        }
        
        if self.lattice.update(phi.result_value(), result) {
            // Lattice changed: propagate to users
            for user in phi.result_value().users() {
                self.ssa_worklist.push((phi.result_value(), user));
            }
        }
        
        Ok(())
    }
    
    /// Evaluate instruction and update lattice.
    fn visit_instruction(&mut self, instr: &Instruction) -> Result<(), SCCPError> {
        let result_lattice = match instr {
            Instruction::BinaryOp { op, left, right, .. } => {
                let left_val = self.lattice.get(*left);
                let right_val = self.lattice.get(*right);
                
                match (left_val, right_val) {
                    (LatticeValue::Constant(l), LatticeValue::Constant(r)) => {
                        self.evaluator.evaluate_binary_op(*op, &l, &r)
                    }
                    (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => {
                        LatticeValue::Bottom
                    }
                    _ => LatticeValue::Top,
                }
            }
            // ... other instruction types
            _ => LatticeValue::Top, // Conservative default
        };
        
        if let Some(result_value) = instr.result_value() {
            if self.lattice.update(result_value, result_lattice) {
                for user in result_value.users() {
                    self.ssa_worklist.push((result_value, user));
                }
            }
        }
        
        Ok(())
    }
    
    /// Evaluate terminator and mark outgoing CFG edges.
    fn visit_terminator(&mut self, term: &Terminator, block_id: BlockId) -> Result<(), SCCPError> {
        match term {
            Terminator::Branch { target } => {
                let edge = CFGEdge { from: block_id, to: *target };
                if self.executable_edges.mark_executable(edge) {
                    self.cfg_worklist.push(edge);
                }
            }
            Terminator::ConditionalBranch { condition, true_target, false_target } => {
                let cond_lattice = self.lattice.get(*condition);
                
                match cond_lattice {
                    LatticeValue::Constant(ConstantValue::Bool(true)) => {
                        let edge = CFGEdge { from: block_id, to: *true_target };
                        if self.executable_edges.mark_executable(edge) {
                            self.cfg_worklist.push(edge);
                        }
                    }
                    LatticeValue::Constant(ConstantValue::Bool(false)) => {
                        let edge = CFGEdge { from: block_id, to: *false_target };
                        if self.executable_edges.mark_executable(edge) {
                            self.cfg_worklist.push(edge);
                        }
                    }
                    _ => {
                        // Non-constant: both edges potentially executable
                        let true_edge = CFGEdge { from: block_id, to: *true_target };
                        let false_edge = CFGEdge { from: block_id, to: *false_target };
                        
                        if self.executable_edges.mark_executable(true_edge) {
                            self.cfg_worklist.push(true_edge);
                        }
                        if self.executable_edges.mark_executable(false_edge) {
                            self.cfg_worklist.push(false_edge);
                        }
                    }
                }
            }
            // ... other terminator types
            _ => {}
        }
        
        Ok(())
    }
    
    /// Get final lattice state after propagation.
    pub fn get_lattice_state(&self) -> &LatticeState {
        &self.lattice
    }
    
    /// Get iteration count for statistics.
    pub fn iteration_count(&self) -> usize {
        self.iteration_count
    }
}
```

**Validation Rules**:

- Worklists must be empty at convergence
- All executable blocks must have at least one executable incoming edge (except entry)
- Lattice values must be monotonic throughout propagation

**Relationships**:

- Owns `LatticeState`, `ExecutableEdgeSet`, worklists, and `ConstantEvaluator`
- Created and invoked by `ConstantFoldingOptimizer`
- Produces lattice state consumed by `IRRewriter`

---

### 9. IRRewriter

**Purpose**: Transforms IR based on SCCP analysis results by replacing constant computations and simplifying control flow.

**Location**: `src/ir/optimizer/constant_folding/rewriter.rs`

**Definition**:

```rust
/// IR rewriter for applying SCCP optimization results.
/// 
/// Transforms the IR by:
/// - Replacing instructions computing constants with constant assignments
/// - Simplifying phi nodes with constant values
/// - Marking unreachable blocks for DCE
#[derive(Debug)]
pub struct IRRewriter<'a> {
    lattice: &'a LatticeState,
    executable_edges: &'a ExecutableEdgeSet,
    stats: OptimizationStats,
}
```

**Fields**:

- `lattice: &'a LatticeState`: Reference to final lattice state from propagator
- `executable_edges: &'a ExecutableEdgeSet`: Reference to executable edges
- `stats: OptimizationStats`: Accumulates optimization metrics

**Operations**:

```rust
impl<'a> IRRewriter<'a> {
    /// Create rewriter from propagation results.
    pub fn new(lattice: &'a LatticeState, executable_edges: &'a ExecutableEdgeSet) -> Self {
        Self {
            lattice,
            executable_edges,
            stats: OptimizationStats::default(),
        }
    }
    
    /// Rewrite function based on SCCP results.
    pub fn rewrite_function(&mut self, function: &mut Function) -> Result<(), RewriteError> {
        // Rewrite all blocks
        for block in function.basic_blocks_mut() {
            self.rewrite_block(block)?;
        }
        
        Ok(())
    }
    
    /// Rewrite a single basic block.
    fn rewrite_block(&mut self, block: &mut BasicBlock) -> Result<(), RewriteError> {
        // Mark unreachable blocks
        if !self.executable_edges.has_executable_predecessor(block.id()) {
            block.mark_unreachable();
            self.stats.blocks_marked_unreachable += 1;
            return Ok(()); // Skip rewriting dead code
        }
        
        // Rewrite phi nodes
        for phi in block.phi_nodes_mut() {
            self.rewrite_phi(phi)?;
        }
        
        // Rewrite instructions
        for instr in block.instructions_mut() {
            self.rewrite_instruction(instr)?;
        }
        
        // Rewrite terminator
        self.rewrite_terminator(block.terminator_mut(), block.id())?;
        
        Ok(())
    }
    
    /// Rewrite phi node if all executable values are constant.
    fn rewrite_phi(&mut self, phi: &mut PhiNode) -> Result<(), RewriteError> {
        let result_value = phi.result_value();
        let lattice_value = self.lattice.get(result_value);
        
        if let LatticeValue::Constant(const_val) = lattice_value {
            // Replace phi with constant assignment
            phi.replace_with_constant(const_val);
            self.stats.phi_nodes_simplified += 1;
        }
        
        Ok(())
    }
    
    /// Rewrite instruction if result is constant.
    fn rewrite_instruction(&mut self, instr: &mut Instruction) -> Result<(), RewriteError> {
        if let Some(result_value) = instr.result_value() {
            let lattice_value = self.lattice.get(result_value);
            
            if let LatticeValue::Constant(const_val) = lattice_value {
                // Replace instruction with constant assignment
                instr.replace_with_constant(const_val);
                self.stats.constants_propagated += 1;
            }
        }
        
        Ok(())
    }
    
    /// Rewrite terminator if branch condition is constant.
    fn rewrite_terminator(
        &mut self,
        term: &mut Terminator,
        block_id: BlockId,
    ) -> Result<(), RewriteError> {
        if let Terminator::ConditionalBranch { condition, true_target, false_target } = term {
            let cond_lattice = self.lattice.get(*condition);
            
            match cond_lattice {
                LatticeValue::Constant(ConstantValue::Bool(true)) => {
                    // Replace conditional branch with unconditional branch to true target
                    *term = Terminator::Branch { target: *true_target };
                    self.stats.branches_resolved += 1;
                }
                LatticeValue::Constant(ConstantValue::Bool(false)) => {
                    // Replace conditional branch with unconditional branch to false target
                    *term = Terminator::Branch { target: *false_target };
                    self.stats.branches_resolved += 1;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Get optimization statistics.
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }
}
```

**Validation Rules**:

- Must preserve SSA form (single assignment invariant)
- Must not modify unreachable blocks (except marking)
- Constant replacements must be type-correct

**Relationships**:

- Consumes `LatticeState` and `ExecutableEdgeSet` from `SCCPropagator`
- Modifies `Function` IR in-place
- Produces `OptimizationStats` for reporting

---

### 10. SCCPConfig

**Purpose**: Configuration options for SCCP optimizer behavior.

**Location**: `src/ir/optimizer/constant_folding/optimizer.rs`

**Definition**:

```rust
/// Configuration for SCCP optimizer.
#[derive(Debug, Clone)]
pub struct SCCPConfig {
    /// Enable verbose diagnostic output
    pub verbose: bool,
    
    /// Maximum iterations before timeout
    pub max_iterations: usize,
}
```

**Fields**:

- `verbose: bool`: Enable detailed logging of lattice transitions and worklist operations
- `max_iterations: usize`: Maximum propagation iterations (default 100)

**Default Implementation**:

```rust
impl Default for SCCPConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            max_iterations: 100,
        }
    }
}
```

**Relationships**:

- Passed to `SCCPropagator` during construction
- Configured by `ConstantFoldingOptimizer`

---

### 11. OptimizationStats

**Purpose**: Tracks optimization metrics for reporting and debugging.

**Location**: `src/ir/optimizer/constant_folding/optimizer.rs`

**Definition**:

```rust
/// Statistics about SCCP optimization pass.
#[derive(Debug, Default, Clone)]
pub struct OptimizationStats {
    /// Number of constants propagated
    pub constants_propagated: usize,
    
    /// Number of branches resolved to unconditional
    pub branches_resolved: usize,
    
    /// Number of phi nodes simplified
    pub phi_nodes_simplified: usize,
    
    /// Number of blocks marked unreachable
    pub blocks_marked_unreachable: usize,
    
    /// Number of iterations to convergence
    pub iterations: usize,
}
```

**Fields**: All fields are counters accumulated during optimization.

**Operations**:

```rust
impl OptimizationStats {
    /// Create empty statistics.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Combine statistics from multiple optimizations.
    pub fn merge(&mut self, other: &Self) {
        self.constants_propagated += other.constants_propagated;
        self.branches_resolved += other.branches_resolved;
        self.phi_nodes_simplified += other.phi_nodes_simplified;
        self.blocks_marked_unreachable += other.blocks_marked_unreachable;
        self.iterations = self.iterations.max(other.iterations);
    }
}

impl std::fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SCCP: {} constants propagated, {} branches resolved, \
             {} phi nodes simplified, {} blocks marked unreachable, \
             {} iterations",
            self.constants_propagated,
            self.branches_resolved,
            self.phi_nodes_simplified,
            self.blocks_marked_unreachable,
            self.iterations
        )
    }
}
```

**Relationships**:

- Updated by `IRRewriter` during IR transformation
- Owned by `ConstantFoldingOptimizer`
- Reported to user after optimization

---

### 12. ConstantFoldingOptimizer

**Purpose**: Top-level orchestrator implementing the Phase trait for integration with the optimization pipeline.

**Location**: `src/ir/optimizer/constant_folding/optimizer.rs`

**Definition**:

```rust
/// Constant Folding Optimizer with Sparse Conditional Constant Propagation.
/// 
/// Implements the Phase trait for integration with the jsavrs optimization pipeline.
pub struct ConstantFoldingOptimizer {
    config: SCCPConfig,
    stats: OptimizationStats,
}
```

**Fields**:

- `config: SCCPConfig`: Configuration options
- `stats: OptimizationStats`: Accumulated statistics across all functions

**Operations**:

```rust
impl ConstantFoldingOptimizer {
    /// Create new optimizer with default configuration.
    pub fn new() -> Self {
        Self {
            config: SCCPConfig::default(),
            stats: OptimizationStats::new(),
        }
    }
    
    /// Create optimizer with custom configuration.
    pub fn with_config(config: SCCPConfig) -> Self {
        Self {
            config,
            stats: OptimizationStats::new(),
        }
    }
    
    /// Optimize a single function.
    fn optimize_function(&mut self, function: &mut Function) -> Result<(), SCCPError> {
        // Create propagator
        let mut propagator = SCCPropagator::new_for_function(function, self.config.clone());
        
        // Run propagation to convergence
        propagator.propagate(function)?;
        
        // Rewrite IR based on results
        let mut rewriter = IRRewriter::new(
            propagator.get_lattice_state(),
            propagator.get_executable_edges(),
        );
        rewriter.rewrite_function(function)?;
        
        // Accumulate statistics
        let mut func_stats = rewriter.get_stats().clone();
        func_stats.iterations = propagator.iteration_count();
        self.stats.merge(&func_stats);
        
        Ok(())
    }
    
    /// Get accumulated optimization statistics.
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn run(&mut self, module: &mut Module) -> Result<(), OptimizationError> {
        for function in module.functions_mut() {
            self.optimize_function(function)
                .map_err(|e| OptimizationError::SCCPError(e))?;
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Constant Folding (SCCP)"
    }
}
```

**Validation Rules**:

- Must not modify module structure (number of functions)
- Must preserve SSA form for all functions
- Must handle errors gracefully without panicking

**Relationships**:

- Implements `Phase` trait for pipeline integration
- Orchestrates `SCCPropagator` and `IRRewriter`
- Reports statistics to caller

---

## Entity Relationships Diagram

```text
┌─────────────────────────────┐
│ ConstantFoldingOptimizer    │ (Implements Phase trait)
│ ├─ config: SCCPConfig       │
│ └─ stats: OptimizationStats │
└──────────┬──────────────────┘
           │ creates & owns
           ▼
┌─────────────────────────────┐
│ SCCPropagator               │
│ ├─ lattice: LatticeState    │◄────────┐
│ ├─ executable_edges: ...    │         │
│ ├─ ssa_worklist: Worklist   │         │
│ ├─ cfg_worklist: Worklist   │         │ references
│ ├─ evaluator: ...           │         │
│ └─ config: SCCPConfig       │         │
└──────────┬──────────────────┘         │
           │ produces results           │
           ▼                            │
┌─────────────────────────────┐         │
│ IRRewriter                  │         │
│ ├─ lattice: &LatticeState   │─────────┘
│ ├─ executable_edges: &...   │
│ └─ stats: OptimizationStats │
└──────────┬──────────────────┘
           │ modifies
           ▼
┌─────────────────────────────┐
│ Function (IR)               │
│ ├─ basic_blocks             │
│ ├─ instructions             │
│ └─ values                   │
└─────────────────────────────┘

Data Flow:
LatticeState ──stores──> HashMap<ValueId, LatticeValue>
LatticeValue ──contains──> ConstantValue (when Constant variant)
ExecutableEdgeSet ──stores──> HashSet<CFGEdge>
Worklist<T> ──stores──> VecDeque<T> + HashSet<T>
```

## State Transition Diagrams

### Lattice Value State Transitions

```text
                    ┌───────────┐
        ┌───────────│  Bottom   │◄─────────┐
        │           │    (⊥)    │          │ Uninitialized value
        │           └─────┬─────┘          │
        │                 │                │
        │  Evaluation     │                │
        │  produces       │                │
        │  constant       │                │
        │                 ▼                │
        │           ┌───────────┐          │
        │           │ Constant  │          │
        └──────────►│   (v)     │          │
         Conflicting└─────┬─────┘          │
         constants        │                │
         meet             │ Conflicting    │
                          │ values         │
                          ▼ meet           │
                    ┌───────────┐          │
                    │    Top    │──────────┘
                    │    (⊤)    │   All subsequent
                    └───────────┘   evaluations
                    (Terminal state)
```

### CFG Edge State Transitions

```text
                ┌─────────────────┐
                │   Unmarked      │
                │ (not executable)│
                └────────┬────────┘
                         │
         Predecessor     │
         becomes         │
         executable      │
         AND             │
         terminator      │
         allows edge     │
                         ▼
                ┌─────────────────┐
                │  Executable     │
                │  (marked true)  │
                └─────────────────┘
                (Terminal state -
                 never unmarked)
```

### SCCP Algorithm Flow

```text
┌─────────────────────────────────────────┐
│ Initialize                              │
│ - Parameters/globals → Top              │
│ - Locals → Bottom                       │
│ - Entry block edges → CFG worklist      │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Main Loop (until worklists empty)       │
│ ┌─────────────────────────────────────┐ │
│ │ Process CFG Worklist                │ │
│ │ - Pop edge (pred → succ)            │ │
│ │ - Mark edge executable              │ │
│ │ - Visit phi nodes in succ           │ │
│ │ - Visit instructions in succ        │ │
│ │ - Evaluate terminator               │ │
│ │ - Add outgoing edges to CFG list    │ │
│ └─────────────────────────────────────┘ │
│ ┌─────────────────────────────────────┐ │
│ │ Process SSA Worklist                │ │
│ │ - Pop (value, use_instruction)      │ │
│ │ - Re-evaluate use_instruction       │ │
│ │ - Update lattice if changed         │ │
│ │ - Add users to SSA worklist         │ │
│ └─────────────────────────────────────┘ │
└──────────────┬──────────────────────────┘
               │ Converged
               ▼
┌─────────────────────────────────────────┐
│ Rewrite IR                              │
│ - Replace constant computations         │
│ - Simplify phi nodes                    │
│ - Resolve constant branches             │
│ - Mark unreachable blocks               │
└─────────────────────────────────────────┘
```

## Validation and Invariants

### Global Invariants (maintained throughout optimization)

1. **SSA Form Preservation**:
   - Every value has exactly one static definition
   - All uses are dominated by definitions
   - Phi nodes correctly merge values from predecessors

2. **Lattice Monotonicity**:
   - Lattice values never decrease in ordering: ⊥ → Constant → ⊤
   - Once a value reaches ⊤, it never changes
   - Meet operations preserve lattice ordering

3. **CFG Consistency**:
   - Executable edges form a valid subgraph of original CFG
   - All executable blocks have at least one executable incoming edge (except entry)
   - Entry block is always executable

4. **Worklist Correctness**:
   - No duplicate items in worklists
   - All pending work eventually processed
   - Convergence guaranteed by lattice monotonicity

### Post-Optimization Validation

1. **IR Validity**:
   - All instructions have valid operands
   - Types are consistent across operations
   - Control flow is well-formed

2. **Optimization Soundness**:
   - Constant replacements preserve semantics
   - Branch resolutions preserve reachability
   - Phi simplifications maintain correct values

3. **Performance Metrics**:
   - Convergence within iteration limits
   - Linear time complexity verified by benchmarks
   - Memory usage within expected bounds

---

**Data Model Status**: ✅ Complete  
**Next Phase**: Phase 1 - Contracts (API specifications)  
**Approver**: [Pending review]
