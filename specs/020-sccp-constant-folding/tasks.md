# Tasks: Constant Folding Optimizer with Sparse Conditional Constant Propagation

**Branch**: `020-sccp-constant-folding`  
**Input**: Design documents from `/specs/020-sccp-constant-folding/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Feature**: Implementation of Wegman-Zadeck Sparse Conditional Constant Propagation algorithm for the jsavrs compiler's intermediate representation.

## Format: `- [ ] [ID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label (US1, US2, US3, US4) - only for user story phases
- File paths are absolute from repository root

## Implementation Strategy

**MVP Scope**: User Story 1 (Basic Constant Propagation) - delivers core SCCP functionality

**Incremental Delivery**: Each user story adds capabilities:
- US1: Constant propagation and folding
- US2: Control flow analysis and branch resolution  
- US3: Phi node simplification
- US4: Type-safe evaluation across all data types

**Parallel Opportunities**: Marked with [P] - independent modules can be developed simultaneously

---

## Phase 1: Setup (Project Structure)

**Purpose**: Establish module structure and foundational types

- [X] T001 Create module structure `src/ir/optimizer/constant_folding/` with mod.rs
- [X] T002 Update `src/ir/optimizer/mod.rs` to include `pub mod constant_folding;`
- [X] T003 [P] Create placeholder files: lattice.rs, evaluator.rs, propagator.rs, rewriter.rs, optimizer.rs

**Checkpoint**: Module structure ready for implementation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: These tasks MUST be complete before any user story implementation

- [X] T004 Implement `LatticeValue` enum (Bottom, Constant, Top) in src/ir/optimizer/constant_folding/lattice.rs
- [X] T005 Implement `ConstantValue` enum with all IR types in src/ir/optimizer/constant_folding/lattice.rs
- [X] T006 Implement `LatticeValue::meet()` operation in src/ir/optimizer/constant_folding/lattice.rs
- [X] T007 [P] Implement helper methods for LatticeValue (is_constant, as_constant, is_bottom, is_top) in src/ir/optimizer/constant_folding/lattice.rs
- [X] T008 [P] Implement `ConstantValue` helper methods (get_type, types_match, as_bool) in src/ir/optimizer/constant_folding/lattice.rs
- [X] T009 Implement `LatticeState` struct with HashMap storage in src/ir/optimizer/constant_folding/propagator.rs
- [X] T010 [P] Implement `LatticeState` methods (get, update, initialize) in src/ir/optimizer/constant_folding/propagator.rs
- [X] T011 Implement `CFGEdge` struct in src/ir/optimizer/constant_folding/propagator.rs
- [X] T012 Implement `ExecutableEdgeSet` struct with HashSet storage in src/ir/optimizer/constant_folding/propagator.rs
- [X] T013 [P] Implement `ExecutableEdgeSet` methods (mark_executable, is_executable, has_executable_predecessor, executable_predecessors) in src/ir/optimizer/constant_folding/propagator.rs
- [X] T014 Implement generic `Worklist<T>` with VecDeque and deduplication in src/ir/optimizer/constant_folding/propagator.rs
- [X] T015 [P] Implement `Worklist<T>` methods (push, pop, is_empty, len) in src/ir/optimizer/constant_folding/propagator.rs
- [X] T016 Define `SCCPConfig` struct (verbose, max_iterations) in src/ir/optimizer/constant_folding/optimizer.rs
- [X] T017 [P] Define `OptimizationStats` struct (constants_propagated, branches_resolved, phi_nodes_simplified, blocks_marked_unreachable, iterations) in src/ir/optimizer/constant_folding/optimizer.rs
- [X] T018 Define `SCCPError` enum with thiserror in src/ir/optimizer/constant_folding/propagator.rs
- [X] T019 [P] Define `RewriteError` enum with thiserror in src/ir/optimizer/constant_folding/rewriter.rs

**Checkpoint**: Foundation complete - user story implementation can proceed in parallel

---

## Phase 3: User Story 1 - Basic Constant Propagation and Folding (Priority: P1) 🎯 MVP

**Goal**: Implement core SCCP algorithm for constant propagation through SSA values and basic arithmetic evaluation

**Independent Test**: Compile function with constant assignments and arithmetic (e.g., `x = 5; y = 10; z = x + y`) and verify optimizer replaces operations with constant results

### Implementation for User Story 1

- [X] T020 [P] [US1] Implement `ConstantEvaluator` struct in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T021 [US1] Implement I32 binary operations (Add, Sub, Mul, Div, Mod) in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T022 [P] [US1] Implement I32 unary operations (Neg) in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T023 [P] [US1] Implement overflow detection using checked_* methods in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T024 [US1] Implement division by zero detection with warning emission in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T025 [US1] Implement `SCCPropagator` struct with all fields in src/ir/optimizer/constant_folding/propagator.rs
- [X] T026 [US1] Implement `SCCPropagator::new_for_function()` with capacity preallocation in src/ir/optimizer/constant_folding/propagator.rs
- [X] T027 [US1] Implement initialization phase (parameters→Top, locals→Bottom, entry edges) in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T028 [US1] Implement main propagation loop (CFG + SSA worklist processing) in SCCPropagator::propagate() in src/ir/optimizer/constant_folding/propagator.rs
- [X] T029 [US1] Implement `visit_instruction()` for binary operations in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T030 [US1] Implement lattice value update and SSA worklist propagation in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T031 [US1] Implement iteration count tracking and max iteration limit in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T032 [US1] Implement `IRRewriter` struct in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T033 [US1] Implement `IRRewriter::new()` constructor in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T034 [US1] Implement constant instruction replacement in IRRewriter::rewrite_instruction() in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T035 [US1] Implement statistics tracking in IRRewriter in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T036 [US1] Implement `ConstantFoldingOptimizer` struct in src/ir/optimizer/constant_folding/optimizer.rs
- [X] T037 [US1] Implement Phase trait for ConstantFoldingOptimizer in src/ir/optimizer/constant_folding/optimizer.rs
- [X] T038 [US1] Implement `ConstantFoldingOptimizer::optimize_function()` orchestration in src/ir/optimizer/constant_folding/optimizer.rs

### Tests for User Story 1

- [X] T039 [P] [US1] Unit tests for LatticeValue meet operation in tests/ir_sccp_lattice_tests.rs
- [X] T040 [P] [US1] Unit tests for ConstantValue type queries in tests/ir_sccp_lattice_tests.rs
- [X] T041 [P] [US1] Unit tests for I32 arithmetic evaluation in tests/ir_sccp_evaluator_tests.rs
- [X] T042 [P] [US1] Unit tests for overflow handling in tests/ir_sccp_evaluator_tests.rs
- [X] T043 [P] [US1] Unit tests for division by zero handling in tests/ir_sccp_evaluator_tests.rs
- [X] T044 [P] [US1] Unit tests for LatticeState operations in tests/ir_sccp_propagator_tests.rs
- [X] T045 [P] [US1] Unit tests for Worklist operations in tests/ir_sccp_propagator_tests.rs
- [X] T046 [US1] Integration test for simple constant propagation (x=5; y=10; z=x+y) in tests/ir_sccp_integration_tests.rs
- [X] T047 [US1] Integration test for chained constant expressions in tests/ir_sccp_integration_tests.rs
- [X] T048 [P] [US1] Snapshot test for constant propagation IR transformation in tests/ir_sccp_snapshot_tests.rs

**Checkpoint**: ✅ User Story 1 complete - basic constant propagation functional and independently testable

---

## Phase 4: User Story 2 - Conditional Branch Resolution (Priority: P2)

**Goal**: Extend SCCP to analyze control flow, resolve constant branch conditions, and mark unreachable code paths

**Independent Test**: Compile function with constant condition (e.g., `if (true) { return 1; } else { return 2; }`) and verify conditional branch converted to unconditional jump with unreachable path marked

### Implementation for User Story 2

- [X] T049 [P] [US2] Implement Boolean constant evaluation (And, Or, Not) in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T050 [P] [US2] Implement comparison operations for I32 (Eq, Lt, Gt, Le, Ge, Ne) in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T051 [US2] Implement `visit_cfg_edge()` in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T052 [US2] Implement `visit_terminator()` for Branch in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T053 [US2] Implement `visit_terminator()` for ConditionalBranch with constant condition handling in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T054 [US2] Implement CFG edge marking (executable vs unreachable) in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T055 [US2] Implement unreachable block marking in IRRewriter::rewrite_block() in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T056 [US2] Implement conditional branch to unconditional branch conversion in IRRewriter::rewrite_terminator() in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T057 [US2] Implement Switch statement constant selector evaluation in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs
- [X] T058 [US2] Implement Switch statement edge marking (only matching case executable) in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs

### Tests for User Story 2

- [X] T059 [P] [US2] Unit tests for Boolean operations in tests/ir_sccp_boolean_tests.rs (33 tests)
- [X] T060 [P] [US2] Unit tests for comparison operations in tests/ir_sccp_boolean_tests.rs (included in 33 tests)
- [X] T061 [P] [US2] Unit tests for ExecutableEdgeSet operations in tests/ir_sccp_edge_tests.rs (21 tests)
- [X] T062 [US2] Integration test for constant true branch resolution in tests/ir_sccp_integration_tests.rs
- [X] T063 [US2] Integration test for constant false branch resolution in tests/ir_sccp_integration_tests.rs
- [X] T064 [US2] Integration test for switch statement with constant selector in tests/ir_sccp_integration_tests.rs
- [X] T065 [US2] Integration test for nested conditional branches in tests/ir_sccp_integration_tests.rs
- [X] T066 [P] [US2] Snapshot test for branch resolution IR transformation in tests/ir_sccp_snapshot_tests.rs
- [X] T067 [P] [US2] Snapshot test for unreachable code marking in tests/ir_sccp_snapshot_tests.rs

**Checkpoint**: ✅ User Story 2 complete - control flow analysis and branch resolution fully functional (T049-T067 complete)

---

## Phase 5: User Story 3 - Phi Node Simplification (Priority: P3)

**Goal**: Simplify phi nodes when incoming edges are unreachable or all executable edges carry same constant value

**Independent Test**: Compile function with phi node merging values from paths where some are unreachable, verify phi simplified to only executable edges or replaced with constant

### Implementation for User Story 3

- [X] T068 [US3] Implement `visit_phi()` in SCCPropagator with executable edge filtering in src/ir/optimizer/constant_folding/propagator.rs (via eval_phi_node method)
- [X] T069 [US3] Implement phi node lattice value computation (meet of executable predecessors) in SCCPropagator in src/ir/optimizer/constant_folding/propagator.rs (via eval_phi_node method)
- [X] T070 [US3] Implement phi node simplification in IRRewriter::rewrite_phi() in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T071 [US3] Implement phi node constant replacement when all executable values match in IRRewriter in src/ir/optimizer/constant_folding/rewriter.rs (integrated into rewrite_phi)
- [X] T072 [US3] Implement phi node preservation for mixed constant/non-constant values in IRRewriter in src/ir/optimizer/constant_folding/rewriter.rs (integrated into rewrite_phi)

### Tests for User Story 3

- [X] T073 [P] [US3] Unit tests for phi node evaluation with executable edges in tests/ir_sccp_propagator_tests.rs
- [X] T074 [US3] Integration test for phi with unreachable predecessors in tests/ir_sccp_integration_tests.rs
- [X] T075 [US3] Integration test for phi with all same constant values in tests/ir_sccp_integration_tests.rs
- [X] T076 [US3] Integration test for phi in unreachable block in tests/ir_sccp_integration_tests.rs
- [X] T077 [US3] Integration test for phi with mixed values (constant + non-constant) in tests/ir_sccp_integration_tests.rs
- [X] T078 [P] [US3] Snapshot test for phi node simplification in tests/ir_sccp_snapshot_tests.rs

**Checkpoint**: ✅ User Story 3 complete - phi node simplification fully functional and tested (T068-T078 complete)

---

## Phase 6: User Story 4 - Type-Safe Evaluation Across Data Types (Priority: P2)

**Goal**: Implement correct constant evaluation for all IR types (signed/unsigned integers, floats, bool, char) with proper edge case handling

**Independent Test**: Create test functions for each type with type-specific constant expressions and verify evaluation respects overflow, precision, and special values

### Implementation for User Story 4

- [X] T079 [P] [US4] Implement I8 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T080 [P] [US4] Implement I16 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T081 [P] [US4] Implement I64 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T082 [P] [US4] Implement U8 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T083 [P] [US4] Implement U16 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T084 [P] [US4] Implement U32 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T085 [P] [US4] Implement U64 arithmetic operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T086 [P] [US4] Implement F32 arithmetic operations with IEEE 754 semantics in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T087 [P] [US4] Implement F64 arithmetic operations with IEEE 754 semantics in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T088 [P] [US4] Implement NaN propagation for floating-point in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T089 [P] [US4] Implement Infinity handling for floating-point in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T090 [P] [US4] Implement signed zero handling for floating-point in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T091 [P] [US4] Implement Char operations in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs
- [X] T092 [P] [US4] Implement bitwise operations (And, Or, Xor, Not, Shl, Shr) for all integer types in ConstantEvaluator in src/ir/optimizer/constant_folding/evaluator.rs

### Tests for User Story 4

- [X] T093 [P] [US4] Unit tests for I8 overflow handling in tests/ir_sccp_evaluator_tests.rs
- [X] T094 [P] [US4] Unit tests for I16 overflow handling in tests/ir_sccp_evaluator_tests.rs
- [X] T095 [P] [US4] Unit tests for I64 overflow handling in tests/ir_sccp_evaluator_tests.rs
- [X] T096 [P] [US4] Unit tests for U8/U16/U32/U64 overflow handling in tests/ir_sccp_evaluator_tests.rs
- [X] T097 [P] [US4] Unit tests for F32 NaN propagation in tests/ir_sccp_evaluator_tests.rs
- [X] T098 [P] [US4] Unit tests for F64 NaN propagation in tests/ir_sccp_evaluator_tests.rs
- [X] T099 [P] [US4] Unit tests for floating-point Infinity handling in tests/ir_sccp_evaluator_tests.rs
- [X] T100 [P] [US4] Unit tests for floating-point signed zero in tests/ir_sccp_evaluator_tests.rs
- [X] T101 [P] [US4] Unit tests for Char Unicode validity in tests/ir_sccp_evaluator_tests.rs
- [X] T102 [P] [US4] Unit tests for bitwise operations in tests/ir_sccp_evaluator_tests.rs
- [X] T103 [US4] Integration test for mixed type constant expressions in tests/ir_sccp_integration_tests.rs
- [X] T104 [P] [US4] Snapshot tests for all type evaluations in tests/ir_sccp_snapshot_tests.rs

**Checkpoint**: ✅ User Story 4 complete - type-safe evaluation for all IR types functional and fully tested (T079-T104 complete)

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Finalization, optimization, documentation, and production readiness

- [X] T105 [P] Implement verbose diagnostic output (lattice transitions, worklist ops, reachability) in src/ir/optimizer/constant_folding/propagator.rs
- [X] T106 [P] Implement OptimizationStats Display trait in src/ir/optimizer/constant_folding/optimizer.rs
- [X] T107 [P] Add comprehensive rustdoc comments for all public APIs in src/ir/optimizer/constant_folding/
- [X] T108 [P] Add inline documentation explaining lattice theory and algorithm invariants in src/ir/optimizer/constant_folding/
- [X] T109 Verify SSA form preservation after all transformations (add debug assertions) in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T110 [P] Verify dominance relations after rewriting (debug assertions) in src/ir/optimizer/constant_folding/rewriter.rs
- [X] T111 Implement SCCP + DCE integration example in examples/ or documentation
- [X] T112 [P] Create criterion benchmarks for convergence speed in benches/sccp_benchmark.rs
- [X] T113 [P] Create criterion benchmarks for different function sizes (100, 1K, 10K instructions) in benches/sccp_benchmark.rs
- [X] T114 [P] Create benchmarks measuring iterations to convergence in benches/sccp_benchmark.rs
- [X] T115 Run cargo clippy and fix all warnings in src/ir/optimizer/constant_folding/
- [X] T116 Run cargo fmt on all files in src/ir/optimizer/constant_folding/
- [X] T117 [P] Verify all tests pass with cargo test
- [X] T118 [P] Run benchmarks to validate performance requirements (SC-003: ≤3 iterations for 95% of functions, SC-004: <1s for 10K instructions)
- [X] T119 Update QWEN.md with SCCP optimizer architecture documentation
- [ ] T120 Update README.md with optimization pipeline documentation including SCCP

**Final Checkpoint**: Feature complete and production-ready

---

## Dependencies & Execution Order

### Critical Path (Must Execute in Order)

1. **Phase 1 (Setup)**: T001 → T002 → T003
2. **Phase 2 (Foundation)**: T004-T019 (blocking for all user stories)
3. **User Stories (Can be parallel)**: 
   - US1 (T020-T048) - MVP
   - US2 (T049-T067) - Depends on US1 core propagator
   - US3 (T068-T078) - Depends on US2 phi handling
   - US4 (T079-T104) - Can be parallel with US2/US3 (just extends evaluator)
4. **Phase 7 (Polish)**: T105-T120 (after all user stories)

### Parallel Execution Opportunities

**Phase 2 Foundational** (after T004-T008 lattice basics):
- Group A: T009-T010 (LatticeState)
- Group B: T011-T013 (ExecutableEdgeSet)  
- Group C: T014-T015 (Worklist)
- Group D: T016-T019 (Config/Stats/Errors)

**User Story 1 Implementation**:
- Group A: T020-T024 (ConstantEvaluator)
- Group B: T025-T031 (SCCPropagator) - depends on Group A
- Group C: T032-T035 (IRRewriter)
- Group D: T036-T038 (Optimizer) - depends on B & C

**User Story 1 Tests**:
- T039-T048 can all run in parallel (different test files)

**User Story 4 Implementation**:
- T079-T092 are all parallel (different type implementations)
- T093-T104 tests all parallel

**Phase 7 Polish**:
- T105-T110 (code improvements) parallel with T112-T114 (benchmarks)
- T115-T118 (verification) sequential
- T119-T120 (documentation) parallel

### Example Parallel Execution (User Story 1)

**Wave 1** (after Foundation complete):
```
Developer A: T020-T024 (ConstantEvaluator)
Developer B: T039-T043 (Lattice/Evaluator tests)
Developer C: T032-T035 (IRRewriter)
```

**Wave 2** (after Wave 1):
```
Developer A: T025-T031 (SCCPropagator)
Developer B: T044-T045 (Propagator unit tests)
Developer C: T036-T038 (Optimizer)
```

**Wave 3** (after Wave 2):
```
Developer A: T046-T048 (Integration/snapshot tests)
Developer B: Code review and refinement
Developer C: Documentation
```

---

## Task Count Summary

- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundation)**: 16 tasks
- **User Story 1 (P1 - MVP)**: 29 tasks (19 implementation + 10 tests)
- **User Story 2 (P2)**: 19 tasks (10 implementation + 9 tests)
- **User Story 3 (P3)**: 11 tasks (5 implementation + 6 tests)
- **User Story 4 (P2)**: 26 tasks (14 implementation + 12 tests)
- **Phase 7 (Polish)**: 16 tasks
- **Total**: 120 tasks

**Parallel Tasks**: 67 tasks marked [P] (56% can be parallelized)

**Independent Stories**: Each user story (US1-US4) is independently testable

**MVP Scope**: Phase 1 + Phase 2 + User Story 1 = 48 tasks (40% of total)

---

## Success Criteria per Phase

### Phase 1 Success
- ✅ Module structure created
- ✅ All files exist and compile (even if empty stubs)

### Phase 2 Success
- ✅ All foundational types compile
- ✅ Lattice meet operation passes property tests
- ✅ Worklist deduplication works correctly
- ✅ No dependencies on unimplemented IR features

### User Story 1 Success
- ✅ Simple constant propagation works (x=5; y=10; z=x+y → z=15)
- ✅ I32 arithmetic evaluated correctly
- ✅ Overflow handled conservatively (→ Top)
- ✅ Convergence within 3 iterations for simple functions
- ✅ All US1 tests pass
- ✅ Snapshot tests match expected IR transformations

### User Story 2 Success
- ✅ Constant branch conditions resolved (if true → unconditional jump)
- ✅ Unreachable blocks marked correctly
- ✅ Switch statements with constant selectors optimized
- ✅ Nested branches handled correctly
- ✅ All US2 tests pass

### User Story 3 Success
- ✅ Phi nodes simplified when all values constant and same
- ✅ Unreachable phi predecessors ignored
- ✅ Mixed value phi nodes handled (→ Top)
- ✅ All US3 tests pass

### User Story 4 Success
- ✅ All integer types (I8-I64, U8-U64) evaluated correctly
- ✅ Floating-point NaN/Infinity handled per IEEE 754
- ✅ Char Unicode validity preserved
- ✅ Bitwise operations work for all integer types
- ✅ All US4 tests pass

### Phase 7 Success
- ✅ Verbose output provides useful debugging information
- ✅ Statistics accurately track optimizations
- ✅ Performance benchmarks meet requirements (SC-003, SC-004)
- ✅ Documentation complete and accurate
- ✅ All clippy warnings resolved
- ✅ Code formatted with cargo fmt

---

## Notes for Implementers

### Testing Philosophy
- **Write tests first** for each user story (TDD approach recommended)
- **Snapshot tests** capture before/after IR transformations for regression prevention
- **Property tests** verify lattice meet operation laws (commutativity, associativity, idempotency)
- **Integration tests** validate end-to-end optimization pipeline

### Common Pitfalls to Avoid
1. **SSA Form Violation**: Never modify LHS of assignments, only RHS
2. **Lattice Non-Monotonicity**: Ensure values never decrease in ordering
3. **Type Confusion**: Always check type compatibility before evaluation
4. **Infinite Loops**: Implement max iteration limit from the start
5. **Floating-Point Equality**: Use bit-level comparison for NaN/zero handling

### Performance Tips
- **Preallocate** HashMap/HashSet with estimated capacities
- **Reuse** worklists instead of creating new ones
- **Profile** with criterion before optimizing
- **Benchmark** convergence iterations separately from execution time

### Integration with Existing Code
- **Phase trait**: Follow existing DCE implementation pattern
- **Error handling**: Use existing `OptimizationError` enum
- **Diagnostics**: Integrate with existing diagnostic emitter
- **Testing**: Follow existing test file naming conventions

---

**Tasks Status**: ✅ Complete  
**Total Estimated Effort**: 120 tasks  
**MVP Effort**: 48 tasks (Phase 1 + 2 + US1)  
**Recommended Team Size**: 2-3 developers for parallel execution  
**Estimated Timeline**: 2-3 weeks for MVP, 4-6 weeks for complete feature

**Next Step**: Begin Phase 1 (Setup) tasks T001-T003
