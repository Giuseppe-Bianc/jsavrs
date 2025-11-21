# Implementation Plan: Sparse Conditional Constant Propagation (SCCP) Optimizer

**Branch**: `017-sccp-optimizer` | **Date**: 2025-11-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/017-sccp-optimizer/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a native Rust SCCP optimization phase that discovers and propagates constant values throughout the IR while simultaneously eliminating unreachable code. The optimizer uses a three-value lattice (Unknown ⊤, Constant, Variable ⊥) with worklist-driven sparse dataflow analysis on SSA form. The architecture consists of six core modules (lattice, constant_folder, analyzer, transformer, worklist, and public API) located at `src/ir/optimizer/constant_folding/`. Analysis operates in-memory with stack-allocated structs using HashMap for O(1) lattice lookups and HashSet for reachability tracking. The optimizer processes each function independently through a two-phase algorithm: (1) initialization with entry block executable and all SSA values at Top, (2) worklist processing until convergence or 10,000 iteration limit. Transformation replaces constant-valued SSA uses with literals, converts constant-condition branches to unconditional jumps, removes dead phi predecessors, and marks unreachable blocks for DCE. Integration happens via the existing Phase trait with alternating SCCP-DCE passes until fixed-point convergence.

## Technical Context

**Language/Version**: Rust 2024 edition (current project standard)
**Primary Dependencies**: petgraph = "0.8.3" (already present), thiserror = "2.0.17" (already present for error handling)
**Storage**: N/A (ephemeral in-memory analysis, no persistence)
**Testing**: cargo test (unit tests), insta = "1.44.1" (snapshot tests already present), criterion = "0.7.0" (performance benchmarks already present)
**Target Platform**: Cross-platform (Windows, macOS, Linux) - compiler infrastructure
**Project Type**: Single Rust project (compiler module at `src/ir/optimizer/constant_folding/`)
**Performance Goals**: O(n) time complexity where n = SSA values + CFG edges, <100ms for 10,000-instruction functions, each CFG edge processed at most twice
**Constraints**: Zero new dependencies, monotonic lattice convergence guarantees termination, conservative correctness (never claim constant unless provable), wrapping arithmetic semantics matching Rust release mode
**Scale/Scope**: Per-function analysis (no interprocedural), supports all primitive types (i8-u64, f32-f64, bool, char), 10,000 iteration safety limit, integrates with existing Phase trait and DeadCodeElimination

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Alignment

**Safety First**: ✅ COMPLIANT
- Leverages Rust's ownership model for memory safety in all data structures
- Uses Result<(), String> for error handling with graceful degradation
- No unsafe code required for core SCCP algorithm
- Conservative analysis ensures soundness (never claim constant unless provable)

**Performance Excellence**: ✅ COMPLIANT
- O(n) time complexity via sparse worklist-driven analysis
- Pre-allocated HashMap capacity based on instruction count estimates
- Lightweight NodeIndex handles (u32) instead of full block references
- Value deduplication prevents redundant worklist insertions
- Zero-cost abstractions through Rust enums and pattern matching

**Cross-Platform Compatibility**: ✅ COMPLIANT
- Pure Rust implementation with no platform-specific code
- Uses existing petgraph dependency already tested cross-platform
- No file I/O, networking, or OS-specific system calls
- Builds as part of standard cargo workflow

**Modular Extensibility**: ✅ COMPLIANT
- Six core modules with clear single responsibilities
- Public API through `impl Phase for SccpOptimizer`
- Lattice, constant_folder, and worklist are reusable components
- Easy to extend constant folding to new operation types
- No coupling to specific IR instruction encodings (uses trait methods)

**Test-Driven Reliability**: ✅ COMPLIANT
- Unit tests in each module file (lattice laws, constant folding correctness)
- Integration tests building synthetic Function IR and asserting properties
- Snapshot tests using existing insta crate for before/after IR comparison
- Property-based testing for lattice meet operation commutativity/idempotence
- Edge case coverage (infinite loops, division by zero, unreachable entry blocks)

**Snapshot Validation**: ✅ COMPLIANT
- Integration with existing insta = "1.44.1" dependency
- Before/after IR dumps captured with assert_snapshot!()
- Validates constant propagation, branch simplification, block reachability
- Regression detection for optimization quality changes

**Documentation Rigor**: ✅ COMPLIANT
- Will create comprehensive research.md with algorithm analysis and design decisions
- Will create detailed data-model.md documenting lattice values, SSA relationships
- Rustdoc comments for all public APIs with examples
- Algorithm documentation explaining Wegman-Zadeck sparse conditional constant propagation
- Design rationale for worklist structure, lattice ordering, and transformation strategy

### Community Principles Alignment

**Collaboration First**: ✅ COMPLIANT
- Public development on feature branch 017-sccp-optimizer
- Code review required before merge to main
- Clear contribution path for extending constant folding to new operations

**Respectful Communication**: ✅ COMPLIANT
- All technical discussions follow Rust Code of Conduct
- Design documented to enable informed feedback

**Shared Learning**: ✅ COMPLIANT
- Comprehensive documentation serves as educational resource on dataflow analysis
- Explains Wegman-Zadeck algorithm for future contributors
- Demonstrates practical application of lattice theory in compilers

**Quality Through Community**: ✅ COMPLIANT
- Mandatory peer review for all code
- Comprehensive testing enables community validation
- Clear acceptance criteria for optimization correctness

**Transparency and Openness**: ✅ COMPLIANT
- All design decisions documented in this plan
- Public spec.md explains requirements and rationale
- Research findings will be documented in research.md

### Gate Decision: **PASS** - Proceed to Phase 0

---

## Post-Design Constitution Re-Evaluation

*Re-evaluated after Phase 1 design completion (research.md, data-model.md, contracts/api.md, quickstart.md)*

### Design Artifacts Review

**Research Document** (`research.md`):
- ✅ Comprehensive lattice theory analysis with mathematical rigor
- ✅ Detailed constant folding semantics matching Rust release-mode behavior
- ✅ Worklist algorithm achieving O(V+E) complexity with proof
- ✅ Conservative error handling preserving soundness
- ✅ Aligns with **Documentation Rigor** and **Performance Excellence** principles

**Data Model** (`data-model.md`):
- ✅ Meticulous entity specifications with complete field documentation
- ✅ Precise relationship diagrams and data flow analysis
- ✅ Comprehensive validation rules and invariants
- ✅ Memory layout estimates for performance verification
- ✅ Aligns with **Documentation Rigor** and **Modular Extensibility** principles

**API Contract** (`contracts/api.md`):
- ✅ Clear public interface with pre/post-conditions
- ✅ Behavioral contracts for all optimization actions
- ✅ Performance guarantees with benchmarking targets
- ✅ Integration patterns and migration guide
- ✅ Aligns with **Modular Extensibility** and **Performance Excellence** principles

**Quick Start Guide** (`quickstart.md`):
- ✅ Phased implementation roadmap with concrete code examples
- ✅ Common pitfalls and debugging strategies
- ✅ Integration patterns with existing pipeline
- ✅ Testing strategies at multiple levels
- ✅ Aligns with **Shared Learning** and **Test-Driven Reliability** principles

### Constitutional Compliance Confirmation

All design artifacts maintain compliance with constitutional principles:

1. **Safety First**: Conservative error handling, no unsafe code, sound optimizations
2. **Performance Excellence**: O(V+E) complexity, benchmark targets, memory efficiency
3. **Cross-Platform Compatibility**: Pure Rust, no platform-specific code
4. **Modular Extensibility**: Six well-defined modules with clear interfaces
5. **Test-Driven Reliability**: Unit, integration, snapshot, and performance tests
6. **Snapshot Validation**: Integration with existing insta crate
7. **Documentation Rigor**: Comprehensive, detailed, precise documentation created
8. **Collaboration First**: Clear contribution path for extending optimizations
9. **Shared Learning**: Educational research and quickstart documents
10. **Transparency and Openness**: All design decisions documented with rationale

### Gate Decision: **PASS** - Ready for Implementation (Phase 2)

## Project Structure

### Documentation (this feature)

```text
specs/017-sccp-optimizer/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/ir/optimizer/constant_folding/
├── mod.rs                # Public API: SccpOptimizer struct, impl Phase trait
├── lattice.rs            # LatticeValue enum (Top, Constant, Bottom), meet operation
├── constant_folder.rs    # Pattern-matched arithmetic evaluation, returns Option<IrLiteralValue>
├── analyzer.rs           # Worklist-driven dataflow algorithm, SSA value tracking
├── transformer.rs        # In-place IR mutation, constant replacement, branch simplification
└── worklist.rs           # FIFO queue management for SSA values and CFG edges

tests/
├── sccp_unit_tests.rs            # Lattice laws, constant folding correctness
├── sccp_integration_tests.rs     # End-to-end on synthetic IR Functions
└── sccp_snapshot_tests.rs        # Before/after IR comparison with insta

benches/
└── sccp_benchmark.rs             # Performance measurement with criterion
```

**Structure Decision**: Single Rust project structure with new module at `src/ir/optimizer/constant_folding/`. This aligns with existing optimizer infrastructure (`src/ir/optimizer/` already contains `phase.rs` and `dce.rs`). The six-file decomposition separates concerns (data structures, algorithms, transformations) for maintainability and testability. Test files follow existing project convention of placing integration tests in `tests/` directory and using snapshot testing with insta crate.

## Complexity Tracking

> **No constitutional violations detected - this section intentionally empty**

All requirements align with constitutional principles without needing justification for complexity.
