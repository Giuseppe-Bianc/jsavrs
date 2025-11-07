// src/tokens/parsers/suffix.rs
//! Type suffix parsing and handling.
//!
//! This module provides functionality for splitting numeric literals into their
//! numeric component and optional type suffix, then routing to appropriate
//! type-specific parsers.

use crate::tokens::number::Number;
use super::numeric::{handle_default_suffix, handle_float_suffix, parse_integer};
/// Splits a numeric literal string into its numeric part and optional type suffix.
///
/// This function performs efficient suffix detection using byte-level pattern matching
/// with optimized fast paths for common cases.
///
/// # Supported Suffixes
///
/// - Single-character: `u`, `f`, `d`
/// - Two-character: `i8`, `u8`
/// - Three-character: `i16`, `i32`, `u16`, `u32`
///
/// # Arguments
///
/// * `slice` - Full numeric literal string including any suffix
///
/// # Returns
///
/// A tuple containing:
/// - Numeric portion (without suffix)
/// - Optional suffix (original case preserved, but matching is case-insensitive)
///
/// # Performance Notes
///
/// Uses byte-level operations and fast-path checking for optimal performance:
/// 1. First checks the last character for single-char suffixes
/// 2. Then checks for 3-char suffixes if string is long enough
/// 3. Finally checks for 2-char suffixes
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::parsers::suffix::split_numeric_and_suffix;
///
/// assert_eq!(split_numeric_and_suffix("42u"), ("42", Some("u")));
/// assert_eq!(split_numeric_and_suffix("3.14F"), ("3.14", Some("F")));
/// assert_eq!(split_numeric_and_suffix("100i16"), ("100", Some("i16")));
/// assert_eq!(split_numeric_and_suffix("6.022e23u32"), ("6.022e23", Some("u32")));
/// assert_eq!(split_numeric_and_suffix("100"), ("100", None));
/// ```
pub fn split_numeric_and_suffix(slice: &str) -> (&str, Option<&str>) {
    if slice.is_empty() {
        return (slice, None);
    }

    let bytes = slice.as_bytes();
    let len = bytes.len();

    // Fast path: check last character first for single-char suffixes
    let last_char = bytes[len - 1].to_ascii_lowercase();

    // Single-char suffixes: 'u' (unsigned), 'f' (float32), 'd' (float64)
    match last_char {
        b'u' | b'f' | b'd' => {
            return (&slice[..len - 1], Some(&slice[len - 1..]));
        }
        _ => {}
    }

    // Multi-char suffixes: check if we have at least 3 chars
    if len < 3 {
        return (slice, None);
    }

    // Check 3-char suffixes (i16, i32, u16, u32)
    if len >= 3 {
        let last_three = &bytes[len - 3..];
        let suffix_lower = [
            last_three[0].to_ascii_lowercase(),
            last_three[1].to_ascii_lowercase(),
            last_three[2].to_ascii_lowercase(),
        ];

        match suffix_lower {
            [b'i', b'1', b'6'] | [b'i', b'3', b'2'] | [b'u', b'1', b'6'] | [b'u', b'3', b'2'] => {
                return (&slice[..len - 3], Some(&slice[len - 3..]));
            }
            _ => {}
        }
    }

    // Check 2-char suffixes (i8, u8)
    if len >= 2 {
        let last_two = &bytes[len - 2..];
        let suffix_lower = [last_two[0].to_ascii_lowercase(), last_two[1].to_ascii_lowercase()];

        match suffix_lower {
            [b'i', b'8'] | [b'u', b'8'] => {
                return (&slice[..len - 2], Some(&slice[len - 2..]));
            }
            _ => {}
        }
    }

    (slice, None)
}

/// Routes numeric literal parsing based on type suffix.
///
/// This function dispatches to the appropriate parser based on the suffix,
/// implementing the language's type inference rules:
/// - No suffix: defaults to i64 for integers, f64 for floats
/// - 'u': unsigned 64-bit integer
/// - 'f': 32-bit float
/// - 'd': 64-bit float (explicit)
/// - Sized suffixes (i8, u32, etc.): specific type
///
/// # Arguments
///
/// * `numeric_part` - Numeric portion without suffix
/// * `suffix` - Optional type suffix (case-insensitive)
///
/// # Returns
///
/// * `Some(Number)` - Parsed number matching the suffix type
/// * `None` - Invalid format or unsupported suffix
///
/// # Type Resolution Table
///
/// | Suffix | Type | Example |
/// |--------|------|---------|
/// | None | i64/f64 | `42` → Integer(42), `3.14` → Float64(3.14) |
/// | `u` | u64 | `42u` → UnsignedInteger(42) |
/// | `i8` | i8 | `42i8` → I8(42) |
/// | `u16` | u16 | `1000u16` → U16(1000) |
/// | `f` | f32 | `3.14f` → Float32(3.14) |
/// | `d` | f64 | `3.14d` → Float64(3.14) |
/// # Examples
///
/// ```
/// use jsavrs::tokens::parsers::suffix::handle_suffix;
/// use jsavrs::tokens::number::Number;
///
/// // Parse with unsigned suffix
/// let result = handle_suffix("42", Some("u"));
/// assert!(matches!(result, Some(Number::UnsignedInteger(42))));
///
/// // Parse with no suffix (defaults to i64/f64)
/// let result = handle_suffix("42", None);
/// assert!(matches!(result, Some(Number::Integer(42))));
///
/// // Parse with no suffix (defaults to i64/f64)
/// let result = handle_suffix("42", None);
/// assert!(matches!(result, Some(Number::Integer(42))));
///
/// // Parse float with f32 suffix
/// let result = handle_suffix("3.14", Some("f"));
/// assert!(matches!(result, Some(Number::Float32(_))));
///
/// // Invalid suffix returns None
/// let result = handle_suffix("42", Some("invalid"));
/// assert_eq!(result, None);
/// ```
pub fn handle_suffix(numeric_part: &str, suffix: Option<&str>) -> Option<Number> {

    match suffix.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("u") => parse_integer::<u64>(numeric_part, Number::UnsignedInteger),
        Some("u8") => parse_integer::<u8>(numeric_part, Number::U8),
        Some("u16") => parse_integer::<u16>(numeric_part, Number::U16),
        Some("u32") => parse_integer::<u32>(numeric_part, Number::U32),
        Some("i8") => parse_integer::<i8>(numeric_part, Number::I8),
        Some("i16") => parse_integer::<i16>(numeric_part, Number::I16),
        Some("i32") => parse_integer::<i32>(numeric_part, Number::I32),
        Some("f") => handle_float_suffix(numeric_part),
        Some("d") | None => handle_default_suffix(numeric_part),
        _ => None, // Unknown suffix
    }
}
