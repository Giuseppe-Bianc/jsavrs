# SCCP Optimizer Data Model

**Feature**: Sparse Conditional Constant Propagation Optimizer  
**Branch**: 016-sccp-optimizer  
**Date**: 2025-11-17

## Overview

This document defines the complete data model for the SCCP optimizer implementation. All data structures maintain strict type safety, enforce SSA and CFG invariants, and support efficient O(edges) time complexity through careful design of indexing and lookup mechanisms.

## 1. Lattice Value Representation

### 1.1 LatticeValue Enum

```rust
/// Represents the constant state of an SSA value in the three-level lattice
/// used by the SCCP algorithm.
///
/// The lattice forms a partial order: Top ⊑ Constant ⊑ Bottom
/// Values can only descend (Top → Constant → Bottom), ensuring termination.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    /// Optimistically unknown - value has not yet been determined
    /// Indicates the analysis has not yet reached this definition
    /// or the value depends on unanalyzed control flow paths
    Top,
    
    /// Proven constant value - the value is guaranteed to equal this literal
    /// on all executable control-flow paths reaching this point
    Constant(IrLiteralValue),
    
    /// Pessimistically varying - the value may be different on different paths
    /// or is unknown at compile time (e.g., function parameters, memory loads)
    Bottom,
}
```

**Field Details**:

- **Top**: Represents the optimistic initial state before analysis. A value at Top has not yet been determined and may still be proven constant.

- **Constant(IrLiteralValue)**: Represents a proven compile-time constant. The wrapped `IrLiteralValue` is the exact value this SSA value will have at runtime on all executable paths. Supported literal types:
  - `I8(i8)`, `I16(i16)`, `I32(i32)`, `I64(i64)` - signed integers
  - `U8(u8)`, `U16(u16)`, `U32(u32)`, `U64(u64)` - unsigned integers
  - `F32(f32)`, `F64(f64)` - floating-point numbers (NaN/Infinity marked as Bottom conservatively)
  - `Bool(bool)` - boolean values (true/false)
  - `Char(char)` - Unicode characters (valid codepoints only)
  - `String(String)` - string literals (always Bottom in initial implementation)

- **Bottom**: Represents the conservative fallback when a value cannot be proven constant. Reasons include:
  - Multiple different constant values reach this point (e.g., phi node with different incoming constants)
  - Value depends on runtime input (function parameters, globals, memory loads)
  - Arithmetic operation produces undefined behavior (overflow, division by zero)
  - Value is not yet known but optimistic analysis is disabled

**Validation Rules**:
- Lattice values must never move upward (Bottom → Constant → Top is invalid)
- Each value can transition at most twice: Top → Constant → Bottom (or Top → Bottom directly)
- After fixed-point, Top values should only exist in unreachable code regions

**Relationships**:
- Each SSA `Value` (temporary, local, parameter) maps to exactly one `LatticeValue` in the analysis state
- Lattice values are stored in `HashMap<Value, LatticeValue>` for O(1) lookup
- Lattice state transitions trigger SSA worklist updates for all uses of the changed value

### 1.2 Lattice Operations

```rust
impl LatticeValue {
    /// Computes the meet (greatest lower bound) of two lattice values.
    /// 
    /// Meet operation properties:
    /// - Commutative: meet(a, b) = meet(b, a)
    /// - Associative: meet(meet(a, b), c) = meet(a, meet(b, c))
    /// - Idempotent: meet(a, a) = a
    /// - Top is identity: meet(Top, x) = x
    /// - Bottom is absorbing: meet(Bottom, x) = Bottom
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(LatticeValue::Top.meet(&LatticeValue::Constant(5)), LatticeValue::Constant(5));
    /// assert_eq!(LatticeValue::Constant(5).meet(&LatticeValue::Constant(5)), LatticeValue::Constant(5));
    /// assert_eq!(LatticeValue::Constant(5).meet(&LatticeValue::Constant(6)), LatticeValue::Bottom);
    /// assert_eq!(LatticeValue::Bottom.meet(&LatticeValue::Constant(5)), LatticeValue::Bottom);
    /// ```
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // Top is top element: meet with anything yields that thing
            (LatticeValue::Top, x) | (x, LatticeValue::Top) => x.clone(),
            
            // Bottom is bottom element: meet with anything yields Bottom
            (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,
            
            // Same constants: meet yields that constant
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) if c1 == c2 => {
                LatticeValue::Constant(c1.clone())
            }
            
            // Different constants: meet yields Bottom (varying)
            (LatticeValue::Constant(_), LatticeValue::Constant(_)) => LatticeValue::Bottom,
        }
    }
    
    /// Returns true if this lattice value is more precise than the other
    /// (i.e., this ⊑ other in the lattice partial order).
    ///
    /// Partial order:
    /// - Top ⊑ anything
    /// - Constant(c) ⊑ Bottom
    /// - Constant(c1) and Constant(c2) are incomparable if c1 ≠ c2
    pub fn is_more_precise_than(&self, other: &Self) -> bool {
        match (self, other) {
            (LatticeValue::Top, _) => true,
            (_, LatticeValue::Bottom) => true,
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) => c1 == c2,
            _ => false,
        }
    }
    
    /// Returns true if this lattice value represents a proven constant
    pub fn is_constant(&self) -> bool {
        matches!(self, LatticeValue::Constant(_))
    }
    
    /// Extracts the constant value if this is Constant, otherwise None
    pub fn as_constant(&self) -> Option<&IrLiteralValue> {
        match self {
            LatticeValue::Constant(lit) => Some(lit),
            _ => None,
        }
    }
}
```

**State Transition Graph**:
```text
┌──────┐
│ Top  │  Initial state (optimistic unknown)
└──┬───┘
   │ VisitInstruction determines value
   ├─────────┐
   │         ↓
   │    ┌────────────┐
   │    │ Constant(c)│  Proven constant
   │    └─────┬──────┘
   │          │ Different constant reaches same value
   ↓          ↓
┌──────────┐
│  Bottom  │  Varying or unknown (pessimistic)
└──────────┘
```

## 2. Worklist Data Structures

### 2.1 SSAWorkList (Definition → Use Edges)

```rust
/// Manages the queue of SSA edges that need reprocessing when a value's
/// lattice state changes.
///
/// Each edge represents (definition_value → use_instruction), indicating
/// that use_instruction depends on definition_value and should be
/// re-evaluated when definition_value's lattice state becomes more precise.
///
/// Duplicate prevention ensures each edge is processed at most once per
/// lattice state change, achieving O(edges) complexity.
pub struct SSAWorkList {
    /// FIFO queue of edges to process
    queue: VecDeque<(Value, InstructionId)>,
    
    /// Set of edges already enqueued in this lattice state
    /// Prevents redundant work when the same edge is triggered multiple times
    seen: HashSet<(Value, InstructionId)>,
}

impl SSAWorkList {
    /// Creates a new empty SSA worklist
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            seen: HashSet::new(),
        }
    }
    
    /// Enqueues an SSA edge if not already in the queue.
    ///
    /// Returns true if the edge was newly enqueued, false if it was a duplicate.
    ///
    /// # Arguments
    ///
    /// * `def_value` - The SSA value whose lattice state changed
    /// * `use_instruction` - The instruction that uses def_value and needs re-evaluation
    ///
    /// # Complexity
    ///
    /// O(1) average case (HashSet insert + VecDeque push_back)
    pub fn enqueue(&mut self, def_value: Value, use_instruction: InstructionId) -> bool {
        let edge = (def_value, use_instruction);
        if self.seen.insert(edge) {
            self.queue.push_back(edge);
            true
        } else {
            false
        }
    }
    
    /// Dequeues the next SSA edge to process (FIFO order).
    ///
    /// Returns None if the worklist is empty.
    ///
    /// # Complexity
    ///
    /// O(1) (VecDeque pop_front)
    pub fn dequeue(&mut self) -> Option<(Value, InstructionId)> {
        self.queue.pop_front()
    }
    
    /// Returns true if the worklist is empty (no more edges to process)
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Clears the worklist and resets duplicate tracking
    /// (used when starting analysis of a new function)
    pub fn clear(&mut self) {
        self.queue.clear();
        self.seen.clear();
    }
}
```

**Field Details**:

- **queue: VecDeque<(Value, InstructionId)>**: FIFO queue of SSA edges awaiting processing
  - First element: `Value` - the definition whose lattice state changed
  - Second element: `InstructionId` - the use instruction to re-evaluate
  - VecDeque provides O(1) push_back and pop_front for efficient FIFO

- **seen: HashSet<(Value, InstructionId)>**: Duplicate detection set
  - Prevents enqueuing the same edge multiple times in one analysis iteration
  - Critical for O(edges) complexity: each edge processed at most 2 times (Top→Constant, Constant→Bottom)

**Validation Rules**:
- Each SSA edge can be enqueued at most twice during the entire analysis (once for each lattice state transition)
- queue.len() ≤ |SSA edges| at any point in time
- seen.len() ≤ |SSA edges| after fixed-point convergence

**Relationships**:
- SSA edges are derived from the IR's def-use chains (each use instruction depends on its operand definitions)
- When a value's lattice state changes (via `update_lattice_value`), all its use edges are enqueued
- Processing an edge calls `visit_instruction` on the use instruction

### 2.2 FlowWorkList (Predecessor → Successor CFG Edges)

```rust
/// Manages the queue of control-flow edges whose destination blocks
/// need visiting.
///
/// Each edge represents (predecessor_block → successor_block), indicating
/// that successor_block has become newly reachable and all its instructions
/// (including phi nodes) need evaluation.
///
/// Duplicate prevention ensures each CFG edge is processed at most once,
/// achieving O(edges) complexity.
pub struct FlowWorkList {
    /// FIFO queue of CFG edges to process
    queue: VecDeque<(BlockId, BlockId)>,
    
    /// Set of edges already enqueued
    /// Prevents redundant work when the same block is reached via multiple paths
    seen: HashSet<(BlockId, BlockId)>,
}

impl FlowWorkList {
    /// Creates a new empty flow worklist
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            seen: HashSet::new(),
        }
    }
    
    /// Enqueues a CFG edge if not already in the queue.
    ///
    /// Returns true if the edge was newly enqueued, false if it was a duplicate.
    ///
    /// # Arguments
    ///
    /// * `pred_block` - The predecessor block (source of control flow)
    /// * `succ_block` - The successor block (destination of control flow)
    ///
    /// # Complexity
    ///
    /// O(1) average case (HashSet insert + VecDeque push_back)
    pub fn enqueue(&mut self, pred_block: BlockId, succ_block: BlockId) -> bool {
        let edge = (pred_block, succ_block);
        if self.seen.insert(edge) {
            self.queue.push_back(edge);
            true
        } else {
            false
        }
    }
    
    /// Dequeues the next CFG edge to process (FIFO order).
    ///
    /// Returns None if the worklist is empty.
    ///
    /// # Complexity
    ///
    /// O(1) (VecDeque pop_front)
    pub fn dequeue(&mut self) -> Option<(BlockId, BlockId)> {
        self.queue.pop_front()
    }
    
    /// Returns true if the worklist is empty (no more edges to process)
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Clears the worklist and resets duplicate tracking
    pub fn clear(&mut self) {
        self.queue.clear();
        self.seen.clear();
    }
}
```

**Field Details**:

- **queue: VecDeque<(BlockId, BlockId)>**: FIFO queue of CFG edges awaiting processing
  - First element: `BlockId` - the predecessor block
  - Second element: `BlockId` - the successor block to visit
  - VecDeque provides O(1) push_back and pop_front for efficient FIFO

- **seen: HashSet<(BlockId, BlockId)>**: Duplicate detection set
  - Prevents processing the same CFG edge multiple times
  - Critical for O(edges) complexity: each edge processed exactly once

**Validation Rules**:
- Each CFG edge can be enqueued exactly once during the entire analysis
- queue.len() ≤ |CFG edges| at any point in time
- seen.len() = |executable CFG edges| after fixed-point convergence

**Relationships**:
- CFG edges are defined by the IR's control-flow graph (block terminators specify successors)
- When a conditional branch's condition becomes Constant(true/false), only one outgoing edge is enqueued
- When an unconditional branch is encountered, its target edge is always enqueued
- Processing an edge calls `visit_block` on the successor block

## 3. Executable Edge Tracking

### 3.1 ExecutableEdges Structure

```rust
/// Tracks which control-flow edges and basic blocks are proven to be
/// reachable during SCCP analysis.
///
/// Initially, only the entry block and its outgoing edges are marked
/// executable. As the analysis progresses and conditional branches with
/// constant conditions are encountered, only the taken branches are marked
/// executable, enabling dead code elimination.
pub struct ExecutableEdges {
    /// Set of CFG edges proven to be reachable
    /// Each edge is represented as (predecessor_block_id, successor_block_id)
    edges: HashSet<(BlockId, BlockId)>,
    
    /// Set of basic blocks proven to be reachable
    /// A block becomes executable when any of its incoming edges is marked executable
    blocks: HashSet<BlockId>,
}

impl ExecutableEdges {
    /// Creates a new empty executable edges set (all code initially unreachable)
    pub fn new() -> Self {
        Self {
            edges: HashSet::new(),
            blocks: HashSet::new(),
        }
    }
    
    /// Marks a CFG edge as executable (reachable).
    ///
    /// If this is the first incoming edge to succ_block, the block is also
    /// marked as executable.
    ///
    /// Returns true if the edge was newly marked executable, false if it was
    /// already marked.
    ///
    /// # Arguments
    ///
    /// * `pred_block` - The predecessor block ID
    /// * `succ_block` - The successor block ID
    ///
    /// # Complexity
    ///
    /// O(1) average case (two HashSet inserts)
    pub fn mark_edge_executable(&mut self, pred_block: BlockId, succ_block: BlockId) -> bool {
        let edge = (pred_block, succ_block);
        let newly_inserted = self.edges.insert(edge);
        
        if newly_inserted {
            // If this is the first incoming edge, mark the block as executable
            self.blocks.insert(succ_block);
        }
        
        newly_inserted
    }
    
    /// Returns true if the given basic block is executable (reachable)
    ///
    /// # Complexity
    ///
    /// O(1) average case (HashSet contains)
    pub fn is_block_executable(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }
    
    /// Returns true if the given CFG edge is executable (reachable)
    ///
    /// # Complexity
    ///
    /// O(1) average case (HashSet contains)
    pub fn is_edge_executable(&self, pred_block: BlockId, succ_block: BlockId) -> bool {
        self.edges.contains(&(pred_block, succ_block))
    }
    
    /// Returns an iterator over all executable basic blocks
    pub fn executable_blocks(&self) -> impl Iterator<Item = &BlockId> {
        self.blocks.iter()
    }
    
    /// Returns the number of executable blocks
    pub fn num_executable_blocks(&self) -> usize {
        self.blocks.len()
    }
    
    /// Returns the number of executable edges
    pub fn num_executable_edges(&self) -> usize {
        self.edges.len()
    }
}
```

**Field Details**:

- **edges: HashSet<(BlockId, BlockId)>**: Set of proven reachable CFG edges
  - Each element is a tuple (predecessor_id, successor_id)
  - Initially empty except for entry block's outgoing edges
  - Grown as conditional branches with constant conditions determine single paths
  - Used for phi node evaluation (only executable predecessor values contribute)

- **blocks: HashSet<BlockId>**: Set of proven reachable basic blocks
  - Each element is a BlockId of a reachable block
  - Initially contains only the entry block
  - A block is marked executable when its first incoming edge becomes executable
  - Used during IR rewrite to remove unreachable blocks

**Validation Rules**:
- Entry block must always be in `blocks` after initialization
- If edge (A, B) is in `edges`, then both A and B must be in `blocks`
- `blocks.len()` ≤ total number of blocks in function
- `edges.len()` ≤ total number of CFG edges in function

**Relationships**:
- CFG edges are derived from block terminators (Branch, ConditionalBranch, Switch, Return, Unreachable)
- When a ConditionalBranch's condition is Constant(true), only the true_target edge is marked executable
- When a ConditionalBranch's condition is Constant(false), only the false_target edge is marked executable
- When a ConditionalBranch's condition is Top or Bottom, both edges are marked executable conservatively
- Phi nodes only consider incoming values from executable predecessor edges

## 4. Optimization Statistics

### 4.1 OptimizationStatistics Structure

```rust
/// Collects metrics about the SCCP optimization pass for reporting and analysis.
///
/// These statistics help evaluate the effectiveness of the optimizer and
/// identify opportunities for further improvement.
#[derive(Debug, Clone, Default)]
pub struct OptimizationStatistics {
    /// Number of SSA values proven to be constant
    pub constants_found: usize,
    
    /// Number of conditional branches eliminated (converted to unconditional)
    pub branches_eliminated: usize,
    
    /// Number of basic blocks removed as unreachable
    pub blocks_removed: usize,
    
    /// Number of instructions replaced with constant values
    pub instructions_replaced: usize,
    
    /// Number of phi nodes simplified (single incoming value or all same constant)
    pub phi_nodes_simplified: usize,
    
    /// Number of iterations required to reach fixed-point convergence
    pub iterations_to_convergence: usize,
    
    /// Total number of SSA values analyzed
    pub total_values_analyzed: usize,
    
    /// Total number of basic blocks analyzed
    pub total_blocks_analyzed: usize,
}

impl OptimizationStatistics {
    /// Creates a new statistics object with all counters at zero
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Merges statistics from another optimization run (for multi-function optimization)
    pub fn merge(&mut self, other: &Self) {
        self.constants_found += other.constants_found;
        self.branches_eliminated += other.branches_eliminated;
        self.blocks_removed += other.blocks_removed;
        self.instructions_replaced += other.instructions_replaced;
        self.phi_nodes_simplified += other.phi_nodes_simplified;
        self.iterations_to_convergence = self.iterations_to_convergence.max(other.iterations_to_convergence);
        self.total_values_analyzed += other.total_values_analyzed;
        self.total_blocks_analyzed += other.total_blocks_analyzed;
    }
    
    /// Returns the percentage of values proven constant
    pub fn constant_percentage(&self) -> f64 {
        if self.total_values_analyzed == 0 {
            0.0
        } else {
            (self.constants_found as f64 / self.total_values_analyzed as f64) * 100.0
        }
    }
    
    /// Returns the percentage of blocks removed
    pub fn block_removal_percentage(&self) -> f64 {
        if self.total_blocks_analyzed == 0 {
            0.0
        } else {
            (self.blocks_removed as f64 / self.total_blocks_analyzed as f64) * 100.0
        }
    }
}

impl std::fmt::Display for OptimizationStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SCCP Optimization Statistics:")?;
        writeln!(f, "  Constants found: {} ({:.1}%)", self.constants_found, self.constant_percentage())?;
        writeln!(f, "  Branches eliminated: {}", self.branches_eliminated)?;
        writeln!(f, "  Blocks removed: {} ({:.1}%)", self.blocks_removed, self.block_removal_percentage())?;
        writeln!(f, "  Instructions replaced: {}", self.instructions_replaced)?;
        writeln!(f, "  Phi nodes simplified: {}", self.phi_nodes_simplified)?;
        writeln!(f, "  Iterations to convergence: {}", self.iterations_to_convergence)?;
        writeln!(f, "  Total values analyzed: {}", self.total_values_analyzed)?;
        writeln!(f, "  Total blocks analyzed: {}", self.total_blocks_analyzed)?;
        Ok(())
    }
}
```

**Field Details**:

- **constants_found**: Count of SSA values that reached `Constant` state (excluding parameters/globals)
- **branches_eliminated**: Count of `ConditionalBranch` terminators converted to unconditional `Branch`
- **blocks_removed**: Count of basic blocks removed as unreachable
- **instructions_replaced**: Count of instructions whose results were replaced with constant literals
- **phi_nodes_simplified**: Count of phi nodes simplified to direct assignments
- **iterations_to_convergence**: Number of worklist processing iterations before both queues emptied
- **total_values_analyzed**: Total SSA values in the function (baseline for percentages)
- **total_blocks_analyzed**: Total basic blocks in the function (baseline for percentages)

**Validation Rules**:
- `constants_found` ≤ `total_values_analyzed`
- `blocks_removed` ≤ `total_blocks_analyzed`
- `iterations_to_convergence` ≤ `max_iterations` configured value
- All counters ≥ 0

## 5. Main Analyzer Structure

### 5.1 SCCPAnalyzer

```rust
/// Main SCCP analyzer that orchestrates the complete optimization process.
///
/// The analyzer maintains the lattice state, worklists, and executable edge
/// tracking for a single function. It performs fixed-point iteration until
/// both worklists are empty, then rewrites the IR based on the analysis results.
pub struct SCCPAnalyzer<'a> {
    /// Reference to the function being optimized
    function: &'a Function,
    
    /// Current lattice state for all SSA values
    /// Maps each Value to its LatticeValue (Top/Constant/Bottom)
    lattice: HashMap<Value, LatticeValue>,
    
    /// SSA worklist (definition → use edges)
    ssa_worklist: SSAWorkList,
    
    /// Control-flow worklist (predecessor → successor edges)
    flow_worklist: FlowWorkList,
    
    /// Executable edge and block tracking
    executable: ExecutableEdges,
    
    /// Maximum iterations before giving up and marking everything Bottom
    max_iterations: usize,
    
    /// Statistics collected during optimization
    stats: OptimizationStatistics,
    
    /// Verbose logging enabled
    verbose: bool,
}

impl<'a> SCCPAnalyzer<'a> {
    /// Creates a new SCCP analyzer for the given function.
    ///
    /// Initializes all SSA values to Top, except:
    /// - Function parameters → Bottom (unknown at compile time)
    /// - Global variables → Bottom (may be modified externally)
    ///
    /// Marks the entry block as executable and enqueues its outgoing edges.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    /// * `max_iterations` - Maximum worklist iterations before timeout
    /// * `verbose` - Whether to log detailed analysis steps
    pub fn new(function: &'a Function, max_iterations: usize, verbose: bool) -> Self {
        let mut lattice = HashMap::new();
        
        // Initialize all values to Top, except parameters and globals
        for value in function.all_values() {
            let initial_state = match value {
                Value::Parameter(_) | Value::Global(_) => LatticeValue::Bottom,
                _ => LatticeValue::Top,
            };
            lattice.insert(value, initial_state);
        }
        
        let mut executable = ExecutableEdges::new();
        let mut flow_worklist = FlowWorkList::new();
        
        // Mark entry block as executable
        let entry_block = function.entry_block_id();
        executable.blocks.insert(entry_block);
        
        // Enqueue entry block's outgoing edges
        for successor in function.cfg.successors(entry_block) {
            executable.mark_edge_executable(entry_block, successor);
            flow_worklist.enqueue(entry_block, successor);
        }
        
        Self {
            function,
            lattice,
            ssa_worklist: SSAWorkList::new(),
            flow_worklist,
            executable,
            max_iterations,
            stats: OptimizationStatistics::new(),
            verbose,
        }
    }
    
    /// Runs the fixed-point analysis until both worklists are empty.
    ///
    /// Returns the collected statistics or an error if validation fails
    /// or maximum iterations is exceeded.
    pub fn analyze(&mut self) -> Result<OptimizationStatistics, SCCPError> {
        // Implementation in evaluator.rs
        unimplemented!()
    }
    
    /// Rewrites the IR based on the analysis results.
    ///
    /// Performs the following transformations:
    /// - Replace instructions with Constant lattice values with literal constants
    /// - Convert conditional branches with constant conditions to unconditional branches
    /// - Remove unreachable basic blocks
    /// - Remove incoming phi edges from unreachable predecessors
    /// - Simplify phi nodes with single incoming value
    ///
    /// # Arguments
    ///
    /// * `function` - The function to rewrite (mutable)
    pub fn rewrite(&self, function: &mut Function) -> Result<(), SCCPError> {
        // Implementation in rewriter.rs
        unimplemented!()
    }
}
```

**Field Details**:

- **function: &'a Function**: Immutable reference to the function being analyzed
  - Used for reading IR structure (blocks, instructions, CFG)
  - Lifetime 'a ensures analyzer doesn't outlive the function
  
- **lattice: HashMap<Value, LatticeValue>**: Current constant state for all SSA values
  - Key: `Value` (SSA temporary, local, parameter, global)
  - Value: `LatticeValue` (Top/Constant/Bottom)
  - O(1) lookup and update
  - Initially all Top except parameters/globals (Bottom)
  
- **ssa_worklist: SSAWorkList**: Queue of SSA edges to reprocess
  - Driven by lattice state changes
  - Each value change enqueues all use edges
  
- **flow_worklist: FlowWorkList**: Queue of CFG edges to process
  - Driven by newly executable control-flow paths
  - Terminator evaluation enqueues successor edges
  
- **executable: ExecutableEdges**: Tracks reachable CFG edges and blocks
  - Used for phi node evaluation (only executable predecessors)
  - Used for dead block elimination (non-executable blocks removed)
  
- **max_iterations: usize**: Safety limit to prevent infinite loops
  - Default: 100 iterations
  - Configurable via optimizer configuration
  - Exceeded iterations log warning and mark remaining Top values as Bottom
  
- **stats: OptimizationStatistics**: Collected metrics
  - Updated during analysis and rewrite phases
  - Returned to caller for reporting
  
- **verbose: bool**: Enables detailed logging
  - Logs each worklist operation
  - Logs lattice state changes
  - Logs terminator evaluations
  - Useful for debugging and understanding optimizer behavior

**Validation Rules**:
- `lattice` must contain entries for all SSA values in the function
- `executable.blocks` must contain at least the entry block
- After fixed-point, `ssa_worklist` and `flow_worklist` must be empty
- No lattice value should move upward (Bottom → Constant → Top is invalid)

## 6. Configuration and Integration

### 6.1 ConstantFoldingOptimizer

```rust
/// Configuration and entry point for the SCCP constant folding optimization pass.
///
/// Implements the Phase trait to integrate with the jsavrs compiler's
/// optimization pipeline.
pub struct ConstantFoldingOptimizer {
    /// Enable verbose logging of optimization decisions
    pub verbose: bool,
    
    /// Maximum worklist iterations before timeout (default: 100)
    pub max_iterations: usize,
    
    /// Enable SCCP analysis (if false, pass does nothing)
    pub sccp_enabled: bool,
}

impl Default for ConstantFoldingOptimizer {
    fn default() -> Self {
        Self {
            verbose: false,
            max_iterations: 100,
            sccp_enabled: true,
        }
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &str {
        "Constant Folding (SCCP)"
    }
    
    fn run(&mut self, module: &mut Module) -> Result<(), String> {
        if !self.sccp_enabled {
            return Ok(());
        }
        
        let mut total_stats = OptimizationStatistics::default();
        
        for function in module.functions.iter_mut() {
            let stats = self.transform_function(function)
                .map_err(|e| format!("SCCP failed on function {}: {}", function.name, e))?;
            total_stats.merge(&stats);
        }
        
        if self.verbose {
            eprintln!("{}", total_stats);
        }
        
        Ok(())
    }
}
```

**Field Details**:

- **verbose: bool**: Controls detailed logging output
  - Default: false (only errors and warnings)
  - When true: logs all lattice changes, worklist operations, transformations
  
- **max_iterations: usize**: Safety limit for fixed-point iteration
  - Default: 100
  - Prevents infinite loops in buggy implementations
  - Exceeded limit logs warning and completes with partial results
  
- **sccp_enabled: bool**: Master enable/disable switch
  - Default: true
  - Allows disabling SCCP without removing from pipeline
  - Useful for A/B testing and debugging

## 7. Data Flow Diagram

```text
┌─────────────────────────────────────────────────────────────────┐
│                    SCCP Analyzer State                          │
├─────────────────────────────────────────────────────────────────┤
│  lattice: HashMap<Value, LatticeValue>                          │
│  ┌────────┬──────────┐                                          │
│  │ Value  │ Lattice  │                                          │
│  ├────────┼──────────┤                                          │
│  │ Temp(1)│   Top    │  Initially all Top                       │
│  │ Temp(2)│   Top    │  (except params/globals → Bottom)        │
│  │ Param(0)│ Bottom  │                                          │
│  │ ...    │   ...    │                                          │
│  └────────┴──────────┘                                          │
│                                                                 │
│  ssa_worklist: VecDeque<(Value, InstructionId)>                 │
│  ┌──────────────────────────────┐                               │
│  │ (Temp(1), Instr(5))          │  Edges to reprocess           │
│  │ (Temp(2), Instr(6))          │  when lattice changes         │
│  │ ...                          │                               │
│  └──────────────────────────────┘                               │
│                                                                 │
│  flow_worklist: VecDeque<(BlockId, BlockId)>                    │
│  ┌──────────────────────────────┐                               │
│  │ (Block(0), Block(1))         │  CFG edges to process         │
│  │ (Block(1), Block(2))         │  when newly executable        │
│  │ ...                          │                               │
│  └──────────────────────────────┘                               │
│                                                                 │
│  executable: ExecutableEdges                                    │
│  ┌────────────────┬───────────────┐                             │
│  │ edges:         │ blocks:       │                             │
│  │ (Block(0), Block(1))│ Block(0) │                             │
│  │ (Block(1), Block(2))│ Block(1) │                             │
│  │ ...            │ ...           │                             │
│  └────────────────┴───────────────┘                             │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ↓
              ┌────────────────────────┐
              │  Fixed-Point Iteration │
              └────────────────────────┘
                           │
        ┌──────────────────┴──────────────────┐
        ↓                                     ↓
┌───────────────────┐              ┌──────────────────┐
│ Process SSA Queue │              │ Process Flow Queue│
│                   │              │                   │
│ Pop (def, use)    │              │ Pop (pred, succ)  │
│ VisitInstruction  │              │ VisitBlock        │
│ Update lattice    │              │ Mark executable   │
│ Enqueue uses      │              │ Enqueue successors│
└───────────────────┘              └──────────────────┘
        │                                     │
        └──────────────────┬──────────────────┘
                           ↓
                ┌────────────────────┐
                │ Both queues empty? │
                └────────┬───────────┘
                         │ Yes
                         ↓
                  ┌──────────────┐
                  │   Rewrite IR │
                  │              │
                  │ - Replace    │
                  │   constants  │
                  │ - Eliminate  │
                  │   branches   │
                  │ - Remove     │
                  │   unreachable│
                  └──────────────┘
```

## 8. Entity Relationship Summary

**LatticeValue** ← 1:1 → **Value** (each SSA value has one lattice state)

**SSAWorkList** contains → **many (Value, InstructionId)** edges

**FlowWorkList** contains → **many (BlockId, BlockId)** edges

**ExecutableEdges** tracks → **many BlockId** (blocks) and **many (BlockId, BlockId)** (edges)

**SCCPAnalyzer** owns → **one of each**: lattice, ssa_worklist, flow_worklist, executable, stats

**ConstantFoldingOptimizer** creates → **one SCCPAnalyzer per Function**

**OptimizationStatistics** aggregated from → **multiple SCCPAnalyzer runs** (one per function)

---

**End of Data Model Document**

This data model provides the complete structural foundation for the SCCP optimizer implementation, with all types, fields, relationships, and validation rules precisely specified.
