# Tasks: Dead Code Elimination (DCE) Optimization

**Input**: Design documents from `/specs/013-dce/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, quickstart.md ✅

**Tests**: All test tasks are included as this is a compiler optimization requiring comprehensive testing.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each optimization capability.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and test infrastructure setup

- [ ] T001 Verify Rust project structure and dependencies in Cargo.toml (petgraph 0.8.3, insta 1.43.2, criterion 0.7.0)
- [ ] T002 Create src/ir/optimizer/dead_code_elimination.rs module stub with module exports in src/ir/optimizer/mod.rs
- [ ] T003 [P] Create test file stubs: tests/ir_dce_reachability_tests.rs, tests/ir_dce_liveness_tests.rs, tests/ir_dce_escape_tests.rs, tests/ir_dce_integration_tests.rs, tests/ir_dce_snapshot_tests.rs
- [ ] T004 [P] Configure rustfmt and clippy for DCE module (verify rustfmt.toml settings)

---

## Phase 2: Foundational (Core Data Structures)

**Purpose**: Core data structures that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Implement OptimizationStats struct in src/ir/optimizer/dead_code_elimination.rs with fields: instructions_removed, blocks_removed, iterations, conservative_warnings
- [ ] T006 [P] Implement ConservativeWarning struct in src/ir/optimizer/dead_code_elimination.rs with fields: instruction_debug, reason, block_label
- [ ] T007 [P] Implement ConservativeReason enum in src/ir/optimizer/dead_code_elimination.rs with variants: MayAlias, UnknownCallPurity, EscapedPointer, PotentialSideEffect
- [ ] T008 Implement InstructionIndex struct in src/ir/optimizer/dead_code_elimination.rs with fields: block_idx (NodeIndex), inst_offset (usize)
- [ ] T009 [P] Implement Display and Debug traits for OptimizationStats, ConservativeWarning, ConservativeReason, InstructionIndex
- [ ] T010 Implement DeadCodeElimination main struct in src/ir/optimizer/dead_code_elimination.rs with fields: max_iterations, enable_statistics, verbose_warnings
- [ ] T011 Implement Default trait for DeadCodeElimination with max_iterations=10, enable_statistics=true, verbose_warnings=false
- [ ] T012 Implement DeadCodeElimination::new() and DeadCodeElimination::with_config() constructors with validation (max_iterations > 0)
- [ ] T013 Implement Phase trait for DeadCodeElimination with name() returning "Dead Code Elimination" and run() stub
- [ ] T014 [P] Add unit tests for OptimizationStats in tests/ir_dce_integration_tests.rs (new(), had_effect(), format_report())
- [ ] T015 [P] Add unit tests for ConservativeWarning and ConservativeReason in tests/ir_dce_integration_tests.rs
- [ ] T016 [P] Add unit tests for InstructionIndex in tests/ir_dce_integration_tests.rs (new(), ordering, display)

**Checkpoint**: Foundation ready - core data structures complete and tested

---

## Phase 3: User Story 1 - Remove Unreachable Code Blocks (Priority: P1) 🎯 MVP

**Goal**: Identify and remove basic blocks that are unreachable from the function entry point

**Independent Test**: Compile functions with code after return statements, impossible branches, verify blocks are removed from IR

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T017 [P] [US1] Create test for unreachable code after unconditional return in tests/ir_dce_reachability_tests.rs (FR-001, FR-002, SC-001)
- [ ] T018 [P] [US1] Create test for impossible if-branch (constant false condition) in tests/ir_dce_reachability_tests.rs (FR-001, FR-002, SC-001)
- [ ] T019 [P] [US1] Create test for unreachable switch case blocks in tests/ir_dce_reachability_tests.rs (FR-001, FR-002, SC-001)
- [ ] T020 [P] [US1] Create test for code after infinite loop in tests/ir_dce_reachability_tests.rs (FR-001, FR-002, SC-001)
- [ ] T021 [P] [US1] Create snapshot test for CFG before/after unreachable block removal in tests/ir_dce_snapshot_tests.rs

### Implementation for User Story 1

- [ ] T022 [US1] Implement ReachabilityAnalyzer struct (private) in src/ir/optimizer/dead_code_elimination.rs
- [ ] T023 [US1] Implement ReachabilityAnalyzer::analyze() using petgraph::visit::Dfs to compute HashSet<NodeIndex> of reachable blocks (FR-001, algorithm from research.md)
- [ ] T024 [US1] Implement ReachabilityAnalyzer edge case handling: no entry block (error), single block (no-op), loops and cycles (visited tracking)
- [ ] T025 [US1] Implement block removal logic in DeadCodeElimination::optimize_function() to remove unreachable blocks from CFG (FR-002, FR-011)
- [ ] T026 [US1] Implement CFG edge updates when removing blocks (update predecessors, update phi node incoming lists) (FR-011, FR-012)
- [ ] T027 [US1] Implement SSA form preservation verification after block removal (FR-013)
- [ ] T028 [US1] Add debug information and source location metadata preservation during block removal (FR-014)
- [ ] T029 [US1] Add statistics tracking for blocks_removed in OptimizationStats
- [ ] T030 [US1] Verify all US1 tests pass (T017-T021)

**Checkpoint**: Unreachable code block removal is fully functional and independently testable

---

## Phase 4: User Story 2 - Eliminate Dead Instructions (Priority: P2)

**Goal**: Remove instructions whose computed values are never used

**Independent Test**: Compile functions with unused temporaries, verify instructions computing unused values are removed

### Tests for User Story 2

- [ ] T031 [P] [US2] Create test for unused temporary variable computation in tests/ir_dce_liveness_tests.rs (FR-003, FR-004, FR-005, SC-002)
- [ ] T032 [P] [US2] Create test for chain of computations with only final result used in tests/ir_dce_liveness_tests.rs (FR-003, FR-004)
- [ ] T033 [P] [US2] Create test for multiple dead definitions of same variable in tests/ir_dce_liveness_tests.rs (FR-003, FR-004, FR-005, SC-002)
- [ ] T034 [P] [US2] Create test for unused phi node removal in tests/ir_dce_liveness_tests.rs (FR-009, SC-002)
- [ ] T035 [P] [US2] Create snapshot test for IR before/after dead instruction removal in tests/ir_dce_snapshot_tests.rs

### Implementation for User Story 2

- [ ] T036 [P] [US2] Implement DefUseChains struct (private) in src/ir/optimizer/dead_code_elimination.rs with fields: value_to_uses, instruction_to_used_values, instruction_to_defined_value
- [ ] T037 [P] [US2] Implement LivenessInfo struct (private) in src/ir/optimizer/dead_code_elimination.rs with fields: first_use, last_use, used_in_blocks
- [ ] T038 [US2] Implement DefUseChains methods: new(), add_definition(), add_use(), get_uses(), get_used_values(), get_defined_value(), has_uses()
- [ ] T039 [US2] Implement LivenessInfo methods: dead(), with_uses(), is_live()
- [ ] T040 [US2] Implement LivenessAnalyzer struct (private) in src/ir/optimizer/dead_code_elimination.rs with field: def_use_chains
- [ ] T041 [US2] Implement LivenessAnalyzer::build_def_use_chains() to scan all instructions and build bidirectional def-use mappings (FR-004)
- [ ] T042 [US2] Implement LivenessAnalyzer::compute_gen_kill_sets() for each block (gen = values used before defined, kill = values defined) (algorithm from research.md)
- [ ] T043 [US2] Implement LivenessAnalyzer::analyze() with backward dataflow fixed-point iteration: live_in[B] = gen[B] ∪ (live_out[B] - kill[B]) (FR-003, algorithm from research.md)
- [ ] T044 [US2] Implement reverse post-order block processing in LivenessAnalyzer for convergence optimization (research.md optimization)
- [ ] T045 [US2] Implement phi node special handling in liveness analysis (phi live if result used, mark incoming values live at predecessor exits) (FR-009)
- [ ] T046 [US2] Implement convergence detection and maximum iteration limit (10) with warning (research.md)
- [ ] T047 [US2] Implement dead instruction removal logic in DeadCodeElimination::optimize_function() based on liveness results (FR-005)
- [ ] T048 [US2] Implement pure instruction classification and removal when result unused (FR-006, research.md SideEffectClass::Pure)
- [ ] T049 [US2] Add statistics tracking for instructions_removed in OptimizationStats
- [ ] T050 [US2] Verify all US2 tests pass (T031-T035)

**Checkpoint**: Dead instruction elimination is fully functional and independently testable

---

## Phase 5: User Story 3 - Optimize Memory Operations Safely (Priority: P3)

**Goal**: Remove unnecessary stores and loads while preserving observable program behavior

**Independent Test**: Compile functions with dead stores/loads to locals, verify removal; ensure stores to escaped pointers are preserved

### Tests for User Story 3

- [ ] T051 [P] [US3] Create test for dead store to local variable in tests/ir_dce_escape_tests.rs (FR-007, FR-019, SC-002)
- [ ] T052 [P] [US3] Create test for dead load from local variable in tests/ir_dce_escape_tests.rs (FR-007, FR-019, SC-002)
- [ ] T053 [P] [US3] Create test for store to potentially-aliased pointer (must preserve) in tests/ir_dce_escape_tests.rs (FR-007, FR-008, SC-002)
- [ ] T054 [P] [US3] Create test for alloca with no loads and non-escaped address in tests/ir_dce_escape_tests.rs (FR-019, SC-002)
- [ ] T055 [P] [US3] Create snapshot test for memory operation optimization in tests/ir_dce_snapshot_tests.rs

### Implementation for User Story 3

- [ ] T056 [P] [US3] Implement EscapeStatus enum in src/ir/optimizer/dead_code_elimination.rs with variants: Local, AddressTaken, Escaped
- [ ] T057 [US3] Implement EscapeAnalyzer struct (private) in src/ir/optimizer/dead_code_elimination.rs
- [ ] T058 [US3] Implement EscapeAnalyzer::analyze() to compute HashMap<ValueId, EscapeStatus> (FR-019, algorithm from research.md)
- [ ] T059 [US3] Implement escape detection for stores: if storing alloca pointer, mark Escaped (research.md escape conditions)
- [ ] T060 [US3] Implement escape detection for function calls: if passing alloca as argument, mark Escaped (research.md escape conditions)
- [ ] T061 [US3] Implement escape detection for returns: if returning alloca pointer, mark Escaped (research.md escape conditions)
- [ ] T062 [US3] Implement escape detection for GetElementPtr: if GEP of alloca, mark AddressTaken (research.md escape conditions)
- [ ] T063 [US3] Implement conservative defaults: function parameters assumed escaped, loaded pointers assumed escaped (research.md)
- [ ] T064 [US3] Implement SideEffectClass enum in src/ir/optimizer/dead_code_elimination.rs with variants: Pure, MemoryRead, MemoryWrite, EffectFul
- [ ] T065 [US3] Implement SideEffectClass::classify() to categorize instructions based on effects and escape info (FR-006, research.md classification rules)
- [ ] T066 [US3] Implement memory operation removal logic: remove stores to Local allocations if no subsequent loads (FR-007)
- [ ] T067 [US3] Implement memory operation removal logic: remove loads if result unused (MemoryRead class) (FR-007)
- [ ] T068 [US3] Implement conservative preservation of stores to Escaped or AddressTaken allocations (FR-007, FR-008)
- [ ] T069 [US3] Implement conservative preservation of function calls with unknown purity (FR-008)
- [ ] T070 [US3] Add conservative warning generation for preserved memory operations (FR-016, verbose_warnings setting)
- [ ] T071 [US3] Verify all US3 tests pass (T051-T055)

**Checkpoint**: Memory operation optimization is fully functional with correct conservative behavior

---

## Phase 6: User Story 4 - Iterative Fixed-Point Optimization (Priority: P4)

**Goal**: Apply multiple passes until no further improvements possible, maximizing code reduction

**Independent Test**: Compile functions with cascading dead code, verify all transitively-dead code is removed

### Tests for User Story 4

- [ ] T072 [P] [US4] Create test for cascading dead code (removing one instruction makes another dead) in tests/ir_dce_integration_tests.rs (FR-010, SC-003, SC-008)
- [ ] T073 [P] [US4] Create test for empty block removal after instruction elimination in tests/ir_dce_integration_tests.rs (FR-010, FR-011, SC-003)
- [ ] T074 [P] [US4] Create test for dead phi node causing predecessor definition to become dead in tests/ir_dce_integration_tests.rs (FR-010, SC-003)
- [ ] T075 [P] [US4] Create test for fixed-point in single iteration (no dead code) in tests/ir_dce_integration_tests.rs (FR-010, SC-008)
- [ ] T076 [P] [US4] Create snapshot test for multi-iteration optimization in tests/ir_dce_snapshot_tests.rs

### Implementation for User Story 4

- [ ] T077 [US4] Implement fixed-point iteration loop in DeadCodeElimination::optimize_function() (FR-010)
- [ ] T078 [US4] Implement iteration logic: repeat reachability → block removal → liveness → instruction removal until no changes (FR-010, algorithm from research.md)
- [ ] T079 [US4] Implement change detection: track whether blocks or instructions were removed in each iteration (FR-010)
- [ ] T080 [US4] Implement convergence detection: fixed-point reached when no changes occur (FR-010, SC-008)
- [ ] T081 [US4] Implement maximum iteration enforcement (max_iterations setting) with warning if exceeded (FR-010, research.md)
- [ ] T082 [US4] Implement empty block detection and removal when all instructions eliminated (FR-010, FR-011)
- [ ] T083 [US4] Implement CFG edge bypass for empty blocks (update predecessors to skip removed block) (FR-011)
- [ ] T084 [US4] Implement iteration count tracking in OptimizationStats (FR-016, SC-003)
- [ ] T085 [US4] Add performance measurement: ensure 10k instruction function completes in <1s (SC-004)
- [ ] T086 [US4] Verify all US4 tests pass (T072-T076)

**Checkpoint**: Fixed-point iteration is fully functional, maximizing optimization effectiveness

---

## Phase 7: Module-Level Integration

**Purpose**: Integrate DCE with module-level optimization pipeline

- [ ] T087 Implement DeadCodeElimination::run(&mut module) to iterate over all functions (FR-017, FR-018)
- [ ] T088 Implement external function declaration skip logic (functions without bodies) (FR-018)
- [ ] T089 Implement per-function statistics collection and aggregation
- [ ] T090 Implement module-level statistics reporting with format_report() (FR-016)
- [ ] T091 [P] Add integration test for multi-function module optimization in tests/ir_dce_integration_tests.rs
- [ ] T092 [P] Add integration test for module with mix of internal and external functions in tests/ir_dce_integration_tests.rs
- [ ] T093 Verify CFG integrity after optimization for all functions in module (FR-013, SC-009)

---

## Phase 8: Terminator Handling

**Purpose**: Correct handling of all terminator kinds

- [ ] T094 [P] Implement Return terminator handling in reachability analysis (FR-020)
- [ ] T095 [P] Implement Branch terminator handling in reachability analysis (FR-020)
- [ ] T096 [P] Implement ConditionalBranch terminator handling in reachability analysis (FR-020)
- [ ] T097 [P] Implement Switch terminator handling in reachability analysis (FR-020)
- [ ] T098 [P] Implement IndirectBranch terminator handling (conservative - assume all targets reachable) (FR-020)
- [ ] T099 [P] Implement Unreachable terminator handling (successors are unreachable) (FR-020)
- [ ] T100 [P] Add test for each terminator kind in tests/ir_dce_reachability_tests.rs
- [ ] T101 [P] Add snapshot test for switch statement optimization in tests/ir_dce_snapshot_tests.rs

---

## Phase 9: Edge Case Handling

**Purpose**: Robust handling of unusual program structures

- [ ] T102 [P] Add test and handling for function with entirely dead code (all unreachable) in tests/ir_dce_integration_tests.rs
- [ ] T103 [P] Add test and handling for circular phi node dependencies in unreachable blocks in tests/ir_dce_integration_tests.rs
- [ ] T104 [P] Add test and handling for phi nodes when all predecessors removed in tests/ir_dce_integration_tests.rs
- [ ] T105 [P] Add test for function calls with unused return values in tests/ir_dce_liveness_tests.rs (FR-008)
- [ ] T106 [P] Add test for complex pointer computation (GEP chain) in tests/ir_dce_escape_tests.rs
- [ ] T107 [P] Add test for indirect branch with computed targets in tests/ir_dce_reachability_tests.rs
- [ ] T108 [P] Add test for debug information preservation after optimization in tests/ir_dce_integration_tests.rs (FR-014)
- [ ] T109 [P] Add test for scope boundary preservation in tests/ir_dce_integration_tests.rs (FR-015)
- [ ] T110 [P] Implement volatile/atomic operation preservation (always EffectFul) in SideEffectClass::classify() (FR-006)
- [ ] T111 Add comprehensive edge case snapshot tests in tests/ir_dce_snapshot_tests.rs

---

## Phase 10: Performance Benchmarking

**Purpose**: Validate performance goals

- [ ] T112 Create benchmark for small function (<100 instructions) in benches/jsavrs_benchmark.rs
- [ ] T113 Create benchmark for medium function (1000 instructions) in benches/jsavrs_benchmark.rs
- [ ] T114 Create benchmark for large function (10000 instructions) in benches/jsavrs_benchmark.rs
- [ ] T115 Create benchmark for module with multiple functions in benches/jsavrs_benchmark.rs
- [ ] T116 Create benchmark for worst-case iteration count (deep nesting) in benches/jsavrs_benchmark.rs
- [ ] T117 Verify SC-004: 10k instruction function completes in <1s on modern hardware
- [ ] T118 Verify SC-003: typical functions reach fixed-point within 5 iterations
- [ ] T119 Run criterion benchmarks and document baseline performance in quickstart.md

---

## Phase 11: Statistics and Diagnostics

**Purpose**: Comprehensive reporting and debugging support

- [ ] T120 Implement OptimizationStats::format_report() with human-readable output including emojis (quickstart.md format)
- [ ] T121 Implement verbose warning generation when verbose_warnings=true (FR-016, SC-011)
- [ ] T122 Implement conservative warning for MayAlias scenario with instruction debug info (FR-016, SC-011)
- [ ] T123 Implement conservative warning for UnknownCallPurity scenario with instruction debug info (FR-016, SC-011)
- [ ] T124 Implement conservative warning for EscapedPointer scenario with instruction debug info (FR-016, SC-011)
- [ ] T125 Implement conservative warning for PotentialSideEffect scenario with instruction debug info (FR-016, SC-011)
- [ ] T126 Add test for statistics accuracy (reported counts match actual removals) in tests/ir_dce_integration_tests.rs (SC-007)
- [ ] T127 Add test for warning generation in verbose mode in tests/ir_dce_integration_tests.rs (SC-011)

---

## Phase 12: Documentation

**Purpose**: Complete rustdoc documentation for public APIs

- [ ] T128 Add module-level rustdoc documentation for dead_code_elimination.rs with overview, algorithm summary, example usage
- [ ] T129 Add complete rustdoc for DeadCodeElimination struct with fields, methods, examples (data-model.md format)
- [ ] T130 Add complete rustdoc for OptimizationStats struct with fields, methods, examples (data-model.md format)
- [ ] T131 Add complete rustdoc for ConservativeWarning struct with fields, methods, examples (data-model.md format)
- [ ] T132 Add complete rustdoc for ConservativeReason enum with variants, explanations (data-model.md format)
- [ ] T133 [P] Add rustdoc examples showing basic usage (quickstart.md examples)
- [ ] T134 [P] Add rustdoc examples showing custom configuration (quickstart.md examples)
- [ ] T135 [P] Add rustdoc examples showing pipeline integration (quickstart.md examples)
- [ ] T136 Verify all public APIs have documentation with cargo doc --no-deps --document-private-items
- [ ] T137 Update README.md with DCE phase description and usage example
- [ ] T138 Update QWEN.md with DCE implementation details and architecture notes

---

## Phase 13: Polish & Cross-Cutting Concerns

**Purpose**: Final quality improvements affecting the entire implementation

- [ ] T139 [P] Run rustfmt on all DCE source files
- [ ] T140 [P] Run clippy on DCE module and fix all warnings
- [ ] T141 Code review and refactoring for clarity and maintainability
- [ ] T142 Verify all FR requirements are satisfied (FR-001 through FR-020)
- [ ] T143 Verify all SC success criteria are met (SC-001 through SC-011)
- [ ] T144 Run full test suite: cargo test --all-features
- [ ] T145 Run snapshot tests and update snapshots if needed: cargo insta test
- [ ] T146 Run benchmarks and verify performance targets: cargo bench
- [ ] T147 Verify existing compiler test suites pass with DCE enabled (SC-010)
- [ ] T148 Run quickstart.md validation examples and verify output matches expectations
- [ ] T149 Security review: verify no unsafe code, no panics in production paths
- [ ] T150 Final CFG integrity verification across all test cases (SC-009)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies - can start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 completion - BLOCKS all user stories
- **Phase 3 (US1)**: Depends on Phase 2 completion - MVP deliverable
- **Phase 4 (US2)**: Depends on Phase 2 completion - Can start in parallel with US1 (different algorithms)
- **Phase 5 (US3)**: Depends on Phase 2 and Phase 4 completion (needs liveness analysis)
- **Phase 6 (US4)**: Depends on Phases 3, 4, 5 completion (orchestrates all analyses)
- **Phase 7 (Integration)**: Depends on Phase 6 completion (needs complete optimization function)
- **Phase 8 (Terminators)**: Depends on Phase 3 completion (extends reachability analysis) - Can run in parallel with Phases 4-6
- **Phase 9 (Edge Cases)**: Depends on Phases 3-8 completion (tests all functionality)
- **Phase 10 (Benchmarks)**: Depends on Phase 7 completion (needs complete module optimization)
- **Phase 11 (Diagnostics)**: Depends on Phases 3-6 completion - Can run in parallel with Phases 7-10
- **Phase 12 (Documentation)**: Depends on implementation completion (Phases 3-11)
- **Phase 13 (Polish)**: Depends on all previous phases

### User Story Dependencies

- **User Story 1 (P1 - Unreachable Blocks)**: Can start after Phase 2 - No dependencies on other stories - **MVP TARGET**
- **User Story 2 (P2 - Dead Instructions)**: Can start after Phase 2 - Independent of US1 (different analysis) - Can run in parallel
- **User Story 3 (P3 - Memory Ops)**: Depends on US2 completion (needs liveness analysis infrastructure)
- **User Story 4 (P4 - Fixed-Point)**: Depends on US1, US2, US3 completion (orchestrates all optimizations)

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD approach)
- Core analyzers before optimization logic
- Helper structs/enums before main algorithms
- Statistics tracking after core functionality
- All tests passing before story marked complete

### Parallel Opportunities

**Phase 1 (Setup)**:
- T003 (test stubs), T004 (formatting) can run in parallel with T001-T002

**Phase 2 (Foundational)**:
- T006-T007 (ConservativeWarning, ConservativeReason) parallel with T005 (OptimizationStats)
- T009 (trait impls) parallel with T008 (InstructionIndex)
- T014-T016 (unit tests) can all run in parallel

**Phase 3 (US1 - Unreachable Blocks)**:
- T017-T021 (all tests) can run in parallel - WRITE FIRST
- After tests exist: T024 (edge cases) parallel with T023 (main algorithm)
- T027-T028 (verification, debug info) parallel with T029 (statistics)

**Phase 4 (US2 - Dead Instructions)**:
- T031-T035 (all tests) can run in parallel - WRITE FIRST
- T036-T037 (DefUseChains, LivenessInfo) parallel implementation
- T044 (optimization) parallel with T045 (phi handling) during implementation

**Phase 5 (US3 - Memory Ops)**:
- T051-T055 (all tests) can run in parallel - WRITE FIRST
- T056-T057 (EscapeStatus, EscapeAnalyzer) parallel with T064 (SideEffectClass)
- T059-T062 (escape detection cases) can run in parallel

**Phase 8 (Terminators)**:
- T094-T099 (all terminator handlers) can run in parallel
- T100-T101 (tests) can run in parallel

**Phase 9 (Edge Cases)**:
- T102-T109 (all edge case tests) can run in parallel

**Phase 10 (Benchmarks)**:
- T112-T116 (all benchmarks) can run in parallel

**Phase 11 (Diagnostics)**:
- T122-T125 (warning generation) can run in parallel
- T126-T127 (diagnostic tests) parallel

**Phase 12 (Documentation)**:
- T128-T135 (all rustdoc) can run in parallel after implementation complete

**Phase 13 (Polish)**:
- T139-T140 (rustfmt, clippy) parallel
- T144-T146 (test suite, snapshots, benchmarks) parallel
- T148-T150 (validation, security, verification) parallel

---

## Parallel Example: User Story 1 (Unreachable Blocks)

```bash
# Step 1: Write all tests in parallel (TDD - tests first!)
Task T017: "Create test for unreachable code after return"
Task T018: "Create test for impossible if-branch"  
Task T019: "Create test for unreachable switch cases"
Task T020: "Create test for code after infinite loop"
Task T021: "Create snapshot test for CFG changes"

# Step 2: Verify all tests FAIL (expected - no implementation yet)

# Step 3: Core implementation (sequential - dependencies)
Task T022: "Implement ReachabilityAnalyzer struct"
Task T023: "Implement ReachabilityAnalyzer::analyze() with DFS"

# Step 4: Additional features in parallel
Task T024: "Edge case handling" (parallel with)
Task T027: "SSA verification" (parallel with)  
Task T028: "Debug info preservation"

# Step 5: Integration (sequential)
Task T025: "Implement block removal logic"
Task T026: "Implement CFG edge updates"
Task T029: "Add statistics tracking"

# Step 6: Verify all tests PASS
Task T030: "Verify all US1 tests pass"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. **Complete Phase 1**: Setup (T001-T004) - ~1 hour
2. **Complete Phase 2**: Foundational data structures (T005-T016) - ~4 hours  
3. **Complete Phase 3**: User Story 1 - Unreachable block removal (T017-T030) - ~8 hours
4. **STOP and VALIDATE**: 
   - Run all US1 tests (T030)
   - Verify CFG integrity
   - Benchmark performance on small functions
   - Demo unreachable code removal on example programs
5. **MVP READY**: Basic DCE working for most common case (unreachable blocks)

**Estimated MVP Time**: ~13 hours of focused development

### Incremental Delivery

1. **Foundation** (Phases 1-2) → Core types ready - ~5 hours
2. **Add US1** (Phase 3) → Test independently → **MVP Demo!** - ~8 hours
3. **Add US2** (Phase 4) → Test independently → Demo dead instruction removal - ~10 hours
4. **Add US3** (Phase 5) → Test independently → Demo memory optimization - ~8 hours  
5. **Add US4** (Phase 6) → Test independently → Demo cascading optimization - ~6 hours
6. **Integration** (Phases 7-8) → Module-level, all terminators - ~6 hours
7. **Quality** (Phases 9-13) → Edge cases, benchmarks, docs, polish - ~12 hours

**Total Estimated Time**: ~55 hours

Each story adds value without breaking previous stories.

### Parallel Team Strategy

With 3 developers after Foundation complete (Phase 2):

- **Developer A**: User Story 1 (Phase 3) - Reachability analysis
- **Developer B**: User Story 2 (Phase 4) - Liveness analysis (parallel - independent)
- **Developer C**: Phase 8 (Terminators) - Can start in parallel with US1

After US1 and US2 complete:
- **Developer A**: User Story 3 (Phase 5) - Memory operations (needs US2)
- **Developer B**: User Story 4 (Phase 6) - Fixed-point (needs US1+US2)
- **Developer C**: Phase 9 (Edge cases) - Testing all functionality

Final integration:
- **All developers**: Phases 10-13 in parallel (benchmarks, diagnostics, docs, polish)

**Parallel Speedup**: ~30 hours with 3 developers

---

## Success Metrics

After completing all tasks, verify:

- ✅ **SC-001**: 100% of unreachable code removed (T017-T020 tests)
- ✅ **SC-002**: At least 90% of provably-dead instructions removed (T031-T034 tests)
- ✅ **SC-003**: Fixed-point within 5 iterations for typical functions (T072-T075 tests)
- ✅ **SC-004**: <1 second for 10k instruction function (T114, T117 benchmarks)
- ✅ **SC-005**: 100% correctness preserved (T143, T147 full test suite)
- ✅ **SC-006**: 15-30% code size reduction on benchmarks (T119 benchmark results)
- ✅ **SC-007**: 100% accurate statistics reporting (T126 test)
- ✅ **SC-008**: Single iteration when no dead code (T075 test)
- ✅ **SC-009**: 100% SSA form validation after optimization (T093, T150 verification)
- ✅ **SC-010**: All existing compiler tests pass (T147)
- ✅ **SC-011**: 95%+ conservative decisions emit warnings (T127 test)

---

## Notes

- **[P]** marks tasks that can run in parallel (different files or independent algorithms)
- **[Story]** label maps each task to its user story for traceability
- Each user story is independently testable (TDD approach - tests first!)
- Tests MUST fail before implementation, MUST pass after implementation
- Commit after each logical task group or when tests transition from red to green
- Use snapshot tests (insta crate) liberally to catch regressions
- Maximum iteration limit prevents infinite loops during development
- Conservative analysis ensures correctness (better to miss optimization than break code)
- Phase 2 (Foundational) is CRITICAL - blocks all user story work
- MVP is achievable with just Phases 1-3 (~13 hours)
- Full implementation with all user stories: ~55 hours solo, ~30 hours with 3 devs
