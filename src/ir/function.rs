// src/ir/function.rs
use super::{basic_block::BasicBlock, types::IrType};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

/// Control Flow Graph representation
#[derive(Debug, Clone, PartialEq)]
pub struct CFG {
    pub blocks: HashMap<String, BasicBlock>,
    pub successors: HashMap<String, HashSet<String>>,
    pub predecessors: HashMap<String, HashSet<String>>,
    pub entry_label: String,
}

impl CFG {
    pub fn new(entry_label: &str) -> Self {
        let mut cfg = Self {
            blocks: HashMap::new(),
            successors: HashMap::new(),
            predecessors: HashMap::new(),
            entry_label: entry_label.to_string(),
        };
        cfg.add_block(BasicBlock::new(entry_label));
        cfg
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        let label = block.label.clone();
        self.blocks.insert(label.clone(), block);
        self.successors.entry(label.clone()).or_default();
        self.predecessors.entry(label.clone()).or_default();
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.successors
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
        self.predecessors
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());

        if let Some(block) = self.blocks.get_mut(to) {
            block.add_predecessor(from.to_string());
        }
    }

    pub fn get_block(&self, label: &str) -> Option<&BasicBlock> {
        self.blocks.get(label)
    }

    pub fn get_block_mut(&mut self, label: &str) -> Option<&mut BasicBlock> {
        self.blocks.get_mut(label)
    }
}

/// Represents a function in IR
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub cfg: CFG,
    pub local_vars: HashMap<String, IrType>,
}

impl Function {
    pub fn new(name: &str, params: Vec<(String, IrType)>, return_type: IrType) -> Self {
        Self {
            name: name.to_string(),
            parameters: params,
            return_type,
            cfg: CFG::new("entry"),
            local_vars: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.cfg.add_block(block);
    }

    pub fn add_local(&mut self, name: String, ty: IrType) {
        self.local_vars.insert(name, ty);
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.cfg.add_edge(from, to);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params_str = self
            .parameters
            .iter()
            .map(|(name, ty)| format!("{name}: {ty}"))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            f,
            "function {} ({}) -> {}:",
            self.name, params_str, self.return_type
        )?;

        // Print blocks in topological order
        let mut visited = HashSet::new();
        let mut stack = vec![self.cfg.entry_label.clone()];

        while let Some(label) = stack.pop() {
            if visited.contains(&label) {
                continue;
            }
            visited.insert(label.clone());

            if let Some(block) = self.cfg.blocks.get(&label) {
                writeln!(f, "{}\n", block)?;

                // Push successors in reverse order for DFS
                if let Some(successors) = self.cfg.successors.get(&label) {
                    let mut sorted_successors: Vec<_> = successors.iter().collect();
                    sorted_successors.sort();
                    for succ in sorted_successors.iter().rev() {
                        stack.push((*succ.clone()).clone());
                    }
                }
            }
        }

        Ok(())
    }
}