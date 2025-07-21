// src/nir/value/literal.rs
use crate::nir::IrType;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
            IrLiteralValue::I8(val) => write!(f, "{}i8", val),
            IrLiteralValue::I16(val) => write!(f, "{}i16", val),
            IrLiteralValue::I32(val) => write!(f, "{}i32", val),
            IrLiteralValue::I64(val) => write!(f, "{}i64", val),
            IrLiteralValue::U8(val) => write!(f, "{}u8", val),
            IrLiteralValue::U16(val) => write!(f, "{}u16", val),
            IrLiteralValue::U32(val) => write!(f, "{}u32", val),
            IrLiteralValue::U64(val) => write!(f, "{}u64", val),
            IrLiteralValue::F32(val) => {
                if val.fract() == 0.0 && val.is_finite() {
                    write!(f, "{:.1}f32", val)
                } else {
                    write!(f, "{}f32", val)
                }
            }
            IrLiteralValue::F64(val) => {
                if val.fract() == 0.0 && val.is_finite() {
                    write!(f, "{:.1}f64", val)
                } else {
                    write!(f, "{}f64", val)
                }
            }
            IrLiteralValue::Bool(val) => write!(f, "{}", val),
            IrLiteralValue::Char(val) => write!(f, "'{}'", val.escape_default()),
        }
    }
}
