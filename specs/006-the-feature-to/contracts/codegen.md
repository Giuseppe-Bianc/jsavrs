# Contract: CodeGenerator

```rust
use crate::ir::Module;
use crate::ir::Function;
use crate::asm::codegen::error::CodegenError;
use crate::asm::data_directive::AssemblyElement;

pub trait CodeGenerator {
    /// Generate assembly code for an entire IR module
    fn generate_module(&mut self, module: &Module) -> Result<String, CodegenError>;

    /// Generate assembly code for a single function
    fn generate_function(&mut self, function: &Function) -> Result<Vec<AssemblyElement>, CodegenError>;
}
```

Responsibilities:
- Orchestrate the code generation pipeline
- Manage `CodegenContext` lifetime
- Produce final NASM-formatted assembly text (module-level)

Error modes:
- Propagate `CodegenError` from subcomponents

Last updated: 2025-10-15