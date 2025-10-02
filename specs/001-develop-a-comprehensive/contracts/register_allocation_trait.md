# API Contract: RegisterAllocation Trait

**Feature**: 001-develop-a-comprehensive  
**Contract Type**: Trait Definition  
**Status**: Design Complete

## Trait Definition

```rust
/// Trait providing register allocation guidance for code generation
pub trait RegisterAllocation {
    /// Returns priority-ordered list of volatile GP registers for temporaries
    ///
    /// # Returns
    /// Slice of GPRegister64 in preferred allocation order
    ///
    /// # Platform Behavior
    /// - Windows: [RAX, R10, R11, RCX, RDX, R8, R9]
    /// - System V: [RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11]
    fn volatile_gp_registers() -> &'static [GPRegister64];
    
    /// Returns priority-ordered list of non-volatile GP registers
    ///
    /// # Returns
    /// Slice of GPRegister64 in preferred allocation order
    ///
    /// # Platform Behavior
    /// - Windows: [RBX, RDI, RSI, R12, R13, R14, R15]
    /// - System V: [RBX, R12, R13, R14, R15]
    fn non_volatile_gp_registers() -> &'static [GPRegister64];
    
    /// Returns priority-ordered list of volatile XMM registers
    ///
    /// # Returns
    /// Slice of XMMRegister in preferred allocation order
    ///
    /// # Platform Behavior
    /// - Windows: [XMM0-XMM5]
    /// - System V: [XMM0-XMM15]
    fn volatile_xmm_registers() -> &'static [XMMRegister];
    
    /// Checks if register is volatile for this calling convention
    ///
    /// # Parameters
    /// - `reg`: Register to check
    ///
    /// # Returns
    /// `true` if caller-saved, `false` if callee-saved or special
    fn is_volatile(reg: X86Register) -> bool;
    
    /// Checks if register is callee-saved for this calling convention
    ///
    /// # Parameters
    /// - `reg`: Register to check
    ///
    /// # Returns
    /// `true` if callee must preserve, `false` otherwise
    fn is_callee_saved(reg: X86Register) -> bool;
}
```

## Implementations

```rust
impl RegisterAllocation for WindowsX64 {
    fn volatile_gp_registers() -> &'static [GPRegister64] {
        &[
            GPRegister64::Rax, GPRegister64::R10, GPRegister64::R11,
            GPRegister64::Rcx, GPRegister64::Rdx, 
            GPRegister64::R8, GPRegister64::R9
        ]
    }
    
    fn non_volatile_gp_registers() -> &'static [GPRegister64] {
        &[
            GPRegister64::Rbx, GPRegister64::Rdi, GPRegister64::Rsi,
            GPRegister64::R12, GPRegister64::R13, GPRegister64::R14, GPRegister64::R15
        ]
    }
    
    fn volatile_xmm_registers() -> &'static [XMMRegister] {
        &[
            XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2,
            XMMRegister::Xmm3, XMMRegister::Xmm4, XMMRegister::Xmm5
        ]
    }
    
    fn is_volatile(reg: X86Register) -> bool {
        reg.is_volatile(Platform::Windows)
    }
    
    fn is_callee_saved(reg: X86Register) -> bool {
        reg.is_callee_saved(Platform::Windows)
    }
}

impl RegisterAllocation for SystemV {
    fn volatile_gp_registers() -> &'static [GPRegister64] {
        &[
            GPRegister64::Rax, GPRegister64::Rcx, GPRegister64::Rdx,
            GPRegister64::Rsi, GPRegister64::Rdi,
            GPRegister64::R8, GPRegister64::R9, GPRegister64::R10, GPRegister64::R11
        ]
    }
    
    fn non_volatile_gp_registers() -> &'static [GPRegister64] {
        &[
            GPRegister64::Rbx, GPRegister64::R12, GPRegister64::R13,
            GPRegister64::R14, GPRegister64::R15
        ]
    }
    
    fn volatile_xmm_registers() -> &'static [XMMRegister] {
        &[
            XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2, XMMRegister::Xmm3,
            XMMRegister::Xmm4, XMMRegister::Xmm5, XMMRegister::Xmm6, XMMRegister::Xmm7,
            XMMRegister::Xmm8, XMMRegister::Xmm9, XMMRegister::Xmm10, XMMRegister::Xmm11,
            XMMRegister::Xmm12, XMMRegister::Xmm13, XMMRegister::Xmm14, XMMRegister::Xmm15
        ]
    }
    
    fn is_volatile(reg: X86Register) -> bool {
        reg.is_volatile(Platform::Linux)
    }
    
    fn is_callee_saved(reg: X86Register) -> bool {
        reg.is_callee_saved(Platform::Linux)
    }
}
```

## Contract Tests

```rust
#[cfg(test)]
mod register_allocation_tests {
    #[test]
    fn test_volatile_gp_count() {
        assert_eq!(WindowsX64::volatile_gp_registers().len(), 7);
        assert_eq!(SystemV::volatile_gp_registers().len(), 9);
    }

    #[test]
    fn test_non_volatile_gp_count() {
        assert_eq!(WindowsX64::non_volatile_gp_registers().len(), 7);
        assert_eq!(SystemV::non_volatile_gp_registers().len(), 5);
    }

    #[test]
    fn test_volatility_consistency() {
        let rax = X86Register::GP64(GPRegister64::Rax);
        assert!(WindowsX64::is_volatile(rax));
        assert!(SystemV::is_volatile(rax));
        
        let rbx = X86Register::GP64(GPRegister64::Rbx);
        assert!(WindowsX64::is_callee_saved(rbx));
        assert!(SystemV::is_callee_saved(rbx));
    }
}
```
