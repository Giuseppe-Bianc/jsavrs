# Contract: ValueMapper

```rust
use crate::ir::Value;
use crate::asm::codegen::context::CodegenContext;
use crate::asm::codegen::operand::Operand;
use crate::asm::codegen::error::CodegenError;

pub trait ValueMapper {
    /// Map an IR value to an assembly operand
    fn map_value(&mut self, value: &Value, context: &mut CodegenContext) -> Result<Operand, CodegenError>;

    /// Determine register class for an IR type
    fn register_class_for_type(&self, ty: &crate::ir::IrType) -> crate::asm::register::RegisterClass;
}
```

Responsibilities:
- Convert `Value` to `Operand`
- Allocate stack slots for locals/temporaries
- Ensure register class correctness

Last updated: 2025-10-15