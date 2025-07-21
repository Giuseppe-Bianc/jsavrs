// src/nir/value/constant.rs
use std::fmt;
use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum IrConstantValue {
    String(String),
    Array(Vec<Value>),
    Struct(String, Vec<Value>),
}

impl fmt::Display for IrConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
        }
    }
}