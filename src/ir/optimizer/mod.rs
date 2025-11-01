pub mod dead_code_elimination;
pub mod phase;

pub use dead_code_elimination::DeadCodeElimination;
pub use phase::{Phase, run_pipeline};
