use crate::ir::{Function, Module, Phase};

use super::propagator::SCCPropagator;
use super::rewriter::IRRewriter;

/// Configuration for the Constant Folding Optimizer
///
/// Controls the behavior of the SCCP optimization pass.
#[derive(Debug, Clone)]
pub struct SCCPConfig {
    /// Whether to emit verbose optimization information to stderr.
    /// Useful for debugging and understanding optimization behavior.
    pub verbose: bool,
    /// Maximum number of iterations before convergence failure.
    /// Prevents infinite loops in degenerate cases (should never be reached in practice).
    /// Typical functions converge in ≤3 iterations (see SC-003).
    pub max_iterations: usize,
}

impl Default for SCCPConfig {
    fn default() -> Self {
        Self { verbose: false, max_iterations: 100 }
    }
}

/// Statistics tracked during SCCP optimization
///
/// Provides metrics for evaluating optimization effectiveness.
/// All counters are cumulative across multiple function optimizations.
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Number of constants propagated and folded
    pub constants_propagated: usize,
    /// Number of conditional branches resolved to unconditional jumps
    pub branches_resolved: usize,
    /// Number of phi nodes simplified (to constants or reduced operands)
    pub phi_nodes_simplified: usize,
    /// Number of basic blocks marked unreachable
    pub blocks_marked_unreachable: usize,
    /// Number of iterations required for convergence (last function optimized)
    pub iterations: usize,
}

impl std::fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SCCP Stats: {} constants, {} branches, {} phis, {} unreachable blocks, {} iterations",
            self.constants_propagated,
            self.branches_resolved,
            self.phi_nodes_simplified,
            self.blocks_marked_unreachable,
            self.iterations
        )
    }
}

/// Constant Folding Optimizer with SCCP
///
/// Orchestrates the SCCP optimization pipeline:
/// 1. Propagation: Analyze function to discover constants and unreachable code
/// 2. Rewriting: Transform IR based on analysis results
///
/// # Example
///
/// ```rust,ignore
/// use jsavrs::ir::optimizer::constant_folding::optimizer::*;
///
/// let mut optimizer = ConstantFoldingOptimizer::default();
/// let stats = optimizer.optimize_function(&mut function)?;
/// println!("Propagated {} constants", stats.constants_propagated);
/// ```
#[derive(Default)]
pub struct ConstantFoldingOptimizer {
    config: SCCPConfig,
    stats: OptimizationStats,
}

impl Drop for ConstantFoldingOptimizer {
    fn drop(&mut self) {
        // Clear statistics to release any held memory
        self.stats = OptimizationStats::default();
    }
}

impl ConstantFoldingOptimizer {
    #[must_use]
    pub fn new(verbose: bool, sccp_enabled: bool) -> Self {
        let _ = sccp_enabled; // For backwards compatibility
        Self { config: SCCPConfig { verbose, ..Default::default() }, stats: OptimizationStats::default() }
    }

    #[must_use]
    pub fn with_config(config: SCCPConfig) -> Self {
        Self { config, stats: OptimizationStats::default() }
    }

    #[must_use]
    pub const fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    /// Optimizes a single function using SCCP algorithm.
    ///
    /// # Arguments
    /// * `function` - The function to optimize
    ///
    /// # Returns
    /// `Ok(stats)` with optimization statistics on success,
    /// `Err` if optimization fails
    ///
    /// # Errors
    /// Returns an error string if:
    /// - SCCP propagation fails during constant analysis
    /// - The propagation algorithm encounters an invalid IR state
    /// - Maximum iteration limit is exceeded without convergence
    pub fn optimize_function(&mut self, function: &mut Function) -> Result<OptimizationStats, String> {
        // Phase 1: Run SCCP propagation
        let mut propagator = SCCPropagator::new_for_function(function);
        propagator.set_verbose(self.config.verbose);
        let iterations = propagator
            .propagate(function, self.config.max_iterations)
            .map_err(|e| format!("SCCP propagation failed: {e}"))?;

        // Phase 2: Rewrite IR based on SCCP results
        let rewriter = IRRewriter::new();

        // TODO: Implement actual IR transformation in next iteration
        // For now, we just track that we ran the analysis

        // Update statistics
        let rewriter_stats = rewriter.into_stats();
        self.stats.constants_propagated += rewriter_stats.constants_propagated;
        self.stats.branches_resolved += rewriter_stats.branches_resolved;
        self.stats.phi_nodes_simplified += rewriter_stats.phi_nodes_simplified;
        self.stats.blocks_marked_unreachable += rewriter_stats.blocks_marked_unreachable;
        self.stats.iterations = iterations;

        Ok(self.stats.clone())
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &'static str {
        "Constant Folding Optimizer (SCCP)"
    }

    fn run(&mut self, ir: &mut Module) {
        // Optimize each function in the module
        for function in &mut ir.functions {
            if let Err(e) = self.optimize_function(function) {
                eprintln!("Error optimizing function {}: {}", function.name, e);
            }
        }

        println!("Total number of instructions after constant folding: {}", ir.count_instructions());
    }
}
