# Tasks: Assembly Code Generator x86-64

**Input**: Design documents from `/specs/001-progettare-e-implementare/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/interfaces.md, quickstart.md

## Phase 3.1: Setup
- [ ] T001 Update `Cargo.toml` dependencies to include `iced-x86`, `thiserror`, `serde`, `serde_json`, `insta`, and `criterion`, wiring feature flags as needed for x86-64 assembly generation.
- [ ] T002 Create assembly generator scaffolding (`src/asm/` directory tree, `src/asm/mod.rs`, `src/asm/generator.rs`, `src/asm/x86_64/mod.rs`, `src/asm/x86_64/instruction_mapper.rs`, `src/asm/x86_64/register_alloc.rs`, `src/asm/x86_64/abi/mod.rs`, `src/asm/optimization/mod.rs`, `src/asm/debug/mod.rs`, `src/asm/output.rs`, `src/asm/error.rs`, `tests/asm_fixtures/`).
- [ ] T003 Wire the new module into `src/lib.rs` (and any shared prelude modules) so the assembly generator feature is discoverable by the existing compiler pipeline.

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
- [ ] T004 [P] Add failing contract tests in `tests/asm_contract_tests.rs` that validate `GenerationRequest`/`GenerationResult` behavior defined in `contracts/interfaces.md` (input validation, error propagation, concurrency flags).
- [ ] T005 [P] Add failing unit tests in `tests/asm_instruction_tests.rs` covering arithmetic, memory, and control-flow IR→x86-64 mappings with iced-x86 encodings.
- [ ] T006 [P] Add failing unit tests in `tests/asm_register_tests.rs` for linear-scan allocation, spilling, and ABI preservation across System V and Microsoft x64 conventions.
- [ ] T007 [P] Add failing unit tests in `tests/asm_error_tests.rs` for stub generation (`; TODO` markers), JSON diagnostics, and error severity thresholds.
- [ ] T008 [P] Add failing unit tests in `tests/asm_debug_tests.rs` ensuring debug level configuration, DWARF section emission, and ±2 line mapping guarantees.
- [ ] T008b [P] Add failing unit tests in `tests/asm_pic_tests.rs` for position-independent code generation, including GOT/PLT section validation, RIP-relative addressing conversion, dynamic symbol resolution, and shared library compatibility verification.
- [ ] T009 [P] Add failing integration test in `tests/asm_tests.rs` that assembles a sample IR module, invokes NASM via helper, and asserts semantic preservation.
- [ ] T010 [P] Add failing snapshot test harness in `tests/asm_snapshot_tests.rs` plus fixture references under `tests/asm_fixtures/` for verifying generated assembly text.

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T011 Implement module root exports, configuration structs, and feature guards in `src/asm/mod.rs`.
- [ ] T012 Implement `AssemblyGenerator` orchestrator (request validation, orchestration pipeline, error funnel) in `src/asm/generator.rs`.
- [ ] T013 Implement dependency graph + scheduling utilities for module ordering in `src/asm/dependency.rs`.
- [ ] T014 Implement `ModuleGenerator` to transform IR functions/basic blocks into assembly segments in `src/asm/module.rs`.
- [ ] T015 Implement `InstructionMapper` trait and baseline arithmetic/memory/control flow mappings with iced-x86 encodings in `src/asm/x86_64/instruction_mapper.rs`.
- [ ] T016 Implement linear-scan `RegisterAllocator` with spill management and ABI-aware register sets in `src/asm/x86_64/register_alloc.rs`.
- [ ] T017 Implement dual ABI support (System V + Microsoft x64) in `src/asm/x86_64/abi/mod.rs` and supporting files.
- [ ] T018 Implement symbol/relocation manager and label generation utilities in `src/asm/symbols.rs`.
- [ ] T019 Implement `AssemblyOutput` builder, section writers, and NASM emission helpers in `src/asm/output.rs`.
- [ ] T020 Implement structured `ErrorHandler` with stub code generation, JSON diagnostics, and continuation thresholds in `src/asm/error.rs`.
- [ ] T021 Implement `DebugInfoGenerator` with multi-level output and DWARF section writers in `src/asm/debug/mod.rs`.
- [ ] T022 Implement optimization pipeline (peephole passes, redundancy removal, constant folding) in `src/asm/optimization/mod.rs`.
- [ ] T023 Implement comprehensive Position-Independent Code (PIC) support for shared library compatibility:
  - [ ] T023a Create `src/asm/x86_64/pic.rs` with GOT (Global Offset Table) management, including GOT entry allocation, GOT section generation, and GOT-relative addressing utilities
  - [ ] T023b Implement PLT (Procedure Linkage Table) handling for external function calls, including PLT entry generation, lazy binding stub creation, and PLT section management
  - [ ] T023c Develop RIP-relative addressing utilities for position-independent data access, including automatic conversion of absolute memory references to RIP-relative form
  - [ ] T023d Create dynamic symbol resolution system for external symbols, including external symbol tracking, relocation entry generation, and symbol import/export management  
  - [ ] T023e Integrate PIC support with instruction mapper (T015) to automatically detect and convert position-dependent instructions to PIC-compatible equivalents
  - [ ] T023f Integrate PIC support with assembly output generator (T019) to emit proper GOT/PLT sections, relocation tables, and dynamic symbol information
  - [ ] T023g Add PIC mode configuration to generator options with validation for shared library vs executable modes and cross-platform compatibility checks
- [ ] T024 Extend `AssemblyGenerator` concurrency path with Rayon-based parallel module processing and thread-safe symbol table usage in `src/asm/generator.rs`.

## Phase 3.4: Integration
- [ ] T025 Integrate assembly generator into compiler pipeline (`src/lib.rs`, `src/main.rs`) to emit `.asm` artifacts alongside existing outputs.
- [ ] T026 Expose CLI flags for target architecture, calling convention, debug level, and PIC mode in `src/cli.rs` (including validation and help text).
- [ ] T027 Implement NASM emitter + file system handoff in `src/asm/emitter.rs`, ensuring configurable output directories and optional automatic assembly run.
- [ ] T028 Implement configuration parsing/persistence for generator options in `src/asm/config.rs` and ensure compatibility with existing settings infrastructure.

## Phase 3.5: Polish
- [ ] T029 [P] Add Criterion benchmarks for assembly generation in `benches/asm_generator_bench.rs` covering small, medium, and large IR modules.
- [ ] T030 [P] Update documentation (`README.md`, new `docs/assembly-generator.md`) with usage instructions, debug levels, and ABI notes.
- [ ] T031 [P] Populate `tests/asm_fixtures/` with representative IR/assembly pairs used by snapshot and integration tests, including PIC-specific test cases with GOT/PLT sections and external symbol references.
- [ ] T032 Finalize quality gates: run `cargo fmt`, `cargo clippy --all-targets --all-features`, execute the new test/benchmark suites, and refresh `specs/001-progettare-e-implementare/quickstart.md` notes with any deviations.

## Dependencies
- T001 → T002 → T003 establish tooling before tests.
- Tests (T004–T010, including T008b) must land before any implementation tasks T011+. Keep snapshot fixtures (T010) ready before ModuleGenerator work (T014).
- T012 depends on T011 & T013; T014 depends on T012 & T015; T015 depends on T002 & T005; T016 depends on T006.
- ABI work (T017) must follow register allocator (T016); symbol/output/error/debug/optimization tasks (T018–T022) depend on preceding pipeline pieces.
- PIC support (T023a-g) requires instruction mapper (T015) and output builder (T019); T023e depends on T015, T023f depends on T019; PIC test task T008b should be implemented alongside T023a for TDD compliance.
- Concurrency (T024) depends on stable generator core (T012) and symbol manager (T018).
- Integration tasks (T025–T028) depend on completion of core implementation tasks.
- Polish tasks (T029–T032) run after all integration tasks finish.

## Parallel Execution Examples
```
# Example: launch contract + unit test tasks together once setup (T001–T003) completes
/task run T004
/task run T005
/task run T006
/task run T007
/task run T008
/task run T008b
/task run T009
/task run T010

# Example: polish batch after integration
/task run T029
/task run T030
/task run T031
```

## Notes
- `[P]` denotes tasks that target independent files with no blocking dependencies and can run in parallel once their predecessors finish.
- Maintain TDD discipline: ensure every test in Phase 3.2 fails before starting Phase 3.3.
- When multiple tasks touch the same file (e.g., `src/asm/generator.rs` in T012 and T024), execute them sequentially to avoid merge conflicts.
- Keep Documentation Rigor principle in mind: update research/data-model documents if major design adjustments occur during implementation.
