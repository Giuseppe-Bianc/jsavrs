---
description: "Task list for Cross-Platform x86_64 Assembly Code Generator"
---

# Tasks: Cross-Platform x86_64 Assembly Code Generator

**Input**: Design documents from `/specs/006-the-feature-to/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The feature specification requests comprehensive testing, so test tasks are included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create project structure per implementation plan in src/asm/codegen/
- [ ] T002 Initialize Rust project dependencies as defined in plan.md
- [ ] T003 [P] Configure linting and formatting tools per rustfmt.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Create CodegenContext struct in src/asm/codegen/context.rs
- [ ] T005 [P] Implement CodegenError enum in src/asm/codegen/error.rs
- [ ] T006 [P] Create Operand enum in src/asm/codegen/operand.rs
- [ ] T007 Create AssemblyInstruction struct in src/asm/codegen/instruction.rs
- [ ] T008 [P] Implement StackFrame struct in src/asm/codegen/stack_frame.rs
- [ ] T009 Create CodeGenerator trait in src/asm/codegen/mod.rs
- [ ] T010 [P] Configure test infrastructure in tests/
- [ ] T011 [P] Implement Error conversion from CodegenError to CompileError

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Generate Target Assembly Code from IR (Priority: P1) 🎯 MVP

**Goal**: System can accept IR input and produce correct assembly code that compiles without errors and follows the appropriate ABI for the target platform

**Independent Test**: The system can accept IR input and produce correct assembly code that compiles without errors and follows the appropriate ABI for the target platform

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T012 [P] [US1] Create basic IR-to-assembly test in tests/codegen_tests.rs
- [ ] T013 [P] [US1] Create Windows ABI compliance test in tests/abi_tests.rs
- [ ] T014 [P] [US1] Create SystemV ABI compliance test in tests/abi_tests.rs
- [ ] T015 [P] [US1] Create simple function generation test in tests/codegen_snapshot_tests.rs

### Implementation for User Story 1

- [ ] T016 [P] [US1] Implement DefaultCodeGenerator struct in src/asm/codegen/generator.rs
- [ ] T017 [US1] Implement generate_module method in src/asm/codegen/generator.rs
- [ ] T018 [US1] Implement generate_function method in src/asm/codegen/generator.rs
- [ ] T019 [P] [US1] Implement InstructionSelector trait in src/asm/codegen/instruction_selector.rs
- [ ] T020 [US1] Implement select_instruction for basic operations in src/asm/codegen/instruction_selector.rs
- [ ] T021 [US1] Implement select_instruction for function calls in src/asm/codegen/instruction_selector.rs
- [ ] T022 [US1] Implement ValueMapper trait in src/asm/codegen/value_mapper.rs
- [ ] T023 [US1] Implement map_value for literals in src/asm/codegen/value_mapper.rs
- [ ] T024 [US1] Implement map_value for local variables in src/asm/codegen/value_mapper.rs
- [ ] T025 [US1] Implement map_value for temporary values in src/asm/codegen/value_mapper.rs
- [ ] T026 [P] [US1] Implement Emitter for text generation in src/asm/codegen/emitter.rs
- [ ] T027 [US1] Implement ABI selection based on platform in src/asm/codegen/generator.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Support Cross-Platform ABI Selection (Priority: P2)

**Goal**: System correctly identifies the target platform and generates assembly code that follows the appropriate ABI conventions without user needing to specify it

**Independent Test**: The system correctly identifies the target platform and generates assembly code that follows the appropriate ABI conventions without user needing to specify it

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️

- [ ] T028 [P] [US2] Create Windows ABI detection test in tests/abi_tests.rs
- [ ] T029 [P] [US2] Create Linux ABI detection test in tests/abi_tests.rs
- [ ] T030 [P] [US2] Create MacOS ABI detection test in tests/abi_tests.rs

### Implementation for User Story 2

- [ ] T031 [US2] Enhance ABI selection logic to auto-detect platform in src/asm/abi.rs
- [ ] T032 [US2] Implement platform-specific parameter mapping in src/asm/codegen/instruction_selector.rs
- [ ] T033 [US2] Implement platform-specific return value handling in src/asm/codegen/instruction_selector.rs
- [ ] T034 [US2] Implement shadow space handling for Windows in src/asm/codegen/function_prologue.rs
- [ ] T035 [US2] Implement red zone handling for SystemV in src/asm/codegen/function_prologue.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Generate Well-Formed Assembly with Proper Resource Management (Priority: P3)

**Goal**: Generated assembly code correctly follows resource usage conventions for the target ABI and maintains proper resource state across function calls

**Independent Test**: Generated assembly code correctly follows resource usage conventions for the target ABI and maintains proper resource state across function calls

### Tests for User Story 3 (OPTIONAL - only if tests requested) ⚠️

- [ ] T036 [P] [US3] Create parameter passing test in tests/codegen_tests.rs
- [ ] T037 [P] [US3] Create return value handling test in tests/codegen_tests.rs
- [ ] T038 [P] [US3] Create resource preservation test in tests/codegen_tests.rs

### Implementation for User Story 3

- [ ] T039 [US3] Implement RegisterAllocator trait in src/asm/codegen/register_allocator.rs
- [ ] T040 [US3] Implement GP register allocation in src/asm/codegen/register_allocator.rs
- [ ] T041 [US3] Implement XMM register allocation in src/asm/codegen/register_allocator.rs
- [ ] T042 [US3] Implement register spilling mechanism in src/asm/codegen/register_allocator.rs
- [ ] T043 [US3] Implement function prologue generation in src/asm/codegen/function_prologue.rs
- [ ] T044 [US3] Implement function epilogue generation in src/asm/codegen/function_epilogue.rs
- [ ] T045 [US3] Implement callee-saved register preservation in src/asm/codegen/function_prologue.rs
- [ ] T046 [US3] Implement stack frame management in src/asm/codegen/stack_frame.rs
- [ ] T047 [US3] Implement alignment in stack frame allocation in src/asm/codegen/stack_frame.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T048 [P] Documentation updates in docs/
- [ ] T049 Code cleanup and refactoring
- [ ] T050 Performance optimization across all stories
- [ ] T051 [P] Additional unit tests for edge cases in tests/unit/
- [ ] T052 Security hardening
- [ ] T053 Run quickstart.md validation
- [ ] T054 [P] Implement error enrichment for assembler failures in src/asm/codegen/error.rs
- [ ] T055 [P] Implement detailed error messages with line numbers in src/asm/codegen/error.rs
- [ ] T056 [P] Complete rustdoc comments for all public APIs in src/asm/codegen/

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "Create basic IR-to-assembly test in tests/codegen_tests.rs"
Task: "Create Windows ABI compliance test in tests/abi_tests.rs"
Task: "Create SystemV ABI compliance test in tests/abi_tests.rs"
Task: "Create simple function generation test in tests/codegen_snapshot_tests.rs"

# Launch all implementation tasks for User Story 1 together:
Task: "Implement DefaultCodeGenerator struct in src/asm/codegen/generator.rs"
Task: "Implement InstructionSelector trait in src/asm/codegen/instruction_selector.rs"
Task: "Implement ValueMapper trait in src/asm/codegen/value_mapper.rs"
Task: "Implement Emitter for text generation in src/asm/codegen/emitter.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
