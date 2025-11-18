//! Branch and terminator analysis for SCCP
//!
//! This module evaluates terminator instructions (branches, conditional jumps)
//! to determine which control-flow edges are executable.

use super::lattice::LatticeValue;
use crate::ir::terminator::{Terminator, TerminatorKind};
use crate::ir::value::ValueKind;
use crate::ir::{IrLiteralValue, Value};
use std::sync::Arc;

/// Result of evaluating a terminator instruction
#[derive(Debug, Clone)]
pub enum TerminatorEvaluation {
    /// Unconditional jump to a single target
    UnconditionalJump { target: Arc<str> },

    /// Conditional jump with both branches possible
    ConditionalJump { true_target: Arc<str>, false_target: Arc<str> },

    /// Only the true branch is executable (condition is constant true)
    OnlyTrueBranch { target: Arc<str> },

    /// Only the false branch is executable (condition is constant false)
    OnlyFalseBranch { target: Arc<str> },

    /// Multi-way switch with specific targets
    Switch { targets: Vec<Arc<str>> },

    /// Return or unreachable - no successors
    NoSuccessors,
}

/// Evaluates terminator instructions to determine executable successors
pub struct BranchAnalyzer;

impl BranchAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates a terminator and determines which successor blocks are executable
    ///
    /// # Arguments
    ///
    /// * `terminator` - The terminator instruction to evaluate
    /// * `get_lattice` - Function to retrieve lattice value for a given Value
    ///
    /// # Returns
    ///
    /// A `TerminatorEvaluation` indicating which successors are executable
    pub fn evaluate_terminator<F>(terminator: &Terminator, get_lattice: F) -> TerminatorEvaluation
    where
        F: Fn(&Value) -> LatticeValue,
    {
        match &terminator.kind {
            TerminatorKind::Branch { label } => Self::evaluate_unconditional_branch(label),

            TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
                Self::evaluate_conditional_branch(condition, true_label, false_label, get_lattice)
            }

            TerminatorKind::Switch { value, cases, default_label, ty: _ } => {
                Self::evaluate_switch_terminator(value, cases, &Arc::from(default_label.as_str()), get_lattice)
            }

            TerminatorKind::Return { .. } | TerminatorKind::Unreachable => TerminatorEvaluation::NoSuccessors,

            TerminatorKind::IndirectBranch { possible_labels, .. } => {
                // Conservative: assume all possible targets are executable
                TerminatorEvaluation::Switch {
                    targets: possible_labels.iter().map(|s| Arc::from(s.as_str())).collect(),
                }
            }
        }
    }

    /// Evaluates an unconditional branch (always executable)
    ///
    /// # Arguments
    ///
    /// * `target` - The target block label
    ///
    /// # Returns
    ///
    /// An unconditional jump to the target
    pub fn evaluate_unconditional_branch(target: &Arc<str>) -> TerminatorEvaluation {
        TerminatorEvaluation::UnconditionalJump { target: Arc::clone(target) }
    }

    /// Evaluates a conditional branch based on the condition's lattice value
    ///
    /// # Arguments
    ///
    /// * `condition` - The branch condition value
    /// * `true_label` - The label for the true branch
    /// * `false_label` - The label for the false branch
    /// * `get_lattice` - Function to retrieve lattice value for the condition
    ///
    /// # Returns
    ///
    /// Evaluation result indicating which branches are executable:
    /// - OnlyTrueBranch if condition is constant true
    /// - OnlyFalseBranch if condition is constant false
    /// - ConditionalJump if condition is Top or Bottom (both branches possible)
    pub fn evaluate_conditional_branch<F>(
        condition: &Value, true_label: &Arc<str>, false_label: &Arc<str>, get_lattice: F,
    ) -> TerminatorEvaluation
    where
        F: Fn(&Value) -> LatticeValue,
    {
        // First check if condition is a literal
        if let ValueKind::Literal(IrLiteralValue::Bool(value)) = &condition.kind {
            return if *value {
                TerminatorEvaluation::OnlyTrueBranch { target: Arc::clone(true_label) }
            } else {
                TerminatorEvaluation::OnlyFalseBranch { target: Arc::clone(false_label) }
            };
        }

        // Check lattice value
        let lattice_value = get_lattice(condition);
        match lattice_value {
            LatticeValue::Constant(IrLiteralValue::Bool(true)) => {
                TerminatorEvaluation::OnlyTrueBranch { target: Arc::clone(true_label) }
            }
            LatticeValue::Constant(IrLiteralValue::Bool(false)) => {
                TerminatorEvaluation::OnlyFalseBranch { target: Arc::clone(false_label) }
            }
            LatticeValue::Top | LatticeValue::Bottom | LatticeValue::Constant(_) => {
                // Unknown or non-boolean constant - both branches possible
                TerminatorEvaluation::ConditionalJump {
                    true_target: Arc::clone(true_label),
                    false_target: Arc::clone(false_label),
                }
            }
        }
    }

    /// Evaluates a switch/multi-way branch terminator
    ///
    /// # Arguments
    ///
    /// * `value` - The selector value for the switch
    /// * `cases` - The list of (constant_value, target_label) pairs
    /// * `default_label` - The default target if no case matches
    /// * `get_lattice` - Function to retrieve lattice value for the selector
    ///
    /// # Returns
    ///
    /// Evaluation result:
    /// - UnconditionalJump to specific case if selector is constant and matches
    /// - UnconditionalJump to default if selector is constant but no match
    /// - Switch with all targets if selector is Top/Bottom (unknown)
    pub fn evaluate_switch_terminator<F>(
        value: &Value, cases: &[(Value, String)], default_label: &Arc<str>, get_lattice: F,
    ) -> TerminatorEvaluation
    where
        F: Fn(&Value) -> LatticeValue,
    {
        // Check if selector is a constant
        let selector_lattice = get_lattice(value);

        if let LatticeValue::Constant(selector_const) = selector_lattice {
            // Try to find a matching case
            for (case_value, case_label) in cases {
                if let ValueKind::Literal(case_const) = &case_value.kind {
                    if case_const == &selector_const {
                        return TerminatorEvaluation::UnconditionalJump { target: Arc::from(case_label.as_str()) };
                    }
                }
            }

            // No match found - use default
            return TerminatorEvaluation::UnconditionalJump { target: Arc::clone(default_label) };
        }

        // Selector is unknown (Top or Bottom) - all targets are possible
        let mut targets: Vec<Arc<str>> = cases.iter().map(|(_, label)| Arc::from(label.as_str())).collect();
        targets.push(Arc::clone(default_label));

        TerminatorEvaluation::Switch { targets }
    }
}

impl Default for BranchAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Value;

    #[test]
    fn test_unconditional_branch() {
        let target = Arc::from("bb1");
        let result = BranchAnalyzer::evaluate_unconditional_branch(&target);

        match result {
            TerminatorEvaluation::UnconditionalJump { target: t } => {
                assert_eq!(t.as_ref(), "bb1");
            }
            _ => panic!("Expected UnconditionalJump"),
        }
    }

    #[test]
    fn test_conditional_branch_constant_true() {
        let condition = Value::new_literal(IrLiteralValue::Bool(true));
        let true_label = Arc::from("bb_true");
        let false_label = Arc::from("bb_false");

        let result =
            BranchAnalyzer::evaluate_conditional_branch(&condition, &true_label, &false_label, |_| LatticeValue::Top);

        match result {
            TerminatorEvaluation::OnlyTrueBranch { target } => {
                assert_eq!(target.as_ref(), "bb_true");
            }
            _ => panic!("Expected OnlyTrueBranch"),
        }
    }

    #[test]
    fn test_conditional_branch_constant_false() {
        let condition = Value::new_literal(IrLiteralValue::Bool(false));
        let true_label = Arc::from("bb_true");
        let false_label = Arc::from("bb_false");

        let result =
            BranchAnalyzer::evaluate_conditional_branch(&condition, &true_label, &false_label, |_| LatticeValue::Top);

        match result {
            TerminatorEvaluation::OnlyFalseBranch { target } => {
                assert_eq!(target.as_ref(), "bb_false");
            }
            _ => panic!("Expected OnlyFalseBranch"),
        }
    }
}
