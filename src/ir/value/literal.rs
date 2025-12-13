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

impl Eq for IrLiteralValue {}

impl std::hash::Hash for IrLiteralValue {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash discriminant first for type differentiation
        // Using match allows compiler to optimize discriminant check
        match self {
            Self::I8(v) => {
                state.write_u8(0); // discriminant
                state.write_i8(*v);
            }
            Self::I16(v) => {
                state.write_u8(1);
                state.write_i16(*v);
            }
            Self::I32(v) => {
                state.write_u8(2);
                state.write_i32(*v);
            }
            Self::I64(v) => {
                state.write_u8(3);
                state.write_i64(*v);
            }
            Self::U8(v) => {
                state.write_u8(4);
                state.write_u8(*v);
            }
            Self::U16(v) => {
                state.write_u8(5);
                state.write_u16(*v);
            }
            Self::U32(v) => {
                state.write_u8(6);
                state.write_u32(*v);
            }
            Self::U64(v) => {
                state.write_u8(7);
                state.write_u64(*v);
            }
            Self::F32(v) => {
                state.write_u8(8);
                state.write_u32(v.to_bits());
            }
            Self::F64(v) => {
                state.write_u8(9);
                state.write_u64(v.to_bits());
            }
            Self::Bool(v) => {
                state.write_u8(10);
                state.write_u8(u8::from(*v));
            }
            Self::Char(v) => {
                state.write_u8(11);
                state.write_u32(u32::from(*v));
            }
        }
    }
}

impl From<&IrLiteralValue> for IrType {
    fn from(imm: &IrLiteralValue) -> Self {
        match imm {
            IrLiteralValue::I8(_) => Self::I8,
            IrLiteralValue::I16(_) => Self::I16,
            IrLiteralValue::I32(_) => Self::I32,
            IrLiteralValue::I64(_) => Self::I64,
            IrLiteralValue::U8(_) => Self::U8,
            IrLiteralValue::U16(_) => Self::U16,
            IrLiteralValue::U32(_) => Self::U32,
            IrLiteralValue::U64(_) => Self::U64,
            IrLiteralValue::F32(_) => Self::F32,
            IrLiteralValue::F64(_) => Self::F64,
            IrLiteralValue::Bool(_) => Self::Bool,
            IrLiteralValue::Char(_) => Self::Char,
        }
    }
}

impl fmt::Display for IrLiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8(val) => f.write_fmt(format_args!("{val}i8")),
            Self::I16(val) => f.write_fmt(format_args!("{val}i16")),
            Self::I32(val) => f.write_fmt(format_args!("{val}i32")),
            Self::I64(val) => f.write_fmt(format_args!("{val}i64")),
            Self::U8(val) => f.write_fmt(format_args!("{val}u8")),
            Self::U16(val) => f.write_fmt(format_args!("{val}u16")),
            Self::U32(val) => f.write_fmt(format_args!("{val}u32")),
            Self::U64(val) => f.write_fmt(format_args!("{val}u64")),
            Self::F32(val) => {
                if val.is_finite() && val.fract() == 0.0 {
                    f.write_fmt(format_args!("{val:.1}f32"))
                } else {
                    f.write_fmt(format_args!("{val}f32"))
                }
            }
            Self::F64(val) => {
                if val.is_finite() && val.fract() == 0.0 {
                    f.write_fmt(format_args!("{val:.1}f64"))
                } else {
                    f.write_fmt(format_args!("{val}f64"))
                }
            }
            Self::Bool(val) => f.write_str(if *val { "true" } else { "false" }),
            Self::Char(val) => {
                f.write_str("'")?;
                val.escape_default().fmt(f)?;
                f.write_str("'")
            }
        }
    }
}
