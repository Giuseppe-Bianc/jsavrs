// src/nir/terminator.rs
use super::{IrType, Value};
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Terminator {
    pub kind: TerminatorKind,
    pub debug_info: DebugInfo, // Added debug info
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminatorKind {
    Return {
        value: Value,
        ty: IrType,
    },
    Branch {
        label: String,
    },
    ConditionalBranch {
        condition: Value,
        true_label: String,
        false_label: String,
    },
    IndirectBranch {
        address: Value,
        possible_labels: Vec<String>,
    },
    Switch {
        value: Value,
        ty: IrType,
        default_label: String,
        cases: Vec<(Value, String)>,
    },
    Unreachable,
}

impl Terminator {
    pub fn is_terminator(&self) -> bool {
        !matches!(self.kind, TerminatorKind::Unreachable)
    }

    pub fn get_targets(&self) -> Vec<String> {
        match &self.kind {
            TerminatorKind::Branch { label } => vec![label.clone()],
            TerminatorKind::ConditionalBranch {
                true_label,
                false_label,
                ..
            } => {
                vec![true_label.clone(), false_label.clone()]
            }
            TerminatorKind::Switch {
                cases,
                default_label,
                ..
            } => {
                let mut targets = cases
                    .iter()
                    .map(|(_, label)| label.clone())
                    .collect::<Vec<_>>();
                targets.push(default_label.clone());
                targets
            }
            TerminatorKind::IndirectBranch {
                possible_labels, ..
            } => possible_labels.clone(),
            _ => Vec::new(),
        }
    }

    pub fn new(kind: TerminatorKind, span: SourceSpan) -> Self {
        Terminator {
            kind,
            debug_info: DebugInfo { source_span: span },
        }
    }
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TerminatorKind::Return { value, ty } => write!(f, "ret {value} {ty}"),
            TerminatorKind::Branch { label } => write!(f, "br {label}"),
            TerminatorKind::ConditionalBranch {
                condition,
                true_label,
                false_label,
            } => write!(f, "br {condition} ? {true_label} : {false_label}"),
            TerminatorKind::Switch {
                value,
                ty,
                default_label,
                cases,
            } => {
                let mut cases_str = String::new();
                for (idx, (val, label)) in cases.iter().enumerate() {
                    if idx > 0 {
                        cases_str.push_str(", ");
                    }
                    write!(&mut cases_str, "{val} => {label}")?;
                }
                write!(
                    f,
                    "switch {value} {ty}: {cases_str}, default {default_label}"
                )
            }
            TerminatorKind::IndirectBranch {
                address,
                possible_labels,
            } => write!(f, "ibr {address} [{}]", possible_labels.join(", ")),
            TerminatorKind::Unreachable => write!(f, "unreachable"),
        }
    }
}
