# Quickstart Guide: Constant Folding Optimizer

**Feature**: `015-constant-folding-sccp`  
**Date**: 2025-11-14  
**Purpose**: Practical guide for using and integrating the constant folding optimizer

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Basic Usage](#basic-usage)
3. [Configuration Options](#configuration-options)
4. [Integration Patterns](#integration-patterns)
5. [Understanding Output](#understanding-output)
6. [Common Scenarios](#common-scenarios)
7. [Troubleshooting](#troubleshooting)
8. [Performance Tips](#performance-tips)

---

## Quick Start

### 30-Second Integration

```rust
use jsavrs::ir::optimizer::constant_folding::ConstantFoldingOptimizer;
use jsavrs::ir::optimizer::Phase;
use jsavrs::ir::Module;

fn main() {
    let mut module = /* load your IR module */;
    
    // Create optimizer (verbose=false, sccp=true)
    let mut optimizer = ConstantFoldingOptimizer::new(false, true);
    
    // Run optimization
    optimizer.run(&mut module);
    
    // Module is now optimized in-place
}
```

**Output** (to stdout):
```
total number of instructions after constant folding: 1234
```

---

## Basic Usage

### Example 1: Simple Constant Folding (No SCCP)

For basic constant expression evaluation without control flow analysis:

```rust
use jsavrs::ir::optimizer::constant_folding::ConstantFoldingOptimizer;
use jsavrs::ir::optimizer::Phase;

fn optimize_simple(module: &mut Module) {
    let mut optimizer = ConstantFoldingOptimizer::new(
        false,  // verbose: no statistics
        false   // sccp: basic folding only
    );
    
    optimizer.run(module);
}
```

**When to use**:
- Quick compilation cycles (minimal overhead)
- Code without complex control flow
- When SCCP overhead is not justified

**Performance**: ~0.1-0.5ms per 1000 instructions

---

### Example 2: Full SCCP Optimization

For comprehensive constant propagation with control flow analysis:

```rust
fn optimize_full(module: &mut Module) {
    let mut optimizer = ConstantFoldingOptimizer::new(
        true,  // verbose: print statistics to stderr
        true   // sccp: full analysis
    );
    
    optimizer.run(module);
}
```

**Output** (stdout):
```
total number of instructions after constant folding: 867
```

**Output** (stderr, verbose mode):
```
=== Constant Folding Optimizer Statistics ===
Functions processed: 12
Instructions removed: 245
Constants folded: 189
Loads propagated: 34
Branches resolved: 8
Blocks removed: 14
===========================================
```

**When to use**:
- Release builds (maximum optimization)
- Code with conditional branches and loops
- When instruction count reduction is critical

**Performance**: ~0.5-1.0ms per 1000 instructions

---

## Configuration Options

### Verbose Mode

**Purpose**: Enable detailed per-function statistics for profiling and debugging

```rust
let optimizer = ConstantFoldingOptimizer::new(
    true,  // ← verbose mode
    true
);
```

**Output Details**:
- Total functions processed
- Instructions removed (before/after counts)
- Constants folded (arithmetic, logical, casts)
- Loads propagated (store-to-load elimination)
- Branches resolved (constant conditions)
- Blocks removed (unreachable code)
- SCCP fallback warnings (if memory limit hit)

**Use Cases**:
- ✅ Debugging optimization effectiveness
- ✅ Profiling which functions benefit most
- ✅ CI/CD performance tracking
- ❌ Production builds (overhead ~5-10%)

---

### SCCP Mode

**Purpose**: Enable Sparse Conditional Constant Propagation with CFG analysis

```rust
let optimizer = ConstantFoldingOptimizer::new(
    false,
    true   // ← SCCP mode
);
```

**What SCCP Adds**:
- Control flow analysis (identify unreachable blocks)
- Phi node simplification (merge constant values)
- Branch resolution (constant conditions → unconditional branches)
- Interprocedural constant propagation (future enhancement)

**Trade-offs**:
- **+30-50%** optimization effectiveness (more instructions removed)
- **+2-3x** analysis time (still <1ms per 1000 instructions)
- **+100KB** memory per function (bounded, released after processing)

**Use Cases**:
- ✅ Code with if/else chains, switch statements
- ✅ Loops with constant iteration counts
- ✅ Release builds, final optimization passes
- ❌ Debug builds (prefer fast compilation)

---

## Integration Patterns

### Pattern 1: Standalone Optimizer

Use as the only optimization pass:

```rust
fn compile(source: &str) -> Module {
    let mut module = parse_and_generate_ir(source);
    
    let mut optimizer = ConstantFoldingOptimizer::new(false, true);
    optimizer.run(&mut module);
    
    module
}
```

---

### Pattern 2: Multi-Pass Optimization Pipeline

Combine with other optimization passes for maximum effectiveness:

```rust
fn optimize_pipeline(module: &mut Module) {
    // Pass 1: Constant folding (simplifies code)
    let mut const_fold = ConstantFoldingOptimizer::new(false, true);
    const_fold.run(module);
    
    // Pass 2: Dead code elimination (removes unreferenced values)
    let mut dce = DeadCodeElimination::new();
    dce.run(module);
    
    // Pass 3: Second constant folding (propagate DCE results)
    let mut const_fold2 = ConstantFoldingOptimizer::new(false, true);
    const_fold2.run(module);
    
    // Pass 4: Inline small functions (if implemented)
    // let mut inliner = FunctionInliner::new(threshold=100);
    // inliner.run(module);
    
    // Pass 5: Final constant folding
    let mut const_fold3 = ConstantFoldingOptimizer::new(true, true);
    const_fold3.run(module);
}
```

**Rationale**: Each pass exposes opportunities for subsequent passes (positive feedback loop).

---

### Pattern 3: Conditional Optimization (Build Modes)

```rust
fn optimize_for_build_mode(module: &mut Module, release: bool) {
    let (verbose, sccp) = if release {
        (true, true)   // Full optimization + statistics
    } else {
        (false, false) // Fast compilation, minimal overhead
    };
    
    let mut optimizer = ConstantFoldingOptimizer::new(verbose, sccp);
    optimizer.run(module);
}
```

---

### Pattern 4: Custom Optimization Levels

```rust
enum OptLevel {
    O0, // No optimization
    O1, // Basic constant folding
    O2, // SCCP enabled
    O3, // Multi-pass with SCCP
}

fn optimize(module: &mut Module, level: OptLevel) {
    match level {
        OptLevel::O0 => {
            // No optimization
        },
        OptLevel::O1 => {
            let mut opt = ConstantFoldingOptimizer::new(false, false);
            opt.run(module);
        },
        OptLevel::O2 => {
            let mut opt = ConstantFoldingOptimizer::new(false, true);
            opt.run(module);
        },
        OptLevel::O3 => {
            // Multi-pass pipeline
            for _ in 0..3 {
                let mut opt = ConstantFoldingOptimizer::new(false, true);
                opt.run(module);
            }
        }
    }
}
```

---

## Understanding Output

### Stdout: Instruction Count

**Format**:
```
total number of instructions after constant folding: <count>
```

**Purpose**: Machine-readable metric for CI/CD tracking

**Example Usage** (in CI):
```bash
#!/bin/bash
output=$(./jsavrs -i input.vn | grep "total number of instructions")
count=$(echo $output | awk '{print $7}')

if [ $count -gt $threshold ]; then
    echo "ERROR: Instruction count $count exceeds threshold $threshold"
    exit 1
fi
```

---

### Stderr: Verbose Statistics

**Format** (when `verbose=true`):
```
=== Constant Folding Optimizer Statistics ===
Functions processed: <N>
Instructions removed: <R>
Constants folded: <F>
Loads propagated: <L>
Branches resolved: <B>
Blocks removed: <D>
SCCP fallbacks (memory limit): <M>
===========================================
```

**Metrics Explained**:
- **Functions processed**: Total functions in module
- **Instructions removed**: Total reduction (before - after)
- **Constants folded**: Arithmetic/logical/cast operations evaluated
- **Loads propagated**: Load instructions replaced with constants
- **Branches resolved**: Conditional branches made unconditional
- **Blocks removed**: Unreachable blocks eliminated
- **SCCP fallbacks**: Functions that exceeded 100KB lattice limit

---

### Warnings

#### Warning: SCCP Memory Limit

```
Warning: SCCP lattice memory limit exceeded for function 'large_function', falling back to basic constant folding
```

**Cause**: Function has >10,000 SSA values, exceeding 100KB lattice limit

**Impact**: Function optimized with basic folding only (still correct, just less effective)

**Solution**:
- Accept the fallback (conservative, safe)
- Refactor function into smaller pieces
- Increase memory limit (requires code change)

---

#### Warning: Invalid SSA Reference

```
Warning: Invalid SSA value reference %123 in constant folding, preserving instruction
```

**Cause**: IR generator produced invalid SSA reference (bug in IR generation)

**Impact**: Single instruction not optimized (rest of function proceeds)

**Solution**: Fix IR generator to produce valid SSA form

---

#### Warning: Missing CFG Information

```
Warning: CFG information missing for function 'malformed_func', skipping control-flow optimizations
```

**Cause**: Function lacks CFG metadata (incomplete IR)

**Impact**: Basic folding only, no SCCP or branch resolution

**Solution**: Ensure IR generator produces complete CFG information

---

## Common Scenarios

### Scenario 1: Optimizing Arithmetic-Heavy Code

**Input Program**:
```c
int compute() {
    int a = 10 + 20;
    int b = a * 2;
    return b + 5;
}
```

**IR Before Optimization**:
```
%1 = add i32 10, 20        ; a = 30
%2 = mul i32 %1, 2         ; b = 60
%3 = add i32 %2, 5         ; return 65
ret i32 %3
```

**IR After Optimization**:
```
ret i32 65
```

**Metrics**:
- Instructions before: 4
- Instructions after: 1
- Constants folded: 3
- Reduction: 75%

---

### Scenario 2: Branch Resolution

**Input Program**:
```c
int conditional() {
    if (true) {
        return 42;
    } else {
        return 0;  // Dead code
    }
}
```

**IR Before Optimization**:
```
entry:
  br i1 true, label %then, label %else

then:
  ret i32 42

else:
  ret i32 0   ; Unreachable
```

**IR After Optimization**:
```
entry:
  br label %then

then:
  ret i32 42

; %else block removed
```

**Metrics**:
- Instructions before: 3
- Instructions after: 2
- Branches resolved: 1
- Blocks removed: 1

---

### Scenario 3: Phi Node Simplification

**Input Program**:
```c
int phi_example(bool flag) {
    int x;
    if (flag) {
        x = 10;
    } else {
        x = 10;  // Same value
    }
    return x * 2;
}
```

**IR Before Optimization**:
```
entry:
  br i1 %flag, label %then, label %else

then:
  br label %merge

else:
  br label %merge

merge:
  %x = phi i32 [10, %then], [10, %else]
  %result = mul i32 %x, 2
  ret i32 %result
```

**IR After Optimization**:
```
entry:
  br i1 %flag, label %then, label %else

then:
  br label %merge

else:
  br label %merge

merge:
  ret i32 20  ; Phi simplified to 10, then mul folded
```

**Metrics**:
- Instructions before: 7
- Instructions after: 5
- Constants folded: 2 (phi merge + multiplication)

---

## Troubleshooting

### Issue: No Optimization Occurring

**Symptoms**: Instruction count unchanged after optimization

**Possible Causes**:
1. No constant expressions in code (all runtime values)
2. SCCP disabled and no simple constants
3. IR malformed (warnings emitted)

**Diagnostic**:
```rust
let mut optimizer = ConstantFoldingOptimizer::new(true, true);
optimizer.run(&mut module);
// Check stderr for warnings
```

**Solutions**:
- Verify input program has constant expressions
- Enable verbose mode to see detailed metrics
- Check for IR generation bugs

---

### Issue: SCCP Fallback Frequent

**Symptoms**: Many "SCCP fallback" warnings

**Cause**: Functions have very large SSA value counts

**Solutions**:
1. Refactor large functions into smaller ones
2. Run basic folding first, then SCCP (may reduce value count)
3. Increase memory limit (code change required)

---

### Issue: Slow Compilation

**Symptoms**: Optimization takes >1 second per 1000 instructions

**Diagnostic**:
- Check function sizes (should be <5000 instructions for <1s)
- Verify no infinite loops in worklist algorithm (should not happen)

**Solutions**:
- Disable SCCP for very large functions
- Profile optimizer with `cargo flamegraph`
- Report performance regression

---

## Performance Tips

### Tip 1: Two-Pass Optimization

For maximum instruction reduction:

```rust
// Pass 1: Basic folding (fast)
let mut opt1 = ConstantFoldingOptimizer::new(false, false);
opt1.run(&mut module);

// Pass 2: SCCP (exposes opportunities from Pass 1)
let mut opt2 = ConstantFoldingOptimizer::new(false, true);
opt2.run(&mut module);
```

**Benefit**: Basic pass reduces SSA value count, making SCCP faster and more effective.

---

### Tip 2: Disable Verbose in Production

```rust
#[cfg(debug_assertions)]
let verbose = true;

#[cfg(not(debug_assertions))]
let verbose = false;

let optimizer = ConstantFoldingOptimizer::new(verbose, true);
```

**Benefit**: Avoid 5-10% overhead of statistics formatting in release builds.

---

### Tip 3: Parallelize Function Optimization

```rust
use rayon::prelude::*;

fn optimize_parallel(module: &mut Module) {
    module.functions_mut().par_iter_mut().for_each(|function| {
        let mut optimizer = ConstantFoldingOptimizer::new(false, true);
        let mut single_fn_module = Module::from_function(function.clone());
        optimizer.run(&mut single_fn_module);
        *function = single_fn_module.functions()[0].clone();
    });
}
```

**Note**: Requires safe parallelization infrastructure (not included in this implementation).

---

## Next Steps

- **Implement**: See `contracts/` for detailed API specifications
- **Test**: Refer to test file patterns in `plan.md`
- **Extend**: Add new folding rules in `evaluator.rs`
- **Profile**: Use `cargo bench` to measure performance improvements

---

## Resources

- **Research**: See `research.md` for algorithm background
- **Data Model**: See `data-model.md` for internal structures
- **Contracts**: See `contracts/` for detailed API documentation
- **SCCP Paper**: Wegman & Zadeck, "Constant Propagation with Conditional Branches" (1991)
