// src/ir/value/constant.rs
use super::Value;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Represents compile-time constant values in the intermediate representation.
///
/// `IrConstantValue` encapsulates immutable values that are known at compile time
/// and can be embedded directly into the generated IR. These values are reference-counted
/// for efficient cloning and sharing across the IR graph.
///
/// # Variants
///
/// * `String` - A string literal constant
/// * `Array` - A constant array with statically-known elements
/// * `Struct` - A named struct constant with field values
///
/// # Examples
///
/// ```
/// use crate::ir::value::constant::IrConstantValue;
/// use std::sync::Arc;
///
/// let string_const = IrConstantValue::String {
///     string: Arc::from("hello"),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrConstantValue {
    /// A string literal constant.
    ///
    /// Uses `Arc<str>` for efficient sharing without heap allocation per clone.
    String { string: Arc<str> },
    /// A constant array with elements known at compile time.
    ///
    /// Elements are stored as `Value` to support heterogeneous constant types.
    Array { elements: Vec<Value> },
    /// A named struct constant with initialized field values.
    ///
    /// * `name` - The struct type name (e.g., `"Point"`, `"Color"`)
    /// * `elements` - Field values in declaration order
    Struct { name: Arc<str>, elements: Vec<Value> },
}

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
