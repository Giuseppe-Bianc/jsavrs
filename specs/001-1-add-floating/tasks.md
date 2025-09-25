# Tasks: IEEE 754 Floating-Point Support

**Input**: Design documents from `/specs/001-1-add-floating/`
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
- [x] T001 Create project structure per implementation plan for jsavrs floating-point support
- [x] T002 Initialize Rust project with IEEE 754 compliance dependencies
- [x] T003 [P] Configure linting and formatting tools for floating-point implementation

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [x] T004 [P] Contract test for FloatingPointRegister enum in tests/floating_point_register_contract_test.rs
- [x] T005 [P] Contract test for FloatingPointInstruction enum in tests/floating_point_instruction_contract_test.rs
- [x] T006 [P] Contract test for FloatingPointOperand enum in tests/floating_point_operand_contract_test.rs
- [x] T007 [P] Contract test for IEEE754ExceptionType enum in tests/ieee754_exception_type_contract_test.rs
- [x] T008 [P] Contract test for RoundingMode enum in tests/rounding_mode_contract_test.rs
- [x] T009 [P] Contract test for MXCSRRegister in tests/mxcsr_register_contract_test.rs
- [x] T010 [P] Contract test for ABIConvention enum in tests/abi_convention_contract_test.rs
- [x] T011 [P] Integration test for floating-point instruction generation in tests/floating_point_generation_integration_test.rs
- [x] T012 [P] Integration test for floating-point register validation in tests/floating_point_registers_integration_test.rs
- [x] T013 [P] Integration test for IEEE 754 exception handling in tests/exception_handling_integration_test.rs
- [x] T014 [P] Integration test for ABI compliance in tests/abi_compliance_integration_test.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [x] T015 [P] FloatingPointRegister enum in src/asm/register.rs
- [x] T016 [P] FloatingPointInstruction enum in src/asm/instruction.rs
- [x] T017 [P] FloatingPointOperand enum in src/asm/operand.rs
- [x] T018 [P] IEEE754ExceptionType enum in src/asm/exception.rs
- [x] T019 [P] RoundingMode enum in src/asm/rounding.rs
- [x] T020 [P] MXCSRRegister implementation in src/asm/mxcsr.rs
- [x] T021 [P] ABIConvention enum in src/asm/abi.rs
- [x] T022 [P] Floating-point validation functions in src/asm/validation.rs
- [x] T023 Update Display implementation for FloatingPointRegister in src/asm/register.rs
- [x] T024 Update Display implementation for FloatingPointInstruction in src/asm/instruction.rs
- [x] T025 Update Display implementation for FloatingPointOperand in src/asm/operand.rs
- [x] T026 Update operand handling for floating-point values in src/asm/operand.rs
- [x] T027 Implement floating-point code generation in src/asm/generator.rs
- [x] T028 Implement MXCSR register management in src/asm/mxcsr.rs
- [x] T029 Implement ABI compliance for floating-point parameters in src/asm/abi.rs
- [x] T030 Implement exception handling for floating-point operations in src/asm/exception.rs
- [x] T031 Implement rounding mode control in src/asm/rounding.rs
- [x] T032 Implement support for subnormal numbers with FTZ/DAZ modes in src/asm/subnormal.rs
- [ ] T033 Implement proper handling of signed zero in operations and comparisons in src/asm/signed_zero.rs

## Phase 3.4: Integration
- [ ] T034 Integrate FloatingPointRegister with existing register system in src/asm/register.rs
- [ ] T035 Connect floating-point instructions to code generation in src/asm/generator.rs
- [ ] T036 Connect floating-point validation to instruction processing in src/asm/validation.rs
- [ ] T037 Add floating-point logging in src/asm/logging.rs
- [ ] T038 Update existing instruction handling to maintain backward compatibility in src/asm/instruction.rs

## Phase 3.5: Polish
- [ ] T039 [P] Unit tests for FloatingPointRegister in tests/floating_point_register_unit_test.rs
- [ ] T040 [P] Unit tests for FloatingPointInstruction in tests/floating_point_instruction_unit_test.rs
- [ ] T041 [P] Unit tests for FloatingPointOperand in tests/floating_point_operand_unit_test.rs
- [ ] T042 [P] Unit tests for IEEE754ExceptionType in tests/ieee754_exception_type_unit_test.rs
- [ ] T043 [P] Unit tests for RoundingMode in tests/rounding_mode_unit_test.rs
- [ ] T044 [P] Unit tests for MXCSRRegister in tests/mxcsr_register_unit_test.rs
- [ ] T045 [P] Unit tests for ABIConvention in tests/abi_convention_unit_test.rs
- [ ] T046 Performance tests for floating-point operations (must maintain efficiency)
- [ ] T047 [P] Update docs/api.md with floating-point API documentation
- [ ] T048 [P] Update docs/floating-point-implementation.md with detailed implementation guide
- [ ] T049 Remove code duplication in register and operand handling
- [ ] T050 Run IEEE754 compliance verification tests
- [ ] T051 [P] Implement FMA (Fused Multiply-Add) operations for supported CPU architectures in src/asm/fma.rs
- [ ] T052 [P] Implement compile-time configuration system for IEEE 754 exception handling modes in src/asm/config.rs

## Dependencies
- Tests (T004-T014) before implementation (T015-T033)
- T015 blocks T016, T023
- T016 blocks T024, T027
- T017 blocks T025, T026
- T020 blocks T028, T034
- T021 blocks T029
- T018 blocks T030
- T019 blocks T031
- Implementation before polish (T039-T050)

## Parallel Example
```
# Launch T004-T010 together:
Task: "Contract test for FloatingPointRegister enum in tests/floating_point_register_contract_test.rs"
Task: "Contract test for FloatingPointInstruction enum in tests/floating_point_instruction_contract_test.rs"
Task: "Contract test for FloatingPointOperand enum in tests/floating_point_operand_contract_test.rs"
Task: "Contract test for IEEE754ExceptionType enum in tests/ieee754_exception_type_contract_test.rs"
Task: "Contract test for RoundingMode enum in tests/rounding_mode_contract_test.rs"
Task: "Contract test for MXCSRRegister in tests/mxcsr_register_contract_test.rs"
Task: "Contract test for ABIConvention enum in tests/abi_convention_contract_test.rs"

# Launch T039-T045 together:
Task: "Unit tests for FloatingPointRegister in tests/floating_point_register_unit_test.rs"
Task: "Unit tests for FloatingPointInstruction in tests/floating_point_instruction_unit_test.rs"
Task: "Unit tests for FloatingPointOperand in tests/floating_point_operand_unit_test.rs"
Task: "Unit tests for IEEE754ExceptionType in tests/ieee754_exception_type_unit_test.rs"
Task: "Unit tests for RoundingMode in tests/rounding_mode_unit_test.rs"
Task: "Unit tests for MXCSRRegister in tests/mxcsr_register_unit_test.rs"
Task: "Unit tests for ABIConvention in tests/abi_convention_unit_test.rs"
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