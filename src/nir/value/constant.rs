// src/nir/value/constant.rs
use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum IrConstantValue {
    String(String),
    Array(Vec<Value>),
    Struct(String, Vec<Value>),
}
