//! Comprehensive error type for assembly generation failures
//!
//! Defines all possible error conditions during assembly code generation
//! as specified in the data model and contract specifications.

use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;

/// Comprehensive error type for assembly generation
#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported IR instruction: {instruction}")]
    UnsupportedInstruction {
        instruction: String,
        span: SourceSpan,
    },
    
    #[error("Register allocation failed: {reason}")]
    RegisterAllocationFailure {
        reason: String,
        function: String,
        instruction_id: Option<String>,
    },
    
    #[error("Invalid operand combination for instruction {instruction}: {details}")]
    InvalidOperandCombination {
        instruction: String,
        details: String,
    },
    
    #[error("ABI constraint violation: {constraint}")]
    ABIViolation {
        constraint: String,
        function: String,
    },
    
    #[error("Type mismatch: {message}")]
    TypeMismatch {
        message: String,
        span: SourceSpan,
    },
    
    #[error("Symbol resolution failed: {symbol}")]
    SymbolResolutionFailure {
        symbol: String,
        span: SourceSpan,
    },
    
    #[error("Stack overflow: frame size {size} exceeds limit {limit}")]
    StackOverflow {
        size: u32,
        limit: u32,
        function: String,
    },
    
    #[error("Target platform not supported: {platform:?}")]
    UnsupportedPlatform {
        platform: crate::asm::platform::TargetPlatform,
    },
    
    #[error("Internal generator error: {message}")]
    InternalError {
        message: String,
        backtrace: Option<String>, // Simplified as backtrace is complex
    },
}

impl From<CodeGenError> for CompileError {
    fn from(codegen_error: CodeGenError) -> Self {
        CompileError::AsmGeneratorError {
            message: format!("{}", codegen_error),
        }
    }
}

/// Validation error for IR compatibility checking
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Unsupported IR instruction type: {instruction_type}")]
    UnsupportedInstructionType {
        instruction_type: String,
    },
    
    #[error("Unsupported data type: {data_type}")]
    UnsupportedDataType {
        data_type: String,
    },
    
    #[error("Function signature incompatible with target ABI")]
    IncompatibleFunctionSignature {
        function_name: String,
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_error_types() {
        let error = CodeGenError::UnsupportedInstruction {
            instruction: "unknown_instr".to_string(),
            span: SourceSpan::default(),
        };
        assert!(format!("{}", error).contains("Unsupported IR instruction"));
    }
}