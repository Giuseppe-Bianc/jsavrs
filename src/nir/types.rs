use crate::location::source_span::SourceSpan;
// src/nir/types.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64, Bool,
    Char, String,
    Void,
    Pointer(Box<IrType>),
    Array(Box<IrType>, usize),
    Custom(String, SourceSpan), // Added source span
    Struct(String, Vec<IrType>, SourceSpan), // New struct type
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::I8 => write!(f, "i8"),
            IrType::I16 => write!(f, "i16"),
            IrType::I32 => write!(f, "i32"),
            IrType::I64 => write!(f, "i64"),
            IrType::U8 => write!(f, "u8"),
            IrType::U16 => write!(f, "u16"),
            IrType::U32 => write!(f, "u32"),
            IrType::U64 => write!(f, "u64"),
            IrType::F32 => write!(f, "f32"),
            IrType::F64 => write!(f, "f64"),
            IrType::Bool => write!(f, "bool"),
            IrType::Char => write!(f, "char"),
            IrType::String => write!(f, "string"),
            IrType::Void => write!(f, "void"),
            IrType::Pointer(inner) => write!(f, "*{inner}"),
            IrType::Array(element_type, size) => write!(f, "[{element_type}; {size}]"),
            IrType::Custom(name, _) => write!(f, "{name}"),
            IrType::Struct(name, fields, _) => {
                let fields_str = fields.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "struct {name} {{ {fields_str} }}")
            }
        }
    }
}