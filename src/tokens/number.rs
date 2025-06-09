// src/tokens/number.rs
use std::fmt;

/// Represents numeric literals in various formats.
///
/// This enum captures different representations of numbers found in source code,
/// preserving their original format for precise error reporting and accurate
/// processing during compilation.
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    /// Signed 8-bit integer literal (e.g., `-42i8`)
    I8(i8),

    /// Signed 16-bit integer literal (e.g., `1234i16`)
    I16(i16),

    /// Signed 32-bit integer literal (e.g., `123456i32`)
    I32(i32),

    /// Signed 64-bit integer literal (e.g., `-42`, `1234`)
    Integer(i64),

    /// Unsigned 8-bit integer literal (e.g., `42u8`)
    U8(u8),

    /// Unsigned 16-bit integer literal (e.g., `1234u16`)
    U16(u16),

    /// Unsigned 32-bit integer literal (e.g., `123456u32`)
    U32(u32),

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
    /// assert_eq!(Number::I8(-42).to_string(), "-42");
    /// assert_eq!(Number::U32(123456).to_string(), "123456");
    /// assert_eq!(Number::Float64(3.14159).to_string(), "3.14159");
    /// assert_eq!(Number::Scientific32(6.022, 23).to_string(), "6.022e23");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::I8(i) => write!(f, "{i}i8"),
            Number::I16(i) => write!(f, "{i}i16"),
            Number::I32(i) => write!(f, "{i}i32"),
            Number::Integer(i) => write!(f, "{i}"),
            Number::U8(u) => write!(f, "{u}u8"),
            Number::U16(u) => write!(f, "{u}u16"),
            Number::U32(u) => write!(f, "{u}u32"),
            Number::UnsignedInteger(u) => write!(f, "{u}"),
            Number::Float32(flt) => write!(f, "{flt}"),
            Number::Float64(flt) => write!(f, "{flt}"),
            Number::Scientific32(base, exp) => write!(f, "{base}e{exp}"),
            Number::Scientific64(base, exp) => write!(f, "{base}e{exp}"),
        }
    }
}
