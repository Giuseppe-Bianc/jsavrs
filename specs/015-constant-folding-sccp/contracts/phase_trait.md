# API Contract: Phase Trait Implementation

**Module**: `src/ir/optimizer/constant_folding/optimizer.rs`  
**Purpose**: Define the contract for integrating the constant folding optimizer with the IR pipeline

---

## Interface Definition

### Phase Trait (from `src/ir/optimizer/phase.rs`)

The constant folding optimizer must implement the existing `Phase` trait:

```rust
/// Trait for compiler optimization phases that transform IR modules.
pub trait Phase {
    /// Returns the human-readable name of this optimization phase.
    fn name(&self) -> &'static str;
    
    /// Executes the optimization phase on the given IR module.
    /// 
    /// # Arguments
    /// * `module` - Mutable reference to the IR module to optimize
    /// 
    /// # Behavior
    /// - Must preserve SSA form throughout all transformations
    /// - Must maintain CFG validity after removing blocks
    /// - Must not change program semantics (semantic preservation)
    /// - May emit diagnostic messages to stderr
    /// - Should report instruction count to stdout after completion
    fn run(&mut self, module: &mut Module);
}
```

---

## Implementation Contract

### ConstantFoldingOptimizer

**Struct Definition**:
```rust
#[derive(Debug)]
pub struct ConstantFoldingOptimizer {
    pub verbose: bool,
    pub sccp: bool,
    statistics: AggregateStatistics,
}
```

**Required Methods**:

#### 1. Constructor

```rust
impl ConstantFoldingOptimizer {
    /// Creates a new constant folding optimizer with specified configuration.
    /// 
    /// # Arguments
    /// * `verbose` - Enable detailed statistics output to stderr
    /// * `sccp` - Enable SCCP mode with control flow analysis
    /// 
    /// # Returns
    /// New optimizer instance with empty statistics
    /// 
    /// # Examples
    /// ```
    /// let optimizer = ConstantFoldingOptimizer::new(true, true);
    /// ```
    pub fn new(verbose: bool, sccp: bool) -> Self;
}
```

**Preconditions**: None  
**Postconditions**: Returns valid optimizer with initialized statistics  
**Complexity**: O(1)

#### 2. Phase Trait Implementation

```rust
impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &'static str {
        "Constant Folding Optimizer"
    }
    
    fn run(&mut self, module: &mut Module);
}
```

**`run()` Method Contract**:

**Preconditions**:
- `module` contains valid IR (well-formed SSA, valid CFG)
- All SSA value references are valid
- All basic block references are valid

**Postconditions**:
- SSA form preserved (each value still has exactly one definition)
- CFG validity maintained (no dangling block references)
- Semantics preserved (optimized program produces identical output)
- `self.statistics` updated with aggregate metrics
- Total instruction count printed to stdout
- If `self.verbose`, detailed statistics printed to stderr

**Behavior**:
1. Iterate over all functions in module
2. For each function:
   - Count initial instructions
   - Run basic constant folding pass (always)
   - If `self.sccp`, run SCCP analysis and transformation
   - Run CFG cleanup pass (always)
   - Update function metrics
   - Accumulate metrics into `self.statistics`
3. Print aggregate statistics if `self.verbose`
4. Print total instruction count to stdout

**Error Handling**:
- Invalid SSA reference → emit warning to stderr, preserve instruction, continue
- Missing CFG information → emit warning to stderr, skip CFG optimizations, continue
- SCCP memory limit exceeded → emit warning to stderr, fall back to basic folding, continue
- Never panics in release builds

**Performance Guarantees**:
- Time complexity: O(n) where n = total instruction count in module
- Space complexity: O(f) where f = max function size, bounded to ~130KB per function
- Processes 1000+ instruction functions in <1 second

**Complexity**: O(n) where n = number of instructions in module

---

## Usage Examples

### Example 1: Basic Usage

```rust
use jsavrs::ir::optimizer::constant_folding::ConstantFoldingOptimizer;
use jsavrs::ir::optimizer::Phase;
use jsavrs::ir::Module;

fn optimize_module(mut module: Module) -> Module {
    let mut optimizer = ConstantFoldingOptimizer::new(false, false);
    optimizer.run(&mut module);
    module
}
```

### Example 2: Verbose SCCP Mode

```rust
let mut optimizer = ConstantFoldingOptimizer::new(true, true);
optimizer.run(&mut module);
// Stderr output:
// === Constant Folding Optimizer Statistics ===
// Functions processed: 42
// Instructions removed: 387
// ...
// Stdout output:
// total number of instructions after constant folding: 1234
```

### Example 3: Integration in Optimization Pipeline

```rust
fn run_optimization_pipeline(mut module: Module) -> Module {
    // Phase 1: Constant folding
    let mut const_fold = ConstantFoldingOptimizer::new(false, true);
    const_fold.run(&mut module);
    
    // Phase 2: Dead code elimination
    let mut dce = DeadCodeElimination::new();
    dce.run(&mut module);
    
    // Phase 3: Second constant folding pass (propagate DCE results)
    let mut const_fold2 = ConstantFoldingOptimizer::new(false, true);
    const_fold2.run(&mut module);
    
    module
}
```

---

## Internal Method Contracts

### `optimize_function`

```rust
fn optimize_function(
    function: &mut Function,
    sccp_enabled: bool,
    verbose: bool
) -> FunctionMetrics;
```

**Preconditions**:
- `function` is well-formed IR function

**Postconditions**:
- `function` transformed in-place with optimizations applied
- Returns metrics for this function
- SSA form and CFG validity preserved

**Behavior**:
1. Create `FunctionMetrics` and record initial state
2. Run basic constant folding
3. If `sccp_enabled`, run SCCP analysis and transformation
4. Run CFG cleanup
5. Update metrics with final state
6. If `verbose`, print function-specific diagnostics
7. Return metrics

**Complexity**: O(n) where n = instructions in function

---

### `basic_constant_fold`

```rust
fn basic_constant_fold(function: &mut Function) -> usize;
```

**Preconditions**:
- `function` is well-formed

**Postconditions**:
- Instructions with constant operands replaced with constant results
- Returns count of folded instructions

**Behavior**:
- Single pass over all instructions
- Evaluate each instruction using `evaluator::fold_instruction()`
- Replace foldable instructions with constant assignments
- Preserve source spans and debug info

**Complexity**: O(n) where n = instructions in function

---

### `sccp_optimize`

```rust
fn sccp_optimize(function: &mut Function) -> Result<(usize, usize), SCCPError>;
```

**Preconditions**:
- `function` has valid CFG
- `function` is well-formed SSA

**Postconditions**:
- Returns Ok((folded_count, removed_blocks)) on success
- Returns Err(SCCPError::MemoryLimit) if lattice exceeds 100KB
- Function transformed with constant propagation and branch resolution
- Unreachable blocks marked for removal (but not yet removed)

**Behavior**:
1. Create `SCCPContext`
2. Run worklist algorithm until fixed point
3. Check memory limit; return error if exceeded
4. Transform instructions based on final lattice values
5. Mark unreachable blocks
6. Return counts

**Complexity**: O(n + e) where n = instructions, e = CFG edges

---

### `cfg_cleanup`

```rust
fn cfg_cleanup(function: &mut Function, reachable_blocks: &HashSet<NodeIndex>) -> usize;
```

**Preconditions**:
- `reachable_blocks` contains all blocks reachable from entry
- Entry block is in `reachable_blocks`

**Postconditions**:
- Unreachable blocks removed from function
- Phi nodes updated to remove unreachable incoming edges
- CFG validity restored
- Returns count of removed blocks

**Behavior**:
1. Remove unreachable incoming edges from all phi nodes
2. Remove unreachable blocks from function
3. Recompute CFG traversal order
4. Return removal count

**Complexity**: O(n + b) where n = instructions, b = blocks

---

## Validation and Testing

### Contract Validation

The implementation must pass the following validation tests:

1. **SSA Preservation Test**: After optimization, verify each value has exactly one definition
2. **CFG Validity Test**: After optimization, verify all block references are valid
3. **Semantic Preservation Test**: Execute before/after, verify identical output
4. **Memory Limit Test**: Large functions trigger fallback without crashing
5. **Idempotence Test**: Running optimizer twice produces same result as once

### Performance Contract Validation

Benchmark suite must verify:
- Functions with 1000 instructions optimize in <1 second
- Memory usage bounded to ~130KB per function
- No memory leaks (all allocations freed after processing)

---

## Error Codes

```rust
#[derive(Debug, thiserror::Error)]
pub enum SCCPError {
    #[error("SCCP lattice memory limit (100KB) exceeded")]
    MemoryLimit,
}
```

**Handling**: When `sccp_optimize()` returns `Err(SCCPError::MemoryLimit)`, the optimizer should:
1. Emit warning to stderr
2. Mark `sccp_fallback` in metrics
3. Continue with basic constant folding
4. Not re-attempt SCCP for this function

---

## Thread Safety

**Not thread-safe**: `ConstantFoldingOptimizer` is designed for single-threaded use. Multiple optimizers can be created for parallel function processing, but each optimizer instance must not be shared across threads without synchronization.

**Rationale**: Mutable statistics accumulation requires exclusive access. For parallel optimization, create separate optimizer instances per thread and merge statistics afterward.

---

## Deprecation Policy

This contract is version 1.0 for the initial release. Breaking changes will:
1. Increment major version
2. Provide migration guide
3. Maintain backward compatibility for at least one minor version

---

## Summary

| Aspect | Requirement |
|--------|-------------|
| Trait | Phase |
| Methods | name(), run() |
| Preconditions | Valid SSA, valid CFG |
| Postconditions | SSA preserved, CFG valid, semantics preserved |
| Performance | O(n) time, O(f) space bounded to 130KB |
| Error Handling | Conservative fallback, no panics |
| Thread Safety | Not thread-safe (single instance per thread) |
