# Phase Trait Contract for SCCP Optimizer

**Feature**: Sparse Conditional Constant Propagation Optimizer  
**Branch**: 016-sccp-optimizer  
**Date**: 2025-11-17

## Overview

This document specifies the API contract for the SCCP optimizer's integration with the jsavrs compiler's optimization pipeline via the `Phase` trait.

## Phase Trait Interface

```rust
/// Trait for optimization and transformation passes in the jsavrs compiler.
///
/// Each phase processes a complete Module (collection of functions) and
/// performs its transformations, returning success or error status.
pub trait Phase {
    /// Returns the human-readable name of this phase for logging and diagnostics.
    fn name(&self) -> &str;
    
    /// Executes the optimization pass on the given module.
    ///
    /// # Arguments
    ///
    /// * `module` - Mutable reference to the IR module to transform
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Pass completed successfully
    /// * `Err(String)` - Pass failed with error message
    ///
    /// # Errors
    ///
    /// This method returns an error if:
    /// - Pre-optimization validation fails (invalid SSA form, malformed CFG)
    /// - Post-optimization validation fails (SSA form broken, CFG invalid)
    /// - Maximum iterations exceeded without convergence
    fn run(&mut self, module: &mut Module) -> Result<(), String>;
}
```

## ConstantFoldingOptimizer Implementation

```rust
pub struct ConstantFoldingOptimizer {
    /// Enable verbose logging (default: false)
    pub verbose: bool,
    
    /// Maximum worklist iterations (default: 100)
    pub max_iterations: usize,
    
    /// Enable SCCP analysis (default: true)
    pub sccp_enabled: bool,
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &str {
        "Constant Folding (SCCP)"
    }
    
    fn run(&mut self, module: &mut Module) -> Result<(), String>;
}
```

### Method: `name()`

**Purpose**: Returns the display name of the optimization pass

**Returns**: `"Constant Folding (SCCP)"`

**Usage**:
```rust
let optimizer = ConstantFoldingOptimizer::default();
println!("Running pass: {}", optimizer.name());
// Output: Running pass: Constant Folding (SCCP)
```

**Guarantees**:
- Always returns the same static string
- Non-empty string
- Suitable for logging and progress reporting

### Method: `run(module: &mut Module)`

**Purpose**: Executes SCCP analysis and IR transformation on all functions in the module

**Parameters**:

| Parameter | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| `module` | `&mut Module` | The IR module to optimize | Must be valid SSA form with well-formed CFG |

**Returns**:

| Type | Description | When |
|------|-------------|------|
| `Ok(())` | Success | All functions optimized successfully |
| `Err(String)` | Error message | Validation failed or iteration limit exceeded |

**Behavior**:

1. **Early Exit**: If `sccp_enabled` is false, returns `Ok(())` immediately without modifications
2. **Per-Function Processing**: Iterates over all functions in `module.functions`
3. **Transform Each Function**:
   - Call `transform_function(function)` (internal method)
   - Collect optimization statistics
   - Propagate errors immediately on failure
4. **Statistics Aggregation**: Merge statistics from all functions
5. **Verbose Logging**: If `verbose` is true, print aggregated statistics to stderr
6. **Return Success**: If all functions transformed successfully

**Error Conditions**:

| Error | Cause | Example Message |
|-------|-------|-----------------|
| Pre-validation failure | Invalid SSA form before optimization | `"SCCP failed on function main: Pre-optimization validation failed: phi node in entry block"` |
| Post-validation failure | SSA form or CFG broken by transformation | `"SCCP failed on function helper: Post-optimization validation failed: duplicate temporary T42"` |
| Max iterations exceeded | Fixed-point not reached within limit | `"SCCP failed on function complex: Maximum iterations (100) exceeded without convergence"` |
| CFG malformation | Invalid branch targets or missing entry block | `"SCCP failed on function broken: CFG integrity violation: branch to non-existent block"` |

**Preconditions**:
- `module` contains valid IR in SSA form
- All functions have well-formed CFGs with entry blocks
- All phi nodes only in blocks with multiple predecessors
- Type information is accurate for all values

**Postconditions** (on success):
- All constant values propagated where possible
- Conditional branches with constant conditions converted to unconditional
- Unreachable blocks removed from all functions
- SSA form preserved in all functions
- CFG validity preserved in all functions

**Side Effects**:
- Modifies IR structure of functions in `module`
- May print statistics to stderr if `verbose` is true
- Updates `self.stats` (internal state)

**Thread Safety**: Not thread-safe (requires mutable reference to self)

**Performance**: O(functions * (SSA_edges + CFG_edges)) time complexity

**Example Usage**:

```rust
// Create optimizer with custom configuration
let mut optimizer = ConstantFoldingOptimizer {
    verbose: true,
    max_iterations: 200,
    sccp_enabled: true,
};

// Run on module
match optimizer.run(&mut module) {
    Ok(()) => println!("SCCP optimization completed successfully"),
    Err(e) => eprintln!("SCCP optimization failed: {}", e),
}
```

## Internal Method: `transform_function`

```rust
impl ConstantFoldingOptimizer {
    fn transform_function(&mut self, function: &mut Function) 
        -> Result<OptimizationStatistics, SCCPError>;
}
```

**Purpose**: Performs SCCP analysis and transformation on a single function

**Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `function` | `&mut Function` | The function to optimize |

**Returns**:

| Type | Description |
|------|-------------|
| `OptimizationStatistics` | Metrics collected during optimization |
| `SCCPError` | Detailed error information |

**Workflow**:

1. **Pre-validation**: Call `validate_preconditions(function)`
   - Verify entry block exists
   - Verify all phi incoming edges reference existing predecessors
   - Verify all branch targets exist in CFG
   - Return error if validation fails

2. **Initialization**: Create `SCCPAnalyzer::new(function, max_iterations, verbose)`
   - Initialize lattice with all values at Top (except parameters/globals → Bottom)
   - Mark entry block and outgoing edges as executable
   - Create empty worklists

3. **Fixed-Point Analysis**: Call `analyzer.analyze()`
   - Process SSAWorkList and FlowWorkList until both empty
   - Update lattice values based on instruction evaluation
   - Mark new CFG edges executable based on terminator evaluation
   - Return statistics or error (max iterations exceeded, internal error)

4. **IR Rewrite**: Call `analyzer.rewrite(function)`
   - Replace constant instructions with literal values
   - Convert conditional branches with constant conditions
   - Remove unreachable blocks
   - Simplify phi nodes

5. **Post-validation**: Call `validate_postconditions(function, analyzer.lattice)`
   - Verify SSA form preserved (no duplicate definitions, all uses dominated by defs)
   - Verify CFG valid (all terminators valid, all targets exist)
   - Verify no Top values remain in executable regions
   - Return error if validation fails

6. **Return Statistics**: Return `analyzer.stats` on success

**Guarantees**:
- Function is either successfully optimized or returned to original state (on error)
- SSA form and CFG validity are preserved
- No observable behavior changes (optimizations are semantically preserving)

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum SCCPError {
    #[error("Pre-optimization validation failed: {0}")]
    PreValidationFailed(String),
    
    #[error("Post-optimization validation failed: {0}")]
    PostValidationFailed(String),
    
    #[error("Maximum iterations ({0}) exceeded without convergence")]
    MaxIterationsExceeded(usize),
    
    #[error("SSA form violation: {0}")]
    SSAViolation(String),
    
    #[error("CFG integrity violation: {0}")]
    CFGViolation(String),
    
    #[error("Lattice invariant violation: value {0} moved upward from {1:?} to {2:?}")]
    LatticeInvariantViolation(String, LatticeValue, LatticeValue),
}
```

**Error Handling**:
- All errors are propagated to caller via `Result` type
- Errors include detailed context (function name, specific violation)
- Warning-level issues (max iterations) log to stderr but don't fail the pass
- Critical errors (SSA violation) fail immediately with detailed message

## Configuration Options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `verbose` | `bool` | `false` | Enable detailed logging to stderr |
| `max_iterations` | `usize` | `100` | Maximum worklist iterations before timeout |
| `sccp_enabled` | `bool` | `true` | Master enable/disable switch for SCCP |

**Configuration Examples**:

```rust
// Verbose logging for debugging
let optimizer = ConstantFoldingOptimizer {
    verbose: true,
    ..Default::default()
};

// Increased iteration limit for complex functions
let optimizer = ConstantFoldingOptimizer {
    max_iterations: 500,
    ..Default::default()
};

// Disable SCCP (pass-through)
let optimizer = ConstantFoldingOptimizer {
    sccp_enabled: false,
    ..Default::default()
};
```

## Integration Example

```rust
use jsavrs::ir::optimizer::ConstantFoldingOptimizer;
use jsavrs::ir::Phase;

fn optimize_module(module: &mut Module) -> Result<(), String> {
    // Create optimization pipeline
    let mut passes: Vec<Box<dyn Phase>> = vec![
        Box::new(ConstantFoldingOptimizer::default()),
        // Other passes...
    ];
    
    // Run all passes
    for pass in passes.iter_mut() {
        eprintln!("Running pass: {}", pass.name());
        pass.run(module)?;
    }
    
    Ok(())
}
```

## Compatibility

**Requires**:
- Valid SSA form IR
- Well-formed CFG with entry block
- Existing `verify_ssa_form` and `cfg.verify()` functions

**Compatible With**:
- Dead Code Elimination (DCE) - can run before or after SCCP
- Other SSA-based optimizations
- Any phase that preserves SSA form

**Not Compatible With** (run SCCP before these):
- SSA destruction passes
- Register allocation (operates on non-SSA form)

---

**End of Phase Trait Contract**
