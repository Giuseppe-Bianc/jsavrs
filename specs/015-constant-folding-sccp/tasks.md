# Tasks: Constant Folding and Propagation Optimizer

**Branch**: `015-constant-folding-sccp`  
**Input**: Design documents from `/specs/015-constant-folding-sccp/`  
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Tests**: Integration and unit tests are included as this is a compiler optimization feature requiring comprehensive validation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each optimization capability.

---

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and module structure creation

- [ ] T001 Create module directory structure `src/ir/optimizer/constant_folding/`
- [ ] T002 Create module declaration file `src/ir/optimizer/constant_folding/mod.rs` with public exports
- [ ] T003 [P] Create placeholder files: `optimizer.rs`, `lattice.rs`, `evaluator.rs`, `worklist.rs`, `statistics.rs`, `utils.rs` in `src/ir/optimizer/constant_folding/`
- [ ] T004 [P] Add module reference in `src/ir/optimizer/mod.rs` to expose constant_folding module
- [ ] T005 Create test file structure: `tests/ir_constant_folding_basic_tests.rs`, `tests/ir_constant_folding_propagation_tests.rs`, `tests/ir_constant_folding_sccp_tests.rs`, `tests/ir_constant_folding_snapshot_tests.rs`, `tests/ir_constant_folding_edge_cases_tests.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete

- [ ] T006 Implement `LatticeValue` enum in `src/ir/optimizer/constant_folding/lattice.rs` with variants `Top`, `Constant(IrLiteralValue)`, `Bottom`
- [ ] T007 Implement `meet()` method for `LatticeValue` in `src/ir/optimizer/constant_folding/lattice.rs` with proper lattice semantics
- [ ] T008 [P] Implement `is_constant()` and `as_constant()` helper methods for `LatticeValue` in `src/ir/optimizer/constant_folding/lattice.rs`
- [ ] T009 Implement `FunctionMetrics` struct in `src/ir/optimizer/constant_folding/statistics.rs` with fields for instruction counts, folded operations, propagated values, and resolved branches
- [ ] T010 Implement `AggregateStatistics` struct in `src/ir/optimizer/constant_folding/statistics.rs` to accumulate per-function metrics
- [ ] T011 [P] Implement `ConstantFoldingOptimizer` struct in `src/ir/optimizer/constant_folding/optimizer.rs` with `verbose` and `sccp` configuration fields
- [ ] T012 Implement `ConstantFoldingOptimizer::new()` constructor in `src/ir/optimizer/constant_folding/optimizer.rs`
- [ ] T013 Implement stub `Phase` trait for `ConstantFoldingOptimizer` in `src/ir/optimizer/constant_folding/optimizer.rs` (returns optimizer name, empty `run()` implementation)
- [ ] T014 [P] Create SSA validation helper functions in `src/ir/optimizer/constant_folding/utils.rs` for verifying value references
- [ ] T015 [P] Create memory estimation helper function in `src/ir/optimizer/constant_folding/utils.rs` for lattice size tracking

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Basic Constant Folding (Priority: P1) 🎯 MVP

**Goal**: Enable compile-time evaluation of constant expressions (arithmetic, logical, comparison, bitwise, unary, cast operations)

**Independent Test**: Compile IR with constant expressions like `add i32 2, 3` and verify the instruction is replaced with constant value `5`, reducing instruction count

### Implementation for User Story 1

- [ ] T016 [P] [US1] Implement integer arithmetic folding functions in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_add_int()`, `fold_sub_int()`, `fold_mul_int()`, `fold_div_int()`, `fold_mod_int()` with wrapping semantics
- [ ] T017 [P] [US1] Implement floating-point arithmetic folding functions in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_fadd()`, `fold_fsub()`, `fold_fmul()`, `fold_fdiv()` following IEEE 754 rules
- [ ] T018 [P] [US1] Implement comparison operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_icmp()` for integer comparisons (eq, ne, lt, gt, le, ge)
- [ ] T019 [P] [US1] Implement logical operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_and()`, `fold_or()` for boolean values
- [ ] T020 [P] [US1] Implement bitwise operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_bitwise_and()`, `fold_bitwise_or()`, `fold_bitwise_xor()`, `fold_shl()`, `fold_shr()`
- [ ] T021 [P] [US1] Implement unary operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_neg()`, `fold_not()`
- [ ] T022 [P] [US1] Implement type cast folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_sext()`, `fold_zext()`, `fold_trunc()`, `fold_fptosi()`, `fold_fptoui()`, `fold_sitofp()`, `fold_uitofp()`
- [ ] T023 [US1] Implement main `fold_instruction()` function in `src/ir/optimizer/constant_folding/evaluator.rs` that dispatches to specific folding functions based on instruction type
- [ ] T024 [US1] Implement basic constant folding pass in `src/ir/optimizer/constant_folding/optimizer.rs`: scan all instructions in a function, identify foldable operations, replace with constants
- [ ] T025 [US1] Update `Phase::run()` implementation in `src/ir/optimizer/constant_folding/optimizer.rs` to iterate functions and apply basic constant folding pass
- [ ] T026 [US1] Implement instruction count reporting to stdout in `src/ir/optimizer/constant_folding/optimizer.rs` after optimization completes
- [ ] T027 [US1] Implement verbose statistics output to stderr in `src/ir/optimizer/constant_folding/optimizer.rs` when verbose mode enabled

### Tests for User Story 1

- [ ] T028 [P] [US1] Write unit tests for integer arithmetic folding in `tests/ir_constant_folding_basic_tests.rs`: test addition, subtraction, multiplication, division, modulo
- [ ] T029 [P] [US1] Write unit tests for floating-point arithmetic in `tests/ir_constant_folding_basic_tests.rs`: test IEEE 754 operations including NaN and infinity handling
- [ ] T030 [P] [US1] Write unit tests for comparison operations in `tests/ir_constant_folding_basic_tests.rs`: test all comparison variants (eq, ne, lt, gt, le, ge)
- [ ] T031 [P] [US1] Write unit tests for logical and bitwise operations in `tests/ir_constant_folding_basic_tests.rs`
- [ ] T032 [P] [US1] Write unit tests for unary operations in `tests/ir_constant_folding_basic_tests.rs`: negation and NOT
- [ ] T033 [P] [US1] Write unit tests for type casts in `tests/ir_constant_folding_basic_tests.rs`: widening, narrowing, signed/unsigned conversions
- [ ] T034 [P] [US1] Write edge case tests in `tests/ir_constant_folding_edge_cases_tests.rs`: division by zero, integer overflow wrapping, NaN propagation, signed zero preservation
- [ ] T035 [US1] Write snapshot tests in `tests/ir_constant_folding_snapshot_tests.rs` using `insta` to verify IR transformation correctness for basic folding scenarios

**Checkpoint**: User Story 1 complete - basic constant folding functional and tested independently

---

## Phase 4: User Story 2 - Simple Constant Propagation (Priority: P2)

**Goal**: Track constant values through store/load sequences, replacing redundant loads with constant values

**Independent Test**: Compile IR with constant store followed by loads, verify loads are replaced with direct constant values eliminating memory accesses

### Implementation for User Story 2

- [ ] T036 [US2] Implement escape analysis helper in `src/ir/optimizer/constant_folding/utils.rs` to determine if local variable escapes function scope
- [ ] T037 [US2] Implement constant store tracking in `src/ir/optimizer/constant_folding/optimizer.rs`: build map of allocation → constant value for non-escaping locals
- [ ] T038 [US2] Implement load replacement logic in `src/ir/optimizer/constant_folding/optimizer.rs`: replace load instructions with constants when allocation is known to hold constant
- [ ] T039 [US2] Integrate constant propagation pass into `Phase::run()` in `src/ir/optimizer/constant_folding/optimizer.rs` to run after basic folding
- [ ] T040 [US2] Update `FunctionMetrics` tracking in `src/ir/optimizer/constant_folding/statistics.rs` to count propagated loads
- [ ] T041 [US2] Add validation to ensure SSA form is preserved after load elimination in `src/ir/optimizer/constant_folding/optimizer.rs`

### Tests for User Story 2

- [ ] T042 [P] [US2] Write unit tests for simple store/load propagation in `tests/ir_constant_folding_propagation_tests.rs`: single store, single load case
- [ ] T043 [P] [US2] Write unit tests for multiple loads from same constant in `tests/ir_constant_folding_propagation_tests.rs`
- [ ] T044 [P] [US2] Write unit tests for propagation with subsequent operations in `tests/ir_constant_folding_propagation_tests.rs`: `add %loaded_const, 10` should fold completely
- [ ] T045 [P] [US2] Write tests for non-constant stores in `tests/ir_constant_folding_propagation_tests.rs`: verify loads are preserved when variable has multiple assignments
- [ ] T046 [P] [US2] Write tests for escaping variables in `tests/ir_constant_folding_propagation_tests.rs`: verify conservative behavior when address escapes
- [ ] T047 [US2] Write snapshot tests in `tests/ir_constant_folding_snapshot_tests.rs` for constant propagation scenarios

**Checkpoint**: User Stories 1 AND 2 complete - basic folding and simple propagation both functional independently

---

## Phase 5: User Story 3 - Advanced SCCP Analysis (Priority: P3)

**Goal**: Implement full Sparse Conditional Constant Propagation with control flow analysis to eliminate unreachable code and propagate constants through complex CFG including phi nodes

**Independent Test**: Compile IR with constant conditional branches, verify unreachable blocks removed, phi nodes simplified, and constants propagated through complex control flow

### Implementation for User Story 3

- [ ] T048 [US3] Implement `SCCPResult` struct in `src/ir/optimizer/constant_folding/worklist.rs` with fields for lattice_values, reachable_blocks, executable_edges, foldable_count, resolvable_branches
- [ ] T049 [US3] Implement `SCCPError` enum in `src/ir/optimizer/constant_folding/worklist.rs` with variants for MemoryLimit, InvalidCFG, InvalidSSA
- [ ] T050 [US3] Implement SCCP initialization in `src/ir/optimizer/constant_folding/worklist.rs`: create lattice map, mark entry block as reachable, initialize worklist with entry block instructions
- [ ] T051 [US3] Implement worklist processing loop in `src/ir/optimizer/constant_folding/worklist.rs`: process SSA edges and CFG edges until fixed point reached
- [ ] T052 [US3] Implement phi node merge logic in `src/ir/optimizer/constant_folding/worklist.rs`: merge incoming values only from reachable predecessors using lattice meet operation
- [ ] T053 [US3] Implement conditional branch resolution in `src/ir/optimizer/constant_folding/worklist.rs`: mark successor blocks based on constant condition values
- [ ] T054 [US3] Implement memory limit checking in `src/ir/optimizer/constant_folding/worklist.rs`: track lattice map size, return error if exceeds 100KB
- [ ] T055 [US3] Implement `sccp_analysis()` main function in `src/ir/optimizer/constant_folding/worklist.rs` orchestrating the full algorithm
- [ ] T056 [US3] Implement CFG cleanup pass in `src/ir/optimizer/constant_folding/optimizer.rs`: remove unreachable blocks, simplify phi nodes, remove dead branches
- [ ] T057 [US3] Implement SCCP-based transformation pass in `src/ir/optimizer/constant_folding/optimizer.rs`: apply SCCP results to replace instructions and clean CFG
- [ ] T058 [US3] Integrate SCCP pass into `Phase::run()` in `src/ir/optimizer/constant_folding/optimizer.rs` when `sccp` flag is enabled
- [ ] T059 [US3] Implement fallback logic in `src/ir/optimizer/constant_folding/optimizer.rs`: handle memory limit errors by falling back to basic folding with warning
- [ ] T060 [US3] Update `FunctionMetrics` in `src/ir/optimizer/constant_folding/statistics.rs` to track branches resolved and blocks removed
- [ ] T061 [US3] Ensure CFG cleanup pass always runs after SCCP analysis regardless of foldable operation count in `src/ir/optimizer/constant_folding/optimizer.rs`

### Tests for User Story 3

- [ ] T062 [P] [US3] Write unit tests for constant branch resolution in `tests/ir_constant_folding_sccp_tests.rs`: `br i1 true, %then, %else` should eliminate else block
- [ ] T063 [P] [US3] Write unit tests for phi node simplification in `tests/ir_constant_folding_sccp_tests.rs`: `phi [5, %b1], [5, %b2]` should become constant `5`
- [ ] T064 [P] [US3] Write unit tests for phi with unreachable predecessors in `tests/ir_constant_folding_sccp_tests.rs`: only reachable incoming values should participate in merge
- [ ] T065 [P] [US3] Write integration tests for nested conditionals in `tests/ir_constant_folding_sccp_tests.rs`: multiple levels of constant branches
- [ ] T066 [P] [US3] Write integration tests for loops with constant iteration counts in `tests/ir_constant_folding_sccp_tests.rs`
- [ ] T067 [P] [US3] Write tests for SCCP memory limit fallback in `tests/ir_constant_folding_sccp_tests.rs`: verify graceful degradation for large functions
- [ ] T068 [P] [US3] Write tests for CFG cleanup pass execution in `tests/ir_constant_folding_sccp_tests.rs`: verify cleanup runs even when no constants found
- [ ] T069 [US3] Write snapshot tests in `tests/ir_constant_folding_snapshot_tests.rs` for complex SCCP scenarios with before/after IR comparison

**Checkpoint**: All user stories complete - full constant folding, propagation, and SCCP functional independently

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Error handling, documentation, and integration improvements

- [ ] T070 [P] Implement malformed IR handling in `src/ir/optimizer/constant_folding/optimizer.rs`: emit warnings for invalid SSA references, skip optimization, preserve original instruction
- [ ] T071 [P] Implement diagnostic message formatting in `src/ir/optimizer/constant_folding/statistics.rs`: structured stderr output for verbose mode
- [ ] T072 [P] Add comprehensive rustdoc comments to all public APIs in `src/ir/optimizer/constant_folding/mod.rs`, `optimizer.rs`, `lattice.rs`, `evaluator.rs`, `worklist.rs`
- [ ] T073 [P] Add module-level documentation in `src/ir/optimizer/constant_folding/mod.rs` with usage examples and architectural overview
- [ ] T074 [P] Document panic conditions, safety requirements, and error cases in all public functions
- [ ] T075 [P] Add integration tests for Phase trait compliance in `tests/ir_constant_folding_basic_tests.rs`: verify `name()` returns correct string, `run()` preserves module validity
- [ ] T076 [P] Add performance benchmarks in `benches/jsavrs_benchmark.rs` for 1000+ instruction functions, verify <1 second target
- [ ] T077 [P] Write quickstart validation tests in `tests/ir_constant_folding_basic_tests.rs`: verify examples from `quickstart.md` compile and run correctly
- [ ] T078 Code cleanup: ensure consistent error handling patterns across all modules
- [ ] T079 Code cleanup: run `cargo fmt` and `cargo clippy` to ensure idiomatic Rust code
- [ ] T080 Final validation: run full test suite with `cargo test` and verify 100% pass rate

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - **BLOCKS all user stories**
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on Foundational completion (can run in parallel with US1 if staffed)
- **User Story 3 (Phase 5)**: Depends on Foundational completion (can run in parallel with US1/US2 if staffed)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Builds on US1 evaluator but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Uses US1 evaluator but independently testable

### Within Each User Story

**User Story 1**:
- All evaluator folding functions (T016-T022) can run in parallel
- `fold_instruction()` (T023) depends on folding functions completing
- Optimizer integration (T024-T027) depends on `fold_instruction()`
- All test tasks (T028-T035) can run in parallel after implementation tasks complete

**User Story 2**:
- Escape analysis (T036) and store tracking (T037) can run in parallel
- Load replacement (T038) depends on T037
- Integration (T039-T041) depends on load replacement
- All test tasks (T042-T047) can run in parallel after implementation tasks complete

**User Story 3**:
- SCCP data structures (T048-T049) can be created in parallel
- Initialization (T050), worklist processing (T051), phi merge (T052), branch resolution (T053), memory limit (T054) can be developed in parallel
- `sccp_analysis()` (T055) depends on all prior SCCP tasks
- CFG cleanup (T056) and SCCP transformation (T057) depend on T055
- Integration (T058-T061) depends on transformation pass
- All test tasks (T062-T069) can run in parallel after implementation tasks complete

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel
- Once Foundational phase completes, all three user stories can start in parallel (if team capacity allows)
- Within each user story, all tasks marked [P] can run in parallel
- All test tasks within a user story can run in parallel once implementation completes

---

## Parallel Example: User Story 1

```bash
# Launch all evaluator folding functions together:
Task T016: "Implement integer arithmetic folding functions in evaluator.rs"
Task T017: "Implement floating-point arithmetic folding functions in evaluator.rs"
Task T018: "Implement comparison operation folding in evaluator.rs"
Task T019: "Implement logical operation folding in evaluator.rs"
Task T020: "Implement bitwise operation folding in evaluator.rs"
Task T021: "Implement unary operation folding in evaluator.rs"
Task T022: "Implement type cast folding in evaluator.rs"

# Launch all test tasks together after implementation:
Task T028: "Write unit tests for integer arithmetic folding"
Task T029: "Write unit tests for floating-point arithmetic"
Task T030: "Write unit tests for comparison operations"
Task T031: "Write unit tests for logical and bitwise operations"
Task T032: "Write unit tests for unary operations"
Task T033: "Write unit tests for type casts"
Task T034: "Write edge case tests"
Task T035: "Write snapshot tests"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Basic Constant Folding)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deliver MVP with basic constant folding capability

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deliver MVP (basic constant folding)
3. Add User Story 2 → Test independently → Deliver enhanced version (with propagation)
4. Add User Story 3 → Test independently → Deliver full version (with SCCP)
5. Complete Polish phase → Final production-ready release

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Basic Constant Folding)
   - Developer B: User Story 2 (Constant Propagation)
   - Developer C: User Story 3 (SCCP Analysis)
3. Stories complete and integrate independently
4. Team collaborates on Polish phase

---

## Notes

- [P] tasks = different files or independent modules, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Performance target: <1 second for 1000+ instruction functions
- Memory limit: 100KB lattice storage per function (with fallback)
- All transformations must preserve SSA form and CFG validity
- Conservative fallback on malformed IR or escape analysis uncertainty

---

## Summary

- **Total Tasks**: 80
- **User Story 1 (P1)**: 20 tasks (12 implementation + 8 tests)
- **User Story 2 (P2)**: 12 tasks (6 implementation + 6 tests)
- **User Story 3 (P3)**: 22 tasks (14 implementation + 8 tests)
- **Setup**: 5 tasks
- **Foundational**: 10 tasks
- **Polish**: 11 tasks
- **Parallel Opportunities**: 52 tasks marked [P] can run in parallel within their phase/story
- **Independent Test Criteria**: Each user story has clear acceptance scenarios and can be validated independently
- **Suggested MVP Scope**: Setup + Foundational + User Story 1 (35 tasks)
