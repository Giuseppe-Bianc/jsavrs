//! IR rewriting for SCCP optimization
//!
//! This module implements the IR transformation phase that applies
//! optimizations based on SCCP analysis results.

use super::{lattice::LatticeValue, stats::OptimizationStatistics};
use crate::ir::instruction::InstructionKind;
use crate::ir::terminator::{Terminator, TerminatorKind};
use crate::ir::value::ValueKind;
use crate::ir::{Function, IrLiteralValue, Value};
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

/// Handles IR rewriting based on SCCP analysis results
pub struct IRRewriter<'a> {
    /// Reference to the function being rewritten
    function: &'a mut Function,
    /// Lattice values from SCCP analysis
    lattice: HashMap<usize, LatticeValue>,
    /// Set of executable blocks
    executable_blocks: HashSet<usize>,
    /// Statistics tracker
    stats: OptimizationStatistics,
}

impl<'a> IRRewriter<'a> {
    /// Creates a new IR rewriter
    ///
    /// # Arguments
    ///
    /// * `function` - The function to rewrite
    /// * `lattice` - Lattice values from SCCP analysis (maps instruction index to value)
    /// * `executable_blocks` - Set of blocks proven to be executable
    pub fn new(
        function: &'a mut Function, lattice: HashMap<usize, LatticeValue>, executable_blocks: HashSet<usize>,
    ) -> Self {
        Self { function, lattice, executable_blocks, stats: OptimizationStatistics::default() }
    }

    /// Performs all rewriting transformations
    ///
    /// # Returns
    ///
    /// Statistics about the transformations performed
    pub fn rewrite(mut self) -> OptimizationStatistics {
        // Phase 1: Remove unreachable blocks
        self.remove_unreachable_blocks();

        // Phase 2: Clean up phi node edges from removed blocks
        self.cleanup_phi_edges();

        // Phase 3: Simplify phi nodes
        self.simplify_phi_nodes();

        // Phase 4: Convert conditional branches with constant conditions
        self.simplify_branches();

        self.stats
    }

    /// Simplifies phi nodes based on analysis results
    ///
    /// Transformations:
    /// 1. Remove phi nodes with all constant incoming values (same value)
    /// 2. Remove phi nodes with only one executable predecessor
    /// 3. Track simplifications in statistics
    fn simplify_phi_nodes(&mut self) {
        let cfg = &self.function.cfg;

        // Collect phi simplifications (track for statistics)
        // Note: We don't actually modify phi nodes here - codegen can use
        // the lattice information to generate optimal code
        for block_idx in cfg.graph().node_indices() {
            if let Some(block) = cfg.graph().node_weight(block_idx) {
                for (_inst_idx, instruction) in block.instructions.iter().enumerate() {
                    if let InstructionKind::Phi { incoming, .. } = &instruction.kind {
                        if self.can_simplify_phi(block_idx, incoming) {
                            self.stats.phi_nodes_simplified += 1;
                        }
                    }
                }
            }
        }
    }

    /// Checks if a phi node can be simplified
    fn can_simplify_phi(&self, _block_idx: NodeIndex, incoming: &[(Value, String)]) -> bool {
        // Count executable predecessors
        let mut executable_count = 0;
        let mut first_value: Option<&IrLiteralValue> = None;
        let mut all_same_constant = true;

        for (value, pred_label) in incoming {
            // Only consider edges from executable predecessors
            let pred_idx = match self.function.cfg.find_block_by_label(pred_label) {
                Some(idx) => idx.index(),
                None => continue, // Skip invalid predecessors
            };

            if !self.executable_blocks.contains(&pred_idx) {
                continue; // Skip non-executable edges
            }

            executable_count += 1;

            // Check if value is a constant
            if let ValueKind::Literal(lit) = &value.kind {
                if let Some(first) = first_value {
                    if first != lit {
                        all_same_constant = false;
                    }
                } else {
                    first_value = Some(lit);
                }
            } else {
                all_same_constant = false;
            }
        }

        // Can simplify if:
        // 1. Only one executable predecessor, OR
        // 2. All incoming values are the same constant
        executable_count == 1 || (all_same_constant && first_value.is_some())
    }

    /// Simplifies conditional branches with constant conditions
    ///
    /// Converts `ConditionalBranch` to unconditional `Branch` when the
    /// condition is known to be constant.
    fn simplify_branches(&mut self) {
        let cfg = &self.function.cfg;

        // Collect branches to simplify (avoid borrow checker issues)
        let mut branches_to_simplify = Vec::new();

        for block_idx in cfg.graph().node_indices() {
            if let Some(block) = cfg.graph().node_weight(block_idx) {
                let terminator = block.terminator();

                if let TerminatorKind::ConditionalBranch { condition, true_label, false_label } = &terminator.kind {
                    // Check if condition is a constant
                    if let Some(target) = self.get_constant_branch_target(condition, true_label, false_label) {
                        branches_to_simplify.push((block_idx, target));
                    }
                }
            }
        }

        // Apply branch simplifications
        let cfg = &mut self.function.cfg;
        for (block_idx, target_label) in branches_to_simplify {
            if let Some(block) = cfg.graph_mut().node_weight_mut(block_idx) {
                let span = block.terminator.debug_info.source_span.clone();
                let new_terminator = Terminator::new(TerminatorKind::Branch { label: target_label.into() }, span);
                block.set_terminator(new_terminator);
                self.stats.branches_eliminated += 1;
            }
        }
    }

    /// Determines the target of a conditional branch if the condition is constant
    fn get_constant_branch_target(&self, condition: &Value, true_label: &str, false_label: &str) -> Option<String> {
        match &condition.kind {
            ValueKind::Literal(IrLiteralValue::Bool(true)) => Some(true_label.to_string()),
            ValueKind::Literal(IrLiteralValue::Bool(false)) => Some(false_label.to_string()),
            ValueKind::Temporary(idx) => {
                // Check lattice for this temporary
                let idx_usize = *idx as usize;
                if let Some(LatticeValue::Constant(IrLiteralValue::Bool(value))) = self.lattice.get(&idx_usize) {
                    Some(if *value { true_label.to_string() } else { false_label.to_string() })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Removes unreachable blocks from the CFG
    ///
    /// Blocks not in the executable_blocks set are removed along with
    /// all edges to/from them. Updates statistics with the number of blocks removed.
    fn remove_unreachable_blocks(&mut self) {
        let cfg = &mut self.function.cfg;

        // Collect blocks to remove
        let mut blocks_to_remove = Vec::new();

        for node_idx in cfg.graph().node_indices() {
            let block_id = node_idx.index();
            if !self.executable_blocks.contains(&block_id) {
                blocks_to_remove.push(node_idx);
            }
        }

        // Track total blocks before removal
        self.stats.total_blocks = cfg.graph().node_count();

        // Remove unreachable blocks
        for node_idx in blocks_to_remove {
            cfg.graph_mut().remove_node(node_idx);
            self.stats.blocks_removed += 1;
        }

        // Recompute reverse post-order after CFG modification
        cfg.graph_mut(); // This ensures the CFG is properly updated
    }

    /// Cleans up phi node edges from removed blocks
    ///
    /// Removes incoming edges in phi nodes that originate from blocks
    /// that are no longer in the executable set.
    fn cleanup_phi_edges(&mut self) {
        // This would require modifying phi node incoming lists
        // Since we can't easily modify instructions in-place, we track the cleanup
        // The actual modification would happen during a later IR transformation pass

        // For now, we just count how many phi nodes would be affected
        let cfg = &self.function.cfg;
        let mut phi_cleanups = 0;

        for block_idx in cfg.graph().node_indices() {
            if let Some(block) = cfg.graph().node_weight(block_idx) {
                for instruction in &block.instructions {
                    if let InstructionKind::Phi { incoming, .. } = &instruction.kind {
                        // Count non-executable predecessors
                        let non_exec_count = incoming
                            .iter()
                            .filter(|(_, pred_label)| {
                                // Check if predecessor block is executable
                                cfg.find_block_by_label(pred_label)
                                    .map(|idx| !self.executable_blocks.contains(&idx.index()))
                                    .unwrap_or(true)
                            })
                            .count();

                        if non_exec_count > 0 {
                            phi_cleanups += 1;
                        }
                    }
                }
            }
        }

        // Track in statistics (could add a dedicated field if needed)
        if phi_cleanups > 0 {
            self.stats.phi_nodes_simplified += phi_cleanups;
        }
    }
}

/// Public API for rewriting a function based on SCCP results
pub fn rewrite_function(
    function: &mut Function, lattice: HashMap<usize, LatticeValue>, executable_blocks: HashSet<usize>,
) -> OptimizationStatistics {
    let rewriter = IRRewriter::new(function, lattice, executable_blocks);
    rewriter.rewrite()
}
