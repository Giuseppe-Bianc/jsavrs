// src/ir/basic_block.rs
use super::types::ScopeId;
use super::{instruction::*, terminator::*};
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: Arc<str>,
    pub source_span: SourceSpan,
    pub instructions: Vec<Instruction>,
    pub(crate) terminator: Terminator,
    pub(crate) scope: Option<ScopeId>,
}

impl BasicBlock {
    pub fn new(label: &str, span: SourceSpan) -> Self {
        Self {
            label: label.into(),
            source_span: span.clone(),
            instructions: Vec::new(),
            terminator: Terminator::new(TerminatorKind::Unreachable, span),
            scope: None,
        }
    }

    #[inline]
    pub fn with_scope(mut self, scope: ScopeId) -> Self {
        self.scope = Some(scope);
        self
    }

    #[inline]
    pub fn terminator(&self) -> &Terminator {
        &self.terminator
    }
    #[inline]
    pub fn terminator_mut(&mut self) -> &mut Terminator {
        &mut self.terminator
    }
    #[inline]
    pub fn set_terminator(&mut self, t: Terminator) {
        self.terminator = t;
    }

    #[inline]
    pub fn scope(&self) -> Option<ScopeId> {
        self.scope
    } // if ScopeId: Copy
    // alternatively (safe regardless of Copy):
    // pub fn scope(&self) -> Option<&ScopeId> { self.scope.as_ref() }
    #[inline]
    pub fn set_scope(&mut self, s: ScopeId) {
        self.scope = Some(s);
    }
    #[inline]
    pub fn clear_scope(&mut self) {
        self.scope = None;
    }
}

impl fmt::Display for BasicBlock {
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
