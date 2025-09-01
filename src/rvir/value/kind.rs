// src/rvir/value/kind.rs
use super::{constant::RIrConstantValue, literal::RIrLiteralValue};

#[derive(Debug, Clone, PartialEq)]
pub enum RValueKind {
    Literal(RIrLiteralValue),
    Constant(RIrConstantValue),
    Local(String),
    Global(String),
    Temporary(u64),
}