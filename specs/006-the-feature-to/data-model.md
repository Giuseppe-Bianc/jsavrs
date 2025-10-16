# Data Model: Code Generation

This document describes the data structures, relationships, and invariants used by the `src/asm/codegen` components.

## IR Format Specification

The jsavrs intermediate representation (IR) consists of several core data structures that are consumed by the code generator:

### Instructions

The `Instruction` struct contains:
- `kind: InstructionKind` - The operation to perform
- `result: Option<Value>` - Optional destination for the result value  
- `debug_info: DebugInfo` - Source location information for error reporting
- `scope: Option<ScopeId>` - Optional lexical scope information

`InstructionKind` enum variants include:
- `Alloca { ty: IrType }` - Stack allocation
- `Store { value: Value, dest: Value }` - Store a value to memory
- `Load { src: Value, ty: IrType }` - Load a value from memory  
- `Binary { op: IrBinaryOp, left: Value, right: Value, ty: IrType }` - Arithmetic/comparison operations
- `Unary { op: IrUnaryOp, operand: Value, ty: IrType }` - Unary operations
- `Call { func: Value, args: Vec<Value>, ty: IrType }` - Function calls
- `GetElementPtr { base: Value, index: Value, element_ty: IrType }` - Pointer arithmetic
- `Cast { kind: CastKind, value: Value, from_ty: IrType, to_ty: IrType }` - Type conversions
- `Phi { ty: IrType, incoming: Vec<(Value, String)> }` - SSA phi nodes
- `Vector { op: VectorOp, operands: Vec<Value>, ty: IrType }` - Vector operations

### Values

The `Value` struct contains:
- `id: ValueId` - Unique identifier (UUID)
- `kind: ValueKind` - Category of value
- `ty: IrType` - Type of the value
- `debug_info: Option<ValueDebugInfo>` - Optional debug information
- `scope: Option<ScopeId>` - Optional lexical scope

`ValueKind` enum variants include:
- `Literal(IrLiteralValue)` - Compile-time constant
- `Constant(IrConstantValue)` - Named compile-time constant  
- `Local(Arc<str>)` - Local variable
- `Global(Arc<str>)` - Global variable/function
- `Temporary(u64)` - Compiler-generated temporary

### Types

`IrType` enum variants include:
- Signed integers: `I8`, `I16`, `I32`, `I64`
- Unsigned integers: `U8`, `U16`, `U32`, `U64` 
- Floating point: `F32`, `F64`
- Other: `Bool`, `Char`, `String`, `Void`
- Complex: `Pointer(Box<IrType>)`, `Array(Box<IrType>, usize)`, `Struct(Arc<str>, Vec<(String, IrType)>, SourceSpan)`

## CodegenContext

- Fields:
  - `current_function: Function` — IR function being processed
  - `abi: Abi` — selected ABI for the target platform
  - `platform: Platform` — target platform
  - `register_allocator: RegisterAllocator` — mutable allocator state
  - `stack_frame: StackFrame` — stack allocations for the current function
  - `label_counter: u64` — unique label generator
  - `emitted_instructions: Vec<(AssemblyInstruction, Option<SourceSpan>)>` — mapping for error reporting
  - `temp_map: HashMap<ValueId, Operand>` — caches resolved operands for values to avoid redundant lookups during instruction selection
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
- `Memory { base: X86Register, index: Option<(X86Register, i32)>, disp: i32, size: usize }` — scale must be 1, 2, 4, or 8
- `Immediate(IrLiteralValue)` — supports integer and floating-point literals compatible with x86 immediate encoding
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
  - `fn spill(&mut self, value_id: ValueId, stack_frame: &mut StackFrame) -> Result<i32, CodegenError>` — returns stack offset where value was spilled

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

_Last updated: 2025-10-15_