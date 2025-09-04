// src/rvir/value/kind.rs
use super::{constant::RIrConstantValue, literal::RIrLiteralValue};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum RValueKind {
    Literal(RIrLiteralValue),
    Constant(RIrConstantValue),
    Local(Arc<str>),
    Global(Arc<str>),
    Temporary(u64),
}