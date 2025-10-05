# Tasks: Comprehensive Test Suite for Type Promotion Module

**Feature Branch**: `002-creating-detailed-precise`  
**Input**: Design documents from `/specs/002-creating-detailed-precise/`  
**Prerequisites**: plan.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

---

## Execution Flow (main)
```
1. ✅ Loaded plan.md from feature directory
   → Tech stack: Rust (cargo test), Insta (snapshots), cargo llvm-cov (coverage)
   → Libraries: std::collections::HashMap, src/ir/types.rs, src/ir/instruction.rs
   → Structure: Single project (src/, tests/ at repository root)

2. ✅ Loaded design documents:
   → data-model.md: 15 entities (8 domain, 7 test)
   → contracts/: test-contracts.md with 6 contract patterns
   → research.md: 14 sections on testing strategies
   → quickstart.md: Validation workflow with coverage targets

3. ✅ Generated tasks by category:
   → Setup: Helper functions, test data constants
   → Tests (TDD): Normal operations, edge cases, corner cases, error handling
   → Validation: Coverage verification, snapshot review, documentation check

4. ✅ Applied task rules:
   → All tests in same file (tests/ir_type_promotion_tests.rs) = sequential append
   → Helper functions can be parallel with tests [P]
   → Coverage/validation tasks sequential (depend on all tests)

5. ✅ Numbered tasks sequentially (T001-T028)

6. ✅ Generated dependency graph (see Dependencies section)

7. ✅ Created parallel execution examples (see below)

8. ✅ Validated task completeness:
   → All 6 contract patterns have tests ✓
   → All 8 domain entities have test coverage ✓
   → All 4 edge case categories covered ✓

9. Return: SUCCESS (tasks ready for execution)
```

---

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- All test tasks append to: `tests/ir_type_promotion_tests.rs` (sequential)
- Helper tasks can be parallel with test tasks

---

## Phase 3.1: Setup (Infrastructure)

### T001: Create Test Helper Functions and Constants ✅ COMPLETED
**File**: `tests/ir_type_promotion_tests.rs` (append to end)  
**Priority**: HIGH (blocks all other tasks)  
**Estimated Effort**: 20 minutes  
**Contract**: C-005 (Helper Functions)  
**Status**: ✅ COMPLETED 2025-10-05

**Description**:
Create reusable helper functions and test data constants to enable efficient test implementation. This includes functions to create PromotionMatrix instances with custom configurations and constants for common type lists.

**Acceptance Criteria**:
- [x] Helper function `create_matrix_with_overflow(behavior: OverflowBehavior) -> PromotionMatrix` implemented
- [x] Helper function `all_numeric_types() -> Vec<IrType>` implemented  
- [x] Constants `ALL_INTEGER_TYPES`, `ALL_FLOAT_TYPES`, `ALL_BASIC_TYPES` defined
- [x] All helpers documented with rustdoc comments (FR-010)
- [x] Code passes `cargo fmt` and `cargo clippy --test ir_type_promotion_tests`

**Implementation Guidance**:
- Reference research.md Section 12 (Test Data Management) for constant patterns
- Follow contract C-005: helpers must be deterministic, used by ≥2 tests
- Use `const` for type arrays, `fn` for matrix creation

**Validation**:
```powershell
# Verify helpers compile
cargo test --test ir_type_promotion_tests --no-run

# Verify no clippy warnings
cargo clippy --test ir_type_promotion_tests -- -D warnings
```

---

## Phase 3.2: Tests First - Normal Operations (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

**CRITICAL**: These tests validate standard type promotion scenarios. All tests append to `tests/ir_type_promotion_tests.rs`.

### T002: Test PromotionMatrix Initialization
**File**: `tests/ir_type_promotion_tests.rs` (append after T001)  
**Priority**: HIGH  
**Estimated Effort**: 15 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests for PromotionMatrix construction methods (`new()`, `with_overflow_behavior()`). Verify default rules are loaded and overflow behavior is correctly configured.

**Acceptance Criteria**:
- [ ] Test `test_promotion_matrix_new_initializes_default_rules()` implemented
- [ ] Test `test_promotion_matrix_with_overflow_behavior_wrap()` implemented
- [ ] Test `test_promotion_matrix_with_overflow_behavior_trap()` implemented
- [ ] All tests have rustdoc comments explaining rationale (FR-010)
- [ ] Tests pass: `cargo test --test ir_type_promotion_tests promotion_matrix_new`

**Implementation Guidance**:
- Verify `matrix.rules.len() > 0` for default initialization
- Test all OverflowBehavior variants: Wrap, Saturate, Trap, CompileError
- Reference data-model.md Section 1.1 (PromotionMatrix entity)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests promotion_matrix_new
```

---

### T003: Test Identity Promotions (T → T)
**File**: `tests/ir_type_promotion_tests.rs` (append after T002)  
**Priority**: HIGH  
**Estimated Effort**: 20 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests verifying identity promotions (same type to same type) for all basic types. Should always return Direct cast with Bitcast and no warnings.

**Acceptance Criteria**:
- [ ] Test `test_identity_promotions_for_all_types()` implemented (iterates ALL_BASIC_TYPES)
- [ ] Verifies `PromotionRule::Direct { cast_kind: CastKind::Bitcast, may_lose_precision: false, may_overflow: false }`
- [ ] Test documented with rationale (identity promotions are no-ops)
- [ ] Test passes: `cargo test --test ir_type_promotion_tests identity_promotions`

**Implementation Guidance**:
- Use `ALL_BASIC_TYPES` constant from T001
- Loop through all types: `for ty in ALL_BASIC_TYPES { ... }`
- Reference research.md Section 10 (Test Matrix - Equivalence Partitioning)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests identity_promotions
```

---

### T004: Test Signed Integer Widening Promotions
**File**: `tests/ir_type_promotion_tests.rs` (append after T003)  
**Priority**: MEDIUM  
**Estimated Effort**: 25 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests for signed integer widening conversions (I8 → I16 → I32 → I64). Verify Direct cast with SignExtend and no precision loss/overflow warnings.

**Acceptance Criteria**:
- [ ] Test `test_i8_to_i16_widening_no_loss()` implemented
- [ ] Test `test_i16_to_i32_widening_no_loss()` implemented
- [ ] Test `test_i32_to_i64_widening_no_loss()` implemented
- [ ] All tests verify `cast_kind: CastKind::SignExtend`
- [ ] All tests verify `may_lose_precision: false` and `may_overflow: false`
- [ ] Tests documented with widening rationale (larger type can hold all smaller type values)

**Implementation Guidance**:
- Reference data-model.md Section 1.8 (TypeGroup::SignedIntegers)
- Follow contract C-001 pattern: Arrange (create matrix) → Act (get rule) → Assert (verify properties)
- Research.md Section 10 shows test matrix for widening conversions

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests widening
```

---

### T005: Test Unsigned Integer Widening Promotions
**File**: `tests/ir_type_promotion_tests.rs` (append after T004)  
**Priority**: MEDIUM  
**Estimated Effort**: 25 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests for unsigned integer widening conversions (U8 → U16 → U32 → U64). Verify Direct cast with ZeroExtend and no warnings.

**Acceptance Criteria**:
- [ ] Test `test_u8_to_u16_widening_no_loss()` implemented
- [ ] Test `test_u16_to_u32_widening_no_loss()` implemented
- [ ] Test `test_u32_to_u64_widening_no_loss()` implemented
- [ ] All tests verify `cast_kind: CastKind::ZeroExtend`
- [ ] All tests verify no precision loss or overflow warnings

**Implementation Guidance**:
- Reference data-model.md Section 1.8 (TypeGroup::UnsignedIntegers)
- ZeroExtend used for unsigned widening (vs SignExtend for signed)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests widening
```

---

### T006: Test Integer to Float Promotions (Exact)
**File**: `tests/ir_type_promotion_tests.rs` (append after T005)  
**Priority**: MEDIUM  
**Estimated Effort**: 30 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests for integer to float promotions where values can be represented exactly (I8 → F32, I32 → F64). Verify IntToFloat cast with no precision loss warnings.

**Acceptance Criteria**:
- [ ] Test `test_i8_to_f32_exact_representation()` implemented
- [ ] Test `test_i32_to_f64_exact_representation()` implemented
- [ ] Both tests verify `cast_kind: CastKind::IntToFloat`
- [ ] Both tests verify `may_lose_precision: false` (values fit exactly in float significand)
- [ ] Tests documented with IEEE 754 precision rationale

**Implementation Guidance**:
- F32 has 24-bit significand → can hold all I8 values exactly
- F64 has 53-bit significand → can hold all I32 values exactly
- Reference research.md Section 3 (Edge Case Category 2: Numeric Boundaries)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests int_to_float
```

---

### T007: Test Float Widening (F32 → F64)
**File**: `tests/ir_type_promotion_tests.rs` (append after T006)  
**Priority**: MEDIUM  
**Estimated Effort**: 15 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement test for float widening (F32 → F64). Verify FloatExtend cast with exact conversion (no precision loss).

**Acceptance Criteria**:
- [ ] Test `test_f32_to_f64_exact_widening()` implemented
- [ ] Verifies `cast_kind: CastKind::FloatExtend`
- [ ] Verifies `may_lose_precision: false` (F64 can represent all F32 values)
- [ ] Test documented with float widening rationale

**Implementation Guidance**:
- F32 → F64 is always exact (no precision loss)
- Reference data-model.md Section 1.8 (TypeGroup::FloatingPoint)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests float_widening
```

---

### T008: Test compute_common_type for Binary Operations
**File**: `tests/ir_type_promotion_tests.rs` (append after T007)  
**Priority**: HIGH  
**Estimated Effort**: 35 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests for `compute_common_type()` method with various binary operation type combinations. Verify correct result type selection and cast generation.

**Acceptance Criteria**:
- [ ] Test `test_compute_common_type_i32_f64_promotes_to_f64()` implemented
- [ ] Test `test_compute_common_type_i32_i64_promotes_to_i64()` implemented
- [ ] Test `test_compute_common_type_u32_i32_promotes_to_i64()` (signedness change)
- [ ] All tests verify correct `result_type` selection
- [ ] All tests verify appropriate casts in `left_cast`/`right_cast` fields
- [ ] Tests documented with type lattice rationale (higher type wins)

**Implementation Guidance**:
- Reference data-model.md Section 1.4 (PromotionResult entity)
- Test type lattice precedence: Float > Signed > Unsigned
- Verify left_cast/right_cast are Some() only when needed

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests compute_common_type
```

---

## Phase 3.3: Tests - Edge Cases (Boundary Conditions)

**CRITICAL**: These tests validate boundary conditions and type system limits. Continue appending to `tests/ir_type_promotion_tests.rs`.

### T009: Test Integer MAX Boundary Values
**File**: `tests/ir_type_promotion_tests.rs` (append after T008)  
**Priority**: HIGH  
**Estimated Effort**: 30 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for integer MAX boundary values (i32::MAX, u32::MAX, etc.). Verify widening conversions have no overflow, narrowing conversions generate warnings.

**Acceptance Criteria**:
- [ ] Test `test_i32_max_to_i64_widening_no_overflow()` implemented
- [ ] Test `test_u32_max_to_u64_widening_no_overflow()` implemented
- [ ] Test `test_u64_max_to_i64_overflow_warning()` implemented (narrowing with potential overflow)
- [ ] All tests use explicit boundary values (e.g., `i32::MAX`)
- [ ] Widening tests verify no PromotionWarning::PotentialOverflow
- [ ] Narrowing tests verify PromotionWarning::PotentialOverflow exists

**Implementation Guidance**:
- Reference research.md Section 3 (Edge Case Category 2: Numeric Boundary Conditions)
- Use actual MAX constants: `i32::MAX`, `u32::MAX`, `u64::MAX`
- Follow contract C-003 pattern with boundary value setup

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests max_boundary
```

---

### T010: Test Integer MIN Boundary Values
**File**: `tests/ir_type_promotion_tests.rs` (append after T009)  
**Priority**: HIGH  
**Estimated Effort**: 25 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for signed integer MIN boundary values (i8::MIN, i32::MIN, etc.). Verify widening preserves negative values correctly.

**Acceptance Criteria**:
- [ ] Test `test_i8_min_to_i16_widening_preserves_sign()` implemented
- [ ] Test `test_i32_min_to_i64_widening_preserves_sign()` implemented
- [ ] Tests verify SignExtend cast preserves negative values
- [ ] Tests verify no overflow warnings for widening

**Implementation Guidance**:
- Use MIN constants: `i8::MIN`, `i32::MIN`
- SignExtend must preserve sign bit correctly
- Reference data-model.md Section 1.3 (CastKind::SignExtend)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests min_boundary
```

---

### T011: Test Float Special Values (NAN, INFINITY)
**File**: `tests/ir_type_promotion_tests.rs` (append after T010)  
**Priority**: MEDIUM  
**Estimated Effort**: 30 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for float special values (f32::NAN, f32::INFINITY, f32::NEG_INFINITY). Verify promotions handle special values correctly and generate appropriate warnings.

**Acceptance Criteria**:
- [ ] Test `test_f32_nan_to_f64_preserves_nan()` implemented
- [ ] Test `test_f32_infinity_to_f64_preserves_infinity()` implemented
- [ ] Test `test_f32_neg_infinity_to_f64_preserves_neg_infinity()` implemented
- [ ] Tests verify PromotionWarning::FloatSpecialValues when appropriate
- [ ] Tests documented with IEEE 754 special value rationale

**Implementation Guidance**:
- Use `f32::NAN`, `f32::INFINITY`, `f32::NEG_INFINITY` constants
- Reference research.md Section 3 (Edge Case Category 2: Float Special Values)
- Verify warnings indicate `may_produce_nan: true` or `may_produce_inf: true`

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests float_special
```

---

### T012: Test Zero Value Promotions
**File**: `tests/ir_type_promotion_tests.rs` (append after T011)  
**Priority**: MEDIUM  
**Estimated Effort**: 20 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for zero value promotions (0_i32 → f32, 0_u8 → i16, etc.). Verify zero is preserved correctly across type boundaries.

**Acceptance Criteria**:
- [ ] Test `test_i32_zero_to_f32_exact()` implemented
- [ ] Test `test_u8_zero_to_i16_exact()` implemented
- [ ] Tests verify zero value preserved after promotion
- [ ] Tests verify no precision loss or overflow warnings

**Implementation Guidance**:
- Zero is a common edge case (boundary between positive/negative)
- Reference research.md Section 10 (Boundary Value Analysis)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests zero
```

---

### T013: Test Type System Boundary Violations
**File**: `tests/ir_type_promotion_tests.rs` (append after T012)  
**Priority**: HIGH  
**Estimated Effort**: 35 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for invalid type combinations (Bool → F32, Char → I32 without rules). Verify system returns None, Forbidden rule, or panics appropriately.

**Acceptance Criteria**:
- [ ] Test `test_bool_to_float_returns_none_or_forbidden()` implemented
- [ ] Test `test_char_to_int_returns_none_or_forbidden()` implemented
- [ ] Tests verify graceful handling (Option::None or PromotionRule::Forbidden)
- [ ] Tests documented with type system boundary rationale (invalid semantic conversions)

**Implementation Guidance**:
- Reference research.md Section 3 (Edge Case Category 1: Type System Boundary Violations)
- Check if `get_promotion_rule()` returns None or Forbidden variant
- If code panics for invalid types, create separate panic test (T018)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests boundary_violations
```

---

### T014: Test Signedness Change Warnings
**File**: `tests/ir_type_promotion_tests.rs` (append after T013)  
**Priority**: MEDIUM  
**Estimated Effort**: 25 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for promotions that change signedness (I32 → U32, U32 → I32). Verify PromotionWarning::SignednessChange is generated.

**Acceptance Criteria**:
- [ ] Test `test_i32_to_u32_signedness_change_warning()` implemented
- [ ] Test `test_u32_to_i32_signedness_change_warning()` implemented
- [ ] Both tests verify PromotionWarning::SignednessChange in warnings vec
- [ ] Tests documented with signedness rationale (potential semantic change)

**Implementation Guidance**:
- Reference data-model.md Section 1.5 (PromotionWarning::SignednessChange)
- Verify warning contains correct from_type and to_type

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests signedness
```

---

### T015: Test Precision Loss Warnings (Large Int → Small Float)
**File**: `tests/ir_type_promotion_tests.rs` (append after T014)  
**Priority**: HIGH  
**Estimated Effort**: 30 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement tests for integer to float promotions where precision is lost (I64 → F32, which has only 24-bit significand). Verify PromotionWarning::PrecisionLoss is generated.

**Acceptance Criteria**:
- [ ] Test `test_i64_to_f32_precision_loss_warning()` implemented
- [ ] Test verifies PromotionWarning::PrecisionLoss with `estimated_loss: SignificantDigits(40)` (64 - 24 bits)
- [ ] Test verifies `may_lose_precision: true` in PromotionRule
- [ ] Test documented with IEEE 754 precision rationale

**Implementation Guidance**:
- F32 significand: 24 bits, I64: 64 bits → 40 bits precision loss
- Reference data-model.md Section 1.7 (PrecisionLossEstimate::SignificantDigits)
- Reference research.md Section 3 (Numeric Boundaries - Precision Loss)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests precision_loss
```

---

## Phase 3.4: Tests - Corner Cases (Rare Combinations)

**CRITICAL**: These tests validate rare/pathological scenarios. Continue appending to `tests/ir_type_promotion_tests.rs`.

### T016: Test Deeply Nested Promotion Sequences
**File**: `tests/ir_type_promotion_tests.rs` (append after T015)  
**Priority**: MEDIUM  
**Estimated Effort**: 25 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement test for deeply nested promotion sequence (I8 → I16 → I32 → I64 → F64). Verify no stack overflow or performance degradation.

**Acceptance Criteria**:
- [ ] Test `test_deeply_nested_promotion_i8_to_f64()` implemented
- [ ] Test verifies final result type is F64
- [ ] Test completes in <100ms (performance target)
- [ ] Test documented with algorithmic bounds rationale (no exponential complexity)

**Implementation Guidance**:
- Reference research.md Section 3 (Edge Case Category 4: Resource Exhaustion)
- Verify `compute_common_type(&IrType::I8, &IrType::F64)` completes quickly
- This tests worst-case promotion chain length

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests deeply_nested
```

---

### T017: Test Circular Dependency Detection (if applicable)
**File**: `tests/ir_type_promotion_tests.rs` (append after T016)  
**Priority**: LOW (may not be testable if matrix doesn't allow construction)  
**Estimated Effort**: 30 minutes  
**Contract**: C-003 (Edge Case Tests)  
**Dependencies**: T001

**Description**:
Implement test for circular type dependency detection if the API allows constructing invalid matrices. If not possible, document why (matrix construction prevents cycles).

**Acceptance Criteria**:
- [ ] Test `test_circular_promotion_panics()` implemented (if testable) OR
- [ ] Test skipped with comment explaining matrix construction prevents circular deps
- [ ] If testable: verify panic with expected message "Circular dependency detected"
- [ ] Test documented with graph acyclic property rationale

**Implementation Guidance**:
- Reference research.md Section 3 (Edge Case Category 3: Circular Dependencies)
- If PromotionMatrix::new() prevents cycles by design, document in comment
- If testable, use #[should_panic(expected = "Circular")] attribute

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests circular
```

---

## Phase 3.5: Tests - Error Handling (Panic and Result Validation)

**CRITICAL**: These tests validate defensive programming. Continue appending to `tests/ir_type_promotion_tests.rs`.

### T018: Test Panic on Invalid Matrix State
**File**: `tests/ir_type_promotion_tests.rs` (append after T017)  
**Priority**: MEDIUM  
**Estimated Effort**: 25 minutes  
**Contract**: C-002 (Panic Tests)  
**Dependencies**: T001

**Description**:
Implement panic tests for internal consistency violations (e.g., uninitialized matrix, invalid type combination that should never occur). Use #[should_panic] attribute.

**Acceptance Criteria**:
- [ ] Test `test_uninitialized_matrix_panics()` implemented (if testable)
- [ ] Test `test_invalid_type_combination_panics()` implemented (if applicable)
- [ ] All panic tests use `#[should_panic(expected = "specific message")]`
- [ ] Tests documented with defensive programming rationale (internal invariant violations)

**Implementation Guidance**:
- Reference research.md Section 5 (Panic vs Result Error Testing Strategy)
- Follow contract C-002: specify expected panic message substring
- Only test panics for truly unrecoverable errors (not user input validation)

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests panics
```

---

### T019: Test Graceful Error Handling (Result/Option)
**File**: `tests/ir_type_promotion_tests.rs` (append after T018)  
**Priority**: MEDIUM  
**Estimated Effort**: 20 minutes  
**Contract**: C-001 (PromotionMatrix Tests)  
**Dependencies**: T001

**Description**:
Implement tests for graceful error handling where functions return Option::None or Result::Err for invalid inputs (e.g., unsupported type conversions).

**Acceptance Criteria**:
- [ ] Test `test_unsupported_conversion_returns_none()` implemented
- [ ] Test verifies Option::None for invalid type pairs
- [ ] Test verifies no panic occurs (graceful handling)
- [ ] Test documented with graceful error handling rationale (expected failure modes)

**Implementation Guidance**:
- Reference research.md Section 5 (When to Use Result Tests)
- Test API functions that return Option<T> or Result<T, E>
- Verify error messages are descriptive if Result::Err

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests error_handling
```

---

## Phase 3.6: Tests - Snapshot Testing (Optional if Complex Output)

### T020: Test Complex PromotionResult with Snapshots
**File**: `tests/ir_type_promotion_tests.rs` (append after T019)  
**Priority**: LOW (optional, only if complex results benefit from snapshots)  
**Estimated Effort**: 25 minutes  
**Contract**: C-004 (Snapshot Tests)  
**Dependencies**: T001

**Description**:
Implement snapshot tests for complex PromotionResult structures with multiple warnings. Use Insta library for regression detection.

**Acceptance Criteria**:
- [ ] Test `test_f64_to_i32_promotion_result_snapshot()` implemented
- [ ] Test uses `insta::assert_debug_snapshot!(result)`
- [ ] Test includes explicit assertions for critical properties (result_type, warnings.len())
- [ ] Snapshot file reviewed and accepted with `cargo insta review`
- [ ] Test documented with regression detection rationale

**Implementation Guidance**:
- Reference research.md Section 9 (Snapshot Testing with Insta)
- Follow contract C-004: combine snapshot with explicit assertions
- Run `cargo insta review` to accept snapshots on first run

**Validation**:
```powershell
cargo test --test ir_type_promotion_tests snapshot
cargo insta review
```

---

## Phase 3.7: Validation and Coverage

**CRITICAL**: These tasks verify 100% coverage target is met. Must run after all tests implemented.

### T021: Verify 100% Line Coverage with cargo llvm-cov
**File**: N/A (validation task)  
**Priority**: HIGH  
**Estimated Effort**: 15 minutes  
**Dependencies**: T002-T020 (all tests must be implemented)

**Description**:
Generate coverage report with cargo llvm-cov and verify src/ir/type_promotion.rs achieves 100% line coverage as required by FR-001 and FR-006.

**Acceptance Criteria**:
- [ ] Coverage report generated: `cargo llvm-cov --package jsavrs --lib --html --open`
- [ ] Line coverage for `src/ir/type_promotion.rs` is 100.00%
- [ ] Function coverage is 100% (all public functions tested)
- [ ] No red lines in HTML report
- [ ] Coverage text output shows: `src/ir/type_promotion.rs    428    428   100.00%`

**Implementation Guidance**:
- Reference quickstart.md Section "Coverage Validation (60 seconds)"
- If coverage <100%, identify uncovered lines in HTML report
- Add additional tests for uncovered lines and re-run

**Validation**:
```powershell
# Generate coverage
cargo llvm-cov --package jsavrs --lib --html --open

# Verify 100% in terminal
cargo llvm-cov --package jsavrs --lib --text | grep "src/ir/type_promotion.rs"
# Expected output: src/ir/type_promotion.rs    428    428   100.00%
```

**If Coverage <100%**:
1. Open HTML report: `target/llvm-cov/html/index.html`
2. Navigate to `src/ir/type_promotion.rs`
3. Identify red (uncovered) lines
4. Add tests for uncovered code paths
5. Re-run T021

---

### T022: Review and Accept Insta Snapshots (if applicable)
**File**: N/A (validation task)  
**Priority**: LOW (only if T020 was implemented)  
**Estimated Effort**: 10 minutes  
**Dependencies**: T020

**Description**:
Review snapshot test outputs with `cargo insta review` and accept new/changed snapshots. Verify snapshots are committed to version control.

**Acceptance Criteria**:
- [ ] All snapshots reviewed: `cargo insta review`
- [ ] Snapshots accepted (no pending changes)
- [ ] `.snapshots/` directory committed to Git
- [ ] Tests pass without snapshot prompts

**Implementation Guidance**:
- Reference quickstart.md Section "Snapshot Validation"
- Press 'a' to accept each snapshot, or 'A' to accept all
- Review snapshot content for reasonableness (no empty/malformed output)

**Validation**:
```powershell
cargo insta review
git status .snapshots/
git add .snapshots/
git commit -m "Add snapshot tests for type promotion"
```

---

### T023: Validate Documentation Completeness
**File**: N/A (validation task)  
**Priority**: HIGH  
**Estimated Effort**: 20 minutes  
**Dependencies**: T002-T020 (all tests must be implemented)

**Description**:
Verify all test functions have rustdoc comments explaining what is tested, expected outcome, and reasoning as required by FR-010.

**Acceptance Criteria**:
- [ ] All test functions have `///` rustdoc comments (not `//`)
- [ ] Each comment includes: summary, rationale, expected behavior
- [ ] Documentation follows contract C-006 template
- [ ] Cargo doc generates documentation without warnings: `cargo doc --package jsavrs --test ir_type_promotion_tests`

**Implementation Guidance**:
- Reference contracts/test-contracts.md Section "Contract: Documentation Standard (C-006)"
- Use grep to find tests without docs: `rg "^fn test_" tests/ir_type_promotion_tests.rs`
- Manually review each test's doc comment for completeness

**Validation**:
```powershell
# Generate documentation
cargo doc --package jsavrs --test ir_type_promotion_tests

# Check for undocumented tests
rg --no-heading "^    fn test_" tests/ir_type_promotion_tests.rs | wc -l
rg --no-heading "^    /// " tests/ir_type_promotion_tests.rs | wc -l
# Line counts should match (each test has doc comment)
```

---

### T024: Run Full Test Suite and Verify Performance
**File**: N/A (validation task)  
**Priority**: HIGH  
**Estimated Effort**: 10 minutes  
**Dependencies**: T002-T023 (all tests and validations complete)

**Description**:
Run full test suite and verify execution time meets performance target (<5 seconds for full suite, <100ms per test) as specified in Technical Context.

**Acceptance Criteria**:
- [ ] All tests pass: `cargo test --test ir_type_promotion_tests`
- [ ] Full suite executes in <5 seconds
- [ ] No individual test exceeds 100ms (if measurable)
- [ ] No test failures or flaky tests (run 3 times to verify)

**Implementation Guidance**:
- Reference quickstart.md Section "Performance Validation"
- Use PowerShell Measure-Command for timing: `Measure-Command { cargo test --test ir_type_promotion_tests }`
- If any test exceeds 100ms, investigate (likely I/O or computation issue)

**Validation**:
```powershell
# Run tests with timing
Measure-Command { cargo test --test ir_type_promotion_tests }
# Expected: TotalSeconds < 5.0

# Verify stability (run 3 times)
cargo test --test ir_type_promotion_tests
cargo test --test ir_type_promotion_tests
cargo test --test ir_type_promotion_tests
# All runs should pass
```

---

### T025: Final Integration Validation
**File**: N/A (validation task)  
**Priority**: HIGH  
**Estimated Effort**: 15 minutes  
**Dependencies**: T024 (all tests passing)

**Description**:
Run full project test suite to verify new tests don't disrupt existing functionality (FR-002). Validate quickstart.md workflow can be executed successfully.

**Acceptance Criteria**:
- [ ] All project tests pass: `cargo test --workspace`
- [ ] New tests execute without disrupting existing tests
- [ ] Quickstart.md validation workflow completes successfully
- [ ] No regressions introduced (existing test count unchanged)

**Implementation Guidance**:
- Reference quickstart.md Section "Quick Validation (30 seconds)"
- Run workspace-wide tests to catch integration issues
- Verify test count increased (40-60 new tests added)

**Validation**:
```powershell
# Run all tests in workspace
cargo test --workspace

# Count new tests added
rg "^    fn test_" tests/ir_type_promotion_tests.rs | wc -l
# Should be ~40-60 more than before
```

---

## Phase 3.8: Polish and Documentation

### T026: Update README.md with Coverage Badge (Optional)
**File**: `README.md`  
**Priority**: LOW  
**Estimated Effort**: 10 minutes  
**Parallelizable**: [P] Yes  
**Dependencies**: T021 (coverage verified)

**Description**:
Add coverage badge to README.md showing 100% coverage for type promotion module. Update testing section with quickstart reference.

**Acceptance Criteria**:
- [ ] Coverage badge added to README.md (shields.io or codecov format)
- [ ] Testing section references `specs/002-creating-detailed-precise/quickstart.md`
- [ ] Badge displays "coverage: 100%" (green)

**Implementation Guidance**:
- Use shields.io for static badge: `![Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen)`
- Add link to quickstart.md for test execution instructions

**Validation**:
```powershell
# Verify badge renders in GitHub markdown preview
```

---

### T027: Create Migration Summary Document
**File**: `specs/002-creating-detailed-precise/SUMMARY.md`  
**Priority**: MEDIUM  
**Estimated Effort**: 20 minutes  
**Parallelizable**: [P] Yes  
**Dependencies**: T025 (all validations complete)

**Description**:
Create summary document detailing test suite implementation: test count, coverage achieved, edge cases covered, execution time, any deviations from plan.

**Acceptance Criteria**:
- [ ] SUMMARY.md created in feature directory
- [ ] Document includes: total test count, coverage percentage, execution time, edge case categories covered
- [ ] Document notes any plan deviations or unimplemented tests (with rationale)
- [ ] Document links to key tests for each edge case category

**Implementation Guidance**:
- Use plan.md Phase 2 Section 4 (Estimated Task Count and Distribution) as template
- Include actual vs estimated metrics
- List key tests as examples of each category

**Validation**:
```powershell
# Verify SUMMARY.md exists
cat specs/002-creating-detailed-precise/SUMMARY.md
```

---

### T028: Commit and Push Feature Branch
**File**: N/A (Git operations)  
**Priority**: HIGH  
**Estimated Effort**: 5 minutes  
**Dependencies**: T027 (all tasks complete)

**Description**:
Commit all changes to feature branch `002-creating-detailed-precise` and push to remote. Prepare for pull request creation.

**Acceptance Criteria**:
- [ ] All files committed: tests, snapshots, documentation
- [ ] Commit message follows convention: "feat: Add comprehensive test suite for type promotion module"
- [ ] Branch pushed to remote: `git push origin 002-creating-detailed-precise`
- [ ] Ready for pull request creation with summary

**Implementation Guidance**:
- Use conventional commit format
- Include co-author tag if applicable
- Reference issue/ticket number in commit message

**Validation**:
```powershell
git status  # Should show "working tree clean"
git log -1  # Verify commit message
git push origin 002-creating-detailed-precise
```

---

## Dependencies Graph

```
T001 (Infrastructure)
  ├─→ T002 (PromotionMatrix Init)
  ├─→ T003 (Identity Promotions)
  ├─→ T004 (Signed Widening)
  ├─→ T005 (Unsigned Widening)
  ├─→ T006 (Int to Float Exact)
  ├─→ T007 (Float Widening)
  ├─→ T008 (compute_common_type)
  ├─→ T009 (Integer MAX)
  ├─→ T010 (Integer MIN)
  ├─→ T011 (Float Special)
  ├─→ T012 (Zero Values)
  ├─→ T013 (Type Boundary Violations)
  ├─→ T014 (Signedness Warnings)
  ├─→ T015 (Precision Loss)
  ├─→ T016 (Deeply Nested)
  ├─→ T017 (Circular Deps)
  ├─→ T018 (Panic Tests)
  ├─→ T019 (Error Handling)
  └─→ T020 (Snapshot Tests)

T002-T020 (All Tests)
  ├─→ T021 (Verify Coverage)
  ├─→ T023 (Validate Documentation)
  └─→ T024 (Performance Validation)

T020 (Snapshot Tests)
  └─→ T022 (Review Snapshots)

T024 (Performance Validation)
  └─→ T025 (Integration Validation)

T021 (Coverage) + T025 (Integration)
  ├─→ T026 (README Badge) [P]
  └─→ T027 (Summary Doc) [P]

T027 (Summary)
  └─→ T028 (Commit & Push)
```

---

## Sequential Execution Example

**Since all tests append to the same file, they must be executed sequentially:**

```powershell
# Step 1: Infrastructure
# Task T001: Create helper functions
# File: tests/ir_type_promotion_tests.rs (end of file)

# Step 2: Normal Operations (T002-T008)
# Task T002: PromotionMatrix init tests
# Task T003: Identity promotions
# ... (continue through T008)

# Step 3: Edge Cases (T009-T015)
# Task T009: Integer MAX boundaries
# ... (continue through T015)

# Step 4: Corner Cases (T016-T017)
# Task T016: Deeply nested promotions
# Task T017: Circular deps (if testable)

# Step 5: Error Handling (T018-T019)
# Task T018: Panic tests
# Task T019: Graceful error handling

# Step 6: Snapshot Tests (T020, optional)
# Task T020: Complex result snapshots

# Step 7: Validation (T021-T025)
# Task T021: Verify 100% coverage ← CRITICAL GATE
# Task T022: Review snapshots (if T020 done)
# Task T023: Validate documentation
# Task T024: Performance validation
# Task T025: Integration validation

# Step 8: Polish (T026-T027, can be parallel)
# Task T026: README badge [P]
# Task T027: Summary document [P]

# Step 9: Finalize
# Task T028: Commit and push
```

---

## Parallel Execution Opportunities (Limited)

**Note**: Since all tests write to the same file (`tests/ir_type_promotion_tests.rs`), true parallel execution is limited. However, these tasks can run in parallel with test implementation:

**Group 1: Documentation Tasks** (can run while tests are being written):
- T026 (README badge) - Different file
- T027 (Summary doc) - Different file

**Group 2: Validation Tasks** (must run after all tests complete):
- T022 (Snapshot review) - If applicable, independent of coverage
- T023 (Documentation validation) - Independent of coverage

**Sequential Dependencies**:
- T001 → T002-T020 (infrastructure blocks all tests)
- T002-T020 → T021 (tests block coverage validation)
- T021 → T026 (coverage blocks badge update)
- T021, T023, T024 → T025 (validations block integration check)
- T025 → T027 (integration blocks summary)
- T027 → T028 (summary blocks commit)

---

## Task Execution Checklist

Use this checklist to track progress:

### Phase 3.1: Setup
- [ ] T001: Helper functions and constants

### Phase 3.2: Normal Operations
- [ ] T002: PromotionMatrix initialization
- [ ] T003: Identity promotions
- [ ] T004: Signed integer widening
- [ ] T005: Unsigned integer widening
- [ ] T006: Int to float (exact)
- [ ] T007: Float widening
- [ ] T008: compute_common_type

### Phase 3.3: Edge Cases
- [ ] T009: Integer MAX boundaries
- [ ] T010: Integer MIN boundaries
- [ ] T011: Float special values
- [ ] T012: Zero value promotions
- [ ] T013: Type boundary violations
- [ ] T014: Signedness warnings
- [ ] T015: Precision loss warnings

### Phase 3.4: Corner Cases
- [ ] T016: Deeply nested sequences
- [ ] T017: Circular dependencies

### Phase 3.5: Error Handling
- [ ] T018: Panic tests
- [ ] T019: Graceful error handling

### Phase 3.6: Snapshot Tests (Optional)
- [ ] T020: Complex result snapshots

### Phase 3.7: Validation
- [ ] T021: Verify 100% coverage ⚠️ CRITICAL
- [ ] T022: Review snapshots (if applicable)
- [ ] T023: Validate documentation
- [ ] T024: Performance validation
- [ ] T025: Integration validation

### Phase 3.8: Polish
- [ ] T026: README badge
- [ ] T027: Summary document
- [ ] T028: Commit and push

---

## Validation Checklist (GATE: Before marking feature complete)

- [ ] **Coverage**: 100% line coverage verified (T021) ✅ REQUIRED
- [ ] **Tests Pass**: All tests pass in full workspace (T025) ✅ REQUIRED
- [ ] **Documentation**: All tests have rustdoc comments (T023) ✅ REQUIRED
- [ ] **Performance**: Full suite executes <5s (T024) ✅ REQUIRED
- [ ] **Edge Cases**: All 4 edge case categories covered (T009-T017) ✅ REQUIRED
- [ ] **Error Handling**: Panic and Result tests implemented (T018-T019) ✅ REQUIRED
- [ ] **No Regressions**: Existing tests unchanged (T025) ✅ REQUIRED
- [ ] **Snapshots Reviewed**: If applicable (T022) ⚠️ IF APPLICABLE
- [ ] **Documentation Updated**: README, summary created (T026-T027) ✅ RECOMMENDED
- [ ] **Committed**: Branch pushed to remote (T028) ✅ REQUIRED

---

## Notes

- **Append-Only Strategy**: All tests append to end of `tests/ir_type_promotion_tests.rs` to minimize merge conflicts (FR-002)
- **TDD Order**: Tests written before coverage validation (tests must exist before measuring coverage)
- **100% Coverage Gate**: Task T021 is CRITICAL - feature cannot be marked complete without 100% coverage
- **Performance Target**: Each test must complete in <100ms (no I/O, pure computation)
- **Documentation Standard**: Every test must have rustdoc comment (FR-010, Contract C-006)
- **No Benchmarks**: Focus on functional correctness only (FR-008, Clarifications Q3)

---

## Success Criteria

This tasks.md is complete when:
- ✅ All 28 tasks defined with clear acceptance criteria
- ✅ All 6 contract patterns represented (C-001 to C-006)
- ✅ All 8 domain entities have test coverage
- ✅ All 4 edge case categories have tests (Type, Numeric, Circular, Resource)
- ✅ Dependencies correctly ordered (Setup → Tests → Validation → Polish)
- ✅ Sequential execution path documented (same file constraint)
- ✅ Validation checklist includes all FR requirements (FR-001 to FR-010)

**Estimated Total Implementation Time**: 8-10 hours (spread across multiple sessions)

**Ready for Execution**: ✅ YES - Tasks are specific, actionable, and immediately executable by LLM or human developer.
