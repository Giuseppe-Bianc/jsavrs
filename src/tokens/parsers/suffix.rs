// src/tokens/parsers/suffix.rs
//! Type suffix parsing and handling.
//!
//! This module provides functionality for splitting numeric literals into their
//! numeric component and optional type suffix, then routing to appropriate
//! type-specific parsers.

use super::numeric::{handle_default_suffix, handle_float_suffix, parse_integer};
use crate::tokens::number::Number;

/// Represents the possible suffix types for numeric literals.
///
/// This enum provides a type-safe way to handle different suffix patterns,
/// making the code more maintainable and testable.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixPattern {
    /// Single-character suffixes: 'u', 'f', 'd'
    SingleChar = 0, // Most frequent case
    /// Two-character suffixes: 'i8', 'u8'
    TwoChar = 1,
    /// Three-character suffixes: 'i16', 'i32', 'u16', 'u32'
    ThreeChar = 2,
}

impl SuffixPattern {
    /// Returns the length in bytes for this suffix pattern.
    #[inline]
    const fn len(self) -> usize {
        match self {
            SuffixPattern::SingleChar => 1,
            SuffixPattern::TwoChar => 2,
            SuffixPattern::ThreeChar => 3,
        }
    }
}

/// Checks if the last character of a byte slice is a single-character suffix.
///
/// Single-character suffixes are: 'u' (unsigned), 'f' (float32), 'd' (float64).
///
/// # Arguments
///
/// * `bytes` - The byte slice to check
///
/// # Returns
///
/// `Some(SuffixPattern::SingleChar)` if the last byte is a valid single-char suffix,
/// `None` otherwise.
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::parsers::suffix::check_single_char_suffix;
/// assert!(check_single_char_suffix(b"42u").is_some());
/// assert!(check_single_char_suffix(b"3.14f").is_some());
/// assert!(check_single_char_suffix(b"100d").is_some());
/// assert!(check_single_char_suffix(b"42x").is_none());
/// ```
#[inline]
pub fn check_single_char_suffix(bytes: &[u8]) -> Option<SuffixPattern> {
    if bytes.is_empty() {
        return None;
    }

    let last_char = bytes[bytes.len() - 1].to_ascii_lowercase();
    match last_char {
        b'u' | b'f' | b'd' => Some(SuffixPattern::SingleChar),
        _ => None,
    }
}

/// Checks if the last three characters form a valid three-character suffix.
///
/// Valid three-character suffixes are: 'i16', 'i32', 'u16', 'u32' (case-insensitive).
///
/// # Arguments
///
/// * `bytes` - The byte slice to check (must have length >= 3)
///
/// # Returns
///
/// `Some(SuffixPattern::ThreeChar)` if the last three bytes form a valid suffix,
/// `None` otherwise.
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::parsers::suffix::check_three_char_suffix;
/// assert!(check_three_char_suffix(b"100i16").is_some());
/// assert!(check_three_char_suffix(b"42u32").is_some());
/// assert!(check_three_char_suffix(b"100abc").is_none());
/// ```
#[inline]
pub fn check_three_char_suffix(bytes: &[u8]) -> Option<SuffixPattern> {
    if bytes.len() < 3 {
        return None;
    }

    let last_three = &bytes[bytes.len() - 3..];
    let suffix_lower = [last_three[0].to_ascii_lowercase(), last_three[1], last_three[2]];

    match suffix_lower {
        [b'i', b'1', b'6'] | [b'i', b'3', b'2'] | [b'u', b'1', b'6'] | [b'u', b'3', b'2'] => {
            Some(SuffixPattern::ThreeChar)
        }
        _ => None,
    }
}

/// Checks if the last two characters form a valid two-character suffix.
///
/// Valid two-character suffixes are: 'i8', 'u8' (case-insensitive).
///
/// # Arguments
///
/// * `bytes` - The byte slice to check (must have length >= 2)
///
/// # Returns
///
/// `Some(SuffixPattern::TwoChar)` if the last two bytes form a valid suffix,
/// `None` otherwise.
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::parsers::suffix::check_two_char_suffix;
/// assert!(check_two_char_suffix(b"42i8").is_some());
/// assert!(check_two_char_suffix(b"255u8").is_some());
/// assert!(check_two_char_suffix(b"42xy").is_none());
/// ```
#[inline]
pub fn check_two_char_suffix(bytes: &[u8]) -> Option<SuffixPattern> {
    if bytes.len() < 2 {
        return None;
    }

    let last_two = &bytes[bytes.len() - 2..];
    let suffix_lower = [last_two[0].to_ascii_lowercase(), last_two[1]];

    match suffix_lower {
        [b'i', b'8'] | [b'u', b'8'] => Some(SuffixPattern::TwoChar),
        _ => None,
    }
}

/// Detects the suffix pattern in a byte slice.
///
/// This function implements an optimized suffix detection algorithm:
/// 1. Check for single-character suffixes (most common)
/// 2. Check for three-character suffixes
/// 3. Check for two-character suffixes
///
/// # Arguments
///
/// * `bytes` - The byte slice to analyze
///
/// # Returns
///
/// `Some(SuffixPattern)` if a valid suffix is detected, `None` otherwise.
///
/// # Performance
///
/// The order of checks is optimized for common cases, checking single-character
/// suffixes first as they are most frequent in typical code.
#[inline]
fn detect_suffix_pattern(bytes: &[u8]) -> Option<SuffixPattern> {
    // Fast path: single-character suffixes (most common)
    if let Some(pattern) = check_single_char_suffix(bytes) {
        return Some(pattern);
    }

    // Check three-character suffixes before two-character ones
    // This ordering can be adjusted based on profiling data
    if let Some(pattern) = check_three_char_suffix(bytes) {
        return Some(pattern);
    }

    check_two_char_suffix(bytes)
}

/// Splits a string at a given position, returning the numeric part and suffix.
///
/// # Arguments
///
/// * `slice` - The full string to split
/// * `split_pos` - The byte position where the suffix begins
///
/// # Returns
///
/// A tuple of (numeric_part, Some(suffix))
///
/// # Safety
///
/// The caller must ensure that `split_pos` is a valid UTF-8 boundary within `slice`.
#[inline]
fn split_at_position(slice: &str, split_pos: usize) -> (&str, Option<&str>) {
    (&slice[..split_pos], Some(&slice[split_pos..]))
}

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

    match detect_suffix_pattern(bytes) {
        Some(pattern) => {
            let suffix_len = pattern.len();
            let split_pos = bytes.len() - suffix_len;
            split_at_position(slice, split_pos)
        }
        None => (slice, None),
    }
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
///
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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Tests for check_single_char_suffix
    // =========================================================================

    #[test]
    fn test_check_single_char_suffix_u() {
        assert_eq!(check_single_char_suffix(b"42u"), Some(SuffixPattern::SingleChar));
    }

    #[test]
    fn test_check_single_char_suffix_uppercase_u() {
        assert_eq!(check_single_char_suffix(b"42U"), Some(SuffixPattern::SingleChar));
    }

    #[test]
    fn test_check_single_char_suffix_f() {
        assert_eq!(check_single_char_suffix(b"3.14f"), Some(SuffixPattern::SingleChar));
    }

    #[test]
    fn test_check_single_char_suffix_uppercase_f() {
        assert_eq!(check_single_char_suffix(b"3.14F"), Some(SuffixPattern::SingleChar));
    }

    #[test]
    fn test_check_single_char_suffix_d() {
        assert_eq!(check_single_char_suffix(b"100d"), Some(SuffixPattern::SingleChar));
    }

    #[test]
    fn test_check_single_char_suffix_invalid() {
        assert_eq!(check_single_char_suffix(b"42x"), None);
    }

    #[test]
    fn test_check_single_char_suffix_digit() {
        assert_eq!(check_single_char_suffix(b"42"), None);
    }

    #[test]
    fn test_check_single_char_suffix_empty() {
        assert_eq!(check_single_char_suffix(b""), None);
    }

    // =========================================================================
    // Tests for check_three_char_suffix
    // =========================================================================

    #[test]
    fn test_check_three_char_suffix_i16() {
        assert_eq!(check_three_char_suffix(b"100i16"), Some(SuffixPattern::ThreeChar));
    }

    #[test]
    fn test_check_three_char_suffix_i32() {
        assert_eq!(check_three_char_suffix(b"42i32"), Some(SuffixPattern::ThreeChar));
    }

    #[test]
    fn test_check_three_char_suffix_u16() {
        assert_eq!(check_three_char_suffix(b"1000u16"), Some(SuffixPattern::ThreeChar));
    }

    #[test]
    fn test_check_three_char_suffix_u32() {
        assert_eq!(check_three_char_suffix(b"42u32"), Some(SuffixPattern::ThreeChar));
    }

    #[test]
    fn test_check_three_char_suffix_case_insensitive() {
        assert_eq!(check_three_char_suffix(b"100I16"), Some(SuffixPattern::ThreeChar));
        assert_eq!(check_three_char_suffix(b"100U32"), Some(SuffixPattern::ThreeChar));
    }

    #[test]
    fn test_check_three_char_suffix_invalid() {
        assert_eq!(check_three_char_suffix(b"100abc"), None);
    }

    #[test]
    fn test_check_three_char_suffix_i64() {
        // i64 is not a valid three-char suffix (it's four chars)
        assert_eq!(check_three_char_suffix(b"100i64"), None);
    }

    #[test]
    fn test_check_three_char_suffix_too_short() {
        assert_eq!(check_three_char_suffix(b"42"), None);
    }

    // =========================================================================
    // Tests for check_two_char_suffix
    // =========================================================================

    #[test]
    fn test_check_two_char_suffix_i8() {
        assert_eq!(check_two_char_suffix(b"42i8"), Some(SuffixPattern::TwoChar));
    }

    #[test]
    fn test_check_two_char_suffix_u8() {
        assert_eq!(check_two_char_suffix(b"255u8"), Some(SuffixPattern::TwoChar));
    }

    #[test]
    fn test_check_two_char_suffix_case_insensitive() {
        assert_eq!(check_two_char_suffix(b"42I8"), Some(SuffixPattern::TwoChar));
        assert_eq!(check_two_char_suffix(b"255U8"), Some(SuffixPattern::TwoChar));
    }

    #[test]
    fn test_check_two_char_suffix_invalid() {
        assert_eq!(check_two_char_suffix(b"42xy"), None);
    }

    #[test]
    fn test_check_two_char_suffix_i6() {
        // i6 is not a valid suffix
        assert_eq!(check_two_char_suffix(b"42i6"), None);
    }

    #[test]
    fn test_check_two_char_suffix_too_short() {
        assert_eq!(check_two_char_suffix(b"4"), None);
    }

    // =========================================================================
    // Tests for detect_suffix_pattern
    // =========================================================================

    #[test]
    fn test_detect_suffix_pattern_single_char() {
        assert_eq!(detect_suffix_pattern(b"42u"), Some(SuffixPattern::SingleChar));
        assert_eq!(detect_suffix_pattern(b"3.14f"), Some(SuffixPattern::SingleChar));
    }

    #[test]
    fn test_detect_suffix_pattern_three_char() {
        assert_eq!(detect_suffix_pattern(b"100i16"), Some(SuffixPattern::ThreeChar));
        assert_eq!(detect_suffix_pattern(b"42u32"), Some(SuffixPattern::ThreeChar));
    }

    #[test]
    fn test_detect_suffix_pattern_two_char() {
        assert_eq!(detect_suffix_pattern(b"42i8"), Some(SuffixPattern::TwoChar));
        assert_eq!(detect_suffix_pattern(b"255u8"), Some(SuffixPattern::TwoChar));
    }

    #[test]
    fn test_detect_suffix_pattern_none() {
        assert_eq!(detect_suffix_pattern(b"42"), None);
        assert_eq!(detect_suffix_pattern(b"3.14"), None);
    }

    #[test]
    fn test_detect_suffix_pattern_priority() {
        // When a string could match multiple patterns, single-char takes priority
        // For example, "u" at the end should be detected as single-char, not as part of "u8"
        assert_eq!(detect_suffix_pattern(b"u"), Some(SuffixPattern::SingleChar));
    }

    // =========================================================================
    // Tests for split_numeric_and_suffix (integration tests)
    // =========================================================================

    #[test]
    fn test_split_numeric_and_suffix_no_suffix() {
        assert_eq!(split_numeric_and_suffix("42"), ("42", None));
        assert_eq!(split_numeric_and_suffix("3.14"), ("3.14", None));
    }

    #[test]
    fn test_split_numeric_and_suffix_single_char() {
        assert_eq!(split_numeric_and_suffix("42u"), ("42", Some("u")));
        assert_eq!(split_numeric_and_suffix("3.14F"), ("3.14", Some("F")));
        assert_eq!(split_numeric_and_suffix("100d"), ("100", Some("d")));
    }

    #[test]
    fn test_split_numeric_and_suffix_two_char() {
        assert_eq!(split_numeric_and_suffix("42i8"), ("42", Some("i8")));
        assert_eq!(split_numeric_and_suffix("255u8"), ("255", Some("u8")));
    }

    #[test]
    fn test_split_numeric_and_suffix_three_char() {
        assert_eq!(split_numeric_and_suffix("100i16"), ("100", Some("i16")));
        assert_eq!(split_numeric_and_suffix("42u32"), ("42", Some("u32")));
        assert_eq!(split_numeric_and_suffix("1000U16"), ("1000", Some("U16")));
    }

    #[test]
    fn test_split_numeric_and_suffix_scientific_notation() {
        assert_eq!(split_numeric_and_suffix("6.022e23u32"), ("6.022e23", Some("u32")));
        assert_eq!(split_numeric_and_suffix("1.5e-10f"), ("1.5e-10", Some("f")));
    }

    #[test]
    fn test_split_numeric_and_suffix_empty() {
        assert_eq!(split_numeric_and_suffix(""), ("", None));
    }

    #[test]
    fn test_split_numeric_and_suffix_preserves_case() {
        assert_eq!(split_numeric_and_suffix("42U"), ("42", Some("U")));
        assert_eq!(split_numeric_and_suffix("100I16"), ("100", Some("I16")));
    }

    #[test]
    fn test_split_numeric_and_suffix_complex_numbers() {
        assert_eq!(split_numeric_and_suffix("0.0f"), ("0.0", Some("f")));
        assert_eq!(split_numeric_and_suffix("123456789u"), ("123456789", Some("u")));
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_suffix_only() {
        // Edge case: string is just a suffix
        assert_eq!(split_numeric_and_suffix("u"), ("", Some("u")));
        assert_eq!(split_numeric_and_suffix("i8"), ("", Some("i8")));
    }

    #[test]
    fn test_very_short_strings() {
        assert_eq!(split_numeric_and_suffix("1"), ("1", None));
        assert_eq!(split_numeric_and_suffix("1u"), ("1", Some("u")));
    }

    #[test]
    fn test_unicode_handling() {
        // The function should handle UTF-8 properly
        assert_eq!(split_numeric_and_suffix("42€"), ("42€", None));
    }
}
