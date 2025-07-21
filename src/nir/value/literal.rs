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
            IrLiteralValue::I8(val) => write!(f, "{val}i8"),
            IrLiteralValue::I16(val) => write!(f, "{val}i16"),
            IrLiteralValue::I32(val) => write!(f, "{val}i32"),
            IrLiteralValue::I64(val) => write!(f, "{val}i64"),
            IrLiteralValue::U8(val) => write!(f, "{val}u8"),
            IrLiteralValue::U16(val) => write!(f, "{val}u16"),
            IrLiteralValue::U32(val) => write!(f, "{val}u32"),
            IrLiteralValue::U64(val) => write!(f, "{val}u64"),
            IrLiteralValue::F32(val) => {
                if val.fract() == 0.0 && val.is_finite() {
                    write!(f, "{val:.1}f32")
                } else {
                    write!(f, "{val}f32")
                }
            }
            IrLiteralValue::F64(val) => {
                if val.fract() == 0.0 && val.is_finite() {
                    write!(f, "{val:.1}f64")
                } else {
                    write!(f, "{val}f64")
                }
            }
            IrLiteralValue::Bool(val) => write!(f, "{val}"),
            IrLiteralValue::Char(val) => write!(f, "'{escaped}'", escaped = val.escape_default()),
        }
    }
}
