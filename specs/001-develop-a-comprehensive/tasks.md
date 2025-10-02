# Tasks: Comprehensive x86-64 ABI Trait System

**Input**: Design documents from `C:\dev\vscode\rust\jsavrs\specs\001-develop-a-comprehensive\`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Extract: Rust 1.75+, trait-based architecture, performance goals (< 0.1% overhead)
   → Tech Stack: tracing 0.1, criterion 0.5, insta 1.x
2. Load design documents:
   → data-model.md: 50+ entities (Platform, Abi, GPRegister64/32/16/8, XMMRegister, etc.)
   → contracts/: 4 trait contracts (CallingConvention, StackManagement, RegisterAllocation, AggregateClassification)
   → quickstart.md: 4 integration scenarios + verification workflow
3. Generate tasks by category:
   → Setup: Rust project dependencies, linting configuration
   → Tests: 4 contract test tasks (TDD - write first, must fail)
   → Core: 5 trait implementation tasks
   → Integration: Cross-compiler validation, snapshot tests, examples
   → Polish: Performance benchmarks, logging, documentation
4. Apply task rules:
   → Contract tests [P] - different files, no dependencies
   → Trait implementations [P] - after tests pass, different files
   → Tests before implementation (TDD)
   → Integration before polish
5. Number tasks sequentially (T001-T020)
6. Validate completeness: All 4 contracts have tests, all traits implemented
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Source**: `c:\dev\vscode\rust\jsavrs\src\asm\` (existing ABI infrastructure)
- **Tests**: `c:\dev\vscode\rust\jsavrs\tests\` (contract and integration tests)
- **Benchmarks**: `c:\dev\vscode\rust\jsavrs\benches\` (performance validation)

## Phase 3.1: Setup & Configuration
- [ ] **T001** Add dependencies to `Cargo.toml`: `tracing = "0.1"`, `criterion = "0.5"`, `insta = "1.x"` at repository root
- [ ] **T002** Configure Criterion benchmarking infrastructure in `benches/abi_benchmarks.rs`
- [ ] **T003** [P] Verify rustfmt and clippy configuration in `rustfmt.toml` at repository root

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests [All Parallel - Different Files]
- [ ] **T004** [P] Write CallingConvention contract tests in `tests/abi_calling_convention_tests.rs`
  - Test parameter register allocation (integer_param_register, float_param_register)
  - Test max parameter counts (max_integer_register_params, max_float_register_params)
  - Test variadic function conventions (variadic_param_register)
  - Test index space behavior (Windows overlapping vs SystemV independent)
  - Verify constant-time lookups via assertions
  - **Expected Result**: All tests FAIL (traits not implemented yet)

- [ ] **T005** [P] Write StackManagement contract tests in `tests/abi_stack_management_tests.rs`
  - Test red zone queries (has_red_zone, red_zone_size_bytes)
  - Test shadow space requirements (requires_shadow_space, shadow_space_bytes)
  - Test stack alignment (min_stack_alignment)
  - Test frame pointer requirements (requires_frame_pointer)
  - Verify Windows vs SystemV differences
  - **Expected Result**: All tests FAIL (traits not implemented yet)

- [ ] **T006** [P] Write RegisterAllocation contract tests in `tests/abi_register_allocation_tests.rs`
  - Test volatile register priority ordering (volatile_gp_registers, volatile_xmm_registers)
  - Test non-volatile register priority ordering (non_volatile_gp_registers)
  - Test volatility checks (is_volatile, is_callee_saved)
  - Verify platform-specific lists (Windows RDI/RSI callee-saved, SystemV volatile)
  - Test edge cases (RSP special handling, RBP optional)
  - **Expected Result**: All tests FAIL (traits not implemented yet)

- [ ] **T007** [P] Write AggregateClassification contract tests in `tests/abi_aggregate_classification_tests.rs`
  - Test small aggregate classification (size ≤ 8 bytes Windows, ≤ 16 bytes SystemV)
  - Test large aggregate classification (by-reference passing)
  - Test structure decomposition (SystemV register splitting)
  - Test field type influence (Integer, Float, Pointer)
  - Verify reference compiler behavior (GCC/Clang/MSVC alignment)
  - **Expected Result**: All tests FAIL (traits not implemented yet)

### Integration Tests [Parallel - Different Files]
- [ ] **T008** [P] Write quickstart scenario integration tests in `tests/abi_integration_tests.rs`
  - Test function prologue generation (Scenario 1 from quickstart.md)
  - Test parameter register allocation (Scenario 2)
  - Test temporary register selection (Scenario 3)
  - Test structure return handling (Scenario 4)
  - Verify Windows x64 add5 function example
  - Verify SystemV compute function example
  - **Expected Result**: All tests FAIL (traits not implemented yet)

## Phase 3.3: Core Implementation (ONLY after tests are failing)
**Dependencies**: T004-T008 must be complete and failing

### Trait Implementations [All Parallel - Different Files]
- [ ] **T009** [P] Implement CallingConvention trait in `src/asm/calling_convention.rs`
  - Create `WindowsX64` struct implementing `CallingConvention`
    - `integer_param_register`: RCX, RDX, R8, R9 (indices 0-3, None for ≥4)
    - `float_param_register`: XMM0-XMM3 (indices 0-3, overlaps with integer indices)
    - `max_integer_register_params`: 4
    - `max_float_register_params`: 4
    - `variadic_param_register`: All parameters on stack after first 4
  - Create `SystemV` struct implementing `CallingConvention`
    - `integer_param_register`: RDI, RSI, RDX, RCX, R8, R9 (indices 0-5)
    - `float_param_register`: XMM0-XMM7 (indices 0-7, independent index space)
    - `max_integer_register_params`: 6
    - `max_float_register_params`: 8
    - `variadic_param_register`: AL register holds XMM count
  - Use static constant arrays for O(1) lookups
  - Add comprehensive rustdoc comments with examples
  - **Verify**: T004 tests now PASS

- [ ] **T010** [P] Implement StackManagement trait in `src/asm/stack_management.rs`
  - Implement for `WindowsX64`:
    - `has_red_zone`: false
    - `red_zone_size_bytes`: 0
    - `requires_shadow_space`: true
    - `shadow_space_bytes`: 32
    - `min_stack_alignment`: 16
    - `requires_frame_pointer`: false
  - Implement for `SystemV`:
    - `has_red_zone`: true
    - `red_zone_size_bytes`: 128
    - `requires_shadow_space`: false
    - `shadow_space_bytes`: 0
    - `min_stack_alignment`: 16
    - `requires_frame_pointer`: false
  - All methods use const evaluation (zero runtime cost)
  - **Verify**: T005 tests now PASS

- [ ] **T011** [P] Implement RegisterAllocation trait in `src/asm/register_allocation.rs`
  - Implement for `WindowsX64`:
    - `volatile_gp_registers`: [RAX, R10, R11, RCX, RDX, R8, R9]
    - `non_volatile_gp_registers`: [RBX, RDI, RSI, R12, R13, R14, R15]
    - `volatile_xmm_registers`: [XMM0-XMM5]
    - `is_volatile`: Check against volatile lists
    - `is_callee_saved`: Check against non-volatile lists
  - Implement for `SystemV`:
    - `volatile_gp_registers`: [RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11]
    - `non_volatile_gp_registers`: [RBX, R12, R13, R14, R15]
    - `volatile_xmm_registers`: [XMM0-XMM15]
  - Use static slices for zero-allocation queries
  - **Verify**: T006 tests now PASS

- [ ] **T012** [P] Implement AggregateClassification trait in `src/asm/aggregate_classification.rs`
  - Define `AggregateClass` enum (ByValue, ByReference, Decomposed)
  - Define `FieldType` enum (Integer, Float, Pointer)
  - Implement for `WindowsX64`:
    - size ≤ 8 → `ByValue(RCX)`
    - size > 8 → `ByReference`
  - Implement for `SystemV`:
    - size > 16 → `ByReference`
    - size ≤ 8, single field → `ByValue` or `Decomposed` based on type
    - size ≤ 16, multiple fields → `Decomposed` with register list
    - Defer complex cases to reference compiler behavior (document with comments)
  - **Verify**: T007 tests now PASS

- [ ] **T013** Update existing `src/asm/register.rs` to use new traits
  - Refactor `is_volatile`, `is_callee_saved`, `is_parameter_register` methods
  - Delegate to appropriate trait implementations
  - Maintain backward compatibility with existing code
  - Add deprecation warnings for old direct method calls (if applicable)
  - **Verify**: Existing tests in `tests/` still pass

## Phase 3.4: Integration & Validation
**Dependencies**: T009-T013 must be complete

- [ ] **T014** Cross-compiler validation tests in `tests/abi_cross_compiler_validation.rs`
  - Generate C test files for function signatures (int add5(int, int, int, int, int))
  - Compile with MSVC (Windows), GCC (Linux), Clang (macOS)
  - Parse generated assembly with regex/asm parser
  - Compare register usage with jsavrs ABI trait queries
  - Assert matching parameter registers, stack offsets, calling conventions
  - Document any discrepancies with reference compiler behavior
  - **Success Criteria**: 95%+ match rate with reference compilers

- [ ] **T015** Snapshot tests for ABI query outputs in `tests/abi_snapshot_tests.rs`
  - Use insta crate for snapshot assertions
  - Snapshot parameter register sequences (0-10 parameters, mixed types)
  - Snapshot volatility classifications for all register types
  - Snapshot structure classification results (various sizes/field types)
  - Generate baseline snapshots with `cargo insta test --review`
  - **Success Criteria**: All snapshots consistent across test runs

- [ ] **T016** Validate quickstart.md examples in `tests/abi_quickstart_validation.rs`
  - Execute all code examples from quickstart.md as integration tests
  - Verify function prologue generation produces expected instructions
  - Verify parameter allocation matches documented behavior
  - Verify temporary register selection follows priority ordering
  - **Success Criteria**: All examples execute without errors, produce expected output

## Phase 3.5: Performance & Observability
**Dependencies**: T014-T016 must be complete

- [ ] **T017** Implement performance benchmarks in `benches/abi_benchmarks.rs`
  - Benchmark `integer_param_register(index)` for 1000 iterations
  - Benchmark `float_param_register(index)` for 1000 iterations
  - Benchmark `is_volatile(register)` for all register types
  - Benchmark `classify_aggregate(size, fields)` for various structures
  - Use Criterion library for statistical analysis
  - **Performance Target**: Median < 10 nanoseconds per query
  - **Success Criteria**: All benchmarks meet performance targets

- [ ] **T018** Integrate tracing instrumentation in `src/asm/calling_convention.rs`, `src/asm/stack_management.rs`, `src/asm/register_allocation.rs`, `src/asm/aggregate_classification.rs`
  - Add `#[tracing::instrument]` to all public trait methods
  - Log ABI decisions at DEBUG level (parameter register choices, structure classifications)
  - Log performance-critical paths at TRACE level
  - Add span context for compiler phases
  - **Verify**: `RUST_LOG=trace cargo test` produces comprehensive logs

- [ ] **T019** [P] Generate rustdoc documentation for all public APIs
  - Add detailed documentation comments to all trait definitions
  - Add examples to all trait methods (rustdoc `/// # Examples` sections)
  - Document platform-specific behavior in method docs
  - Document performance contracts (`/// # Performance`)
  - Run `cargo doc --open` to verify rendering
  - **Success Criteria**: 100% public API coverage, no rustdoc warnings

- [ ] **T020** [P] Run duplication analysis with `similarity-rs --skip-test`
  - Execute: `similarity-rs --skip-test --threshold 0.8`
  - Identify duplicated ABI query logic across WindowsX64/SystemV implementations
  - Extract common patterns into shared helper functions in `src/asm/abi_common.rs`
  - Document justified duplication (platform-specific behavior that can't be abstracted)
  - **Success Criteria**: < 5% duplicated code in ABI implementation modules

## Dependencies Graph
```
Setup (T001-T003)
   ↓
Contract Tests (T004-T007) [PARALLEL]
   ↓
Integration Tests (T008) [PARALLEL with T004-T007]
   ↓
Trait Implementations (T009-T013) [PARALLEL - after tests fail]
   ↓
Existing Code Update (T013) [SEQUENTIAL - modifies shared file]
   ↓
Validation (T014-T016) [SEQUENTIAL - T014 first, then T015-T016 parallel]
   ↓
Performance & Observability (T017-T020) [T017-T018 sequential, T019-T020 parallel]
```

## Parallel Execution Examples

### Phase 1: Write All Contract Tests Simultaneously
```bash
# Execute T004-T008 in parallel (5 separate test files)
Task: "Write CallingConvention contract tests in tests/abi_calling_convention_tests.rs"
Task: "Write StackManagement contract tests in tests/abi_stack_management_tests.rs"
Task: "Write RegisterAllocation contract tests in tests/abi_register_allocation_tests.rs"
Task: "Write AggregateClassification contract tests in tests/abi_aggregate_classification_tests.rs"
Task: "Write quickstart integration tests in tests/abi_integration_tests.rs"
```

### Phase 2: Implement All Traits Simultaneously (After Tests Fail)
```bash
# Execute T009-T012 in parallel (4 separate source files)
Task: "Implement CallingConvention trait in src/asm/calling_convention.rs"
Task: "Implement StackManagement trait in src/asm/stack_management.rs"
Task: "Implement RegisterAllocation trait in src/asm/register_allocation.rs"
Task: "Implement AggregateClassification trait in src/asm/aggregate_classification.rs"
```

### Phase 3: Parallel Documentation and Analysis
```bash
# Execute T019-T020 in parallel (different concerns)
Task: "Generate rustdoc documentation for all public APIs"
Task: "Run duplication analysis with similarity-rs --skip-test"
```

## Notes
- **[P] Marking**: Tasks marked [P] modify different files and have no dependencies
- **TDD Enforcement**: T004-T008 must be written first and must fail before T009-T013
- **Commit Strategy**: Commit after each task completion (atomic commits)
- **Performance Validation**: T017 benchmarks must run on every commit to detect regressions
- **Cross-Compiler Testing**: T014 requires GCC, Clang, MSVC installations (document in CI/CD setup)

## Task Generation Rules Applied

1. **From Contracts** (contracts/*.md):
   - calling_convention_trait.md → T004 (contract test), T009 (implementation)
   - stack_management_trait.md → T005 (contract test), T010 (implementation)
   - register_allocation_trait.md → T006 (contract test), T011 (implementation)
   - aggregate_classification_trait.md → T007 (contract test), T012 (implementation)

2. **From Data Model** (data-model.md):
   - Entities already exist in src/asm/register.rs (GPRegister64, XMMRegister, etc.)
   - T013 updates existing code to integrate with new traits

3. **From Quickstart** (quickstart.md):
   - Scenario 1-4 → T008 (integration test)
   - Verification workflow → T014 (cross-compiler validation)
   - Example code → T016 (quickstart validation)

4. **Ordering Applied**:
   - Setup (T001-T003) before everything
   - Tests (T004-T008) before implementation (TDD principle)
   - Implementation (T009-T013) after tests fail
   - Validation (T014-T016) after implementation complete
   - Polish (T017-T020) after validation passes

## Validation Checklist
*GATE: Verified before task execution*

- [x] All 4 contracts have corresponding test tasks (T004-T007)
- [x] All 4 contracts have implementation tasks (T009-T012)
- [x] All tests come before implementation (T004-T008 before T009-T013)
- [x] Parallel tasks truly independent (different files)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task (except T013 sequential)
- [x] Integration tests validate quickstart examples (T008, T016)
- [x] Performance benchmarks validate < 0.1% overhead (T017)
- [x] Cross-compiler validation ensures correctness (T014)
- [x] Documentation coverage meets rigor standards (T019)
