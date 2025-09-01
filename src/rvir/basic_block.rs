// src/rvir/basic_block.rs
use super::{instruction::*, terminator::*};
use crate::location::source_span::SourceSpan;
use super::types::RScopeId;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct RBasicBlock {
    pub label: String,
    pub source_span: SourceSpan,
    pub instructions: Vec<RInstruction>,
    pub(crate) terminator: RTerminator,
    pub(crate) scope: Option<RScopeId>,
}

impl RBasicBlock {
    pub fn new(label: &str, span: SourceSpan) -> Self {
        Self {
            label: label.to_string(),
            source_span: span.clone(),
            instructions: Vec::new(),
            terminator: RTerminator::new(RTerminatorKind::Unreachable, span),
            scope: None,
        }
    }

    pub fn with_scope(mut self, scope: RScopeId) -> Self {
        self.scope = Some(scope);
        self
    }
}

impl fmt::Display for RBasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(scope) = &self.scope {
            writeln!(f, "// Scope: {scope}")?;
        }
        writeln!(f, "{}:", self.label)?;
        for inst in &self.instructions {
            writeln!(f, "  {inst}")?;
        }
        writeln!(f, "  {}", self.terminator)
    }
}
