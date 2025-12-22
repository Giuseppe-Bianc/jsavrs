//! Comprehensive test suite for the `Immediate` type in the `asm::instruction` module.
//!
//! This module tests all functionality related to immediate (constant) operands
//! including size calculations, type conversions, signedness checks, range validation,
//! and display formatting. Edge cases and boundary conditions are thoroughly covered.

use jsavrs::asm::Immediate;

#[test]
fn test_imm8_size_bits_returns_8() {
    let imm = Immediate::Imm8(0);
    assert_eq!(imm.size_bits(), 8);
}

#[test]
fn test_imm8u_size_bits_returns_8() {
    let imm = Immediate::Imm8u(0);
    assert_eq!(imm.size_bits(), 8);
}

#[test]
fn test_imm16_size_bits_returns_16() {
    let imm = Immediate::Imm16(0);
    assert_eq!(imm.size_bits(), 16);
}

#[test]
fn test_imm16u_size_bits_returns_16() {
    let imm = Immediate::Imm16u(0);
    assert_eq!(imm.size_bits(), 16);
}

#[test]
fn test_imm32_size_bits_returns_32() {
    let imm = Immediate::Imm32(0);
    assert_eq!(imm.size_bits(), 32);
}

#[test]
fn test_imm32u_size_bits_returns_32() {
    let imm = Immediate::Imm32u(0);
    assert_eq!(imm.size_bits(), 32);
}

#[test]
fn test_imm64_size_bits_returns_64() {
    let imm = Immediate::Imm64(0);
    assert_eq!(imm.size_bits(), 64);
}

#[test]
fn test_imm64u_size_bits_returns_64() {
    let imm = Immediate::Imm64u(0);
    assert_eq!(imm.size_bits(), 64);
}

#[test]
fn test_imm8_size_bytes_returns_1() {
    let imm = Immediate::Imm8(42);
    assert_eq!(imm.size_bytes(), 1);
}

#[test]
fn test_imm8u_size_bytes_returns_1() {
    let imm = Immediate::Imm8u(255);
    assert_eq!(imm.size_bytes(), 1);
}

#[test]
fn test_imm16_size_bytes_returns_2() {
    let imm = Immediate::Imm16(1000);
    assert_eq!(imm.size_bytes(), 2);
}

#[test]
fn test_imm16u_size_bytes_returns_2() {
    let imm = Immediate::Imm16u(65535);
    assert_eq!(imm.size_bytes(), 2);
}

#[test]
fn test_imm32_size_bytes_returns_4() {
    let imm = Immediate::Imm32(-1);
    assert_eq!(imm.size_bytes(), 4);
}

#[test]
fn test_imm32u_size_bytes_returns_4() {
    let imm = Immediate::Imm32u(u32::MAX);
    assert_eq!(imm.size_bytes(), 4);
}

#[test]
fn test_imm64_size_bytes_returns_8() {
    let imm = Immediate::Imm64(i64::MIN);
    assert_eq!(imm.size_bytes(), 8);
}

#[test]
fn test_imm64u_size_bytes_returns_8() {
    let imm = Immediate::Imm64u(u64::MAX);
    assert_eq!(imm.size_bytes(), 8);
}

#[test]
fn test_imm8_positive_to_i64() {
    let imm = Immediate::Imm8(127);
    assert_eq!(imm.as_i64(), 127);
}

#[test]
fn test_imm8_negative_to_i64() {
    let imm = Immediate::Imm8(-128);
    assert_eq!(imm.as_i64(), -128);
}

#[test]
fn test_imm8_zero_to_i64() {
    let imm = Immediate::Imm8(0);
    assert_eq!(imm.as_i64(), 0);
}

#[test]
fn test_imm8_minus_one_to_i64() {
    let imm = Immediate::Imm8(-1);
    assert_eq!(imm.as_i64(), -1);
}

#[test]
fn test_imm8u_max_to_i64() {
    let imm = Immediate::Imm8u(255);
    assert_eq!(imm.as_i64(), 255);
}

#[test]
fn test_imm8u_zero_to_i64() {
    let imm = Immediate::Imm8u(0);
    assert_eq!(imm.as_i64(), 0);
}

#[test]
fn test_imm16_positive_to_i64() {
    let imm = Immediate::Imm16(32767);
    assert_eq!(imm.as_i64(), 32767);
}

#[test]
fn test_imm16_negative_to_i64() {
    let imm = Immediate::Imm16(-32768);
    assert_eq!(imm.as_i64(), -32768);
}

#[test]
fn test_imm16u_max_to_i64() {
    let imm = Immediate::Imm16u(65535);
    assert_eq!(imm.as_i64(), 65535);
}

#[test]
fn test_imm32_positive_to_i64() {
    let imm = Immediate::Imm32(i32::MAX);
    assert_eq!(imm.as_i64(), i64::from(i32::MAX));
}

#[test]
fn test_imm32_negative_to_i64() {
    let imm = Immediate::Imm32(i32::MIN);
    assert_eq!(imm.as_i64(), i64::from(i32::MIN));
}

#[test]
fn test_imm32u_max_to_i64() {
    let imm = Immediate::Imm32u(u32::MAX);
    assert_eq!(imm.as_i64(), i64::from(u32::MAX));
}

#[test]
fn test_imm64_max_to_i64() {
    let imm = Immediate::Imm64(i64::MAX);
    assert_eq!(imm.as_i64(), i64::MAX);
}

#[test]
fn test_imm64_min_to_i64() {
    let imm = Immediate::Imm64(i64::MIN);
    assert_eq!(imm.as_i64(), i64::MIN);
}

#[test]
fn test_imm64u_large_value_to_i64() {
    // u64::MAX as i64 wraps to -1
    let imm = Immediate::Imm64u(u64::MAX);
    assert_eq!(imm.as_i64(), -1);
}

#[test]
fn test_imm64u_half_max_to_i64() {
    // Value that fits in i64 positive range
    let imm = Immediate::Imm64u(i64::MAX as u64);
    assert_eq!(imm.as_i64(), i64::MAX);
}

#[test]
fn test_imm64u_exceeds_i64_max_wraps() {
    // Value just above i64::MAX wraps to negative
    let val: u64 = (i64::MAX as u64) + 1;
    let imm = Immediate::Imm64u(val);
    assert_eq!(imm.as_i64(), i64::MIN);
}

#[test]
fn test_imm8_positive_to_u64() {
    let imm = Immediate::Imm8(127);
    assert_eq!(imm.as_u64(), 127);
}

#[test]
fn test_imm8_negative_to_u64_wraps() {
    // -1 as u64 is u64::MAX due to two's complement interpretation
    let imm = Immediate::Imm8(-1);
    assert_eq!(imm.as_u64(), u64::MAX);
}

#[test]
#[allow(clippy::cast_sign_loss)]
fn test_imm8_min_to_u64_wraps() {
    // -128 as u64 via i8 -> u64 cast
    let imm = Immediate::Imm8(-128);
    let expected = (-128_i8) as u64;
    assert_eq!(imm.as_u64(), expected);
}

#[test]
fn test_imm8u_max_to_u64() {
    let imm = Immediate::Imm8u(255);
    assert_eq!(imm.as_u64(), 255);
}

#[test]
fn test_imm8u_zero_to_u64() {
    let imm = Immediate::Imm8u(0);
    assert_eq!(imm.as_u64(), 0);
}

#[test]
fn test_imm16_positive_to_u64() {
    let imm = Immediate::Imm16(32767);
    assert_eq!(imm.as_u64(), 32767);
}

#[test]
fn test_imm16_negative_to_u64_wraps() {
    let imm = Immediate::Imm16(-1);
    assert_eq!(imm.as_u64(), u64::MAX);
}

#[test]
fn test_imm16u_max_to_u64() {
    let imm = Immediate::Imm16u(u16::MAX);
    assert_eq!(imm.as_u64(), u64::from(u16::MAX));
}

#[test]
fn test_imm32_positive_to_u64() {
    let imm = Immediate::Imm32(i32::MAX);
    assert_eq!(imm.as_u64(), i32::MAX as u64);
}

#[test]
fn test_imm32_negative_to_u64_wraps() {
    let imm = Immediate::Imm32(-1);
    assert_eq!(imm.as_u64(), u64::MAX);
}

#[test]
fn test_imm32u_max_to_u64() {
    let imm = Immediate::Imm32u(u32::MAX);
    assert_eq!(imm.as_u64(), u64::from(u32::MAX));
}

#[test]
fn test_imm64_positive_to_u64() {
    let imm = Immediate::Imm64(i64::MAX);
    assert_eq!(imm.as_u64(), i64::MAX as u64);
}

#[test]
fn test_imm64_negative_to_u64_wraps() {
    let imm = Immediate::Imm64(-1);
    assert_eq!(imm.as_u64(), u64::MAX);
}

#[test]
#[allow(clippy::cast_sign_loss)]
fn test_imm64_min_to_u64() {
    let imm = Immediate::Imm64(i64::MIN);
    assert_eq!(imm.as_u64(), i64::MIN as u64);
}

#[test]
fn test_imm64u_max_to_u64() {
    let imm = Immediate::Imm64u(u64::MAX);
    assert_eq!(imm.as_u64(), u64::MAX);
}

#[test]
fn test_imm64u_zero_to_u64() {
    let imm = Immediate::Imm64u(0);
    assert_eq!(imm.as_u64(), 0);
}

#[test]
fn test_imm8_is_signed() {
    let imm = Immediate::Imm8(42);
    assert!(imm.is_signed());
}

#[test]
fn test_imm8u_is_not_signed() {
    let imm = Immediate::Imm8u(42);
    assert!(!imm.is_signed());
}

#[test]
fn test_imm16_is_signed() {
    let imm = Immediate::Imm16(-1000);
    assert!(imm.is_signed());
}

#[test]
fn test_imm16u_is_not_signed() {
    let imm = Immediate::Imm16u(1000);
    assert!(!imm.is_signed());
}

#[test]
fn test_imm32_is_signed() {
    let imm = Immediate::Imm32(i32::MIN);
    assert!(imm.is_signed());
}

#[test]
fn test_imm32u_is_not_signed() {
    let imm = Immediate::Imm32u(u32::MAX);
    assert!(!imm.is_signed());
}

#[test]
fn test_imm64_is_signed() {
    let imm = Immediate::Imm64(0);
    assert!(imm.is_signed());
}

#[test]
fn test_imm64u_is_not_signed() {
    let imm = Immediate::Imm64u(0);
    assert!(!imm.is_signed());
}

// --- 8-bit fits_in tests ---

#[test]
fn test_imm8_zero_fits_in_8() {
    let imm = Immediate::Imm8(0);
    assert!(imm.fits_in(8));
}

#[test]
fn test_imm8_max_fits_in_8() {
    let imm = Immediate::Imm8(i8::MAX);
    assert!(imm.fits_in(8));
}

#[test]
fn test_imm8_min_fits_in_8() {
    let imm = Immediate::Imm8(i8::MIN);
    assert!(imm.fits_in(8));
}

#[test]
fn test_imm16_small_value_fits_in_8() {
    let imm = Immediate::Imm16(100);
    assert!(imm.fits_in(8));
}

#[test]
fn test_imm16_large_value_does_not_fit_in_8() {
    let imm = Immediate::Imm16(200);
    assert!(!imm.fits_in(8));
}

#[test]
fn test_imm16_negative_small_fits_in_8() {
    let imm = Immediate::Imm16(-100);
    assert!(imm.fits_in(8));
}

#[test]
fn test_imm16_negative_large_does_not_fit_in_8() {
    let imm = Immediate::Imm16(-200);
    assert!(!imm.fits_in(8));
}

#[test]
fn test_imm8u_fits_in_8_signed_range() {
    // u8 value 127 fits in i8 range
    let imm = Immediate::Imm8u(127);
    assert!(imm.fits_in(8));
}

#[test]
fn test_imm8u_exceeds_8_signed_range() {
    // u8 value 128 exceeds i8::MAX (127)
    let imm = Immediate::Imm8u(128);
    assert!(!imm.fits_in(8));
}

#[test]
fn test_imm8u_max_does_not_fit_in_8_signed() {
    let imm = Immediate::Imm8u(255);
    assert!(!imm.fits_in(8));
}

// --- 16-bit fits_in tests ---

#[test]
fn test_imm16_max_fits_in_16() {
    let imm = Immediate::Imm16(i16::MAX);
    assert!(imm.fits_in(16));
}

#[test]
fn test_imm16_min_fits_in_16() {
    let imm = Immediate::Imm16(i16::MIN);
    assert!(imm.fits_in(16));
}

#[test]
fn test_imm32_fits_in_16() {
    let imm = Immediate::Imm32(10000);
    assert!(imm.fits_in(16));
}

#[test]
fn test_imm32_exceeds_16() {
    let imm = Immediate::Imm32(40000);
    assert!(!imm.fits_in(16));
}

#[test]
fn test_imm32_negative_fits_in_16() {
    let imm = Immediate::Imm32(-30000);
    assert!(imm.fits_in(16));
}

#[test]
fn test_imm32_negative_exceeds_16() {
    let imm = Immediate::Imm32(-40000);
    assert!(!imm.fits_in(16));
}

// --- 32-bit fits_in tests ---

#[test]
fn test_imm32_max_fits_in_32() {
    let imm = Immediate::Imm32(i32::MAX);
    assert!(imm.fits_in(32));
}

#[test]
fn test_imm32_min_fits_in_32() {
    let imm = Immediate::Imm32(i32::MIN);
    assert!(imm.fits_in(32));
}

#[test]
fn test_imm64_fits_in_32() {
    let imm = Immediate::Imm64(1_000_000);
    assert!(imm.fits_in(32));
}

#[test]
fn test_imm64_exceeds_32() {
    let imm = Immediate::Imm64(i64::from(i32::MAX) + 1);
    assert!(!imm.fits_in(32));
}

#[test]
fn test_imm64_negative_exceeds_32() {
    let imm = Immediate::Imm64(i64::from(i32::MIN) - 1);
    assert!(!imm.fits_in(32));
}

// --- 64-bit fits_in tests ---

#[test]
fn test_any_value_fits_in_64() {
    let imm1 = Immediate::Imm64(i64::MAX);
    let imm2 = Immediate::Imm64(i64::MIN);
    let imm3 = Immediate::Imm8(0);
    let imm4 = Immediate::Imm64u(u64::MAX);

    assert!(imm1.fits_in(64));
    assert!(imm2.fits_in(64));
    assert!(imm3.fits_in(64));
    assert!(imm4.fits_in(64));
}

// --- Invalid bit width tests ---

#[test]
fn test_fits_in_zero_bits_returns_false() {
    let imm = Immediate::Imm8(0);
    assert!(!imm.fits_in(0));
}

#[test]
fn test_fits_in_4_bits_returns_false() {
    let imm = Immediate::Imm8(5);
    assert!(!imm.fits_in(4));
}

#[test]
fn test_fits_in_128_bits_returns_false() {
    let imm = Immediate::Imm64(100);
    assert!(!imm.fits_in(128));
}

#[test]
fn test_fits_in_1_bit_returns_false() {
    let imm = Immediate::Imm8(0);
    assert!(!imm.fits_in(1));
}

#[test]
fn test_from_i8() {
    let imm: Immediate = 42_i8.into();
    assert_eq!(imm, Immediate::Imm8(42));
}

#[test]
fn test_from_i8_negative() {
    let imm: Immediate = (-42_i8).into();
    assert_eq!(imm, Immediate::Imm8(-42));
}

#[test]
fn test_from_i8_min() {
    let imm: Immediate = i8::MIN.into();
    assert_eq!(imm, Immediate::Imm8(i8::MIN));
}

#[test]
fn test_from_i8_max() {
    let imm: Immediate = i8::MAX.into();
    assert_eq!(imm, Immediate::Imm8(i8::MAX));
}

#[test]
fn test_from_u8() {
    let imm: Immediate = 200_u8.into();
    assert_eq!(imm, Immediate::Imm8u(200));
}

#[test]
fn test_from_u8_zero() {
    let imm: Immediate = 0_u8.into();
    assert_eq!(imm, Immediate::Imm8u(0));
}

#[test]
fn test_from_u8_max() {
    let imm: Immediate = u8::MAX.into();
    assert_eq!(imm, Immediate::Imm8u(u8::MAX));
}

#[test]
fn test_from_i16() {
    let imm: Immediate = 1000_i16.into();
    assert_eq!(imm, Immediate::Imm16(1000));
}

#[test]
fn test_from_i16_negative() {
    let imm: Immediate = (-1000_i16).into();
    assert_eq!(imm, Immediate::Imm16(-1000));
}

#[test]
fn test_from_i16_min() {
    let imm: Immediate = i16::MIN.into();
    assert_eq!(imm, Immediate::Imm16(i16::MIN));
}

#[test]
fn test_from_i16_max() {
    let imm: Immediate = i16::MAX.into();
    assert_eq!(imm, Immediate::Imm16(i16::MAX));
}

#[test]
fn test_from_u16() {
    let imm: Immediate = 50000_u16.into();
    assert_eq!(imm, Immediate::Imm16u(50000));
}

#[test]
fn test_from_u16_max() {
    let imm: Immediate = u16::MAX.into();
    assert_eq!(imm, Immediate::Imm16u(u16::MAX));
}

#[test]
fn test_from_i32() {
    let imm: Immediate = 100_000_i32.into();
    assert_eq!(imm, Immediate::Imm32(100_000));
}

#[test]
fn test_from_i32_negative() {
    let imm: Immediate = (-100_000_i32).into();
    assert_eq!(imm, Immediate::Imm32(-100_000));
}

#[test]
fn test_from_i32_min() {
    let imm: Immediate = i32::MIN.into();
    assert_eq!(imm, Immediate::Imm32(i32::MIN));
}

#[test]
fn test_from_i32_max() {
    let imm: Immediate = i32::MAX.into();
    assert_eq!(imm, Immediate::Imm32(i32::MAX));
}

#[test]
fn test_from_u32() {
    let imm: Immediate = 3_000_000_000_u32.into();
    assert_eq!(imm, Immediate::Imm32u(3_000_000_000));
}

#[test]
fn test_from_u32_max() {
    let imm: Immediate = u32::MAX.into();
    assert_eq!(imm, Immediate::Imm32u(u32::MAX));
}

#[test]
fn test_from_i64() {
    let imm: Immediate = 10_000_000_000_i64.into();
    assert_eq!(imm, Immediate::Imm64(10_000_000_000));
}

#[test]
fn test_from_i64_negative() {
    let imm: Immediate = (-10_000_000_000_i64).into();
    assert_eq!(imm, Immediate::Imm64(-10_000_000_000));
}

#[test]
fn test_from_i64_min() {
    let imm: Immediate = i64::MIN.into();
    assert_eq!(imm, Immediate::Imm64(i64::MIN));
}

#[test]
fn test_from_i64_max() {
    let imm: Immediate = i64::MAX.into();
    assert_eq!(imm, Immediate::Imm64(i64::MAX));
}

#[test]
fn test_from_u64() {
    let imm: Immediate = 10_000_000_000_u64.into();
    assert_eq!(imm, Immediate::Imm64u(10_000_000_000));
}

#[test]
fn test_from_u64_max() {
    let imm: Immediate = u64::MAX.into();
    assert_eq!(imm, Immediate::Imm64u(u64::MAX));
}

#[test]
fn test_from_u64_zero() {
    let imm: Immediate = 0_u64.into();
    assert_eq!(imm, Immediate::Imm64u(0));
}

#[test]
fn test_imm8_positive_display() {
    let imm = Immediate::Imm8(42);
    assert_eq!(format!("{imm}"), "42");
}

#[test]
fn test_imm8_negative_display() {
    let imm = Immediate::Imm8(-42);
    assert_eq!(format!("{imm}"), "-42");
}

#[test]
fn test_imm8_zero_display() {
    let imm = Immediate::Imm8(0);
    assert_eq!(format!("{imm}"), "0");
}

#[test]
fn test_imm8u_display_hex_format() {
    let imm = Immediate::Imm8u(15);
    assert_eq!(format!("{imm}"), "0x0f");
}

#[test]
fn test_imm8u_max_display() {
    let imm = Immediate::Imm8u(255);
    assert_eq!(format!("{imm}"), "0xff");
}

#[test]
fn test_imm8u_zero_display() {
    let imm = Immediate::Imm8u(0);
    assert_eq!(format!("{imm}"), "0x00");
}

#[test]
fn test_imm16_positive_display() {
    let imm = Immediate::Imm16(1000);
    assert_eq!(format!("{imm}"), "1000");
}

#[test]
fn test_imm16_negative_display() {
    let imm = Immediate::Imm16(-1000);
    assert_eq!(format!("{imm}"), "-1000");
}

#[test]
fn test_imm16u_display_hex_format() {
    let imm = Immediate::Imm16u(4096);
    assert_eq!(format!("{imm}"), "0x1000");
}

#[test]
fn test_imm16u_max_display() {
    let imm = Immediate::Imm16u(65535);
    assert_eq!(format!("{imm}"), "0xffff");
}

#[test]
fn test_imm32_positive_display() {
    let imm = Immediate::Imm32(100_000);
    assert_eq!(format!("{imm}"), "100000");
}

#[test]
fn test_imm32_negative_display() {
    let imm = Immediate::Imm32(-100_000);
    assert_eq!(format!("{imm}"), "-100000");
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_imm32u_display_hex_format() {
    let imm = Immediate::Imm32u(0xDEADBEEF);
    assert_eq!(format!("{imm}"), "0xdeadbeef");
}

#[test]
fn test_imm32u_max_display() {
    let imm = Immediate::Imm32u(u32::MAX);
    assert_eq!(format!("{imm}"), "0xffffffff");
}

#[test]
fn test_imm64_positive_display() {
    let imm = Immediate::Imm64(10_000_000_000);
    assert_eq!(format!("{imm}"), "10000000000");
}

#[test]
fn test_imm64_negative_display() {
    let imm = Immediate::Imm64(-10_000_000_000);
    assert_eq!(format!("{imm}"), "-10000000000");
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_imm64u_display_hex_format() {
    let imm = Immediate::Imm64u(0x123456789ABCDEF0);
    assert_eq!(format!("{imm}"), "0x123456789abcdef0");
}

#[test]
fn test_imm64u_max_display() {
    let imm = Immediate::Imm64u(u64::MAX);
    assert_eq!(format!("{imm}"), "0xffffffffffffffff");
}

#[test]
fn test_imm64u_zero_display() {
    let imm = Immediate::Imm64u(0);
    assert_eq!(format!("{imm}"), "0x0000000000000000");
}

#[test]
fn test_immediate_clone() {
    let imm1 = Immediate::Imm32(12345);
    let imm2 = imm1.clone();
    assert_eq!(imm1, imm2);
}

#[test]
fn test_immediate_eq_same_variant_same_value() {
    let imm1 = Immediate::Imm16(-500);
    let imm2 = Immediate::Imm16(-500);
    assert_eq!(imm1, imm2);
}

#[test]
fn test_immediate_neq_same_variant_different_value() {
    let imm1 = Immediate::Imm16(-500);
    let imm2 = Immediate::Imm16(500);
    assert_ne!(imm1, imm2);
}

#[test]
fn test_immediate_neq_different_variant_same_logical_value() {
    // Same numeric value but different signedness
    let imm1 = Immediate::Imm8(100);
    let imm2 = Immediate::Imm8u(100);
    assert_ne!(imm1, imm2);
}

#[test]
fn test_immediate_neq_different_size_same_value() {
    let imm1 = Immediate::Imm8(10);
    let imm2 = Immediate::Imm16(10);
    assert_ne!(imm1, imm2);
}

#[test]
fn test_imm8_debug() {
    let imm = Immediate::Imm8(42);
    let debug_str = format!("{imm:?}");
    assert!(debug_str.contains("Imm8"));
    assert!(debug_str.contains("42"));
}

#[test]
fn test_imm8u_debug() {
    let imm = Immediate::Imm8u(200);
    let debug_str = format!("{imm:?}");
    assert!(debug_str.contains("Imm8u"));
    assert!(debug_str.contains("200"));
}

#[test]
fn test_imm64_debug() {
    let imm = Immediate::Imm64(-999);
    let debug_str = format!("{imm:?}");
    assert!(debug_str.contains("Imm64"));
    assert!(debug_str.contains("-999"));
}

#[test]
fn test_boundary_i8_max_as_i64() {
    let imm = Immediate::Imm8(i8::MAX);
    assert_eq!(imm.as_i64(), 127);
}

#[test]
fn test_boundary_i8_min_as_i64() {
    let imm = Immediate::Imm8(i8::MIN);
    assert_eq!(imm.as_i64(), -128);
}

#[test]
fn test_boundary_u8_max_as_u64() {
    let imm = Immediate::Imm8u(u8::MAX);
    assert_eq!(imm.as_u64(), 255);
}

#[test]
fn test_boundary_i16_max_fits_in_16() {
    let imm = Immediate::Imm16(i16::MAX);
    assert!(imm.fits_in(16));
    assert!(imm.fits_in(32));
    assert!(imm.fits_in(64));
}

#[test]
fn test_boundary_i16_min_fits_in_16() {
    let imm = Immediate::Imm16(i16::MIN);
    assert!(imm.fits_in(16));
    assert!(imm.fits_in(32));
    assert!(imm.fits_in(64));
}

#[test]
fn test_boundary_i16_max_does_not_fit_in_8() {
    let imm = Immediate::Imm16(i16::MAX);
    assert!(!imm.fits_in(8));
}

#[test]
fn test_boundary_i16_min_does_not_fit_in_8() {
    let imm = Immediate::Imm16(i16::MIN);
    assert!(!imm.fits_in(8));
}

#[test]
fn test_all_zeros_various_types() {
    assert_eq!(Immediate::Imm8(0).as_i64(), 0);
    assert_eq!(Immediate::Imm8u(0).as_i64(), 0);
    assert_eq!(Immediate::Imm16(0).as_i64(), 0);
    assert_eq!(Immediate::Imm16u(0).as_i64(), 0);
    assert_eq!(Immediate::Imm32(0).as_i64(), 0);
    assert_eq!(Immediate::Imm32u(0).as_i64(), 0);
    assert_eq!(Immediate::Imm64(0).as_i64(), 0);
    assert_eq!(Immediate::Imm64u(0).as_i64(), 0);
}

#[test]
fn test_minus_one_various_signed_types() {
    assert_eq!(Immediate::Imm8(-1).as_i64(), -1);
    assert_eq!(Immediate::Imm16(-1).as_i64(), -1);
    assert_eq!(Immediate::Imm32(-1).as_i64(), -1);
    assert_eq!(Immediate::Imm64(-1).as_i64(), -1);
}
