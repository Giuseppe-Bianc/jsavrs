//! Dead Code Elimination (DCE) Optimization Phase
//!
//! Removes unreachable basic blocks and unused instructions from IR functions
//! while preserving all observable program behavior. Uses reachability analysis,
//! liveness analysis, and escape analysis to safely identify removable code.
//!
//! # Algorithm
//!
//! The optimization runs in a fixed-point loop:
//! 1. Reachability analysis: Mark blocks reachable from entry
//! 2. Block removal: Remove unreachable blocks, update CFG edges
//! 3. Liveness analysis: Compute live values via backward dataflow
//! 4. Escape analysis: Determine which allocations escape their scope
//! 5. Instruction removal: Remove dead instructions based on liveness/effects
//! 6. Repeat until no changes (fixed-point reached)
//!
//! # Module Organization
//!
//! - `analyzer` - Liveness and reachability analysis
//! - `def_use` - Definition-use chain tracking
//! - `escape` - Escape analysis for memory operations
//! - `stats` - Statistics and diagnostics
//! - `optimizer` - Main DCE optimization logic

mod analyzer;
mod def_use;
mod escape;
mod optimizer;
mod stats;

pub use optimizer::DeadCodeElimination;
pub use stats::{ConservativeReason, ConservativeWarning, OptimizationStats};

use petgraph::graph::NodeIndex;
use std::fmt;

// ============================================================================
// Shared Data Structures
// ============================================================================

/// Unique identifier for an instruction within a function.
///
/// Combines block index and instruction offset within that block
/// to provide a stable, comparable identifier for instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstructionIndex {
    /// The basic block containing this instruction.
    pub block_idx: NodeIndex,

    /// The offset of this instruction within the block's instruction list.
    pub inst_offset: usize,
}

impl InstructionIndex {
    /// Creates a new instruction index.
    pub fn new(block_idx: NodeIndex, inst_offset: usize) -> Self {
        Self { block_idx, inst_offset }
    }
}

impl fmt::Display for InstructionIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block[{}].inst[{}]", self.block_idx.index(), self.inst_offset)
    }
}
