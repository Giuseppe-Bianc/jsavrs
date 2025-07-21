// src/nir/value/debug_info.rs
use crate::location::source_span::SourceSpan;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueDebugInfo {
    pub name: Option<String>,
    pub source_span: SourceSpan,
}