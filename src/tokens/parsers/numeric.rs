// src/tokens/parsers/numeric.rs
//! Core numeric literal parsing functionality.
//!
//! This module contains the main parsing logic for decimal numeric literals,
//! including integers, floating-point numbers, and scientific notation.

use super::suffix::{handle_suffix, split_numeric_and_suffix};
use crate::tokens::number::Number;
use crate::tokens::token_kind::TokenKind;

/// Parses a numeric literal token into a structured [`Number`] representation.
///
/// This function is called by the Logos lexer when it encounters a numeric literal.
/// It handles all numeric formats including:
/// - Pure integers (e.g., `42`)
/// - Floating-point numbers (e.g., `3.14`)
/// - Scientific notation (e.g., `6.022e23`)
/// - Type suffixes (e.g., `42u8`, `3.14f`, `100i16`)
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number)` - Successfully parsed numeric literal
/// * `None` - Invalid numeric format or overflow/underflow
pub fn parse_number(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (numeric_part, suffix) = split_numeric_and_suffix(slice);
    handle_suffix(numeric_part, suffix)
}

/// Helper function to parse integer literals with generic type support.
///
/// This function validates that the numeric string represents a valid integer
/// (no decimal point or exponent), then attempts to parse it into the target type.
///
/// # Type Parameters
///
/// * `T` - The target integer type (must implement `FromStr`)
///
/// # Arguments
///
/// * `numeric_part` - The numeric string without suffix
/// * `map_fn` - Function to wrap the parsed value in a [`Number`] variant
///
/// # Returns
///
/// * `Some(Number)` - Successfully parsed and wrapped integer
/// * `None` - Invalid format or value out of range for type `T`
pub fn parse_integer<T>(numeric_part: &str, map_fn: fn(T) -> Number) -> Option<Number>
where
    T: std::str::FromStr,
{
    if is_valid_integer_literal(numeric_part) { numeric_part.parse::<T>().ok().map(map_fn) } else { None }
}

/// Validates that a string represents a pure integer literal.
///
/// A valid integer literal must:
/// - Contain only ASCII digits (0-9)
/// - Have no decimal point (`.`)
/// - Have no exponent marker (`e` or `E`)
/// - Have no sign character (handled as separate token by lexer)
///
/// # Arguments
///
/// * `numeric_part` - Numeric string to validate
///
/// # Returns
///
/// `true` if the string is a valid integer literal, `false` otherwise
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::parsers::numeric::is_valid_integer_literal;
///
/// assert!(is_valid_integer_literal("42"));
/// assert!(is_valid_integer_literal("1234567890"));
/// assert!(!is_valid_integer_literal("3.14"));      // Has decimal point
/// assert!(!is_valid_integer_literal("6.022e23"));  // Has exponent
/// assert!(!is_valid_integer_literal("-42"));       // Has sign
/// ```
#[must_use]
pub fn is_valid_integer_literal(numeric_part: &str) -> bool {
    if numeric_part.is_empty() {
        return false;
    }
    if numeric_part.bytes().any(|b| b == b'.' || b == b'e' || b == b'E') {
        return false;
    }
    numeric_part.bytes().all(|b| b.is_ascii_digit())
}

/// Parses numeric strings with 32-bit float suffix ('f').
///
/// Handles both regular floating-point notation and scientific notation,
/// producing either [`Number::Float32`] or [`Number::Scientific32`] respectively.
///
/// # Arguments
///
/// * `numeric_part` - Numeric string without the 'f' suffix
///
/// # Returns
///
/// * `Some(Number::Float32)` - For regular float literals
/// * `Some(Number::Scientific32)` - For scientific notation
/// * `None` - If parsing fails
#[must_use]
pub fn handle_float_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, true).or_else(|| numeric_part.parse::<f32>().ok().map(Number::Float32))
}

/// Parses numeric strings with default or 'd' suffix.
///
/// Implements the default type inference rules:
/// - Integer literals (no decimal/exponent) → i64
/// - Floating-point literals → f64
/// - Scientific notation → Scientific64
///
/// # Arguments
///
/// * `numeric_part` - Numeric string without suffix (or with 'd' suffix removed)
///
/// # Returns
///
/// * `Some(Number::Integer)` - For integer literals
/// * `Some(Number::Float64)` - For floating-point literals
/// * `Some(Number::Scientific64)` - For scientific notation
/// * `None` - If parsing fails
#[must_use]
pub fn handle_default_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, false).or_else(|| handle_non_scientific(numeric_part))
}

/// Parses non-scientific notation numbers (integers and simple floats).
///
/// Determines the appropriate type based on the presence of a decimal point:
/// Parses non-scientific notation numbers (integers and simple floats).
///
/// Determines the appropriate type based on the presence of a decimal point:
/// - No decimal point → i64 integer
/// - Has decimal point → f64 float
///
/// # Arguments
///
/// * `numeric_part` - Numeric string to parse
///
/// # Returns
///
/// * `Some(Number::Integer)` - For literals without decimal point
/// * `Some(Number::Float64)` - For literals with decimal point
/// * `None` - If parsing fails (overflow, underflow, or invalid format)
///
/// # Examples
///
/// ```
/// # use jsavrs::tokens::parsers::numeric::handle_non_scientific;
/// # use jsavrs::tokens::number::Number;
/// let int_result = handle_non_scientific("42");
/// assert!(matches!(int_result, Some(Number::Integer(_))));
///
/// let float_result = handle_non_scientific("3.14");
/// assert!(matches!(float_result, Some(Number::Float64(_))));
/// ```
///
/// # Panics
///
/// This function does not panic.
#[must_use]
pub fn handle_non_scientific(numeric_part: &str) -> Option<Number> {
    if numeric_part.contains('.') {
        numeric_part.parse::<f64>().ok().map(Number::Float64)
    } else {
        numeric_part.parse::<i64>().ok().map(Number::Integer)
    }
}

/// Parses scientific notation numbers (e.g., "6.022e23").
///
/// Scientific notation format: `base[e|E][+|-]exponent`
/// where the base can be an integer or floating-point number.
///
/// # Arguments
///
/// * `s` - Full numeric string in scientific notation
/// * `is_f32` - If `true`, parses as 32-bit float; if `false`, as 64-bit float
///
/// # Returns
///
/// * `Some(Number::Scientific32)` - For 32-bit scientific notation
/// * `Some(Number::Scientific64)` - For 64-bit scientific notation
/// * `None` - If not in scientific notation or parsing fails
///
/// # Format Details
///
/// - Exponent marker: `e` or `E` (case-insensitive)
/// - Optional sign: `+` or `-` before exponent
/// - Base: can be integer or floating-point
/// - Exponent: must be valid i32 integer
#[must_use]
pub fn parse_scientific(s: &str, is_f32: bool) -> Option<Number> {
    let pos = s.find(['e', 'E'])?;
    let (base_str, exp_str) = s.split_at(pos);
    let exp = exp_str[1..].parse::<i32>().ok()?;

    if is_f32 {
        let base = base_str.parse::<f32>().ok()?;
        Some(Number::Scientific32(base, exp))
    } else {
        let base = base_str.parse::<f64>().ok()?;
        Some(Number::Scientific64(base, exp))
    }
}
