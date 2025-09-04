// src/rvir/value/constant.rs
use super::RValue;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum RIrConstantValue {
    String { string: Arc<str> },
    Array { elements: Vec<RValue> },
    Struct { name: Arc<str>, elements: Vec<RValue> },
}

impl fmt::Display for RIrConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RIrConstantValue::String { string } => write!(f, "\"{}\"", string.escape_default()),
            RIrConstantValue::Array { elements } => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{elem}")?;
                }
                write!(f, "]")
            }
            RIrConstantValue::Struct { name, elements } => {
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
