//! Worklist data structures for SCCP algorithm
//!
//! This module implements the dual worklists used by SCCP:
//! - SSAWorkList: Queue of (definition → use) SSA edges
//! - FlowWorkList: Queue of (predecessor → successor) control-flow edges

use crate::ir::Value;
use std::collections::{HashSet, VecDeque};

/// Manages the queue of SSA edges that need reprocessing when a value's
/// lattice state changes.
///
/// Each edge represents (definition_value → use_instruction), indicating
/// that use_instruction depends on definition_value and should be
/// re-evaluated when definition_value's lattice state becomes more precise.
///
/// Duplicate prevention ensures each edge is processed at most once per
/// lattice state change, achieving O(edges) complexity.
pub struct SSAWorkList {
    /// FIFO queue of edges to process
    /// (block_index, instruction_index within block)
    queue: VecDeque<(usize, usize)>,

    /// Set of edges already enqueued in this lattice state
    /// Prevents redundant work when the same edge is triggered multiple times
    seen: HashSet<(usize, usize)>,
}
impl SSAWorkList {
    /// Creates a new empty SSA worklist
    pub fn new() -> Self {
        Self { queue: VecDeque::new(), seen: HashSet::new() }
    }

    /// Enqueues an SSA edge if not already in the queue.
    ///
    /// Returns true if the edge was newly enqueued, false if it was a duplicate.
    ///
    /// # Arguments
    ///
    /// * `block_idx` - The block index containing the instruction
    /// * `inst_idx` - The instruction index within the block
    ///
    /// # Complexity
    ///
    /// O(1) average case (HashSet insert + VecDeque push_back)
    pub fn enqueue(&mut self, block_idx: usize, inst_idx: usize) -> bool {
        let edge = (block_idx, inst_idx);
        if self.seen.insert(edge) {
            self.queue.push_back(edge);
            true
        } else {
            false
        }
    }

    /// Dequeues the next SSA edge to process (FIFO order).
    ///
    /// Returns None if the worklist is empty.
    ///
    /// # Complexity
    ///
    /// O(1) (VecDeque pop_front)
    pub fn dequeue(&mut self) -> Option<(usize, usize)> {
        self.queue.pop_front()
    }
    /// Returns true if the worklist is empty (no more edges to process)
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Clears the worklist and resets duplicate tracking
    /// (used when starting analysis of a new function)
    pub fn clear(&mut self) {
        self.queue.clear();
        self.seen.clear();
    }
}

impl Default for SSAWorkList {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages the queue of control-flow edges whose destination blocks
/// need visiting.
///
/// Each edge represents (predecessor_block → successor_block), indicating
/// that successor_block has become newly reachable and all its instructions
/// (including phi nodes) need evaluation.
///
/// Duplicate prevention ensures each CFG edge is processed at most once,
/// achieving O(edges) complexity.
pub struct FlowWorkList {
    /// FIFO queue of CFG edges to process
    /// (predecessor_block_index, successor_block_index)
    queue: VecDeque<(usize, usize)>,

    /// Set of edges already enqueued
    /// Prevents redundant work when the same block is reached via multiple paths
    seen: HashSet<(usize, usize)>,
}
impl FlowWorkList {
    /// Creates a new empty flow worklist
    pub fn new() -> Self {
        Self { queue: VecDeque::new(), seen: HashSet::new() }
    }

    /// Enqueues a CFG edge if not already in the queue.
    ///
    /// Returns true if the edge was newly enqueued, false if it was a duplicate.
    ///
    /// # Arguments
    ///
    /// * `pred_block` - The predecessor block index (source of control flow)
    /// * `succ_block` - The successor block index (destination of control flow)
    ///
    /// # Complexity
    ///
    /// O(1) average case (HashSet insert + VecDeque push_back)
    pub fn enqueue(&mut self, pred_block: usize, succ_block: usize) -> bool {
        let edge = (pred_block, succ_block);
        if self.seen.insert(edge) {
            self.queue.push_back(edge);
            true
        } else {
            false
        }
    }

    /// Dequeues the next CFG edge to process (FIFO order).
    ///
    /// Returns None if the worklist is empty.
    ///
    /// # Complexity
    ///
    /// O(1) (VecDeque pop_front)
    pub fn dequeue(&mut self) -> Option<(usize, usize)> {
        self.queue.pop_front()
    }
    /// Returns true if the worklist is empty (no more edges to process)
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Clears the worklist and resets duplicate tracking
    /// (used when starting analysis of a new function)
    pub fn clear(&mut self) {
        self.queue.clear();
        self.seen.clear()
    }
}

impl Default for FlowWorkList {
    fn default() -> Self {
        Self::new()
    }
}
