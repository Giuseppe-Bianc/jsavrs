# Internal API Contracts: ABI-Compliant Parameter Passing

**Feature**: 025-abi-param-passing  
**Date**: 2026-02-12

---

## Module: `src/codegen/target.rs`

### `resolve_abi(target_triple: TargetTriple) -> Result<Abi, CompileError>`

Resolves a `TargetTriple` to its corresponding `Abi`. Returns `CompileError` for non-x86_64 targets.

**Input**: `TargetTriple` enum value  
**Output**: `Result<Abi, CompileError>`

| Input | Output |
|-------|--------|
| `X86_64UnknownLinuxGnu` | `Ok(Abi::SYSTEM_V_LINUX)` |
| `X86_64PcWindowsGnu` | `Ok(Abi::WINDOWS)` |
| `X86_64AppleDarwin` | `Ok(Abi::SYSTEM_V_MACOS)` |
| `AArch64UnknownLinuxGnu` | `Err(CompileError::AsmGeneratorError { ... })` |
| `AArch64AppleDarwin` | `Err(CompileError::AsmGeneratorError { ... })` |
| `AArch64PcWindowsGnu` | `Err(CompileError::AsmGeneratorError { ... })` |
| `I686PcWindowsGnu` | `Err(CompileError::AsmGeneratorError { ... })` |
| `I686UnknownLinuxGnu` | `Err(CompileError::AsmGeneratorError { ... })` |
| `Wasm32UnknownEmscripten` | `Err(CompileError::AsmGeneratorError { ... })` |

**Error contract**: Error message must include the unsupported triple's string representation (`target_triple.as_str()`).

---

## Module: `src/codegen/param.rs`

### `classify_param_type(ir_type: &IrType) -> Result<ParamClass, CompileError>`

Classifies a single `IrType` into `ParamClass::Integer` or `ParamClass::Float`.

| IrType | Result |
|--------|--------|
| `I8`, `I16`, `I32`, `I64` | `Ok(ParamClass::Integer)` |
| `U8`, `U16`, `U32`, `U64` | `Ok(ParamClass::Integer)` |
| `Bool` | `Ok(ParamClass::Integer)` |
| `Char` | `Ok(ParamClass::Integer)` |
| `Pointer(*)` | `Ok(ParamClass::Integer)` |
| `F32`, `F64` | `Ok(ParamClass::Float)` |
| `String` | `Err(CompileError)` |
| `Array(*, *)` | `Err(CompileError)` |
| `Struct(*, *, *)` | `Err(CompileError)` |
| `Custom(*, *)` | `Err(CompileError)` |
| `Void` | `Err(CompileError)` |

### `classify_parameters(params: &[IrParameter], abi: &Abi) -> Result<Vec<ParamAssignment>, CompileError>`

Classifies all parameters according to the ABI.

**Preconditions**:

- `abi` must be a valid x86_64 ABI (enforced by `resolve_abi`)

**Postconditions**:

- Returns exactly `params.len()` assignments on success
- Each `ParamAssignment.index` matches the parameter's position
- No two assignments share the same `ParamLocation`
- Stack offsets are in ascending order
- First stack offset is 16 (System V) or 48 (Windows)

**Algorithm (System V)**:
```
gp_idx = 0, fp_idx = 0, stack_offset = 16
for (i, param) in params:
    match classify(param.ty):
        Integer: if gp_idx < 6 → GpRegister(INT_REGS[gp_idx++])
                 else → Stack(stack_offset); stack_offset += 8
        Float:   if fp_idx < 8 → XmmRegister(FP_REGS[fp_idx++])
                 else → Stack(stack_offset); stack_offset += 8
```

**Algorithm (Windows x64)**:
```
slot = 0, stack_offset = 48
for (i, param) in params:
    if slot < 4:
        match classify(param.ty):
            Integer → GpRegister(INT_REGS[slot])
            Float   → XmmRegister(FP_REGS[slot])
        slot++
    else:
        Stack(stack_offset); stack_offset += 8
```

---

## Module: `src/codegen/stack.rs`

### `compute_stack_frame(func: &Function, abi: &Abi, used_callee_saved_gp: &[GPRegister64], used_callee_saved_xmm: &[XMMRegister]) -> StackFrame`

Computes the stack frame layout for a function.

**Input**:
- `func` — the IR function (provides `local_vars` for size calculation)
- `abi` — target ABI (provides shadow space, red zone size)
- `used_callee_saved_gp` — GP registers that need preservation
- `used_callee_saved_xmm` — XMM registers that need preservation (Windows only)

**Output**: `StackFrame` struct with all fields computed.

**Alignment Contract**:
```
After: push rbp + push(callee_saved_gp...) + sub rsp, total_stack_alloc
RSP must be ≡ 0 (mod 16)
```

---

## Module: `src/codegen/prologue.rs`

### `gen_prologue(abi: &Abi, stack_frame: &StackFrame) -> Vec<String>`

Generates the prologue assembly instruction strings.

**Output contract** (System V, no callee-saved, leaf with locals ≤ 128):
```asm
push rbp
mov rbp, rsp
; (no sub rsp — using red zone)
```

**Output contract** (System V, with callee-saved RBX, R12, non-leaf):
```asm
push rbp
mov rbp, rsp
push rbx
push r12
sub rsp, N    ; N ensures 16-byte alignment
```

**Output contract** (Windows, with callee-saved RBX):
```asm
push rbp
mov rbp, rsp
push rbx
sub rsp, N    ; N includes 32-byte shadow space + padding for alignment
```

---

## Module: `src/codegen/epilogue.rs`

### `gen_epilogue(abi: &Abi, stack_frame: &StackFrame) -> Vec<String>`

Generates the epilogue assembly instruction strings. Mirrors the prologue in reverse.

**Output contract** (matching System V prologue with RBX, R12):
```asm
add rsp, N
pop r12
pop rbx
pop rbp
ret
```

**Alternative using frame pointer shortcut**:
```asm
lea rsp, [rbp - 16]    ; 16 = 2 callee-saved pushes * 8
pop r12
pop rbx
pop rbp
ret
```

---

## Module: `src/codegen/ret.rs`

### `get_return_register(return_type: &IrType, abi: &Abi) -> Result<Option<ReturnLocation>, CompileError>`

Returns the register(s) where the return value should be placed.

**Output contract**:

| IrType | Result |
|--------|--------|
| `Void` | `Ok(None)` |
| `I8`/`U8`/`Bool` | `Ok(Some(ReturnLocation::Gp(EAX)))` (or RAX with zero-extension) |
| `I16`/`U16`/`Char` | `Ok(Some(ReturnLocation::Gp(EAX)))` |
| `I32`/`U32` | `Ok(Some(ReturnLocation::Gp32(EAX)))` |
| `I64`/`U64`/`Pointer` | `Ok(Some(ReturnLocation::Gp64(RAX)))` |
| `F32` | `Ok(Some(ReturnLocation::Xmm(XMM0)))` |
| `F64` | `Ok(Some(ReturnLocation::Xmm(XMM0)))` |
| `String`/`Array`/`Struct` | `Err(CompileError)` |

---

## Module: `src/codegen/error.rs`

### Error Construction Helpers

```rust
pub fn unsupported_param_type(ir_type: &IrType, func_name: &str) -> CompileError
pub fn unsupported_target(triple: &TargetTriple) -> CompileError
```

These produce `CompileError::AsmGeneratorError` with standardized messages. Centralizing error construction ensures consistent formatting.

---

## Integration Contract: `AsmGen::gen_function`

The existing `gen_function(&mut self, func: &Function)` method is extended to orchestrate:

```
1. resolve_abi(self.ir.target_triple)      → Abi  (or push error, skip)
2. classify_parameters(&func.params, &abi) → Vec<ParamAssignment>  (or push error, skip)
3. compute_stack_frame(func, &abi, ...)    → StackFrame
4. gen_prologue(&abi, &stack_frame)        → emit to assembly_file
5. [existing IR → instruction translation]
6. gen_return(&func.return_type, &abi)     → emit to assembly_file
7. gen_epilogue(&abi, &stack_frame)        → emit to assembly_file
```

On any error, the error is pushed to `self.errors` and the function is skipped (no partial output).
