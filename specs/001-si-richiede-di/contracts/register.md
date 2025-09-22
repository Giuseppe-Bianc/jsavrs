# Contract: Registers

## Overview
This document provides a formal specification for the Register component of the jsavrs compiler's Assembly (ASM) generation module. It defines the public API, outlines the intended behavior, and describes the usage patterns for the internal representation and textual formatting of CPU registers. This specification aims to ensure consistency, clarity, and correctness in how registers are modeled and displayed throughout the compiler.


## Public API

### Register
```rust
pub struct Register {
    name: String,
    size: u8,
    encoding: u8,
}

impl Register {
    pub fn new(name: &str, size: u8, encoding: u8) -> Self
    pub fn get_alias(&self, size: u8) -> Option<Register>
    pub fn format(&self) -> String
}
```

## API Behavior

### new(name: &str, size: u8, encoding: u8)
- **Description**: Instantiates a new `Register` object with the specified `name`, `size`, and `encoding`. The function ensures that all input parameters comply with predefined system constraints.
- **Preconditions**: 
  - `name` must be a non-empty string that adheres to the system's identifier rules.
  - `size` must be one of the following valid widths (in bits): 8, 16, 32, or 64.
  - `encoding` must correspond to an allowable register encoding index as defined by the system.
- **Postconditions**: 
  - A new `Register` instance is created and initialized with the specified attributes, making it available for further operations within the system.
- **Errors**: None

### get_alias(size: u8)
- **Description**: Retrieves the alias of a register corresponding to the specified size. If an alias of the requested size exists, it will be returned; otherwise, the function indicates the absence of such an alias.
- **Preconditions**: 
  - The parameter `size` must be one of the valid register sizes: 8, 16, 32, or 64.
- **Postconditions**:
  - Returns `Some(Register)` if an alias of the requested size exists.
  - Returns `None` if no alias corresponds to the requested size.
- **Errors**: None

### format()
- **Description**: Formats the contents of the register into an assembly language representation. This function generates a textual representation suitable for inclusion in assembly code, reflecting the current state of the register.
- **Preconditions**: None. The function can be invoked on any valid register instance without prior setup.
- **Postconditions**: Returns a string representing the current state of the register in standard assembly language syntax. The output string accurately reflects the register’s value and format.
- **Errors**: None

## Usage Examples

### Basic Usage
```rust
let reg = Register::new("rax", 64, 0);
let formatted = reg.format(); // "rax"

let eax_alias = reg.get_alias(32); // Some(Register { name: "eax", size: 32, ... })
```

## Implementation Constraints
1. Register names must adhere to the naming conventions of the target architecture, ensuring consistency and avoiding conflicts with reserved identifiers.
2. Register sizes must comply with the size and alignment specifications of the target architecture, including bit-width and memory alignment requirements.
3. Register encodings must conform to the binary encoding scheme defined by the target architecture, ensuring accurate interpretation and execution of instructions.