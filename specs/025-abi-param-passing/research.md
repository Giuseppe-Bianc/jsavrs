# Phase 0 Research: x86_64 ABI Calling Conventions

**Feature Branch**: `025-abi-param-passing` | **Date**: 2026-02-12  
**Status**: Complete  
**Sources**: System V AMD64 ABI spec (version 1.0, Lu et al. 2023), Microsoft x64 Calling Convention docs (MSDN, updated 2025-07-26), Wikipedia x86 calling conventions (2026-01-31), Agner Fog's calling conventions PDF, existing `jsavrs` codebase (`src/asm/abi.rs`, `src/asm/register/x86_register.rs`)

---

## Topic 1: System V AMD64 ABI — Parameter Passing

### Decision

**CONFIRMED.** The register assignment order is:

| Slot | Integer/Pointer | Floating-Point |
|------|-----------------|----------------|
| 1    | RDI             | XMM0           |
| 2    | RSI             | XMM1           |
| 3    | RDX             | XMM2           |
| 4    | RCX             | XMM3           |
| 5    | R8              | XMM4           |
| 6    | R9              | XMM5           |
| 7    | —               | XMM6           |
| 8    | —               | XMM7           |

- **Integer and floating-point counters are independent.** A function `f(int a, double b, int c, double d)` assigns `a → RDI`, `b → XMM0`, `c → RSI`, `d → XMM1`. The integer counter increments only for integer/pointer arguments; the FP counter increments only for FP arguments.
- **Stack spill for excess parameters** starts at `[RBP+16]` when a frame pointer is used. The stack layout at function entry (after `push RBP; mov RBP, RSP`) is:
  - `[RBP+0]` = saved caller's RBP
  - `[RBP+8]` = return address (pushed by `call`)
  - `[RBP+16]` = first stack-spilled argument (7th integer or 9th FP, whichever comes first positionally)
  - `[RBP+24]` = second stack-spilled argument
  - ... each subsequent argument at +8 byte increments
- **No shadow space.** On System V, the return address is directly adjacent to the first stack argument.

### Rationale

The System V AMD64 ABI spec §3.2.3 ("Parameter Passing") defines the classification algorithm. Each parameter is classified independently as INTEGER or SSE (among others), and consumes a register from the corresponding class's sequence. The two register sequences are completely independent — a float parameter does not consume an integer slot and vice versa. This is the key differentiator from the Microsoft convention.

The `[RBP+16]` offset is a direct consequence of the x86_64 `call` instruction pushing an 8-byte return address, followed by the prologue's `push RBP` (another 8 bytes). With RBP pointing at the saved frame pointer, the caller's stack arguments begin 16 bytes above RBP.

### Alternatives Considered

- **RSP-relative addressing**: Possible but fragile — RSP changes as locals are allocated and registers are saved. RBP-relative is stable and simplifies offset calculation. The spec and plan already mandate always emitting a frame pointer, so RBP-relative is the correct choice.

### Codebase Alignment

The existing constants in `src/asm/register/x86_register.rs` match exactly:
```rust
pub const INT_PARAM_REGS_SYSTEMV: &[GPRegister64] =
    &[Rdi, Rsi, Rdx, Rcx, R8, R9]; // 6 registers ✓

pub const FLOAT_PARAM_REGS_SYSTEMV: &[XMMRegister] =
    &[Xmm0, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7]; // 8 registers ✓
```

---

## Topic 2: Microsoft x64 Calling Convention — Parameter Passing

### Decision

**CONFIRMED.** Microsoft x64 uses a **shared positional slot model**:

| Position | Integer/Pointer | Floating-Point |
|----------|-----------------|----------------|
| 1        | RCX             | XMM0           |
| 2        | RDX             | XMM1           |
| 3        | R8              | XMM2           |
| 4        | R9              | XMM3           |

- **Each parameter consumes one positional slot regardless of type.** For `f(int a, double b, int c, float d)`:
  - Position 1: `a` (int) → **RCX**
  - Position 2: `b` (double) → **XMM1** (not XMM0 — slot 1 was consumed by the int)
  - Position 3: `c` (int) → **R8**
  - Position 4: `d` (float) → **XMM3**
  
  This is directly confirmed by Microsoft's own example: `func3(int a, double b, int c, float d)` → `a in RCX, b in XMM1, c in R8, d in XMM3`.

- **32-byte shadow space is mandatory.** The caller must always allocate 32 bytes (4 × 8) on the stack before the `call` instruction, even if the callee has fewer than 4 parameters. The callee may use this space to spill register parameters ("home" them).

- **Stack spill offsets** depend on frame layout. With the standard frame pointer setup:
  - Before `call`: caller has allocated 32-byte shadow space + any stack args above it
  - `call` pushes 8-byte return address
  - Prologue does `push RBP; mov RBP, RSP`
  - Stack layout from RBP's perspective:
    - `[RBP+0]` = saved RBP
    - `[RBP+8]` = return address
    - `[RBP+16]` = shadow space slot 1 (home for RCX)
    - `[RBP+24]` = shadow space slot 2 (home for RDX)
    - `[RBP+32]` = shadow space slot 3 (home for R8)
    - `[RBP+40]` = shadow space slot 4 (home for R9)
    - `[RBP+48]` = **first stack-spilled argument (5th parameter)**
    - `[RBP+56]` = second stack-spilled argument (6th parameter)
    - ... each subsequent at +8

### Rationale

Microsoft's MSDN documentation states: "There's a strict one-to-one correspondence between a function call's arguments and the registers used for those arguments." The parameter type table shows integer and FP registers in the same positional columns, confirming shared slots. The shadow space is allocated by the **caller** and is part of the calling convention's invariant — it simplifies variadic function support and debugging.

The `[RBP+48]` offset for the 5th argument: 8 (saved RBP) + 8 (return address) + 32 (shadow space) = 48 bytes from RBP to the first real stack argument. This matches for **callee-side** frame pointer setup. From the **caller's** perspective before the `call`, the 5th argument is at `[RSP+32]` (immediately above the shadow space).

### Alternatives Considered

- **Not allocating shadow space for functions with ≤4 params**: INCORRECT — the ABI mandates it unconditionally. Violating this breaks callee assumptions and debugger expectations.
- **RSP-relative offsets in callee**: When `sub RSP, N` allocates locals, RSP-relative offsets to stack args become `[RSP + N + 48]`. RBP-relative is simpler and stable.

### Codebase Alignment

```rust
pub const INT_PARAM_REGS_WINDOWS: &[GPRegister64] =
    &[Rcx, Rdx, R8, R9]; // 4 registers ✓

pub const FLOAT_PARAM_REGS_WINDOWS: &[XMMRegister] =
    &[Xmm0, Xmm1, Xmm2, Xmm3]; // 4 registers ✓

// Abi::shadow_space() returns 32 for Windows ✓
```

---

## Topic 3: Stack Alignment

### Decision

**CONFIRMED.** Both ABIs require **16-byte stack alignment at the point of the `call` instruction**.

Detailed alignment mechanics:

1. **Before `call`**: RSP must be 16-byte aligned. The `call` instruction pushes an 8-byte return address, making RSP **misaligned** (8 mod 16) at function entry.
2. **`push RBP`** (8 bytes) restores RSP to 16-byte alignment.
3. **After `push RBP; mov RBP, RSP`**: RSP is 16-byte aligned again.
4. **Local variable allocation** (`sub RSP, N`): N must be chosen so RSP remains 16-byte aligned. If the number of callee-saved register pushes is odd, an extra 8-byte padding is needed.

**Alignment formula for the prologue**:

```
pushes = 1 (RBP) + number_of_callee_saved_pushes
total_alloc = locals_size + shadow_space (Windows only)
padding = if (pushes + total_alloc / 8) % 2 != 0 then 8 else 0
final_sub = total_alloc + padding
```

More precisely: after all pushes, RSP is `entry_RSP - 8*pushes`. Since `entry_RSP` is 8-mod-16 (due to the return address), RSP after pushes is `(8 + 8*pushes)` below a 16-byte boundary. If `pushes` is odd, RSP is 16-byte aligned; if even, it's 8-mod-16. The `sub RSP, N` must bring it to 16-byte alignment before any `call`.

### Rationale

The System V ABI §3.2.2 states: "The end of the input argument area shall be aligned on a 16-byte boundary." Microsoft's MSDN states: "The stack will always be maintained 16-byte aligned, except within the prolog." SSE instructions like `movaps` require 16-byte alignment and will fault otherwise. All modern x86_64 ABIs agree on this requirement.

### Alternatives Considered

- **8-byte alignment only**: INCORRECT — legacy 32-bit approach. x86_64 universally requires 16.
- **Aligning at `call` vs at function entry**: The ABI specifies alignment at `call` time. At function entry, RSP is 16-byte aligned - 8 (because `call` pushed 8 bytes). This is an important distinction.

---

## Topic 4: Red Zone (System V Only)

### Decision

**CONFIRMED.**

- **128 bytes below RSP** are reserved as the "red zone" on System V AMD64.
- This area **will not be clobbered** by signal handlers, interrupt handlers, or asynchronous events.
- **Safe to use ONLY in leaf functions** — functions that do not call any other functions.
- If a function calls another function, the `call` instruction will push a return address that overwrites the red zone.
- **Windows has NO red zone** — `Abi::red_zone()` correctly returns 0 for Windows.

**Usage in codegen**:
- If a System V function is a leaf AND its total local variable size ≤ 128 bytes, the prologue can skip `sub RSP, N` and use `[RSP-8]`, `[RSP-16]`, etc. for locals.
- The frame pointer setup (`push RBP; mov RBP, RSP`) should still be emitted (per plan requirement FR-005), but RSP adjustment for locals can be omitted.
- If the function calls any other function OR locals exceed 128 bytes, the red zone optimization must be disabled.

### Rationale

System V ABI §3.2.2 defines the red zone: "The 128-byte area beyond the location pointed to by %rsp is considered to be reserved and shall not be modified by signal or interrupt handlers." The `-mno-red-zone` flag exists in GCC/Clang to disable this optimization (required for kernel code where interrupts may clobber below RSP).

### Alternatives Considered

- **Always using RSP adjustment**: Simpler implementation, works everywhere, but wastes instructions for simple leaf functions. Since the plan spec (FR-008) explicitly requires respecting the red zone for leaf functions on System V, we must implement the optimization.
- **Red zone for non-leaf functions**: INCORRECT and DANGEROUS — a `call` would overwrite the red zone area.

### Codebase Alignment

```rust
// Abi::red_zone() already returns the correct values:
// SystemV → 128, Windows → 0 ✓
```

---

## Topic 5: Callee-Saved Registers

### Decision

**CONFIRMED.**

**System V AMD64 callee-saved (non-volatile) GP registers:**
| Register | Notes |
|----------|-------|
| RBX      | General purpose |
| RBP      | Frame pointer (always saved per our plan) |
| R12      | General purpose |
| R13      | General purpose |
| R14      | General purpose |
| R15      | General purpose |
| RSP      | Stack pointer (implicitly preserved) |

- **All XMM registers (XMM0–XMM15) are caller-saved on System V.** No XMM register preservation needed in callee.

**Microsoft x64 callee-saved (non-volatile) GP registers:**
| Register | Notes |
|----------|-------|
| RBX      | General purpose |
| RBP      | Frame pointer |
| RDI      | **Callee-saved on Windows, caller-saved on System V** |
| RSI      | **Callee-saved on Windows, caller-saved on System V** |
| R12      | General purpose |
| R13      | General purpose |
| R14      | General purpose |
| R15      | General purpose |
| RSP      | Stack pointer (implicitly preserved) |

**Microsoft x64 callee-saved XMM registers:**
| Register     | Notes |
|--------------|-------|
| XMM6–XMM15   | **10 XMM registers are callee-saved on Windows** |

**Key difference**: Windows has 2 additional GP callee-saved registers (RDI, RSI) and 10 callee-saved XMM registers. This means:
- On Windows, if the function uses RDI or RSI, they must be pushed/popped in prologue/epilogue.
- On Windows, if the function uses XMM6–XMM15, they must be saved (typically via `movaps [RSP+offset], XMMn` — requires 16-byte aligned stack slot for each 128-bit XMM register).

### Rationale

System V ABI §3.2.1 "Registers and the Stack Frame": "Registers %rbx, %rsp, %rbp, %r12 through %r15 'belong' to the calling function." Microsoft MSDN Caller/Callee-Saved Registers section: "The x64 ABI considers registers RBX, RBP, RDI, RSI, RSP, R12, R13, R14, R15, and XMM6-XMM15 nonvolatile."

### Alternatives Considered

- **Saving all registers unconditionally**: Wasteful — only registers actually modified by the function body need saving. The codegen should track which callee-saved registers are used and save only those (as specified in FR-012).
- **Ignoring XMM callee-saved on Windows**: INCORRECT — would corrupt the caller's XMM6–XMM15 values. This is a common source of bugs in Windows x64 codegen.

### Codebase Alignment

```rust
pub const CALLEE_SAVED_GP_SYSTEMV: &[GPRegister64] =
    &[Rbx, Rbp, R12, R13, R14, R15]; // ✓

pub const CALLEE_SAVED_GP_WINDOWS: &[GPRegister64] =
    &[Rbx, Rbp, Rdi, Rsi, R12, R13, R14, R15]; // ✓ (includes Rdi, Rsi)

pub const CALLEE_SAVED_XMM_WINDOWS: &[XMMRegister] =
    &[Xmm6, Xmm7, Xmm8, Xmm9, Xmm10, Xmm11, Xmm12, Xmm13, Xmm14, Xmm15]; // ✓

// System V: callee_saved_xmm_registers() returns &[] ✓
```

---

## Topic 6: Return Value Registers

### Decision

**CONFIRMED.**

**Integer return values (both ABIs):**
| Size        | Register(s) | Notes |
|-------------|-------------|-------|
| ≤ 8 bits    | AL (in RAX) | Zero/sign-extended as needed |
| ≤ 16 bits   | AX (in RAX) | Zero/sign-extended as needed |
| ≤ 32 bits   | EAX (in RAX)| Upper 32 bits of RAX are zeroed on write |
| ≤ 64 bits   | RAX         | Primary return register |
| 65–128 bits | RAX:RDX     | RAX = low 64 bits, RDX = high 64 bits |

**Floating-point return values:**

| Size        | System V     | Windows      |
|-------------|-------------|--------------|
| ≤ 64 bits (float/double) | XMM0 | XMM0 |
| 65–128 bits | XMM0:XMM1   | XMM0 only    |

**Key difference**: System V allows 128-bit FP returns split across XMM0:XMM1. Windows returns 128-bit vector types (`__m128`) in XMM0 alone (it's a single 128-bit register).

**Void return**: No register assignment, but epilogue and `ret` are still emitted.

### Rationale

System V ABI §3.2.3, classification and return: INTEGER class ≤ 2 eightbytes → RAX, RDX. SSE class ≤ 2 eightbytes → XMM0, XMM1. Microsoft MSDN: "A scalar return value that can fit into 64 bits, including the __m64 type, is returned through RAX. Nonscalar types including floats, doubles, and vector types such as __m128 are returned in XMM0."

### Alternatives Considered

- **Always using RAX for integer returns regardless of size**: This is effectively what happens — writes to EAX zero the upper 32 bits of RAX automatically on x86_64. Smaller sub-registers (AL, AX) are contained within RAX. The codegen should use the appropriately-sized mov instruction but the return value is always "in RAX."
- **Struct return via registers vs pointer**: For structs > 2 eightbytes (System V) or > 64 bits (Windows), a hidden pointer parameter is prepended. This is out of scope (per spec: only scalar returns).

### Codebase Alignment

```rust
pub const INT_RETURN_REGS: &[GPRegister64] = &[Rax, Rdx]; // ✓ (shared both ABIs)
pub const FLOAT_RETURN_REGS_SYSTEMV: &[XMMRegister] = &[Xmm0, Xmm1]; // ✓
pub const FLOAT_RETURN_REGS_WINDOWS: &[XMMRegister] = &[Xmm0]; // ✓ (Windows: single only)
```

---

## Topic 7: Best Practices for ABI Parameter Classification in Rust

### Decision

Use a **flat enum + linear classify function** returning a `Vec<ParamAssignment>`.

#### Recommended Data Model

```rust
/// Where a parameter is physically located after ABI classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamLocation {
    /// Parameter is passed in a general-purpose register.
    GpRegister(GPRegister64),
    /// Parameter is passed in an XMM register.
    XmmRegister(XMMRegister),
    /// Parameter is passed on the stack at an RBP-relative offset.
    Stack { offset: i32 },
}

/// The ABI classification result for a single parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamAssignment {
    /// Index of the parameter in the function signature (0-based).
    pub index: usize,
    /// Name of the parameter (from IR).
    pub name: String,
    /// The IR type of the parameter.
    pub ir_type: IrType,
    /// Where this parameter is located after ABI assignment.
    pub location: ParamLocation,
}

/// Classify all parameters of a function according to the ABI.
pub fn classify_parameters(
    params: &[IrParameter],
    abi: &Abi,
) -> Result<Vec<ParamAssignment>, CompileError> {
    // ...
}
```

#### Classification Algorithm (System V)

```
gp_index = 0
fp_index = 0
stack_offset = 16  // [RBP+16] = first stack arg

for each param in params:
    if param.type is integer/pointer/bool/char:
        if gp_index < 6:
            assign GpRegister(INT_PARAM_REGS[gp_index])
            gp_index += 1
        else:
            assign Stack(stack_offset)
            stack_offset += 8
    elif param.type is float/double:
        if fp_index < 8:
            assign XmmRegister(FLOAT_PARAM_REGS[fp_index])
            fp_index += 1
        else:
            assign Stack(stack_offset)
            stack_offset += 8
    else:
        return CompileError (unsupported type)
```

#### Classification Algorithm (Windows x64)

```
slot_index = 0
stack_offset = 48  // [RBP+48] = first stack arg (after ret addr + shadow)

for each param in params:
    if slot_index < 4:
        if param.type is float/double:
            assign XmmRegister(FLOAT_PARAM_REGS[slot_index])
        else:
            assign GpRegister(INT_PARAM_REGS[slot_index])
        slot_index += 1
    else:
        assign Stack(stack_offset)
        stack_offset += 8
```

### Rationale

1. **Enum for `ParamLocation`**: Idiomatic Rust — sum type cleanly models the three mutually exclusive possibilities (GP register, XMM register, stack). Pattern matching ensures exhaustive handling. No runtime overhead vs. tagged structs.

2. **`Vec<ParamAssignment>` return**: Simple, predictable, O(n) allocation. Functions typically have < 10 parameters, so Vec overhead is negligible. Returning a Vec rather than an iterator allows the caller to index, iterate, or slice freely.

3. **Free function, not builder pattern**: The classification is a pure, single-pass transformation with no configuration knobs beyond `(params, abi)`. A builder pattern would add complexity without benefit — there's nothing to incrementally configure. A plain function is the simplest correct solution.

4. **`Result<T, CompileError>` return**: Unsupported types (String, Array, Struct) produce errors, not panics. This aligns with the project's no-`unwrap()` policy and error handling conventions.

5. **Separate `classify_parameters` function in `param.rs`**: Clean module boundary — keeps classification logic isolated from instruction emission (prologue.rs, epilogue.rs, ret.rs). Testable independently. Follows Single Responsibility pattern from AGENTS.md.

### Alternatives Considered

| Alternative | Verdict | Why Not |
|-------------|---------|---------|
| **Builder pattern** (`ParamClassifier::new(abi).add(param).build()`) | Rejected | Over-engineered for a stateless, single-pass operation. No incremental state to accumulate beyond two counters. Adds API surface without benefit. |
| **Trait-based dispatch** (`trait ParamClassifier` with SystemV/Windows impls) | Rejected | Only 2 variants with a simple `match` on `AbiKind`. A trait hierarchy adds indirection and cognitive overhead for minimal benefit. If more ABIs are added later, refactoring from match to trait is straightforward. |
| **Return `SmallVec<[ParamAssignment; 8]>`** | Considered | Would avoid heap allocation for ≤8 params. Adds a dependency (`smallvec`) or manual inline-vec logic. For a compiler codegen path that's not perf-critical, `Vec` is clearer and sufficient. |
| **Return iterator (lazy)** | Rejected | Would require complex lifetime/borrowing due to the mutable counters. Callers need random access to assignments (e.g., "what register is param 3 in?"), which iterators don't support well. |
| **Single `Register` enum instead of separate GP/XMM** | Rejected | GP and XMM registers are fundamentally different — they use different instruction sets (`mov` vs `movss`/`movsd`), different sizes, and different save/restore mechanics. Keeping them as separate enum variants in `ParamLocation` makes the codegen's pattern matching clearer and prevents mixing. |

---

## Summary Table: Key ABI Differences

| Property | System V AMD64 | Microsoft x64 |
|----------|---------------|---------------|
| **Integer param registers** | RDI, RSI, RDX, RCX, R8, R9 (6) | RCX, RDX, R8, R9 (4) |
| **FP param registers** | XMM0–XMM7 (8) | XMM0–XMM3 (4) |
| **Register slot model** | Independent (int and FP counters separate) | Shared positional (one counter) |
| **Shadow space** | None (0 bytes) | 32 bytes mandatory |
| **Red zone** | 128 bytes (leaf functions only) | None (0 bytes) |
| **Stack arg start (from RBP)** | `[RBP+16]` | `[RBP+48]` |
| **Stack alignment** | 16-byte at `call` | 16-byte at `call` |
| **Callee-saved GP** | RBX, RBP, R12–R15 | RBX, RBP, RDI, RSI, R12–R15 |
| **Callee-saved XMM** | None | XMM6–XMM15 |
| **Integer return** | RAX (≤64b), RAX:RDX (128b) | RAX (≤64b), RAX:RDX (128b) |
| **FP return** | XMM0 (≤64b), XMM0:XMM1 (128b) | XMM0 only |

---

## Open Questions Resolved

1. ~~Do integer and FP counters share slots on System V?~~ **No — they are independent.** CONFIRMED.
2. ~~Is shadow space required even for functions with 0 params on Windows?~~ **Yes — always 32 bytes.** CONFIRMED.
3. ~~What's the first stack arg offset on Windows with frame pointer?~~ **`[RBP+48]`** (8 saved RBP + 8 return addr + 32 shadow). CONFIRMED.
4. ~~Is the red zone safe for non-leaf functions?~~ **No — only leaf functions.** CONFIRMED.
5. ~~Are XMM registers callee-saved on System V?~~ **No — all XMM are caller-saved on System V.** CONFIRMED.
6. ~~How does Windows handle 128-bit FP returns?~~ **Single XMM0 register (it's 128 bits wide).** System V uses XMM0:XMM1 for multi-eightbyte SSE returns. CONFIRMED.
7. ~~Best Rust pattern for param classification?~~ **Enum + classify function returning `Vec<ParamAssignment>`.** Decided.

---

## References

1. Lu, H.J. et al. "System V Application Binary Interface: AMD64 Architecture Processor Supplement (With LP64 and ILP32 Programming Models) Version 1.0." GitLab, 2023-05-23. §3.2.1–§3.2.3.
2. Microsoft. "x64 calling convention." MSDN, updated 2025-07-26. https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention
3. Microsoft. "Overview of x64 ABI conventions — x64 register usage." MSDN. https://learn.microsoft.com/en-us/cpp/build/x64-software-conventions
4. Microsoft. "x64 stack usage." MSDN, updated 2025-11-05. https://learn.microsoft.com/en-us/cpp/build/stack-usage
5. Wikipedia. "x86 calling conventions." Last edited 2026-01-31. https://en.wikipedia.org/wiki/X86_calling_conventions
6. Fog, Agner. "Calling conventions for different C++ compilers and operating systems." 2010. https://agner.org/optimize/calling_conventions.pdf
