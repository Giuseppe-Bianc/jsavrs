# Research: IR to x86-64 Assembly Code Generator

**Feature**: 021-ir-x86-codegen  
**Date**: 2025-12-16  
**Status**: Complete

## Overview

This document captures research findings for implementing the IR to x86-64 code generator. Each section addresses a specific unknown from the Technical Context.

---

## 1. Linear Scan Register Allocation

### Decision

Implement Linear Scan register allocation with liveness analysis using a greedy single-pass algorithm.

### Rationale

- **O(n) complexity**: Linear time vs O(n²) for graph coloring, suitable for large functions
- **Production-proven**: Used in V8, HotSpot JIT, and LLVM's fast register allocator
- **Quality/speed tradeoff**: Produces ~10-15% more spills than graph coloring but 5-10x faster
- **Incremental implementation**: Can start simple and add optimizations later

### Algorithm Overview

```text
1. Compute liveness intervals for all IR values
2. Sort intervals by start position
3. Maintain active list of currently live intervals
4. For each interval in order:
   a. Expire intervals that end before current start
   b. If free register available, allocate it
   c. Otherwise, spill: either current or longest active interval
5. Generate spill/reload code for spilled intervals
```

### Key Data Structures

```rust
struct LiveInterval {
    value: ValueId,           // IR value this interval represents
    start: usize,             // First use position
    end: usize,               // Last use position
    reg: Option<PhysReg>,     // Allocated register (None if spilled)
    spill_slot: Option<u32>,  // Stack slot if spilled
}

struct LinearScanAllocator {
    intervals: Vec<LiveInterval>,
    active: BTreeSet<IntervalId>,  // Sorted by end position
    free_regs: Vec<PhysReg>,
    next_spill_slot: u32,
}
```

### Spill Weight Heuristics

When deciding which interval to spill:

1. Prefer intervals with longer remaining lifetime
2. Prefer intervals with fewer uses (less reload cost)
3. Avoid spilling intervals used in hot loops

### Alternatives Considered

| Algorithm            | Complexity | Code Quality | Why Rejected                            |
| -------------------- | ---------- | ------------ | --------------------------------------- |
| Graph Coloring       | O(n²)      | Optimal      | Too complex for initial implementation  |
| Naive (always spill) | O(n)       | Poor         | Excessive memory traffic                |
| PBQP                 | O(n³)      | Near-optimal | Overkill for this project               |

---

## 2. SSA Phi Node Resolution

### Decision

Resolve phi nodes using the parallel copy approach with cycle detection and sequentialization.

### Rationale

- **Correctness**: Handles all cases including swap patterns
- **Efficiency**: Minimizes temporary registers/memory
- **Well-documented**: Standard approach in SSA literature

### Algorithm Overview

1. **Collect phi moves**: For each predecessor block, collect all (src, dst) moves
2. **Detect cycles**: Build dependency graph, find strongly connected components
3. **Break cycles**: Insert temporary for one edge in each cycle
4. **Sequentialize**: Topological sort of non-cyclic moves
5. **Insert at block ends**: Place moves before terminator in predecessor blocks

### Example: Swap Pattern

```nasm
; IR phi nodes in block3:
; %x = phi [%a from block1] [%b from block2]
; %y = phi [%b from block1] [%a from block2]

; From block1: need to move (%a→%x, %b→%y) - no cycle
; From block2: need to move (%b→%x, %a→%y) - CYCLE!

; Resolution for block2:
mov tmp, %a    ; Break cycle with temporary
mov %y, tmp    ; Now safe to overwrite %a
mov %x, %b     ; Original move
```

### Data Structures

```rust
struct PhiMove {
    src: ValueId,
    dst: ValueId,
}

struct ParallelCopy {
    moves: Vec<PhiMove>,
}

impl ParallelCopy {
    fn sequentialize(&self) -> Vec<(ValueId, ValueId)>;
    fn find_cycles(&self) -> Vec<Vec<PhiMove>>;
}
```

### Alternatives Considered

| Approach                    | Why Rejected                        |
| --------------------------- | ----------------------------------- |
| Naive sequential insertion  | Incorrect for swap patterns         |
| Always use temporaries      | Wastes registers                    |
| Out-of-SSA before codegen   | Requires separate pass, less clean  |

---

## 3. x86-64 Instruction Selection

### Decision

Use pattern-based instruction selection with direct mapping from IR operations to x86-64 instructions.

### Rationale

- **Simplicity**: Direct 1:1 or 1:few mappings for most operations
- **Extensibility**: Easy to add new patterns
- **Debuggability**: Clear correspondence between IR and assembly

### Instruction Mapping Table

| IR Operation   | x86-64 Instruction(s)      | Notes                     |
| -------------- | -------------------------- | ------------------------- |
| `add i32`      | `add eax, ebx`             | 2-address form            |
| `add i64`      | `add rax, rbx`             | 64-bit                    |
| `sub`          | `sub`                      | Same as add               |
| `mul i32`      | `imul eax, ebx`            | Signed multiply           |
| `mul u32`      | `imul eax, ebx`            | Same instruction          |
| `div i32`      | `cdq; idiv ebx`            | Signed, uses EDX:EAX      |
| `div u32`      | `xor edx, edx; div ebx`    | Unsigned                  |
| `rem`          | Same as div                | Result in EDX             |
| `and/or/xor`   | `and/or/xor`               | Direct mapping            |
| `shl/shr`      | `shl/shr`                  | CL for variable shift     |
| `load i32`     | `mov eax, [addr]`          | Size from type            |
| `store i32`    | `mov [addr], eax`          | Size from type            |
| `cmp + br`     | `cmp; jcc`                 | Fuse compare+branch       |
| `call`         | `call label`               | ABI-dependent setup       |
| `ret`          | `ret`                      | After epilogue            |

### Addressing Modes

```rust
enum Operand {
    Reg(PhysReg),
    Imm(i64),
    Mem { base: PhysReg, index: Option<PhysReg>, scale: u8, disp: i32 },
    Label(String),
}

// Array access: base + index * scale
// Struct field: base + offset
// Stack slot: [rbp - offset]
```

### Type Size Mapping

| IR Type         | Register      | Memory Size | Instruction Suffix |
| --------------- | ------------- | ----------- | ------------------ |
| i8/u8/bool/char | AL, BL, ...   | byte        | b                  |
| i16/u16         | AX, BX, ...   | word        | w                  |
| i32/u32         | EAX, EBX, ... | dword       | d                  |
| i64/u64/ptr     | RAX, RBX, ... | qword       | q                  |
| f32             | XMM0-15       | dword       | ss                 |
| f64             | XMM0-15       | qword       | sd                 |

### Alternatives Considered

| Approach                      | Why Rejected                      |
| ----------------------------- | --------------------------------- |
| Tree pattern matching (BURG)  | Overkill for x86-64               |
| Macro expansion               | Less control over output          |
| Peephole-only                 | Misses optimization opportunities |

---

## 4. NASM Syntax and Directives

### Decision

Use Intel syntax with NASM-specific directives for maximum compatibility.

### Rationale

- **Explicit requirement**: Spec requires NASM compatibility (FR-042)
- **Readability**: Intel syntax is more readable than AT&T
- **Portability**: NASM runs on all target platforms

### Section Directives

```nasm
section .text       ; Executable code
section .data       ; Initialized data
section .bss        ; Uninitialized data
section .rodata     ; Read-only data (Linux/macOS)
section .rdata      ; Read-only data (Windows)
```

### Symbol Visibility

```nasm
global main         ; Export symbol
extern printf       ; Import external symbol
```

### Data Directives

```nasm
db 0x41             ; Define byte
dw 0x1234           ; Define word (2 bytes)
dd 0x12345678       ; Define doubleword (4 bytes)
dq 0x123456789ABCDEF0 ; Define quadword (8 bytes)
times 100 db 0      ; Repeat directive

align 16            ; Alignment directive
```

### Platform-Specific Differences

| Aspect          | Linux   | macOS   | Windows |
| --------------- | ------- | ------- | ------- |
| Symbol prefix   | none    | `_`     | none    |
| Section names   | .rodata | .rodata | .rdata  |
| Default format  | elf64   | macho64 | win64   |

### NASM Command Line

```bash
# Linux
nasm -f elf64 -o output.o input.asm

# macOS
nasm -f macho64 -o output.o input.asm

# Windows
nasm -f win64 -o output.obj input.asm
```

---

## 5. Function Prologue and Epilogue

### Decision

Generate standard frame pointer prologues with callee-saved register preservation.

### Standard Prologue (all platforms)

```nasm
push rbp            ; Save old frame pointer
mov rbp, rsp        ; Set new frame pointer
sub rsp, N          ; Allocate stack space (16-byte aligned)
; Save callee-saved registers as needed
push rbx
push r12
; ... etc
```

### Standard Epilogue

```nasm
; Restore callee-saved registers (reverse order)
pop r12
pop rbx
mov rsp, rbp        ; Restore stack pointer
pop rbp             ; Restore frame pointer
ret
```

### Windows Shadow Space

```nasm
; Caller side (before call)
sub rsp, 32         ; Allocate shadow space
call function
add rsp, 32         ; Deallocate shadow space

; Callee side (in prologue)
; Can use [rbp+16..rbp+48] for spilling RCX, RDX, R8, R9
```

### System V Red Zone

On System V (Linux/macOS), leaf functions can use 128 bytes below RSP without adjusting the stack pointer:

```nasm
; Leaf function - no stack adjustment needed
; Can use [rsp-128] through [rsp-1]
mov [rsp-8], rax    ; Valid in red zone
ret
```

---

## 6. Callee-Saved Register Handling

### Platform Differences

| Register    | System V         | Windows          |
|-------------|------------------|------------------|
| RAX         | Caller-saved     | Caller-saved     |
| RCX         | Caller-saved     | Caller-saved     |
| RDX         | Caller-saved     | Caller-saved     |
| RBX         | **Callee-saved** | **Callee-saved** |
| RSP         | **Callee-saved** | **Callee-saved** |
| RBP         | **Callee-saved** | **Callee-saved** |
| RSI         | Caller-saved     | **Callee-saved** |
| RDI         | Caller-saved     | **Callee-saved** |
| R8-R11      | Caller-saved     | Caller-saved     |
| R12-R15     | **Callee-saved** | **Callee-saved** |
| XMM0-5      | Caller-saved     | Caller-saved     |
| XMM6-15     | Caller-saved     | **Callee-saved** |

### Strategy

1. Before allocation, mark callee-saved registers as "must preserve"
2. Track which callee-saved registers are actually used
3. Generate push/pop only for used callee-saved registers
4. Account for pushed registers in stack frame size calculation

---

## 7. Jump Table Implementation

### Decision

Use indexed jump tables for switch statements with ≥4 contiguous cases.

### Jump Table Format

```nasm
section .rodata
.switch_table:
    dq .case_0
    dq .case_1
    dq .case_2
    dq .case_3

section .text
    ; Assume switch value in RAX
    cmp rax, 3
    ja .default         ; Bounds check
    lea rcx, [rel .switch_table]
    jmp [rcx + rax * 8] ; Indexed jump
```

### Cascaded Comparisons (< 4 cases)

```nasm
    cmp rax, 0
    je .case_0
    cmp rax, 1
    je .case_1
    jmp .default
```

### Sparse Case Handling

For non-contiguous cases, normalize the index:

```nasm
    sub rax, min_case   ; Normalize to 0-based
    cmp rax, range
    ja .default
```

---

## Summary of Decisions

| Topic                | Decision                            | Key Benefit                        |
| -------------------- | ----------------------------------- | ---------------------------------- |
| Register Allocation  | Linear Scan                         | O(n) complexity, production-proven |
| Phi Resolution       | Parallel copy + sequentialization   | Handles all cases correctly        |
| Instruction Selection| Pattern-based direct mapping        | Simple, debuggable                 |
| Assembly Syntax      | NASM Intel                          | Required by spec, readable         |
| Prologue/Epilogue    | Standard frame pointer              | Debuggable, ABI-compliant          |
| Switch Statements    | Jump table ≥4 cases                 | Efficient for dense switches       |

---

## References

1. Poletto & Sarkar, "Linear Scan Register Allocation", ACM TOPLAS 1999
2. Wimmer & Mössenböck, "Optimized Interval Splitting in a Linear Scan Register Allocator", VEE 2005
3. Briggs et al., "Practical Improvements to the Construction and Destruction of SSA Form", SPE 1998
4. System V AMD64 ABI Specification
5. Microsoft x64 Calling Convention Documentation
6. NASM Documentation: <https://www.nasm.us/doc/>
