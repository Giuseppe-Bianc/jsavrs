# Quickstart Guide: Multi-pass IR Optimizer

**Feature**: 012-multi-pass-ir-optimizer  
**Date**: 2025-11-01  
**Purpose**: Practical guide for integrating and using the optimizer in the jsavrs compilation pipeline

## Overview

The multi-pass IR optimizer transforms SSA-form Modules through systematic analysis and transformation passes while guaranteeing semantic preservation. This guide covers:

1. **Integration** - Adding optimizer to compilation pipeline
2. **Configuration** - Selecting optimization levels and passes
3. **Usage** - Running optimizer and interpreting results
4. **Extension** - Adding custom passes and analyses
5. **Debugging** - Troubleshooting optimization issues

---

## 1. Integration with Compilation Pipeline

### 1.1 Minimal Integration

Add optimizer phase after SSA transformation in `src/main.rs`:

```rust
use jsavrs::ir::optimizer::{optimize_module, OptimizerConfig, OptLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (existing lexer, parser, semantic analysis) ...
    
    // Generate IR
    let mut module = nir_generator.generate(&ast)?;
    
    // Transform to SSA form
    let ssa_transformer = SsaTransformer::new();
    ssa_transformer.transform(&mut module)?;
    
    // **NEW**: Optimize (if --opt-level > O0)
    let opt_level = args.opt_level.unwrap_or(OptLevel::O0);
    if opt_level != OptLevel::O0 {
        let config = OptimizerConfig::config_for_level(opt_level);
        let report = optimize_module(&mut module, config)?;
        
        if args.verbose {
            println!("Optimization Report:");
            println!("{}", report);
        }
    }
    
    // Generate code
    let asm = code_generator.generate(&module)?;
    
    Ok(())
}
```

### 1.2 CLI Argument Parsing

Add `--opt-level` flag in `src/cli.rs`:

```rust
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    // ... existing args ...
    
    /// Optimization level (O0, O1, O2, O3)
    #[arg(short = 'O', long, value_enum, default_value = "o0")]
    pub opt_level: OptLevel,
    
    /// Verbose output (includes optimization metrics)
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OptLevel {
    #[value(name = "0")]
    O0,
    #[value(name = "1")]
    O1,
    #[value(name = "2")]
    O2,
    #[value(name = "3")]
    O3,
}
```

**Usage**:
```bash
# No optimization (fast compilation)
jsavrs compile input.js

# Basic optimization
jsavrs compile -O1 input.js

# Full optimization with verbose output
jsavrs compile -O2 --verbose input.js

# Aggressive optimization
jsavrs compile -O3 input.js
```

---

## 2. Configuration

### 2.1 Optimization Levels

**O0 (No Optimization)**:
- **Purpose**: Fast compilation, debugging
- **Passes**: None
- **Compile Time**: Baseline
- **Output Size**: Largest
- **Use Case**: Development, debugging

**O1 (Basic Optimization)**:
- **Purpose**: Moderate improvement with minimal compile-time cost
- **Passes**: SCCP (sparse conditional constant propagation), ADCE (aggressive dead code elimination)
- **Max Iterations**: 1 (single pass)
- **Alias Analysis**: Conservative (may-alias for all pointers except provably distinct)
- **Compile Time**: +10-30% vs O0
- **Output Size**: 5-15% smaller than O0
- **Use Case**: Development with some optimization

**O2 (Full Optimization)**:
- **Purpose**: Production builds, balance between speed and size
- **Passes**: All passes (SCCP, ADCE, Copy Prop, GVN/CSE, LICM, IV Opt, Loop Unroll, Inst Combine, Algebraic Simp, Strength Reduction, Phi Opt, Memory Opts)
- **Max Iterations**: 10 (fixed-point convergence)
- **Alias Analysis**: Andersen's inclusion-based points-to analysis
- **Loop Unroll Threshold**: 4 iterations
- **Compile Time**: +50-100% vs O0
- **Output Size**: 15-30% smaller than O0
- **Use Case**: Production builds

**O3 (Aggressive Optimization)**:
- **Purpose**: Maximum performance at any compile-time cost
- **Passes**: Same as O2 with increased thresholds
- **Max Iterations**: 10
- **Alias Analysis**: Andersen's
- **Loop Unroll Threshold**: 8 iterations
- **Compile Time**: +100-200% vs O0
- **Output Size**: 20-35% smaller than O0 (may be larger due to unrolling)
- **Use Case**: Performance-critical production builds

### 2.2 Custom Configuration

```rust
use jsavrs::ir::optimizer::{
    OptimizerConfig, OptLevel, AliasAnalysisKind,
    passes::{Sccp, Adce, CopyPropagation, GvnCse, Licm},
};

// Start with O2 configuration
let mut config = OptimizerConfig::config_for_level(OptLevel::O2);

// Customize passes
config.early_passes.clear();
config.early_passes.push(Box::new(Sccp));
config.early_passes.push(Box::new(Adce));

// Increase unroll threshold
config.loop_unroll_threshold = 16;

// Force conservative alias analysis for faster compilation
config.alias_analysis_kind = AliasAnalysisKind::Conservative;

// Enable provenance tracking for debugging
config.record_provenance = true;

// Run with custom config
let report = optimize_module(&mut module, config)?;
```

---

## 3. Usage

### 3.1 Basic Usage

```rust
use jsavrs::ir::optimizer::{optimize_module, OptimizerConfig, OptLevel};

// After SSA transformation
let config = OptimizerConfig::config_for_level(OptLevel::O2);
let report = optimize_module(&mut module, config)?;

println!("Optimization completed:");
println!("  Instructions eliminated: {}", report.aggregate_metrics.total_instructions_eliminated);
println!("  Reduction: {:.1}%", report.aggregate_metrics.reduction_percentage);
println!("  Time: {:?}", report.aggregate_metrics.total_elapsed);
```

### 3.2 Interpreting OptimizerReport

```rust
pub struct OptimizerReport {
    /// Per-pass metrics
    pub per_pass_metrics: Vec<(String, PassMetrics)>,
    
    /// Aggregate metrics
    pub aggregate_metrics: AggregateMetrics,
}

// Example output
for (pass_name, metrics) in &report.per_pass_metrics {
    println!("Pass: {}", pass_name);
    println!("  Instructions eliminated: {}", metrics.instructions_eliminated);
    println!("  Constants propagated: {}", metrics.constants_propagated);
    println!("  CSE hits: {}", metrics.cse_hits);
    println!("  Time: {:?}", metrics.elapsed);
}

println!("\nAggregate:");
println!("  Before: {} instructions", report.aggregate_metrics.instruction_count_before);
println!("  After: {} instructions", report.aggregate_metrics.instruction_count_after);
println!("  Reduction: {:.1}%", report.aggregate_metrics.reduction_percentage);
println!("  Total time: {:?}", report.aggregate_metrics.total_elapsed);
```

**Example Output**:
```
Pass: SCCP
  Instructions eliminated: 42
  Constants propagated: 15
  CSE hits: 0
  Time: 1.2ms

Pass: ADCE
  Instructions eliminated: 18
  Constants propagated: 0
  CSE hits: 0
  Time: 0.8ms

Pass: GVN
  Instructions eliminated: 7
  Constants propagated: 0
  CSE hits: 7
  Time: 2.1ms

Aggregate:
  Before: 320 instructions
  After: 253 instructions
  Reduction: 20.9%
  Total time: 12.4ms
```

---

## 4. Extension

### 4.1 Adding a Custom Pass

**Step 1**: Implement `OptimizationPass` trait:

```rust
// src/ir/optimizer/passes/my_custom_pass.rs

use crate::ir::optimizer::{
    OptimizationPass, PassResult, PassMetrics, AnalysisKind,
};
use crate::ir::{Function, Instruction, InstructionKind};
use std::time::Instant;

pub struct MyCustomPass;

impl OptimizationPass for MyCustomPass {
    fn name(&self) -> &'static str {
        "MyCustomPass"
    }
    
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::UseDef]
    }
    
    fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::UseDef]
    }
    
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
        let start = Instant::now();
        let mut changed = false;
        let mut metrics = PassMetrics::default();
        
        // Get required analyses
        let use_def = analysis_mgr.get_analysis::<UseDefManager>(
            function,
            AnalysisKind::UseDef
        );
        
        // Perform optimization
        for block in function.cfg.blocks.values_mut() {
            for instruction in &mut block.instructions {
                if self.can_optimize(instruction, use_def) {
                    self.optimize_instruction(instruction);
                    changed = true;
                    metrics.instructions_eliminated += 1;
                }
            }
        }
        
        metrics.elapsed = start.elapsed();
        PassResult { changed, metrics }
    }
}

impl MyCustomPass {
    fn can_optimize(&self, instruction: &Instruction, use_def: &UseDefManager) -> bool {
        // Custom optimization logic
        false
    }
    
    fn optimize_instruction(&self, instruction: &mut Instruction) {
        // Modify instruction
    }
}
```

**Step 2**: Register pass in configuration:

```rust
use crate::ir::optimizer::passes::MyCustomPass;

let mut config = OptimizerConfig::config_for_level(OptLevel::O2);
config.middle_passes.push(Box::new(MyCustomPass));

let report = optimize_module(&mut module, config)?;
```

### 4.2 Adding a Custom Analysis

**Step 1**: Implement `Analysis` trait:

```rust
// src/ir/optimizer/analysis/my_custom_analysis.rs

use crate::ir::optimizer::Analysis;
use crate::ir::Function;
use std::collections::HashMap;

pub struct MyCustomAnalysis {
    data: HashMap<String, MyData>,
}

struct MyData {
    // Analysis result data
}

impl Analysis for MyCustomAnalysis {
    fn compute(function: &Function) -> Self {
        let mut data = HashMap::new();
        
        // Scan function and compute analysis
        for (block_label, block) in &function.cfg.blocks {
            let my_data = MyData { /* ... */ };
            data.insert(block_label.clone(), my_data);
        }
        
        MyCustomAnalysis { data }
    }
    
    fn invalidate(&mut self) {
        self.data.clear();
    }
}

impl MyCustomAnalysis {
    pub fn query(&self, block: &str) -> Option<&MyData> {
        self.data.get(block)
    }
}
```

**Step 2**: Add to `AnalysisKind` enum:

```rust
// src/ir/optimizer/analysis/mod.rs

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisKind {
    // ... existing variants ...
    MyCustom,
}
```

**Step 3**: Use in pass:

```rust
impl OptimizationPass for MyPass {
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::MyCustom]
    }
    
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
        let my_analysis = analysis_mgr.get_analysis::<MyCustomAnalysis>(
            function,
            AnalysisKind::MyCustom
        );
        
        let data = my_analysis.query("block_label");
        // ... use analysis result ...
    }
}
```

---

## 5. Debugging

### 5.1 Verification Failures

When verification fails, the optimizer automatically rolls back:

```
Error: Verification failed in pass SCCP
Reason: DuplicateDefinition { value_id: %42, block: "bb2" }
Action: Rollback to pre-pass state, continue with next function
```

**Debugging steps**:

1. **Enable verbose logging**:
   ```rust
   env_logger::init();  // Set RUST_LOG=debug
   ```

2. **Inspect pre/post IR**:
   ```rust
   if args.dump_ir {
       println!("Before optimization:");
       println!("{}", module);
       
       let report = optimize_module(&mut module, config)?;
       
       println!("After optimization:");
       println!("{}", module);
   }
   ```

3. **Isolate failing pass**:
   ```rust
   // Test passes individually
   let mut config = OptimizerConfig::default();
   config.early_passes.push(Box::new(Sccp));
   config.max_iterations = 1;
   
   let report = optimize_module(&mut module, config)?;
   ```

### 5.2 Performance Issues

**Symptom**: Compilation time too high at O2/O3.

**Solutions**:

1. **Reduce max iterations**:
   ```rust
   config.max_iterations = 5;  // Default: 10
   ```

2. **Use conservative alias analysis**:
   ```rust
   config.alias_analysis_kind = AliasAnalysisKind::Conservative;
   ```

3. **Disable expensive passes**:
   ```rust
   config.middle_passes.retain(|p| p.name() != "LoopUnroll");
   ```

4. **Profile optimizer**:
   ```bash
   cargo build --release
   perf record --call-graph=dwarf ./target/release/jsavrs compile -O2 input.js
   perf report
   ```

### 5.3 Unexpected Output Changes

**Symptom**: Optimized program produces different output than unoptimized.

**Debugging**:

1. **Verify semantic preservation**:
   ```bash
   # Run with and without optimization
   jsavrs compile -O0 input.js -o unopt.asm
   jsavrs compile -O2 input.js -o opt.asm
   
   # Execute both and compare outputs
   ./unopt > unopt.out
   ./opt > opt.out
   diff unopt.out opt.out
   ```

2. **Bisect passes** (find culprit):
   ```rust
   // Enable half of passes, test, repeat
   ```

3. **Enable provenance tracking**:
   ```rust
   config.record_provenance = true;
   // Inspect Value::optimization_provenance to see transformation history
   ```

4. **Report bug** with minimal reproducer:
   ```
   Issue: SCCP incorrectly folds constant at block bb3
   Input: [minimal source code]
   Expected: [correct output]
   Actual: [incorrect output]
   Config: O2, max_iterations=10
   ```

---

## 6. Best Practices

### 6.1 When to Optimize

- **Always** for production builds (O2 or O3)
- **Sometimes** for development builds (O1 for moderate improvement)
- **Never** for debugging (O0 preserves source structure)

### 6.2 Optimization Level Selection

| Use Case | Optimization Level | Rationale |
|----------|-------------------|-----------|
| Development (edit-compile-test) | O0 | Fast compilation, easy debugging |
| Development with profiling | O1 | Some optimization without masking issues |
| CI/CD builds | O2 | Good balance for automated testing |
| Production releases | O2 or O3 | Maximum performance |
| Size-constrained targets | O2 | Good size reduction without excessive unrolling |
| Performance-critical code | O3 | Maximum speed at cost of larger binaries |

### 6.3 Performance Tuning

1. **Measure first**:
   ```bash
   hyperfine 'jsavrs compile -O0 input.js' 'jsavrs compile -O2 input.js'
   ```

2. **Profile hot paths**:
   ```bash
   cargo flamegraph --bin jsavrs -- compile -O2 large_input.js
   ```

3. **Tune incrementally**:
   - Start with O2
   - If compile time too high, reduce `max_iterations` or switch to O1
   - If output too large, increase `loop_unroll_threshold`

---

## 7. Common Patterns

### 7.1 Conditional Optimization

```rust
let config = if args.optimize_for_size {
    let mut cfg = OptimizerConfig::config_for_level(OptLevel::O2);
    cfg.loop_unroll_threshold = 1;  // Minimize unrolling
    cfg
} else if args.optimize_for_speed {
    OptimizerConfig::config_for_level(OptLevel::O3)
} else {
    OptimizerConfig::config_for_level(OptLevel::O1)
};
```

### 7.2 Function-Level Opt Level

```rust
for function in &mut module.functions {
    let opt_level = if function.is_hot() {
        OptLevel::O3  // Aggressive for hot functions
    } else {
        OptLevel::O1  // Basic for cold functions
    };
    
    let config = OptimizerConfig::config_for_level(opt_level);
    // ... (run on single function)
}
```

### 7.3 Incremental Optimization

```rust
// First pass: basic optimizations
let config_o1 = OptimizerConfig::config_for_level(OptLevel::O1);
let report1 = optimize_module(&mut module, config_o1)?;

if report1.aggregate_metrics.reduction_percentage > 10.0 {
    // Second pass: full optimizations (if first pass was effective)
    let config_o2 = OptimizerConfig::config_for_level(OptLevel::O2);
    let report2 = optimize_module(&mut module, config_o2)?;
}
```

---

## Next Steps

1. **Implement core infrastructure** (PassManager, AnalysisManager, verification)
2. **Implement analyses** (UseDefManager, ReachingDefinitions, LoopInfo, etc.)
3. **Implement passes** (SCCP, ADCE, GVN/CSE, LICM, etc.)
4. **Test thoroughly** (unit tests, integration tests, benchmarks)
5. **Integrate with pipeline** (main.rs, cli.rs)
6. **Document** (rustdoc for all public APIs)

See `tasks.md` (generated by `/speckit.tasks`) for detailed implementation plan.

---

## Resources

- **Research Document**: `specs/012-multi-pass-ir-optimizer/research.md` (algorithms and data structures)
- **Data Model**: `specs/012-multi-pass-ir-optimizer/data-model.md` (detailed entity specifications)
- **Contracts**: `specs/012-multi-pass-ir-optimizer/contracts/` (trait definitions and guarantees)
- **LLVM Passes**: https://llvm.org/docs/Passes.html (reference implementations)
- **SSA Book**: "SSA-based Compiler Design" (Zadeck et al.) (theoretical foundation)

---

**Version**: 1.0  
**Last Updated**: 2025-11-01
