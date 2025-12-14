use jsavrs::tokens::number::Number;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Helper function to calculate hash of a number
fn hash_number(number: &Number) -> u64 {
    let mut hasher = DefaultHasher::new();
    number.hash(&mut hasher);
    hasher.finish()
}

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
    assert_eq!(Number::I16(i16::MAX).to_string(), format!("{}i16", i16::MAX));
    assert_eq!(Number::I16(i16::MIN).to_string(), format!("{}i16", i16::MIN));
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_i32_display() {
    assert_eq!(Number::I32(0).to_string(), "0i32");
    assert_eq!(Number::I32(123456).to_string(), "123456i32");
    assert_eq!(Number::I32(-123456).to_string(), "-123456i32");
    assert_eq!(Number::I32(i32::MAX).to_string(), format!("{}i32", i32::MAX));
    assert_eq!(Number::I32(i32::MIN).to_string(), format!("{}i32", i32::MIN));
}

#[test]
fn test_unsigned_integer_display() {
    assert_eq!(Number::UnsignedInteger(0).to_string(), "0");
    assert_eq!(Number::UnsignedInteger(42).to_string(), "42");
    assert_eq!(Number::UnsignedInteger(u64::MAX).to_string(), u64::MAX.to_string());
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
#[allow(clippy::unreadable_literal)]
fn test_u32_display() {
    assert_eq!(Number::U32(0).to_string(), "0u32");
    assert_eq!(Number::U32(123456).to_string(), "123456u32");
    assert_eq!(Number::U32(u32::MAX).to_string(), format!("{}u32", u32::MAX));
}

#[test]
#[allow(clippy::approx_constant)]
fn test_float32_display() {
    assert_eq!(Number::Float32(0.0).to_string(), "0");
    assert_eq!(Number::Float32(-0.0).to_string(), "-0");
    assert_eq!(Number::Float32(3.14).to_string(), "3.14");
    assert_eq!(Number::Float32(-2.71).to_string(), "-2.71");
    assert_eq!(Number::Float32(f32::INFINITY).to_string(), "inf");
    assert_eq!(Number::Float32(f32::NEG_INFINITY).to_string(), "-inf");
    assert!(Number::Float32(f32::NAN).to_string().contains("NaN")); // not equal to itself
}

#[test]
#[allow(clippy::approx_constant, clippy::unreadable_literal)]
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
#[allow(clippy::unreadable_literal)]
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
        assert_eq!(format!("{number}"), number.to_string());
    }
}

// PartialEq and Hash tests
#[test]
#[allow(clippy::approx_constant, clippy::unreadable_literal)]
fn test_partial_eq_same_variants() {
    // Test equality for same variants with same values
    assert_eq!(Number::I8(42), Number::I8(42));
    assert_eq!(Number::I16(1000), Number::I16(1000));
    assert_eq!(Number::I32(50000), Number::I32(50000));
    assert_eq!(Number::Integer(123456789), Number::Integer(123456789));
    assert_eq!(Number::U8(200), Number::U8(200));
    assert_eq!(Number::U16(40000), Number::U16(40000));
    assert_eq!(Number::U32(3000000000), Number::U32(3000000000));
    assert_eq!(Number::UnsignedInteger(123456789012345), Number::UnsignedInteger(123456789012345));
    assert_eq!(Number::Float32(3.14), Number::Float32(3.14));
    assert_eq!(Number::Float64(2.71828), Number::Float64(2.71828));
    assert_eq!(Number::Scientific32(1.5, 10), Number::Scientific32(1.5, 10));
    assert_eq!(Number::Scientific64(2.0, 20), Number::Scientific64(2.0, 20));
}

#[test]
#[allow(clippy::approx_constant)]
fn test_partial_eq_different_variants() {
    // Test inequality between different variants
    assert_ne!(Number::I8(42), Number::I16(42));
    assert_ne!(Number::I32(100), Number::Integer(100));
    assert_ne!(Number::U8(200), Number::U16(200));
    assert_ne!(Number::U32(1000), Number::UnsignedInteger(1000));
    assert_ne!(Number::Float32(3.14), Number::Float64(3.14));
    assert_ne!(Number::Scientific32(1.5, 10), Number::Scientific64(1.5, 10));
}

#[test]
#[allow(clippy::approx_constant, clippy::unreadable_literal)]
fn test_partial_eq_same_variant_different_values() {
    // Test inequality for same variants with different values
    assert_ne!(Number::I8(42), Number::I8(24));
    assert_ne!(Number::I16(1000), Number::I16(2000));
    assert_ne!(Number::I32(50000), Number::I32(60000));
    assert_ne!(Number::Integer(123456789), Number::Integer(987654321));
    assert_ne!(Number::U8(200), Number::U8(100));
    assert_ne!(Number::U16(40000), Number::U16(30000));
    assert_ne!(Number::U32(3000000000), Number::U32(2000000000));
    assert_ne!(Number::UnsignedInteger(123456789012345), Number::UnsignedInteger(987654321098765));
    assert_ne!(Number::Float32(3.14), Number::Float32(3.14159));
    assert_ne!(Number::Float64(2.71828), Number::Float64(2.718));
    assert_ne!(Number::Scientific32(1.5, 10), Number::Scientific32(1.5, 20));
    assert_ne!(Number::Scientific64(2.0, 20), Number::Scientific64(3.0, 20));
}

#[test]
#[allow(clippy::cast_lossless)]
fn test_partial_eq_float_bitwise() {
    // Test that floating-point equality is bitwise
    // These two values are equal numerically but have different bit representations
    let neg_zero = -0.0;
    let pos_zero = 0.0;
    assert_ne!(Number::Float32(neg_zero), Number::Float32(pos_zero));
    assert_ne!(Number::Float64(neg_zero as f64), Number::Float64(pos_zero as f64));

    // NaN values are not equal to each other
    // Create two NaN values with different bit patterns
    let nan1 = f32::NAN;
    let nan2 = f32::from_bits(f32::NAN.to_bits() ^ 1); // Flip one bit in the fraction
    assert_ne!(Number::Float32(nan1), Number::Float32(nan2));

    // Also test f64 NaNs
    let nan3 = f64::NAN;
    let nan4 = f64::from_bits(f64::NAN.to_bits() ^ 1); // Flip one bit in the fraction
    assert_ne!(Number::Float64(nan3), Number::Float64(nan4));
}

#[test]
#[allow(clippy::approx_constant, clippy::unreadable_literal)]
fn test_hash_consistency() {
    // Test that equal values produce the same hash
    let numbers = [
        Number::I8(42),
        Number::I16(1000),
        Number::I32(50000),
        Number::Integer(123456789),
        Number::U8(200),
        Number::U16(40000),
        Number::U32(3000000000),
        Number::UnsignedInteger(123456789012345),
        Number::Float32(3.14),
        Number::Float64(2.71828),
        Number::Scientific32(1.5, 10),
        Number::Scientific64(2.0, 20),
    ];

    for num in numbers {
        assert_eq!(hash_number(&num), hash_number(&num));
    }
}

#[test]
#[allow(clippy::approx_constant)]
fn test_hash_different_values() {
    // Test that different values produce different hashes
    let num1 = Number::I32(100);
    let num2 = Number::I32(200);
    assert_ne!(hash_number(&num1), hash_number(&num2));

    let num3 = Number::Float64(3.14);
    let num4 = Number::Float64(3.14159);
    assert_ne!(hash_number(&num3), hash_number(&num4));

    let num5 = Number::Scientific32(1.5, 10);
    let num6 = Number::Scientific32(1.5, 20);
    assert_ne!(hash_number(&num5), hash_number(&num6));
}

#[test]
#[allow(clippy::approx_constant)]
fn test_hash_different_variants() {
    // Test that different variants produce different hashes even with same numeric value
    let num1 = Number::I32(100);
    let num2 = Number::Integer(100);
    assert_ne!(hash_number(&num1), hash_number(&num2));

    let num3 = Number::U32(100);
    let num4 = Number::UnsignedInteger(100);
    assert_ne!(hash_number(&num3), hash_number(&num4));

    let num5 = Number::Float32(3.14);
    let num6 = Number::Float64(3.14);
    assert_ne!(hash_number(&num5), hash_number(&num6));

    let num7 = Number::Scientific32(1.5, 10);
    let num8 = Number::Scientific64(1.5, 10);
    assert_ne!(hash_number(&num7), hash_number(&num8));
}

#[test]
fn test_hash_float_bitwise() {
    // Test that floating-point hashes are based on bit representation
    let neg_zero = Number::Float64(-0.0);
    let pos_zero = Number::Float64(0.0);
    assert_ne!(hash_number(&neg_zero), hash_number(&pos_zero));

    // Create two NaN values with different bit patterns
    let nan1 = Number::Float32(f32::NAN);
    let nan2 = Number::Float32(f32::from_bits(f32::NAN.to_bits() ^ 1)); // Flip one bit in the fraction
    assert_ne!(hash_number(&nan1), hash_number(&nan2));

    // Similarly for f64
    let nan3 = Number::Float64(f64::NAN);
    let nan4 = Number::Float64(f64::from_bits(f64::NAN.to_bits() ^ 1)); // Flip one bit in the fraction
    assert_ne!(hash_number(&nan3), hash_number(&nan4));
}
