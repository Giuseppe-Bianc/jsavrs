# Data Model: Constant Folding Optimizer

**Feature**: `015-constant-folding-sccp`  
**Date**: 2025-11-14  
**Purpose**: Define core data structures, state machines, and relationships for the constant folding optimizer

---

## Overview

This document defines the data model for the constant folding and SCCP optimizer, including lattice value representation, optimization metrics, and internal state management. All structures are designed for performance, memory efficiency, and maintainability in accordance with the research decisions documented in `research.md`.

---

## Core Data Structures

### 1. LatticeValue

**Purpose**: Represents compile-time knowledge about an SSA value during SCCP analysis.

**Definition**:
```rust
/// Represents the lattice value of an SSA value during SCCP analysis.
/// 
/// The lattice forms a partial order:
/// ```text
///       Top (unknown)
///      /   \
/// Constant  ... (other constants)
///      \   /
///      Bottom (not constant)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum LatticeValue {
    /// Not yet analyzed; initial state for all values except function parameters.
    Top,
    
    /// Proven to be a specific compile-time constant.
    Constant(IrLiteralValue),
    
    /// Proven to be non-constant or have multiple possible values.
    Bottom,
}
```

**State Transitions**:
```text
Top → Constant: When analysis determines a definite value
Top → Bottom: When analysis determines non-constant (rare; usually goes through Constant)
Constant → Bottom: When conflicting values discovered (e.g., phi merge with different constants)
Constant → Constant: When same value confirmed (merge identity)

Invalid transitions (never occur):
Bottom → Top (monotonic lattice)
Bottom → Constant (monotonic lattice)
Constant → Top (monotonic lattice)
```

**Operations**:
```rust
impl LatticeValue {
    /// Compute the meet (greatest lower bound) of two lattice values.
    /// 
    /// Meet operation properties:
    /// - Commutative: meet(a, b) = meet(b, a)
    /// - Associative: meet(a, meet(b, c)) = meet(meet(a, b), c)
    /// - Idempotent: meet(a, a) = a
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (LatticeValue::Top, x) | (x, LatticeValue::Top) => x.clone(),
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
    
    /// Returns true if this lattice value represents a constant.
    pub fn is_constant(&self) -> bool {
        matches!(self, LatticeValue::Constant(_))
    }
    
    /// Extracts the constant value if this is a Constant variant.
    pub fn as_constant(&self) -> Option<&IrLiteralValue> {
        if let LatticeValue::Constant(c) = self {
            Some(c)
        } else {
            None
        }
    }
}
```

**Memory Footprint**:
- Enum discriminant: 1 byte (optimized by compiler)
- IrLiteralValue: ~16 bytes (varies by type, but bounded)
- **Total per entry**: ~20-24 bytes (with padding)

**Validation Rules**:
- Never transition from Bottom to Top/Constant (lattice monotonicity)
- Meet operation must produce valid lattice value
- Constant variant must hold valid IrLiteralValue for its type

---

### 2. ConstantFoldingOptimizer

**Purpose**: Main optimizer struct implementing the Phase trait and managing configuration.

**Definition**:
```rust
/// Constant folding and propagation optimizer implementing the Phase trait.
/// 
/// Supports two modes:
/// - Basic mode (sccp=false): Constant folding only, no control flow analysis
/// - SCCP mode (sccp=true): Full Sparse Conditional Constant Propagation with CFG analysis
#[derive(Debug)]
pub struct ConstantFoldingOptimizer {
    /// Enable verbose statistics output to stderr.
    pub verbose: bool,
    
    /// Enable SCCP analysis (worklist algorithm with control flow).
    pub sccp: bool,
    
    /// Accumulated statistics across all processed functions.
    statistics: AggregateStatistics,
}
```

**Constructor**:
```rust
impl ConstantFoldingOptimizer {
    /// Creates a new constant folding optimizer with the specified configuration.
    /// 
    /// # Arguments
    /// * `verbose` - If true, emit detailed per-function statistics to stderr
    /// * `sccp` - If true, enable SCCP mode with control flow analysis
    pub fn new(verbose: bool, sccp: bool) -> Self {
        Self {
            verbose,
            sccp,
            statistics: AggregateStatistics::default(),
        }
    }
}
```

**Relationships**:
- Implements `Phase` trait (defined in `src/ir/optimizer/phase.rs`)
- Owns `AggregateStatistics` for cross-function metrics
- Consumes and mutates `Module` during `run()` method

---

### 3. FunctionMetrics

**Purpose**: Tracks optimization metrics for a single function.

**Definition**:
```rust
/// Per-function optimization metrics for diagnostics and profiling.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FunctionMetrics {
    /// Name of the function being optimized (for reporting).
    pub function_name: String,
    
    /// Number of instructions before optimization.
    pub instructions_before: usize,
    
    /// Number of instructions after optimization.
    pub instructions_after: usize,
    
    /// Number of constant expressions folded (e.g., `2 + 3` → `5`).
    pub constants_folded: usize,
    
    /// Number of load instructions replaced with constants.
    pub loads_propagated: usize,
    
    /// Number of conditional branches resolved to unconditional branches.
    pub branches_resolved: usize,
    
    /// Number of basic blocks removed as unreachable.
    pub blocks_removed: usize,
    
    /// True if SCCP mode was enabled for this function.
    pub sccp_enabled: bool,
    
    /// True if SCCP fell back to basic mode due to memory limit.
    pub sccp_fallback: bool,
}
```

**Derived Metrics**:
```rust
impl FunctionMetrics {
    /// Calculates the number of instructions removed.
    pub fn instructions_removed(&self) -> usize {
        self.instructions_before.saturating_sub(self.instructions_after)
    }
    
    /// Calculates the percentage of instructions removed.
    pub fn removal_percentage(&self) -> f64 {
        if self.instructions_before == 0 {
            0.0
        } else {
            (self.instructions_removed() as f64 / self.instructions_before as f64) * 100.0
        }
    }
    
    /// Returns true if any optimizations were performed.
    pub fn has_changes(&self) -> bool {
        self.constants_folded > 0 
            || self.loads_propagated > 0 
            || self.branches_resolved > 0 
            || self.blocks_removed > 0
    }
}
```

**Validation Rules**:
- `instructions_after` ≤ `instructions_before` (optimizations only remove, never add)
- If `sccp_fallback` is true, then `sccp_enabled` must also be true
- Counts (`constants_folded`, etc.) must not exceed `instructions_before`

---

### 4. AggregateStatistics

**Purpose**: Accumulates metrics across all functions in a module.

**Definition**:
```rust
/// Module-level aggregate statistics for all functions processed.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AggregateStatistics {
    /// Total number of functions processed.
    pub total_functions: usize,
    
    /// Total instructions removed across all functions.
    pub total_instructions_removed: usize,
    
    /// Total constant expressions folded.
    pub total_constants_folded: usize,
    
    /// Total load instructions propagated.
    pub total_loads_propagated: usize,
    
    /// Total conditional branches resolved.
    pub total_branches_resolved: usize,
    
    /// Total basic blocks removed.
    pub total_blocks_removed: usize,
    
    /// Number of functions where SCCP fell back to basic mode.
    pub sccp_fallback_count: usize,
}
```

**Operations**:
```rust
impl AggregateStatistics {
    /// Accumulates metrics from a single function.
    pub fn add(&mut self, metrics: &FunctionMetrics) {
        self.total_functions += 1;
        self.total_instructions_removed += metrics.instructions_removed();
        self.total_constants_folded += metrics.constants_folded;
        self.total_loads_propagated += metrics.loads_propagated;
        self.total_branches_resolved += metrics.branches_resolved;
        self.total_blocks_removed += metrics.blocks_removed;
        
        if metrics.sccp_fallback {
            self.sccp_fallback_count += 1;
        }
    }
    
    /// Prints a summary to stderr if verbose mode is enabled.
    pub fn print_summary(&self, verbose: bool) {
        if verbose {
            eprintln!("=== Constant Folding Optimizer Statistics ===");
            eprintln!("Functions processed: {}", self.total_functions);
            eprintln!("Instructions removed: {}", self.total_instructions_removed);
            eprintln!("Constants folded: {}", self.total_constants_folded);
            eprintln!("Loads propagated: {}", self.total_loads_propagated);
            eprintln!("Branches resolved: {}", self.total_branches_resolved);
            eprintln!("Blocks removed: {}", self.total_blocks_removed);
            
            if self.sccp_fallback_count > 0 {
                eprintln!("SCCP fallbacks (memory limit): {}", self.sccp_fallback_count);
            }
            eprintln!("===========================================");
        }
    }
}
```

---

### 5. SCCPContext

**Purpose**: Encapsulates SCCP analysis state for a single function (internal to `worklist.rs`).

**Definition**:
```rust
/// Internal state for SCCP worklist algorithm.
/// 
/// This structure is created per-function and discarded after analysis completes.
/// Memory usage is bounded by the 100KB lattice limit.
#[derive(Debug)]
pub(crate) struct SCCPContext {
    /// Lattice value for each SSA value.
    /// Key: ValueId, Value: LatticeValue (Top/Constant/Bottom)
    lattice_map: HashMap<ValueId, LatticeValue>,
    
    /// Set of executable CFG edges (predecessor → successor).
    executable_edges: HashSet<(NodeIndex, NodeIndex)>,
    
    /// Set of reachable basic blocks.
    reachable_blocks: HashSet<NodeIndex>,
    
    /// Worklist of SSA values to process (FIFO queue for breadth-first).
    worklist: VecDeque<ValueId>,
    
    /// Worklist of CFG edges to process (for newly executable edges).
    edge_worklist: VecDeque<(NodeIndex, NodeIndex)>,
}
```

**Initialization**:
```rust
impl SCCPContext {
    /// Creates a new SCCP context for the given function.
    /// 
    /// Initially:
    /// - All lattice values are Top (except entry block which starts processing)
    /// - No edges are executable
    /// - Only entry block is reachable
    pub fn new(function: &Function) -> Self {
        let estimated_values = function.value_count();
        let estimated_blocks = function.block_count();
        
        Self {
            lattice_map: HashMap::with_capacity(estimated_values),
            executable_edges: HashSet::with_capacity(estimated_blocks * 2), // Avg 2 successors
            reachable_blocks: HashSet::with_capacity(estimated_blocks),
            worklist: VecDeque::with_capacity(estimated_values / 4), // Conservative estimate
            edge_worklist: VecDeque::with_capacity(estimated_blocks),
        }
    }
    
    /// Checks if lattice memory usage exceeds the 100KB limit.
    /// 
    /// Uses conservative estimate: 24 bytes per entry (ValueId + LatticeValue + overhead).
    pub fn exceeds_memory_limit(&self) -> bool {
        const ENTRY_SIZE: usize = 24;
        const MAX_BYTES: usize = 100_000;
        
        self.lattice_map.len() * ENTRY_SIZE > MAX_BYTES
    }
}
```

**Lifecycle**:
1. Created at function analysis start
2. Populated during worklist processing
3. Used to build transformation plan
4. Discarded after function optimization completes (memory freed)

**Memory Bounds**:
- Lattice map: ~100KB maximum (enforced by `exceeds_memory_limit()`)
- Executable edges: O(E) where E = CFG edge count (typically < 10KB)
- Reachable blocks: O(B) where B = block count (typically < 1KB)
- Worklists: Bounded by value/edge count, but transient (processed FIFO)
- **Total**: ~110-120KB peak per function (within acceptable limits)

---

## State Machines

### SCCP Analysis State Machine

**States**:
1. **Initialization**: Entry block marked reachable, all others unreachable
2. **Processing**: Worklist drains, lattice values updated, edges marked executable
3. **Fixed Point**: Worklist empty, no more changes
4. **Transformation**: Apply optimizations based on final lattice state
5. **Cleanup**: Remove unreachable blocks, update phi nodes

**State Transitions**:
```text
Initialization
    ↓
Processing ←──┐ (worklist non-empty)
    ↓         │
    └─────────┘
    ↓ (worklist empty)
Fixed Point
    ↓
Transformation
    ↓
Cleanup
    ↓
Complete
```

**Detailed Processing Loop**:
```rust
fn sccp_analysis(function: &Function) -> SCCPContext {
    let mut ctx = SCCPContext::new(function);
    
    // Initialization: mark entry reachable
    let entry = function.entry_block();
    ctx.reachable_blocks.insert(entry);
    ctx.edge_worklist.push_back((entry, entry)); // Sentinel for entry
    
    // Processing: drain worklists
    while !ctx.edge_worklist.is_empty() || !ctx.worklist.is_empty() {
        // Process CFG edges first (discovers new blocks/instructions)
        while let Some((pred, succ)) = ctx.edge_worklist.pop_front() {
            if ctx.executable_edges.insert((pred, succ)) {
                // New executable edge: process all instructions in successor
                if ctx.reachable_blocks.insert(succ) {
                    for instruction in function.block(succ).instructions() {
                        ctx.worklist.push_back(instruction.result_value());
                    }
                }
                // Re-evaluate phi nodes in successor
                for phi in function.block(succ).phis() {
                    ctx.worklist.push_back(phi.result_value());
                }
            }
        }
        
        // Process SSA values
        while let Some(value_id) = ctx.worklist.pop_front() {
            let new_lattice = evaluate_value(value_id, function, &ctx.lattice_map);
            let old_lattice = ctx.lattice_map.get(&value_id).unwrap_or(&LatticeValue::Top);
            
            if new_lattice != *old_lattice {
                ctx.lattice_map.insert(value_id, new_lattice.clone());
                
                // Propagate changes to uses
                for use_site in function.uses_of(value_id) {
                    ctx.worklist.push_back(use_site);
                }
                
                // Handle constant branches
                if let Some(branch) = is_conditional_branch(value_id, function) {
                    if let LatticeValue::Constant(cond) = new_lattice {
                        // Mark one successor executable
                        let target = if cond.as_bool() { branch.true_target } else { branch.false_target };
                        ctx.edge_worklist.push_back((branch.parent_block, target));
                    }
                }
            }
        }
        
        // Check memory limit
        if ctx.exceeds_memory_limit() {
            eprintln!("Warning: SCCP memory limit exceeded, falling back to basic folding");
            return fallback_context(); // Return empty context to trigger fallback
        }
    }
    
    // Fixed point reached
    ctx
}
```

---

## Relationships Between Entities

### Entity Relationship Diagram

```text
┌─────────────────────────────┐
│ ConstantFoldingOptimizer    │
│ ┌─────────────────────────┐ │
│ │ verbose: bool           │ │
│ │ sccp: bool              │ │
│ │ statistics              │─┼─────┐
│ └─────────────────────────┘ │     │
└─────────────────────────────┘     │
         │                           │
         │ implements                │
         ↓                           ↓
┌─────────────────────┐    ┌──────────────────────┐
│ Phase (trait)       │    │ AggregateStatistics  │
│ ┌─────────────────┐ │    │ ┌──────────────────┐ │
│ │ name()          │ │    │ │ total_functions  │ │
│ │ run(&mut Module)│ │    │ │ total_*_removed  │ │
│ └─────────────────┘ │    │ │ sccp_fallback_*  │ │
└─────────────────────┘    │ └──────────────────┘ │
                           └──────────────────────┘
                                    ↑
                                    │ accumulates
                                    │
                           ┌──────────────────┐
                           │ FunctionMetrics  │
                           │ ┌──────────────┐ │
                           │ │ function_name│ │
                           │ │ instr_before │ │
                           │ │ instr_after  │ │
                           │ │ *_folded     │ │
                           │ └──────────────┘ │
                           └──────────────────┘
                                    ↑
                                    │ produced by
                                    │
┌───────────────────────────────────┴────┐
│ SCCPContext (per-function analysis)    │
│ ┌────────────────────────────────────┐ │
│ │ lattice_map: HashMap<ValueId, LV>  │ │
│ │ executable_edges: HashSet<...>     │ │
│ │ reachable_blocks: HashSet<...>     │ │
│ │ worklist: VecDeque<ValueId>        │ │
│ └────────────────────────────────────┘ │
└────────────────────────────────────────┘
         │
         │ contains
         ↓
┌──────────────────┐
│ LatticeValue     │
│ ┌──────────────┐ │
│ │ Top          │ │
│ │ Constant(val)│ │
│ │ Bottom       │ │
│ └──────────────┘ │
└──────────────────┘
```

### Data Flow

```text
Module (input)
    ↓
ConstantFoldingOptimizer.run()
    ↓
For each Function in Module:
    ↓
    Create FunctionMetrics (track changes)
    ↓
    ┌──────────────────────────────┐
    │ Basic Constant Folding Pass  │ (always runs)
    └──────────────────────────────┘
    ↓
    ┌──────────────────────────────┐
    │ SCCP Analysis (if enabled)   │
    │   Create SCCPContext         │
    │   Worklist algorithm         │
    │   Build lattice_map          │
    └──────────────────────────────┘
    ↓
    ┌──────────────────────────────┐
    │ Transformation Pass          │
    │   Apply constant replacements│
    │   Update phi nodes           │
    └──────────────────────────────┘
    ↓
    ┌──────────────────────────────┐
    │ CFG Cleanup Pass             │ (always runs)
    │   Remove unreachable blocks  │
    │   Remove dead phi edges      │
    └──────────────────────────────┘
    ↓
    Update FunctionMetrics
    ↓
    AggregateStatistics.add(metrics)
    ↓
End Function loop
    ↓
AggregateStatistics.print_summary(verbose)
    ↓
Print total instruction count to stdout
    ↓
Module (output, mutated)
```

---

## Validation Rules

### Lattice Invariants
1. Lattice values must respect partial order: Top ≥ Constant ≥ Bottom
2. Meet operation must be commutative, associative, and idempotent
3. No upward transitions in lattice (monotonicity)

### Metrics Invariants
1. `instructions_after` ≤ `instructions_before` (no instruction creation)
2. Sum of individual counts ≤ `instructions_before` (no double-counting)
3. If `sccp_fallback`, then `sccp_enabled` must be true
4. Aggregate totals = sum of per-function metrics

### SCCP Context Invariants
1. Entry block always in `reachable_blocks`
2. If edge `(A, B)` in `executable_edges`, then B in `reachable_blocks`
3. Lattice map memory ≤ 100KB (enforced by `exceeds_memory_limit()`)
4. Worklists contain only valid ValueId and NodeIndex references

### SSA Preservation
1. Each value defined exactly once (not validated by this module, assumed from IR)
2. All uses dominated by definition (CFG dominance preserved)
3. Phi nodes have correct arity after edge removal (cleanup pass responsibility)

---

## Performance Characteristics

| Structure | Size (bytes) | Lookup | Insert | Iteration |
|-----------|-------------|--------|--------|-----------|
| LatticeValue | ~20-24 | N/A | N/A | N/A |
| HashMap<ValueId, LatticeValue> | ~100KB max | O(1) avg | O(1) avg | O(n) |
| HashSet<NodeIndex> | ~1-10KB | O(1) avg | O(1) avg | O(n) |
| VecDeque<ValueId> | Dynamic | O(1) front/back | O(1) push/pop | N/A |
| FunctionMetrics | ~100 bytes | N/A | N/A | N/A |
| AggregateStatistics | ~64 bytes | N/A | N/A | N/A |

**Total Memory Per Function**: ~110-130KB peak during SCCP analysis, released immediately after.

---

## Next Steps

Proceed to **Phase 1: Contracts** generation:
- Define API contracts for Phase trait implementation
- Define evaluator contract for constant folding operations
- Define worklist contract for SCCP algorithm
