// src/nir/value/mod.rs
pub mod debug_info;
pub mod kind;
pub mod literal;
pub mod constant;

pub use self::{
    constant::IrConstantValue,
    debug_info::ValueDebugInfo,
    kind::ValueKind,
    literal::IrLiteralValue,
};

use super::types::IrType;
use crate::location::source_span::SourceSpan;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub id: u64,
    pub kind: ValueKind,
    pub ty: IrType,
    pub debug_info: Option<ValueDebugInfo>,
}

impl Value {
    pub fn new_literal(imm: IrLiteralValue) -> Self {
        let ty = match &imm {
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
        };
        Value {
            id: Self::next_id(),
            kind: ValueKind::Literal(imm),
            ty,
            debug_info: None,
        }
    }

    pub fn new_constant(imm: IrConstantValue, ty: IrType) -> Self {
        Value {
            id: Self::next_id(),
            kind: ValueKind::Constant(imm),
            ty,
            debug_info: None,
        }
    }

    pub fn new_local(name: String, ty: IrType) -> Self {
        Value {
            id: Self::next_id(),
            kind: ValueKind::Local(name),
            ty,
            debug_info: None,
        }
    }

    pub fn new_temporary(id: u64, ty: IrType) -> Self {
        Value {
            id,
            kind: ValueKind::Temporary(id),
            ty,
            debug_info: None,
        }
    }

    pub fn with_debug_info(mut self, name: Option<String>, span: SourceSpan) -> Self {
        self.debug_info = Some(ValueDebugInfo {
            name,
            source_span: span,
        });
        self
    }

    fn next_id() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ValueKind::Literal(imm) => match imm {
                IrLiteralValue::I8(i) => write!(f, "{}i8", i),
                IrLiteralValue::I16(i) => write!(f, "{}i16", i),
                IrLiteralValue::I32(i) => write!(f, "{}i32", i),
                IrLiteralValue::I64(i) => write!(f, "{}i64", i),
                IrLiteralValue::U8(u) => write!(f, "{}u8", u),
                IrLiteralValue::U16(u) => write!(f, "{}u16", u),
                IrLiteralValue::U32(u) => write!(f, "{}u32", u),
                IrLiteralValue::U64(u) => write!(f, "{}u64", u),
                IrLiteralValue::F32(flt) => write!(f, "{}f32", flt),
                IrLiteralValue::F64(flt) => write!(f, "{}f64", flt),
                IrLiteralValue::Bool(b) => write!(f, "{}", b),
                IrLiteralValue::Char(c) => write!(f, "'{}'", c),
            },
            ValueKind::Constant(imm) => match imm {
                IrConstantValue::String(s) => write!(f, "\"{}\"", s.escape_default()),
                IrConstantValue::Array(elems) => {
                    let elems_str = elems
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "[{}]", elems_str)
                }
                IrConstantValue::Struct(name, fields) => {
                    let fields_str = fields
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "{}<{}>", name, fields_str)
                }
            },
            ValueKind::Local(name) => write!(f, "%{}", name),
            ValueKind::Global(name) => write!(f, "@{}", name),
            ValueKind::Temporary(id) => write!(f, "t{}", id),
        }
    }
}
