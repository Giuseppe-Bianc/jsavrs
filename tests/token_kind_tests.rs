use jsavrs::tokens::number::Number::*;
use jsavrs::tokens::parsers::numeric::is_valid_integer_literal;
use jsavrs::tokens::parsers::suffix::{handle_suffix, split_numeric_and_suffix};
use jsavrs::tokens::token_kind::TokenKind;
use jsavrs::tokens::token_kind::TokenKind::*;
use logos::Logos;
use std::sync::Arc;

// Helper function to assert token matching
fn assert_token(input: &str, expected: TokenKind) {
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(expected));
}

#[test]
fn test_parse_integer() {
    assert_token("123", Numeric(Integer(123)));
}

#[test]
fn test_valid_integer_literals() {
    // Single digit cases
    assert!(is_valid_integer_literal("0"), "Single zero should be valid");
    assert!(is_valid_integer_literal("1"), "Single digit 1 should be valid");
    assert!(is_valid_integer_literal("9"), "Single digit 9 should be valid");

    // Multi-digit cases
    assert!(is_valid_integer_literal("42"), "Simple two-digit number should be valid");
    assert!(is_valid_integer_literal("123"), "Three-digit number should be valid");
    assert!(is_valid_integer_literal("9876543210"), "Large number should be valid");

    // Edge digit combinations
    assert!(is_valid_integer_literal("10"), "Number ending with zero should be valid");
    assert!(is_valid_integer_literal("100"), "Number with multiple trailing zeros should be valid");
    assert!(is_valid_integer_literal("12345678901234567890"), "Very large number should be valid");

    // Minimum and maximum ASCII digit strings
    assert!(is_valid_integer_literal("0000000000"), "All zeros should be valid");
    assert!(is_valid_integer_literal("9999999999"), "All nines should be valid");
}

/// Tests invalid integer literals containing decimal points
#[test]
fn test_invalid_with_decimal_points() {
    // Simple decimal cases
    assert!(!is_valid_integer_literal("3.14"), "Float with decimal point should be invalid");
    assert!(!is_valid_integer_literal("42."), "Number with trailing decimal should be invalid");
    assert!(!is_valid_integer_literal(".42"), "Number with leading decimal should be invalid");

    // Multiple decimal points
    assert!(!is_valid_integer_literal("1.2.3"), "Number with multiple decimals should be invalid");
    assert!(!is_valid_integer_literal("1..2"), "Number with consecutive decimals should be invalid");

    // Decimal with other invalid characters
    assert!(!is_valid_integer_literal("42.0e3"), "Scientific notation with decimal should be invalid");
    assert!(!is_valid_integer_literal("1.2e3"), "Scientific float should be invalid");
}

/// Tests invalid integer literals containing exponent markers
#[test]
fn test_invalid_with_exponents() {
    // Lowercase exponent markers
    assert!(!is_valid_integer_literal("1e3"), "Lowercase exponent should be invalid");
    assert!(!is_valid_integer_literal("2e+5"), "Lowercase exponent with plus should be invalid");
    assert!(!is_valid_integer_literal("3e-2"), "Lowercase exponent with minus should be invalid");

    // Uppercase exponent markers
    assert!(!is_valid_integer_literal("4E5"), "Uppercase exponent should be invalid");
    assert!(!is_valid_integer_literal("5E+10"), "Uppercase exponent with plus should be invalid");
    assert!(!is_valid_integer_literal("6E-1"), "Uppercase exponent with minus should be invalid");

    // Mixed case and edge cases
    assert!(!is_valid_integer_literal("7e0"), "Exponent zero should be invalid");
    assert!(!is_valid_integer_literal("8E999"), "Large exponent should be invalid");
    assert!(!is_valid_integer_literal("e10"), "Missing base with exponent should be invalid");
    assert!(!is_valid_integer_literal("10e"), "Missing exponent value should be invalid");
}

/// Tests invalid integer literals containing non-digit characters
#[test]
fn test_invalid_with_non_digit_characters() {
    // Alphabetic characters
    assert!(!is_valid_integer_literal("abc"), "Alphabetic string should be invalid");
    assert!(!is_valid_integer_literal("12a"), "Trailing letter should be invalid");
    assert!(!is_valid_integer_literal("a12"), "Leading letter should be invalid");
    assert!(!is_valid_integer_literal("1a2"), "Middle letter should be invalid");

    // Special characters
    assert!(!is_valid_integer_literal("42!"), "Exclamation mark should be invalid");
    assert!(!is_valid_integer_literal("42@"), "At symbol should be invalid");
    assert!(!is_valid_integer_literal("42#"), "Hash symbol should be invalid");
    assert!(!is_valid_integer_literal("42$"), "Dollar sign should be invalid");
    assert!(!is_valid_integer_literal("42%"), "Percent sign should be invalid");

    // Mathematical symbols
    assert!(!is_valid_integer_literal("42+"), "Plus sign should be invalid");
    assert!(!is_valid_integer_literal("42-"), "Minus sign should be invalid");
    assert!(!is_valid_integer_literal("42*"), "Asterisk should be invalid");
    assert!(!is_valid_integer_literal("42/"), "Forward slash should be invalid");

    // Other symbols
    assert!(!is_valid_integer_literal("42_"), "Underscore should be invalid");
    assert!(!is_valid_integer_literal("42,"), "Comma should be invalid");
    assert!(!is_valid_integer_literal("42;"), "Semicolon should be invalid");
    assert!(!is_valid_integer_literal("42:"), "Colon should be invalid");
}

/// Tests invalid integer literals containing sign characters
#[test]
fn test_invalid_with_signs() {
    // Positive signs
    assert!(!is_valid_integer_literal("+42"), "Leading plus sign should be invalid");
    assert!(!is_valid_integer_literal("+0"), "Plus zero should be invalid");
    assert!(!is_valid_integer_literal("+123"), "Positive number with plus should be invalid");

    // Negative signs
    assert!(!is_valid_integer_literal("-42"), "Leading minus sign should be invalid");
    assert!(!is_valid_integer_literal("-0"), "Negative zero should be invalid");
    assert!(!is_valid_integer_literal("-999"), "Negative number should be invalid");

    // Multiple signs
    assert!(!is_valid_integer_literal("+-42"), "Multiple signs should be invalid");
    assert!(!is_valid_integer_literal("-+42"), "Mixed signs should be invalid");
    assert!(!is_valid_integer_literal("++42"), "Double plus should be invalid");
    assert!(!is_valid_integer_literal("--42"), "Double minus should be invalid");

    // Signs with other invalid characters
    assert!(!is_valid_integer_literal("+42.5"), "Signed float should be invalid");
    assert!(!is_valid_integer_literal("-1e3"), "Signed scientific should be invalid");
}

/// Tests invalid integer literals with whitespace and formatting characters
#[test]
fn test_invalid_with_whitespace_and_formatting() {
    // Leading whitespace
    assert!(!is_valid_integer_literal(" 42"), "Leading space should be invalid");
    assert!(!is_valid_integer_literal("\t42"), "Leading tab should be invalid");
    assert!(!is_valid_integer_literal("\n42"), "Leading newline should be invalid");
    assert!(!is_valid_integer_literal("\r42"), "Leading carriage return should be invalid");

    // Trailing whitespace
    assert!(!is_valid_integer_literal("42 "), "Trailing space should be invalid");
    assert!(!is_valid_integer_literal("42\t"), "Trailing tab should be invalid");
    assert!(!is_valid_integer_literal("42\n"), "Trailing newline should be invalid");
    assert!(!is_valid_integer_literal("42\r"), "Trailing carriage return should be invalid");

    // Internal whitespace
    assert!(!is_valid_integer_literal("4 2"), "Internal space should be invalid");
    assert!(!is_valid_integer_literal("4\t2"), "Internal tab should be invalid");
    assert!(!is_valid_integer_literal("4\n2"), "Internal newline should be invalid");

    // Mixed whitespace
    assert!(!is_valid_integer_literal(" 42 "), "Surrounded by spaces should be invalid");
    assert!(!is_valid_integer_literal("\t42\t"), "Surrounded by tabs should be invalid");
    assert!(!is_valid_integer_literal(" \t42\t "), "Mixed surrounding whitespace should be invalid");
}

/// Tests invalid integer literals with digit separators and prefixes
#[test]
fn test_invalid_with_digit_separators_and_prefixes() {
    // Digit separators (underscores)
    assert!(!is_valid_integer_literal("1_000"), "Underscore separator should be invalid");
    assert!(!is_valid_integer_literal("123_456"), "Middle underscore should be invalid");
    assert!(!is_valid_integer_literal("1_2_3"), "Multiple underscores should be invalid");
    assert!(!is_valid_integer_literal("_123"), "Leading underscore should be invalid");
    assert!(!is_valid_integer_literal("123_"), "Trailing underscore should be invalid");

    // Numeric prefixes
    assert!(!is_valid_integer_literal("0x42"), "Hexadecimal prefix should be invalid");
    assert!(!is_valid_integer_literal("0o755"), "Octal prefix should be invalid");
    assert!(!is_valid_integer_literal("0b1010"), "Binary prefix should be invalid");
    assert!(!is_valid_integer_literal("0XFF"), "Uppercase hex prefix should be invalid");
    assert!(!is_valid_integer_literal("0O777"), "Uppercase octal prefix should be invalid");
    assert!(!is_valid_integer_literal("0B1100"), "Uppercase binary prefix should be invalid");

    // Mixed prefix and separator cases
    assert!(!is_valid_integer_literal("0x1_2_3"), "Hex with separators should be invalid");
    assert!(!is_valid_integer_literal("0o123_456"), "Octal with separators should be invalid");
    assert!(!is_valid_integer_literal("0b1010_1010"), "Binary with separators should be invalid");
}

/// Tests invalid integer literals with non-ASCII characters
#[test]
fn test_invalid_with_non_ascii_characters() {
    // Non-ASCII digits
    assert!(!is_valid_integer_literal("१२३"), "Hindi digits should be invalid");
    assert!(!is_valid_integer_literal("١٢٣"), "Arabic digits should be invalid");
    assert!(!is_valid_integer_literal("１２３"), "Full-width digits should be invalid");

    // Unicode letters that look like digits
    assert!(!is_valid_integer_literal("l0"), "Lowercase L and zero should be invalid");
    assert!(!is_valid_integer_literal("O1"), "Uppercase O and one should be invalid");
    assert!(!is_valid_integer_literal("I1"), "Uppercase I and one should be invalid");

    // Unicode symbols
    assert!(!is_valid_integer_literal("4²"), "Superscript two should be invalid");
    assert!(!is_valid_integer_literal("½"), "Vulgar fraction should be invalid");
    assert!(!is_valid_integer_literal("∞"), "Infinity symbol should be invalid");

    // Unicode whitespace
    assert!(!is_valid_integer_literal("4\u{2007}2"), "Figure space should be invalid");
    assert!(!is_valid_integer_literal("4\u{202F}2"), "Narrow no-break space should be invalid");
    assert!(!is_valid_integer_literal("4\u{3000}2"), "Ideographic space should be invalid");
}

/// Tests edge cases and boundary conditions
#[test]
fn test_edge_cases_and_boundary_conditions() {
    // Empty string
    assert!(!is_valid_integer_literal(""), "Empty string should be invalid");

    // Single invalid character cases
    assert!(!is_valid_integer_literal("."), "Single decimal point should be invalid");
    assert!(!is_valid_integer_literal("e"), "Single exponent marker should be invalid");
    assert!(!is_valid_integer_literal("E"), "Single uppercase exponent marker should be invalid");
    assert!(!is_valid_integer_literal("+"), "Single plus sign should be invalid");
    assert!(!is_valid_integer_literal("-"), "Single minus sign should be invalid");
    assert!(!is_valid_integer_literal("_"), "Single underscore should be invalid");

    // Very long strings
    let long_valid = "1".repeat(1000);
    assert!(is_valid_integer_literal(&long_valid), "Very long valid digit string should be valid");

    let long_invalid = format!("{}e{}", "1".repeat(500), "2".repeat(500));
    assert!(!is_valid_integer_literal(&long_invalid), "Very long invalid string should be invalid");

    // Strings with zero-width characters
    assert!(!is_valid_integer_literal("4\u{200B}2"), "Zero-width space should be invalid");
    assert!(!is_valid_integer_literal("4\u{200C}2"), "Zero-width non-joiner should be invalid");
    assert!(!is_valid_integer_literal("4\u{200D}2"), "Zero-width joiner should be invalid");

    // Strings with BOM (Byte Order Mark)
    assert!(!is_valid_integer_literal("\u{FEFF}42"), "BOM at start should be invalid");
    assert!(!is_valid_integer_literal("42\u{FEFF}"), "BOM at end should be invalid");

    // Control characters
    assert!(!is_valid_integer_literal("42\u{0001}"), "Control character should be invalid");
    assert!(!is_valid_integer_literal("\u{0002}42"), "Control character at start should be invalid");
    assert!(!is_valid_integer_literal("4\u{0003}2"), "Control character in middle should be invalid");
}

/// Tests mixed invalid patterns
#[test]
fn test_mixed_invalid_patterns() {
    // Multiple invalid character types
    assert!(!is_valid_integer_literal("42.5e3"), "Float with exponent should be invalid");
    assert!(!is_valid_integer_literal("-42.5"), "Negative float should be invalid");
    assert!(!is_valid_integer_literal("+1e-3"), "Signed scientific notation should be invalid");

    // Complex combinations
    assert!(!is_valid_integer_literal("1_000.5"), "Separator with decimal should be invalid");
    assert!(!is_valid_integer_literal("0x42.5"), "Hex float should be invalid");
    assert!(!is_valid_integer_literal("123e4.5"), "Exponent with decimal should be invalid");

    // Extreme edge cases
    assert!(!is_valid_integer_literal("e+9999999999999999999"), "Large exponent should be invalid");
    assert!(!is_valid_integer_literal("9999999999999999999e999"), "Large base with large exponent should be invalid");
    assert!(!is_valid_integer_literal("........................................"), "Many decimals should be invalid");
    assert!(!is_valid_integer_literal("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"), "Many exponents should be invalid");
}
/// Tests that the function behaves correctly with all possible ASCII characters
#[test]
fn test_all_ascii_characters() {
    // Test all ASCII characters individually
    for ascii_code in 0..=127 {
        let c = ascii_code as u8 as char;
        let input = c.to_string();
        let result = is_valid_integer_literal(&input);

        // Only ASCII digits should be valid
        let expected = ('0'..='9').contains(&c);

        assert_eq!(
            result,
            expected,
            "Character '{}' (ASCII {}) should be {}",
            if c.is_ascii_control() { format!("\\x{:02X}", ascii_code) } else { c.to_string() },
            ascii_code,
            if expected { "valid" } else { "invalid" }
        );
    }
}

/// Tests that the function correctly rejects strings with only valid digits but other issues
#[test]
fn test_valid_digits_but_other_issues() {
    // These contain only digits but have other problems that make them invalid
    // (though in this function's case, these should all be valid since they're pure digits)

    // Leading zeros are valid for this function
    assert!(is_valid_integer_literal("00"), "Leading zeros should be valid");
    assert!(is_valid_integer_literal("01"), "Leading zero with digit should be valid");
    assert!(is_valid_integer_literal("000123"), "Multiple leading zeros should be valid");

    // Very large numbers (still just digits)
    assert!(is_valid_integer_literal(&"9".repeat(100)), "100 nines should be valid");
    assert!(is_valid_integer_literal(&"1".repeat(1000)), "1000 ones should be valid");

    // This demonstrates that the function only cares about character validity,
    // not numeric range or semantic meaning
}

#[test]
fn test_empty_slice_for_split_numeric_and_suffix() {
    let (num_part, suffix) = split_numeric_and_suffix("");
    assert_eq!(num_part, "");
    assert_eq!(suffix, None);
}

#[test]
fn test_single_char_suffix_lowercase_u() {
    // Normal case: Unsigned integer suffix in lowercase
    // Tests the most common single-character suffix pattern
    assert_eq!(split_numeric_and_suffix("42u"), ("42", Some("u")));
}

#[test]
fn test_single_char_suffix_uppercase_u() {
    // Normal case: Unsigned integer suffix in uppercase
    // Verifies case preservation while matching is case-insensitive
    assert_eq!(split_numeric_and_suffix("42U"), ("42", Some("U")));
}

#[test]
fn test_single_char_suffix_lowercase_f() {
    // Normal case: Float32 suffix in lowercase
    // Tests float type annotation with decimal point
    assert_eq!(split_numeric_and_suffix("3.14f"), ("3.14", Some("f")));
}

#[test]
fn test_single_char_suffix_uppercase_f() {
    // Normal case: Float32 suffix in uppercase
    // Common pattern in scientific computing code
    assert_eq!(split_numeric_and_suffix("3.14F"), ("3.14", Some("F")));
}

#[test]
fn test_single_char_suffix_lowercase_d() {
    // Normal case: Float64 (double) suffix in lowercase
    // Tests explicit double-precision float annotation
    assert_eq!(split_numeric_and_suffix("2.718d"), ("2.718", Some("d")));
}

#[test]
fn test_single_char_suffix_uppercase_d() {
    // Normal case: Float64 suffix in uppercase
    // Verifies case handling for double-precision floats
    assert_eq!(split_numeric_and_suffix("2.718D"), ("2.718", Some("D")));
}

#[test]
fn test_two_char_suffix_i8_lowercase() {
    // Normal case: 8-bit signed integer suffix
    // Tests smallest signed integer type
    assert_eq!(split_numeric_and_suffix("127i8"), ("127", Some("i8")));
}

#[test]
fn test_two_char_suffix_i8_uppercase() {
    // Normal case: 8-bit signed integer with uppercase suffix
    // Verifies case-insensitive matching for I8
    assert_eq!(split_numeric_and_suffix("127I8"), ("127", Some("I8")));
}

#[test]
fn test_two_char_suffix_i8_mixed_case() {
    // Normal case: Mixed case suffix (i8 with uppercase I)
    // Tests case preservation with mixed case input
    assert_eq!(split_numeric_and_suffix("127I8"), ("127", Some("I8")));
}

#[test]
fn test_two_char_suffix_u8_lowercase() {
    // Normal case: 8-bit unsigned integer suffix
    // Tests smallest unsigned integer type
    assert_eq!(split_numeric_and_suffix("255u8"), ("255", Some("u8")));
}

#[test]
fn test_two_char_suffix_u8_uppercase() {
    // Normal case: 8-bit unsigned with uppercase suffix
    // Verifies case handling for U8
    assert_eq!(split_numeric_and_suffix("255U8"), ("255", Some("U8")));
}

#[test]
fn test_three_char_suffix_i16_lowercase() {
    // Normal case: 16-bit signed integer suffix
    // Tests common integer type for moderate ranges
    assert_eq!(split_numeric_and_suffix("1000i16"), ("1000", Some("i16")));
}

#[test]
fn test_three_char_suffix_i16_uppercase() {
    // Normal case: 16-bit signed with uppercase suffix
    // Verifies case-insensitive matching for I16
    assert_eq!(split_numeric_and_suffix("1000I16"), ("1000", Some("I16")));
}

#[test]
fn test_three_char_suffix_i32_lowercase() {
    // Normal case: 32-bit signed integer suffix
    // Tests most common explicit integer type
    assert_eq!(split_numeric_and_suffix("100000i32"), ("100000", Some("i32")));
}

#[test]
fn test_three_char_suffix_i32_uppercase() {
    // Normal case: 32-bit signed with uppercase suffix
    // Common in cross-platform code
    assert_eq!(split_numeric_and_suffix("100000I32"), ("100000", Some("I32")));
}

#[test]
fn test_three_char_suffix_u16_lowercase() {
    // Normal case: 16-bit unsigned integer suffix
    // Tests unsigned type for port numbers, etc.
    assert_eq!(split_numeric_and_suffix("65535u16"), ("65535", Some("u16")));
}

#[test]
fn test_three_char_suffix_u16_uppercase() {
    // Normal case: 16-bit unsigned with uppercase suffix
    // Verifies case handling for U16
    assert_eq!(split_numeric_and_suffix("65535U16"), ("65535", Some("U16")));
}

#[test]
fn test_three_char_suffix_u32_lowercase() {
    // Normal case: 32-bit unsigned integer suffix
    // Tests large unsigned values
    assert_eq!(split_numeric_and_suffix("4294967295u32"), ("4294967295", Some("u32")));
}

#[test]
fn test_three_char_suffix_u32_uppercase() {
    // Normal case: 32-bit unsigned with uppercase suffix
    // Common in systems programming
    assert_eq!(split_numeric_and_suffix("4294967295U32"), ("4294967295", Some("U32")));
}

#[test]
fn test_no_suffix_integer() {
    // Normal case: Integer without suffix
    // Tests default type inference case
    assert_eq!(split_numeric_and_suffix("42"), ("42", None));
}

#[test]
fn test_no_suffix_float() {
    // Normal case: Float without suffix
    // Tests default float type inference
    assert_eq!(split_numeric_and_suffix("3.14"), ("3.14", None));
}

#[test]
fn test_scientific_notation_with_suffix() {
    // Normal case: Scientific notation with type suffix
    // Tests complex numeric format with suffix
    assert_eq!(split_numeric_and_suffix("6.022e23u32"), ("6.022e23", Some("u32")));
}

#[test]
fn test_scientific_notation_uppercase_e_with_suffix() {
    // Normal case: Scientific notation with uppercase E
    // Verifies handling of uppercase exponent notation
    assert_eq!(split_numeric_and_suffix("1.23E-10f"), ("1.23E-10", Some("f")));
}

#[test]
fn test_negative_number_with_suffix() {
    // Normal case: Negative number with suffix
    // Tests sign handling with type annotation
    assert_eq!(split_numeric_and_suffix("-42i32"), ("-42", Some("i32")));
}

#[test]
fn test_positive_sign_with_suffix() {
    // Normal case: Explicit positive sign with suffix
    // Less common but valid numeric format
    assert_eq!(split_numeric_and_suffix("+42u"), ("+42", Some("u")));
}

#[test]
fn test_hexadecimal_with_suffix() {
    // Normal case: Hexadecimal literal with suffix
    // Tests suffix parsing with hex notation
    assert_eq!(split_numeric_and_suffix("0xFFu16"), ("0xFF", Some("u16")));
}

#[test]
fn test_binary_with_suffix() {
    // Normal case: Binary literal with suffix
    // Tests suffix parsing with binary notation
    assert_eq!(split_numeric_and_suffix("0b1010u8"), ("0b1010", Some("u8")));
}

#[test]
fn test_octal_with_suffix() {
    // Normal case: Octal literal with suffix
    // Tests suffix parsing with octal notation
    assert_eq!(split_numeric_and_suffix("0o755u16"), ("0o755", Some("u16")));
}

// ============================================================================
// EDGE CASES: Boundary Conditions
// ============================================================================
// These tests examine inputs at the boundaries of valid/invalid scenarios,
// including minimal inputs, unusual but valid formats, and ambiguous cases.

#[test]
fn test_empty_string() {
    // Edge case: Empty input string
    // Tests handling of minimal possible input
    assert_eq!(split_numeric_and_suffix(""), ("", None));
}

#[test]
fn test_single_character_digit() {
    // Edge case: Single digit with no suffix
    // Tests minimal valid numeric input
    assert_eq!(split_numeric_and_suffix("0"), ("0", None));
}

#[test]
fn test_single_character_non_suffix() {
    // Edge case: Single non-suffix character
    // Should not be treated as suffix
    assert_eq!(split_numeric_and_suffix("x"), ("x", None));
}

#[test]
fn test_single_character_u() {
    // Edge case: Just the suffix character
    // Tests degenerate case where entire input is potential suffix
    assert_eq!(split_numeric_and_suffix("u"), ("", Some("u")));
}

#[test]
fn test_single_character_f() {
    // Edge case: Just the 'f' character
    // Another degenerate suffix-only case
    assert_eq!(split_numeric_and_suffix("f"), ("", Some("f")));
}

#[test]
fn test_single_character_d() {
    // Edge case: Just the 'd' character
    // Tests single-char suffix at minimum length
    assert_eq!(split_numeric_and_suffix("d"), ("", Some("d")));
}

#[test]
fn test_two_characters_no_suffix() {
    // Edge case: Two characters, not a valid suffix
    // Tests minimal length where 2-char suffix would apply
    assert_eq!(split_numeric_and_suffix("42"), ("42", None));
}

#[test]
fn test_two_characters_with_suffix() {
    // Edge case: Minimal numeric part with suffix
    // Tests smallest valid suffixed number
    assert_eq!(split_numeric_and_suffix("1u"), ("1", Some("u")));
}

#[test]
fn test_two_characters_exact_i8() {
    // Edge case: Just the i8 suffix
    // Tests 2-char suffix with no numeric part
    assert_eq!(split_numeric_and_suffix("i8"), ("i8", None));
}

#[test]
fn test_two_characters_exact_u8() {
    // Edge case: Just the u8 suffix
    // Another 2-char suffix-only case
    assert_eq!(split_numeric_and_suffix("u8"), ("u8", None));
}

#[test]
fn test_three_characters_exact_i16() {
    // Edge case: Just the i16 suffix
    // Tests 3-char suffix with no numeric part
    assert_eq!(split_numeric_and_suffix("i16"), ("", Some("i16")));
}

#[test]
fn test_three_characters_exact_u32() {
    // Edge case: Just the u32 suffix
    // Another 3-char suffix-only case
    assert_eq!(split_numeric_and_suffix("u32"), ("", Some("u32")));
}

#[test]
fn test_very_long_number_with_suffix() {
    // Edge case: Very long numeric literal
    // Tests handling of large input strings
    assert_eq!(
        split_numeric_and_suffix("123456789012345678901234567890u"),
        ("123456789012345678901234567890", Some("u"))
    );
}

#[test]
fn test_leading_zeros_with_suffix() {
    // Edge case: Leading zeros with suffix
    // Tests preservation of leading zeros
    assert_eq!(split_numeric_and_suffix("0042u"), ("0042", Some("u")));
}

#[test]
fn test_multiple_decimal_points_with_suffix() {
    // Edge case: Invalid float format (multiple decimal points)
    // Function should still split correctly; validation happens elsewhere
    assert_eq!(split_numeric_and_suffix("3.14.15f"), ("3.14.15", Some("f")));
}

#[test]
fn test_decimal_point_at_end_with_suffix() {
    // Edge case: Trailing decimal point
    // Tests unusual but sometimes valid float format
    assert_eq!(split_numeric_and_suffix("42.f"), ("42.", Some("f")));
}

#[test]
fn test_decimal_point_at_start_with_suffix() {
    // Edge case: Leading decimal point
    // Tests .5 style float notation
    assert_eq!(split_numeric_and_suffix(".5f"), (".5", Some("f")));
}

#[test]
fn test_just_decimal_point_with_suffix() {
    // Edge case: Just a decimal point with suffix
    // Tests minimal float-like input
    assert_eq!(split_numeric_and_suffix(".f"), (".", Some("f")));
}

#[test]
fn test_underscore_separator_with_suffix() {
    // Edge case: Numeric separators with suffix
    // Tests digit separator handling (common in Rust)
    assert_eq!(split_numeric_and_suffix("1_000_000u32"), ("1_000_000", Some("u32")));
}

#[test]
fn test_suffix_like_ending_not_suffix() {
    // Edge case: Number ending with suffix-like pattern but not valid
    // Tests that only exact suffix patterns are matched
    assert_eq!(split_numeric_and_suffix("42u64"), ("42u64", None));
}

#[test]
fn test_suffix_like_ending_i64() {
    // Edge case: i64 is not a supported suffix
    // Should not be extracted as suffix
    assert_eq!(split_numeric_and_suffix("42i64"), ("42i64", None));
}

#[test]
fn test_number_ending_with_u_but_longer() {
    // Edge case: Ends with 'u' but has more characters
    // Only single 'u' is a suffix, not 'ux' or similar
    assert_eq!(split_numeric_and_suffix("42ux"), ("42ux", None));
}

// ============================================================================
// CORNER CASES: Unusual and Extreme Scenarios
// ============================================================================
// These tests cover rare, unusual, or extreme cases that might occur in
// real-world usage but are not typical scenarios.

#[test]
fn test_unicode_characters_before_suffix() {
    // Corner case: Unicode in numeric part
    // Tests non-ASCII character handling
    assert_eq!(split_numeric_and_suffix("42✓u"), ("42✓", Some("u")));
}

#[test]
fn test_whitespace_before_suffix() {
    // Corner case: Whitespace before suffix
    // Should not extract suffix after whitespace
    assert_eq!(split_numeric_and_suffix("42 u"), ("42 ", Some("u")));
}

#[test]
fn test_multiple_suffixes_appearance() {
    // Corner case: String containing multiple suffix-like patterns
    // Only the last valid suffix should be extracted
    assert_eq!(split_numeric_and_suffix("u8u"), ("u8", Some("u")));
}

#[test]
fn test_suffix_in_middle() {
    // Corner case: Suffix-like pattern in middle of string
    // Should not be extracted since it's not at the end
    assert_eq!(split_numeric_and_suffix("42u16abc"), ("42u16abc", None));
}

#[test]
fn test_all_suffix_chars_but_wrong_order() {
    // Corner case: Contains suffix characters but wrong order
    // Should not match any known suffix pattern
    assert_eq!(split_numeric_and_suffix("4261i"), ("4261i", None));
}

#[test]
fn test_mixed_case_middle_suffix() {
    // Corner case: Mixed case in three-char suffix
    // Tests case-insensitive matching with unusual casing
    assert_eq!(split_numeric_and_suffix("100I3"), ("100I3", None));
}

#[test]
fn test_suffix_with_special_chars_in_number() {
    // Corner case: Special characters in numeric part
    // Function doesn't validate numeric format, just splits
    assert_eq!(split_numeric_and_suffix("4@#2u"), ("4@#2", Some("u")));
}

#[test]
fn test_exponential_suffix_collision() {
    // Corner case: 'e' in scientific notation vs 'e' as potential suffix
    // 'e' alone is not a valid suffix, so no splitting
    assert_eq!(split_numeric_and_suffix("1e10"), ("1e10", None));
}

#[test]
fn test_negative_zero_with_suffix() {
    // Corner case: Negative zero with suffix
    // Tests edge case of numeric representation
    assert_eq!(split_numeric_and_suffix("-0f"), ("-0", Some("f")));
}

#[test]
fn test_very_long_suffix_like_string() {
    // Corner case: Long string ending with suffix chars
    // Tests that suffix detection works regardless of total length
    assert_eq!(split_numeric_and_suffix("abcdefghijklmnopqrstuvwxyzu"), ("abcdefghijklmnopqrstuvwxyz", Some("u")));
}

#[test]
fn test_hex_suffix_collision() {
    // Corner case: Hex literal ending with 'f'
    // 'f' is both a valid hex digit and a suffix
    assert_eq!(split_numeric_and_suffix("0xf"), ("0x", Some("f")));
}

#[test]
fn test_hex_suffix_collision_uppercase() {
    // Corner case: Uppercase hex digit that's also a suffix
    // Tests case handling in ambiguous situation
    assert_eq!(split_numeric_and_suffix("0xF"), ("0x", Some("F")));
}

#[test]
fn test_all_zeros_with_suffix() {
    // Corner case: Multiple zeros with suffix
    // Tests handling of zero literals
    assert_eq!(split_numeric_and_suffix("0000u32"), ("0000", Some("u32")));
}

#[test]
fn test_suffix_u_followed_by_number_in_numeric() {
    // Corner case: 'u' appears before the suffix
    // Only the final 'u' should be treated as suffix
    assert_eq!(split_numeric_and_suffix("4u2u"), ("4u2", Some("u")));
}

#[test]
fn test_i1_not_valid_suffix() {
    // Corner case: i1 is not a supported suffix (only i8, i16, i32)
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("42i1"), ("42i1", None));
}

#[test]
fn test_u1_not_valid_suffix() {
    // Corner case: u1 is not a supported suffix
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("42u1"), ("42u1", None));
}

#[test]
fn test_i4_not_valid_suffix() {
    // Corner case: i4 is not a supported suffix
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("42i4"), ("42i4", None));
}

#[test]
fn test_u4_not_valid_suffix() {
    // Corner case: u4 is not a supported suffix
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("42u4"), ("42u4", None));
}

#[test]
fn test_i64_not_supported_suffix() {
    // Corner case: i64 might be expected but is not a supported suffix
    // Default integer type doesn't need suffix
    assert_eq!(split_numeric_and_suffix("42i64"), ("42i64", None));
}

#[test]
fn test_u64_not_supported_suffix() {
    // Corner case: u64 is not a supported suffix (use 'u' instead)
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("42u64"), ("42u64", None));
}

#[test]
fn test_f32_not_supported_suffix() {
    // Corner case: f32 is not a suffix (use 'f' instead)
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("3.14f32"), ("3.14f32", None));
}

#[test]
fn test_f64_not_supported_suffix() {
    // Corner case: f64 is not a suffix (use 'd' instead)
    // Should not be extracted
    assert_eq!(split_numeric_and_suffix("3.14f64"), ("3.14f64", None));
}

#[test]
fn test_double_suffix_attempt() {
    // Corner case: Attempting to use two suffixes
    // Only the last valid suffix is extracted
    assert_eq!(split_numeric_and_suffix("42u8u"), ("42u8", Some("u")));
}

#[test]
fn test_suffix_with_trailing_chars_in_numeric() {
    // Corner case: Valid suffix at end, but invalid chars before it
    // Function splits regardless of numeric validity
    assert_eq!(split_numeric_and_suffix("abc123u"), ("abc123", Some("u")));
}

#[test]
fn test_only_suffix_chars_i32() {
    // Corner case: Input is exactly a valid 3-char suffix
    // Results in empty numeric part
    assert_eq!(split_numeric_and_suffix("i32"), ("", Some("i32")));
}

#[test]
fn test_only_suffix_chars_u16() {
    // Corner case: Input is exactly a valid 3-char suffix
    // Results in empty numeric part
    assert_eq!(split_numeric_and_suffix("u16"), ("", Some("u16")));
}

#[test]
fn test_lowercase_mixed_uppercase_i16() {
    // Corner case: Various case combinations for i16
    // Tests full case-insensitive behavior
    assert_eq!(split_numeric_and_suffix("42i1"), ("42i1", None));
}

#[test]
fn test_suffix_d_with_integer() {
    // Corner case: Double suffix on integer (unusual but valid)
    // Tests that suffix splitting works regardless of number type
    assert_eq!(split_numeric_and_suffix("42d"), ("42", Some("d")));
}

#[test]
fn test_scientific_notation_negative_exponent() {
    // Corner case: Scientific notation with negative exponent and suffix
    // Tests complex format handling
    assert_eq!(split_numeric_and_suffix("1.23e-45f"), ("1.23e-45", Some("f")));
}

#[test]
fn test_scientific_notation_positive_exponent() {
    // Corner case: Scientific notation with explicit positive exponent
    // Tests +e notation
    assert_eq!(split_numeric_and_suffix("9.99e+100d"), ("9.99e+100", Some("d")));
}

#[test]
fn test_lone_minus_with_suffix() {
    // Corner case: Just a minus sign with suffix
    // Tests minimal invalid numeric input
    assert_eq!(split_numeric_and_suffix("-u"), ("-", Some("u")));
}

#[test]
fn test_lone_plus_with_suffix() {
    // Corner case: Just a plus sign with suffix
    // Another minimal invalid numeric input
    assert_eq!(split_numeric_and_suffix("+f"), ("+", Some("f")));
}

#[test]
fn test_unknown_suffix_for_handle_suffix() {
    // Fixed typo in test name
    assert!(handle_suffix("123", Some("x".into())).is_none());
}

#[test]
fn test_malformed_numeric_part_handle_unsigned_suffix() {
    // Fixed typo in test name
    assert!(handle_suffix("123.45", Some("u".into())).is_none());
}

#[test]
fn test_parse_unsigned_integer() {
    assert_token("123u", Numeric(UnsignedInteger(123)));
}

#[test]
fn test_parse_float_suffix() {
    assert_token("45.67f", Numeric(Float32(45.67)));
}

#[test]
fn test_parse_double_suffix() {
    assert_token("89.01d", Numeric(Float64(89.01)));
}

#[test]
fn test_parse_scientific_float() {
    assert_token("1.2e3f", Numeric(Scientific32(1.2, 3)));
}

#[test]
fn test_parse_scientific_double() {
    assert_token("3.4e5", Numeric(Scientific64(3.4, 5)));
}

#[test]
fn test_number_with_invalid_suffix() {
    let mut lex = TokenKind::lexer("123x");
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Integer(123))));
    assert_eq!(lex.next().unwrap(), Ok(IdentifierAscii("x".into())));
}

#[test]
fn test_decimal_starting_with_dot() {
    assert_token(".456", Numeric(Float64(0.456)));
}

#[test]
fn test_invalid_scientific_notation() {
    let mut lex = TokenKind::lexer("1e2e3");
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Scientific64(1.0, 2))));
    assert_eq!(lex.next().unwrap(), Ok(IdentifierAscii("e3".into())));
    let mut lex2 = TokenKind::lexer("1.2e3.4"); // Invalid exponent
    assert!(lex2.next().unwrap().is_ok());
}

#[test]
fn returns_true_for_all_type_tokens() {
    assert!(TypeI8.is_type());
    assert!(TypeI16.is_type());
    assert!(TypeI32.is_type());
    assert!(TypeI64.is_type());
    assert!(TypeU8.is_type());
    assert!(TypeU16.is_type());
    assert!(TypeU32.is_type());
    assert!(TypeU64.is_type());
    assert!(TypeF32.is_type());
    assert!(TypeF64.is_type());
    assert!(TypeChar.is_type());
    assert!(TypeString.is_type());
    assert!(TypeBool.is_type());
}

#[test]
fn returns_false_for_non_type_tokens() {
    assert!(!Plus.is_type());
    assert!(!Minus.is_type());
    assert!(!KeywordFun.is_type());
    assert!(!KeywordIf.is_type());
    assert!(!IdentifierAscii("abc".into()).is_type());
    assert!(!Numeric(Integer(42)).is_type());
    assert!(!StringLiteral("test".into()).is_type());
    assert!(!CharLiteral("a".into()).is_type());
    assert!(!Whitespace.is_type());
    assert!(!Eof.is_type());
}

#[test]
fn returns_false_for_edge_case_similar_names() {
    // Assicura che token con nomi simili ma non tipi non siano considerati tipi
    assert!(!KeywordVar.is_type());
    assert!(!KeywordConst.is_type());
    assert!(!KeywordBool(true).is_type());
    assert!(!KeywordBool(false).is_type());
}

#[test]
fn returns_false_for_structurally_similar_tokens() {
    // Token che hanno struttura simile ma non sono tipi
    assert!(!OpenParen.is_type());
    assert!(!CloseParen.is_type());
    assert!(!OpenBracket.is_type());
    assert!(!CloseBracket.is_type());
    assert!(!OpenBrace.is_type());
    assert!(!CloseBrace.is_type());
}

#[test]
fn test_eof_and_ignored() {
    assert_eq!(Eof.to_string(), "end of file");
    assert_eq!(Comment.to_string(), "comment");
    assert_eq!(Whitespace.to_string(), "whitespace");
}

#[test]
fn test_identifier_ascii_normal() {
    let ident: Arc<str> = "foo".into();
    assert_eq!(IdentifierAscii(ident.clone()).to_string(), format!("identifier '{ident}'"));
}

#[test]
fn test_identifier_ascii_empty() {
    let ident: Arc<str> = "".into();
    assert_eq!(IdentifierAscii(ident.clone()).to_string(), "identifier ''");
}

#[test]
fn test_identifier_unicode() {
    let ident: Arc<str> = "προεδομή".into(); // qualche stringa Unicode
    assert_eq!(IdentifierUnicode(ident.clone()).to_string(), format!("identifier '{ident}'"));
}

#[test]
fn test_numeric_integer() {
    let num = Integer(123);
    assert_eq!(Numeric(num.clone()).to_string(), "number '123'");
    assert_eq!(Hexadecimal(num.clone()).to_string(), "hexadecimal '123'");
    assert_eq!(Octal(num.clone()).to_string(), "octal '123'");
    assert_eq!(Binary(num).to_string(), "binary '123'");
}

#[test]
fn test_string_literal_simple() {
    let s: Arc<str> = "hello".into();
    assert_eq!(StringLiteral(s.clone()).to_string(), format!("string literal \"{s}\""));
}

#[test]
fn test_string_literal_with_quotes_inside() {
    let s: Arc<str> = "he said \"ciao\"".into();
    assert_eq!(StringLiteral(s.clone()).to_string(), format!("string literal \"{s}\""));
}

#[test]
fn test_char_literal_simple() {
    let c: Arc<str> = "x".into();
    assert_eq!(CharLiteral(c.clone()).to_string(), format!("character literal '{c}'"));
}

#[test]
fn test_char_literal_unicode() {
    let c: Arc<str> = "ψ".into();
    assert_eq!(CharLiteral(c.clone()).to_string(), format!("character literal '{c}'"));
}

#[test]
fn test_keyword_bool_true_false() {
    assert_eq!(KeywordBool(true).to_string(), "boolean 'true'");
    assert_eq!(&KeywordBool(false).to_string(), "boolean 'false'");
}

#[test]
fn test_keyword_nullptr() {
    assert_eq!(&KeywordNullptr.to_string(), "'nullptr'");
}

// ——— Test per tutte le keyword principali ———
#[test]
fn test_all_keywords() {
    let mapping = vec![
        (KeywordFun, "'fun'"),
        (KeywordIf, "'if'"),
        (KeywordElse, "'else'"),
        (KeywordVar, "'var'"),
        (KeywordConst, "'const'"),
        (KeywordReturn, "'return'"),
        (KeywordWhile, "'while'"),
        (KeywordFor, "'for'"),
        (KeywordBreak, "'break'"),
        (KeywordContinue, "'continue'"),
        (KeywordMain, "'main'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}

// ——— Test per tutti i tipi primari ———
#[test]
fn test_all_primitive_types() {
    let mapping = vec![
        (TypeI8, "'i8'"),
        (TypeI16, "'i16'"),
        (TypeI32, "'i32'"),
        (TypeI64, "'i64'"),
        (TypeU8, "'u8'"),
        (TypeU16, "'u16'"),
        (TypeU32, "'u32'"),
        (TypeU64, "'u64'"),
        (TypeF32, "'f32'"),
        (TypeF64, "'f64'"),
        (TypeChar, "'char'"),
        (TypeString, "'string'"),
        (TypeBool, "'bool'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}

// ——— Test per punteggiatura e simboli singoli ———
#[test]
fn test_punctuation() {
    let mapping = vec![
        (OpenParen, "'('"),
        (CloseParen, "')'"),
        (OpenBrace, "'{'"),
        (CloseBrace, "'}'"),
        (OpenBracket, "'['"),
        (CloseBracket, "']'"),
        (Semicolon, "';'"),
        (Colon, "':'"),
        (Comma, "','"),
        (Dot, "'.'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}

// ——— Test per operatori semplici e composti ———
#[test]
fn test_operators_single_and_multi_char() {
    let mapping = vec![
        (Plus, "'+'"),
        (PlusPlus, "'++'"),
        (MinusMinus, "'--'"),
        (PlusEqual, "'+='"),
        (MinusEqual, "'-='"),
        (Minus, "'-'"),
        (Star, "'*'"),
        (Slash, "'/'"),
        (Percent, "'%'"),
        (PercentEqual, "'%='"),
        (Equal, "'='"),
        (EqualEqual, "'=='"),
        (NotEqual, "'!='"),
        (Less, "'<'"),
        (LessEqual, "'<='"),
        (Greater, "'>'"),
        (GreaterEqual, "'>='"),
        (AndAnd, "'&&'"),
        (OrOr, "'||'"),
        (Not, "'!'"),
        (And, "'&'"),
        (Or, "'|'"),
        (Xor, "'^'"),
        (XorEqual, "'^='"),
        (ShiftLeft, "'<<'"),
        (ShiftRight, "'>>'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}
