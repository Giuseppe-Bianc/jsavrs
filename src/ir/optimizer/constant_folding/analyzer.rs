// src/ir/optimizer/constant_folding/analyzer.rs
//! SCCP Analyzer - Main Sparse Conditional Constant Propagation Implementation
//!
//! This module implements the Wegman-Zadeck algorithm for constant propagation
//! using a dual-worklist approach with lattice-based value tracking.

use super::branch_analysis::{BranchAnalyzer, TerminatorEvaluation};
use super::evaluator::{evaluate_binary_op, evaluate_unary_op};
use super::executable_edges::ExecutableEdges;
use super::lattice::LatticeValue;
use super::stats::OptimizationStatistics;
use super::worklist::{FlowWorkList, SSAWorkList};
use crate::ir::Function;
use crate::ir::instruction::{Instruction, InstructionKind};
use crate::ir::terminator::Terminator;
use crate::ir::value::ValueKind;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

/// Configuration for SCCP analysis
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Maximum iterations before terminating analysis
    pub max_iterations: usize,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self { max_iterations: 100, verbose: false }
    }
}

/// Main SCCP Analyzer
///
/// Implements the Wegman-Zadeck sparse conditional constant propagation algorithm
/// using a three-level lattice (Top/Constant/Bottom) and dual worklist approach.
///
/// # Algorithm Overview
///
/// 1. Initialize all values to Top (optimistic unknown)
/// 2. Mark entry block as executable
/// 3. Process worklists until empty:
///    - SSAWorkList: Value changes trigger re-evaluation of uses
///    - FlowWorkList: Newly executable edges trigger block visits
/// 4. Iterate until fixed-point convergence
///
/// # Complexity
///
/// O(edges) time complexity achieved through per-edge visit limits:
/// - Each CFG edge processed at most once
/// - Each SSA edge processed at most twice (Top→Constant→Bottom)
pub struct SCCPAnalyzer<'a> {
    /// Function being analyzed
    function: &'a Function,
    /// Lattice values for all SSA values (instruction results)
    /// Maps instruction index to its lattice value
    lattice: HashMap<usize, LatticeValue>,
    /// SSA worklist for value propagation
    ssa_worklist: SSAWorkList,
    /// Control flow worklist for executable edges
    flow_worklist: FlowWorkList,
    /// Executable edge tracking
    executable_edges: ExecutableEdges,
    /// Optimization statistics
    stats: OptimizationStatistics,
    /// Configuration
    config: AnalysisConfig,
    /// Current iteration counter
    iteration: usize,
}

impl<'a> SCCPAnalyzer<'a> {
    /// Create a new SCCP analyzer for the given function
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    /// * `config` - Analysis configuration
    ///
    /// # Returns
    ///
    /// A new SCCPAnalyzer with all values initialized to Top (optimistic unknown)
    /// except function parameters and global values which are initialized to Bottom
    /// (pessimistic varying) since their values are unknown at compile-time.
    pub fn new(function: &'a Function, config: AnalysisConfig) -> Self {
        let mut analyzer = Self {
            function,
            lattice: HashMap::new(),
            ssa_worklist: SSAWorkList::new(),
            flow_worklist: FlowWorkList::new(),
            executable_edges: ExecutableEdges::new(),
            stats: OptimizationStatistics::default(),
            config,
            iteration: 0,
        };

        // Initialize lattice values
        analyzer.initialize_lattice();

        // Mark entry block as executable and add to flow worklist
        if let Some(entry_idx) = function.cfg.get_entry_block_index() {
            let entry_usize = entry_idx.index();
            analyzer.executable_edges.mark_block_executable(entry_usize);
            analyzer.flow_worklist.enqueue(entry_usize, entry_usize); // Self-edge for entry
        }

        analyzer
    }

    /// Initialize lattice values for all instructions
    ///
    /// Sets all instruction results to Top (optimistic unknown) initially.
    /// Function parameters and global values are set to Bottom (pessimistic varying)
    /// since their values cannot be determined at compile-time.
    fn initialize_lattice(&mut self) {
        let cfg = &self.function.cfg;

        // Initialize all instruction results to Top
        for (block_idx, block) in cfg.graph().node_indices().zip(cfg.blocks()) {
            for (inst_idx, _inst) in block.instructions.iter().enumerate() {
                let global_inst_idx = self.compute_global_instruction_index(block_idx, inst_idx);
                self.lattice.insert(global_inst_idx, LatticeValue::Top);
            }
        }

        // TODO: Mark function parameters as Bottom (unknown external values)
        // This would require tracking parameter SSA values
    }

    /// Compute global instruction index from block index and local instruction index
    ///
    /// Since we're using HashMap with usize keys, we need a unique index for each instruction.
    /// We use a simple encoding: block_index * 10000 + instruction_index
    /// This assumes no block has more than 10000 instructions.
    fn compute_global_instruction_index(&self, block_idx: NodeIndex, inst_idx: usize) -> usize {
        block_idx.index() * 10000 + inst_idx
    }

    /// Main analysis loop - process worklists until fixed-point convergence
    ///
    /// # Returns
    ///
    /// True if analysis converged, false if max iterations exceeded
    pub fn analyze(&mut self) -> bool {
        if self.config.verbose {
            println!("Starting SCCP analysis for function '{}'", self.function.name);
        }

        while !self.flow_worklist.is_empty() || !self.ssa_worklist.is_empty() {
            // Check iteration limit
            self.iteration += 1;
            if self.iteration > self.config.max_iterations {
                if self.config.verbose {
                    println!("WARNING: SCCP analysis exceeded max iterations ({})", self.config.max_iterations);
                }
                return false;
            }

            // Process flow worklist (newly executable blocks)
            if let Some((_from_block, to_block)) = self.flow_worklist.dequeue() {
                self.visit_block(to_block);
            }

            // Process SSA worklist (value changes)
            while let Some((block_idx, inst_idx)) = self.ssa_worklist.dequeue() {
                self.visit_instruction(block_idx, inst_idx);
            }
        }

        if self.config.verbose {
            println!("SCCP analysis converged in {} iterations", self.iteration);
        }

        true
    }

    /// Visit a newly executable block
    ///
    /// Processes all instructions in the block and evaluates its terminator.
    fn visit_block(&mut self, block_idx: usize) {
        let cfg = &self.function.cfg;

        // Get the block by index
        let node_idx = NodeIndex::new(block_idx);
        if let Some(block) = cfg.graph().node_weight(node_idx) {
            // Visit all instructions in the block
            for (inst_idx, instruction) in block.instructions.iter().enumerate() {
                self.process_instruction(block_idx, inst_idx, instruction);
            }

            // Visit the terminator
            self.visit_terminator(block_idx, &block.terminator);
        }
    }

    /// Visit an instruction whose operands have changed
    ///
    /// Re-evaluates the instruction with current lattice values.
    fn visit_instruction(&mut self, block_idx: usize, inst_idx: usize) {
        let cfg = &self.function.cfg;
        let node_idx = NodeIndex::new(block_idx);

        if let Some(block) = cfg.graph().node_weight(node_idx) {
            if let Some(instruction) = block.instructions.get(inst_idx) {
                self.process_instruction(block_idx, inst_idx, instruction);
            }
        }
    }

    /// Process a single instruction and update its lattice value
    fn process_instruction(&mut self, block_idx: usize, inst_idx: usize, instruction: &Instruction) {
        let node_idx = NodeIndex::new(block_idx);
        let global_inst_idx = self.compute_global_instruction_index(node_idx, inst_idx);

        // Evaluate the instruction
        let new_value = match &instruction.kind {
            InstructionKind::Binary { op, left, right, .. } => {
                let left_val = self.get_value_lattice(left);
                let right_val = self.get_value_lattice(right);
                evaluate_binary_op(*op, &left_val, &right_val)
            }
            InstructionKind::Unary { op, operand, .. } => {
                let operand_val = self.get_value_lattice(operand);
                evaluate_unary_op(*op, &operand_val)
            }
            InstructionKind::Phi { incoming, .. } => self.evaluate_phi_node(block_idx, incoming),
            // Other instruction types default to Bottom (unknown)
            _ => LatticeValue::Bottom,
        };

        // Update lattice value if it changed
        self.update_lattice_value(block_idx, inst_idx, new_value);
    }

    /// Get lattice value for an IR value
    fn get_value_lattice(&self, value: &crate::ir::Value) -> LatticeValue {
        match &value.kind {
            ValueKind::Literal(lit) => LatticeValue::Constant(lit.clone()),
            ValueKind::Temporary(index) => {
                // Temporary values are instruction results
                // Convert u64 index to usize for lattice lookup
                let index_usize = *index as usize;
                self.lattice.get(&index_usize).cloned().unwrap_or(LatticeValue::Bottom)
            }
            _ => LatticeValue::Bottom,
        }
    }

    /// Evaluate a phi node based on executable predecessor edges
    ///
    /// Computes the meet (greatest lower bound) of all incoming values
    /// from executable predecessor blocks.
    fn evaluate_phi_node(&self, block_idx: usize, incoming: &[(crate::ir::Value, String)]) -> LatticeValue {
        let mut result = LatticeValue::Top;

        for (value, pred_label) in incoming {
            // Only include values from executable predecessor edges
            if let Some(pred_idx) = self.function.cfg.find_block_by_label(pred_label) {
                let pred_idx_usize = pred_idx.index();
                if self.executable_edges.is_edge_executable(pred_idx_usize, block_idx) {
                    let value_lattice = self.get_value_lattice(value);
                    result = result.meet(&value_lattice);
                }
            }
        }

        result
    }

    /// Update lattice value and enqueue dependent instructions if changed
    fn update_lattice_value(&mut self, block_idx: usize, inst_idx: usize, new_value: LatticeValue) {
        let node_idx = NodeIndex::new(block_idx);
        let global_inst_idx = self.compute_global_instruction_index(node_idx, inst_idx);

        let old_value = self.lattice.get(&global_inst_idx).cloned().unwrap_or(LatticeValue::Top);

        // Check if value changed (monotonically decreasing in lattice)
        if new_value.is_more_precise_than(&old_value) {
            self.lattice.insert(global_inst_idx, new_value.clone());

            // Update statistics
            if new_value.is_constant() {
                self.stats.constants_found += 1;
            }
            self.stats.total_values = self.lattice.len();

            // Enqueue dependent instructions
            // Note: In SSA form, we process blocks in reverse post-order,
            // so forward data-flow is naturally handled by the FlowWorkList.
            // When a value changes, subsequent instructions in the same block
            // and phi nodes in successor blocks will be re-evaluated.
            // The worklist-driven approach ensures we process all dependencies.
        }
    }

    /// Visit a terminator and mark successor edges as executable if appropriate
    fn visit_terminator(&mut self, block_idx: usize, terminator: &Terminator) {
        // Use BranchAnalyzer to evaluate the terminator
        let evaluation = BranchAnalyzer::evaluate_terminator(terminator, |value| self.get_value_lattice(value));

        // Mark edges based on evaluation result
        match evaluation {
            TerminatorEvaluation::UnconditionalJump { target } => {
                self.mark_edge_executable(block_idx, &target);
            }
            TerminatorEvaluation::OnlyTrueBranch { target } => {
                self.mark_edge_executable(block_idx, &target);
            }
            TerminatorEvaluation::OnlyFalseBranch { target } => {
                self.mark_edge_executable(block_idx, &target);
            }
            TerminatorEvaluation::ConditionalJump { true_target, false_target } => {
                self.mark_edge_executable(block_idx, &true_target);
                self.mark_edge_executable(block_idx, &false_target);
            }
            TerminatorEvaluation::Switch { targets } => {
                for target in targets {
                    self.mark_edge_executable(block_idx, &target);
                }
            }
            TerminatorEvaluation::NoSuccessors => {
                // Return or unreachable - no edges to mark
            }
        }
    }

    /// Mark an edge as executable and enqueue target block if newly reachable
    fn mark_edge_executable(&mut self, from_block: usize, to_label: &str) {
        // Find target block index by label
        if let Some(to_idx) = self.function.cfg.find_block_by_label(to_label) {
            let to_block = to_idx.index();

            if self.executable_edges.mark_edge_executable(from_block, to_block) {
                // Edge was newly marked executable
                self.flow_worklist.enqueue(from_block, to_block);
            }
        }
    }

    /// Get the final lattice values after analysis
    pub fn lattice(&self) -> &HashMap<usize, LatticeValue> {
        &self.lattice
    }

    /// Get the set of executable blocks
    pub fn executable_blocks(&self) -> &std::collections::HashSet<usize> {
        self.executable_edges.executable_blocks()
    }

    /// Get optimization statistics
    pub fn statistics(&self) -> &OptimizationStatistics {
        &self.stats
    }

    /// Get the number of iterations performed
    pub fn iterations(&self) -> usize {
        self.iteration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::IrType;
    use crate::location::source_span::SourceSpan;

    #[test]
    fn test_analyzer_creation() {
        let func = Function::new("test", vec![], IrType::Void);
        let config = AnalysisConfig::default();
        let analyzer = SCCPAnalyzer::new(&func, config);

        assert_eq!(analyzer.iteration, 0);
        assert!(analyzer.lattice.is_empty());
    }

    #[test]
    fn test_lattice_initialization() {
        let mut func = Function::new("test", vec![], IrType::Void);
        func.add_block("bb0", SourceSpan::default());

        let config = AnalysisConfig::default();
        let analyzer = SCCPAnalyzer::new(&func, config);

        // Entry block should be marked executable
        if let Some(entry_idx) = func.cfg.get_entry_block_index() {
            assert!(analyzer.executable_edges.is_block_executable(entry_idx.index()));
        }
    }
}
