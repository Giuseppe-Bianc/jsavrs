// src/rvir/value/literal.rs
use crate::rvir::RIrType;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RIrLiteralValue {
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

impl From<&RIrLiteralValue> for RIrType {
    fn from(imm: &RIrLiteralValue) -> Self {
        match imm {
            RIrLiteralValue::I8(_) => RIrType::I8,
            RIrLiteralValue::I16(_) => RIrType::I16,
            RIrLiteralValue::I32(_) => RIrType::I32,
            RIrLiteralValue::I64(_) => RIrType::I64,
            RIrLiteralValue::U8(_) => RIrType::U8,
            RIrLiteralValue::U16(_) => RIrType::U16,
            RIrLiteralValue::U32(_) => RIrType::U32,
            RIrLiteralValue::U64(_) => RIrType::U64,
            RIrLiteralValue::F32(_) => RIrType::F32,
            RIrLiteralValue::F64(_) => RIrType::F64,
            RIrLiteralValue::Bool(_) => RIrType::Bool,
            RIrLiteralValue::Char(_) => RIrType::Char,
        }
    }
}

impl fmt::Display for RIrLiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RIrLiteralValue::I8(val) => write!(f, "{val}i8"),
            RIrLiteralValue::I16(val) => write!(f, "{val}i16"),
            RIrLiteralValue::I32(val) => write!(f, "{val}i32"),
            RIrLiteralValue::I64(val) => write!(f, "{val}i64"),
            RIrLiteralValue::U8(val) => write!(f, "{val}u8"),
            RIrLiteralValue::U16(val) => write!(f, "{val}u16"),
            RIrLiteralValue::U32(val) => write!(f, "{val}u32"),
            RIrLiteralValue::U64(val) => write!(f, "{val}u64"),
            RIrLiteralValue::F32(val) => {
                if val.fract() == 0.0 && val.is_finite() {
                    write!(f, "{val:.1}f32")
                } else {
                    write!(f, "{val}f32")
                }
            }
            RIrLiteralValue::F64(val) => {
                if val.fract() == 0.0 && val.is_finite() {
                    write!(f, "{val:.1}f64")
                } else {
                    write!(f, "{val}f64")
                }
            }
            RIrLiteralValue::Bool(val) => write!(f, "{val}"),
            RIrLiteralValue::Char(val) => write!(f, "'{escaped}'", escaped = val.escape_default()),
        }
    }
}
