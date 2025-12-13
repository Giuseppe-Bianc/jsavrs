// src/tokens/parsers/base.rs
//! Base-specific number literal parsing (binary, octal, hexadecimal).
//!
//! This module handles parsing of non-decimal numeric literals with special
//! prefixes, supporting different numeric bases and optional unsigned suffixes.

use crate::tokens::number::Number;
use crate::tokens::token_kind::TokenKind;

/// Generic parser for base-specific number literals (binary, octal, hexadecimal).
///
/// Handles literals with prefixes:
/// - Binary: `#b` (e.g., `#b1010`)
/// - Octal: `#o` (e.g., `#o755`)
/// - Hexadecimal: `#x` (e.g., `#xDEADBEEF`)
///
/// Supports optional unsigned suffix (`u` or `U`).
///
/// # Arguments
///
/// * `radix` - Numeric base (2 for binary, 8 for octal, 16 for hexadecimal)
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed literals (no 'u' suffix)
/// * `Some(Number::UnsignedInteger)` - For unsigned literals (with 'u' suffix)
/// * `None` - If parsing fails or contains invalid digits for the radix
///
/// # Panics
///
/// Panics if the lexer slice is shorter than 2 characters (i.e., missing the required prefix).
///
/// # Errors
///
/// Returns `None` if:
/// - The number string contains invalid digits for the specified radix
/// - The number string is empty after removing prefix and suffix
/// - The parsed value overflows i64 (signed) or u64 (unsigned)
///
/// # Implementation Notes
///
/// - Strips the 2-character prefix (`#b`, `#o`, or `#x`)
/// - Checks for optional trailing `u` or `U` suffix
/// - Uses `i64::from_str_radix` or `u64::from_str_radix` for parsing
#[inline]
pub fn parse_base_number(radix: u32, lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    if slice.len() < 2 {
        return None;
    }
    let (_, num_part) = slice.split_at(2); // Remove prefix ("#b", "#o", or "#x")
    let (num_str, suffix_u) = match num_part.chars().last() {
        Some('u' | 'U') => (&num_part[..num_part.len() - 1], true),
        _ => (num_part, false),
    };

    if suffix_u {
        u64::from_str_radix(num_str, radix).ok().map(Number::UnsignedInteger)
    } else {
        i64::from_str_radix(num_str, radix).ok().map(Number::Integer)
    }
}

/// Parses binary literals prefixed with `#b`.
///
/// Binary literals use base-2 representation with digits 0 and 1.
/// Supports optional unsigned suffix.
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed binary literals
/// * `Some(Number::UnsignedInteger)` - For unsigned binary literals (with 'u' suffix)
/// * `None` - If parsing fails or contains non-binary digits
///
/// # Format
///
/// - Prefix: `#b` (required)
/// - Digits: `0`, `1` only
/// - Suffix: `u` or `U` (optional, for unsigned)
#[inline]
pub fn parse_binary(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(2, lex)
}

/// Parses octal literals prefixed with `#o`.
///
/// Octal literals use base-8 representation with digits 0-7.
/// Supports optional unsigned suffix.
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed octal literals
/// * `Some(Number::UnsignedInteger)` - For unsigned octal literals (with 'u' suffix)
/// * `None` - If parsing fails or contains non-octal digits
///
/// # Format
///
/// - Prefix: `#o` (required)
/// - Digits: `0-7` only
/// - Suffix: `u` or `U` (optional, for unsigned)
///
/// # Panics
///
/// Panics if the lexer slice is shorter than 2 characters. See [`parse_base_number`] for details.
pub fn parse_octal(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(8, lex)
}

/// Parses hexadecimal literals prefixed with `#x`.
///
/// Hexadecimal literals use base-16 representation with digits 0-9 and A-F (case-insensitive).
/// Supports optional unsigned suffix.
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed hexadecimal literals
/// * `Some(Number::UnsignedInteger)` - For unsigned hexadecimal literals (with 'u' suffix)
/// * `None` - If parsing fails or contains non-hexadecimal digits
///
/// # Format
///
/// - Prefix: `#x` (required)
/// - Digits: `0-9`, `A-F`, `a-f`
/// - Suffix: `u` or `U` (optional, for unsigned)
///
/// # Panics
///
/// Panics if the lexer slice is shorter than 2 characters. See [`parse_base_number`] for details.
pub fn parse_hex(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(16, lex)
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use logos::Logos;

    // Helper function to create a lexer and call parse_base_number
    fn test_parse_base(input: &str, radix: u32) -> Option<Number> {
        let mut lex = TokenKind::lexer(input);
        lex.bump(input.len()); // Simulate that the lexer has consumed the input
        parse_base_number(radix, &mut lex)
    }

    // =========================================================================
    // NORMAL CASES - Binary (radix 2)
    // =========================================================================

    #[test]
    fn test_binary_simple_valid() {
        // Normal case: Simple binary number without suffix
        let result = test_parse_base("#b1010", 2);
        assert_eq!(result, Some(Number::Integer(0b1010)));
    }

    #[test]
    fn test_binary_zero() {
        // Normal case: Binary zero
        let result = test_parse_base("#b0", 2);
        assert_eq!(result, Some(Number::Integer(0)));
    }

    #[test]
    fn test_binary_one() {
        // Normal case: Binary one
        let result = test_parse_base("#b1", 2);
        assert_eq!(result, Some(Number::Integer(1)));
    }

    #[test]
    fn test_binary_all_ones() {
        // Normal case: Binary number with all ones
        let result = test_parse_base("#b1111", 2);
        assert_eq!(result, Some(Number::Integer(0b1111)));
    }

    #[test]
    fn test_binary_unsigned_lowercase() {
        // Normal case: Binary with lowercase 'u' suffix
        let result = test_parse_base("#b1010u", 2);
        assert_eq!(result, Some(Number::UnsignedInteger(0b1010)));
    }

    #[test]
    fn test_binary_unsigned_uppercase() {
        // Normal case: Binary with uppercase 'U' suffix
        let result = test_parse_base("#b1010U", 2);
        assert_eq!(result, Some(Number::UnsignedInteger(0b1010)));
    }

    // =========================================================================
    // NORMAL CASES - Octal (radix 8)
    // =========================================================================

    #[test]
    fn test_octal_simple_valid() {
        // Normal case: Simple octal number without suffix
        let result = test_parse_base("#o755", 8);
        assert_eq!(result, Some(Number::Integer(0o755)));
    }

    #[test]
    fn test_octal_zero() {
        // Normal case: Octal zero
        let result = test_parse_base("#o0", 8);
        assert_eq!(result, Some(Number::Integer(0)));
    }

    #[test]
    fn test_octal_max_digit() {
        // Normal case: Octal with maximum digit (7)
        let result = test_parse_base("#o777", 8);
        assert_eq!(result, Some(Number::Integer(0o777)));
    }

    #[test]
    fn test_octal_unsigned() {
        // Normal case: Octal with unsigned suffix
        let result = test_parse_base("#o644u", 8);
        assert_eq!(result, Some(Number::UnsignedInteger(0o644)));
    }

    // =========================================================================
    // NORMAL CASES - Hexadecimal (radix 16)
    // =========================================================================

    #[test]
    fn test_hex_simple_valid() {
        // Normal case: Simple hexadecimal number without suffix
        let result = test_parse_base("#xDEADBEEF", 16);
        assert_eq!(result, Some(Number::Integer(0xDEADBEEF)));
    }

    #[test]
    fn test_hex_lowercase() {
        // Normal case: Hexadecimal with lowercase letters
        let result = test_parse_base("#xdeadbeef", 16);
        assert_eq!(result, Some(Number::Integer(0xdeadbeef)));
    }

    #[test]
    #[allow(clippy::mixed_case_hex_literals)]
    fn test_hex_mixed_case() {
        // Normal case: Hexadecimal with mixed case letters
        let result = test_parse_base("#xDeAdBeEf", 16);
        assert_eq!(result, Some(Number::Integer(0xDeAdBeEf)));
    }

    #[test]
    fn test_hex_zero() {
        // Normal case: Hexadecimal zero
        let result = test_parse_base("#x0", 16);
        assert_eq!(result, Some(Number::Integer(0)));
    }

    #[test]
    fn test_hex_all_f() {
        // Normal case: Hexadecimal with all F's
        let result = test_parse_base("#xFFFF", 16);
        assert_eq!(result, Some(Number::Integer(0xFFFF)));
    }

    #[test]
    fn test_hex_unsigned() {
        // Normal case: Hexadecimal with unsigned suffix
        let result = test_parse_base("#xCAFEu", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0xCAFE)));
    }

    #[test]
    fn test_hex_digits_only() {
        // Normal case: Hexadecimal with only numeric digits
        let result = test_parse_base("#x123456", 16);
        assert_eq!(result, Some(Number::Integer(0x123456)));
    }

    // =========================================================================
    // EDGE CASES - Minimum valid inputs
    // =========================================================================

    #[test]
    fn test_prefix_only_no_digits() {
        // Edge case: Only prefix without any digits (empty number string)
        let result = test_parse_base("#b", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_prefix_with_suffix_only() {
        // Edge case: Prefix and suffix but no digits
        let result = test_parse_base("#bu", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_single_char_input() {
        // Edge case: Input shorter than minimum required (2 chars for prefix)
        // This should return None as per the function's early return
        let result = test_parse_base("#", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_empty_string() {
        // Edge case: Empty input string
        let result = test_parse_base("", 2);
        assert_eq!(result, None);
    }

    // =========================================================================
    // EDGE CASES - Invalid digits for radix
    // =========================================================================

    #[test]
    fn test_binary_invalid_digit_2() {
        // Edge case: Binary number with invalid digit (2)
        let result = test_parse_base("#b1012", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_invalid_digit_8() {
        // Edge case: Binary number with invalid digit (8)
        let result = test_parse_base("#b1018", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_invalid_letter() {
        // Edge case: Binary number with alphabetic character
        let result = test_parse_base("#b10A1", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_octal_invalid_digit_8() {
        // Edge case: Octal number with invalid digit (8)
        let result = test_parse_base("#o7568", 8);
        assert_eq!(result, None);
    }

    #[test]
    fn test_octal_invalid_digit_9() {
        // Edge case: Octal number with invalid digit (9)
        let result = test_parse_base("#o7569", 8);
        assert_eq!(result, None);
    }

    #[test]
    fn test_octal_invalid_letter() {
        // Edge case: Octal number with alphabetic character
        let result = test_parse_base("#o75A", 8);
        assert_eq!(result, None);
    }

    #[test]
    fn test_hex_invalid_letter_g() {
        // Edge case: Hexadecimal with invalid letter (G)
        let result = test_parse_base("#xDEADG", 16);
        assert_eq!(result, None);
    }

    #[test]
    fn test_hex_invalid_letter_z() {
        // Edge case: Hexadecimal with invalid letter (Z)
        let result = test_parse_base("#xDEADZ", 16);
        assert_eq!(result, None);
    }

    // =========================================================================
    // EDGE CASES - Special characters and whitespace
    // =========================================================================

    #[test]
    fn test_binary_with_space() {
        // Edge case: Binary number with embedded space
        let result = test_parse_base("#b10 10", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_binary_with_underscore() {
        // Edge case: Binary number with underscore separator (not supported)
        let result = test_parse_base("#b1010_1010", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_hex_with_special_char() {
        // Edge case: Hexadecimal with special character
        let result = test_parse_base("#xDEAD-BEEF", 16);
        assert_eq!(result, None);
    }

    // =========================================================================
    // CORNER CASES - Boundary values for i64
    // =========================================================================

    #[test]
    fn test_binary_i64_max() {
        // Corner case: Maximum positive i64 value in binary
        // i64::MAX = 9223372036854775807 = 0x7FFFFFFFFFFFFFFF
        let result = test_parse_base("#b111111111111111111111111111111111111111111111111111111111111111", 2);
        assert_eq!(result, Some(Number::Integer(i64::MAX)));
    }

    #[test]
    fn test_octal_i64_max() {
        // Corner case: Maximum positive i64 value in octal
        // i64::MAX = 0o777777777777777777777
        let result = test_parse_base("#o777777777777777777777", 8);
        assert_eq!(result, Some(Number::Integer(i64::MAX)));
    }

    #[test]
    fn test_hex_i64_max() {
        // Corner case: Maximum positive i64 value in hexadecimal
        // i64::MAX = 0x7FFFFFFFFFFFFFFF
        let result = test_parse_base("#x7FFFFFFFFFFFFFFF", 16);
        assert_eq!(result, Some(Number::Integer(i64::MAX)));
    }

    #[test]
    fn test_hex_i64_overflow() {
        // Corner case: Value exceeding i64::MAX (should fail for signed)
        // 0x8000000000000000 = i64::MIN in two's complement, but as positive it overflows
        let result = test_parse_base("#x8000000000000000", 16);
        assert_eq!(result, None);
    }

    #[test]
    fn test_hex_i64_large_overflow() {
        // Corner case: Large value far exceeding i64::MAX
        let result = test_parse_base("#xFFFFFFFFFFFFFFFF", 16);
        assert_eq!(result, None);
    }

    // =========================================================================
    // CORNER CASES - Boundary values for u64
    // =========================================================================

    #[test]
    fn test_binary_u64_max() {
        // Corner case: Maximum u64 value in binary
        // u64::MAX = 18446744073709551615 = 64 ones
        let result = test_parse_base("#b1111111111111111111111111111111111111111111111111111111111111111u", 2);
        assert_eq!(result, Some(Number::UnsignedInteger(u64::MAX)));
    }

    #[test]
    fn test_octal_u64_max() {
        // Corner case: Maximum u64 value in octal
        // u64::MAX = 0o1777777777777777777777
        let result = test_parse_base("#o1777777777777777777777u", 8);
        assert_eq!(result, Some(Number::UnsignedInteger(u64::MAX)));
    }

    #[test]
    fn test_hex_u64_max() {
        // Corner case: Maximum u64 value in hexadecimal
        // u64::MAX = 0xFFFFFFFFFFFFFFFF
        let result = test_parse_base("#xFFFFFFFFFFFFFFFFu", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(u64::MAX)));
    }

    #[test]
    fn test_hex_u64_overflow() {
        // Corner case: Value exceeding u64::MAX (should fail)
        // This is u64::MAX + 1, which requires 65 bits
        let result = test_parse_base("#x10000000000000000u", 16);
        assert_eq!(result, None);
    }

    // =========================================================================
    // CORNER CASES - Unsigned suffix with values in different ranges
    // =========================================================================

    #[test]
    fn test_unsigned_value_exceeds_i64_but_fits_u64() {
        // Corner case: Unsigned value that doesn't fit in i64 but fits in u64
        // 0x8000000000000000 = 9223372036854775808 (i64::MAX + 1)
        let result = test_parse_base("#x8000000000000000u", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0x8000000000000000)));
    }

    #[test]
    fn test_unsigned_near_u64_max() {
        // Corner case: Unsigned value very close to u64::MAX
        let result = test_parse_base("#xFFFFFFFFFFFFFFFEu", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0xFFFFFFFFFFFFFFFE)));
    }

    #[test]
    fn test_unsigned_zero() {
        // Corner case: Unsigned zero
        let result = test_parse_base("#x0u", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0)));
    }

    #[test]
    fn test_unsigned_one() {
        // Corner case: Unsigned one
        let result = test_parse_base("#b1u", 2);
        assert_eq!(result, Some(Number::UnsignedInteger(1)));
    }

    // =========================================================================
    // CORNER CASES - Long number strings
    // =========================================================================

    #[test]
    fn test_binary_long_valid() {
        // Corner case: Long binary number string (still valid)
        let result = test_parse_base("#b101010101010101010101010101010101010101010101010101010101010101", 2);
        assert_eq!(result, Some(Number::Integer(0b101010101010101010101010101010101010101010101010101010101010101)));
    }

    #[test]
    fn test_binary_too_long_overflow() {
        // Corner case: Binary string with too many bits (65 bits, overflows i64)
        let result = test_parse_base("#b10000000000000000000000000000000000000000000000000000000000000000", 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_hex_very_long_overflow() {
        // Corner case: Very long hexadecimal string causing overflow
        let result = test_parse_base("#x123456789ABCDEF0123456789", 16);
        assert_eq!(result, None);
    }

    // =========================================================================
    // CORNER CASES - Unusual but valid suffix positions
    // =========================================================================

    #[test]
    fn test_suffix_uppercase_u_middle_of_hex() {
        // Corner case: Uppercase U that looks like hex digit but is at the end
        // This tests that suffix detection happens before hex parsing
        let result = test_parse_base("#xDEADU", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0xDEAD)));
    }

    #[test]
    fn test_suffix_lowercase_u_middle_of_hex() {
        // Corner case: Lowercase u that looks like potential typo
        let result = test_parse_base("#xDEADu", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0xDEAD)));
    }

    #[test]
    fn test_multiple_u_suffix() {
        // Corner case: Multiple 'u' characters (only last one treated as suffix)
        // The 'u' before the last one should be treated as invalid digit
        let result = test_parse_base("#b1010uu", 2);
        assert_eq!(result, None);
    }

    // =========================================================================
    // CORNER CASES - Different radix values
    // =========================================================================

    #[test]
    fn test_radix_3_valid() {
        // Corner case: Base-3 (ternary) number
        let result = test_parse_base("#t210", 3);
        assert_eq!(result, Some(Number::Integer(21))); // 2*9 + 1*3 + 0 = 21
    }

    #[test]
    fn test_radix_3_invalid_digit() {
        // Corner case: Base-3 with invalid digit (3)
        let result = test_parse_base("#t213", 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_radix_36_max() {
        // Corner case: Base-36 (maximum standard radix) with all valid chars
        let result = test_parse_base("#zZ9", 36);
        // Z = 35 in base 36, so "Z9" = 35*36 + 9 = 1269
        assert_eq!(result, Some(Number::Integer(1269)));
    }

    // =========================================================================
    // CORNER CASES - Case sensitivity
    // =========================================================================

    #[test]
    fn test_hex_uppercase_letters() {
        // Corner case: All uppercase hex letters
        let result = test_parse_base("#xABCDEF", 16);
        assert_eq!(result, Some(Number::Integer(0xABCDEF)));
    }

    #[test]
    fn test_hex_lowercase_letters() {
        // Corner case: All lowercase hex letters
        let result = test_parse_base("#xabcdef", 16);
        assert_eq!(result, Some(Number::Integer(0xabcdef)));
    }

    #[test]
    fn test_octal_with_uppercase_suffix() {
        // Corner case: Octal with uppercase U suffix
        let result = test_parse_base("#o777U", 8);
        assert_eq!(result, Some(Number::UnsignedInteger(0o777)));
    }

    // =========================================================================
    // CORNER CASES - Leading zeros
    // =========================================================================

    #[test]
    fn test_binary_leading_zeros() {
        // Corner case: Binary with leading zeros
        let result = test_parse_base("#b0001010", 2);
        assert_eq!(result, Some(Number::Integer(0b1010)));
    }

    #[test]
    fn test_octal_leading_zeros() {
        // Corner case: Octal with leading zeros
        let result = test_parse_base("#o0000755", 8);
        assert_eq!(result, Some(Number::Integer(0o755)));
    }

    #[test]
    fn test_hex_leading_zeros() {
        // Corner case: Hex with leading zeros
        let result = test_parse_base("#x0000DEAD", 16);
        assert_eq!(result, Some(Number::Integer(0xDEAD)));
    }

    #[test]
    fn test_all_zeros() {
        // Corner case: Number consisting only of zeros
        let result = test_parse_base("#b00000000", 2);
        assert_eq!(result, Some(Number::Integer(0)));
    }

    #[test]
    fn test_all_zeros_unsigned() {
        // Corner case: Unsigned number consisting only of zeros
        let result = test_parse_base("#x00000000u", 16);
        assert_eq!(result, Some(Number::UnsignedInteger(0)));
    }
}
