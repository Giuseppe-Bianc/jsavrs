# Quickstart: x86-64 Assembly Code Generator

## Overview
This guide demonstrates how to use the x86-64 Assembly Code Generator to translate intermediate representation (IR) into NASM-compatible assembly code.

## Prerequisites
- Rust 1.75 or higher
- NASM assembler installed on your system
- Existing jsavrs IR module with valid code to translate

## Basic Usage

### 1. Initialize the Assembly Generator
```rust
use jsavrs::ir::module::IRModule;
use jsavrs::code_generator::assembly_generator::AssemblyGenerator;
use jsavrs::code_generator::target_platform::TargetPlatform;

// Load your IR module
let ir_module = IRModule::from_source(/* your source */);

// Create an assembly generator for your target platform
let mut generator = AssemblyGenerator::new(
    ir_module,
    TargetPlatform::LinuxX86_64  // or WindowsX86_64, MacOSX86_64
);
```

### 2. Generate Assembly Code
```rust
// Generate the assembly code
let assembly_code = generator.generate()?;

// The generated assembly follows NASM syntax and is ready for assembly
println!("{}", assembly_code);
```

### 3. Assemble and Link
```bash
# Save the generated assembly to a file
echo "$ASSEMBLY_CODE" > output.asm

# Assemble with NASM
nasm -f elf64 output.asm -o output.o  # Use -f win64 for Windows

# Link to create executable
ld output.o -o output_program  # Use appropriate linker for your platform
```

## Example Translation

### Input IR (simplified representation):
```
function add_numbers(a: i32, b: i32) -> i32 {
  result = a + b
  return result
}
```

### Generated NASM Assembly (Linux x86-64):
```nasm
section .text
global add_numbers

add_numbers:
    ; Function prologue
    push rbp
    mov rbp, rsp
    
    ; Perform addition: rax = rdi + rsi (first two parameters)
    mov eax, edi
    add eax, esi
    
    ; Function epilogue
    mov rsp, rbp
    pop rbp
    ret

section .data

section .bss
```

## Advanced Usage

### Custom Register Allocation
```rust
use jsavrs::code_generator::register_allocator::RoundRobinAllocator;

// Use a specific register allocation strategy
let allocator = RoundRobinAllocator::new();
generator.set_register_allocator(allocator);
```

### Platform-Specific Calling Conventions
```rust
use jsavrs::code_generator::calling_convention::{WindowsX64, SystemV};

// The generator automatically uses the correct calling convention
// based on the target platform, but you can override if needed
generator.set_calling_convention(Box::new(WindowsX64::default()));
```

## Testing Your Generated Assembly

### 1. Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let ir_module = create_test_ir_for_addition();
        let mut generator = AssemblyGenerator::new(ir_module, TargetPlatform::LinuxX86_64);
        let assembly = generator.generate().unwrap();
        
        // Verify the assembly contains expected instructions
        assert!(assembly.contains("add"));
        assert!(assembly.contains("ret"));
    }
}
```

### 2. Integration Testing
```rust
// Test the complete pipeline: IR → Assembly → Executable → Output Verification
// This tests semantic equivalence between the original IR and generated assembly
```

## Error Handling

The assembly generator may return the following errors:

- `UnsupportedIRError`: When the IR contains constructs not yet supported by the generator
- `RegisterAllocationError`: When register allocation fails (e.g., too many live variables)
- `ABIComplianceError`: When the generated code doesn't comply with the target platform ABI

```rust
match generator.generate() {
    Ok(assembly) => {
        // Assembly generation successful
        println!("{}", assembly);
    }
    Err(AssemblyGenerationError::UnsupportedIR { ir_location, description }) => {
        eprintln!("Unsupported IR at {}: {}", ir_location, description);
    }
    Err(AssemblyGenerationError::RegisterAllocation { description }) => {
        eprintln!("Register allocation failed: {}", description);
    }
    Err(AssemblyGenerationError::ABICompliance { platform, description }) => {
        eprintln!("ABI compliance error on {}: {}", platform, description);
    }
}
```

## Performance Tips

- The generator is designed to handle modules with up to 10,000 IR instructions in under 5 seconds
- Memory usage is optimized to stay within 2x the size of the input IR
- For best performance, ensure your IR is properly optimized before passing to the generator

## Validation

To verify that your generated assembly maintains semantic equivalence with the original IR:

1. Run the assembly generation on a test IR module
2. Verify the output assembly code follows NASM syntax
3. Assemble and execute the code with test inputs
4. Compare the results with expected outputs from the original IR logic