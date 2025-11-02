# Dead Code Elimination: Research & Technical Analysis

**Feature**: Dead Code Elimination (DCE) Optimization Phase  
**Date**: 2025-11-02  
**Status**: Research Complete

## Executive Summary

This document provides comprehensive research and analysis for implementing a Dead Code Elimination (DCE) optimization phase for the jsavrs compiler. DCE is a fundamental compiler optimization that removes code which is never executed (unreachable code) or whose results are never used (dead code). The implementation uses well-established compiler techniques including control-flow graph analysis, dataflow analysis, and iterative fixed-point computation.

## Core Algorithmic Approaches

### 1. Reachability Analysis (Unreachable Code Elimination)

**Objective**: Identify and remove basic blocks that cannot be reached from the function entry point through any control-flow path.

**Algorithm Selection**: Depth-First Search (DFS) from entry block

**Decision Rationale**:
- **DFS chosen over BFS**: DFS requires less memory (O(depth) vs O(width)) and is sufficient for reachability marking. The traversal order doesn't affect correctness since we only need to mark reachable nodes.
- **Single-pass sufficiency**: A single DFS traversal from the entry block marks all reachable blocks. Unreachable blocks are simply those not visited during traversal.
- **Petgraph integration**: The existing petgraph dependency provides optimized DFS implementation through `petgraph::visit::Dfs`, avoiding manual stack management and providing tested correctness.

**Implementation Strategy**:
```rust
// Pseudocode for reachability analysis
fn analyze_reachability(cfg: &ControlFlowGraph) -> HashSet<NodeIndex> {
    let mut reachable = HashSet::new();
    let entry = cfg.get_entry_block_index().unwrap();
    
    let mut dfs = Dfs::new(&cfg.graph(), entry);
    while let Some(node) = dfs.next(&cfg.graph()) {
        reachable.insert(node);
    }
    
    reachable
}
```

**Complexity Analysis**:
- Time: O(V + E) where V = blocks, E = edges
- Space: O(V) for visited set
- Expected performance: <1ms for typical functions (<100 blocks)

**Edge Cases Handled**:
- Functions with no entry block (invalid CFG - error condition)
- Functions with single block (entry only - trivial, no removal)
- Loops and cycles (DFS handles naturally through visited tracking)
- Multiple unreachable components (all unmarked blocks removed)

**Alternative Approaches Considered**:

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| BFS Traversal | Processes blocks level-by-level | Higher memory usage O(width) | Rejected: no benefit for reachability |
| Dominator-based | More precise for some analyses | Overkill for simple reachability | Rejected: unnecessary complexity |
| Worklist Algorithm | Generic framework | More code for same result | Rejected: DFS is simpler |

### 2. Liveness Analysis (Dead Instruction Elimination)

**Objective**: Determine which computed values are used (live) and which are never used (dead), enabling removal of instructions that compute unused values.

**Algorithm Selection**: Backward dataflow analysis with fixed-point iteration

**Decision Rationale**:
- **Backward analysis**: Liveness is naturally a backward problem - a value is live if it's used in the future. Forward analysis would require complex prediction of future uses.
- **Fixed-point iteration**: Required because CFG may contain loops where liveness information propagates around cycles. Iteration continues until no changes occur (fixed-point reached).
- **Gen-Kill framework**: Standard dataflow approach where:
  - `gen[B]` = values used before being defined in block B
  - `kill[B]` = values defined in block B
  - `live_in[B] = gen[B] ∪ (live_out[B] - kill[B])`
  - `live_out[B] = ∪ live_in[S] for all successors S of B`

**Implementation Strategy**:
```rust
// Pseudocode for liveness analysis
fn analyze_liveness(function: &Function, cfg: &ControlFlowGraph) 
    -> HashMap<ValueId, LivenessInfo> {
    
    // Phase 1: Build def-use chains
    let def_use_chains = build_def_use_chains(function);
    
    // Phase 2: Compute gen/kill sets per block
    let (gen_sets, kill_sets) = compute_gen_kill_sets(function, cfg);
    
    // Phase 3: Fixed-point iteration
    let mut live_in = HashMap::new();
    let mut live_out = HashMap::new();
    let mut changed = true;
    
    while changed {
        changed = false;
        
        // Process blocks in reverse post-order (optimization)
        for block_idx in reverse_post_order(cfg) {
            // Compute live_out from successors
            let new_live_out: HashSet<ValueId> = cfg.successors(block_idx)
                .flat_map(|succ| &live_in[&succ])
                .copied()
                .collect();
            
            // Compute live_in = gen ∪ (live_out - kill)
            let new_live_in = &gen_sets[&block_idx] | 
                (&new_live_out - &kill_sets[&block_idx]);
            
            if new_live_in != live_in[&block_idx] {
                live_in.insert(block_idx, new_live_in);
                live_out.insert(block_idx, new_live_out);
                changed = true;
            }
        }
    }
    
    live_in
}
```

**Convergence Analysis**:
- **Guaranteed termination**: Liveness is a monotone framework (sets can only grow), and there's a finite upper bound (all values). Therefore, fixed-point is guaranteed in finite iterations.
- **Typical convergence**: Most functions reach fixed-point in 2-3 iterations
- **Worst case**: Functions with deep nesting and many loops may require O(depth) iterations
- **Maximum iteration limit**: 10 iterations with warning (indicates potential algorithm bug or pathological CFG)

**Reverse Post-Order Optimization**:
- Processing blocks in reverse post-order (from exits toward entry) minimizes iterations
- Information propagates backward more efficiently
- Obtained via `petgraph::algo::postorder()` or similar

**Phi Node Handling**:
Phi nodes require special treatment because they represent values that depend on the predecessor from which control flows:

```rust
// Phi node liveness: a phi is live if its result is used
// Each incoming value is considered used at the predecessor's exit
for phi in block.phi_nodes {
    if phi.result is live {
        for (value, pred_label) in phi.incoming {
            mark_live_at_exit_of(pred_label, value);
        }
    }
}
```

**Complexity Analysis**:
- Time: O(I × (V + E)) where I = iterations (typically 2-3), V = instructions, E = edges
- Space: O(V) for live sets per block
- Expected performance: <10ms for typical functions

**Alternative Approaches Considered**:

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| Forward dataflow | Simpler conceptually | Doesn't match liveness semantics | Rejected: backward is natural |
| SSA-based sparse analysis | Faster for large functions | Complex implementation | Deferred: future optimization |
| Use-def chains only | Simple for acyclic code | Incorrect for loops | Rejected: requires fixed-point |

### 3. Escape Analysis (Safe Memory Operation Removal)

**Objective**: Determine which allocated objects (allocas) have their addresses taken or may be accessed through pointers, enabling safe removal of stores to provably-local allocations.

**Algorithm Selection**: Flow-insensitive conservative escape analysis

**Decision Rationale**:
- **Flow-insensitive**: Analyzes entire function without considering execution order. Simpler and faster than flow-sensitive analysis, with acceptable precision for DCE purposes.
- **Conservative**: Over-approximates escaping (marks as escaped when uncertain). Ensures soundness - never removes an observable store.
- **Single-pass**: No iteration required since we're building a monotone summary (once escaped, always escaped).

**Escape Conditions**:
An alloca's address is considered "escaped" if:
1. Stored to memory: `store &alloca, *ptr` → pointer may be dereferenced elsewhere
2. Passed to function call: `call foo(&alloca)` → callee may store or alias it
3. Used in GetElementPtr with stored result: `gep &alloca → result stored` → computed address escapes
4. Returned from function: `ret &alloca` → caller receives pointer

An alloca is "local" (non-escaped) if:
- Only used in loads and stores: `load from &alloca`, `store value to &alloca`
- Address never flows to memory or function calls

**Implementation Strategy**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EscapeStatus {
    Local,        // Provably local, address never escapes
    AddressTaken, // Address computed (GEP) but not stored/passed
    Escaped,      // Address stored, passed to call, or returned
}

fn analyze_escape(function: &Function) -> HashMap<ValueId, EscapeStatus> {
    let mut escape_map = HashMap::new();
    
    // Initialize all allocas as Local
    for instruction in function.all_instructions() {
        if let InstructionKind::Alloca { .. } = instruction.kind {
            if let Some(result) = instruction.result {
                escape_map.insert(result.id, EscapeStatus::Local);
            }
        }
    }
    
    // Single pass: mark escapes
    for instruction in function.all_instructions() {
        match &instruction.kind {
            InstructionKind::GetElementPtr { base, .. } => {
                // GEP computes address - mark as AddressTaken
                if let Some(status) = escape_map.get_mut(&base.id) {
                    *status = EscapeStatus::AddressTaken.max(*status);
                }
            }
            InstructionKind::Store { value, dest } => {
                // If storing an alloca pointer, it escapes
                if escape_map.contains_key(&value.id) {
                    escape_map.insert(value.id, EscapeStatus::Escaped);
                }
            }
            InstructionKind::Call { args, .. } => {
                // If passing alloca to call, it escapes
                for arg in args {
                    if escape_map.contains_key(&arg.id) {
                        escape_map.insert(arg.id, EscapeStatus::Escaped);
                    }
                }
            }
            TerminatorKind::Return { value, .. } => {
                // If returning alloca pointer, it escapes
                if escape_map.contains_key(&value.id) {
                    escape_map.insert(value.id, EscapeStatus::Escaped);
                }
            }
            _ => {}
        }
    }
    
    escape_map
}
```

**Conservative Defaults**:
- Function parameters: assumed escaped (caller may alias)
- Loaded pointers: assumed escaped (may point to globals)
- Global variables: always escaped by definition

**Precision vs Performance Trade-off**:
- Flow-insensitive analysis: faster, less precise
- Flow-sensitive analysis: slower, more precise (tracks escape state per program point)
- **Decision**: Flow-insensitive sufficient for DCE - primary goal is safety, not maximum optimization

**Complexity Analysis**:
- Time: O(I) where I = total instructions (single pass)
- Space: O(A) where A = number of allocas
- Expected performance: <1ms for typical functions

**Alternative Approaches Considered**:

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| Flow-sensitive | More precise | Much slower, complex | Rejected: overkill for DCE |
| Andersen's points-to | Very precise alias info | Expensive (O(n³)) | Rejected: too slow |
| No escape analysis | Simplest | Can't remove any stores | Rejected: misses optimizations |

### 4. Side-Effect Classification

**Objective**: Categorize each instruction by its observable effects to determine removal safety.

**Classification Taxonomy**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SideEffectClass {
    Pure,         // No observable effects, removable if unused
    MemoryRead,   // Reads memory, removable if unused (unless volatile)
    MemoryWrite,  // Writes memory, conditional removal (check escape)
    EffectFul,    // Has side effects, never remove
}
```

**Classification Rules**:

| Instruction Kind | Side Effect Class | Removal Condition |
|------------------|-------------------|-------------------|
| Binary (add, sub, mul, etc.) | Pure | Result unused |
| Unary (neg, not) | Pure | Result unused |
| Cast | Pure | Result unused |
| GetElementPtr | Pure | Result unused (address computation only) |
| Load | MemoryRead | Result unused AND not volatile |
| Store | MemoryWrite | Target is non-escaped local AND no subsequent loads |
| Alloca | MemoryWrite | No stores/loads to it AND non-escaped |
| Call | EffectFul | Never remove (unless marked pure) |
| Phi | Pure | Result unused |
| Vector ops | Pure | Result unused |

**Implementation Strategy**:
```rust
fn classify_side_effects(instruction: &Instruction, escape_info: &HashMap<ValueId, EscapeStatus>) 
    -> SideEffectClass {
    
    match &instruction.kind {
        InstructionKind::Binary { .. } 
        | InstructionKind::Unary { .. }
        | InstructionKind::Cast { .. }
        | InstructionKind::GetElementPtr { .. }
        | InstructionKind::Phi { .. }
        | InstructionKind::Vector { .. } => SideEffectClass::Pure,
        
        InstructionKind::Load { .. } => {
            // Future: check volatile/atomic attributes
            SideEffectClass::MemoryRead
        }
        
        InstructionKind::Store { dest, .. } => {
            // Check if store is to non-escaped local
            if matches!(escape_info.get(&dest.id), Some(EscapeStatus::Local)) {
                SideEffectClass::MemoryWrite
            } else {
                SideEffectClass::EffectFul // Conservative: may be observable
            }
        }
        
        InstructionKind::Alloca { .. } => SideEffectClass::MemoryWrite,
        
        InstructionKind::Call { .. } => {
            // Future: check for pure/readonly function attributes
            SideEffectClass::EffectFul
        }
    }
}
```

**Conservative Decision Points**:

1. **Unknown function purity**: All calls assumed effectful unless annotated
   - **Rationale**: Function may perform I/O, modify globals, or have other side effects
   - **Future enhancement**: Function attribute system (e.g., `#[pure]`, `#[readonly]`)

2. **Potential aliasing**: Stores through pointers assumed observable
   - **Rationale**: Without precise alias analysis, may affect reachable memory
   - **Mitigation**: Escape analysis provides partial information

3. **Volatile/atomic operations**: Currently all loads treated as potentially volatile
   - **Rationale**: IR doesn't yet represent volatile/atomic attributes
   - **Future enhancement**: Extend IR to mark volatile loads/stores

**Complexity Analysis**:
- Time: O(1) per instruction
- Space: O(1) per classification (can be computed on-demand)
- Expected performance: negligible (simple pattern matching)

### 5. Fixed-Point Iteration Strategy

**Objective**: Repeatedly apply removal until no further changes occur, handling cascading dead code where removing one instruction makes others dead.

**Algorithm Selection**: Outer fixed-point loop with reachability + liveness + removal in each iteration

**Decision Rationale**:
- **Cascading opportunities**: Removing an instruction may make its operands unused (dead), or may make blocks unreachable (empty blocks)
- **Fixed-point guarantee**: Each iteration can only remove code (monotone), and there's a finite lower bound (empty function), so convergence is guaranteed
- **Separate iterations**: Keep reachability, liveness, and removal as separate phases within each iteration for clarity and testability

**Implementation Strategy**:
```rust
fn run_dce_optimization(function: &mut Function) -> OptimizationStats {
    let mut stats = OptimizationStats::default();
    const MAX_ITERATIONS: usize = 10;
    
    for iteration in 0..MAX_ITERATIONS {
        let mut changed = false;
        
        // Step 1: Remove unreachable blocks
        let reachable_blocks = analyze_reachability(&function.cfg);
        let blocks_removed = remove_unreachable_blocks(function, &reachable_blocks);
        changed |= blocks_removed > 0;
        stats.blocks_removed += blocks_removed;
        
        // Step 2: Analyze liveness
        let live_values = analyze_liveness(function);
        
        // Step 3: Perform escape analysis
        let escape_info = analyze_escape(function);
        
        // Step 4: Remove dead instructions
        let instructions_removed = remove_dead_instructions(
            function, 
            &live_values, 
            &escape_info
        );
        changed |= instructions_removed > 0;
        stats.instructions_removed += instructions_removed;
        
        stats.iterations = iteration + 1;
        
        if !changed {
            break; // Fixed-point reached
        }
        
        if iteration == MAX_ITERATIONS - 1 {
            eprintln!("⚠ Warning: DCE reached maximum iterations for function '{}'", 
                function.name);
        }
    }
    
    stats
}
```

**Convergence Properties**:
- **Monotonicity**: Code size can only decrease
- **Termination**: Guaranteed within finite iterations (code is finite)
- **Typical convergence**: 1-2 iterations for most functions
- **Cascading example**: 
  - Iteration 1: Remove unused computation → makes input values unused
  - Iteration 2: Remove now-unused inputs → function unchanged
  - Fixed-point reached

**Incremental Update Optimization** (future enhancement):
Instead of full reanalysis each iteration, track which blocks were affected:
```rust
// Track affected blocks
let mut affected_blocks = HashSet::new();

// After removing instruction, mark containing block as affected
affected_blocks.insert(block_idx);

// In next iteration, only reanalyze affected blocks
// (requires more complex bookkeeping but faster for large functions)
```

**Complexity Analysis**:
- Time: O(K × (V + E + I)) where K = iterations (1-3 typical, 10 max)
- Space: O(V + E) for analysis structures (reused across iterations)
- Expected performance: <20ms for typical functions over all iterations

## Integration with Existing Infrastructure

### Phase Trait Implementation

The DCE optimizer implements the existing `Phase` trait defined in `src/ir/optimizer/phase.rs`:

```rust
pub trait Phase {
    fn name(&self) -> &'static str;
    fn run(&mut self, ir: &mut Module);
}
```

**Implementation**:
```rust
pub struct DeadCodeElimination {
    // Configuration options (if any)
    max_iterations: usize,
    enable_statistics: bool,
}

impl Phase for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "Dead Code Elimination"
    }
    
    fn run(&mut self, module: &mut Module) {
        for function in &mut module.functions {
            // Skip declarations (no body)
            if function.cfg.blocks().count() == 0 {
                continue;
            }
            
            let stats = self.optimize_function(function);
            
            if self.enable_statistics {
                print_optimization_stats(&function.name, &stats);
            }
            
            // Verify CFG integrity
            if let Err(e) = function.verify() {
                eprintln!("⚠ Warning: CFG verification failed after DCE for '{}': {}", 
                    function.name, e);
            }
        }
    }
}
```

### CFG Modification Operations

**Block Removal**:
```rust
fn remove_unreachable_blocks(function: &mut Function, reachable: &HashSet<NodeIndex>) 
    -> usize {
    let unreachable: Vec<NodeIndex> = function.cfg.graph()
        .node_indices()
        .filter(|idx| !reachable.contains(idx))
        .collect();
    
    let count = unreachable.len();
    
    for block_idx in unreachable {
        // Remove edges to this block
        for pred_idx in function.cfg.graph().neighbors_directed(block_idx, petgraph::Incoming) {
            function.cfg.graph_mut().remove_edge(pred_idx, block_idx);
        }
        
        // Remove edges from this block
        for succ_idx in function.cfg.graph().neighbors_directed(block_idx, petgraph::Outgoing) {
            function.cfg.graph_mut().remove_edge(block_idx, succ_idx);
            
            // Update phi nodes in successor
            update_phi_nodes_after_predecessor_removal(function, succ_idx, block_idx);
        }
        
        // Remove the block itself
        function.cfg.graph_mut().remove_node(block_idx);
    }
    
    count
}
```

**Instruction Removal**:
```rust
fn remove_dead_instructions(
    function: &mut Function,
    live_values: &HashMap<ValueId, LivenessInfo>,
    escape_info: &HashMap<ValueId, EscapeStatus>
) -> usize {
    let mut removed_count = 0;
    
    for block in function.cfg.blocks_mut() {
        let original_len = block.instructions.len();
        
        block.instructions.retain(|inst| {
            let is_live = match &inst.result {
                Some(result) => live_values.contains_key(&result.id),
                None => true, // Instructions without results (e.g., store) checked separately
            };
            
            let side_effect_class = classify_side_effects(inst, escape_info);
            
            let should_keep = match side_effect_class {
                SideEffectClass::Pure => is_live,
                SideEffectClass::MemoryRead => is_live, // Could remove unused loads
                SideEffectClass::MemoryWrite => true,   // Conservative: keep stores
                SideEffectClass::EffectFul => true,     // Always keep
            };
            
            should_keep
        });
        
        removed_count += original_len - block.instructions.len();
    }
    
    removed_count
}
```

**Phi Node Updates**:
```rust
fn update_phi_nodes_after_predecessor_removal(
    function: &mut Function, 
    block_idx: NodeIndex,
    removed_pred_idx: NodeIndex
) {
    let removed_label = &function.cfg.graph()[removed_pred_idx].label;
    
    if let Some(block) = function.cfg.graph_mut().node_weight_mut(block_idx) {
        for inst in &mut block.instructions {
            if let InstructionKind::Phi { incoming, .. } = &mut inst.kind {
                incoming.retain(|(_, pred_label)| pred_label != removed_label.as_ref());
            }
        }
    }
}
```

### Validation and Diagnostics

**Post-Optimization Verification**:
After optimization, call existing verification methods:
```rust
// Check CFG structural integrity
function.verify()?;

// Optional: Re-run SSA verification if available
// This ensures no undefined value uses were introduced
if let Some(ssa_verifier) = ssa_verification_available() {
    ssa_verifier.verify(function)?;
}
```

**Statistics Reporting**:
```rust
struct OptimizationStats {
    instructions_removed: usize,
    blocks_removed: usize,
    iterations: usize,
    conservative_warnings: Vec<ConservativeWarning>,
}

struct ConservativeWarning {
    instruction_debug: String,
    reason: ConservativeReason,
}

enum ConservativeReason {
    MayAlias,
    UnknownCallPurity,
    EscapedPointer,
    PotentialSideEffect,
}

fn print_optimization_stats(function_name: &str, stats: &OptimizationStats) {
    println!("📊 DCE Statistics for function '{}':", function_name);
    println!("  ✂️  Instructions removed: {}", stats.instructions_removed);
    println!("  🗑️  Blocks removed: {}", stats.blocks_removed);
    println!("  🔄 Iterations: {}", stats.iterations);
    
    if !stats.conservative_warnings.is_empty() {
        println!("  ⚠️  Conservative decisions: {}", stats.conservative_warnings.len());
        for warning in &stats.conservative_warnings {
            println!("    - {} (reason: {:?})", warning.instruction_debug, warning.reason);
        }
    }
}
```

## Performance Considerations

### Expected Performance Characteristics

Based on algorithmic complexity analysis:

| Function Size | Instructions | Blocks | Expected DCE Time | Bottleneck |
|---------------|--------------|--------|-------------------|------------|
| Small | <100 | <10 | <1ms | Negligible |
| Medium | 100-1000 | 10-100 | 1-10ms | Liveness analysis |
| Large | 1000-10000 | 100-500 | 10-100ms | Fixed-point iteration |
| Very Large | >10000 | >500 | 100-1000ms | All phases |

### Optimization Opportunities

1. **Sparse Analysis** (future): Instead of computing liveness for all values, track only SSA names, reducing dataflow state size
2. **Incremental Updates**: Track modified blocks between iterations, only reanalyze affected regions
3. **Parallel Function Processing**: Multiple functions can be optimized in parallel (module-level parallelism)
4. **Def-Use Chain Caching**: Build once, incrementally update, avoid full rebuild each iteration

### Memory Usage

| Data Structure | Size | Lifetime |
|----------------|------|----------|
| Reachability set | O(blocks) | Per iteration |
| Liveness info | O(values) | Per iteration |
| Escape analysis | O(allocas) | Per iteration |
| Def-use chains | O(instructions + uses) | Entire optimization |

**Peak memory**: O(instructions + values + blocks) - dominated by def-use chains

## Testing Strategy

### Unit Tests

**Reachability Analysis Tests** (`ir_dce_reachability_tests.rs`):
- Test function with single block (trivial case)
- Test function with linear CFG (all reachable)
- Test function with unreachable blocks after return
- Test function with unreachable blocks from impossible branch
- Test function with loops (ensure loop blocks are reachable)
- Test function with multiple unreachable components

**Liveness Analysis Tests** (`ir_dce_liveness_tests.rs`):
- Test single use of value (live)
- Test unused temporary (dead)
- Test value used in multiple blocks (live across blocks)
- Test phi node with all incoming values (all live if phi is live)
- Test phi node with unused result (all incoming values dead)
- Test value in loop (liveness propagates around loop)

**Escape Analysis Tests** (`ir_dce_escape_tests.rs`):
- Test alloca only used in load/store (local)
- Test alloca address taken by GEP (address taken)
- Test alloca passed to function call (escaped)
- Test alloca stored to memory (escaped)
- Test alloca returned from function (escaped)

### Integration Tests

**End-to-End Optimization Tests** (`ir_dce_integration_tests.rs`):
- Test removal of code after unconditional return
- Test removal of impossible branch
- Test removal of unused computation chain
- Test preservation of effectful operations (calls)
- Test preservation of live instructions
- Test cascading dead code elimination (multiple iterations)
- Test fixed-point convergence (no changes after optimization)

### Snapshot Tests

**IR Output Validation** (`ir_dce_snapshot_tests.rs`):
Using insta for snapshot testing:
```rust
#[test]
fn test_dce_removes_unreachable_after_return() {
    let mut module = create_test_module_with_dead_code();
    let mut dce = DeadCodeElimination::default();
    dce.run(&mut module);
    
    insta::assert_snapshot!(module.to_string());
}
```

Snapshot captures:
- Complete IR before optimization
- Complete IR after optimization
- Diff highlighting exactly what was removed

### Property-Based Tests

Using `proptest` or similar (future):
- **Property**: Optimization preserves observable behavior
- **Property**: Optimization is idempotent (running twice produces same result as once)
- **Property**: CFG remains valid after optimization
- **Property**: SSA form is preserved (no undefined uses)

## Risks and Mitigations

### Risk 1: Incorrect Aliasing Assumptions

**Risk**: Conservative alias analysis may miss optimization opportunities or (worse) incorrectly assume non-aliasing and remove observable stores.

**Mitigation**:
- Default to conservative (assume aliasing)
- Only optimize provably-local allocas
- Extensive testing with aliasing scenarios
- Future: implement more precise alias analysis (e.g., Steensgaard's or Andersen's)

**Impact**: Medium - primarily affects optimization effectiveness, not correctness (due to conservative defaults)

### Risk 2: CFG Corruption

**Risk**: Incorrect edge removal or block deletion could leave dangling references or invalid phi nodes.

**Mitigation**:
- Always verify CFG after optimization using `function.verify()`
- Systematic phi node updates when removing predecessors
- Test suite includes CFG validation for all transformations
- Petgraph provides safe APIs that prevent some corruption patterns

**Impact**: High - could cause compiler crashes or incorrect code generation

**Mitigation Priority**: High - must be rock-solid

### Risk 3: Performance Regression

**Risk**: Slow optimization could increase overall compilation time unacceptably.

**Mitigation**:
- Benchmark on representative real-world code
- Set iteration limits (10 max)
- Profile to identify bottlenecks
- Incremental optimization strategies for large functions
- Consider making DCE optional (controlled by optimization level)

**Impact**: Medium - affects developer experience but not correctness

### Risk 4: SSA Form Violation

**Risk**: Removing definitions without removing corresponding uses creates undefined values.

**Mitigation**:
- Liveness analysis explicitly tracks uses
- Only remove instructions whose results are unused
- Post-optimization SSA verification (if available)
- Test suite includes SSA validation

**Impact**: Critical - would generate incorrect code

**Mitigation Priority**: Critical - must be prevented

### Risk 5: Debug Information Loss

**Risk**: Removing instructions may lose important debug metadata (source locations, variable names).

**Mitigation**:
- Preserve `debug_info` fields on remaining instructions
- Consider logging removed instructions with debug info in diagnostics
- Future: more sophisticated debug info preservation strategies

**Impact**: Low-Medium - affects debugging but not correctness

## Future Enhancements

### Phase 1 Follow-Up (After Initial Implementation)

1. **Function Attribute System**:
   - Mark pure functions: `#[pure]` - no side effects, removable if unused
   - Mark readonly functions: `#[readonly]` - reads memory but doesn't modify
   - Enables more aggressive call elimination

2. **Volatile/Atomic Support**:
   - Extend IR to represent volatile loads/stores
   - Extend IR to represent atomic operations
   - Ensure volatile/atomic operations are never removed

3. **Incremental Analysis**:
   - Track which blocks changed between iterations
   - Only reanalyze affected blocks
   - Significantly faster for large functions

### Phase 2: Advanced Optimizations

1. **Sparse Liveness Analysis**:
   - SSA-based approach: track only SSA names, not all values
   - Faster for large functions (fewer dataflow variables)
   - More complex implementation

2. **Interprocedural DCE**:
   - Analyze across function boundaries
   - Remove unused private functions
   - Remove unused parameters (signature optimization)

3. **Precise Alias Analysis**:
   - Andersen's or Steensgaard's points-to analysis
   - More aggressive store removal
   - Significantly more complex

4. **Loop-Aware Optimization**:
   - Detect loop-invariant computations
   - Hoist or remove invariant dead code
   - Requires loop detection infrastructure

## Conclusion

The proposed Dead Code Elimination implementation uses well-established compiler optimization techniques adapted for the jsavrs compiler's IR structure. The design prioritizes:

1. **Correctness**: Conservative analysis ensures no observable behavior changes
2. **Simplicity**: Standard algorithms (DFS, backward dataflow) with clear implementation
3. **Performance**: O(V+E) complexity for typical functions, fast convergence (2-3 iterations)
4. **Extensibility**: Modular design allows future enhancements (better alias analysis, interprocedural optimization)
5. **Integration**: Seamless integration with existing Phase trait and IR infrastructure

Key decisions:
- ✅ DFS for reachability (simple, efficient)
- ✅ Backward dataflow for liveness (natural fit)
- ✅ Flow-insensitive escape analysis (sufficient precision, fast)
- ✅ Conservative side-effect classification (safe defaults)
- ✅ Fixed-point iteration (handles cascading opportunities)

The implementation is ready to proceed to Phase 1 (Design & Contracts) where detailed data structures and APIs will be specified.
