// src/rvir/cfg.rs
use super::basic_block::RBasicBlock;
use super::instruction::RInstruction;
use super::terminator::RTerminator;
//use super::types::RIrType;
//use super::value::RValue;
//use crate::location::source_span::SourceSpan;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    graph: DiGraph<RBasicBlock, ()>,
    entry_label: String,
}
impl ControlFlowGraph {
    pub fn new(entry_label: String) -> Self {
        ControlFlowGraph {
            graph: DiGraph::new(),
            entry_label,
        }
    }

    pub fn entry_label(&self) -> &str {
        &self.entry_label
    }

    pub fn add_block(&mut self, block: RBasicBlock) -> NodeIndex {
        self.graph.add_node(block)
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }

    pub fn find_block_by_label(&self, label: &str) -> Option<NodeIndex> {
        self.graph
            .node_indices()
            .find(|&idx| self.graph[idx].label == label)
    }

    pub fn get_block(&self, label: &str) -> Option<&RBasicBlock> {
        self.find_block_by_label(label).map(|idx| &self.graph[idx])
    }

    pub fn get_block_mut(&mut self, label: &str) -> Option<&mut RBasicBlock> {
        if let Some(idx) = self.find_block_by_label(label) {
            self.graph.node_weight_mut(idx)
        } else {
            None
        }
    }

    pub fn get_entry_block(&self) -> Option<&RBasicBlock> {
        self.get_block(&self.entry_label)
    }

    pub fn get_entry_block_index(&self) -> Option<NodeIndex> {
        self.find_block_by_label(&self.entry_label)
    }

    pub fn add_instruction_to_block(
        &mut self,
        block_label: &str,
        instruction: RInstruction,
    ) -> bool {
        if let Some(block) = self.get_block_mut(block_label) {
            block.instructions.push(instruction);
            true
        } else {
            false
        }
    }

    pub fn set_block_terminator(&mut self, block_label: &str, terminator: RTerminator) -> bool {
        if let Some(block) = self.get_block_mut(block_label) {
            block.terminator = terminator;
            true
        } else {
            false
        }
    }

    pub fn connect_blocks(&mut self, from_label: &str, to_label: &str) -> bool {
        if let (Some(from_idx), Some(to_idx)) = (
            self.find_block_by_label(from_label),
            self.find_block_by_label(to_label),
        ) {
            self.add_edge(from_idx, to_idx);
            true
        } else {
            false
        }
    }

    pub fn blocks(&self) -> impl Iterator<Item = &RBasicBlock> {
        self.graph.node_weights()
    }

    pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut RBasicBlock> {
        self.graph.node_weights_mut()
    }

    pub fn dfs_post_order(&self) -> Box<dyn Iterator<Item = NodeIndex> + '_> {
        if let Some(entry_idx) = self.get_entry_block_index() {
            let mut dfs = Dfs::new(&self.graph, entry_idx);
            Box::new(std::iter::from_fn(move || dfs.next(&self.graph)))
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        // Verifica che esista un blocco di ingresso
        if self.get_entry_block().is_none() {
            return Err(format!(
                "CFG non ha un blocco di ingresso con etichetta '{}'",
                self.entry_label
            ));
        }

        // Verifica che tutti i blocchi abbiano un terminatore
        for block in self.blocks() {
            if !block.terminator.is_terminator() {
                return Err(format!(
                    "Blocco '{}' non ha un terminatore valido",
                    block.label
                ));
            }
        }

        // Verifica che tutti i target dei terminator esistano
        let label_set: HashSet<&str> =
            self.blocks().map(|b| b.label.as_str()).collect();
        for block in self.blocks() {
            for target_label in block.terminator.get_targets() {
                if !label_set.contains(target_label.as_str()) {
                    return Err(format!(
                        "Blocco '{}' riferisce a un blocco inesistente '{}'",
                        block.label, target_label
                    ));
                }
            }
        }

        Ok(())
    }
}
