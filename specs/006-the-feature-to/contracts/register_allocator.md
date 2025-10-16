# Contract: RegisterAllocator

```rust
use crate::ir::ValueId;
use crate::ir::IrType;
use crate::asm::codegen::context::CodegenContext;
use crate::asm::codegen::operand::Operand;
use crate::asm::codegen::error::CodegenError;

pub trait RegisterAllocator {
    fn allocate(&mut self, value_id: ValueId, ty: &IrType, context: &mut CodegenContext) -> Result<Operand, CodegenError>;
    fn free(&mut self, value_id: ValueId);
    fn spill(&mut self, value_id: ValueId, context: &mut CodegenContext) -> Result<i32, CodegenError>;
}
```

Responsibilities:
- Manage pools of GP and XMM registers
- Spill and restore values to the `StackFrame`

Last updated: 2025-10-15