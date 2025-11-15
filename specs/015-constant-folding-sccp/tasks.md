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
- [ ] T002 Create module declaration file `src/ir/optimizer/constant_folding/mod.rs` with public exports (1 of 7 source files)
- [ ] T003 [P] Create placeholder files: `optimizer.rs`, `lattice.rs`, `evaluator.rs`, `worklist.rs`, `statistics.rs`, `utils.rs` in `src/ir/optimizer/constant_folding/` (6 additional files, total 7 per plan.md)
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
- [ ] T014 [P] Create SSA validation helper functions in `src/ir/optimizer/constant_folding/utils.rs` for verifying value references: `fn validate_ssa_value(value_id: &ValueId, module: &IrModule) -> Result<(), SsaValidationError>` (verifies definition exists), `fn validate_dominance(use_location: &BlockId, def_location: &BlockId, cfg: &ControlFlowGraph) -> bool` (verifies block-level dominance is sufficient for SSA - definition block must dominate use block; uses immediate dominators algorithm from existing IR infrastructure's dominance analysis; instruction ordering within blocks validated separately during IR construction; CFG parameter provides dominance tree - if unavailable, returns false triggering conservative fallback), `fn validate_phi_incoming(phi: &PhiNode, predecessors: &[BlockId]) -> Result<(), PhiValidationError>` (verifies phi incoming values match predecessors)
- [ ] T015 [P] Create memory estimation helper function in `src/ir/optimizer/constant_folding/utils.rs` for lattice size tracking: `fn estimate_lattice_memory(lattice_map: &HashMap<ValueId, LatticeValue>) -> usize` (returns estimated bytes, fails if exceeds 100KB threshold)

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Basic Constant Folding (Priority: P1) 🎯 MVP

**Goal**: Enable compile-time evaluation of constant expressions (arithmetic, logical, comparison, bitwise, unary, cast operations)

**Independent Test**: Compile IR with constant expressions like `add i32 2, 3` and verify the instruction is replaced with constant value `5`, reducing instruction count

### Implementation for User Story 1

- [ ] T016 [P] [US1] Implement integer arithmetic folding functions in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_add_int()`, `fold_sub_int()`, `fold_mul_int()`, `fold_div_int()`, `fold_mod_int()` with wrapping semantics (operation categories defined in `data-model.md`)
- [ ] T017 [P] [US1] Implement floating-point arithmetic folding functions in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_fadd()`, `fold_fsub()`, `fold_fmul()`, `fold_fdiv()` following IEEE 754 rules (use canonical quiet NaN representation, do not preserve NaN payloads)
- [ ] T018 [P] [US1] Implement comparison operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_icmp()` for integer comparisons (eq, ne, lt, gt, le, ge)
- [ ] T019 [P] [US1] Implement logical operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_and()`, `fold_or()` for boolean values
- [ ] T020 [P] [US1] Implement bitwise operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_bitwise_and()`, `fold_bitwise_or()`, `fold_bitwise_xor()`, `fold_shl()`, `fold_shr()`
- [ ] T021 [P] [US1] Implement unary operation folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_neg()`, `fold_not()`
- [ ] T022 [P] [US1] Implement type cast folding in `src/ir/optimizer/constant_folding/evaluator.rs`: `fold_sext()`, `fold_zext()`, `fold_trunc()`, `fold_fptosi()`, `fold_fptoui()`, `fold_sitofp()`, `fold_uitofp()`
- [ ] T023 [US1] Implement main `fold_instruction()` function in `src/ir/optimizer/constant_folding/evaluator.rs` that dispatches to specific folding functions based on instruction type (handles nested constant expressions recursively by checking if operands are constants and folding bottom-up)
- [ ] T024 [US1] Implement basic constant folding pass in `src/ir/optimizer/constant_folding/optimizer.rs`: scan all instructions in a function, identify foldable operations, replace with constants while preserving debug info and source span metadata from original instructions (per FR-020)
- [ ] T025 [US1] Update `Phase::run()` implementation in `src/ir/optimizer/constant_folding/optimizer.rs` to iterate functions and apply basic constant folding pass
- [ ] T026 [US1] Implement instruction count reporting to stdout in `src/ir/optimizer/constant_folding/optimizer.rs` after optimization completes
- [ ] T027 [US1] Implement verbose statistics output to stderr in `src/ir/optimizer/constant_folding/optimizer.rs` when verbose mode enabled (structured format: one line per function with fields 'function_name', 'instructions_folded', 'values_propagated', 'branches_resolved', 'blocks_removed' as tab-separated values)

### Tests for User Story 1

- [ ] T028 [P] [US1] Write unit tests for integer arithmetic folding in `tests/ir_constant_folding_basic_tests.rs`: test addition, subtraction, multiplication, division, modulo; include nested constant expression test with at least 3 nesting levels (e.g., `mul(add(sub(10, 3), 2), 4)`) to validate acceptance scenario from spec.md User Story 1
- [ ] T029 [P] [US1] Write unit tests for floating-point arithmetic in `tests/ir_constant_folding_basic_tests.rs`: test IEEE 754 operations including NaN and infinity handling
- [ ] T030 [P] [US1] Write unit tests for comparison operations in `tests/ir_constant_folding_basic_tests.rs`: test all comparison variants (eq, ne, lt, gt, le, ge)
- [ ] T031 [P] [US1] Write unit tests for logical and bitwise operations in `tests/ir_constant_folding_basic_tests.rs`
- [ ] T032 [P] [US1] Write unit tests for unary operations in `tests/ir_constant_folding_basic_tests.rs`: negation and NOT
- [ ] T033 [P] [US1] Write unit tests for type casts in `tests/ir_constant_folding_basic_tests.rs`: widening, narrowing, signed/unsigned conversions
- [ ] T034 [P] [US1] Write edge case tests in `tests/ir_constant_folding_edge_cases_tests.rs`: division by zero (verify `div i32 10, 0` and `fdiv 1.0, 0.0` are NOT folded and original instruction preserved per FR-009), integer overflow wrapping, NaN propagation, signed zero preservation
- [ ] T035 [US1] Write snapshot tests in `tests/ir_constant_folding_snapshot_tests.rs` using `insta` to verify IR transformation correctness for basic folding scenarios

**Checkpoint**: User Story 1 complete - basic constant folding functional and tested independently

---

## Phase 4: User Story 2 - Simple Constant Propagation (Priority: P2)

**Goal**: Track constant values through store/load sequences, replacing redundant loads with constant values

**Independent Test**: Compile IR with constant store followed by loads, verify loads are replaced with direct constant values eliminating memory accesses

### Implementation for User Story 2

- [ ] T036 [US2] Implement escape analysis helper in `src/ir/optimizer/constant_folding/utils.rs` to determine if local variable escapes function scope: `fn analyze_escape(value_id: &ValueId, function: &IrFunction) -> EscapeStatus` where `EscapeStatus` enum has variants `Local` (proven local-only through single-pass analysis with no indirect uses), `Escaped` (address taken/passed to function/stored to global/returned/stored to another escaping variable), `Unknown` (indirect uses through pointers, recursive data structures, or analysis depth exceeds 100 instruction traversal limit)
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
- [ ] T046 [P] [US2] Write tests for escaping variables in `tests/ir_constant_folding_propagation_tests.rs`: verify conservative behavior when address escapes; include test case where escape analysis returns Unknown status (e.g., indirect uses through pointers, analysis depth exceeds 100 instruction limit) and verify loads are preserved without optimization per FR-021
- [ ] T047 [US2] Write snapshot tests in `tests/ir_constant_folding_snapshot_tests.rs` for constant propagation scenarios

**Checkpoint**: User Stories 1 AND 2 complete - basic folding and simple propagation both functional independently

---

## Phase 5: User Story 3 - Advanced SCCP Analysis (Priority: P3)

**Goal**: Implement full Sparse Conditional Constant Propagation with control flow analysis to eliminate unreachable code and propagate constants through complex CFG including phi nodes

**Independent Test**: Compile IR with constant conditional branches, verify unreachable blocks removed, phi nodes simplified, and constants propagated through complex control flow

### Implementation for User Story 3

- [ ] T048 [US3] Implement `SCCPResult` struct in `src/ir/optimizer/constant_folding/worklist.rs` with fields for lattice_values, reachable_blocks, executable_edges, foldable_count, resolvable_branches
- [ ] T049 [US3] Implement `SCCPError` enum in `src/ir/optimizer/constant_folding/worklist.rs` with variants for MemoryLimit, InvalidCFG, InvalidSSA
- [ ] T050 [US3] Implement SCCP initialization in `src/ir/optimizer/constant_folding/worklist.rs`: create lattice map (HashMap<ValueId, LatticeValue>), mark entry block as reachable, initialize function parameters with Top lattice value per FR-018a, initialize worklist with entry block instructions (uses VecDeque for worklist; petgraph used only for existing CFG dominance/reachability analysis, not SCCP worklist)
- [ ] T051 [US3] Implement worklist processing loop in `src/ir/optimizer/constant_folding/worklist.rs`: process SSA edges and CFG edges until fixed point reached
- [ ] T052 [US3] Implement phi node merge logic in `src/ir/optimizer/constant_folding/worklist.rs`: merge incoming values only from reachable predecessors using lattice meet operation (merge logic does NOT remove unreachable incoming edges; edges marked for deferred removal in CFG cleanup pass per FR-016)
- [ ] T053 [US3] Implement conditional branch resolution in `src/ir/optimizer/constant_folding/worklist.rs`: mark successor blocks based on constant condition values
- [ ] T054 [US3] Implement memory limit checking in `src/ir/optimizer/constant_folding/worklist.rs`: track lattice map size, return error if exceeds 100KB
- [ ] T055 [US3] Implement `sccp_analysis()` main function in `src/ir/optimizer/constant_folding/worklist.rs` orchestrating the full algorithm
- [ ] T056 [US3] Implement CFG cleanup pass in `src/ir/optimizer/constant_folding/optimizer.rs`: remove unreachable blocks, simplify phi nodes by removing incoming values from unreachable predecessors and replacing single-incoming-value phis with direct values (per FR-024), remove dead branches (depends on T052 phi merge logic completion to ensure edge removal happens after SCCP analysis per FR-016)
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

- [ ] T070 [P] Implement malformed IR handling in `src/ir/optimizer/constant_folding/optimizer.rs`: detect malformed IR cases including (1) dangling SSA value references (ValueId with no corresponding definition in module), (2) type mismatches in operations (e.g., integer operation on float value), (3) missing CFG dominance information (dominator tree unavailable for block), (4) invalid phi node structure (incoming values count mismatch with predecessors); emit warnings for each case to stderr using existing IR diagnostic format, skip optimization for affected instruction, preserve original instruction unmodified
- [ ] T071 [P] Implement diagnostic message formatting in `src/ir/optimizer/constant_folding/statistics.rs`: structured stderr output for verbose mode
- [ ] T072 [P] Add comprehensive rustdoc comments to all public APIs in `src/ir/optimizer/constant_folding/mod.rs`, `optimizer.rs`, `lattice.rs`, `evaluator.rs`, `worklist.rs` including panic conditions (evaluator division operations, worklist memory allocation), safety requirements, and error cases; add module-level documentation in `mod.rs` with usage examples and architectural overview
- [ ] T073 [P] Add integration tests for Phase trait compliance in `tests/ir_constant_folding_basic_tests.rs`: verify `name()` returns correct string "ConstantFoldingOptimizer", verify `run()` preserves module validity and compiles successfully against existing Phase trait interface from current IR infrastructure version, validate no breaking changes to existing optimizer pipeline integration (validates NFR-003 external test harness via existing Cargo test integration in CI pipeline; no CI configuration changes required)
- [ ] T074 [P] Add performance benchmarks in `benches/jsavrs_benchmark.rs` for functions with 1500+ instructions using representative workload (30% arithmetic operations, 30% conditional branches, 20% phi nodes, 20% load/store operations), verify <1 second target; include benchmark with conservative fallback scenario (50% Unknown escape statuses, frequent dominance recomputation) to verify complexity remains O(n log n) and does not degrade to pathological O(n²)
- [ ] T075 [P] Write performance assertion test in `tests/ir_constant_folding_basic_tests.rs` that fails if processing a function with exactly 1500 instructions exceeds 1 second (validates NFR-001)
- [ ] T076 [P] Write test validating CFG cleanup runs when SCCP finds zero foldable operations in `tests/ir_constant_folding_sccp_tests.rs` (validates FR-029): create function with no constants, enable SCCP, verify cleanup pass executes
- [ ] T077 [P] Write quickstart validation tests in `tests/ir_constant_folding_basic_tests.rs`: verify examples from `quickstart.md` compile and run correctly
- [ ] T078 [P] Write memory ordering preservation test in `tests/ir_constant_folding_basic_tests.rs` (validates FR-022): create IR with memory operations (stores to potentially-aliasing locations, volatile operations, or operations with side effects), apply optimization, verify instruction ordering preserved for all memory-visible operations
- [ ] T078a [P] Write debug metadata preservation test in `tests/ir_constant_folding_snapshot_tests.rs` (validates FR-020): create IR with debug info and source span metadata attached to instructions, apply constant folding optimization, use insta snapshot to verify debug metadata is preserved in optimized IR for folded and non-folded instructions
- [ ] T078b [P] Write CFG structural validity test in `tests/ir_constant_folding_sccp_tests.rs` (validates FR-013): after SCCP and CFG cleanup, verify: (1) all blocks reachable from entry block, (2) all blocks have valid terminator instructions, (3) no dangling edge references to removed blocks, (4) phi nodes only reference existing predecessor blocks
- [ ] T079 Code cleanup: ensure consistent error handling patterns across all modules
- [ ] T080 Code cleanup: run `cargo fmt` and `cargo clippy` to ensure idiomatic Rust code
- [ ] T081 Final validation: run full test suite with `cargo test` and verify 100% pass rate
- [ ] T082 [P] Documentation Rigor Constitutional Compliance Audit (validates FR-030, FR-031 per Documentation Rigor principle): Perform comprehensive validation of `research.md` and `data-model.md` against constitutional standard requiring "detailed, precise, meticulous, and in-depth" documentation with "no important detail unexplored"
  - [ ] T082.1 Validate `research.md` Algorithm Analysis: Verify document contains Big-O complexity proofs for all major algorithms (SCCP worklist, constant folding dispatch, escape analysis traversal) with mathematical derivations showing worst-case, average-case, and best-case scenarios; ensure space complexity analysis included
  - [ ] T082.2 Validate `research.md` Alternative Approaches: Verify document discusses alternative implementation strategies considered (e.g., demand-driven vs. worklist SCCP, value numbering vs. lattice-based propagation, SSA destruction approaches) with explicit rationale for rejection including quantitative trade-off analysis (performance, memory, maintainability metrics)
  - [ ] T082.3 Validate `research.md` Performance Characteristics: Verify document contains detailed performance analysis including cache behavior, memory allocation patterns, instruction-level parallelism opportunities, and profiling data from representative workloads; ensure memory estimation formulas documented with concrete examples
  - [ ] T082.4 Validate `data-model.md` LatticeValue Documentation: Verify complete documentation of LatticeValue enum including: (1) formal lattice theory foundation with partial ordering definition, (2) state transition diagram showing all valid transitions between Top/Constant/Bottom, (3) meet operation semantics with truth tables for all combinations, (4) memory layout and size calculations per variant
  - [ ] T082.5 Validate `data-model.md` SCCP State Machine: Verify comprehensive state machine documentation including: (1) formal finite state machine definition with states and transitions, (2) initialization conditions for all SSA value types (parameters, phi nodes, constants, operations), (3) fixed-point convergence proof or termination argument, (4) edge case handling (unreachable blocks, empty phi nodes, cyclic dependencies)
  - [ ] T082.6 Validate `data-model.md` Worklist Algorithm Data Flows: Verify detailed data flow diagrams showing: (1) SSA edge processing with input/output lattice states, (2) CFG edge propagation mechanisms, (3) phi node merge algorithm with step-by-step example, (4) interaction between worklist and lattice map with concrete memory access patterns
  - [ ] T082.7 Validate `data-model.md` Component Relationships: Verify architectural diagrams documenting: (1) module dependency graph (lattice → evaluator → worklist → optimizer), (2) data structure ownership and lifetime relationships, (3) trait boundaries and abstraction layers, (4) integration points with existing IR infrastructure (Phase trait, CFG, dominance analysis)
  - [ ] T082.8 Validate Public API Documentation: Audit all public APIs in `src/ir/optimizer/constant_folding/` to verify rustdoc comments contain: (1) comprehensive descriptions with usage examples, (2) explicit panic conditions with triggering scenarios, (3) error case documentation with recovery strategies, (4) safety requirements for any unsafe code blocks, (5) performance characteristics and complexity guarantees
  - [ ] T082.9 Validate Architectural Decision Documentation: Verify `research.md` documents key architectural decisions including: (1) choice of hash map over vector for lattice storage (with memory/performance trade-offs), (2) deferred phi edge removal design (FR-016 rationale), (3) conservative fallback strategies (escape analysis, malformed IR), (4) single-pass vs. multi-pass optimization pipeline placement
  - [ ] T082.10 Cross-Reference Validation: Verify all claims in `spec.md` requirements are supported by detailed explanations in `research.md` or `data-model.md`; ensure no "TODO", "TBD", or placeholder text remains; confirm mathematical notation is consistent and well-defined throughout documentation

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

- **Total Tasks**: 94 (T078a, T078b added for FR-020 debug metadata and FR-013 CFG validity; spec updates: FR-018a SCCP parameters, FR-021 escape analysis limit, FR-023 dominance handling, FR-025 malformed IR taxonomy, FR-027 TSV format, FR-008 canonical NaN, NFR-003 snapshot testing)
- **User Story 1 (P1)**: 20 tasks (12 implementation + 8 tests)
- **User Story 2 (P2)**: 12 tasks (6 implementation + 6 tests)
- **User Story 3 (P3)**: 22 tasks (14 implementation + 8 tests)
- **Setup**: 5 tasks
- **Foundational**: 10 tasks
- **Polish**: 25 tasks (T070-T081 + T082 with 10 sub-tasks for constitutional Documentation Rigor validation)
- **Parallel Opportunities**: 56 tasks marked [P] can run in parallel within their phase/story
- **Independent Test Criteria**: Each user story has clear acceptance scenarios and can be validated independently
- **Suggested MVP Scope**: Setup + Foundational + User Story 1 (35 tasks)
- **Requirement Coverage**: All 35 functional requirements (FR-001 to FR-031 + FR-018a) and 3 NFRs now have complete task coverage with HIGH-priority issues resolved
