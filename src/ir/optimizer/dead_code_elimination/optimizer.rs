//! Main Dead Code Elimination optimizer implementation.

use crate::ir::{Function, InstructionKind, Module, Phase};
use console::style;
use petgraph::Direction;
use std::collections::HashSet;
use std::fmt::Write;
use std::sync::Arc;

use super::InstructionIndex;
use super::analyzer::{LivenessAnalyzer, ReachabilityAnalyzer};
use super::escape::{EscapeAnalyzer, EscapeStatus};
use super::stats::{ConservativeReason, OptimizationStats};

/// Dead Code Elimination optimization phase.
///
/// Removes unreachable basic blocks and unused instructions from IR functions
/// while preserving all observable program behavior.
#[derive(Debug, Clone)]
pub struct DeadCodeElimination {
    /// Maximum number of fixed-point iterations before stopping.
    pub max_iterations: usize,

    /// Whether to collect and report detailed optimization statistics.
    pub enable_statistics: bool,

    /// Whether to emit warnings for conservative decisions.
    pub verbose_warnings: bool,

    /// Whether to emit warnings for conservative decisions.
    pub verbose: bool,

    /// Statistics from the last optimization run.
    last_stats: OptimizationStats,
}

impl DeadCodeElimination {
    /// Creates a new DCE optimizer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new DCE optimizer with custom settings.
    ///
    /// # Panics
    ///
    /// Panics if `max_iterations` is 0.
    pub fn with_config(max_iterations: usize, enable_statistics: bool, verbose: bool, verbose_warnings: bool) -> Self {
        assert!(max_iterations > 0, "max_iterations must be > 0");
        Self { max_iterations, enable_statistics, verbose, verbose_warnings, last_stats: OptimizationStats::default() }
    }

    /// Returns a reference to the statistics from the last optimization run.
    pub fn get_statistics(&self) -> &OptimizationStats {
        &self.last_stats
    }

    /// Optimizes a single function with DCE using fixed-point iteration.
    fn optimize_function(&mut self, function: &mut Function) -> OptimizationStats {
        let mut stats = OptimizationStats::new();

        for iteration in 1..=self.max_iterations {
            stats.iterations = iteration;
            let mut changed = false;

            // Phase 1: Reachability analysis and block removal
            changed |= self.remove_unreachable_blocks(function, &mut stats);

            // Phase 2: Dead instruction elimination
            let dead_insts_removed = self.remove_dead_instructions(function);
            if dead_insts_removed > 0 {
                changed = true;
                stats.instructions_removed += dead_insts_removed;
            }

            // Check convergence
            if !changed {
                break;
            }

            if iteration == self.max_iterations && self.verbose_warnings {
                eprintln!(
                    "Warning: DCE did not converge for function '{}' after {} iterations",
                    function.name, self.max_iterations
                );
            }
        }

        self.last_stats = stats.clone();
        stats
    }

    /// Removes unreachable blocks from the function.
    fn remove_unreachable_blocks(&mut self, function: &mut Function, stats: &mut OptimizationStats) -> bool {
        let reachable_blocks = ReachabilityAnalyzer::analyze(&function.cfg);

        let blocks_to_remove: Vec<Arc<str>> = function
            .cfg
            .blocks()
            .filter_map(|block| {
                let block_idx = function.cfg.find_block_by_label(&block.label)?;
                (!reachable_blocks.contains(&block_idx)).then(|| block.label.clone())
            })
            .collect();

        if blocks_to_remove.is_empty() {
            return false;
        }

        self.update_phi_nodes_for_removed_blocks(&mut function.cfg, &blocks_to_remove);

        for label in &blocks_to_remove {
            if function.cfg.remove_block(label) {
                stats.blocks_removed += 1;
            }
        }

        if cfg!(debug_assertions) {
            self.verify_ssa_form_preservation(&function.cfg);
        }

        true
    }

    /// Removes dead instructions using liveness and escape analysis.
    fn remove_dead_instructions(&mut self, function: &mut Function) -> usize {
        let mut total_removed = 0;

        loop {
            let mut analyzer = LivenessAnalyzer::new();
            analyzer.build_def_use_chains(function);
            analyzer.compute_gen_kill_sets(function);

            if !analyzer.analyze(function) && self.verbose_warnings {
                eprintln!("Warning: Liveness analysis did not converge for function '{}'", function.name);
            }

            let mut escape_analyzer = EscapeAnalyzer::new();
            escape_analyzer.analyze(function);

            let dead_instructions = self.identify_dead_instructions(function, &analyzer, &escape_analyzer);

            if dead_instructions.is_empty() {
                break;
            }

            total_removed += self.remove_instructions(function, dead_instructions);
        }

        total_removed
    }

    /// Identifies dead instructions that can be safely removed.
    fn identify_dead_instructions(
        &self, function: &Function, analyzer: &LivenessAnalyzer, escape_analyzer: &EscapeAnalyzer,
    ) -> Vec<(Arc<str>, usize)> {
        let estimated_capacity = function.cfg.blocks().map(|b| b.instructions.len()).sum::<usize>() / 10;
        let mut dead_instructions = Vec::with_capacity(estimated_capacity);

        for block_idx in function.cfg.graph().node_indices() {
            let block = &function.cfg.graph()[block_idx];

            for (inst_offset, instruction) in block.instructions.iter().enumerate() {
                let inst_idx = InstructionIndex { block_idx, inst_offset };
                let is_dead = analyzer.is_instruction_dead(&inst_idx);

                let can_remove =
                    self.can_remove_instruction(instruction, &inst_idx, is_dead, function, analyzer, escape_analyzer);

                if can_remove {
                    dead_instructions.push((block.label.clone(), inst_offset));
                }
            }
        }

        dead_instructions
    }

    /// Determines if an instruction can be safely removed.
    fn can_remove_instruction(
        &self, instruction: &crate::ir::Instruction, inst_idx: &InstructionIndex, is_dead: bool, function: &Function,
        analyzer: &LivenessAnalyzer, escape_analyzer: &EscapeAnalyzer,
    ) -> bool {
        match &instruction.kind {
            InstructionKind::Store { dest, .. } => self.can_remove_store(dest, function, escape_analyzer, inst_idx),

            InstructionKind::Load { src, .. } => is_dead && self.can_remove_load(src, escape_analyzer, inst_idx),

            InstructionKind::Alloca { .. } => {
                if let Some(result) = &instruction.result {
                    !analyzer.def_use_chains.has_uses(result)
                } else {
                    false
                }
            }

            InstructionKind::Call { .. } => {
                if is_dead && self.verbose {
                    self.log_conservative_warning(
                        inst_idx,
                        ConservativeReason::UnknownCallPurity,
                        "Call instruction preserved due to potential side effects",
                    );
                }
                false
            }

            _ => is_dead && self.is_pure_instruction(instruction),
        }
    }

    /// Checks if a store can be safely removed.
    fn can_remove_store(
        &self, dest: &crate::ir::Value, function: &Function, escape_analyzer: &EscapeAnalyzer,
        inst_idx: &InstructionIndex,
    ) -> bool {
        match escape_analyzer.get_status(dest) {
            EscapeStatus::Local => !self.has_loads_from(function, dest),
            EscapeStatus::AddressTaken | EscapeStatus::Escaped => {
                self.log_conservative_warning(
                    inst_idx,
                    ConservativeReason::EscapedPointer,
                    "Store to escaped or address-taken allocation preserved",
                );
                false
            }
        }
    }

    /// Checks if a load can be safely removed.
    fn can_remove_load(
        &self, src: &crate::ir::Value, escape_analyzer: &EscapeAnalyzer, inst_idx: &InstructionIndex,
    ) -> bool {
        match escape_analyzer.get_status(src) {
            EscapeStatus::Local => true,
            EscapeStatus::AddressTaken | EscapeStatus::Escaped => {
                self.log_conservative_warning(
                    inst_idx,
                    ConservativeReason::EscapedPointer,
                    "Load from escaped or address-taken allocation preserved",
                );

                false
            }
        }
    }

    /// Removes a list of instructions from the function.
    fn remove_instructions(&self, function: &mut Function, dead_instructions: Vec<(Arc<str>, usize)>) -> usize {
        let mut total_removed = 0;

        // Group by block label
        let mut by_block: std::collections::HashMap<Arc<str>, Vec<usize>> = std::collections::HashMap::new();
        for (label, offset) in dead_instructions {
            by_block.entry(label).or_default().push(offset);
        }

        for (block_label, mut offsets) in by_block {
            // Sort in reverse to remove from back to front
            offsets.sort_unstable_by(|a, b| b.cmp(a));

            if let Some(block) = function.cfg.get_block_mut(&block_label) {
                for offset in offsets {
                    if offset < block.instructions.len() {
                        block.instructions.remove(offset);
                        total_removed += 1;
                    }
                }
            }
        }

        total_removed
    }

    /// Checks if there are any Load instructions from the given pointer.
    fn has_loads_from(&self, function: &Function, ptr: &crate::ir::Value) -> bool {
        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                if let InstructionKind::Load { src, .. } = &instruction.kind
                    && src == ptr
                {
                    return true;
                }
            }
        }
        false
    }

    /// Logs a conservative warning for a preserved instruction.
    fn log_conservative_warning(&self, inst_idx: &InstructionIndex, _reason: ConservativeReason, message: &str) {
        if self.verbose_warnings {
            eprintln!(
                "Conservative DCE: Block {}, Instruction {}: {}",
                inst_idx.block_idx.index(),
                inst_idx.inst_offset,
                message
            );
        }
    }

    /// Checks if an instruction is pure (has no side effects).
    #[inline]
    fn is_pure_instruction(&self, instruction: &crate::ir::Instruction) -> bool {
        matches!(
            instruction.kind,
            InstructionKind::Binary { .. }
                | InstructionKind::Unary { .. }
                | InstructionKind::Cast { .. }
                | InstructionKind::GetElementPtr { .. }
                | InstructionKind::Load { .. }
        )
    }

    /// Updates phi node incoming lists to remove references to deleted blocks.
    fn update_phi_nodes_for_removed_blocks(
        &self, cfg: &mut crate::ir::cfg::ControlFlowGraph, removed_labels: &[Arc<str>],
    ) {
        for block in cfg.blocks_mut() {
            for instruction in &mut block.instructions {
                if let InstructionKind::Phi { incoming, .. } = &mut instruction.kind {
                    incoming.retain(|(_, predecessor_label)| {
                        !removed_labels.iter().any(|removed| **removed == **predecessor_label)
                    });
                }
            }
        }
    }

    /// Verifies that SSA form is preserved after block removal.
    fn verify_ssa_form_preservation(&self, cfg: &crate::ir::cfg::ControlFlowGraph) {
        let all_labels: HashSet<&str> = cfg.blocks().map(|b| b.label.as_ref()).collect();

        for block in cfg.blocks() {
            let block_idx = cfg.find_block_by_label(&block.label);

            let predecessors: HashSet<Arc<str>> = if let Some(idx) = block_idx {
                cfg.graph()
                    .neighbors_directed(idx, Direction::Incoming)
                    .map(|pred_idx| cfg.graph()[pred_idx].label.clone())
                    .collect()
            } else {
                HashSet::new()
            };

            for instruction in &block.instructions {
                if let InstructionKind::Phi { incoming, .. } = &instruction.kind {
                    assert!(
                        !incoming.is_empty(),
                        "SSA violation: Phi node in block '{}' has zero incoming edges",
                        block.label
                    );

                    for (_, pred_label) in incoming {
                        assert!(
                            all_labels.contains(pred_label.as_str()),
                            "SSA violation: Phi node in block '{}' references non-existent block '{}'",
                            block.label,
                            pred_label
                        );

                        assert!(
                            predecessors.iter().any(|p| &**p == pred_label.as_str()),
                            "SSA violation: Phi node in block '{}' references '{}' which is not a CFG predecessor",
                            block.label,
                            pred_label
                        );
                    }
                }
            }
        }
    }
}

impl Default for DeadCodeElimination {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            enable_statistics: true,
            verbose: false,
            verbose_warnings: false,
            last_stats: OptimizationStats::default(),
        }
    }
}

impl Phase for DeadCodeElimination {
    /// Returns the name of this optimization phase
    fn name(&self) -> &'static str {
        "Dead Code Elimination"
    }

    /// Runs the Dead Code Elimination optimization on the entire module.
    ///
    /// Optimizes all functions in the module by removing unreachable blocks and
    /// dead instructions. Statistics are collected if enabled and printed after
    /// optimization completes.
    fn run(&mut self, module: &mut Module) {
        let function_names: Vec<String> = module.functions().iter().map(|f| f.name.clone()).collect();
        let mut aggregated_stats = OptimizationStats::default();

        for name in function_names {
            if let Some(function) = module.get_function_mut(&name) {
                if function.cfg.graph().node_count() == 0 {
                    continue;
                }

                let stats = self.optimize_function(function);

                aggregated_stats.instructions_removed += stats.instructions_removed;
                aggregated_stats.blocks_removed += stats.blocks_removed;
                aggregated_stats.iterations += stats.iterations;
                aggregated_stats.conservative_warnings.extend(stats.conservative_warnings);
            }
        }

        self.last_stats = aggregated_stats;

        if self.verbose {
            self.print_statistics();
        }
    }
}

impl DeadCodeElimination {
    /// Prints optimization statistics.
    fn print_statistics(&self) {
        let dce_stats = self.get_statistics();

        if dce_stats.had_effect() {
            let mut output = String::with_capacity(256);
            writeln!(output, "\n{}", style("Dead Code Elimination Statistics:").cyan().bold()).unwrap();
            writeln!(output, "✂️  Instructions removed: {}", dce_stats.instructions_removed).unwrap();
            writeln!(output, "🗑️  Blocks removed: {}", dce_stats.blocks_removed).unwrap();
            writeln!(output, "🔄  Iterations to convergence: {}", dce_stats.iterations).unwrap();

            if !dce_stats.conservative_warnings.is_empty() {
                writeln!(output, "⚠️  Conservative warnings: {}", dce_stats.conservative_warnings.len()).unwrap();
                output.reserve(dce_stats.conservative_warnings.len() * 50);

                for warning in &dce_stats.conservative_warnings {
                    writeln!(output, "    - {}", style(&warning.reason).yellow()).unwrap();
                }
            }

            print!("{}", output);
        } else {
            println!("{}", style("No dead code found - module already optimal").green());
        }
    }
}
