//src/ir/value.rs
use super::types::IrType;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub kind: ValueKind,
    pub ty: IrType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Immediate(ImmediateValue),
    Local(String),
    Global(String),
    Temporary(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImmediateValue {
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
    String(String),
}

impl Value {
    pub fn new_immediate(imm: ImmediateValue) -> Self {
        let ty = match &imm {
            ImmediateValue::I8(_) => IrType::I8,
            ImmediateValue::I16(_) => IrType::I16,
            ImmediateValue::I32(_) => IrType::I32,
            ImmediateValue::I64(_) => IrType::I64,
            ImmediateValue::U8(_) => IrType::U8,
            ImmediateValue::U16(_) => IrType::U16,
            ImmediateValue::U32(_) => IrType::U32,
            ImmediateValue::U64(_) => IrType::U64,
            ImmediateValue::F32(_) => IrType::F32,
            ImmediateValue::F64(_) => IrType::F64,
            ImmediateValue::Bool(_) => IrType::Bool,
            ImmediateValue::Char(_) => IrType::Char,
            ImmediateValue::String(_) => IrType::String,
        };
        Value {
            kind: ValueKind::Immediate(imm),
            ty,
        }
    }

    pub fn new_local(name: String, ty: IrType) -> Self {
        Value {
            kind: ValueKind::Local(name),
            ty,
        }
    }

    pub fn new_temporary(id: String, ty: IrType) -> Self {
        Value {
            kind: ValueKind::Temporary(id),
            ty,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ValueKind::Immediate(imm) => write!(f, "{imm}"),
            ValueKind::Local(name) => write!(f, "%{name}"),
            ValueKind::Global(name) => write!(f, "@{name}"),
            ValueKind::Temporary(id) => write!(f, "t{id}"),
        }
    }
}

impl fmt::Display for ImmediateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImmediateValue::I8(i) => write!(f, "{i}i8"),
            ImmediateValue::I16(i) => write!(f, "{i}i16"),
            ImmediateValue::I32(i) => write!(f, "{i}i32"),
            ImmediateValue::I64(i) => write!(f, "{i}i64"),
            ImmediateValue::U8(u) => write!(f, "{u}u8"),
            ImmediateValue::U16(u) => write!(f, "{u}u16"),
            ImmediateValue::U32(u) => write!(f, "{u}u32"),
            ImmediateValue::U64(u) => write!(f, "{u}u64"),
            ImmediateValue::F32(flt) => write!(f, "{flt}f32"),
            ImmediateValue::F64(flt) => write!(f, "{flt}f64"),
            ImmediateValue::Bool(b) => write!(f, "{b}"),
            ImmediateValue::Char(c) => write!(f, "'{c}'"),
            ImmediateValue::String(s) => write!(f, "\"{}\"", s.escape_default()),
        }
    }
}
