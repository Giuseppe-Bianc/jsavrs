// src/ir/ssa.rs
//! Static Single Assignment (SSA) transformation for IR.
//!
//! This module implements algorithms for converting IR to SSA form, including:
//! - Phi-function insertion using dominance frontiers
//! - Variable renaming to ensure single assignment
//! - Conversion from memory-based variables to SSA registers

use super::cfg::ControlFlowGraph;
use super::dominance::DominanceInfo;
use super::function::Function;
use super::instruction::{Instruction, InstructionKind};
use super::value::ValueKind;
use super::types::IrType;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

/// Manages the SSA transformation process.
#[derive(Debug)]
pub struct SsaTransformer {
    /// Dominance information for the CFG
    dominance_info: DominanceInfo,
    /// Counter for generating unique variable names
    // temp_counter: u64,
    /// Map from original variable names to their definitions in each block
    var_defs: HashMap<String, HashMap<NodeIndex, Vec<String>>>,
    /// Set of variables that need phi-functions
    phi_variables: HashSet<String>,
}

impl SsaTransformer {
    /// Creates a new SSA transformer.
    pub fn new() -> Self {
        Self {
            dominance_info: DominanceInfo::new(),
            // temp_counter: 0,
            var_defs: HashMap::new(),
            phi_variables: HashSet::new(),
        }
    }

    /// Transforms a function to SSA form.
    pub fn transform_function(&mut self, func: &mut Function) -> Result<(), String> {
        // Compute dominance information
        self.dominance_info.compute_dominators(&func.cfg)?;
        self.dominance_info.compute_dominance_frontiers(&func.cfg)?;
        
        // Identify variables that need phi-functions
        self.identify_phi_variables(&func.cfg);
        
        // Insert phi-functions
        self.insert_phi_functions(&mut func.cfg);
        
        // Rename variables
        self.rename_variables(func);
        
        Ok(())
    }
    
    /// Identifies variables that need phi-functions by analyzing definitions.
    fn identify_phi_variables(&mut self, cfg: &ControlFlowGraph) {
        self.phi_variables.clear();
        self.var_defs.clear();
        
        // For each block, find variable definitions
        for node_idx in cfg.graph().node_indices() {
            if let Some(block) = cfg.get_block(&cfg.graph()[node_idx].label) {
                for instruction in &block.instructions {
                    // Check for store instructions that define variables
                    if let InstructionKind::Store { value: _, dest } = &instruction.kind {
                        if let ValueKind::Temporary(temp_id) = &dest.kind {
                            let var_name = format!("t{}", temp_id);
                            self.var_defs
                                .entry(var_name.clone())
                                .or_insert_with(HashMap::new)
                                .entry(node_idx)
                                .or_insert_with(Vec::new)
                                .push(var_name.clone());
                            self.phi_variables.insert(var_name);
                        }
                    }
                }
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
    fn add_phi_function(&mut self, cfg: &mut ControlFlowGraph, node: NodeIndex, _var_name: &str) {
        if let Some(block) = cfg.graph_mut().node_weight_mut(node) {
            // Create phi-function instruction
            let phi_inst = Instruction::new(
                InstructionKind::Phi {
                    ty: IrType::I32, // TODO: Get actual type
                    incoming: vec![], // Will be filled during renaming
                },
                block.source_span.clone(),
            );
            
            // Add to beginning of block
            block.instructions.insert(0, phi_inst);
        }
    }
    
    /// Renames variables to ensure single assignment.
    fn rename_variables(&mut self, _func: &mut Function) {
        // TODO: Implement variable renaming algorithm
        // This would traverse the dominator tree and assign unique names
        // to each variable definition
    }
    
    // Generates a new unique temporary name.
    // fn new_temp_name(&mut self) -> String {
    //     let name = format!("ssa_{}", self.temp_counter);
    //     self.temp_counter += 1;
    //     name
    // }
}

impl Default for SsaTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ssa_transformer_new() {
        let transformer = SsaTransformer::new();
        assert!(transformer.phi_variables.is_empty());
        assert!(transformer.var_defs.is_empty());
    }
}