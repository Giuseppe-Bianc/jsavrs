// src/nir/value/constant.rs
use super::Value;
use std::fmt;

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
                write!(f, "[")?;
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{elem}")?;
                }
                write!(f, "]")
            }
            IrConstantValue::Struct(name, fields) => {
                write!(f, "{}<", name)?;
                for (i, field) in fields.iter().enumerate() {
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
