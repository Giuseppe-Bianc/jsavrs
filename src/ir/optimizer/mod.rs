pub mod constant_folding;
pub mod dead_code_elimination;
pub mod phase;

pub use constant_folding::ConstantFoldingOptimizer;
pub use dead_code_elimination::DeadCodeElimination;
pub use phase::{Phase, run_pipeline};
