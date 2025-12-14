// src/ir/dominance.rs
//! Dominance analysis for control flow graphs.

use super::cfg::ControlFlowGraph;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

/// Information about dominance relationships in a control flow graph.
#[derive(Debug, Clone)]
pub struct DominanceInfo {
    /// Immediate dominators for each node
    pub idom: HashMap<NodeIndex, Option<NodeIndex>>,
    /// Dominance frontiers for each node
    pub dominance_frontiers: HashMap<NodeIndex, HashSet<NodeIndex>>,
    /// Dominator tree children for each node
    pub dom_tree_children: HashMap<NodeIndex, Vec<NodeIndex>>,
}

impl DominanceInfo {
    /// Creates new empty dominance information.
    #[must_use]
    pub fn new() -> Self {
        Self { idom: HashMap::new(), dominance_frontiers: HashMap::new(), dom_tree_children: HashMap::new() }
    }

    /// Computes the dominator tree using the "A Simple, Fast Dominance Algorithm" by Keith D. Cooper, Timothy J. Harvey, and Ken Kennedy.
    ///
    /// # Errors
    ///
    /// Returns an error string if the CFG has no entry block.
    ///
    /// # Panics
    ///
    /// Panics if the internal dominator computation encounters an inconsistent state,
    /// which should never occur in a well-formed CFG. This represents an internal
    /// algorithm error rather than invalid input.
    pub fn compute_dominators(&mut self, cfg: &ControlFlowGraph) -> Result<(), String> {
        let entry_idx = cfg.get_entry_block_index().ok_or_else(|| "CFG has no entry block".to_string())?;

        // Initialize idom mappings
        self.idom.clear();

        // Set the entry node's immediate dominator to itself
        self.idom.insert(entry_idx, Some(entry_idx));

        // Get the precomputed reverse post-order traversal from CFG
        let reverse_post_order = cfg.reverse_post_order();

        // Filter out the entry node as it's already initialized
        let post_order: Vec<NodeIndex> =
            reverse_post_order.iter().filter(|&&node| node != entry_idx).copied().collect();

        // Initialize all other nodes to have no immediate dominator
        for &node in &post_order {
            self.idom.insert(node, None);
        }

        // Pre-compute predecessors for all nodes to avoid repeated computation
        let mut predecessors: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::with_capacity(post_order.len().min(1));
        for &node in &post_order {
            let preds: Vec<NodeIndex> = cfg.graph().neighbors_directed(node, petgraph::Direction::Incoming).collect();
            predecessors.insert(node, preds);
        }
        // Also add predecessors for entry node
        let entry_preds: Vec<NodeIndex> =
            cfg.graph().neighbors_directed(entry_idx, petgraph::Direction::Incoming).collect();
        predecessors.insert(entry_idx, entry_preds);

        // Iteratively compute immediate dominators
        let mut changed = true;
        while changed {
            changed = false;

            for &node in &post_order {
                let preds = &predecessors[&node];

                if preds.is_empty() {
                    // This shouldn't happen for nodes other than entry in a well-formed CFG
                    continue;
                }

                // Find the first processed predecessor
                let mut new_idom: Option<NodeIndex> = None;
                for &pred in preds {
                    if self.idom.get(&pred).and_then(|&x| x).is_some() {
                        new_idom = Some(pred);
                        break;
                    }
                }

                if new_idom.is_none() {
                    continue;
                }

                let mut new_idom = new_idom.unwrap();

                // Intersect the dominators of all processed predecessors
                for &pred in preds {
                    if pred != new_idom && self.idom.get(&pred).and_then(|&x| x).is_some() {
                        new_idom = self.intersect(new_idom, pred, &self.idom);
                    }
                }

                // Update if changed
                let current_idom = self.idom.get(&node).and_then(|&x| x);
                if current_idom != Some(new_idom) {
                    self.idom.insert(node, Some(new_idom));
                    changed = true;
                }
            }
        }

        // Build dominator tree children
        self.build_dominator_tree();

        Ok(())
    }

    /// Computes dominance frontiers for all nodes in the CFG.
    ///
    /// # Errors
    ///
    /// Returns an error string if the CFG has no entry block.
    pub fn compute_dominance_frontiers(&mut self, cfg: &ControlFlowGraph) -> Result<(), String> {
        self.dominance_frontiers.clear();

        let entry_idx = cfg.get_entry_block_index().ok_or_else(|| "CFG has no entry block".to_string())?;

        // For each node b
        for b in cfg.graph().node_indices() {
            // Get predecessors of b
            let preds: Vec<NodeIndex> = cfg.graph().neighbors_directed(b, petgraph::Direction::Incoming).collect();

            // If b has more than one predecessor, it's a join point
            if preds.len() >= 2 {
                // For each predecessor p of b
                for &p in &preds {
                    let mut runner = p;

                    // While runner does not dominate b
                    while !self.dominates(runner, b) && runner != entry_idx {
                        // Add b to runner's dominance frontier
                        self.dominance_frontiers.entry(runner).or_default().insert(b);

                        // Move up the dominator tree
                        if let Some(Some(idom)) = self.idom.get(&runner) {
                            runner = *idom;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Builds the dominator tree children mapping from the immediate dominators.
    fn build_dominator_tree(&mut self) {
        self.dom_tree_children.clear();

        for (&node, &idom_opt) in &self.idom {
            if let Some(idom) = idom_opt
                && node != idom
            {
                self.dom_tree_children.entry(idom).or_default().push(node);
            }
        }
    }

    /// Intersects two dominator paths to find their common ancestor.
    #[allow(clippy::expect_used, clippy::unused_self)]
    fn intersect(&self, node1: NodeIndex, node2: NodeIndex, idom: &HashMap<NodeIndex, Option<NodeIndex>>) -> NodeIndex {
        let mut finger1 = node1;
        let mut finger2 = node2;

        // Phase 1: Equalize depths in O(|d1 - d2|) instead of O(|d1 - d2| * 2)
        while finger1.index() > finger2.index() {
            finger1 = idom.get(&finger1).and_then(|&opt| opt).expect("Invalid dominator tree structure");
        }

        while finger2.index() > finger1.index() {
            finger2 = idom.get(&finger2).and_then(|&opt| opt).expect("Invalid dominator tree structure");
        }

        // Phase 2: Move both up simultaneously until convergence
        while finger1 != finger2 {
            finger1 = idom.get(&finger1).and_then(|&opt| opt).expect("Invalid dominator tree structure");
            finger2 = idom.get(&finger2).and_then(|&opt| opt).expect("Invalid dominator tree structure");
        }

        finger1
    }

    /// Checks if node1 dominates node2.
    #[must_use]
    pub fn dominates(&self, node1: NodeIndex, node2: NodeIndex) -> bool {
        let mut current = node2;

        // Walk up the dominator tree from node2
        while current != node1 {
            if let Some(Some(idom)) = self.idom.get(&current) {
                // If we reach a node without an immediate dominator, node1 doesn't dominate node2
                if *idom == current {
                    return false; // Reached root without finding node1
                }
                current = *idom;
            } else {
                return false; // No immediate dominator found
            }
        }

        true
    }

    /// Gets the immediate dominator of a node, if it exists.
    #[must_use]
    pub fn immediate_dominator(&self, node: NodeIndex) -> Option<NodeIndex> {
        self.idom.get(&node).and_then(|&x| x)
    }

    /// Gets the dominance frontier of a node.
    #[must_use]
    pub fn dominance_frontier(&self, node: NodeIndex) -> Option<&HashSet<NodeIndex>> {
        self.dominance_frontiers.get(&node)
    }

    /// Gets the children of a node in the dominator tree.
    #[must_use]
    pub fn dominator_tree_children(&self, node: NodeIndex) -> Option<&Vec<NodeIndex>> {
        self.dom_tree_children.get(&node)
    }
}

impl Default for DominanceInfo {
    fn default() -> Self {
        Self::new()
    }
}
