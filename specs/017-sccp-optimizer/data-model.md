# Data Model: SCCP Optimizer

**Feature**: SCCP Optimizer  
**Branch**: `017-sccp-optimizer`  
**Date**: 2025-11-19

## Overview

This document provides a comprehensive, detailed, and precise specification of all data structures, entities, and their relationships within the Sparse Conditional Constant Propagation (SCCP) optimization phase. The data model is designed to support efficient sparse dataflow analysis on SSA-form intermediate representation with strict correctness guarantees through lattice-theoretic foundations.

## Core Entities

### 1. LatticeValue

**Purpose**: Represents the abstract state of knowledge about an SSA value's compile-time determinability.

**Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    /// Top (⊤): Not yet analyzed, no information known about the value
    Top,
    
    /// Constant: Provably holds a specific constant value across all executions
    Constant(IrLiteralValue),
    
    /// Bottom (⊥): Variable, can have different values at runtime
    Bottom,
}
```

**Fields**:
- `Top`: Unit variant representing the initial unknown state
- `Constant(IrLiteralValue)`: Wrapper around a concrete compile-time constant value
- `Bottom`: Unit variant representing non-constant (runtime-determined) state

**Lattice Ordering**: Top > Constant > Bottom (where > means "less precise than")

**Invariants**:
- Values only move down the lattice (monotonic)
- Once Bottom, always Bottom (no upward movement)
- Two different constants meet to Bottom (different values = variable)

**Operations**:

**Meet Operation** (greatest lower bound):
```rust
impl LatticeValue {
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (LatticeValue::Top, v) | (v, LatticeValue::Top) => v.clone(),
            (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) => {
                if c1 == c2 {
                    LatticeValue::Constant(c1.clone())
                } else {
                    LatticeValue::Bottom
                }
            }
        }
    }
}
```

**Properties**:
- **Commutativity**: `a.meet(b) == b.meet(a)`
- **Associativity**: `(a.meet(b)).meet(c) == a.meet(b.meet(c))`
- **Idempotence**: `a.meet(a) == a`

**Validation Rules**:
- Never transition from Bottom to Constant or Top
- Never transition from Constant to Top
- IrLiteralValue must be valid for its type

**Relationships**:
- **Contains**: `IrLiteralValue` (when in Constant state)
- **Used By**: `SccpAnalyzer` (tracks all SSA values)
- **Computed By**: `constant_folder` module (produces Constant values)

---

### 2. SccpAnalyzer

**Purpose**: Orchestrates the sparse conditional constant propagation dataflow analysis using worklist-driven iteration until fixed-point convergence.

**Definition**:
```rust
pub struct SccpAnalyzer {
    /// Maps each SSA value to its current lattice state
    lattice_values: HashMap<Value, LatticeValue>,
    
    /// Set of basic blocks determined to be reachable during analysis
    executable_blocks: HashSet<NodeIndex>,
    
    /// Work items requiring processing (SSA values and CFG edges)
    worklist: Worklist,
    
    /// Maximum iterations before conservative fallback (default: 10,000)
    max_iterations: usize,
    
    /// Current iteration count for termination detection
    iteration_count: usize,
}
```

**Fields**:

- **lattice_values**: `HashMap<Value, LatticeValue>`
  - **Purpose**: Central analysis state tracking constant/variable status of every SSA value
  - **Key**: `Value` - SSA value identifier from IR
  - **Value**: `LatticeValue` - Current abstract state (Top/Constant/Bottom)
  - **Initialization**: All values start as `Top` (unknown)
  - **Capacity**: Pre-allocated to estimated instruction count for O(1) insertion

- **executable_blocks**: `HashSet<NodeIndex>`
  - **Purpose**: Tracks which basic blocks are determined to be reachable through executable CFG paths
  - **Element**: `NodeIndex` - Unique identifier for basic block in CFG
  - **Initialization**: Empty set; entry block added during setup
  - **Usage**: Only process instructions in executable blocks; ignore dead code

- **worklist**: `Worklist`
  - **Purpose**: Maintains work items requiring processing during iterative analysis
  - **Type**: Custom struct containing SSA and CFG worklists (see Worklist entity)
  - **Processing**: FIFO order with deduplication to prevent redundant work

- **max_iterations**: `usize`
  - **Purpose**: Safety limit to prevent infinite loops on malformed IR
  - **Default**: 10,000 iterations
  - **Behavior on Exceed**: Degrade all remaining Top values to Bottom (conservative)

- **iteration_count**: `usize`
  - **Purpose**: Tracks current iteration for termination and statistics
  - **Initialization**: 0
  - **Increment**: Each worklist processing cycle

**Invariants**:
- `lattice_values` contains entry for every SSA value in the function
- `executable_blocks` contains at least the entry block (if valid IR)
- All values in `lattice_values` follow monotonic lattice ordering over time
- `iteration_count <= max_iterations` at all times

**Lifecycle**:

1. **Construction**: `SccpAnalyzer::new(max_iterations: usize)`
2. **Initialization**: `initialize(&mut self, function: &Function)`
   - Populate `lattice_values` with all SSA values at Top
   - Mark entry block as executable
   - Add entry block's outgoing edges to worklist
3. **Analysis**: `analyze(&mut self, function: &Function) -> AnalysisResult`
   - Iteratively process worklists until convergence
   - Update lattice values as constants are discovered
   - Mark blocks executable as branches resolve
4. **Completion**: Return `AnalysisResult` with final lattice states and executable blocks

**Operations**:

- **evaluate_instruction**: Computes new lattice value for instruction result based on operand lattice values
- **update_phi**: Special handling for phi node analysis considering only executable predecessors
- **mark_executable**: Adds block to executable set and processes its instructions
- **propagate**: Adds uses of a changed value to SSA worklist

**Relationships**:
- **Uses**: `Worklist` (manages work items)
- **Uses**: `LatticeValue` (represents analysis state)
- **Uses**: `constant_folder` (evaluates constant expressions)
- **Produces**: `AnalysisResult` (output for transformation phase)
- **Operates On**: `Function` from IR module

---

### 3. Worklist

**Purpose**: Efficiently manages pending work items during sparse dataflow analysis with automatic deduplication to prevent redundant processing.

**Definition**:
```rust
pub struct Worklist {
    /// SSA values whose lattice state has changed and need propagation
    ssa_worklist: VecDeque<Value>,
    
    /// CFG edges that have become executable and need processing
    cfg_worklist: VecDeque<(NodeIndex, NodeIndex)>,
    
    /// Tracks SSA values already in worklist to prevent duplicates
    ssa_seen: HashSet<Value>,
    
    /// Tracks CFG edges already in worklist to prevent duplicates
    cfg_seen: HashSet<(NodeIndex, NodeIndex)>,
}
```

**Fields**:

- **ssa_worklist**: `VecDeque<Value>`
  - **Purpose**: Queue of SSA values requiring re-evaluation of their uses
  - **Element**: `Value` - SSA value ID whose lattice state changed
  - **Order**: FIFO (first-in-first-out) for predictable processing
  - **Size**: Bounded by total number of SSA values in function

- **cfg_worklist**: `VecDeque<(NodeIndex, NodeIndex)>`
  - **Purpose**: Queue of control flow edges that became executable
  - **Element**: `(NodeIndex, NodeIndex)` - Tuple of (source_block, destination_block)
  - **Order**: FIFO for breadth-first CFG traversal
  - **Size**: Bounded by total number of CFG edges in function

- **ssa_seen**: `HashSet<Value>`
  - **Purpose**: Deduplication set preventing same SSA value from being queued multiple times
  - **Behavior**: Checked before insertion; prevents redundant work
  - **Cleared**: When item is dequeued (allows re-insertion if value changes again)

- **cfg_seen**: `HashSet<(NodeIndex, NodeIndex)>`
  - **Purpose**: Deduplication set preventing same CFG edge from being queued multiple times
  - **Behavior**: Once an edge is executable, it stays executable (never re-queued)
  - **Persistent**: Not cleared; edge execution is permanent state

**Invariants**:
- `ssa_worklist.len() <= total_ssa_values`
- `cfg_worklist.len() <= total_cfg_edges`
- Every element in `ssa_worklist` exists in `ssa_seen`
- Every element in `cfg_worklist` exists in `cfg_seen`
- No duplicate values within each worklist

**Operations**:

```rust
impl Worklist {
    pub fn new() -> Self;
    
    /// Add SSA value to worklist if not already present
    pub fn add_ssa(&mut self, value: Value);
    
    /// Add CFG edge to worklist if not already present
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex);
    
    /// Remove and return next SSA value, or None if empty
    pub fn pop_ssa(&mut self) -> Option<Value>;
    
    /// Remove and return next CFG edge, or None if empty
    pub fn pop_cfg(&mut self) -> Option<(NodeIndex, NodeIndex)>;
    
    /// Check if both worklists are empty (fixed-point reached)
    pub fn is_empty(&self) -> bool;
}
```

**Performance Characteristics**:
- **add_ssa**: O(1) average (HashMap lookup + VecDeque append)
- **add_edge**: O(1) average (HashMap lookup + VecDeque append)
- **pop_ssa**: O(1) (VecDeque pop_front)
- **pop_cfg**: O(1) (VecDeque pop_front)
- **is_empty**: O(1) (length checks)

**Memory Usage**: O(V + E) where V = SSA values, E = CFG edges

**Relationships**:
- **Owned By**: `SccpAnalyzer`
- **Manages**: `Value` references (SSA values)
- **Manages**: `NodeIndex` pairs (CFG edges)

---

### 4. AnalysisResult

**Purpose**: Encapsulates the complete output state of SCCP analysis, providing transformation phase with all necessary information to mutate IR.

**Definition**:
```rust
pub struct AnalysisResult {
    /// Final lattice value for each SSA value after fixed-point convergence
    pub lattice_values: HashMap<Value, LatticeValue>,
    
    /// Set of basic blocks determined to be reachable
    pub executable_blocks: HashSet<NodeIndex>,
    
    /// Number of worklist iterations performed
    pub iterations: usize,
    
    /// Whether analysis completed normally or hit iteration limit
    pub converged: bool,
}
```

**Fields**:

- **lattice_values**: `HashMap<Value, LatticeValue>`
  - **Purpose**: Complete constant propagation results for entire function
  - **Content**: Every SSA value mapped to its final lattice state
  - **Usage by Transformer**: Identify which values are constant for replacement

- **executable_blocks**: `HashSet<NodeIndex>`
  - **Purpose**: Complete reachability information for CFG
  - **Content**: All blocks determined to be reachable from entry
  - **Usage by Transformer**: Identify unreachable blocks and dead phi predecessors

- **iterations**: `usize`
  - **Purpose**: Convergence statistics for performance monitoring
  - **Value**: Number of worklist processing cycles executed
  - **Usage**: Logging and performance analysis

- **converged**: `bool`
  - **Purpose**: Indicates whether analysis reached fixed-point naturally
  - **True**: Worklists emptied naturally (normal termination)
  - **False**: Iteration limit exceeded (conservative fallback occurred)
  - **Usage**: Warn user about potential incomplete optimization

**Invariants**:
- All keys in `lattice_values` are valid SSA values from the analyzed function
- `executable_blocks` contains at least the entry block (if valid IR)
- `iterations > 0` (at least one iteration always executes)
- If `converged == false`, then `iterations == max_iterations`

**Validation Rules**:
- No lattice value is Top if `converged == true` (all unknowns should be resolved)
- All values in `lattice_values` are Bottom or Constant if `converged == false` (conservative)

**Relationships**:
- **Produced By**: `SccpAnalyzer::analyze()`
- **Consumed By**: `SccpTransformer::transform()`
- **Contains**: `LatticeValue` instances (analysis results)

---

### 5. SccpTransformer

**Purpose**: Performs in-place mutation of IR based on SCCP analysis results, replacing constant values, simplifying branches, and marking dead code.

**Definition**:
```rust
pub struct SccpTransformer {
    /// Statistics collected during transformation
    stats: TransformStats,
    
    /// Whether to log detailed transformation actions
    verbose: bool,
}
```

**Fields**:

- **stats**: `TransformStats`
  - **Purpose**: Tracks transformation actions for reporting and verification
  - **Content**: Counts of replaced values, simplified branches, cleaned phi nodes
  - **Type**: See TransformStats entity

- **verbose**: `bool`
  - **Purpose**: Controls logging verbosity during transformation
  - **True**: Log each transformation action (constant replacement, branch simplification)
  - **False**: Silent operation (only final statistics)

**Operations**:

```rust
impl SccpTransformer {
    pub fn new(verbose: bool) -> Self;
    
    /// Main transformation entry point
    pub fn transform(&mut self, function: &mut Function, result: &AnalysisResult) -> bool;
    
    /// Replace uses of constant SSA values with literal constants
    fn replace_constants(&mut self, function: &mut Function, result: &AnalysisResult);
    
    /// Convert conditional branches with constant conditions to unconditional
    fn simplify_branches(&mut self, function: &mut Function, result: &AnalysisResult);
    
    /// Remove dead predecessors from phi nodes
    fn clean_phi_nodes(&mut self, function: &mut Function, result: &AnalysisResult);
    
    /// Mark unreachable blocks for DCE removal
    fn mark_unreachable(&mut self, function: &mut Function, result: &AnalysisResult);
}
```

**Transformation Actions**:

1. **Constant Replacement**:
   - For each SSA value with `LatticeValue::Constant(c)`
   - Replace all uses with literal constant `c`
   - Mark defining instruction as dead (flag for DCE)

2. **Branch Simplification**:
   - For each `ConditionalBranch` with constant condition
   - Replace with `Branch` (unconditional) to taken target
   - Update CFG edges (remove non-taken edge)

3. **Phi Node Cleanup**:
   - For each phi node in executable blocks
   - Remove incoming entries from non-executable predecessors
   - If phi becomes constant after cleanup, replace uses and mark dead

4. **Unreachable Marking**:
   - Blocks not in `executable_blocks` set remain unmarked
   - DCE will remove them in subsequent pass

**Return Value**: `bool` indicating whether any IR modifications were made

**Relationships**:
- **Consumes**: `AnalysisResult` (read-only input)
- **Mutates**: `Function` (in-place IR modification)
- **Produces**: `TransformStats` (side effect tracking)

---

### 6. TransformStats

**Purpose**: Comprehensive statistics tracking all transformation actions for reporting, debugging, and performance analysis.

**Definition**:
```rust
#[derive(Debug, Clone, Default)]
pub struct TransformStats {
    /// Number of SSA values replaced with constant literals
    pub constants_propagated: usize,
    
    /// Number of instructions marked dead after constant replacement
    pub instructions_marked_dead: usize,
    
    /// Number of conditional branches simplified to unconditional
    pub branches_simplified: usize,
    
    /// Number of phi nodes cleaned (dead predecessors removed)
    pub phi_nodes_cleaned: usize,
    
    /// Number of phi nodes fully replaced with constants
    pub phi_nodes_replaced: usize,
    
    /// Number of basic blocks marked unreachable
    pub unreachable_blocks: usize,
}
```

**Fields**: Each field is a counter (`usize`) tracking specific transformation actions.

**Operations**:

```rust
impl TransformStats {
    pub fn new() -> Self;
    
    /// Combine statistics from multiple transformations
    pub fn merge(&mut self, other: &TransformStats);
    
    /// Check if any transformations occurred
    pub fn has_changes(&self) -> bool {
        self.constants_propagated > 0 
            || self.branches_simplified > 0 
            || self.phi_nodes_cleaned > 0
    }
}
```

**Usage Pattern**:
```rust
// In SccpTransformer
self.stats.constants_propagated += 1;
self.stats.instructions_marked_dead += 1;

// After transformation
if verbose {
    println!("SCCP: Propagated {} constants", stats.constants_propagated);
    println!("SCCP: Simplified {} branches", stats.branches_simplified);
}
```

**Relationships**:
- **Owned By**: `SccpTransformer`
- **Reported By**: `SccpOptimizer` (after transformation completes)

---

### 7. SccpOptimizer

**Purpose**: High-level public API implementing the Phase trait for integration with compiler pipeline.

**Definition**:
```rust
pub struct SccpOptimizer {
    /// Enable verbose logging output
    pub verbose: bool,
    
    /// Maximum analysis iterations before conservative fallback
    pub max_iterations: usize,
    
    /// Master enable/disable switch
    pub enabled: bool,
}
```

**Fields**:

- **verbose**: `bool`
  - Controls logging for both analysis and transformation phases
  
- **max_iterations**: `usize`
  - Passed to SccpAnalyzer as safety limit
  - Default: 10,000
  
- **enabled**: `bool`
  - Master switch to disable optimization entirely
  - When false, `run()` returns immediately without modifications

**Operations**:

```rust
impl SccpOptimizer {
    pub fn new() -> Self;
    pub fn with_verbose(mut self) -> Self;
    pub fn with_max_iterations(mut self, max: usize) -> Self;
    
    fn run_on_function(&mut self, function: &mut Function) -> bool {
        // 1. Analyze
        let mut analyzer = SccpAnalyzer::new(self.max_iterations);
        let result = analyzer.analyze(function);
        
        // 2. Transform
        let mut transformer = SccpTransformer::new(self.verbose);
        transformer.transform(function, &result)
    }
}

impl Phase for SccpOptimizer {
    fn name(&self) -> &'static str { "SCCP" }
    
    fn run(&mut self, module: &mut Module) -> bool {
        if !self.enabled {
            return false;
        }
        
        let mut changed = false;
        for function in module.functions_mut() {
            changed |= self.run_on_function(function);
        }
        changed
    }
}
```

**Relationships**:
- **Implements**: `Phase` trait (compiler pipeline integration)
- **Creates**: `SccpAnalyzer` (per function)
- **Creates**: `SccpTransformer` (per function)
- **Operates On**: `Module` (containing multiple functions)

---

## Supporting Types from Existing IR

### Value

**Source**: `src/ir/value/mod.rs`

**Purpose**: Unique identifier for SSA values in the IR.

**Properties**:
- Implements `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`
- Used as key in HashMap for O(1) lattice lookups

**Relationships**:
- **Identified By**: `LatticeValue` (via HashMap key)
- **Queued In**: `Worklist` (SSA worklist element type)

---

### IrLiteralValue

**Source**: `src/ir/value/literal.rs`

**Purpose**: Represents concrete compile-time constant values of primitive types.

**Definition**:
```rust
pub enum IrLiteralValue {
    I8(i8), I16(i16), I32(i32), I64(i64),
    U8(u8), U16(u16), U32(u32), U64(u64),
    F32(f32), F64(f64),
    Bool(bool),
    Char(char),
}
```

**Properties**:
- Implements `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` (with custom f32/f64 handling)
- Convertible to `IrType` via `From` trait
- Display formatting for IR printing

**Relationships**:
- **Contained By**: `LatticeValue::Constant`
- **Produced By**: `constant_folder` module
- **Used By**: IR transformation (replacement values)

---

### NodeIndex

**Source**: `petgraph` crate

**Purpose**: Unique identifier for basic blocks in the control flow graph.

**Properties**:
- Lightweight (u32 internally)
- Implements `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`
- Stable across graph modifications (if using StableGraph)

**Relationships**:
- **Identifies**: Basic blocks in CFG
- **Stored In**: `executable_blocks` (HashSet element)
- **Queued In**: `Worklist` (CFG edge element)

---

### InstructionKind

**Source**: `src/ir/instruction.rs`

**Purpose**: Discriminates different types of IR instructions for analysis.

**Relevant Variants**:
```rust
pub enum InstructionKind {
    Binary { op: IrBinaryOp, left: Value, right: Value, ty: IrType },
    Unary { op: IrUnaryOp, operand: Value, ty: IrType },
    Phi { ty: IrType, incoming: Vec<(Value, String)> },
    Call { func: Value, args: Vec<Value>, ty: IrType },
    Load { src: Value, ty: IrType },
    // ... others
}
```

**Analysis Behavior**:
- **Binary/Unary**: Constant folding eligible if operands are constant
- **Phi**: Special analysis considering only executable predecessors
- **Call**: Always produces Bottom (no interprocedural analysis)
- **Load**: Always produces Bottom (no alias analysis)

**Relationships**:
- **Analyzed By**: `SccpAnalyzer::evaluate_instruction()`
- **Folded By**: `constant_folder` module (Binary/Unary only)

---

## Entity Relationships Diagram

```
SccpOptimizer (Phase impl)
    │
    ├── creates ──→ SccpAnalyzer (per function)
    │                   │
    │                   ├── owns ──→ Worklist
    │                   │               ├── queues ──→ Value (SSA)
    │                   │               └── queues ──→ (NodeIndex, NodeIndex) (CFG edges)
    │                   │
    │                   ├── owns ──→ HashMap<Value, LatticeValue>
    │                   │                               │
    │                   │                               └── contains ──→ IrLiteralValue
    │                   │
    │                   ├── owns ──→ HashSet<NodeIndex> (executable blocks)
    │                   │
    │                   └── produces ──→ AnalysisResult
    │                                       │
    │                                       ├── lattice_values
    │                                       ├── executable_blocks
    │                                       └── convergence stats
    │
    └── creates ──→ SccpTransformer (per function)
                        │
                        ├── consumes ──→ AnalysisResult
                        ├── mutates ──→ Function (IR)
                        └── produces ──→ TransformStats
```

## Data Flow

### Analysis Phase

```
Input: Function (IR)
  ↓
Initialize:
  - lattice_values[v] = Top ∀ SSA values v
  - executable_blocks = {entry_block}
  - worklist.cfg = {entry_block outgoing edges}
  ↓
Iterate:
  while !worklist.is_empty() && iterations < max {
    if cfg_edge = worklist.pop_cfg() {
      mark destination block executable
      process phi nodes
      add block's instructions to SSA worklist
    }
    if ssa_value = worklist.pop_ssa() {
      re-evaluate uses of ssa_value
      if lattice changed → add affected values to worklist
    }
  }
  ↓
Output: AnalysisResult {
  lattice_values: HashMap<Value, LatticeValue>,
  executable_blocks: HashSet<NodeIndex>,
  iterations: usize,
  converged: bool
}
```

### Transformation Phase

```
Input: Function (IR, mutable) + AnalysisResult
  ↓
For each AnalysisResult.lattice_values entry:
  if value is Constant(c) {
    replace all uses with literal c
    mark defining instruction dead
    stats.constants_propagated++
  }
  ↓
For each block's terminator:
  if ConditionalBranch with constant condition {
    replace with Branch to taken target
    stats.branches_simplified++
  }
  ↓
For each phi node:
  remove incoming from non-executable predecessors
  if phi is now constant → replace and mark dead
  stats.phi_nodes_cleaned++
  ↓
Output: bool (true if any changes made)
Side Effect: Mutated Function + TransformStats
```

## Memory Layout and Size Estimates

For a function with:
- V = 1000 SSA values
- B = 100 basic blocks  
- E = 150 CFG edges

**SccpAnalyzer**:
- `lattice_values`: 1000 × (8 bytes Value + 24 bytes LatticeValue) = 32 KB
- `executable_blocks`: 100 × 4 bytes NodeIndex = 400 bytes
- `worklist`: (1000 + 150) × 24 bytes = 27.6 KB
- **Total**: ~60 KB

**AnalysisResult**:
- `lattice_values`: 32 KB (moved from analyzer)
- `executable_blocks`: 400 bytes (moved from analyzer)
- `iterations`, `converged`: 16 bytes
- **Total**: ~32.4 KB

**Scalability**: Linear growth with function size (O(V + E) space complexity)

## Validation Rules Summary

### LatticeValue
- ✅ Never move upward in lattice (Top → Constant → Bottom only)
- ✅ IrLiteralValue must be valid for its type
- ✅ Meet operation must satisfy commutative, associative, idempotent properties

### SccpAnalyzer
- ✅ All SSA values in function must have lattice value entry
- ✅ Entry block must be in executable_blocks (if valid IR)
- ✅ iteration_count <= max_iterations at all times

### Worklist
- ✅ No duplicates within each worklist (enforced by seen sets)
- ✅ All worklist elements reference valid IR entities

### AnalysisResult
- ✅ If converged == true, no lattice values should remain at Top
- ✅ If converged == false, all lattice values must be Bottom or Constant

### SccpTransformer
- ✅ All Value references in AnalysisResult must exist in Function
- ✅ All NodeIndex references must be valid block indices
- ✅ Transformation must preserve IR validity (SSA form, CFG structure)

## Conclusion

This comprehensive data model provides a complete, precise, and detailed specification of all entities, relationships, invariants, and validation rules for the SCCP optimization phase. The design ensures:

1. **Correctness**: Lattice-theoretic foundations guarantee sound analysis
2. **Efficiency**: Sparse representation and worklist algorithm achieve O(V+E) complexity
3. **Maintainability**: Clear separation of concerns across entities
4. **Extensibility**: Modular design allows independent enhancement of components

All data structures are designed for optimal performance while maintaining strict correctness invariants required for compiler optimization safety.
