//src/ir/function.rs
use super::{
    basic_block::BasicBlock,
    types::IrType,
};
use std::{collections::HashMap, fmt};

/// Represents a function in IR
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub basic_blocks: Vec<BasicBlock>,
    pub local_vars: HashMap<String, IrType>,
}

impl Function {
    pub fn new(name: &str, params: Vec<(String, IrType)>, return_type: IrType) -> Self {
        Self {
            name: name.to_string(),
            parameters: params,
            return_type,
            basic_blocks: Vec::new(),
            local_vars: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.basic_blocks.push(block);
    }

    pub fn add_local(&mut self, name: String, ty: IrType) {
        self.local_vars.insert(name, ty);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params_str = self
            .parameters
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "function {} ({}) -> {}:", self.name, params_str, self.return_type)?;

        for block in &self.basic_blocks {
            writeln!(f, "{}", block)?;
        }

        Ok(())
    }
}