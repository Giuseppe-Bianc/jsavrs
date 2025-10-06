
# Tasks: Comprehensive Type Promotion Engine Test Suite

**Input**: Design documents from `/specs/003-create-detailed-precise/`
**Prerequisites**: plan.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

## Execution Flow (main)
```
1. Load plan.md from feature directory ✅
   → Tech stack: Rust 1.75+, cargo test, insta, mockall/manual mocking
   → Structure: Single project (tests/ at repository root)
2. Load design documents ✅
   → data-model.md: Test entities, fixtures, helpers identified
   → contracts/: 3 contract files (analyze_binary_promotion, insert_promotion_casts, remaining_contracts_summary)
   → research.md: Testing strategies, coverage targets documented
   → quickstart.md: Test execution workflows defined
3. Generate tasks by category:
   → Setup: Test infrastructure, dependencies, fixtures
   → Tests: All test implementations (TDD - tests before any engine changes)
   → Integration: Real PromotionMatrix tests
   → Unit: Mocked PromotionMatrix tests
   → Polish: Coverage verification, documentation, performance validation
4. Apply task rules:
   → Different test functions = mark [P] for parallel
   → Test infrastructure setup = sequential
   → All tests before any implementation changes
5. Number tasks sequentially (T001-T075)
6. Dependency graph: Setup → Tests → Verification
7. Parallel execution: Most test functions are independent [P]
8. Validation:
   → All contracts have tests ✅
   → All entities tested ✅
   → 100% coverage target ✅
9. Return: SUCCESS (tasks ready for execution)
```

---

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (independent test functions, different files)
- Include exact file paths in descriptions
- All paths are absolute from repository root: `C:\dev\vscode\rust\jsavrs\`

---

## Path Conventions
**Project Structure**: Single Rust project
- **Test File**: `tests/ir_type_promotion_engine_tests.rs` (NEW FILE)
- **System Under Test**: `src/ir/type_promotion_engine.rs`
- **Dependencies**: `Cargo.toml` (add insta to dev-dependencies)
- **Snapshots**: `tests/snapshots/ir_type_promotion_engine_tests/` (auto-generated)

---

## Phase 3.1: Test Infrastructure Setup

### Setup Tasks (Sequential - must complete before tests)

- [ ] **T001** Create test file `tests/ir_type_promotion_engine_tests.rs` with module structure, imports (`use jsavrs::ir::*`, `use insta::*`), and 8 test group module comments (analyze_binary_promotion, insert_promotion_casts, warnings, edge_cases, corner_cases, integration, unit, concurrent)

- [ ] **T002** Add `insta = "1.34"` to `Cargo.toml` under `[dev-dependencies]` section for snapshot testing support

- [ ] **T003** [P] Implement `TestFixtureBuilder` struct in `tests/ir_type_promotion_engine_tests.rs` with builder pattern methods: `new()`, `with_engine()`, `with_span()`, `with_ir_context()`, `build()` returning `TestFixture` struct

- [ ] **T004** [P] Implement `MockPromotionMatrix` struct in `tests/ir_type_promotion_engine_tests.rs` with methods: `new()`, `set_rule()`, `set_common_type()`, `get_promotion_rule()`, `compute_common_type()` for unit test isolation

- [ ] **T005** [P] Create `assertion_helpers` module in `tests/ir_type_promotion_engine_tests.rs` with helper functions: `assert_result_type()`, `assert_left_cast()`, `assert_right_cast()`, `assert_has_warning()`, `assert_is_sound()`

- [ ] **T006** [P] Create `test_types` constant module in `tests/ir_type_promotion_engine_tests.rs` with arrays: `ALL_SIGNED_INTEGERS`, `ALL_UNSIGNED_INTEGERS`, `ALL_FLOATS`, `ALL_INTEGERS`, `ALL_NUMERIC`, `ALL_TYPES` (12 IrType variants)

- [ ] **T007** [P] Create `test_operations` constant module in `tests/ir_type_promotion_engine_tests.rs` with arrays: `ARITHMETIC_OPS`, `COMPARISON_OPS`, `LOGICAL_OPS`, `BITWISE_OPS`, `ALL_OPS` (18 IrBinaryOp variants)

---

## Phase 3.2: Core Function Tests - analyze_binary_promotion (TDD)

**CRITICAL: These tests MUST be written and MUST compile/run (failing or passing based on current implementation) before ANY changes to TypePromotionEngine**

### Identity Promotion Tests (12 tests - can run in parallel)

- [ ] **T008** [P] Test `test_analyze_binary_promotion_identity_i8_add` in `tests/ir_type_promotion_engine_tests.rs`: Validate I8+I8→I8 with no casts, no warnings, is_sound=true

- [ ] **T009** [P] Test `test_analyze_binary_promotion_identity_i16_add` in `tests/ir_type_promotion_engine_tests.rs`: Validate I16+I16→I16 with no casts, no warnings, is_sound=true

- [ ] **T010** [P] Test `test_analyze_binary_promotion_identity_i32_add` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32+I32→I32 with no casts, no warnings, is_sound=true

- [ ] **T011** [P] Test `test_analyze_binary_promotion_identity_i64_add` in `tests/ir_type_promotion_engine_tests.rs`: Validate I64+I64→I64 with no casts, no warnings, is_sound=true

- [ ] **T012** [P] Test `test_analyze_binary_promotion_identity_u8_u16_u32_u64` in `tests/ir_type_promotion_engine_tests.rs`: Validate U8+U8→U8, U16+U16→U16, U32+U32→U32, U64+U64→U64 (combined test for unsigned integers)

- [ ] **T013** [P] Test `test_analyze_binary_promotion_identity_f32_add` in `tests/ir_type_promotion_engine_tests.rs`: Validate F32+F32→F32 with no casts, no warnings, is_sound=true

- [ ] **T014** [P] Test `test_analyze_binary_promotion_identity_f64_add` in `tests/ir_type_promotion_engine_tests.rs`: Validate F64+F64→F64 with no casts, no warnings, is_sound=true

- [ ] **T015** [P] Test `test_analyze_binary_promotion_identity_bool_char` in `tests/ir_type_promotion_engine_tests.rs`: Validate Bool+Bool→Bool, Char+Char→Char identity promotions

### Widening Promotion Tests (12 tests - representative subset)

- [ ] **T016** [P] Test `test_analyze_binary_promotion_widening_i8_to_i16` in `tests/ir_type_promotion_engine_tests.rs`: Validate I8+I16→I16 with left cast IntSignExtend, no warnings

- [ ] **T017** [P] Test `test_analyze_binary_promotion_widening_i8_to_i32` in `tests/ir_type_promotion_engine_tests.rs`: Validate I8+I32→I32 with left cast IntSignExtend, no warnings

- [ ] **T018** [P] Test `test_analyze_binary_promotion_widening_i8_to_i64` in `tests/ir_type_promotion_engine_tests.rs`: Validate I8+I64→I64 with left cast IntSignExtend, no warnings

- [ ] **T019** [P] Test `test_analyze_binary_promotion_widening_i16_to_i32` in `tests/ir_type_promotion_engine_tests.rs`: Validate I16+I32→I32 with left cast IntSignExtend, no warnings

- [ ] **T020** [P] Test `test_analyze_binary_promotion_widening_i32_to_i64` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32+I64→I64 with left cast IntSignExtend, no warnings

- [ ] **T021** [P] Test `test_analyze_binary_promotion_widening_u8_to_u16_u32_u64` in `tests/ir_type_promotion_engine_tests.rs`: Validate U8 widening to U16, U32, U64 with IntZeroExtend casts

- [ ] **T022** [P] Test `test_analyze_binary_promotion_widening_u16_to_u32_u64` in `tests/ir_type_promotion_engine_tests.rs`: Validate U16 widening to U32, U64 with IntZeroExtend casts

- [ ] **T023** [P] Test `test_analyze_binary_promotion_widening_u32_to_u64` in `tests/ir_type_promotion_engine_tests.rs`: Validate U32+U64→U64 with left cast IntZeroExtend, no warnings

### Cross-Signedness Promotion Tests (4 tests)

- [ ] **T024** [P] Test `test_analyze_binary_promotion_cross_signedness_i8_u8` in `tests/ir_type_promotion_engine_tests.rs`: Validate I8+U8 promotion with SignednessChange warning, promoted to next larger signed type

- [ ] **T025** [P] Test `test_analyze_binary_promotion_cross_signedness_i16_u16` in `tests/ir_type_promotion_engine_tests.rs`: Validate I16+U16 promotion with SignednessChange warning

- [ ] **T026** [P] Test `test_analyze_binary_promotion_cross_signedness_i32_u32` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32+U32→I64 promotion with SignednessChange warning, is_sound=false

- [ ] **T027** [P] Test `test_analyze_binary_promotion_cross_signedness_i64_u64` in `tests/ir_type_promotion_engine_tests.rs`: Validate I64+U64→I64 promotion with SignednessChange warning

### Integer-Float Promotion Tests (8 tests - representative subset)

- [ ] **T028** [P] Test `test_analyze_binary_promotion_i32_to_f32` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32+F32→F32 with left IntToFloat cast, no precision loss

- [ ] **T029** [P] Test `test_analyze_binary_promotion_i64_to_f32_precision_loss` in `tests/ir_type_promotion_engine_tests.rs`: Validate I64+F32→F32 with left IntToFloat cast, PrecisionLoss warning for large values

- [ ] **T030** [P] Test `test_analyze_binary_promotion_i32_to_f64` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32+F64→F64 with left IntToFloat cast, no warnings

- [ ] **T031** [P] Test `test_analyze_binary_promotion_i64_to_f64` in `tests/ir_type_promotion_engine_tests.rs`: Validate I64+F64→F64 with left IntToFloat cast, no warnings

- [ ] **T032** [P] Test `test_analyze_binary_promotion_u32_to_f32` in `tests/ir_type_promotion_engine_tests.rs`: Validate U32+F32→F32 with left IntToFloat cast

- [ ] **T033** [P] Test `test_analyze_binary_promotion_u64_to_f64` in `tests/ir_type_promotion_engine_tests.rs`: Validate U64+F64→F64 with left IntToFloat cast

- [ ] **T034** [P] Test `test_analyze_binary_promotion_all_integers_to_f32` in `tests/ir_type_promotion_engine_tests.rs`: Parameterized test for all 8 integer types promoting to F32

- [ ] **T035** [P] Test `test_analyze_binary_promotion_all_integers_to_f64` in `tests/ir_type_promotion_engine_tests.rs`: Parameterized test for all 8 integer types promoting to F64

### Float Promotion Tests (2 tests)

- [ ] **T036** [P] Test `test_analyze_binary_promotion_f32_to_f64_widening` in `tests/ir_type_promotion_engine_tests.rs`: Validate F32+F64→F64 with left FloatExtend cast, no precision loss

- [ ] **T037** [P] Test `test_analyze_binary_promotion_f64_to_f32_narrowing` in `tests/ir_type_promotion_engine_tests.rs`: Validate F64+F32→F32 (or F64 as result) with PrecisionLoss warning (SignificantDigits)

### Operation-Specific Tests (18 tests - one per operation category)

- [ ] **T038** [P] Test `test_analyze_binary_promotion_arithmetic_operations` in `tests/ir_type_promotion_engine_tests.rs`: Test Add, Subtract, Multiply, Divide, Modulo with I32+F32 (5 operations, validate result_type=F32)

- [ ] **T039** [P] Test `test_analyze_binary_promotion_comparison_operations` in `tests/ir_type_promotion_engine_tests.rs`: Test Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual with I32+F32 (6 operations, validate result_type=Bool or promotion behavior)

- [ ] **T040** [P] Test `test_analyze_binary_promotion_logical_operations` in `tests/ir_type_promotion_engine_tests.rs`: Test And, Or with Bool types

- [ ] **T041** [P] Test `test_analyze_binary_promotion_bitwise_operations` in `tests/ir_type_promotion_engine_tests.rs`: Test BitwiseAnd, BitwiseOr, BitwiseXor, ShiftLeft, ShiftRight with I32+U32 (mixed signedness)

- [ ] **T042** [P] Test `test_analyze_binary_promotion_division_special_handling` in `tests/ir_type_promotion_engine_tests.rs`: Test Divide operation with potential overflow scenarios

---

## Phase 3.3: Core Function Tests - insert_promotion_casts (TDD)

### No Casts Required Tests (3 tests - representative)

- [ ] **T043** [P] Test `test_insert_promotion_casts_no_casts_identity_i32` in `tests/ir_type_promotion_engine_tests.rs`: Validate that I32+I32 identity promotion returns original values unchanged, no IR instructions inserted

- [ ] **T044** [P] Test `test_insert_promotion_casts_no_casts_identity_f64` in `tests/ir_type_promotion_engine_tests.rs`: Validate that F64+F64 identity promotion returns original values unchanged

- [ ] **T045** [P] Test `test_insert_promotion_casts_no_casts_identity_all_types` in `tests/ir_type_promotion_engine_tests.rs`: Parameterized test for all 12 IrType identity promotions

### Left Operand Cast Tests (5 tests)

- [ ] **T046** [P] Test `test_insert_promotion_casts_left_only_i8_to_i32_sign_extend` in `tests/ir_type_promotion_engine_tests.rs`: Validate I8→I32 cast insertion, verify CastKind::IntSignExtend, returned value has type I32, cast instruction in IR

- [ ] **T047** [P] Test `test_insert_promotion_casts_left_only_u8_to_u32_zero_extend` in `tests/ir_type_promotion_engine_tests.rs`: Validate U8→U32 cast insertion, verify CastKind::IntZeroExtend

- [ ] **T048** [P] Test `test_insert_promotion_casts_left_only_i32_to_f32_int_to_float` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32→F32 cast insertion, verify CastKind::IntToFloat

- [ ] **T049** [P] Test `test_insert_promotion_casts_left_only_f32_to_f64_float_extend` in `tests/ir_type_promotion_engine_tests.rs`: Validate F32→F64 cast insertion, verify CastKind::FloatExtend

- [ ] **T050** [P] Test `test_insert_promotion_casts_left_only_source_span_preservation` in `tests/ir_type_promotion_engine_tests.rs`: Validate that cast instructions preserve SourceSpan from input

### Right Operand Cast Tests (3 tests)

- [ ] **T051** [P] Test `test_insert_promotion_casts_right_only_i16_to_i64` in `tests/ir_type_promotion_engine_tests.rs`: Validate right operand I16→I64 cast insertion, left operand unchanged

- [ ] **T052** [P] Test `test_insert_promotion_casts_right_only_u32_to_u64` in `tests/ir_type_promotion_engine_tests.rs`: Validate right operand U32→U64 cast insertion

- [ ] **T053** [P] Test `test_insert_promotion_casts_right_only_i64_to_f64` in `tests/ir_type_promotion_engine_tests.rs`: Validate right operand I64→F64 cast insertion

### Both Operands Cast Tests (3 tests)

- [ ] **T054** [P] Test `test_insert_promotion_casts_both_i32_u32_to_i64_bilateral` in `tests/ir_type_promotion_engine_tests.rs`: Validate I32+U32→I64 requires both operands cast, verify two cast instructions inserted in order (left first, then right)

- [ ] **T055** [P] Test `test_insert_promotion_casts_both_i8_u8_to_i16` in `tests/ir_type_promotion_engine_tests.rs`: Validate bilateral casting for I8+U8→I16

- [ ] **T056** [P] Test `test_insert_promotion_casts_both_operands_verify_instruction_order` in `tests/ir_type_promotion_engine_tests.rs`: Validate that left cast instruction is inserted before right cast instruction

### CastKind Validation Tests (7 tests - one per cast type)

- [ ] **T057** [P] Test `test_insert_promotion_casts_cast_kind_int_sign_extend` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::IntSignExtend used for signed integer widening (I8→I16, I16→I32, I32→I64)

- [ ] **T058** [P] Test `test_insert_promotion_casts_cast_kind_int_zero_extend` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::IntZeroExtend used for unsigned integer widening

- [ ] **T059** [P] Test `test_insert_promotion_casts_cast_kind_int_truncate` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::IntTruncate used for integer narrowing (if applicable)

- [ ] **T060** [P] Test `test_insert_promotion_casts_cast_kind_int_to_float` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::IntToFloat used for integer to float conversions

- [ ] **T061** [P] Test `test_insert_promotion_casts_cast_kind_float_to_int` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::FloatToInt used for float to integer conversions

- [ ] **T062** [P] Test `test_insert_promotion_casts_cast_kind_float_extend` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::FloatExtend used for F32→F64

- [ ] **T063** [P] Test `test_insert_promotion_casts_cast_kind_float_truncate` in `tests/ir_type_promotion_engine_tests.rs`: Validate CastKind::FloatTruncate used for F64→F32

---

## Phase 3.4: Warning Generation Tests (TDD)

### PrecisionLoss Warning Tests (5 tests)

- [ ] **T064** [P] Test `test_warning_precision_loss_f64_to_f32_significant_digits` in `tests/ir_type_promotion_engine_tests.rs`: Validate PrecisionLoss warning for F64→F32 with estimated_loss=SignificantDigits, verify warning message content

- [ ] **T065** [P] Test `test_warning_precision_loss_i64_to_f32_large_value` in `tests/ir_type_promotion_engine_tests.rs`: Validate PrecisionLoss warning for large I64→F32 conversion

- [ ] **T066** [P] Test `test_warning_precision_loss_float_to_int_fractional` in `tests/ir_type_promotion_engine_tests.rs`: Validate PrecisionLoss warning for F32→I32 with estimated_loss=FractionalPart

- [ ] **T067** [P] Test `test_warning_precision_loss_integer_narrowing_value_range` in `tests/ir_type_promotion_engine_tests.rs`: Validate PrecisionLoss warning for I64→I32 narrowing with estimated_loss=ValueRange

- [ ] **T068** [P] Test `test_warning_precision_loss_message_format_validation` in `tests/ir_type_promotion_engine_tests.rs`: Validate detailed message content and format for all PrecisionLoss scenarios

### PotentialOverflow Warning Tests (3 tests)

- [ ] **T069** [P] Test `test_warning_potential_overflow_f64_to_i32_out_of_range` in `tests/ir_type_promotion_engine_tests.rs`: Validate PotentialOverflow warning when float value exceeds integer range

- [ ] **T070** [P] Test `test_warning_potential_overflow_division_int_min_by_neg_one` in `tests/ir_type_promotion_engine_tests.rs`: Validate PotentialOverflow warning for division overflow scenario (INT_MIN / -1)

- [ ] **T071** [P] Test `test_warning_potential_overflow_u64_to_i32_exceeds_range` in `tests/ir_type_promotion_engine_tests.rs`: Validate PotentialOverflow warning for value exceeding target type capacity

### SignednessChange Warning Tests (3 tests)

- [ ] **T072** [P] Test `test_warning_signedness_change_i32_u32_mixed_signedness` in `tests/ir_type_promotion_engine_tests.rs`: Validate SignednessChange warning for I32+U32, verify from_signed=true, to_signed (depends on result), may_affect_comparisons=true

- [ ] **T073** [P] Test `test_warning_signedness_change_comparison_signed_unsigned` in `tests/ir_type_promotion_engine_tests.rs`: Validate SignednessChange warning in comparison operations with mixed signedness

- [ ] **T074** [P] Test `test_warning_signedness_change_message_format` in `tests/ir_type_promotion_engine_tests.rs`: Validate detailed message content explaining comparison impact

### Multiple Warnings Tests (2 tests)

- [ ] **T075** [P] Test `test_warning_multiple_u64_to_i32_all_warnings` in `tests/ir_type_promotion_engine_tests.rs`: Validate U64→I32 generates SignednessChange + PrecisionLoss + PotentialOverflow warnings simultaneously

- [ ] **T076** [P] Test `test_warning_multiple_warnings_is_sound_false` in `tests/ir_type_promotion_engine_tests.rs`: Validate that presence of any warning sets is_sound=false in PromotionResult

---

## Phase 3.5: Edge Case Tests (TDD)

### Type Boundary Tests (5 tests)

- [ ] **T077** [P] Test `test_edge_case_i8_to_i64_smallest_to_largest` in `tests/ir_type_promotion_engine_tests.rs`: Validate promotion from smallest signed integer to largest (I8→I64) with no precision loss

- [ ] **T078** [P] Test `test_edge_case_u8_to_u64_smallest_to_largest` in `tests/ir_type_promotion_engine_tests.rs`: Validate promotion from smallest unsigned integer to largest (U8→U64)

- [ ] **T079** [P] Test `test_edge_case_i32_max_value_promotion` in `tests/ir_type_promotion_engine_tests.rs`: Test promotion with i32::MAX value at type boundary

- [ ] **T080** [P] Test `test_edge_case_i64_u64_same_width_cross_signedness` in `tests/ir_type_promotion_engine_tests.rs`: Validate I64↔U64 same-width different-signedness edge case

- [ ] **T081** [P] Test `test_edge_case_boundary_values_all_numeric_types` in `tests/ir_type_promotion_engine_tests.rs`: Parameterized test with min/max values for all numeric types

### Float-Integer Boundary Tests (4 tests)

- [ ] **T082** [P] Test `test_edge_case_float_nan_to_integer_conversion` in `tests/ir_type_promotion_engine_tests.rs`: Validate handling of NaN in float-to-integer conversions (may require FloatSpecialValues warning)

- [ ] **T083** [P] Test `test_edge_case_float_infinity_to_integer_conversion` in `tests/ir_type_promotion_engine_tests.rs`: Validate handling of Infinity and -Infinity in float-to-integer conversions

- [ ] **T084** [P] Test `test_edge_case_large_integer_to_f32_precision_loss` in `tests/ir_type_promotion_engine_tests.rs`: Validate precision loss for integers > 2^24 converting to F32

- [ ] **T085** [P] Test `test_edge_case_float_max_min_values` in `tests/ir_type_promotion_engine_tests.rs`: Test promotions with f32::MAX, f32::MIN, f64::MAX, f64::MIN

### Promotion Matrix Edge Tests (3 tests)

- [ ] **T086** [P] Test `test_edge_case_compute_common_type_none_fallback` in `tests/ir_type_promotion_engine_tests.rs`: Test behavior when PromotionMatrix::compute_common_type() returns None (fallback to left_type)

- [ ] **T087** [P] Test `test_edge_case_get_promotion_rule_none_no_cast` in `tests/ir_type_promotion_engine_tests.rs`: Test behavior when PromotionMatrix::get_promotion_rule() returns None

- [ ] **T088** [P] Test `test_edge_case_bidirectional_promotion_symmetry` in `tests/ir_type_promotion_engine_tests.rs`: Validate that A→B and B→A promotions follow expected rules

### Operation-Specific Edge Tests (3 tests)

- [ ] **T089** [P] Test `test_edge_case_division_overflow_int_min_by_neg_one` in `tests/ir_type_promotion_engine_tests.rs`: Test division operation with INT_MIN / -1 overflow scenario

- [ ] **T090** [P] Test `test_edge_case_bitwise_operations_signed_integers` in `tests/ir_type_promotion_engine_tests.rs`: Test bitwise operations with signed integers (may have special handling)

- [ ] **T091** [P] Test `test_edge_case_modulo_negative_operands` in `tests/ir_type_promotion_engine_tests.rs`: Test modulo operation with negative operands

---

## Phase 3.6: Corner Case Tests (TDD)

### Multi-Warning Scenarios (3 tests)

- [ ] **T092** [P] Test `test_corner_case_multiple_warnings_u64_to_i32` in `tests/ir_type_promotion_engine_tests.rs`: Validate 3+ warnings in single promotion (SignednessChange + PrecisionLoss + PotentialOverflow)

- [ ] **T093** [P] Test `test_corner_case_cascading_warnings_promotion_chain` in `tests/ir_type_promotion_engine_tests.rs`: Test promotion chains with accumulated warnings

- [ ] **T094** [P] Test `test_corner_case_warning_message_content_validation` in `tests/ir_type_promotion_engine_tests.rs`: Validate all warning messages follow expected format and contain accurate metadata

### System Boundary Tests (2 tests)

- [ ] **T095** [P] Test `test_corner_case_missing_promotion_rules_graceful_degradation` in `tests/ir_type_promotion_engine_tests.rs`: Test graceful fallback behavior when promotion rules are missing

- [ ] **T096** [P] Test `test_corner_case_invalid_type_combinations` in `tests/ir_type_promotion_engine_tests.rs`: Test behavior with unusual type combinations (if possible to construct)

### Helper Method Validation (Integrated - 1 test)

- [ ] **T097** [P] Test `test_corner_case_helper_methods_through_engine_usage` in `tests/ir_type_promotion_engine_tests.rs`: Validate is_signed_integer(), is_unsigned_integer(), get_bit_width() work correctly through engine behavior by explicitly iterating through all 12 IrType variants (I8, I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Char) and verifying helper method return values for each variant (not separate tests per spec clarification)

---

## Phase 3.7: Integration Tests with Real PromotionMatrix (TDD)

### Real-World Scenarios (5 tests)

- [ ] **T098** [P] Test `test_integration_real_matrix_common_programming_scenarios` in `tests/ir_type_promotion_engine_tests.rs`: Test common type combinations used in real programs (I32+F32, I64+F64, U32+I32) with real PromotionMatrix

- [ ] **T099** [P] Test `test_integration_real_matrix_complex_multi_step_promotions` in `tests/ir_type_promotion_engine_tests.rs`: Test operations involving multiple casts with real matrix

- [ ] **T100** [P] Test `test_integration_real_matrix_warning_generation` in `tests/ir_type_promotion_engine_tests.rs`: Validate warning generation with real matrix rules

- [ ] **T101** [P] Test `test_integration_real_matrix_all_operations` in `tests/ir_type_promotion_engine_tests.rs`: Test all 18 IrBinaryOp variants with real PromotionMatrix

- [ ] **T102** [P] Test `test_integration_real_matrix_performance_acceptable` in `tests/ir_type_promotion_engine_tests.rs`: Validate acceptable performance with real matrix (<100ms per promotion)

---

## Phase 3.8: Unit Tests with Mocked PromotionMatrix (TDD)

### Matrix Behavior Isolation (4 tests)

- [ ] **T103** [P] Test `test_unit_mocked_matrix_compute_common_type_specific_values` in `tests/ir_type_promotion_engine_tests.rs`: Mock compute_common_type() to return specific types, verify engine uses result correctly

- [ ] **T104** [P] Test `test_unit_mocked_matrix_get_promotion_rule_specific_rules` in `tests/ir_type_promotion_engine_tests.rs`: Mock get_promotion_rule() to return specific rules, verify engine applies them

- [ ] **T105** [P] Test `test_unit_mocked_matrix_overflow_behavior_configuration` in `tests/ir_type_promotion_engine_tests.rs`: Mock different overflow behaviors, verify engine respects configuration

- [ ] **T106** [P] Test `test_unit_mocked_matrix_engine_logic_independence` in `tests/ir_type_promotion_engine_tests.rs`: Validate engine decision-making logic independently of matrix implementation

### Error Path Tests (2 tests)

- [ ] **T107** [P] Test `test_unit_mocked_matrix_compute_common_type_none_fallback` in `tests/ir_type_promotion_engine_tests.rs`: Mock compute_common_type() to return None, verify fallback to left_type

- [ ] **T108** [P] Test `test_unit_mocked_matrix_get_promotion_rule_none_no_cast` in `tests/ir_type_promotion_engine_tests.rs`: Mock get_promotion_rule() to return None, verify no cast generated

---

## Phase 3.9: Concurrent Execution Tests (TDD)

### Thread-Safety Validation (3 tests)

- [ ] **T109** [P] Test `test_concurrent_execution_10_threads_100_operations` in `tests/ir_type_promotion_engine_tests.rs`: Create Arc<TypePromotionEngine>, spawn 10 threads each performing 100 analyze_binary_promotion calls, verify all complete without panics and produce consistent results

- [ ] **T110** [P] Test `test_concurrent_execution_varied_type_combinations` in `tests/ir_type_promotion_engine_tests.rs`: Test multiple threads analyzing different type pairs simultaneously (I32+F32, U64+I64, F32+F64), verify result independence

- [ ] **T111** [P] Test `test_concurrent_execution_high_thread_count_stress_test` in `tests/ir_type_promotion_engine_tests.rs`: Stress test with 50-100 threads performing promotions, verify system stability and result consistency

---

## Phase 3.10: Coverage Verification & Polish

### Coverage & Documentation (7 tasks - sequential)

- [ ] **T112** Run `cargo test ir_type_promotion_engine_tests` to execute full test suite, verify all tests pass or document expected failures

- [ ] **T113** Run `cargo install cargo-llvm-cov` and `cargo llvm-cov --html --package jsavrs --test ir_type_promotion_engine_tests` to generate coverage report in `target/llvm-cov/html/`

- [ ] **T114** Open coverage report, verify 100% line coverage for `src/ir/type_promotion_engine.rs`, document any uncovered lines (red highlights) and add tests to cover them

- [ ] **T115** Verify 100% branch coverage for all conditional logic in `src/ir/type_promotion_engine.rs` using coverage report, add tests for any uncovered branches

- [ ] **T116** Run `cargo install cargo-insta` and `cargo insta review` to review all generated snapshots in `tests/snapshots/ir_type_promotion_engine_tests/`, accept correct snapshots with `cargo insta accept`

- [ ] **T117** Add comprehensive module-level documentation to `tests/ir_type_promotion_engine_tests.rs` explaining test organization, fixture usage, and assertion strategies (following Rust documentation standards)

- [ ] **T118** Measure test execution performance with `time cargo test ir_type_promotion_engine_tests --release`, verify total execution time <10 seconds, individual tests <100ms (except concurrent tests <2s)

---

## Dependencies

### Sequential Dependencies:
1. **Setup → Tests**: T001-T007 (infrastructure) must complete before any test implementation (T008-T111)
2. **Tests → Coverage**: All tests (T008-T111) must be implemented before coverage verification (T112-T115)
3. **Coverage → Documentation**: Coverage verification (T112-T115) before final documentation (T116-T118)

### Parallel Execution:
- **Infrastructure Tasks**: T003-T007 [P] (different sections of test file, no conflicts)
- **All Test Functions**: T008-T111 [P] (independent test functions, can run in parallel)
- **Setup Tasks Sequential**: T001-T002 must complete first (file creation, dependencies)
- **Polish Tasks Sequential**: T112-T118 (each depends on previous completion)

---

## Parallel Execution Examples

### Example 1: Infrastructure Setup (Parallel)
```bash
# After T001-T002 complete, launch T003-T007 together:
Task: "Implement TestFixtureBuilder in tests/ir_type_promotion_engine_tests.rs"
Task: "Implement MockPromotionMatrix in tests/ir_type_promotion_engine_tests.rs"
Task: "Create assertion_helpers module in tests/ir_type_promotion_engine_tests.rs"
Task: "Create test_types constants in tests/ir_type_promotion_engine_tests.rs"
Task: "Create test_operations constants in tests/ir_type_promotion_engine_tests.rs"
```

### Example 2: Identity Promotion Tests (Parallel)
```bash
# Launch T008-T015 together (all identity tests):
Task: "Test identity promotion I8+I8→I8"
Task: "Test identity promotion I16+I16→I16"
Task: "Test identity promotion I32+I32→I32"
Task: "Test identity promotion I64+I64→I64"
Task: "Test identity promotion unsigned integers"
Task: "Test identity promotion F32+F32→F32"
Task: "Test identity promotion F64+F64→F64"
Task: "Test identity promotion Bool, Char"
```

### Example 3: Warning Tests (Parallel)
```bash
# Launch T064-T076 together (all warning tests):
Task: "Test PrecisionLoss warning F64→F32"
Task: "Test PrecisionLoss warning I64→F32"
Task: "Test PrecisionLoss warning float→int"
Task: "Test PotentialOverflow warning scenarios"
Task: "Test SignednessChange warning scenarios"
Task: "Test multiple warnings simultaneously"
```

---

## Task Consolidation Summary

**Total Discrete Tasks**: 118 tasks
- **Phase 3.1 (Setup)**: 7 tasks (T001-T007)
- **Phase 3.2 (analyze_binary_promotion tests)**: 35 tests (T008-T042)
- **Phase 3.3 (insert_promotion_casts tests)**: 21 tests (T043-T063)
- **Phase 3.4 (Warning tests)**: 13 tests (T064-T076)
- **Phase 3.5 (Edge case tests)**: 15 tests (T077-T091)
- **Phase 3.6 (Corner case tests)**: 6 tests (T092-T097)
- **Phase 3.7 (Integration tests)**: 5 tests (T098-T102)
- **Phase 3.8 (Unit tests with mocks)**: 6 tests (T103-T108)
- **Phase 3.9 (Concurrent tests)**: 3 tests (T109-T111)
- **Phase 3.10 (Coverage & polish)**: 7 tasks (T112-T118)

**Parallel Tasks**: 104 tasks marked [P] (88% can run in parallel)
**Sequential Tasks**: 14 tasks (12% must run sequentially)

---

## Validation Checklist

**GATE: Checked before task execution**

- [x] All contracts have corresponding tests ✅
  - analyze_binary_promotion_contract.md → T008-T042 (35 tests)
  - insert_promotion_casts_contract.md → T043-T063 (21 tests)
  - remaining_contracts_summary.md → T064-T111 (47 tests)

- [x] All entities have test coverage ✅
  - TypePromotionEngine → T008-T111 (all function tests)
  - PromotionResult → T008-T111 (validated in all tests)
  - TypePromotion → T043-T063 (cast tests)
  - PromotionWarning → T064-T076 (warning tests)

- [x] All tests come before implementation ✅
  - Tests are for existing implementation (TypePromotionEngine already exists)
  - No implementation changes required - only comprehensive test coverage

- [x] Parallel tasks truly independent ✅
  - Each test function in separate test case
  - No shared mutable state between tests
  - Infrastructure helpers in separate modules

- [x] Each task specifies exact file path ✅
  - All tasks specify `tests/ir_type_promotion_engine_tests.rs`
  - Infrastructure setup specifies `Cargo.toml` for dependencies

- [x] No task modifies same file section as another [P] task ✅
  - Test functions are independent
  - Infrastructure tasks (T003-T007) modify different modules within test file

---

## Notes

- **[P] Marking**: 104 tasks (88%) can execute in parallel - primarily independent test functions
- **TDD Approach**: Tests validate existing TypePromotionEngine implementation - no engine changes required
- **Snapshot Testing**: Will use `insta` crate - snapshots generated/accepted in T116
- **Coverage Target**: 100% line and branch coverage verified in T114-T115
- **Test Organization**: 8 logical test groups with clear module comments
- **Execution Time**: Target <10 seconds for full suite (T118 verification)
- **Community Principles**: Tests exemplify Shared Learning (clear examples), Quality Through Community (comprehensive coverage), Documentation Rigor (detailed comments)

---

## Success Criteria

**Test Suite Complete When**:
1. ✅ All 111 test functions implemented (T008-T118)
2. ✅ 100% line coverage for `src/ir/type_promotion_engine.rs` (T114)
3. ✅ 100% branch coverage verified (T115)
4. ✅ All snapshots reviewed and accepted (T116)
5. ✅ Test execution time <10 seconds (T118)
6. ✅ All tests pass: `cargo test ir_type_promotion_engine_tests`
7. ✅ Documentation complete (T117)

---

**Tasks.md Status**: ✅ COMPLETE - Ready for implementation
**Next Step**: Begin with Phase 3.1 Setup (T001-T007), then proceed to test implementation
