use jsavrs::ir::type_promotion::TypePromotion;
use jsavrs::ir::{CastKind, IrType};
use jsavrs::location::source_span::SourceSpan;

#[test]
fn test_with_flags() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(
        IrType::I32,             // from_type
        IrType::I64,             // to_type
        CastKind::IntSignExtend, // cast_kind
        true,                    // may_lose_precision
        false,                   // may_overflow
        span.clone(),            // source_span
    );

    assert_eq!(promotion.from_type, IrType::I32);
    assert_eq!(promotion.to_type, IrType::I64);
    assert_eq!(promotion.cast_kind, CastKind::IntSignExtend);
    assert!(promotion.may_lose_precision);
    assert!(!promotion.may_overflow);
    assert_eq!(promotion.source_span, span);
}

#[test]
fn test_with_flags_all_false() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(
        IrType::F32,
        IrType::F64,
        CastKind::FloatExtend,
        false, // may_lose_precision
        false, // may_overflow
        span.clone(),
    );

    assert_eq!(promotion.from_type, IrType::F32);
    assert_eq!(promotion.to_type, IrType::F64);
    assert_eq!(promotion.cast_kind, CastKind::FloatExtend);
    assert!(!promotion.may_lose_precision);
    assert!(!promotion.may_overflow);
    assert_eq!(promotion.source_span, span);
}

#[test]
fn test_with_flags_all_true() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(
        IrType::I64,
        IrType::I32,
        CastKind::IntTruncate,
        true, // may_lose_precision
        true, // may_overflow
        span.clone(),
    );

    assert_eq!(promotion.from_type, IrType::I64);
    assert_eq!(promotion.to_type, IrType::I32);
    assert_eq!(promotion.cast_kind, CastKind::IntTruncate);
    assert!(promotion.may_lose_precision);
    assert!(promotion.may_overflow);
    assert_eq!(promotion.source_span, span);
}

#[test]
fn test_is_widening_int_zero_extend() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(IrType::U8, IrType::U32, CastKind::IntZeroExtend, false, false, span);

    assert!(promotion.is_widening());
}

#[test]
fn test_is_widening_int_sign_extend() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(IrType::I16, IrType::I64, CastKind::IntSignExtend, false, false, span);

    assert!(promotion.is_widening());
}

#[test]
fn test_is_widening_float_extend() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(IrType::F32, IrType::F64, CastKind::FloatExtend, false, false, span);

    assert!(promotion.is_widening());
}

#[test]
fn test_is_narrowing_int_truncate() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(IrType::I64, IrType::I16, CastKind::IntTruncate, false, false, span);

    assert!(promotion.is_narrowing());
}

#[test]
fn test_is_narrowing_float_truncate() {
    let span = SourceSpan::default();
    let promotion = TypePromotion::with_flags(IrType::F64, IrType::F32, CastKind::FloatTruncate, false, false, span);

    assert!(promotion.is_narrowing());
}

#[test]
fn test_not_widening_narrowing() {
    let span = SourceSpan::default();
    // Test different cast kinds that should not be considered as widening or narrowing
    let test_cases = vec![
        CastKind::IntToFloat,
        CastKind::FloatToInt,
        CastKind::Bitcast,
        CastKind::IntBitcast,
        CastKind::BoolToInt,
        CastKind::IntToBool,
        CastKind::BoolToFloat,
        CastKind::FloatToBool,
        CastKind::CharToInt,
        CastKind::IntToChar,
        CastKind::CharToString,
        CastKind::StringToChar,
        CastKind::StringToInt,
        CastKind::StringToFloat,
        CastKind::StringToBool,
        CastKind::IntToString,
        CastKind::FloatToString,
        CastKind::BoolToString,
    ];

    for cast_kind in test_cases {
        let promotion = TypePromotion::with_flags(IrType::I32, IrType::F32, cast_kind, false, false, span.clone());

        assert!(!promotion.is_widening(), "CastKind::{:?} should not be considered widening", cast_kind);
        assert!(!promotion.is_narrowing(), "CastKind::{:?} should not be considered narrowing", cast_kind);
    }
}

#[test]
fn test_widening_vs_narrowing_exclusivity() {
    let span = SourceSpan::default();
    // A promotion should never be both widening and narrowing
    let widening_cases = vec![CastKind::IntZeroExtend, CastKind::IntSignExtend, CastKind::FloatExtend];

    for cast_kind in widening_cases {
        let promotion = TypePromotion::with_flags(IrType::I32, IrType::I64, cast_kind, false, false, span.clone());

        assert!(promotion.is_widening());
        assert!(!promotion.is_narrowing());
    }

    let narrowing_cases = vec![CastKind::IntTruncate, CastKind::FloatTruncate];

    for cast_kind in narrowing_cases {
        let promotion = TypePromotion::with_flags(IrType::I64, IrType::I32, cast_kind, false, false, span.clone());

        assert!(promotion.is_narrowing());
        assert!(!promotion.is_widening());
    }
}
