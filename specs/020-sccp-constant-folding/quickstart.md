# Quickstart Guide: SCCP Constant Folding Optimizer

**Feature**: Constant Folding Optimizer with SCCP  
**Branch**: `020-sccp-constant-folding`  
**Date**: 2025-12-05  
**For**: Developers integrating or using the SCCP optimizer

## Overview

This guide provides practical examples for using the Sparse Conditional Constant Propagation (SCCP) optimizer in the jsavrs compiler. It covers basic usage, integration into the optimization pipeline, configuration options, and common scenarios.

## Quick Start (5 Minutes)

### 1. Basic Usage

Add the SCCP optimizer to your compilation pipeline:

```rust
use jsavrs::ir::optimizer::constant_folding::ConstantFoldingOptimizer;
use jsavrs::ir::optimizer::Phase;

// Create optimizer with default configuration
let mut sccp = ConstantFoldingOptimizer::new();

// Run on your IR module
sccp.run(&mut module)?;

// Access statistics
let stats = sccp.get_stats();
println!("SCCP: {} constants propagated", stats.constants_propagated);
```

**Output**:

```text
SCCP: 42 constants propagated, 5 branches resolved, 
      3 phi nodes simplified, 2 blocks marked unreachable, 
      2 iterations
```

### 2. Integration with DCE

Combine SCCP with Dead Code Elimination for maximum optimization:

```rust
use jsavrs::ir::optimizer::{
    constant_folding::ConstantFoldingOptimizer,
    dead_code_elimination::DeadCodeElimination,
    Phase,
};

// Step 1: Constant propagation
let mut sccp = ConstantFoldingOptimizer::new();
sccp.run(&mut module)?;

// Step 2: Remove dead code identified by SCCP
let mut dce = DeadCodeElimination::new();
dce.run(&mut module)?;

println!("Combined stats:");
println!("  Constants propagated: {}", sccp.get_stats().constants_propagated);
println!("  Dead blocks removed: {}", dce.get_stats().blocks_removed);
```

### 3. Configuration

Enable verbose diagnostics for debugging:

```rust
use jsavrs::ir::optimizer::constant_folding::{
    ConstantFoldingOptimizer,
    SCCPConfig,
};

let config = SCCPConfig {
    verbose: true,
    max_iterations: 100,
};

let mut sccp = ConstantFoldingOptimizer::with_config(config);
sccp.run(&mut module)?;
```

**Verbose Output** (stderr):

```text
[SCCP] Value v42: Bottom → Constant(I32(10))
[SCCP] Value v43: Constant(I32(10)) → Constant(I32(10))
[SCCP] CFG worklist: added edge bb2 → bb5
[SCCP] Block bb7 unreachable (no executable predecessors)
```

---

## Common Use Cases

### Use Case 1: Simple Constant Folding

**Scenario**: Optimize arithmetic on constant literals

**Input IR**:

```rust
function example():
    %1 = const 10
    %2 = const 32
    %3 = add %1, %2
    %4 = mul %3, 2
    return %4
```

**Code**:

```rust
let mut sccp = ConstantFoldingOptimizer::new();
sccp.run(&mut module)?;
```

**Output IR**:

```rust
function example():
    %1 = const 10
    %2 = const 32
    %3 = const 42   // add %1, %2 → 42
    %4 = const 84   // mul 42, 2 → 84
    return %4
```

**Statistics**:

- Constants propagated: 2
- Iterations: 1

---

### Use Case 2: Branch Resolution

**Scenario**: Eliminate dead paths based on constant conditions

**Input IR**:

```rust
function example():
    %cond = const true
    br %cond, label %true_block, label %false_block
    
  %true_block:
    return 1
    
  %false_block:
    return 2
```

**Code**:

```rust
let mut sccp = ConstantFoldingOptimizer::new();
sccp.run(&mut module)?;

// Follow with DCE to remove dead block
let mut dce = DeadCodeElimination::new();
dce.run(&mut module)?;
```

**Output IR** (after SCCP):

```rust
function example():
    %cond = const true
    br label %true_block  // Conditional → unconditional
    
  %true_block:
    return 1
    
  %false_block:  // Marked unreachable
    return 2
```

**Output IR** (after SCCP + DCE):

```rust
function example():
    %cond = const true
    br label %true_block
    
  %true_block:
    return 1
    
  // %false_block removed by DCE
```

**Statistics**:

- Branches resolved: 1
- Blocks marked unreachable: 1
- Blocks removed (DCE): 1

---

### Use Case 3: Phi Node Simplification

**Scenario**: Simplify phi nodes when all incoming values are constant

**Input IR**:

```rust
function example(%flag):
    br %flag, label %left, label %right
    
  %left:
    %v1 = const 42
    br label %merge
    
  %right:
    %v2 = const 42
    br label %merge
    
  %merge:
    %result = phi [%left: %v1, %right: %v2]
    return %result
```

**Code**:

```rust
let mut sccp = ConstantFoldingOptimizer::new();
sccp.run(&mut module)?;
```

**Output IR**:

```rust
function example(%flag):
    br %flag, label %left, label %right
    
  %left:
    %v1 = const 42
    br label %merge
    
  %right:
    %v2 = const 42
    br label %merge
    
  %merge:
    %result = const 42  // Phi simplified
    return %result
```

**Statistics**:

- Phi nodes simplified: 1

---

### Use Case 4: Nested Branches

**Scenario**: Resolve nested conditional branches with constant conditions

**Input IR**:

```rust
function example():
    %outer_cond = const true
    br %outer_cond, label %outer_true, label %outer_false
    
  %outer_true:
    %inner_cond = const false
    br %inner_cond, label %inner_true, label %inner_false
    
  %inner_true:
    return 1
    
  %inner_false:
    return 2
    
  %outer_false:
    return 3
```

**Code**:

```rust
let mut sccp = ConstantFoldingOptimizer::new();
sccp.run(&mut module)?;

let mut dce = DeadCodeElimination::new();
dce.run(&mut module)?;
```

**Output IR** (after SCCP + DCE):

```rust
function example():
    %outer_cond = const true
    br label %outer_true
    
  %outer_true:
    %inner_cond = const false
    br label %inner_false
    
  %inner_false:
    return 2
```

**Statistics**:

- Branches resolved: 2
- Blocks marked unreachable: 2 (%inner_true, %outer_false)
- Blocks removed (DCE): 2

---

## Advanced Configuration

### Custom Iteration Limits

For very large functions, increase iteration limit:

```rust
let config = SCCPConfig {
    verbose: false,
    max_iterations: 500,  // Default is 100
};

let mut sccp = ConstantFoldingOptimizer::with_config(config);
sccp.run(&mut module)?;
```

### Error Handling

Handle convergence failures gracefully:

```rust
use jsavrs::ir::optimizer::constant_folding::SCCPError;

match sccp.run(&mut module) {
    Ok(()) => {
        println!("SCCP succeeded: {}", sccp.get_stats());
    }
    Err(OptimizationError::SCCPError(SCCPError::MaxIterationsExceeded(limit))) => {
        eprintln!("Warning: SCCP did not converge within {} iterations", limit);
        eprintln!("Proceeding with partial optimization");
        // Module is still valid, optimization is conservative
    }
    Err(e) => return Err(e),
}
```

---

## Integration Patterns

### Pattern 1: Iterative Optimization

Run SCCP and DCE iteratively until fixed point:

```rust
loop {
    let initial_instruction_count = module.count_instructions();
    
    // SCCP pass
    let mut sccp = ConstantFoldingOptimizer::new();
    sccp.run(&mut module)?;
    
    // DCE pass
    let mut dce = DeadCodeElimination::new();
    dce.run(&mut module)?;
    
    let final_instruction_count = module.count_instructions();
    
    if final_instruction_count == initial_instruction_count {
        break;  // No further optimization possible
    }
}
```

### Pattern 2: Optimization Pipeline

Integrate SCCP into a larger optimization pipeline:

```rust
fn optimize_module(module: &mut Module) -> Result<(), OptimizationError> {
    // Phase 1: Early optimizations
    let mut simplifier = InstructionSimplifier::new();
    simplifier.run(module)?;
    
    // Phase 2: SCCP for constant propagation
    let mut sccp = ConstantFoldingOptimizer::new();
    sccp.run(module)?;
    
    // Phase 3: DCE to remove dead code
    let mut dce = DeadCodeElimination::new();
    dce.run(module)?;
    
    // Phase 4: Further optimizations...
    
    Ok(())
}
```

### Pattern 3: Conditional Optimization

Only run SCCP if profiling suggests benefit:

```rust
if module.has_constant_expressions() {
    let mut sccp = ConstantFoldingOptimizer::new();
    sccp.run(&mut module)?;
    
    // Only run DCE if SCCP found unreachable code
    if sccp.get_stats().blocks_marked_unreachable > 0 {
        let mut dce = DeadCodeElimination::new();
        dce.run(&mut module)?;
    }
}
```

---

## Debugging and Diagnostics

### Enable Verbose Output

```rust
let config = SCCPConfig {
    verbose: true,
    max_iterations: 100,
};

let mut sccp = ConstantFoldingOptimizer::with_config(config);
sccp.run(&mut module)?;
```

**Interpreting Verbose Output**:

```text
[SCCP] Value v42: Bottom → Constant(I32(10))
```

- Value `v42` was initially uninitialized (Bottom)
- SCCP proved it always has the constant value 10

```text
[SCCP] CFG worklist: added edge bb2 → bb5
```

- Control flow edge from block 2 to block 5 became executable
- Block 5 will be analyzed in next iteration

```text
[SCCP] Block bb7 unreachable (no executable predecessors)
```

- No control flow path reaches block 7
- Block will be marked for DCE

### Analyze Statistics

```rust
let stats = sccp.get_stats();

println!("Optimization impact:");
println!("  Constants propagated: {}", stats.constants_propagated);
println!("  Branches resolved: {}", stats.branches_resolved);
println!("  Phi nodes simplified: {}", stats.phi_nodes_simplified);
println!("  Unreachable blocks: {}", stats.blocks_marked_unreachable);
println!("  Convergence iterations: {}", stats.iterations);

if stats.iterations > 10 {
    println!("Warning: High iteration count may indicate complex control flow");
}
```

---

## Performance Considerations

### When to Use SCCP

**Recommended**:

- Functions with constant literals and arithmetic
- Code with conditional branches on constants
- Programs with dead code paths

**Less Beneficial**:

- Functions with no constants
- Heavily dynamic code (all runtime values)
- Small functions (overhead may outweigh benefits)

### Performance Expectations

| Function Size           | Typical Time  | Iterations |
| ----------------------- | ------------- | ---------- |
| <100 instructions       | <1 ms         | 1-2        |
| 100-1000 instructions   | 1-10 ms       | 2-3        |
| 1000-10000 instructions | 10-100 ms     | 2-4        |
| >10000 instructions     | 100-1000 ms   | 3-5        |

### Memory Usage

- **Overhead**: ~24 bytes per SSA value + 16 bytes per CFG edge
- **Example**: 1000-instruction function ≈ 50 KB overhead

---

## Common Issues and Solutions

### Issue: High Iteration Count

**Symptom**: `iterations > 10` in statistics

**Causes**:

- Complex control flow with many nested branches
- Large phi nodes with many predecessors

**Solutions**:

- Increase `max_iterations` if function is valid
- Review IR for unnecessary complexity
- Consider function splitting

### Issue: No Constants Propagated

**Symptom**: `constants_propagated == 0`

**Causes**:

- No constant expressions in function
- All values are runtime-dependent
- Previous optimizations already folded constants

**Solutions**:

- Verify input IR has constant literals
- Check if earlier passes already optimized
- This is normal for some functions

### Issue: SSA Verification Failures After SCCP

**Symptom**: Dominance errors, multiple definitions

**Causes**:

- Bug in rewriter (report as issue)
- Corrupted IR before SCCP

**Solutions**:

- Verify IR before running SCCP
- Enable debug assertions
- Report bug with minimal reproducer

---

## Testing Your Integration

### Unit Test Example

```rust
#[test]
fn test_sccp_constant_propagation() {
    let ir = r#"
    function test():
        %1 = const 10
        %2 = const 32
        %3 = add %1, %2
        return %3
    "#;
    
    let mut module = parse_ir(ir);
    
    let mut sccp = ConstantFoldingOptimizer::new();
    sccp.run(&mut module).unwrap();
    
    let stats = sccp.get_stats();
    assert_eq!(stats.constants_propagated, 1);  // %3 replaced
}
```

### Snapshot Test Example

```rust
#[test]
fn test_sccp_branch_resolution() {
    let input = r#"
    function test():
        %cond = const true
        br %cond, label %true_block, label %false_block
      %true_block:
        return 1
      %false_block:
        return 2
    "#;
    
    let mut module = parse_ir(input);
    
    let mut sccp = ConstantFoldingOptimizer::new();
    sccp.run(&mut module).unwrap();
    
    insta::assert_snapshot!("sccp_branch_resolution", module.to_string());
}
```

---

## Next Steps

1. **Review API Contracts**: See `contracts/` directory for detailed API documentation
2. **Explore Research**: Read `research.md` for algorithm details and design decisions
3. **Understand Data Model**: Study `data-model.md` for entity relationships
4. **Implementation**: Proceed to Phase 2 (tasks.md) for development plan

---

## Reference

### Configuration Options

| Option           | Type    | Default | Description                        |
| ---------------- | ------- | ------- | ---------------------------------- |
| `verbose`        | `bool`  | `false` | Enable diagnostic output           |
| `max_iterations` | `usize` | `100`   | Maximum propagation iterations     |

### Statistics Fields

| Field                       | Type    | Description                                     |
| --------------------------- | ------- | ----------------------------------------------- |
| `constants_propagated`      | `usize` | Instructions replaced with constants            |
| `branches_resolved`         | `usize` | Conditional branches converted to unconditional |
| `phi_nodes_simplified`      | `usize` | Phi nodes replaced with constants               |
| `blocks_marked_unreachable` | `usize` | Blocks marked for DCE                           |
| `iterations`                | `usize` | SCCP iterations to convergence                  |

---

**Quickstart Status**: ✅ Complete  
**Last Updated**: 2025-12-05  
**Maintainer**: jsavrs Optimizer Team
