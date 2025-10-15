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
        crate::error::compile_error::CompileError::GenericError(format!("Codegen error: {:?}", err))
    }
}
```

Last updated: 2025-10-15