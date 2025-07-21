// src/nir/basic_block.rs
use super::{instruction::*, terminator::*};
use std::fmt;
use crate::location::source_span::SourceSpan;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub source_span: SourceSpan, // Added source span
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub predecessors: Vec<String>,
    pub dominator_info: Option<DominatorInfo>, // For optimization passes
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
        }
    }

    pub fn add_predecessor(&mut self, pred_label: String) {
        if !self.predecessors.contains(&pred_label) {
            self.predecessors.push(pred_label);
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
