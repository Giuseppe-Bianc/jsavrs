# Quickstart Guide: Multi-pass IR Optimizer for jsavrs

## Overview
This guide provides a quick introduction to using and implementing the multi-pass IR optimizer for the jsavrs compiler. The optimizer transforms SSA-form IR Modules through rigorous analysis and systematic transformations while guaranteeing semantic preservation.

## Prerequisites
- Rust 1.75+ installed
- Familiarity with the jsavrs compiler architecture
- Understanding of Static Single Assignment (SSA) form
- Knowledge of basic compiler optimization concepts

## Getting Started

### 1. Basic Optimizer Usage

To optimize a module with the default configuration:

```rust
use jsavrs::ir::optimizer::{optimize_module, OptimizerConfig, OptLevel};

// Create optimizer configuration
let config = OptimizerConfig::config_for_level(OptLevel::O2);

// Optimize your module
let (report, metrics) = optimize_module(&mut module, config)
    .expect("Optimization failed");

// Review the optimization report
println!("Optimization report: {:?}", report);
```

### 2. Configuration Options

The optimizer supports different optimization levels:

```rust
use jsavrs::ir::optimizer::{OptimizerConfig, OptLevel};

// No optimizations (fastest compilation)
let config_o0 = OptimizerConfig::config_for_level(OptLevel::O0);

// Basic optimizations
let config_o1 = OptimizerConfig::config_for_level(OptLevel::O1);

// Most optimizations (recommended for release builds)
let config_o2 = OptimizerConfig::config_for_level(OptLevel::O2);

// Aggressive optimizations (longer compile times)
let config_o3 = OptimizerConfig::config_for_level(OptLevel::O3);
```

### 3. Custom Configuration

You can customize the optimizer behavior:

```rust
use jsavrs::ir::optimizer::{OptimizerConfig, OptLevel, AliasAnalysisKind};

let mut config = OptimizerConfig::config_for_level(OptLevel::O2);
config.max_iterations = 15;  // Increase max iterations
config.loop_unroll_threshold = 8;  // Increase unroll threshold
config.record_provenance = true;  // Enable optimization tracking
config.alias_analysis_kind = AliasAnalysisKind::Andersen;  // Use Andersen analysis
```

## Architecture Overview

The optimizer follows a three-layer architecture:

### Analysis Framework (`src/ir/optimizer/analysis/`)
- Provides data flow analysis (reaching definitions, live variables, etc.)
- Implements use-def and def-use chain management
- Offers alias analysis for memory optimization

### Transformation Passes (`src/ir/optimizer/passes/`)
- Early passes (constant propagation, DCE, copy propagation)
- Middle passes (GVN/CSE, LICM, IV optimization)
- Late passes (instruction combining, algebraic simplification)

### Verification Infrastructure (`src/ir/optimizer/verification/`)
- Validates SSA form and CFG consistency
- Provides rollback mechanism for failed optimizations
- Ensures semantic preservation

## Adding a New Optimization Pass

To implement a new optimization pass:

1. Create a new file in `src/ir/optimizer/passes/`
2. Implement the `OptimizationPass` trait:

```rust
use jsavrs::ir::optimizer::OptimizationPass;

pub struct MyNewPass {
    // Pass-specific fields
}

impl OptimizationPass for MyNewPass {
    fn name(&self) -> &'static str {
        "my-new-pass"
    }
    
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        // Return required analysis kinds
        &[AnalysisKind::UseDef, AnalysisKind::LiveVars]
    }
    
    fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
        // Return invalidated analysis kinds
        &[AnalysisKind::UseDef]
    }
    
    fn run(&mut self, function: &mut Function, analysis_mgr: &AnalysisManager) -> Result<PassResult, PassError> {
        // Implementation of the optimization
        // Return Ok(PassResult { changed: bool, metrics: PassMetrics })
    }
}
```

3. Register your pass in the appropriate phase in `OptimizerConfig::config_for_level()`

## Running Tests

To run the optimizer tests:

```bash
# Run all optimizer tests
cargo test -p jsavrs --tests optimizer

# Run specific optimization pass tests
cargo test -p jsavrs --test sccp_tests

# Run integration tests
cargo test -p jsavrs --test basic_optimization

# Run property-based tests
cargo test -p jsavrs --test ssa_preservation
```

## Performance Benchmarks

To run performance benchmarks:

```bash
# Run optimizer-specific benchmarks
cargo bench -p jsavrs --bench optimizer_bench
```

## Debugging Optimized Code

To help debug optimization issues:

1. Enable provenance tracking in optimizer config:

```rust
let mut config = OptimizerConfig::config_for_level(OptLevel::O2);
config.record_provenance = true;
```

2. Use the metrics to analyze optimization effectiveness:

```rust
let (report, metrics) = optimize_module(&mut module, config)?;
println!("Instructions eliminated: {}", metrics.instructions_eliminated);
println!("Constants propagated: {}", metrics.constants_propagated);
```

## Common Issues and Troubleshooting

### SSA Violations
If you encounter SSA form violations:
- Check that all phi nodes have exactly one value per predecessor
- Ensure all values are defined before use
- Verify that definition dominates all uses

### Performance Issues
If optimization is too slow:
- Reduce max_iterations in config
- Use a lower optimization level
- Identify which pass is slowest using metrics

### Incorrect Optimizations
If optimizations change program semantics:
- Enable verification checks
- Use FunctionSnapshot to verify before/after states
- Report the issue with minimal reproduction case

## Next Steps

- Review the detailed research.md for in-depth technical analysis
- Explore the data-model.md for entity relationships
- Check the API contracts in the contracts/ directory
- Look at existing pass implementations for examples