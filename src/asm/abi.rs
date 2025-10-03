use super::register::{GPRegister64, XMMRegister, X86Register};
use super::Platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abi {
    SystemV,
    Windows,
}

#[allow(dead_code)]
impl Abi {
    pub fn from_platform(platform: Platform) -> Self {
        match platform {
            Platform::Windows => Abi::Windows,
            _ => Abi::SystemV,
        }
    }

    /// Returns the required stack alignment in bytes.
    pub fn alignment(&self) -> u32 {
        16
    }


    /// Returns the size of the red zone in bytes.
    /// The red zone is an optimization where leaf functions can use 
    /// stack space below RSP without adjusting the stack pointer.
    pub fn red_zone(&self) -> u32 {
        match self {
            Abi::SystemV => 128,
            Abi::Windows => 0,
        }
    }

    /// Returns the size of the shadow space in bytes.
    /// Shadow space is stack space that must be allocated by the caller
    /// for the callee to spill register parameters if needed.
    pub fn shadow_space(&self) -> u32 {
        match self {
            Abi::SystemV => 0,
            Abi::Windows => 32, // 4 registers × 8 bytes
        }
    }

    /// Returns the integer parameter registers in order.
    pub fn int_param_registers(&self) -> &'static [GPRegister64] {
        match self {
            Abi::SystemV => &[
                GPRegister64::Rdi,
                GPRegister64::Rsi,
                GPRegister64::Rdx,
                GPRegister64::Rcx,
                GPRegister64::R8,
                GPRegister64::R9,
            ],
            Abi::Windows => &[
                GPRegister64::Rcx,
                GPRegister64::Rdx,
                GPRegister64::R8,
                GPRegister64::R9,
            ],
        }
    }

    /// Returns the floating-point parameter registers in order.
    pub fn float_param_registers(&self) -> &'static [XMMRegister] {
        match self {
            Abi::SystemV => &[
                XMMRegister::Xmm0,
                XMMRegister::Xmm1,
                XMMRegister::Xmm2,
                XMMRegister::Xmm3,
                XMMRegister::Xmm4,
                XMMRegister::Xmm5,
                XMMRegister::Xmm6,
                XMMRegister::Xmm7,
            ],
            Abi::Windows => &[
                XMMRegister::Xmm0,
                XMMRegister::Xmm1,
                XMMRegister::Xmm2,
                XMMRegister::Xmm3,
            ],
        }
    }

    /// Returns the integer return value registers.
    /// First register is primary, second is used for 128-bit returns.
    pub fn int_return_registers(&self) -> &'static [GPRegister64] {
        &[GPRegister64::Rax, GPRegister64::Rdx]
    }

    /// Returns the floating-point return value registers.
    pub fn float_return_registers(&self) -> &'static [XMMRegister] {
        match self {
            Abi::SystemV => &[XMMRegister::Xmm0, XMMRegister::Xmm1],
            Abi::Windows => &[XMMRegister::Xmm0],
        }
    }

    /// Returns the callee-saved (non-volatile) general purpose registers.
    /// These must be preserved across function calls.
    pub fn callee_saved_gp_registers(&self) -> &'static [GPRegister64] {
        match self {
            Abi::SystemV => &[
                GPRegister64::Rbx,
                GPRegister64::Rbp,
                GPRegister64::R12,
                GPRegister64::R13,
                GPRegister64::R14,
                GPRegister64::R15,
            ],
            Abi::Windows => &[
                GPRegister64::Rbx,
                GPRegister64::Rbp,
                GPRegister64::Rdi,
                GPRegister64::Rsi,
                GPRegister64::R12,
                GPRegister64::R13,
                GPRegister64::R14,
                GPRegister64::R15,
            ],
        }
    }

    /// Returns the callee-saved (non-volatile) XMM registers.
    pub fn callee_saved_xmm_registers(&self) -> &'static [XMMRegister] {
        match self {
            Abi::SystemV => &[], // All XMM registers are caller-saved
            Abi::Windows => &[
                XMMRegister::Xmm6,
                XMMRegister::Xmm7,
                XMMRegister::Xmm8,
                XMMRegister::Xmm9,
                XMMRegister::Xmm10,
                XMMRegister::Xmm11,
                XMMRegister::Xmm12,
                XMMRegister::Xmm13,
                XMMRegister::Xmm14,
                XMMRegister::Xmm15,
            ],
        }
    }

    /// Returns the caller-saved (volatile) general purpose registers.
    pub fn caller_saved_gp_registers(&self) -> &'static [GPRegister64] {
        match self {
            Abi::SystemV => &[
                GPRegister64::Rax,
                GPRegister64::Rcx,
                GPRegister64::Rdx,
                GPRegister64::Rsi,
                GPRegister64::Rdi,
                GPRegister64::R8,
                GPRegister64::R9,
                GPRegister64::R10,
                GPRegister64::R11,
            ],
            Abi::Windows => &[
                GPRegister64::Rax,
                GPRegister64::Rcx,
                GPRegister64::Rdx,
                GPRegister64::R8,
                GPRegister64::R9,
                GPRegister64::R10,
                GPRegister64::R11,
            ],
        }
    }

    /// Returns the caller-saved (volatile) XMM registers.
    pub fn caller_saved_xmm_registers(&self) -> &'static [XMMRegister] {
        match self {
            Abi::SystemV => &[
                XMMRegister::Xmm0,
                XMMRegister::Xmm1,
                XMMRegister::Xmm2,
                XMMRegister::Xmm3,
                XMMRegister::Xmm4,
                XMMRegister::Xmm5,
                XMMRegister::Xmm6,
                XMMRegister::Xmm7,
                XMMRegister::Xmm8,
                XMMRegister::Xmm9,
                XMMRegister::Xmm10,
                XMMRegister::Xmm11,
                XMMRegister::Xmm12,
                XMMRegister::Xmm13,
                XMMRegister::Xmm14,
                XMMRegister::Xmm15,
            ],
            Abi::Windows => &[
                XMMRegister::Xmm0,
                XMMRegister::Xmm1,
                XMMRegister::Xmm2,
                XMMRegister::Xmm3,
                XMMRegister::Xmm4,
                XMMRegister::Xmm5,
            ],
        }
    }

    /// Checks if a register is callee-saved (non-volatile).
    pub fn is_callee_saved(&self, reg: X86Register) -> bool {
        match reg {
            X86Register::GP64(gp) => self.callee_saved_gp_registers().contains(&gp),
            X86Register::Xmm(xmm) => self.callee_saved_xmm_registers().contains(&xmm),
            _ => false,
        }
    }

    /// Checks if a register is caller-saved (volatile).
    pub fn is_caller_saved(&self, reg: X86Register) -> bool {
        match reg {
            X86Register::GP64(gp) => self.caller_saved_gp_registers().contains(&gp),
            X86Register::Xmm(xmm) => self.caller_saved_xmm_registers().contains(&xmm),
            _ => false,
        }
    }

    /// Returns whether the frame pointer (RBP) must be used.
    /// Some ABIs require it for stack unwinding or debugging.
    pub fn requires_frame_pointer(&self) -> bool {
        match self {
            Abi::SystemV => false, // Optional, can use RBP as general purpose
            Abi::Windows => false, // Optional, but recommended for debugging
        }
    }

    /// Returns the register used for struct return pointers (if too large for registers).
    pub fn struct_return_pointer_register(&self) -> GPRegister64 {
        match self {
            Abi::SystemV => GPRegister64::Rdi, // First parameter position
            Abi::Windows => GPRegister64::Rcx, // First parameter position
        }
    }

    /// Returns the maximum size in bytes for a struct to be returned in registers.
    pub fn max_struct_return_size(&self) -> usize {
        match self {
            Abi::SystemV => 16, // Can return up to 128 bits in RAX:RDX or XMM0:XMM1
            Abi::Windows => 8,  // Only 64-bit structs returned in RAX
        }
    }

    /// Returns whether stack parameters are pushed left-to-right or right-to-left.
    pub fn stack_param_order_is_left_to_right(&self) -> bool {
        true // Both ABIs push remaining parameters left-to-right after register params
    }

    /// Returns the offset from the stack pointer where the first stack parameter is located.
    /// This accounts for the return address pushed by the call instruction.
    pub fn first_stack_param_offset(&self) -> u32 {
        match self {
            Abi::SystemV => 8, // Just the return address
            Abi::Windows => 8 + self.shadow_space(), // Return address + shadow space
        }
    }

    /// Returns whether variadic arguments are allowed and how they're handled.
    pub fn variadic_info(&self) -> VariadicInfo {
        match self {
            Abi::SystemV => VariadicInfo {
                supported: true,
                requires_va_list: true,
                // AL register contains number of vector registers used
                requires_vector_count_in_al: true,
            },
            Abi::Windows => VariadicInfo {
                supported: true,
                requires_va_list: true,
                requires_vector_count_in_al: false,
            },
        }
    }

    /// Returns the scratch register typically used for internal operations.
    pub fn scratch_register(&self) -> GPRegister64 {
        GPRegister64::R11 // Commonly used as scratch in both ABIs
    }

    /// Returns information about calling convention name and documentation.
    pub fn name(&self) -> &'static str {
        match self {
            Abi::SystemV => "System V AMD64 ABI",
            Abi::Windows => "Microsoft x64 Calling Convention",
        }
    }
}

/// Information about variadic function support.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VariadicInfo {
    /// Whether variadic functions are supported.
    pub supported: bool,
    /// Whether va_list must be used to access variadic args.
    pub requires_va_list: bool,
    /// Whether AL must contain the number of vector registers used (System V).
    pub requires_vector_count_in_al: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abi_from_platform() {
        assert_eq!(Abi::from_platform(Platform::Windows), Abi::Windows);
        assert_eq!(Abi::from_platform(Platform::Linux), Abi::SystemV);
        assert_eq!(Abi::from_platform(Platform::MacOS), Abi::SystemV);
    }

    #[test]
    fn test_alignment() {
        assert_eq!(Abi::SystemV.alignment(), 16);
        assert_eq!(Abi::Windows.alignment(), 16);
    }

    #[test]
    fn test_red_zone() {
        assert_eq!(Abi::SystemV.red_zone(), 128);
        assert_eq!(Abi::Windows.red_zone(), 0);
    }

    #[test]
    fn test_shadow_space() {
        assert_eq!(Abi::SystemV.shadow_space(), 0);
        assert_eq!(Abi::Windows.shadow_space(), 32);
    }

    #[test]
    fn test_int_param_registers() {
        assert_eq!(Abi::SystemV.int_param_registers().len(), 6);
        assert_eq!(Abi::Windows.int_param_registers().len(), 4);
        assert_eq!(Abi::SystemV.int_param_registers()[0], GPRegister64::Rdi);
        assert_eq!(Abi::Windows.int_param_registers()[0], GPRegister64::Rcx);
    }

    #[test]
    fn test_callee_saved() {
        let abi = Abi::SystemV;
        assert!(abi.is_callee_saved(X86Register::GP64(GPRegister64::Rbx)));
        assert!(!abi.is_callee_saved(X86Register::GP64(GPRegister64::Rax)));
        
        let win_abi = Abi::Windows;
        assert!(win_abi.is_callee_saved(X86Register::GP64(GPRegister64::Rdi)));
        assert!(win_abi.is_callee_saved(X86Register::Xmm(XMMRegister::Xmm6)));
    }
}