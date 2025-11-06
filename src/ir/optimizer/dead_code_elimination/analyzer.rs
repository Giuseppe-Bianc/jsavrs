//! Liveness and reachability analysis.

use crate::ir::{Function, InstructionKind, Terminator, TerminatorKind, Value};
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

use super::InstructionIndex;
use super::def_use::DefUseChains;

/// Liveness analyzer using backward dataflow analysis.
///
/// Computes which values are live at each program point using def-use chains
/// and backward propagation through the control flow graph.
#[derive(Debug)]
pub struct LivenessAnalyzer {
    /// Def-use chains for tracking value dependencies.
    pub def_use_chains: DefUseChains,

    /// Gen sets: values used before being defined in each block.
    gen_sets: HashMap<NodeIndex, HashSet<Value>>,

    /// Kill sets: values defined in each block.
    kill_sets: HashMap<NodeIndex, HashSet<Value>>,

    /// Live-in sets: values live at the start of each block.
    live_in: HashMap<NodeIndex, HashSet<Value>>,

    /// Live-out sets: values live at the end of each block.
    live_out: HashMap<NodeIndex, HashSet<Value>>,
}

impl LivenessAnalyzer {
    /// Creates a new liveness analyzer.
    pub fn new() -> Self {
        Self {
            def_use_chains: DefUseChains::new(),
            gen_sets: HashMap::new(),
            kill_sets: HashMap::new(),
            live_in: HashMap::new(),
            live_out: HashMap::new(),
        }
    }

    /// Builds def-use chains by scanning all instructions in the function.
    pub fn build_def_use_chains(&mut self, function: &Function) {
        for block_idx in function.cfg.graph().node_indices() {
            let block = &function.cfg.graph()[block_idx];

            for (inst_offset, instruction) in block.instructions.iter().enumerate() {
                let inst_idx = InstructionIndex { block_idx, inst_offset };

                if let Some(ref result) = instruction.result {
                    self.def_use_chains.add_definition(inst_idx, result.clone());
                }

                let used_values = extract_used_values(instruction);
                for value in used_values {
                    self.def_use_chains.add_use(inst_idx, value);
                }
            }

            // Process terminator uses
            let terminator_uses = extract_terminator_uses(&block.terminator);
            let term_idx = InstructionIndex { block_idx, inst_offset: block.instructions.len() };
            for value in terminator_uses {
                self.def_use_chains.add_use(term_idx, value);
            }
        }
    }

    /// Computes gen and kill sets for each basic block.
    pub fn compute_gen_kill_sets(&mut self, function: &Function) {
        for block_idx in function.cfg.graph().node_indices() {
            let block = &function.cfg.graph()[block_idx];

            let estimated_size = block.instructions.len().saturating_div(2);
            let mut gen_set = HashSet::with_capacity(estimated_size);
            let mut kill_set = HashSet::with_capacity(estimated_size);

            for (inst_offset, _instruction) in block.instructions.iter().enumerate() {
                let inst_idx = InstructionIndex { block_idx, inst_offset };

                let used_values = self.def_use_chains.get_used_values(&inst_idx);
                gen_set.extend(used_values.into_iter().filter(|v| !kill_set.contains(v)));

                if let Some(defined_value) = self.def_use_chains.get_defined_value(&inst_idx) {
                    kill_set.insert(defined_value.clone());
                }
            }

            // Process terminator uses
            let term_idx = InstructionIndex { block_idx, inst_offset: block.instructions.len() };
            let used_values = self.def_use_chains.get_used_values(&term_idx);
            gen_set.extend(used_values.into_iter().filter(|v| !kill_set.contains(v)));

            self.gen_sets.insert(block_idx, gen_set);
            self.kill_sets.insert(block_idx, kill_set);
        }
    }

    /// Performs backward dataflow analysis to compute live variable sets.
    pub fn analyze(&mut self, function: &Function) -> bool {
        const MAX_ITERATIONS: usize = 10;

        // Initialize all live sets to empty
        for block_idx in function.cfg.graph().node_indices() {
            self.live_in.insert(block_idx, HashSet::new());
            self.live_out.insert(block_idx, HashSet::new());
        }

        let rpo = compute_reverse_post_order(function);

        let mut iteration = 0;
        let mut changed = true;

        while changed && iteration < MAX_ITERATIONS {
            changed = false;
            iteration += 1;

            for &block_idx in rpo.iter().rev() {
                let mut new_live_out = HashSet::new();
                for successor in function.cfg.graph().neighbors(block_idx) {
                    if let Some(succ_live_in) = self.live_in.get(&successor) {
                        new_live_out.extend(succ_live_in.iter().cloned());
                    }
                }

                let gen_set = self.gen_sets.get(&block_idx);
                let kill_set = self.kill_sets.get(&block_idx);

                let mut new_live_in = gen_set.cloned().unwrap_or_default();

                if let Some(kill) = kill_set {
                    new_live_in.extend(new_live_out.iter().filter(|v| !kill.contains(v)).cloned());
                } else {
                    new_live_in.extend(new_live_out.iter().cloned());
                }

                let changed_in = self.live_in.get(&block_idx) != Some(&new_live_in);
                let changed_out = self.live_out.get(&block_idx) != Some(&new_live_out);

                if changed_in || changed_out {
                    changed = true;
                    self.live_in.insert(block_idx, new_live_in);
                    self.live_out.insert(block_idx, new_live_out);
                }
            }
        }

        if iteration >= MAX_ITERATIONS {
            eprintln!(
                "Warning: Liveness analysis did not converge after {} iterations for function '{}'",
                MAX_ITERATIONS, function.name
            );
            return false;
        }

        true
    }

    /// Checks if an instruction is dead (its result is never used).
    pub fn is_instruction_dead(&self, inst_idx: &InstructionIndex) -> bool {
        if let Some(defined_value) = self.def_use_chains.get_defined_value(inst_idx) {
            !self.def_use_chains.has_uses(defined_value)
        } else {
            false
        }
    }
}

impl Default for LivenessAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Analyzer for identifying reachable basic blocks via CFG traversal.
pub struct ReachabilityAnalyzer;

impl ReachabilityAnalyzer {
    /// Analyzes reachability starting from the function entry block.
    pub fn analyze(cfg: &crate::ir::cfg::ControlFlowGraph) -> HashSet<NodeIndex> {
        let mut reachable = HashSet::new();

        if let Some(entry_idx) = cfg.get_entry_block_index() {
            let mut dfs = petgraph::visit::Dfs::new(cfg.graph(), entry_idx);
            while let Some(node_idx) = dfs.next(cfg.graph()) {
                reachable.insert(node_idx);
            }
        }

        reachable
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extracts all values used by an instruction.
fn extract_used_values(instruction: &crate::ir::Instruction) -> Vec<Value> {
    match &instruction.kind {
        InstructionKind::Binary { left, right, .. } => vec![left.clone(), right.clone()],
        InstructionKind::Unary { operand, .. } => vec![operand.clone()],
        InstructionKind::Load { src, .. } => vec![src.clone()],
        InstructionKind::Store { value, dest } => vec![value.clone(), dest.clone()],
        InstructionKind::Call { func, args, .. } => {
            let mut values = Vec::with_capacity(args.len() + 1);
            values.push(func.clone());
            values.extend(args.iter().cloned());
            values
        }
        InstructionKind::GetElementPtr { base, index, .. } => vec![base.clone(), index.clone()],
        InstructionKind::Cast { value, .. } => vec![value.clone()],
        InstructionKind::Phi { incoming, .. } => incoming.iter().map(|(v, _)| v.clone()).collect(),
        InstructionKind::Vector { operands, .. } => operands.clone(),
        InstructionKind::Alloca { .. } => Vec::new(),
    }
}

/// Extracts all values used by a terminator.
fn extract_terminator_uses(terminator: &Terminator) -> Vec<Value> {
    match &terminator.kind {
        TerminatorKind::Return { value, .. } => vec![value.clone()],
        TerminatorKind::ConditionalBranch { condition, .. } => vec![condition.clone()],
        TerminatorKind::IndirectBranch { address, .. } => vec![address.clone()],
        TerminatorKind::Switch { value, cases, .. } => {
            let mut values = Vec::with_capacity(cases.len() + 1);
            values.push(value.clone());
            values.extend(cases.iter().map(|(v, _)| v.clone()));
            values
        }
        TerminatorKind::Branch { .. } | TerminatorKind::Unreachable => Vec::new(),
    }
}

/// Computes reverse post-order traversal of the CFG.
fn compute_reverse_post_order(function: &Function) -> Vec<NodeIndex> {
    use petgraph::visit::{DfsEvent, depth_first_search};

    let mut post_order = Vec::new();
    let entry_node = function.cfg.get_entry_block_index().expect("Function should have entry node");

    depth_first_search(function.cfg.graph(), Some(entry_node), |event| {
        if let DfsEvent::Finish(node, _) = event {
            post_order.push(node);
        }
    });

    post_order.reverse();
    post_order
}
