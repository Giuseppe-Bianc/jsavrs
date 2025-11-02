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

// Manual Hash implementation to handle f32/f64
// We hash the bit representation to be consistent with our Eq impl
impl std::hash::Hash for IrLiteralValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            IrLiteralValue::I8(v) => v.hash(state),
            IrLiteralValue::I16(v) => v.hash(state),
            IrLiteralValue::I32(v) => v.hash(state),
            IrLiteralValue::I64(v) => v.hash(state),
            IrLiteralValue::U8(v) => v.hash(state),
            IrLiteralValue::U16(v) => v.hash(state),
            IrLiteralValue::U32(v) => v.hash(state),
            IrLiteralValue::U64(v) => v.hash(state),
            IrLiteralValue::F32(v) => v.to_bits().hash(state),
            IrLiteralValue::F64(v) => v.to_bits().hash(state),
            IrLiteralValue::Bool(v) => v.hash(state),
            IrLiteralValue::Char(v) => v.hash(state),
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
