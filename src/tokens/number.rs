// src/tokens/number.rs
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents numeric literals in various formats.
///
/// This enum captures different representations of numbers found in source code,
/// preserving their original format for precise error reporting and accurate
/// processing during compilation.
///
/// # Type Safety
///
/// Each variant enforces type-specific bounds at the lexical level, allowing
/// early detection of overflow/underflow errors during tokenization rather than
/// deferring them to later compilation phases.
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::number::Number;
///
/// // Integer literals
/// let small = Number::I8(-42);
/// let large = Number::Integer(9223372036854775807);
///
/// // Unsigned literals
/// let byte = Number::U8(255);
/// let unsigned = Number::UnsignedInteger(42);
///
/// // Floating-point literals
/// let pi = Number::Float64(3.14159);
/// let planck = Number::Scientific64(6.62607015, -34);
/// ```
#[derive(Debug, Clone)]
pub enum Number {
    /// Signed 8-bit integer literal (e.g., `-42i8`)
    ///
    /// Range: -128 to 127
    I8(i8),

    /// Signed 16-bit integer literal (e.g., `1234i16`)
    ///
    /// Range: -32,768 to 32,767
    I16(i16),

    /// Signed 32-bit integer literal (e.g., `123456i32`)
    ///
    /// Range: -2,147,483,648 to 2,147,483,647
    I32(i32),

    /// Signed 64-bit integer literal (e.g., `-42`, `1234`)
    ///
    /// Default integer type when no suffix is specified.
    /// Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    Integer(i64),

    /// Unsigned 8-bit integer literal (e.g., `42u8`)
    ///
    /// Range: 0 to 255
    U8(u8),

    /// Unsigned 16-bit integer literal (e.g., `1234u16`)
    ///
    /// Range: 0 to 65,535
    U16(u16),

    /// Unsigned 32-bit integer literal (e.g., `123456u32`)
    ///
    /// Range: 0 to 4,294,967,295
    U32(u32),

    /// Unsigned 64-bit integer literal (e.g., `42u`, `1234u`)
    ///
    /// Range: 0 to 18,446,744,073,709,551,615
    UnsignedInteger(u64),

    /// 32-bit floating point literal (e.g., `3.14f`, `6.022e23f`)
    ///
    /// IEEE 754 single-precision floating-point format.
    /// Provides approximately 7 decimal digits of precision.
    Float32(f32),

    /// 64-bit floating point literal (e.g., `3.14159`, `6.02214076e23`)
    ///
    /// IEEE 754 double-precision floating-point format.
    /// Default floating-point type when no suffix is specified.
    /// Provides approximately 15-17 decimal digits of precision.
    Float64(f64),

    /// Scientific notation with 32-bit base and exponent (e.g., `6.022e23f`)
    ///
    /// Stores the number in the form: base × 10^exponent
    ///
    /// # Fields
    ///
    /// * `f32` - Base value (mantissa)
    /// * `i32` - Exponent (power of 10)
    ///
    /// # Examples
    ///
    /// `6.022e23f` represents 6.022 × 10^23 (Avogadro's number)
    Scientific32(f32, i32),

    /// Scientific notation with 64-bit base and exponent (e.g., `6.02214076e23`)
    ///
    /// Stores the number in the form: base × 10^exponent
    ///
    /// # Fields
    ///
    /// * `f64` - Base value (mantissa)
    /// * `i32` - Exponent (power of 10)
    ///
    /// # Examples
    ///
    /// `6.02214076e23` represents 6.02214076 × 10^23 (precise Avogadro's number)
    Scientific64(f64, i32),
}

impl PartialEq for Number {
    /// Compares two [`Number`] values for equality.
    ///
    /// # Implementation Notes
    ///
    /// - Integer variants use standard equality comparison
    /// - Floating-point variants use bitwise equality via `to_bits()` to ensure
    ///   that NaN values are handled consistently and `-0.0` is distinguished from `+0.0`
    /// - Different variants are never equal, even if they represent the same mathematical value
    ///   (e.g., `Integer(42)` ≠ `Float64(42.0)`)
    ///
    /// # Arguments
    ///
    /// * `other` - The other [`Number`] to compare against
    ///
    /// # Returns
    ///
    /// `true` if both variants match and their values are equal, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::tokens::number::Number;
    ///
    /// assert_eq!(Number::Integer(42), Number::Integer(42));
    /// assert_ne!(Number::Integer(42), Number::Float64(42.0));
    /// assert_eq!(Number::Float64(3.14), Number::Float64(3.14));
    /// ```
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
            // For floating-point values, we use bitwise equality to handle NaN and signed zeros
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
    /// Computes a hash value for the [`Number`].
    ///
    /// # Implementation Notes
    ///
    /// - Integer variants hash their values directly
    /// - Floating-point variants hash their bit representation via `to_bits()`
    ///   to ensure consistency with the [`PartialEq`] implementation
    /// - Scientific notation variants hash both their base (as bits) and exponent
    ///
    /// # Arguments
    ///
    /// * `state` - The hasher to feed the hash data into
    ///
    /// # Panics
    ///
    /// This method does not panic.
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash discriminant first for better distribution
        core::mem::discriminant(self).hash(state);
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
            // to ensure consistency with PartialEq
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
    /// - Type suffixes are preserved (i8, u32, etc.)
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// A [`fmt::Result`] indicating success or failure
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::tokens::number::Number;
    ///
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
