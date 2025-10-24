//! Assembly instruction data structures and helpers.
//!
//! This module groups types and utilities used to represent x86-64 assembly
//! instructions, their operands, and related small-value helpers. It exposes a
//! small, focused public surface by re-exporting key items from the
//! submodules so callers can `use asm::instruction::*` to access instruction
//! related types.
//!
//! Submodules:
//! - `immediate` — representations and helpers for immediate (constant)
//!   operands.
//! - `memory_operand` — memory addressing forms (base/index/scale/disp) and
//!   helpers to build and query them.
//! - `operand` — generic operand enum used by instructions (register, memory,
//!   immediate) and utilities to work with operands uniformly.
//! - `instruction` — instruction-level metadata: the instruction struct,
//!   opcode/encoding helpers, formatting and validation utilities.
//!
//! Design contract (short):
//! - Inputs: parsed or synthesized operand and opcode information.
//! - Outputs: strongly-typed Rust values that model assembly constructs.
//! - Side effects: none (pure data structures and small helpers); parsing/IO
//!   is handled elsewhere.
#![allow(clippy::module_inception)]
mod immediate;
mod instruction;
mod memory_operand;
mod operand;

pub use immediate::*;
pub use instruction::*;
pub use memory_operand::*;
pub use operand::*;
