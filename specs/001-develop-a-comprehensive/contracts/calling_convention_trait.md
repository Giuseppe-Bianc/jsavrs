# API Contract: CallingConvention Trait

**Feature**: 001-develop-a-comprehensive  
**Contract Type**: Trait Definition  
**Status**: Design Complete

## Contract Overview

The `CallingConvention` trait provides a platform-independent interface for querying x86-64 ABI calling convention specifications. Implementations must provide constant-time lookups (< 0.1% compilation overhead) for parameter register allocation, volatility classification, and return value conventions.

## Trait Definition

```rust
/// Trait defining platform-specific calling convention specifications
pub trait CallingConvention {
    /// Returns the target platform for this calling convention
    ///
    /// # Returns
    /// Platform enum variant (Windows, Linux, or MacOS)
    ///
    /// # Performance
    /// Constant-time operation
    fn platform() -> Platform;
    
    /// Returns the ABI variant for this calling convention
    ///
    /// # Returns
    /// Abi enum variant (SystemV or Windows)
    ///
    /// # Performance
    /// Constant-time operation
    fn abi() -> Abi;
    
    /// Gets the register allocated for the Nth integer/pointer parameter
    ///
    /// # Parameters
    /// - `index`: Zero-based parameter position (0 = first parameter)
    ///
    /// # Returns
    /// - `Some(register)` if parameter can be passed in a register
    /// - `None` if parameter must be passed on the stack
    ///
    /// # Examples
    /// ```rust
    /// // Windows x64: First integer parameter uses RCX
    /// assert_eq!(WindowsX64::integer_param_register(0), Some(GPRegister64::Rcx));
    /// 
    /// // System V: First integer parameter uses RDI
    /// assert_eq!(SystemV::integer_param_register(0), Some(GPRegister64::Rdi));
    /// 
    /// // Beyond register limit returns None
    /// assert_eq!(WindowsX64::integer_param_register(4), None);
    /// ```
    ///
    /// # Performance
    /// O(1) - Array indexing with bounds check
    fn integer_param_register(index: usize) -> Option<GPRegister64>;
    
    /// Gets the register allocated for the Nth floating-point parameter
    ///
    /// # Parameters
    /// - `index`: Zero-based parameter position (0 = first FP parameter)
    ///
    /// # Returns
    /// - `Some(register)` if parameter can be passed in a register
    /// - `None` if parameter must be passed on the stack
    ///
    /// # Platform Behavior
    /// - **Windows**: Indices 0-3 map to XMM0-XMM3, overlaps with integer params
    /// - **System V**: Indices 0-7 map to XMM0-XMM7, independent of integer params
    ///
    /// # Examples
    /// ```rust
    /// // Windows x64: Second FP parameter uses XMM1
    /// assert_eq!(WindowsX64::float_param_register(1), Some(XMMRegister::Xmm1));
    /// 
    /// // System V: Supports 8 FP registers
    /// assert_eq!(SystemV::float_param_register(7), Some(XMMRegister::Xmm7));
    /// ```
    ///
    /// # Performance
    /// O(1) - Array indexing with bounds check
    fn float_param_register(index: usize) -> Option<XMMRegister>;
    
    /// Returns the maximum number of integer parameters passed in registers
    ///
    /// # Returns
    /// - Windows x64: 4
    /// - System V: 6
    ///
    /// # Performance
    /// Const evaluation (zero runtime cost)
    fn max_integer_register_params() -> usize;
    
    /// Returns the maximum number of floating-point parameters passed in registers
    ///
    /// # Returns
    /// - Windows x64: 4
    /// - System V: 8
    ///
    /// # Performance
    /// Const evaluation (zero runtime cost)
    fn max_float_register_params() -> usize;
    
    /// Returns true if integer and FP parameter indices share the same index space
    ///
    /// # Returns
    /// - Windows x64: `true` (parameter N is either int OR float, not both)
    /// - System V: `false` (independent register allocation)
    ///
    /// # Examples
    /// ```rust
    /// // Windows: param[1] could be RDX or XMM1, but not both
    /// assert!(WindowsX64::params_share_index_space());
    /// 
    /// // System V: param[1] could be both RSI (int) and XMM1 (fp)
    /// assert!(!SystemV::params_share_index_space());
    /// ```
    ///
    /// # Performance
    /// Const evaluation (zero runtime cost)
    fn params_share_index_space() -> bool;
}
```

## Implementation Requirements

### Windows x64 Implementation

```rust
pub struct WindowsX64;

impl CallingConvention for WindowsX64 {
    fn platform() -> Platform {
        Platform::Windows
    }
    
    fn abi() -> Abi {
        Abi::Windows
    }
    
    fn integer_param_register(index: usize) -> Option<GPRegister64> {
        const PARAMS: [GPRegister64; 4] = [
            GPRegister64::Rcx, GPRegister64::Rdx, 
            GPRegister64::R8, GPRegister64::R9
        ];
        PARAMS.get(index).copied()
    }
    
    fn float_param_register(index: usize) -> Option<XMMRegister> {
        const PARAMS: [XMMRegister; 4] = [
            XMMRegister::Xmm0, XMMRegister::Xmm1,
            XMMRegister::Xmm2, XMMRegister::Xmm3
        ];
        PARAMS.get(index).copied()
    }
    
    fn max_integer_register_params() -> usize { 4 }
    fn max_float_register_params() -> usize { 4 }
    fn params_share_index_space() -> bool { true }
}
```

### System V Implementation

```rust
pub struct SystemV;

impl CallingConvention for SystemV {
    fn platform() -> Platform {
        Platform::Linux  // Also applies to MacOS
    }
    
    fn abi() -> Abi {
        Abi::SystemV
    }
    
    fn integer_param_register(index: usize) -> Option<GPRegister64> {
        const PARAMS: [GPRegister64; 6] = [
            GPRegister64::Rdi, GPRegister64::Rsi, GPRegister64::Rdx,
            GPRegister64::Rcx, GPRegister64::R8, GPRegister64::R9
        ];
        PARAMS.get(index).copied()
    }
    
    fn float_param_register(index: usize) -> Option<XMMRegister> {
        const PARAMS: [XMMRegister; 8] = [
            XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2, XMMRegister::Xmm3,
            XMMRegister::Xmm4, XMMRegister::Xmm5, XMMRegister::Xmm6, XMMRegister::Xmm7
        ];
        PARAMS.get(index).copied()
    }
    
    fn max_integer_register_params() -> usize { 6 }
    fn max_float_register_params() -> usize { 8 }
    fn params_share_index_space() -> bool { false }
}
```

## Contract Tests

### Test Suite Structure

```rust
#[cfg(test)]
mod calling_convention_tests {
    use super::*;

    #[test]
    fn test_windows_integer_params() {
        assert_eq!(WindowsX64::integer_param_register(0), Some(GPRegister64::Rcx));
        assert_eq!(WindowsX64::integer_param_register(1), Some(GPRegister64::Rdx));
        assert_eq!(WindowsX64::integer_param_register(2), Some(GPRegister64::R8));
        assert_eq!(WindowsX64::integer_param_register(3), Some(GPRegister64::R9));
        assert_eq!(WindowsX64::integer_param_register(4), None);
    }

    #[test]
    fn test_systemv_integer_params() {
        assert_eq!(SystemV::integer_param_register(0), Some(GPRegister64::Rdi));
        assert_eq!(SystemV::integer_param_register(1), Some(GPRegister64::Rsi));
        assert_eq!(SystemV::integer_param_register(2), Some(GPRegister64::Rdx));
        assert_eq!(SystemV::integer_param_register(3), Some(GPRegister64::Rcx));
        assert_eq!(SystemV::integer_param_register(4), Some(GPRegister64::R8));
        assert_eq!(SystemV::integer_param_register(5), Some(GPRegister64::R9));
        assert_eq!(SystemV::integer_param_register(6), None);
    }

    #[test]
    fn test_windows_float_params() {
        assert_eq!(WindowsX64::float_param_register(0), Some(XMMRegister::Xmm0));
        assert_eq!(WindowsX64::float_param_register(3), Some(XMMRegister::Xmm3));
        assert_eq!(WindowsX64::float_param_register(4), None);
    }

    #[test]
    fn test_systemv_float_params() {
        assert_eq!(SystemV::float_param_register(0), Some(XMMRegister::Xmm0));
        assert_eq!(SystemV::float_param_register(7), Some(XMMRegister::Xmm7));
        assert_eq!(SystemV::float_param_register(8), None);
    }

    #[test]
    fn test_max_params() {
        assert_eq!(WindowsX64::max_integer_register_params(), 4);
        assert_eq!(WindowsX64::max_float_register_params(), 4);
        assert_eq!(SystemV::max_integer_register_params(), 6);
        assert_eq!(SystemV::max_float_register_params(), 8);
    }

    #[test]
    fn test_index_space_sharing() {
        assert!(WindowsX64::params_share_index_space());
        assert!(!SystemV::params_share_index_space());
    }

    #[test]
    fn test_abi_platform_mapping() {
        assert_eq!(WindowsX64::platform(), Platform::Windows);
        assert_eq!(WindowsX64::abi(), Abi::Windows);
        assert_eq!(SystemV::platform(), Platform::Linux);
        assert_eq!(SystemV::abi(), Abi::SystemV);
    }
}
```

## Performance Contract

### Requirements
- All method calls MUST complete in < 10 nanoseconds
- Array lookups MUST be O(1) constant time
- No heap allocations permitted
- All methods SHOULD be inlined by compiler

### Verification
```rust
#[bench]
fn bench_integer_param_lookup(b: &mut Bencher) {
    b.iter(|| {
        black_box(WindowsX64::integer_param_register(black_box(2)))
    });
    // Expected: < 10ns per iteration
}
```

## Error Handling

### Type System Guarantees
- Invalid platform/register combinations prevented at compile time
- Out-of-bounds indices return `None` (no panics)
- Exhaustive pattern matching enforced

### Invalid States
None possible - trait design prevents invalid configurations

## Dependencies

- `register.rs`: GPRegister64, XMMRegister enums
- `abi.rs`: Platform, Abi enums

## Version History

- v1.0 (2025-10-02): Initial contract definition
