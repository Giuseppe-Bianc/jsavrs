// src/ir/value/kind.rs
use super::{constant::IrConstantValue, literal::IrLiteralValue};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Kind of IR value (literal, constant, variable reference, etc.)
///
/// # Hash Implementation
///
/// Manual implementation optimized for performance:
/// - Discriminant is written explicitly using small integers for better performance
/// - `Arc<str>` strings are hashed by content (not pointer) for correctness
/// - Inline hint helps optimizer eliminate overhead in hot paths
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind {
    Literal(IrLiteralValue),
    Constant(IrConstantValue),
    Local(Arc<str>),
    Global(Arc<str>),
    Temporary(u64),
}

impl Hash for ValueKind {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Literal(lit) => {
                state.write_u8(0);
                lit.hash(state);
            }
            Self::Constant(cnst) => {
                state.write_u8(1);
                cnst.hash(state);
            }
            Self::Local(name) => {
                state.write_u8(2);
                name.as_ref().hash(state);
            }
            Self::Global(name) => {
                state.write_u8(3);
                name.as_ref().hash(state);
            }
            Self::Temporary(id) => {
                state.write_u8(4);
                state.write_u64(*id);
            }
        }
    }
}
