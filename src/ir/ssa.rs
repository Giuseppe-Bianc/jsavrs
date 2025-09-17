// src/ir/ssa.rs
//! Static Single Assignment (SSA) transformation for IR.
//!
//! This module implements algorithms for converting IR to SSA form, including:
//! - Phi-function insertion using dominance frontiers
//! - Variable renaming to ensure single assignment
//! - Conversion from memory-based variables to SSA registers
//! - Verification of SSA properties
//!
//! ## SSA Transformation Process
//!
//! The transformation follows these steps:
//! 1. Compute dominance information for the control flow graph
//! 2. Identify variables that need phi-functions (those defined in multiple blocks)
//! 3. Insert phi-functions at appropriate dominance frontiers
//! 4. Rename variables using a recursive dominator tree traversal
//! 5. Verify the resulting SSA form meets all requirements

use super::cfg::ControlFlowGraph;
use super::dominance::DominanceInfo;
use super::function::Function;
use super::instruction::{Instruction, InstructionKind};
use super::types::IrType;
use super::value::{Value, ValueKind};
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

/// Manages the SSA transformation process.
#[derive(Debug)]
pub struct SsaTransformer {
    /// Dominance information for the CFG
    dominance_info: DominanceInfo,
    /// Counter for generating unique variable names
    temp_counter: u64,
    /// Map from original variable names to their definitions in each block
    var_defs: HashMap<String, HashMap<NodeIndex, Vec<String>>>,
    /// Set of variables that need phi-functions
    phi_variables: HashSet<String>,
    /// Map from variable names to their current SSA values (stack for scoping)
    value_stack: HashMap<String, Vec<Value>>,
    /// Map to store the type of each variable
    variable_types: HashMap<String, IrType>,
}

impl SsaTransformer {
    /// Creates a new SSA transformer.
    pub fn new(temp_counter: Option<u64>) -> Self {
        Self {
            dominance_info: DominanceInfo::new(),
            temp_counter: temp_counter.unwrap_or(1000000),
            var_defs: HashMap::new(),
            phi_variables: HashSet::new(),
            value_stack: HashMap::new(),
            variable_types: HashMap::new(),
        }
    }

    /// Transforms a function to SSA form.
    ///
    /// # Arguments
    ///
    /// * `func` - The function to transform to SSA form
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the transformation was successful
    /// * `Err(String)` with an error message if the transformation failed
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// * Dominance computation fails
    /// * The function has no entry block
    /// * The SSA verification fails
    pub fn transform_function(&mut self, func: &mut Function) -> Result<(), String> {
        // Clear previous transformation data
        self.clear_transformation_data();

        // Compute dominance information
        self.dominance_info
            .compute_dominators(&func.cfg)
            .map_err(|e| format!("Failed to compute dominators: {}", e))?;
        self.dominance_info
            .compute_dominance_frontiers(&func.cfg)
            .map_err(|e| format!("Failed to compute dominance frontiers: {}", e))?;

        // Identify variables that need phi-functions
        self.identify_phi_variables(&func.cfg);

        // Insert phi-functions
        self.insert_phi_functions(&mut func.cfg);

        // Initialize value stacks for phi variables
        for var_name in &self.phi_variables {
            self.value_stack.insert(var_name.clone(), Vec::new());
        }

        // Get the entry block node index
        let entry_idx = func.cfg.get_entry_block_index().ok_or_else(|| "No entry block found".to_string())?;

        // Rename variables using recursive dominator tree traversal
        self.rename_variables_recursive(func, entry_idx);

        // Verify the transformation produced valid SSA form
        self.verify_ssa_form(func).map_err(|e| format!("SSA verification failed: {}", e))?;

        Ok(())
    }

    /// Clears transformation data between function transformations
    fn clear_transformation_data(&mut self) {
        self.var_defs.clear();
        self.phi_variables.clear();
        self.value_stack.clear();
        self.variable_types.clear();
        // Don't reset temp_counter here - it should be unique across the entire module
        // self.temp_counter = 0;
    }

    /// Identifies variables that need phi-functions by analyzing definitions.
    /// Only variables that are defined in multiple blocks need phi-functions.
    fn identify_phi_variables(&mut self, cfg: &ControlFlowGraph) {
        self.phi_variables.clear();
        self.var_defs.clear();
        self.variable_types.clear();

        // For each block, find variable definitions
        for node_idx in cfg.graph().node_indices() {
            if let Some(block) = cfg.get_block(&cfg.graph()[node_idx].label) {
                for instruction in &block.instructions {
                    match &instruction.kind {
                        // Check for store instructions that define variables
                        InstructionKind::Store { value: _, dest } => {
                            if let ValueKind::Temporary(temp_id) = &dest.kind {
                                // Get the variable name from debug info if available
                                let var_name = if let Some(debug_info) = &dest.debug_info {
                                    if let Some(name) = &debug_info.name {
                                        name.to_string()
                                    } else {
                                        format!("t{}", temp_id)
                                    }
                                } else {
                                    format!("t{}", temp_id)
                                };

                                // Store the variable type
                                if let IrType::Pointer(inner_ty) = &dest.ty {
                                    self.variable_types.insert(var_name.clone(), (**inner_ty).clone());
                                }

                                // Record this definition
                                self.var_defs
                                    .entry(var_name.clone())
                                    .or_insert_with(HashMap::new)
                                    .entry(node_idx)
                                    .or_insert_with(Vec::new)
                                    .push(var_name.clone());
                            }
                        }
                        // Check for alloca instructions that define variables
                        InstructionKind::Alloca { ty } => {
                            if let Some(result) = &instruction.result
                                && let ValueKind::Temporary(temp_id) = &result.kind
                            {
                                // Get the variable name from debug info if available
                                let var_name = if let Some(debug_info) = &result.debug_info {
                                    if let Some(name) = &debug_info.name {
                                        name.to_string()
                                    } else {
                                        format!("t{}", temp_id)
                                    }
                                } else {
                                    format!("t{}", temp_id)
                                };

                                // Store the variable type
                                self.variable_types.insert(var_name.clone(), ty.clone());

                                // Record this definition
                                self.var_defs
                                    .entry(var_name.clone())
                                    .or_insert_with(HashMap::new)
                                    .entry(node_idx)
                                    .or_insert_with(Vec::new)
                                    .push(var_name.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Only mark variables that are defined in multiple blocks as needing phi-functions
        for (var_name, defs) in &self.var_defs {
            if defs.len() > 1 {
                self.phi_variables.insert(var_name.clone());
            }
        }
    }

    /// Inserts phi-functions at dominance frontiers.
    fn insert_phi_functions(&mut self, cfg: &mut ControlFlowGraph) {
        // Collect all phi variables first to avoid borrowing issues
        let phi_vars: Vec<String> = self.phi_variables.iter().cloned().collect();

        // For each variable that needs phi-functions
        for var_name in &phi_vars {
            // Get the set of blocks where this variable is defined
            let def_blocks = if let Some(defs) = self.var_defs.get(var_name) {
                defs.keys().cloned().collect::<HashSet<_>>()
            } else {
                continue;
            };

            // Collect all nodes that need phi-functions first
            let mut nodes_needing_phi = Vec::new();

            // Worklist algorithm for placing phi-functions
            let mut worklist = def_blocks.clone();
            let mut added_phis = HashSet::new();

            while let Some(block) = worklist.iter().next().cloned() {
                worklist.remove(&block);

                // For each node in the dominance frontier of this block
                if let Some(frontier) = self.dominance_info.dominance_frontier(block) {
                    for &frontier_node in frontier {
                        // If we haven't added a phi-function here yet
                        if !added_phis.contains(&frontier_node) {
                            nodes_needing_phi.push((frontier_node, var_name.clone()));
                            added_phis.insert(frontier_node);

                            // If this node doesn't define the variable, add it to worklist
                            if !def_blocks.contains(&frontier_node) {
                                worklist.insert(frontier_node);
                            }
                        }
                    }
                }
            }

            // Now add all the phi-functions
            for (node, var_name) in nodes_needing_phi {
                self.add_phi_function(cfg, node, &var_name);
            }
        }
    }

    /// Adds a phi-function to a block.
    fn add_phi_function(&mut self, cfg: &mut ControlFlowGraph, node: NodeIndex, var_name: &str) {
        if let Some(block) = cfg.graph_mut().node_weight_mut(node) {
            // Get the variable type, defaulting to I32 if not found
            let ty = self.variable_types.get(var_name).cloned().unwrap_or(IrType::I32);

            // Create phi-function instruction with result that has debug info containing the variable name
            let phi_inst = Instruction::new(
                InstructionKind::Phi {
                    ty: ty.clone(),
                    incoming: vec![], // Will be filled during renaming
                },
                block.source_span.clone(),
            )
            .with_result(
                Value::new_temporary(self.temp_counter, ty.clone())
                    .with_debug_info(Some(var_name.into()), block.source_span.clone()),
            );

            self.temp_counter += 1;

            // Add to beginning of block
            block.instructions.insert(0, phi_inst);
        }
    }

    /// Replaces a value with the current SSA value from the stack if it's a variable
    fn replace_value_with_current_ssa(&mut self, value: &mut Value) {
        if let ValueKind::Temporary(temp_id) = &value.kind {
            // Get the variable name from debug info if available
            let var_name = if let Some(debug_info) = &value.debug_info {
                if let Some(name) = &debug_info.name { name.to_string() } else { format!("t{}", temp_id) }
            } else {
                format!("t{}", temp_id)
            };

            // Get the current value from the stack
            if let Some(stack) = self.value_stack.get(&var_name)
                && let Some(current_value) = stack.last()
            {
                *value = current_value.clone();
            }
        }
    }

    /// Renames variables to ensure single assignment using recursive dominator tree traversal.
    fn rename_variables_recursive(&mut self, func: &mut Function, block_idx: NodeIndex) {
        // Process this block
        self.process_block(func, block_idx);

        // Get children in dominator tree and collect them to avoid borrowing issues
        let children = if let Some(children) = self.dominance_info.dominator_tree_children(block_idx) {
            children.to_vec() // Convert to owned Vec to avoid borrowing conflicts
        } else {
            Vec::new()
        };

        // Recursively process children in dominator tree
        for child_idx in children {
            self.rename_variables_recursive(func, child_idx);
        }

        // Pop values from stack after processing children (to maintain proper scoping)
        self.pop_block_values(block_idx, func);
    }

    /// Pops values from the stack after processing a block's children
    fn pop_block_values(&mut self, block_idx: NodeIndex, func: &Function) {
        // Get block label
        let block_label = {
            let graph = func.cfg.graph();
            if let Some(block) = graph.node_weight(block_idx) {
                block.label.to_string()
            } else {
                return;
            }
        };

        // Get block
        let block = if let Some(block) = func.cfg.get_block(&block_label) {
            block
        } else {
            return;
        };

        // For each instruction in the block, if it defines a variable, pop it from the stack
        for instruction in &block.instructions {
            if let Some(result) = &instruction.result
                && let ValueKind::Temporary(_) = &result.kind
            {
                // Get the variable name from debug info if available
                if let Some(debug_info) = &result.debug_info
                    && let Some(var_name) = &debug_info.name
                {
                    // Pop the value from the stack - convert var_name to String
                    let var_name_string = var_name.to_string();
                    if let Some(stack) = self.value_stack.get_mut(&var_name_string) {
                        stack.pop();
                    }
                }
            }
        }
    }

    /// Process a single block during renaming
    fn process_block(&mut self, func: &mut Function, block_idx: NodeIndex) {
        // Get block label
        let block_label = {
            let graph = func.cfg.graph();
            if let Some(block) = graph.node_weight(block_idx) {
                block.label.to_string()
            } else {
                return;
            }
        };

        // Get mutable reference to block
        let block = if let Some(block) = func.cfg.get_block_mut(&block_label) {
            block
        } else {
            return;
        };

        // Process phi-functions in this block
        // First, collect information about phi-functions that need to be updated
        let mut phi_updates = Vec::new();
        for (i, instruction) in block.instructions.iter().enumerate() {
            if let InstructionKind::Phi { ty, .. } = &instruction.kind {
                // Store the index, type, and variable name for updating later
                if let Some(result) = &instruction.result {
                    if let Some(debug_info) = &result.debug_info {
                        if let Some(var_name) = &debug_info.name {
                            phi_updates.push((i, ty.clone(), var_name.to_string()));
                        }
                    }
                }
            }
        }

        // Update phi-functions
        for (i, ty, var_name) in phi_updates {
            // The phi-function already has a result value with debug info, but we need to update it
            // with a new unique SSA name
            let source_span =
                block.instructions[i].result.as_ref().unwrap().debug_info.as_ref().unwrap().source_span.clone();
            let new_value =
                Value::new_temporary(self.temp_counter, ty).with_debug_info(Some(var_name.clone().into()), source_span);
            self.temp_counter += 1;

            // Update the phi-function result, preserving the debug info that identifies the variable
            block.instructions[i].result = Some(new_value.clone());

            // Push the new value onto the stack
            self.value_stack.entry(var_name.clone()).or_insert_with(Vec::new).push(new_value);
        }

        // Process instructions in this block
        for instruction in &mut block.instructions {
            match &mut instruction.kind {
                InstructionKind::Store { value, dest } => {
                    // For store instructions, we need to replace the value with current SSA value
                    self.replace_value_with_current_ssa(value);

                    // For store instructions, we need to replace the destination with a new SSA value
                    if let ValueKind::Temporary(temp_id) = &dest.kind {
                        // Get the variable name from debug info if available
                        let var_name = if let Some(debug_info) = &dest.debug_info {
                            if let Some(name) = &debug_info.name { name.to_string() } else { format!("t{}", temp_id) }
                        } else {
                            format!("t{}", temp_id)
                        };

                        // Create a new unique name for this definition
                        let ty = dest.ty.clone();
                        let new_value = Value::new_temporary(self.temp_counter, ty).with_debug_info(
                            Some(var_name.clone().into()),
                            dest.debug_info.as_ref().map(|d| d.source_span.clone()).unwrap_or_default(),
                        );
                        self.temp_counter += 1;

                        // Update the store instruction's destination
                        *dest = new_value.clone();

                        // Push the new value onto the stack
                        self.value_stack.entry(var_name).or_insert_with(Vec::new).push(new_value);
                    }
                }
                InstructionKind::Load { src, .. } => {
                    // For load instructions, we need to replace the source with current SSA value
                    self.replace_value_with_current_ssa(src);
                }
                InstructionKind::Binary { left, right, .. } => {
                    // For binary operations, replace operands with current SSA values
                    self.replace_value_with_current_ssa(left);
                    self.replace_value_with_current_ssa(right);
                }
                InstructionKind::Unary { operand, .. } => {
                    // For unary operations, replace operand with current SSA value
                    self.replace_value_with_current_ssa(operand);
                }
                InstructionKind::Phi { .. } => {
                    // Phi-functions are already processed
                }
                InstructionKind::Call { func: callee, args, .. } => {
                    // For call instructions, we need to replace the function and arguments with current SSA values
                    self.replace_value_with_current_ssa(callee);
                    for arg in args {
                        self.replace_value_with_current_ssa(arg);
                    }
                }
                _ => {
                    // For other instructions, we might need to replace operands
                    // This is a simplified approach - a full implementation would be more comprehensive
                }
            }
        }

        // Process successors and add incoming edges to their phi-functions
        let successors: Vec<NodeIndex> =
            func.cfg.graph().neighbors_directed(block_idx, petgraph::Direction::Outgoing).collect();

        // For each variable that needs phi-functions, add its current value to phi-functions in successor blocks
        for var_name in &self.phi_variables {
            // Get the current value of this variable from the stack
            if let Some(stack) = self.value_stack.get(var_name) {
                if let Some(current_value) = stack.last() {
                    // Add this value to phi-functions for this variable in all successor blocks
                    for &succ_idx in &successors {
                        // Get the successor block label
                        let succ_label = {
                            let graph = func.cfg.graph();
                            if let Some(block) = graph.node_weight(succ_idx) {
                                block.label.to_string()
                            } else {
                                continue;
                            }
                        };

                        // Process phi-functions in the successor block
                        if let Some(succ_block) = func.cfg.get_block_mut(&succ_label) {
                            // Get the predecessor label for this edge
                            let pred_label = block_label.clone();

                            // Find the phi-function for this variable and add the incoming value
                            for instruction in succ_block.instructions.iter_mut() {
                                if let InstructionKind::Phi { ref mut incoming, .. } = instruction.kind {
                                    // Check if this phi-function is for the current variable by looking at the result's debug info
                                    if let Some(result) = &instruction.result {
                                        if let Some(debug_info) = &result.debug_info {
                                            if let Some(phi_var_name) = &debug_info.name {
                                                if phi_var_name.as_ref() == var_name {
                                                    // This is the phi-function for the current variable
                                                    incoming.push((current_value.clone(), pred_label.clone()));
                                                    break; // Found the phi-function, no need to continue
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Verifies that the function is in proper SSA form.
    /// In SSA form, each temporary ID should be unique across the entire function.
    pub fn verify_ssa_form(&self, func: &Function) -> Result<(), String> {
        let mut temp_ids = HashSet::new();

        // Collect all temporary IDs in the function
        for block in func.cfg.blocks() {
            for instruction in &block.instructions {
                if let Some(result) = &instruction.result {
                    if let ValueKind::Temporary(temp_id) = &result.kind {
                        // In SSA form, each temporary ID should be unique
                        if temp_ids.contains(temp_id) {
                            // This is an error - duplicate temporary ID
                            if let Some(debug_info) = &result.debug_info {
                                if let Some(var_name) = &debug_info.name {
                                    return Err(format!(
                                        "Variable '{}' has duplicate temporary ID {}",
                                        var_name, temp_id
                                    ));
                                }
                            }
                            return Err(format!("Duplicate temporary ID {}", temp_id));
                        }
                        temp_ids.insert(*temp_id);
                    }
                }
            }
        }

        Ok(())
    }

    /// Verifies semantic preservation by comparing control flow structures.
    pub fn verify_semantic_preservation(
        &self, original_func: &Function, transformed_func: &Function,
    ) -> Result<(), String> {
        // This is a simplified check - a full implementation would compare execution traces
        // For now, we'll just verify that the number of blocks remains the same
        let original_block_count = original_func.cfg.blocks().count();
        let transformed_block_count = transformed_func.cfg.blocks().count();

        if original_block_count != transformed_block_count {
            return Err("Control flow structure changed during transformation".to_string());
        }

        Ok(())
    }
}

impl Default for SsaTransformer {
    fn default() -> Self {
        Self::new(None)
    }
}
