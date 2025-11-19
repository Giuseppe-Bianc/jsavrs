# Implementation Plan: Sparse Conditional Constant Propagation Optimizer

**Branch**: `016-sccp-optimizer` | **Date**: 2025-11-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/016-sccp-optimizer/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a Sparse Conditional Constant Propagation (SCCP) optimizer for the jsavrs Rust compiler using the Wegman-Zadeck algorithm. The optimizer operates on SSA-form intermediate representation to identify compile-time constant values, eliminate conditional branches with constant conditions, and remove unreachable code. The implementation uses a three-level lattice (Top/Constant/Bottom) with dual worklist-driven analysis (SSAWorkList for value propagation, FlowWorkList for control-flow traversal) achieving O(edges) time complexity. The optimizer integrates with existing IR infrastructure through the Phase trait, maintains SSA and CFG validity, and provides configurable verbosity and iteration limits with comprehensive statistics collection.

## Technical Context

**Language/Version**: Rust 2024 Edition (Rust 1.75+)  
**Primary Dependencies**: petgraph 0.8.3 (CFG traversal), console 0.16.1 (styled output), thiserror 2.0.17 (error handling)  
**Storage**: N/A (in-memory IR transformation during compilation)  
**Testing**: cargo test (unit tests), insta 1.43.2 (snapshot testing), criterion 0.7.0 (performance benchmarks)  
**Target Platform**: Cross-platform (Windows, macOS, Linux) - compiler infrastructure
**Project Type**: Single project - new optimizer module within existing compiler codebase  
**Performance Goals**: Analyze and optimize functions with 10,000+ instructions in <1 second, achieve fixed-point convergence in <100 iterations for 99% of real-world code  
**Constraints**: O(edges) time complexity, zero false positives (incorrect constant values), 100% SSA/CFG validity preservation  
**Scale/Scope**: New module `src/ir/optimizer/constant_folding/` with 7 submodules (lattice, worklist, evaluator, branch_analysis, executable_edges, rewriter, stats), integration with existing IR types and Phase trait

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Safety First**: ✅ Implementation uses Rust's ownership model, leverages existing IR types (no unsafe code required), maintains SSA invariants through validation
**Performance Excellence**: ✅ O(edges) time complexity proven through per-edge processing limits, efficient HashMap/HashSet data structures, zero-cost abstractions
**Cross-Platform Compatibility**: ✅ Pure Rust implementation with cross-platform dependencies (petgraph, console, thiserror), no platform-specific code
**Modular Extensibility**: ✅ Structured as independent module `src/ir/optimizer/constant_folding/` with 7 submodules, integrates via Phase trait interface
**Test-Driven Reliability**: ✅ Comprehensive testing strategy: unit tests per module, integration tests on complete functions, regression tests, performance benchmarks
**Snapshot Validation**: ✅ Uses insta crate for IR output validation, snapshot testing for optimization results
**Documentation Rigor**: ✅ Detailed research.md explaining SCCP algorithm and lattice theory, comprehensive data-model.md documenting lattice values and worklist structures
**Collaboration First**: ✅ Open development on public branch 016-sccp-optimizer, clear documentation for future contributors
**Respectful Communication**: ✅ N/A for implementation (community principle)
**Shared Learning**: ✅ Detailed documentation serves as educational resource for understanding SCCP and compiler optimization techniques
**Quality Through Community**: ✅ Code review process, comprehensive testing, adherence to Rust idioms verified by cargo clippy
**Transparency and Openness**: ✅ All development on public GitHub branch, detailed plan.md and research.md documenting design decisions

**GATE RESULT**: ✅ PASS - All constitution principles satisfied, no violations requiring justification

## Project Structure

### Documentation (this feature)

```text
specs/016-sccp-optimizer/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command) - SCCP algorithm analysis, lattice theory, Wegman-Zadeck paper review
├── data-model.md        # Phase 1 output (/speckit.plan command) - LatticeValue enum, worklist structures, executable edge tracking
├── quickstart.md        # Phase 1 output (/speckit.plan command) - How to use SCCP optimizer, configuration options, example usage
├── contracts/           # Phase 1 output (/speckit.plan command) - Phase trait API, LatticeValue meet/join operations
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/ir/optimizer/constant_folding/
├── mod.rs                      # Module declaration and public API
├── lattice.rs                  # LatticeValue enum (Top/Constant(IrLiteralValue)/Bottom), meet/join operations
├── worklist.rs                 # SSAWorkList and FlowWorkList with VecDeque + HashSet duplicate prevention
├── evaluator.rs                # Abstract interpretation for InstructionKind variants
├── branch_analysis.rs          # TerminatorKind condition evaluation
├── executable_edges.rs         # CFG edge and block reachability tracking
├── rewriter.rs                 # IR mutation: constant replacement, branch elimination, unreachable block removal
└── stats.rs                    # OptimizationStatistics collection

src/ir/optimizer/
└── mod.rs                      # Export ConstantFoldingOptimizer, coordinate with existing DCE

tests/
├── ir_sccp_lattice_tests.rs           # Unit tests for lattice meet/join operations
├── ir_sccp_worklist_tests.rs          # Unit tests for worklist enqueue/dequeue/duplicate prevention
├── ir_sccp_evaluator_tests.rs         # Unit tests for instruction evaluation with all lattice value combinations
├── ir_sccp_branch_tests.rs            # Unit tests for terminator evaluation
├── ir_sccp_integration_tests.rs       # Integration tests on complete functions
├── ir_sccp_regression_tests.rs        # Regression tests for unchanged code
├── ir_sccp_snapshot_tests.rs          # Snapshot tests for IR output validation (using insta)
└── ir_sccp_performance_tests.rs       # Performance benchmarks for linear scaling verification

benches/
└── sccp_benchmark.rs           # Criterion benchmarks for 1k/5k/10k instruction functions
```

**Structure Decision**: Single project structure selected. The SCCP optimizer is a new module within the existing `src/ir/optimizer/` directory, organized into 7 focused submodules for maintainability and testability. This aligns with the existing jsavrs compiler architecture where optimization passes are modular components integrated through the Phase trait. The submodule organization (lattice, worklist, evaluator, branch_analysis, executable_edges, rewriter, stats) maps directly to the key algorithmic components described in the Wegman-Zadeck paper, facilitating independent development and testing of each component.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
