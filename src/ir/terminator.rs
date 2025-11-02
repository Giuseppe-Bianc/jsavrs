// src/ir/terminator.rs

use super::{IrType, Value};
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;

/// Represents a control-flow terminator in the intermediate representation (IR).
///
/// A terminator defines how the control flow exits a basic block —
/// such as returning from a function, branching to another block,
/// or handling unreachable code.
///
/// Each terminator includes debugging metadata (`DebugInfo`)
/// for accurate source location mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct Terminator {
    /// The specific kind of terminator (e.g., return, branch, switch, etc.).
    pub kind: TerminatorKind,
    /// Associated debug information, such as the originating source span.
    pub debug_info: DebugInfo,
}

/// Stores metadata about a terminator for debugging and diagnostics.
///
/// This includes the source code span that corresponds to the IR instruction,
/// enabling more accurate compiler error messages and source mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    /// The region in the source code where this terminator originated.
    pub source_span: SourceSpan,
}

/// Defines the different types of terminators that can appear in the IR.
///
/// Each variant represents a distinct way to transfer control or signal termination.
/// For instance, a `Return` exits the function, a `Branch` moves control to a new block,
/// and `Unreachable` marks code paths that cannot be executed.
#[derive(Debug, Clone, PartialEq)]
pub enum TerminatorKind {
    /// Returns control from the current function with a specified value and type.
    Return {
        /// The value being returned.
        value: Value,
        /// The type of the returned value.
        ty: IrType,
    },

    /// Performs an unconditional branch to a target block.
    Branch {
        /// The label of the target block.
        label: Arc<str>,
    },

    /// Performs a conditional branch based on a boolean condition.
    ConditionalBranch {
        /// The condition to evaluate.
        condition: Value,
        /// The label of the block executed when the condition is true.
        true_label: Arc<str>,
        /// The label of the block executed when the condition is false.
        false_label: Arc<str>,
    },

    /// Performs an indirect branch where the target is computed dynamically.
    ///
    /// This is typically used for jump tables or computed goto-like behavior.
    IndirectBranch {
        /// The address determining the jump destination.
        address: Value,
        /// A list of all possible target labels (for analysis or validation).
        possible_labels: Vec<String>,
    },

    /// A multi-way conditional branch, similar to a `switch` statement.
    ///
    /// Transfers control to a specific target block based on the value provided.
    Switch {
        /// The value being matched on.
        value: Value,
        /// The type of the value (usually an integer or enum).
        ty: IrType,
        /// The label of the default block (executed when no case matches).
        default_label: String,
        /// A list of case-value and target-label pairs.
        cases: Vec<(Value, String)>,
    },

    /// Marks code that should never be reached during normal execution.
    ///
    /// Often used to indicate unreachable compiler-generated paths or
    /// after fatal errors and assertions.
    Unreachable,
}

impl Terminator {
    /// Checks whether this terminator represents a valid control-flow terminator.
    ///
    /// # Returns
    /// - `true` if the terminator affects control flow (e.g., branch, return, switch).
    /// - `false` if it is `Unreachable`.
    pub fn is_terminator(&self) -> bool {
        !matches!(self.kind, TerminatorKind::Unreachable)
    }

    /// Retrieves all target labels that this terminator may transfer control to.
    ///
    /// This is useful for constructing or analyzing the control-flow graph (CFG).
    ///
    /// # Returns
    /// A vector of label strings representing all possible jump destinations.
    pub fn get_targets(&self) -> Vec<String> {
        match &self.kind {
            TerminatorKind::Branch { label } => vec![label.clone().to_string()],
            TerminatorKind::ConditionalBranch { true_label, false_label, .. } => {
                vec![true_label.clone().to_string(), false_label.clone().to_string()]
            }
            TerminatorKind::Switch { cases, default_label, .. } => {
                let mut targets = cases.iter().map(|(_, label)| label.clone()).collect::<Vec<_>>();
                targets.push(default_label.clone());
                targets
            }
            TerminatorKind::IndirectBranch { possible_labels, .. } => possible_labels.clone(),
            _ => Vec::new(),
        }
    }

    /// Retrieves all values used by this terminator.
    ///
    /// This is useful for liveness analysis to determine which values are live
    /// at the end of a basic block.
    ///
    /// # Returns
    /// A vector of references to values used by this terminator.
    pub fn get_used_values(&self) -> Vec<&Value> {
        match &self.kind {
            TerminatorKind::Return { value, .. } => vec![value],
            TerminatorKind::ConditionalBranch { condition, .. } => vec![condition],
            TerminatorKind::Switch { value, .. } => vec![value],
            TerminatorKind::IndirectBranch { address, .. } => vec![address],
            _ => Vec::new(),
        }
    }

    /// Creates a new [`Terminator`] with the given kind and source span.
    ///
    /// # Arguments
    /// * `kind` - The specific kind of terminator to create.
    /// * `span` - The source location metadata associated with this terminator.
    ///
    /// # Returns
    /// A new [`Terminator`] instance containing the given parameters.
    pub fn new(kind: TerminatorKind, span: SourceSpan) -> Self {
        Terminator { kind, debug_info: DebugInfo { source_span: span } }
    }
}

/// Provides a human-readable string representation of a [`Terminator`] instance.
///
/// This formatting is primarily used for debugging, IR visualization, or textual dumps.
/// Each terminator variant is represented using a concise and consistent syntax.
impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TerminatorKind::Return { value, ty } => write!(f, "ret {value} {ty}"),
            TerminatorKind::Branch { label } => write!(f, "br {label}"),
            TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
                write!(f, "br {condition} ? {true_label} : {false_label}")
            }
            TerminatorKind::Switch { value, ty, default_label, cases } => {
                let mut cases_str = String::new();
                for (idx, (val, label)) in cases.iter().enumerate() {
                    if idx > 0 {
                        cases_str.push_str(", ");
                    }
                    write!(&mut cases_str, "{val} => {label}")?;
                }
                write!(f, "switch {value} {ty}: {cases_str}, default {default_label}")
            }
            TerminatorKind::IndirectBranch { address, possible_labels } => {
                write!(f, "ibr {address} [{}]", possible_labels.join(", "))
            }
            TerminatorKind::Unreachable => write!(f, "unreachable"),
        }
    }
}
