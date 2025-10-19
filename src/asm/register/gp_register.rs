/// General-purpose 64-bit registers.
///
/// Primary registers for integer arithmetic, addressing, and data manipulation.
/// Conventional uses: RAX (accumulator/return), RCX (counter), RDX (data),
/// RSI/RDI (string operations), RBP (base pointer), RSP (stack pointer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister64 {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

/// General-purpose 32-bit registers.
///
/// Lower 32 bits of 64-bit GP registers. Writing to 32-bit registers
/// zero-extends to 64 bits (upper 32 bits cleared). This eliminates
/// partial register stalls in 64-bit mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister32 {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

/// General-purpose 16-bit registers.
///
/// Lower 16 bits of GP registers. Writing does NOT zero-extend; upper bits
/// remain unchanged. May cause partial register dependencies on some CPUs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister16 {
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Bp,
    Sp,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,
}

/// General-purpose 8-bit registers.
///
/// AL/BL/CL/DL: Low byte of AX/BX/CX/DX.
/// AH/BH/CH/DH: High byte (bits 8-15), incompatible with REX prefix.
/// SIL/DIL/BPL/SPL: Low byte of SI/DI/BP/SP, require REX prefix.
/// R8B-R15B: Low byte of R8-R15.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister8 {
    Al,
    Bl,
    Cl,
    Dl,
    Ah,
    Bh,
    Ch,
    Dh,
    Sil,
    Dil,
    Bpl,
    Spl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}
