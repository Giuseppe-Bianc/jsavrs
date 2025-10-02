# Quick Start Guide: x86-64 ABI Trait System

**Feature**: 001-develop-a-comprehensive  
**Date**: October 2, 2025  
**Status**: Phase 1 Complete

## Overview

This guide demonstrates how to use the x86-64 ABI trait system in the jsavrs compiler for querying calling conventions, register allocation, and stack management across Windows, Linux, and macOS platforms.

## Basic Usage

### 1. Querying Parameter Registers

```rust
use jsavrs::asm::{CallingConvention, WindowsX64, SystemV, GPRegister64, XMMRegister};

// Windows x64: Get register for first integer parameter
let reg = WindowsX64::integer_param_register(0);
assert_eq!(reg, Some(GPRegister64::Rcx));

// System V: Get register for first integer parameter
let reg = SystemV::integer_param_register(0);
assert_eq!(reg, Some(GPRegister64::Rdi));

// Check if parameter index requires stack placement
let reg = WindowsX64::integer_param_register(5);
assert_eq!(reg, None); // Beyond 4th parameter → stack
```

### 2. Checking Register Volatility

```rust
use jsavrs::asm::{RegisterAllocation, X86Register, Platform};

// Check if RAX is volatile (caller-saved)
let rax = X86Register::GP64(GPRegister64::Rax);
assert!(WindowsX64::is_volatile(rax));

// Check if RBX is callee-saved (must be preserved)
let rbx = X86Register::GP64(GPRegister64::Rbx);
assert!(WindowsX64::is_callee_saved(rbx));

// Platform-specific behavior
assert!(rax.is_volatile(Platform::Windows));
assert!(rax.is_volatile(Platform::Linux));
```

### 3. Stack Management Queries

```rust
use jsavrs::asm::{StackManagement, WindowsX64, SystemV};

// Check red zone availability
if SystemV::has_red_zone() {
    let size = SystemV::red_zone_size_bytes();
    println!("Red zone available: {} bytes", size); // 128
}

// Check shadow space requirements
if WindowsX64::requires_shadow_space() {
    let size = WindowsX64::shadow_space_bytes();
    println!("Shadow space required: {} bytes", size); // 32
}

// Get stack alignment
let align = WindowsX64::min_stack_alignment(); // 16
```

## Common Scenarios

### Scenario 1: Generating Function Prologue

```rust
use jsavrs::asm::*;

fn generate_prologue<CC: CallingConvention + StackManagement>() -> Vec<Instruction> {
    let mut instrs = Vec::new();
    
    // Push frame pointer if needed
    if CC::requires_frame_pointer() {
        instrs.push(Instruction::Push { 
            src: Operand::Register(X86Register::GP64(GPRegister64::Rbp))
        });
        instrs.push(Instruction::Mov {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rbp)),
            src: Operand::Register(X86Register::GP64(GPRegister64::Rsp)),
        });
    }
    
    // Allocate shadow space if required
    if CC::requires_shadow_space() {
        let space = CC::shadow_space_bytes() as i32;
        instrs.push(Instruction::Sub {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rsp)),
            src: Operand::Immediate(Immediate::Imm32(space)),
        });
    }
    
    instrs
}

// Usage
let windows_prologue = generate_prologue::<WindowsX64>();
let systemv_prologue = generate_prologue::<SystemV>();
```

### Scenario 2: Allocating Registers for Parameters

```rust
fn allocate_param_registers<CC: CallingConvention>(
    param_types: &[ParamType]
) -> Vec<Option<X86Register>> {
    param_types.iter().enumerate().map(|(i, ty)| {
        match ty {
            ParamType::Integer => 
                CC::integer_param_register(i).map(X86Register::GP64),
            ParamType::Float => 
                CC::float_param_register(i).map(X86Register::Xmm),
        }
    }).collect()
}

enum ParamType {
    Integer,
    Float,
}

// Usage
let types = vec![ParamType::Integer, ParamType::Float, ParamType::Integer];
let windows_regs = allocate_param_registers::<WindowsX64>(&types);
// Windows: [Some(RCX), Some(XMM1), Some(R8)]

let systemv_regs = allocate_param_registers::<SystemV>(&types);
// System V: [Some(RDI), Some(XMM0), Some(RSI)]
```

### Scenario 3: Selecting Temporary Registers

```rust
use jsavrs::asm::{RegisterAllocation};

fn select_temp_register<RA: RegisterAllocation>() -> GPRegister64 {
    // Get first available volatile register
    RA::volatile_gp_registers()[0]
}

// Usage
let temp_win = select_temp_register::<WindowsX64>(); // RAX
let temp_sv = select_temp_register::<SystemV>();     // RAX
```

### Scenario 4: Handling Structure Returns

```rust
use jsavrs::asm::{AggregateClassification, FieldType};

fn determine_return_mechanism<AC: AggregateClassification>(
    struct_size: usize,
    fields: &[FieldType]
) -> String {
    match AC::classify_aggregate(struct_size, fields) {
        AggregateClass::ByValue(reg) => 
            format!("Return in register: {}", reg),
        AggregateClass::ByReference => 
            "Return via hidden pointer parameter".to_string(),
        AggregateClass::Decomposed(regs) => 
            format!("Return decomposed across {} registers", regs.len()),
    }
}

// Usage
let small_struct = determine_return_mechanism::<WindowsX64>(8, &[FieldType::Integer]);
// "Return in register: rcx"

let large_struct = determine_return_mechanism::<WindowsX64>(16, &[]);
// "Return via hidden pointer parameter"
```

## Platform-Specific Examples

### Windows x64

```rust
use jsavrs::asm::*;

// Function: int add(int a, int b, int c, int d, int e)
// Parameters: RCX, RDX, R8, R9, [stack]
fn generate_add_windows() -> Vec<Instruction> {
    vec![
        // a in RCX, b in RDX, c in R8, d in R9, e on stack
        Instruction::Add {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
            src: Operand::Register(X86Register::GP64(GPRegister64::Rdx)),
        },
        Instruction::Add {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
            src: Operand::Register(X86Register::GP64(GPRegister64::R8)),
        },
        Instruction::Add {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
            src: Operand::Register(X86Register::GP64(GPRegister64::R9)),
        },
        // Load 5th parameter from stack (after shadow space)
        Instruction::Add {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
            src: Operand::Memory(MemoryOperand {
                base: Some(GPRegister64::Rsp),
                index: None,
                scale: Scale::One,
                displacement: 40, // 32 (shadow) + 8 (return addr)
                size: OperandSize::Qword,
            }),
        },
        // Result in RAX
        Instruction::Mov {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rax)),
            src: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
        },
        Instruction::Ret,
    ]
}
```

### System V (Linux/macOS)

```rust
// Function: double compute(int a, double b, int c, double d)
// Parameters: RDI (a), XMM0 (b), RSI (c), XMM1 (d)
fn generate_compute_systemv() -> Vec<Instruction> {
    vec![
        // Convert a (RDI) to double
        Instruction::Cvtsi2sd {
            dest: Operand::Register(X86Register::Xmm(XMMRegister::Xmm2)),
            src: Operand::Register(X86Register::GP64(GPRegister64::Rdi)),
        },
        // Add b (XMM0)
        Instruction::Addsd {
            dest: Operand::Register(X86Register::Xmm(XMMRegister::Xmm2)),
            src: Operand::Register(X86Register::Xmm(XMMRegister::Xmm0)),
        },
        // Convert c (RSI) to double
        Instruction::Cvtsi2sd {
            dest: Operand::Register(X86Register::Xmm(XMMRegister::Xmm3)),
            src: Operand::Register(X86Register::GP64(GPRegister64::Rsi)),
        },
        // Add d (XMM1)
        Instruction::Addsd {
            dest: Operand::Register(X86Register::Xmm(XMMRegister::Xmm3)),
            src: Operand::Register(X86Register::Xmm(XMMRegister::Xmm1)),
        },
        // Multiply results
        Instruction::Mulsd {
            dest: Operand::Register(X86Register::Xmm(XMMRegister::Xmm2)),
            src: Operand::Register(X86Register::Xmm(XMMRegister::Xmm3)),
        },
        // Result in XMM0
        Instruction::Movsd {
            dest: Operand::Register(X86Register::Xmm(XMMRegister::Xmm0)),
            src: Operand::Register(X86Register::Xmm(XMMRegister::Xmm2)),
        },
        Instruction::Ret,
    ]
}
```

## Verification Against Reference Compilers

### Step 1: Create Test C Program

```c
// test_abi.c
int add5(int a, int b, int c, int d, int e) {
    return a + b + c + d + e;
}
```

### Step 2: Compile with Reference Compiler

```powershell
# Windows (MSVC)
cl /c /Fa test_abi.c

# Linux (GCC)
gcc -S -O2 test_abi.c -o test_abi.s

# macOS (Clang)
clang -S -O2 test_abi.c -o test_abi.s
```

### Step 3: Inspect Assembly Output

```powershell
# View generated assembly
cat test_abi.s
```

### Step 4: Compare with jsavrs Output

```rust
// Generate jsavrs assembly
let instrs = generate_add_windows();
for instr in instrs {
    println!("{}", instr); // Implement Display for Instruction
}
```

### Step 5: Validate Register Usage

- Verify parameter registers match
- Check shadow space allocation
- Confirm return value placement
- Validate volatile register handling

## Performance Validation

```rust
use std::time::Instant;

fn benchmark_abi_queries() {
    let iterations = 1_000_000;
    
    let start = Instant::now();
    for i in 0..iterations {
        let _ = WindowsX64::integer_param_register(i % 6);
    }
    let elapsed = start.elapsed();
    
    let ns_per_query = elapsed.as_nanos() / iterations;
    println!("Average query time: {} ns", ns_per_query);
    
    // Verify < 10 ns target
    assert!(ns_per_query < 10, "Performance target violated!");
}
```

## Common Pitfalls

### 1. Forgetting Shadow Space

```rust
// ❌ WRONG: No shadow space on Windows
Instruction::Call { target: Operand::Label("func".into()) }

// ✅ CORRECT: Allocate shadow space first
Instruction::Sub {
    dest: Operand::Register(X86Register::GP64(GPRegister64::Rsp)),
    src: Operand::Immediate(Immediate::Imm32(32)),
}
Instruction::Call { target: Operand::Label("func".into()) }
```

### 2. Mixing Integer and FP Parameter Indices

```rust
// Windows: Indices overlap
let reg1 = WindowsX64::integer_param_register(1); // RDX
let reg2 = WindowsX64::float_param_register(1);   // XMM1
// param[1] is EITHER RDX OR XMM1, not both

// System V: Independent indices
let reg1 = SystemV::integer_param_register(1);    // RSI
let reg2 = SystemV::float_param_register(1);      // XMM1
// param can use both RSI (int) AND XMM1 (fp) simultaneously
```

### 3. Assuming Red Zone Availability

```rust
// ❌ WRONG: Red zone not available on Windows
// Using stack below RSP without adjustment

// ✅ CORRECT: Check platform
if SystemV::has_red_zone() {
    // Can use red zone for leaf functions
} else {
    // Must adjust RSP for local variables
}
```

## Next Steps

1. **Implement Traits**: Create trait implementations in `src/asm/calling_convention.rs`
2. **Write Tests**: Add comprehensive unit tests (see contracts/)
3. **Integrate**: Use traits in code generator modules
4. **Benchmark**: Verify < 0.1% compilation time overhead
5. **Validate**: Cross-check with GCC/Clang/MSVC output

## Additional Resources

- System V AMD64 ABI Specification
- Microsoft x64 Calling Convention Documentation
- Intel Software Developer Manuals (Volume 1-3)
- jsavrs research.md (detailed architectural decisions)
- jsavrs data-model.md (complete entity specifications)

---

**Document Version**: 1.0  
**Last Updated**: October 2, 2025  
**Status**: Ready for Implementation
