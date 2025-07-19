// src/ir/basic_block.rs
use super::{instruction::Instruction, terminator::Terminator};
use std::fmt;

/// A sequence of non-branching instructions ending with a terminator
#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub predecessors: Vec<String>,
}

impl BasicBlock {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
            predecessors: Vec::new(),
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
        write!(f, "  {}", self.terminator)
    }
}
