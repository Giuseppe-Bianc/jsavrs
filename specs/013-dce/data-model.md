# Dead Code Elimination: Data Model Specification

**Feature**: Dead Code Elimination (DCE) Optimization Phase  
**Date**: 2025-11-02  
**Status**: Design Complete

## Overview

This document provides comprehensive data model specifications for the Dead Code Elimination optimization phase. All data structures are designed following Rust best practices, leveraging the type system for safety and using efficient standard library collections.

## Core Data Structures

### 1. DeadCodeElimination - Main Optimizer Struct

**Purpose**: Primary struct implementing the `Phase` trait, orchestrating all DCE analysis and transformation operations.

**Definition**:
```rust
/// Dead Code Elimination optimization phase.
///
/// Removes unreachable basic blocks and unused instructions from IR functions
/// while preserving all observable program behavior. Uses reachability analysis,
/// liveness analysis, and escape analysis to safely identify removable code.
///
/// # Algorithm
///
/// The optimization runs in a fixed-point loop:
/// 1. Reachability analysis: Mark blocks reachable from entry
/// 2. Block removal: Remove unreachable blocks, update CFG edges
/// 3. Liveness analysis: Compute live values via backward dataflow
/// 4. Escape analysis: Determine which allocations escape their scope
/// 5. Instruction removal: Remove dead instructions based on liveness/effects
/// 6. Repeat until no changes (fixed-point reached)
///
/// # Configuration
///
/// - `max_iterations`: Maximum fixed-point iterations (default 10)
/// - `enable_statistics`: Whether to collect and report optimization metrics
/// - `verbose_warnings`: Whether to emit detailed conservative decision warnings
///
/// # Example
///
/// ```rust
/// use jsavrs::ir::{Module, Phase};
/// use jsavrs::ir::optimizer::DeadCodeElimination;
///
/// let mut module = Module::new("test", None);
/// // ... add functions ...
///
/// let mut dce = DeadCodeElimination::default();
/// dce.run(&mut module);
/// ```
#[derive(Debug, Clone)]
pub struct DeadCodeElimination {
    /// Maximum number of fixed-point iterations before stopping.
    /// Prevents infinite loops in case of algorithm bugs.
    /// Default: 10
    pub max_iterations: usize,
    
    /// Whether to collect and report detailed optimization statistics.
    /// Default: true
    pub enable_statistics: bool,
    
    /// Whether to emit warnings for conservative decisions that prevent removal.
    /// Useful for debugging missed optimization opportunities.
    /// Default: false
    pub verbose_warnings: bool,
}
```

**Fields**:

| Field | Type | Purpose | Default | Constraints |
|-------|------|---------|---------|-------------|
| `max_iterations` | `usize` | Limit fixed-point iterations | 10 | Must be > 0 |
| `enable_statistics` | `bool` | Enable metrics collection | true | - |
| `verbose_warnings` | `bool` | Emit conservative warnings | false | - |

**Methods**:

```rust
impl DeadCodeElimination {
    /// Creates a new DCE optimizer with default settings.
    pub fn new() -> Self;
    
    /// Creates a new DCE optimizer with custom settings.
    ///
    /// # Arguments
    ///
    /// * `max_iterations` - Maximum fixed-point iterations (must be > 0)
    /// * `enable_statistics` - Whether to collect optimization metrics
    /// * `verbose_warnings` - Whether to emit conservative decision warnings
    ///
    /// # Panics
    ///
    /// Panics if `max_iterations` is 0.
    pub fn with_config(
        max_iterations: usize, 
        enable_statistics: bool,
        verbose_warnings: bool
    ) -> Self;
    
    /// Optimizes a single function with DCE.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to optimize (modified in-place)
    ///
    /// # Returns
    ///
    /// Statistics about the optimization (removals, iterations, warnings)
    ///
    /// # Errors
    ///
    /// Returns error if CFG verification fails after optimization.
    fn optimize_function(&mut self, function: &mut Function) 
        -> Result<OptimizationStats, String>;
}

impl Phase for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "Dead Code Elimination"
    }
    
    fn run(&mut self, module: &mut Module);
}

impl Default for DeadCodeElimination {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            enable_statistics: true,
            verbose_warnings: false,
        }
    }
}
```

**Relationships**:
- **Implements**: `Phase` trait (defined in `src/ir/optimizer/phase.rs`)
- **Uses**: `Module`, `Function`, `ControlFlowGraph` (existing IR structures)
- **Creates**: `OptimizationStats`, `ReachabilityAnalyzer`, `LivenessAnalyzer`, `EscapeAnalyzer`

**Validation Rules**:
- `max_iterations` must be at least 1 (enforced by panic in constructor)
- After optimization, must call `function.verify()` to ensure CFG integrity

**State Transitions**:
```
Uninitialized → Initialized (via new/default)
Initialized → Running (via run())
Running → Analyzing (reachability/liveness/escape)
Analyzing → Transforming (removing blocks/instructions)
Transforming → Running (next iteration) OR Complete (fixed-point)
```

---

### 2. OptimizationStats - Metrics Collection

**Purpose**: Tracks optimization results and performance metrics for reporting and debugging.

**Definition**:
```rust
/// Statistics collected during Dead Code Elimination optimization.
///
/// Provides metrics about the optimization's effectiveness including
/// the number of instructions and blocks removed, iterations required
/// to reach fixed-point, and any conservative decisions that prevented
/// more aggressive optimization.
///
/// # Fields
///
/// * `instructions_removed` - Total instructions eliminated
/// * `blocks_removed` - Total basic blocks eliminated
/// * `iterations` - Number of fixed-point iterations required
/// * `conservative_warnings` - Decisions that prevented removal
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OptimizationStats {
    /// Total number of instructions removed across all iterations.
    pub instructions_removed: usize,
    
    /// Total number of basic blocks removed across all iterations.
    pub blocks_removed: usize,
    
    /// Number of fixed-point iterations performed.
    /// 1 means no changes were made (optimization was no-op).
    /// Higher values indicate cascading dead code removal.
    pub iterations: usize,
    
    /// Warnings about conservative decisions that prevented removal.
    /// Empty if `verbose_warnings` is disabled on the optimizer.
    pub conservative_warnings: Vec<ConservativeWarning>,
}
```

**Methods**:
```rust
impl OptimizationStats {
    /// Creates empty statistics (no removals).
    pub fn new() -> Self;
    
    /// Checks if any code was removed.
    pub fn had_effect(&self) -> bool {
        self.instructions_removed > 0 || self.blocks_removed > 0
    }
    
    /// Formats statistics for human-readable display.
    pub fn format_report(&self, function_name: &str) -> String;
}

impl fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
```

**Relationships**:
- **Returned by**: `DeadCodeElimination::optimize_function()`
- **Contains**: `Vec<ConservativeWarning>`

**Example Usage**:
```rust
let stats = dce.optimize_function(&mut function)?;
if stats.had_effect() {
    println!("{}", stats.format_report(&function.name));
}
```

---

### 3. ConservativeWarning - Diagnostic Information

**Purpose**: Records when conservative analysis prevents optimization, providing debugging insight.

**Definition**:
```rust
/// Warning about a conservative decision that prevented code removal.
///
/// Used for diagnostics and debugging to understand why certain
/// instructions or blocks were not removed despite appearing unused.
///
/// # Example
///
/// An instruction that computes an unused value but calls a function
/// of unknown purity will generate a warning with reason `UnknownCallPurity`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConservativeWarning {
    /// Human-readable description of the instruction.
    /// Example: "call @unknown_func(42) in block 'entry'"
    pub instruction_debug: String,
    
    /// The specific reason this instruction was conservatively kept.
    pub reason: ConservativeReason,
    
    /// Optional: The basic block label where this instruction appears.
    pub block_label: Option<String>,
}
```

**Fields**:

| Field | Type | Purpose | Optional |
|-------|------|---------|----------|
| `instruction_debug` | `String` | Instruction description | No |
| `reason` | `ConservativeReason` | Why kept | No |
| `block_label` | `Option<String>` | Location context | Yes |

**Methods**:
```rust
impl ConservativeWarning {
    /// Creates a new warning.
    ///
    /// # Arguments
    ///
    /// * `instruction_debug` - Human-readable instruction description
    /// * `reason` - The conservative reason for keeping the instruction
    /// * `block_label` - Optional block location for context
    pub fn new(
        instruction_debug: String, 
        reason: ConservativeReason,
        block_label: Option<String>
    ) -> Self;
}

impl fmt::Display for ConservativeWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
```

---

### 4. ConservativeReason - Enumeration of Decision Factors

**Purpose**: Categorizes why an instruction was conservatively kept rather than removed.

**Definition**:
```rust
/// Reasons why an instruction was conservatively preserved.
///
/// Each variant represents a specific limitation in the analysis
/// that prevented proving the instruction was safe to remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConservativeReason {
    /// The instruction may alias with other memory locations.
    /// Unable to prove the store is to a purely local allocation.
    MayAlias,
    
    /// The function being called has unknown purity.
    /// Cannot determine if it has side effects.
    UnknownCallPurity,
    
    /// The pointer operand escapes the current function.
    /// Store may be observable by caller or other code.
    EscapedPointer,
    
    /// The instruction may have other side effects.
    /// Includes: I/O operations, volatile access, atomic operations.
    PotentialSideEffect,
}
```

**Variants**:

| Variant | Meaning | Example |
|---------|---------|---------|
| `MayAlias` | Store may affect aliased memory | `store value, *param_ptr` |
| `UnknownCallPurity` | Call to function without purity annotation | `call @unknown_func()` |
| `EscapedPointer` | Pointer passed to call or stored | `call @foo(&local)` |
| `PotentialSideEffect` | Other observable effects | Future: volatile loads |

**Methods**:
```rust
impl ConservativeReason {
    /// Returns a human-readable explanation of this reason.
    pub fn explanation(&self) -> &'static str;
}

impl fmt::Display for ConservativeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
```

---

### 5. ReachabilityAnalyzer - Unreachable Block Detection

**Purpose**: Performs control-flow graph traversal to identify blocks reachable from the function entry.

**Definition**:
```rust
/// Analyzer for identifying reachable basic blocks via CFG traversal.
///
/// Uses depth-first search starting from the function entry block to
/// mark all blocks that can be reached through control-flow edges.
/// Blocks not marked are unreachable and can be safely removed.
///
/// # Algorithm
///
/// Standard DFS from entry block using petgraph::visit::Dfs:
/// 1. Start at entry block
/// 2. Visit all successor blocks recursively
/// 3. Mark each visited block as reachable
/// 4. Any unmarked blocks are unreachable
///
/// # Complexity
///
/// - Time: O(V + E) where V = blocks, E = edges
/// - Space: O(V) for visited set
struct ReachabilityAnalyzer;
```

**Methods**:
```rust
impl ReachabilityAnalyzer {
    /// Analyzes reachability starting from the function entry block.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The control-flow graph to analyze
    ///
    /// # Returns
    ///
    /// A `HashSet` of `NodeIndex` values for all reachable blocks.
    ///
    /// # Panics
    ///
    /// Panics if the CFG has no entry block (invalid CFG).
    ///
    /// # Example
    ///
    /// ```rust
    /// let reachable = ReachabilityAnalyzer::analyze(&function.cfg);
    /// for block_idx in function.cfg.graph().node_indices() {
    ///     if !reachable.contains(&block_idx) {
    ///         // This block is unreachable
    ///     }
    /// }
    /// ```
    pub fn analyze(cfg: &ControlFlowGraph) -> HashSet<NodeIndex>;
}
```

**Data Flow**:
```
Input: ControlFlowGraph
  ↓
DFS Traversal from Entry
  ↓
Mark Visited Blocks
  ↓
Output: HashSet<NodeIndex> (reachable blocks)
```

**Relationships**:
- **Uses**: `ControlFlowGraph`, `petgraph::visit::Dfs`, `petgraph::graph::NodeIndex`
- **Called by**: `DeadCodeElimination::optimize_function()`

---

### 6. LivenessAnalyzer - Dead Value Detection

**Purpose**: Performs backward dataflow analysis to determine which values are live (used) and which are dead (unused).

**Definition**:
```rust
/// Analyzer for computing value liveness via backward dataflow analysis.
///
/// Determines which computed values are used (live) at each program point
/// and which are never used (dead). Uses a fixed-point iteration over the
/// control-flow graph, propagating liveness information backward from uses
/// to definitions.
///
/// # Algorithm
///
/// Backward dataflow analysis with gen/kill sets:
/// 1. Build def-use chains
/// 2. Compute gen (used) and kill (defined) sets per block
/// 3. Iterate: `live_in[B] = gen[B] ∪ (live_out[B] - kill[B])`
/// 4. Iterate: `live_out[B] = ∪ live_in[successors]`
/// 5. Repeat until fixed-point (no changes)
///
/// # Complexity
///
/// - Time: O(I × (V + E)) where I = iterations (typically 2-3)
/// - Space: O(V) for live sets per block
struct LivenessAnalyzer {
    /// Cache of def-use chains to avoid recomputation.
    def_use_chains: DefUseChains,
}
```

**Methods**:
```rust
impl LivenessAnalyzer {
    /// Creates a new liveness analyzer.
    pub fn new() -> Self;
    
    /// Analyzes liveness for all values in a function.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    /// * `cfg` - The control-flow graph
    ///
    /// # Returns
    ///
    /// A `HashMap` mapping each live `ValueId` to its `LivenessInfo`.
    /// Values not in the map are dead (unused).
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut analyzer = LivenessAnalyzer::new();
    /// let live_values = analyzer.analyze(function, &function.cfg);
    ///
    /// for instruction in block.instructions {
    ///     if let Some(result) = &instruction.result {
    ///         if !live_values.contains_key(&result.id) {
    ///             // This instruction's result is dead
    ///         }
    ///     }
    /// }
    /// ```
    pub fn analyze(
        &mut self, 
        function: &Function, 
        cfg: &ControlFlowGraph
    ) -> HashMap<ValueId, LivenessInfo>;
    
    /// Builds def-use chains for a function.
    ///
    /// Maps each defined value to the instructions that use it,
    /// and maps each instruction to the values it uses.
    fn build_def_use_chains(&mut self, function: &Function);
    
    /// Computes gen and kill sets for each block.
    ///
    /// - Gen: values used before defined in the block
    /// - Kill: values defined in the block
    fn compute_gen_kill_sets(
        &self,
        function: &Function,
        cfg: &ControlFlowGraph
    ) -> (HashMap<NodeIndex, HashSet<ValueId>>, HashMap<NodeIndex, HashSet<ValueId>>);
}
```

**Relationships**:
- **Uses**: `DefUseChains`, `LivenessInfo`
- **Called by**: `DeadCodeElimination::optimize_function()`

---

### 7. LivenessInfo - Per-Value Liveness Data

**Purpose**: Records where a value is first used and last used for precise liveness tracking.

**Definition**:
```rust
/// Liveness information for a single value.
///
/// Tracks the first and last instructions where the value is used,
/// enabling precise determination of the value's live range.
///
/// # Fields
///
/// * `first_use` - Instruction index of first use (optional if unused)
/// * `last_use` - Instruction index of last use (optional if unused)
/// * `used_in_blocks` - Set of block indices where value is used
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivenessInfo {
    /// The first instruction that uses this value.
    /// `None` if the value is never used (dead).
    pub first_use: Option<InstructionIndex>,
    
    /// The last instruction that uses this value.
    /// `None` if the value is never used (dead).
    pub last_use: Option<InstructionIndex>,
    
    /// Set of basic blocks where this value is used.
    /// Empty if the value is never used (dead).
    pub used_in_blocks: HashSet<NodeIndex>,
}
```

**Methods**:
```rust
impl LivenessInfo {
    /// Creates liveness info for an unused (dead) value.
    pub fn dead() -> Self;
    
    /// Creates liveness info for a value with specific use locations.
    pub fn with_uses(
        first_use: InstructionIndex,
        last_use: InstructionIndex,
        used_in_blocks: HashSet<NodeIndex>
    ) -> Self;
    
    /// Checks if this value is live (has any uses).
    pub fn is_live(&self) -> bool {
        self.first_use.is_some()
    }
}
```

**Relationships**:
- **Contained in**: `HashMap<ValueId, LivenessInfo>` (output of `LivenessAnalyzer`)
- **Uses**: `InstructionIndex`, `NodeIndex`

---

### 8. DefUseChains - Definition-Use Relationships

**Purpose**: Efficiently tracks which instructions define values and which instructions use those values.

**Definition**:
```rust
/// Bidirectional mapping between value definitions and uses.
///
/// Enables efficient lookup of:
/// - Given a value, which instructions use it? (def → uses)
/// - Given an instruction, which values does it use? (use → defs)
///
/// # Implementation
///
/// Uses two `HashMap` structures for O(1) bidirectional queries.
#[derive(Debug, Clone, Default)]
struct DefUseChains {
    /// Maps each defined value to the list of instructions that use it.
    /// Key: ValueId, Value: Vec<InstructionIndex>
    value_to_uses: HashMap<ValueId, Vec<InstructionIndex>>,
    
    /// Maps each instruction to the values it uses.
    /// Key: InstructionIndex, Value: Vec<ValueId>
    instruction_to_used_values: HashMap<InstructionIndex, Vec<ValueId>>,
    
    /// Maps each instruction to the value it defines (if any).
    /// Key: InstructionIndex, Value: ValueId
    instruction_to_defined_value: HashMap<InstructionIndex, ValueId>,
}
```

**Methods**:
```rust
impl DefUseChains {
    /// Creates empty def-use chains.
    pub fn new() -> Self;
    
    /// Records that an instruction defines a value.
    pub fn add_definition(&mut self, inst_idx: InstructionIndex, value_id: ValueId);
    
    /// Records that an instruction uses a value.
    pub fn add_use(&mut self, inst_idx: InstructionIndex, value_id: ValueId);
    
    /// Returns all instructions that use a given value.
    pub fn get_uses(&self, value_id: ValueId) -> &[InstructionIndex];
    
    /// Returns all values used by a given instruction.
    pub fn get_used_values(&self, inst_idx: InstructionIndex) -> &[ValueId];
    
    /// Returns the value defined by a given instruction, if any.
    pub fn get_defined_value(&self, inst_idx: InstructionIndex) -> Option<ValueId>;
    
    /// Checks if a value has any uses.
    pub fn has_uses(&self, value_id: ValueId) -> bool {
        self.get_uses(value_id).len() > 0
    }
}
```

**Relationships**:
- **Used by**: `LivenessAnalyzer`
- **Built from**: `Function`, `ControlFlowGraph`

---

### 9. EscapeAnalyzer - Pointer Escape Detection

**Purpose**: Determines which allocated objects (allocas) have their addresses taken or may be accessed through pointers.

**Definition**:
```rust
/// Analyzer for determining which allocations escape their defining function.
///
/// Uses flow-insensitive analysis to conservatively track whether the
/// address of an allocation:
/// - Is stored to memory (escapes to caller/globals)
/// - Is passed to a function call (escapes to callee)
/// - Is returned from the function (escapes to caller)
///
/// # Algorithm
///
/// Single-pass conservative analysis:
/// 1. Mark all allocas as Local initially
/// 2. Scan all instructions for escape conditions:
///    - Store of alloca pointer → Escaped
///    - Call with alloca argument → Escaped
///    - Return of alloca pointer → Escaped
///    - GEP of alloca → AddressTaken
/// 3. Result: mapping of ValueId to EscapeStatus
///
/// # Complexity
///
/// - Time: O(I) where I = total instructions
/// - Space: O(A) where A = number of allocas
struct EscapeAnalyzer;
```

**Methods**:
```rust
impl EscapeAnalyzer {
    /// Analyzes escape behavior for all allocations in a function.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    ///
    /// # Returns
    ///
    /// A `HashMap` mapping each alloca's `ValueId` to its `EscapeStatus`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let escape_info = EscapeAnalyzer::analyze(function);
    ///
    /// for instruction in block.instructions {
    ///     if let InstructionKind::Store { dest, .. } = &instruction.kind {
    ///         if matches!(escape_info.get(&dest.id), Some(EscapeStatus::Local)) {
    ///             // This store is to a provably-local allocation
    ///             // Safe to remove if value is dead
    ///         }
    ///     }
    /// }
    /// ```
    pub fn analyze(function: &Function) -> HashMap<ValueId, EscapeStatus>;
}
```

**Relationships**:
- **Returns**: `HashMap<ValueId, EscapeStatus>`
- **Called by**: `DeadCodeElimination::optimize_function()`

---

### 10. EscapeStatus - Escape Classification

**Purpose**: Categorizes the escape behavior of an allocated object.

**Definition**:
```rust
/// Classification of how an allocation's address is used.
///
/// Determines whether the allocation is provably local (safe to optimize)
/// or may be accessed from outside the function (must preserve).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EscapeStatus {
    /// The allocation's address never leaves the local function scope.
    /// Only used in direct loads and stores.
    /// Safe to remove if dead.
    Local,
    
    /// The address is computed (e.g., via GetElementPtr) but not stored or passed.
    /// Potentially safe to optimize depending on context.
    AddressTaken,
    
    /// The address escapes the function scope via:
    /// - Being stored to memory
    /// - Being passed to a function call
    /// - Being returned from the function
    /// Must conservatively preserve all operations on this allocation.
    Escaped,
}
```

**Variant Semantics**:

| Status | Meaning | Operations | Removability |
|--------|---------|------------|--------------|
| `Local` | Never escapes | Load/Store only | High - if dead |
| `AddressTaken` | Address computed | Load/Store/GEP | Medium - case-by-case |
| `Escaped` | Leaves function | Any | Low - preserve |

**Methods**:
```rust
impl EscapeStatus {
    /// Returns true if operations on this allocation are safe to remove when dead.
    pub fn is_safe_to_optimize(&self) -> bool {
        matches!(self, EscapeStatus::Local)
    }
    
    /// Returns the most conservative (least optimizable) of two statuses.
    pub fn max(self, other: Self) -> Self;
}

impl Ord for EscapeStatus {
    // Ordering: Local < AddressTaken < Escaped
    fn cmp(&self, other: &Self) -> Ordering;
}
```

---

### 11. SideEffectClass - Instruction Effect Classification

**Purpose**: Categorizes instructions by their observable effects to determine removal safety.

**Definition**:
```rust
/// Classification of instruction side effects.
///
/// Determines whether an instruction can be safely removed based on
/// whether it has observable effects beyond computing a value.
///
/// # Removal Rules
///
/// - `Pure`: Removable if result is unused
/// - `MemoryRead`: Removable if result is unused (unless volatile)
/// - `MemoryWrite`: Removable only if target is provably local and dead
/// - `EffectFul`: Never removable (always has observable behavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SideEffectClass {
    /// No observable side effects. Pure computation.
    /// Examples: add, sub, cast, getelementptr
    /// Removal condition: Result is unused
    Pure,
    
    /// Reads memory but doesn't modify program state.
    /// Examples: load (non-volatile)
    /// Removal condition: Result is unused AND not volatile
    MemoryRead,
    
    /// Modifies memory, may be observable.
    /// Examples: store (to potentially-aliased memory), alloca
    /// Removal condition: Target is provably local AND has no subsequent loads
    MemoryWrite,
    
    /// Has definite side effects, always observable.
    /// Examples: call (unknown purity), volatile/atomic operations
    /// Removal condition: Never (always keep)
    EffectFul,
}
```

**Classification Table**:

| Instruction | Side Effect Class | Removal Condition |
|-------------|-------------------|-------------------|
| Binary (add, sub, mul, div, etc.) | Pure | Result unused |
| Unary (neg, not) | Pure | Result unused |
| Cast | Pure | Result unused |
| GetElementPtr | Pure | Result unused |
| Phi | Pure | Result unused |
| Vector ops | Pure | Result unused |
| Load | MemoryRead | Result unused AND not volatile |
| Store (local) | MemoryWrite | Target is local AND dead |
| Store (escaped) | EffectFul | Never remove |
| Alloca | MemoryWrite | No stores/loads AND non-escaped |
| Call | EffectFul | Never remove (unless pure) |

**Methods**:
```rust
impl SideEffectClass {
    /// Determines the side effect class for a given instruction.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The instruction to classify
    /// * `escape_info` - Escape analysis results for allocations
    ///
    /// # Returns
    ///
    /// The appropriate `SideEffectClass` for this instruction.
    ///
    /// # Example
    ///
    /// ```rust
    /// let class = SideEffectClass::classify(&instruction, &escape_info);
    ///
    /// match class {
    ///     SideEffectClass::Pure if !is_live => {
    ///         // Safe to remove
    ///     }
    ///     SideEffectClass::EffectFul => {
    ///         // Never remove
    ///     }
    ///     _ => { /* Case-by-case */ }
    /// }
    /// ```
    pub fn classify(
        instruction: &Instruction,
        escape_info: &HashMap<ValueId, EscapeStatus>
    ) -> Self;
    
    /// Returns true if this instruction can be removed when its result is unused.
    pub fn is_removable_if_unused(&self) -> bool {
        matches!(self, SideEffectClass::Pure | SideEffectClass::MemoryRead)
    }
}
```

---

### 12. InstructionIndex - Instruction Location Identifier

**Purpose**: Uniquely identifies an instruction within a function for def-use chain tracking.

**Definition**:
```rust
/// Unique identifier for an instruction within a function.
///
/// Combines block index and instruction offset within that block
/// to provide a stable, comparable identifier for instructions.
///
/// # Invariants
///
/// - `block_idx` must be a valid NodeIndex in the CFG
/// - `inst_offset` must be < block.instructions.len()
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionIndex {
    /// The basic block containing this instruction.
    pub block_idx: NodeIndex,
    
    /// The offset of this instruction within the block's instruction list.
    pub inst_offset: usize,
}
```

**Methods**:
```rust
impl InstructionIndex {
    /// Creates a new instruction index.
    ///
    /// # Arguments
    ///
    /// * `block_idx` - The block's NodeIndex in the CFG
    /// * `inst_offset` - The instruction's offset within the block
    pub fn new(block_idx: NodeIndex, inst_offset: usize) -> Self;
}

impl Ord for InstructionIndex {
    // Orders by block_idx, then inst_offset
    fn cmp(&self, other: &Self) -> Ordering;
}

impl fmt::Display for InstructionIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block[{}].inst[{}]", self.block_idx.index(), self.inst_offset)
    }
}
```

**Relationships**:
- **Used by**: `DefUseChains`, `LivenessInfo`
- **References**: `NodeIndex` (from petgraph)

---

## Data Structure Relationships Diagram

```
┌────────────────────────────┐
│  DeadCodeElimination       │ (Main orchestrator)
│  - max_iterations          │
│  - enable_statistics       │
│  - verbose_warnings        │
└────────────┬───────────────┘
             │ orchestrates
             ├─────────────────────────┬─────────────────────────┐
             │                         │                         │
             ▼                         ▼                         ▼
    ┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
    │ Reachability    │      │ Liveness        │      │ Escape          │
    │ Analyzer        │      │ Analyzer        │      │ Analyzer        │
    └────────┬────────┘      └────────┬────────┘      └────────┬────────┘
             │                         │                         │
             │ produces                │ produces                │ produces
             ▼                         ▼                         ▼
    ┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
    │ HashSet<        │      │ HashMap<        │      │ HashMap<        │
    │   NodeIndex>    │      │   ValueId,      │      │   ValueId,      │
    │                 │      │   LivenessInfo> │      │   EscapeStatus> │
    └─────────────────┘      └────────┬────────┘      └─────────────────┘
                                      │
                                      │ uses
                                      ▼
                             ┌─────────────────┐
                             │ DefUseChains    │
                             │ - value_to_uses │
                             │ - inst_to_vals  │
                             └─────────────────┘
                                      │
                                      │ references
                                      ▼
                             ┌─────────────────┐
                             │ InstructionIndex│
                             │ - block_idx     │
                             │ - inst_offset   │
                             └─────────────────┘

┌────────────────────────────┐
│  OptimizationStats         │ (Results reporting)
│  - instructions_removed    │
│  - blocks_removed          │
│  - iterations              │
│  - conservative_warnings   │
└────────────┬───────────────┘
             │ contains
             ▼
    ┌─────────────────┐
    │ Conservative    │
    │ Warning         │
    │ - instruction   │
    │ - reason        │
    │ - block_label   │
    └────────┬────────┘
             │ categorized by
             ▼
    ┌─────────────────┐
    │ Conservative    │
    │ Reason          │
    │ (enum)          │
    └─────────────────┘
```

## Type Aliases and Helper Types

```rust
/// Node index in the control-flow graph.
/// Re-exported from petgraph for convenience.
pub use petgraph::graph::NodeIndex;

/// Set of basic block indices.
pub type BlockSet = HashSet<NodeIndex>;

/// Liveness information map: ValueId → LivenessInfo
pub type LivenessMap = HashMap<ValueId, LivenessInfo>;

/// Escape information map: ValueId → EscapeStatus
pub type EscapeMap = HashMap<ValueId, EscapeStatus>;
```

## Validation Rules Summary

### DeadCodeElimination
- ✅ `max_iterations` must be > 0
- ✅ After optimization, `function.verify()` must pass
- ✅ SSA form must be preserved (no undefined uses)

### OptimizationStats
- ✅ `instructions_removed` and `blocks_removed` must equal actual removals
- ✅ `iterations` must be ≤ `max_iterations`

### LivenessInfo
- ✅ If `is_live()`, then `first_use` and `last_use` must be Some
- ✅ `last_use` must be ≥ `first_use` (chronological order)

### DefUseChains
- ✅ Every use must have a corresponding definition
- ✅ No instruction can define multiple values (SSA property)

### InstructionIndex
- ✅ `block_idx` must be valid in CFG
- ✅ `inst_offset` must be < block.instructions.len()

## Memory and Performance Characteristics

| Data Structure | Space Complexity | Build Time | Query Time |
|----------------|------------------|------------|------------|
| `DeadCodeElimination` | O(1) | N/A | N/A |
| `OptimizationStats` | O(W) warnings | N/A | N/A |
| `ReachabilityAnalyzer` result | O(V) | O(V+E) | O(1) |
| `LivenessAnalyzer` result | O(V) | O(I×(V+E)) | O(1) |
| `DefUseChains` | O(I+U) | O(I+U) | O(1) |
| `EscapeAnalyzer` result | O(A) | O(I) | O(1) |

Where:
- V = number of values
- E = number of CFG edges
- I = number of instructions
- A = number of allocas
- U = total uses of all values
- W = number of warnings

## Thread Safety and Concurrency

All data structures are designed for single-threaded use within the optimization phase:

- **Not thread-safe**: `DeadCodeElimination`, analyzers (designed for sequential use)
- **Immutable after construction**: Analysis results (can be shared across threads if needed)
- **No interior mutability**: No RefCell, Mutex, or Arc required

**Future enhancement**: Module-level parallelism - multiple functions can be optimized in parallel using separate `DeadCodeElimination` instances.

## Error Handling Strategy

```rust
/// Errors that can occur during DCE optimization.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DceError {
    /// CFG verification failed after optimization.
    #[error("CFG verification failed: {0}")]
    CfgVerificationFailed(String),
    
    /// Invalid instruction index (should never occur).
    #[error("Invalid instruction index: {0}")]
    InvalidInstructionIndex(String),
    
    /// Maximum iterations reached (potential infinite loop).
    #[error("Maximum iterations ({0}) reached without convergence")]
    MaxIterationsExceeded(usize),
}
```

**Error Handling Policy**:
- **CFG verification failure**: Return error, do not panic (allows caller to handle gracefully)
- **Internal invariant violations**: Use `debug_assert!` in debug builds, return error in release
- **Maximum iterations**: Emit warning, return partial results (optimization is best-effort)

## Conclusion

This data model provides a comprehensive, type-safe foundation for the Dead Code Elimination optimization phase. All structures are designed following Rust best practices:

- ✅ Leverage type system for safety (enums, Option, Result)
- ✅ Use efficient standard collections (HashMap, HashSet)
- ✅ Provide clear documentation with examples
- ✅ Define precise validation rules and invariants
- ✅ Enable easy testing and debugging (Debug, Clone, PartialEq)

The model is ready for implementation in Phase 2.
