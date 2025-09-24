# Tasks: Assembly SSE and SSE2 Support

**Input**: Design documents from `/specs/003-aggiorna-tutto-il/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → If not found: ERROR "No implementation plan found"
   → Extract: tech stack, libraries, structure
2. Load optional design documents:
   → data-model.md: Extract entities → model tasks
   → contracts/: Each file → contract test task
   → research.md: Extract decisions → setup tasks
   → quickstart.md: Extract test scenarios → integration tests
3. Generate tasks by category:
   → Setup: project init, dependencies, linting
   → Tests: contract tests, integration tests
   → Core: models, services, CLI commands
   → Integration: DB, middleware, logging
   → Polish: unit tests, performance, docs
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → All contracts have tests?
   → All entities have models?
   → All endpoints implemented?
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure

## Phase 3.1: Setup
- [ ] T001 Set up SSE/SSE2 development environment with Rust 1.75+
- [ ] T002 [P] Add CPUID detection dependencies to Cargo.toml
- [ ] T003 [P] Configure linting and formatting tools (cargo fmt, cargo clippy)

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] Contract test for SIMD operations in tests/contract/test_simd_operations.rs
- [ ] T005 [P] Integration test for basic SSE instruction generation in tests/test_basic_sse.rs
- [ ] T006 [P] Integration test for basic SSE2 instruction generation in tests/test_basic_sse2.rs
- [ ] T007 [P] Integration test for CPU feature detection in tests/test_cpu_detection.rs
- [ ] T008 [P] Integration test for scalar fallback mechanism in tests/test_scalar_fallback.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T009 [P] SSEInstruction struct and methods in src/asm/sse_instruction.rs
- [ ] T010 [P] Operand enum and methods for SIMD in src/asm/operand.rs
- [ ] T011 [P] CPUFeature struct for SSE/SSE2 detection in src/asm/cpu_feature.rs
- [ ] T012 [P] AssemblyBlock struct for SIMD code blocks in src/asm/assembly_block.rs
- [ ] T013 [P] InstructionMetadata for SIMD optimizations in src/asm/instruction_metadata.rs
- [ ] T014 [P] SIMDValue struct for vectorized data in src/asm/simd_value.rs
- [ ] T015 [P] SIMDProcessor trait and implementation in src/asm/simd_processor.rs
- [ ] T016 Implement trait-based dispatch for SSE/SSE2 in src/asm/simd_dispatch.rs
- [ ] T017 Add SSE/SSE2 instruction generation to src/asm/generator.rs
- [ ] T018 Update register handling to support XMM registers in src/asm/register.rs
- [ ] T019 Implement CPUID detection logic in src/asm/cpu_detector.rs
- [ ] T020 Add aligned/unaligned memory access implementations in src/asm/memory_access.rs
- [ ] T021 Update module structure in src/asm/mod.rs to include new SIMD components

## Phase 3.4: Integration
- [ ] T022 Integrate SSE/SSE2 support into compiler IR phase
- [ ] T023 Add compile-time flags for SSE/SSE2 in src/cli.rs
- [ ] T024 Implement runtime CPU detection during compilation
- [ ] T025 Add fallback logic for non-SSE processors
- [ ] T026 Update existing assembly generation to use new SIMD components
- [ ] T027 Add memory alignment handling for SSE instructions
- [ ] T028 Implement configurable precision modes for floating-point operations

## Phase 3.5: Polish
- [ ] T029 [P] Unit tests for SSEInstruction in tests/test_sse_instruction.rs
- [ ] T030 [P] Unit tests for CPUFeature in tests/test_cpu_feature.rs
- [ ] T031 [P] Unit tests for AssemblyBlock in tests/test_assembly_block.rs
- [ ] T032 [P] Unit tests for SIMDValue in tests/test_simd_value.rs
- [ ] T033 [P] Unit tests for SIMDProcessor in tests/test_simd_processor.rs
- [ ] T034 Performance benchmarks for SIMD operations in benches/simd_benchmark.rs
- [ ] T035 [P] Update documentation for SSE/SSE2 implementation in docs/sse_support.md
- [ ] T036 Add comments and documentation to all SSE/SSE2 code changes
- [ ] T037 [P] Create SIMD validation harness as specified in requirements
- [ ] T038 [P] Update quickstart examples in examples/simd_examples.rs
- [ ] T039 Run manual testing according to quickstart.md validation steps
- [ ] T040 [P] Microbenchmark tests to validate 20-50% performance improvement
- [ ] T041 [P] Cross-platform compatibility tests on Windows, Linux, and macOS

## Dependencies
- Tests (T004-T008) before implementation (T009-T021)
- T009 blocks T012, T026
- T010 blocks T009, T026
- T011 blocks T015, T024
- T012 blocks T013
- T013 blocks T012
- T015 blocks T022
- T016 blocks T022
- T017 blocks T024
- T018 blocks T026
- T019 blocks T024, T25
- T020 blocks T027
- T009-T021 blocks integration (T022-T028)
- Implementation before polish (T029-T041)

## Parallel Example
```
# Launch T029-T033 together:
Task: "Unit tests for SSEInstruction in tests/test_sse_instruction.rs"
Task: "Unit tests for CPUFeature in tests/test_cpu_feature.rs"
Task: "Unit tests for AssemblyBlock in tests/test_assembly_block.rs"
Task: "Unit tests for SIMDValue in tests/test_simd_value.rs"
Task: "Unit tests for SIMDProcessor in tests/test_simd_processor.rs"
```

## Notes
- [P] tasks = different files, no dependencies
- Verify tests fail before implementing
- Commit after each task
- Avoid: vague tasks, same file conflicts

## Task Generation Rules
*Applied during main() execution*

1. **From Contracts**:
   - Each contract file → contract test task [P]
   - Each endpoint → implementation task
   
2. **From Data Model**:
   - Each entity → model creation task [P]
   - Relationships → service layer tasks
   
3. **From User Stories**:
   - Each story → integration test [P]
   - Quickstart scenarios → validation tasks

4. **Ordering**:
   - Setup → Tests → Models → Services → Endpoints → Polish
   - Dependencies block parallel execution

5. **Community Integration**:
   - Each feature should include documentation contributions
   - Code reviews must follow respectful communication principles
   - Tests should exemplify shared learning opportunities

## Validation Checklist
*GATE: Checked by main() before returning*

- [ ] All contracts have corresponding tests
- [ ] All entities have model tasks
- [ ] All tests come before implementation
- [ ] Parallel tasks truly independent
- [ ] Each task specifies exact file path
- [ ] No task modifies same file as another [P] task