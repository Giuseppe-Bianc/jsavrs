# Feature Specification: ABI-Compliant Parameter Passing and Return Value Handling

**Feature Branch**: `025-abi-param-passing`  
**Created**: 2026-02-12  
**Status**: Draft  
**Input**: User description: "Implement parameter passing and return value handling in the assembly code generator's gen_function function. The function must correctly map IR function parameters to their corresponding registers or stack positions according to the target platform's ABI (System V for Linux/macOS, Microsoft x64 for Windows). Parameters beyond register capacity must be spilled to the stack at the correct offsets. Return values must be placed in the appropriate return register(s) based on the return type - RAX/RDX for integers up to 128 bits, XMM0/XMM1 for floating-point returns. The implementation must respect the ABI's calling convention including register preservation rules (callee-saved vs caller-saved), shadow space allocation on Windows (32 bytes), and red zone considerations on System V (128 bytes). Function prologue must allocate stack space for local variables and preserve callee-saved registers that will be used. Function epilogue must restore preserved registers and adjust the stack pointer before returning. The parameter mapping must handle both integer parameters (passed in RDI/RSI/RDX/RCX/R8/R9 on System V, RCX/RDX/R8/R9 on Windows) and floating-point parameters (XMM0-XMM7 on System V, XMM0-XMM3 on Windows)."

## Clarifications

### Session 2026-02-12

- Q: How should non-scalar IR parameter types (Pointer, Bool, Char, String, Array, Struct) be classified for ABI register assignment? → A: Pointer, Bool, and Char are treated as integer types (passed in GP registers). String, Array, and Struct parameters generate a compile error (out of scope).
- Q: Should the function prologue always emit the frame pointer setup (push RBP / mov RBP, RSP), or should it be optional? → A: Always emit the frame pointer. Omission may be added as a future optimization.
- Q: How should the `by_val` attribute on `IrParameter` affect register assignment? → A: Ignore `by_val` — classify parameters solely by their `IrType`. `by_val` is only relevant for aggregates which are out of scope.
- Q: Should stack-spilled parameter offsets be calculated relative to RBP (frame pointer) or RSP (stack pointer)? → A: Relative to RBP. Since the frame pointer is always emitted, RBP-relative offsets are stable and simplify offset calculation.
- Q: How should the code generator handle unsupported target triples (non-x86_64)? → A: Emit a CompileError and skip the function (no panic). The user receives a clear diagnostic.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Integer Parameter Mapping to Registers (Priority: P1)

A compiler developer compiles a function that accepts integer parameters. The code generator must correctly assign each parameter to the ABI-designated register for the target platform. On System V, the first six integer parameters are placed in RDI, RSI, RDX, RCX, R8, R9 respectively. On Windows x64, the first four integer parameters are placed in RCX, RDX, R8, R9. Any remaining integer parameters beyond the register capacity are placed on the stack at the correct offsets.

**Why this priority**: Integer parameter passing is the most fundamental calling convention capability. Without correct register assignment for integer arguments, no function calls can work correctly. This is the MVP for any ABI-compliant code generator.

**Independent Test**: Can be fully tested by compiling functions with 1 to 10 integer parameters on each target platform and verifying that the generated assembly assigns registers and stack slots correctly.

**Acceptance Scenarios**:

1. **Given** an IR function with 3 integer parameters targeting System V, **When** assembly is generated, **Then** the parameters are mapped to RDI, RSI, and RDX in that order.
2. **Given** an IR function with 3 integer parameters targeting Windows x64, **When** assembly is generated, **Then** the parameters are mapped to RCX, RDX, and R8 in that order.
3. **Given** an IR function with 8 integer parameters targeting System V, **When** assembly is generated, **Then** the first 6 are in registers (RDI through R9) and the remaining 2 are at the correct stack offsets.
4. **Given** an IR function with 6 integer parameters targeting Windows x64, **When** assembly is generated, **Then** the first 4 are in registers (RCX, RDX, R8, R9) and the remaining 2 are at the correct stack offsets.

---

### User Story 2 - Function Prologue and Epilogue Generation (Priority: P1)

A compiler developer compiles any function and the code generator must produce a correct function prologue and epilogue. The prologue must save the frame pointer, allocate stack space for locals, and preserve all callee-saved registers that the function body uses. The epilogue must restore those registers and the stack pointer, then return. On Windows, the prologue must also allocate 32 bytes of shadow space. On System V, the red zone (128 bytes below RSP) may be used for leaf functions instead of adjusting RSP.

**Why this priority**: Prologue/epilogue generation is equally foundational to parameter passing — without it, the stack frame is invalid and any function call or return will corrupt the program's state.

**Independent Test**: Can be fully tested by compiling functions that use varying numbers of callee-saved registers and verifying that push/pop instructions and stack adjustments appear correctly in the generated assembly.

**Acceptance Scenarios**:

1. **Given** a function targeting Windows x64 that uses callee-saved registers RBX and R12, **When** assembly is generated, **Then** the prologue pushes RBP, RBX, R12, sets up the frame pointer, and allocates stack space including 32 bytes of shadow space.
2. **Given** a function targeting System V that uses no callee-saved registers and has no locals, **When** assembly is generated, **Then** the prologue always includes the frame pointer setup (`push RBP; mov RBP, RSP`) and the red zone may be used for small locals without adjusting RSP.
3. **Given** any function, **When** assembly is generated, **Then** the epilogue restores all callee-saved registers in the reverse order they were saved, restores the stack pointer, and issues a `ret` instruction.
4. **Given** a function that uses callee-saved registers RSI and RDI on Windows, **When** assembly is generated, **Then** those registers are NOT preserved (they are caller-saved on Windows x64).

---

### User Story 3 - Return Value Placement (Priority: P2)

A compiler developer compiles a function that returns a value. The code generator must place the return value in the ABI-specified return register. Integer return values up to 64 bits go in RAX. Integer return values up to 128 bits use the RAX:RDX register pair. Floating-point return values go in XMM0, and 128-bit floating-point returns use XMM0:XMM1.

**Why this priority**: Return values are essential for function output but can be tested after basic parameter passing and prologue/epilogue are working, since a function must first set up correctly before it can return correctly.

**Independent Test**: Can be fully tested by compiling functions with different return types (integer 32-bit, integer 64-bit, integer 128-bit, float, double) and verifying the return register assignment in generated assembly.

**Acceptance Scenarios**:

1. **Given** a function returning a 32-bit integer, **When** assembly is generated, **Then** the return value is placed in EAX (lower 32 bits of RAX).
2. **Given** a function returning a 64-bit integer, **When** assembly is generated, **Then** the return value is placed in RAX.
3. **Given** a function returning a 128-bit integer, **When** assembly is generated, **Then** the return value is split across RAX (low 64 bits) and RDX (high 64 bits).
4. **Given** a function returning a double-precision float, **When** assembly is generated, **Then** the return value is placed in XMM0.

---

### User Story 4 - Floating-Point Parameter Mapping (Priority: P2)

A compiler developer compiles a function that accepts floating-point parameters. On System V, up to 8 floating-point parameters are passed in XMM0 through XMM7. On Windows x64, floating-point parameters use the same positional slots as integer parameters (XMM0–XMM3 for positions 1–4), and excess parameters go on the stack.

**Why this priority**: Floating-point support extends the parameter passing to cover the full range of data types but builds on the same infrastructure as integer parameter mapping.

**Independent Test**: Can be fully tested by compiling functions with varying numbers of floating-point parameters on each target platform and verifying register/stack assignments.

**Acceptance Scenarios**:

1. **Given** an IR function with 3 floating-point parameters targeting System V, **When** assembly is generated, **Then** parameters are mapped to XMM0, XMM1, and XMM2.
2. **Given** an IR function with 10 floating-point parameters targeting System V, **When** assembly is generated, **Then** the first 8 are in XMM0–XMM7 and the remaining 2 are on the stack.
3. **Given** an IR function with 2 floating-point parameters targeting Windows x64, **When** assembly is generated, **Then** parameters are mapped to XMM0 and XMM1 (positional slots 1 and 2).
4. **Given** an IR function with 6 floating-point parameters targeting Windows x64, **When** assembly is generated, **Then** the first 4 use XMM0–XMM3 and the remaining 2 go on the stack.

---

### User Story 5 - Mixed Integer and Floating-Point Parameters (Priority: P3)

A compiler developer compiles a function with a mix of integer and floating-point parameters. The code generator must correctly track separate register allocation counters for integer and floating-point registers on System V (independent integer and FP register sequences), and shared positional slots on Windows x64 (each parameter consumes one positional slot regardless of type).

**Why this priority**: Mixed-type parameter passing is a common real-world scenario but is the most complex to implement correctly, especially with the divergent integer/FP tracking rules between the two ABIs.

**Independent Test**: Can be fully tested by compiling functions with interleaved integer and float parameters and verifying the register assignments match the ABI specification for each platform.

**Acceptance Scenarios**:

1. **Given** a function `f(int, float, int, float)` targeting System V, **When** assembly is generated, **Then** integers go in RDI and RSI, floats go in XMM0 and XMM1 (independent sequences).
2. **Given** a function `f(int, float, int, float)` targeting Windows x64, **When** assembly is generated, **Then** parameter 1 (int) goes in RCX, parameter 2 (float) goes in XMM1, parameter 3 (int) goes in R8, parameter 4 (float) goes in XMM3 (shared positional slots).
3. **Given** a function with many mixed parameters exceeding register capacity, **When** assembly is generated, **Then** excess parameters are placed on the stack in the correct order regardless of type.

---

### Edge Cases

- What happens when a function has zero parameters? The prologue/epilogue must still be generated correctly without any parameter mapping.
- What happens when a function has zero local variables? Stack allocation should be minimal (shadow space on Windows, no RSP adjustment needed on System V for leaf functions using the red zone).
- What happens when all callee-saved registers are used? The prologue must push all of them and the epilogue must pop all of them in reverse order, with stack alignment maintained at 16 bytes.
- How does the system handle a function with only stack-spilled parameters (more parameters than available registers)? All excess parameters must be read from their correct RBP-relative stack offsets (starting at `[RBP+16]`, incrementing by 8 bytes per parameter).
- What happens when the stack allocation size must be aligned to 16 bytes? The generated code must ensure the stack pointer remains 16-byte aligned at all times, inserting padding if necessary.
- What happens with a void function (no return value)? No return register assignment is generated, but the epilogue and `ret` instruction are still emitted.
- What happens when a parameter has an unsupported type (String, Array, Struct)? The code generator must emit a compile error indicating that the type is not supported for ABI parameter passing.
- What happens when the target triple is not x86_64 (e.g., AArch64, i686, Wasm32)? The code generator must emit a `CompileError` with a clear diagnostic message and skip code generation for that function, without panicking.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The code generator MUST map integer parameters to the correct ABI-specified registers based on the target platform — RDI/RSI/RDX/RCX/R8/R9 for System V, RCX/RDX/R8/R9 for Windows x64.
- **FR-002**: The code generator MUST map floating-point parameters to the correct ABI-specified XMM registers — XMM0–XMM7 for System V, XMM0–XMM3 (positional) for Windows x64.
- **FR-003**: The code generator MUST spill parameters that exceed the register capacity to the stack and access them using RBP-relative offsets (e.g., `[RBP+16]` for the first stack parameter, `[RBP+24]` for the second, etc.).
- **FR-004**: The code generator MUST use independent register allocation sequences for integer and floating-point parameters on System V, and shared positional slots on Windows x64.
- **FR-005**: The code generator MUST always emit a frame pointer setup (`push RBP; mov RBP, RSP`) in the function prologue, followed by stack space allocation for locals and preservation of all callee-saved registers that the function body will modify. Frame pointer omission is not supported in this feature.
- **FR-006**: The code generator MUST emit a function epilogue that restores all preserved callee-saved registers in reverse order, restores the stack pointer, and issues a `ret` instruction.
- **FR-007**: The code generator MUST allocate 32 bytes of shadow space in the function prologue on Windows x64 targets.
- **FR-008**: The code generator MUST respect the 128-byte red zone on System V targets for leaf functions, avoiding unnecessary RSP adjustments when locals fit within the red zone.
- **FR-009**: The code generator MUST place integer return values in RAX (up to 64 bits) or the RAX:RDX pair (up to 128 bits).
- **FR-010**: The code generator MUST place floating-point return values in XMM0 (up to 64 bits) or XMM0:XMM1 (up to 128 bits).
- **FR-011**: The code generator MUST ensure the stack pointer is 16-byte aligned at all times, inserting padding bytes as needed.
- **FR-012**: The code generator MUST distinguish between callee-saved and caller-saved registers according to the target ABI and only preserve callee-saved registers that the function modifies.
- **FR-013**: The code generator MUST handle functions with zero parameters by generating a valid prologue/epilogue without parameter mapping.
- **FR-014**: The code generator MUST handle void functions (no return value) by omitting return register assignment while still emitting a valid epilogue.
- **FR-015**: The code generator MUST classify `Pointer`, `Bool`, and `Char` IR parameter types as integer types for register assignment purposes, and MUST emit a compile error for `String`, `Array`, and `Struct` parameter types (unsupported in this feature).
- **FR-016**: The code generator MUST ignore the `by_val` attribute on parameters and classify each parameter solely by its `IrType` for register assignment. The `by_val` attribute is only semantically relevant for aggregate types, which are out of scope.
- **FR-017**: The code generator MUST emit a `CompileError` and skip function code generation when the target triple does not map to a supported ABI (i.e., non-x86_64 targets). The compiler MUST NOT panic for unsupported targets.

### Key Entities

- **Parameter**: An input value to a function, characterized by its position (ordinal index), data type (integer or floating-point), and size (8, 16, 32, 64, or 128 bits). Each parameter is assigned to either a register or a stack slot. For classification purposes, `Pointer`, `Bool`, and `Char` types are treated as integer types; `String`, `Array`, and `Struct` types are unsupported and produce a compile error.
- **Register Assignment**: The mapping of a parameter to a specific hardware register (general-purpose or XMM) based on the ABI, parameter position, and data type.
- **Stack Slot**: A memory location on the stack, identified by an RBP-relative offset (e.g., `[RBP+16]`), used for parameters that exceed register capacity. The first stack parameter is at `[RBP+16]` (after the saved RBP and return address), with subsequent parameters at 8-byte increments.
- **Callee-Saved Register Set**: The set of registers that a function must preserve across a call (RBX, RBP, R12–R15 on both ABIs; additionally RSI, RDI on Windows x64).
- **Function Prologue**: The instruction sequence at function entry responsible for frame setup, stack allocation, and register preservation.
- **Function Epilogue**: The instruction sequence at function exit responsible for register restoration, stack de-allocation, and returning control to the caller.
- **ABI Configuration**: The target platform's calling convention rules, including register ordering, shadow space requirements, red zone size, and stack alignment constraints.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All functions with up to 6 integer parameters (System V) and 4 integer parameters (Windows) produce correct register assignments that match the ABI specification with 100% accuracy.
- **SC-002**: All functions with parameters exceeding register capacity produce correct stack spill offsets, verified against the expected ABI layout.
- **SC-003**: Every generated function contains a valid prologue and epilogue — callee-saved registers are preserved and restored in the correct order, stack alignment is maintained at 16 bytes throughout.
- **SC-004**: All existing compiler tests continue to pass after the change (zero regressions).
- **SC-005**: Functions with mixed integer and floating-point parameters produce correct independent (System V) or positional (Windows) register assignments for both ABIs.
- **SC-006**: Return values are placed in the correct register(s) for all supported return types — integer (RAX, RAX:RDX), floating-point (XMM0, XMM0:XMM1), and void (no register assignment).

## Assumptions

- The compiler's IR already distinguishes between integer and floating-point parameter types, providing sufficient type information for ABI-correct register classification.
- The target platform is determined before code generation (i.e., a target configuration or flag already exists in the compilation pipeline).
- The existing assembly code generator infrastructure supports emitting push/pop, mov, sub/add RSP, and ret instructions — this feature extends the `gen_function` function rather than building the instruction emitter from scratch.
- Stack alignment to 16 bytes is the baseline requirement for both ABIs (standard x86-64 alignment).
- Struct and aggregate return values (which may require hidden pointer parameters) are out of scope for this feature; only scalar integer and floating-point returns are addressed.
- Variadic functions (e.g., printf-style) are out of scope; only fixed-arity functions are handled.
- The `by_val` parameter attribute is ignored for register classification; it is only relevant for aggregate types which are excluded from this feature's scope.
