# Tasks: IR to x86-64 Assembly Code Generator

**Input**: Design documents from `/specs/021-ir-x86-codegen/`  
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

**Tests**: Included (insta snapshots + unit tests as per spec.md success criteria)

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label (US1, US2, ... US10)
- All paths relative to repository root

---

## Phase 1: Setup (Project Infrastructure)

**Purpose**: Module structure and basic types

- [ ] T001 Create codegen module structure in src/asm/codegen/mod.rs with submodule declarations
- [ ] T002 [P] Create error types in src/asm/codegen/error.rs (CodeGenError enum)
- [ ] T003 [P] Create stats types in src/asm/codegen/stats.rs (CodeGenStats, InstructionKind)
- [ ] T004 [P] Create options types in src/asm/codegen/mod.rs (CodeGenOptions with Default)
- [ ] T005 Create regalloc module structure in src/asm/regalloc/mod.rs with submodule declarations
- [ ] T006 [P] Create phi module structure in src/asm/phi/mod.rs with submodule declarations
- [ ] T007 [P] Create lowering module structure in src/asm/lowering/mod.rs with submodule declarations
- [ ] T008 Update src/asm/mod.rs to export new codegen, regalloc, phi, lowering modules

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure required by ALL user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 Implement PhysicalRegister enum (GP/Simd variants) in src/asm/regalloc/mapping.rs
- [ ] T010 Implement SpillSlotId and SpillSlot types in src/asm/regalloc/spill.rs
- [ ] T011 Implement RegisterMapping struct with get/is_spilled methods in src/asm/regalloc/mapping.rs
- [ ] T012 [P] Implement ValueId type alias and InstructionId in src/asm/codegen/context.rs
- [ ] T013 Implement StackFrame struct (size, spill_slots, locals, callee_saved_used) in src/asm/codegen/context.rs
- [ ] T014 Implement GenerationContext struct in src/asm/codegen/context.rs
- [ ] T015 [P] Add RegisterClass enum (GeneralPurpose, Simd) in src/asm/regalloc/interval.rs
- [ ] T016 Create test fixtures for IR module/function creation in tests/codegen_fixtures.rs

**Checkpoint**: Foundation ready - user story implementation can begin

---

## Phase 3: User Story 1 - Generate Basic x86-64 Assembly from IR (Priority: P1) 🎯 MVP

**Goal**: Transform validated IR module into x86-64 assembly with correct Intel syntax

**Independent Test**: Provide simple IR with arithmetic ops, verify .text section with correct instructions

### Tests for User Story 1

- [ ] T017 [P] [US1] Create snapshot test for arithmetic operations in tests/codegen_snapshot_tests.rs
- [ ] T018 [P] [US1] Create unit tests for arithmetic lowering in tests/codegen_arithmetic_tests.rs

### Implementation for User Story 1

- [ ] T019 [US1] Implement arithmetic instruction lowering (add, sub, mul) in src/asm/lowering/arithmetic.rs
- [ ] T020 [US1] Implement arithmetic instruction lowering (div, rem) with special handling in src/asm/lowering/arithmetic.rs
- [ ] T021 [US1] Implement bitwise operations (and, or, xor, shl, shr) in src/asm/lowering/arithmetic.rs
- [ ] T022 [US1] Implement comparison lowering (cmp + setcc) in src/asm/lowering/arithmetic.rs
- [ ] T023 [US1] Implement InstructionEmitter for basic block emission in src/asm/codegen/emitter.rs
- [ ] T024 [US1] Implement label generation for basic blocks in src/asm/codegen/emitter.rs
- [ ] T025 [US1] Implement CodeGenerator::generate() main entry point in src/asm/codegen/mod.rs
- [ ] T026 [US1] Implement function-level code generation loop in src/asm/codegen/mod.rs
- [ ] T027 [US1] Add register size selection (AL/AX/EAX/RAX) based on IR type in src/asm/lowering/arithmetic.rs
- [ ] T100 [US1] Implement memory load lowering in src/asm/lowering/memory.rs
- [ ] T101 [US1] Implement memory store lowering in src/asm/lowering/memory.rs
- [ ] T102 [US1] Implement array element address calculation in src/asm/lowering/memory.rs
- [ ] T103 [US1] Implement struct field address calculation in src/asm/lowering/memory.rs
- [ ] T104 [P] [US1] Implement type conversion lowering (extend/truncate) in src/asm/lowering/conversion.rs
- [ ] T105 [P] [US1] Implement float-to-int conversion in src/asm/lowering/conversion.rs
- [ ] T106 [P] [US1] Implement int-to-float conversion in src/asm/lowering/conversion.rs
- [ ] T107 [US1] Implement conditional/unconditional branch lowering in src/asm/lowering/control.rs
- [ ] T117 [US1] Implement zero-sized type handling (no storage, ops optimized away) in src/asm/lowering/mod.rs
- [ ] T118 [US1] Implement unreachable block detection and warning emission in src/asm/codegen/emitter.rs

**Checkpoint**: User Story 1 complete - basic arithmetic IR compiles to assembly

---

## Phase 4: User Story 2 - Platform-Specific Code Generation (Priority: P1)

**Goal**: Generate assembly following each platform's calling conventions

**Independent Test**: Generate same IR function for Linux/macOS/Windows, verify different register usage

### Tests for User Story 2

- [ ] T028 [P] [US2] Create platform ABI tests for Linux in tests/codegen_platform_tests.rs
- [ ] T029 [P] [US2] Create platform ABI tests for Windows in tests/codegen_platform_tests.rs
- [ ] T030 [P] [US2] Create platform ABI tests for macOS in tests/codegen_platform_tests.rs

### Implementation for User Story 2

- [ ] T031 [US2] Implement parameter register selection for System V ABI in src/asm/codegen/context.rs
- [ ] T032 [US2] Implement parameter register selection for Windows x64 ABI in src/asm/codegen/context.rs
- [ ] T033 [US2] Implement shadow space allocation for Windows in src/asm/codegen/emitter.rs
- [ ] T034 [US2] Implement red zone handling for System V in src/asm/codegen/emitter.rs
- [ ] T035 [US2] Add underscore symbol prefixing for macOS in src/asm/codegen/emitter.rs
- [ ] T036 [US2] Implement floating-point parameter passing for System V (XMM0-7) in src/asm/codegen/context.rs
- [ ] T037 [US2] Implement slot-based float parameters for Windows (XMM0-3) in src/asm/codegen/context.rs

**Checkpoint**: User Story 2 complete - platform-specific ABI compliance

---

## Phase 5: User Story 3 - Function Prologue and Epilogue Generation (Priority: P1)

**Goal**: Correct stack frames with callee-saved register preservation

**Independent Test**: Generate function using callee-saved registers, verify push/pop in prologue/epilogue

### Tests for User Story 3

- [ ] T038 [P] [US3] Create prologue/epilogue snapshot tests in tests/codegen_snapshot_tests.rs
- [ ] T039 [P] [US3] Create callee-saved register preservation tests in tests/codegen_regalloc_tests.rs

### Implementation for User Story 3

- [ ] T040 [US3] Implement prologue emission (push rbp, mov rbp rsp) in src/asm/codegen/emitter.rs
- [ ] T041 [US3] Implement epilogue emission (mov rsp rbp, pop rbp, ret) in src/asm/codegen/emitter.rs
- [ ] T042 [US3] Implement callee-saved register detection in src/asm/codegen/context.rs
- [ ] T043 [US3] Implement callee-saved register push/pop in src/asm/codegen/emitter.rs
- [ ] T044 [US3] Implement stack frame size calculation with 16-byte alignment in src/asm/codegen/context.rs
- [ ] T045 [US3] Implement stack space allocation (sub rsp, size) in src/asm/codegen/emitter.rs
- [ ] T109 [US3] Implement return instruction lowering in src/asm/lowering/control.rs

**Checkpoint**: User Story 3 complete - proper function frames

---

## Phase 6: User Story 4 - Register Allocation and Spilling (Priority: P2)

**Goal**: Efficient register allocation with spilling when needed

**Independent Test**: Provide IR with >16 live values, verify spill/reload code generated

### Tests for User Story 4

- [ ] T046 [P] [US4] Create liveness analysis unit tests in tests/codegen_regalloc_tests.rs
- [ ] T047 [P] [US4] Create Linear Scan allocation tests in tests/codegen_regalloc_tests.rs
- [ ] T048 [P] [US4] Create spill code generation tests in tests/codegen_regalloc_tests.rs

### Implementation for User Story 4

- [ ] T049 [US4] Implement LiveInterval struct in src/asm/regalloc/interval.rs
- [ ] T050 [US4] Implement liveness analysis (live_in/live_out per block) in src/asm/regalloc/liveness.rs
- [ ] T051 [US4] Implement instruction numbering for intervals in src/asm/regalloc/liveness.rs
- [ ] T052 [US4] Implement interval building from liveness in src/asm/regalloc/liveness.rs
- [ ] T053 [US4] Implement LinearScanAllocator struct in src/asm/regalloc/mod.rs
- [ ] T054 [US4] Implement active set management (expire_old_intervals) in src/asm/regalloc/mod.rs
- [ ] T055 [US4] Implement spill slot allocation in src/asm/regalloc/spill.rs
- [ ] T056 [US4] Implement spill/reload instruction emission in src/asm/codegen/emitter.rs
- [ ] T057 [US4] Integrate register allocation into CodeGenerator pipeline in src/asm/codegen/mod.rs

**Checkpoint**: User Story 4 complete - Linear Scan allocation working

---

## Phase 7: User Story 5 - SSA Phi Node Resolution (Priority: P2)

**Goal**: Resolve phi nodes into move instructions at predecessor ends

**Independent Test**: IR with if-else merging to phi, verify correct moves in predecessors

### Tests for User Story 5

- [ ] T058 [P] [US5] Create phi resolution unit tests in tests/codegen_phi_tests.rs
- [ ] T059 [P] [US5] Create parallel copy cycle detection tests in tests/codegen_phi_tests.rs

### Implementation for User Story 5

- [ ] T060 [US5] Implement PhiMove struct (src, dst, size) in src/asm/phi/mod.rs
- [ ] T061 [US5] Implement phi node collection per block in src/asm/phi/mod.rs
- [ ] T062 [US5] Implement parallel copy graph construction in src/asm/phi/parallel_copy.rs
- [ ] T063 [US5] Implement cycle detection in parallel copies in src/asm/phi/parallel_copy.rs
- [ ] T064 [US5] Implement copy sequentialization with temporary for cycles in src/asm/phi/parallel_copy.rs
- [ ] T065 [US5] Implement phi move insertion at predecessor block ends in src/asm/codegen/emitter.rs
- [ ] T066 [US5] Integrate phi resolution into CodeGenerator pipeline in src/asm/codegen/mod.rs

**Checkpoint**: User Story 5 complete - SSA phi nodes correctly resolved

---

## Phase 8: User Story 6 - Data Section Generation (Priority: P2)

**Goal**: Place strings, globals, constants in correct assembly sections

**Independent Test**: IR with string literal and global variable, verify .data/.rodata/.bss sections

### Tests for User Story 6

- [ ] T067 [P] [US6] Create data section snapshot tests in tests/codegen_snapshot_tests.rs
- [ ] T068 [P] [US6] Create global variable tests in tests/codegen_integration_tests.rs

### Implementation for User Story 6

- [ ] T069 [US6] Implement string literal emission to .rodata in src/asm/lowering/data.rs
- [ ] T070 [US6] Implement initialized global emission to .data in src/asm/lowering/data.rs
- [ ] T071 [US6] Implement uninitialized global emission to .bss in src/asm/lowering/data.rs
- [ ] T072 [US6] Implement data directive selection (db/dw/dd/dq) by size in src/asm/lowering/data.rs
- [ ] T073 [US6] Implement alignment directives for data types in src/asm/lowering/data.rs
- [ ] T074 [US6] Implement null termination for strings in src/asm/lowering/data.rs
- [ ] T075 [US6] Integrate data section emission into CodeGenerator in src/asm/codegen/mod.rs

**Checkpoint**: User Story 6 complete - data sections properly generated

---

## Phase 9: User Story 7 - Function Call Generation (Priority: P2)

**Goal**: Correct argument passing and return value handling for calls

**Independent Test**: IR calling function with 8 params (some float), verify register/stack usage

### Tests for User Story 7

- [ ] T076 [P] [US7] Create function call snapshot tests in tests/codegen_snapshot_tests.rs
- [ ] T077 [P] [US7] Create variadic and 20-parameter function call tests (per SC-006) in tests/codegen_integration_tests.rs

### Implementation for User Story 7

- [ ] T078 [US7] Implement call instruction lowering in src/asm/lowering/control.rs
- [ ] T079 [US7] Implement register argument setup for calls in src/asm/lowering/control.rs
- [ ] T080 [US7] Implement stack argument push (excess params) in src/asm/lowering/control.rs
- [ ] T081 [US7] Implement return value retrieval (RAX/XMM0) in src/asm/lowering/control.rs
- [ ] T082 [US7] Implement caller-saved register spill before call in src/asm/codegen/emitter.rs
- [ ] T083 [US7] Implement global and extern directive emission (global for exported symbols, extern for external calls) in src/asm/codegen/emitter.rs
- [ ] T084 [US7] Implement large return value handling (hidden pointer) in src/asm/lowering/control.rs
- [ ] T085 [US7] Implement variadic function call support in src/asm/lowering/control.rs

**Checkpoint**: User Story 7 complete - function calls working

---

## Phase 10: User Story 8 - Debug Comments and Annotations (Priority: P3)

**Goal**: Include debug info in assembly output for tracing

**Independent Test**: Generate with emit_debug_comments=true, verify comments present

### Implementation for User Story 8

- [ ] T086 [US8] Implement debug comment emission for variable names in src/asm/codegen/emitter.rs
- [ ] T087 [US8] Implement source location comments in src/asm/codegen/emitter.rs
- [ ] T088 [US8] Implement basic block boundary comments in src/asm/codegen/emitter.rs
- [ ] T089 [US8] Create snapshot tests for debug output in tests/codegen_snapshot_tests.rs

**Checkpoint**: User Story 8 complete - debug comments available

---

## Phase 11: User Story 9 - Control Flow Optimization (Priority: P3)

**Goal**: Eliminate unnecessary jumps for fall-through blocks

**Independent Test**: Consecutive blocks with unconditional jump, verify no jmp emitted

### Implementation for User Story 9

- [ ] T090 [US9] Implement block ordering analysis for fall-through in src/asm/codegen/context.rs
- [ ] T091 [US9] Implement unconditional jump elimination in src/asm/lowering/control.rs
- [ ] T092 [US9] Implement conditional branch fall-through optimization in src/asm/lowering/control.rs
- [ ] T093 [US9] Create fall-through optimization tests in tests/codegen_snapshot_tests.rs

**Checkpoint**: User Story 9 complete - optimized control flow

---

## Phase 12: User Story 10 - Code Generation Statistics (Priority: P3)

**Goal**: Collect and report generation metrics

**Independent Test**: Generate with collect_stats=true, verify stats.report() output

### Implementation for User Story 10

- [ ] T094 [US10] Implement instruction counting in CodeGenStats in src/asm/codegen/stats.rs
- [ ] T095 [US10] Implement register usage tracking in src/asm/codegen/stats.rs
- [ ] T096 [US10] Implement spill count tracking in src/asm/codegen/stats.rs
- [ ] T097 [US10] Implement stats.report() formatting in src/asm/codegen/stats.rs
- [ ] T098 [US10] Integrate stats collection into CodeGenerator in src/asm/codegen/mod.rs
- [ ] T099 [US10] Create stats output tests in tests/codegen_integration_tests.rs

**Checkpoint**: User Story 10 complete - statistics available

---

## Phase 13: Additional Lowering & Integration

**Purpose**: Advanced control flow (switch statements per FR-046)

- [ ] T108 [US7] Implement switch statement lowering (jump table ≥4 cases, cascaded otherwise) in src/asm/lowering/control.rs
- [ ] T116 [US7] Create jump table generation tests in tests/codegen_snapshot_tests.rs

---

## Phase 14: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and quality improvements

- [ ] T110 [P] Create NASM validation integration tests in tests/codegen_integration_tests.rs
- [ ] T111 Run quickstart.md scenarios and verify outputs
- [ ] T112 [P] Add rustdoc documentation to all public APIs
- [ ] T113 Code cleanup: run clippy and fix warnings
- [ ] T114 Run full test suite and verify >90% coverage on new modules
- [ ] T115 Update README.md with codegen usage examples

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1 (Setup) ─────────────────────────────────────────┐
                                                          │
Phase 2 (Foundational) ◄──────────────────────────────────┘
    │
    ├──▶ Phase 3 (US1: Basic Assembly) 🎯 MVP
    │         │
    │         ▼
    ├──▶ Phase 4 (US2: Platform-Specific)
    │         │
    │         ▼
    ├──▶ Phase 5 (US3: Prologue/Epilogue)
    │
    ├──▶ Phase 6 (US4: Register Allocation)
    │         │
    │         ▼
    ├──▶ Phase 7 (US5: Phi Resolution)
    │
    ├──▶ Phase 8 (US6: Data Sections)
    │
    ├──▶ Phase 9 (US7: Function Calls)
    │
    ├──▶ Phase 10 (US8: Debug Comments)
    │
    ├──▶ Phase 11 (US9: Control Flow Opt)
    │
    └──▶ Phase 12 (US10: Statistics)
              │
              ▼
         Phase 13 (Additional Lowering)
              │
              ▼
         Phase 14 (Polish)
```

### User Story Dependencies

| Story                    |  Depends On  | Can Parallel With |
| ------------------------ | ------------ | ----------------- |
| US1 (Basic Assembly)     | Foundational | None (MVP)        |
| US2 (Platform ABI)       | US1          | US3               |
| US3 (Prologue/Epilogue)  | US1          | US2               |
| US4 (Register Alloc)     | US1, US3     | US6               |
| US5 (Phi Resolution)     | US4          | US6               |
| US6 (Data Sections)      | US1          | US4, US5          |
| US7 (Function Calls)     | US2, US3     | US8               |
| US8 (Debug Comments)     | US1          | US9, US10         |
| US9 (Control Flow Opt)   | US1, US3     | US8, US10         |
| US10 (Statistics)        | US1          | US8, US9          |

### Parallel Opportunities Within User Stories

**Phase 1 (Setup)**: T002, T003, T004, T006, T007 can run in parallel  
**Phase 2 (Foundational)**: T012, T015 can run in parallel  
**Phase 3 (US1)**: T017, T018 tests parallel; T104, T105, T106 type conversions parallel  
**Phase 6 (US4)**: T046, T047, T048 can run in parallel  
**Phase 7 (US5)**: T058, T059 can run in parallel
**Phase 13 (US7)**: T108, T116 switch lowering tasks

---

## Parallel Example: User Story 1

```bash
# Launch all tests for US1 together:
Task T017: Create snapshot test for arithmetic operations
Task T018: Create unit tests for arithmetic lowering

# Then implementation (sequential within story):
Task T019 → T020 → T021 → T022 → T023 → T024 → T025 → T026 → T027
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (**CRITICAL**)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Basic arithmetic IR → assembly works
5. Continue with US2, US3 for production-ready P1 features

### Incremental Delivery

| Milestone | User Stories | Capability                                      |
| --------- | ------------ | ----------------------------------------------- |
| MVP       | US1          | Basic IR → assembly                             |
| Alpha     | US1-3        | Platform-aware with proper frames               |
| Beta      | US1-7        | Full function calls and register allocation     |
| Release   | US1-10       | Complete with debug info and stats              |

### P1 Stories Complete Threshold

After completing US1, US2, US3:

- Basic assembly generation ✓
- All 3 platforms supported ✓
- Correct function frames ✓
- Can compile simple programs end-to-end

---

## Notes

- `[P]` tasks use different files with no dependencies on incomplete tasks
- `[Story]` label maps tasks to user stories for traceability
- Each user story is independently testable after completion
- Verify tests fail before implementing (TDD)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- All file paths are relative to repository root
