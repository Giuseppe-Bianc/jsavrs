# Quickstart Guide: IR to x86-64 Assembly Translator

## Overview

This guide provides a quick introduction to using the IR to x86-64 Assembly Translator module in the jsavrs compiler. The translator converts IR structures from `src/ir/` into NASM-compatible x86-64 assembly using existing `src/asm/` infrastructure.

## Prerequisites

- Rust 1.93.0 or later
- NASM assembler for assembling generated code
- jsavrs compiler with IR module available

## Installation

The translator module is built into the jsavrs compiler. No additional installation is required beyond the standard jsavrs build process:

```bash
# Clone the repository
git clone https://github.com/jsavrs/jsavrs.git
cd jsavrs

# Build the compiler with the translator module
cargo build --release
```

## Basic Usage

### Translating an IR Module

To translate an IR module to x86-64 assembly:

```rust
use jsavrs::translator::Translator;
use jsavrs::ir::Module;

// Assuming you have an IR module ready
let ir_module = /* your IR module */;
let asm_code = Translator::translate_module(&ir_module)?;

// The asm_code is a String containing NASM-compatible x86-64 assembly
println!("{}", asm_code);
```

### Command Line Interface

The translator can also be accessed through the command line:

```bash
# Translate an IR file to assembly
jsavrs translate --input my_module.ir --output my_module.asm

# Specify target ABI (defaults to platform default)
jsavrs translate --input my_module.ir --output my_module.asm --target-abi windows-x64
jsavrs translate --input my_module.ir --output my_module.asm --target-abi system-v

# Enable source mapping
jsavrs translate --input my_module.ir --output my_module.asm --emit-mapping
```

## Configuration Options

### Target ABI Selection

The translator supports two major x86-64 ABIs:

- `system-v`: System V AMD64 ABI (used on Linux/macOS)
- `windows-x64`: Windows x64 ABI (used on Windows)

Select the target ABI using the `--target-abi` flag or configure it programmatically:

```rust
use jsavrs::translator::{Translator, TranslationConfig, AbiType};

let config = TranslationConfig {
    target_abi: AbiType::SystemV,  // or AbiType::Windows64
    emit_mapping: false,
    debug_symbols: true,
};
let mut translator = Translator::new(config);
let asm_code = translator.translate_module(&ir_module)?;
```

### Source Mapping

Enable source mapping to generate a `.map` file that connects IR nodes to assembly lines:

```bash
jsavrs translate --input my_module.ir --output my_module.asm --emit-mapping
```

This creates a `my_module.map` file with the format "IR_LINE:COL → ASM_LINE:LABEL".

## Example Translation

### Sample IR Structure

Consider a simple IR function:

```rust
// This represents an IR function equivalent to:
// int add(int a, int b) { return a + b; }

let ir_func = IrFunction {
    name: "add".to_string(),
    parameters: vec![
        IrParameter { name: "a".to_string(), param_type: IrType::Int32 },
        IrParameter { name: "b".to_string(), param_type: IrType::Int32 },
    ],
    return_type: IrType::Int32,
    basic_blocks: vec![
        BasicBlock {
            id: BasicBlockId(0),
            instructions: vec![
                Instruction {
                    id: InstructionId(0),
                    kind: InstructionKind::BinaryOp(BinaryOp::Add),
                    operands: vec![Value::Param(0), Value::Param(1)],
                }
            ],
            terminator: Terminator::Return(Some(Value::Instruction(0))),
        }
    ],
};
```

### Generated Assembly (System V ABI)

The translator would generate assembly similar to:

```nasm
section .text
global add
add:
    ; Function prologue
    push rbp
    mov rbp, rsp
    
    ; Perform addition: a + b
    ; In System V ABI: a is in edi, b is in rsi
    mov eax, edi    ; Move first parameter to eax
    add eax, esi    ; Add second parameter to eax
    
    ; Function epilogue
    pop rbp
    ret
```

### Generated Assembly (Windows x64 ABI)

With Windows ABI, it would generate:

```nasm
section .text
global add
add:
    ; Function prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32     ; Windows shadow space
    
    ; Perform addition: a + b
    ; In Windows x64 ABI: a is in ecx, b is in edx
    mov eax, ecx    ; Move first parameter to eax
    add eax, edx    ; Add second parameter to eax
    
    ; Function epilogue
    add rsp, 32     ; Restore shadow space
    pop rbp
    ret
```

## Error Handling

The translator uses comprehensive error handling:

```rust
use jsavrs::translator::Translator;

match Translator::translate_module(&ir_module) {
    Ok(assembly_code) => {
        println!("Translation successful!");
        println!("{}", assembly_code);
    }
    Err(translation_error) => {
        eprintln!("Translation failed: {}", translation_error.message);
        if let Some(location) = &translation_error.ir_location {
            eprintln!("Error at IR location: {}:{}", location.line, location.column);
        }
    }
}
```

## Performance Tips

- The translator targets <100ms per function with <1GB memory usage
- For large modules, consider processing functions individually
- Use release builds for optimal translation performance

## Integration with Existing Codebase

The translator integrates seamlessly with existing jsavrs components:

```rust
use jsavrs::ir::Module;
use jsavrs::translator::Translator;
use jsavrs::asm::AssemblyFile;

// After IR generation in your compilation pipeline
let ir_module = /* your IR from previous compilation phases */;

// Translate to assembly
let asm_text = Translator::translate_module(&ir_module)?;

// Optionally parse into AssemblyFile for further processing
let mut asm_file = AssemblyFile::parse(&asm_text)?;
// Further processing can happen here
```

## Testing

Run the translator tests to verify functionality:

```bash
# Run all translator tests
cargo test -p jsavrs translator

# Run translator benchmarks
cargo bench -p jsavrs jsavrs_benchmark

# Update assembly snapshots
cargo insta review
```

## Troubleshooting

- If translation fails with unsupported IR constructs, the translator will emit a clear diagnostic error with location information
- If assembly doesn't assemble with NASM, check that the target ABI matches your intended platform
- For performance issues, use the benchmarking tools to identify bottlenecks
