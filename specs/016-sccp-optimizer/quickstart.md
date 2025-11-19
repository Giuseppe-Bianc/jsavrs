# SCCP Optimizer Quick Start Guide

**Feature**: Sparse Conditional Constant Propagation Optimizer  
**Branch**: 016-sccp-optimizer  
**Date**: 2025-11-17

## Overview

This guide provides practical examples for using the Sparse Conditional Constant Propagation (SCCP) optimizer in the jsavrs compiler. SCCP is a powerful optimization that identifies compile-time constant values, eliminates conditional branches with constant conditions, and removes unreachable code.

## Installation and Setup

The SCCP optimizer is built into the jsavrs compiler as part of the IR optimization pipeline. No additional installation is required.

**Build the compiler** (if not already built):
```bash
cargo build --release
```

**Run tests** (to verify SCCP implementation):
```bash
cargo test ir_sccp
```

## Basic Usage

### Example 1: Simple Constant Propagation

**Input IR** (before SCCP):
```text
function calculate():
    %1 = const 5
    %2 = const 10
    %3 = add %1, %2
    %4 = mul %3, 2
    return %4
```

**Optimized IR** (after SCCP):
```text
function calculate():
    return 30
```

**Explanation**: SCCP determines that:
- `%1` is constant `5`
- `%2` is constant `10`
- `%3` is constant `15` (5 + 10)
- `%4` is constant `30` (15 * 2)

All intermediate calculations are replaced with the final constant `30`.

**How to run**:
```rust
use jsavrs::ir::optimizer::ConstantFoldingOptimizer;
use jsavrs::ir::Phase;

let mut optimizer = ConstantFoldingOptimizer::default();
optimizer.run(&mut module)?;
```

---

### Example 2: Branch Elimination

**Input IR** (before SCCP):
```text
function check_constant():
    %1 = const true
    br %1, then_block, else_block

then_block:
    %2 = const 42
    return %2

else_block:
    %3 = const 99
    return %3
```

**Optimized IR** (after SCCP):
```text
function check_constant():
    br then_block

then_block:
    return 42

# else_block removed (unreachable)
```

**Explanation**: SCCP determines that:
- `%1` is constant `true`
- The conditional branch always takes the `true` path
- `else_block` is unreachable and removed
- The conditional branch is converted to an unconditional jump

---

### Example 3: Phi Node Simplification

**Input IR** (before SCCP):
```text
function merge_constants():
    %1 = const 5
    br merge_block

merge_block:
    %2 = phi [%1, entry_block]
    %3 = add %2, 10
    return %3
```

**Optimized IR** (after SCCP):
```text
function merge_constants():
    br merge_block

merge_block:
    return 15
```

**Explanation**: SCCP determines that:
- `%1` is constant `5`
- Phi node `%2` has only one incoming value (`5`), simplified to constant
- `%3` is constant `15` (5 + 10)
- All intermediate values replaced with final constant

---

## Configuration Options

### Default Configuration

```rust
use jsavrs::ir::optimizer::ConstantFoldingOptimizer;

// Default settings: verbose=false, max_iterations=100, sccp_enabled=true
let optimizer = ConstantFoldingOptimizer::default();
```

### Verbose Logging

Enable detailed optimization logs for debugging:

```rust
let mut optimizer = ConstantFoldingOptimizer {
    verbose: true,
    ..Default::default()
};

optimizer.run(&mut module)?;
```

**Example output**:
```text
SCCP Optimization Statistics:
  Constants found: 145 (72.5%)
  Branches eliminated: 8
  Blocks removed: 12 (15.0%)
  Instructions replaced: 203
  Phi nodes simplified: 17
  Iterations to convergence: 23
  Total values analyzed: 200
  Total blocks analyzed: 80
```

### Custom Iteration Limit

Increase the maximum iteration limit for complex functions:

```rust
let mut optimizer = ConstantFoldingOptimizer {
    max_iterations: 500,  // Increased from default 100
    ..Default::default()
};
```

**When to use**: Large functions with complex control flow may require more iterations to reach fixed-point convergence.

### Disable SCCP

Temporarily disable SCCP without removing from pipeline:

```rust
let mut optimizer = ConstantFoldingOptimizer {
    sccp_enabled: false,  // Pass-through, no optimization
    ..Default::default()
};
```

**When to use**: Debugging, A/B testing, or comparing optimized vs. unoptimized performance.

---

## Integration with Optimization Pipeline

### Adding SCCP to Your Pipeline

```rust
use jsavrs::ir::optimizer::ConstantFoldingOptimizer;
use jsavrs::ir::Phase;

fn build_optimization_pipeline() -> Vec<Box<dyn Phase>> {
    vec![
        // Early optimizations
        Box::new(ConstantFoldingOptimizer::default()),
        
        // SCCP enables DCE by marking unreachable code
        Box::new(DeadCodeElimination::default()),
        
        // Further optimizations...
    ]
}

fn optimize_module(module: &mut Module) -> Result<(), String> {
    let mut passes = build_optimization_pipeline();
    
    for pass in passes.iter_mut() {
        eprintln!("Running pass: {}", pass.name());
        pass.run(module)?;
    }
    
    Ok(())
}
```

### Recommended Pass Ordering

1. **SCCP** (this optimizer) - Identify constants and eliminate dead branches
2. **Dead Code Elimination (DCE)** - Remove unreachable instructions and values
3. **Loop Optimizations** - Optimize remaining loops (unrolling, invariant code motion)
4. **Further Constant Folding** - Catch opportunities enabled by previous passes

**Rationale**: SCCP creates opportunities for DCE by marking unreachable code. DCE cleanup enables further optimizations.

---

## Advanced Examples

### Example 4: Type-Safe Integer Overflow Handling

**Input IR** (before SCCP):
```text
function overflow_test():
    %1 = const 2147483647  # i32::MAX
    %2 = const 1
    %3 = add %1, %2        # Would overflow
    return %3
```

**Optimized IR** (after SCCP):
```text
function overflow_test():
    %1 = const 2147483647
    %2 = const 1
    %3 = add %1, %2  # Marked as Bottom (varying), not optimized
    return %3
```

**Explanation**: SCCP uses checked arithmetic (`checked_add`) and marks overflow as `Bottom` (non-constant) to avoid incorrect optimizations. The IR remains unchanged because the result cannot be proven constant.

---

### Example 5: Complex Control Flow with Constants

**Input IR** (before SCCP):
```text
function complex():
    %1 = const 10
    %2 = lt %1, 20         # %2 = true
    br %2, path_a, path_b

path_a:
    %3 = const 100
    br merge

path_b:
    %4 = const 200
    br merge

merge:
    %5 = phi [%3, path_a], [%4, path_b]
    return %5
```

**Optimized IR** (after SCCP):
```text
function complex():
    br path_a

path_a:
    br merge

merge:
    return 100

# path_b removed (unreachable)
```

**Explanation**: SCCP determines that:
- `%1` is constant `10`
- `%2` is constant `true` (10 < 20)
- Only `path_a` is reachable
- `path_b` is removed as unreachable
- Phi node `%5` simplified to constant `100` (only one incoming path)

---

## Debugging and Troubleshooting

### Common Issues

#### Issue 1: "Maximum iterations exceeded without convergence"

**Symptom**: Error message when running SCCP
```text
SCCP failed on function complex_function: Maximum iterations (100) exceeded without convergence
```

**Cause**: Function has very complex control flow requiring more than 100 iterations

**Solution**: Increase `max_iterations`:
```rust
let mut optimizer = ConstantFoldingOptimizer {
    max_iterations: 500,
    ..Default::default()
};
```

#### Issue 2: "SSA form violation" after optimization

**Symptom**: Post-optimization validation fails
```text
SCCP failed on function broken: Post-optimization validation failed: duplicate temporary T42
```

**Cause**: Bug in SCCP implementation or pre-existing invalid IR

**Solution**: 
1. Verify input IR is valid SSA form before SCCP
2. Run with `verbose: true` to see where violation occurs
3. Report bug with minimal reproduction case

#### Issue 3: No constants found (0%)

**Symptom**: Statistics show 0% constants found
```text
  Constants found: 0 (0.0%)
```

**Cause**: Function has no compile-time constant computations (all values depend on runtime input)

**Solution**: This is expected behavior for functions without constants. SCCP conservatively marks runtime values as `Bottom`.

**Example**:
```text
function runtime_dependent(%arg):
    %1 = add %arg, 1  # Depends on runtime argument
    return %1
```

### Verbose Logging for Debugging

Enable verbose mode to see detailed optimization steps:

```rust
let mut optimizer = ConstantFoldingOptimizer {
    verbose: true,
    ..Default::default()
};
```

**Sample verbose output**:
```text
[SCCP] Analyzing function: calculate
[SCCP] Initializing lattice: 15 values
[SCCP] Entry block: B0
[SCCP] Iteration 1:
  [SSA] Processing edge: T1 → I5
  [Lattice] T3: Top → Constant(I32(15))
  [SSA] Enqueued: T3 → I7
  [Flow] Processing edge: B0 → B1
  [Block] Marked executable: B1
[SCCP] Iteration 2:
  [SSA] Processing edge: T3 → I7
  [Lattice] T4: Top → Constant(I32(30))
[SCCP] Convergence: 2 iterations
[SCCP] Rewriting IR...
  [Replace] T3 → Constant(I32(15))
  [Replace] T4 → Constant(I32(30))
[SCCP] Statistics:
  Constants found: 4 (80.0%)
  Instructions replaced: 4
```

---

## Performance Tips

### Tip 1: Run SCCP Early in Pipeline

SCCP is most effective when run early, before other optimizations have modified the IR structure.

**Recommended**:
```rust
vec![
    Box::new(ConstantFoldingOptimizer::default()),  // Run first
    Box::new(DeadCodeElimination::default()),
    // Other passes...
]
```

### Tip 2: Combine with DCE

SCCP marks unreachable code but may not remove all dead instructions. Follow with Dead Code Elimination:

```rust
vec![
    Box::new(ConstantFoldingOptimizer::default()),
    Box::new(DeadCodeElimination::default()),  // Cleanup after SCCP
]
```

### Tip 3: Measure Impact

Use verbose mode to measure SCCP's impact on your code:

```rust
let mut optimizer = ConstantFoldingOptimizer {
    verbose: true,
    ..Default::default()
};
```

**Metrics to watch**:
- **Constants found %**: Higher is better (more optimization opportunities)
- **Branches eliminated**: Direct code size reduction
- **Blocks removed %**: Dead code elimination effectiveness
- **Iterations to convergence**: Should be << `max_iterations` (typically 10-50)

---

## Testing Your Code with SCCP

### Unit Test Example

```rust
#[test]
fn test_constant_propagation() {
    let module = parse_ir(r#"
        function test():
            %1 = const 5
            %2 = const 10
            %3 = add %1, %2
            return %3
    "#);
    
    let mut optimizer = ConstantFoldingOptimizer::default();
    optimizer.run(&mut module).unwrap();
    
    // Verify %3 is replaced with constant 15
    let function = &module.functions[0];
    assert!(function.contains_constant_return(15));
}
```

### Integration Test Example

```rust
#[test]
fn test_full_optimization_pipeline() {
    let module = load_test_module("test_program.ir");
    
    // Run full pipeline
    let mut passes: Vec<Box<dyn Phase>> = vec![
        Box::new(ConstantFoldingOptimizer::default()),
        Box::new(DeadCodeElimination::default()),
    ];
    
    for pass in passes.iter_mut() {
        pass.run(&mut module).unwrap();
    }
    
    // Verify expected optimizations
    assert_eq!(module.functions[0].basic_blocks.len(), expected_blocks);
}
```

### Snapshot Testing with Insta

```rust
#[test]
fn test_sccp_snapshot() {
    let module = load_test_module("complex_function.ir");
    
    let mut optimizer = ConstantFoldingOptimizer::default();
    optimizer.run(&mut module).unwrap();
    
    // Snapshot the optimized IR
    insta::assert_snapshot!(module.to_string());
}
```

---

## Reference

### Configuration Struct

```rust
pub struct ConstantFoldingOptimizer {
    pub verbose: bool,           // Default: false
    pub max_iterations: usize,   // Default: 100
    pub sccp_enabled: bool,      // Default: true
}
```

### Statistics Struct

```rust
pub struct OptimizationStatistics {
    pub constants_found: usize,
    pub branches_eliminated: usize,
    pub blocks_removed: usize,
    pub instructions_replaced: usize,
    pub phi_nodes_simplified: usize,
    pub iterations_to_convergence: usize,
    pub total_values_analyzed: usize,
    pub total_blocks_analyzed: usize,
}
```

### Error Types

```rust
pub enum SCCPError {
    PreValidationFailed(String),
    PostValidationFailed(String),
    MaxIterationsExceeded(usize),
    SSAViolation(String),
    CFGViolation(String),
    LatticeInvariantViolation(String, LatticeValue, LatticeValue),
}
```

---

## Further Reading

- **Research Document**: `specs/016-sccp-optimizer/research.md` - Algorithm details and theory
- **Data Model**: `specs/016-sccp-optimizer/data-model.md` - Internal data structures
- **Contracts**: `specs/016-sccp-optimizer/contracts/` - API specifications

---

**End of Quick Start Guide**

For questions or issues, please consult the detailed research and data model documents or open a GitHub issue.
