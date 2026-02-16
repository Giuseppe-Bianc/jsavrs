// src/ir/cfg.rs
use super::basic_block::BasicBlock;
use super::instruction::Instruction;
use super::terminator::Terminator;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    graph: DiGraph<BasicBlock, ()>,
    pub entry_label: Arc<str>,
    reverse_post_order: Vec<NodeIndex>,
    /// Tracks whether `reverse_post_order` is valid with respect to current graph topology.
    /// Set to `false` when nodes or edges are added/removed; lazily recomputed on access.
    reverse_post_order_valid: bool,
}
impl ControlFlowGraph {
    #[must_use]
    pub fn new(entry_label: Arc<str>) -> Self {
        Self { graph: DiGraph::new(), entry_label, reverse_post_order: Vec::new(), reverse_post_order_valid: false }
    }

    #[must_use]
    pub const fn graph(&self) -> &DiGraph<BasicBlock, ()> {
        &self.graph
    }

    pub const fn graph_mut(&mut self) -> &mut DiGraph<BasicBlock, ()> {
        &mut self.graph
    }

    #[must_use]
    pub fn entry_label(&self) -> &str {
        &self.entry_label
    }

    #[must_use]
    pub fn reverse_post_order(&mut self) -> &[NodeIndex] {
        if !self.reverse_post_order_valid {
            self.recompute_reverse_post_order();
            self.reverse_post_order_valid = true;
        }
        &self.reverse_post_order
    }

    pub fn add_block(&mut self, block: BasicBlock) -> NodeIndex {
        let idx = self.graph.add_node(block);
        self.reverse_post_order_valid = false;
        idx
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
        self.reverse_post_order_valid = false;
    }

    #[inline]
    #[must_use]
    pub fn find_block_by_label(&self, label: &str) -> Option<NodeIndex> {
        self.graph.node_indices().find(|&idx| self.graph[idx].label.as_ref() == label)
    }

    /// Returns a reference to the block with the given label, if it exists.
    #[must_use]
    pub fn get_block(&self, label: &str) -> Option<&BasicBlock> {
        self.find_block_by_label(label).map(|idx| &self.graph[idx])
    }

    /// Returns a mutable reference to the block with the given label, if it exists.
    pub fn get_block_mut(&mut self, label: &str) -> Option<&mut BasicBlock> {
        self.find_block_by_label(label).and_then(|idx| self.graph.node_weight_mut(idx))
    }

    #[must_use]
    pub fn get_entry_block(&self) -> Option<&BasicBlock> {
        self.get_block(&self.entry_label)
    }

    #[must_use]
    pub fn get_entry_block_index(&self) -> Option<NodeIndex> {
        self.find_block_by_label(&self.entry_label)
    }

    /// Adds an instruction to the block with the given label.
    /// Returns true if the block was found and the instruction was added, false otherwise.
    pub fn add_instruction_to_block(&mut self, block_label: &str, instruction: Instruction) -> bool {
        if let Some(block) = self.get_block_mut(block_label) {
            block.instructions.push(instruction);
            true
        } else {
            false
        }
    }

    /// Sets the terminator for the block with the given label.
    /// Returns true if the block was found and the terminator was set, false otherwise.
    pub fn set_block_terminator(&mut self, block_label: &str, terminator: Terminator) -> bool {
        if let Some(block) = self.get_block_mut(block_label) {
            block.terminator = terminator;
            true
        } else {
            false
        }
    }

    pub fn connect_blocks(&mut self, from_label: &str, to_label: &str) -> bool {
        if let (Some(from_idx), Some(to_idx)) =
            (self.find_block_by_label(from_label), self.find_block_by_label(to_label))
        {
            self.add_edge(from_idx, to_idx);
            true
        } else {
            false
        }
    }

    pub fn blocks(&self) -> impl Iterator<Item = &BasicBlock> {
        self.graph.node_weights()
    }

    pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut BasicBlock> {
        self.graph.node_weights_mut()
    }

    /// Removes a block from the CFG by its label.
    ///
    /// # Arguments
    ///
    /// * `label` - The label of the block to remove
    ///
    /// # Returns
    ///
    /// `true` if the block was found and removed, `false` otherwise
    ///
    /// # Note
    ///
    /// This also removes all incoming and outgoing edges for the block.
    /// Callers should ensure the removal maintains CFG validity.
    #[must_use]
    pub fn remove_block(&mut self, label: &str) -> bool {
        if let Some(idx) = self.find_block_by_label(label) {
            self.graph.remove_node(idx);
            self.reverse_post_order_valid = false;
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn dfs_post_order(&self) -> Box<dyn Iterator<Item = NodeIndex> + '_> {
        if let Some(entry_idx) = self.get_entry_block_index() {
            let mut dfs = Dfs::new(&self.graph, entry_idx);
            Box::new(std::iter::from_fn(move || dfs.next(&self.graph)))
        } else {
            Box::new(std::iter::empty())
        }
    }

    /// Verifies the integrity and well-formedness of the control flow graph.
    ///
    /// Performs validation checks to ensure the CFG meets expected invariants:
    /// - An entry block with the expected label exists
    /// - All blocks have valid terminator instructions
    /// - All terminator target labels refer to existing blocks
    ///
    /// # Returns
    ///
    /// `Ok(())` if the CFG is valid, otherwise returns an error describing the
    /// validation failure.
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// - The CFG has no entry block with the expected label
    /// - Any block lacks a valid terminator instruction
    /// - Any terminator refers to a non-existent target block
    pub fn verify(&self) -> Result<(), String> {
        // Verify that an entry block exists
        if self.get_entry_block().is_none() {
            return Err(format!("CFG has no entry block with label '{}'", self.entry_label));
        }

        // Verify that all blocks have a terminator
        for block in self.blocks() {
            if !block.terminator.is_terminator() {
                return Err(format!("Block '{}' has no valid terminator", block.label));
            }
        }

        // Verify that all terminator targets exist
        let label_set: HashSet<Arc<str>> = self.blocks().map(|b| b.label.clone()).collect();

        for block in self.blocks() {
            for target_label in block.terminator.get_targets() {
                if !label_set.contains(&target_label) {
                    return Err(format!("Block '{}' refers to non-existent block '{}'", block.label, target_label));
                }
            }
        }

        Ok(())
    }

    // Add this method to recompute RPOT when CFG changes
    pub fn recompute_reverse_post_order(&mut self) {
        if let Some(entry_idx) = self.get_entry_block_index() {
            let mut post_order = Vec::new();
            let mut dfs = Dfs::new(&self.graph, entry_idx);
            while let Some(node) = dfs.next(&self.graph) {
                post_order.push(node);
            }
            post_order.reverse(); // This makes it reverse post-order
            self.reverse_post_order = post_order;
        } else {
            self.reverse_post_order.clear();
        }
    }
}
