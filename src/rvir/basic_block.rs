use super::types::RScopeId;
// src/rvir/basic_block.rs
use super::{instruction::*, terminator::*};
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct RBasicBlock {
    pub label: Arc<str>,
    pub source_span: SourceSpan,
    pub instructions: Vec<RInstruction>,
    pub(crate) terminator: RTerminator,
    pub(crate) scope: Option<RScopeId>,
}

impl RBasicBlock {
    pub fn new(label: &str, span: SourceSpan) -> Self {
        Self {
            label: label.into(),
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

    pub fn terminator(&self) -> &RTerminator {
        &self.terminator
    }
    pub fn terminator_mut(&mut self) -> &mut RTerminator {
        &mut self.terminator
    }
    pub fn set_terminator(&mut self, t: RTerminator) {
        self.terminator = t;
    }

    pub fn scope(&self) -> Option<RScopeId> {
        self.scope
    } // if RScopeId: Copy
    // alternatively (safe regardless of Copy):
    // pub fn scope(&self) -> Option<&RScopeId> { self.scope.as_ref() }
    pub fn set_scope(&mut self, s: RScopeId) {
        self.scope = Some(s);
    }
    pub fn clear_scope(&mut self) {
        self.scope = None;
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
