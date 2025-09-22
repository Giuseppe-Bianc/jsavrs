# Contract: Instructions

## Overview
This document defines the interface specification for the Instruction component of the jsavrs compiler's assembly (ASM) generation module. It details the public API, behavioral specifications, and usage patterns for the representation and formatting of assembly instructions within the compiler's back-end. This specification provides a clear framework for developers to implement and utilize the Instruction component consistently and effectively.


## Public API

### Instruction
```rust
pub struct Instruction {
    mnemonic: String,
    operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(mnemonic: &str) -> Self
    pub fn add_operand(&mut self, operand: Operand)
    pub fn format(&self) -> String
}
```

## API Behavior

### new(mnemonic: &str)
- **Description**:  
  Instantiates a new `Instruction` object using the specified mnemonic. This function initializes the instruction with no operands and prepares it for further configuration or execution.
- **Preconditions**:  
  - The `mnemonic` parameter must correspond to a recognized instruction mnemonic as defined by the instruction set specification.
- **Postconditions**:  
  - A new `Instruction` instance is instantiated with the specified mnemonic.  
  - The `operands` vector of the newly created instruction is initialized as empty.
- **Errors**: None


### add_operand(operand: Operand)
- **Description**: Adds the specified operand to the instruction's operand list, updating the internal state of the instruction accordingly.
- **Preconditions**: The function requires no specific preconditions; the instruction object must be properly initialized prior to invoking this method.
- **Postconditions**: The specified operand is appended to the instruction's operand list, ensuring that the instruction reflects the addition.
- **Errors**: None

### format()
- **Description**: Converts a machine instruction or intermediate representation into a formatted assembly language string. The output includes the mnemonic, operands, and addressing modes according to standard assembly conventions, making it suitable for display, debugging, or further processing.
- **Preconditions**: No preconditions are required. The function can be invoked with any valid instruction object.
- **Postconditions**: Returns a string representing the instruction in standard assembly syntax, including all relevant components such as mnemonic, operands, and addressing information.
- **Errors**: None

## Usage Examples

### Basic Usage
```rust
let mut mov_inst = Instruction::new("mov");
mov_inst.add_operand(Operand::Register(Register::new("rax", 64, 0)));
mov_inst.add_operand(Operand::Immediate(42));

let formatted = mov_inst.format(); // "mov rax, 42"
```

## Implementation Constraints
1. Assembly instruction mnemonics must conform to the syntax and operational rules of the target architecture, ensuring that they are recognized and correctly executed by the processor.
2. Operands must be of types, sizes, and addressing modes that are supported by their corresponding instructions, guaranteeing proper execution and avoiding runtime errors.
3. The generated assembly code must adhere to the syntax, formatting, and conventions of the NASM assembler, ensuring compatibility and correct assembly.