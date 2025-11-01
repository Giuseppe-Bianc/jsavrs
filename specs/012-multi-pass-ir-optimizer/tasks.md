# Tasks: Multi-pass IR Optimizer

**Input**: Design documents from `/specs/012-multi-pass-ir-optimizer/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Tests are OPTIONAL and only included where explicitly specified in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and optimizer infrastructure setup

- [X] T001 Create optimizer module structure in src/ir/optimizer/ with subdirectories: analysis/, passes/, verification/
- [X] T002 [P] Add petgraph, bit-vec, rustc-hash, and serde (with derive feature) dependencies to Cargo.toml for graph analysis, dataflow sets, fast hashing (FxHashMap/FxHashSet), and structured metrics serialization
- [X] T003 [P] Create optimizer configuration module in src/ir/optimizer/config.rs with OptLevel enum (O0, O1, O2, O3) and OptimizerConfig struct
- [X] T004 [P] Create optimizer error types in src/ir/optimizer/error.rs with OptimizerError enum for all error conditions including VerificationError with fields: pass_name (String), error_kind (enum), function_name (String), block_label (Option<String>), instruction_index (Option<usize>), message (String)
- [X] T005 [P] Create metrics module in src/ir/optimizer/metrics.rs with PassMetrics and OptimizerReport structs; OptimizerReport must be serializable (derive Serialize) for structured output as JSON or human-readable text via Display trait

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T006 Create Analysis trait in src/ir/optimizer/analysis/mod.rs with compute() and invalidate() methods, plus AnalysisKind enum
- [ ] T007 Create OptimizationPass trait in src/ir/optimizer/passes/mod.rs with name(), required_analyses(), invalidated_analyses(), and run() methods
- [ ] T008 Create AnalysisManager in src/ir/optimizer/pass_manager.rs for caching and invalidating analysis results with HashMap-based cache
- [ ] T009 Create PassManager in src/ir/optimizer/pass_manager.rs with run_optimization_pipeline() and optimize_function() methods
- [ ] T010 [P] Create verification module structure in src/ir/optimizer/verification/mod.rs with VerificationError enum
- [ ] T011 [P] Implement SSA form verification in src/ir/optimizer/verification/ssa_verify.rs with verify_ssa_form() checking unique definitions, dominance, phi consistency
- [ ] T012 [P] Implement CFG consistency verification in src/ir/optimizer/verification/cfg_verify.rs with verify_cfg_consistency() checking reachability and terminator validity
- [ ] T013 [P] Implement type consistency verification in src/ir/optimizer/verification/type_verify.rs with verify_type_consistency() checking operand type matching
- [ ] T013a [P] Implement conservative edge case handling verification in src/ir/optimizer/verification/edge_cases.rs that validates conservative treatment of inline assembly blocks (marked with inline_asm attribute), external/FFI function calls (identified by external linkage), and volatile memory operations, ensuring no speculative optimizations cross these boundaries
- [ ] T014 Implement function snapshot and rollback in src/ir/optimizer/verification/rollback.rs with FunctionSnapshot for capturing/restoring function state
- [ ] T015 Create optimizer main entry point in src/ir/optimizer/mod.rs with optimize_module() function orchestrating the full pipeline

- [ ] T015a [P] Implement large function bailout logic in src/ir/optimizer/mod.rs that checks function size against OptimizerConfig thresholds (>5000 instructions OR >500 basic blocks OR loops >1000 iterations) before optimization; emit warning diagnostic and apply reduced optimization strategy (essential passes only, max 2 iterations, skip expensive analyses like Andersen alias analysis) for oversized functions

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Optimize Release Builds (Priority: P1) 🎯 MVP

**Goal**: Implement core optimization passes (SCCP, ADCE, GVN/CSE, loop optimizations) that reduce instruction count by >=5% while preserving semantics

**Independent Test**: Run benchmark suite (`cargo bench`) with O2 optimization and verify >=5% median instruction reduction vs O0, with identical program outputs

### Analysis Framework for User Story 1

- [ ] T016 [P] [US1] Implement UseDefManager analysis in src/ir/optimizer/analysis/use_def.rs with use-def and def-use chain construction using HashMap<ValueId, Vec<InstructionRef>>
- [ ] T017 [P] [US1] Implement ReachingDefinitions analysis in src/ir/optimizer/analysis/reaching_defs.rs with worklist-based dataflow using BitVec for def sets
- [ ] T018 [P] [US1] Implement LiveVariables analysis in src/ir/optimizer/analysis/live_vars.rs with backward dataflow propagation using BitVec for live sets
- [ ] T019 [P] [US1] Implement ConstantLattice and constant propagation analysis in src/ir/optimizer/analysis/constants.rs with Top/Constant/Bottom lattice states
- [ ] T020 [P] [US1] Implement ConservativeAnalysis in src/ir/optimizer/analysis/alias.rs that returns may-alias for all pointer pairs except provably distinct allocas
- [ ] T021 [US1] Implement AndersenAnalysis in src/ir/optimizer/analysis/alias.rs with inclusion-based points-to analysis using constraint graph solving
- [ ] T022 [P] [US1] Implement LoopInfo analysis in src/ir/optimizer/analysis/loops.rs with natural loop detection using dominator tree back-edge detection
- [ ] T023 [P] [US1] Implement GlobalValueNumbering analysis in src/ir/optimizer/analysis/gvn.rs with expression hashing for CSE using FxHashMap from rustc-hash

### Early-Phase Passes for User Story 1

- [ ] T024 [US1] Implement SCCP pass in src/ir/optimizer/passes/sccp.rs with executable edge tracking, lattice-based constant propagation, and unreachable code detection
- [ ] T025 [US1] Implement ADCE pass in src/ir/optimizer/passes/adce.rs with mark-and-sweep over use-def graph, removing unused instructions and unreachable blocks
- [ ] T026 [US1] Implement copy propagation pass in src/ir/optimizer/passes/copy_prop.rs that eliminates trivial assignments by rewriting use-def chains

### Middle-Phase Passes for User Story 1

- [ ] T027 [US1] Implement GVN/CSE pass in src/ir/optimizer/passes/gvn_cse.rs that assigns unique identifiers to expressions and replaces duplicates respecting memory dependencies
- [ ] T028 [US1] Implement LICM pass in src/ir/optimizer/passes/licm.rs that hoists loop-invariant computations to preheaders when safe per dominance and alias analysis
- [ ] T029 [US1] Implement induction variable optimization pass in src/ir/optimizer/passes/iv_opt.rs that identifies IV patterns and replaces multiplications with additions
- [ ] T030 [US1] Implement loop unrolling pass in src/ir/optimizer/passes/loop_unroll.rs with configurable thresholds, body replication, and phi node updates

### Late-Phase Passes for User Story 1

- [ ] T031 [P] [US1] Implement instruction combining pass in src/ir/optimizer/passes/inst_combine.rs with pattern matching for consecutive shifts and arithmetic operations
- [ ] T032 [P] [US1] Implement algebraic simplification pass in src/ir/optimizer/passes/algebraic_simp.rs applying identity laws (x+0→x, x*1→x, etc.)
- [ ] T033 [P] [US1] Implement strength reduction pass in src/ir/optimizer/passes/strength_reduction.rs converting multiplications by powers of 2 to shifts
- [ ] T034 [P] [US1] Implement phi optimization pass in src/ir/optimizer/passes/phi_opt.rs removing trivial phi nodes and coalescing equivalent phis
- [ ] T034a [P] [US1] Implement type/cast optimization pass in src/ir/optimizer/passes/type_cast_opt.rs that eliminates redundant casts using TypePromotion matrix analysis, combines cascaded casts into single operations, and narrows types when value range analysis proves safety

### Memory Optimization Passes for User Story 1

- [ ] T035 [P] [US1] Implement store-to-load forwarding pass in src/ir/optimizer/passes/store_to_load_fwd.rs eliminating loads after stores to same location verified by alias analysis
- [ ] T036 [P] [US1] Implement redundant load elimination pass in src/ir/optimizer/passes/redundant_loads.rs tracking memory state across blocks using available expressions
- [ ] T037 [P] [US1] Implement dead store elimination pass in src/ir/optimizer/passes/dead_stores.rs removing stores to overwritten locations using backward dataflow

### Configuration for User Story 1

- [ ] T038 [US1] Implement OptimizerConfig::config_for_level() factory method creating O2 configuration with all passes, max_iterations=10, Andersen alias analysis
- [ ] T039 [US1] Update PassManager to execute early→middle→late pass sequences with fixed-point detection and verification after each modified pass

**Checkpoint**: At this point, User Story 1 should be fully functional - O2 optimization reduces instruction count by >=5% with preserved semantics

---

## Phase 4: User Story 2 - Fast Iteration During Development (Priority: P2)

**Goal**: Implement fast O0/O1 optimization levels with minimal compile-time overhead (<30% for O1 vs baseline)

**Independent Test**: Build test suite with O0 and O1, measure compile times; O0 adds minimal overhead, O1 performs limited passes with single iteration and overhead <30%

### Implementation for User Story 2

- [ ] T040 [US2] Implement O0 configuration in src/ir/optimizer/config.rs with empty pass vectors and max_iterations=0
- [ ] T041 [US2] Implement O1 configuration in src/ir/optimizer/config.rs with only SCCP and ADCE in early_passes, max_iterations=1, Conservative alias analysis
- [ ] T042 [US2] Add optimization level bypass logic in src/ir/optimizer/mod.rs optimize_module() to skip pass execution entirely for O0
- [ ] T043 [US2] Add compile-time tracking in PassManager for measuring per-pass and total optimization time
- [ ] T044 [US2] Validate O1 compile-time overhead is <30% vs baseline (O0 with completely bypassed optimizer pipeline) through criterion benchmarks in benches/optimizer_bench.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - O0/O1 provide fast compilation, O2 provides full optimization

---

## Phase 5: User Story 3 - Reliable Semantic Preservation and Debuggability (Priority: P1)

**Goal**: Ensure all optimizations preserve semantics through comprehensive verification and automatic rollback, with debug info (SourceSpan) preserved for 90% of remaining instructions

**Independent Test**: Run full test suite (`cargo test`) with all optimization levels; optimizer either passes verification or automatically rolls back failing changes; debug info preserved for >=90% of non-eliminated instructions

### Implementation for User Story 3

- [ ] T045 [US3] Integrate verification checks in PassManager::optimize_function() to run verify_ssa_form(), verify_cfg_consistency(), and verify_type_consistency() after each pass reporting changes
- [ ] T046 [US3] Implement automatic rollback on verification failure using FunctionSnapshot in PassManager when verification detects SSA/CFG/type errors; track verification failure count per function and skip remaining passes when max failures reached (default: 3 per function)
- [ ] T047 [US3] Add detailed diagnostic output for verification failures in src/ir/optimizer/error.rs showing which pass failed, why, and which invariant was violated
- [ ] T048 [US3] Implement debug information preservation across all transformation passes ensuring SourceSpan is maintained or propagated for all non-eliminated instructions
- [ ] T049 [US3] Add optimization provenance tracking in src/ir/optimizer/metrics.rs when config.record_provenance=true, recording transformation history for each value
- [ ] T050 [US3] Implement verification metrics reporting in OptimizerReport showing verification success rate, rollback count, and debug info preservation percentage
- [ ] T051 [US3] Add comprehensive error handling with Result<T, OptimizerError> throughout optimizer ensuring all errors are properly propagated and logged
- [ ] T052 [US3] Validate test suite passes at all optimization levels (O0, O1, O2, O3) with zero unchecked SSA/CFG errors and >=90% debug info preservation

**Checkpoint**: All user stories should now be independently functional - optimizer reliably preserves semantics with automatic verification and rollback

---

## Phase 6: Integration & Polish

**Purpose**: Integrate optimizer into compilation pipeline and add cross-cutting improvements

- [ ] T053 Add --opt-level CLI flag in src/cli.rs accepting O0, O1, O2, O3 with default O0
- [ ] T054 [P] Add --verbose CLI flag in src/cli.rs for displaying optimizer metrics
- [ ] T055 Integrate optimizer in src/main.rs compilation pipeline after SSA transformation, before code generation
- [ ] T056 [P] Add comprehensive unit tests for individual passes in tests/ directory (sccp_tests.rs, adce_tests.rs, gvn_cse_tests.rs, licm_tests.rs, iv_opt_tests.rs, loop_unroll_tests.rs)
- [ ] T057 [P] Add verification tests in tests/verification_tests.rs testing SSA/CFG/type verification and rollback mechanisms
- [ ] T058 [P] Add integration tests in tests/integration_tests.rs testing end-to-end optimization with program execution and output validation
- [ ] T059 [P] Add snapshot tests using insta for IR transformation validation in tests/snapshots/
- [ ] T060 [P] Add property tests using proptest for SSA/CFG invariant preservation in tests/property_tests.rs
- [ ] T061 [P] Add criterion benchmarks in benches/optimizer_bench.rs measuring optimization performance on representative programs
- [ ] T062 [P] Add comprehensive rustdoc documentation with Examples sections for all public APIs in src/ir/optimizer/
- [ ] T063 Implement O3 configuration in src/ir/optimizer/config.rs with increased loop_unroll_threshold=8 and aggressive optimizations
- [ ] T064 [P] Add configuration validation ensuring max_iterations >= 1, loop_unroll_threshold >= 1, and pass vectors are consistent with opt_level
- [ ] T065 Run quickstart.md validation scenarios verifying optimizer integration and usage examples

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase - Core optimization passes implementing release build optimization
- **User Story 2 (Phase 4)**: Depends on Foundational phase - Can start in parallel with US1, implements fast O0/O1 configurations
- **User Story 3 (Phase 5)**: Depends on Foundational phase AND User Story 1 (needs passes to verify) - Adds verification and rollback to existing passes
- **Integration & Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories - Implements core optimization passes
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independently testable, adds O0/O1 fast paths - May run in parallel with US1
- **User Story 3 (P1)**: Depends on User Story 1 completion - Adds verification and rollback to existing US1 passes - Cannot test verification without passes to verify

### Within Each User Story

**User Story 1**:
- Analysis framework tasks (T016-T023) can run in parallel except T021 (Andersen) depends on T020 (Conservative base)
- Early-phase passes (T024-T026) depend on analysis framework completion
- Middle-phase passes (T027-T030) depend on early-phase passes for use-def chains and dominance
- Late-phase passes (T031-T034) can run in parallel, depend on middle-phase passes
- Memory passes (T035-T037) can run in parallel, depend on alias analysis (T020-T021)
- Configuration (T038-T039) depends on all passes being complete

**User Story 2**:
- All tasks sequential (T040-T044) configuring fast optimization levels

**User Story 3**:
- Verification integration (T045-T047) depends on verification functions from Phase 2
- Debug preservation (T048-T049) depends on all passes from US1
- Validation (T050-T052) depends on all verification features complete

### Parallel Opportunities

**Setup (Phase 1)**:
- T002, T003, T004, T005 can all run in parallel (different files)

**Foundational (Phase 2)**:
- T010, T011, T012, T013, T013a can run in parallel (verification modules)
- T006, T007 must complete before T008, T009 (traits before manager)

**User Story 1 Analysis**:
```bash
# Launch in parallel:
T016: UseDefManager (use_def.rs)
T017: ReachingDefinitions (reaching_defs.rs)
T018: LiveVariables (live_vars.rs)
T019: ConstantLattice (constants.rs)
T020: ConservativeAnalysis (alias.rs base)
T022: LoopInfo (loops.rs)
T023: GlobalValueNumbering (gvn.rs)
# Then:
T021: AndersenAnalysis (alias.rs extension)
```

**User Story 1 Late-Phase Passes**:
```bash
# Launch in parallel:
T031: Instruction combining (inst_combine.rs)
T032: Algebraic simplification (algebraic_simp.rs)
T033: Strength reduction (strength_reduction.rs)
T034: Phi optimization (phi_opt.rs)
T034a: Type/cast optimization (type_cast_opt.rs)
```

**User Story 1 Memory Passes**:
```bash
# Launch in parallel:
T035: Store-to-load forwarding (store_to_load_fwd.rs)
T036: Redundant load elimination (redundant_loads.rs)
T037: Dead store elimination (dead_stores.rs)
```

**Integration & Polish**:
```bash
# Launch in parallel:
T054: Verbose flag (cli.rs)
T056: Unit tests (tests/)
T057: Verification tests (tests/)
T058: Integration tests (tests/)
T059: Snapshot tests (tests/)
T060: Property tests (tests/)
T061: Benchmarks (benches/)
T062: Documentation (src/ir/optimizer/)
T064: Configuration validation (config.rs)
```

---

## Implementation Strategy

### MVP First (User Story 1 + User Story 3 Core)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (core optimization passes)
4. Complete Phase 5: User Story 3 (verification and rollback - critical for correctness)
5. **STOP and VALIDATE**: Test O2 optimization with full verification on benchmark suite
6. Deploy/demo if ready

**Rationale**: US1 (P1) delivers core optimization value, US3 (P1) ensures correctness. US2 (P2) is nice-to-have for dev velocity but not essential for MVP.

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently on benchmarks → Deploy/Demo (MVP with O2 optimization!)
3. Add User Story 3 → Test verification and rollback → Deploy/Demo (production-ready with guarantees)
4. Add User Story 2 → Test fast O0/O1 modes → Deploy/Demo (complete feature with all opt levels)
5. Add Integration & Polish → Final release

### Parallel Team Strategy

With multiple developers after Foundational phase completes:

- **Developer A**: User Story 1 Analysis Framework (T016-T023)
- **Developer B**: User Story 1 Early-Phase Passes (T024-T026)
- **Developer C**: Phase 2 Verification modules (T011-T014) + User Story 3 prep

After US1 analysis complete:
- **Developer A**: User Story 1 Middle-Phase Passes (T027-T030)
- **Developer B**: User Story 1 Late-Phase Passes (T031-T034)
- **Developer C**: User Story 1 Memory Passes (T035-T037)

After US1 complete:
- **Developer A**: User Story 2 (fast optimization levels)
- **Developer B**: User Story 3 (verification integration)
- **Developer C**: Integration & Polish (tests, benchmarks, docs)

---

## Success Validation

### User Story 1 Validation (SC-001)

- Run benchmark suite (benches/jsavrs_benchmark.rs criterion suite plus representative test programs from tests/ exercising loops, arithmetic, memory operations, and control flow) at O2 optimization level
- Measure instruction count reduction vs O0 baseline
- **Target**: >=5% median reduction across representative benchmarks
- Verify program outputs are identical (semantic preservation)
- Command: `cargo bench` with O2 flag

### User Story 2 Validation (SC-003)

- Build test suite with O0, O1, and O2 optimization levels
- Measure total compilation time for each level (baseline = O0 with completely bypassed optimizer pipeline)
- **Target**: O0 minimal overhead (<5%), O1 <30% overhead, O2 <100% overhead vs baseline
- Command: `cargo build --release` with timing instrumentation

### User Story 3 Validation (SC-002, SC-004)

- Run full test suite (`cargo test`) with O1, O2, O3 optimization
- **Target**: >=95% of Functions pass verification without rollback
- Measure debug info preservation rate
- **Target**: >=90% of remaining instructions retain SourceSpan
- Verify verification failures trigger automatic rollback
- Command: `cargo test -- --nocapture` with verification metrics enabled

### Integration Validation

- Complete quickstart.md usage scenarios
- Verify CLI integration with --opt-level flag
- Run snapshot tests (`cargo test -- --nocapture`) verifying IR transformations
- Run property tests validating SSA/CFG invariants
- Run criterion benchmarks measuring optimization performance

---

## Notes

- **[P] tasks**: Different files, no dependencies - can run in parallel
- **[Story] labels**: Map tasks to specific user stories for traceability and independent testing
- **Verification is critical**: User Story 3 ensures correctness - must complete before production use
- **Optimization levels**: O0 (dev fast), O1 (dev optimized), O2 (production balanced), O3 (production aggressive)
- **Terminology**: Use "optimization pipeline" (not "pass sequence" or "phases") for consistency with API naming (`run_optimization_pipeline`); use "SSA value" consistently throughout implementation (note: `ValueKind::Temporary` enum variant exists in IR for representing temporary values during code generation, but documentation and user-facing text should use the broader term "SSA value" to describe all value types in SSA form); "blocks removed" includes both unreachable blocks (CFG analysis) and dead blocks (no side effects)
- **CFG Maintenance**: When passes modify CFG structure (merge/split/remove blocks), use semi-NCA (Semi-Naive Common Ancestor) algorithm for incremental dominator tree recomputation as specified in FR-006 and detailed in research.md
- **Testing strategy**: Unit tests for passes, integration tests for end-to-end, snapshot tests for IR validation, property tests for invariants
- **Commit strategy**: Commit after each task or logical group, use feature branches per user story
- **Stop at checkpoints**: Validate each user story independently before proceeding
- **Dependencies**: Use `cargo tree` to verify petgraph, bit-vec, rustc-hash, and serde integration
- **Documentation**: Follow Rust rustdoc conventions with Examples sections
- **Error handling**: Use Result<T, OptimizerError> throughout for proper error propagation; verification failures trigger function-level rollback and are tracked per-function with a configurable limit (default: 3 failures per function) before skipping remaining passes for that function; all verification failures are reported in OptimizerReport

````
