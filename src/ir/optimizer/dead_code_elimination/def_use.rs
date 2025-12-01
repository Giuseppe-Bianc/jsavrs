//! Definition-use chain tracking for liveness analysis.

use crate::ir::Value;
use std::collections::{HashMap, HashSet};

use super::InstructionIndex;

/// Definition-Use chains for tracking value dependencies.
///
/// Maps each value to the instructions that use it, and each instruction
/// to the values it defines and uses. Essential for liveness analysis.
#[derive(Debug, Clone)]
pub struct DefUseChains {
    /// Maps each value to the set of instruction indices that use it.
    value_to_uses: HashMap<Value, HashSet<InstructionIndex>>,

    /// Maps each instruction to the values it uses.
    pub(super) instruction_to_used_values: HashMap<InstructionIndex, HashSet<Value>>,

    /// Maps each instruction to the value it defines (if any).
    instruction_to_defined_value: HashMap<InstructionIndex, Value>,
}

impl DefUseChains {
    /// Creates an empty DefUseChains structure.
    pub fn new() -> Self {
        Self {
            value_to_uses: HashMap::new(),
            instruction_to_used_values: HashMap::new(),
            instruction_to_defined_value: HashMap::new(),
        }
    }

    /// Records that an instruction defines a value.
    #[inline]
    pub fn add_definition(&mut self, inst_idx: InstructionIndex, value: &Value) {
        self.instruction_to_defined_value.insert(inst_idx, value.clone());
    }

    /// Records that an instruction uses a value.
    #[inline]
    pub fn add_use(&mut self, inst_idx: InstructionIndex, value: &Value) {
        self.instruction_to_used_values.entry(inst_idx).or_default().insert(value.clone());
        self.value_to_uses.entry(value.clone()).or_default().insert(inst_idx);
    }

    /// Returns the set of instructions that use the given value.
    #[allow(dead_code)]
    pub fn get_uses(&self, value: &Value) -> HashSet<InstructionIndex> {
        self.value_to_uses.get(value).cloned().unwrap_or_default()
    }

    /// Returns the value defined by the given instruction, if any.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The instruction index to query
    ///
    /// # Returns
    ///
    /// `Some(&value)` if the instruction defines a value, `None` otherwise.
    pub fn get_defined_value(&self, inst_idx: &InstructionIndex) -> Option<&Value> {
        self.instruction_to_defined_value.get(inst_idx)
    }

    /// Checks if the given value has any uses.
    #[inline]
    pub fn has_uses(&self, value: &Value) -> bool {
        self.value_to_uses.get(value).is_some_and(|uses| !uses.is_empty())
    }
}

impl Default for DefUseChains {
    fn default() -> Self {
        Self::new()
    }
}

/// Liveness information for a value.
///
/// Tracks where a value is first defined, last used, and which blocks
/// reference it. Used to determine if a value's defining instruction
/// can be eliminated.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LivenessInfo {
    /// The instruction index where this value is first used.
    pub first_use: Option<InstructionIndex>,

    /// The instruction index where this value is last used.
    pub last_use: Option<InstructionIndex>,

    /// Set of basic blocks where this value is referenced.
    pub used_in_blocks: HashSet<petgraph::graph::NodeIndex>,
}

#[allow(dead_code)]
impl LivenessInfo {
    /// Creates a new LivenessInfo for a dead value (never used).
    pub fn dead() -> Self {
        Self { first_use: None, last_use: None, used_in_blocks: HashSet::new() }
    }

    /// Creates a new LivenessInfo with the given use information.
    pub fn with_uses(
        first_use: Option<InstructionIndex>, last_use: Option<InstructionIndex>,
        used_in_blocks: HashSet<petgraph::graph::NodeIndex>,
    ) -> Self {
        Self { first_use, last_use, used_in_blocks }
    }

    /// Returns whether this value is live (has at least one use).
    pub fn is_live(&self) -> bool {
        self.last_use.is_some()
    }
}
