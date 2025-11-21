# API Contract: SCCP Optimizer Phase

**Module**: `src/ir/optimizer/sccp`  
**Version**: 1.0.0  
**Date**: 2025-11-19

## Overview

This document defines the public API contract for the Sparse Conditional Constant Propagation (SCCP) optimization phase. The API is designed for integration with the jsavrs compiler's optimization pipeline through the Phase trait.

## Public Interface

### Phase Implementation

```rust
/// Sparse Conditional Constant Propagation optimizer
///
/// Discovers compile-time constant values and unreachable code paths
/// through simultaneous dataflow analysis of values and control flow.
pub struct SccpOptimizer {
    /// Enable verbose logging of optimization actions
    pub verbose: bool,
    
    /// Maximum analysis iterations before conservative fallback
    pub max_iterations: usize,
    
    /// Master enable/disable switch
    pub enabled: bool,
}

impl SccpOptimizer {
    /// Creates a new SCCP optimizer with default settings
    ///
    /// # Default Configuration
    /// - verbose: false
    /// - max_iterations: 10,000
    /// - enabled: true
    pub fn new() -> Self;
    
    /// Enables verbose logging output
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_verbose(self) -> Self;
    
    /// Sets maximum analysis iterations
    ///
    /// # Parameters
    /// - max: Maximum iteration count (must be > 0)
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Panics
    /// If max == 0
    pub fn with_max_iterations(self, max: usize) -> Self;
    
    /// Disables the optimizer
    ///
    /// # Returns
    /// Self for method chaining
    pub fn disabled(self) -> Self;
}

impl Phase for SccpOptimizer {
    fn name(&self) -> &'static str;
    
    /// Runs SCCP optimization on all functions in the module
    ///
    /// # Parameters
    /// - ir: Mutable reference to IR module
    ///
    /// # Returns
    /// - true: IR was modified (constants propagated, branches simplified, etc.)
    /// - false: No changes made (already optimal or disabled)
    ///
    /// # Side Effects
    /// - Mutates IR in-place (replaces uses, simplifies branches, marks dead code)
    /// - Prints statistics to stdout if verbose == true
    ///
    /// # Guarantees
    /// - Preserves program semantics (optimizations are sound)
    /// - Maintains SSA form validity
    /// - Leaves IR in consistent state even if iteration limit exceeded
    fn run(&mut self, ir: &mut Module) -> bool;
}
```

**Pre-conditions**:
- `ir` must be valid SSA-form IR
- Functions must have proper CFG structure
- Entry blocks must be valid and reachable

**Post-conditions**:
- IR remains in valid SSA form
- All constant-valued SSA uses replaced with literals
- Constant-condition branches simplified to unconditional
- Dead code marked for DCE removal
- No semantic changes to program behavior

**Error Handling**:
- Invalid IR → conservative analysis (may miss optimizations)
- Iteration limit exceeded → degrade remaining Top values to Bottom (conservative)
- Never panics (graceful degradation)

---

### Statistics and Observability

```rust
/// Statistics from SCCP transformation
#[derive(Debug, Clone, Default)]
pub struct SccpStats {
    /// Number of SSA values replaced with constant literals
    pub constants_propagated: usize,
    
    /// Number of instructions marked dead
    pub instructions_marked_dead: usize,
    
    /// Number of conditional branches simplified
    pub branches_simplified: usize,
    
    /// Number of phi nodes cleaned
    pub phi_nodes_cleaned: usize,
    
    /// Number of phi nodes fully replaced
    pub phi_nodes_replaced: usize,
    
    /// Number of unreachable blocks marked
    pub unreachable_blocks: usize,
    
    /// Analysis iterations performed
    pub iterations: usize,
    
    /// Whether analysis converged naturally
    pub converged: bool,
}
```

**Access**: Internal use only (printed if verbose mode enabled)

**Logging Output** (when verbose == true):
```
SCCP: Propagated 42 constants
SCCP: Simplified 7 branches
SCCP: Marked 3 blocks unreachable
SCCP: Converged in 12 iterations
```

---

## Integration Patterns

### Standalone Usage

```rust
use jsavrs::ir::optimizer::sccp::SccpOptimizer;
use jsavrs::ir::optimizer::Phase;

let mut module = /* load IR */;
let mut sccp = SccpOptimizer::new().with_verbose();

if sccp.run(&mut module) {
    println!("SCCP made optimizations");
}
```

### Pipeline Integration (Recommended)

```rust
use jsavrs::ir::optimizer::{
    phase::run_pipeline,
    sccp::SccpOptimizer,
    dead_code_elimination::DeadCodeElimination,
};

fn optimize_module(module: &mut Module) {
    let mut changed = true;
    let mut iterations = 0;
    
    while changed && iterations < 3 {
        changed = false;
        iterations += 1;
        
        // Alternating SCCP and DCE until fixed-point
        let mut sccp = SccpOptimizer::new();
        changed |= sccp.run(module);
        
        let mut dce = DeadCodeElimination::new();
        changed |= dce.run(module);
    }
}
```

**Rationale**: SCCP discovers dead code, DCE removes it, enabling further SCCP optimizations.

---

## Behavioral Contracts

### Constant Folding

**Contract**: All compile-time evaluable expressions are folded to constants

**Supported Operations**:
- Arithmetic: `+`, `-`, `*`, `/`, `%` (wrapping semantics)
- Bitwise: `&`, `|`, `^`, `<<`, `>>` (wrapping shifts)
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`
- Unary: negation, logical NOT

**Supported Types**: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char

**Conservative Cases** (return Bottom):
- Function calls (no interprocedural analysis)
- Memory loads (no alias analysis)
- Division by zero
- Type mismatches
- String, array, pointer types

**Example**:
```rust
// Input IR:
x = 5
y = 3
z = x + y

// Output IR (after SCCP):
z = 8  // x and y uses replaced, x=5 and y=3 marked dead
```

---

### Branch Simplification

**Contract**: All constant-condition branches converted to unconditional jumps

**Example**:
```rust
// Input IR:
x = 42
if (x > 40) {  // Constant condition: true
    A
} else {
    B
}

// Output IR (after SCCP):
x = 42  // (marked dead)
goto A  // Unconditional branch
// Block B marked unreachable
```

---

### Phi Node Resolution

**Contract**: Phi nodes with identical constant values from all executable predecessors resolve to that constant

**Example**:
```rust
// Input IR:
if (true) {  // Constant condition
    x1 = 5
} else {
    x2 = 5  // Unreachable
}
x3 = phi [x1, true_block], [x2, false_block]

// Output IR (after SCCP):
goto true_block
x1 = 5
x3 = 5  // Phi resolved to constant, phi instruction marked dead
```

---

### Unreachable Code Marking

**Contract**: All blocks not reachable through executable CFG paths are marked for DCE

**Note**: SCCP does not physically remove blocks; it marks them unreachable for DCE to remove.

---

## Performance Guarantees

**Time Complexity**: O(V + E) where V = SSA values, E = CFG edges
- Each SSA value changes at most twice (Top → Constant → Bottom)
- Each CFG edge processed at most once when marked executable

**Space Complexity**: O(V + E)
- HashMap for lattice values: O(V)
- HashSet for executable blocks: O(B) where B = blocks
- Worklists with deduplication: O(V + E)

**Benchmarks** (target performance on standard development hardware):
- 1,000 instruction function: <10ms
- 10,000 instruction function: <100ms
- Convergence: 1-3 iterations for typical functions

---

## Error Conditions and Handling

| Condition | Behavior | Recovery |
|-----------|----------|----------|
| Iteration limit exceeded | Degrade remaining Top to Bottom | Conservative analysis completes |
| Invalid IR (unreachable entry) | Log warning, force entry executable | Continue analysis |
| Type mismatch in folding | Return None → Bottom | Conservative for that value |
| Division by zero | Return None → Bottom | Conservative for that value |

**Guarantee**: Never panics, always produces valid (though possibly suboptimal) output

---

## Versioning and Compatibility

**Semantic Versioning**: Major.Minor.Patch

**Current Version**: 1.0.0

**Breaking Changes** (major version bump):
- Changes to `SccpOptimizer` public fields
- Removal of public methods
- Changes to Phase trait return value semantics

**Non-Breaking Changes** (minor version bump):
- Addition of new public methods
- New configuration options (with defaults)
- Performance improvements

**Patch Changes**:
- Bug fixes
- Documentation updates

---

## Testing Contract

### Required Test Coverage

**Unit Tests**:
- Lattice operations (commutativity, associativity, idempotence)
- Constant folding for all operation types
- Worklist management (deduplication, ordering)

**Integration Tests**:
- End-to-end constant propagation
- Branch simplification
- Phi node resolution
- Unreachable code detection

**Snapshot Tests**:
- Before/after IR comparison for representative programs
- Regression detection for optimization quality

**Performance Tests**:
- Benchmark compliance with O(V+E) complexity
- Latency targets for various function sizes

### Test Invariants

- SCCP output always produces semantically equivalent program
- SSA form preserved after transformation
- CFG structure remains valid
- Fixed-point: running SCCP again produces no changes

---

## Dependencies

**Required**:
- `petgraph = "0.8.3"` (already present in Cargo.toml)
- `thiserror = "2.0.17"` (already present for error handling)
- Existing IR types: `Value`, `IrLiteralValue`, `IrType`, `Instruction`, `Function`, `Module`

**No New Dependencies**: Implementation uses only existing project dependencies

---

## Migration Guide

### For Compiler Pipeline Authors

**Before** (without SCCP):
```rust
run_pipeline(module, vec![
    Box::new(DeadCodeElimination::new()),
]);
```

**After** (with SCCP):
```rust
// Recommended: alternating SCCP+DCE
let mut changed = true;
while changed {
    changed = false;
    changed |= SccpOptimizer::new().run(module);
    changed |= DeadCodeElimination::new().run(module);
}
```

### For Downstream Optimizations

**Expectations**: After SCCP runs:
- All easily discoverable constants are propagated
- Unreachable blocks are marked (but not removed until DCE)
- Branch conditions may be unconditional jumps
- Phi nodes may have reduced predecessors or be constant

**Recommendations**:
- Run DCE after SCCP to physically remove dead code
- Schedule SCCP early in pipeline (before expensive analyses)
- Re-run SCCP after inlining or other interprocedural optimizations

---

## Advanced API Details and Extended Specifications

### Detailed Error Handling and Recovery

**Error Categories**:

1. **Fatal Errors** (return `Err`):
   - Invalid IR structure (no entry block)
   - Corrupted CFG (missing nodes referenced in edges)
   - SSA form violation (undefined values, multiple definitions)

2. **Recoverable Errors** (log warning, continue with fallback):
   - Iteration limit exceeded (conservative fallback to Bottom)
   - Unexpected instruction format (mark as non-constant)
   - Type system inconsistencies (conservative analysis)

3. **Non-Errors** (expected behavior):
   - No optimizations found (return `Ok(false)`)
   - All values already constant (return `Ok(false)`)
   - Unreachable code detected (expected, not an error)

**Error Reporting Contract**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum SccpError {
    #[error("Function has no entry block")]
    NoEntryBlock,
    
    #[error("Entry block {0:?} not found in CFG")]
    EntryBlockNotInCfg(NodeIndex),
    
    #[error("CFG edge references non-existent block {0:?}")]
    InvalidCfgEdge(NodeIndex),
    
    #[error("SSA value {0:?} used before definition")]
    UndefinedValue(Value),
    
    #[error("Iteration limit {0} exceeded (possible infinite loop or complex CFG)")]
    IterationLimitExceeded(usize),
}

impl Phase for SccpOptimizer {
    fn run(&mut self, ir: &mut Module) -> Result<bool, SccpError> {
        // Early validation
        self.validate_ir(ir)?;
        
        // Analysis with error recovery
        match self.analyze(ir) {
            Ok(result) => self.transform(ir, result),
            Err(SccpError::IterationLimitExceeded(_)) => {
                log::warn!("SCCP iteration limit exceeded, applying conservative analysis");
                self.apply_conservative_fallback(ir)
            }
            Err(fatal) => Err(fatal),
        }
    }
}
```

### Configuration API Extensions

**Advanced Configuration Options**:

```rust
impl SccpOptimizer {
    /// Configure custom constant folding behavior
    pub fn with_custom_folder(self, folder: Box<dyn ConstantFolder>) -> Self;
    
    /// Enable/disable specific optimization classes
    pub fn with_optimization_flags(self, config: OptimizationFlags) -> Self;
    
    /// Set diagnostic output level
    pub fn with_diagnostic_level(self, level: DiagnosticLevel) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct OptimizationFlags {
    pub arithmetic_folding: bool,
    pub branch_simplification: bool,
    pub phi_resolution: bool,
    pub dead_code_marking: bool,
}

impl Default for OptimizationFlags {
    fn default() -> Self {
        Self {
            arithmetic_folding: true,
            branch_simplification: true,
            phi_resolution: true,
            dead_code_marking: true,
        }
    }
}
```

### Statistics and Observability API

**Comprehensive Statistics Collection**:

```rust
#[derive(Debug, Clone, Default)]
pub struct SccpStatistics {
    // Analysis metrics
    pub iterations_performed: usize,
    pub cfg_edges_processed: usize,
    pub ssa_values_processed: usize,
    
    // Optimization outcomes
    pub constants_propagated: usize,
    pub branches_simplified: usize,
    pub phis_resolved: usize,
    pub unreachable_blocks_found: usize,
    pub dead_instructions_marked: usize,
    
    // Performance metrics
    pub analysis_time_ns: u64,
    pub transformation_time_ns: u64,
    pub total_time_ns: u64,
    
    // Memory usage
    pub peak_lattice_size: usize,
    pub peak_worklist_size: usize,
}

impl SccpOptimizer {
    /// Get detailed statistics from most recent run
    pub fn statistics(&self) -> &SccpStatistics;
    
    /// Enable statistics collection (minor performance overhead)
    pub fn with_statistics_enabled(self) -> Self;
}
```

### Extensibility Points

**Custom Constant Folder Trait**:

```rust
/// Trait for custom constant folding behavior
pub trait ConstantFolder: Send + Sync {
    /// Fold binary operation if both operands are constant
    fn fold_binary(
        &self,
        op: IrBinaryOp,
        left: &IrLiteralValue,
        right: &IrLiteralValue
    ) -> Option<IrLiteralValue>;
    
    /// Fold unary operation if operand is constant
    fn fold_unary(
        &self,
        op: IrUnaryOp,
        operand: &IrLiteralValue
    ) -> Option<IrLiteralValue>;
    
    /// Fold cast operation if operand is constant
    fn fold_cast(
        &self,
        target_type: &IrType,
        operand: &IrLiteralValue
    ) -> Option<IrLiteralValue>;
}

/// Default constant folder with wrapping arithmetic
pub struct DefaultConstantFolder;

impl ConstantFolder for DefaultConstantFolder {
    // Implementation with wrapping semantics
}
```

### Integration Patterns and Best Practices

**Pattern 1: Basic Pipeline Integration**
```rust
pub fn optimize_module(module: &mut Module) -> Result<(), OptimizationError> {
    let mut sccp = SccpOptimizer::new();
    let mut dce = DeadCodeEliminator::new();
    
    sccp.run(module)?;
    dce.run(module)?;
    
    Ok(())
}
```

**Pattern 2: Fixed-Point Iteration**
```rust
pub fn optimize_to_fixpoint(module: &mut Module) -> Result<usize, OptimizationError> {
    let mut sccp = SccpOptimizer::new();
    let mut dce = DeadCodeEliminator::new();
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 10;
    
    loop {
        iterations += 1;
        
        let sccp_changed = sccp.run(module)?;
        let dce_changed = dce.run(module)?;
        
        if !sccp_changed && !dce_changed { break; }
        if iterations >= MAX_ITERATIONS {
            log::warn!("Optimization pipeline did not converge after {} iterations", MAX_ITERATIONS);
            break;
        }
    }
    
    Ok(iterations)
}
```

**Pattern 3: Conditional Optimization**
```rust
pub fn optimize_selectively(module: &mut Module) -> Result<(), OptimizationError> {
    for function in module.functions_mut() {
        let complexity = function.instruction_count();
        
        if complexity > 1000 {
            // Large function: run full optimization
            let mut sccp = SccpOptimizer::new()
                .with_max_iterations(10000)
                .with_verbose();
            sccp.run_on_function(function)?;
        } else {
            // Small function: quick optimization
            let mut sccp = SccpOptimizer::new().with_max_iterations(100);
            sccp.run_on_function(function)?;
        }
    }
    
    Ok(())
}
```

**Pattern 4: Diagnostic Mode**
```rust
pub fn debug_optimize(module: &mut Module) -> Result<(), OptimizationError> {
    let mut sccp = SccpOptimizer::new()
        .with_verbose()
        .with_diagnostic_level(DiagnosticLevel::TRACE)
        .with_statistics_enabled();
    
    sccp.run(module)?;
    
    let stats = sccp.statistics();
    println!("SCCP Statistics:");
    println!("  Iterations: {}", stats.iterations_performed);
    println!("  Constants found: {}", stats.constants_propagated);
    println!("  Branches simplified: {}", stats.branches_simplified);
    println!("  Analysis time: {} ms", stats.analysis_time_ns / 1_000_000);
    
    Ok(())
}
```

### Thread Safety and Concurrency

**Concurrency Contract**:
- `SccpOptimizer` is `Send` but **not** `Sync`
- Parallel optimization of independent functions is safe:
  ```rust
  module.functions_mut()
      .par_iter_mut()  // rayon parallel iterator
      .try_for_each(|func| {
          let mut sccp = SccpOptimizer::new();
          sccp.run_on_function(func)
      })?;
  ```
- No global state, each `SccpOptimizer` instance is independent

### Performance Tuning Guidelines

**Iteration Limit Selection**:
- Small functions (<100 instructions): `max_iterations = 100`
- Medium functions (100-1000): `max_iterations = 1000` (default)
- Large functions (>1000): `max_iterations = 10000`
- Extremely large (>10000): Consider skipping SCCP

**Memory Optimization**:
- Pre-allocate with capacity hints
- Clear caches between function optimizations

**Profiling Hooks**:
```rust
impl SccpOptimizer {
    pub fn with_profiling_callback(
        self,
        callback: Box<dyn Fn(ProfilingEvent)>
    ) -> Self;
}

pub enum ProfilingEvent {
    AnalysisStarted { function_name: String, instruction_count: usize },
    IterationCompleted { iteration: usize, worklist_size: usize },
    AnalysisCompleted { iterations: usize, constants_found: usize },
    TransformationStarted,
    TransformationCompleted { changes_made: usize },
}
```

---

## Dependencies

**Required**:
- `petgraph = "0.8.3"` (already present)
- `thiserror = "2.0.17"` (already present)
- Existing IR types: `Value`, `IrLiteralValue`, `IrType`, `Instruction`, `Function`, `Module`

**Optional Future Dependencies**:
- `rayon` for parallel function optimization
- `criterion` for performance benchmarking (dev-dependency)

---

## Migration Guide

### For Compiler Pipeline Authors

**Before** (without SCCP):
```rust
run_pipeline(module, vec![
    Box::new(DeadCodeElimination::new()),
]);
```

**After** (with SCCP):
```rust
// Recommended: alternating SCCP+DCE
let mut changed = true;
while changed {
    changed = false;
    changed |= SccpOptimizer::new().run(module)?;
    changed |= DeadCodeElimination::new().run(module)?;
}
```

**Advanced** (with configuration):
```rust
let mut sccp = SccpOptimizer::new()
    .with_max_iterations(5000)
    .with_verbose()
    .with_statistics_enabled();

let iterations = optimize_to_fixpoint(module)?;
let stats = sccp.statistics();
println!("Converged after {} iterations", iterations);
println!("SCCP found {} constants", stats.constants_propagated);
```

### For Downstream Optimizations

**Expectations**: After SCCP runs:
- All easily discoverable constants are propagated
- Unreachable blocks are marked (but not removed until DCE)
- Branch conditions may be unconditional jumps
- Phi nodes may have reduced predecessors or be constant

**Recommendations**:
- Run DCE after SCCP to physically remove dead code
- Schedule SCCP early in pipeline (before expensive analyses)
- Re-run SCCP after inlining or other interprocedural optimizations
- Use statistics API to track optimization effectiveness

### For Custom Optimization Authors

**Leveraging SCCP Results**:
```rust
pub struct ArrayBoundsCheckEliminator;

impl Phase for ArrayBoundsCheckEliminator {
    fn run(&mut self, module: &mut Module) -> Result<bool, OptError> {
        // SCCP should have already run, so constants are propagated
        for func in module.functions_mut() {
            for inst in func.instructions_mut() {
                if let Instruction::ArrayAccess { index, array_len } = inst {
                    if let Value::Literal(IrLiteralValue::I32(idx)) = index {
                        if *idx >= 0 && *idx < *array_len {
                            inst.mark_bounds_check_unnecessary();
                        }
                    }
                }
            }
        }
        Ok(true)
    }
}
```

---

## Versioning and Compatibility

**Semantic Versioning Policy**:

**Current Version**: 1.0.0

**Breaking Changes** (major version bump):
- Changes to `SccpOptimizer` public fields
- Removal of public methods
- Changes to Phase trait return value semantics
- Changes to error types in public API

**Non-Breaking Changes** (minor version bump):
- Addition of new public methods
- New configuration options (with defaults)
- Performance improvements
- New error variants (if using non-exhaustive enums)

**Patch Changes**:
- Bug fixes
- Documentation updates
- Internal implementation improvements

**Deprecation Policy**:
- Deprecated features will emit warnings for 2 minor versions
- Removal only occurs on major version bump
- Migration guide provided for all breaking changes

---

## Conclusion

This exhaustively detailed API contract guarantees sound, efficient constant propagation with comprehensive behavioral specifications, rigorous performance targets, extensible integration patterns, and production-ready error handling. The Phase trait integration ensures seamless pipeline composition while maintaining strict correctness invariants, thread-safety boundaries, and observable performance characteristics.

Key API guarantees:
1. **Correctness**: Sound optimization preserving program semantics
2. **Performance**: O(V+E) time and space complexity with configurable limits
3. **Observability**: Detailed statistics and diagnostic capabilities
4. **Extensibility**: Custom constant folders and profiling hooks
5. **Reliability**: Graceful degradation and comprehensive error recovery
6. **Maintainability**: Semantic versioning and backward compatibility commitments
