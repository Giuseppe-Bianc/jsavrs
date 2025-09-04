// src/rvir/terminator.rs
use super::{RIrType, RValue};
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct RTerminator {
    pub kind: RTerminatorKind,
    pub debug_info: DebugInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RTerminatorKind {
    Return {
        value: RValue,
        ty: RIrType,
    },
    Branch {
        label: Arc<str>,
    },
    ConditionalBranch {
        condition: RValue,
        true_label: Arc<str>,
        false_label: Arc<str>,
    },
    IndirectBranch {
        address: RValue,
        possible_labels: Vec<String>,
    },
    Switch {
        value: RValue,
        ty: RIrType,
        default_label: String,
        cases: Vec<(RValue, String)>,
    },
    Unreachable,
}

impl RTerminator {
    pub fn is_terminator(&self) -> bool {
        !matches!(self.kind, RTerminatorKind::Unreachable)
    }

    pub fn get_targets(&self) -> Vec<String> {
        match &self.kind {
            RTerminatorKind::Branch { label } => vec![label.clone().to_string()],
            RTerminatorKind::ConditionalBranch {
                true_label,
                false_label,
                ..
            } => vec![true_label.clone().to_string(), false_label.clone().to_string()],
            RTerminatorKind::Switch {
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
            RTerminatorKind::IndirectBranch {
                possible_labels, ..
            } => possible_labels.clone(),
            _ => Vec::new(),
        }
    }

    pub fn new(kind: RTerminatorKind, span: SourceSpan) -> Self {
        RTerminator {
            kind,
            debug_info: DebugInfo { source_span: span },
        }
    }
}

impl fmt::Display for RTerminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            RTerminatorKind::Return { value, ty } => write!(f, "ret {value} {ty}"),
            RTerminatorKind::Branch { label } => write!(f, "br {label}"),
            RTerminatorKind::ConditionalBranch {
                condition,
                true_label,
                false_label,
            } => write!(f, "br {condition} ? {true_label} : {false_label}"),
            RTerminatorKind::Switch {
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
            RTerminatorKind::IndirectBranch {
                address,
                possible_labels,
            } => write!(f, "ibr {address} [{}]", possible_labels.join(", ")),
            RTerminatorKind::Unreachable => write!(f, "unreachable"),
        }
    }
}
