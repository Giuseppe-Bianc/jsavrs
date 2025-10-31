# Tasks: Multi-pass IR Optimizer for jsavrs

## Feature Overview
Multi-pass IR optimizer for the jsavrs compiler that transforms SSA-form Modules through rigorous analysis and systematic transformations while guaranteeing semantic preservation. The optimizer accepts Modules containing Functions with complete CFGs, dominator trees from DominanceInfo, and valid phi nodes at control flow joins, then produces semantically equivalent optimized IR maintaining SSA invariants.

## User Stories
1. **US1 - Optimize Release Builds (P1)**: As a compiler maintainer, I want the optimizer to minimize instruction count and enhance loop execution efficiency in production builds.
2. **US2 - Fast Iteration During Development (P2)**: As a developer, I want a fast, low-optimization mode so that compilation latency remains low during edit-compile-test cycles.
3. **US3 - Reliable Semantic Preservation and Debuggability (P1)**: As a QA engineer, I require each optimization pass to be fully verifiable and easily revertible.

---

## Phase 1: Setup

- [ ] T001 Create src/ir/optimizer/ directory structure
- [ ] T002 Create src/ir/optimizer/mod.rs as main entry point
- [ ] T003 Create src/ir/optimizer/analysis/ directory
- [ ] T004 Create src/ir/optimizer/passes/ directory
- [ ] T005 Create src/ir/optimizer/verification/ directory
- [ ] T006 Create src/ir/optimizer/pass/ directory
- [ ] T007 Create src/ir/optimizer/config.rs
- [ ] T008 Create src/ir/optimizer/metrics.rs

---

## Phase 2: Foundational Components

- [ ] T009 Create OptimizerConfig struct in src/ir/optimizer/config.rs
- [ ] T010 Implement OptLevel enum in src/ir/optimizer/config.rs
- [ ] T011 Implement config_for_level function in src/ir/optimizer/config.rs
- [ ] T012 Create OptimizerError enum in src/ir/optimizer/mod.rs
- [ ] T013 Create PassMetrics struct in src/ir/optimizer/metrics.rs
- [ ] T014 Create OptimizerReport struct in src/ir/optimizer/metrics.rs
- [ ] T015 Define Analysis trait in src/ir/optimizer/analysis/mod.rs
- [ ] T016 Define OptimizationPass trait in src/ir/optimizer/passes/mod.rs
- [ ] T017 Define AnalysisKind enum in src/ir/optimizer/analysis/mod.rs
- [ ] T018 Create AnalysisManager struct in src/ir/optimizer/analysis/mod.rs
- [ ] T019 Create PassManager struct in src/ir/optimizer/pass/manager.rs

---

## Phase 3: [US1] Optimize Release Builds

### Story Goal: Minimize instruction count and enhance loop execution efficiency in production builds

### Independent Test Criteria:
1. Run a benchmark suite on inputs before and after optimization, verifying behavior equivalence and performance improvements
2. For SSA Module with loops and repeated expressions, after O2/O3 optimization, Module contains fewer instructions (>= 5% reduction) with preserved outputs
3. For Module with induction-variable patterns, after IV optimization, generated IR replaces multiplications with additions where safe

### Implementation Tasks:

#### Core Analysis Infrastructure
- [ ] T020 [P] [US1] Create UseDefManager struct in src/ir/optimizer/analysis/use_def.rs
- [ ] T021 [P] [US1] Implement UseDefManager::build_from_function
- [ ] T022 [P] [US1] Implement UseDefManager::get_def and UseDefManager::get_uses
- [ ] T023 [P] [US1] Create ReachingDefinitions struct in src/ir/optimizer/analysis/reaching_defs.rs
- [ ] T024 [P] [US1] Implement ReachingDefinitions::compute using worklist algorithm
- [ ] T025 [P] [US1] Create LiveVariables struct in src/ir/optimizer/analysis/live_vars.rs
- [ ] T026 [P] [US1] Implement LiveVariables::compute using backward dataflow
- [ ] T027 [P] [US1] Create ConstantLattice enum in src/ir/optimizer/analysis/constants.rs
- [ ] T028 [P] [US1] Create ConstantPropagation struct in src/ir/optimizer/analysis/constants.rs

#### Core Passes Implementation
- [ ] T029 [P] [US1] Create Sparse Conditional Constant Propagation (SCCP) pass in src/ir/optimizer/passes/sccp.rs
- [ ] T030 [P] [US1] Implement SCCP::run method with executable edge management
- [ ] T031 [P] [US1] Create Aggressive Dead Code Elimination (DCE) pass in src/ir/optimizer/passes/dce.rs
- [ ] T032 [P] [US1] Implement DCE::run method using mark-and-sweep over use-def chains
- [ ] T033 [P] [US1] Create Copy Propagation pass in src/ir/optimizer/passes/copy_prop.rs
- [ ] T034 [P] [US1] Implement Copy Propagation::run method with use-def chain updates
- [ ] T035 [P] [US1] Create Global Value Numbering (GVN) analysis in src/ir/optimizer/analysis/gvn.rs
- [ ] T036 [P] [US1] Create GVN/CSE pass in src/ir/optimizer/passes/gvn_cse.rs
- [ ] T037 [P] [US1] Implement GVN/CSE::run method with expression hashing
- [ ] T038 [P] [US1] Create LoopInfo analysis in src/ir/optimizer/analysis/loops.rs
- [ ] T039 [P] [US1] Implement LoopInfo::compute using dominator tree back-edge detection
- [ ] T040 [P] [US1] Create Loop Invariant Code Motion (LICM) pass in src/ir/optimizer/passes/licm.rs
- [ ] T041 [P] [US1] Implement LICM::run method with preheader creation and invariant motion
- [ ] T042 [P] [US1] Create Induction Variable Optimization pass in src/ir/optimizer/passes/iv_opt.rs
- [ ] T043 [P] [US1] Implement IV Opt::run method with phi pattern recognition

#### Advanced Optimizations
- [ ] T044 [P] [US1] Create Instruction Combining pass in src/ir/optimizer/passes/instruction_combining.rs
- [ ] T045 [P] [US1] Implement Instruction Combining::run method with pattern matching
- [ ] T046 [P] [US1] Create Algebraic Simplification pass in src/ir/optimizer/passes/algebraic_simp.rs
- [ ] T047 [P] [US1] Implement Algebraic Simplification::run method with identity rules
- [ ] T048 [P] [US1] Create Strength Reduction pass in src/ir/optimizer/passes/strength_red.rs
- [ ] T049 [P] [US1] Implement Strength Reduction::run method for power-of-two operations
- [ ] T050 [P] [US1] Create Phi Optimization pass in src/ir/optimizer/passes/phi_opt.rs
- [ ] T051 [P] [US1] Implement Phi Optimization::run method for trivial phi elimination
- [ ] T052 [P] [US1] Create Loop Unrolling pass in src/ir/optimizer/passes/loop_unroll.rs
- [ ] T053 [P] [US1] Implement Loop Unrolling::run method with trip count verification

---

## Phase 4: [US2] Fast Iteration During Development

### Story Goal: Provide fast, low-optimization mode to keep compilation latency low during development

### Independent Test Criteria:
1. Build with O0 and O1 and measure compile time; O0 should add minimal overhead vs. baseline; O1 performs limited passes with single iteration

### Implementation Tasks:

#### Configuration for Fast Builds
- [ ] T054 [US2] Update OptimizerConfig to support O0 and O1 configurations with minimal passes
- [ ] T055 [US2] Implement O0 config with no optimization passes
- [ ] T056 [US2] Implement O1 config with SCCP + single DCE iteration only
- [ ] T057 [US2] Add max_iterations parameter to limit pass iterations for O1 level

#### Performance Measurement
- [ ] T058 [US2] Enhance PassMetrics to track compilation time overhead
- [ ] T059 [US2] Implement compile time verification in tests to ensure < 30% overhead for O1

---

## Phase 5: [US3] Reliable Semantic Preservation and Debuggability

### Story Goal: Ensure each optimization pass is fully verifiable and revertible, preserving semantics and debug info

### Independent Test Criteria:
1. Run project test suite with/without optimizations; optimizer must pass verification or auto-rollback failing changes; debug info preserved for remaining instructions

### Implementation Tasks:

#### Verification Infrastructure
- [ ] T060 [P] [US3] Create verify_ssa_form function in src/ir/optimizer/verification/ssa_check.rs
- [ ] T061 [P] [US3] Implement SSA verification checking temporary uniqueness, domination, phi node predecessors
- [ ] T062 [P] [US3] Create verify_cfg_consistency function in src/ir/optimizer/verification/cfg_check.rs
- [ ] T063 [P] [US3] Implement CFG verification checking entry block, reachability, terminator validity
- [ ] T064 [P] [US3] Create verify_type_consistency function in src/ir/optimizer/verification/type_check.rs
- [ ] T065 [P] [US3] Implement type verification checking instruction operand types
- [ ] T066 [US3] Create verify_and_rollback function in src/ir/optimizer/verification/mod.rs

#### Rollback Mechanism
- [ ] T067 [US3] Create FunctionSnapshot struct in src/ir/optimizer/verification/rollback.rs
- [ ] T068 [P] [US3] Implement FunctionSnapshot::capture for function state preservation
- [ ] T069 [P] [US3] Implement FunctionSnapshot::restore for state rollback
- [ ] T070 [US3] Update PassManager to use snapshots with verification and rollback on failure

#### Memory Optimization Passes
- [ ] T071 [P] [US3] Create Store-to-Load Forwarding pass in src/ir/optimizer/passes/store_to_load.rs
- [ ] T072 [P] [US3] Implement Store-to-Load::run method with alias analysis verification
- [ ] T073 [P] [US3] Create Redundant Load Elimination pass in src/ir/optimizer/passes/redundant_loads.rs
- [ ] T074 [P] [US3] Implement Redundant Load Elimination::run method with available loads tracking
- [ ] T075 [P] [US3] Create Dead Store Elimination pass in src/ir/optimizer/passes/dead_store.rs
- [ ] T076 [P] [US3] Implement Dead Store Elimination::run method with live store tracking

#### Alias Analysis
- [ ] T077 [P] [US3] Define AliasAnalysis trait in src/ir/optimizer/analysis/alias.rs
- [ ] T078 [P] [US3] Create AndersenAnalysis implementation in src/ir/optimizer/analysis/alias.rs
- [ ] T079 [P] [US3] Create ConservativeAnalysis implementation in src/ir/optimizer/analysis/alias.rs
- [ ] T080 [US3] Update OptimizerConfig with alias analysis selection for optimization levels

#### Debug Info Preservation
- [ ] T081 [US3] Ensure all optimization passes preserve SourceSpan in Instruction::debug_info
- [ ] T082 [US3] Implement optional provenance tracking in Value with ProvenanceRecord
- [ ] T083 [US3] Update config_for_level to enable provenance tracking optionally

---

## Phase 6: Polish & Cross-Cutting Concerns

### Main Entry Point Implementation
- [ ] T084 Implement optimize_module function in src/ir/optimizer/mod.rs
- [ ] T085 Connect PassManager to run_optimization_pipeline in src/ir/optimizer/pass/manager.rs
- [ ] T086 Implement PassManager::run to iterate through early, middle, late passes
- [ ] T087 Update src/ir/mod.rs to include optimizer module

### Testing Infrastructure
- [ ] T088 [P] Create tests/sccp_tests.rs for Sparse Conditional Constant Propagation
- [ ] T089 [P] Create tests/dce_tests.rs for Dead Code Elimination
- [ ] T090 [P] Create tests/gvn_tests.rs for Global Value Numbering
- [ ] T091 [P] Create tests/licm_tests.rs for Loop Invariant Code Motion
- [ ] T092 [P] Create tests/basic_optimization.rs for basic tests
- [ ] T093 [P] Create tests/loop_optimization.rs for loop optimization tests
- [ ] T094 [P] Create tests/memory_optimization.rs for memory optimization tests
- [ ] T095 [P] Create tests/ssa_preservation.rs for property-based testing
- [ ] T096 Create benches/optimizer_bench.rs for benchmarking with criterion

### Documentation and Error Handling
- [ ] T097 Add rustdoc documentation to all public APIs in optimizer modules
- [ ] T098 Implement proper error handling with OptimizerError for all optimization functions
- [ ] T099 Update README with optimizer usage examples
- [ ] T100 Perform final integration test with real jsavrs compilation pipeline

---

## Dependencies

### User Story Completion Order
US3 (P1) → US1 (P1) → US2 (P2)

US3 must be completed first to ensure semantic preservation and verification infrastructure is in place before implementing the optimization passes in US1. US2 builds on the configuration system established in US1.

### Task Dependencies
- T001-T019 must be completed before any user story tasks
- T020-T028 (analysis infrastructure) required before optimization passes (T029-T053)
- T060-T069 (verification infrastructure) required before optimization passes to enable verification
- T084 (optimize_module) requires all previous components

---

## Parallel Execution Examples Per Story

### US1 (Optimize Release Builds) Parallel Tasks:
- T020, T023, T025, T027 can run in parallel (analysis implementations)
- T029, T031, T033, T035, T038 can run in parallel (initial pass implementations)
- T044, T046, T048, T050, T052 can run in parallel (advanced pass implementations)

### US3 (Reliable Semantic Preservation) Parallel Tasks:
- T060, T062, T064 can run in parallel (verification implementations)
- T071, T073, T075 can run in parallel (memory optimization passes)
- T077, T078, T079 can run in parallel (alias analysis implementations)

---

## Implementation Strategy

### MVP Scope (US3 only - P1)
1. Complete Phase 1 (Setup) and Phase 2 (Foundational)
2. Complete T060-T069 (Verification infrastructure)
3. Complete T067-T069 (Function snapshots) and T070 (rollback)
4. Complete T084-T086 (Basic optimizer functionality)
5. This provides a basic optimizer with verification and rollback but no actual optimization passes

### Incremental Delivery
1. **MVP**: Verification and rollback infrastructure (US3 tasks 60-70, 84-86)
2. **Phase 2**: Basic optimizations (SCCP, DCE) (US1 tasks 29-34)
3. **Phase 3**: Advanced optimizations (US1 remaining tasks)
4. **Phase 4**: Configuration for fast builds (US2 tasks)