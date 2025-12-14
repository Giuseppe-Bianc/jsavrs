// src/nir/function.rs
use super::cfg::ControlFlowGraph;
use super::scope_manager::ScopeManager;
use super::types::{IrType, ScopeId};
use crate::ir::{BasicBlock, Instruction, Terminator};
use crate::location::source_span::SourceSpan;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionAttributes {
    pub is_entry: bool,
    pub is_varargs: bool,
    pub calling_convention: Arc<str>,
    pub source_span: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrParameter {
    pub name: Arc<str>,
    pub ty: IrType,
    pub attributes: ParamAttributes,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParamAttributes {
    pub by_val: bool,
    pub no_alias: bool,
    pub source_span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Arc<str>,
    pub parameters: Vec<IrParameter>,
    pub return_type: IrType,
    pub cfg: ControlFlowGraph,
    pub local_vars: HashMap<Arc<str>, IrType>,
    pub attributes: FunctionAttributes,
    pub(crate) scope_manager: ScopeManager,
}

impl Function {
    #[must_use]
    pub fn new(name: &str, params: Vec<IrParameter>, return_type: IrType) -> Self {
        Self {
            name: name.into(),
            parameters: params,
            return_type,
            cfg: ControlFlowGraph::new(Arc::from(format!("entry_{name}"))),
            local_vars: HashMap::new(),
            attributes: FunctionAttributes::default(),
            scope_manager: ScopeManager::new(),
        }
    }

    pub fn add_block(&mut self, label: &str, span: SourceSpan) -> bool {
        let block = BasicBlock::new(label, span).with_scope(self.scope_manager.current_scope());

        let block_idx = self.cfg.add_block(block);

        // Connect the entry block to the first added block (if not already done)
        if label != self.cfg.entry_label() && self.cfg.blocks().count() == 1 {
            // If this is the first block added (after the implicit one), connect it to entry
            if let Some(entry_idx) = self.cfg.get_entry_block_index() {
                self.cfg.add_edge(entry_idx, block_idx);
            }
        }

        true
    }

    pub fn add_instruction(&mut self, block_label: &str, instruction: Instruction) -> bool {
        self.cfg.add_instruction_to_block(block_label, instruction)
    }

    pub fn set_terminator(&mut self, block_label: &str, terminator: Terminator) -> bool {
        self.cfg.set_block_terminator(block_label, terminator)
    }

    pub fn connect_blocks(&mut self, from: &str, to: &str) -> bool {
        self.cfg.connect_blocks(from, to)
    }

    pub fn enter_scope(&mut self) -> ScopeId {
        self.scope_manager.enter_scope()
    }

    pub fn exit_scope(&mut self) {
        self.scope_manager.exit_scope();
    }

    /// Verifies the integrity of the function's control flow graph.
    ///
    /// Performs validation checks on the function's CFG to ensure it is well-formed
    /// and follows expected invariants.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the function's CFG is valid, otherwise returns an error describing
    /// the validation failure.
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// - The CFG structure is invalid or malformed
    /// - Basic blocks contain invalid terminators
    /// - Control flow edges are inconsistent
    /// - Any other CFG invariants are violated
    pub fn verify(&self) -> Result<(), String> {
        self.cfg.verify()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params_str =
            self.parameters.iter().map(|param| format!("{}: {}", param.name, param.ty)).collect::<Vec<_>>().join(", ");

        writeln!(f, "function {} ({}) -> {}:", self.name, params_str, self.return_type)?;
        let blocks_len = self.cfg.blocks().count();
        if blocks_len == 0 {
            writeln!(f, "<empty>")?;
            return Ok(());
        } else if blocks_len == 1 {
            writeln!(f, "block:")?;
        } else {
            writeln!(f, "blocks:")?;
        }

        for block in self.cfg.blocks() {
            writeln!(f, "{block}")?;
        }

        Ok(())
    }
}
