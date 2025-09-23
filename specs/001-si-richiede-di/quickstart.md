# Quickstart Guide: ASM Generation Components

## Purpose
This guide offers a concise overview of the enhanced assembly generation modules within the jsavrs compiler. It presents the fundamental usage patterns and illustrates the process of producing assembly code from intermediate representations (IRs), which are compiler-generated abstract code structures. The guide aims to assist developers in understanding and effectively utilizing these components for efficient code generation.


## Prerequisites

Before proceeding, ensure that you meet the following technical prerequisites:

- A foundational understanding of x86-64 assembly language, which is essential for low-level code analysis, optimization, and compiler development.
- Familiarity with the Jsavrs compiler architecture, including its core components and operational workflow, to facilitate efficient compilation and debugging of programs.
- A properly configured Rust development environment, including the Rust toolchain and any necessary libraries, to enable the development, compilation, and testing of Rust-based projects.


## Basic Usage

### 1. Creating an ASM Generator
To begin generating assembly code, create a new ASM generator instance:

```rust
use jsavrs::asm::generator::ASMGenerator;

let mut generator = ASMGenerator::new();
```

### 2. Adding Sections
Add sections to organize your assembly code:

```rust
generator.add_section(".text");
generator.switch_section(".text");
```

### 3. Adding Instructions
Create and add instructions to the current section:

```rust
use jsavrs::asm::instruction::Instruction;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

let mut mov_inst = Instruction::new("mov");
mov_inst.add_operand(Operand::Register(Register::new("rax", 64, 0)));
mov_inst.add_operand(Operand::Immediate(42));

generator.add_instruction(mov_inst);
```

### 4. Adding Labels
Add labels for control flow:

```rust
generator.add_label("start");
```

### 5. Generating Assembly Code
Generate the final assembly code as a string:

```rust
let assembly_code = generator.generate();
println!("{}", assembly_code);
```

## Example: Simple Function
Here's a complete example that generates a simple function:

```rust
use jsavrs::asm::generator::ASMGenerator;
use jsavrs::asm::instruction::Instruction;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

fn generate_simple_function() -> String {
    let mut generator = ASMGenerator::new();
    
    // Add text section
    generator.add_section(".text");
    generator.switch_section(".text");
    
    // Add function label
    generator.add_label("simple_func");
    
    // Move immediate value to register
    let mut mov_inst = Instruction::new("mov");
    mov_inst.add_operand(Operand::Register(Register::new("rax", 64, 0)));
    mov_inst.add_operand(Operand::Immediate(42));
    generator.add_instruction(mov_inst);
    
    // Return instruction
    let ret_inst = Instruction::new("ret");
    generator.add_instruction(ret_inst);
    
    generator.generate()
}
```

## Advanced Usage

### Working with Memory Operands
Create memory operands for more complex instructions:

```rust
use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

// Create a memory operand: [rbp - 8]
let memory_operand = Operand::Memory(
    Some(Register::new("rbp", 64, 5)),  // base register
    None,                               // no index register
    1,                                  // scale factor
    -8                                  // displacement
);
```

### Targeting Different Operating Systems
Specify the target OS for OS-specific code generation:

```rust
use jsavrs::asm::generator::ASMGenerator;
use jsavrs::asm::target_os::TargetOS;

let mut generator = ASMGenerator::new();
generator.set_target_os(TargetOS::Linux);  // or Windows, MacOS
```

## Testing Your Code

### Unit Tests
Write unit tests for your ASM generation code:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_mov_instruction() {
        let assembly = generate_simple_function();
        assert!(assembly.contains("mov rax, 42"));
        assert!(assembly.contains("ret"));
    }
}
```

### Snapshot Tests
Use insta for snapshot testing of generated assembly:

```rust
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use insta::assert_snapshot;
    
    #[test]
    fn test_simple_function_snapshot() {
        let assembly = generate_simple_function();
        assert_snapshot!(assembly);
    }
}
```

## Best Practices

1. **Modularity**: Structure your code into distinct, cohesive modules, each encapsulating a specific function or data segment, to enhance readability and maintainability.

2. **Documentation**: Include detailed comments that clarify complex assembly sequences and document the rationale behind design decisions, ensuring the code is understandable and maintainable by others.

3. **Testing**: Develop and execute thorough tests for all generated assembly code, including unit and integration tests, to verify correctness and prevent potential errors.

4. **Validation**: Utilize built-in validation tools to verify instruction syntax and operand compatibility, ensuring correctness and portability across platforms.

5. **Extensibility**: Design code with extensibility in mind, enabling straightforward integration of new instructions, additional target platforms, and future enhancements.

## Troubleshooting

### Common Issues

1. **Invalid Instructions**: Ensure that all instructions utilize valid mnemonics and operands compatible in both type and size. Incorrect or unsupported instructions may result in compilation errors or undefined behavior.

2. **Register Mismatches**: Verify that the sizes of registers align with the operational requirements of the corresponding instructions. Using mismatched registers can lead to runtime errors or incorrect computations.

3. **Section Management**: Ensure that the correct section is selected before adding instructions. Proper section management is essential for organizing code and ensuring correct assembly compilation.

### Getting Help
- Refer to the API documentation for comprehensive information on each component, including supported instructions and operand types.
- Examine the examples provided in the tests directory to understand practical applications and correct usage patterns.
- Consult the JSAVRS User Manual for detailed guidance on compiler configuration, advanced features, and troubleshooting techniques.

## Next Steps
- Examine the advanced capabilities of the ASM generator, including feature modules, code generation techniques, and customization options.
- Develop a thorough understanding of defining custom instructions, emphasizing their integration and impact on ASM generation.
- Master techniques for extending the register and operand systems to enhance flexibility and support complex instruction sets.
- Analyze the performance optimization guidelines to improve efficiency, scalability, and execution speed of ASM-generated code.
