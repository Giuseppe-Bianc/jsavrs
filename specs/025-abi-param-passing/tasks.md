# Tasks: ABI-Compliant Parameter Passing and Return Value Handling

**Input**: Design documents from `/specs/025-abi-param-passing/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

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

- [ ] T001 Create new module files in src/codegen/ per implementation plan
- [ ] T002 [P] Add new module declarations to src/codegen/mod.rs
- [ ] T003 [P] Verify existing infrastructure (Abi, IrType, IrParameter, CompileError) is accessible

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Create src/codegen/param.rs with ParamLocation enum
- [ ] T005 Create src/codegen/param.rs with ParamAssignment struct
- [ ] T006 Create src/codegen/param.rs with ParamClass enum
- [ ] T007 Create src/codegen/stack.rs with StackFrame struct
- [ ] T008 Create src/codegen/prologue.rs with PrologueConfig struct
- [ ] T009 Create src/codegen/epilogue.rs with EpilogueConfig struct
- [ ] T010 Create src/codegen/ret.rs with ReturnConfig struct
- [ ] T011 Create src/codegen/target.rs with resolve_abi function (must also replace existing `todo!()` in `AsmGen::target_triple_to_abi()` for non-x86_64 targets with a `CompileError` return)
- [ ] T012 Create src/codegen/error.rs with error construction helpers

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Integer Parameter Mapping to Registers (Priority: P1) 🎯 MVP

**Goal**: Implement correct assignment of integer parameters to ABI-specified registers based on target platform (System V: RDI, RSI, RDX, RCX, R8, R9; Windows: RCX, RDX, R8, R9)

**Independent Test**: Can be fully tested by compiling functions with 1 to 10 integer parameters on each target platform and verifying that the generated assembly assigns registers and stack slots correctly.

### Implementation for User Story 1

- [ ] T013 Implement classify_param_type function in src/codegen/param.rs
- [ ] T014 Implement classify_parameters function for integer parameters in src/codegen/param.rs
- [ ] T015 [US1] Extend gen_function to call classify_parameters for integer parameters
- [ ] T016 [P] [US1] Generate assembly for integer parameter register assignments in System V
- [ ] T017 [P] [US1] Generate assembly for integer parameter register assignments in Windows x64
- [ ] T018 [US1] Handle integer parameter stack spilling for excess parameters in System V
- [ ] T019 [US1] Handle integer parameter stack spilling for excess parameters in Windows x64

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Function Prologue and Epilogue Generation (Priority: P1)

**Goal**: Generate correct function prologue and epilogue that save/restore frame pointer, allocate stack space for locals, and preserve callee-saved registers according to ABI

**Independent Test**: Can be fully tested by compiling functions that use varying numbers of callee-saved registers and verifying that push/pop instructions and stack adjustments appear correctly in the generated assembly.

### Implementation for User Story 2

- [ ] T020 Implement compute_stack_frame function in src/codegen/stack.rs
- [ ] T021 Implement gen_prologue function in src/codegen/prologue.rs
- [ ] T022 Implement gen_epilogue function in src/codegen/epilogue.rs
- [ ] T023 [US2] Add frame pointer setup (push RBP; mov RBP, RSP) to prologue
- [ ] T024 [US2] Add stack space allocation for locals in prologue
- [ ] T025 [US2] Add callee-saved register preservation to prologue for System V
- [ ] T026 [US2] Add callee-saved register preservation to prologue for Windows x64
- [ ] T027 [US2] Add callee-saved register restoration to epilogue for System V
- [ ] T028 [US2] Add callee-saved register restoration to epilogue for Windows x64
- [ ] T029 [US2] Add Windows shadow space allocation to prologue
- [ ] T030 [US2] Add System V red zone handling to prologue

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Return Value Placement (Priority: P2)

**Goal**: Place return values in ABI-specified return registers (RAX for integers, XMM0 for floats)

**Independent Test**: Can be fully tested by compiling functions with different return types (integer 32-bit, integer 64-bit, float, double) and verifying the return register assignment in generated assembly.

### Implementation for User Story 3

- [ ] T031 Implement get_return_register function in src/codegen/ret.rs
- [ ] T032 [P] [US3] Handle integer return value placement in RAX (EAX for 32-bit, RAX for 64-bit)
- [ ] T033 [US3] Handle floating-point return value placement in XMM0 (F32 and F64)
- [ ] T034 [US3] Handle void return value (no register assignment)

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Floating-Point Parameter Mapping (Priority: P2)

**Goal**: Map floating-point parameters to ABI-specified XMM registers (System V: XMM0-XMM7; Windows: XMM0-XMM3 in positional slots)

**Independent Test**: Can be fully tested by compiling functions with varying numbers of floating-point parameters on each target platform and verifying register/stack assignments.

### Implementation for User Story 4

- [ ] T035 [US4] Extend classify_parameters to handle floating-point parameters in System V (depends on T013, T014)
- [ ] T036 [US4] Extend classify_parameters to handle floating-point parameters in Windows x64 (depends on T013, T014)
- [ ] T037 [US4] Generate assembly for floating-point parameter register assignments in System V
- [ ] T038 [US4] Generate assembly for floating-point parameter register assignments in Windows x64
- [ ] T039 [US4] Handle floating-point parameter stack spilling for excess parameters in System V
- [ ] T040 [US4] Handle floating-point parameter stack spilling for excess parameters in Windows x64

**Checkpoint**: At this point, User Stories 1, 2, 3, AND 4 should all work independently

---

## Phase 7: User Story 5 - Mixed Integer and Floating-Point Parameters (Priority: P3)

**Goal**: Handle functions with mixed integer and floating-point parameters, tracking separate register allocation counters for System V (independent sequences) and shared positional slots for Windows x64

**Independent Test**: Can be fully tested by compiling functions with interleaved integer and float parameters and verifying the register assignments match the ABI specification for each platform.

### Implementation for User Story 5

- [ ] T041 [US5] Implement correct mixed parameter classification for System V
- [ ] T042 [US5] Implement correct mixed parameter classification for Windows x64
- [ ] T043 [US5] Generate correct assembly for mixed parameter assignments in System V
- [ ] T044 [US5] Generate correct assembly for mixed parameter assignments in Windows x64
- [ ] T045 [P] [US5] Test mixed parameter handling for System V (independent integer/FP sequences)
- [ ] T046 [P] [US5] Test mixed parameter handling for Windows x64 (shared positional slots)

**Checkpoint**: All user stories should now be independently functional

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T047 [P] Add comprehensive error handling for unsupported parameter types (String, Array, Struct) — emit CompileError with error code E3001
- [ ] T048 [P] Add error handling for unsupported target triples (non-x86_64) — emit CompileError with error code E3002; also replace existing `todo!()` in `AsmGen::target_triple_to_abi()` with `CompileError` return
- [ ] T049 [P] Add rustdoc documentation to all public functions and types
- [ ] T050 [P] Add parameter validation to ensure stack alignment is maintained
- [ ] T051 [P] Add tests for edge cases (zero parameters, zero local variables, void returns, full callee-saved registers, only stack-spilled parameters exceeding register capacity)
- [ ] T052 [P] Add negative test: verify RSI/RDI are NOT preserved as callee-saved on Windows x64 (they are caller-saved on Windows — US2 acceptance scenario 4)
- [ ] T053 [P] Add test verifying by_val attribute is ignored — parameter with by_val=true must be classified by IrType only (FR-016)
- [ ] T054 [P] Update AsmGen::gen_function to properly orchestrate all new modules
- [ ] T055 [P] Run quickstart.md validation commands to ensure all functionality works
- [ ] T056 [P] Add insta snapshot tests for complete function assembly output
- [ ] T057 [P] Add criterion benchmark in benches/ measuring parameter classification throughput to validate O(n) performance goal
- [ ] T058 [P] Run full existing test suite (`cargo test`) and verify zero regressions (SC-004)
- [ ] T059 [P] Run `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings` on all new code
- [ ] T060 [P] Verify no `unwrap()` calls in new production modules: `grep -r unwrap src/codegen/` must return zero hits

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
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) + US1 core classification tasks (T013, T014) — depends on param infrastructure from US1
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) + US1 (T013, T014) + US4 — depends on both US1 and US4

### Within Each User Story

- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all implementation tasks for User Story 1 together:
Task: "Implement classify_param_type function in src/codegen/param.rs"
Task: "Implement classify_parameters function for integer parameters in src/codegen/param.rs"
Task: "Extend gen_function to call classify_parameters for integer parameters in src/codegen/asmgen.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 1 and 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Integer Parameter Mapping)
4. Complete Phase 4: User Story 2 (Prologue/Epilogue Generation)
5. **STOP and VALIDATE**: Test US1 and US2 independently
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (must complete T013/T014 before US4 can start)
   - Developer B: User Story 2
   - Developer C: User Story 3
3. After US1 core classification (T013, T014) is complete:
   - Developer D: User Story 4
4. Stories complete and integrate independently

---

## Notes

- **Performance**: O(n) code generation per function (where n = number of parameters) is ensured by design — single-pass iteration over the parameter list with constant-time register/stack assignment per parameter
- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence