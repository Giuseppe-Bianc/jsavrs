# Data Model: Code Generation

This document describes the data structures, relationships, and invariants used by the `src/asm/codegen` components.

## CodegenContext

- Fields:
  - `current_function: Function` — IR function being processed
  - `abi: Abi` — selected ABI for the target platform
  - `platform: Platform` — target platform
  - `register_allocator: RegisterAllocator` — mutable allocator state
  - `stack_frame: StackFrame` — stack allocations for the current function
  - `label_counter: u64` — unique label generator
  - `emitted_instructions: Vec<(AssemblyInstruction, Option<SourceSpan>)>` — mapping for error reporting
  - `temp_map: HashMap<ValueId, Operand>` — for keeping temporary allocations

- Invariants:
  - `stack_frame` offsets are aligned to `abi.alignment()`
  - `register_allocator` maintains a one-to-one mapping between live `ValueId` and allocated registers, unless spilled
  - `emitted_instructions` preserves chronological order mapping to IR debug spans

## StackFrame

- Fields:
  - `current_offset: i32` — current top-of-frame offset (negative values)
  - `alignment: usize` — ABI alignment (16)
  - `local_area_size: i32` — total computed size of locals
  - `allocations: HashMap<ValueId, i32>` — mapping ValueId -> offset

- Methods:
  - `fn allocate(&mut self, value_id: ValueId, size: usize, alignment: usize) -> i32` — allocate slot and return offset
  - `fn finalize(&mut self)` — finalize allocations and compute `local_area_size`

- Invariants:
  - Offsets stored are negative and monotonic decreasing as allocations grow
  - No overlapping offsets in allocations map

## Operand

Enum `Operand`:
- `Register(X86Register)`
- `Memory { base: X86Register, index: Option<(X86Register, i32)>, disp: i32, size: usize }`
- `Immediate(IrLiteralValue)`
- `Label(String)`

Invariants:
- Memory `disp` aligned to `size` or ABI alignment where appropriate
- Register class matches `IrType` size and float/int classification

## RegisterAllocator (model)

- Fields:
  - `gp_allocations: HashMap<ValueId, GPRegister64>`
  - `xmm_allocations: HashMap<ValueId, XMMRegister>`
  - `free_gp: Vec<GPRegister64>`
  - `free_xmm: Vec<XMMRegister>`

- Methods:
  - `fn allocate(&mut self, value_id: ValueId, ty: &IrType) -> Result<Operand, CodegenError>`
  - `fn free(&mut self, value_id: ValueId)`
  - `fn spill(&mut self, value_id: ValueId, stack_frame: &mut StackFrame) -> Result<i32, CodegenError>`

Invariants:
- `free_gp` excludes ABI-reserved callee-saved registers unless those are explicitly saved first
- Allocation ensures no duplicate allocations for same `ValueId`

## AssemblyInstruction

- Fields:
  - `mnemonic: InstructionMnemonic` (enum)
  - `operands: Vec<Operand>`
  - `comment: Option<String>`

Invariants:
- `operands.len()` matches `mnemonic` arity
- Operand types are compatible with mnemonic operand classes

## Relationships

- `CodegenContext` owns `StackFrame` and `RegisterAllocator` and coordinates between them
- `ValueMapper` reads `CodegenContext` to determine where a value resides and may mutate `RegisterAllocator` and `StackFrame` to allocate resources
- `InstructionSelector` consumes `Operand`s produced by `ValueMapper` and emits `AssemblyInstruction`s
- `Emitter` consumes `AssemblyInstruction`s and `AssemblySection`s to produce NASM text

## Examples

1. Allocation example:
```rust
let offset = stack_frame.allocate(value.id, 8, 8);
register_allocator.spill(value.id, &mut stack_frame)?;
```

2. Operand produced for a local integer:
```rust
Operand::Memory { base: GPRegister64::Rbp.into(), index: None, disp: -16, size: 8 }
```

_Last updated: 2025-10-15"}},{