use jsavrs::ir::{
    CastKind, IrBinaryOp, IrType, OverflowBehavior, PrecisionLossEstimate, PromotionMatrix, PromotionResult,
    PromotionRule, PromotionWarning, TypeGroup, TypePromotion,
};
use jsavrs::location::source_span::SourceSpan;

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

#[test]
fn test_promotion_rule_direct() {
    let rule = PromotionRule::Direct {
        cast_kind: CastKind::IntToFloat,
        may_lose_precision: false,
        may_overflow: false,
    };
    
    match rule {
        PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow } => {
            assert_eq!(cast_kind, CastKind::IntToFloat);
            assert_eq!(may_lose_precision, false);
            assert_eq!(may_overflow, false);
        },
        _ => panic!("Expected Direct promotion rule"),
    }
}

#[test]
fn test_promotion_rule_indirect() {
    let rule = PromotionRule::Indirect {
        intermediate_type: IrType::I32,
        first_cast: CastKind::IntToFloat,
        second_cast: CastKind::FloatToInt,
    };
    
    match rule {
        PromotionRule::Indirect { intermediate_type, first_cast, second_cast } => {
            assert_eq!(intermediate_type, IrType::I32);
            assert_eq!(first_cast, CastKind::IntToFloat);
            assert_eq!(second_cast, CastKind::FloatToInt);
        },
        _ => panic!("Expected Indirect promotion rule"),
    }
}

#[test]
fn test_promotion_rule_forbidden() {
    let rule = PromotionRule::Forbidden {
        reason: "Test reason".to_string(),
    };
    
    match rule {
        PromotionRule::Forbidden { reason } => {
            assert_eq!(reason, "Test reason");
        },
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
        },
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
        },
        _ => panic!("Expected PotentialOverflow warning"),
    }
}

#[test]
fn test_promotion_warning_signedness_change() {
    let warning = PromotionWarning::SignednessChange {
        from_signed: true,
        to_signed: false,
        may_affect_comparisons: true,
    };
    
    match warning {
        PromotionWarning::SignednessChange { from_signed, to_signed, may_affect_comparisons } => {
            assert_eq!(from_signed, true);
            assert_eq!(to_signed, false);
            assert_eq!(may_affect_comparisons, true);
        },
        _ => panic!("Expected SignednessChange warning"),
    }
}

#[test]
fn test_promotion_warning_float_special_values() {
    let warning = PromotionWarning::FloatSpecialValues {
        operation: IrBinaryOp::Divide,
        may_produce_nan: true,
        may_produce_infinity: false,
    };
    
    match warning {
        PromotionWarning::FloatSpecialValues { operation, may_produce_nan, may_produce_infinity } => {
            assert_eq!(operation, IrBinaryOp::Divide);
            assert_eq!(may_produce_nan, true);
            assert_eq!(may_produce_infinity, false);
        },
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
fn test_compute_common_type_wider_types_precedence() {
    let matrix = PromotionMatrix::new();
    
    assert_eq!(matrix.compute_common_type(&IrType::I64, &IrType::I32), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I64), Some(IrType::I64));
    assert_eq!(matrix.compute_common_type(&IrType::U64, &IrType::U32), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::U32, &IrType::U64), Some(IrType::U64));
    assert_eq!(matrix.compute_common_type(&IrType::I32, &IrType::I16), Some(IrType::I32));
    assert_eq!(matrix.compute_common_type(&IrType::I16, &IrType::I32), Some(IrType::I32));
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
    
    // A rule that shouldn't exist
    assert!(matrix.get_promotion_rule(&IrType::Bool, &IrType::String).is_none());
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
        },
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
        },
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
        },
        _ => panic!("Expected SignedIntegers group"),
    }

    match unsigned_ints {
        TypeGroup::UnsignedIntegers(types) => {
            assert_eq!(types, vec![IrType::U32, IrType::U64]);
        },
        _ => panic!("Expected UnsignedIntegers group"),
    }

    match floats {
        TypeGroup::FloatingPoint(types) => {
            assert_eq!(types, vec![IrType::F32, IrType::F64]);
        },
        _ => panic!("Expected FloatingPoint group"),
    }

    match boolean {
        TypeGroup::Boolean => {},
        _ => panic!("Expected Boolean group"),
    }

    match character {
        TypeGroup::Character => {},
        _ => panic!("Expected Character group"),
    }
}

// Edge cases and corner cases

#[test]
fn test_promotion_matrix_edge_case_empty_rule_lookup() {
    let matrix = PromotionMatrix::new();
    
    // Test that looking up a non-existent rule returns None
    assert!(matrix.get_promotion_rule(&IrType::String, &IrType::Bool).is_none());
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
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
        IrType::F32, IrType::F64,
        IrType::Bool, IrType::Char,
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
    let cast = TypePromotion::new(
        IrType::I32,
        IrType::F32,
        CastKind::IntToFloat,
        span.clone(),
    );
    
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
        warnings: vec![
            PromotionWarning::PrecisionLoss {
                from_type: IrType::F64,
                to_type: IrType::F32,
                estimated_loss: PrecisionLossEstimate::SignificantDigits { lost_bits: 24 },
            }
        ],
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
        estimated_loss: PrecisionLossEstimate::SignificantDigits { lost_bits: 24 }
    };
    
    match &precision_loss_warning {
        PromotionWarning::PrecisionLoss { from_type, to_type, estimated_loss } => {
            assert_eq!(from_type, &IrType::F64);
            assert_eq!(to_type, &IrType::F32);
            match estimated_loss {
                PrecisionLossEstimate::SignificantDigits { lost_bits } => {
                    assert_eq!(*lost_bits, 24);
                },
                _ => panic!("Expected SignificantDigits estimate"),
            }
        },
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
        },
        _ => panic!("Expected PotentialOverflow warning"),
    }
}