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
/// # Implementation Notes
///
/// - Strips the 2-character prefix (`#b`, `#o`, or `#x`)
/// - Checks for optional trailing `u` or `U` suffix
/// - Uses `i64::from_str_radix` or `u64::from_str_radix` for parsing
#[inline]
pub fn parse_base_number(radix: u32, lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (_, num_part) = slice.split_at(2); // Remove prefix ("#b", "#o", or "#x")
    let (num_str, suffix_u) = match num_part.chars().last() {
        Some('u') | Some('U') => (&num_part[..num_part.len() - 1], true),
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
pub fn parse_hex(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(16, lex)
}