// src/ir/mod.rs
pub mod ir;
pub mod types;
pub mod values;
pub mod symbol_table;
pub mod builder;
pub mod validator;
pub mod optimizations;

pub use ir::*;
pub use types::*;
pub use values::*;
pub use symbol_table::*;
pub use builder::*;
pub use validator::*;
pub use optimizations::*;