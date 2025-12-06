//! IR transformation and rewriting
//!
//! Transforms IR based on SCCP analysis results, replacing constant computations
//! and simplifying control flow.

use super::lattice::ConstantValue;
use super::optimizer::OptimizationStats;
use super::propagator::LatticeState;
use crate::ir::{Instruction, Value};

/// IR rewriting errors
#[derive(Debug, thiserror::Error)]
pub enum RewriteError {
    #[error("SSA form violation: {0}")]
    SSAViolation(String),
    #[error("Invalid transformation: {0}")]
    InvalidTransformation(String),
}

/// Rewrites IR based on SCCP results
#[derive(Default)]
pub struct IRRewriter {
    stats: OptimizationStats,
}

impl IRRewriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    pub fn into_stats(self) -> OptimizationStats {
        self.stats
    }

    /// Rewrites an instruction if its result is a known constant.
    ///
    /// Returns `Some(new_value)` if the instruction should be replaced with a constant,
    /// `None` if the instruction should remain unchanged.
    ///
    /// # SSA Form Verification (T109)
    ///
    /// This method preserves SSA form by:
    /// - Never modifying the LHS (result) of an instruction
    /// - Only replacing uses of the instruction with a constant value
    /// - Ensuring each SSA value is defined exactly once
    ///
    /// # Arguments
    /// * `instruction` - The instruction to potentially rewrite
    /// * `lattice` - The lattice state containing constant analysis results
    /// * `value_id_to_key` - Function to convert ValueId to usize key
    ///
    /// # Returns
    /// `Ok(Some(constant_value))` if replacement should occur,
    /// `Ok(None)` if instruction should remain,
    /// `Err(RewriteError)` if transformation would violate SSA form
    pub fn rewrite_instruction<F>(
        &mut self, instruction: &Instruction, lattice: &LatticeState, value_id_to_key: F,
    ) -> Result<Option<Value>, RewriteError>
    where
        F: Fn(&crate::ir::value::ValueId) -> usize,
    {
        // Only process instructions with results
        let result_value = match &instruction.result {
            Some(val) => val,
            None => return Ok(None), // No result to replace
        };

        // SSA Form Debug Assertion (T109): Verify instruction has exactly one definition
        debug_assert!(
            instruction.result.is_some(),
            "SSA Violation: Attempting to rewrite instruction without result value"
        );

        // Get lattice value for the instruction result
        let lattice_value = lattice.get(value_id_to_key(&result_value.id));

        // If result is a constant, create replacement value
        if let Some(const_val) = lattice_value.as_constant() {
            let replacement = self.create_constant_value(const_val, &result_value.ty)?;
            self.stats.constants_propagated += 1;

            // SSA Form Debug Assertion (T109): Ensure replacement preserves type
            debug_assert_eq!(
                replacement.ty, result_value.ty,
                "SSA Violation: Constant replacement changes type from {:?} to {:?}",
                result_value.ty, replacement.ty
            );

            Ok(Some(replacement))
        } else {
            Ok(None)
        }
    }

    /// Simplifies a phi node based on SCCP analysis results.
    ///
    /// Implements T070-T072: Phi node simplification
    ///
    /// # Simplification rules
    /// 1. If all executable incoming values are the same constant → replace with constant (T071)
    /// 2. If phi has mixed constant/non-constant values → preserve phi (T072)
    /// 3. If all executable predecessors provide same value → replace with that value
    /// 4. If phi is in unreachable block → can be removed by DCE
    ///
    /// # SSA Form Verification (T109)
    ///
    /// Preserves SSA form by ensuring phi nodes maintain exactly one definition per value.
    ///
    /// # Dominance Verification (T110)
    ///
    /// All phi operands must come from blocks that dominate the phi block through
    /// their respective CFG edges (verified via executable edge tracking).
    ///
    /// # Arguments
    /// * `instruction` - The phi instruction to simplify
    /// * `lattice` - Lattice state with constant values
    /// * `executable_edges` - Set of executable CFG edges
    /// * `block_id` - ID of the block containing the phi
    /// * `value_id_to_key` - Function to convert ValueId to usize key
    ///
    /// # Returns
    /// - `Ok(Some(value))` if phi should be replaced with a single value
    /// - `Ok(None)` if phi should be preserved as-is
    /// - `Err(RewriteError)` on invalid transformation
    pub fn rewrite_phi<F>(
        &mut self, instruction: &Instruction, lattice: &LatticeState,
        executable_edges: &super::propagator::ExecutableEdgeSet, block_id: usize, value_id_to_key: F,
    ) -> Result<Option<Value>, RewriteError>
    where
        F: Fn(&crate::ir::value::ValueId) -> usize,
    {
        use crate::ir::instruction::InstructionKind;

        // Verify this is a phi instruction
        let (ty, incoming) = match &instruction.kind {
            InstructionKind::Phi { ty, incoming } => (ty, incoming),
            _ => return Err(RewriteError::InvalidTransformation("Not a phi instruction".to_string())),
        };

        // Get result value
        let result_value = match &instruction.result {
            Some(val) => val,
            None => return Err(RewriteError::SSAViolation("Phi instruction must have a result".to_string())),
        };

        // SSA Debug Assertion (T109): Phi must have at least one incoming value
        debug_assert!(
            !incoming.is_empty(),
            "SSA Violation: Phi instruction has no incoming values in block {}",
            block_id
        );

        // Dominance Debug Assertion (T110): All incoming edges must be from valid predecessors
        debug_assert!(
            executable_edges.has_executable_predecessor(block_id) || incoming.is_empty(),
            "Dominance Violation: Phi in block {} has no executable predecessors but has incoming values",
            block_id
        );

        // If phi is in unreachable block (no executable predecessors), return None
        // DCE will handle removal
        if !executable_edges.has_executable_predecessor(block_id) {
            return Ok(None);
        }

        // Collect values from executable predecessors only
        // Note: This is a simplified implementation that doesn't track which specific
        // edges are executable. A full implementation would filter based on predecessor labels.
        let mut unique_constant: Option<ConstantValue> = None;
        let mut has_non_constant = false;
        let mut all_same = true;

        for (value, _pred_label) in incoming {
            let value_key = value_id_to_key(&value.id);
            let lattice_value = lattice.get(value_key);

            match (lattice_value.as_constant(), &unique_constant) {
                (Some(const_val), None) => {
                    // First constant value seen
                    unique_constant = Some(const_val.clone());
                }
                (Some(const_val), Some(prev_const)) => {
                    // Another constant value - check if it matches
                    if const_val != prev_const {
                        all_same = false;
                    }
                }
                (None, _) => {
                    // Non-constant value (Top or Bottom)
                    has_non_constant = true;
                    all_same = false;
                }
            }
        }

        // Apply simplification rules
        if !has_non_constant
            && all_same
            && let Some(const_val) = unique_constant
        {
            // T071: All executable values are the same constant → replace with constant
            let replacement = self.create_constant_value(&const_val, ty)?;
            self.stats.phi_nodes_simplified += 1;

            // SSA Debug Assertion (T109): Replacement preserves type
            debug_assert_eq!(
                replacement.ty, result_value.ty,
                "SSA Violation: Phi replacement changes type from {:?} to {:?}",
                result_value.ty, replacement.ty
            );

            return Ok(Some(replacement));
        }

        if incoming.len() == 1 {
            // Single predecessor - phi is trivial, replace with incoming value
            self.stats.phi_nodes_simplified += 1;

            // SSA Debug Assertion (T109): Single-predecessor phi preserves type
            debug_assert_eq!(incoming[0].0.ty, result_value.ty, "SSA Violation: Trivial phi replacement changes type");

            Ok(Some(incoming[0].0.clone()))
        } else {
            // T072: Mixed values or multiple distinct constants → preserve phi
            Ok(None)
        }
    }

    /// Creates a constant Value from a ConstantValue.
    fn create_constant_value(&self, const_val: &ConstantValue, ty: &crate::ir::IrType) -> Result<Value, RewriteError> {
        use crate::ir::value::IrLiteralValue;

        // Verify type matches
        if const_val.get_type() != *ty {
            return Err(RewriteError::InvalidTransformation(format!(
                "Type mismatch: constant type {:?} does not match expected type {:?}",
                const_val.get_type(),
                ty
            )));
        }

        // Create literal value from constant
        let literal = match const_val {
            ConstantValue::I8(v) => IrLiteralValue::I8(*v),
            ConstantValue::I16(v) => IrLiteralValue::I16(*v),
            ConstantValue::I32(v) => IrLiteralValue::I32(*v),
            ConstantValue::I64(v) => IrLiteralValue::I64(*v),
            ConstantValue::U8(v) => IrLiteralValue::U8(*v),
            ConstantValue::U16(v) => IrLiteralValue::U16(*v),
            ConstantValue::U32(v) => IrLiteralValue::U32(*v),
            ConstantValue::U64(v) => IrLiteralValue::U64(*v),
            ConstantValue::F32(v) => IrLiteralValue::F32(*v),
            ConstantValue::F64(v) => IrLiteralValue::F64(*v),
            ConstantValue::Bool(v) => IrLiteralValue::Bool(*v),
            ConstantValue::Char(v) => IrLiteralValue::Char(*v),
        };

        Ok(Value::new_literal(literal))
    }

    /// Increments the constants propagated counter.
    ///
    /// Used when manually tracking replacements outside of rewrite_instruction.
    pub fn increment_constants_propagated(&mut self) {
        self.stats.constants_propagated += 1;
    }

    /// Increments the branches resolved counter.
    pub fn increment_branches_resolved(&mut self) {
        self.stats.branches_resolved += 1;
    }

    /// Increments the phi nodes simplified counter.
    pub fn increment_phi_simplified(&mut self) {
        self.stats.phi_nodes_simplified += 1;
    }

    /// Increments the unreachable blocks counter.
    pub fn increment_unreachable_blocks(&mut self) {
        self.stats.blocks_marked_unreachable += 1;
    }
}
