# Assembly Generator Quick Start Guide

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 26 settembre 2025  
**Design Phase**: Phase 1 - Quick Start Implementation

## Overview

This guide provides step-by-step instructions for implementing and testing the assembly code generator for the jsavrs compiler framework.

## Prerequisites

### Development Environment Setup

1. **Rust Toolchain**:
   ```bash
   rustup update
   rustc --version  # Should be 1.75 or later
   cargo --version
   ```

2. **Required Dependencies** (add to `Cargo.toml`):
   ```toml
   [dependencies]
   iced-x86 = "1.21.0"
   thiserror = "1.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   
   [dev-dependencies]
   insta = "1.34"
   criterion = "0.5"
   ```

3. **NASM Assembler** (for validation testing):
   ```bash
   # Ubuntu/Debian
   sudo apt-get install nasm
   
   # macOS
   brew install nasm
   
   # Windows (via chocolatey)
   choco install nasm
   ```

## Implementation Steps

### Step 1: Core Module Structure Creation

Create the assembly generator module structure:

```bash
mkdir -p src/asm/x86_64/abi
mkdir -p src/asm/optimization
mkdir -p src/asm/debug
mkdir -p tests/asm_fixtures
```

**Files to create**:
1. `src/asm/mod.rs` - Module root with public interfaces
2. `src/asm/generator.rs` - Main AssemblyGenerator implementation
3. `src/asm/x86_64/mod.rs` - x86-64 specific module
4. `src/asm/x86_64/instruction_mapper.rs` - IR to x86-64 mapping
5. `src/asm/x86_64/register_alloc.rs` - Register allocation logic

### Step 2: Basic AssemblyGenerator Implementation

**Implementation Priority Order**:
1. Basic IR parsing and module loading
2. Simple instruction mapping (arithmetic operations first)
3. Minimal register allocation (stack-based fallback)
4. NASM output generation
5. Error handling integration

**First Milestone Test**:
```rust
#[test]
fn test_basic_arithmetic_generation() {
    let ir_module = create_simple_addition_ir(); // Add 1 + 2
    let generator = AssemblyGenerator::new(TargetConfiguration::default());
    
    let result = generator.generate_assembly(vec![ir_module]).unwrap();
    
    assert!(result.assembly_files.len() == 1);
    assert!(result.assembly_files[0].content.contains("add"));
    assert!(validate_nasm_syntax(&result.assembly_files[0].content));
}
```

### Step 3: Instruction Mapping Implementation

**Implementation Order**:
1. **Arithmetic Operations**: `add`, `sub`, `mul`, `div` for integers
2. **Memory Operations**: `mov` for loads/stores
3. **Control Flow**: `jmp`, `je`, `jne` for basic branching
4. **Function Calls**: `call`, `ret` with basic ABI support

**Test Pattern for Each Operation**:
```rust
#[test]
fn test_add_instruction_mapping() {
    let mapper = X86_64InstructionMapper::new();
    let ir_add = ArithmeticOperation {
        operation_type: ArithmeticType::Add,
        operands: vec![IRValue::Integer(5), IRValue::Integer(3)],
        result_type: IRType::I32,
        source_location: None,
    };
    
    let x86_instructions = mapper.map_arithmetic(ir_add).unwrap();
    
    assert!(x86_instructions.len() > 0);
    assert!(x86_instructions[0].mnemonic == "add");
}
```

### Step 4: Register Allocation Implementation

**Implementation Phases**:
1. **Simple Stack Allocation**: All variables on stack (proof of concept)
2. **Basic Register Allocation**: Use available registers, spill when full
3. **Linear Scan Algorithm**: Efficient allocation with live range analysis
4. **Optimization**: Coalescing and smart spilling

**Test Strategy**:
```rust
#[test]
fn test_register_allocation_simple() {
    let mut allocator = LinearScanAllocator::new();
    let live_ranges = create_test_live_ranges();
    let calling_convention = SystemVABI::new();
    
    let allocation = allocator.allocate_registers(&live_ranges, &calling_convention).unwrap();
    
    assert!(allocation.register_assignments.len() > 0);
    assert!(validate_calling_convention_compliance(&allocation, &calling_convention));
}
```

### Step 5: Error Handling Integration

**Implementation Components**:
1. Error classification and severity assignment
2. Stub code generation for unsupported IR constructs
3. JSON diagnostic output
4. Recovery strategies for common failure modes

**Test Coverage**:
```rust
#[test]
fn test_unsupported_ir_handling() {
    let generator = AssemblyGenerator::new(TargetConfiguration::default());
    let unsupported_ir = create_vector_operation_ir(); // Not yet supported
    
    let result = generator.generate_assembly(vec![unsupported_ir]).unwrap();
    
    assert!(result.errors.len() > 0);
    assert!(result.assembly_files[0].content.contains("TODO: Unsupported IR construct"));
}
```

## Testing and Validation

### Unit Test Strategy

**Test Organization**:
```
tests/
├── asm_tests.rs              # Integration tests
├── asm_snapshot_tests.rs     # Insta snapshot validation
├── asm_instruction_tests.rs  # Individual instruction mapping
├── asm_register_tests.rs     # Register allocation validation
└── asm_error_tests.rs        # Error handling verification
```

**Snapshot Testing Setup**:
```rust
use insta::assert_snapshot;

#[test]
fn test_function_generation_snapshot() {
    let ir_function = create_fibonacci_function_ir();
    let generator = AssemblyGenerator::new(TargetConfiguration::default());
    
    let result = generator.generate_assembly(vec![ir_function]).unwrap();
    
    assert_snapshot!(result.assembly_files[0].content);
}
```

### Integration Testing

**NASM Validation Pipeline**:
```rust
fn validate_nasm_syntax(assembly_code: &str) -> bool {
    let temp_file = write_temp_file(assembly_code);
    let nasm_output = Command::new("nasm")
        .args(["-f", "elf64", temp_file.path()])
        .output()
        .expect("Failed to run NASM");
    
    nasm_output.status.success()
}
```

**End-to-End Test Example**:
```rust
#[test]
fn test_complete_program_generation() {
    // 1. Create IR for simple program (hello world or factorial)
    let ir_modules = create_hello_world_ir();
    
    // 2. Generate assembly
    let generator = AssemblyGenerator::new(TargetConfiguration::default());
    let result = generator.generate_assembly(ir_modules).unwrap();
    
    // 3. Validate NASM compatibility
    for assembly_file in &result.assembly_files {
        assert!(validate_nasm_syntax(&assembly_file.content));
    }
    
    // 4. Check for expected outputs
    assert!(result.errors.is_empty());
    assert!(result.assembly_files.len() > 0);
}
```

### Performance Benchmarking

**Benchmark Setup**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_assembly_generation(c: &mut Criterion) {
    let large_ir_module = create_large_ir_module(); // 10k+ instructions
    
    c.bench_function("assembly_generation_10k", |b| {
        b.iter(|| {
            let generator = AssemblyGenerator::new(TargetConfiguration::default());
            black_box(generator.generate_assembly(vec![large_ir_module.clone()]))
        })
    });
}
```

## Common Issues and Solutions

### Issue 1: NASM Syntax Errors
**Problem**: Generated assembly doesn't assemble with NASM
**Solution**: 
1. Validate instruction syntax against NASM documentation
2. Check operand ordering and size specifiers
3. Ensure proper section declarations (`.text`, `.data`, `.bss`)

### Issue 2: Register Allocation Failures
**Problem**: Unable to allocate registers for complex functions
**Solution**:
1. Implement stack spilling mechanism
2. Verify calling convention compliance
3. Add register pressure detection and fallback strategies

### Issue 3: Debug Information Generation Overhead
**Problem**: Debug information slows compilation significantly
**Solution**:
1. Profile debug information generation passes
2. Implement lazy debug symbol creation
3. Add configurable debug level selection

## Success Criteria Validation

### Functional Validation
- [ ] Generated assembly assembles successfully with NASM
- [ ] Assembly execution produces correct results for test programs
- [ ] All supported IR constructs generate appropriate x86-64 instructions
- [ ] Error handling produces useful diagnostics for unsupported features

### Performance Validation
- [ ] Assembly generation completes within 2x IR processing time
- [ ] Debug information overhead stays within specified limits
- [ ] Memory usage remains reasonable for large programs

### Quality Validation
- [ ] All tests pass consistently across platforms (Windows, macOS, Linux)
- [ ] Snapshot tests catch regressions in generated code
- [ ] Error messages provide actionable guidance for developers

## Development Workflow

1. **Implement Feature**: Add new functionality following TDD principles
2. **Run Unit Tests**: `cargo test asm::`
3. **Update Snapshots**: `cargo insta review` (when output changes are expected)
4. **Run Integration Tests**: Full NASM validation pipeline
5. **Performance Check**: `cargo bench` to ensure no regressions
6. **Code Review**: Follow jsavrs community guidelines

This quickstart guide provides a structured approach to implementing the assembly generator while maintaining quality and reliability standards throughout the development process.