// src/nir/value/kind.rs
use super::{constant::IrConstantValue, literal::IrLiteralValue};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Literal(IrLiteralValue),
    Constant(IrConstantValue),
    Local(String),
    Global(String),
    Temporary(u64),
}