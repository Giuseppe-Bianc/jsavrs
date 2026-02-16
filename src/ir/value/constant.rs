// src/ir/value/constant.rs
use super::Value;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum IrConstantValue {
    String { string: Arc<str> },
    Array { elements: Vec<Value> },
    Struct { name: Arc<str>, elements: Vec<Value> },
}

impl PartialEq for IrConstantValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String { string: a }, Self::String { string: b }) => a == b,
            (Self::Array { elements: a }, Self::Array { elements: b }) => {
                // Short-circuit if lengths differ to avoid expensive element comparisons
                if a.len() != b.len() {
                    return false;
                }
                // Compare elements only if lengths match
                a == b
            },
            (Self::Struct { name: a_name, elements: a_elements }, Self::Struct { name: b_name, elements: b_elements }) => {
                // Check name first as it's cheaper than comparing elements
                if a_name != b_name {
                    return false;
                }
                // Short-circuit if lengths differ to avoid expensive element comparisons
                if a_elements.len() != b_elements.len() {
                    return false;
                }
                // Compare elements only if names and lengths match
                a_elements == b_elements
            },
            _ => false, // Different variants are not equal
        }
    }
}

impl Eq for IrConstantValue {}

impl Hash for IrConstantValue {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::String { string } => {
                state.write_u8(0);
                string.as_ref().hash(state);
            }
            Self::Array { elements } => {
                state.write_u8(1);
                elements.hash(state);
            }
            Self::Struct { name, elements } => {
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
            Self::String { string } => {
                f.write_str("\"")?;
                string.escape_default().fmt(f)?;
                f.write_str("\"")
            }
            Self::Array { elements } => {
                f.write_str("[")?;
                write_comma_separated(f, elements)?;
                f.write_str("]")
            }
            Self::Struct { name, elements } => {
                name.fmt(f)?;
                f.write_str("<")?;
                write_comma_separated(f, elements)?;
                f.write_str(">")
            }
        }
    }
}
