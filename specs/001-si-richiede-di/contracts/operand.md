# Contract: Operands

## Overview
This document provides a detailed specification for the Operand component of the Jsavrs compiler's assembly (ASM) code generation module. It details the public API, outlines the expected behavior, and describes recommended usage patterns for representing and formatting operands in assembly instructions. The purpose of this document is to guide developers in the consistent implementation and utilization of the Operand component, ensuring correctness and maintainability within the compiler's ASM generation workflow.


## Public API

### Operand
```rust
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Memory {
        base: Option<Register>,
        index: Option<Register>,
        scale: u8,
        displacement: i32,
    },
}

impl Operand {
    pub fn format(&self) -> String
}
```

## API Behavior

### Register(Register)
- **Description**: Represents a register operand used in CPU instructions, serving as a reference to a specific storage location within the processor. This operand encapsulates the concept of a register, which holds temporary data for computational operations.
- **Preconditions**: None
- **Postconditions**: None
- **Errors**: None

### Immediate(i64)
- **Description**: An immediate value operand refers to a constant value that is specified directly within an instruction, rather than being retrieved from a register or memory location. This type of operand is commonly used in assembly language and low-level programming to provide fixed numerical values for computations or control flow operations.
- **Preconditions**: None
- **Postconditions**: None
- **Errors**: None

### Memory Operand Components (base, index, scale, displacement)
- **Description**: 
  Represents a memory operand in which the base register, index register, scale factor, and displacement are optional components. These components are used together to compute the effective address of a memory location. The base register provides a reference address, the index register can be scaled by the scale factor to modify the address, and the displacement is an additional constant offset.
- **Preconditions**: 
  - The `scale` factor must be 1, 2, 4, or 8, as only these values are valid for scaling the index register during effective address computation.
  - All registers used as base or index must be valid general-purpose registers supported by the target architecture.
  - The displacement, if provided, must be a valid signed integer within the architecture's addressing limits.
- **Postconditions**: None
- **Errors**: None

### format()
- **Description**: Converts the specified operand into a formatted assembly-language representation. The output string reflects the operand's syntax as it would appear in assembly code.
- **Preconditions**: None
- **Postconditions**:
  - Returns a string containing the assembly-language representation of the operand. The format of the string is consistent with standard assembly syntax conventions.
- **Errors**: None

## Usage Examples

### Register Operand
```rust
let reg = Register::new("rax", 64, 0);
let operand = Operand::Register(reg);
let formatted = operand.format(); // "rax"
```

### Immediate Operand
```rust
let operand = Operand::Immediate(42);
let formatted = operand.format(); // "42"
```

### Memory Operand
```rust
let base_reg = Register::new("rbp", 64, 5);
let operand = Operand::Memory {
    base: Some(base_reg),
    index: None,
    scale: 1,
    displacement: -8,
};
let formatted = operand.format(); // "[rbp - 8]"
```

## Implementation Constraints
1. Memory operand scale factors must conform to standard architectural values: 1, 2, 4, or 8.
2. Operands must be formatted according to NASM syntax and conventions, including addressing modes and operand ordering.
3. Register operands must correspond to registers defined by the target architecture to ensure valid references.