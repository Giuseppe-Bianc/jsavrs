---

description: "Task list for IR to x86-64 Assembly Translator implementation"
---

# Tasks: IR to x86-64 Assembly Translator

**Input**: Design documents from `/specs/[023-ir-to-asm-translator]/`
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

<!--
  ============================================================================
  IMPORTANT: The tasks below are ACTUAL tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/
  
  Tasks are organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create src/translator/ directory structure
- [ ] T002 [P] Create initial module files: mod.rs, context.rs, function_translator.rs, block_translator.rs, instruction_translator.rs, terminator_translator.rs
- [ ] T003 [P] Create src/translator/codegen/ directory and abi_adapter.rs file
- [ ] T004 Add translator module to src/lib.rs exports
- [ ] T005 Configure insta and criterion dependencies in Cargo.toml if not already present

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

Examples of foundational tasks (adjust based on your project):

- [ ] T006 Define core data structures in src/translator/mod.rs (Translator, TranslationConfig, TranslationError)
- [ ] T007 [P] Implement TranslationConfig struct with target_abi, emit_mapping, debug_symbols fields
- [ ] T008 [P] Implement TranslationError and ErrorCode enums with E4001 variant
- [ ] T009 Create TranslationContext struct in src/translator/context.rs with ABI, symbol_table, register_allocator, etc.
- [ ] T010 Define assembly-related data structures (AssemblyInstruction, AssemblyOperand, MemoryLocation)
- [ ] T011 [P] Implement AbiAdapter in src/translator/codegen/abi_adapter.rs
- [ ] T012 [P] Create basic tests infrastructure in tests/translator_basic.rs
- [ ] T013 [P] Create benchmark infrastructure in benches/jsavrs_benchmark.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Translate IR to Valid Assembly (Priority: P1) 🎯 MVP

**Goal**: Convert IR structures from src/ir into valid x86-64 assembly code that can be assembled with NASM without errors

**Independent Test**: Providing a simple IR input and verifying that the output is syntactically correct x86-64 assembly that assembles successfully with NASM

### Implementation for User Story 1

- [ ] T014 [P] [US1] Implement basic Translator::translate_module function in src/translator/mod.rs
- [ ] T015 [P] [US1] Implement FunctionTranslator in src/translator/function_translator.rs
- [ ] T016 [US1] Implement BlockTranslator in src/translator/block_translator.rs
- [ ] T017 [US1] Implement InstructionTranslator in src/translator/instruction_translator.rs with basic arithmetic operations
- [ ] T018 [US1] Implement TerminatorTranslator in src/translator/terminator_translator.rs
- [ ] T019 [US1] Add basic IR to assembly mapping for BinaryOp variants (Add, Sub, Mul, Div)
- [ ] T020 [US1] Implement assembly emission to AssemblyFile structure
- [ ] T021 [US1] Add NASM syntax validation to ensure output assembles correctly
- [ ] T022 [US1] Create basic test case with simple function in tests/translator_basic.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Maintain Semantic Consistency (Priority: P1)

**Goal**: Ensure the translator maintains semantic consistency between IR and assembly so that generated code behaves identically to the original high-level representation

**Independent Test**: Comparing the execution behavior of the original IR and the generated assembly code with identical inputs to verify functional equivalence

### Implementation for User Story 2

- [ ] T023 [P] [US2] Enhance InstructionTranslator to handle Load and Store operations with semantic consistency
- [ ] T024 [US2] Implement proper control flow translation for conditional logic in TerminatorTranslator
- [ ] T025 [US2] Add function call translation with proper calling convention handling
- [ ] T026 [US2] Implement parameter passing consistency between IR and assembly
- [ ] T027 [US2] Add return value handling with semantic consistency
- [ ] T028 [US2] Create semantic consistency tests comparing IR and assembly behavior
- [ ] T029 [US2] Add validation to ensure equivalent execution behavior

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Follow Architecture Conventions (Priority: P2)

**Goal**: Ensure the translator follows x86-64 architecture conventions so that generated assembly integrates properly with other system components and standard toolchains

**Independent Test**: Verifying that generated assembly follows x86-64 register usage, calling conventions, and stack management patterns for compatibility with standard toolchains

### Implementation for User Story 3

- [ ] T030 [P] [US3] Implement System V AMD64 ABI support in AbiAdapter
- [ ] T031 [P] [US3] Implement Windows x64 ABI support in AbiAdapter
- [ ] T032 [US3] Add proper register allocation following x86-64 conventions
- [ ] T033 [US3] Implement stack frame management with proper prologue/epilogue
- [ ] T034 [US3] Add calling convention compliance for parameter passing
- [ ] T035 [US3] Implement proper x86-64 section organization (.text, .data, .bss)
- [ ] T036 [US3] Add architecture-specific validation tests in tests/translator_abi.rs

**Checkpoint**: At this point, User Stories 1, 2 AND 3 should all work independently

---

## Phase 6: User Story 4 - Integrate with Existing Assembly Generator (Priority: P2)

**Goal**: Ensure the translator integrates with existing code in src/asm so that existing abstractions are leveraged without duplicating logic

**Independent Test**: Verifying that the translator utilizes existing src/asm components appropriately without reimplementing their functionality

### Implementation for User Story 4

- [ ] T037 [P] [US4] Integrate with existing src/asm/instruction.rs Instruction enum
- [ ] T038 [US4] Use existing AssemblyFile::text_sec_add_instruction() for final assembly emission
- [ ] T039 [US4] Leverage existing src/asm/abi.rs for ABI abstractions
- [ ] T040 [US4] Reuse existing assembly infrastructure instead of duplicating functionality
- [ ] T041 [US4] Ensure compatibility with existing src/asm components
- [ ] T042 [US4] Add integration tests to verify proper use of existing components in tests/translator_basic.rs

**Checkpoint**: At this point, all user stories should be independently functional

---

## Phase 7: User Story 5 - Support Extensibility for Future IR Changes (Priority: P3)

**Goal**: Design the translator to be extensible so that future evolutions of the IR can be accommodated without major rewrites

**Independent Test**: Extending the IR with a new construct and verifying that the translator can handle it with minimal changes

### Implementation for User Story 5

- [ ] T043 [P] [US5] Design extensible architecture for new IR instruction types
- [ ] T044 [US5] Implement modular translation components that can be extended
- [ ] T045 [US5] Add configuration options for future extension points
- [ ] T046 [US5] Create extensibility tests demonstrating easy addition of new IR constructs
- [ ] T047 [US5] Document extension patterns for future IR evolution

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T048 [P] Add comprehensive error handling with detailed diagnostics throughout translator
- [ ] T049 [P] Implement structured logging with configurable levels (trace/debug/info/warn/error)
- [ ] T050 [P] Add input validation including structural, reference, type, and bounds checking
- [ ] T051 [P] Implement source mapping generation when --emit-mapping flag is enabled
- [ ] T052 [P] Add debug symbol generation (DWARF on Unix, PDB on Windows)
- [ ] T053 [P] Performance optimization to meet <100ms per function target
- [ ] T054 [P] Add comprehensive tests for error scenarios in tests/translator_errors.rs
- [ ] T055 [P] Documentation updates for the translator module
- [ ] T056 [P] Update quickstart.md with usage examples
- [ ] T057 Run performance benchmarks to validate <100ms target in benches/jsavrs_benchmark.rs
- [ ] T058 Run quickstart.md validation to ensure examples work correctly

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
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Builds on US1 components
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Integrates with existing src/asm
- **User Story 5 (P5)**: Can start after Foundational (Phase 2) - May build on other stories but should be independently testable

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
# Launch all parallel tasks for User Story 1 together:
Task: "Implement basic Translator::translate_module function in src/translator/mod.rs"
Task: "Implement FunctionTranslator in src/translator/function_translator.rs"
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
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
   - Developer D: User Story 4
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