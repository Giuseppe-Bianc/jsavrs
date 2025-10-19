/// Registri General Purpose a 64-bit
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

/// Registri General Purpose a 32-bit
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

/// Registri General Purpose a 16-bit
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

/// Registri General Purpose a 8-bit
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
