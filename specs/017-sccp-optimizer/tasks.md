# Tasks: Sparse Conditional Constant Propagation (SCCP) Optimizer

**Input**: Design documents from `/specs/017-sccp-optimizer/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api.md, quickstart.md  
**Feature Branch**: `017-sccp-optimizer`  
**Project Type**: Rust compiler module (single project structure)

**Tests**: Tests are NOT explicitly requested in the specification. Following the template guidance, test tasks are INCLUDED to ensure reliability and correctness, which are paramount for compiler optimizations.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each optimization capability.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4, US5)
- Include exact file paths in descriptions

## Path Conventions

Single Rust project structure:
- Source code: `src/ir/optimizer/constant_folding/`
- Unit tests: Within module files using `#[cfg(test)]`
- Integration tests: `tests/sccp_*.rs`
- Benchmarks: `benches/sccp_benchmark.rs`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, directory structure, and module declaration

- [ ] T001 Create directory structure at `src/ir/optimizer/constant_folding/` for SCCP module
- [ ] T002 Create module declaration file at `src/ir/optimizer/constant_folding/mod.rs` with public exports and module structure
- [ ] T003 [P] Update `src/ir/optimizer/mod.rs` to declare and export the `constant_folding` module
- [ ] T004 [P] Verify project builds successfully with empty SCCP module structure using `cargo build`

**Verification**: `cargo build` completes without errors, SCCP module is accessible from other parts of codebase

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story implementation

**⚠️ CRITICAL**: No user story work can begin until this phase is complete. These components are fundamental dependencies for all SCCP optimization capabilities.

- [ ] T005 Implement `LatticeValue` enum in `src/ir/optimizer/constant_folding/lattice.rs` with three variants: `Top` (unknown), `Constant(IrLiteralValue)`, and `Bottom` (variable)
- [ ] T006 Implement `meet` operation for `LatticeValue` in `src/ir/optimizer/constant_folding/lattice.rs` following lattice-theoretic semantics (commutativity, associativity, idempotence)
- [ ] T007 [P] Add helper methods to `LatticeValue` in `src/ir/optimizer/constant_folding/lattice.rs`: `is_top()`, `is_constant()`, `is_bottom()`, `as_constant() -> Option<&IrLiteralValue>`
- [ ] T008 [P] Create unit tests for `LatticeValue` in `src/ir/optimizer/constant_folding/lattice.rs` testing meet operation properties (commutativity, associativity, idempotence, monotonicity)
- [ ] T009 [P] Create unit tests for `LatticeValue` edge cases in `src/ir/optimizer/constant_folding/lattice.rs`: same constant meet, different constants meet to Bottom, Top meet behaviors
- [ ] T010 Implement `Worklist` struct in `src/ir/optimizer/constant_folding/worklist.rs` with `ssa_worklist: VecDeque<Value>` and `cfg_worklist: VecDeque<(NodeIndex, NodeIndex)>`
- [ ] T011 Add deduplication tracking to `Worklist` in `src/ir/optimizer/constant_folding/worklist.rs` using `ssa_seen: HashSet<Value>` and `cfg_seen: HashSet<(NodeIndex, NodeIndex)>`
- [ ] T012 Implement `Worklist` methods in `src/ir/optimizer/constant_folding/worklist.rs`: `new()`, `add_ssa(value)`, `add_cfg(edge)`, `pop_ssa()`, `pop_cfg()`, `is_empty()`
- [ ] T013 [P] Create unit tests for `Worklist` in `src/ir/optimizer/constant_folding/worklist.rs` testing FIFO ordering, deduplication, and worklist emptiness detection
- [ ] T014 Create skeleton `SccpAnalyzer` struct in `src/ir/optimizer/constant_folding/analyzer.rs` with fields: `lattice_values: HashMap<Value, LatticeValue>`, `executable_blocks: HashSet<NodeIndex>`, `worklist: Worklist`, `max_iterations: usize`, `iteration_count: usize`
- [ ] T015 Implement `SccpAnalyzer::new(max_iterations: usize)` constructor in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T016 Create skeleton `SccpTransformer` struct in `src/ir/optimizer/constant_folding/transformer.rs` for IR mutation logic
- [ ] T017 Create `SccpOptimizer` public API struct in `src/ir/optimizer/constant_folding/mod.rs` with fields: `verbose: bool`, `max_iterations: usize`, `enabled: bool`
- [ ] T018 Implement builder methods in `src/ir/optimizer/constant_folding/mod.rs`: `SccpOptimizer::new()`, `with_verbose()`, `with_max_iterations(max)`, `disabled()`
- [ ] T019 [P] Implement stub `Phase` trait for `SccpOptimizer` in `src/ir/optimizer/constant_folding/mod.rs` with `name()` returning `"SCCP"` and `run(&mut self, ir: &mut Module)` returning `false`
- [ ] T020 [P] Create integration test file at `tests/sccp_integration_tests.rs` with basic framework for end-to-end SCCP testing
- [ ] T021 [P] Create snapshot test file at `tests/sccp_snapshot_tests.rs` using `insta` crate for before/after IR comparison
- [ ] T022 [P] Create benchmark file at `benches/sccp_benchmark.rs` using `criterion` crate with skeleton benchmark setup

**Checkpoint**: Foundation ready - all core data structures exist, module builds, Phase trait stub implemented. User story implementation can now begin in parallel.

---

## Phase 3: User Story 1 - Constant Expression Simplification (Priority: P1) 🎯 MVP

**Goal**: Implement the core value proposition of SCCP - identifying and replacing values that are always constant. This includes lattice-based SSA value tracking, constant folding for arithmetic operations, and basic constant propagation logic.

**Independent Test**: Compile a program with constant arithmetic expressions (e.g., `x = 5 + 3; y = x * 2;`) and verify the optimized IR replaces variable uses with constants (8 and 16). Test using snapshot testing with `insta` crate.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T023 [P] [US1] Create unit test for constant folding of signed integer addition (i8, i16, i32, i64) in `src/ir/optimizer/constant_folding/constant_folder.rs` testing wrapping_add semantics
- [ ] T024 [P] [US1] Create unit test for constant folding of unsigned integer addition (u8, u16, u32, u64) in `src/ir/optimizer/constant_folding/constant_folder.rs` testing wrapping_add semantics
- [ ] T025 [P] [US1] Create unit test for constant folding of floating-point addition (f32, f64) in `src/ir/optimizer/constant_folding/constant_folder.rs` with standard float arithmetic
- [ ] T026 [P] [US1] Create unit test for constant folding of signed integer subtraction with wrapping semantics in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T027 [P] [US1] Create unit test for constant folding of unsigned integer subtraction with wrapping semantics in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T028 [P] [US1] Create unit test for constant folding of integer multiplication with wrapping overflow behavior in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T029 [P] [US1] Create unit test for constant folding of integer division including division-by-zero returning None in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T030 [P] [US1] Create unit test for constant folding of integer modulo operation in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T031 [P] [US1] Create unit test for constant folding type mismatch scenarios returning None in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T032 [P] [US1] Create integration test in `tests/sccp_integration_tests.rs` for direct constant assignment propagation (e.g., `x = 42` → all uses become 42)
- [ ] T033 [P] [US1] Create integration test in `tests/sccp_integration_tests.rs` for constant arithmetic propagation (e.g., `y = 5 + 3` → y becomes constant 8)
- [ ] T034 [P] [US1] Create integration test in `tests/sccp_integration_tests.rs` for chained constant propagation (e.g., `x = 42; y = x + 1; z = y * 2` → 42, 43, 86)
- [ ] T035 [P] [US1] Create snapshot test in `tests/sccp_snapshot_tests.rs` for before/after IR comparison of constant expression simplification using `insta::assert_snapshot!`

### Implementation for User Story 1

- [ ] T036 [P] [US1] Implement `fold_binary_add` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer and float types using wrapping_add for integers
- [ ] T037 [P] [US1] Implement `fold_binary_subtract` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer and float types using wrapping_sub for integers
- [ ] T038 [P] [US1] Implement `fold_binary_multiply` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer and float types using wrapping_mul for integers
- [ ] T039 [P] [US1] Implement `fold_binary_divide` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` with division-by-zero checking (return None for zero divisor)
- [ ] T040 [P] [US1] Implement `fold_binary_modulo` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for integer types with modulo-by-zero checking
- [ ] T041 [US1] Implement main `fold_binary(op: IrBinaryOp, left: IrLiteralValue, right: IrLiteralValue) -> Option<IrLiteralValue>` function in `src/ir/optimizer/constant_folding/constant_folder.rs` using pattern matching to dispatch to helper functions
- [ ] T042 [US1] Add comprehensive pattern matching for all primitive types (i8-u64, f32-f64) in `fold_binary` function in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T043 [US1] Implement `initialize(&mut self, function: &Function)` method in `src/ir/optimizer/constant_folding/analyzer.rs` to populate `lattice_values` with all SSA values at Top, mark entry block executable
- [ ] T044 [US1] Implement `evaluate_instruction(&mut self, instr: &Instruction, function: &Function) -> LatticeValue` in `src/ir/optimizer/constant_folding/analyzer.rs` to compute lattice value for instruction results
- [ ] T045 [US1] Implement logic in `evaluate_instruction` to handle binary operations by getting operand lattice values and calling `constant_folder::fold_binary` if both operands are constant
- [ ] T046 [US1] Implement `update_value(&mut self, value: Value, new_lattice: LatticeValue)` in `src/ir/optimizer/constant_folding/analyzer.rs` to perform meet operation and add changed values to worklist
- [ ] T047 [US1] Implement `propagate(&mut self, value: Value, function: &Function)` in `src/ir/optimizer/constant_folding/analyzer.rs` to add all uses of a changed value to the SSA worklist
- [ ] T048 [US1] Implement worklist processing loop in `analyze(&mut self, function: &Function)` method in `src/ir/optimizer/constant_folding/analyzer.rs` iterating until worklist empty or max_iterations reached
- [ ] T049 [US1] Implement SSA worklist processing in `analyze` method to re-evaluate instructions that use changed values in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T050 [US1] Add iteration counting and max_iterations safety limit enforcement in `analyze` method in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T051 [US1] Implement conservative fallback in `analyze` method to degrade all remaining Top values to Bottom when iteration limit exceeded in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T052 [US1] Implement `replace_constant_uses(&self, function: &mut Function, lattice_values: &HashMap<Value, LatticeValue>)` in `src/ir/optimizer/constant_folding/transformer.rs` to replace SSA value uses with constant operands in IR
- [ ] T053 [US1] Implement `mark_constant_instructions_dead(&self, function: &mut Function, lattice_values: &HashMap<Value, LatticeValue>)` in `src/ir/optimizer/constant_folding/transformer.rs` to mark instructions computing constants as dead for DCE
- [ ] T054 [US1] Wire up `SccpOptimizer::run` implementation in `src/ir/optimizer/constant_folding/mod.rs` to create analyzer, run analysis, create transformer, apply transformations, return true if changes made
- [ ] T055 [US1] Add verbose logging in `SccpOptimizer::run` to output constants propagated count when `self.verbose == true` in `src/ir/optimizer/constant_folding/mod.rs`
- [ ] T056 [US1] Add enable/disable flag checking in `SccpOptimizer::run` to skip optimization if `self.enabled == false` in `src/ir/optimizer/constant_folding/mod.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional - constant expressions can be identified and propagated, basic arithmetic operations are folded, and the Phase trait integration works. Test independently with snapshot tests.

---

## Phase 4: User Story 2 - Unreachable Code Detection Through Constant Conditions (Priority: P1)

**Goal**: Implement the "conditional" and "sparse" part of SCCP by analyzing control flow based on constant conditions. When branch conditions are compile-time determinable, mark unreachable paths for DCE elimination. This completes the core SCCP algorithm when combined with User Story 1.

**Independent Test**: Compile a program with constant conditions (e.g., `if (true) { A } else { B }`) and verify the false branch is marked unreachable in the optimized IR. Use CFG analysis to confirm unreachable blocks are not in the executable set.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T057 [P] [US2] Create unit test for branch evaluation with constant true condition in `src/ir/optimizer/constant_folding/analyzer.rs` verifying only true successor marked executable
- [ ] T058 [P] [US2] Create unit test for branch evaluation with constant false condition in `src/ir/optimizer/constant_folding/analyzer.rs` verifying only false successor marked executable
- [ ] T059 [P] [US2] Create unit test for branch evaluation with variable/unknown condition in `src/ir/optimizer/constant_folding/analyzer.rs` verifying both successors marked executable
- [ ] T060 [P] [US2] Create integration test in `tests/sccp_integration_tests.rs` for constant true branch marking only true path reachable
- [ ] T061 [P] [US2] Create integration test in `tests/sccp_integration_tests.rs` for constant false branch marking only false path reachable
- [ ] T062 [P] [US2] Create integration test in `tests/sccp_integration_tests.rs` for nested conditionals where outer condition is constant true
- [ ] T063 [P] [US2] Create integration test in `tests/sccp_integration_tests.rs` for loop with constant false entry condition marking loop body unreachable
- [ ] T064 [P] [US2] Create snapshot test in `tests/sccp_snapshot_tests.rs` for before/after IR showing unreachable blocks marked correctly

### Implementation for User Story 2

- [ ] T065 [P] [US2] Implement `mark_executable(&mut self, block: NodeIndex, function: &Function)` in `src/ir/optimizer/constant_folding/analyzer.rs` to add block to executable set and queue its instructions
- [ ] T066 [P] [US2] Implement `evaluate_branch(&self, condition_value: Value) -> BranchResult` in `src/ir/optimizer/constant_folding/analyzer.rs` returning which successors are reachable based on condition lattice value
- [ ] T067 [US2] Extend `evaluate_instruction` in `src/ir/optimizer/constant_folding/analyzer.rs` to handle conditional branch instructions by evaluating branch conditions
- [ ] T068 [US2] Implement CFG edge worklist processing in `analyze` method in `src/ir/optimizer/constant_folding/analyzer.rs` to process newly executable edges and mark successor blocks reachable
- [ ] T069 [US2] Add logic to check if block is in `executable_blocks` before processing its instructions in `analyze` method in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T070 [US2] Implement `simplify_branches(&self, function: &mut Function, lattice_values: &HashMap<Value, LatticeValue>)` in `src/ir/optimizer/constant_folding/transformer.rs` to convert constant-condition branches to unconditional jumps
- [ ] T071 [US2] Implement branch simplification logic to replace conditional branch with unconditional jump when condition is constant in `src/ir/optimizer/constant_folding/transformer.rs`
- [ ] T072 [US2] Implement `mark_unreachable_blocks(&self, function: &mut Function, executable_blocks: &HashSet<NodeIndex>)` in `src/ir/optimizer/constant_folding/transformer.rs` to mark blocks not in executable set for DCE removal
- [ ] T073 [US2] Add unreachable blocks count to verbose logging output in `SccpOptimizer::run` in `src/ir/optimizer/constant_folding/mod.rs`
- [ ] T074 [US2] Add branches simplified count to verbose logging output in `SccpOptimizer::run` in `src/ir/optimizer/constant_folding/mod.rs`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. SCCP can now perform both constant propagation and unreachable code detection. This forms the complete core SCCP algorithm.

---

## Phase 5: User Story 3 - Phi Node Constant Resolution (Priority: P2)

**Goal**: Extend constant propagation across control flow merges by analyzing phi nodes. When all executable paths provide the same constant value, recognize that the merged value is also constant, enabling further propagation opportunities.

**Independent Test**: Compile a program with a phi node where both incoming values are the same constant (e.g., `x = condition ? 5 : 5`) and verify the phi result is identified as constant 5. Test with programs where different paths provide different constants and verify result is marked as variable.

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T075 [P] [US3] Create unit test for phi node with all same constant inputs in `src/ir/optimizer/constant_folding/analyzer.rs` verifying result is that constant
- [ ] T076 [P] [US3] Create unit test for phi node with different constant inputs in `src/ir/optimizer/constant_folding/analyzer.rs` verifying result is Bottom (variable)
- [ ] T077 [P] [US3] Create unit test for phi node with some unreachable predecessors in `src/ir/optimizer/constant_folding/analyzer.rs` verifying only reachable inputs considered
- [ ] T078 [P] [US3] Create unit test for phi node where all reachable predecessors provide same constant but unreachable differ in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T079 [P] [US3] Create integration test in `tests/sccp_integration_tests.rs` for phi node with identical constant values from both branches
- [ ] T080 [P] [US3] Create integration test in `tests/sccp_integration_tests.rs` for phi node in loop where value becomes constant after several iterations
- [ ] T081 [P] [US3] Create snapshot test in `tests/sccp_snapshot_tests.rs` for phi node constant resolution with before/after IR comparison

### Implementation for User Story 3

- [ ] T082 [P] [US3] Implement `update_phi(&mut self, phi: &PhiNode, block: NodeIndex, function: &Function) -> LatticeValue` in `src/ir/optimizer/constant_folding/analyzer.rs` to compute phi result considering only executable predecessors
- [ ] T083 [US3] Implement logic in `update_phi` to iterate over phi operands and check if predecessor blocks are in `executable_blocks` set in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T084 [US3] Implement logic in `update_phi` to perform meet operation on lattice values from all executable predecessors in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T085 [US3] Add phi node processing to `analyze` method worklist loop in `src/ir/optimizer/constant_folding/analyzer.rs` to re-evaluate phi nodes when predecessor blocks become executable
- [ ] T086 [US3] Implement `clean_phi_nodes(&self, function: &mut Function, executable_blocks: &HashSet<NodeIndex>)` in `src/ir/optimizer/constant_folding/transformer.rs` to remove dead predecessor entries from phi node operand lists
- [ ] T087 [US3] Implement physical removal of dead predecessor entries from phi nodes during transformation in `src/ir/optimizer/constant_folding/transformer.rs`
- [ ] T088 [US3] Add phi nodes cleaned count to verbose logging output in `SccpOptimizer::run` in `src/ir/optimizer/constant_folding/mod.rs`
- [ ] T089 [US3] Add phi nodes replaced count (phi results that became constant) to verbose logging output in `SccpOptimizer::run` in `src/ir/optimizer/constant_folding/mod.rs`

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently. SCCP can now propagate constants across control flow merges through phi nodes, discovering additional optimization opportunities.

---

## Phase 6: User Story 4 - Bitwise and Comparison Operation Folding (Priority: P3)

**Goal**: Extend constant folding to bitwise operations and comparisons beyond basic arithmetic. This allows SCCP to optimize a wider variety of constant expressions commonly found in real code.

**Independent Test**: Compile programs with constant bitwise operations (e.g., `x = 0xFF & 0x0F`) and comparisons (e.g., `b = 5 > 3`) and verify the optimized IR contains the constant results (15 and true respectively).

### Tests for User Story 4

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T090 [P] [US4] Create unit test for bitwise AND operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types
- [ ] T091 [P] [US4] Create unit test for bitwise OR operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types
- [ ] T092 [P] [US4] Create unit test for bitwise XOR operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types
- [ ] T093 [P] [US4] Create unit test for shift left operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` using wrapping_shl for all integer types
- [ ] T094 [P] [US4] Create unit test for shift right operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` using wrapping_shr for all integer types
- [ ] T095 [P] [US4] Create unit test for equality comparison constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T096 [P] [US4] Create unit test for inequality comparison constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T097 [P] [US4] Create unit test for less-than comparison constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T098 [P] [US4] Create unit test for less-than-or-equal comparison constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T099 [P] [US4] Create unit test for greater-than comparison constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T100 [P] [US4] Create unit test for greater-than-or-equal comparison constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T101 [P] [US4] Create unit test for unary negation operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for all signed numeric types using wrapping_neg
- [ ] T102 [P] [US4] Create unit test for logical NOT operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for Bool type
- [ ] T103 [P] [US4] Create unit test for logical AND operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for Bool type
- [ ] T104 [P] [US4] Create unit test for logical OR operation constant folding in `src/ir/optimizer/constant_folding/constant_folder.rs` for Bool type
- [ ] T105 [P] [US4] Create integration test in `tests/sccp_integration_tests.rs` for complex expressions combining arithmetic, bitwise, and comparison operations
- [ ] T106 [P] [US4] Create snapshot test in `tests/sccp_snapshot_tests.rs` for bitwise and comparison operation folding with before/after IR comparison

### Implementation for User Story 4

- [ ] T107 [P] [US4] Implement `fold_binary_bitwise_and` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types using & operator
- [ ] T108 [P] [US4] Implement `fold_binary_bitwise_or` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types using | operator
- [ ] T109 [P] [US4] Implement `fold_binary_bitwise_xor` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types using ^ operator
- [ ] T110 [P] [US4] Implement `fold_binary_shift_left` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types using wrapping_shl
- [ ] T111 [P] [US4] Implement `fold_binary_shift_right` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all integer types using wrapping_shr
- [ ] T112 [P] [US4] Implement `fold_comparison_eq` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all primitive types returning Bool
- [ ] T113 [P] [US4] Implement `fold_comparison_ne` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all primitive types returning Bool
- [ ] T114 [P] [US4] Implement `fold_comparison_lt` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T115 [P] [US4] Implement `fold_comparison_le` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T116 [P] [US4] Implement `fold_comparison_gt` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T117 [P] [US4] Implement `fold_comparison_ge` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for all numeric types returning Bool
- [ ] T118 [P] [US4] Implement `fold_logical_and` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for Bool type using && operator
- [ ] T119 [P] [US4] Implement `fold_logical_or` helper function in `src/ir/optimizer/constant_folding/constant_folder.rs` for Bool type using || operator
- [ ] T120 [US4] Implement `fold_unary(op: IrUnaryOp, operand: IrLiteralValue) -> Option<IrLiteralValue>` function in `src/ir/optimizer/constant_folding/constant_folder.rs` with pattern matching
- [ ] T121 [US4] Add unary negation cases to `fold_unary` for all signed numeric types using wrapping_neg in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T122 [US4] Add logical NOT case to `fold_unary` for Bool type in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T123 [US4] Extend `fold_binary` function in `src/ir/optimizer/constant_folding/constant_folder.rs` to include all bitwise operation cases calling helper functions
- [ ] T124 [US4] Extend `fold_binary` function in `src/ir/optimizer/constant_folding/constant_folder.rs` to include all comparison operation cases calling helper functions
- [ ] T125 [US4] Extend `fold_binary` function in `src/ir/optimizer/constant_folding/constant_folder.rs` to include logical AND and OR cases
- [ ] T126 [US4] Extend `evaluate_instruction` in `src/ir/optimizer/constant_folding/analyzer.rs` to handle unary operations by calling `constant_folder::fold_unary`
- [ ] T127 [US4] Add support for char type constant values in constant folding logic in `src/ir/optimizer/constant_folding/constant_folder.rs`

**Checkpoint**: All user stories 1-4 should now be independently functional. SCCP can optimize arithmetic, bitwise, comparison, and logical operations.

---

## Phase 7: User Story 5 - Conservative Analysis for Uncertain Operations (Priority: P2)

**Goal**: Ensure compiler correctness by conservatively marking operations whose results cannot be determined at compile time as variable. This prevents incorrect optimizations that would change program behavior.

**Independent Test**: Compile programs with function calls, memory loads, and division by zero, and verify these operations are conservatively marked as producing variable (non-constant) results. Verify no incorrect constant propagation occurs.

### Tests for User Story 5

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T128 [P] [US5] Create unit test verifying function call results marked as Bottom (variable) in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T129 [P] [US5] Create unit test verifying memory load results marked as Bottom (variable) in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T130 [P] [US5] Create unit test verifying division by zero returns None from fold_binary in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T131 [P] [US5] Create unit test verifying modulo by zero returns None from fold_binary in `src/ir/optimizer/constant_folding/constant_folder.rs`
- [ ] T132 [P] [US5] Create unit test verifying string values marked as Bottom in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T133 [P] [US5] Create unit test verifying array values marked as Bottom in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T134 [P] [US5] Create unit test verifying pointer values marked as Bottom in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T135 [P] [US5] Create integration test in `tests/sccp_integration_tests.rs` for program with function calls ensuring no interprocedural constant propagation
- [ ] T136 [P] [US5] Create integration test in `tests/sccp_integration_tests.rs` for program with memory loads ensuring conservative variable marking
- [ ] T137 [P] [US5] Create integration test in `tests/sccp_integration_tests.rs` verifying wrapping semantics for integer overflow/underflow matching Rust release mode
- [ ] T138 [P] [US5] Create integration test in `tests/sccp_integration_tests.rs` for conditional branch with non-constant condition ensuring both branches marked reachable

### Implementation for User Story 5

- [ ] T139 [P] [US5] Implement conservative handling for Call instructions in `evaluate_instruction` in `src/ir/optimizer/constant_folding/analyzer.rs` returning Bottom
- [ ] T140 [P] [US5] Implement conservative handling for Load instructions in `evaluate_instruction` in `src/ir/optimizer/constant_folding/analyzer.rs` returning Bottom
- [ ] T141 [P] [US5] Implement conservative handling for Store instructions in `evaluate_instruction` in `src/ir/optimizer/constant_folding/analyzer.rs` (no value produced)
- [ ] T142 [US5] Add type checking in `evaluate_instruction` to conservatively mark string types as Bottom in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T143 [US5] Add type checking in `evaluate_instruction` to conservatively mark array types as Bottom in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T144 [US5] Add type checking in `evaluate_instruction` to conservatively mark pointer types as Bottom in `src/ir/optimizer/constant_folding/analyzer.rs`
- [ ] T145 [US5] Verify all `wrapping_*` methods used throughout `constant_folder.rs` for integer arithmetic (add, sub, mul, div, neg, shl, shr)
- [ ] T146 [US5] Add documentation comments explaining conservative decisions in `src/ir/optimizer/constant_folding/analyzer.rs` for function calls, loads, and complex types
- [ ] T147 [US5] Add soundness verification test in `tests/sccp_integration_tests.rs` comparing optimized vs unoptimized execution results for identical behavior

**Checkpoint**: All user stories should now be independently functional. SCCP is complete with correct conservative handling ensuring optimization soundness.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories, final integration, and quality assurance

- [ ] T148 [P] Implement comprehensive rustdoc documentation for all public APIs in `src/ir/optimizer/constant_folding/mod.rs` with examples showing SccpOptimizer usage
- [ ] T149 [P] Implement rustdoc documentation for lattice module in `src/ir/optimizer/constant_folding/lattice.rs` explaining lattice theory and meet operation
- [ ] T150 [P] Implement rustdoc documentation for constant_folder module in `src/ir/optimizer/constant_folding/constant_folder.rs` explaining folding semantics and wrapping behavior
- [ ] T151 [P] Implement rustdoc documentation for analyzer module in `src/ir/optimizer/constant_folding/analyzer.rs` explaining Wegman-Zadeck algorithm
- [ ] T152 [P] Implement rustdoc documentation for transformer module in `src/ir/optimizer/constant_folding/transformer.rs` explaining IR mutation strategy
- [ ] T153 [P] Implement rustdoc documentation for worklist module in `src/ir/optimizer/constant_folding/worklist.rs` explaining sparse analysis
- [ ] T154 Implement performance benchmark in `benches/sccp_benchmark.rs` for 1,000-instruction functions measuring analysis time
- [ ] T155 Implement performance benchmark in `benches/sccp_benchmark.rs` for 10,000-instruction functions verifying <100ms completion time
- [ ] T156 [P] Implement benchmark for constant-heavy programs in `benches/sccp_benchmark.rs` measuring optimization impact
- [ ] T157 [P] Create integration test for SCCP+DCE alternating phases in `tests/sccp_integration_tests.rs` verifying fixed-point convergence within 3 iterations
- [ ] T158 Add edge case test for infinite loop with no exit in `tests/sccp_integration_tests.rs` verifying analysis terminates via iteration limit
- [ ] T159 Add edge case test for unreachable entry block in `tests/sccp_integration_tests.rs` verifying graceful handling with warning
- [ ] T160 Add edge case test for phi node with zero executable predecessors in `tests/sccp_integration_tests.rs` verifying phi stays Unknown (Top)
- [ ] T161 Implement verbose logging for analysis iterations count in `src/ir/optimizer/constant_folding/mod.rs`
- [ ] T162 Implement verbose logging for convergence status in `src/ir/optimizer/constant_folding/mod.rs`
- [ ] T163 [P] Add integration with existing DeadCodeElimination phase demonstrating SCCP-DCE pipeline in `tests/sccp_integration_tests.rs`
- [ ] T164 [P] Create example demonstrating SccpOptimizer usage in documentation or examples directory
- [ ] T165 Run `cargo fmt` on all SCCP module files to ensure consistent code formatting
- [ ] T166 Run `cargo clippy -- -D warnings` on SCCP module and fix all clippy warnings
- [ ] T167 Run full test suite with `cargo test` and verify all tests pass
- [ ] T168 Run benchmarks with `cargo bench` and verify performance targets met (<100ms for 10K instructions)
- [ ] T169 Verify snapshot tests with `cargo insta test` and review all snapshots for correctness
- [ ] T170 [P] Update project README.md or CHANGELOG.md to document SCCP optimizer feature
- [ ] T171 Run quickstart.md validation by following the development roadmap and verifying all steps work correctly
- [ ] T172 Perform code review of all SCCP modules ensuring adherence to Rust best practices and project conventions
- [ ] T173 Verify constitutional compliance: Safety (no unsafe code), Performance (O(n) complexity), Cross-platform (pure Rust), Modularity (six clear modules)
- [ ] T174 Create pull request from `017-sccp-optimizer` branch to `main` with comprehensive description

**Final Checkpoint**: Complete SCCP optimization phase ready for production use, fully tested, documented, and integrated.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) AND User Story 1 (requires constant evaluation and lattice infrastructure)
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2) AND User Story 2 (requires block reachability analysis for phi nodes)
- **User Story 4 (Phase 6)**: Depends on Foundational (Phase 2) AND User Story 1 (extends constant folding infrastructure)
- **User Story 5 (Phase 7)**: Depends on Foundational (Phase 2) AND User Story 1 (extends evaluation logic with conservative cases)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Foundation for all constant propagation - must complete first
- **User Story 2 (P1)**: Builds on US1's constant evaluation to analyze branches
- **User Story 3 (P2)**: Requires US2's reachability analysis to correctly evaluate phi nodes
- **User Story 4 (P3)**: Extends US1's constant folding, can be implemented after US1
- **User Story 5 (P2)**: Extends US1's evaluation logic, can be implemented after US1

**Recommended Sequence**: Setup → Foundational → US1 → US2 → US3 → US4 (parallel with US5) → US5 → Polish

### Within Each User Story

1. **Tests FIRST** - All tests for a user story MUST be written and FAIL before implementation begins
2. **Helper functions** - Implement small, focused helper functions first (constant folding helpers)
3. **Core logic** - Implement main algorithms (analyze, evaluate, transform)
4. **Integration** - Wire up to public API and Phase trait
5. **Logging and observability** - Add verbose output and statistics

### Parallel Opportunities

**Setup Phase (Phase 1)**:
- T003 and T004 can run in parallel (different files)

**Foundational Phase (Phase 2)**:
- T007, T008, T009 can run in parallel (lattice tests, different test cases)
- T013 can run in parallel with lattice work (worklist tests)
- T019, T020, T021, T022 can run in parallel (stub implementations, different files)

**User Story 1 Tests**:
- T023-T031 can all run in parallel (different test files/cases)
- T032-T035 can all run in parallel (different integration/snapshot tests)

**User Story 1 Implementation**:
- T036-T040 can all run in parallel (different helper functions, no dependencies)

**User Story 2 Tests**:
- T057-T064 can all run in parallel (different test cases)

**User Story 2 Implementation**:
- T065-T066 can run in parallel (different helper functions)

**User Story 3 Tests**:
- T075-T081 can all run in parallel (different test cases)

**User Story 3 Implementation**:
- T082-T084 are sequential (phi update logic)
- T088-T089 can run in parallel (different logging additions)

**User Story 4 Tests**:
- T090-T106 can all run in parallel (different test cases, different files)

**User Story 4 Implementation**:
- T107-T119 can all run in parallel (different helper functions)
- T123-T125 are sequential (extending same fold_binary function)

**User Story 5 Tests**:
- T128-T138 can all run in parallel (different test cases)

**User Story 5 Implementation**:
- T139-T141 can run in parallel (different instruction types)
- T142-T144 can run in parallel (different type checks)

**Polish Phase (Phase 8)**:
- T148-T153 can all run in parallel (different file documentation)
- T154-T156 can all run in parallel (different benchmarks)
- T158-T160 can all run in parallel (different edge case tests)
- T170 can run in parallel with other polish tasks

---

## Parallel Example: User Story 1

```bash
# Write all tests for User Story 1 together (TDD approach):
Parallel Task Group 1 (US1 Tests):
- T023: "Unit test for signed integer addition constant folding"
- T024: "Unit test for unsigned integer addition constant folding"
- T025: "Unit test for floating-point addition constant folding"
- T026: "Unit test for signed integer subtraction constant folding"
- T027: "Unit test for unsigned integer subtraction constant folding"
- T028: "Unit test for integer multiplication constant folding"
- T029: "Unit test for integer division constant folding"
- T030: "Unit test for integer modulo constant folding"
- T031: "Unit test for type mismatch scenarios"
- T032: "Integration test for direct constant assignment"
- T033: "Integration test for constant arithmetic propagation"
- T034: "Integration test for chained constant propagation"
- T035: "Snapshot test for constant expression simplification"

# Implement all helper functions for constant folding in parallel:
Parallel Task Group 2 (US1 Helpers):
- T036: "Implement fold_binary_add helper"
- T037: "Implement fold_binary_subtract helper"
- T038: "Implement fold_binary_multiply helper"
- T039: "Implement fold_binary_divide helper"
- T040: "Implement fold_binary_modulo helper"
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only - Core SCCP)

This provides the essential SCCP optimization capability:

1. **Phase 1: Setup** - Project structure (T001-T004)
2. **Phase 2: Foundational** - Core data structures (T005-T022)
3. **Phase 3: User Story 1** - Constant propagation (T023-T056)
4. **Phase 4: User Story 2** - Unreachable code detection (T057-T074)
5. **STOP and VALIDATE**: Test User Stories 1 + 2 together with snapshot tests
6. Run benchmarks to verify performance targets
7. Deploy/demo if ready

**MVP Deliverable**: Functional SCCP optimizer that discovers constants and identifies unreachable code, ready for integration into compiler pipeline.

### Incremental Delivery

1. **Foundation** (Phases 1-2): Setup + Core data structures → Foundation ready (T001-T022)
2. **MVP** (Phases 3-4): User Stories 1 + 2 → Core SCCP working (T023-T074)
3. **Enhanced** (Phase 5): Add User Story 3 → Phi node optimization (T075-T089)
4. **Full Featured** (Phases 6-7): Add User Stories 4 + 5 → Complete operation coverage (T090-T147)
5. **Production Ready** (Phase 8): Polish and cross-cutting concerns (T148-T174)

Each increment adds value without breaking previous capabilities.

### Parallel Team Strategy

With multiple developers:

1. **Team completes Foundation together** (Phases 1-2)
2. **Once Foundational phase done**:
   - Developer A: User Story 1 (Constant propagation core)
   - Developer B: Write all tests for User Stories 2-5
   - Developer C: Documentation and benchmark infrastructure
3. **After User Story 1 completes**:
   - Developer A: User Story 2 (Control flow analysis)
   - Developer B: User Story 4 (Bitwise/comparison ops)
   - Developer C: User Story 5 (Conservative analysis)
4. **After User Story 2 completes**:
   - Developer A: User Story 3 (Phi nodes)
5. **Final integration**: All developers on Phase 8 (Polish)

---

## Task Count Summary

- **Total Tasks**: 174
- **Phase 1 (Setup)**: 4 tasks
- **Phase 2 (Foundational)**: 18 tasks
- **Phase 3 (User Story 1 - P1)**: 34 tasks (13 tests + 21 implementation)
- **Phase 4 (User Story 2 - P1)**: 18 tasks (8 tests + 10 implementation)
- **Phase 5 (User Story 3 - P2)**: 15 tasks (7 tests + 8 implementation)
- **Phase 6 (User Story 4 - P3)**: 38 tasks (17 tests + 21 implementation)
- **Phase 7 (User Story 5 - P2)**: 20 tasks (11 tests + 9 implementation)
- **Phase 8 (Polish)**: 27 tasks

**Parallel Opportunities**: 98 tasks marked with [P] can be executed in parallel within their phase constraints

**MVP Task Count**: 74 tasks (Phases 1-4 = Setup + Foundational + US1 + US2)

**Test Coverage**: 56 test tasks ensuring comprehensive quality assurance

---

## Notes

- All tasks follow strict checklist format: `- [ ] [ID] [P?] [Story?] Description with file path`
- [P] marker indicates parallelizable tasks (different files, no dependencies on incomplete work)
- [Story] label (US1-US5) maps tasks to specific user stories for traceability
- Each user story is independently completable and testable
- Tests are written FIRST following TDD principles (all test tasks come before implementation tasks)
- File paths are absolute and specific to jsavrs project structure
- Wrapping semantics used throughout for Rust release-mode compatibility
- Conservative analysis ensures optimization soundness (never incorrect)
- Integration with existing Phase trait and DCE optimizer
- Performance target: <100ms for 10,000-instruction functions
- Complexity target: O(n) where n = SSA values + CFG edges
- Zero new dependencies (uses existing petgraph, thiserror, insta, criterion)

---

**Format Validation**: ✅ All 174 tasks follow checklist format with checkbox, Task ID, appropriate labels ([P] and [Story]), and file paths in descriptions.
