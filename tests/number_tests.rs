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
fn test_unsigned_integer_display() {
    assert_eq!(Number::UnsignedInteger(0).to_string(), "0");
    assert_eq!(Number::UnsignedInteger(42).to_string(), "42");
    assert_eq!(Number::UnsignedInteger(u64::MAX).to_string(), u64::MAX.to_string());
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
    assert_eq!(Number::Scientific64(1.23456789, 5).to_string(), "1.23456789e5");
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
    assert_eq!(num.to_string(), format!("{}e{}", f64::MIN_POSITIVE, i32::MIN));
}

#[test]
fn test_extreme_float_values() {
    assert_eq!(Number::Float64(f64::MAX).to_string(), f64::MAX.to_string());
    assert_eq!(Number::Float64(f64::MIN_POSITIVE).to_string(), f64::MIN_POSITIVE.to_string());
    assert_eq!(Number::Float64(f64::EPSILON).to_string(), f64::EPSILON.to_string());
}

#[test]
fn test_display_trait_consistency() {
    let numbers = vec![
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