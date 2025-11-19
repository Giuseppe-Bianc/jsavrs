//! Optimization statistics collection for SCCP
//!
//! This module tracks and reports metrics about the SCCP optimization process.

use std::fmt;

/// Collects statistics about SCCP optimization results
#[derive(Debug, Clone, Default)]
pub struct OptimizationStatistics {
    /// Number of SSA values proven to be constant
    pub constants_found: usize,

    /// Total number of SSA values analyzed
    pub total_values: usize,

    /// Number of conditional branches eliminated
    pub branches_eliminated: usize,

    /// Number of unreachable blocks removed
    pub blocks_removed: usize,

    /// Total number of blocks analyzed
    pub total_blocks: usize,

    /// Number of instructions replaced with constants
    pub instructions_replaced: usize,

    /// Number of phi nodes simplified
    pub phi_nodes_simplified: usize,

    /// Number of iterations to reach fixed-point
    pub iterations: usize,
}

impl OptimizationStatistics {
    /// Creates a new statistics tracker with all metrics initialized to zero
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the percentage of values found to be constant
    pub fn constant_percentage(&self) -> f64 {
        if self.total_values == 0 { 0.0 } else { (self.constants_found as f64 / self.total_values as f64) * 100.0 }
    }

    /// Returns the percentage of blocks removed
    pub fn block_removal_percentage(&self) -> f64 {
        if self.total_blocks == 0 { 0.0 } else { (self.blocks_removed as f64 / self.total_blocks as f64) * 100.0 }
    }
}

impl fmt::Display for OptimizationStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SCCP Optimization Statistics:")?;
        writeln!(f, "  Constants found: {} ({:.1}%)", self.constants_found, self.constant_percentage())?;
        writeln!(f, "  Branches eliminated: {}", self.branches_eliminated)?;
        writeln!(f, "  Blocks removed: {} ({:.1}%)", self.blocks_removed, self.block_removal_percentage())?;
        writeln!(f, "  Instructions replaced: {}", self.instructions_replaced)?;
        writeln!(f, "  Phi nodes simplified: {}", self.phi_nodes_simplified)?;
        writeln!(f, "  Iterations to convergence: {}", self.iterations)?;
        writeln!(f, "  Total values analyzed: {}", self.total_values)?;
        writeln!(f, "  Total blocks analyzed: {}", self.total_blocks)?;
        Ok(())
    }
}
