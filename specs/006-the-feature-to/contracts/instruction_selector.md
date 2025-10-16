# Contract: InstructionSelector

```rust
use crate::ir::Instruction;
use crate::asm::codegen::context::CodegenContext;
use crate::asm::codegen::error::CodegenError;
use crate::asm::data_directive::AssemblyElement;

pub trait InstructionSelector {
    /// Select assembly instructions for an IR instruction
    fn select_instruction(&self, instruction: &Instruction, context: &mut CodegenContext) -> Result<Vec<AssemblyElement>, CodegenError>;

    /// Check if an IR instruction is supported by baseline x86_64
    fn is_supported(&self, instruction: &Instruction) -> bool;
}
```

Responsibilities:
- Map IR instructions to sequences of assembly elements
- Ensure ABI-specific sequences are used when required

Last updated: 2025-10-15