// src/nir/value/constant.rs
use super::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum IrConstantValue {
    String { string: String },
    Array { elements: Vec<Value> },
    Struct { name: String, elements: Vec<Value> },
}

impl fmt::Display for IrConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrConstantValue::String { string } => write!(f, "\"{}\"", string.escape_default()),
            IrConstantValue::Array { elements } => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{elem}")?;
                }
                write!(f, "]")
            }
            IrConstantValue::Struct { name, elements } => {
                write!(f, "{name}<")?;
                for (i, field) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{field}")?;
                }
                write!(f, ">")
            }
        }
    }
}
