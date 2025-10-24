//! Immediate value types and helpers.
//!
//! These types model sized immediate (constant) operands used by
//! instructions. Each variant encodes width (8/16/32/64 bits) and whether
//! the value is signed or unsigned. Utility methods provide size queries and
//! conversions to common integer representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Immediate {
    /// 8-bit signed immediate (i8).
    Imm8(i8),
    /// 8-bit unsigned immediate (u8).
    Imm8u(u8),
    /// 16-bit signed immediate (i16).
    Imm16(i16),
    /// 16-bit unsigned immediate (u16).
    Imm16u(u16),
    /// 32-bit signed immediate (i32).
    Imm32(i32),
    /// 32-bit unsigned immediate (u32).
    Imm32u(u32),
    /// 64-bit signed immediate (i64).
    Imm64(i64),
    /// 64-bit unsigned immediate (u64).
    Imm64u(u64),
}

impl Immediate {
    /// Ottiene la dimensione in bit dell'immediato
    pub fn size_bits(&self) -> usize {
        match self {
            Self::Imm8(_) | Self::Imm8u(_) => 8,
            Self::Imm16(_) | Self::Imm16u(_) => 16,
            Self::Imm32(_) | Self::Imm32u(_) => 32,
            Self::Imm64(_) | Self::Imm64u(_) => 64,
        }
    }

    /// Returns the size of the immediate in bytes.
    ///
    /// Inputs: &self
    /// Outputs: usize — number of bytes (1, 2, 4, or 8).
    /// Side effects: none.
    pub fn size_bytes(&self) -> usize {
        self.size_bits() / 8
    }

    /// Converte l'immediato a i64
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::Imm8(v) => *v as i64,
            Self::Imm8u(v) => *v as i64,
            Self::Imm16(v) => *v as i64,
            Self::Imm16u(v) => *v as i64,
            Self::Imm32(v) => *v as i64,
            Self::Imm32u(v) => *v as i64,
            Self::Imm64(v) => *v,
            Self::Imm64u(v) => *v as i64,
        }
    }

    /// Convert the immediate to a signed 64-bit integer (i64).
    ///
    /// Inputs: &self
    /// Outputs: i64 — the value as a signed 64-bit integer. For unsigned
    /// variants this performs a widening cast. No overflow can occur because
    /// the target type is the largest supported width.
    /// Side effects: none.
    pub fn as_u64(&self) -> u64 {
        match self {
            Self::Imm8(v) => *v as u64,
            Self::Imm8u(v) => *v as u64,
            Self::Imm16(v) => *v as u64,
            Self::Imm16u(v) => *v as u64,
            Self::Imm32(v) => *v as u64,
            Self::Imm32u(v) => *v as u64,
            Self::Imm64(v) => *v as u64,
            Self::Imm64u(v) => *v,
        }
    }

    /// Verifica se l'immediato è con segno
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Imm8(_) | Self::Imm16(_) | Self::Imm32(_) | Self::Imm64(_))
    }
    /// Check whether the immediate fits in the given signed bit width.
    ///
    /// Inputs:
    /// - `bits`: target width in bits (commonly 8, 16, 32, or 64).
    ///
    /// Outputs: `bool` — true if the immediate value can be represented in the
    /// signed range of the requested width. For 64 bits this always returns
    /// `true` (since all supported values are representable in i64).
    ///
    /// Side effects: none.
    pub fn fits_in(&self, bits: usize) -> bool {
        match bits {
            8 => {
                let val = self.as_i64();
                val >= i8::MIN as i64 && val <= i8::MAX as i64
            }
            16 => {
                let val = self.as_i64();
                val >= i16::MIN as i64 && val <= i16::MAX as i64
            }
            32 => {
                let val = self.as_i64();
                val >= i32::MIN as i64 && val <= i32::MAX as i64
            }
            64 => true,
            _ => false,
        }
    }
}

/// Conversion from `i8` into `Immediate::Imm8`.
///
/// Purpose: provide a convenient, idiomatic way to create an 8-bit signed
/// immediate from an `i8` value.
///
/// Inputs: `v: i8`
/// Outputs: `Immediate::Imm8(v)`
/// Side effects: none.
impl From<i8> for Immediate {
    fn from(v: i8) -> Self {
        Self::Imm8(v)
    }
}

/// Conversion from `u8` into `Immediate::Imm8u`.
///
/// Purpose: create an 8-bit unsigned immediate from a `u8` value.
/// Inputs: `v: u8` — the value to wrap.
/// Outputs: `Immediate::Imm8u(v)`.
/// Side effects: none.
impl From<u8> for Immediate {
    fn from(v: u8) -> Self {
        Self::Imm8u(v)
    }
}

/// Conversion from `i16` into `Immediate::Imm16`.
///
/// Inputs: `v: i16`.
/// Outputs: `Immediate::Imm16(v)`.
/// Side effects: none.
impl From<i16> for Immediate {
    fn from(v: i16) -> Self {
        Self::Imm16(v)
    }
}

/// Conversion from `u16` into `Immediate::Imm16u`.
///
/// Inputs: `v: u16`.
/// Outputs: `Immediate::Imm16u(v)`.
/// Side effects: none.
impl From<u16> for Immediate {
    fn from(v: u16) -> Self {
        Self::Imm16u(v)
    }
}

/// Conversion from `i32` into `Immediate::Imm32`.
///
/// Inputs: `v: i32`.
/// Outputs: `Immediate::Imm32(v)`.
/// Side effects: none.
impl From<i32> for Immediate {
    fn from(v: i32) -> Self {
        Self::Imm32(v)
    }
}

/// Conversion from `u32` into `Immediate::Imm32u`.
///
/// Inputs: `v: u32`.
/// Outputs: `Immediate::Imm32u(v)`.
/// Side effects: none.
impl From<u32> for Immediate {
    fn from(v: u32) -> Self {
        Self::Imm32u(v)
    }
}

/// Conversion from `i64` into `Immediate::Imm64`.
///
/// Inputs: `v: i64`.
/// Outputs: `Immediate::Imm64(v)`.
/// Side effects: none.
impl From<i64> for Immediate {
    fn from(v: i64) -> Self {
        Self::Imm64(v)
    }
}

/// Conversion from `u64` into `Immediate::Imm64u`.
///
/// Inputs: `v: u64`.
/// Outputs: `Immediate::Imm64u(v)`.
/// Side effects: none.

impl From<u64> for Immediate {
    fn from(v: u64) -> Self {
        Self::Imm64u(v)
    }
}

/// Textual formatting for immediates.
///
/// Purpose: provide a human-friendly representation for immediate values.
/// - Signed variants are printed as decimal numbers (e.g. `-5`).
/// - Unsigned variants are printed as zero-prefixed hexadecimal with width
///   matching the underlying storage (e.g. `0x0f`, `0x0000000f`).
///
/// Inputs: `&self` and a `Formatter`.
/// Outputs: formatted text written to the formatter; returns `fmt::Result`.
/// Side effects: none beyond writing to the provided formatter.
impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imm8(v) => write!(f, "{}", v),
            Self::Imm8u(v) => write!(f, "0x{:02x}", v),
            Self::Imm16(v) => write!(f, "{}", v),
            Self::Imm16u(v) => write!(f, "0x{:04x}", v),
            Self::Imm32(v) => write!(f, "{}", v),
            Self::Imm32u(v) => write!(f, "0x{:08x}", v),
            Self::Imm64(v) => write!(f, "{}", v),
            Self::Imm64u(v) => write!(f, "0x{:016x}", v),
        }
    }
}
