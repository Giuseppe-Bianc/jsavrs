// src/ir/value/constant.rs
use super::Value;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrConstantValue {
    String { string: Arc<str> },
    Array { elements: Vec<Value> },
    Struct { name: Arc<str>, elements: Vec<Value> },
}

impl fmt::Display for IrConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrConstantValue::String { string } => {
                f.write_str("\"")?;
                string.escape_default().fmt(f)?;
                f.write_str("\"")
            }
            IrConstantValue::Array { elements } => {
                f.write_str("[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    elem.fmt(f)?;
                }
                f.write_str("]")
            }
            IrConstantValue::Struct { name, elements } => {
                name.fmt(f)?;
                f.write_str("<")?;
                for (i, field) in elements.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    field.fmt(f)?;
                }
                f.write_str(">")
            }
        }
    }
}
