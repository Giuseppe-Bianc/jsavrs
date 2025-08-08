// src/nir/basic_block.rs
use super::{instruction::*, terminator::*};
use crate::location::source_span::SourceSpan;
use super::types::ScopeId;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub source_span: SourceSpan,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub predecessors: Vec<String>,
    pub dominator_info: Option<DominatorInfo>,
    pub scope: Option<ScopeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DominatorInfo {
    pub dominators: Vec<String>,
    pub immediate_dominator: Option<String>,
}

impl BasicBlock {
    pub fn new(label: &str, span: SourceSpan) -> Self {
        Self {
            label: label.to_string(),
            source_span: span,
            instructions: Vec::new(),
            terminator: Terminator::new(TerminatorKind::Unreachable, SourceSpan::default()),
            predecessors: Vec::new(),
            dominator_info: None,
            scope: None,
        }
    }

    pub fn add_predecessor(&mut self, pred_label: String) {
        if !self.predecessors.contains(&pred_label) {
            self.predecessors.push(pred_label);
        }
    }

    pub fn with_scope(mut self, scope: ScopeId) -> Self {
        self.scope = Some(scope);
        self
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(scope) = self.scope {
            writeln!(f, "// Scope: {scope}")?;
        }

        if !self.predecessors.is_empty() {
            writeln!(f, "// Predecessors: {}", self.predecessors.join(", "))?;
        }

        writeln!(f, "{}:", self.label)?;
        for inst in &self.instructions {
            writeln!(f, "  {inst}")?;
        }
        write!(f, "  {term}", term = self.terminator)
    }
}
