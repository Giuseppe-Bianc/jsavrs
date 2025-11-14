use crate::ir::{Module, Phase};

pub struct ConstantFoldingOptimizer {
    /// Whether to emit warnings for conservative decisions.
    pub verbose: bool,
}

impl ConstantFoldingOptimizer {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &'static str {
        "Constant Folding Optimizer"
    }

    fn run(&mut self, _ir: &mut Module) {
        if self.verbose {
            //println!("{ir}");
        }
    }
}
