# Contract: Target OS

## Overview
This document specifies the interface contract for the TargetOS component of the JSAVRS compiler's assembly (ASM) generation module. It details the public API, expected behavior, and recommended usage patterns for abstracting and managing target operating systems during the process of assembly code generation. The document aims to provide a clear and comprehensive reference for developers, ensuring consistent implementation and interaction with the TargetOS component across different modules and target platforms.


## Public API

### TargetOS
```rust
pub enum TargetOS {
    Linux,
    Windows,
    MacOS,
}

impl TargetOS {
    pub fn get_calling_convention(&self) -> CallingConvention
    pub fn get_system_calls(&self) -> SystemCalls
}
```

## API Behavior

### Linux
- **Description**: Represents the Linux operating system
- **Preconditions**: None
- **Postconditions**: None
- **Errors**: None

### Windows
- **Description**: Represents the Windows operating system
- **Preconditions**: None
- **Postconditions**: None
- **Errors**: None

### MacOS
- **Description**: Represents the macOS operating system
- **Preconditions**: None
- **Postconditions**: None
- **Errors**: None

### get_calling_convention()
- **Description**: Retrieves the calling convention for the target operating system. This function determines the method by which arguments are passed to functions and how return values are handled, depending on the OS.
- **Preconditions**: None
- **Postconditions**:
  - Returns a string or enumeration representing the calling convention used by the target operating system.
- **Errors**: None

### get_system_calls()
- **Description**: Retrieves a dictionary containing the system call names and their corresponding numeric identifiers for the specified operating system. This function provides detailed insight into the OS-level API calls available for system programming and analysis.
- **Preconditions**: None
- **Postconditions**:
  - Returns a dictionary where each key represents a system call name and each value corresponds to its numeric identifier.  
  - The output is specific to the target operating system.
- **Errors**: None

## Usage Examples

### Basic Usage
```rust
let target_os = TargetOS::Linux;
let calling_convention = target_os.get_calling_convention();
let system_calls = target_os.get_system_calls();
```

## Implementation Constraints
1. Calling conventions must strictly adhere to the specifications of the target operating system to ensure correct function invocation and parameter passing.
2. System call numbers and interfaces must accurately correspond to those defined by the target operating system to guarantee proper interaction with kernel-level services.
3. The generated assembly code must be fully compatible with the architecture and conventions of the target operating system, ensuring correct execution and maintainability.
