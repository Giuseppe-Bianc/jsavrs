use crate::ir::{Module, Phase};

pub struct ConstantFoldingOptimizer {
    /// Whether to emit warnings for conservative decisions.
    pub verbose: bool,
    pub sccp: bool,
}

impl ConstantFoldingOptimizer {
    pub fn new(verbose: bool, sccp: bool) -> Self {
        Self { verbose, sccp }
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &'static str {
        "Constant Folding Optimizer"
    }

    fn run(&mut self, ir: &mut Module) {
        if self.sccp {
            println!("SCCP enabled");
        }
        println!("total number of instructions after constant folding: {}", ir.count_instructions());
        if self.verbose {
            //println!("{ir}");
        }
    }
}
