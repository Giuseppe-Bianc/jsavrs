//! Assembly code generation for x86-64 architecture using NASM syntax
pub mod generator;
pub mod instruction;
pub mod operand;
pub mod register;
pub mod exception;
pub mod rounding;
pub mod mxcsr;
pub mod abi;
pub mod validation;
