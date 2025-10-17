# Quickstart Guide: x86-64 NASM Assembly Code Generator

**Feature**: 007-x86-64-asm-generator  
**Audience**: jsavrs compiler developers  
**Last Updated**: 2025-10-17

## Overview

The x86-64 NASM assembly code generator translates jsavrs Intermediate Representation (IR) into executable assembly code. This guide provides practical examples for integrating and using the generator in the compiler pipeline.

## Prerequisites

- Rust 2024 Edition (1.85+)
- Familiarity with jsavrs IR structure (`src/ir/`)
- Basic understanding of x86-64 assembly and calling conventions
- NASM assembler for testing generated code

## Quick Start (5 minutes)

### 1. Generate Assembly from IR

```rust
use jsavrs::asm::generator::AsmGenerator;
use jsavrs::ir::{Module, TargetTriple};

// Assuming you have an IR module from the frontend
let ir_module: Module = /* ... from parser/semantic analysis ... */;

// Create generator for target platform
let mut generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);

// Generate assembly
let result = generator.generate(&ir_module);

// Handle results
if let Some(assembly_code) = result.assembly {
    // Write to file
    std::fs::write("output.asm", assembly_code)?;
    println!("✓ Generated assembly: output.asm");
    println!("  Functions: {}", result.stats.functions_generated);
    println!("  Instructions: {}", result.stats.assembly_instructions);
} else {
    eprintln!("✗ Generation failed:");
    for error in result.errors {
        eprintln!("  {}", error);
    }
}
```

### 2. Assemble and Link (Linux Example)

```bash
# Assemble with NASM
nasm -f elf64 output.asm -o output.o

# Link with GCC (handles C runtime setup)
gcc output.o -o program

# Run
./program
echo $?  # Print exit code
```

### 3. Windows Example

```bash
# Assemble with NASM
nasm -f win64 output.asm -o output.obj

# Link with MSVC linker
link output.obj /OUT:program.exe /SUBSYSTEM:CONSOLE

# Run
program.exe
echo %ERRORLEVEL%
```

## Common Integration Patterns

### Pattern 1: Compiler Pipeline Integration

```rust
use jsavrs::{
    lexer::Lexer,
    parser::Parser,
    semantic::SemanticAnalyzer,
    ir::generator::NirGenerator,
    asm::generator::AsmGenerator,
};

/// Complete compilation pipeline: source → assembly
pub fn compile_to_assembly(
    source_code: &str,
    target: TargetTriple,
) -> Result<String, CompilerError> {
    // 1. Lex
    let lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize()?;
    
    // 2. Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // 3. Semantic analysis
    let mut analyzer = SemanticAnalyzer::new();
    let checked_ast = analyzer.analyze(&ast)?;
    
    // 4. Generate IR
    let mut ir_gen = NirGenerator::new();
    let ir_module = ir_gen.generate(&checked_ast)?;
    
    // 5. Generate assembly
    let mut asm_gen = AsmGenerator::new(target);
    let result = asm_gen.generate(&ir_module);
    
    if !result.errors.is_empty() {
        return Err(CompilerError::CodeGenErrors(result.errors));
    }
    
    Ok(result.assembly.unwrap())
}
```

### Pattern 2: Cross-Platform Build

```rust
/// Generate assembly for all supported platforms
pub fn build_cross_platform(
    ir_module: &Module,
    output_dir: &Path,
) -> Result<(), std::io::Error> {
    let platforms = vec![
        (TargetTriple::X86_64UnknownLinuxGnu, "linux"),
        (TargetTriple::X86_64PcWindowsMsvc, "windows"),
        (TargetTriple::X86_64AppleDarwin, "macos"),
    ];
    
    for (target, name) in platforms {
        let mut generator = AsmGenerator::new(target);
        let result = generator.generate(ir_module);
        
        if let Some(asm) = result.assembly {
            let filename = output_dir.join(format!("output_{}.asm", name));
            std::fs::write(&filename, asm)?;
            println!("✓ Generated {} assembly", name);
        } else {
            eprintln!("✗ Failed to generate {} assembly", name);
            for error in result.errors {
                eprintln!("  {}", error);
            }
        }
    }
    
    Ok(())
}
```

### Pattern 3: Error Recovery

```rust
/// Generate assembly with detailed error reporting
pub fn compile_with_diagnostics(
    ir_module: &Module,
    target: TargetTriple,
) -> CodeGenResult {
    let mut generator = AsmGenerator::new(target);
    let result = generator.generate(ir_module);
    
    // Categorize errors for better reporting
    for error in &result.errors {
        match error {
            CodeGenError::UnsupportedType { ty, location } => {
                eprintln!("ERROR: Unsupported type '{}' at {}", ty, location);
                eprintln!("  Hint: Only I8-I64, U8-U64, F32, F64, Bool, Char, Pointer, Void are supported");
            }
            CodeGenError::RegisterAllocationFailed(msg) => {
                eprintln!("ERROR: Register allocation failed: {}", msg);
                eprintln!("  Hint: Function may have too many live values (>20)");
            }
            CodeGenError::MalformedInstruction { reason, location } => {
                eprintln!("ERROR: Malformed IR at {}: {}", location, reason);
                eprintln!("  Hint: This indicates a bug in the IR generator");
            }
            _ => eprintln!("ERROR: {}", error),
        }
    }
    
    // Print statistics even on partial success
    println!("\nStatistics:");
    println!("  Functions generated: {}/{}", 
        result.stats.functions_generated,
        result.stats.functions_generated + result.stats.functions_failed);
    println!("  IR instructions: {}", result.stats.instructions_translated);
    println!("  Assembly instructions: {}", result.stats.assembly_instructions);
    println!("  Register spills: {}", result.stats.register_spills);
    
    result
}
```

## Testing Generated Assembly

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    
    #[test]
    fn test_simple_arithmetic() {
        // Create IR manually or parse from text
        let ir = r#"
            define i32 @add(i32 %a, i32 %b) {
            entry:
                %result = add i32 %a, %b
                ret i32 %result
            }
        "#;
        
        let module = parse_ir_text(ir).unwrap();
        let mut generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);
        let result = generator.generate(&module);
        
        assert!(result.errors.is_empty(), "Should generate without errors");
        assert_snapshot!(result.assembly.unwrap());
    }
    
    #[test]
    fn test_windows_calling_convention() {
        let ir = r#"
            declare void @external_func(i32, i32)
            
            define void @caller() {
            entry:
                call void @external_func(i32 42, i32 58)
                ret void
            }
        "#;
        
        let module = parse_ir_text(ir).unwrap();
        let mut generator = AsmGenerator::new(TargetTriple::X86_64PcWindowsMsvc);
        let result = generator.generate(&module);
        
        let asm = result.assembly.unwrap();
        
        // Verify Windows x64 calling convention
        assert!(asm.contains("mov ecx, 42"), "First param in RCX");
        assert!(asm.contains("mov edx, 58"), "Second param in RDX");
        assert!(asm.contains("sub rsp, 32"), "Shadow space allocated");
    }
}
```

### Integration Test with NASM

```rust
#[test]
#[ignore] // Requires NASM installed
fn test_assemble_and_execute() {
    let ir = r#"
        define i32 @main() {
        entry:
            ret i32 42
        }
    "#;
    
    let module = parse_ir_text(ir).unwrap();
    let mut generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);
    let result = generator.generate(&module);
    
    // Write assembly
    let asm_path = "/tmp/test.asm";
    std::fs::write(asm_path, result.assembly.unwrap()).unwrap();
    
    // Assemble
    let output = std::process::Command::new("nasm")
        .args(&["-f", "elf64", asm_path, "-o", "/tmp/test.o"])
        .output()
        .expect("Failed to run NASM");
    assert!(output.status.success(), "NASM should succeed");
    
    // Link
    let output = std::process::Command::new("gcc")
        .args(&["/tmp/test.o", "-o", "/tmp/test"])
        .output()
        .expect("Failed to link");
    assert!(output.status.success(), "Linking should succeed");
    
    // Execute
    let output = std::process::Command::new("/tmp/test")
        .output()
        .expect("Failed to execute");
    assert_eq!(output.status.code(), Some(42), "Should return 42");
}
```

## Debugging Tips

### 1. Enable Verbose Assembly Comments

The generator automatically includes IR instruction comments in the assembly output:

```nasm
; IR: %t0 = add i32 %a, %b
mov eax, edi
add eax, esi
```

This helps correlate assembly instructions with IR operations for debugging.

### 2. Inspect Register Allocation

```rust
let result = generator.generate(&ir_module);

println!("Register allocation stats:");
println!("  Spills: {}", result.stats.register_spills);
println!("  Stack size: {} bytes", result.stats.total_stack_size);

// High spill count may indicate:
// - Functions with many live values (>16 for x86-64 GP registers)
// - Deeply nested expressions
// - Consider refactoring IR or improving allocation algorithm
```

### 3. Compare Platform Outputs

```rust
let targets = vec![
    TargetTriple::X86_64UnknownLinuxGnu,
    TargetTriple::X86_64PcWindowsMsvc,
];

for target in targets {
    let mut gen = AsmGenerator::new(target);
    let result = gen.generate(&ir_module);
    println!("\n=== {} ===", target);
    println!("{}", result.assembly.unwrap());
}

// Compare calling conventions, stack layouts, symbol mangling
```

## Performance Optimization

### Measure Generation Time

```rust
use std::time::Instant;

let start = Instant::now();
let result = generator.generate(&ir_module);
let duration = start.elapsed();

println!("Code generation took {:?}", duration);
println!("Throughput: {:.2} instructions/ms",
    result.stats.instructions_translated as f64 / duration.as_millis() as f64);
```

### Expected Performance

- **Target**: <1 second per 1000 IR instructions (best-effort)
- **Typical**: 0.5-2.0 ms per function (10-100 basic blocks)
- **Bottlenecks**: Liveness analysis (O(n*e)), register allocation (O(n))

## Common Issues and Solutions

### Issue: "Unsupported type i128"

**Cause**: IR contains 128-bit integer types

**Solution**: jsavrs only supports I8-I64, U8-U64. Modify frontend to use smaller types or arrays.

### Issue: "Register allocation failed"

**Cause**: Function has >20 live values simultaneously

**Solutions**:
1. Refactor source code to reduce temporary variables
2. Enable register spilling (automatically done)
3. Check if IR generator is creating unnecessary temporaries

### Issue: "CFG verification failed: missing terminator"

**Cause**: IR basic block lacks terminator instruction

**Solution**: Bug in IR generator - ensure all blocks end with terminator (ret, br, etc.)

### Issue: Assembly won't assemble with NASM

**Cause**: Invalid NASM syntax or incorrect target format

**Solutions**:
1. Verify target triple matches NASM format (`-f elf64` for Linux, `-f win64` for Windows)
2. Check for syntax errors in instruction formatting
3. Report bug with minimal IR example

## Next Steps

- **Read**: [research.md](./research.md) for algorithm details
- **Study**: [data-model.md](./data-model.md) for internal data structures
- **Review**: [contracts/](./contracts/) for API specifications
- **Contribute**: See AGENTS.md for development workflow

---

**Guide Version**: 1.0.0  
**Last Updated**: 2025-10-17
