# Contract: Codegen Errors

```rust
use crate::ir::{IrType, ValueId};

pub enum CodegenError {
    UnsupportedInstruction { instruction: String, reason: String },
    UnsupportedType { ty: IrType, reason: String },
    RegisterAllocationFailed { value_id: ValueId },
    StackOverflow { requested_size: usize },
    InvalidOperand { description: String },
    AbiViolation { description: String },
    AssemblerFailure { assembler_output: String, enriched_message: String },
}

impl From<CodegenError> for crate::error::compile_error::CompileError {
    fn from(err: CodegenError) -> Self {
        // TODO: map CodegenError to CompileError variants with SourceSpan when available
        let message = match err {
            CodegenError::UnsupportedInstruction { instruction, reason } => 
                format!("Unsupported instruction '{}': {}", instruction, reason),
            CodegenError::UnsupportedType { ty, reason } => 
                format!("Unsupported type '{:?}': {}", ty, reason),
            CodegenError::RegisterAllocationFailed { value_id } => 
                format!("Register allocation failed for value {:?}", value_id),
            CodegenError::StackOverflow { requested_size } => 
                format!("Stack overflow: requested {} bytes", requested_size),
            CodegenError::InvalidOperand { description } => 
                format!("Invalid operand: {}", description),
            CodegenError::AbiViolation { description } => 
                format!("ABI violation: {}", description),
            CodegenError::AssemblerFailure { enriched_message, .. } => 
                enriched_message,
        };
        crate::error::compile_error::CompileError::AsmGeneratorError(message)
    }
}
```

Last updated: 2025-10-15