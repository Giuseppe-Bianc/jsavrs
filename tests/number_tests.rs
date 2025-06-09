use jsavrs::tokens::number::Number;

#[test]
fn test_integer_display() {
    assert_eq!(Number::Integer(0).to_string(), "0");
    assert_eq!(Number::Integer(42).to_string(), "42");
    assert_eq!(Number::Integer(-42).to_string(), "-42");
    assert_eq!(Number::Integer(i64::MAX).to_string(), i64::MAX.to_string());
    assert_eq!(Number::Integer(i64::MIN).to_string(), i64::MIN.to_string());
}

#[test]
fn test_i8_display() {
    assert_eq!(Number::I8(0).to_string(), "0i8");
    assert_eq!(Number::I8(42).to_string(), "42i8");
    assert_eq!(Number::I8(-42).to_string(), "-42i8");
    assert_eq!(Number::I8(i8::MAX).to_string(), format!("{}i8", i8::MAX));
    assert_eq!(Number::I8(i8::MIN).to_string(), format!("{}i8", i8::MIN));
}

#[test]
fn test_i16_display() {
    assert_eq!(Number::I16(0).to_string(), "0i16");
    assert_eq!(Number::I16(1234).to_string(), "1234i16");
    assert_eq!(Number::I16(-1234).to_string(), "-1234i16");
    assert_eq!(
        Number::I16(i16::MAX).to_string(),
        format!("{}i16", i16::MAX)
    );
    assert_eq!(
        Number::I16(i16::MIN).to_string(),
        format!("{}i16", i16::MIN)
    );
}

#[test]
fn test_i32_display() {
    assert_eq!(Number::I32(0).to_string(), "0i32");
    assert_eq!(Number::I32(123456).to_string(), "123456i32");
    assert_eq!(Number::I32(-123456).to_string(), "-123456i32");
    assert_eq!(
        Number::I32(i32::MAX).to_string(),
        format!("{}i32", i32::MAX)
    );
    assert_eq!(
        Number::I32(i32::MIN).to_string(),
        format!("{}i32", i32::MIN)
    );
}

#[test]
fn test_unsigned_integer_display() {
    assert_eq!(Number::UnsignedInteger(0).to_string(), "0");
    assert_eq!(Number::UnsignedInteger(42).to_string(), "42");
    assert_eq!(
        Number::UnsignedInteger(u64::MAX).to_string(),
        u64::MAX.to_string()
    );
}

#[test]
fn test_u8_display() {
    assert_eq!(Number::U8(0).to_string(), "0u8");
    assert_eq!(Number::U8(42).to_string(), "42u8");
    assert_eq!(Number::U8(u8::MAX).to_string(), format!("{}u8", u8::MAX));
}

#[test]
fn test_u16_display() {
    assert_eq!(Number::U16(0).to_string(), "0u16");
    assert_eq!(Number::U16(1234).to_string(), "1234u16");
    assert_eq!(Number::U16(u16::MAX).to_string(), format!("{}u16", u16::MAX));
}

#[test]
fn test_u32_display() {
    assert_eq!(Number::U32(0).to_string(), "0u32");
    assert_eq!(Number::U32(123456).to_string(), "123456u32");
    assert_eq!(Number::U32(u32::MAX).to_string(), format!("{}u32", u32::MAX));
}


#[allow(clippy::approx_constant)]
#[test]
fn test_float32_display() {
    assert_eq!(Number::Float32(0.0).to_string(), "0");
    assert_eq!(Number::Float32(-0.0).to_string(), "-0");
    assert_eq!(Number::Float32(3.14).to_string(), "3.14");
    assert_eq!(Number::Float32(-2.71).to_string(), "-2.71");
    assert_eq!(Number::Float32(f32::INFINITY).to_string(), "inf");
    assert_eq!(Number::Float32(f32::NEG_INFINITY).to_string(), "-inf");
    assert!(Number::Float32(f32::NAN).to_string().contains("NaN")); // not equal to itself
}

#[allow(clippy::approx_constant)]
#[test]
fn test_float64_display() {
    assert_eq!(Number::Float64(0.0).to_string(), "0");
    assert_eq!(Number::Float64(-0.0).to_string(), "-0");
    assert_eq!(Number::Float64(3.1415926535).to_string(), "3.1415926535");
    assert_eq!(Number::Float64(-2.7182818284).to_string(), "-2.7182818284");
    assert_eq!(Number::Float64(f64::INFINITY).to_string(), "inf");
    assert_eq!(Number::Float64(f64::NEG_INFINITY).to_string(), "-inf");
    assert!(Number::Float64(f64::NAN).to_string().contains("NaN"));
}

#[test]
fn test_scientific32_display() {
    assert_eq!(Number::Scientific32(1.23, 3).to_string(), "1.23e3");
    assert_eq!(Number::Scientific32(4.56, -2).to_string(), "4.56e-2");
    assert_eq!(Number::Scientific32(0.0, 10).to_string(), "0e10");
    assert_eq!(Number::Scientific32(-0.0, -10).to_string(), "-0e-10");
}

#[test]
fn test_scientific64_display() {
    assert_eq!(
        Number::Scientific64(1.23456789, 5).to_string(),
        "1.23456789e5"
    );
    assert_eq!(Number::Scientific64(9.87, -3).to_string(), "9.87e-3");
    assert_eq!(Number::Scientific64(0.0, 0).to_string(), "0e0");
    assert_eq!(Number::Scientific64(-0.0, 0).to_string(), "-0e0");
}

// Edge cases: test extremes and possible formatting inconsistencies
#[test]
fn test_extreme_scientific_values() {
    let num = Number::Scientific64(f64::MAX, i32::MAX);
    assert_eq!(num.to_string(), format!("{}e{}", f64::MAX, i32::MAX));

    let num = Number::Scientific64(f64::MIN_POSITIVE, i32::MIN);
    assert_eq!(
        num.to_string(),
        format!("{}e{}", f64::MIN_POSITIVE, i32::MIN)
    );
}

#[test]
fn test_extreme_float_values() {
    assert_eq!(Number::Float64(f64::MAX).to_string(), f64::MAX.to_string());
    assert_eq!(
        Number::Float64(f64::MIN_POSITIVE).to_string(),
        f64::MIN_POSITIVE.to_string()
    );
    assert_eq!(
        Number::Float64(f64::EPSILON).to_string(),
        f64::EPSILON.to_string()
    );
}

#[test]
fn test_display_trait_consistency() {
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
    for number in numbers {
        assert_eq!(format!("{}", number), number.to_string());
    }
}
