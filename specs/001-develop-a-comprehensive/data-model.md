# Data Model: Comprehensive x86-64 ABI Trait System

**Feature**: 001-develop-a-comprehensive  
**Date**: October 2, 2025  
**Status**: Phase 1 Complete

## Overview

This document provides a detailed, precise, meticulous, and in-depth specification of all entities, data structures, types, and relationships required for the comprehensive x86-64 ABI trait system. Every entity is documented with its fields, validation rules, state transitions, and relationships to other entities in the system.

## 1. Core Platform Entities

### 1.1 Platform Enumeration

**Entity**: `Platform`

**Purpose**: Represents the target operating system for which assembly code is being generated

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
}
```

**Fields**:
- `Windows`: Microsoft Windows operating system (x64)
- `Linux`: Linux operating system (System V AMD64 ABI)
- `MacOS`: Apple macOS operating system (System V AMD64 ABI with potential extensions)

**Validation Rules**:
- Must be one of the three enumerated variants
- Type system enforces exhaustive pattern matching
- No invalid states possible

**Relationships**:
- Maps to `Abi` via `Abi::from_platform()`
- Used as parameter for register volatility queries
- Used as parameter for calling convention queries

**State Transitions**: Immutable (no state changes after selection)

**Usage Context**: Selected once at compilation start; determines all subsequent ABI queries

---

### 1.2 ABI Variant Enumeration

**Entity**: `Abi`

**Purpose**: Distinguishes between the two primary x86-64 calling conventions

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Abi {
    SystemV,
    Windows,
}
```

**Fields**:
- `SystemV`: System V AMD64 ABI (Linux, macOS, BSD)
- `Windows`: Microsoft x64 calling convention

**Validation Rules**:
- Must be one of the two variants
- Type system enforces validity
- No runtime validation required

**Relationships**:
- Derived from `Platform` via mapping function
- Linux → SystemV
- MacOS → SystemV
- Windows → Windows

**Mapping Function**:
```rust
impl Abi {
    pub fn from_platform(platform: Platform) -> Self {
        match platform {
            Platform::Windows => Abi::Windows,
            Platform::Linux | Platform::MacOS => Abi::SystemV,
        }
    }
}
```

**State Transitions**: Immutable after derivation from Platform

**Usage Context**: Determines specific calling convention rules for code generation

---

## 2. Register Entities

### 2.1 General Purpose Register Hierarchies

**Entity**: `GPRegister64` (64-bit General Purpose Registers)

**Purpose**: Represents 64-bit general-purpose registers in x86-64 architecture

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPRegister64 {
    Rax, Rbx, Rcx, Rdx,
    Rsi, Rdi, Rbp, Rsp,
    R8, R9, R10, R11, R12, R13, R14, R15,
}
```

**Fields**: 16 register variants representing all available 64-bit GPRs

**Properties**:
- **Size**: 64 bits / 8 bytes
- **Volatility**: Platform-dependent (see volatility classification)
- **Special Roles**:
  - `Rax`: Primary return value, accumulator
  - `Rsp`: Stack pointer (reserved)
  - `Rbp`: Base/frame pointer (optional)
  - `Rcx, Rdx, R8, R9`: Windows parameter registers
  - `Rdi, Rsi, Rdx, Rcx, R8, R9`: System V parameter registers

**Validation Rules**:
- All 16 variants are valid
- No duplicate register allocation (enforced by type system)
- Stack pointer (Rsp) should not be used for general computation

**Relationships**:
- Superset of `GPRegister32` (lower 32 bits)
- Superset of `GPRegister16` (lower 16 bits)
- Superset of `GPRegister8` (lower 8 bits)

---

**Entity**: `GPRegister32`, `GPRegister16`, `GPRegister8`

**Purpose**: Represents 32-bit, 16-bit, and 8-bit views of general-purpose registers

**Definitions**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPRegister32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp,
    R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPRegister16 {
    Ax, Bx, Cx, Dx, Si, Di, Bp, Sp,
    R8w, R9w, R10w, R11w, R12w, R13w, R14w, R15w,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPRegister8 {
    Al, Bl, Cl, Dl,
    Ah, Bh, Ch, Dh,
    Sil, Dil, Bpl, Spl,
    R8b, R9b, R10b, R11b, R12b, R13b, R14b, R15b,
}
```

**Register Aliasing Relationships**:
- `Rax` contains `Eax` (bits 0-31), `Ax` (bits 0-15), `Al` (bits 0-7), `Ah` (bits 8-15)
- Writes to 32-bit registers zero-extend to 64 bits
- Writes to 16-bit and 8-bit registers preserve upper bits

**Validation Rules**:
- High-byte registers (Ah, Bh, Ch, Dh) cannot be used with REX prefix in 64-bit mode
- Accessing partial registers requires awareness of aliasing semantics

---

### 2.2 SIMD Register Hierarchies

**Entity**: `XMMRegister` (128-bit SSE Registers)

**Purpose**: Represents SSE vector registers for floating-point and SIMD operations

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XMMRegister {
    Xmm0, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7,
    Xmm8, Xmm9, Xmm10, Xmm11, Xmm12, Xmm13, Xmm14, Xmm15,
}
```

**Properties**:
- **Size**: 128 bits / 16 bytes
- **Element Types**: Can contain scalar or packed floats/doubles
- **Volatility**: Platform-dependent
  - Windows: XMM0-XMM5 volatile, XMM6-XMM15 non-volatile (lower 128 bits only)
  - System V: XMM0-XMM15 all volatile

**Usage**:
- **Floating-Point Parameters**: Windows (XMM0-XMM3), System V (XMM0-XMM7)
- **Return Values**: XMM0 for scalar floats/doubles
- **SIMD Operations**: Packed arithmetic and logical operations

---

**Entity**: `YMMRegister` (256-bit AVX Registers)

**Purpose**: Represents AVX 256-bit vector registers

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YMMRegister {
    Ymm0, Ymm1, Ymm2, Ymm3, Ymm4, Ymm5, Ymm6, Ymm7,
    Ymm8, Ymm9, Ymm10, Ymm11, Ymm12, Ymm13, Ymm14, Ymm15,
}
```

**Properties**:
- **Size**: 256 bits / 32 bytes
- **Relationship**: YMM registers overlay XMM registers (lower 128 bits)
- **Volatility**: Upper 128 bits always volatile on both platforms

**Special Considerations**:
- VZEROUPPER instruction recommended before function calls to avoid performance penalty
- Parameter passing follows reference compiler conventions (match GCC/Clang/MSVC)

---

**Entity**: `ZMMRegister` (512-bit AVX-512 Registers)

**Purpose**: Represents AVX-512 512-bit vector registers

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZMMRegister {
    Zmm0, Zmm1, Zmm2, Zmm3, Zmm4, Zmm5, Zmm6, Zmm7,
    Zmm8, Zmm9, Zmm10, Zmm11, Zmm12, Zmm13, Zmm14, Zmm15,
    Zmm16, Zmm17, Zmm18, Zmm19, Zmm20, Zmm21, Zmm22, Zmm23,
    Zmm24, Zmm25, Zmm26, Zmm27, Zmm28, Zmm29, Zmm30, Zmm31,
}
```

**Properties**:
- **Size**: 512 bits / 64 bytes
- **Extended Set**: Zmm16-Zmm31 only available on AVX-512 hardware
- **Relationship**: ZMM overlays YMM overlays XMM (lower bits)
- **Volatility**: Upper 256 bits (256-511) always volatile

**Parameter Passing**: Follows reference compiler behavior (deferred to GCC/Clang/MSVC)

---

**Entity**: `MaskRegister` (AVX-512 Mask Registers)

**Purpose**: Represents AVX-512 predicate/mask registers for conditional operations

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaskRegister {
    K0, K1, K2, K3, K4, K5, K6, K7,
}
```

**Properties**:
- **Size**: 64 bits (one bit per element in ZMM operations)
- **Special Role**: K0 is hardwired to all-ones (no masking) in some contexts
- **Volatility**: Follows same rules as ZMM registers

---

### 2.3 Specialized Register Types

**Entity**: `FPURegister` (x87 FPU Registers)

**Purpose**: Represents legacy x87 floating-point stack registers

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FPURegister {
    St0, St1, St2, St3, St4, St5, St6, St7,
}
```

**Properties**:
- **Size**: 80 bits (extended precision)
- **Architecture**: Stack-based register file
- **Modern Usage**: Largely superseded by SSE/AVX for floating-point

**Validation Rules**:
- Stack depth limited to 8 registers
- Stack overflow/underflow must be managed

---

**Entity**: `MMXRegister` (MMX Registers)

**Purpose**: Represents legacy MMX SIMD registers

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MMXRegister {
    Mm0, Mm1, Mm2, Mm3, Mm4, Mm5, Mm6, Mm7,
}
```

**Properties**:
- **Size**: 64 bits
- **Architecture**: Overlays FPU register stack (ST0-ST7)
- **Modern Usage**: Deprecated in favor of SSE

**Validation Rules**:
- Cannot mix MMX and FPU operations without EMMS instruction
- Transitioning between modes incurs performance penalty

---

**Entity**: `SegmentRegister`

**Purpose**: Represents x86 segment registers (legacy and special-purpose)

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentRegister {
    Cs, Ds, Es, Fs, Gs, Ss,
}
```

**Properties**:
- **Size**: 16 bits (selector value)
- **64-bit Mode**:
  - CS, DS, ES, SS: Largely unused (flat memory model)
  - FS, GS: Used for thread-local storage and special OS structures

**Special Uses**:
- **Windows**: GS points to TEB (Thread Environment Block)
- **Linux**: FS points to thread-local storage

---

**Entity**: `ControlRegister`

**Purpose**: Represents system control registers

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlRegister {
    Cr0, Cr2, Cr3, Cr4, Cr8,
}
```

**Properties**:
- **Privilege Level**: Ring 0 (kernel mode) only
- **Special Roles**:
  - CR0: System control flags
  - CR2: Page fault linear address
  - CR3: Page directory base register
  - CR4: Extended feature enable flags
  - CR8: Task priority register (x64 only)

**Validation Rules**:
- Application-level code cannot access these registers
- Out of scope for typical jsavrs usage

---

**Entity**: `DebugRegister`

**Purpose**: Represents hardware debugging registers

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugRegister {
    Dr0, Dr1, Dr2, Dr3, Dr6, Dr7,
}
```

**Properties**:
- **Privilege Level**: Ring 0 (kernel mode) only
- **Usage**: Hardware breakpoints and watchpoints

**Validation Rules**:
- Application-level code cannot access these registers
- Out of scope for typical jsavrs usage

---

**Entity**: `FlagsRegister`

**Purpose**: Represents processor flags register

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlagsRegister {
    Rflags,  // 64-bit
    Eflags,  // 32-bit
    Flags,   // 16-bit
}
```

**Properties**:
- **Size**: 64 bits (Rflags), 32 bits (Eflags), 16 bits (Flags)
- **Content**: Status flags (ZF, SF, CF, OF, etc.) and control flags

**Validation Rules**:
- Cannot be directly written in most cases (use specific instructions)
- Some flags reserved and must be zero

---

**Entity**: `InstructionPointer`

**Purpose**: Represents program counter/instruction pointer register

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionPointer {
    Rip,  // 64-bit
    Eip,  // 32-bit
    Ip,   // 16-bit
}
```

**Properties**:
- **Size**: 64 bits (Rip in 64-bit mode)
- **Usage**: Automatically updated by CPU; used for RIP-relative addressing

**Validation Rules**:
- Cannot be directly written (use JMP, CALL, RET)
- RIP-relative addressing enables position-independent code

---

### 2.4 Unified Register Taxonomy

**Entity**: `X86Register`

**Purpose**: Unified enumeration grouping all register types for polymorphic handling

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum X86Register {
    GP64(GPRegister64),
    GP32(GPRegister32),
    GP16(GPRegister16),
    GP8(GPRegister8),
    Fpu(FPURegister),
    Mmx(MMXRegister),
    Xmm(XMMRegister),
    Ymm(YMMRegister),
    Zmm(ZMMRegister),
    Mask(MaskRegister),
    Segment(SegmentRegister),
    Control(ControlRegister),
    Debug(DebugRegister),
    Flags(FlagsRegister),
    InstructionPointer(InstructionPointer),
}
```

**Methods**:

```rust
impl X86Register {
    /// Returns size in bits
    pub fn size_bits(&self) -> usize;
    
    /// Returns size in bytes
    pub fn size_bytes(&self) -> usize;
    
    /// Checks if register is volatile (caller-saved) for given platform
    pub fn is_volatile(&self, platform: Platform) -> bool;
    
    /// Checks if register is non-volatile (callee-saved) for given platform
    pub fn is_callee_saved(&self, platform: Platform) -> bool;
    
    /// Checks if register can be used for Nth parameter on given platform
    pub fn is_parameter_register(&self, platform: Platform, index: usize) -> bool;
    
    /// Checks if register is used for return values on given platform
    pub fn is_return_register(&self, platform: Platform) -> bool;
    
    /// Returns NASM syntax name
    pub fn nasm_name(&self) -> String;
}
```

**Validation Rules**:
- Methods must handle all variants exhaustively
- Platform-specific behavior correctly implemented
- Size calculations match hardware specifications

---

## 3. Register Classification Entities

### 3.1 Volatility Classification

**Entity**: `RegisterVolatility`

**Purpose**: Conceptual classification of registers based on calling convention preservation rules

**Definition** (conceptual, not explicitly coded):
```rust
pub enum RegisterVolatility {
    Volatile,      // Caller-saved, may be clobbered by callee
    NonVolatile,   // Callee-saved, must be preserved by callee
    Special,       // Reserved (RSP, RBP)
}
```

**Classification Rules**:

**Windows x64 Volatility**:
- **Volatile GP**: RAX, RCX, RDX, R8, R9, R10, R11
- **Non-Volatile GP**: RBX, RBP, RDI, RSI, RSP, R12-R15
- **Volatile XMM**: XMM0-XMM5
- **Non-Volatile XMM**: XMM6-XMM15 (lower 128 bits only)

**System V Volatility**:
- **Volatile GP**: RAX, RCX, RDX, RSI, RDI, R8-R11
- **Non-Volatile GP**: RBX, RBP, RSP, R12-R15
- **Volatile XMM**: XMM0-XMM15 (all)

**Validation Rules**:
- Stack pointer (RSP) always preserved
- Frame pointer (RBP) typically preserved (platform-dependent)
- Return value registers (RAX, XMM0) inherently volatile

---

### 3.2 Parameter Register Mapping

**Entity**: `ParameterRegisterMapping`

**Purpose**: Maps parameter indices to specific registers for each platform

**Windows x64 Mapping**:
```
Integer Parameters:
  Index 0 → RCX
  Index 1 → RDX
  Index 2 → R8
  Index 3 → R9
  Index 4+ → Stack (with shadow space)

Floating-Point Parameters:
  Index 0 → XMM0
  Index 1 → XMM1
  Index 2 → XMM2
  Index 3 → XMM3
  Index 4+ → Stack

Note: Integer and FP parameters share the same index space
```

**System V Mapping**:
```
Integer Parameters:
  Index 0 → RDI
  Index 1 → RSI
  Index 2 → RDX
  Index 3 → RCX
  Index 4 → R8
  Index 5 → R9
  Index 6+ → Stack

Floating-Point Parameters:
  Index 0 → XMM0
  Index 1 → XMM1
  Index 2 → XMM2
  Index 3 → XMM3
  Index 4 → XMM4
  Index 5 → XMM5
  Index 6 → XMM6
  Index 7 → XMM7
  Index 8+ → Stack

Note: Integer and FP parameters have independent register allocations
```

**Implementation** (constant-time lookup tables):
```rust
const WINDOWS_INT_PARAMS: [GPRegister64; 4] = [
    GPRegister64::Rcx, GPRegister64::Rdx, GPRegister64::R8, GPRegister64::R9
];

const SYSTEMV_INT_PARAMS: [GPRegister64; 6] = [
    GPRegister64::Rdi, GPRegister64::Rsi, GPRegister64::Rdx,
    GPRegister64::Rcx, GPRegister64::R8, GPRegister64::R9
];

const WINDOWS_FP_PARAMS: [XMMRegister; 4] = [
    XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2, XMMRegister::Xmm3
];

const SYSTEMV_FP_PARAMS: [XMMRegister; 8] = [
    XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2, XMMRegister::Xmm3,
    XMMRegister::Xmm4, XMMRegister::Xmm5, XMMRegister::Xmm6, XMMRegister::Xmm7
];
```

**Validation Rules**:
- Out-of-bounds indices return `None` or indicate stack placement
- Mixed integer/FP parameter handling follows platform rules
- Windows: Overlapping indices (param N uses INT or FP, not both)
- System V: Independent indices for INT and FP

---

### 3.3 Return Value Register Classification

**Entity**: `ReturnRegisterMapping`

**Purpose**: Identifies registers used for function return values

**Classification**:
```
Scalar Integer Returns (all platforms):
  - Primary: RAX (64-bit, 32-bit, 16-bit, 8-bit)
  - Secondary: RDX (for 128-bit returns, e.g., returning pairs)

Scalar Floating-Point Returns (all platforms):
  - XMM0 (for float, double, vector types)

Structure Returns:
  - System V: RAX + XMM0 + XMM1 (if structure decomposed)
  - Windows: RAX (8 bytes or less), otherwise hidden pointer

Large Aggregate Returns:
  - Hidden pointer parameter (caller allocates, callee populates)
  - Pointer passed as first parameter (Windows) or in RDI (System V implicit)
```

**Validation Rules**:
- Return value size determines register vs. memory return
- ABI-specific thresholds:
  - Windows: > 8 bytes requires hidden pointer
  - System V: > 16 bytes requires hidden pointer (with exceptions for simple types)

---

## 4. Immediate Value Entities

**Entity**: `Immediate`

**Purpose**: Represents constant values embedded in instructions

**Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Immediate {
    Imm8(i8),      // 8-bit signed
    Imm8u(u8),     // 8-bit unsigned
    Imm16(i16),    // 16-bit signed
    Imm16u(u16),   // 16-bit unsigned
    Imm32(i32),    // 32-bit signed
    Imm32u(u32),   // 32-bit unsigned
    Imm64(i64),    // 64-bit signed
    Imm64u(u64),   // 64-bit unsigned
}
```

**Methods**:
```rust
impl Immediate {
    pub fn size_bits(&self) -> usize;
    pub fn size_bytes(&self) -> usize;
    pub fn as_i64(&self) -> i64;
    pub fn as_u64(&self) -> u64;
    pub fn is_signed(&self) -> bool;
    pub fn fits_in(&self, bits: usize) -> bool;
}
```

**Validation Rules**:
- Sign-extended vs. zero-extended based on signed/unsigned variant
- Conversion to 64-bit must preserve value semantics
- `fits_in()` checks if value can be represented in smaller encoding

**Usage Context**:
- Instruction operands for constants
- Immediate addressing modes
- Optimization: Smaller immediates preferred when possible

---

## 5. Memory Operand Entities

**Entity**: `MemoryOperand`

**Purpose**: Represents x86-64 memory addressing modes

**Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    pub base: Option<GPRegister64>,
    pub index: Option<GPRegister64>,
    pub scale: Scale,
    pub displacement: i32,
    pub size: OperandSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    One,
    Two,
    Four,
    Eight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandSize {
    Byte,
    Word,
    Dword,
    Qword,
    Xmmword,
    Ymmword,
    Zmmword,
}
```

**Addressing Mode Formula**:
```
Effective Address = base + (index * scale) + displacement
```

**Validation Rules**:
- **Base Register**: Any GPR64 except index (if index present)
- **Index Register**: Any GPR64 except RSP
- **Scale**: Must be 1, 2, 4, or 8
- **Displacement**: 32-bit signed offset
- **Size**: Must match instruction operand size

**Special Cases**:
- **RIP-Relative**: Base = RIP, Index = None, Displacement = offset
- **Absolute**: Base = None, Index = None, Displacement = address (rare in 64-bit)
- **SIB (Scale-Index-Base)**: Requires base + index + scale encoding

**Usage Context**:
- Load/store instructions (MOV, LEA, etc.)
- Accessing arrays, structures, local variables
- Position-independent code (RIP-relative)

---

## 6. Instruction Operand Entities

**Entity**: `Operand`

**Purpose**: Unified representation of instruction operands

**Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(X86Register),
    Immediate(Immediate),
    Memory(MemoryOperand),
    Label(String),
}
```

**Variants**:
- **Register**: Direct register operand
- **Immediate**: Constant value
- **Memory**: Memory address
- **Label**: Symbolic reference (resolved by assembler/linker)

**Validation Rules**:
- Instruction-specific operand type checking
- Size compatibility between operands
- Addressing mode restrictions

---

## 7. Instruction Entity

**Entity**: `Instruction`

**Purpose**: Represents x86-64 assembly instructions

**Definition** (partial, comprehensive set in instruction.rs):
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // Data Movement
    Mov { dest: Operand, src: Operand },
    Movsx { dest: Operand, src: Operand },
    Movzx { dest: Operand, src: Operand },
    Lea { dest: Operand, src: Operand },
    Push { src: Operand },
    Pop { dest: Operand },
    
    // Arithmetic
    Add { dest: Operand, src: Operand },
    Sub { dest: Operand, src: Operand },
    Imul { dest: Operand, src: Option<Operand>, imm: Option<Immediate> },
    Idiv { src: Operand },
    Inc { dest: Operand },
    Dec { dest: Operand },
    
    // Logical
    And { dest: Operand, src: Operand },
    Or { dest: Operand, src: Operand },
    Xor { dest: Operand, src: Operand },
    Not { dest: Operand },
    
    // Control Flow
    Jmp { target: Operand },
    Je { target: Operand },
    Jne { target: Operand },
    Call { target: Operand },
    Ret,
    
    // SSE/AVX
    Movaps { dest: Operand, src: Operand },
    Movapd { dest: Operand, src: Operand },
    Addss { dest: Operand, src: Operand },
    Addsd { dest: Operand, src: Operand },
    
    // ... (comprehensive set in implementation)
}
```

**Validation Rules**:
- Operand types must match instruction requirements
- Operand sizes must be compatible
- Memory-to-memory operations not allowed (except specific string ops)
- Register constraints (e.g., IMUL/IDIV use RAX/RDX implicitly)

---

## 8. Assembly Section Entities

**Entity**: `Section`

**Purpose**: Represents standard ELF/PE assembly sections

**Definition**:
```rust
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Section {
    Text,    // Executable code
    Data,    // Initialized data
    Bss,     // Uninitialized data
    Rodata,  // Read-only data
}
```

**Properties**:
- **Text**: Contains executable instructions, read-only + executable
- **Data**: Contains initialized global/static variables, read-write
- **Bss**: Contains zero-initialized variables, read-write (no disk space)
- **Rodata**: Contains constants, read-only

**Methods**:
```rust
impl Section {
    pub fn name(&self) -> &'static str;  // Returns ".text", ".data", etc.
    pub fn is_text(&self) -> bool;
    pub fn is_data(&self) -> bool;
    pub fn is_bss(&self) -> bool;
    pub fn is_rodata(&self) -> bool;
}
```

---

**Entity**: `DataDirective`

**Purpose**: Represents assembly data declaration directives

**Definition**:
```rust
#[derive(Debug, Clone)]
pub enum DataDirective {
    Db(Vec<u8>),       // Define Byte
    Dw(Vec<u16>),      // Define Word
    Dd(Vec<u32>),      // Define Dword
    Dq(Vec<u64>),      // Define Qword
    Asciz(String),     // ASCII string with null terminator
    Ascii(String),     // ASCII string without null terminator
    Resb(usize),       // Reserve Bytes
    Resw(usize),       // Reserve Words
    Resd(usize),       // Reserve Dwords
    Resq(usize),       // Reserve Qwords
}
```

**Usage Context**:
- Data section: Initialized global variables
- Rodata section: String literals, constant data
- BSS section: Reserved space (Resb/Resw/Resd/Resq)

---

**Entity**: `AssemblyElement`

**Purpose**: Represents individual elements within an assembly section

**Definition**:
```rust
#[derive(Debug, Clone)]
pub enum AssemblyElement {
    Label(String),
    Instruction(Instruction),
    Data(String, DataDirective),
    Comment(String),
    EmptyLine,
}
```

---

**Entity**: `AssemblySection`

**Purpose**: Container for assembly elements within a specific section

**Definition**:
```rust
#[derive(Debug, Clone)]
pub struct AssemblySection {
    pub section: Section,
    pub elements: Vec<AssemblyElement>,
}

impl AssemblySection {
    pub fn new(section: Section) -> Self;
    pub fn add_label(&mut self, label: impl Into<String>);
    pub fn add_instruction(&mut self, instr: Instruction);
    pub fn add_data(&mut self, label: impl Into<String>, directive: DataDirective);
    pub fn add_comment(&mut self, comment: impl Into<String>);
    pub fn add_empty_line(&mut self);
}
```

**Validation Rules**:
- Instructions only valid in Text section
- Data directives only valid in Data/Rodata sections
- Reserved space only valid in BSS section
- Labels valid in any section

---

## 9. ABI Trait Contracts (Conceptual Entities)

### 9.1 CallingConvention Trait

**Entity**: `CallingConvention` (trait)

**Purpose**: Defines interface for platform-specific calling convention queries

**Definition**:
```rust
pub trait CallingConvention {
    /// Returns the platform this calling convention targets
    fn platform() -> Platform;
    
    /// Returns the ABI variant
    fn abi() -> Abi;
    
    /// Gets the register for the Nth integer parameter (None if stack)
    fn integer_param_register(index: usize) -> Option<GPRegister64>;
    
    /// Gets the register for the Nth floating-point parameter (None if stack)
    fn float_param_register(index: usize) -> Option<XMMRegister>;
    
    /// Maximum number of integer parameters in registers
    fn max_integer_register_params() -> usize;
    
    /// Maximum number of floating-point parameters in registers
    fn max_float_register_params() -> usize;
    
    /// Returns true if integer and FP parameter indices overlap
    fn params_share_index_space() -> bool;
}
```

**Implementations**:
- `WindowsX64CallingConvention`
- `SystemVCallingConvention`

---

### 9.2 StackManagement Trait

**Entity**: `StackManagement` (trait)

**Purpose**: Defines interface for stack layout and management queries

**Definition**:
```rust
pub trait StackManagement {
    /// Returns true if red zone is available
    fn has_red_zone() -> bool;
    
    /// Returns red zone size in bytes (0 if unavailable)
    fn red_zone_size_bytes() -> usize;
    
    /// Returns minimum stack alignment in bytes
    fn min_stack_alignment() -> usize;
    
    /// Returns true if shadow space required
    fn requires_shadow_space() -> bool;
    
    /// Returns shadow space size in bytes
    fn shadow_space_bytes() -> usize;
    
    /// Returns true if frame pointer required
    fn requires_frame_pointer() -> bool;
}
```

**Implementations**:
- `WindowsStackManagement`: shadow_space = 32, no red zone
- `SystemVStackManagement`: red_zone = 128, no shadow space

---

### 9.3 RegisterAllocation Trait

**Entity**: `RegisterAllocation` (trait)

**Purpose**: Provides guidance for efficient register allocation

**Definition**:
```rust
pub trait RegisterAllocation {
    /// Returns priority-ordered list of volatile GP registers for temporaries
    fn volatile_gp_registers() -> &'static [GPRegister64];
    
    /// Returns priority-ordered list of non-volatile GP registers
    fn non_volatile_gp_registers() -> &'static [GPRegister64];
    
    /// Returns priority-ordered list of volatile XMM registers
    fn volatile_xmm_registers() -> &'static [XMMRegister];
    
    /// Checks if register is volatile for this convention
    fn is_volatile(reg: X86Register) -> bool;
    
    /// Checks if register is callee-saved for this convention
    fn is_callee_saved(reg: X86Register) -> bool;
}
```

---

### 9.4 AggregateClassification Trait

**Entity**: `AggregateClassification` (trait)

**Purpose**: Classifies structure/union parameter passing

**Definition**:
```rust
pub trait AggregateClassification {
    /// Classifies how an aggregate type should be passed
    fn classify_aggregate(size: usize, fields: &[FieldType]) -> AggregateClass;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateClass {
    ByValue,              // Pass in register(s)
    ByReference,          // Pass pointer
    Decomposed(Vec<X86Register>),  // Decompose into multiple registers
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Integer,
    Float,
    Pointer,
}
```

---

### 9.5 VariadicConvention Trait

**Entity**: `VariadicConvention` (trait)

**Purpose**: Handles variadic function-specific ABI rules

**Definition**:
```rust
pub trait VariadicConvention {
    /// Returns true if AL register contains FP parameter count
    fn requires_al_register() -> bool;
    
    /// Returns true if register save area required
    fn requires_register_save_area() -> bool;
    
    /// Gets location for Nth variadic parameter
    fn variadic_param_location(index: usize) -> ParamLocation;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamLocation {
    Register(X86Register),
    Stack(i32),  // Offset from stack pointer
    RegisterSaveArea(usize),  // Index in save area
}
```

---

## 10. Entity Relationships Diagram

```
Platform
  │
  ├──> Abi (via from_platform)
  │
  └──> CallingConvention (implementation selection)
         │
         ├──> StackManagement
         ├──> RegisterAllocation
         ├──> AggregateClassification
         └──> VariadicConvention

X86Register
  │
  ├──> GPRegister64/32/16/8
  ├──> XMMRegister
  ├──> YMMRegister
  ├──> ZMMRegister
  ├──> FPURegister
  ├──> MMXRegister
  ├──> MaskRegister
  ├──> SegmentRegister
  ├──> ControlRegister
  ├──> DebugRegister
  ├──> FlagsRegister
  └──> InstructionPointer

Operand
  │
  ├──> Register (X86Register)
  ├──> Immediate
  ├──> Memory (MemoryOperand)
  └──> Label (String)

Instruction
  └──> uses Operand(s)

AssemblySection
  ├──> Section (Text/Data/Bss/Rodata)
  └──> Vec<AssemblyElement>
         │
         ├──> Label
         ├──> Instruction
         ├──> Data (DataDirective)
         ├──> Comment
         └──> EmptyLine
```

---

## 11. Validation Rules Summary

### Type System Enforcement
- All enums are exhaustive (no invalid variants)
- Register aliasing relationships enforced through separate types
- Platform-specific behavior selected via trait implementations
- Compile-time validation prevents invalid register-platform combinations

### Runtime Validation (Minimal)
- Operand size compatibility checks in instruction construction
- Memory operand constraint validation (e.g., RSP cannot be index)
- Stack alignment verification (debug builds)

### Performance Guarantees
- All ABI queries: O(1) constant-time through array lookups
- No heap allocations for core entities
- Inline-eligible methods for zero-cost abstraction
- Static dispatch through trait implementations

---

## 12. State Transition Rules

### Immutable Entities
- `Platform`: Selected once at compilation start
- `Abi`: Derived from platform, never changes
- All register enums: Represent hardware state, immutable
- `Section`: Section classification is static

### Mutable Entities
- `AssemblySection`: Accumulates elements during code generation
- `Instruction` operands: May be transformed during optimization

### State Lifecycle
1. **Initialization**: Platform selection → ABI derivation → Trait implementation selection
2. **Query Phase**: Constant-time ABI queries for code generation decisions
3. **Generation Phase**: Assembly sections populated with instructions/data
4. **Output Phase**: Sections serialized to NASM syntax

---

## 13. Conclusion

This data model provides a comprehensive, detailed, precise, and meticulous specification of all entities required for the x86-64 ABI trait system. Every entity is fully documented with fields, methods, validation rules, relationships, and usage contexts.

**Key Design Principles Satisfied**:
- ✅ Type safety through Rust's enum and trait system
- ✅ Performance through constant-time lookups and zero-cost abstractions
- ✅ Extensibility through trait-based architecture
- ✅ Clarity through comprehensive documentation
- ✅ Correctness through validation rules and exhaustive matching

**Readiness for Implementation**:
All entities are sufficiently specified to begin implementation with confidence in correctness and adherence to ABI specifications.

---

**Document Version**: 1.0  
**Last Updated**: October 2, 2025  
**Status**: Phase 1 Complete
