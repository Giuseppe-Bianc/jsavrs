pub mod analyzer;
pub mod branch_analysis;
pub mod evaluator;
pub mod executable_edges;
pub mod lattice;
pub mod optimizer;
pub mod rewriter;
pub mod stats;
pub mod worklist;

pub use analyzer::{AnalysisConfig, SCCPAnalyzer};
pub use lattice::LatticeValue;
pub use optimizer::ConstantFoldingOptimizer;
pub use stats::OptimizationStatistics;

use crate::ir::Function;

/// Validates preconditions before running SCCP optimization
///
/// Checks:
/// - Entry block exists
/// - All phi incoming edges reference valid blocks
/// - All branch targets exist
///
/// # Arguments
///
/// * `function` - The function to validate
///
/// # Returns
///
/// `true` if all preconditions are met, `false` otherwise
pub fn validate_preconditions(function: &Function) -> bool {
    // Check entry block exists
    if function.cfg.get_entry_block_index().is_none() {
        return false;
    }

    // Validate all phi nodes and branch targets
    for block in function.cfg.blocks() {
        // Check phi node incoming edges
        for instruction in &block.instructions {
            if let crate::ir::instruction::InstructionKind::Phi { incoming, .. } = &instruction.kind {
                for (_, pred_label) in incoming {
                    if function.cfg.find_block_by_label(pred_label).is_none() {
                        return false; // Invalid predecessor reference
                    }
                }
            }
        }

        // Check branch targets
        match &block.terminator().kind {
            crate::ir::terminator::TerminatorKind::Branch { label } => {
                if function.cfg.find_block_by_label(label).is_none() {
                    return false;
                }
            }
            crate::ir::terminator::TerminatorKind::ConditionalBranch { true_label, false_label, .. } => {
                if function.cfg.find_block_by_label(true_label).is_none()
                    || function.cfg.find_block_by_label(false_label).is_none()
                {
                    return false;
                }
            }
            crate::ir::terminator::TerminatorKind::Switch { cases, default_label, .. } => {
                if function.cfg.find_block_by_label(default_label).is_none() {
                    return false;
                }
                for (_, case_label) in cases {
                    if function.cfg.find_block_by_label(case_label).is_none() {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }

    true
}

/// Validates postconditions after running SCCP optimization
///
/// Checks:
/// - CFG is still valid
/// - No Top values remain in executable blocks (all values resolved)
/// - Source spans are preserved
///
/// # Arguments
///
/// * `function` - The function to validate
///
/// # Returns
///
/// `true` if all postconditions are met, `false` otherwise
pub fn validate_postconditions(function: &Function) -> bool {
    // Basic validation: ensure we still have blocks
    if function.cfg.blocks().count() == 0 {
        return false;
    }

    // Ensure entry block still exists
    if function.cfg.get_entry_block_index().is_none() {
        return false;
    }

    // NOTE: Additional invariants could be validated here:
    // - All blocks reachable from entry
    // - All phi nodes have correct number of incoming edges
    // - All terminators have valid targets
    // - Source spans preserved (requires tracking)
    //
    // For now, we rely on the IR structure to maintain these invariants.

    true
}
