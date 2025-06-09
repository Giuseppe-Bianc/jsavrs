use insta::assert_snapshot;
use jsavrs::tokens::number::Number;

#[test]
fn test_integer_display_snapshot() {
    assert_snapshot!("integer_0", Number::Integer(0).to_string());
    assert_snapshot!("integer_42", Number::Integer(42).to_string());
    assert_snapshot!("integer_neg_42", Number::Integer(-42).to_string());
    assert_snapshot!("integer_max", Number::Integer(i64::MAX).to_string());
    assert_snapshot!("integer_min", Number::Integer(i64::MIN).to_string());
}

#[test]
fn test_i8_display() {
    assert_snapshot!("i8_0", Number::I8(0).to_string(), "0i8");
    assert_snapshot!("i8_42", Number::I8(42).to_string(), "42i8");
    assert_snapshot!("i8_neg_42", Number::I8(-42).to_string(), "-42i8");
    assert_snapshot!("i8_max", Number::I8(i8::MAX).to_string());
    assert_snapshot!("i8_min", Number::I8(i8::MIN).to_string());
}

#[test]
fn test_i16_display() {
    assert_snapshot!("i16_0", Number::I16(0).to_string());
    assert_snapshot!("i16_1234", Number::I16(1234).to_string());
    assert_snapshot!("i16_neg_1234", Number::I16(-1234).to_string());
    assert_snapshot!("i16_max", Number::I16(i16::MAX).to_string());
    assert_snapshot!("i16_min", Number::I16(i16::MIN).to_string());
}

#[test]
fn test_i32_display() {
    assert_snapshot!("i32_0", Number::I32(0).to_string());
    assert_snapshot!("i32_123456", Number::I32(123456).to_string());
    assert_snapshot!("i32_neg_123456", Number::I32(-123456).to_string());
    assert_snapshot!("i32_max", Number::I32(i32::MAX).to_string());
    assert_snapshot!("i32_min", Number::I32(i32::MIN).to_string());
}

#[test]
fn test_unsigned_integer_display() {
    assert_snapshot!("unsigned_0", Number::UnsignedInteger(0).to_string());
    assert_snapshot!("unsigned_42", Number::UnsignedInteger(42).to_string());
    assert_snapshot!(
        "unsigned_max",
        Number::UnsignedInteger(u64::MAX).to_string()
    );
}

#[test]
fn test_u8_display() {
    assert_snapshot!("u8_0", Number::U8(0).to_string());
    assert_snapshot!("u8_42", Number::U8(42).to_string());
    assert_snapshot!("u8_max", Number::U8(u8::MAX).to_string());
}

#[test]
fn test_u16_display() {
    assert_snapshot!("u16_0", Number::U16(0).to_string());
    assert_snapshot!("u16_1234", Number::U16(1234).to_string());
    assert_snapshot!("u16_max", Number::U16(u16::MAX).to_string());
}

#[test]
fn test_u32_display() {
    assert_snapshot!("u32_0", Number::U32(0).to_string());
    assert_snapshot!("u32_123456", Number::U32(123456).to_string());
    assert_snapshot!("u32_max", Number::U32(u32::MAX).to_string());
}

#[allow(clippy::approx_constant)]
#[test]
fn test_float32_display_snapshot() {
    assert_snapshot!("float32_0", Number::Float32(0.0).to_string());
    assert_snapshot!("float32_neg0", Number::Float32(-0.0).to_string());
    assert_snapshot!("float32_pi", Number::Float32(3.14).to_string());
    assert_snapshot!("float32_neg_e", Number::Float32(-2.71).to_string());
    assert_snapshot!("float32_inf", Number::Float32(f32::INFINITY).to_string());
    assert_snapshot!(
        "float32_neg_inf",
        Number::Float32(f32::NEG_INFINITY).to_string()
    );
    assert_snapshot!("float32_nan", Number::Float32(f32::NAN).to_string());
}

#[allow(clippy::approx_constant)]
#[test]
fn test_float64_display_snapshot() {
    assert_snapshot!("float64_0", Number::Float64(0.0).to_string());
    assert_snapshot!("float64_neg0", Number::Float64(-0.0).to_string());
    assert_snapshot!("float64_pi", Number::Float64(3.1415926535).to_string());
    assert_snapshot!("float64_neg_e", Number::Float64(-2.7182818284).to_string());
    assert_snapshot!("float64_inf", Number::Float64(f64::INFINITY).to_string());
    assert_snapshot!(
        "float64_neg_inf",
        Number::Float64(f64::NEG_INFINITY).to_string()
    );
    assert_snapshot!("float64_nan", Number::Float64(f64::NAN).to_string());
}

#[test]
fn test_scientific32_display_snapshot() {
    assert_snapshot!(
        "scientific32_1_23e3",
        Number::Scientific32(1.23, 3).to_string()
    );
    assert_snapshot!(
        "scientific32_4_56e-2",
        Number::Scientific32(4.56, -2).to_string()
    );
    assert_snapshot!(
        "scientific32_0e10",
        Number::Scientific32(0.0, 10).to_string()
    );
    assert_snapshot!(
        "scientific32_neg0e-10",
        Number::Scientific32(-0.0, -10).to_string()
    );
}

#[test]
fn test_scientific64_display_snapshot() {
    assert_snapshot!(
        "scientific64_1_23456789e5",
        Number::Scientific64(1.23456789, 5).to_string()
    );
    assert_snapshot!(
        "scientific64_9_87e-3",
        Number::Scientific64(9.87, -3).to_string()
    );
    assert_snapshot!("scientific64_0e0", Number::Scientific64(0.0, 0).to_string());
    assert_snapshot!(
        "scientific64_neg0e0",
        Number::Scientific64(-0.0, 0).to_string()
    );
}

#[test]
fn test_extreme_scientific_values_snapshot() {
    let num = Number::Scientific64(f64::MAX, i32::MAX);
    assert_snapshot!("scientific64_max", num.to_string());

    let num = Number::Scientific64(f64::MIN_POSITIVE, i32::MIN);
    assert_snapshot!("scientific64_min_pos", num.to_string());
}

#[test]
fn test_extreme_float_values_snapshot() {
    assert_snapshot!("float64_max", Number::Float64(f64::MAX).to_string());
    assert_snapshot!(
        "float64_min_pos",
        Number::Float64(f64::MIN_POSITIVE).to_string()
    );
    assert_snapshot!("float64_epsilon", Number::Float64(f64::EPSILON).to_string());
}

#[test]
fn test_display_trait_consistency_snapshot() {
    let numbers = vec![
        Number::I8(10),
        Number::I16(10),
        Number::I32(10),
        Number::U8(10),
        Number::U16(10),
        Number::U32(10),
        Number::Integer(10),
        Number::UnsignedInteger(10),
        Number::Float32(10.0),
        Number::Float64(10.0),
        Number::Scientific32(1.0, 1),
        Number::Scientific64(1.0, 1),
    ];
    for (i, number) in numbers.iter().enumerate() {
        assert_snapshot!(format!("display_consistency_{}", i), format!("{}", number));
    }
}
