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
- [ ] T013 [P] Baseline regression test: validate existing assembly output still matches in tests/test_regression_baseline.rs
- [ ] T014 [P] Baseline functionality test: verify compiler still produces correct output for non-SIMD code in tests/test_functionality_baseline.rs
- [ ] T015 [P] Performance baseline establishment: benchmark current performance before SIMD implementation in tests/test_performance_baseline.rs
- [ ] T016 [P] Specialized SIMD validation tool setup: configure tools for comprehensive SSE/SSE2 validation in tests/test_simd_validation_tools.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T017 [P] Create SSE/SSE2 SIMD instruction entities in src/asm/simd/instruction.rs
- [ ] T018 [P] Create XMM register entities in src/asm/simd/register.rs
- [ ] T019 [P] Create SIMD operations entities in src/asm/simd/operation.rs
- [ ] T020 [P] Create CPU detection mechanisms in src/cpu_detection.rs
- [ ] T021 [P] Create fallback mechanisms in src/asm/simd/fallback.rs
- [ ] T022 SimdInstructionGenerator trait implementation in src/asm/simd/generator.rs
- [ ] T023 ADDPS instruction generation implementation in src/asm/simd/generator.rs
- [ ] T024 MULPS instruction generation implementation in src/asm/simd/generator.rs
- [ ] T025 ADDPD instruction generation implementation in src/asm/simd/generator.rs
- [ ] T026 MULPD instruction generation implementation in src/asm/simd/generator.rs
- [ ] T027 SUBPS instruction generation implementation in src/asm/simd/generator.rs
- [ ] T028 CPU feature detection implementation using CPUID in src/cpu_detection.rs
- [ ] T029 Trait-based dispatch for SIMD/scalar selection in src/asm/simd/dispatch.rs

## Phase 3.4: Memory Alignment Implementation
- [ ] T030 Aligned memory access implementation: implement SIMD operations with 16-byte alignment requirements in src/asm/simd/alignment.rs
- [ ] T031 Unaligned memory access implementation: implement SIMD operations that work with unaligned memory in src/asm/simd/alignment.rs
- [ ] T032 Memory alignment detection and selection logic: runtime detection of memory alignment and selection of appropriate implementation in src/asm/simd/alignment.rs
- [ ] T033 SIMD error handling structures with alignment considerations in src/asm/simd/error.rs

## Phase 3.5: Integration
- [ ] T034 Create SIMD intermediate representation module in src/ir/simd.rs
- [ ] T035 Implement SIMD IR optimization pass logic in src/ir/simd/mod.rs
- [ ] T036 Integrate SIMD IR optimizations with main IR processing in src/ir/mod.rs
- [ ] T037 Detect vectorizable operations during IR generation in src/ir/generator.rs
- [ ] T038 Connect SIMD generator to main assembly generator in src/asm/generator.rs
- [ ] T039 Add SIMD optimization passes to the compiler pipeline in src/lib.rs
- [ ] T040 Implement SIMD auto-detection at runtime in src/main.rs
- [ ] T041 Add SIMD feature flags to CLI in src/cli.rs

## Phase 3.6: Comprehensive Regression Testing
- [ ] T042 Full regression test: validate all existing functionality still works after SIMD implementation in tests/test_full_regression.rs
- [ ] T043 Before/after comparison test: ensure non-SIMD operations still produce identical output in tests/test_no_regression.rs
- [ ] T044 Behavioral validation test: verify program behavior unchanged for existing code in tests/test_behavioral_validation.rs
- [ ] T045 Cross-platform compatibility check: ensure SIMD changes work on all platforms in tests/test_cross_platform.rs

## Phase 3.7: Specialized SIMD Validation
- [ ] T046 Specialized SIMD validation using dedicated tools: validate all SSE/SSE2 implementations with specialized validation frameworks in tests/test_specialized_simd_validation.rs
- [ ] T047 SIMD precision validation: verify floating-point precision consistency between scalar and SIMD implementations in tests/test_simd_precision.rs
- [ ] T048 SIMD security validation: validate against side-channel and memory safety risks in tests/test_simd_security.rs
- [ ] T049 SIMD performance validation: verify 20-50% performance targets are met using specialized benchmarks in tests/test_simd_performance_validation.rs
- [ ] T050 Aligned memory access validation: verify 16-byte aligned SIMD operations work correctly in tests/test_aligned_memory_validation.rs
- [ ] T051 Unaligned memory access validation: verify unaligned SIMD operations work correctly in tests/test_unaligned_memory_validation.rs

## Phase 3.8: Polish
- [ ] T052 [P] Unit tests for SIMD instruction generation in tests/test_simd_instructions.rs
- [ ] T053 [P] Unit tests for CPU detection logic in tests/test_cpu_detection.rs
- [ ] T054 [P] Performance benchmarks for SIMD vs scalar operations in benches/simd_benches.rs
- [ ] T055 Performance tests (target: 20-50% improvement) in tests/test_simd_performance.rs
- [ ] T056 [P] Update docs/api.md with SIMD implementation details
- [ ] T057 [P] Update README.md with SIMD usage instructions
- [ ] T058 Update manual-testing.md with SIMD validation steps
- [ ] T059 Run SIMD-specific validation using quickstart scenarios in tests/test_simd_validation.rs
- [ ] T060 Create real-world workload performance tests for validation in tests/test_real_world_workloads.rs
- [ ] T061 [P] Document non-vectorizable sections with reasons and alternatives in docs/non_vectorizable.md
- [ ] T062 [P] Ensure all SSE/SSE2 changes have clear comments in source files
- [ ] T063 [P] Final integration test: ensure SIMD implementation doesn't break existing compiler functionality in tests/test_final_integration.rs

## Dependencies
- Setup (T001-T003) before tests (T004-T016)
- Tests (T004-T016) before implementation (T017-T029)
- T017-T021 blocks T022-T029
- T020 blocks T028
- T022 blocks T034
- Implementation (T017-T029) before Memory Alignment Implementation (T030-T033)
- T030-T033 blocks Integration (T034-T041)
- T034-T037 blocks T038
- T038 blocks T039
- Integration before regression testing (T042-T045)
- Regression testing before specialized SIMD validation (T046-T051)
- Specialized SIMD validation before polish (T052-T063)

## Parallel Example
```
# Launch T004-T016 together:
Task: "Contract test for ADDPS instruction generation in tests/test_simd_addps.rs"
Task: "Contract test for MULPS instruction generation in tests/test_simd_mulps.rs"
Task: "Contract test for ADDPD instruction generation in tests/test_simd_addpd.rs"
Task: "Contract test for MULPD instruction generation in tests/test_simd_mulpd.rs"
Task: "Contract test for SUBPS instruction generation in tests/test_simd_subps.rs"
Task: "Integration test for SIMD/trait-based dispatch in tests/test_simd_dispatch.rs"
Task: "Integration test for CPU feature detection in tests/test_cpu_detection.rs"
Task: "Integration test for SIMD/scalar fallback in tests/test_simd_fallback.rs"
Task: "Integration test for SIMD IR optimization in tests/test_ir_simd.rs"
Task: "Baseline regression test: validate existing assembly output still matches in tests/test_regression_baseline.rs"
Task: "Baseline functionality test: verify compiler still produces correct output for non-SIMD code in tests/test_functionality_baseline.rs"
Task: "Performance baseline establishment: benchmark current performance before SIMD implementation in tests/test_performance_baseline.rs"
Task: "Specialized SIMD validation tool setup: configure tools for comprehensive SSE/SSE2 validation in tests/test_simd_validation_tools.rs"
```

```
# Launch T017-T021 together:
Task: "Create SSE/SSE2 SIMD instruction entities in src/asm/simd/instruction.rs"
Task: "Create XMM register entities in src/asm/simd/register.rs"
Task: "Create SIMD operations entities in src/asm/simd/operation.rs"
Task: "Create CPU detection mechanisms in src/cpu_detection.rs"
Task: "Create fallback mechanisms in src/asm/simd/fallback.rs"
```

```
# Launch T052-T057 together:
Task: "Unit tests for SIMD instruction generation in tests/test_simd_instructions.rs"
Task: "Unit tests for CPU detection logic in tests/test_cpu_detection.rs"
Task: "Performance benchmarks for SIMD vs scalar operations in benches/simd_benches.rs"
Task: "Performance tests (target: 20-50% improvement) in tests/test_simd_performance.rs"
Task: "Update docs/api.md with SIMD implementation details"
Task: "Update README.md with SIMD usage instructions"
```

## Notes
- [P] tasks = different files, no dependencies
- Verify tests fail before implementing
- Commit after each task
- Avoid: vague tasks, same file conflicts
- Special attention to regression testing (T042-T045) to ensure existing functionality is preserved
- Specialized SIMD validation tasks (T046-T049) address requirement for specialized validation tools
- Memory alignment tasks (T030-T033) and validation (T050-T051) address FR-012 requirement for both aligned and unaligned code paths

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
   - Setup → Tests → Models → Services → Memory Alignment → Endpoints → Regression → Specialized Validation → Polish
   - Dependencies block parallel execution
   
5. **Community Integration**:
   - Each feature must include detailed documentation following Documentation Rigor principle (research.md, data-model.md), utilizing AI tools when appropriate for enhanced detail
   - Code reviews must follow respectful communication principles
   - Tests should exemplify shared learning opportunities

6. **Regression Testing**:
   - Special emphasis added to ensure existing functionality is preserved (T042-T045)
   - Comprehensive before/after validation to prevent any behavioral changes

7. **Specialized SIMD Validation**:
   - Dedicated phase (Phase 3.7) for specialized SIMD validation tools (T046-T049)
   - Addresses FR-011 requirement for specialized SIMD validation tools

8. **Memory Alignment Handling**:
   - Dedicated phase (Phase 3.4) for both aligned and unaligned memory implementations (T030-T033)
   - Dedicated validation tasks (T050-T051) for both aligned and unaligned memory paths
   - Addresses FR-012 requirement for both aligned and unaligned code paths

## Validation Checklist
*GATE: Checked by main() before returning*

- [ ] All contracts have corresponding tests
- [ ] All entities have model tasks
- [ ] All tests come before implementation
- [ ] Regression tests included to verify existing functionality preservation
- [ ] Specialized SIMD validation tasks included to address FR-011
- [ ] Memory alignment tasks included to address FR-012 (both aligned and unaligned paths)
- [ ] Parallel tasks truly independent
- [ ] Each task specifies exact file path
- [ ] No task modifies same file as another [P] task