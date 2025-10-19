use super::Platform;
use super::register::{GPRegister64, X86Register, XMMRegister};
use std::fmt;

/// Represents the kind of Application Binary Interface (ABI) convention.
///
/// This determines how function calls are made, including parameter passing,
/// register usage, and stack layout conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiKind {
    /// System V AMD64 ABI used on Unix-like systems (Linux, macOS, BSD).
    SystemV,
    /// Microsoft x64 calling convention used on Windows.
    Windows,
}

impl fmt::Display for AbiKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbiKind::SystemV => write!(f, "System V AMD64 ABI"),
            AbiKind::Windows => write!(f, "Microsoft x64 Calling Convention"),
        }
    }
}

/// Represents a complete ABI specification for x86-64 function calls.
///
/// Combines the calling convention kind with the target platform to provide
/// complete information about parameter passing, register usage, and stack layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Abi {
    /// The calling convention kind (System V or Windows).
    pub kind: AbiKind,
    /// The target platform (Linux, macOS, Windows, etc.).
    pub platform: Platform,
}

impl fmt::Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.kind, self.platform)
    }
}

#[allow(dead_code)]
impl Abi {
    pub fn from_platform(platform: Platform) -> Self {
        let kind = match platform {
            Platform::Windows => AbiKind::Windows,
            _ => AbiKind::SystemV,
        };
        Abi { kind, platform }
    }

    /// System V ABI for Linux x86-64 platform.
    pub const SYSTEM_V_LINUX: Abi = Abi { kind: AbiKind::SystemV, platform: Platform::Linux };
    /// System V ABI for macOS x86-64 platform.
    pub const SYSTEM_V_MACOS: Abi = Abi { kind: AbiKind::SystemV, platform: Platform::MacOS };
    /// Microsoft x64 calling convention for Windows platform.
    pub const WINDOWS: Abi = Abi { kind: AbiKind::Windows, platform: Platform::Windows };

    /// Returns the required stack alignment in bytes.
    pub fn alignment(&self) -> u32 {
        16
    }

    /// Returns the size of the red zone in bytes.
    /// The red zone is an optimization where leaf functions can use
    /// stack space below RSP without adjusting the stack pointer.
    pub fn red_zone(&self) -> u32 {
        match self.kind {
            AbiKind::SystemV => 128,
            AbiKind::Windows => 0,
        }
    }

    /// Returns the size of the shadow space in bytes.
    /// Shadow space is stack space that must be allocated by the caller
    /// for the callee to spill register parameters if needed.
    pub fn shadow_space(&self) -> u32 {
        match self.kind {
            AbiKind::SystemV => 0,
            AbiKind::Windows => 32, // 4 registers × 8 bytes
        }
    }

    /// Returns the integer parameter registers in order.
    pub fn int_param_registers(&self) -> &'static [GPRegister64] {
        match self.kind {
            AbiKind::SystemV => super::register::INT_PARAM_REGS_SYSTEMV,
            AbiKind::Windows => super::register::INT_PARAM_REGS_WINDOWS,
        }
    }

    /// Returns the floating-point parameter registers in order.
    pub fn float_param_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => super::register::FLOAT_PARAM_REGS_SYSTEMV,
            AbiKind::Windows => super::register::FLOAT_PARAM_REGS_WINDOWS,
        }
    }

    /// Returns the integer return value registers.
    /// First register is primary, second is used for 128-bit returns.
    pub fn int_return_registers(&self) -> &'static [GPRegister64] {
        super::register::INT_RETURN_REGS
    }

    /// Returns the floating-point return value registers.
    pub fn float_return_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => super::register::FLOAT_RETURN_REGS_SYSTEMV,
            AbiKind::Windows => super::register::FLOAT_RETURN_REGS_WINDOWS,
        }
    }

    /// Returns the callee-saved (non-volatile) general purpose registers.
    /// These must be preserved across function calls.
    pub fn callee_saved_gp_registers(&self) -> &'static [GPRegister64] {
        match self.kind {
            AbiKind::SystemV => super::register::CALLEE_SAVED_GP_SYSTEMV,
            AbiKind::Windows => super::register::CALLEE_SAVED_GP_WINDOWS,
        }
    }

    /// Returns the callee-saved (non-volatile) XMM registers.
    pub fn callee_saved_xmm_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => &[], // All XMM registers are caller-saved
            AbiKind::Windows => super::register::CALLEE_SAVED_XMM_WINDOWS,
        }
    }

    /// Returns the caller-saved (volatile) general purpose registers.
    pub fn caller_saved_gp_registers(&self) -> &'static [GPRegister64] {
        match self.kind {
            AbiKind::SystemV => super::register::CALLER_SAVED_GP_SYSTEMV,
            AbiKind::Windows => super::register::CALLER_SAVED_GP_WINDOWS,
        }
    }

    /// Returns the caller-saved (volatile) XMM registers.
    pub fn caller_saved_xmm_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => super::register::CALLER_SAVED_XMM_SYSTEMV,
            AbiKind::Windows => super::register::CALLER_SAVED_XMM_WINDOWS,
        }
    }

    /// Checks if a register is callee-saved (non-volatile).
    pub fn is_callee_saved(&self, reg: X86Register) -> bool {
        // Delegate to X86Register logic using the stored platform.
        reg.is_callee_saved(self.platform)
    }

    /// Checks if a register is caller-saved (volatile).
    pub fn is_caller_saved(&self, reg: X86Register) -> bool {
        reg.is_volatile(self.platform)
    }

    ///  Checks if a register is used for passing the parameter at the given index.
    pub fn is_parameter_register(&self, reg: X86Register, param_index: usize) -> bool {
        reg.is_parameter_register(self.platform, param_index)
    }

    /// Checks if a register is used for the return value.
    pub fn is_return_register(&self, reg: X86Register) -> bool {
        reg.is_return_register(self.platform)
    }

    /// Returns whether the frame pointer (RBP) must be used.
    /// Some ABIs require it for stack unwinding or debugging.
    pub fn requires_frame_pointer(&self) -> bool {
        match self.kind {
            AbiKind::SystemV => false, // Optional, can use RBP as general purpose
            AbiKind::Windows => false, // Optional, but recommended for debugging
        }
    }

    /// Returns the register used for struct return pointers (if too large for registers).
    pub fn struct_return_pointer_register(&self) -> GPRegister64 {
        match self.kind {
            AbiKind::SystemV => GPRegister64::Rdi, // First parameter position
            AbiKind::Windows => GPRegister64::Rcx, // First parameter position
        }
    }

    /// Returns the maximum size in bytes for a struct to be returned in registers.
    pub fn max_struct_return_size(&self) -> usize {
        match self.kind {
            AbiKind::SystemV => 16, // Can return up to 128 bits in RAX:RDX or XMM0:XMM1
            AbiKind::Windows => 8,  // Only 64-bit structs returned in RAX
        }
    }

    /// Returns whether stack parameters are pushed left-to-right or right-to-left.
    pub fn stack_param_order_is_left_to_right(&self) -> bool {
        true // Both ABIs push remaining parameters left-to-right after register params
    }

    /// Returns the offset from the stack pointer where the first stack parameter is located.
    /// This accounts for the return address pushed by the call instruction.
    pub fn first_stack_param_offset(&self) -> u32 {
        match self.kind {
            AbiKind::SystemV => 8,                       // Just the return address
            AbiKind::Windows => 8 + self.shadow_space(), // Return address + shadow space
        }
    }

    /// Returns whether variadic arguments are allowed and how they're handled.
    pub fn variadic_info(&self) -> VariadicInfo {
        match self.kind {
            AbiKind::SystemV => VariadicInfo {
                supported: true,
                requires_va_list: true,
                // AL register contains number of vector registers used
                requires_vector_count_in_al: true,
            },
            AbiKind::Windows => {
                VariadicInfo { supported: true, requires_va_list: true, requires_vector_count_in_al: false }
            }
        }
    }

    /// Returns the scratch register typically used for internal operations.
    pub fn scratch_register(&self) -> GPRegister64 {
        GPRegister64::R11 // Commonly used as scratch in both ABIs
    }

    /// Returns information about calling convention name and documentation.
    pub fn name(&self) -> &'static str {
        match self.kind {
            AbiKind::SystemV => "System V AMD64 ABI",
            AbiKind::Windows => "Microsoft x64 Calling Convention",
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

impl fmt::Display for VariadicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VariadicInfo {{ supported: {}, requires_va_list: {}, requires_vector_count_in_al: {} }}",
            self.supported, self.requires_va_list, self.requires_vector_count_in_al
        )
    }
}
