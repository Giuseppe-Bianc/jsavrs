// src/ir/value/kind.rs
use super::{constant::IrConstantValue, literal::IrLiteralValue};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Literal(IrLiteralValue),
    Constant(IrConstantValue),
    Local(Arc<str>),
    Global(Arc<str>),
    Temporary(u64),
}
