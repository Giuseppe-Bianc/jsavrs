# Tasks: Assembly SSE and SSE2 Support

**Input**: Design documents from `/specs/[003-aggiorna-tutto-il]/`
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
- [ ] T001 [P] Configure SIMD-specific Rust features and dependencies in Cargo.toml
- [ ] T002 [P] Create SIMD module structure in src/asm/simd/ with initial files
- [ ] T003 [P] Update rustfmt.toml to include new SIMD formatting rules

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] Contract test for ADDPS instruction generation in tests/test_simd_addps.rs
- [ ] T005 [P] Contract test for MULPS instruction generation in tests/test_simd_mulps.rs
- [ ] T006 [P] Contract test for ADDPD instruction generation in tests/test_simd_addpd.rs
- [ ] T007 [P] Contract test for MULPD instruction generation in tests/test_simd_mulpd.rs
- [ ] T008 [P] Contract test for SUBPS instruction generation in tests/test_simd_subps.rs
- [ ] T009 [P] Integration test for SIMD/trait-based dispatch in tests/test_simd_dispatch.rs
- [ ] T010 [P] Integration test for CPU feature detection in tests/test_cpu_detection.rs
- [ ] T011 [P] Integration test for SIMD/scalar fallback in tests/test_simd_fallback.rs
- [ ] T012 [P] Integration test for SIMD IR optimization in tests/test_ir_simd.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T013 [P] Create SSE/SSE2 SIMD instruction entities in src/asm/simd/instruction.rs
- [ ] T014 [P] Create XMM register entities in src/asm/simd/register.rs
- [ ] T015 [P] Create SIMD operations entities in src/asm/simd/operation.rs
- [ ] T016 [P] Create CPU detection mechanisms in src/cpu_detection.rs
- [ ] T017 [P] Create fallback mechanisms in src/asm/simd/fallback.rs
- [ ] T018 SimdInstructionGenerator trait implementation in src/asm/simd/generator.rs
- [ ] T019 ADDPS instruction generation implementation in src/asm/simd/generator.rs
- [ ] T020 MULPS instruction generation implementation in src/asm/simd/generator.rs
- [ ] T021 ADDPD instruction generation implementation in src/asm/simd/generator.rs
- [ ] T022 MULPD instruction generation implementation in src/asm/simd/generator.rs
- [ ] T023 SUBPS instruction generation implementation in src/asm/simd/generator.rs
- [ ] T024 CPU feature detection implementation using CPUID in src/cpu_detection.rs
- [ ] T025 Trait-based dispatch for SIMD/scalar selection in src/asm/simd/dispatch.rs
- [ ] T026 Memory alignment handling for SIMD operations in src/asm/simd/alignment.rs
- [ ] T027 SIMD error handling structures in src/asm/simd/error.rs

## Phase 3.4: Integration
- [ ] T028 Create SIMD intermediate representation module in src/ir/simd.rs
- [ ] T029 Implement SIMD IR optimization pass logic in src/ir/simd/mod.rs
- [ ] T030 Integrate SIMD IR optimizations with main IR processing in src/ir/mod.rs
- [ ] T031 Detect vectorizable operations during IR generation in src/ir/generator.rs
- [ ] T032 Connect SIMD generator to main assembly generator in src/asm/generator.rs
- [ ] T033 Add SIMD optimization passes to the compiler pipeline in src/lib.rs
- [ ] T034 Implement SIMD auto-detection at runtime in src/main.rs
- [ ] T035 Add SIMD feature flags to CLI in src/cli.rs

## Phase 3.5: Polish
- [ ] T036 [P] Unit tests for SIMD instruction generation in tests"/test_simd_instructions.rs
- [ ] T037 [P] Unit tests for CPU detection logic in tests"/test_cpu_detection.rs
- [ ] T038 [P] Performance benchmarks for SIMD vs scalar operations in benches/simd_benches.rs
- [ ] T039 Performance tests (target: 20-50% improvement) in tests/simd.rs
- [ ] T040 [P] Update docs/api.md with SIMD implementation details
- [ ] T041 [P] Update README.md with SIMD usage instructions
- [ ] T042 Update manual-testing.md with SIMD validation steps
- [ ] T043 Run SIMD-specific validation using quickstart scenarios in tests/simd_validation.rs
- [ ] T044 Create real-world workload performance tests for validation in tests/real_world_workloads.rs

## Dependencies
- Setup (T001-T003) before tests (T004-T012)
- Tests (T004-T012) before implementation (T013-T027)
- T013-T017 blocks T018-T025
- T016 blocks T024
- T018 blocks T028
- T028-T031 blocks T032
- T032 blocks T033
- Implementation before polish (T036-T044)

## Parallel Example
```
# Launch T004-T012 together:
Task: "Contract test for ADDPS instruction generation in tests/test_simd_addps.rs"
Task: "Contract test for MULPS instruction generation in tests/test_simd_mulps.rs"
Task: "Contract test for ADDPD instruction generation in tests/test_simd_addpd.rs"
Task: "Contract test for MULPD instruction generation in tests/test_simd_mulpd.rs"
Task: "Contract test for SUBPS instruction generation in tests/test_simd_subps.rs"
Task: "Integration test for SIMD/trait-based dispatch in tests/test_simd_dispatch.rs"
Task: "Integration test for CPU feature detection in tests/test_cpu_detection.rs"
Task: "Integration test for SIMD/scalar fallback in tests/test_simd_fallback.rs"
Task: "Integration test for SIMD IR optimization in tests/test_ir_simd.rs"
```

```
# Launch T013-T017 together:
Task: "Create SSE/SSE2 SIMD instruction entities in src/asm/simd/instruction.rs"
Task: "Create XMM register entities in src/asm/simd/register.rs"
Task: "Create SIMD operations entities in src/asm/simd/operation.rs"
Task: "Create CPU detection mechanisms in src/cpu_detection.rs"
Task: "Create fallback mechanisms in src/asm/simd/fallback.rs"
```

```
# Launch T036-T041 together:
Task: "Unit tests for SIMD instruction generation in tests"/test_simd_instructions.rs"
Task: "Unit tests for CPU detection logic in tests"/test_cpu_detection.rs"
Task: "Performance benchmarks for SIMD vs scalar operations in benches/simd_benches.rs"
Task: "Performance tests (target: 20-50% improvement) in tests/simd.rs"
Task: "Update docs/api.md with SIMD implementation details"
Task: "Update README.md with SIMD usage instructions"
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
   - Each feature must include detailed documentation following Documentation Rigor principle (research.md, data-model.md), utilizing AI tools when appropriate for enhanced detail
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