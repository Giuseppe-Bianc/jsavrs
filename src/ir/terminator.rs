//src/ir/terminator.rs
use super::{IrType, Value};
use std::fmt;

/// Terminator instructions for basic blocks
#[derive(Debug, Clone, PartialEq)]
pub enum Terminator {
    Return(Value, IrType),
    Branch(String),
    ConditionalBranch {
        condition: Value,
        true_label: String,
        false_label: String,
    },
    Switch {
        value: Value,
        ty: IrType,
        default_label: String,
        cases: Vec<(Value, String)>, // Changed to Value for cases
    },
    Unreachable,
}

impl Terminator {
    pub fn is_terminator(&self) -> bool {
        !matches!(self, Terminator::Unreachable)
    }
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminator::Return(value, ty) => write!(f, "ret {} {}", value, ty),
            Terminator::Branch(label) => write!(f, "br {}", label),
            Terminator::ConditionalBranch {
                condition,
                true_label,
                false_label,
            } => write!(
                f,
                "br {} ? {} : {}",
                condition, true_label, false_label
            ),
            Terminator::Switch {
                value,
                ty,
                default_label,
                cases,
            } => {
                let cases_str = cases
                    .iter()
                    .map(|(val, label)| format!("{} => {}", val, label))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "switch {} {}: {} default {}", value, ty, cases_str, default_label)
            }
            Terminator::Unreachable => write!(f, "unreachable"),
        }
    }
}