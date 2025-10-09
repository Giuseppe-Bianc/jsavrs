# Implementation Tasks: Comprehensive Type Casting System Enhancement

**Feature**: `004-enhance-the-casting`  
**Branch**: `004-enhance-the-casting`  
**Generated**: 2025-10-08  
**Status**: Ready for implementation

---

## Task Overview

This document provides a dependency-ordered, actionable task list for implementing comprehensive type casting support across all 13 fundamental data types in the jsavrs compiler. Tasks are organized by user story to enable independent implementation and testing.

### Total Task Count: 42 tasks
- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 5 tasks  
- **Phase 3 (US1 - Basic Numeric Conversions)**: 15 tasks
- **Phase 4 (US2 - Boolean and Character Conversions)**: 10 tasks
- **Phase 5 (US3 - String Conversions)**: 6 tasks
- **Phase 6 (Polish & Integration)**: 3 tasks

### Parallel Execution Opportunities: 18 tasks marked [P]

### Implementation Strategy
- **MVP Scope**: User Story 1 (Basic Numeric Type Conversions) - Foundation for all other conversions
- **Incremental Delivery**: Each user story phase is independently testable and deliverable
- **Test Strategy**: Tests explicitly requested in spec - TDD approach with tests before implementation

---

## Phase 1: Setup & Infrastructure

**Goal**: Prepare development environment and validate existing infrastructure

### T001: Verify CastKind Enum Completeness [X]
**File**: `src/ir/instruction.rs`  
**Description**: Read and verify that all 24 CastKind variants exist as documented in research.md  
**Validation**: 
```rust
// Verify these variants exist:
IntZeroExtend, IntSignExtend, IntTruncate, IntBitcast,
IntToFloat, FloatToInt, FloatTruncate, FloatExtend,
BoolToInt, IntToBool, BoolToFloat, FloatToBool,
CharToInt, IntToChar, CharToString, StringToChar,
StringToInt, StringToFloat, StringToBool,
IntToString, FloatToString, BoolToString, Bitcast
```
**Dependencies**: None  
**Story**: Setup
**Status**: [X] COMPLETED - All 24 CastKind variants verified

---

### T002: Verify IrType Enum Coverage [X]
**File**: `src/ir/types.rs`  
**Description**: Read and confirm all 13 fundamental types are defined in IrType enum  
**Validation**: 
```rust
// Verify these variants exist:
I8, I16, I32, I64, U8, U16, U32, U64,
F32, F64, Bool, Char, String
```
**Dependencies**: None  
**Story**: Setup
**Status**: [X] COMPLETED - All 13 fundamental types verified

---

### T003: Review Existing PromotionMatrix Implementation [X]
**File**: `src/ir/type_promotion.rs`  
**Description**: Read lines 1-498 to understand current PromotionMatrix structure, initialization logic, and helper methods  
**Validation**: Confirm understanding of:
- `promotion_rules: HashMap<(IrType, IrType), PromotionRule>`
- `initialize_default_promotions()` method
- Existing helper methods: `add_integer_widening_promotions()`, `add_float_integer_promotions()`, `add_cross_signedness_promotions()`, `add_identity_promotions()`
**Dependencies**: None  
**Story**: Setup
**Status**: [X] COMPLETED - PromotionMatrix structure and helper methods reviewed

---

## Phase 2: Foundational Enhancements (Blocking Prerequisites)

**Goal**: Extend core data structures to support all conversion types before implementing user stories

### T004: Add Runtime Support Flags to PromotionRule [X]
**File**: `src/ir/type_promotion.rs`  
**Description**: Enhance `PromotionRule::Direct` variant to include `requires_runtime_support` and `requires_validation` boolean fields  
**Implementation**:
```rust
Direct {
    cast_kind: CastKind,
    may_lose_precision: bool,
    may_overflow: bool,
    requires_runtime_support: bool,    // NEW
    requires_validation: bool,         // NEW
}
```
**Validation**: Ensure existing code compiles by adding default `false` values to all existing Direct rules  
**Dependencies**: T003  
**Story**: Foundational
**Status**: [X] COMPLETED

---

### T005: Add New PromotionWarning Variants [X]
**File**: `src/ir/type_promotion.rs`  
**Description**: Add `InvalidStringConversion` and `InvalidUnicodeCodePoint` variants to `PromotionWarning` enum, and update existing `FloatSpecialValues` variant to use type conversion context fields (replacing binary operation fields)  
**Implementation**:
```rust
/// Invalid string conversion (unparseable)
InvalidStringConversion {
    string_value: Option<String>,
    target_type: IrType,
    reason: String,
},

/// Invalid Unicode code point for char
InvalidUnicodeCodePoint {
    value: u32,
    reason: String,
},

/// UPDATE EXISTING VARIANT:
/// FloatSpecialValues - change from binary operation context to type conversion context
FloatSpecialValues {
    value_type: FloatSpecialValueType,  // NaN | PosInf | NegInf
    source_type: IrType,                // F32 or F64
    target_type: IrType,                // I8-I64, U8-U64
    applied_behavior: OverflowBehavior, // Wrap | Saturate | Trap | CompileError
    source_span: SourceSpan,
},

/// Add helper enum for float special value types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSpecialValueType {
    NaN,
    PositiveInfinity,
    NegativeInfinity,
}
```
**Dependencies**: T003  
**Story**: Foundational

---

### T006: Update add_promotion_rule [X] to Handle New Fields
**File**: `src/ir/type_promotion.rs`  
**Description**: Modify `add_promotion_rule()` method to handle new `requires_runtime_support` and `requires_validation` fields when creating symmetric/inverse rules  
**Validation**: Ensure all existing promotion rule additions still compile  
**Dependencies**: T004  
**Story**: Foundational

---

### T007: Create Integer Narrowing [X] Helper Method Signature
**File**: `src/ir/type_promotion.rs`  
**Description**: Add private method `add_integer_narrowing_promotions(&mut self)` to PromotionMatrix implementation block  
**Implementation**:
```rust
/// Add all integer narrowing conversion rules (24 rules)
/// Narrowing: Larger → Smaller within same signedness
fn add_integer_narrowing_promotions(&mut self) {
    // Implementation in T015
}
```
**Dependencies**: T004  
**Story**: Foundational

---

### T008: Create Helper Method [X] Signatures for Bool/Char/String
**File**: `src/ir/type_promotion.rs` [P]  
**Description**: Add private method signatures for boolean, character, and string conversion helpers  
**Implementation**:
```rust
/// Add all boolean conversion rules (24 rules)
fn add_boolean_promotions(&mut self) {
    // Implementation in T023
}

/// Add all character conversion rules (14 rules)
fn add_character_promotions(&mut self) {
    // Implementation in T031
}

/// Add all string conversion rules (25 rules)
fn add_string_promotions(&mut self) {
    // Implementation in T035
}
```
**Dependencies**: T004  
**Story**: Foundational

---

## Phase 3: User Story 1 - Basic Numeric Type Conversions (P1)

**Goal**: Implement comprehensive integer and float conversions with warnings  
**Independent Test**: Programs with integer widening/narrowing and integer-float conversions compile correctly with appropriate warnings  
**Checkpoint**: After T021, all numeric conversions should be functional

### T009: [TEST] Write Integer Widening [X] Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for all integer widening conversions (12 signed + 12 unsigned = 24 tests)  
**Test Cases**:
```rust
#[test]
fn test_integer_widening_u8_to_u16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U8, &IrType::U16).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntZeroExtend);
    assert_eq!(rule.may_lose_precision, false);
    assert_eq!(rule.may_overflow, false);
}
// ... 23 more similar tests for all widening pairs
```
**Expected Results**: All tests should pass after T013 implementation  
**Dependencies**: T003  
**Story**: US1

---

### T010: [TEST] Write Integer Narrowing [X] Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for all integer narrowing conversions (12 signed + 12 unsigned = 24 tests)  
**Test Cases**:
```rust
#[test]
fn test_integer_narrowing_u64_to_u16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U16).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntTruncate);
    assert_eq!(rule.may_lose_precision, true);
    assert_eq!(rule.may_overflow, true);
}
// ... 23 more tests
```
**Expected Results**: All tests should pass after T015 implementation  
**Dependencies**: T003  
**Story**: US1

---

### T011: [TEST] Write Integer-Float [X] Conversion Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for integer↔float conversions (16 int→float + 16 float→int = 32 tests)  
**Test Cases**:
```rust
#[test]
fn test_i32_to_f32_conversion() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::F32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntToFloat);
}

#[test]
fn test_f64_to_i32_conversion_with_precision_loss() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::I32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::FloatToInt);
    assert_eq!(rule.may_lose_precision, true);
    assert_eq!(rule.may_overflow, true);
}
// ... 30 more tests
```
**Expected Results**: All tests should pass after T017 implementation  
**Dependencies**: T003  
**Story**: US1

---

### T012: [TEST] Write Float-Float [X] Conversion Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for f32↔f64 conversions (2 tests)  
**Test Cases**:
```rust
#[test]
fn test_f32_to_f64_extension() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F32, &IrType::F64).unwrap();
    assert_eq!(rule.cast_kind, CastKind::FloatExtend);
    assert_eq!(rule.may_lose_precision, false);
}

#[test]
fn test_f64_to_f32_truncation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::F32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::FloatTruncate);
    assert_eq!(rule.may_lose_precision, true);
}
```
**Expected Results**: Tests should pass (already implemented)  
**Dependencies**: T003  
**Story**: US1

---

### T013: Verify Existing Integer Widening [X] Rules
**File**: `src/ir/type_promotion.rs`  
**Description**: Review `add_integer_widening_promotions()` implementation to ensure all 24 widening rules are correctly defined  
**Validation**: Run T009 tests to confirm they pass  
**Dependencies**: T007, T009  
**Story**: US1

---

### T014: Verify Existing Float-Integer [X] Rules
**File**: `src/ir/type_promotion.rs`  
**Description**: Review `add_float_integer_promotions()` to ensure all 32 integer↔float rules are correctly defined  
**Validation**: Run T011 tests to confirm they pass  
**Dependencies**: T007, T011  
**Story**: US1

---

### T015: Implement Integer Narrowing [X] Promotion Rules
**File**: `src/ir/type_promotion.rs`  
**Description**: Implement `add_integer_narrowing_promotions()` to define all 24 narrowing conversion rules  
**Implementation**:
```rust
fn add_integer_narrowing_promotions(&mut self) {
    // Signed narrowing (6 rules)
    let signed_types = [(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)];
    for i in 0..signed_types.len() {
        for j in 0..i {
            let (from_type, _) = &signed_types[i];
            let (to_type, _) = &signed_types[j];
            self.add_promotion_rule(
                from_type.clone(),
                to_type.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::IntTruncate,
                    may_lose_precision: true,
                    may_overflow: true,
                    requires_runtime_support: false,
                    requires_validation: false,
                },
            );
        }
    }
    
    // Unsigned narrowing (6 rules) - similar pattern
    let unsigned_types = [(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)];
    // ... similar loop
}
```
**Validation**: Run T010 tests - all should pass  
**Dependencies**: T007, T010  
**Story**: US1

---

### T016: Add Narrowing Initialization [X] to PromotionMatrix
**File**: `src/ir/type_promotion.rs`  
**Description**: Update `initialize_default_promotions()` to call `add_integer_narrowing_promotions()`  
**Implementation**: Add after existing widening promotions call:
```rust
fn initialize_default_promotions(&mut self) {
    // ... existing calls ...
    self.add_integer_narrowing_promotions();  // NEW
}
```
**Validation**: Run full test suite - all US1 tests should pass  
**Dependencies**: T015  
**Story**: US1

---

### T017: [TEST] Write Snapshot Tests [X] for Numeric Warnings [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write insta snapshot tests for warning messages generated during numeric conversions  
**Test Cases**:
```rust
#[test]
fn test_narrowing_overflow_warning_snapshot() {
    let matrix = PromotionMatrix::new();
    let warning = generate_warning_for_narrowing(&IrType::U64, &IrType::U16);
    insta::assert_debug_snapshot!(warning);
}

#[test]
fn test_float_to_int_precision_loss_snapshot() {
    let matrix = PromotionMatrix::new();
    let warning = generate_warning_for_conversion(&IrType::F64, &IrType::I32);
    insta::assert_debug_snapshot!(warning);
}
```
**Expected Results**: Snapshots capture consistent warning formats  
**Dependencies**: T016  
**Story**: US1

---

### T018: [TEST] Write Edge Case Tests for Numeric Conversions [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write tests for edge cases: large integers, signed↔unsigned same-width, NaN/infinity with deterministic behavior verification  
**Test Cases**:
```rust
#[test]
fn test_cross_signedness_same_width_i32_to_u32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::U32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntBitcast);
}

#[test]
fn test_float_nan_to_int_warning() {
    // Test that NaN conversion generates FloatSpecialValues warning
    // Verify Wrap mode: NaN → 0 (deterministic)
}

#[test]
fn test_float_infinity_to_signed_int() {
    // Verify Wrap mode: +∞ → INT_MAX, -∞ → INT_MIN (deterministic)
}

#[test]
fn test_float_infinity_to_unsigned_int() {
    // Verify Wrap mode: +∞ → UINT_MAX, -∞ → 0 (deterministic)
}

#[test]
fn test_large_int_to_f32_precision_loss() {
    // Test that i64 > 24 bits generates precision loss warning
}
```
**Expected Results**: All edge cases handled correctly with deterministic behavior  
**Dependencies**: T016  
**Story**: US1

---

### T019: Implement Precision Loss Warning Generation
**File**: `src/ir/type_promotion.rs`  
**Description**: Enhance warning generation logic to create `PrecisionLoss` warnings for narrowing, float→int, and f64→f32 conversions  
**Implementation**: Add to promotion analysis logic:
```rust
if rule.may_lose_precision {
    let estimate = match (from_type, to_type) {
        (IrType::F64, IrType::I32) => PrecisionLossEstimate::FractionalPart,
        (IrType::F64, IrType::F32) => PrecisionLossEstimate::SignificantDigits { lost_bits: 29 },
        (IrType::U64, IrType::U16) => PrecisionLossEstimate::ValueRange { from_bits: 64, to_bits: 16 },
        _ => PrecisionLossEstimate::None,
    };
    warnings.push(PromotionWarning::PrecisionLoss { from_type, to_type, estimated_loss: estimate });
}
```
**Validation**: Run T017 snapshot tests  
**Dependencies**: T016  
**Story**: US1

---

### T020: Implement Signedness Change Warning
**File**: `src/ir/type_promotion.rs`  
**Description**: Add `SignednessChange` warning generation for same-width signed↔unsigned conversions  
**Implementation**:
```rust
if rule.cast_kind == CastKind::IntBitcast {
    let from_signed = from_type.is_signed_integer();
    let to_signed = to_type.is_signed_integer();
    if from_signed != to_signed {
        warnings.push(PromotionWarning::SignednessChange {
            from_signed,
            to_signed,
            may_affect_comparisons: true,
        });
    }
}
```
**Validation**: Run T018 edge case tests  
**Dependencies**: T019  
**Story**: US1

---

### T021: [TEST] Validate All 169 Type Pairs Coverage (Numeric Only)
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write comprehensive test to verify all numeric type pairs (8 integers × 8 integers + integer↔float) are defined  
**Test Case**:
```rust
#[test]
fn test_all_numeric_type_pairs_defined() {
    let matrix = PromotionMatrix::new();
    let int_types = vec![
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
    ];
    let float_types = vec![IrType::F32, IrType::F64];
    
    // Test all int×int pairs
    for from in &int_types {
        for to in &int_types {
            assert!(matrix.get_promotion_rule(from, to).is_some(),
                "Missing rule for {:?} → {:?}", from, to);
        }
    }
    
    // Test all int↔float pairs
    for int_ty in &int_types {
        for float_ty in &float_types {
            assert!(matrix.get_promotion_rule(int_ty, float_ty).is_some());
            assert!(matrix.get_promotion_rule(float_ty, int_ty).is_some());
        }
    }
}
```
**Expected Results**: All numeric conversions defined  
**Dependencies**: T016  
**Story**: US1

---

### T022: Update Module Documentation for Numeric Conversions
**File**: `src/ir/type_promotion.rs`  
**Description**: Update module-level rustdoc comments to document integer narrowing, widening, and float conversion support  
**Implementation**: Add to module docs:
```rust
//! ## Integer Narrowing Conversions (24 rules)
//! 
//! Narrowing conversions (larger → smaller type) use `IntTruncate` and may result in:
//! - Precision loss (high-order bits discarded)
//! - Overflow (value exceeds target range)
//! 
//! Example: `u64 → u16` truncates upper 48 bits, generating `PrecisionLoss` warning
//!
//! ## Integer-Float Conversions (32 rules)
//! ...
```
**Dependencies**: T021  
**Story**: US1

---

**CHECKPOINT US1**: All numeric conversions implemented and tested. MVP ready for delivery.

---

## Phase 4: User Story 2 - Boolean and Character Conversions (P2)

**Goal**: Implement boolean↔numeric and char↔integer conversions with Unicode validation  
**Independent Test**: Programs with boolean and character conversions compile correctly with Unicode validation  
**Checkpoint**: After T032, boolean and character conversions are fully functional

### T023: [TEST] Write Boolean Conversion Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for all boolean conversions (10 bool→numeric + 10 numeric→bool + 2 bool↔String + 2 bool↔char = 24 tests)  
**Test Cases**:
```rust
#[test]
fn test_bool_to_i32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::BoolToInt);
    assert_eq!(rule.may_lose_precision, false);
}

#[test]
fn test_i32_to_bool_zero_test() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::Bool).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntToBool);
}

#[test]
fn test_f64_to_bool_nan_handling() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::Bool).unwrap();
    assert_eq!(rule.cast_kind, CastKind::FloatToBool);
    // NaN → true (non-zero)
}
```
**Expected Results**: All tests should pass after T025 implementation  
**Dependencies**: T008  
**Story**: US2

---

### T024: [TEST] Write Character Conversion Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for character conversions (2 char↔u32 + 12 char↔other integers + 2 char↔String = 16 tests)  
**Test Cases**:
```rust
#[test]
fn test_char_to_u32_unicode_scalar() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::U32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::CharToInt);
    assert_eq!(rule.requires_validation, false);
}

#[test]
fn test_u32_to_char_with_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntToChar);
    assert_eq!(rule.requires_validation, true);  // Unicode validation
}

#[test]
fn test_char_to_string_runtime_support() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::String).unwrap();
    assert_eq!(rule.cast_kind, CastKind::CharToString);
    assert_eq!(rule.requires_runtime_support, true);
}
```
**Expected Results**: All tests should pass after T027 implementation  
**Dependencies**: T008  
**Story**: US2

---

### T025: Implement Boolean Promotion Rules
**File**: `src/ir/type_promotion.rs`  
**Description**: Implement `add_boolean_promotions()` to define all 24 boolean conversion rules  
**Implementation**:
```rust
fn add_boolean_promotions(&mut self) {
    // Bool → Integers (8 rules)
    let int_types = [IrType::I8, IrType::I16, IrType::I32, IrType::I64,
                     IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    for int_ty in &int_types {
        self.add_promotion_rule(
            IrType::Bool,
            int_ty.clone(),
            PromotionRule::Direct {
                cast_kind: CastKind::BoolToInt,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
    }
    
    // Integers → Bool (8 rules)
    for int_ty in &int_types {
        self.add_promotion_rule(
            int_ty.clone(),
            IrType::Bool,
            PromotionRule::Direct {
                cast_kind: CastKind::IntToBool,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
    }
    
    // Bool ↔ Floats (4 rules)
    self.add_promotion_rule(IrType::Bool, IrType::F32, PromotionRule::Direct {
        cast_kind: CastKind::BoolToFloat, ...
    });
    // ... similar for F64, and reverse conversions with FloatToBool
    
    // Bool ↔ String (2 rules)
    self.add_promotion_rule(IrType::Bool, IrType::String, PromotionRule::Direct {
        cast_kind: CastKind::BoolToString,
        requires_runtime_support: true,
        ...
    });
    // ... StringToBool with validation
    
    // Bool ↔ char (2 rules via Indirect)
    self.add_promotion_rule(IrType::Bool, IrType::Char, PromotionRule::Indirect {
        intermediate_type: IrType::U32,
        first_cast: CastKind::BoolToInt,
        second_cast: CastKind::IntToChar,
        requires_runtime_support: false,
    });
}
```
**Validation**: Run T023 tests - all should pass  
**Dependencies**: T008, T023  
**Story**: US2

---

### T026: Add Boolean Initialization to PromotionMatrix
**File**: `src/ir/type_promotion.rs`  
**Description**: Update `initialize_default_promotions()` to call `add_boolean_promotions()`  
**Implementation**:
```rust
fn initialize_default_promotions(&mut self) {
    // ... existing calls ...
    self.add_boolean_promotions();  // NEW
}
```
**Validation**: Run T023 tests  
**Dependencies**: T025  
**Story**: US2

---

### T027: Implement Character Promotion Rules
**File**: `src/ir/type_promotion.rs`  
**Description**: Implement `add_character_promotions()` to define all 16 character conversion rules  
**Implementation**:
```rust
fn add_character_promotions(&mut self) {
    // char → u32 (1 rule)
    self.add_promotion_rule(
        IrType::Char,
        IrType::U32,
        PromotionRule::Direct {
            cast_kind: CastKind::CharToInt,
            may_lose_precision: false,
            may_overflow: false,
            requires_runtime_support: false,
            requires_validation: false,
        },
    );
    
    // u32 → char (1 rule with validation)
    self.add_promotion_rule(
        IrType::U32,
        IrType::Char,
        PromotionRule::Direct {
            cast_kind: CastKind::IntToChar,
            may_lose_precision: false,
            may_overflow: false,
            requires_runtime_support: false,
            requires_validation: true,  // Unicode scalar validation
        },
    );
    
    // char → other integers (6 rules via Indirect through u32)
    let other_int_types = [IrType::I8, IrType::I16, IrType::I32, IrType::I64,
                           IrType::U8, IrType::U16, IrType::U64];
    for int_ty in &other_int_types {
        self.add_promotion_rule(
            IrType::Char,
            int_ty.clone(),
            PromotionRule::Indirect {
                intermediate_type: IrType::U32,
                first_cast: CastKind::CharToInt,
                second_cast: determine_cast_from_u32(int_ty),  // Helper function
                requires_runtime_support: false,
            },
        );
    }
    
    // Other integers → char (6 rules via Indirect through u32)
    // ... similar pattern with validation
    
    // char ↔ String (2 rules)
    self.add_promotion_rule(IrType::Char, IrType::String, PromotionRule::Direct {
        cast_kind: CastKind::CharToString,
        requires_runtime_support: true,
        ...
    });
    self.add_promotion_rule(IrType::String, IrType::Char, PromotionRule::Direct {
        cast_kind: CastKind::StringToChar,
        requires_runtime_support: true,
        requires_validation: true,  // Length check
        ...
    });
}
```
**Validation**: Run T024 tests - all should pass  
**Dependencies**: T008, T024  
**Story**: US2

---

### T028: Add Character Initialization to PromotionMatrix
**File**: `src/ir/type_promotion.rs`  
**Description**: Update `initialize_default_promotions()` to call `add_character_promotions()`  
**Implementation**:
```rust
fn initialize_default_promotions(&mut self) {
    // ... existing calls ...
    self.add_character_promotions();  // NEW
}
```
**Validation**: Run T024 tests  
**Dependencies**: T027  
**Story**: US2

---

### T029: [TEST] Write Unicode Validation Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write tests for invalid Unicode scenarios (surrogates, out-of-range)  
**Test Cases**:
```rust
#[test]
fn test_surrogate_u32_to_char_requires_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();
    assert_eq!(rule.requires_validation, true);
    
    // Expected: Validation rejects 0xD800 - 0xDFFF (surrogates)
}

#[test]
fn test_out_of_range_u32_to_char() {
    // Expected: Validation rejects values > 0x10FFFF
}

#[test]
fn test_valid_unicode_ranges() {
    // Test 0x0000-0xD7FF and 0xE000-0x10FFFF are accepted
}
```
**Expected Results**: Tests verify validation logic  
**Dependencies**: T028  
**Story**: US2

---

### T030: Implement Unicode Validation Warning Generation
**File**: `src/ir/type_promotion.rs`  
**Description**: Add `InvalidUnicodeCodePoint` warning generation for u32→char conversions  
**Implementation**:
```rust
if rule.requires_validation && to_type == IrType::Char {
    if let Some(value) = get_static_u32_value(from_value) {  // If const-evaluable
        if !is_valid_unicode_scalar(value) {
            let reason = if (0xD800..=0xDFFF).contains(&value) {
                "surrogate code point (reserved for UTF-16)"
            } else if value > 0x10FFFF {
                "value exceeds maximum Unicode code point U+10FFFF"
            } else {
                "invalid Unicode scalar value"
            };
            warnings.push(PromotionWarning::InvalidUnicodeCodePoint { value, reason: reason.to_string() });
        }
    }
}

fn is_valid_unicode_scalar(value: u32) -> bool {
    value <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&value)
}
```
**Validation**: Run T029 validation tests  
**Dependencies**: T028, T029  
**Story**: US2

---

### T031: [TEST] Write Snapshot Tests for Boolean/Character Warnings [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write insta snapshot tests for warning messages from boolean and character conversions  
**Test Cases**:
```rust
#[test]
fn test_invalid_unicode_warning_snapshot() {
    let warning = PromotionWarning::InvalidUnicodeCodePoint {
        value: 0xD800,
        reason: "surrogate code point".to_string(),
    };
    insta::assert_debug_snapshot!(warning);
}
```
**Expected Results**: Snapshots capture consistent formats  
**Dependencies**: T030  
**Story**: US2

---

### T032: Update Documentation for Boolean/Character Conversions
**File**: `src/ir/type_promotion.rs`  
**Description**: Add rustdoc documentation for boolean and character conversion rules  
**Implementation**:
```rust
//! ## Boolean Conversions (24 rules)
//! 
//! Boolean to numeric: `true` → 1, `false` → 0
//! Numeric to boolean: 0 → `false`, non-zero → `true` (including NaN)
//! 
//! ## Character Conversions (16 rules)
//! 
//! - `char` → `u32`: Direct Unicode scalar value extraction
//! - `u32` → `char`: Validated conversion (rejects surrogates U+D800-U+DFFF and values >U+10FFFF)
//! - `char` ↔ other integers: Indirect via `u32`
//! - `char` ↔ `String`: Runtime support required
```
**Dependencies**: T031  
**Story**: US2

---

**CHECKPOINT US2**: Boolean and character conversions implemented and tested. Independent user story complete.

---

## Phase 5: User Story 3 - String Conversions (P3)

**Goal**: Implement string↔primitive conversions with parsing validation  
**Independent Test**: Programs with string formatting and parsing compile with appropriate runtime support flags  
**Checkpoint**: After T037, all 169 type conversions are fully implemented

### T033: [TEST] Write String Conversion Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write unit tests for all string conversions (12 primitive→String + 12 String→primitive + 1 String→String = 25 tests)  
**Test Cases**:
```rust
#[test]
fn test_i32_to_string_formatting() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::String).unwrap();
    assert_eq!(rule.cast_kind, CastKind::IntToString);
    assert_eq!(rule.requires_runtime_support, true);
    assert_eq!(rule.requires_validation, false);  // Always succeeds
}

#[test]
fn test_string_to_i32_parsing() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I32).unwrap();
    assert_eq!(rule.cast_kind, CastKind::StringToInt);
    assert_eq!(rule.requires_runtime_support, true);
    assert_eq!(rule.requires_validation, true);  // Parse may fail
}

#[test]
fn test_bool_to_string() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::String).unwrap();
    assert_eq!(rule.cast_kind, CastKind::BoolToString);
}
```
**Expected Results**: All tests should pass after T035 implementation  
**Dependencies**: T008  
**Story**: US3

---

### T034: [TEST] Write String Parsing Error Test Cases [P]
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write tests for invalid string parsing scenarios  
**Test Cases**:
```rust
#[test]
fn test_invalid_string_to_int_requires_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I32).unwrap();
    assert_eq!(rule.requires_validation, true);
    // Expected: "abc" → i32 generates InvalidStringConversion warning
}

#[test]
fn test_string_to_char_length_check() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Char).unwrap();
    assert_eq!(rule.requires_validation, true);
    // Expected: Multi-char or empty string generates error
}
```
**Expected Results**: Tests verify validation requirements  
**Dependencies**: T008  
**Story**: US3

---

### T035: Implement String Promotion Rules
**File**: `src/ir/type_promotion.rs`  
**Description**: Implement `add_string_promotions()` to define all 25 string conversion rules  
**Implementation**:
```rust
fn add_string_promotions(&mut self) {
    // Integers → String (8 rules)
    let int_types = [IrType::I8, IrType::I16, IrType::I32, IrType::I64,
                     IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    for int_ty in &int_types {
        self.add_promotion_rule(
            int_ty.clone(),
            IrType::String,
            PromotionRule::Direct {
                cast_kind: CastKind::IntToString,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: true,  // Formatting
                requires_validation: false,      // Always succeeds
            },
        );
    }
    
    // String → Integers (8 rules)
    for int_ty in &int_types {
        self.add_promotion_rule(
            IrType::String,
            int_ty.clone(),
            PromotionRule::Direct {
                cast_kind: CastKind::StringToInt,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: true,  // Parsing
                requires_validation: true,       // Parse may fail
            },
        );
    }
    
    // Floats → String (2 rules)
    self.add_promotion_rule(IrType::F32, IrType::String, PromotionRule::Direct {
        cast_kind: CastKind::FloatToString,
        requires_runtime_support: true,
        ...
    });
    // ... similar for F64
    
    // String → Floats (2 rules)
    self.add_promotion_rule(IrType::String, IrType::F32, PromotionRule::Direct {
        cast_kind: CastKind::StringToFloat,
        requires_runtime_support: true,
        requires_validation: true,
        ...
    });
    // ... similar for F64
    
    // String → String (1 rule - identity)
    self.add_promotion_rule(IrType::String, IrType::String, PromotionRule::Direct {
        cast_kind: CastKind::Bitcast,  // No-op
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
    });
    
    // Note: Bool↔String and char↔String already added in T025 and T027
}
```
**Validation**: Run T033 tests - all should pass  
**Dependencies**: T008, T033  
**Story**: US3

---

### T036: Add String Initialization to PromotionMatrix
**File**: `src/ir/type_promotion.rs`  
**Description**: Update `initialize_default_promotions()` to call `add_string_promotions()`  
**Implementation**:
```rust
fn initialize_default_promotions(&mut self) {
    // ... existing calls ...
    self.add_string_promotions();  // NEW
}
```
**Validation**: Run T033 and T034 tests  
**Dependencies**: T035  
**Story**: US3

---

### T037: Implement String Parsing Warning Generation
**File**: `src/ir/type_promotion.rs`  
**Description**: Add `InvalidStringConversion` warning generation for String→primitive conversions  
**Implementation**:
```rust
if rule.requires_validation && from_type == IrType::String {
    if let Some(string_value) = get_static_string_value(from_value) {  // If const-evaluable
        let parse_result = try_parse_to_target_type(string_value, to_type);
        if parse_result.is_err() {
            warnings.push(PromotionWarning::InvalidStringConversion {
                string_value: Some(string_value.clone()),
                target_type: to_type.clone(),
                reason: format!("Cannot parse '{}' as {:?}", string_value, to_type),
            });
        }
    } else {
        // Dynamic string - mark as requiring runtime validation
        warnings.push(PromotionWarning::InvalidStringConversion {
            string_value: None,
            target_type: to_type.clone(),
            reason: "String parsing may fail at runtime".to_string(),
        });
    }
}
```
**Validation**: Run T034 validation tests  
**Dependencies**: T036, T034  
**Story**: US3

---

### T038: Update Documentation for String Conversions
**File**: `src/ir/type_promotion.rs`  
**Description**: Add rustdoc documentation for string conversion rules  
**Implementation**:
```rust
//! ## String Conversions (25 rules)
//! 
//! ### Primitive → String (Always Succeeds)
//! - Formatting operations: `IntToString`, `FloatToString`, `BoolToString`
//! - Runtime support required for heap allocation
//! 
//! ### String → Primitive (May Fail)
//! - Parsing operations: `StringToInt`, `StringToFloat`, `StringToBool`
//! - Runtime support + validation required
//! - Generates `InvalidStringConversion` warning for unparseable strings
```
**Dependencies**: T037  
**Story**: US3

---

**CHECKPOINT US3**: All string conversions implemented. All 169 type conversion pairs now defined!

---

## Phase 6: Polish & Integration

**Goal**: Validate completeness, optimize performance, and finalize documentation

### T039: [TEST] Validate All 169 Type Pairs Complete
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write comprehensive test to verify all 169 fundamental type pairs (13×13) are defined  
**Test Case**:
```rust
#[test]
fn test_all_169_type_pairs_defined() {
    let matrix = PromotionMatrix::new();
    let all_types = vec![
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
        IrType::F32, IrType::F64,
        IrType::Bool, IrType::Char, IrType::String,
    ];
    
    let mut defined_count = 0;
    for from in &all_types {
        for to in &all_types {
            assert!(
                matrix.get_promotion_rule(from, to).is_some(),
                "Missing rule for {:?} → {:?}",
                from, to
            );
            defined_count += 1;
        }
    }
    
    assert_eq!(defined_count, 169, "Expected 169 promotion rules (13×13)");
}
```
**Expected Results**: Test passes - all 169 pairs defined  
**Dependencies**: T038  
**Story**: Polish

---

### T040: [TEST] Validate All 24 CastKind Variants Used
**File**: `tests/ir_type_promotion_tests.rs`  
**Description**: Write test to verify all 24 CastKind variants are utilized in the promotion matrix  
**Test Case**:
```rust
#[test]
fn test_all_cast_kinds_utilized() {
    let matrix = PromotionMatrix::new();
    let expected_cast_kinds = HashSet::from([
        CastKind::IntZeroExtend, CastKind::IntSignExtend, CastKind::IntTruncate,
        CastKind::IntBitcast, CastKind::IntToFloat, CastKind::FloatToInt,
        CastKind::FloatTruncate, CastKind::FloatExtend,
        CastKind::BoolToInt, CastKind::IntToBool, CastKind::BoolToFloat, CastKind::FloatToBool,
        CastKind::CharToInt, CastKind::IntToChar, CastKind::CharToString, CastKind::StringToChar,
        CastKind::StringToInt, CastKind::StringToFloat, CastKind::StringToBool,
        CastKind::IntToString, CastKind::FloatToString, CastKind::BoolToString,
        CastKind::Bitcast,
    ]);
    
    let mut found_cast_kinds = HashSet::new();
    // Iterate all promotion rules and collect CastKind variants
    // ...
    
    assert_eq!(found_cast_kinds, expected_cast_kinds, "All 24 CastKind variants must be used");
}
```
**Expected Results**: All 24 variants utilized  
**Dependencies**: T039  
**Story**: Polish

---

### T041: Add Performance Benchmark for Promotion Lookup
**File**: `benches/jsavrs_benchmark.rs`  
**Description**: Add criterion benchmark to measure promotion rule lookup performance  
**Implementation**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsavrs::ir::type_promotion::PromotionMatrix;
use jsavrs::ir::types::IrType;

fn benchmark_promotion_lookup(c: &mut Criterion) {
    let matrix = PromotionMatrix::new();
    
    c.bench_function("promotion_lookup_common_case", |b| {
        b.iter(|| {
            black_box(matrix.get_promotion_rule(
                &IrType::I32,
                &IrType::I64,
            ))
        })
    });
    
    c.bench_function("promotion_lookup_10000_iterations", |b| {
        b.iter(|| {
            for _ in 0..10000 {
                black_box(matrix.get_promotion_rule(
                    &IrType::I32,
                    &IrType::F64,
                ));
            }
        })
    });
}

criterion_group!(benches, benchmark_promotion_lookup);
criterion_main!(benches);
```
**Target Performance**: 10,000 lookups < 1ms (as per spec SC-007 and SC-010)  
**Dependencies**: T040  
**Story**: Polish

---

### T042: Final Documentation Review and Module Summary
**File**: `src/ir/type_promotion.rs`  
**Description**: Update module-level documentation with comprehensive summary of all 169 promotion rules  
**Implementation**:
```rust
//! # Type Promotion System for IR
//!
//! This module implements a comprehensive type promotion system supporting all 169 possible
//! conversions among 13 fundamental data types: i8, i16, i32, i64, u8, u16, u32, u64,
//! f32, f64, bool, char, and String.
//!
//! ## Complete Promotion Matrix (169 rules)
//!
//! | Category | Rule Count | Examples |
//! |----------|------------|----------|
//! | Identity | 13 | i32→i32, bool→bool |
//! | Integer Widening | 24 | u8→u32 (zero-extend), i8→i32 (sign-extend) |
//! | Integer Narrowing | 24 | u64→u16 (truncate with overflow warning) |
//! | Cross-Signedness | 8 | i32↔u32 (bitcast with signedness warning) |
//! | Integer↔Float | 32 | i32→f32, f64→i32 (precision loss warnings) |
//! | Float↔Float | 2 | f32→f64 (extend), f64→f32 (truncate) |
//! | Boolean | 24 | bool→int (0/1), int→bool (zero test) |
//! | Character | 16 | char→u32, u32→char (Unicode validation) |
//! | String | 25 | int→String (format), String→int (parse) |
//!
//! ## Performance Characteristics
//!
//! - O(1) HashMap lookup for all promotion rules
//! - <1ms for complete type promotion analysis
//! - <5% compilation time overhead for extensive type conversions
//!
//! ## Usage Example
//! ...
```
**Dependencies**: T041  
**Story**: Polish

---

## Dependency Graph

```
Phase 1 (Setup)
├── T001: Verify CastKind
├── T002: Verify IrType
└── T003: Review PromotionMatrix

Phase 2 (Foundational) [Blocks all user stories]
├── T004: Add Runtime Flags ← T003
├── T005: Add Warning Variants ← T003
├── T006: Update add_promotion_rule ← T004
├── T007: Create Integer Narrowing Helper ← T004
└── T008: Create Bool/Char/String Helpers [P] ← T004

Phase 3 (US1 - Basic Numeric) [P1]
├── Tests [P]
│   ├── T009: Integer Widening Tests ← T003
│   ├── T010: Integer Narrowing Tests ← T003
│   ├── T011: Integer-Float Tests ← T003
│   └── T012: Float-Float Tests ← T003
├── T013: Verify Widening ← T007, T009
├── T014: Verify Float-Int ← T007, T011
├── T015: Implement Narrowing ← T007, T010
├── T016: Add to Init ← T015
├── T017: Snapshot Tests [P] ← T016
├── T018: Edge Case Tests [P] ← T016
├── T019: Precision Loss Warnings ← T016
├── T020: Signedness Warnings ← T019
├── T021: Validate Coverage ← T016
└── T022: Update Docs ← T021

Phase 4 (US2 - Boolean & Character) [P2] [Can start after Phase 2]
├── Tests [P]
│   ├── T023: Boolean Tests ← T008
│   └── T024: Character Tests ← T008
├── T025: Implement Boolean Rules ← T008, T023
├── T026: Add Bool Init ← T025
├── T027: Implement Character Rules ← T008, T024
├── T028: Add Char Init ← T027
├── T029: Unicode Validation Tests [P] ← T028
├── T030: Unicode Warnings ← T028, T029
├── T031: Snapshot Tests [P] ← T030
└── T032: Update Docs ← T031

Phase 5 (US3 - String) [P3] [Can start after Phase 2]
├── Tests [P]
│   ├── T033: String Conversion Tests ← T008
│   └── T034: String Error Tests ← T008
├── T035: Implement String Rules ← T008, T033
├── T036: Add String Init ← T035
├── T037: String Parsing Warnings ← T036, T034
└── T038: Update Docs ← T037

Phase 6 (Polish) [After all user stories]
├── T039: Validate 169 Pairs ← T038
├── T040: Validate 24 CastKinds ← T039
├── T041: Performance Benchmark ← T040
└── T042: Final Documentation ← T041
```

---

## Parallel Execution Examples

### Within User Story 1 (Basic Numeric)
```bash
# Tests can run in parallel after T003
cargo test test_integer_widening &    # T009
cargo test test_integer_narrowing &   # T010
cargo test test_integer_float &       # T011
cargo test test_float_float &         # T012
wait

# After T016, these can run in parallel
cargo test --test snapshot_numeric &  # T017
cargo test --test edge_cases &        # T018
```

### Across User Stories (After Phase 2 completes)
```bash
# US1, US2, US3 tests can be written in parallel
(cd tests && write_numeric_tests.sh) &    # US1 T009-T012
(cd tests && write_boolean_tests.sh) &    # US2 T023-T024
(cd tests && write_string_tests.sh) &     # US3 T033-T034
wait

# However, implementation must follow US priority order:
# 1. Complete US1 (P1) first - foundation
# 2. Then US2 (P2) - builds on numeric foundation
# 3. Finally US3 (P3) - builds on all previous
```

---

## Independent Test Criteria

### User Story 1 (P1) - Basic Numeric Conversions
**✅ Complete when:**
- All integer widening tests pass (24 tests - T009)
- All integer narrowing tests pass (24 tests - T010)
- All integer-float conversion tests pass (32 tests - T011)
- Float-float conversion tests pass (2 tests - T012)
- Precision loss warnings generated correctly (T019)
- Signedness warnings generated correctly (T020)
- All numeric type pairs defined (T021)

**Deliverable**: Compiler correctly handles all numeric type conversions with appropriate warnings

---

### User Story 2 (P2) - Boolean and Character Conversions
**✅ Complete when:**
- All boolean conversion tests pass (24 tests - T023)
- All character conversion tests pass (16 tests - T024)
- Unicode validation tests pass (invalid surrogates, out-of-range - T029)
- Invalid Unicode warnings generated correctly (T030)
- Boolean and character type pairs defined

**Deliverable**: Compiler correctly handles boolean and character conversions with Unicode validation

---

### User Story 3 (P3) - String Conversions
**✅ Complete when:**
- All string conversion tests pass (25 tests - T033)
- String parsing error tests pass (T034)
- Invalid string conversion warnings generated correctly (T037)
- All string type pairs defined

**Deliverable**: Compiler correctly handles string formatting and parsing with validation

---

## MVP Scope (User Story 1 Only)

For **minimum viable product**, implement only **Phase 3 (User Story 1)**:
- Tasks T001-T008 (Setup + Foundational)
- Tasks T009-T022 (US1 implementation)

This delivers:
- ✅ All integer widening/narrowing conversions
- ✅ All integer-float conversions
- ✅ Float-float conversions
- ✅ Precision loss and overflow warnings
- ✅ 82 out of 169 type conversion pairs (~49%)
- ✅ Core functionality for typical numeric operations

Boolean, character, and string conversions (US2, US3) can be delivered incrementally after MVP.

---

## Summary

- **Total Tasks**: 42
- **Parallel Tasks**: 18 marked [P]
- **Test Tasks**: 15 (explicitly requested in spec)
- **Implementation Tasks**: 22
- **Documentation Tasks**: 5

**Estimated Timeline**:
- **MVP (US1 only)**: 2-3 days
- **Full Implementation (US1+US2+US3)**: 3-5 days
- **Polish & Testing**: 1 day

**Critical Path**: Phase 1 → Phase 2 → Phase 3 (US1) → Phase 4 (US2) → Phase 5 (US3) → Phase 6 (Polish)

**Success Criteria**: All 10 success criteria from spec.md met after Phase 6 completion

---

**Tasks Status**: ✅ READY FOR EXECUTION  
**Generated**: 2025-10-08  
**Next Step**: Begin with T001 (Verify CastKind Enum Completeness)
