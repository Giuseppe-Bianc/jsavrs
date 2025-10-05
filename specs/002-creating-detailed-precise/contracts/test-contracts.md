# Test Function Contracts

**Date**: 2025-10-05  
**Feature**: Comprehensive Test Suite for Type Promotion Module  
**Purpose**: Formal specifications for test function signatures and behaviors

---

## Overview

This directory contains API contracts for the test suite. Since this is a test implementation project (not a public API), the contracts are primarily **behavioral specifications** rather than strict function signatures.

---

## Contract: PromotionMatrix Tests

### Contract ID: `C-001`

**Function Pattern**: `test_promotion_matrix_<operation>_<scenario>`

**Signature Template**:
```rust
#[test]
fn test_promotion_matrix_{operation}_{scenario}() {
    // Arrange
    let matrix = PromotionMatrix::new();
    
    // Act
    let result = matrix.{operation}(/* args */);
    
    // Assert
    assert!(/* expected_condition */);
}
```

**Behavioral Contract**:
- **Preconditions**: None (tests create fresh `PromotionMatrix`)
- **Postconditions**: Matrix state unchanged (tests are read-only)
- **Invariants**: All assertions must pass
- **Performance**: Execution time <100ms per test

**Examples**:
```rust
#[test]
fn test_promotion_matrix_get_promotion_rule_i32_to_f64() { /* ... */ }

#[test]
fn test_promotion_matrix_compute_common_type_signed_unsigned() { /* ... */ }
```

---

## Contract: Panic Tests

### Contract ID: `C-002`

**Function Pattern**: `test_<entity>_<invalid_scenario>_panics`

**Signature Template**:
```rust
#[test]
#[should_panic(expected = "{specific_panic_message}")]
fn test_{entity}_{invalid_scenario}_panics() {
    // Arrange: Set up invalid state
    
    // Act: Trigger panic
    let _ = /* operation that panics */;
    
    // Assert: (Implicit - test passes if panic occurs with expected message)
}
```

**Behavioral Contract**:
- **Preconditions**: Invalid input or state configured
- **Postconditions**: Function panics with specific message
- **Invariants**: Panic message must match `expected` substring
- **Performance**: <100ms (fast failure)

**Examples**:
```rust
#[test]
#[should_panic(expected = "Invalid type combination")]
fn test_promotion_rule_bool_to_float_panics() { /* ... */ }
```

---

## Contract: Edge Case Tests

### Contract ID: `C-003`

**Function Pattern**: `test_<entity>_<boundary_condition>`

**Signature Template**:
```rust
#[test]
fn test_{entity}_{boundary_condition}() {
    // Arrange: Set up boundary value (MIN, MAX, zero, etc.)
    let boundary_value = /* MIN/MAX/special value */;
    
    // Act: Perform promotion
    let result = /* promotion operation */;
    
    // Assert: Validate boundary handling
    assert_eq!(result.{property}, expected_value);
    assert!(/* no overflow/precision loss warnings if expected */);
}
```

**Behavioral Contract**:
- **Preconditions**: Boundary value (e.g., `i32::MAX`, `f32::NAN`)
- **Postconditions**: Correct promotion or appropriate warning
- **Invariants**: No panics for valid boundary values
- **Performance**: <100ms per test

**Examples**:
```rust
#[test]
fn test_promotion_i32_max_to_i64_no_overflow() { /* ... */ }

#[test]
fn test_promotion_f32_nan_propagation() { /* ... */ }
```

---

## Contract: Snapshot Tests

### Contract ID: `C-004`

**Function Pattern**: `test_<entity>_<scenario>_snapshot`

**Signature Template**:
```rust
#[test]
fn test_{entity}_{scenario}_snapshot() {
    // Arrange
    let input = /* complex test case */;
    
    // Act
    let result = /* operation producing complex output */;
    
    // Assert: Snapshot comparison
    insta::assert_debug_snapshot!(result);
    
    // Additional explicit assertions for critical properties
    assert_eq!(result.{critical_field}, expected_value);
}
```

**Behavioral Contract**:
- **Preconditions**: Complex input scenario
- **Postconditions**: Output matches stored snapshot
- **Invariants**: Snapshot must be reviewed on first run (`cargo insta review`)
- **Performance**: <100ms per test (snapshot comparison is fast)

**Examples**:
```rust
#[test]
fn test_promotion_result_f64_to_i32_with_warnings_snapshot() { /* ... */ }
```

---

## Contract: Helper Functions

### Contract ID: `C-005`

**Function Pattern**: `<verb>_<noun>` (no `test_` prefix)

**Signature Template**:
```rust
fn {verb}_{noun}({params}) -> {return_type} {
    // Implementation
}
```

**Behavioral Contract**:
- **Preconditions**: Specified in function documentation
- **Postconditions**: Returns valid instance of `return_type`
- **Invariants**: Deterministic (same inputs → same outputs)
- **Performance**: <10ms (helpers are lightweight)
- **Usage**: Must be called by at least 2 test functions (avoid one-off helpers)

**Examples**:
```rust
/// Creates a PromotionMatrix with custom overflow behavior.
fn create_matrix_with_overflow(behavior: OverflowBehavior) -> PromotionMatrix {
    PromotionMatrix::with_overflow_behavior(behavior)
}

/// Returns all basic numeric types for parameterized testing.
fn all_numeric_types() -> Vec<IrType> {
    vec![
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
        IrType::F32, IrType::F64,
    ]
}
```

---

## Contract: Documentation Standard

### Contract ID: `C-006`

**Applies to**: All test functions

**Documentation Template**:
```rust
/// {One-sentence summary of what test validates}
///
/// # Rationale
/// {Explanation of why this test exists, what edge case it covers}
///
/// # Expected Behavior
/// - {Bullet point 1: Expected outcome}
/// - {Bullet point 2: No warnings/errors expected}
/// - {Bullet point 3: Specific assertion targets}
///
/// # Related
/// {Optional: Links to spec requirements (e.g., FR-003), related tests}
#[test]
fn test_{entity}_{operation}_{scenario}() {
    // Test body
}
```

**Behavioral Contract**:
- **Mandatory**: All test functions must have rustdoc comment
- **Content**: Explains **why** (not **how**) test exists
- **Rationale**: Must reference feature spec requirements or edge case taxonomy
- **Performance**: N/A (documentation has no runtime cost)

**Example**:
```rust
/// Tests that promotion from i32 to f64 is direct cast without precision loss.
///
/// # Rationale
/// F64 has 53-bit significand, which can exactly represent all i32 values.
/// This test validates FR-003 (precision preservation for safe conversions).
///
/// # Expected Behavior
/// - Promotion rule: Direct cast (IntToFloat)
/// - Warnings: None (no precision loss)
/// - Result type: F64
///
/// # Related
/// - FR-003: Type promotion rules
/// - Edge Case Category: Type System Boundaries
#[test]
fn test_promotion_matrix_i32_to_f64_preserves_precision() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::F64);
    
    assert!(matches!(rule, Some(PromotionRule::Direct { 
        cast_kind: CastKind::IntToFloat,
        may_lose_precision: false,
        ..
    })));
}
```

---

## Contract Validation Rules

| Contract ID | Validation Method | Enforcement |
|-------------|-------------------|-------------|
| C-001 | Naming convention regex `^test_promotion_matrix_[a-z_]+$` | Code review |
| C-002 | Panic message in `#[should_panic(expected = "...")]` | Compiler + runtime |
| C-003 | Boundary value in test body (e.g., `i32::MAX`) | Code review |
| C-004 | Presence of `insta::assert_debug_snapshot!` | Grep search |
| C-005 | Called by ≥2 test functions | Grep search (`rg "helper_name"`) |
| C-006 | Non-empty rustdoc comment (`///`) | Cargo doc warnings |

---

## Contract Violations and Exceptions

### Exception: `C-005` (Helper Usage)

**Scenario**: Helper function used by only 1 test during initial implementation.

**Resolution**: Inline helper into test body OR mark as `#[allow(dead_code)]` with TODO comment:
```rust
#[allow(dead_code)] // TODO: Reuse in test_other_scenario
fn create_matrix_with_overflow(behavior: OverflowBehavior) -> PromotionMatrix {
    // ...
}
```

### Exception: `C-006` (Documentation)

**Scenario**: Test name is fully self-documenting (e.g., `test_promotion_matrix_new_initializes_default_rules`).

**Resolution**: Minimal documentation allowed if test name + assertions are crystal clear:
```rust
/// Verifies PromotionMatrix::new() initializes default promotion rules.
#[test]
fn test_promotion_matrix_new_initializes_default_rules() {
    let matrix = PromotionMatrix::new();
    assert!(matrix.rules.len() > 0); // At least identity promotions
}
```

---

## Summary

This contracts specification establishes:
- **5 test function patterns** (C-001 to C-005) with signature templates
- **1 documentation standard** (C-006) with rustdoc requirements
- **Behavioral contracts** defining pre/post conditions, invariants, performance
- **Validation rules** for contract adherence
- **Exception handling** for edge cases

All tests must conform to these contracts to ensure consistency, maintainability, and alignment with constitutional principles (FR-010: Documentation Rigor).
