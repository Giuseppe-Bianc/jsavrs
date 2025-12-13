//! Escape analysis for memory operations.
//!
//! Determines whether values (typically allocations) escape their function,
//! which affects the safety of removing stores and loads.

use crate::ir::{Function, InstructionKind, Value};
use std::collections::HashMap;

/// Escape status of a value (typically an allocation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EscapeStatus {
    /// The value is purely local - address never leaves the function.
    Local,

    /// The value's address is taken but may not escape.
    AddressTaken,

    /// The value escapes the function.
    Escaped,
}

/// Side effect classification for instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SideEffectClass {
    /// Pure computation with no side effects.
    Pure,

    /// Memory read operation.
    MemoryRead,

    /// Memory write operation.
    MemoryWrite,

    /// Has observable side effects.
    EffectFul,
}

impl SideEffectClass {
    /// Classifies an instruction based on its side effects.
    #[allow(dead_code)]
    pub const fn classify(instruction: &crate::ir::Instruction, _escape_analyzer: &EscapeAnalyzer) -> Self {
        match &instruction.kind {
            InstructionKind::Binary { .. }
            | InstructionKind::Unary { .. }
            | InstructionKind::Cast { .. }
            | InstructionKind::GetElementPtr { .. }
            | InstructionKind::Vector { .. }
            | InstructionKind::Alloca { .. } => Self::Pure,

            InstructionKind::Load { .. } => Self::MemoryRead,
            InstructionKind::Store { .. } => Self::MemoryWrite,
            InstructionKind::Call { .. } | InstructionKind::Phi { .. } => Self::EffectFul,
        }
    }
}

/// Escape analyzer for flow-insensitive escape analysis.
#[derive(Debug)]
pub struct EscapeAnalyzer {
    /// Escape status for each value (typically alloca results).
    escape_map: HashMap<Value, EscapeStatus>,
}

impl Drop for EscapeAnalyzer {
    fn drop(&mut self) {
        // Explicitly clear HashMap to release memory eagerly
        self.escape_map.clear();
    }
}

impl EscapeAnalyzer {
    /// Creates a new escape analyzer.
    pub fn new() -> Self {
        Self { escape_map: HashMap::new() }
    }

    /// Performs flow-insensitive escape analysis on a function.
    pub fn analyze(&mut self, function: &Function) {
        self.initialize_allocas(function);
        self.scan_for_escapes(function);
    }

    /// Initializes all allocas as Local.
    fn initialize_allocas(&mut self, function: &Function) {
        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                if let InstructionKind::Alloca { .. } = instruction.kind
                    && let Some(result) = &instruction.result
                {
                    self.escape_map.insert(result.clone(), EscapeStatus::Local);
                }
            }
        }
    }

    /// Scans for escape conditions.
    fn scan_for_escapes(&mut self, function: &Function) {
        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                match &instruction.kind {
                    InstructionKind::Store { value, .. } if self.is_alloca_value(value) => {
                        self.mark_escaped(value);
                    }

                    InstructionKind::Call { args, .. } => {
                        for arg in args {
                            if self.is_alloca_value(arg) {
                                self.mark_escaped(arg);
                            }
                        }
                    }

                    InstructionKind::GetElementPtr { base, .. } if self.is_alloca_value(base) => {
                        self.mark_address_taken(base);
                    }

                    _ => {}
                }
            }

            if let crate::ir::TerminatorKind::Return { value, .. } = &block.terminator.kind
                && self.is_alloca_value(value)
            {
                self.mark_escaped(value);
            }
        }
    }

    /// Checks if a value is an alloca result we're tracking.
    fn is_alloca_value(&self, value: &Value) -> bool {
        self.escape_map.contains_key(value)
    }

    /// Marks a value as having its address taken.
    #[inline]
    fn mark_address_taken(&mut self, value: &Value) {
        if let Some(status) = self.escape_map.get_mut(value)
            && *status == EscapeStatus::Local
        {
            *status = EscapeStatus::AddressTaken;
        }
    }

    /// Marks a value as escaped.
    #[inline]
    fn mark_escaped(&mut self, value: &Value) {
        self.escape_map.insert(value.clone(), EscapeStatus::Escaped);
    }

    /// Gets the escape status of a value.
    #[inline]
    pub fn get_status(&self, value: &Value) -> EscapeStatus {
        self.escape_map.get(value).copied().unwrap_or(EscapeStatus::Escaped)
    }
}

impl Default for EscapeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
