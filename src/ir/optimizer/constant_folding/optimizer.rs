use crate::ir::{Module, Phase};

/// Configuration for the Constant Folding Optimizer
pub struct ConstantFoldingOptimizer {
    /// Whether to emit verbose optimization information
    pub verbose: bool,

    /// Whether to enable SCCP optimization
    pub sccp_enabled: bool,
}

impl ConstantFoldingOptimizer {
    pub fn new(verbose: bool, sccp_enabled: bool) -> Self {
        Self { verbose, sccp_enabled }
    }
}

impl Default for ConstantFoldingOptimizer {
    fn default() -> Self {
        Self { verbose: false, sccp_enabled: true }
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &'static str {
        "Constant Folding Optimizer (SCCP)"
    }

    fn run(&mut self, ir: &mut Module) {
        if !self.sccp_enabled && self.verbose {
            println!("SCCP optimization disabled");
        }
        println!("Total instructions after constant folding: {}", ir.count_instructions());
    }
}
