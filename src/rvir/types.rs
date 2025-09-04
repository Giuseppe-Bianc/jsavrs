// src/rvir/types.rs
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RIrType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    String,
    Void,
    Pointer(Box<RIrType>),
    Array(Box<RIrType>, usize),
    Custom(Arc<str>, SourceSpan), // Added source span
    Struct(Arc<str>, Vec<(String, RIrType)>, SourceSpan), // New struct type
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct RScopeId(Uuid);

impl RScopeId {
    pub fn new() -> Self {
        RScopeId(Uuid::new_v4())
    }
}

impl Default for RScopeId {
    fn default() -> Self {
        RScopeId::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct RResourceId(Uuid);

impl RResourceId {
    pub fn new() -> Self {
        RResourceId(Uuid::new_v4())
    }
}

impl Default for RResourceId {
    fn default() -> Self {
        RResourceId::new()
    }
}

// Display implementation for RScopeId
impl fmt::Display for RScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Display implementation for RResourceId
impl fmt::Display for RResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for RIrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RIrType::I8 => write!(f, "i8"),
            RIrType::I16 => write!(f, "i16"),
            RIrType::I32 => write!(f, "i32"),
            RIrType::I64 => write!(f, "i64"),
            RIrType::U8 => write!(f, "u8"),
            RIrType::U16 => write!(f, "u16"),
            RIrType::U32 => write!(f, "u32"),
            RIrType::U64 => write!(f, "u64"),
            RIrType::F32 => write!(f, "f32"),
            RIrType::F64 => write!(f, "f64"),
            RIrType::Bool => write!(f, "bool"),
            RIrType::Char => write!(f, "char"),
            RIrType::String => write!(f, "string"),
            RIrType::Void => write!(f, "void"),
            RIrType::Pointer(inner) => write!(f, "*{inner}"),
            RIrType::Array(element_type, size) => write!(f, "[{element_type}; {size}]"),
            RIrType::Custom(name, _) => write!(f, "{name}"),
            RIrType::Struct(name, fields, _) => {
                let fields_str = fields
                    .iter()
                    .map(|(field_name, ty)| format!("{field_name}: {ty}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "struct {name} {{ {fields_str} }}")
            }
        }
    }
}