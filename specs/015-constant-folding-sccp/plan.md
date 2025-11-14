# Implementation Plan: Constant Folding and Propagation Optimizer

**Branch**: `015-constant-folding-sccp` | **Date**: 2025-11-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/015-constant-folding-sccp/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a comprehensive constant folding and propagation optimizer as a native Rust compiler phase that eliminates compile-time evaluable operations and propagates constant values through SSA-based IR. The implementation uses Sparse Conditional Constant Propagation (SCCP) with worklist algorithm, integrates seamlessly with existing IR infrastructure, and follows Rust 1.91.1 best practices with no new external dependencies.

## Technical Context

**Language/Version**: Rust 1.91.1 (2024 edition)  
**Primary Dependencies**: 
- Standard library collections: `HashMap`, `HashSet`, `VecDeque`
- `petgraph` (already in Cargo.toml v0.8.3): CFG traversal operations
- `thiserror` (already in Cargo.toml v2.0.17): Error types
- `uuid` (already in Cargo.toml v1.18.1): SSA value IDs

**Storage**: N/A (operates on in-memory IR structures)  
**Testing**: 
- Cargo built-in test framework for unit and integration tests
- `insta` (already in dev-dependencies v1.43.2): Snapshot testing for IR transformations
- `criterion` (already in dev-dependencies v0.7.0): Performance benchmarks

**Target Platform**: Cross-platform (Windows, macOS, Linux) - compiler optimization phase  
**Project Type**: Single project (compiler phase module integrated into existing codebase)  
**Performance Goals**: 
- Process functions with 1000+ instructions in <1 second
- Linear or near-linear time complexity O(n) relative to instruction count
- SCCP lattice memory bounded to 100KB per function (~10,000-12,000 entries)

**Constraints**: 
- No new external dependencies (use existing crate versions only)
- Must preserve SSA form and CFG validity throughout transformations
- Must maintain semantic equivalence (no behavior changes)
- Conservative fallback when escape analysis uncertain
- Must handle malformed IR gracefully without panicking

**Scale/Scope**: 
- Module-level optimization (processes entire IR modules)
- Function-local analysis (per-function lattice state)
- Supports complex CFG: loops, nested conditionals, phi nodes
- Estimated 6-7 source files, ~2000-3000 lines of implementation code

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Safety First**: Implementation leverages Rust's ownership model, avoiding unsafe code except where absolutely necessary (e.g., if manipulating raw pointers in IR, which should be documented). All transformations preserve SSA invariants and CFG validity.

✅ **Performance Excellence**: O(n) worklist algorithm, pre-allocated hash maps, single-pass scanning, hash-based O(1) lookups, memory-bounded (100KB lattice per function), lazy evaluation for expensive checks.

✅ **Cross-Platform Compatibility**: Pure Rust implementation with no platform-specific dependencies, tested on Windows/macOS/Linux via CI.

✅ **Modular Extensibility**: Implements existing `Phase` trait, separated into 6-7 focused modules (`mod.rs`, `optimizer.rs`, `lattice.rs`, `evaluator.rs`, `worklist.rs`, `statistics.rs`, `utils.rs`), clean interfaces, drop-in integration.

✅ **Test-Driven Reliability**: Unit tests per module, integration tests for end-to-end optimization, snapshot tests via insta for IR comparison, semantic preservation tests, property-based testing for edge cases.

✅ **Snapshot Validation**: Uses existing `insta` dependency for regression detection in IR transformations, validating output consistency across compiler phases.

✅ **Documentation Rigor**: Comprehensive rustdoc comments with examples, panic/error/safety documentation, module-level overviews, architectural decision documentation.

✅ **Collaboration First**: Open development via GitHub PR process, code review integration, clear contribution path through modular design.

✅ **Respectful Communication**: Adherence to Rust Code of Conduct in all project interactions.

✅ **Shared Learning**: Detailed documentation serves as educational resource for compiler optimization techniques, SCCP algorithm implementation, and SSA-based transformations.

✅ **Quality Through Community**: All code reviewed, tested across platforms, meets project quality standards before merge.

**Gate Status**: ✅ PASSED - No violations. Implementation aligns with all constitutional principles.

## Project Structure

### Documentation (this feature)

```text
specs/015-constant-folding-sccp/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output: Technical decisions and rationale
├── data-model.md        # Phase 1 output: Core data structures and lattice design
├── quickstart.md        # Phase 1 output: Usage guide and integration examples
├── contracts/           # Phase 1 output: API contracts and interfaces
│   └── phase_trait.rs   # Phase trait implementation contract
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/ir/optimizer/constant_folding/
├── mod.rs              # Public API, module exports, ConstantFoldingOptimizer struct
├── optimizer.rs        # Phase trait implementation, orchestration logic, mode selection
├── lattice.rs          # LatticeValue enum (Top/Constant/Bottom), merge operations
├── evaluator.rs        # Constant folding logic for all IR operations (arithmetic, logical, casts)
├── worklist.rs         # SCCP worklist algorithm, SSA edge processing, fixed-point iteration
├── statistics.rs       # OptimizationMetrics struct, per-function and aggregate tracking
└── utils.rs            # SSA validation, memory estimation, conservative fallback helpers

tests/
├── ir_constant_folding_basic_tests.rs        # Unit: arithmetic, logical, cast folding
├── ir_constant_folding_propagation_tests.rs  # Unit: store/load constant propagation
├── ir_constant_folding_sccp_tests.rs         # Integration: SCCP with complex CFG
├── ir_constant_folding_snapshot_tests.rs     # Snapshot: IR transformation validation
└── ir_constant_folding_edge_cases_tests.rs   # Edge cases: div-by-zero, NaN, overflow
```

**Structure Decision**: Single project structure selected. This is a compiler optimization phase that integrates into the existing `src/ir/optimizer/` hierarchy. The constant folding module sits alongside existing optimizations like `dead_code_elimination/`, maintaining architectural consistency. All implementation files are co-located under `src/ir/optimizer/constant_folding/` for cohesion. Tests follow the existing project convention of descriptive file names in the root `tests/` directory with `ir_` prefix for IR-related tests.

## Complexity Tracking

> No violations - complexity tracking not required. Implementation follows constitution principles without exceptions.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
