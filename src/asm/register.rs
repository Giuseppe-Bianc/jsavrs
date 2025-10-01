/// Piattaforma target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
}

/// Registri General Purpose a 64-bit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister64 {
    RAX, RBX, RCX, RDX,
    RSI, RDI, RBP, RSP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

/// Registri General Purpose a 32-bit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister32 {
    EAX, EBX, ECX, EDX,
    ESI, EDI, EBP, ESP,
    R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D,
}

/// Registri General Purpose a 16-bit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister16 {
    AX, BX, CX, DX,
    SI, DI, BP, SP,
    R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W,
}

/// Registri General Purpose a 8-bit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPRegister8 {
    AL, BL, CL, DL,
    AH, BH, CH, DH,
    SIL, DIL, BPL, SPL,
    R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B,
}

/// Registri x87 FPU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FPURegister {
    ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7,
}

/// Registri MMX
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MMXRegister {
    MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7,
}

/// Registri XMM (SSE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XMMRegister {
    XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7,
    XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15,
}

/// Registri YMM (AVX)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YMMRegister {
    YMM0, YMM1, YMM2, YMM3, YMM4, YMM5, YMM6, YMM7,
    YMM8, YMM9, YMM10, YMM11, YMM12, YMM13, YMM14, YMM15,
}

/// Registri ZMM (AVX-512)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZMMRegister {
    ZMM0, ZMM1, ZMM2, ZMM3, ZMM4, ZMM5, ZMM6, ZMM7,
    ZMM8, ZMM9, ZMM10, ZMM11, ZMM12, ZMM13, ZMM14, ZMM15,
    ZMM16, ZMM17, ZMM18, ZMM19, ZMM20, ZMM21, ZMM22, ZMM23,
    ZMM24, ZMM25, ZMM26, ZMM27, ZMM28, ZMM29, ZMM30, ZMM31,
}

/// Registri Mask (AVX-512)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskRegister {
    K0, K1, K2, K3, K4, K5, K6, K7,
}

/// Registri di segmento
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentRegister {
    CS, DS, ES, FS, GS, SS,
}

/// Registri di controllo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlRegister {
    CR0, CR2, CR3, CR4, CR8,
}

/// Registri di debug
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugRegister {
    DR0, DR1, DR2, DR3, DR6, DR7,
}

/// Registro dei flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagsRegister {
    RFLAGS,  // 64-bit
    EFLAGS,  // 32-bit
    FLAGS,   // 16-bit
}

/// Registro instruction pointer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionPointer {
    RIP,  // 64-bit
    EIP,  // 32-bit
    IP,   // 16-bit
}

/// Enumerazione principale che raggruppa tutti i tipi di registri
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Register {
    GP64(GPRegister64),
    GP32(GPRegister32),
    GP16(GPRegister16),
    GP8(GPRegister8),
    FPU(FPURegister),
    MMX(MMXRegister),
    XMM(XMMRegister),
    YMM(YMMRegister),
    ZMM(ZMMRegister),
    Mask(MaskRegister),
    Segment(SegmentRegister),
    Control(ControlRegister),
    Debug(DebugRegister),
    Flags(FlagsRegister),
    InstructionPointer(InstructionPointer),
}

impl X86Register {
    /// Ottiene la denominazione NASM del registro
    pub fn nasm_name(&self) -> String {
        match self {
            X86Register::GP64(r) => format!("{:?}", r).to_lowercase(),
            X86Register::GP32(r) => format!("{:?}", r).to_lowercase(),
            X86Register::GP16(r) => format!("{:?}", r).to_lowercase(),
            X86Register::GP8(r) => format!("{:?}", r).to_lowercase(),
            X86Register::FPU(r) => {
                let idx = match r {
                    FPURegister::ST0 => 0, FPURegister::ST1 => 1,
                    FPURegister::ST2 => 2, FPURegister::ST3 => 3,
                    FPURegister::ST4 => 4, FPURegister::ST5 => 5,
                    FPURegister::ST6 => 6, FPURegister::ST7 => 7,
                };
                format!("st{}", idx)
            }
            X86Register::MMX(r) => format!("{:?}", r).to_lowercase(),
            X86Register::XMM(r) => format!("{:?}", r).to_lowercase(),
            X86Register::YMM(r) => format!("{:?}", r).to_lowercase(),
            X86Register::ZMM(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Mask(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Segment(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Control(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Debug(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Flags(r) => format!("{:?}", r).to_lowercase(),
            X86Register::InstructionPointer(r) => format!("{:?}", r).to_lowercase(),
        }
    }

    /// Ottiene una descrizione del registro
    pub fn description(&self) -> &'static str {
        match self {
            X86Register::GP64(_) => "Registro general-purpose a 64-bit",
            X86Register::GP32(_) => "Registro general-purpose a 32-bit",
            X86Register::GP16(_) => "Registro general-purpose a 16-bit",
            X86Register::GP8(_) => "Registro general-purpose a 8-bit",
            X86Register::FPU(_) => "Registro x87 FPU",
            X86Register::MMX(_) => "Registro MMX",
            X86Register::XMM(_) => "Registro XMM (SSE)",
            X86Register::YMM(_) => "Registro YMM (AVX)",
            X86Register::ZMM(_) => "Registro ZMM (AVX-512)",
            X86Register::Mask(_) => "Registro maschera (AVX-512)",
            X86Register::Segment(_) => "Registro di segmento",
            X86Register::Control(_) => "Registro di controllo",
            X86Register::Debug(_) => "Registro di debug",
            X86Register::Flags(_) => "Registro dei flag",
            X86Register::InstructionPointer(_) => "Instruction pointer",
        }
    }

    /// Verifica se il registro è volatile secondo la calling convention
    pub fn is_volatile(&self, platform: Platform) -> bool {
        match self {
            X86Register::GP64(r) => match platform {
                Platform::Windows => matches!(r, 
                    GPRegister64::RAX | GPRegister64::RCX | 
                    GPRegister64::RDX | GPRegister64::R8 |
                    GPRegister64::R9 | GPRegister64::R10 | GPRegister64::R11
                ),
                Platform::Linux | Platform::MacOS => matches!(r,
                    GPRegister64::RAX | GPRegister64::RCX | GPRegister64::RDX |
                    GPRegister64::RSI | GPRegister64::RDI |
                    GPRegister64::R8 | GPRegister64::R9 | GPRegister64::R10 | GPRegister64::R11
                ),
            },
            X86Register::XMM(r) => match platform {
                Platform::Windows => matches!(r,
                    XMMRegister::XMM0 | XMMRegister::XMM1 | XMMRegister::XMM2 |
                    XMMRegister::XMM3 | XMMRegister::XMM4 | XMMRegister::XMM5
                ),
                Platform::Linux | Platform::MacOS => true, // Tutti volatili in System V
            },
            X86Register::YMM(r) => match platform {
                Platform::Windows => matches!(r,
                    YMMRegister::YMM0 | YMMRegister::YMM1 | YMMRegister::YMM2 |
                    YMMRegister::YMM3 | YMMRegister::YMM4 | YMMRegister::YMM5
                ),
                Platform::Linux | Platform::MacOS => true,
            },
            _ => false,
        }
    }

    /// Verifica se il registro è non-volatile (callee-saved)
    pub fn is_callee_saved(&self, platform: Platform) -> bool {
        match self {
            X86Register::GP64(r) => match platform {
                Platform::Windows => matches!(r,
                    GPRegister64::RBX | GPRegister64::RBP | GPRegister64::RDI |
                    GPRegister64::RSI | GPRegister64::RSP |
                    GPRegister64::R12 | GPRegister64::R13 | GPRegister64::R14 | GPRegister64::R15
                ),
                Platform::Linux | Platform::MacOS => matches!(r,
                    GPRegister64::RBX | GPRegister64::RBP | GPRegister64::RSP |
                    GPRegister64::R12 | GPRegister64::R13 | GPRegister64::R14 | GPRegister64::R15
                ),
            },
            X86Register::XMM(r) => match platform {
                Platform::Windows => matches!(r,
                    XMMRegister::XMM6 | XMMRegister::XMM7 | XMMRegister::XMM8 |
                    XMMRegister::XMM9 | XMMRegister::XMM10 | XMMRegister::XMM11 |
                    XMMRegister::XMM12 | XMMRegister::XMM13 | XMMRegister::XMM14 | XMMRegister::XMM15
                ),
                Platform::Linux | Platform::MacOS => false,
            },
            _ => false,
        }
    }

    /// Ottiene la dimensione del registro in bit
    pub fn size_bits(&self) -> usize {
        match self {
            X86Register::GP64(_) | X86Register::Flags(FlagsRegister::RFLAGS) 
            | X86Register::InstructionPointer(InstructionPointer::RIP) => 64,
            X86Register::GP32(_) | X86Register::Flags(FlagsRegister::EFLAGS)
            | X86Register::InstructionPointer(InstructionPointer::EIP) => 32,
            X86Register::GP16(_) | X86Register::Flags(FlagsRegister::FLAGS)
            | X86Register::InstructionPointer(InstructionPointer::IP)
            | X86Register::Segment(_) => 16,
            X86Register::GP8(_) => 8,
            X86Register::FPU(_) => 80,
            X86Register::MMX(_) => 64,
            X86Register::XMM(_) => 128,
            X86Register::YMM(_) => 256,
            X86Register::ZMM(_) => 512,
            X86Register::Mask(_) => 64,
            X86Register::Control(_) | X86Register::Debug(_) => 64,
        }
    }

    /// Ottiene la dimensione del registro in byte
    pub fn size_bytes(&self) -> usize {
        self.size_bits() / 8
    }

    /// Verifica se il registro può essere usato per passaggio parametri
    pub fn is_parameter_register(&self, platform: Platform, param_index: usize) -> bool {
        match platform {
            Platform::Windows => {
                // Windows x64 calling convention
                match self {
                    X86Register::GP64(r) => matches!(
                        (r, param_index),
                        (GPRegister64::RCX, 0) | (GPRegister64::RDX, 1) |
                        (GPRegister64::R8, 2) | (GPRegister64::R9, 3)
                    ),
                    X86Register::XMM(r) => matches!(
                        (r, param_index),
                        (XMMRegister::XMM0, 0) | (XMMRegister::XMM1, 1) |
                        (XMMRegister::XMM2, 2) | (XMMRegister::XMM3, 3)
                    ),
                    _ => false,
                }
            }
            Platform::Linux | Platform::MacOS => {
                // System V AMD64 ABI
                match self {
                    X86Register::GP64(r) => matches!(
                        (r, param_index),
                        (GPRegister64::RDI, 0) | (GPRegister64::RSI, 1) |
                        (GPRegister64::RDX, 2) | (GPRegister64::RCX, 3) |
                        (GPRegister64::R8, 4) | (GPRegister64::R9, 5)
                    ),
                    X86Register::XMM(r) => param_index < 8 && matches!(
                        (r, param_index),
                        (XMMRegister::XMM0, 0) | (XMMRegister::XMM1, 1) |
                        (XMMRegister::XMM2, 2) | (XMMRegister::XMM3, 3) |
                        (XMMRegister::XMM4, 4) | (XMMRegister::XMM5, 5) |
                        (XMMRegister::XMM6, 6) | (XMMRegister::XMM7, 7)
                    ),
                    _ => false,
                }
            }
        }
    }

    /// Verifica se il registro viene usato per il valore di ritorno
    pub fn is_return_register(&self, platform: Platform) -> bool {
        match platform {
            Platform::Windows | Platform::Linux | Platform::MacOS => {
                matches!(
                    self,
                    X86Register::GP64(GPRegister64::RAX) |
                    X86Register::GP64(GPRegister64::RDX) | // Per valori a 128-bit
                    X86Register::XMM(XMMRegister::XMM0) |
                    X86Register::XMM(XMMRegister::XMM1)    // System V per struct
                )
            }
        }
    }
}

// Implementazione del trait Display per tutti i tipi di registri
impl std::fmt::Display for GPRegister64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for GPRegister32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for GPRegister16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for GPRegister8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for FPURegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = match self {
            FPURegister::ST0 => 0, FPURegister::ST1 => 1,
            FPURegister::ST2 => 2, FPURegister::ST3 => 3,
            FPURegister::ST4 => 4, FPURegister::ST5 => 5,
            FPURegister::ST6 => 6, FPURegister::ST7 => 7,
        };
        write!(f, "st{}", idx)
    }
}

impl std::fmt::Display for MMXRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for XMMRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for YMMRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for ZMMRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for MaskRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for SegmentRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for ControlRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for DebugRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for FlagsRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for InstructionPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl std::fmt::Display for X86Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nasm_name())
    }
}

/*
// Esempio di utilizzo
#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[test]
    fn test_nasm_names() {
        let rax = X86Register::GP64(GPRegister64::RAX);
        assert_eq!(rax.nasm_name(), "rax");
        
        let xmm0 = X86Register::XMM(XMMRegister::XMM0);
        assert_eq!(xmm0.nasm_name(), "xmm0");
        
        let st0 = X86Register::FPU(FPURegister::ST0);
        assert_eq!(st0.nasm_name(), "st0");
        
        let ymm5 = X86Register::YMM(YMMRegister::YMM5);
        assert_eq!(ymm5.nasm_name(), "ymm5");
    }

    #[test]
    fn test_volatility() {
        let rax = X86Register::GP64(GPRegister64::RAX);
        assert!(rax.is_volatile(Platform::Windows));
        assert!(rax.is_volatile(Platform::Linux));
        
        let rbx = X86Register::GP64(GPRegister64::RBX);
        assert!(!rbx.is_volatile(Platform::Windows));
        assert!(!rbx.is_volatile(Platform::Linux));
        assert!(rbx.is_callee_saved(Platform::Windows));
        assert!(rbx.is_callee_saved(Platform::Linux));
        
        // RSI è volatile su Linux/macOS ma non-volatile su Windows
        let rsi = X86Register::GP64(GPRegister64::RSI);
        assert!(!rsi.is_volatile(Platform::Windows));
        assert!(rsi.is_volatile(Platform::Linux));
    }

    #[test]
    fn test_register_sizes() {
        assert_eq!(X86Register::GP64(GPRegister64::RAX).size_bits(), 64);
        assert_eq!(X86Register::GP64(GPRegister64::RAX).size_bytes(), 8);
        assert_eq!(X86Register::GP32(GPRegister32::EAX).size_bits(), 32);
        assert_eq!(X86Register::GP16(GPRegister16::AX).size_bits(), 16);
        assert_eq!(X86Register::GP8(GPRegister8::AL).size_bits(), 8);
        assert_eq!(X86Register::XMM(XMMRegister::XMM0).size_bits(), 128);
        assert_eq!(X86Register::YMM(YMMRegister::YMM0).size_bits(), 256);
        assert_eq!(X86Register::ZMM(ZMMRegister::ZMM0).size_bits(), 512);
    }

    #[test]
    fn test_parameter_registers() {
        // Windows calling convention
        let rcx = X86Register::GP64(GPRegister64::RCX);
        assert!(rcx.is_parameter_register(Platform::Windows, 0));
        assert!(!rcx.is_parameter_register(Platform::Linux, 0));
        
        // System V calling convention
        let rdi = X86Register::GP64(GPRegister64::RDI);
        assert!(rdi.is_parameter_register(Platform::Linux, 0));
        assert!(!rdi.is_parameter_register(Platform::Windows, 0));
        
        let xmm0 = X86Register::XMM(XMMRegister::XMM0);
        assert!(xmm0.is_parameter_register(Platform::Windows, 0));
        assert!(xmm0.is_parameter_register(Platform::Linux, 0));
    }

    #[test]
    fn test_return_registers() {
        let rax = X86Register::GP64(GPRegister64::RAX);
        assert!(rax.is_return_register(Platform::Windows));
        assert!(rax.is_return_register(Platform::Linux));
        
        let xmm0 = X86Register::XMM(XMMRegister::XMM0);
        assert!(xmm0.is_return_register(Platform::Windows));
        assert!(xmm0.is_return_register(Platform::Linux));
    }

    #[test]
    fn test_display_trait() {
        // Test Display per X86Register
        let rax = X86Register::GP64(GPRegister64::RAX);
        assert_eq!(format!("{}", rax), "rax");
        
        let xmm0 = X86Register::XMM(XMMRegister::XMM0);
        assert_eq!(format!("{}", xmm0), "xmm0");
        
        let st0 = X86Register::FPU(FPURegister::ST0);
        assert_eq!(format!("{}", st0), "st0");
        
        let ymm15 = X86Register::YMM(YMMRegister::YMM15);
        assert_eq!(format!("{}", ymm15), "ymm15");
        
        let k3 = X86Register::Mask(MaskRegister::K3);
        assert_eq!(format!("{}", k3), "k3");
        
        // Test Display per sottotipi
        assert_eq!(format!("{}", GPRegister64::R15), "r15");
        assert_eq!(format!("{}", GPRegister8::SIL), "sil");
        assert_eq!(format!("{}", SegmentRegister::FS), "fs");
        assert_eq!(format!("{}", ControlRegister::CR3), "cr3");
        assert_eq!(format!("{}", FlagsRegister::RFLAGS), "rflags");
    }
}*/