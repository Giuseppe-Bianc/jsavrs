//! Unit tests for type promotion warning generation functions.
//!
//! This test module verifies that warning generation functions correctly return
//! `Some` or `None` based on the promotion rules and type conversions.

use jsavrs::ir::type_promotion::{
    PrecisionLossEstimate, PromotionRule, generate_precision_loss_warning, generate_signedness_change_warning,
    generate_unicode_validation_warning,
};
use jsavrs::ir::{CastKind, IrType};

// ============================================================================
// Tests for generate_precision_loss_warning returning None
// ============================================================================

#[test]
fn test_precision_loss_warning_none_when_no_precision_loss() {
    // Rule that doesn't lose precision
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntSignExtend,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None,
    };

    let result = generate_precision_loss_warning(&IrType::I8, &IrType::I16, &rule);
    assert!(result.is_none(), "Expected None when may_lose_precision is false");
}

#[test]
fn test_precision_loss_warning_none_when_no_estimate() {
    // Rule with may_lose_precision=true but no estimate
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntTruncate,
        may_lose_precision: true,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None, // No estimate provided
    };

    let result = generate_precision_loss_warning(&IrType::I32, &IrType::I16, &rule);
    assert!(result.is_none(), "Expected None when precision_loss_estimate is None");
}

#[test]
fn test_precision_loss_warning_none_for_indirect_rule() {
    // Indirect rules don't have precision loss estimates
    let rule = PromotionRule::Indirect {
        intermediate_type: IrType::U32,
        first_cast: CastKind::IntSignExtend,
        second_cast: CastKind::Bitcast,
        requires_runtime_support: false,
    };

    let result = generate_precision_loss_warning(&IrType::I8, &IrType::U16, &rule);
    assert!(result.is_none(), "Expected None for Indirect promotion rule");
}

#[test]
fn test_precision_loss_warning_none_for_forbidden_rule() {
    // Forbidden rules don't generate warnings
    let rule = PromotionRule::Forbidden { reason: "Test".to_string() };

    let result = generate_precision_loss_warning(&IrType::I8, &IrType::Bool, &rule);
    assert!(result.is_none(), "Expected None for Forbidden promotion rule");
}

#[test]
fn test_precision_loss_warning_some_when_valid() {
    // Valid case: should return Some
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::FloatTruncate,
        may_lose_precision: true,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: Some(PrecisionLossEstimate::SignificantDigits { lost_bits: 29 }),
    };

    let result = generate_precision_loss_warning(&IrType::F64, &IrType::F32, &rule);
    assert!(result.is_some(), "Expected Some when all conditions are met");
}

// ============================================================================
// Tests for generate_signedness_change_warning returning None
// ============================================================================

#[test]
fn test_signedness_warning_none_when_not_bitcast() {
    // Rule that's not IntBitcast
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntSignExtend,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None,
    };

    let result = generate_signedness_change_warning(&IrType::I8, &IrType::I16, &rule);
    assert!(result.is_none(), "Expected None when cast_kind is not IntBitcast");
}

#[test]
fn test_signedness_warning_none_when_same_signedness() {
    // Both types have same signedness
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntBitcast,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None,
    };

    // Both signed
    let result = generate_signedness_change_warning(&IrType::I32, &IrType::I32, &rule);
    assert!(result.is_none(), "Expected None when both types are signed");

    // Both unsigned
    let result = generate_signedness_change_warning(&IrType::U32, &IrType::U32, &rule);
    assert!(result.is_none(), "Expected None when both types are unsigned");
}

#[test]
fn test_signedness_warning_none_for_non_integer_types() {
    // Float types don't have signedness
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntBitcast,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None,
    };

    let result = generate_signedness_change_warning(&IrType::F32, &IrType::F64, &rule);
    assert!(result.is_none(), "Expected None for non-integer types");
}

#[test]
fn test_signedness_warning_none_for_indirect_rule() {
    // Indirect rules don't generate signedness warnings
    let rule = PromotionRule::Indirect {
        intermediate_type: IrType::I32,
        first_cast: CastKind::IntSignExtend,
        second_cast: CastKind::IntBitcast,
        requires_runtime_support: false,
    };

    let result = generate_signedness_change_warning(&IrType::I16, &IrType::U32, &rule);
    assert!(result.is_none(), "Expected None for Indirect promotion rule");
}

#[test]
fn test_signedness_warning_some_when_valid() {
    // Valid case: should return Some
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntBitcast,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None,
    };

    let result = generate_signedness_change_warning(&IrType::I32, &IrType::U32, &rule);
    assert!(result.is_some(), "Expected Some when signedness changes");
}

// ============================================================================
// Tests for generate_unicode_validation_warning returning None
// ============================================================================

#[test]
fn test_unicode_warning_none_for_non_char_target() {
    // Target type is not Char
    let result = generate_unicode_validation_warning(0xD800, &IrType::U32);
    assert!(result.is_none(), "Expected None when target type is not Char");

    let result = generate_unicode_validation_warning(0xD800, &IrType::I32);
    assert!(result.is_none(), "Expected None when target type is I32");

    let result = generate_unicode_validation_warning(0xFFFFFFFF, &IrType::Bool);
    assert!(result.is_none(), "Expected None when target type is Bool");
}

#[test]
fn test_unicode_warning_none_for_valid_scalars() {
    // Valid Unicode scalar values should not generate warnings
    let result = generate_unicode_validation_warning(0x0041, &IrType::Char); // 'A'
    assert!(result.is_none(), "Expected None for valid Unicode scalar 'A'");

    let result = generate_unicode_validation_warning(0x0000, &IrType::Char); // NULL
    assert!(result.is_none(), "Expected None for valid Unicode scalar NULL");

    let result = generate_unicode_validation_warning(0xD7FF, &IrType::Char); // Before surrogates
    assert!(result.is_none(), "Expected None for valid scalar before surrogates");

    let result = generate_unicode_validation_warning(0xE000, &IrType::Char); // After surrogates
    assert!(result.is_none(), "Expected None for valid scalar after surrogates");

    let result = generate_unicode_validation_warning(0x10FFFF, &IrType::Char); // Max valid
    assert!(result.is_none(), "Expected None for max valid Unicode scalar");
}

#[test]
fn test_unicode_warning_some_for_surrogates() {
    // Surrogate values should generate warnings
    let result = generate_unicode_validation_warning(0xD800, &IrType::Char);
    assert!(result.is_some(), "Expected Some for surrogate start 0xD800");

    let result = generate_unicode_validation_warning(0xDFFF, &IrType::Char);
    assert!(result.is_some(), "Expected Some for surrogate end 0xDFFF");

    let result = generate_unicode_validation_warning(0xDABC, &IrType::Char);
    assert!(result.is_some(), "Expected Some for surrogate in middle");
}

#[test]
fn test_unicode_warning_some_for_out_of_range() {
    // Values beyond U+10FFFF should generate warnings
    let result = generate_unicode_validation_warning(0x110000, &IrType::Char);
    assert!(result.is_some(), "Expected Some for value beyond max");

    let result = generate_unicode_validation_warning(0xFFFFFFFF, &IrType::Char);
    assert!(result.is_some(), "Expected Some for very large value");
}
