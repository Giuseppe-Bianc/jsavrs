// Unit tests for SCCP ConstantEvaluator - Type-Safe Evaluation
// Tests overflow handling, floating-point semantics, and bitwise operations

use jsavrs::ir::optimizer::constant_folding::evaluator::{BinaryOp, BitwiseOp, ConstantEvaluator, UnaryOp};
use jsavrs::ir::optimizer::constant_folding::{ConstantValue, LatticeValue};

// ============================================================================
// T093: I8 Overflow Handling Tests
// ============================================================================

#[test]
fn test_i8_addition_overflow() {
    // i8::MAX + 1 should overflow to Top
    assert_eq!(
        ConstantEvaluator::eval_binary_i8(BinaryOp::Add, i8::MAX, 1),
        LatticeValue::Top,
        "I8::MAX + 1 should overflow"
    );

    // i8::MIN + (-1) should overflow
    assert_eq!(
        ConstantEvaluator::eval_binary_i8(BinaryOp::Add, i8::MIN, -1),
        LatticeValue::Top,
        "I8::MIN + (-1) should overflow"
    );

    // Normal addition should work
    assert_eq!(
        ConstantEvaluator::eval_binary_i8(BinaryOp::Add, 100, 20),
        LatticeValue::Constant(ConstantValue::I8(120))
    );
}

#[test]
fn test_i8_subtraction_overflow() {
    // i8::MIN - 1 should overflow
    assert_eq!(ConstantEvaluator::eval_binary_i8(BinaryOp::Sub, i8::MIN, 1), LatticeValue::Top);

    // Normal subtraction
    assert_eq!(ConstantEvaluator::eval_binary_i8(BinaryOp::Sub, 50, 20), LatticeValue::Constant(ConstantValue::I8(30)));
}

#[test]
fn test_i8_multiplication_overflow() {
    // i8::MAX * 2 should overflow
    assert_eq!(ConstantEvaluator::eval_binary_i8(BinaryOp::Mul, i8::MAX, 2), LatticeValue::Top);

    // Normal multiplication
    assert_eq!(ConstantEvaluator::eval_binary_i8(BinaryOp::Mul, 10, 5), LatticeValue::Constant(ConstantValue::I8(50)));
}

#[test]
fn test_i8_division_by_zero() {
    assert_eq!(
        ConstantEvaluator::eval_binary_i8(BinaryOp::Div, 100, 0),
        LatticeValue::Top,
        "Division by zero should return Top"
    );
}

#[test]
fn test_i8_negation_overflow() {
    // -i8::MIN should overflow
    assert_eq!(ConstantEvaluator::eval_unary_i8(UnaryOp::Neg, i8::MIN), LatticeValue::Top);

    // Normal negation
    assert_eq!(ConstantEvaluator::eval_unary_i8(UnaryOp::Neg, 42), LatticeValue::Constant(ConstantValue::I8(-42)));
}

// ============================================================================
// T094: I16 Overflow Handling Tests
// ============================================================================

#[test]
fn test_i16_addition_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_i16(BinaryOp::Add, i16::MAX, 1), LatticeValue::Top);

    assert_eq!(
        ConstantEvaluator::eval_binary_i16(BinaryOp::Add, 10000, 5000),
        LatticeValue::Constant(ConstantValue::I16(15000))
    );
}

#[test]
fn test_i16_subtraction_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_i16(BinaryOp::Sub, i16::MIN, 1), LatticeValue::Top);
}

#[test]
fn test_i16_multiplication_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_i16(BinaryOp::Mul, i16::MAX, 2), LatticeValue::Top);
}

#[test]
fn test_i16_negation_overflow() {
    assert_eq!(ConstantEvaluator::eval_unary_i16(UnaryOp::Neg, i16::MIN), LatticeValue::Top);
}

// ============================================================================
// T095: I64 Overflow Handling Tests
// ============================================================================

#[test]
fn test_i64_addition_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_i64(BinaryOp::Add, i64::MAX, 1), LatticeValue::Top);

    assert_eq!(
        ConstantEvaluator::eval_binary_i64(BinaryOp::Add, 1_000_000_000, 2_000_000_000),
        LatticeValue::Constant(ConstantValue::I64(3_000_000_000))
    );
}

#[test]
fn test_i64_subtraction_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_i64(BinaryOp::Sub, i64::MIN, 1), LatticeValue::Top);
}

#[test]
fn test_i64_multiplication_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_i64(BinaryOp::Mul, i64::MAX, 2), LatticeValue::Top);
}

#[test]
fn test_i64_negation_overflow() {
    assert_eq!(ConstantEvaluator::eval_unary_i64(UnaryOp::Neg, i64::MIN), LatticeValue::Top);
}

// ============================================================================
// T096: Unsigned Integer Overflow Handling Tests
// ============================================================================

#[test]
fn test_u8_overflow() {
    // u8::MAX + 1 should overflow
    assert_eq!(ConstantEvaluator::eval_binary_u8(BinaryOp::Add, u8::MAX, 1), LatticeValue::Top);

    // u8::MIN (0) - 1 should underflow
    assert_eq!(ConstantEvaluator::eval_binary_u8(BinaryOp::Sub, 0, 1), LatticeValue::Top);

    // Normal operation
    assert_eq!(
        ConstantEvaluator::eval_binary_u8(BinaryOp::Add, 100, 50),
        LatticeValue::Constant(ConstantValue::U8(150))
    );
}

#[test]
fn test_u16_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_u16(BinaryOp::Add, u16::MAX, 1), LatticeValue::Top);
    assert_eq!(ConstantEvaluator::eval_binary_u16(BinaryOp::Sub, 0, 1), LatticeValue::Top);
}

#[test]
fn test_u32_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_u32(BinaryOp::Add, u32::MAX, 1), LatticeValue::Top);
    assert_eq!(ConstantEvaluator::eval_binary_u32(BinaryOp::Sub, 0, 1), LatticeValue::Top);
}

#[test]
fn test_u64_overflow() {
    assert_eq!(ConstantEvaluator::eval_binary_u64(BinaryOp::Add, u64::MAX, 1), LatticeValue::Top);
    assert_eq!(ConstantEvaluator::eval_binary_u64(BinaryOp::Sub, 0, 1), LatticeValue::Top);
}

// ============================================================================
// T097: F32 NaN Propagation Tests
// ============================================================================

#[test]
fn test_f32_nan_propagation() {
    // NaN + anything = NaN
    let nan = f32::NAN;
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, nan, 1.0);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(val.is_nan(), "NaN + 1.0 should be NaN");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }

    // anything + NaN = NaN
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, 1.0, nan);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(val.is_nan(), "1.0 + NaN should be NaN");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }

    // NaN * anything = NaN
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Mul, nan, 2.0);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(val.is_nan(), "NaN * 2.0 should be NaN");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }
}

#[test]
fn test_f32_nan_check() {
    assert!(ConstantEvaluator::is_nan_f32(f32::NAN));
    assert!(!ConstantEvaluator::is_nan_f32(1.0));
    assert!(!ConstantEvaluator::is_nan_f32(f32::INFINITY));
}

// ============================================================================
// T098: F64 NaN Propagation Tests
// ============================================================================

#[test]
fn test_f64_nan_propagation() {
    let nan = f64::NAN;
    let result = ConstantEvaluator::eval_binary_f64(BinaryOp::Add, nan, 1.0);
    if let LatticeValue::Constant(ConstantValue::F64(val)) = result {
        assert!(val.is_nan(), "NaN + 1.0 should be NaN");
    } else {
        panic!("Expected Constant(F64), got {:?}", result);
    }
}

#[test]
fn test_f64_nan_check() {
    assert!(ConstantEvaluator::is_nan_f64(f64::NAN));
    assert!(!ConstantEvaluator::is_nan_f64(1.0));
}

// ============================================================================
// T099: Floating-Point Infinity Handling Tests
// ============================================================================

#[test]
fn test_f32_infinity_operations() {
    let inf = f32::INFINITY;
    let neg_inf = f32::NEG_INFINITY;

    // inf + 1 = inf
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, inf, 1.0);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(val.is_infinite() && val.is_sign_positive(), "inf + 1 should be inf");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }

    // inf * 2 = inf
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Mul, inf, 2.0);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(val.is_infinite() && val.is_sign_positive(), "inf * 2 should be inf");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }

    // inf + (-inf) = NaN
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, inf, neg_inf);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(val.is_nan(), "inf + (-inf) should be NaN");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }
}

#[test]
fn test_f64_infinity_operations() {
    let inf = f64::INFINITY;

    let result = ConstantEvaluator::eval_binary_f64(BinaryOp::Add, inf, 1.0);
    if let LatticeValue::Constant(ConstantValue::F64(val)) = result {
        assert!(val.is_infinite() && val.is_sign_positive());
    } else {
        panic!("Expected Constant(F64), got {:?}", result);
    }
}

#[test]
fn test_f32_infinity_check() {
    assert!(ConstantEvaluator::is_infinite_f32(f32::INFINITY));
    assert!(ConstantEvaluator::is_infinite_f32(f32::NEG_INFINITY));
    assert!(!ConstantEvaluator::is_infinite_f32(1.0));
}

#[test]
fn test_f64_infinity_check() {
    assert!(ConstantEvaluator::is_infinite_f64(f64::INFINITY));
    assert!(ConstantEvaluator::is_infinite_f64(f64::NEG_INFINITY));
    assert!(!ConstantEvaluator::is_infinite_f64(1.0));
}

// ============================================================================
// T100: Floating-Point Signed Zero Tests
// ============================================================================

#[test]
fn test_f32_signed_zero() {
    let pos_zero = 0.0f32;
    let neg_zero = -0.0f32;

    // Check detection
    assert!(!ConstantEvaluator::is_neg_zero_f32(pos_zero), "0.0 should not be negative zero");
    assert!(ConstantEvaluator::is_neg_zero_f32(neg_zero), "-0.0 should be negative zero");

    // Signed zeros in operations
    let result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, pos_zero, neg_zero);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert_eq!(val, 0.0);
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }
}

#[test]
fn test_f64_signed_zero() {
    let pos_zero = 0.0f64;
    let neg_zero = -0.0f64;

    assert!(!ConstantEvaluator::is_neg_zero_f64(pos_zero));
    assert!(ConstantEvaluator::is_neg_zero_f64(neg_zero));
}

#[test]
fn test_f32_negation_preserves_zero_sign() {
    let pos_zero = 0.0f32;
    let result = ConstantEvaluator::eval_unary_f32(UnaryOp::Neg, pos_zero);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = result {
        assert!(ConstantEvaluator::is_neg_zero_f32(val), "Negating +0.0 should give -0.0");
    } else {
        panic!("Expected Constant(F32), got {:?}", result);
    }
}

// ============================================================================
// T101: Char Unicode Validity Tests
// ============================================================================

#[test]
fn test_char_equality() {
    assert_eq!(ConstantEvaluator::eval_char_eq('A', 'A'), LatticeValue::Constant(ConstantValue::Bool(true)));
    assert_eq!(ConstantEvaluator::eval_char_eq('A', 'B'), LatticeValue::Constant(ConstantValue::Bool(false)));
}

#[test]
fn test_char_inequality() {
    assert_eq!(ConstantEvaluator::eval_char_ne('A', 'B'), LatticeValue::Constant(ConstantValue::Bool(true)));
    assert_eq!(ConstantEvaluator::eval_char_ne('X', 'X'), LatticeValue::Constant(ConstantValue::Bool(false)));
}

#[test]
fn test_char_unicode() {
    // Test with various Unicode characters
    assert_eq!(ConstantEvaluator::eval_char_eq('😀', '😀'), LatticeValue::Constant(ConstantValue::Bool(true)));
    assert_eq!(ConstantEvaluator::eval_char_eq('α', 'β'), LatticeValue::Constant(ConstantValue::Bool(false)));
}

// ============================================================================
// T102: Bitwise Operations Tests
// ============================================================================

#[test]
fn test_bitwise_and() {
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i32(BitwiseOp::And, 0b1100, 0b1010),
        LatticeValue::Constant(ConstantValue::I32(0b1000))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_u8(BitwiseOp::And, 255, 128),
        LatticeValue::Constant(ConstantValue::U8(128))
    );
}

#[test]
fn test_bitwise_or() {
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i32(BitwiseOp::Or, 0b1100, 0b1010),
        LatticeValue::Constant(ConstantValue::I32(0b1110))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_u16(BitwiseOp::Or, 0x00FF, 0xFF00),
        LatticeValue::Constant(ConstantValue::U16(0xFFFF))
    );
}

#[test]
fn test_bitwise_xor() {
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i32(BitwiseOp::Xor, 0b1100, 0b1010),
        LatticeValue::Constant(ConstantValue::I32(0b0110))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_u32(BitwiseOp::Xor, 0xAAAA, 0x5555),
        LatticeValue::Constant(ConstantValue::U32(0xFFFF))
    );
}

#[test]
fn test_bitwise_shifts() {
    // Left shift
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i32(BitwiseOp::Shl, 1, 4),
        LatticeValue::Constant(ConstantValue::I32(16))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_u8(BitwiseOp::Shl, 1, 7),
        LatticeValue::Constant(ConstantValue::U8(128))
    );

    // Right shift
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i32(BitwiseOp::Shr, 16, 4),
        LatticeValue::Constant(ConstantValue::I32(1))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_u64(BitwiseOp::Shr, 128, 3),
        LatticeValue::Constant(ConstantValue::U64(16))
    );
}

#[test]
fn test_bitwise_not() {
    assert_eq!(
        ConstantEvaluator::eval_bitwise_not_i8(0b00001111),
        LatticeValue::Constant(ConstantValue::I8(!0b00001111i8))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_not_u8(0b10101010),
        LatticeValue::Constant(ConstantValue::U8(0b01010101))
    );
    assert_eq!(ConstantEvaluator::eval_bitwise_not_i32(0), LatticeValue::Constant(ConstantValue::I32(-1)));
}

#[test]
fn test_bitwise_all_integer_types() {
    // Test bitwise operations work for all integer types
    assert_eq!(ConstantEvaluator::eval_bitwise_i8(BitwiseOp::And, 15, 7), LatticeValue::Constant(ConstantValue::I8(7)));
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i16(BitwiseOp::And, 255, 127),
        LatticeValue::Constant(ConstantValue::I16(127))
    );
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i64(BitwiseOp::And, 0xFFFF, 0xFF00),
        LatticeValue::Constant(ConstantValue::I64(0xFF00))
    );
}
