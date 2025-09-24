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
- [ ] T001 Install required dependencies for SIMD support (cpu-feature, std::arch)
- [ ] T002 [P] Configure Rust SIMD compilation flags and target features
- [ ] T003 [P] Add SIMD-specific testing utilities and validation harness

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] Contract test for SIMD add_vectors operation in tests/contract/test_simd_operations.rs
- [ ] T005 [P] Contract test for SIMD multiply_vectors operation in tests/contract/test_simd_operations.rs
- [ ] T006 [P] Contract test for SIMD subtract_vectors operation in tests/contract/test_simd_operations.rs
- [ ] T007 [P] Contract test for SIMD vectorize_loop operation in tests/contract/test_simd_operations.rs
- [ ] T008 [P] Contract test for SIMD check_cpu_support operation in tests/contract/test_simd_operations.rs
- [ ] T009 [P] Integration test for SSE instruction generation in tests/integration/test_sse_generation.rs
- [ ] T010 [P] Integration test for SSE2 instruction generation in tests/integration/test_sse2_generation.rs
- [ ] T011 [P] Integration test for CPU feature detection in tests/integration/test_feature_detection.rs
- [ ] T012 [P] Integration test for vector loop optimization in tests/integration/test_vectorization.rs
- [ ] T013 [P] Integration test for scalar fallback functionality in tests/integration/test_fallback.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T014 [P] SSEInstruction model in src/asm/sse_instruction.rs
- [ ] T015 [P] Operand model in src/asm/operand.rs
- [ ] T016 [P] CPUFeature model in src/asm/cpu_feature.rs
- [ ] T017 [P] AssemblyBlock model in src/asm/assembly_block.rs
- [ ] T018 [P] InstructionMetadata model in src/asm/instruction_metadata.rs
- [ ] T019 [P] SIMDValue model in src/value/simd_value.rs
- [ ] T020 [P] SIMDProcessor model in src/compiler/simd_processor.rs
- [ ] T021 [P] SIMDOperations trait in src/asm/simd_operations.rs
- [ ] T022 SIMD add_vectors implementation in src/asm/simd_operations.rs
- [ ] T023 SIMD multiply_vectors implementation in src/asm/simd_operations.rs
- [ ] T024 SIMD subtract_vectors implementation in src/asm/simd_operations.rs
- [ ] T025 SIMD vectorize_loop implementation in src/optimizer/vectorizer.rs
- [ ] T026 SIMD check_cpu_support implementation in src/compiler/cpu_detector.rs
- [ ] T027 SSE instruction generation in src/asm/generator.rs
- [ ] T028 SSE2 instruction generation in src/asm/generator.rs
- [ ] T029 CPU feature detection implementation in src/compiler/cpu_detector.rs
- [ ] T030 Scalar fallback implementations in src/asm/scalar_fallback.rs

## Phase 3.4: Integration
- [ ] T031 Connect SIMD operations to main compiler pipeline
- [ ] T032 Integrate SIMD detection with code generation phase
- [ ] T033 Add SIMD-specific error handling and logging
- [ ] T034 Add CLI flags for enabling/disabling SIMD optimization

## Phase 3.5: Polish
- [ ] T035 [P] Unit tests for SSEInstruction in tests/unit/test_sse_instruction.rs
- [ ] T036 [P] Unit tests for Operand in tests/unit/test_operand.rs
- [ ] T037 [P] Unit tests for CPUFeature in tests/unit/test_cpu_feature.rs
- [ ] T038 [P] Unit tests for AssemblyBlock in tests/unit/test_assembly_block.rs
- [ ] T039 [P] Unit tests for SIMDValue in tests/unit/test_simd_value.rs
- [ ] T040 [P] Unit tests for SIMDProcessor in tests/unit/test_simd_processor.rs
- [ ] T041 [P] Unit tests for SIMDOperations trait in tests/unit/test_simd_operations.rs
- [ ] T042 Performance tests for SIMD operations (<200ms for 1000 ops)
- [ ] T043 [P] Update docs/simd_support.md
- [ ] T044 [P] Update docs/api.md with SIMD features
- [ ] T045 Remove code duplication in scalar/SIMD implementations
- [ ] T046 Run manual-testing.md validation scenarios

## Dependencies
- Tests (T004-T013) before implementation (T014-T030)
- T014-T021 blocks T022-T030
- T021 blocks T025
- T016 blocks T029
- T031 blocks T032
- Implementation before polish (T035-T046)

## Parallel Example
```
# Launch T004-T008 together:
Task: "Contract test for SIMD add_vectors operation in tests/contract/test_simd_operations.rs"
Task: "Contract test for SIMD multiply_vectors operation in tests/contract/test_simd_operations.rs"
Task: "Contract test for SIMD subtract_vectors operation in tests/contract/test_simd_operations.rs"
Task: "Contract test for SIMD vectorize_loop operation in tests/contract/test_simd_operations.rs"
Task: "Contract test for SIMD check_cpu_support operation in tests/contract/test_simd_operations.rs"

# Launch T014-T021 together:
Task: "SSEInstruction model in src/asm/sse_instruction.rs"
Task: "Operand model in src/asm/operand.rs"
Task: "CPUFeature model in src/asm/cpu_feature.rs"
Task: "AssemblyBlock model in src/asm/assembly_block.rs"
Task: "InstructionMetadata model in src/asm/instruction_metadata.rs"
Task: "SIMDValue model in src/value/simd_value.rs"
Task: "SIMDProcessor model in src/compiler/simd_processor.rs"
Task: "SIMDOperations trait in src/asm/simd_operations.rs"
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