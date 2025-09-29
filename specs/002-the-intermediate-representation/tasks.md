# Tasks: IR Type Promotion System

**Input**: Design documents from `/specs/002-the-intermediate-representation/`
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
- [ ] T002 Initialize Rust project with existing dependencies (logos, uuid, petgraph, iced-x86)
- [ ] T003 [P] Configure linting and formatting tools (rustfmt, clippy)

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] Contract test analyze_binary_promotion in tests/type_promotion_engine.rs
- [ ] T005 [P] Contract test insert_promotion_casts in tests/type_promotion_engine.rs
- [ ] T006 [P] Contract test get_promotion_rule in tests/promotion_matrix.rs
- [ ] T007 [P] Contract test compute_common_type in tests/promotion_matrix.rs
- [ ] T008 [P] Contract test new in tests/type_promotion.rs
- [ ] T009 [P] Contract test generate_cast_instruction in tests/type_promotion.rs
- [ ] T010 [P] Contract test format_for_user in tests/promotion_warning.rs
- [ ] T011 [P] Contract test severity_level in tests/promotion_warning.rs
- [ ] T012 [P] Integration test i32 + f32 promotion in tests/binary_promotion.rs
- [ ] T013 [P] Integration test signed + unsigned promotion in tests/binary_promotion.rs
- [ ] T014 [P] Integration test complex expression promotion in tests/binary_promotion.rs
- [ ] T015 [P] Integration test special float value handling in tests/binary_promotion.rs
- [ ] T016 [P] Integration test invalid promotion error in tests/binary_promotion.rs
- [ ] T017 [P] Quickstart scenario test: Integer + Float Promotion in tests/scenario1.rs
- [ ] T018 [P] Quickstart scenario test: Signed/Unsigned Integer Mixing in tests/scenario2.rs
- [ ] T019 [P] Quickstart scenario test: Complex Expression with Multiple Promotions in tests/scenario3.rs
- [ ] T020 [P] Quickstart scenario test: Special Float Values in tests/scenario4.rs
- [ ] T021 [P] Snapshot test for i32 + f32 IR generation in tests/promotion_snapshot.rs
- [ ] T022 [P] Snapshot test for i32 + u32 → i64 IR generation in tests/promotion_snapshot.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T023 [P] TypePromotion struct implementation in src/ir/type_promotion.rs
- [ ] T024 [P] PromotionMatrix struct implementation in src/ir/type_promotion.rs
- [ ] T025 [P] PromotionRule enum implementation in src/ir/type_promotion.rs
- [ ] T026 [P] TypeGroup enum implementation in src/ir/type_promotion.rs
- [ ] T027 [P] BinaryOperationPromotion struct implementation in src/ir/generator.rs
- [ ] T028 [P] PromotionResult struct implementation in src/ir/type_promotion.rs
- [ ] T029 [P] PromotionWarning enum implementation in src/ir/type_promotion.rs
- [ ] T030 [P] OverflowBehavior enum implementation in src/ir/type_promotion.rs
- [ ] T031 [P] PrecisionLossEstimate enum implementation in src/ir/type_promotion.rs
- [ ] T032 [P] analyze_binary_promotion function in src/ir/type_promotion_engine.rs
- [ ] T033 [P] insert_promotion_casts function in src/ir/type_promotion_engine.rs
- [ ] T034 [P] get_promotion_rule function in src/ir/type_promotion_engine.rs
- [ ] T035 [P] compute_common_type function in src/ir/type_promotion_engine.rs
- [ ] T036 Enhance generate_binary in src/ir/generator.rs to use promotion analysis
- [ ] T037 Add insert_promotion_cast helper in src/ir/generator.rs
- [ ] T038 Update error handling for type promotion failures in src/ir/generator.rs
- [ ] T039 Implement cast insertion logic in IR generation in src/ir/generator.rs
- [ ] T040 Add promotion warning system integration in src/ir/generator.rs
- [ ] T041 Implement special float value handling (NaN, infinity) in src/ir/type_promotion.rs
- [ ] T042 Add overflow detection and configurable behavior in src/ir/type_promotion.rs
- [ ] T043 Handle precision loss warnings and user feedback in src/ir/type_promotion.rs
- [ ] T044 Implement signed/unsigned interaction edge cases in src/ir/type_promotion.rs

## Phase 3.4: Integration
- [ ] T045 Verify assembly generation compatibility with promoted IR in src/codegen/
- [ ] T046 Update code generation for new cast instruction patterns in src/codegen/
- [ ] T047 Cross-platform validation for promotion behavior in src/ir/type_promotion.rs
- [ ] T048 Memory layout consistency checks in src/ir/type_promotion.rs

## Phase 3.5: Polish
- [ ] T049 [P] Update module documentation for new promotion system in src/ir/type_promotion.rs
- [ ] T050 [P] Create migration guide for breaking changes in docs/migration.md
- [ ] T051 [P] Add user-facing documentation for type promotion rules in docs/type_promotion.md
- [ ] T052 [P] Performance benchmark suite for promotion overhead in benches/type_promotion_benchmark.rs
- [ ] T053 Run existing test suite to ensure no regressions
- [ ] T054 Update README with type promotion examples

## Dependencies
- Tests (T004-T022) before implementation (T023-T044)
- T023-T031 blocks T032-T035
- T032-T035 blocks T036
- T036 blocks T037-T040
- T036-T040 blocks T041-T044
- Implementation before polish (T049-T054)

## Parallel Example
```
# Launch T004-T011 together:
Task: "Contract test analyze_binary_promotion in tests/type_promotion_engine.rs"
Task: "Contract test insert_promotion_casts in tests/type_promotion_engine.rs"
Task: "Contract test get_promotion_rule in tests/promotion_matrix.rs"
Task: "Contract test compute_common_type in tests/promotion_matrix.rs"
Task: "Contract test new in tests/type_promotion.rs"
Task: "Contract test generate_cast_instruction in tests/type_promotion.rs"
Task: "Contract test format_for_user in tests/promotion_warning.rs"
Task: "Contract test severity_level in tests/promotion_warning.rs"

# Launch T023-T031 together:
Task: "TypePromotion struct implementation in src/ir/type_promotion.rs"
Task: "PromotionMatrix struct implementation in src/ir/type_promotion.rs"
Task: "PromotionRule enum implementation in src/ir/type_promotion.rs"
Task: "TypeGroup enum implementation in src/ir/type_promotion.rs"
Task: "BinaryOperationPromotion struct implementation in src/ir/generator.rs"
Task: "PromotionResult struct implementation in src/ir/type_promotion.rs"
Task: "PromotionWarning enum implementation in src/ir/type_promotion.rs"
Task: "OverflowBehavior enum implementation in src/ir/type_promotion.rs"
Task: "PrecisionLossEstimate enum implementation in src/ir/type_promotion.rs"
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