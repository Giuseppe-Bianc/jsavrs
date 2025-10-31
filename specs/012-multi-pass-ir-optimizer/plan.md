# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

**Language/Version**: Rust 1.75+ (current stable used by jsavrs project)  
**Primary Dependencies**: petgraph (for CFG representation), bit-vec (for dense boolean sets in dataflow), existing jsavrs IR infrastructure (Module, Function, BasicBlock, Instruction, Value, ControlFlowGraph from petgraph DiGraph, DominanceInfo, SsaTransformer) with zero external dependencies beyond petgraph already used for CFG representation
**Storage**: In-memory data structures (HashMap, Vec, BitVec) for analysis results, no persistent storage required
**Testing**: cargo test for unit and integration tests, proptest for property-based testing, criterion for benchmarking
**Target Platform**: Multi-platform (Windows, macOS, Linux) as per jsavrs cross-platform compatibility principle
**Project Type**: Single compiler component module within existing jsavrs compiler codebase
**Performance Goals**: < 30% compile-time overhead for O1, < 100% for O2 relative to baseline compilation; >= 5% median instruction-count reduction at O2; optimizer verification must report zero unchecked SSA/CFG errors for >= 95% of Functions
**Constraints**: Must maintain SSA invariants, preserve debug information for 90%+ of remaining instructions, respect configurable thresholds (max iterations=10, loop unroll threshold=4), handle external/FFI calls conservatively
**Scale/Scope**: Designed for typical compiler Functions and Modules within jsavrs codebase, with configurable optimization levels (O0, O1, O2, O3)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Safety First**: Implementation must leverage Rust's ownership model with zero unsafe code except where present in petgraph graph indexing; use proper Result<T, OptimizerError> for error handling where OptimizerError enum contains VerificationFailed, AnalysisFailed, and PassError variants; ensure memory safety during transformations by properly maintaining SSA form and CFG integrity.

**Performance Excellence**: All data structures must be optimized using Vec for instruction sequences enabling cache-friendly iteration, HashMap with FxHasher for faster hashing of small keys like ValueId, BitVec from bit-vec crate for dense boolean sets in dataflow; use petgraph DiGraph<BasicBlock, ()> reused from existing CFG to avoid duplication; implement appropriate algorithms with optimal complexity for analysis and transformation passes.

**Cross-Platform Compatibility**: Ensure the optimizer works consistently across Windows, macOS, and Linux; verify that all performance measurements and thresholds work consistently across platforms; maintain consistent behavior regardless of host environment.

**Modular Extensibility**: The optimizer architecture must follow three layer design: analysis framework in src/ir/optimizer/analysis/, transformation passes in src/ir/optimizer/passes/, and verification in src/ir/optimizer/verification/ with clearly defined interfaces; implement trait-based designs (Analysis trait, OptimizationPass trait) for extensibility; support plugin/pass manager allowing external passes to register.

**Test-Driven Reliability**: All optimization passes must include comprehensive unit tests with hand-built CFGs, integration tests loading source files and verifying output equivalence, and property tests using proptest to validate SSA/CFG invariants preserved via quickcheck_ssa_preservation; use cargo test framework and insta for snapshot validation.

**Snapshot Validation**: Use Insta library for snapshot testing to ensure consistent output and catch regressions in optimized IR; validate that SSA form, CFG well-formedness, and type consistency are preserved across all transformations; maintain reproducible benchmarks using criterion.

**Documentation Rigor**: Document public APIs with comprehensive rustdoc comments including Examples section showing usage; create detailed research.md and data_model.md files with complete precision and meticulous attention to detail; document all new modules and components following existing project conventions.

## Project Structure

### Documentation (this feature)

```text
specs/012-multi-pass-ir-optimizer/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
└── ir/
    └── optimizer/                 # Main optimizer module
        ├── mod.rs                 # Main entry point with optimize_module function
        ├── analysis/              # Analysis framework
        │   ├── mod.rs             # Analysis trait and AnalysisManager
        │   ├── use_def.rs         # UseDefManager with use-def and def-use chains
        │   ├── reaching_defs.rs   # ReachingDefinitions analysis using worklist algorithm
        │   ├── live_vars.rs       # LiveVariables analysis via backward dataflow
        │   ├── constants.rs       # ConstantLattice for sparse conditional constant propagation
        │   ├── alias.rs           # AliasAnalysis trait with AndersenAnalysis and ConservativeAnalysis
        │   ├── loops.rs           # LoopInfo detecting natural loops using dominator tree
        │   └── gvn.rs             # GlobalValueNumbering for expression hashing
        ├── passes/                # Transformation passes
        │   ├── mod.rs             # OptimizationPass trait and pass implementations
        │   ├── sccp.rs            # Sparse Conditional Constant Propagation pass
        │   ├── dce.rs             # Aggressive Dead Code Elimination pass
        │   ├── copy_prop.rs       # Copy Propagation pass
        │   ├── gvn_cse.rs         # Global Value Numbering + Common Subexpression Elimination
        │   ├── licm.rs            # Loop Invariant Code Motion pass
        │   ├── iv_opt.rs          # Induction Variable Optimization pass
        │   ├── loop_unroll.rs     # Loop Unrolling pass
        │   ├── instruction_combining.rs # Instruction Combining pass
        │   ├── algebraic_simp.rs  # Algebraic Simplification pass
        │   ├── strength_red.rs    # Strength Reduction pass
        │   ├── phi_opt.rs         # Phi Node Optimization pass
        │   ├── store_to_load.rs   # Store-to-Load Forwarding pass
        │   ├── redundant_loads.rs # Redundant Load Elimination pass
        │   └── dead_store.rs      # Dead Store Elimination pass
        ├── verification/          # Verification infrastructure
        │   ├── mod.rs             # Main verification functions
        │   ├── ssa_check.rs       # SSA form verification
        │   ├── cfg_check.rs       # CFG consistency verification
        │   ├── type_check.rs      # Type consistency verification
        │   └── rollback.rs        # FunctionSnapshot for verification rollback
        ├── pass/                  # Pass management
        │   └── manager.rs         # PassManager with optimization pipeline
        ├── config.rs              # OptimizerConfig and OptLevel enum
        └── metrics.rs             # PassMetrics and OptimizerReport

tests/
├── sccp_tests.rs              # Unit tests for Sparse Conditional Constant Propagation
├── dce_tests.rs               # Unit tests for Dead Code Elimination
├── gvn_tests.rs               # Unit tests for Global Value Numbering
├── licm_tests.rs              # Unit tests for Loop Invariant Code Motion
├── integration/               # Integration tests
│   ├── basic_optimization.rs  # Basic optimization integration tests
│   ├── loop_optimization.rs   # Loop optimization integration tests
│   └── memory_optimization.rs # Memory optimization integration tests
└── property/                # Property-based tests
    └── ssa_preservation.rs    # SSA preservation property tests

benches/
└── optimizer_bench.rs         # Criterion benchmarks for optimization performance
```

**Structure Decision**: The optimizer is implemented as a submodule within the existing jsavrs IR module (`src/ir/optimizer/`) following the modular extensibility principle. The architecture is organized into three layers: analysis framework, transformation passes, and verification infrastructure, with additional modules for configuration, metrics, and pass management. The test structure includes unit tests for individual passes, integration tests for complete optimization workflows, and property tests for verifying invariants.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
