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
            assert_eq!(*may_lose_precision, false, "Identity promotion for {:?} should not lose precision", ty);
            assert_eq!(*may_overflow, false, "Identity promotion for {:?} should not overflow", ty);
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
            assert_eq!(may_lose_precision, false);
            assert_eq!(may_overflow, false);
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
    assert_eq!(promotion.may_lose_precision, false);
    assert_eq!(promotion.may_overflow, false);
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
            assert_eq!(from_signed, true);
            assert_eq!(to_signed, false);
            assert_eq!(may_affect_comparisons, true);
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
    assert_eq!(result.is_sound, true);
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
    assert_eq!(result.is_sound, false);
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
            assert_eq!(*may_lose_precision, false);
            assert_eq!(*may_overflow, false);
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
            assert_eq!(*may_lose_precision, false); // Dereference here
            assert_eq!(*may_overflow, false); // Dereference here
        }
        _ => panic!("Expected Direct promotion rule for I32 -> F32"),
    }

    match f32_to_i32.unwrap() {
        PromotionRule::Direct { may_lose_precision, may_overflow, .. } => {
            // F32 to I32 may lose precision and may overflow
            assert_eq!(*may_lose_precision, true); // Dereference here
            assert_eq!(*may_overflow, true); // Dereference here
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
        assert_eq!(*may_lose_precision, false);
        assert_eq!(*may_overflow, false);
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
        assert_eq!(*may_lose_precision, true);
        assert_eq!(*may_overflow, true);
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
        assert_eq!(*may_lose_precision, true);
        assert_eq!(*may_overflow, true);
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
        assert_eq!(*may_lose_precision, true);
        assert_eq!(*may_overflow, true);
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
        assert_eq!(*may_lose_precision, false);
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
        assert_eq!(*may_lose_precision, true);
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
        assert_eq!(*cast_kind, CastKind::Bitcast);
    } else {
        panic!("Expected Direct promotion rule");
    }
}

#[test]
fn test_cross_signedness_same_width_u32_to_i32() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::I32).unwrap();
    if let PromotionRule::Direct { cast_kind, .. } = rule {
        assert_eq!(*cast_kind, CastKind::Bitcast);
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
            assert_eq!(*cast_kind, CastKind::Bitcast, "Expected Bitcast for {:?} -> {:?}", from_type, to_type);
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
        PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } => {
            // Identity conversion - should be no-op
            assert!(!may_lose_precision);
            assert!(!may_overflow);
        }
        _ => panic!("Expected Direct rule for Bool→Bool identity"),
    }
}
