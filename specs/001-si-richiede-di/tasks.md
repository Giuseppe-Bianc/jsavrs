# Tasks: ASM Generation Components Improvement

**Input**: Design documents from `/specs/001-si-richiede-di/`
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
- [ ] T001 Create project structure per implementation plan
- [ ] T002 Initialize Rust project with NASM dependencies
- [ ] T003 [P] Configure linting and formatting tools (cargo fmt, cargo clippy)

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] Contract test for asm-generator in tests/test_asm_generator.rs
- [ ] T005 [P] Contract test for instruction in tests/test_instruction.rs
- [ ] T006 [P] Contract test for operand in tests/test_operand.rs
- [ ] T007 [P] Contract test for register in tests/test_register.rs
- [ ] T008 [P] Contract test for target-os in tests/test_target_os.rs
- [ ] T009 [P] Integration test for generate_simple_function in tests/integration/test_asm_generation.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T010 [P] ASM Generator struct in src/asm/generator.rs
- [ ] T011 [P] Instruction enum in src/asm/instruction.rs
- [ ] T012 [P] Operand enum in src/asm/operand.rs
- [ ] T013 [P] Register enum in src/asm/register.rs
- [ ] T014 [P] TargetOS enum in src/asm/target_os.rs
- [ ] T015 Implement ASM Generator new() method in src/asm/generator.rs
- [ ] T016 Implement ASM Generator add_section() method in src/asm/generator.rs
- [ ] T017 Implement ASM Generator switch_section() method in src/asm/generator.rs
- [ ] T018 Implement ASM Generator add_instruction() method in src/asm/generator.rs
- [ ] T019 Implement ASM Generator add_label() method in src/asm/generator.rs
- [ ] T020 Implement ASM Generator generate() method in src/asm/generator.rs
- [ ] T021 Implement Instruction new() method in src/asm/instruction.rs
- [ ] T022 Implement Instruction add_operand() method in src/asm/instruction.rs
- [ ] T023 Implement Instruction format() method in src/asm/instruction.rs
- [ ] T024 Implement Register new() method in src/asm/register.rs
- [ ] T025 Implement Register get_alias() method in src/asm/register.rs
- [ ] T026 Implement Register format() method in src/asm/register.rs
- [ ] T027 Implement Operand format() method in src/asm/operand.rs
- [ ] T028 Implement TargetOS get_calling_convention() method in src/asm/target_os.rs
- [ ] T029 Implement TargetOS get_system_calls() method in src/asm/target_os.rs

## Phase 3.4: Integration
- [ ] T030 Connect ASM Generator to intermediate representations
- [ ] T031 Add OS-specific code generation based on TargetOS
- [ ] T032 Add validation checks for instruction operands
- [ ] T033 Add proper error handling for invalid instructions

## Phase 3.5: Polish
- [ ] T034 [P] Unit tests for ASM Generator in tests/test_asm_generator.rs
- [ ] T035 [P] Unit tests for Instruction in tests/test_instruction.rs
- [ ] T036 [P] Unit tests for Operand in tests/test_operand.rs
- [ ] T037 [P] Unit tests for Register in tests/test_register.rs
- [ ] T038 [P] Unit tests for TargetOS in tests/test_target_os.rs
- [ ] T039 Performance tests for ASM generation (should complete in <200ms)
- [ ] T040 [P] Update documentation/api.md with ASM Generator usage
- [ ] T041 [P] Update documentation for Instruction module
- [ ] T042 [P] Update documentation for Operand module
- [ ] T043 [P] Update documentation for Register module
- [ ] T044 [P] Update documentation for TargetOS module
- [ ] T045 Remove code duplication identified in research.md
- [ ] T046 Run manual-testing.md validation

## Dependencies
- Tests (T004-T009) before implementation (T010-T033)
- T010 blocks T015-T020
- T011 blocks T021-T023
- T012 blocks T027
- T013 blocks T024-T026
- T014 blocks T028-T029
- T010, T011, T012, T013, T014 blocks T030
- Implementation before polish (T034-T046)

## Parallel Example
```
# Launch T004-T009 together:
Task: "Contract test for asm-generator in tests/test_asm_generator.rs"
Task: "Contract test for instruction in tests/test_instruction.rs"
Task: "Contract test for operand in tests/test_operand.rs"
Task: "Contract test for register in tests/test_register.rs"
Task: "Contract test for target-os in tests/test_target_os.rs"
Task: "Integration test for generate_simple_function in tests/integration/test_asm_generation.rs"
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