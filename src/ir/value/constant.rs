// src/ir/value/constant.rs
use super::Value;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrConstantValue {
    String { string: Arc<str> },
    Array { elements: Vec<Value> },
    Struct { name: Arc<str>, elements: Vec<Value> },
}

impl Hash for IrConstantValue {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            IrConstantValue::String { string } => {
                state.write_u8(0);
                string.as_ref().hash(state);
            }
            IrConstantValue::Array { elements } => {
                state.write_u8(1);
                elements.hash(state);
            }
            IrConstantValue::Struct { name, elements } => {
                state.write_u8(2);
                name.as_ref().hash(state);
                elements.hash(state);
            }
        }
    }
}

fn write_comma_separated<T: fmt::Display>(f: &mut fmt::Formatter<'_>, items: &[T]) -> fmt::Result {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            f.write_str(", ")?;
        }
        item.fmt(f)?;
    }
    Ok(())
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
                write_comma_separated(f, elements)?;
                f.write_str("]")
            }
            IrConstantValue::Struct { name, elements } => {
                name.fmt(f)?;
                f.write_str("<")?;
                write_comma_separated(f, elements)?;
                f.write_str(">")
            }
        }
    }
}
