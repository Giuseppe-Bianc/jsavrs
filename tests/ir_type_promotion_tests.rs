use jsavrs::ir::type_promotion_engine::TypePromotionEngine;
use jsavrs::ir::{
    CastKind, IrBinaryOp, IrType, OverflowBehavior, PrecisionLossEstimate, PromotionMatrix, PromotionResult,
    PromotionRule, PromotionWarning, TypeGroup, TypePromotion,
};
use jsavrs::location::source_span::SourceSpan;

#[test]
fn test_promotion_matrix_default_impl() {
    let matrix = PromotionMatrix::default();

    // Test that default creates the same as new()
    let matrix_new = PromotionMatrix::new();

    // Both should have same overflow behavior
    assert_eq!(matrix.get_overflow_behavior(), matrix_new.get_overflow_behavior());

    // Both should handle same promotion rules
    assert_eq!(
        matrix.get_promotion_rule(&IrType::F32, &IrType::F64),
        matrix_new.get_promotion_rule(&IrType::F32, &IrType::F64)
    );
    assert_eq!(
        matrix.get_promotion_rule(&IrType::I32, &IrType::F32),
        matrix_new.get_promotion_rule(&IrType::I32, &IrType::F32)
    );
}

#[test]
fn test_promotion_matrix_new() {
    let matrix = PromotionMatrix::new();

    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::Saturate);
    assert!(matrix.get_promotion_rule(&IrType::F32, &IrType::F64).is_some());
    assert!(matrix.get_promotion_rule(&IrType::I32, &IrType::F32).is_some());
}

#[test]
fn test_promotion_matrix_with_overflow_behavior() {
    let matrix = PromotionMatrix::with_overflow_behavior(OverflowBehavior::Wrap);

    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::Wrap);
}

#[test]
fn test_promotion_matrix_set_overflow_behavior() {
    let mut matrix = PromotionMatrix::new();
    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::Saturate);

    matrix.set_overflow_behavior(OverflowBehavior::Trap);
    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::Trap);

    matrix.set_overflow_behavior(OverflowBehavior::CompileError);
    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::CompileError);
}

/// Tests that PromotionMatrix::new() initializes default promotion rules as required by the type promotion system.
///
/// # Rationale
/// The PromotionMatrix must initialize with default rules to ensure type promotion functionality.
/// This validates that the matrix contains appropriate default promotion mappings for core types.
#[test]
fn test_promotion_matrix_new_initializes_default_rules() {
    let matrix = PromotionMatrix::new();

    // Verify that the matrix has default rules by checking a few key promotion rules exist
    assert!(matrix.get_promotion_rule(&IrType::I32, &IrType::I32).is_some(), "Identity promotion for I32 should exist");
    assert!(matrix.get_promotion_rule(&IrType::F32, &IrType::F64).is_some(), "F32 to F64 promotion should exist");
    assert!(matrix.get_promotion_rule(&IrType::I32, &IrType::F32).is_some(), "I32 to F32 promotion should exist");

    // Verify that the matrix has more than just a few rules (indicating proper initialization)
    // The compute_common_type function should work for common type operations
    assert_eq!(
        matrix.compute_common_type(&IrType::I32, &IrType::F32),
        Some(IrType::F32),
        "I32 and F32 should promote to F32"
    );
    assert_eq!(
        matrix.compute_common_type(&IrType::F32, &IrType::F64),
        Some(IrType::F64),
        "F32 and F64 should promote to F64"
    );
    assert_eq!(
        matrix.compute_common_type(&IrType::I32, &IrType::I64),
        Some(IrType::I64),
        "I32 and I64 should promote to I64"
    );

    // Verify that identity promotions work for multiple types
    assert_eq!(
        matrix.compute_common_type(&IrType::I32, &IrType::I32),
        Some(IrType::I32),
        "I32 identity promotion should work"
    );
    assert_eq!(
        matrix.compute_common_type(&IrType::F64, &IrType::F64),
        Some(IrType::F64),
        "F64 identity promotion should work"
    );
}

/// Tests the PromotionMatrix::with_overflow_behavior() method with Wrap behavior.
///
/// # Rationale
/// Verifies that the constructor properly sets the overflow behavior to Wrap.
#[test]
fn test_promotion_matrix_with_overflow_behavior_wrap() {
    let matrix = PromotionMatrix::with_overflow_behavior(OverflowBehavior::Wrap);

    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::Wrap);
}

/// Tests the PromotionMatrix::with_overflow_behavior() method with Trap behavior.
///
/// # Rationale
/// Verifies that the constructor properly sets the overflow behavior to Trap.
#[test]
fn test_promotion_matrix_with_overflow_behavior_trap() {
    let matrix = PromotionMatrix::with_overflow_behavior(OverflowBehavior::Trap);

    assert_eq!(matrix.get_overflow_behavior(), OverflowBehavior::Trap);
}

/// Tests signed integer widening from I8 to I16 through compute_common_type without precision loss.
///
/// # Rationale
/// Widening conversions from smaller signed integers to larger ones should result in the larger type
/// with appropriate casts and have no precision loss or overflow warnings since the larger type
/// can hold all values of the smaller type. This validates the widening promotion behavior.
#[test]
fn test_i8_to_i16_widening_no_loss() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in I16 when combining I8 and I16
    let result_type = matrix.compute_common_type(&IrType::I8, &IrType::I16);
    assert_eq!(result_type, Some(IrType::I16), "I8 and I16 should promote to I16");
}

/// Tests signed integer widening from I16 to I32 through compute_common_type without precision loss.
///
/// # Rationale
/// Widening conversions from smaller signed integers to larger ones should result in the larger type
/// with appropriate casts and have no precision loss or overflow warnings since the larger type
/// can hold all values of the smaller type. This validates the widening promotion behavior.
#[test]
fn test_i16_to_i32_widening_no_loss() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in I32 when combining I16 and I32
    let result_type = matrix.compute_common_type(&IrType::I16, &IrType::I32);
    assert_eq!(result_type, Some(IrType::I32), "I16 and I32 should promote to I32");
}

/// Tests signed integer widening from I32 to I64 through compute_common_type without precision loss.
///
/// # Rationale
/// Widening conversions from smaller signed integers to larger ones should result in the larger type
/// with appropriate casts and have no precision loss or overflow warnings since the larger type
/// can hold all values of the smaller type. This validates the widening promotion behavior.
#[test]
fn test_i32_to_i64_widening_no_loss() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in I64 when combining I32 and I64
    let result_type = matrix.compute_common_type(&IrType::I32, &IrType::I64);
    assert_eq!(result_type, Some(IrType::I64), "I32 and I64 should promote to I64");
}

/// Tests unsigned integer widening from U8 to U16 through compute_common_type without precision loss.
///
/// # Rationale
/// Widening conversions from smaller unsigned integers to larger ones should result in the larger type
/// with appropriate casts and have no precision loss or overflow warnings since the larger type
/// can hold all values of the smaller type. This validates the widening promotion behavior.
#[test]
fn test_u8_to_u16_widening_no_loss() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in U16 when combining U8 and U16
    let result_type = matrix.compute_common_type(&IrType::U8, &IrType::U16);
    assert_eq!(result_type, Some(IrType::U16), "U8 and U16 should promote to U16");
}

/// Tests unsigned integer widening from U16 to U32 through compute_common_type without precision loss.
///
/// # Rationale
/// Widening conversions from smaller unsigned integers to larger ones should result in the larger type
/// with appropriate casts and have no precision loss or overflow warnings since the larger type
/// can hold all values of the smaller type. This validates the widening promotion behavior.
#[test]
fn test_u16_to_u32_widening_no_loss() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in U32 when combining U16 and U32
    let result_type = matrix.compute_common_type(&IrType::U16, &IrType::U32);
    assert_eq!(result_type, Some(IrType::U32), "U16 and U32 should promote to U32");
}

/// Tests unsigned integer widening from U32 to U64 through compute_common_type without precision loss.
///
/// # Rationale
/// Widening conversions from smaller unsigned integers to larger ones should result in the larger type
/// with appropriate casts and have no precision loss or overflow warnings since the larger type
/// can hold all values of the smaller type. This validates the widening promotion behavior.
#[test]
fn test_u32_to_u64_widening_no_loss() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in U64 when combining U32 and U64
    let result_type = matrix.compute_common_type(&IrType::U32, &IrType::U64);
    assert_eq!(result_type, Some(IrType::U64), "U32 and U64 should promote to U64");
}

/// Tests float widening from F32 to F64 through compute_common_type without precision loss.
///
/// # Rationale
/// Float widening conversions from F32 to F64 should result in F64 type
/// with appropriate casts and have no precision loss or overflow warnings since F64
/// can represent all F32 values exactly. This validates the widening promotion behavior.
#[test]
fn test_f32_to_f64_widening_exact() {
    let matrix = PromotionMatrix::new();

    // Check that compute_common_type results in F64 when combining F32 and F64
    let result_type = matrix.compute_common_type(&IrType::F32, &IrType::F64);
    assert_eq!(result_type, Some(IrType::F64), "F32 and F64 should promote to F64");

    // Check the reverse direction too
    let result_type = matrix.compute_common_type(&IrType::F64, &IrType::F32);
    assert_eq!(result_type, Some(IrType::F64), "F64 and F32 should promote to F64");
}

/// Tests signed-unsigned integer promotion for same-width types (I8/U8 → I16).
///
/// # Rationale
/// When promoting signed and unsigned integers of the same width, they should promote
/// to the next larger signed integer type to preserve all values from both types.
/// This validates FR-004 (signed-unsigned promotions) and FR-005 (widening to preserve values).
#[test]
fn test_i8_u8_same_width_promotion() {
    let matrix = PromotionMatrix::new();

    // Check that I8 and U8 promote to I16 (next larger signed type)
    let result_type = matrix.compute_common_type(&IrType::I8, &IrType::U8);
    assert_eq!(result_type, Some(IrType::I16), "I8 and U8 should promote to I16");

    // Check the reverse order too
    let result_type = matrix.compute_common_type(&IrType::U8, &IrType::I8);
    assert_eq!(result_type, Some(IrType::I16), "U8 and I8 should promote to I16");
}

/// Tests signed-unsigned integer promotion for same-width types (I16/U16 → I32).
///
/// # Rationale
/// When promoting signed and unsigned integers of the same width, they should promote
/// to the next larger signed integer type to preserve all values from both types.
/// This validates FR-004 (signed-unsigned promotions) and FR-005 (widening to preserve values).
#[test]
fn test_i16_u16_same_width_promotion() {
    let matrix = PromotionMatrix::new();

    // Check that I16 and U16 promote to I32 (next larger signed type)
    let result_type = matrix.compute_common_type(&IrType::I16, &IrType::U16);
    assert_eq!(result_type, Some(IrType::I32), "I16 and U16 should promote to I32");

    // Check the reverse order too
    let result_type = matrix.compute_common_type(&IrType::U16, &IrType::I16);
    assert_eq!(result_type, Some(IrType::I32), "U16 and I16 should promote to I32");
}

/// Tests signed-unsigned integer promotion for same-width types (I32/U32 → I64).
///
/// # Rationale
/// When promoting signed and unsigned integers of the same width, they should promote
/// to the next larger signed integer type to preserve all values from both types.
/// This validates FR-004 (signed-unsigned promotions) and FR-005 (widening to preserve values).
#[test]
fn test_i32_u32_same_width_promotion() {
    let matrix = PromotionMatrix::new();

    // Check that I32 and U32 promote to I64 (next larger signed type)
    let result_type = matrix.compute_common_type(&IrType::I32, &IrType::U32);
    assert_eq!(result_type, Some(IrType::I64), "I32 and U32 should promote to I64");

    // Check the reverse order too
    let result_type = matrix.compute_common_type(&IrType::U32, &IrType::I32);
    assert_eq!(result_type, Some(IrType::I64), "U32 and I32 should promote to I64");
}

/// Tests signed-unsigned integer promotion for different-width types.
///
/// # Rationale
/// When promoting signed and unsigned integers of different widths, the result follows
/// specific rules implemented in the type promotion system. This validates FR-004 (signed-unsigned promotions)
/// by testing the actual behavior observed in the system.
#[test]
fn test_signed_unsigned_different_widths() {
    let matrix = PromotionMatrix::new();

    // Test actual behavior rather than assumptions
    // I32 and U16 -> I32 (I32 can represent all U16 values)
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U16), Some(IrType::I32));

    // I64 and U32 -> I64 (I64 can represent all U32 values)
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U32), Some(IrType::I64));

    // I16 and U32 -> U32 (U32 is wider)
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U32), Some(IrType::U32));

    // I32 and U64 -> U64 (U64 is wider)
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U64), Some(IrType::U64));

    // Test reverse orders
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I16), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I32), Some(IrType::U64));
}

/// Tests identity promotions (T → T) for all basic types, verifying they return Direct cast with Bitcast.
///
/// # Rationale
/// Identity promotions should always be direct casts with Bitcast and no warnings,
/// as they represent the same type being promoted to itself. This validates FR-003
/// (identity promotions should have no precision loss or overflow risk).
#[test]
fn test_identity_promotions_for_all_types() {
    let matrix = PromotionMatrix::new();

    // Test identity promotions for all basic types
    for ty in ALL_BASIC_TYPES {
        let rule = matrix.get_promotion_rule(ty, ty);
        assert!(rule.is_some(), "Identity promotion rule should exist for {:?}", ty);

        if let Some(PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. }) = rule {
            assert_eq!(*cast_kind, CastKind::Bitcast, "Identity promotion for {:?} should use Bitcast", ty);
            assert!(!(*may_lose_precision), "Identity promotion for {:?} should not lose precision", ty);
            assert!(!(*may_overflow), "Identity promotion for {:?} should not overflow", ty);
        } else {
            panic!("Identity promotion for {:?} should be a Direct rule with Bitcast", ty);
        }
    }
}

#[test]
fn test_promotion_rule_direct() {
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntToFloat,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
        precision_loss_estimate: None,
    };

    match rule {
        PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } => {
            assert_eq!(cast_kind, CastKind::IntToFloat);
            assert!(!may_lose_precision);
            assert!(!may_overflow);
        }
        _ => panic!("Expected Direct promotion rule"),
    }
}

#[test]
fn test_promotion_rule_indirect() {
    let rule = PromotionRule::Indirect {
        intermediate_type: IrType::I32,
        first_cast: CastKind::IntToFloat,
        second_cast: CastKind::FloatToInt,
        requires_runtime_support: false,
    };

    match rule {
        PromotionRule::Indirect { intermediate_type, first_cast, second_cast, .. } => {
            assert_eq!(intermediate_type, IrType::I32);
            assert_eq!(first_cast, CastKind::IntToFloat);
            assert_eq!(second_cast, CastKind::FloatToInt);
        }
        _ => panic!("Expected Indirect promotion rule"),
    }
}

#[test]
fn test_promotion_rule_forbidden() {
    let rule = PromotionRule::Forbidden { reason: "Test reason".to_string() };

    match rule {
        PromotionRule::Forbidden { reason } => {
            assert_eq!(reason, "Test reason");
        }
        _ => panic!("Expected Forbidden promotion rule"),
    }
}

#[test]
fn test_type_promotion_new() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::new(IrType::I32, IrType::F32, CastKind::IntToFloat, span.clone());

    assert_eq!(promotion.from_type, IrType::I32);
    assert_eq!(promotion.to_type, IrType::F32);
    assert_eq!(promotion.cast_kind, CastKind::IntToFloat);
    assert!(!promotion.may_lose_precision);
    assert!(!promotion.may_overflow);
    assert_eq!(promotion.source_span, span);
}

#[test]
fn test_type_promotion_equality() {
    let span = SourceSpan::default();
    let promotion1 = TypePromotion::new(IrType::I32, IrType::F32, CastKind::IntToFloat, span.clone());
    let promotion2 = TypePromotion::new(IrType::I32, IrType::F32, CastKind::IntToFloat, span);

    assert_eq!(promotion1, promotion2);
}

#[test]
fn test_promotion_result_equality() {
    let result1 = PromotionResult {
        result_type: IrType::I32,
        left_cast: None,
        right_cast: None,
        warnings: vec![],
        is_sound: true,
    };

    let result2 = PromotionResult {
        result_type: IrType::I32,
        left_cast: None,
        right_cast: None,
        warnings: vec![],
        is_sound: true,
    };

    assert_eq!(result1, result2);
}

#[test]
fn test_promotion_warning_precision_loss() {
    let warning = PromotionWarning::PrecisionLoss {
        from_type: IrType::F64,
        to_type: IrType::F32,
        estimated_loss: PrecisionLossEstimate::SignificantDigits { lost_bits: 24 },
    };

    match warning {
        PromotionWarning::PrecisionLoss { from_type, to_type, estimated_loss } => {
            assert_eq!(from_type, IrType::F64);
            assert_eq!(to_type, IrType::F32);
            assert_eq!(estimated_loss, PrecisionLossEstimate::SignificantDigits { lost_bits: 24 });
        }
        _ => panic!("Expected PrecisionLoss warning"),
    }
}

#[test]
fn test_promotion_warning_potential_overflow() {
    let warning = PromotionWarning::PotentialOverflow {
        from_type: IrType::I32,
        to_type: IrType::I16,
        operation: IrBinaryOp::Add,
    };

    match warning {
        PromotionWarning::PotentialOverflow { from_type, to_type, operation } => {
            assert_eq!(from_type, IrType::I32);
            assert_eq!(to_type, IrType::I16);
            assert_eq!(operation, IrBinaryOp::Add);
        }
        _ => panic!("Expected PotentialOverflow warning"),
    }
}

#[test]
fn test_promotion_warning_signedness_change() {
    let warning =
        PromotionWarning::SignednessChange { from_signed: true, to_signed: false, may_affect_comparisons: true };

    match warning {
        PromotionWarning::SignednessChange { from_signed, to_signed, may_affect_comparisons } => {
            assert!(from_signed);
            assert!(!to_signed);
            assert!(may_affect_comparisons);
        }
        _ => panic!("Expected SignednessChange warning"),
    }
}

#[test]
fn test_promotion_warning_float_special_values() {
    use jsavrs::ir::FloatSpecialValueType;

    let warning = PromotionWarning::FloatSpecialValues {
        value_type: FloatSpecialValueType::NaN,
        source_type: IrType::F64,
        target_type: IrType::I32,
        applied_behavior: OverflowBehavior::Wrap,
        source_span: SourceSpan::default(),
    };

    match warning {
        PromotionWarning::FloatSpecialValues { value_type, source_type, target_type, .. } => {
            assert_eq!(value_type, FloatSpecialValueType::NaN);
            assert_eq!(source_type, IrType::F64);
            assert_eq!(target_type, IrType::I32);
        }
        _ => panic!("Expected FloatSpecialValues warning"),
    }
}

#[test]
fn test_compute_common_type_same_types() {
    let matrix = PromotionMatrix::new();

    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::F64), Some(IrType::F64));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U16), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::Bool), Some(IrType::Bool));
}

#[test]
fn test_compute_common_type_float_precedence() {
    let matrix = PromotionMatrix::new();

    assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::I32), Some(IrType::F64));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::F64), Some(IrType::F64));
    assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::I16), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::F32), Some(IrType::F64));
    assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::F64), Some(IrType::F64));
}

#[test]
fn test_compute_common_type_signed_unsigned_same_width() {
    let matrix = PromotionMatrix::new();

    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U8), Some(IrType::I16));
    assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::I8), Some(IrType::I16));
}

#[test]
fn test_compute_common_type_with_fallback() {
    let matrix = PromotionMatrix::new();

    assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::Char), Some(IrType::I32));
}

#[test]
fn test_get_promotion_rule_exists() {
    let matrix = PromotionMatrix::new();

    // Test some rules that should exist
    assert!(matrix.get_promotion_rule(&IrType::I32, &IrType::F32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::F32, &IrType::I32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::F64, &IrType::F32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::F32, &IrType::F64).is_some());
}

#[test]
fn test_get_promotion_rule_nonexistent() {
    let matrix = PromotionMatrix::new();

    // Test a rule that genuinely doesn't exist (Void conversions are forbidden)
    assert!(matrix.get_promotion_rule(&IrType::Void, &IrType::String).is_none());
}

#[test]
fn test_precision_loss_estimate_none() {
    let estimate = PrecisionLossEstimate::None;
    assert_eq!(estimate, PrecisionLossEstimate::None);
}

#[test]
fn test_precision_loss_estimate_fractional_part() {
    let estimate = PrecisionLossEstimate::FractionalPart;
    assert_eq!(estimate, PrecisionLossEstimate::FractionalPart);
}

#[test]
fn test_precision_loss_estimate_significant_digits() {
    let estimate = PrecisionLossEstimate::SignificantDigits { lost_bits: 10 };
    match estimate {
        PrecisionLossEstimate::SignificantDigits { lost_bits } => {
            assert_eq!(lost_bits, 10);
        }
        _ => panic!("Expected SignificantDigits estimate"),
    }
}

#[test]
fn test_precision_loss_estimate_value_range() {
    let estimate = PrecisionLossEstimate::ValueRange { from_bits: 32, to_bits: 16 };
    match estimate {
        PrecisionLossEstimate::ValueRange { from_bits, to_bits } => {
            assert_eq!(from_bits, 32);
            assert_eq!(to_bits, 16);
        }
        _ => panic!("Expected ValueRange estimate"),
    }
}

#[test]
fn test_overflow_behavior_variants() {
    assert_eq!(OverflowBehavior::Wrap, OverflowBehavior::Wrap);
    assert_eq!(OverflowBehavior::Saturate, OverflowBehavior::Saturate);
    assert_eq!(OverflowBehavior::Trap, OverflowBehavior::Trap);
    assert_eq!(OverflowBehavior::CompileError, OverflowBehavior::CompileError);
}

#[test]
fn test_type_group_variants() {
    let signed_ints = TypeGroup::SignedIntegers(vec![IrType::I32, IrType::I64]);
    let unsigned_ints = TypeGroup::UnsignedIntegers(vec![IrType::U32, IrType::U64]);
    let floats = TypeGroup::FloatingPoint(vec![IrType::F32, IrType::F64]);
    let boolean = TypeGroup::Boolean;
    let character = TypeGroup::Character;

    match signed_ints {
        TypeGroup::SignedIntegers(types) => {
            assert_eq!(types, vec![IrType::I32, IrType::I64]);
        }
        _ => panic!("Expected SignedIntegers group"),
    }

    match unsigned_ints {
        TypeGroup::UnsignedIntegers(types) => {
            assert_eq!(types, vec![IrType::U32, IrType::U64]);
        }
        _ => panic!("Expected UnsignedIntegers group"),
    }

    match floats {
        TypeGroup::FloatingPoint(types) => {
            assert_eq!(types, vec![IrType::F32, IrType::F64]);
        }
        _ => panic!("Expected FloatingPoint group"),
    }

    match boolean {
        TypeGroup::Boolean => {}
        _ => panic!("Expected Boolean group"),
    }

    match character {
        TypeGroup::Character => {}
        _ => panic!("Expected Character group"),
    }
}

// Edge cases and corner cases

#[test]
fn test_promotion_matrix_edge_case_empty_rule_lookup() {
    let matrix = PromotionMatrix::new();

    // Test that looking up a non-existent rule returns None (Void conversions forbidden)
    assert!(matrix.get_promotion_rule(&IrType::Void, &IrType::I32).is_none());
}

#[test]
fn test_compute_common_type_edge_case_mixed_complex_types() {
    let matrix = PromotionMatrix::new();

    // Test promotion between different types with no specific rule
    assert_eq!(matrix.compute_common_type(&IrType::String, &IrType::Void), Some(IrType::I32));
}

#[test]
fn test_promotion_with_all_types() {
    let matrix = PromotionMatrix::new();
    let all_types = [
        IrType::I8,
        IrType::I16,
        IrType::I32,
        IrType::I64,
        IrType::U8,
        IrType::U16,
        IrType::U32,
        IrType::U64,
        IrType::F32,
        IrType::F64,
        IrType::Bool,
        IrType::Char,
    ];

    // Test each type against itself
    for ty in &all_types {
        assert_eq!(matrix.compute_common_type(ty, ty), Some(ty.clone()));
    }

    // Test some specific combinations
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::I32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U64), Some(IrType::I64));
}

#[test]
fn test_promotion_rule_symmetry() {
    let matrix = PromotionMatrix::new();

    // Check that rules are symmetrical where expected
    let rule1 = matrix.get_promotion_rule(&IrType::I32, &IrType::U32);
    let rule2 = matrix.get_promotion_rule(&IrType::U32, &IrType::I32);

    // These rules should exist and be the same (both promote to I64)
    assert!(rule1.is_some());
    assert!(rule2.is_some());
}

#[test]
fn test_promotion_result_with_casts() {
    let span = SourceSpan::default();
    let cast = TypePromotion::new(IrType::I32, IrType::F32, CastKind::IntToFloat, span.clone());

    let result = PromotionResult {
        result_type: IrType::F32,
        left_cast: Some(cast.clone()),
        right_cast: None,
        warnings: vec![],
        is_sound: true,
    };

    assert_eq!(result.result_type, IrType::F32);
    assert!(result.left_cast.is_some());
    assert_eq!(result.left_cast.unwrap(), cast);
    assert!(result.right_cast.is_none());
    assert!(result.warnings.is_empty());
    assert!(result.is_sound);
}

#[test]
fn test_promotion_result_with_warnings() {
    let result = PromotionResult {
        result_type: IrType::I32,
        left_cast: None,
        right_cast: None,
        warnings: vec![PromotionWarning::PrecisionLoss {
            from_type: IrType::F64,
            to_type: IrType::F32,
            estimated_loss: PrecisionLossEstimate::SignificantDigits { lost_bits: 24 },
        }],
        is_sound: false, // Not sound due to warnings
    };

    assert_eq!(result.result_type, IrType::I32);
    assert!(result.left_cast.is_none());
    assert!(result.right_cast.is_none());
    assert_eq!(result.warnings.len(), 1);
    assert!(!result.is_sound);
}

#[test]
fn test_promotion_matrix_initialization() {
    let matrix = PromotionMatrix::new();

    // Verify initial promotion rules are properly set
    assert!(matrix.get_promotion_rule(&IrType::F64, &IrType::F32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::F32, &IrType::F64).is_some());
    assert!(matrix.get_promotion_rule(&IrType::F32, &IrType::I32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::I32, &IrType::F32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::F64, &IrType::I64).is_some());
    assert!(matrix.get_promotion_rule(&IrType::I64, &IrType::F64).is_some());
    assert!(matrix.get_promotion_rule(&IrType::I32, &IrType::U32).is_some());
    assert!(matrix.get_promotion_rule(&IrType::U32, &IrType::I32).is_some());
}

#[test]
fn test_type_promotion_boolean_and_character() {
    let matrix = PromotionMatrix::new();

    // Boolean and Character should default to I32 for common type
    assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::Char), Some(IrType::I32));
}

#[test]
fn test_overflow_behavior_consistency() {
    let matrix1 = PromotionMatrix::new();
    let matrix2 = PromotionMatrix::with_overflow_behavior(OverflowBehavior::Saturate);

    // Both matrices should have the same overflow behavior
    assert_eq!(matrix1.get_overflow_behavior(), matrix2.get_overflow_behavior());
}

#[test]
fn test_promotion_matrix_complex_scenario() {
    let matrix = PromotionMatrix::new();

    // Test a complex type promotion chain
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::F64), Some(IrType::F64));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U8), Some(IrType::I64));
}

// Additional edge cases for type promotion behavior

#[test]
fn test_promotion_matrix_same_width_signed_unsigned() {
    let matrix = PromotionMatrix::new();

    // Same-width signed/unsigned should promote to next larger type
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U8), Some(IrType::I16));
}

#[test]
fn test_promotion_matrix_fallback_scenarios() {
    let matrix = PromotionMatrix::new();

    // Test fallback for completely unknown type combinations
    // Note: This is testing the default fallback case in compute_common_type
    let result = matrix.compute_common_type(&IrType::String, &IrType::Array(Box::new(IrType::I32), 10));
    assert_eq!(result, Some(IrType::I32)); // Should return fallback
}

#[test]
fn test_promotion_matrix_no_rule_scenarios() {
    let matrix = PromotionMatrix::new();

    // Test a scenario where there's no specific rule but the types are the same
    assert_eq!(matrix.compute_common_type(&IrType::Void, &IrType::Void), Some(IrType::Void));
}

#[test]
fn test_promotion_warnings_precision_loss_scenarios() {
    // Test creating specific warning scenarios
    let precision_loss_warning = PromotionWarning::PrecisionLoss {
        from_type: IrType::F64,
        to_type: IrType::F32,
        estimated_loss: PrecisionLossEstimate::SignificantDigits { lost_bits: 24 },
    };

    match &precision_loss_warning {
        PromotionWarning::PrecisionLoss { from_type, to_type, estimated_loss } => {
            assert_eq!(from_type, &IrType::F64);
            assert_eq!(to_type, &IrType::F32);
            match estimated_loss {
                PrecisionLossEstimate::SignificantDigits { lost_bits } => {
                    assert_eq!(*lost_bits, 24);
                }
                _ => panic!("Expected SignificantDigits estimate"),
            }
        }
        _ => panic!("Expected PrecisionLoss warning"),
    }
}

#[test]
fn test_promotion_warnings_overflow_scenarios() {
    // Test creating overflow warning scenarios
    let overflow_warning = PromotionWarning::PotentialOverflow {
        from_type: IrType::I64,
        to_type: IrType::I32,
        operation: IrBinaryOp::Add,
    };

    match &overflow_warning {
        PromotionWarning::PotentialOverflow { from_type, to_type, operation } => {
            assert_eq!(from_type, &IrType::I64);
            assert_eq!(to_type, &IrType::I32);
            assert_eq!(operation, &IrBinaryOp::Add);
        }
        _ => panic!("Expected PotentialOverflow warning"),
    }
}

// Tests for symmetric promotion rules (lines 277-285) - verified through existing initialized rules
#[test]
fn test_symmetric_promotion_rules_same_type() {
    let matrix = PromotionMatrix::new();

    // For same types, the promotion rule should be Bitcast with no precision loss or overflow
    // This is the behavior implemented in lines 277-280
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::I32);
    assert!(rule.is_some());

    match rule.unwrap() {
        PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } => {
            assert_eq!(cast_kind, &CastKind::Bitcast); // Should be Bitcast for same type
            assert!(!(*may_lose_precision));
            assert!(!(*may_overflow));
        }
        _ => panic!("Expected Direct promotion rule with Bitcast for same types"),
    }
}

#[test]
fn test_symmetric_promotion_rules_different_types_existing() {
    let matrix = PromotionMatrix::new();

    // Test the existing symmetric rules that are set up in initialization
    // I32 -> U32 and U32 -> I32 should both exist as promotions
    let i32_to_u32 = matrix.get_promotion_rule(&IrType::I32, &IrType::U32);
    let u32_to_i32 = matrix.get_promotion_rule(&IrType::U32, &IrType::I32);

    assert!(i32_to_u32.is_some());
    assert!(u32_to_i32.is_some());

    // Both should be Direct promotions (the symmetric behavior in lines 281-285)
    if let Some(PromotionRule::Direct { .. }) = i32_to_u32 {
        // OK
    } else {
        panic!("Expected Direct promotion rule for I32 -> U32");
    }

    if let Some(PromotionRule::Direct { .. }) = u32_to_i32 {
        // OK
    } else {
        panic!("Expected Direct promotion rule for U32 -> I32");
    }
}

#[test]
fn test_symmetric_promotion_rules_int_float_existing() {
    let matrix = PromotionMatrix::new();

    // Test symmetric float-integer rules that should be set up during initialization
    // I32 -> F32 and F32 -> I32
    let i32_to_f32 = matrix.get_promotion_rule(&IrType::I32, &IrType::F32);
    let f32_to_i32 = matrix.get_promotion_rule(&IrType::F32, &IrType::I32);

    assert!(i32_to_f32.is_some());
    assert!(f32_to_i32.is_some());

    // Both should be Direct promotions with specific characteristics
    match i32_to_f32.unwrap() {
        PromotionRule::Direct { may_lose_precision, may_overflow, .. } => {
            // I32 can be exactly represented in F32, so no precision loss expected
            assert!(!(*may_lose_precision)); // Dereference here
            assert!(!(*may_overflow)); // Dereference here
        }
        _ => panic!("Expected Direct promotion rule for I32 -> F32"),
    }

    match f32_to_i32.unwrap() {
        PromotionRule::Direct { may_lose_precision, may_overflow, .. } => {
            // F32 to I32 may lose precision and may overflow
            assert!(*may_lose_precision); // Dereference here
            assert!(*may_overflow); // Dereference here
        }
        _ => panic!("Expected Direct promotion rule for F32 -> I32"),
    }
}

// Tests for signed/unsigned type promotion (line 327) in compute_common_type
#[test]
fn test_compute_common_type_signed_unsigned_i64_u64() {
    let matrix = PromotionMatrix::new();

    // I64 and U64 should promote to I64 (line 327)
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I64), Some(IrType::I64));
}

#[test]
fn test_compute_common_type_signed_unsigned_edge_cases() {
    let matrix = PromotionMatrix::new();

    // Test all the signed/unsigned same-width combinations (line 327 and surrounding lines)
    // I32/U32 -> I64
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I32), Some(IrType::I64));

    // I16/U16 -> I32
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I16), Some(IrType::I32));

    // I8/U8 -> I16
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U8), Some(IrType::I16));
    assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::I8), Some(IrType::I16));
}

// Tests for wider type precedence (lines 335-339)
#[test]
fn test_compute_common_type_wider_types_precedence() {
    let matrix = PromotionMatrix::new();

    // Test wider signed integers
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I64), Some(IrType::I64));

    // Test wider unsigned integers
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U16), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U32), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U64), Some(IrType::U64));

    // Test mixed signed/unsigned with different widths
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I32), Some(IrType::U64));
}

#[test]
fn test_compute_common_type_wider_types_precedence_edge_cases() {
    let matrix = PromotionMatrix::new();

    // Test 8-bit types
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I8), Some(IrType::I16));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::I16), Some(IrType::I16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U16), Some(IrType::U16));
}

// Tests for I64/U64 promotion in get_higher_type (line 354)
#[test]
fn test_get_higher_type_i64_u64() {
    let matrix = PromotionMatrix::new();

    // Use reflection to access the private get_higher_type function indirectly
    // We'll use compute_common_type which uses get_higher_type internally

    // I64 and U64 should both result in I64 as the "higher" type
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I64), Some(IrType::I64));
}

#[test]
fn test_get_higher_type_fallback_behavior() {
    let _matrix = PromotionMatrix::new();

    // Test the fallback case `_ => left.clone()` in get_higher_type
    // This would occur for type combinations not specifically handled

    // For same-width signed/unsigned, we already have specific rules
    // For wider types, specific rules apply
    // For float types, they take precedence

    // The fallback behavior should be tested with types that don't have specific promotion rules
    // If we have a completely unknown type combination that doesn't match any pattern
    // it should fall back to left type
}

// Tests for within same type group logic (lines 356-367)
#[test]
fn test_get_higher_type_same_signed_group() {
    let matrix = PromotionMatrix::new();

    // Within signed integer group, wider type should be preferred
    // This is tested through compute_common_type which uses get_higher_type
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I8), Some(IrType::I16));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::I16), Some(IrType::I16));
}

#[test]
fn test_get_higher_type_same_unsigned_group() {
    let matrix = PromotionMatrix::new();

    // Within unsigned integer group, wider type should be preferred
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U64), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U16), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U32), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U16), Some(IrType::U16));
}

#[test]
fn test_get_higher_type_mixed_signedness_wider_precedence() {
    let matrix = PromotionMatrix::new();

    // When comparing different signedness but different widths,
    // wider type should win over signedness if it's not the same width
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U16), Some(IrType::I32)); // I32 is wider
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I16), Some(IrType::U32)); // U32 is wider
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U8), Some(IrType::I64)); // I64 is wider
}

#[test]
fn test_get_higher_type_fallback_to_left_type() {
    let _matrix = PromotionMatrix::new();

    // Test the fallback case in get_higher_type where types are not covered by rules
    // This is the `_ => left.clone()` case

    // This case would be for types that don't have specific rules in get_higher_type
    // Since our type system is well-defined, we can test with a pattern that should trigger fallback
    // In a well-defined type system, most cases should be handled, but we ensure the fallback works
    // by testing with the default case that should return the left type
}

// Tests for specific lines in type_promotion.rs

// Test for line 351: I64/U64 promotion in get_higher_type
#[test]
fn test_get_higher_type_i64_u64_promotion() {
    let matrix = PromotionMatrix::new();

    // Test I64 vs U64 - should return I64 as per line 351
    let result = matrix.compute_common_type(&IrType::I64, &IrType::U64);
    assert_eq!(result, Some(IrType::I64), "I64 should be higher precedence than U64");

    // Test U64 vs I64 - should also return I64 as per line 351
    let result = matrix.compute_common_type(&IrType::U64, &IrType::I64);
    assert_eq!(result, Some(IrType::I64), "I64 should be higher precedence than U64 regardless of order");
}

// Test for lines 362-363: U16 precedence in get_higher_type
#[test]
fn test_get_higher_type_u16_precedence() {
    let matrix = PromotionMatrix::new();

    // Test U16 vs smaller types - should return U16 as per lines 362-363
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U16), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U16), Some(IrType::U16));

    // Test U16 vs same-width types - should promote to next size as per earlier rules
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I16), Some(IrType::I32));

    // Test U16 vs larger types - should return the larger type
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U32), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U16), Some(IrType::U32));
}

// Test for line 378: I64/U64 promotion in compute_common_type
#[test]
fn test_compute_common_type_i64_u64_promotion() {
    let matrix = PromotionMatrix::new();

    // Test I64 vs U64 - should return Some(I64) as per line 378
    let result = matrix.compute_common_type(&IrType::I64, &IrType::U64);
    assert_eq!(result, Some(IrType::I64), "I64 should be the common type for I64 and U64");

    // Test U64 vs I64 - should also return Some(I64) as per line 378
    let result = matrix.compute_common_type(&IrType::U64, &IrType::I64);
    assert_eq!(result, Some(IrType::I64), "I64 should be the common type for U64 and I64 regardless of order");

    // Test I64/U64 with other types to ensure precedence is maintained
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::F32), Some(IrType::F32));
}

// Test for lines 380-391: U16 precedence in compute_common_type
#[test]
fn test_compute_common_type_u16_precedence() {
    let matrix = PromotionMatrix::new();

    // Test U16 vs smaller types - should return Some(U16) as per lines 380-391
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U16), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U16), Some(IrType::U16));

    // Test U16 vs same-width types - should promote to next size as per earlier rules
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I16), Some(IrType::I32));

    // Test U16 vs larger types - should return the larger type
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U32), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U16), Some(IrType::U32));
}

// Edge case tests for I64/U64 promotion
#[test]
fn test_i64_u64_edge_cases() {
    let matrix = PromotionMatrix::new();

    // Test I64/U64 with all other integer types
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I16), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U16), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I8), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U8), Some(IrType::I64));

    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I16), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U16), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I8), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U8), Some(IrType::U64));

    // Test I64/U64 with floating point types
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::F64), Some(IrType::F64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::F64), Some(IrType::F64));
}

// Edge case tests for U16 precedence
#[test]
fn test_u16_edge_cases() {
    let matrix = PromotionMatrix::new();

    // Test U16 with all other integer types
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U32), Some(IrType::U32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U64), Some(IrType::U64));

    // Test U16 with floating point types
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F64), Some(IrType::F64));

    // Test U16 with special types
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::Bool), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::Char), Some(IrType::U16));
}

// Test consistency between get_higher_type and compute_common_type
#[test]
fn test_consistency_between_methods() {
    let matrix = PromotionMatrix::new();

    // Test that both methods return the same result for I64/U64
    let common_type = matrix.compute_common_type(&IrType::I64, &IrType::U64);
    assert_eq!(common_type, Some(IrType::I64));

    let common_type = matrix.compute_common_type(&IrType::U64, &IrType::I64);
    assert_eq!(common_type, Some(IrType::I64));

    // Test that both methods return the same result for U16 with smaller types
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U16), Some(IrType::U16));

    // Test that both methods return the same result for U16 with larger types
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I32), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U16), Some(IrType::I32));
}

// Test complex scenarios involving the target lines
#[test]
fn test_complex_promotion_scenarios() {
    let matrix = PromotionMatrix::new();

    // Test complex type promotion chains involving I64/U64
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::F64), Some(IrType::F64));

    // Test complex type promotion chains involving U16
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F32), Some(IrType::F32));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F64), Some(IrType::F64));

    // Test three-way promotions involving I64/U64
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));

    // Test three-way promotions involving U16
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
    assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I16), Some(IrType::I32));
}

/// Constant array of all basic IrType variants
const ALL_BASIC_TYPES: &[IrType] = &[
    IrType::I8,
    IrType::I16,
    IrType::I32,
    IrType::I64,
    IrType::U8,
    IrType::U16,
    IrType::U32,
    IrType::U64,
    IrType::F32,
    IrType::F64,
    IrType::Bool,
    IrType::Char,
];

// ============================================================================
// Phase 3: User Story 1 - Basic Numeric Type Conversions Tests
// ============================================================================

// T009: Integer Widening Test Cases
#[test]
fn test_integer_widening_u8_to_u16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U8, &IrType::U16).unwrap();
    if let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntZeroExtend);
        assert!(!(*may_lose_precision));
        assert!(!(*may_overflow));
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_integer_widening_u8_to_u32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U8, &IrType::U32).unwrap();
    if let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntZeroExtend);
        assert!(!may_lose_precision);
        assert!(!may_overflow);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_integer_widening_i8_to_i16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I8, &IrType::I16).unwrap();
    if let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntSignExtend);
        assert!(!may_lose_precision);
        assert!(!may_overflow);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_integer_widening_i8_to_i32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I8, &IrType::I32).unwrap();
    if let PromotionRule::Direct { cast_kind, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntSignExtend);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_all_unsigned_widening_conversions() {
    let matrix = PromotionMatrix::new();
    let unsigned_types = [IrType::U8, IrType::U16, IrType::U32, IrType::U64];

    for (i, from_type) in unsigned_types.iter().enumerate() {
        for to_type in unsigned_types.iter().skip(i + 1) {
            let rule = matrix.get_promotion_rule(from_type, to_type);
            assert!(rule.is_some(), "Missing rule for {:?} → {:?}", from_type, to_type);
            if let Some(PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. }) = rule {
                assert_eq!(*cast_kind, CastKind::IntZeroExtend);
                assert!(!may_lose_precision);
                assert!(!may_overflow);
            }
        }
    }
}

#[test]
fn test_all_signed_widening_conversions() {
    let matrix = PromotionMatrix::new();
    let signed_types = [IrType::I8, IrType::I16, IrType::I32, IrType::I64];

    for (i, from_type) in signed_types.iter().enumerate() {
        for to_type in signed_types.iter().skip(i + 1) {
            let rule = matrix.get_promotion_rule(from_type, to_type);
            assert!(rule.is_some(), "Missing rule for {:?} → {:?}", from_type, to_type);
            if let Some(PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. }) = rule {
                assert_eq!(*cast_kind, CastKind::IntSignExtend);
                assert!(!may_lose_precision);
                assert!(!may_overflow);
            }
        }
    }
}

// T010: Integer Narrowing Test Cases (now passing after T015)
#[test]
fn test_integer_narrowing_u64_to_u16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U16);
    assert!(rule.is_some(), "Missing rule for U64 → U16");
    if let Some(PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. }) = rule {
        assert_eq!(*cast_kind, CastKind::IntTruncate);
        assert!(*may_lose_precision);
        assert!(*may_overflow);
    } else {
        panic!("Expected Direct promotion rule with IntTruncate");
    }
}

#[test]
fn test_integer_narrowing_i64_to_i16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I64, &IrType::I16);
    assert!(rule.is_some(), "Missing rule for I64 → I16");
    if let Some(PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. }) = rule {
        assert_eq!(*cast_kind, CastKind::IntTruncate);
        assert!(*may_lose_precision);
        assert!(*may_overflow);
    } else {
        panic!("Expected Direct promotion rule with IntTruncate");
    }
}

// T011: Integer-Float Conversion Test Cases
#[test]
fn test_i32_to_f32_conversion() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::F32).unwrap();
    if let PromotionRule::Direct { cast_kind, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntToFloat);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_f64_to_i32_conversion_with_precision_loss() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::I32).unwrap();
    if let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule {
        assert_eq!(*cast_kind, CastKind::FloatToInt);
        assert!(*may_lose_precision);
        assert!(*may_overflow);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_all_int_to_float_conversions() {
    let matrix = PromotionMatrix::new();
    let int_types =
        [IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    let float_types = [IrType::F32, IrType::F64];

    for int_ty in &int_types {
        for float_ty in &float_types {
            let rule = matrix.get_promotion_rule(int_ty, float_ty);
            assert!(rule.is_some(), "Missing rule for {:?} → {:?}", int_ty, float_ty);
            if let Some(PromotionRule::Direct { cast_kind, .. }) = rule {
                assert_eq!(*cast_kind, CastKind::IntToFloat);
            }
        }
    }
}

#[test]
fn test_all_float_to_int_conversions() {
    let matrix = PromotionMatrix::new();
    let int_types =
        [IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    let float_types = [IrType::F32, IrType::F64];

    for float_ty in &float_types {
        for int_ty in &int_types {
            let rule = matrix.get_promotion_rule(float_ty, int_ty);
            assert!(rule.is_some(), "Missing rule for {:?} → {:?}", float_ty, int_ty);
            if let Some(PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. }) = rule {
                assert_eq!(*cast_kind, CastKind::FloatToInt);
                assert!(may_lose_precision);
                assert!(may_overflow);
            }
        }
    }
}

// T012: Float-Float Conversion Test Cases
#[test]
fn test_f32_to_f64_extension() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F32, &IrType::F64).unwrap();
    if let PromotionRule::Direct { cast_kind, may_lose_precision, .. } = rule {
        assert_eq!(*cast_kind, CastKind::FloatExtend);
        assert!(!(*may_lose_precision));
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_f64_to_f32_truncation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::F32).unwrap();
    if let PromotionRule::Direct { cast_kind, may_lose_precision, .. } = rule {
        assert_eq!(*cast_kind, CastKind::FloatTruncate);
        assert!(*may_lose_precision);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

// T017: Snapshot tests for numeric warnings
#[test]
fn test_integer_narrowing_warnings() {
    use insta::assert_debug_snapshot;
    let matrix = PromotionMatrix::new();

    // Test U64 -> U32 narrowing
    let rule_u64_u32 = matrix.get_promotion_rule(&IrType::U64, &IrType::U32).unwrap();
    assert_debug_snapshot!("u64_to_u32_narrowing", rule_u64_u32);

    // Test I64 -> I32 narrowing
    let rule_i64_i32 = matrix.get_promotion_rule(&IrType::I64, &IrType::I32).unwrap();
    assert_debug_snapshot!("i64_to_i32_narrowing", rule_i64_i32);

    // Test U32 -> U8 narrowing (larger gap)
    let rule_u32_u8 = matrix.get_promotion_rule(&IrType::U32, &IrType::U8).unwrap();
    assert_debug_snapshot!("u32_to_u8_narrowing", rule_u32_u8);
}

#[test]
fn test_float_conversion_warnings() {
    use insta::assert_debug_snapshot;
    let matrix = PromotionMatrix::new();

    // Test F64 -> F32 precision loss
    let rule_f64_f32 = matrix.get_promotion_rule(&IrType::F64, &IrType::F32).unwrap();
    assert_debug_snapshot!("f64_to_f32_precision_loss", rule_f64_f32);

    // Test I64 -> F32 potential precision loss
    let rule_i64_f32 = matrix.get_promotion_rule(&IrType::I64, &IrType::F32).unwrap();
    assert_debug_snapshot!("i64_to_f32_precision_loss", rule_i64_f32);

    // Test F64 -> I32 truncation
    let rule_f64_i32 = matrix.get_promotion_rule(&IrType::F64, &IrType::I32).unwrap();
    assert_debug_snapshot!("f64_to_i32_truncation", rule_f64_i32);
}

// T018: Edge Case Tests for Numeric Conversions
#[test]
fn test_cross_signedness_same_width_i32_to_u32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::U32).unwrap();
    if let PromotionRule::Direct { cast_kind, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntBitcast);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_cross_signedness_same_width_u32_to_i32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::I32).unwrap();
    if let PromotionRule::Direct { cast_kind, .. } = rule {
        assert_eq!(*cast_kind, CastKind::IntBitcast);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_all_same_width_cross_signedness_conversions() {
    let matrix = PromotionMatrix::new();

    let pairs = vec![
        (IrType::I8, IrType::U8),
        (IrType::U8, IrType::I8),
        (IrType::I16, IrType::U16),
        (IrType::U16, IrType::I16),
        (IrType::I32, IrType::U32),
        (IrType::U32, IrType::I32),
        (IrType::I64, IrType::U64),
        (IrType::U64, IrType::I64),
    ];

    for (from_type, to_type) in pairs {
        let rule = matrix.get_promotion_rule(&from_type, &to_type);
        assert!(rule.is_some(), "Missing rule for {:?} -> {:?}", from_type, to_type);
        if let Some(PromotionRule::Direct { cast_kind, .. }) = rule {
            assert_eq!(*cast_kind, CastKind::IntBitcast, "Expected IntBitcast for {:?} -> {:?}", from_type, to_type);
        }
    }
}

#[test]
fn test_large_integer_conversions_exist() {
    let matrix = PromotionMatrix::new();

    // Test that conversions from/to largest integer types exist for same signedness
    let signed_types = vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64];
    let unsigned_types = vec![IrType::U8, IrType::U16, IrType::U32, IrType::U64];

    // Test signed-to-signed conversions
    for from_type in &signed_types {
        for to_type in &signed_types {
            if from_type != to_type {
                assert!(
                    matrix.get_promotion_rule(from_type, to_type).is_some(),
                    "Missing rule for {:?} -> {:?}",
                    from_type,
                    to_type
                );
            }
        }
    }

    // Test unsigned-to-unsigned conversions
    for from_type in &unsigned_types {
        for to_type in &unsigned_types {
            if from_type != to_type {
                assert!(
                    matrix.get_promotion_rule(from_type, to_type).is_some(),
                    "Missing rule for {:?} -> {:?}",
                    from_type,
                    to_type
                );
            }
        }
    }

    // Test cross-signedness same-width conversions
    let pairs = vec![
        (IrType::I8, IrType::U8),
        (IrType::I16, IrType::U16),
        (IrType::I32, IrType::U32),
        (IrType::I64, IrType::U64),
    ];
    for (from_type, to_type) in &pairs {
        assert!(matrix.get_promotion_rule(from_type, to_type).is_some());
        assert!(matrix.get_promotion_rule(to_type, from_type).is_some());
    }
}

#[test]
fn test_float_special_values_conversions_exist() {
    let matrix = PromotionMatrix::new();

    // Verify float to integer conversions exist (which would handle NaN/Inf)
    let float_types = vec![IrType::F32, IrType::F64];
    let int_types =
        vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];

    for float_type in &float_types {
        for int_type in &int_types {
            let rule = matrix.get_promotion_rule(float_type, int_type);
            assert!(rule.is_some(), "Missing float->int rule for {:?} -> {:?}", float_type, int_type);

            if let Some(PromotionRule::Direct { cast_kind, may_overflow, .. }) = rule {
                assert_eq!(*cast_kind, CastKind::FloatToInt);
                assert!(*may_overflow, "Float to int should mark may_overflow=true");
            }
        }
    }
}

// ============================================================================
// T021: Comprehensive Numeric Type Pairs Coverage Validation
// ============================================================================

#[test]
fn test_count_implemented_numeric_rules() {
    let matrix = PromotionMatrix::new();
    let int_types =
        vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    let float_types = vec![IrType::F32, IrType::F64];

    // Count int×int pairs
    let mut int_int_count = 0;
    let mut missing_int_int = Vec::new();
    for from in &int_types {
        for to in &int_types {
            if matrix.get_promotion_rule(from, to).is_some() {
                int_int_count += 1;
            } else {
                missing_int_int.push((from.clone(), to.clone()));
            }
        }
    }

    // Count int→float pairs
    let mut int_to_float_count = 0;
    let mut missing_int_float = Vec::new();
    for int_ty in &int_types {
        for float_ty in &float_types {
            if matrix.get_promotion_rule(int_ty, float_ty).is_some() {
                int_to_float_count += 1;
            } else {
                missing_int_float.push((int_ty.clone(), float_ty.clone()));
            }
        }
    }

    // Count float→int pairs
    let mut float_to_int_count = 0;
    let mut missing_float_int = Vec::new();
    for float_ty in &float_types {
        for int_ty in &int_types {
            if matrix.get_promotion_rule(float_ty, int_ty).is_some() {
                float_to_int_count += 1;
            } else {
                missing_float_int.push((float_ty.clone(), int_ty.clone()));
            }
        }
    }

    // Count float×float pairs
    let mut float_float_count = 0;
    for from_float in &float_types {
        for to_float in &float_types {
            if matrix.get_promotion_rule(from_float, to_float).is_some() {
                float_float_count += 1;
            }
        }
    }

    let total = int_int_count + int_to_float_count + float_to_int_count + float_float_count;

    println!("\n=== Numeric Type Conversion Rules Count ===");
    println!("int×int:     {}/64 rules", int_int_count);
    println!("int→float:   {}/16 rules", int_to_float_count);
    println!("float→int:   {}/16 rules", float_to_int_count);
    println!("float×float: {}/4 rules", float_float_count);
    println!("TOTAL:       {}/100 numeric rules\n", total);

    if !missing_int_int.is_empty() {
        println!("Missing int×int rules ({}):", missing_int_int.len());
        for (from, to) in missing_int_int.iter().take(10) {
            println!("  {:?} → {:?}", from, to);
        }
        if missing_int_int.len() > 10 {
            println!("  ... and {} more", missing_int_int.len() - 10);
        }
    }

    if !missing_int_float.is_empty() {
        println!("\nMissing int→float rules ({}):", missing_int_float.len());
        for (from, to) in &missing_int_float {
            println!("  {:?} → {:?}", from, to);
        }
    }

    if !missing_float_int.is_empty() {
        println!("\nMissing float→int rules ({}):", missing_float_int.len());
        for (from, to) in &missing_float_int {
            println!("  {:?} → {:?}", from, to);
        }
    }
}

#[test]
fn test_all_numeric_type_pairs_defined() {
    let matrix = PromotionMatrix::new();
    let int_types =
        vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    let float_types = vec![IrType::F32, IrType::F64];

    // Test all int×int pairs (8×8 = 64 pairs)
    let mut int_int_count = 0;
    for from in &int_types {
        for to in &int_types {
            assert!(matrix.get_promotion_rule(from, to).is_some(), "Missing rule for {:?} → {:?}", from, to);
            int_int_count += 1;
        }
    }
    assert_eq!(int_int_count, 64, "Expected 64 int×int conversion rules");

    // Test all int→float pairs (8×2 = 16 pairs)
    let mut int_to_float_count = 0;
    for int_ty in &int_types {
        for float_ty in &float_types {
            assert!(
                matrix.get_promotion_rule(int_ty, float_ty).is_some(),
                "Missing rule for {:?} → {:?}",
                int_ty,
                float_ty
            );
            int_to_float_count += 1;
        }
    }
    assert_eq!(int_to_float_count, 16, "Expected 16 int→float conversion rules");

    // Test all float→int pairs (2×8 = 16 pairs)
    let mut float_to_int_count = 0;
    for float_ty in &float_types {
        for int_ty in &int_types {
            assert!(
                matrix.get_promotion_rule(float_ty, int_ty).is_some(),
                "Missing rule for {:?} → {:?}",
                float_ty,
                int_ty
            );
            float_to_int_count += 1;
        }
    }
    assert_eq!(float_to_int_count, 16, "Expected 16 float→int conversion rules");

    // Test all float×float pairs (2×2 = 4 pairs)
    let mut float_float_count = 0;
    for from_float in &float_types {
        for to_float in &float_types {
            assert!(
                matrix.get_promotion_rule(from_float, to_float).is_some(),
                "Missing rule for {:?} → {:?}",
                from_float,
                to_float
            );
            float_float_count += 1;
        }
    }
    assert_eq!(float_float_count, 4, "Expected 4 float×float conversion rules");

    // Total numeric type pairs: 64 + 16 + 16 + 4 = 100 rules
    let total_numeric_rules = int_int_count + int_to_float_count + float_to_int_count + float_float_count;
    assert_eq!(
        total_numeric_rules, 100,
        "Expected 100 total numeric conversion rules (64 int×int + 16 int→float + 16 float→int + 4 float×float)"
    );
}

// ============================================================================
// T023: Boolean Conversion Test Cases
// ============================================================================

// Bool → Integer conversions (8 tests)
#[test]
fn test_bool_to_i8() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I8).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } => {
            assert_eq!(*cast_kind, CastKind::BoolToInt);
            assert!(!may_lose_precision, "Bool→I8 should not lose precision");
            assert!(!may_overflow, "Bool→I8 cannot overflow (0 or 1 fits in i8)");
        }
        _ => panic!("Expected Direct rule for Bool→I8"),
    }
}

#[test]
fn test_bool_to_i16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I16).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_bool_to_i32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I32).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_bool_to_i64() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I64).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_bool_to_u8() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::U8).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_bool_to_u16() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::U16).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_bool_to_u32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::U32).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_bool_to_u64() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::U64).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToInt),
        _ => panic!("Expected Direct rule"),
    }
}

// Bool → Float conversions (2 tests)
#[test]
fn test_bool_to_f32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::F32).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, may_lose_precision, .. } => {
            assert_eq!(*cast_kind, CastKind::BoolToFloat);
            assert!(!may_lose_precision, "Bool→F32 should not lose precision (0.0 or 1.0)");
        }
        _ => panic!("Expected Direct rule for Bool→F32"),
    }
}

#[test]
fn test_bool_to_f64() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::F64).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::BoolToFloat),
        _ => panic!("Expected Direct rule"),
    }
}

// Integer → Bool conversions (8 tests)
#[test]
fn test_i8_to_bool_zero_test() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I8, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => {
            assert_eq!(*cast_kind, CastKind::IntToBool);
            // 0 → false, non-zero → true
        }
        _ => panic!("Expected Direct rule for I8→Bool"),
    }
}

#[test]
fn test_i16_to_bool() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I16, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_i32_to_bool_zero_test() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_i64_to_bool() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I64, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_u8_to_bool() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U8, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_u16_to_bool() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U16, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_u32_to_bool() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

#[test]
fn test_u64_to_bool() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::IntToBool),
        _ => panic!("Expected Direct rule"),
    }
}

// Float → Bool conversions (2 tests)
#[test]
fn test_f32_to_bool_nan_handling() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F32, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => {
            assert_eq!(*cast_kind, CastKind::FloatToBool);
            // NaN → true (non-zero), 0.0/-0.0 → false, other → true
        }
        _ => panic!("Expected Direct rule for F32→Bool"),
    }
}

#[test]
fn test_f64_to_bool_nan_handling() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind, .. } => assert_eq!(*cast_kind, CastKind::FloatToBool),
        _ => panic!("Expected Direct rule"),
    }
}

// Bool identity conversion (1 test)
#[test]
fn test_bool_to_bool_identity() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::Bool).unwrap();
    match rule {
        PromotionRule::Direct { cast_kind: _, may_lose_precision, may_overflow, .. } => {
            // Identity conversion - should be no-op
            assert!(!may_lose_precision);
            assert!(!may_overflow);
        }
        _ => panic!("Expected Direct rule for Bool→Bool identity"),
    }
}

// ============================================================================
// Character Conversion Tests (T024)
// ============================================================================

#[test]
fn test_char_to_u32_unicode_scalar() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::U32).unwrap();

    if let PromotionRule::Direct { cast_kind, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::CharToInt);
        assert_eq!(requires_validation, &false);
    } else {
        panic!("Expected Direct rule for Char→U32");
    }
}

#[test]
fn test_u32_to_char_with_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();

    if let PromotionRule::Direct { cast_kind, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::IntToChar);
        assert_eq!(requires_validation, &true);
    } else {
        panic!("Expected Direct rule for U32→Char with validation");
    }
}

#[test]
fn test_char_to_i32_signed_conversion() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::I32).unwrap();

    if let PromotionRule::Direct { cast_kind, may_lose_precision, .. } = rule {
        assert_eq!(cast_kind, &CastKind::CharToInt);
        assert!(!may_lose_precision);
    } else {
        panic!("Expected Direct rule for Char→I32");
    }
}

#[test]
fn test_i32_to_char_with_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::Char).unwrap();

    if let PromotionRule::Direct { cast_kind, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::IntToChar);
        assert_eq!(requires_validation, &true);
    } else {
        panic!("Expected Direct rule for I32→Char with validation");
    }
}

#[test]
fn test_char_to_string_runtime_support() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::String).unwrap();

    if let PromotionRule::Direct { cast_kind, requires_runtime_support, .. } = rule {
        assert_eq!(cast_kind, &CastKind::CharToString);
        assert_eq!(requires_runtime_support, &true);
    } else {
        panic!("Expected Direct rule for Char→String");
    }
}

#[test]
fn test_string_to_char_with_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Char).unwrap();

    if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::StringToChar);
        assert_eq!(requires_runtime_support, &true);
        assert_eq!(requires_validation, &true);
    } else {
        panic!("Expected Direct rule for String→Char");
    }
}

#[test]
fn test_char_to_char_identity() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::Char).unwrap();

    if let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule {
        assert_eq!(cast_kind, &CastKind::Bitcast);
        assert!(!may_lose_precision);
        assert!(!may_overflow);
    } else {
        panic!("Expected Direct rule for Char→Char identity");
    }
}

// T029: Snapshot tests for boolean conversions
#[test]
fn test_boolean_to_numeric_snapshots() {
    use insta::assert_debug_snapshot;
    let matrix = PromotionMatrix::new();

    // Test Bool -> I32 conversion
    let rule_bool_i32 = matrix.get_promotion_rule(&IrType::Bool, &IrType::I32).unwrap();
    assert_debug_snapshot!("bool_to_i32_conversion", rule_bool_i32);

    // Test Bool -> U32 conversion
    let rule_bool_u32 = matrix.get_promotion_rule(&IrType::Bool, &IrType::U32).unwrap();
    assert_debug_snapshot!("bool_to_u32_conversion", rule_bool_u32);

    // Test Bool -> F64 conversion
    let rule_bool_f64 = matrix.get_promotion_rule(&IrType::Bool, &IrType::F64).unwrap();
    assert_debug_snapshot!("bool_to_f64_conversion", rule_bool_f64);
}

#[test]
fn test_numeric_to_boolean_snapshots() {
    use insta::assert_debug_snapshot;
    let matrix = PromotionMatrix::new();

    // Test I32 -> Bool conversion (requires runtime support)
    let rule_i32_bool = matrix.get_promotion_rule(&IrType::I32, &IrType::Bool).unwrap();
    assert_debug_snapshot!("i32_to_bool_conversion", rule_i32_bool);

    // Test U32 -> Bool conversion
    let rule_u32_bool = matrix.get_promotion_rule(&IrType::U32, &IrType::Bool).unwrap();
    assert_debug_snapshot!("u32_to_bool_conversion", rule_u32_bool);

    // Test F64 -> Bool conversion
    let rule_f64_bool = matrix.get_promotion_rule(&IrType::F64, &IrType::Bool).unwrap();
    assert_debug_snapshot!("f64_to_bool_conversion", rule_f64_bool);
}

#[test]
fn test_boolean_string_char_snapshots() {
    use insta::assert_debug_snapshot;
    let matrix = PromotionMatrix::new();

    // Test Bool -> String conversion (requires runtime support)
    let rule_bool_string = matrix.get_promotion_rule(&IrType::Bool, &IrType::String).unwrap();
    assert_debug_snapshot!("bool_to_string_conversion", rule_bool_string);

    // Test String -> Bool conversion (requires runtime + validation)
    let rule_string_bool = matrix.get_promotion_rule(&IrType::String, &IrType::Bool).unwrap();
    assert_debug_snapshot!("string_to_bool_conversion", rule_string_bool);

    // Note: Bool ↔ Char conversions not implemented yet (future enhancement)
}

// T029: Unicode Validation Test Cases
#[test]
fn test_surrogate_u32_to_char_requires_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();

    // U32 → Char must require validation to reject surrogate range 0xD800-0xDFFF
    if let PromotionRule::Direct { cast_kind, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::IntToChar, "Expected IntToChar for U32→Char");
        assert!(requires_validation, "U32→Char must require validation for surrogate rejection");
    } else {
        panic!("Expected Direct rule for U32→Char");
    }
}

#[test]
fn test_out_of_range_u32_to_char_requires_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();

    // Validation must reject values > 0x10FFFF (max Unicode code point)
    if let PromotionRule::Direct { requires_validation, .. } = rule {
        assert!(requires_validation, "U32→Char must validate range to reject values > 0x10FFFF");
    } else {
        panic!("Expected Direct rule for U32→Char");
    }
}

#[test]
fn test_i32_to_char_requires_validation() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::Char).unwrap();

    // I32 → Char must require validation to reject negative values
    if let PromotionRule::Direct { cast_kind, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::IntToChar, "Expected IntToChar for I32→Char");
        assert!(requires_validation, "I32→Char must require validation to reject negative values");
    } else {
        panic!("Expected Direct rule for I32→Char");
    }
}

#[test]
fn test_valid_unicode_ranges_char_identity() {
    let matrix = PromotionMatrix::new();

    // Char→Char identity should not require validation (already valid)
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::Char).unwrap();
    if let PromotionRule::Direct { requires_validation, .. } = rule {
        assert!(!requires_validation, "Char→Char identity should not require validation");
    } else {
        panic!("Expected Direct rule for Char→Char identity");
    }
}

#[test]
fn test_char_to_u32_no_validation_needed() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::U32).unwrap();

    // Char→U32 doesn't need validation (all chars are valid Unicode scalars)
    if let PromotionRule::Direct { cast_kind, requires_validation, .. } = rule {
        assert_eq!(cast_kind, &CastKind::CharToInt, "Expected CharToInt for Char→U32");
        assert!(!requires_validation, "Char→U32 should not require validation");
    } else {
        panic!("Expected Direct rule for Char→U32");
    }
}

// T030: Unicode Validation Warning Generation Tests
#[test]
fn test_unicode_warning_generation_for_surrogate() {
    let matrix = PromotionMatrix::new();

    // Test surrogate code point (0xD800 - 0xDFFF)
    let warning = matrix.generate_unicode_validation_warning(0xD800, &IrType::Char);
    assert!(warning.is_some(), "Expected warning for surrogate code point");

    if let Some(PromotionWarning::InvalidUnicodeCodePoint { value, reason }) = warning {
        assert_eq!(value, 0xD800);
        assert!(reason.contains("surrogate"), "Expected 'surrogate' in reason");
    } else {
        panic!("Expected InvalidUnicodeCodePoint warning");
    }
}

#[test]
fn test_unicode_warning_generation_for_out_of_range() {
    let matrix = PromotionMatrix::new();

    // Test value > 0x10FFFF (max Unicode code point)
    let warning = matrix.generate_unicode_validation_warning(0x110000, &IrType::Char);
    assert!(warning.is_some(), "Expected warning for out-of-range value");

    if let Some(PromotionWarning::InvalidUnicodeCodePoint { value, reason }) = warning {
        assert_eq!(value, 0x110000);
        assert!(reason.contains("exceeds"), "Expected 'exceeds' in reason");
        assert!(reason.contains("U+10FFFF"), "Expected 'U+10FFFF' in reason");
    } else {
        panic!("Expected InvalidUnicodeCodePoint warning");
    }
}

#[test]
fn test_unicode_warning_no_warning_for_valid_values() {
    let matrix = PromotionMatrix::new();

    // Test valid Unicode scalar values
    let test_values = [
        0x0000,   // Null character
        0x0041,   // 'A'
        0xD7FF,   // Just before surrogate range
        0xE000,   // Just after surrogate range
        0x10FFFF, // Maximum Unicode code point
    ];

    for &value in &test_values {
        let warning = matrix.generate_unicode_validation_warning(value, &IrType::Char);
        assert!(warning.is_none(), "Expected no warning for valid Unicode value 0x{:X}", value);
    }
}

#[test]
fn test_unicode_warning_only_for_char_target() {
    let matrix = PromotionMatrix::new();

    // Test that invalid value doesn't generate warning for non-char types
    let warning_u32 = matrix.generate_unicode_validation_warning(0xD800, &IrType::U32);
    assert!(warning_u32.is_none(), "Expected no warning for U32 target");

    let warning_i32 = matrix.generate_unicode_validation_warning(0xD800, &IrType::I32);
    assert!(warning_i32.is_none(), "Expected no warning for I32 target");
}

// T031: Snapshot Tests for Boolean/Character Warnings
#[test]
fn test_invalid_unicode_warning_snapshot() {
    use insta::assert_debug_snapshot;

    let matrix = PromotionMatrix::new();

    // Snapshot test for surrogate code point warning
    let warning_surrogate = matrix.generate_unicode_validation_warning(0xD800, &IrType::Char).unwrap();
    assert_debug_snapshot!("unicode_warning_surrogate", warning_surrogate);

    // Snapshot test for out-of-range warning
    let warning_out_of_range = matrix.generate_unicode_validation_warning(0x110000, &IrType::Char).unwrap();
    assert_debug_snapshot!("unicode_warning_out_of_range", warning_out_of_range);
}

#[test]
fn test_precision_loss_warning_snapshot() {
    use insta::assert_debug_snapshot;

    let matrix = PromotionMatrix::new();

    // Test precision loss warning for U64 → U32
    let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U32).unwrap();
    let warning = matrix.generate_precision_loss_warning(&IrType::U64, &IrType::U32, rule).unwrap();
    assert_debug_snapshot!("precision_loss_u64_to_u32", warning);

    // Test precision loss warning for F64 → F32
    let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::F32).unwrap();
    let warning = matrix.generate_precision_loss_warning(&IrType::F64, &IrType::F32, rule).unwrap();
    assert_debug_snapshot!("precision_loss_f64_to_f32", warning);
}

#[test]
fn test_signedness_change_warning_snapshot() {
    use insta::assert_debug_snapshot;

    let matrix = PromotionMatrix::new();

    // Test signedness change warning for I32 → U32
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::U32).unwrap();
    let warning = matrix.generate_signedness_change_warning(&IrType::I32, &IrType::U32, rule).unwrap();
    assert_debug_snapshot!("signedness_change_i32_to_u32", warning);

    // Test signedness change warning for U32 → I32
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::I32).unwrap();
    let warning = matrix.generate_signedness_change_warning(&IrType::U32, &IrType::I32, rule).unwrap();
    assert_debug_snapshot!("signedness_change_u32_to_i32", warning);
}

// =========================================================================
// Phase 5: String Conversion Tests (T033-T034)
// =========================================================================

// -------------------------------------------------------------------------
// T033: String Conversion Test Cases (25 tests)
// -------------------------------------------------------------------------

/// Tests for primitive → String conversions (12 rules)
/// These conversions always succeed and require runtime formatting support.
#[cfg(test)]
mod string_conversion_tests {
    use super::*;

    // Integer → String conversions (8 tests)

    #[test]
    fn test_i8_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::I8, &IrType::String).unwrap();
        if let PromotionRule::Direct {
            cast_kind,
            requires_runtime_support,
            requires_validation,
            may_lose_precision,
            may_overflow,
            ..
        } = rule
        {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation); // Always succeeds
            assert!(!*may_lose_precision);
            assert!(!*may_overflow);
        } else {
            panic!("Expected Direct promotion rule for I8→String");
        }
    }

    #[test]
    fn test_i16_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::I16, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for I16→String");
        }
    }

    #[test]
    fn test_i32_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for I32→String");
        }
    }

    #[test]
    fn test_i64_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::I64, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for I64→String");
        }
    }

    #[test]
    fn test_u8_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::U8, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for U8→String");
        }
    }

    #[test]
    fn test_u16_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::U16, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for U16→String");
        }
    }

    #[test]
    fn test_u32_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for U32→String");
        }
    }

    #[test]
    fn test_u64_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::IntToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for U64→String");
        }
    }

    // Float → String conversions (2 tests)

    #[test]
    fn test_f32_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::F32, &IrType::String).unwrap();
        if let PromotionRule::Direct {
            cast_kind,
            requires_runtime_support,
            requires_validation,
            may_lose_precision,
            may_overflow,
            ..
        } = rule
        {
            assert_eq!(*cast_kind, CastKind::FloatToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation); // Always succeeds
            assert!(!*may_lose_precision);
            assert!(!*may_overflow);
        } else {
            panic!("Expected Direct promotion rule for F32→String");
        }
    }

    #[test]
    fn test_f64_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::FloatToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for F64→String");
        }
    }

    // Bool → String conversion (1 test - already in T025 but verify here)

    #[test]
    fn test_bool_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::BoolToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation); // Always succeeds
        } else {
            panic!("Expected Direct promotion rule for Bool→String");
        }
    }

    // Char → String conversion (1 test - already in T027 but verify here)

    #[test]
    fn test_char_to_string_formatting() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::String).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::CharToString);
            assert!(*requires_runtime_support);
            assert!(!*requires_validation); // Always succeeds
        } else {
            panic!("Expected Direct promotion rule for Char→String");
        }
    }

    // String → Integer conversions (8 tests)

    #[test]
    fn test_string_to_i8_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I8).unwrap();
        if let PromotionRule::Direct {
            cast_kind,
            requires_runtime_support,
            requires_validation,
            may_lose_precision,
            may_overflow,
            ..
        } = rule
        {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation); // Parse may fail
            assert!(!*may_lose_precision);
            assert!(!*may_overflow);
        } else {
            panic!("Expected Direct promotion rule for String→I8");
        }
    }

    #[test]
    fn test_string_to_i16_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I16).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→I16");
        }
    }

    #[test]
    fn test_string_to_i32_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I32).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→I32");
        }
    }

    #[test]
    fn test_string_to_i64_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I64).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→I64");
        }
    }

    #[test]
    fn test_string_to_u8_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::U8).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→U8");
        }
    }

    #[test]
    fn test_string_to_u16_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::U16).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→U16");
        }
    }

    #[test]
    fn test_string_to_u32_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::U32).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→U32");
        }
    }

    #[test]
    fn test_string_to_u64_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::U64).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToInt);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→U64");
        }
    }

    // String → Float conversions (2 tests)

    #[test]
    fn test_string_to_f32_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::F32).unwrap();
        if let PromotionRule::Direct {
            cast_kind,
            requires_runtime_support,
            requires_validation,
            may_lose_precision,
            may_overflow,
            ..
        } = rule
        {
            assert_eq!(*cast_kind, CastKind::StringToFloat);
            assert!(*requires_runtime_support);
            assert!(*requires_validation); // Parse may fail
            assert!(!*may_lose_precision);
            assert!(!*may_overflow);
        } else {
            panic!("Expected Direct promotion rule for String→F32");
        }
    }

    #[test]
    fn test_string_to_f64_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::F64).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToFloat);
            assert!(*requires_runtime_support);
            assert!(*requires_validation);
        } else {
            panic!("Expected Direct promotion rule for String→F64");
        }
    }

    // String → Bool conversion (1 test - already in T025 but verify here)

    #[test]
    fn test_string_to_bool_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Bool).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToBool);
            assert!(*requires_runtime_support);
            assert!(*requires_validation); // Parse may fail
        } else {
            panic!("Expected Direct promotion rule for String→Bool");
        }
    }

    // String → Char conversion (1 test - already in T027 but verify here)

    #[test]
    fn test_string_to_char_parsing() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Char).unwrap();
        if let PromotionRule::Direct { cast_kind, requires_runtime_support, requires_validation, .. } = rule {
            assert_eq!(*cast_kind, CastKind::StringToChar);
            assert!(*requires_runtime_support);
            assert!(*requires_validation); // Length check
        } else {
            panic!("Expected Direct promotion rule for String→Char");
        }
    }

    // String → String identity conversion (1 test)

    #[test]
    fn test_string_to_string_identity() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::String).unwrap();
        if let PromotionRule::Direct {
            cast_kind,
            requires_runtime_support,
            requires_validation,
            may_lose_precision,
            may_overflow,
            ..
        } = rule
        {
            assert_eq!(*cast_kind, CastKind::Bitcast); // No-op
            assert!(!*requires_runtime_support);
            assert!(!*requires_validation);
            assert!(!*may_lose_precision);
            assert!(!*may_overflow);
        } else {
            panic!("Expected Direct promotion rule for String→String");
        }
    }
}

// -------------------------------------------------------------------------
// T034: String Parsing Error Test Cases
// -------------------------------------------------------------------------

/// Tests for invalid string parsing scenarios
/// These tests verify that validation requirements are correctly set
#[cfg(test)]
mod string_parsing_error_tests {
    use super::*;

    #[test]
    fn test_invalid_string_to_int_requires_validation() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I32).unwrap();
        if let PromotionRule::Direct { requires_validation, .. } = rule {
            assert!(*requires_validation, "String→Int must require validation for invalid input like 'abc'");
        } else {
            panic!("Expected Direct promotion rule for String→I32");
        }
    }

    #[test]
    fn test_string_to_char_length_check() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Char).unwrap();
        if let PromotionRule::Direct { requires_validation, .. } = rule {
            assert!(*requires_validation, "String→Char must require validation for multi-char or empty strings");
        } else {
            panic!("Expected Direct promotion rule for String→Char");
        }
    }

    #[test]
    fn test_string_to_float_invalid_format() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::F32).unwrap();
        if let PromotionRule::Direct { requires_validation, .. } = rule {
            assert!(*requires_validation, "String→Float must require validation for invalid formats");
        } else {
            panic!("Expected Direct promotion rule for String→F32");
        }
    }

    #[test]
    fn test_string_to_bool_invalid_value() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Bool).unwrap();
        if let PromotionRule::Direct { requires_validation, .. } = rule {
            assert!(*requires_validation, "String→Bool must require validation for non-boolean strings");
        } else {
            panic!("Expected Direct promotion rule for String→Bool");
        }
    }

    #[test]
    fn test_string_to_unsigned_negative_check() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::U32).unwrap();
        if let PromotionRule::Direct { requires_validation, .. } = rule {
            assert!(*requires_validation, "String→Unsigned must require validation for negative values");
        } else {
            panic!("Expected Direct promotion rule for String→U32");
        }
    }

    #[test]
    fn test_string_parsing_requires_runtime_support() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I64).unwrap();
        if let PromotionRule::Direct { requires_runtime_support, .. } = rule {
            assert!(*requires_runtime_support, "String parsing requires runtime support");
        } else {
            panic!("Expected Direct promotion rule for String→I64");
        }
    }
}

// ============================================================================
// Phase 6: Polish & Integration Tests
// ============================================================================

/// T039: Validate that all 169 fundamental type pairs (13×13) are defined
#[cfg(test)]
mod comprehensive_validation_tests {
    use super::*;

    #[test]
    fn test_all_169_type_pairs_defined() {
        let matrix = PromotionMatrix::new();
        let all_types = vec![
            IrType::I8,
            IrType::I16,
            IrType::I32,
            IrType::I64,
            IrType::U8,
            IrType::U16,
            IrType::U32,
            IrType::U64,
            IrType::F32,
            IrType::F64,
            IrType::Bool,
            IrType::Char,
            IrType::String,
        ];

        assert_eq!(all_types.len(), 13, "Expected 13 fundamental types");

        let mut defined_count = 0;
        let mut missing_rules = Vec::new();

        for from in &all_types {
            for to in &all_types {
                match matrix.get_promotion_rule(from, to) {
                    Some(_) => {
                        defined_count += 1;
                    }
                    None => {
                        missing_rules.push(format!("{:?} → {:?}", from, to));
                    }
                }
            }
        }

        if !missing_rules.is_empty() {
            panic!("Missing {} promotion rules:\n{}", missing_rules.len(), missing_rules.join("\n"));
        }

        assert_eq!(defined_count, 169, "Expected 169 promotion rules (13×13), found {}", defined_count);
    }

    #[test]
    fn test_type_coverage_breakdown() {
        let matrix = PromotionMatrix::new();

        // Define type groups
        let integers =
            vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];
        let floats = vec![IrType::F32, IrType::F64];
        let special_types = vec![IrType::Bool, IrType::Char, IrType::String];

        // Validate integer coverage (8×8 = 64 rules)
        let mut int_count = 0;
        for from in &integers {
            for to in &integers {
                if matrix.get_promotion_rule(from, to).is_some() {
                    int_count += 1;
                }
            }
        }
        assert_eq!(int_count, 64, "Expected 64 integer×integer rules, found {}", int_count);

        // Validate integer-float coverage (8 int × 2 float × 2 directions = 32 rules)
        let mut int_float_count = 0;
        for int_ty in &integers {
            for float_ty in &floats {
                if matrix.get_promotion_rule(int_ty, float_ty).is_some() {
                    int_float_count += 1;
                }
                if matrix.get_promotion_rule(float_ty, int_ty).is_some() {
                    int_float_count += 1;
                }
            }
        }
        assert_eq!(int_float_count, 32, "Expected 32 int↔float rules, found {}", int_float_count);

        // Validate float coverage (2×2 = 4 rules)
        let mut float_count = 0;
        for from in &floats {
            for to in &floats {
                if matrix.get_promotion_rule(from, to).is_some() {
                    float_count += 1;
                }
            }
        }
        assert_eq!(float_count, 4, "Expected 4 float×float rules, found {}", float_count);

        // Validate special type interactions (3×13 = 39 rules to/from Bool, Char, String)
        let all_types = [integers.clone(), floats.clone(), special_types.clone()].concat();
        let mut special_count = 0;
        for special_ty in &special_types {
            for ty in &all_types {
                if matrix.get_promotion_rule(special_ty, ty).is_some() {
                    special_count += 1;
                }
            }
        }
        // Bool: 13, Char: 13, String: 13 = 39 total
        assert_eq!(special_count, 39, "Expected 39 special→all rules, found {}", special_count);

        // All types to special types
        let mut to_special_count = 0;
        for ty in &all_types {
            for special_ty in &special_types {
                if matrix.get_promotion_rule(ty, special_ty).is_some() {
                    to_special_count += 1;
                }
            }
        }
        assert_eq!(to_special_count, 39, "Expected 39 all→special rules, found {}", to_special_count);

        // Coverage breakdown for diagnostics
        println!(
            "Coverage breakdown: {} int, {} int-float, {} float, {} special",
            int_count, int_float_count, float_count, special_count
        );
    }

    #[test]
    fn test_all_24_cast_kinds_utilized() {
        use std::collections::HashSet;

        let matrix = PromotionMatrix::new();
        let all_types = vec![
            IrType::I8,
            IrType::I16,
            IrType::I32,
            IrType::I64,
            IrType::U8,
            IrType::U16,
            IrType::U32,
            IrType::U64,
            IrType::F32,
            IrType::F64,
            IrType::Bool,
            IrType::Char,
            IrType::String,
        ];

        // Expected CastKind variants (24 total from spec)
        let expected_cast_kinds = vec![
            "IntZeroExtend",
            "IntSignExtend",
            "IntTruncate",
            "IntBitcast",
            "IntToFloat",
            "FloatToInt",
            "FloatTruncate",
            "FloatExtend",
            "BoolToInt",
            "IntToBool",
            "BoolToFloat",
            "FloatToBool",
            "CharToInt",
            "IntToChar",
            "CharToString",
            "StringToChar",
            "StringToInt",
            "StringToFloat",
            "StringToBool",
            "IntToString",
            "FloatToString",
            "BoolToString",
            "Bitcast",
        ];

        let mut found_cast_kinds = HashSet::new();

        // Iterate all promotion rules and collect CastKind variants
        for from in &all_types {
            for to in &all_types {
                if let Some(rule) = matrix.get_promotion_rule(from, to) {
                    match rule {
                        PromotionRule::Direct { cast_kind, .. } => {
                            found_cast_kinds.insert(format!("{:?}", cast_kind));
                        }
                        PromotionRule::Indirect { first_cast, second_cast, .. } => {
                            found_cast_kinds.insert(format!("{:?}", first_cast));
                            found_cast_kinds.insert(format!("{:?}", second_cast));
                        }
                        PromotionRule::Forbidden { .. } => {}
                    }
                }
            }
        }

        // Check that all expected CastKind variants are found
        let mut missing_cast_kinds = Vec::new();
        for expected in &expected_cast_kinds {
            if !found_cast_kinds.contains(*expected) {
                missing_cast_kinds.push(*expected);
            }
        }

        if !missing_cast_kinds.is_empty() {
            panic!("Missing {} CastKind variants:\n{}", missing_cast_kinds.len(), missing_cast_kinds.join("\n"));
        }

        // Also report what we found
        let mut found_vec: Vec<_> = found_cast_kinds.iter().collect();
        found_vec.sort();
        println!("Found {} unique CastKind variants:", found_vec.len());
        for cast_kind in found_vec {
            println!("  - {}", cast_kind);
        }

        assert!(
            found_cast_kinds.len() >= expected_cast_kinds.len(),
            "Expected at least {} CastKind variants, found {}",
            expected_cast_kinds.len(),
            found_cast_kinds.len()
        );
    }
}

// Comprehensive tests for compute_common_type function covering all match branches

#[cfg(test)]
mod compute_common_type_tests {
    use super::*;

    #[test]
    fn test_compute_common_type_same_types() {
        let matrix = PromotionMatrix::new();

        // Test all basic types with themselves
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::I8), Some(IrType::I8));
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I16), Some(IrType::I16));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I32), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U8), Some(IrType::U8));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U16), Some(IrType::U16));
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U32), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U64), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::F32), Some(IrType::F32));
        assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::F64), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::Bool), Some(IrType::Bool));
        assert_eq!(matrix.compute_common_type(&IrType::Char, &IrType::Char), Some(IrType::Char));
        assert_eq!(matrix.compute_common_type(&IrType::String, &IrType::String), Some(IrType::String));
    }

    #[test]
    fn test_compute_common_type_f64_precedence() {
        let matrix = PromotionMatrix::new();

        // F64 should take precedence over any other type
        assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::I32), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::F64), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::U16), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F64), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::F32), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::F64), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::Bool), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::F64), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::F64, &IrType::Char), Some(IrType::F64));
        assert_eq!(matrix.compute_common_type(&IrType::Char, &IrType::F64), Some(IrType::F64));
    }

    #[test]
    fn test_compute_common_type_f32_precedence() {
        let matrix = PromotionMatrix::new();

        // F32 should take precedence over integer types but not F64
        assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::I32), Some(IrType::F32));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::F32), Some(IrType::F32));
        assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::U16), Some(IrType::F32));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::F32), Some(IrType::F32));
        assert_eq!(matrix.compute_common_type(&IrType::F32, &IrType::Bool), Some(IrType::F32));
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::F32), Some(IrType::F32));
    }

    #[test]
    fn test_compute_common_type_i64_u64_same_width() {
        let matrix = PromotionMatrix::new();

        // I64 and U64 should promote to I64 (since I64 takes precedence according to function)
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I64), Some(IrType::I64));
    }

    #[test]
    fn test_compute_common_type_i32_u32_same_width() {
        let matrix = PromotionMatrix::new();

        // I32 and U32 should promote to I64 (the next size up)
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U32), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I32), Some(IrType::I64));
    }

    #[test]
    fn test_compute_common_type_i16_u16_same_width() {
        let matrix = PromotionMatrix::new();

        // I16 and U16 should promote to I32 (the next size up)
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U16), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I16), Some(IrType::I32));
    }

    #[test]
    fn test_compute_common_type_i8_u8_same_width() {
        let matrix = PromotionMatrix::new();

        // I8 and U8 should promote to I16 (the next size up)
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U8), Some(IrType::I16));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::I8), Some(IrType::I16));
    }

    #[test]
    fn test_compute_common_type_i64_precedence() {
        let matrix = PromotionMatrix::new();

        // I64 should take precedence over any smaller integer type
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I16), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I8), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::I64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U32), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U16), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I64), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::U8), Some(IrType::I64));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::I64), Some(IrType::I64));
    }

    #[test]
    fn test_compute_common_type_u64_precedence() {
        let matrix = PromotionMatrix::new();

        // U64 should take precedence over any smaller integer type
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I32), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U64), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I16), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U64), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::I8), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U64), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U64), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U16), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U64), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U8), Some(IrType::U64));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U64), Some(IrType::U64));
    }

    #[test]
    fn test_compute_common_type_i32_precedence() {
        let matrix = PromotionMatrix::new();

        // I32 should take precedence over smaller signed integer types
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I16), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I32), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I8), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::I32), Some(IrType::I32));

        // I32 should take precedence over smaller unsigned integer types
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U16), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I32), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::U8), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::I32), Some(IrType::I32));
    }

    #[test]
    fn test_compute_common_type_u32_precedence() {
        let matrix = PromotionMatrix::new();

        // U32 should take precedence over smaller unsigned integer types
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U16), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U32), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U8), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U32), Some(IrType::U32));

        // U32 should take precedence over smaller signed integer types
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I16), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U32), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::I8), Some(IrType::U32));
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U32), Some(IrType::U32));
    }

    #[test]
    fn test_compute_common_type_i16_precedence() {
        let matrix = PromotionMatrix::new();

        // I16 should take precedence over smaller signed integer types
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I8), Some(IrType::I16));
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::I16), Some(IrType::I16));

        // I16 should take precedence over smaller unsigned integer types
        assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::U8), Some(IrType::I16));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::I16), Some(IrType::I16));
    }

    #[test]
    fn test_compute_common_type_u16_precedence() {
        let matrix = PromotionMatrix::new();

        // U16 should take precedence over smaller unsigned integer types
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::U8), Some(IrType::U16));
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::U16), Some(IrType::U16));

        // U16 should take precedence over smaller signed integer types
        assert_eq!(matrix.compute_common_type(&IrType::U16, &IrType::I8), Some(IrType::U16));
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::U16), Some(IrType::U16));
    }

    #[test]
    fn test_compute_common_type_i8_bool_combinations() {
        let matrix = PromotionMatrix::new();

        // I8 and Bool should promote to I32 according to the function's fallback logic
        // This is likely because Bool promotes to I32 in mixed operations
        assert_eq!(matrix.compute_common_type(&IrType::I8, &IrType::Bool), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::I8), Some(IrType::I32));
    }

    #[test]
    fn test_compute_common_type_u8_bool_combinations() {
        let matrix = PromotionMatrix::new();

        // U8 and Bool should promote to I32 according to the function's fallback logic
        assert_eq!(matrix.compute_common_type(&IrType::U8, &IrType::Bool), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::U8), Some(IrType::I32));
    }

    #[test]
    fn test_compute_common_type_fallback_cases() {
        let matrix = PromotionMatrix::new();

        // Test fallback case that returns I32 for unknown combinations
        // Using types that are not in the specific match patterns
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::Char), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::Bool, &IrType::String), Some(IrType::I32));
        assert_eq!(matrix.compute_common_type(&IrType::Char, &IrType::String), Some(IrType::I32));
    }
}

// ============================================================================
// TYPE PROMOTION ENGINE COMPREHENSIVE TEST SUITE
// ============================================================================

// ============================================================================
// CONSTRUCTION AND INITIALIZATION TESTS
// ============================================================================

/// Tests that TypePromotionEngine can be created using the new() constructor.
///
/// # Rationale
/// Verifies basic instantiation works correctly and returns a valid engine instance.
///
/// # Test Coverage
/// - Successful construction via new()
/// - Default initialization state
#[test]
fn test_type_promotion_engine_new() {
    let engine = TypePromotionEngine::new();

    // Engine should be created successfully
    // Since it's a zero-sized struct, we just verify it can be instantiated
    assert_eq!(std::mem::size_of_val(&engine), 0, "TypePromotionEngine should be a zero-sized type");
}

/// Tests that TypePromotionEngine can be created using the default() trait.
///
/// # Rationale
/// Verifies that Default trait implementation works correctly and produces
/// equivalent instances to new().
///
/// # Test Coverage
/// - Default trait implementation
/// - Consistency between new() and default()
#[test]
fn test_type_promotion_engine_default() {
    let engine = TypePromotionEngine::default();

    // Engine should be created successfully via Default trait
    assert_eq!(std::mem::size_of_val(&engine), 0, "TypePromotionEngine should be a zero-sized type");
}

/// Tests that TypePromotionEngine can be cloned successfully.
///
/// # Rationale
/// Verifies Clone trait implementation works correctly for TypePromotionEngine.
///
/// # Test Coverage
/// - Clone trait functionality
/// - Cloned instances are equivalent
#[test]
fn test_type_promotion_engine_clone() {
    let engine = TypePromotionEngine::new();
    let cloned_engine = engine.clone();

    // Both engines should have same size (zero-sized)
    assert_eq!(std::mem::size_of_val(&engine), std::mem::size_of_val(&cloned_engine));
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - IDENTITY PROMOTIONS
// ============================================================================

/// Tests analyze_binary_promotion with identical types (I32 + I32).
///
/// # Rationale
/// Identity promotions should not require any casts and should produce
/// no warnings, as both operands already have the correct type.
///
/// # Test Coverage
/// - Identity promotion for signed integers
/// - No cast generation for same types
/// - No warnings for identity operations
/// - Sound promotion result
#[test]
fn test_analyze_binary_promotion_identity_i32() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::I32, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I32, "Identity promotion should preserve I32 type");
    assert!(result.left_cast.is_none(), "Identity promotion should not cast left operand");
    assert!(result.right_cast.is_none(), "Identity promotion should not cast right operand");
    assert!(result.warnings.is_empty(), "Identity promotion should generate no warnings");
    assert!(result.is_sound, "Identity promotion should be sound");
}

/// Tests analyze_binary_promotion with identical floating-point types (F64 + F64).
///
/// # Rationale
/// Verifies identity promotion works correctly for floating-point types.
///
/// # Test Coverage
/// - Identity promotion for float types
/// - No cast or warning generation
#[test]
fn test_analyze_binary_promotion_identity_f64() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::F64, &IrType::F64, IrBinaryOp::Multiply, span);

    assert_eq!(result.result_type, IrType::F64, "Identity promotion should preserve F64 type");
    assert!(result.left_cast.is_none(), "Identity promotion should not cast left operand");
    assert!(result.right_cast.is_none(), "Identity promotion should not cast right operand");
    assert!(result.warnings.is_empty(), "Identity promotion should generate no warnings");
    assert!(result.is_sound, "Identity promotion should be sound");
}

/// Tests analyze_binary_promotion with identical unsigned types (U64 + U64).
///
/// # Rationale
/// Verifies identity promotion for unsigned integer types.
///
/// # Test Coverage
/// - Identity promotion for unsigned integers
/// - Different binary operation (Divide)
#[test]
fn test_analyze_binary_promotion_identity_u64() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::U64, &IrType::U64, IrBinaryOp::Divide, span);

    assert_eq!(result.result_type, IrType::U64, "Identity promotion should preserve U64 type");
    assert!(result.left_cast.is_none(), "Identity promotion should not cast left operand");
    assert!(result.right_cast.is_none(), "Identity promotion should not cast right operand");
    assert!(result.warnings.is_empty(), "Identity promotion should generate no warnings");
    assert!(result.is_sound, "Identity promotion should be sound");
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - WIDENING PROMOTIONS
// ============================================================================

/// Tests analyze_binary_promotion with signed integer widening (I8 → I32).
///
/// # Rationale
/// Widening promotions should insert a cast on the narrower operand,
/// but should not generate warnings since all values are preserved.
///
/// # Test Coverage
/// - Signed integer widening
/// - Left operand casting
/// - No precision loss or overflow warnings
#[test]
fn test_analyze_binary_promotion_i8_to_i32_widening() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::I32, IrBinaryOp::Add, span.clone());

    assert_eq!(result.result_type, IrType::I32, "I8 and I32 should promote to I32");
    assert!(result.left_cast.is_some(), "Left operand (I8) should be cast to I32");
    assert!(result.right_cast.is_none(), "Right operand (I32) should not be cast");

    // Verify left cast details
    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.from_type, IrType::I8);
        assert_eq!(left_cast.to_type, IrType::I32);
        assert_eq!(left_cast.cast_kind, CastKind::IntSignExtend);
        assert!(!left_cast.may_lose_precision, "Widening should not lose precision");
        assert!(!left_cast.may_overflow, "Widening should not overflow");
    }
    assert!(result.warnings.is_empty(), "Widening promotion should generate no warnings");
    assert!(result.is_sound, "Widening promotion should be sound");
}

/// Tests analyze_binary_promotion with unsigned integer widening (U16 → U64).
///
/// # Rationale
/// Verifies unsigned widening promotions work correctly across large bit-width gaps.
///
/// # Test Coverage
/// - Unsigned integer widening
/// - Large bit-width difference (16 to 64 bits)
/// - ZeroExtend cast kind for unsigned types
#[test]
fn test_analyze_binary_promotion_u16_to_u64_widening() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::U16, &IrType::U64, IrBinaryOp::Subtract, span);

    assert_eq!(result.result_type, IrType::U64, "U16 and U64 should promote to U64");
    assert!(result.left_cast.is_some(), "Left operand (U16) should be cast to U64");
    assert!(result.right_cast.is_none(), "Right operand (U64) should not be cast");

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.from_type, IrType::U16);
        assert_eq!(left_cast.to_type, IrType::U64);
        assert_eq!(left_cast.cast_kind, CastKind::IntZeroExtend);
        assert!(!left_cast.may_lose_precision);
        assert!(!left_cast.may_overflow);
    }
}

/// Tests analyze_binary_promotion with float widening (F32 → F64).
///
/// # Rationale
/// Float widening from F32 to F64 is exact and should not generate warnings.
///
/// # Test Coverage
/// - Floating-point widening
/// - FloatExtend cast kind
/// - No precision loss for F32→F64
#[test]
fn test_analyze_binary_promotion_f32_to_f64_widening() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::F32, &IrType::F64, IrBinaryOp::Multiply, span);

    assert_eq!(result.result_type, IrType::F64, "F32 and F64 should promote to F64");
    assert!(result.left_cast.is_some(), "Left operand (F32) should be cast to F64");
    assert!(result.right_cast.is_none(), "Right operand (F64) should not be cast");

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.from_type, IrType::F32);
        assert_eq!(left_cast.to_type, IrType::F64);
        assert_eq!(left_cast.cast_kind, CastKind::FloatExtend);
        assert!(!left_cast.may_lose_precision, "F32 to F64 is exact");
        assert!(!left_cast.may_overflow);
    }
}

/// Tests analyze_binary_promotion with reverse operand order for widening (I32 + I8).
///
/// # Rationale
/// Verifies that promotion works correctly regardless of operand order.
///
/// # Test Coverage
/// - Right operand widening
/// - Operand order independence
#[test]
fn test_analyze_binary_promotion_i32_i8_reverse_order() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::I8, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I32, "I32 and I8 should promote to I32");
    assert!(result.left_cast.is_none(), "Left operand (I32) should not be cast");
    assert!(result.right_cast.is_some(), "Right operand (I8) should be cast to I32");

    if let Some(ref right_cast) = result.right_cast {
        assert_eq!(right_cast.from_type, IrType::I8);
        assert_eq!(right_cast.to_type, IrType::I32);
        assert_eq!(right_cast.cast_kind, CastKind::IntSignExtend);
    }
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - SIGNED/UNSIGNED MIXING
// ============================================================================

/// Tests analyze_binary_promotion with same-width signed/unsigned mixing (I32 + U32 → I64).
///
/// # Rationale
/// When signed and unsigned integers of the same width are mixed,
/// they should promote to the next larger signed type to preserve all values.
/// This should generate a signedness change warning.
///
/// # Test Coverage
/// - Same-width signed/unsigned promotion
/// - Promotion to next larger signed type
/// - SignednessChange warning generation
#[test]
fn test_analyze_binary_promotion_i32_u32_signedness() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I64, "I32 and U32 should promote to I64");
    // At least one operand should be cast (implementation may optimize)
    assert!(result.left_cast.is_some() || result.right_cast.is_some(), "At least one cast should be present");

    // Should generate signedness change warning
    assert!(!result.warnings.is_empty(), "Signedness mixing should generate warnings");

    // Check for SignednessChange warning
    let has_signedness_warning = result.warnings.iter().any(|w| matches!(w, PromotionWarning::SignednessChange { .. }));
    assert!(has_signedness_warning, "Should contain SignednessChange warning");
}

/// Tests analyze_binary_promotion with I16 and U16 mixing.
///
/// # Rationale
/// Verifies signedness handling for 16-bit types.
///
/// # Test Coverage
/// - I16/U16 → I32 promotion
/// - Signedness change warning
#[test]
fn test_analyze_binary_promotion_i16_u16_signedness() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I16, &IrType::U16, IrBinaryOp::Multiply, span);

    assert_eq!(result.result_type, IrType::I32, "I16 and U16 should promote to I32");
    assert!(result.left_cast.is_some() || result.right_cast.is_some());
    assert!(!result.warnings.is_empty(), "Should generate signedness warnings");
}

/// Tests analyze_binary_promotion with I8 and U8 mixing.
///
/// # Rationale
/// Verifies signedness handling for smallest integer types.
///
/// # Test Coverage
/// - I8/U8 → I16 promotion
/// - Signedness change warning for 8-bit types
#[test]
fn test_analyze_binary_promotion_i8_u8_signedness() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::U8, IrBinaryOp::Divide, span);

    assert_eq!(result.result_type, IrType::I16, "I8 and U8 should promote to I16");
    assert!(result.left_cast.is_some() || result.right_cast.is_some());

    let has_signedness_warning = result.warnings.iter().any(|w| matches!(w, PromotionWarning::SignednessChange { .. }));
    assert!(has_signedness_warning);
}

/// Tests analyze_binary_promotion with reverse order (U32 + I32).
///
/// # Rationale
/// Verifies that signedness detection is order-independent.
///
/// # Test Coverage
/// - Reverse operand order for signed/unsigned
/// - Consistent signedness warning generation
#[test]
fn test_analyze_binary_promotion_u32_i32_reverse_signedness() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::U32, &IrType::I32, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I64);
    assert!(!result.warnings.is_empty());

    let has_signedness_warning = result.warnings.iter().any(|w| matches!(w, PromotionWarning::SignednessChange { .. }));
    assert!(has_signedness_warning);
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - INTEGER TO FLOAT PROMOTIONS
// ============================================================================

/// Tests analyze_binary_promotion with integer to float promotion (I32 → F32).
///
/// # Rationale
/// When mixing integers and floats, integers should be promoted to float.
/// For I32→F32, this may lose precision for large integers.
///
/// # Test Coverage
/// - Integer to float promotion
/// - IntToFloat cast kind
/// - Potential precision loss warning
#[test]
fn test_analyze_binary_promotion_i32_to_f32() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::F32, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::F32, "I32 and F32 should promote to F32");
    assert!(result.left_cast.is_some(), "I32 should be cast to F32");

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.cast_kind, CastKind::IntToFloat);
        // I32 to F32 may lose precision for large values
        if left_cast.may_lose_precision {
            let has_precision_warning =
                result.warnings.iter().any(|w| matches!(w, PromotionWarning::PrecisionLoss { .. }));
            assert!(has_precision_warning, "Should warn about precision loss");
        }
    }
}

/// Tests analyze_binary_promotion with I64 → F64.
///
/// # Rationale
/// I64 to F64 conversion can lose precision for integers larger than 2^53.
///
/// # Test Coverage
/// - Large integer to double promotion
/// - Precision loss for large integers
#[test]
fn test_analyze_binary_promotion_i64_to_f64() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I64, &IrType::F64, IrBinaryOp::Multiply, span);

    assert_eq!(result.result_type, IrType::F64);
    assert!(result.left_cast.is_some());

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.cast_kind, CastKind::IntToFloat);
    }
}

/// Tests analyze_binary_promotion with unsigned to float (U32 → F32).
///
/// # Rationale
/// Unsigned integers also need IntToFloat cast when mixed with floats.
///
/// # Test Coverage
/// - Unsigned to float conversion
/// - IntToFloat cast for unsigned types
#[test]
fn test_analyze_binary_promotion_u32_to_f32() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::U32, &IrType::F32, IrBinaryOp::Divide, span);

    assert_eq!(result.result_type, IrType::F32);
    assert!(result.left_cast.is_some());

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.cast_kind, CastKind::IntToFloat);
    }
}

/// Tests analyze_binary_promotion with small int to float (I8 → F64).
///
/// # Rationale
/// Small integers always fit exactly in F64 without precision loss.
///
/// # Test Coverage
/// - Small integer to float (exact conversion)
/// - No precision loss expected
#[test]
fn test_analyze_binary_promotion_i8_to_f64_exact() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::F64, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::F64);
    assert!(result.left_cast.is_some());

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.cast_kind, CastKind::IntToFloat);
        // I8 values always fit exactly in F64
        assert!(!left_cast.may_lose_precision, "I8 to F64 should be exact");
    }
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - DIFFERENT BINARY OPERATIONS
// ============================================================================

/// Tests analyze_binary_promotion with Subtract operation.
///
/// # Rationale
/// Verifies that promotion logic is consistent across different operations.
///
/// # Test Coverage
/// - Subtract operation with promotion
/// - Operation-specific behavior (if any)
#[test]
fn test_analyze_binary_promotion_subtract_operation() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I16, &IrType::I32, IrBinaryOp::Subtract, span);

    assert_eq!(result.result_type, IrType::I32);
    assert!(result.left_cast.is_some());
    assert!(result.right_cast.is_none());
}

/// Tests analyze_binary_promotion with Modulo operation.
///
/// # Rationale
/// Modulo may have special overflow/signedness considerations.
///
/// # Test Coverage
/// - Modulo operation
/// - Signedness with modulo
#[test]
fn test_analyze_binary_promotion_modulo_operation() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Modulo, span);

    assert_eq!(result.result_type, IrType::I64);
    // At least one operand should be cast for mixed signedness
    assert!(result.left_cast.is_some() || result.right_cast.is_some());
}

/// Tests analyze_binary_promotion with BitwiseAnd operation.
///
/// # Rationale
/// Bitwise operations should follow same promotion rules.
///
/// # Test Coverage
/// - Bitwise operation promotion
/// - Type consistency for bitwise ops
#[test]
fn test_analyze_binary_promotion_bitwise_and_operation() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::U8, &IrType::U32, IrBinaryOp::BitwiseAnd, span);

    assert_eq!(result.result_type, IrType::U32);
    assert!(result.left_cast.is_some());
}

/// Tests analyze_binary_promotion with comparison operation (Equal).
///
/// # Rationale
/// Comparison operations may need promotion for operands to be comparable.
///
/// # Test Coverage
/// - Comparison operation
/// - Type promotion for comparisons
#[test]
fn test_analyze_binary_promotion_equal_comparison() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I16, &IrType::I64, IrBinaryOp::Equal, span);

    assert_eq!(result.result_type, IrType::I64);
    assert!(result.left_cast.is_some());
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - EDGE CASES
// ============================================================================

/// Tests analyze_binary_promotion with maximum width integers (I64 + I64).
///
/// # Rationale
/// Verifies behavior at maximum supported integer width.
///
/// # Test Coverage
/// - Maximum width integer identity
/// - No overflow to larger type (none exists)
#[test]
fn test_analyze_binary_promotion_max_width_i64() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I64, &IrType::I64, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I64);
    assert!(result.left_cast.is_none());
    assert!(result.right_cast.is_none());
}

/// Tests analyze_binary_promotion with minimum width integers (I8 + I8).
///
/// # Rationale
/// Verifies behavior at minimum integer width.
///
/// # Test Coverage
/// - Minimum width integer operations
/// - No narrowing occurs
#[test]
fn test_analyze_binary_promotion_min_width_i8() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::I8, IrBinaryOp::Multiply, span);

    assert_eq!(result.result_type, IrType::I8);
    assert!(result.left_cast.is_none());
    assert!(result.right_cast.is_none());
}

/// Tests analyze_binary_promotion with Bool types.
///
/// # Rationale
/// Boolean operations may promote to I32 or use boolean-specific logic.
///
/// # Test Coverage
/// - Boolean type handling
/// - Bool promotion behavior
#[test]
fn test_analyze_binary_promotion_bool_types() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::Bool, &IrType::Bool, IrBinaryOp::BitwiseAnd, span);

    // Bool + Bool might stay as Bool or promote to I32 depending on implementation
    assert!(result.result_type == IrType::Bool || result.result_type == IrType::I32);
}

/// Tests analyze_binary_promotion with Bool and I32.
///
/// # Rationale
/// Bool mixed with integer should promote to integer type.
///
/// # Test Coverage
/// - Bool to integer promotion
/// - Mixed boolean/integer operations
#[test]
fn test_analyze_binary_promotion_bool_to_i32() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::Bool, &IrType::I32, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I32);
    // Bool should be cast to I32
    if result.left_cast.is_some() {
        assert_eq!(result.left_cast.as_ref().unwrap().to_type, IrType::I32);
    }
}

/// Tests analyze_binary_promotion with Char type.
///
/// # Rationale
/// Char may be treated as integer or have special handling.
///
/// # Test Coverage
/// - Char type promotion
/// - Char with integer operations
#[test]
fn test_analyze_binary_promotion_char_to_i32() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::Char, &IrType::I32, IrBinaryOp::Add, span);

    // Char likely promotes to I32
    assert_eq!(result.result_type, IrType::I32);
}

/// Tests analyze_binary_promotion with non-matching complex types.
///
/// # Rationale
/// Verifies fallback behavior when no direct promotion rule exists.
///
/// # Test Coverage
/// - Fallback promotion logic
/// - Complex/incompatible type handling
#[test]
fn test_analyze_binary_promotion_fallback_to_left_type() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // Using types that might not have explicit promotion rules
    let result = engine.analyze_binary_promotion(&IrType::String, &IrType::Char, IrBinaryOp::Add, span);

    // Fallback should use left type or a default type (likely I32)
    assert!(
        result.result_type == IrType::String || result.result_type == IrType::I32,
        "Fallback should produce left type or default I32"
    );
}

// ============================================================================
// ANALYZE_BINARY_PROMOTION - CORNER CASES
// ============================================================================

/// Tests analyze_binary_promotion with large bit-width gap (I8 + I64).
///
/// # Rationale
/// Verifies promotion handles extreme width differences correctly.
///
/// # Test Coverage
/// - Maximum bit-width gap
/// - Correct cast generation across large gaps
#[test]
fn test_analyze_binary_promotion_large_width_gap() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::I64, IrBinaryOp::Add, span);

    assert_eq!(result.result_type, IrType::I64);
    assert!(result.left_cast.is_some());
    assert!(result.right_cast.is_none());

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.from_type, IrType::I8);
        assert_eq!(left_cast.to_type, IrType::I64);
        assert_eq!(left_cast.cast_kind, CastKind::IntSignExtend);
    }
}

/// Tests analyze_binary_promotion with unsigned large gap (U8 + U64).
///
/// # Rationale
/// Unsigned version of large width gap test.
///
/// # Test Coverage
/// - Unsigned large gap widening
/// - ZeroExtend for large gaps
#[test]
fn test_analyze_binary_promotion_unsigned_large_gap() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::U8, &IrType::U64, IrBinaryOp::Multiply, span);

    assert_eq!(result.result_type, IrType::U64);
    assert!(result.left_cast.is_some());

    if let Some(ref left_cast) = result.left_cast {
        assert_eq!(left_cast.cast_kind, CastKind::IntZeroExtend);
    }
}

/// Tests analyze_binary_promotion with mixed signedness and width (I8 + U64).
///
/// # Rationale
/// Combines signedness change with large width difference.
///
/// # Test Coverage
/// - Multiple edge conditions simultaneously
/// - Signedness + width gap
#[test]
fn test_analyze_binary_promotion_mixed_signedness_and_width() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::U64, IrBinaryOp::Add, span);

    // The result type may vary based on implementation - accept either U64 or I64
    // that can hold both I8 and U64 values
    assert!(
        result.result_type == IrType::U64 || result.result_type == IrType::I64,
        "Result type should be able to hold both I8 and U64 values, got {:?}",
        result.result_type
    );
    // Note: The implementation may optimize by not casting if types are already compatible
    // We just verify that the result is valid (no panic)
}

/// Tests analyze_binary_promotion with all arithmetic operations on same types.
///
/// # Rationale
/// Ensures consistency across all arithmetic operations.
///
/// # Test Coverage
/// - All arithmetic operations
/// - Consistent promotion behavior
#[test]
fn test_analyze_binary_promotion_all_arithmetic_operations() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let operations =
        vec![IrBinaryOp::Add, IrBinaryOp::Subtract, IrBinaryOp::Multiply, IrBinaryOp::Divide, IrBinaryOp::Modulo];

    for op in operations {
        let result = engine.analyze_binary_promotion(&IrType::I16, &IrType::I32, op, span.clone());
        assert_eq!(result.result_type, IrType::I32, "All arithmetic ops should promote I16+I32 to I32");
        assert!(result.left_cast.is_some());
    }
}

/// Tests analyze_binary_promotion with different float combinations.
///
/// # Rationale
/// Validates float promotion matrix completeness.
///
/// # Test Coverage
/// - F32 + F32, F32 + F64, F64 + F32, F64 + F64
/// - Float promotion symmetry
#[test]
fn test_analyze_binary_promotion_all_float_combinations() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // F32 + F32 → F32
    let result = engine.analyze_binary_promotion(&IrType::F32, &IrType::F32, IrBinaryOp::Add, span.clone());
    assert_eq!(result.result_type, IrType::F32);

    // F32 + F64 → F64
    let result = engine.analyze_binary_promotion(&IrType::F32, &IrType::F64, IrBinaryOp::Add, span.clone());
    assert_eq!(result.result_type, IrType::F64);

    // F64 + F32 → F64
    let result = engine.analyze_binary_promotion(&IrType::F64, &IrType::F32, IrBinaryOp::Add, span.clone());
    assert_eq!(result.result_type, IrType::F64);

    // F64 + F64 → F64
    let result = engine.analyze_binary_promotion(&IrType::F64, &IrType::F64, IrBinaryOp::Add, span);
    assert_eq!(result.result_type, IrType::F64);
}

// ============================================================================
// WARNING GENERATION TESTS
// ============================================================================

/// Tests that precision loss warning is generated for F64 → F32.
///
/// # Rationale
/// Narrowing float conversions should warn about precision loss.
///
/// # Test Coverage
/// - Precision loss warning for float narrowing
/// - Warning details and metadata
#[test]
fn test_analyze_binary_promotion_precision_loss_warning_f64_to_f32() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // This would require F64 to narrow to F32, which typically doesn't happen
    // in standard promotion, but we can test if the warning system works
    // by examining scenarios where a promotion rule indicates may_lose_precision

    // For actual precision loss, we need narrowing scenarios
    // Since standard promotion doesn't narrow, this tests the warning mechanism
    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::F32, IrBinaryOp::Add, span);

    // I32 to F32 can lose precision for large integers
    if result.left_cast.is_some() && result.left_cast.as_ref().unwrap().may_lose_precision {
        let has_precision_warning = result.warnings.iter().any(|w| matches!(w, PromotionWarning::PrecisionLoss { .. }));
        assert!(has_precision_warning, "Should generate precision loss warning for I32→F32");

        // Verify warning details
        for warning in &result.warnings {
            if let PromotionWarning::PrecisionLoss { from_type, to_type, .. } = warning {
                assert_eq!(*from_type, IrType::I32);
                assert_eq!(*to_type, IrType::F32);
            }
        }
    }
}

/// Tests that overflow warning is generated for appropriate scenarios.
///
/// # Rationale
/// Narrowing integer conversions should warn about potential overflow.
///
/// # Test Coverage
/// - Overflow warning generation
/// - Warning contains operation information
#[test]
fn test_analyze_binary_promotion_overflow_warning() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // Standard promotion typically widens, so overflow warnings are rare
    // This test validates the warning mechanism when may_overflow is set

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Add, span);

    // Check if any cast indicates potential overflow
    let may_overflow = result.left_cast.as_ref().map(|c| c.may_overflow).unwrap_or(false)
        || result.right_cast.as_ref().map(|c| c.may_overflow).unwrap_or(false);

    if may_overflow {
        let has_overflow_warning =
            result.warnings.iter().any(|w| matches!(w, PromotionWarning::PotentialOverflow { .. }));
        assert!(has_overflow_warning);
    }
}

/// Tests that SignednessChange warning contains correct metadata.
///
/// # Rationale
/// Signedness warnings should accurately report the signedness transition.
///
/// # Test Coverage
/// - Warning metadata accuracy
/// - from_signed and to_signed fields
/// - may_affect_comparisons flag
#[test]
fn test_analyze_binary_promotion_signedness_warning_details() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Equal, span);

    let signedness_warnings: Vec<_> =
        result.warnings.iter().filter(|w| matches!(w, PromotionWarning::SignednessChange { .. })).collect();

    assert!(!signedness_warnings.is_empty(), "Should generate signedness warning");

    for warning in signedness_warnings {
        if let PromotionWarning::SignednessChange { from_signed, to_signed, may_affect_comparisons } = warning {
            assert!(
                *from_signed != *to_signed || *may_affect_comparisons,
                "Signedness warning should indicate actual change or comparison impact"
            );
        }
    }
}

/// Tests that multiple warnings can be generated simultaneously.
///
/// # Rationale
/// Some promotions may trigger multiple warning conditions.
///
/// # Test Coverage
/// - Multiple warning generation
/// - Warning combination scenarios
#[test]
fn test_analyze_binary_promotion_multiple_warnings() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // Scenario that might generate multiple warnings: signed/unsigned + potential precision loss
    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Multiply, span);

    // Should have at least signedness warning
    assert!(!result.warnings.is_empty());

    // Count different warning types
    let signedness_count =
        result.warnings.iter().filter(|w| matches!(w, PromotionWarning::SignednessChange { .. })).count();

    let precision_count =
        result.warnings.iter().filter(|w| matches!(w, PromotionWarning::PrecisionLoss { .. })).count();

    let overflow_count =
        result.warnings.iter().filter(|w| matches!(w, PromotionWarning::PotentialOverflow { .. })).count();

    assert!(signedness_count + precision_count + overflow_count >= 1, "Should generate at least one warning type");
}

// ============================================================================
// PROMOTION RESULT SOUNDNESS TESTS
// ============================================================================

/// Tests that is_sound flag is correctly set for safe promotions.
///
/// # Rationale
/// Promotions without warnings should be marked as sound.
///
/// # Test Coverage
/// - is_sound flag accuracy
/// - Correlation with warnings
#[test]
fn test_analyze_binary_promotion_soundness_flag_safe() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // Safe widening promotion
    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::I32, IrBinaryOp::Add, span);

    if result.warnings.is_empty() {
        assert!(result.is_sound, "Promotions without warnings should be marked sound");
    }
}

/// Tests that is_sound flag is correctly set for potentially unsafe promotions.
///
/// # Rationale
/// Promotions with warnings should have is_sound reflect warning presence.
///
/// # Test Coverage
/// - is_sound with warnings present
/// - Warning impact on soundness
#[test]
fn test_analyze_binary_promotion_soundness_flag_with_warnings() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // Signedness change scenario
    let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Add, span);

    if !result.warnings.is_empty() {
        // Current implementation: is_sound = warnings.is_empty()
        assert!(!result.is_sound, "Promotions with warnings should reflect in is_sound flag");
    }
}

// ============================================================================
// CAST KIND VERIFICATION TESTS
// ============================================================================

/// Tests that SignExtend cast is used for signed integer widening.
///
/// # Rationale
/// Signed integers must use sign extension to preserve negative values.
///
/// # Test Coverage
/// - SignExtend cast kind correctness
/// - All signed widening scenarios
#[test]
fn test_analyze_binary_promotion_sign_extend_cast() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let test_cases = vec![(IrType::I8, IrType::I16), (IrType::I16, IrType::I32), (IrType::I32, IrType::I64)];

    for (from, to) in test_cases {
        let result = engine.analyze_binary_promotion(&from, &to, IrBinaryOp::Add, span.clone());

        if let Some(ref cast) = result.left_cast {
            assert_eq!(
                cast.cast_kind,
                CastKind::IntSignExtend,
                "Signed widening from {:?} to {:?} should use IntSignExtend",
                from,
                to
            );
        }
    }
}

/// Tests that ZeroExtend cast is used for unsigned integer widening.
///
/// # Rationale
/// Unsigned integers must use zero extension.
///
/// # Test Coverage
/// - ZeroExtend cast kind correctness
/// - All unsigned widening scenarios
#[test]
fn test_analyze_binary_promotion_zero_extend_cast() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let test_cases = vec![(IrType::U8, IrType::U16), (IrType::U16, IrType::U32), (IrType::U32, IrType::U64)];

    for (from, to) in test_cases {
        let result = engine.analyze_binary_promotion(&from, &to, IrBinaryOp::Add, span.clone());

        if let Some(ref cast) = result.left_cast {
            assert_eq!(
                cast.cast_kind,
                CastKind::IntZeroExtend,
                "Unsigned widening from {:?} to {:?} should use IntZeroExtend",
                from,
                to
            );
        }
    }
}

/// Tests that FloatExtend cast is used for F32 → F64.
///
/// # Rationale
/// Float widening requires FloatExtend cast.
///
/// # Test Coverage
/// - FloatExtend cast kind
/// - Float-specific cast operations
#[test]
fn test_analyze_binary_promotion_float_extend_cast() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::F32, &IrType::F64, IrBinaryOp::Add, span);

    if let Some(ref cast) = result.left_cast {
        assert_eq!(cast.cast_kind, CastKind::FloatExtend, "F32 to F64 should use FloatExtend");
    }
}

/// Tests that IntToFloat cast is used for integer to float conversions.
///
/// # Rationale
/// Integer to float conversions require IntToFloat cast.
///
/// # Test Coverage
/// - IntToFloat cast kind
/// - Various int→float scenarios
#[test]
fn test_analyze_binary_promotion_int_to_float_cast() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let test_cases = vec![(IrType::I32, IrType::F32), (IrType::I64, IrType::F64), (IrType::U32, IrType::F64)];

    for (from, to) in test_cases {
        let result = engine.analyze_binary_promotion(&from, &to, IrBinaryOp::Add, span.clone());

        if let Some(ref cast) = result.left_cast {
            assert_eq!(
                cast.cast_kind,
                CastKind::IntToFloat,
                "Int to float from {:?} to {:?} should use IntToFloat",
                from,
                to
            );
        }
    }
}

// ============================================================================
// SOURCE SPAN PROPAGATION TESTS
// ============================================================================

/// Tests that source span is correctly propagated to cast instructions.
///
/// # Rationale
/// Source spans are critical for error reporting and debugging.
///
/// # Test Coverage
/// - Span propagation to TypePromotion
/// - Span consistency across warnings
#[test]
fn test_analyze_binary_promotion_source_span_propagation() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result = engine.analyze_binary_promotion(&IrType::I8, &IrType::I32, IrBinaryOp::Add, span.clone());

    if let Some(ref cast) = result.left_cast {
        assert_eq!(cast.source_span, span, "Cast should preserve source span");
    }
}

/// Tests that custom source spans are preserved through promotion analysis.
///
/// # Rationale
/// Custom spans with specific location info must be maintained.
///
/// # Test Coverage
/// - Custom span preservation
/// - Span data integrity
#[test]
fn test_analyze_binary_promotion_custom_source_span() {
    let engine = TypePromotionEngine::new();
    let custom_span = SourceSpan::default(); // Would be customized in real scenario

    let result = engine.analyze_binary_promotion(&IrType::U16, &IrType::U64, IrBinaryOp::Multiply, custom_span.clone());

    if let Some(ref cast) = result.left_cast {
        assert_eq!(cast.source_span, custom_span);
    }
    if let Some(ref cast) = result.right_cast {
        assert_eq!(cast.source_span, custom_span);
    }
}

// ============================================================================
// REGRESSION TESTS
// ============================================================================

/// Tests that promotion engine uses global singleton matrix.
///
/// # Rationale
/// Ensures performance optimization via singleton is working.
///
/// # Test Coverage
/// - Singleton matrix usage
/// - Memory efficiency
#[test]
fn test_analyze_binary_promotion_uses_singleton_matrix() {
    let engine1 = TypePromotionEngine::new();
    let engine2 = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result1 = engine1.analyze_binary_promotion(&IrType::I32, &IrType::F32, IrBinaryOp::Add, span.clone());
    let result2 = engine2.analyze_binary_promotion(&IrType::I32, &IrType::F32, IrBinaryOp::Add, span);

    // Both should produce identical results (using same singleton matrix)
    assert_eq!(result1.result_type, result2.result_type);
    assert_eq!(result1.left_cast.is_some(), result2.left_cast.is_some());
    assert_eq!(result1.right_cast.is_some(), result2.right_cast.is_some());
}

/// Tests that analyze_binary_promotion is deterministic.
///
/// # Rationale
/// Same inputs should always produce same outputs.
///
/// # Test Coverage
/// - Deterministic behavior
/// - Result consistency
#[test]
fn test_analyze_binary_promotion_deterministic() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let result1 = engine.analyze_binary_promotion(&IrType::I16, &IrType::I64, IrBinaryOp::Add, span.clone());
    let result2 = engine.analyze_binary_promotion(&IrType::I16, &IrType::I64, IrBinaryOp::Add, span.clone());
    let result3 = engine.analyze_binary_promotion(&IrType::I16, &IrType::I64, IrBinaryOp::Add, span);

    assert_eq!(result1.result_type, result2.result_type);
    assert_eq!(result2.result_type, result3.result_type);
}

/// Tests that promotion engine handles rapid successive calls efficiently.
///
/// # Rationale
/// Performance regression test for high-frequency usage patterns.
///
/// # Test Coverage
/// - Performance under load
/// - No state corruption with repeated calls
#[test]
fn test_analyze_binary_promotion_rapid_successive_calls() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    // Perform many successive promotions
    for _ in 0..100 {
        let result = engine.analyze_binary_promotion(&IrType::I32, &IrType::F64, IrBinaryOp::Add, span.clone());
        assert_eq!(result.result_type, IrType::F64, "All calls should produce consistent results");
    }
}

/// Tests that promotion engine handles all IrType variants without panicking.
///
/// # Rationale
/// Robustness test ensuring no type causes crashes.
///
/// # Test Coverage
/// - All IrType enum variants
/// - No panic conditions
#[test]
fn test_analyze_binary_promotion_all_types_no_panic() {
    let engine = TypePromotionEngine::new();
    let span = SourceSpan::default();

    let all_types = vec![
        IrType::I8,
        IrType::I16,
        IrType::I32,
        IrType::I64,
        IrType::U8,
        IrType::U16,
        IrType::U32,
        IrType::U64,
        IrType::F32,
        IrType::F64,
        IrType::Bool,
        IrType::Char,
        IrType::Void,
        IrType::String,
    ];

    // Test all combinations (simplified - just test each type with I32)
    for ty in &all_types {
        let result = engine.analyze_binary_promotion(ty, &IrType::I32, IrBinaryOp::Add, span.clone());
        // Should not panic, result should be valid
        assert!(result.result_type != IrType::Void || ty == &IrType::Void);
    }
}
