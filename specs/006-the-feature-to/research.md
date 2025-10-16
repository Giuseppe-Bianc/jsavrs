# Research: Cross-Platform x86_64 Assembly Code Generator

## Executive Summary

This research document resolves all "NEEDS CLARIFICATION" items from the feature plan and documents concrete design choices and alternatives. It provides an IR instruction catalog, instruction-selection patterns for baseline x86_64, ABI calling convention details (Windows vs System V), a value-mapping architecture, stack-frame layout algorithm, register allocation strategy, and error-handling design.

## IR Instruction Catalog

Source: `src/ir/instruction.rs` (InstructionKind enum)

- Alloca { ty: IrType } — stack allocation similar to `alloca` in LLVM.
- Store { value: Value, dest: Value } — store a value to memory
- Load { src: Value, ty: IrType } — load a value from memory
- Binary { op, left, right, ty } — arithmetic/bitwise/comparison operations
- Unary { op, operand, ty } — unary operations
- Call { func, args, ty } — function call
- GetElementPtr { base, index, element_ty } — pointer arithmetic
- Cast { kind, value, from_ty, to_ty } — type conversions
- Phi { ty, incoming } — SSA phi nodes
- Vector { op, operands, ty } — vector ops (may be limited on baseline)

Notes:
- Each `Instruction` includes `result: Option<Value>` for destination and `DebugInfo` with `SourceSpan` for error mapping.
- `Value` kinds: Literal, Constant, Local, Global, Temporary; carries `IrType`.

## Instruction Selection Patterns (Baseline x86_64)

Principles:
- Prefer single-instruction translations when possible.
- Expand to multi-instruction sequences when required (division, modulo, large copies).
- Use GP registers for integer-like types; XMM registers for floating-point (SSE2 for f32/f64 where available). On baseline x86_64, SSE2 is considered available for modern OSes; if target forbids SSE2, fall back to x87 sequences (rare).

Mapping examples:
- Binary Add/Sub/Mul for integers: `add r/m64, r64` or `imul r64, r/m64` (with appropriate sizes)
- Divide/Modulo: use `idiv`/`div` requiring sign/zero extension into RDX:RAX and constrained operand in RAX
- Comparison: `cmp` + conditional set/move/jump (e.g., `sete al` then `movzx r64, al`)
- Load/Store: `mov` variants with size-specific encodings
- Casts: use `movsx`/`movzx` for integer widen/truncate; use SSE `cvtsi2sd`/`cvtsi2ss` and `cvttsd2si` for int<->float
- GetElementPtr: address calc `lea` instruction when possible; otherwise `imul` + `add` sequences

Unsupported or special-case operations:
- Vector operations: baseline supports SSE2; wide vector ops may be limited. For operations requiring AVX/AVX2, generator must reject or lower to scalar sequences.
- Large struct copies: implement via looped `mov` (`rep movsb`) or sequence of `mov` depending on size.

## ABI Calling Conventions: Windows vs System V

Source: `src/asm/abi.rs` and official ABI docs

Commonalities:
- Stack alignment: 16 bytes
- Integer and float parameter registers differ in ordering and count
- Windows requires 32-byte shadow space allocated by caller
- System V provides 128-byte red zone for leaf functions

Windows x64 specifics:
- Integer registers: RCX, RDX, R8, R9
- XMM registers: XMM0..XMM3
- Shadow space: 32 bytes (caller reserves)
- Callee-saved GP: RBX, RBP, RDI, RSI, R12-R15

System V AMD64 specifics:
- Integer registers: RDI, RSI, RDX, RCX, R8, R9
- XMM registers: XMM0..XMM7 (caller-saved)
- Red zone available: 128 bytes
- Callee-saved GP: RBX, RBP, R12-R15

Return value rules:
- Small integers returned in RAX (and RDX for 128-bit) per `abi.rs` implementation
- Floating returns in XMM0 (and XMM1 on SystemV for larger returns)
- Structs: small structs within size limits returned in registers; larger ones via hidden pointer (caller allocates, passes pointer as implicit first argument)

Variadic functions:
- Windows: variadic handling requires register parameter shadowing; specifics handled by ABI helper
- System V: similar but register saving conventions differ; generator must track which registers are used for fixed args so varargs know where to read

## Value Mapping Design

Goal: Type-safe conversion of `Value` to `Operand` enum. No string-based assembly building.

Proposed `Operand` enum (summary):
- Register(X86Register) — holds `X86Register` enum (GP64/XMM/etc.)
- Memory { base: X86Register, index: Option<(X86Register, i32)>, disp: i32, size: usize }
- Immediate(IrLiteralValue) — immediate literal encoded with type size
- Label(String) — for globals and function symbols (only for emitter)

ValueMapper responsibilities:
- Decide whether a `Value` is in a register, on stack, or immediate
- Ensure register class matches `IrType` via `register_class_for_type`
- Allocate stack slots for `Alloca` and temporaries using `StackFrame::allocate`
- For `GetElementPtr`, compute base+index*scale+disp operands using `LEA` when possible

Register class selection (matching earlier plan):
- Integers, pointers → GP
- F32/F64 → XMM
- Bools/Chars treated as integers with size 8/32

## Stack Frame Management

`StackFrame` structure:
- current_offset: i32 (offset from RBP, negative or zero)
- alignment: usize (ABI alignment, typically 16)
- local_area_size: i32
- allocations: HashMap<ValueId, i32> (maps ValueId → offset)

Allocation algorithm:
- Start with 0 (top of local area just below RBP)
- For new allocation of N bytes with alignment A:
  - Align current_offset down to A (align_down)
  - current_offset -= size
  - Save offset in allocations map
- Finalize frame: round local_area_size up to ABI alignment, subtract shadow space if Windows, ensure space for spilled values and saved callee-saved registers

Alignment helper (corrected):

fn align_down(value: i32, alignment: i32) -> i32 {
// For negative values (stack offsets), align to more negative (larger magnitude)
  if value >= 0 {
    value - (value % alignment)
  } else {
    // For negative: align to next lower (more negative) multiple
    let rem = (-value) % alignment;
    if rem == 0 { value } else { value - (alignment - rem) }
  }
}

Edge cases:
- Alloca with dynamic size requires runtime adjustment of RSP and must consider red zone rules (System V red zone cannot be used if rsp adjusted)
- Large stack frames may exceed allowable stack limits; generator will emit checks if necessary (optional later)

## Register Allocation Strategy

Initial approach: Linear-scan allocator

Rationale:
- Simpler to implement and sufficient for correctness in initial implementation
- Works well for relatively small function sizes typical in compiler-generated code

Key points:
- Maintain liveness intervals per ValueId
- Separate pools for GP64 and XMM registers
- Use `Abi` information to avoid allocating callee-saved registers without saving them
- Spilling: allocate stack slot via `StackFrame::allocate` and generate `mov` to/from memory
- Handle constrained instructions (e.g., `idiv` requires RAX/RDX) by reserving registers and emitting moves as needed

Future improvements: graph-coloring allocator for optimized register usage

## Error Handling Strategy

Define `CodegenError` enum with variants:
- UnsupportedInstruction { instruction: Instruction, reason: String }
- UnsupportedType { ty: IrType, reason: String }
- RegisterAllocationFailed { value_id: ValueId }
- StackOverflow { requested: usize }
- InvalidOperand { description }
- AbiViolation { description }
- AssemblerFailure { assembler_output: String, enriched: String }

Conversion: implement `From<CodegenError> for CompileError` mapping errors to existing compile error reporting structures (`src/error/compile_error.rs`) and include `SourceSpan` from `Instruction::debug_info` where available.

Assembler failure enrichment:
- When NASM returns an error, capture stdout/stderr
- Try to parse line/column information and map back to emitted assembly line and ultimately to IR `SourceSpan` using `CodegenContext`'s emitted instruction map

## Architecture Decisions (recap)

- Modular pipeline with `src/asm/codegen/` components
- Type-safe `Operand` enums and enumerated registers/instructions
- Linear-scan register allocator for initial implementation
- Precise `StackFrame` allocation with ABI alignment enforcement
- Dedicated `function_prologue.rs` and `function_epilogue.rs` for ABI compliance
- Comprehensive `CodegenError` and conversion to `CompileError`

## Alternatives Considered

- Use LLVM's backend: rejected to keep project self-contained and to maintain tight control over emitted NASM syntax and ABI specifics
- Use graph-coloring register allocator immediately: rejected for initial complexity; plan to iterate later
- Rely on string templates for assembly emission: rejected to avoid runtime errors; prefer enum-based emitter

## Next Steps (Phase 1 inputs)

- Create `data-model.md` capturing the data structures and invariants
- Create `contracts/` files for component APIs
- Create `quickstart.md` with examples and validation steps
- Run `.specify/scripts/powershell/update-agent-context.ps1 -AgentType copilot` to update AI agent context


---

_Last updated: 2025-10-15_
