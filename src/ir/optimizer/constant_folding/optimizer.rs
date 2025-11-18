use super::{
    analyzer::{AnalysisConfig, SCCPAnalyzer},
    rewriter,
    stats::OptimizationStatistics,
    validate_postconditions, validate_preconditions,
};
use crate::ir::{Module, Phase};

/// Configuration for the Constant Folding Optimizer
pub struct ConstantFoldingOptimizer {
    /// Whether to emit verbose optimization information
    pub verbose: bool,

    /// Whether to enable SCCP optimization
    pub sccp_enabled: bool,

    /// Maximum iterations for fixed-point convergence
    pub max_iterations: usize,
}

impl ConstantFoldingOptimizer {
    pub fn new(verbose: bool, sccp_enabled: bool) -> Self {
        Self { verbose, sccp_enabled, max_iterations: 100 }
    }

    /// Runs SCCP optimization on a module
    fn run_sccp(&mut self, module: &mut Module) {
        if self.verbose {
            println!("=== SCCP Optimization Started ===");
        }

        let mut total_stats = OptimizationStatistics::new();

        // Process each function in the module
        for function in &mut module.functions {
            if self.verbose {
                println!("Processing function: {}", function.name);
            }

            let stats = self.analyze_function(function);

            // Accumulate statistics
            total_stats.constants_found += stats.constants_found;
            total_stats.total_values += stats.total_values;
            total_stats.branches_eliminated += stats.branches_eliminated;
            total_stats.blocks_removed += stats.blocks_removed;
            total_stats.total_blocks += stats.total_blocks;
            total_stats.instructions_replaced += stats.instructions_replaced;
            total_stats.phi_nodes_simplified += stats.phi_nodes_simplified;
            total_stats.iterations += stats.iterations;
        }

        if self.verbose {
            println!("=== SCCP Optimization Complete ===");
            println!("{}", total_stats);
        }
    }

    /// Analyzes a single function using SCCP algorithm
    fn analyze_function(&self, function: &mut crate::ir::Function) -> OptimizationStatistics {
        // Validate preconditions
        if !validate_preconditions(function) {
            if self.verbose {
                println!("WARNING: Precondition validation failed for function '{}'", function.name);
            }
            return OptimizationStatistics::default();
        }

        // Create analysis configuration
        let config = AnalysisConfig { max_iterations: self.max_iterations, verbose: self.verbose };

        // Create and run SCCP analyzer
        let mut analyzer = SCCPAnalyzer::new(function, config);
        let converged = analyzer.analyze();

        if !converged && self.verbose {
            println!("WARNING: SCCP analysis did not converge for function '{}'", function.name);
        }

        // Get analysis results
        let lattice = analyzer.lattice().clone();
        let executable_blocks = analyzer.executable_blocks().clone();

        // Get initial statistics from analyzer
        let mut stats = analyzer.statistics().clone();
        stats.iterations = analyzer.iterations();

        // Apply rewriting transformations based on analysis results
        let rewrite_stats = rewriter::rewrite_function(function, lattice, executable_blocks);

        // Merge statistics
        stats.phi_nodes_simplified += rewrite_stats.phi_nodes_simplified;
        stats.branches_eliminated += rewrite_stats.branches_eliminated;
        stats.blocks_removed += rewrite_stats.blocks_removed;
        stats.total_blocks = rewrite_stats.total_blocks;

        // Validate postconditions
        if !validate_postconditions(function) {
            if self.verbose {
                println!("WARNING: Postcondition validation failed for function '{}'", function.name);
            }
        }

        stats
    }
}

impl Default for ConstantFoldingOptimizer {
    fn default() -> Self {
        Self { verbose: false, sccp_enabled: true, max_iterations: 100 }
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &'static str {
        "Constant Folding Optimizer (SCCP)"
    }

    fn run(&mut self, ir: &mut Module) {
        if self.sccp_enabled {
            self.run_sccp(ir);
        } else if self.verbose {
            println!("SCCP optimization disabled");
        }

        if self.verbose {
            println!("Total instructions after constant folding: {}", ir.count_instructions());
        }
    }
}
