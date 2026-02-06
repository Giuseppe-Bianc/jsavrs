//! Definition-use chain tracking for liveness analysis.

use crate::ir::Value;
use crate::ir::value::ValueId;
use std::collections::{HashMap, HashSet};

use super::InstructionIndex;

/// Definition-Use chains for tracking value dependencies.
///
/// Maps each value to the instructions that use it, and each instruction
/// to the values it defines and uses. Essential for liveness analysis.
#[derive(Debug, Clone)]
pub struct DefUseChains {
    /// Maps each value id to the set of instruction indices that use it.
    value_to_uses: HashMap<ValueId, HashSet<InstructionIndex>>,

    /// Maps each instruction to the ids of the values it uses.
    instruction_to_used_values: HashMap<InstructionIndex, HashSet<ValueId>>,

    /// Maps each instruction to the id of the value it defines (if any).
    instruction_to_defined_value: HashMap<InstructionIndex, ValueId>,
}

impl Drop for DefUseChains {
    fn drop(&mut self) {
        // Explicitly clear all HashMaps to release memory eagerly
        self.value_to_uses.clear();
        self.instruction_to_used_values.clear();
        self.instruction_to_defined_value.clear();
    }
}

impl DefUseChains {
    /// Creates an empty `DefUseChains` structure.
    pub fn new() -> Self {
        Self {
            value_to_uses: HashMap::new(),
            instruction_to_used_values: HashMap::new(),
            instruction_to_defined_value: HashMap::new(),
        }
    }

    /// Records that an instruction defines a value.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The index of the instruction that produces the definition.
    /// * `value` - A reference to the [`Value`] being defined. Only the `ValueId`
    ///   is stored internally for efficiency.
    ///
    /// # Note
    ///
    /// If the instruction already has a recorded definition, it will be overwritten.
    #[inline]
    pub fn add_definition(&mut self, inst_idx: InstructionIndex, value: &Value) {
        // Store only the inexpensive `ValueId` to avoid hashing/cloning full `Value`
        self.instruction_to_defined_value.insert(inst_idx, value.id);
    }

    /// Records that an instruction uses a value.
    #[inline]
    pub fn add_use(&mut self, inst_idx: InstructionIndex, value: &Value) {
        let id = value.id;
        self.instruction_to_used_values.entry(inst_idx).or_default().insert(id);
        self.value_to_uses.entry(id).or_default().insert(inst_idx);
    }

    /// Returns the set of instructions that use the given value.
    #[allow(dead_code)]
    pub fn get_uses(&self, value: &Value) -> HashSet<InstructionIndex> {
        self.value_to_uses.get(&value.id).cloned().unwrap_or_default()
    }

    /// Returns a reference to the mapping of instructions to the values they use.
    ///
    /// This provides read-only access to the instruction-to-used-values map,
    /// which is essential for analyzing value dependencies during liveness analysis.
    ///
    /// # Returns
    ///
    /// A reference to the `HashMap` mapping each instruction index to the set of values it uses.
    #[inline]
    pub const fn get_instruction_to_used_values(&self) -> &HashMap<InstructionIndex, HashSet<ValueId>> {
        &self.instruction_to_used_values
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
    pub fn get_defined_value(&self, inst_idx: &InstructionIndex) -> Option<ValueId> {
        self.instruction_to_defined_value.get(inst_idx).copied()
    }

    /// Checks if the given value has any uses.
    ///
    /// # Arguments
    ///
    /// * `value_id` - The [`ValueId`] to check for uses.
    ///
    /// # Returns
    ///
    /// `true` if at least one instruction uses this value, `false` otherwise.
    #[inline]
    pub fn has_uses(&self, value_id: ValueId) -> bool {
        self.value_to_uses.get(&value_id).is_some_and(|uses| !uses.is_empty())
    }

    /// Convenience wrapper to check uses by `Value` reference.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the [`Value`] to check for uses.
    ///
    /// # Returns
    ///
    /// `true` if at least one instruction uses this value, `false` otherwise.
    #[inline]
    pub fn has_uses_value(&self, value: &Value) -> bool {
        self.has_uses(value.id)
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
    /// Creates a new `LivenessInfo` for a dead value (never used).
    pub fn dead() -> Self {
        Self { first_use: None, last_use: None, used_in_blocks: HashSet::new() }
    }

    /// Creates a new `LivenessInfo` with the given use information.
    pub const fn with_uses(
        first_use: Option<InstructionIndex>, last_use: Option<InstructionIndex>,
        used_in_blocks: HashSet<petgraph::graph::NodeIndex>,
    ) -> Self {
        Self { first_use, last_use, used_in_blocks }
    }

    /// Returns whether this value is live (has at least one use).
    ///
    /// # Returns
    ///
    /// `true` if the value has at least one use (i.e., `last_use` is `Some`),
    /// `false` otherwise.
    pub const fn is_live(&self) -> bool {
        self.last_use.is_some()
    }
}
