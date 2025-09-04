// src/nir/value/debug_info.rs
use crate::location::source_span::SourceSpan;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueDebugInfo {
    pub name: Option<Arc<str>>,
    pub source_span: SourceSpan,
}