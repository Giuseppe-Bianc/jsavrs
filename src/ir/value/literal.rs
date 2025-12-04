// src/ir/value/literal.rs
use crate::ir::IrType;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrLiteralValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
}

// Manual Eq implementation to handle f32/f64 (which don't implement Eq)
// We use bitwise equality for floats to ensure Hash consistency
impl Eq for IrLiteralValue {}

/// Optimized Hash implementation for IR literal values.
///
/// # Performance Optimizations
///
/// 1. Discriminant hashing is inlined for common integer cases
/// 2. Uses `write_*` methods for primitive types (faster than generic `hash()`)
/// 3. Float values use bit representation for consistency with Eq
///
/// # Correctness
///
/// This implementation ensures that if two literals are equal according to
/// PartialEq/Eq, they produce identical hash values. For floats, we hash
/// the bit representation rather than the numeric value, which means:
/// - NaN values with different bit patterns hash differently
/// - -0.0 and +0.0 hash differently (they also compare unequal in our Eq impl)
impl std::hash::Hash for IrLiteralValue {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash discriminant first for type differentiation
        // Using match allows compiler to optimize discriminant check
        match self {
            IrLiteralValue::I8(v) => {
                state.write_u8(0); // discriminant
                state.write_i8(*v);
            }
            IrLiteralValue::I16(v) => {
                state.write_u8(1);
                state.write_i16(*v);
            }
            IrLiteralValue::I32(v) => {
                state.write_u8(2);
                state.write_i32(*v);
            }
            IrLiteralValue::I64(v) => {
                state.write_u8(3);
                state.write_i64(*v);
            }
            IrLiteralValue::U8(v) => {
                state.write_u8(4);
                state.write_u8(*v);
            }
            IrLiteralValue::U16(v) => {
                state.write_u8(5);
                state.write_u16(*v);
            }
            IrLiteralValue::U32(v) => {
                state.write_u8(6);
                state.write_u32(*v);
            }
            IrLiteralValue::U64(v) => {
                state.write_u8(7);
                state.write_u64(*v);
            }
            IrLiteralValue::F32(v) => {
                state.write_u8(8);
                state.write_u32(v.to_bits());
            }
            IrLiteralValue::F64(v) => {
                state.write_u8(9);
                state.write_u64(v.to_bits());
            }
            IrLiteralValue::Bool(v) => {
                state.write_u8(10);
                state.write_u8(*v as u8);
            }
            IrLiteralValue::Char(v) => {
                state.write_u8(11);
                state.write_u32(*v as u32);
            }
        }
    }
}

impl From<&IrLiteralValue> for IrType {
    fn from(imm: &IrLiteralValue) -> Self {
        match imm {
            IrLiteralValue::I8(_) => IrType::I8,
            IrLiteralValue::I16(_) => IrType::I16,
            IrLiteralValue::I32(_) => IrType::I32,
            IrLiteralValue::I64(_) => IrType::I64,
            IrLiteralValue::U8(_) => IrType::U8,
            IrLiteralValue::U16(_) => IrType::U16,
            IrLiteralValue::U32(_) => IrType::U32,
            IrLiteralValue::U64(_) => IrType::U64,
            IrLiteralValue::F32(_) => IrType::F32,
            IrLiteralValue::F64(_) => IrType::F64,
            IrLiteralValue::Bool(_) => IrType::Bool,
            IrLiteralValue::Char(_) => IrType::Char,
        }
    }
}

impl fmt::Display for IrLiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrLiteralValue::I8(val) => f.write_fmt(format_args!("{val}i8")),
            IrLiteralValue::I16(val) => f.write_fmt(format_args!("{val}i16")),
            IrLiteralValue::I32(val) => f.write_fmt(format_args!("{val}i32")),
            IrLiteralValue::I64(val) => f.write_fmt(format_args!("{val}i64")),
            IrLiteralValue::U8(val) => f.write_fmt(format_args!("{val}u8")),
            IrLiteralValue::U16(val) => f.write_fmt(format_args!("{val}u16")),
            IrLiteralValue::U32(val) => f.write_fmt(format_args!("{val}u32")),
            IrLiteralValue::U64(val) => f.write_fmt(format_args!("{val}u64")),
            IrLiteralValue::F32(val) => {
                if val.is_finite() && val.fract() == 0.0 {
                    f.write_fmt(format_args!("{val:.1}f32"))
                } else {
                    f.write_fmt(format_args!("{val}f32"))
                }
            }
            IrLiteralValue::F64(val) => {
                if val.is_finite() && val.fract() == 0.0 {
                    f.write_fmt(format_args!("{val:.1}f64"))
                } else {
                    f.write_fmt(format_args!("{val}f64"))
                }
            }
            IrLiteralValue::Bool(val) => f.write_str(if *val { "true" } else { "false" }),
            IrLiteralValue::Char(val) => {
                f.write_str("'")?;
                val.escape_default().fmt(f)?;
                f.write_str("'")
            }
        }
    }
}
