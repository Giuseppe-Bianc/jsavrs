//! Executable edge tracking for control-flow analysis
//!
//! This module tracks which CFG edges and blocks are proven reachable
//! during SCCP analysis.

use std::collections::HashSet;

/// Tracks which control-flow edges and blocks are proven executable
/// during SCCP analysis.
///
/// An edge or block is "executable" if it can be reached from the entry
/// block along some path of executable edges.
pub struct ExecutableEdges {
    /// Set of executable CFG edges (predecessor_index → successor_index)
    edges: HashSet<(usize, usize)>,

    /// Set of executable block indices
    blocks: HashSet<usize>,

    /// Counter for edge processing (debug/validation)
    edge_visits: usize,
}
impl ExecutableEdges {
    /// Creates a new empty executable edges tracker
    pub fn new() -> Self {
        Self { edges: HashSet::new(), blocks: HashSet::new(), edge_visits: 0 }
    }

    /// Marks a CFG edge as executable.
    ///
    /// Returns true if the edge was newly marked (not previously executable).
    ///
    /// # Arguments
    ///
    /// * `pred` - The predecessor block index
    /// * `succ` - The successor block index
    pub fn mark_edge_executable(&mut self, pred: usize, succ: usize) -> bool {
        let edge = (pred, succ);
        if self.edges.insert(edge) {
            self.blocks.insert(succ);
            self.edge_visits += 1;
            true
        } else {
            false
        }
    }

    /// Marks a block as executable without a specific edge.
    ///
    /// Used for marking the entry block as executable initially.
    ///
    /// # Arguments
    ///
    /// * `block` - The block index to mark as executable
    pub fn mark_block_executable(&mut self, block: usize) {
        self.blocks.insert(block);
    }

    /// Returns true if the given block is executable
    pub fn is_block_executable(&self, block: usize) -> bool {
        self.blocks.contains(&block)
    }

    /// Returns true if the given edge is executable
    pub fn is_edge_executable(&self, pred: usize, succ: usize) -> bool {
        self.edges.contains(&(pred, succ))
    }

    /// Returns the set of executable blocks
    pub fn executable_blocks(&self) -> &HashSet<usize> {
        &self.blocks
    }
    /// Returns the number of edge visits (for complexity validation)
    pub fn edge_visits(&self) -> usize {
        self.edge_visits
    }
}

impl Default for ExecutableEdges {
    fn default() -> Self {
        Self::new()
    }
}
