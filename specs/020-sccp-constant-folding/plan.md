# Implementation Plan: Constant Folding Optimizer with Sparse Conditional Constant Propagation

**Branch**: `020-sccp-constant-folding` | **Date**: 2025-12-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/020-sccp-constant-folding/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a production-ready constant folding optimization phase using the Wegman-Zadeck Sparse Conditional Constant Propagation (SCCP) algorithm for the jsavrs compiler's intermediate representation. The optimizer will integrate seamlessly with the existing optimization pipeline and Dead Code Elimination phase while maintaining SSA form invariants. The implementation follows a modular architecture with four core components: lattice.rs for value state tracking (Bottom/Constant/Top), evaluator.rs for type-safe constant expression evaluation, propagator.rs for the worklist algorithm managing SSA and CFG edge processing, and rewriter.rs for IR transformations.

## Technical Context

**Language/Version**: 1.91.1 Rust 2024 Edition (from Cargo.toml edition = "2024")  
**Primary Dependencies**: petgraph (0.8.3) for graph operations, thiserror (2.0.17) for error handling, existing IR infrastructure (ControlFlowGraph, DominanceInfo, SSA)  
**Storage**: In-memory data structures (HashMap for lattice values, HashSet for executable edges, VecDeque for worklists)  
**Testing**: cargo test with insta (1.44.3) for snapshot testing, criterion (0.8.0) for performance benchmarks  
**Target Platform**: Cross-platform (Windows, macOS, Linux) with Rust native compilation
**Project Type**: Single project (compiler optimization module within larger compiler infrastructure)  
**Performance Goals**: Linear time complexity O(n) for n instructions, convergence within 1-3 iterations for 95% of functions, <1 second per function for up to 10,000 instructions  
**Constraints**: Preserve SSA form invariants, maintain conservative soundness (never assume constant unless proven), coordinate with DCE phase without direct code removal  
**Scale/Scope**: Support functions with up to 10,000 instructions, handle all IR types (I8-I64, U8-U64, F32, F64, Bool, Char, String), integrate with existing Phase trait interface

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Safety First**: ✅ The implementation leverages Rust's ownership model for memory safety. All data structures use safe Rust abstractions (HashMap, HashSet, VecDeque). No unsafe code is required for this optimization phase.

**Performance Excellence**: ✅ The SCCP algorithm is specifically chosen for its sparse analysis characteristics, achieving O(n) complexity through worklist-based processing. Performance benchmarks with criterion will track optimization time and convergence iterations.

**Cross-Platform Compatibility**: ✅ Implementation uses standard Rust library components and existing cross-platform IR infrastructure. No platform-specific code required.

**Modular Extensibility**: ✅ Clear separation into lattice.rs, evaluator.rs, propagator.rs, rewriter.rs, and optimizer.rs modules. Each component has well-defined interfaces and can be extended independently (e.g., adding new constant evaluation rules).

**Test-Driven Reliability**: ✅ Comprehensive test strategy includes unit tests for lattice operations, snapshot tests for IR transformations with insta, and integration tests for end-to-end optimization. Criterion benchmarks measure convergence and performance.

**Snapshot Validation**: ✅ Insta library integration for validating IR transformations before/after optimization. Snapshots capture constant propagation, branch resolution, and phi node simplification.

**Documentation Rigor**: ✅ This plan includes research.md (Phase 0) for algorithm analysis, data-model.md (Phase 1) for lattice/entity design, and comprehensive rustdoc comments for all public APIs. AI-assisted documentation ensures thoroughness.

**Collaboration First**: ✅ Modular design enables parallel development of evaluator and propagator components. Clear interfaces facilitate code review and knowledge sharing.

**Respectful Communication**: ✅ Standard open source contribution process applies. Code reviews focus on technical correctness, performance, and maintainability.

**Shared Learning**: ✅ Implementation serves as educational resource for understanding SCCP algorithm and compiler optimization techniques. Detailed comments explain lattice theory and algorithm invariants.

**Quality Through Community**: ✅ All code undergoes peer review. Integration with existing CI/CD pipeline ensures cargo clippy and cargo test pass before merge.

**Transparency and Openness**: ✅ All development in public GitHub repository with detailed commit messages and pull request descriptions explaining design decisions.

**Verdict**: ✅ PASS - All constitution principles satisfied with no violations requiring justification.

## Project Structure

### Documentation (this feature)

```text
specs/020-sccp-constant-folding/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command) - Algorithm analysis and design decisions
├── data-model.md        # Phase 1 output (/speckit.plan command) - Lattice values and entity relationships
├── quickstart.md        # Phase 1 output (/speckit.plan command) - Usage examples and integration guide
├── contracts/           # Phase 1 output (/speckit.plan command) - Module interfaces and API contracts
│   ├── lattice-api.md
│   ├── evaluator-api.md
│   ├── propagator-api.md
│   └── rewriter-api.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/ir/optimizer/constant_folding/
├── mod.rs              # Module declarations and re-exports
├── lattice.rs          # LatticeValue enum and meet/join operations
├── evaluator.rs        # Constant expression evaluation for all IR types
├── propagator.rs       # SCCP worklist algorithm (SSA + CFG edge processing)
├── rewriter.rs         # IR transformation and simplification
└── optimizer.rs        # Phase trait implementation and orchestration

src/ir/optimizer/
├── mod.rs              # Update to include constant_folding module
├── phase.rs            # Existing Phase trait definition
└── dead_code_elimination/  # Existing DCE phase for coordination

tests/
├── ir_sccp_lattice_tests.rs              # Unit tests for lattice operations
├── ir_sccp_evaluator_tests.rs            # Unit tests for constant evaluation
├── ir_sccp_propagator_tests.rs           # Unit tests for worklist algorithm
├── ir_sccp_rewriter_tests.rs             # Unit tests for IR transformations
├── ir_sccp_integration_tests.rs          # End-to-end SCCP + DCE integration
└── ir_sccp_snapshot_tests.rs             # Insta snapshot tests for IR output

benches/
└── sccp_benchmark.rs   # Criterion benchmarks for convergence and performance
```

**Structure Decision**: Single project structure selected as this is an optimization module within the existing jsavrs compiler. The constant_folding module resides under src/ir/optimizer/ alongside the existing dead_code_elimination module, maintaining consistency with the established compiler architecture. This placement enables direct integration with the existing Phase trait interface and coordination with DCE through the shared optimization pipeline.

---

**Constitution Check Status**: ✅ PASS (no violations, no complexity tracking needed)

**Next Steps**: Proceed to Phase 0 (Research) to analyze SCCP algorithm details, lattice theory, and integration patterns.
