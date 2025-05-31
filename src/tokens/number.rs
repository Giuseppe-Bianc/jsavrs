// src/tokens/number.rs
use std::fmt;

/// Represents numeric literals in various formats.
///
/// This enum captures different representations of numbers found in source code,
/// preserving their original format for precise error reporting and accurate
/// processing during compilation.
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    /// Signed 64-bit integer literal (e.g., `-42`, `1234`)
    Integer(i64),

    /// Unsigned 64-bit integer literal (e.g., `42u`, `1234u`)
    UnsignedInteger(u64),

    /// 32-bit floating point literal (e.g., `3.14f`, `6.022e23f`)
    Float32(f32),

    /// 64-bit floating point literal (e.g., `3.14159`, `6.02214076e23`)
    Float64(f64),

    /// Scientific notation with 32-bit base and exponent (e.g., `6.022e23f`)
    ///
    /// Stores:
    /// - Base value (`f32`)
    /// - Exponent (`i32`)
    Scientific32(f32, i32),

    /// Scientific notation with 64-bit base and exponent (e.g., `6.02214076e23`)
    ///
    /// Stores:
    /// - Base value (`f64`)
    /// - Exponent (`i32`)
    Scientific64(f64, i32),
}

impl fmt::Display for Number {
    /// Formats the number according to its original representation.
    ///
    /// Maintains the original formatting where possible:
    /// - Integers display without decimal points
    /// - Floats display with decimal points
    /// - Scientific notation uses 'e' exponent marker
    ///
    /// # Examples
    /// ```
    /// use jsavrs::tokens::number::Number;
    /// assert_eq!(Number::Integer(-42).to_string(), "-42");
    /// assert_eq!(Number::Float64(3.14159).to_string(), "3.14159");
    /// assert_eq!(Number::Scientific32(6.022, 23).to_string(), "6.022e23");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{i}"),
            Number::UnsignedInteger(u) => write!(f, "{u}"),
            Number::Float32(flt) => write!(f, "{flt}"),
            Number::Float64(flt) => write!(f, "{flt}"),
            Number::Scientific32(base, exp) => write!(f, "{base}e{exp}"),
            Number::Scientific64(base, exp) => write!(f, "{base}e{exp}"),
        }
    }
}
