// src/ir/basic_block.rs
use super::{instruction::Instruction, terminator::Terminator};
use std::fmt;

/// A sequence of non-branching instructions ending with a terminator
#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.label)?;
        for inst in &self.instructions {
            writeln!(f, "  {}", inst)?;
        }
        write!(f, "  {}", self.terminator)
    }
}