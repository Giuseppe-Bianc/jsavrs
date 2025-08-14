// src/tokens/number.rs
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents numeric literals in various formats.
///
/// This enum captures different representations of numbers found in source code,
/// preserving their original format for precise error reporting and accurate
/// processing during compilation.
#[derive(Debug, Clone)]
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


impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::I8(a), Number::I8(b)) => a == b,
            (Number::I16(a), Number::I16(b)) => a == b,
            (Number::I32(a), Number::I32(b)) => a == b,
            (Number::Integer(a), Number::Integer(b)) => a == b,
            (Number::U8(a), Number::U8(b)) => a == b,
            (Number::U16(a), Number::U16(b)) => a == b,
            (Number::U32(a), Number::U32(b)) => a == b,
            (Number::UnsignedInteger(a), Number::UnsignedInteger(b)) => a == b,
            // For floating-point values, we use bitwise equality
            (Number::Float32(a), Number::Float32(b)) => a.to_bits() == b.to_bits(),
            (Number::Float64(a), Number::Float64(b)) => a.to_bits() == b.to_bits(),
            (Number::Scientific32(a_base, a_exp), Number::Scientific32(b_base, b_exp)) => {
                a_base.to_bits() == b_base.to_bits() && a_exp == b_exp
            }
            (Number::Scientific64(a_base, a_exp), Number::Scientific64(b_base, b_exp)) => {
                a_base.to_bits() == b_base.to_bits() && a_exp == b_exp
            }
            _ => false, // Different variants are not equal
        }
    }
}

impl Eq for Number {}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Number::I8(i) => i.hash(state),
            Number::I16(i) => i.hash(state),
            Number::I32(i) => i.hash(state),
            Number::Integer(i) => i.hash(state),
            Number::U8(u) => u.hash(state),
            Number::U16(u) => u.hash(state),
            Number::U32(u) => u.hash(state),
            Number::UnsignedInteger(u) => u.hash(state),
            // For floating-point values, we hash their bit representation
            Number::Float32(f) => f.to_bits().hash(state),
            Number::Float64(f) => f.to_bits().hash(state),
            Number::Scientific32(base, exp) => {
                base.to_bits().hash(state);
                exp.hash(state);
            }
            Number::Scientific64(base, exp) => {
                base.to_bits().hash(state);
                exp.hash(state);
            }
        }
    }
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
    /// assert_eq!(Number::I8(-42).to_string(), "-42i8");
    /// assert_eq!(Number::U32(123456).to_string(), "123456u32");
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
