use super::Platform;
use super::register::{GPRegister64, X86Register, XMMRegister};
use std::fmt;

/// Represents the kind of Application Binary Interface (ABI) convention.
///
/// This determines how function calls are made, including parameter passing,
/// register usage, and stack layout conventions.
///
/// # ABI Differences
///
/// - **`SystemV`**: Used on Unix-like systems (Linux, macOS, BSD). Passes first 6 integer
///   arguments in registers (RDI, RSI, RDX, RCX, R8, R9), has a 128-byte red zone,
///   and treats all XMM registers as caller-saved.
/// - **Windows**: Used on Windows x64. Passes first 4 arguments in registers
///   (RCX, RDX, R8, R9), requires 32 bytes of shadow space, has no red zone,
///   and treats XMM6-XMM15 as callee-saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiKind {
    /// System V AMD64 ABI used on Unix-like systems (Linux, macOS, BSD).
    SystemV,
    /// Microsoft x64 calling convention used on Windows.
    Windows,
}

impl fmt::Display for AbiKind {
    /// Formats the ABI kind as a human-readable string.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the ABI convention name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemV => write!(f, "System V AMD64 ABI"),
            Self::Windows => write!(f, "Microsoft x64 Calling Convention"),
        }
    }
}

/// Represents a complete ABI specification for x86-64 function calls.
///
/// Combines the calling convention kind with the target platform to provide
/// complete information about parameter passing, register usage, and stack layout.
///
/// # Examples
///
/// ```
/// use jsavrs::asm::AbiKind;
/// use jsavrs::asm::Abi;
/// use jsavrs::asm::Platform;
/// let abi = Abi::from_platform(Platform::Linux);
/// assert_eq!(abi.kind, AbiKind::SystemV);
/// assert_eq!(abi.alignment(), 16);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Abi {
    /// The calling convention kind (System V or Windows).
    pub kind: AbiKind,
    /// The target platform (Linux, macOS, Windows, etc.).
    pub platform: Platform,
}

impl fmt::Display for Abi {
    /// Formats the ABI as a human-readable string combining convention and platform.
    ///
    /// # Returns
    ///
    /// A formatted string in the form "{convention} on {platform}".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.kind, self.platform)
    }
}

#[allow(dead_code)]
impl Abi {
    /// Creates an ABI instance from a target platform.
    ///
    /// Automatically selects the appropriate calling convention based on the platform:
    /// - Windows platform → Windows x64 convention
    /// - All other platforms → System V AMD64 convention
    ///
    /// # Arguments
    ///
    /// * `platform` - The target platform (Linux, macOS, Windows, etc.)
    ///
    /// # Returns
    ///
    /// A new `Abi` instance with the appropriate calling convention for the platform.
    #[must_use]
    pub const fn from_platform(platform: Platform) -> Self {
        let kind = match platform {
            Platform::Windows => AbiKind::Windows,
            _ => AbiKind::SystemV,
        };
        Self { kind, platform }
    }

    /// System V ABI for Linux x86-64 platform.
    ///
    /// Pre-configured constant for the most common Unix-like platform.
    pub const SYSTEM_V_LINUX: Self = Self { kind: AbiKind::SystemV, platform: Platform::Linux };

    /// System V ABI for macOS x86-64 platform.
    ///
    /// Pre-configured constant for Apple's desktop operating system.
    pub const SYSTEM_V_MACOS: Self = Self { kind: AbiKind::SystemV, platform: Platform::MacOS };

    /// Microsoft x64 calling convention for Windows platform.
    ///
    /// Pre-configured constant for the Windows operating system.
    pub const WINDOWS: Self = Self { kind: AbiKind::Windows, platform: Platform::Windows };

    /// Returns the required stack alignment in bytes.
    ///
    /// Both System V and Windows x64 ABIs require 16-byte stack alignment
    /// at function entry (before the call instruction pushes the return address).
    ///
    /// # Returns
    ///
    /// The stack alignment requirement in bytes (always 16 for x86-64).
    #[must_use]
    pub const fn alignment(&self) -> u32 {
        16
    }

    /// Returns the size of the red zone in bytes.
    ///
    /// The red zone is an optimization where leaf functions can use
    /// stack space below RSP without adjusting the stack pointer.
    /// This space is guaranteed not to be clobbered by signal handlers
    /// or asynchronous events.
    ///
    /// # Returns
    ///
    /// - System V: 128 bytes (allowing efficient leaf function optimization)
    /// - Windows: 0 bytes (no red zone available)
    #[must_use]
    pub const fn red_zone(&self) -> u32 {
        match self.kind {
            AbiKind::SystemV => 128,
            AbiKind::Windows => 0,
        }
    }

    /// Returns the size of the shadow space in bytes.
    ///
    /// Shadow space is stack space that must be allocated by the caller
    /// for the callee to spill register parameters if needed. This is
    /// sometimes called "home space" or "register parameter space".
    ///
    /// # Returns
    ///
    /// - System V: 0 bytes (no shadow space required)
    /// - Windows: 32 bytes (4 registers × 8 bytes each)
    #[must_use]
    pub const fn shadow_space(&self) -> u32 {
        match self.kind {
            AbiKind::SystemV => 0,
            AbiKind::Windows => 32, // 4 registers × 8 bytes
        }
    }

    /// Returns the integer parameter registers in order.
    ///
    /// These registers are used to pass integer and pointer arguments to functions.
    /// Arguments beyond the available registers are passed on the stack.
    ///
    /// # Returns
    ///
    /// - System V: `[RDI, RSI, RDX, RCX, R8, R9]` (6 registers)
    /// - Windows: `[RCX, RDX, R8, R9]` (4 registers)
    #[must_use]
    pub const fn int_param_registers(&self) -> &'static [GPRegister64] {
        match self.kind {
            AbiKind::SystemV => super::register::INT_PARAM_REGS_SYSTEMV,
            AbiKind::Windows => super::register::INT_PARAM_REGS_WINDOWS,
        }
    }

    /// Returns the floating-point parameter registers in order.
    ///
    /// These XMM registers are used to pass floating-point and vector arguments.
    ///
    /// # Returns
    ///
    /// - System V: `[XMM0-XMM7]` (8 registers)
    /// - Windows: `[XMM0-XMM3]` (4 registers, interleaved with integer parameters)
    #[must_use]
    pub const fn float_param_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => super::register::FLOAT_PARAM_REGS_SYSTEMV,
            AbiKind::Windows => super::register::FLOAT_PARAM_REGS_WINDOWS,
        }
    }

    /// Returns the integer return value registers.
    ///
    /// First register is primary (RAX), second is used for 128-bit returns (RDX).
    /// This is consistent across both System V and Windows ABIs.
    ///
    /// # Returns
    ///
    /// A slice containing `[RAX, RDX]` for integer return values.
    #[must_use]
    pub const fn int_return_registers(&self) -> &'static [GPRegister64] {
        super::register::INT_RETURN_REGS
    }

    /// Returns the floating-point return value registers.
    ///
    /// Used for returning floating-point and vector values from functions.
    ///
    /// # Returns
    ///
    /// - System V: `[XMM0, XMM1]` (can return up to 128 bits)
    /// - Windows: `[XMM0]` (single register only)
    #[must_use]
    pub const fn float_return_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => super::register::FLOAT_RETURN_REGS_SYSTEMV,
            AbiKind::Windows => super::register::FLOAT_RETURN_REGS_WINDOWS,
        }
    }

    /// Returns the callee-saved (non-volatile) general purpose registers.
    ///
    /// These must be preserved across function calls. If a function modifies
    /// any of these registers, it must save them on entry and restore them
    /// before returning.
    ///
    /// # Returns
    ///
    /// - System V: `[RBX, RBP, R12, R13, R14, R15]`
    /// - Windows: `[RBX, RBP, RDI, RSI, R12, R13, R14, R15]`
    #[must_use]
    pub const fn callee_saved_gp_registers(&self) -> &'static [GPRegister64] {
        match self.kind {
            AbiKind::SystemV => super::register::CALLEE_SAVED_GP_SYSTEMV,
            AbiKind::Windows => super::register::CALLEE_SAVED_GP_WINDOWS,
        }
    }

    /// Returns the callee-saved (non-volatile) XMM registers.
    ///
    /// These XMM registers must be preserved across function calls.
    ///
    /// # Returns
    ///
    /// - System V: Empty slice (all XMM registers are caller-saved)
    /// - Windows: `[XMM6-XMM15]` (10 registers that must be preserved)
    #[must_use]
    pub const fn callee_saved_xmm_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => &[], // All XMM registers are caller-saved
            AbiKind::Windows => super::register::CALLEE_SAVED_XMM_WINDOWS,
        }
    }

    /// Returns the caller-saved (volatile) general purpose registers.
    ///
    /// These registers can be freely modified by a called function without
    /// preservation. The caller must save any values it needs before making
    /// a function call.
    ///
    /// # Returns
    ///
    /// A slice of general purpose registers that are volatile for this ABI.
    #[must_use]
    pub const fn caller_saved_gp_registers(&self) -> &'static [GPRegister64] {
        match self.kind {
            AbiKind::SystemV => super::register::CALLER_SAVED_GP_SYSTEMV,
            AbiKind::Windows => super::register::CALLER_SAVED_GP_WINDOWS,
        }
    }

    /// Returns the caller-saved (volatile) XMM registers.
    ///
    /// These XMM registers can be freely modified by a called function.
    ///
    /// # Returns
    ///
    /// - System V: All XMM registers `[XMM0-XMM15]`
    /// - Windows: `[XMM0-XMM5]` (XMM6-XMM15 are callee-saved)
    #[must_use]
    pub const fn caller_saved_xmm_registers(&self) -> &'static [XMMRegister] {
        match self.kind {
            AbiKind::SystemV => super::register::CALLER_SAVED_XMM_SYSTEMV,
            AbiKind::Windows => super::register::CALLER_SAVED_XMM_WINDOWS,
        }
    }

    /// Checks if a register is callee-saved (non-volatile).
    ///
    /// Delegates to the register's own implementation using the stored platform.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to check
    ///
    /// # Returns
    ///
    /// `true` if the register must be preserved across function calls, `false` otherwise.
    #[must_use]
    pub const fn is_callee_saved(&self, reg: X86Register) -> bool {
        // Delegate to X86Register logic using the stored platform.
        reg.is_callee_saved(self.platform)
    }

    /// Checks if a register is caller-saved (volatile).
    ///
    /// Delegates to the register's own implementation using the stored platform.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to check
    ///
    /// # Returns
    ///
    /// `true` if the register can be freely modified by callees, `false` otherwise.
    #[must_use]
    pub const fn is_caller_saved(&self, reg: X86Register) -> bool {
        reg.is_volatile(self.platform)
    }

    /// Checks if a register is used for passing the parameter at the given index.
    ///
    /// Determines whether a specific register is used to pass the nth parameter
    /// according to this ABI's calling convention.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to check
    /// * `param_index` - The zero-based parameter index
    ///
    /// # Returns
    ///
    /// `true` if the register is used for the specified parameter position, `false` otherwise.
    #[must_use]
    pub const fn is_parameter_register(&self, reg: X86Register, param_index: usize) -> bool {
        reg.is_parameter_register(self.platform, param_index)
    }

    /// Checks if a register is used for the return value.
    ///
    /// Determines whether a register is used to return values from functions.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to check
    ///
    /// # Returns
    ///
    /// `true` if the register is used for return values (e.g., RAX, RDX, XMM0), `false` otherwise.
    #[must_use]
    pub const fn is_return_register(&self, reg: X86Register) -> bool {
        reg.is_return_register(self.platform)
    }

    /// Returns whether the frame pointer (RBP) must be used.
    ///
    /// Some ABIs require RBP to be used as a frame pointer for stack unwinding
    /// or debugging purposes. Both System V and Windows make it optional,
    /// allowing RBP to be used as a general-purpose register if desired.
    ///
    /// # Returns
    ///
    /// - System V: `false` (optional, can use RBP as general purpose)
    /// - Windows: `false` (optional, but recommended for debugging)
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn requires_frame_pointer(&self) -> bool {
        match self.kind {
            AbiKind::SystemV => false, // Optional, can use RBP as general purpose
            AbiKind::Windows => false, // Optional, but recommended for debugging
        }
    }

    /// Returns the register used for struct return pointers.
    ///
    /// When a struct is too large to be returned in registers, the caller
    /// allocates space for it and passes a pointer to that space in this register.
    /// The callee writes the return value to this location.
    ///
    /// # Returns
    ///
    /// - System V: `RDI` (first integer parameter position)
    /// - Windows: `RCX` (first integer parameter position)
    #[must_use]
    pub const fn struct_return_pointer_register(&self) -> GPRegister64 {
        match self.kind {
            AbiKind::SystemV => GPRegister64::Rdi, // First parameter position
            AbiKind::Windows => GPRegister64::Rcx, // First parameter position
        }
    }

    /// Returns the maximum size in bytes for a struct to be returned in registers.
    ///
    /// Structs larger than this size must be returned via pointer (using the
    /// struct return pointer register).
    ///
    /// # Returns
    ///
    /// - System V: 16 bytes (can return up to 128 bits in RAX:RDX or XMM0:XMM1)
    /// - Windows: 8 bytes (only 64-bit structs returned in RAX)
    #[must_use]
    pub const fn max_struct_return_size(&self) -> usize {
        match self.kind {
            AbiKind::SystemV => 16, // Can return up to 128 bits in RAX:RDX or XMM0:XMM1
            AbiKind::Windows => 8,  // Only 64-bit structs returned in RAX
        }
    }

    /// Returns whether stack parameters are pushed left-to-right or right-to-left.
    ///
    /// This determines the order in which arguments beyond the register parameters
    /// are pushed onto the stack.
    ///
    /// # Returns
    ///
    /// `true` for both ABIs (parameters are pushed left-to-right after register params).
    #[must_use]
    pub const fn stack_param_order_is_left_to_right(&self) -> bool {
        true // Both ABIs push remaining parameters left-to-right after register params
    }

    /// Returns the offset from the stack pointer where the first stack parameter is located.
    ///
    /// This accounts for the return address pushed by the call instruction,
    /// and any required shadow space.
    ///
    /// # Returns
    ///
    /// - System V: 8 bytes (just the return address)
    /// - Windows: 40 bytes (8-byte return address + 32 bytes shadow space)
    #[must_use]
    pub const fn first_stack_param_offset(&self) -> u32 {
        match self.kind {
            AbiKind::SystemV => 8,                       // Just the return address
            AbiKind::Windows => 8 + self.shadow_space(), // Return address + shadow space
        }
    }

    /// Returns information about variadic function support and requirements.
    ///
    /// Variadic functions (like printf) have special requirements that vary by ABI.
    ///
    /// # Returns
    ///
    /// A `VariadicInfo` struct describing variadic function handling for this ABI.
    #[must_use]
    pub const fn variadic_info(&self) -> VariadicInfo {
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
    ///
    /// This register is commonly used for temporary calculations and is
    /// safe to use without preservation in both ABIs.
    ///
    /// # Returns
    ///
    /// `R11`, which is caller-saved in both System V and Windows ABIs.
    #[must_use]
    pub const fn scratch_register() -> GPRegister64 {
        GPRegister64::R11 // Commonly used as scratch in both ABIs
    }

    /// Returns the human-readable name of this calling convention.
    ///
    /// # Returns
    ///
    /// A static string slice with the official name of the calling convention.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self.kind {
            AbiKind::SystemV => "System V AMD64 ABI",
            AbiKind::Windows => "Microsoft x64 Calling Convention",
        }
    }
}

/// Information about variadic function support in a calling convention.
///
/// Variadic functions (like `printf` and `scanf`) accept a variable number
/// of arguments. Different ABIs have different requirements for how these
/// functions must be implemented.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VariadicInfo {
    /// Whether variadic functions are supported by this ABI.
    pub supported: bool,
    /// Whether `va_list` must be used to access variadic arguments.
    pub requires_va_list: bool,
    /// Whether AL must contain the number of vector registers used (System V requirement).
    ///
    /// In System V, before calling a variadic function, the caller must set AL
    /// to the number of XMM registers containing arguments (0-8).
    pub requires_vector_count_in_al: bool,
}

impl fmt::Display for VariadicInfo {
    /// Formats the variadic info as a human-readable debug string.
    ///
    /// # Returns
    ///
    /// A formatted string showing all variadic function requirements.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VariadicInfo {{ supported: {}, requires_va_list: {}, requires_vector_count_in_al: {} }}",
            self.supported, self.requires_va_list, self.requires_vector_count_in_al
        )
    }
}
