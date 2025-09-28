# Research Document: x86-64 Assembly Code Generator

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 2025-09-28  
**Status**: Phase 0 Research Complete

## Executive Summary

This research document provides comprehensive analysis for implementing an x86-64 assembly code generator that translates jsavrs intermediate representation (IR) into NASM-compatible assembly code. The research covers technology choices, architectural patterns, performance considerations, and integration strategies to ensure a robust, maintainable, and high-performance solution.

## Technology Research and Decisions

### 1. Instruction Encoding Library Selection

**Decision**: iced-x86 library  
**Rationale**: 
- Mature Rust library specifically designed for x86/x64 instruction encoding and decoding
- Provides comprehensive coverage of x86-64 instruction set including SSE/AVX extensions
- Type-safe API that prevents invalid instruction generation
- Active development with regular updates for new instruction sets
- Zero-cost abstractions with compile-time instruction validation
- Excellent documentation and community support
- Already proven in production compiler environments

**Alternatives Considered**:
- **Manual instruction encoding**: Rejected due to complexity, error-prone nature, and maintenance overhead
- **LLVM backend**: Rejected for being overly complex for our specific use case and introducing heavy dependencies
- **Capstone Engine**: Primarily a disassembler; encoding capabilities are limited
- **Keystone Engine**: C library requiring unsafe bindings, less ergonomic than native Rust solution

**Implementation Approach**:
```rust
// Example iced-x86 integration pattern
use iced_x86::code_asm::*;

pub struct InstructionEncoder {
    assembler: CodeAssembler,
}

impl InstructionEncoder {
    pub fn encode_add(&mut self, dest: Register64, src: Register64) -> Result<(), IcedError> {
        self.assembler.add(dest, src)
    }
}
```

### 2. Register Management Architecture

**Decision**: Enum-based register system with compile-time validation  
**Rationale**:
- Rust enums provide compile-time exhaustiveness checking
- Eliminates entire class of register allocation bugs through type safety
- Clear mapping between IR virtual registers and x86-64 physical registers
- Enables efficient pattern matching for instruction selection
- Supports future extensions for SSE/AVX register classes

**Design Pattern**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register64 {
    RAX, RBX, RCX, RDX, RSI, RDI, RSP, RBP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterClass {
    General(Register64),
    XMM(XMMRegister),
    YMM(YMMRegister),
}
```

**Integration with iced-x86**:
- Direct mapping from enum variants to iced-x86 register constants
- Compile-time verification of register-instruction compatibility
- Type-safe operand construction

### 3. Calling Convention Implementation

**Decision**: Trait-based extensible calling convention system  
**Rationale**:
- Supports both Windows x64 ABI and System V ABI (Linux/macOS)
- Extensible design allows future calling convention additions
- Clear separation of concerns between ABI details and code generation logic
- Testable in isolation from main code generation logic

**Architecture**:
```rust
pub trait CallingConvention {
    fn parameter_registers(&self) -> &[Register64];
    fn return_register(&self) -> Register64;
    fn caller_saved_registers(&self) -> &[Register64];
    fn callee_saved_registers(&self) -> &[Register64];
    fn stack_alignment(&self) -> u32;
}

pub struct WindowsX64ABI;
pub struct SystemVABI;
```

### 4. IR Integration Strategy

**Decision**: Direct integration with existing jsavrs IR modules  
**Rationale**:
- Leverages existing IR infrastructure in @src/ir and @src/ir/value
- Maintains consistency with current compiler architecture
- Enables incremental development and testing
- Reuses existing IR validation and optimization passes

**Integration Points**:
- `src/ir/instruction.rs`: Source of IR instructions to translate
- `src/ir/value/`: Type system for IR values and operands
- `src/ir/basic_block.rs`: Control flow structure
- `src/ir/function.rs`: Function-level code generation context

### 5. Assembly Output Format

**Decision**: NASM syntax with modular section organization  
**Rationale**:
- NASM is widely available on all target platforms
- Human-readable output aids debugging and verification
- Supports all necessary features for x86-64 code generation
- Standard .text/.data/.bss section organization
- Compatible with standard linkers on all platforms

**Output Structure**:
```nasm
section .text
global _start

function_name:
    ; Function prologue
    push rbp
    mov rbp, rsp
    ; Function body
    ; Function epilogue
    mov rsp, rbp
    pop rbp
    ret

section .data
    ; Initialized data

section .bss
    ; Uninitialized data
```

## Architectural Patterns and Best Practices

### 1. Visitor Pattern for IR Traversal

**Pattern**: Implement visitor pattern for IR node processing  
**Benefits**:
- Clean separation between IR structure and code generation logic
- Extensible for future IR node types
- Maintainable and testable code organization

**Implementation Strategy**:
```rust
pub trait IRVisitor {
    fn visit_function(&mut self, func: &IRFunction) -> Result<(), CodeGenError>;
    fn visit_basic_block(&mut self, block: &BasicBlock) -> Result<(), CodeGenError>;
    fn visit_instruction(&mut self, instr: &IRInstruction) -> Result<(), CodeGenError>;
}

pub struct AssemblyGenerator {
    output: String,
    register_allocator: RegisterAllocator,
    current_function: Option<String>,
}
```

### 2. Builder Pattern for Complex Instructions

**Pattern**: Use builder pattern for complex instruction generation  
**Benefits**:
- Fluent API for instruction construction
- Compile-time validation of instruction parameters
- Easier testing and maintenance

### 3. Strategy Pattern for Optimization

**Pattern**: Strategy pattern for different optimization levels  
**Benefits**:
- Configurable optimization without changing core logic
- Testable optimization strategies in isolation
- Future extensibility for advanced optimizations

## Performance Considerations

### 1. Memory Management

**Approach**: Pre-allocated buffers with growth strategies
- Use `Vec::with_capacity()` for known upper bounds
- Implement buffer pooling for frequent allocations
- Monitor memory usage to stay within 2x IR size limit

### 2. Instruction Selection Efficiency

**Approach**: Pattern matching with lookup tables
- Pre-computed instruction templates for common patterns
- Efficient IR pattern matching using Rust's match expressions
- Minimize dynamic allocations during code generation

### 3. Benchmarking Strategy

**Implementation**:
```rust
// Integration with existing criterion.rs benchmarks
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_code_generation(c: &mut Criterion) {
    let ir = load_test_ir();
    
    c.bench_function("assembly_generation", |b| {
        b.iter(|| {
            let mut generator = AssemblyGenerator::new();
            black_box(generator.generate(black_box(&ir)))
        })    });
}

criterion_group!(benches, bench_code_generation);
criterion_main!(benches);
```

## Cross-Platform Considerations

### 1. ABI Differences

**Windows x64 ABI**:
- First 4 parameters in RCX, RDX, R8, R9
- Caller allocates shadow space (32 bytes)
- 16-byte stack alignment
- Different calling convention for varargs

**System V ABI (Linux/macOS)**:
- First 6 parameters in RDI, RSI, RDX, RCX, R8, R9
- No shadow space requirement
- 16-byte stack alignment before calls
- Red zone optimization allowed

### 2. Symbol Naming Conventions

**Windows x64 (COFF)**: No leading underscore for C symbols
**Linux x86-64 (ELF)**: No leading underscore for C symbols  
**macOS x86-64 (Mach-O)**: Leading underscore required for external C symbols
**Note**: macOS retains underscore prefix even in 64-bit binaries due to Mach-O format requirements
**Solution**: Platform-specific symbol mangling based on object file format (COFF/ELF vs Mach-O)

### 3. Section Directives

**Implementation**: Platform-specific section directive generation
```rust
fn emit_section_directive(&mut self, section: Section, target_os: TargetOS) {
    match (section, target_os) {
        (Section::Text, TargetOS::Windows) => writeln!(self.output, "section .text"),
        (Section::Text, TargetOS::Linux) => writeln!(self.output, "section .text executable"),
        // ... other combinations
    }
}
```

## Testing Strategy and Semantic Equivalence

### 1. Unit Testing Approach

**Component Testing**:
- Register allocation algorithms
- Instruction encoding correctness
- Calling convention implementation
- Individual IR instruction translation

### 2. Integration Testing

**End-to-End Testing**:
- Complete IR module translation
- Cross-platform assembly generation
- NASM assembly and linking verification
- Performance benchmarking

### 3. Semantic Equivalence Validation

**Approach**: Automated comparison of IR execution vs assembly execution
```rust
#[test]
fn test_semantic_equivalence() {
    let ir = parse_test_ir("simple_function.ir");
    let assembly = generate_assembly(&ir);
    
    // Compile and execute both versions
    let ir_result = execute_ir(&ir, &test_inputs);
    let asm_result = execute_assembly(&assembly, &test_inputs);
    
    assert_eq!(ir_result, asm_result);
}
```

### 4. Snapshot Testing with Insta

**Implementation**:
- Capture assembly output for regression testing
- Version control assembly output changes
- Cross-platform snapshot normalization

## Error Handling and Diagnostics

### 1. Error Categories

**Code Generation Errors**:
- Unsupported IR instructions
- Register allocation failures
- ABI constraint violations
- Invalid operand combinations

**Error Type Design**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported IR instruction: {instruction}")]
    UnsupportedInstruction { instruction: String },
    
    #[error("Register allocation failed for function {function}")]
    RegisterAllocationFailure { function: String },
    
    #[error("ABI violation: {details}")]
    ABIViolation { details: String },
}
```

### 2. Diagnostic Output

**Implementation**: Detailed error messages with source location information
- Line numbers in generated assembly
- IR instruction correlation
- Suggested fixes for common issues

## Integration with Existing jsavrs Architecture

### 1. Module Organization

**New Modules**:
```
src/
├── asm/                  # New assembly generation module
│   ├── mod.rs           # Module entry point
│   ├── generator.rs     # Main code generator
│   ├── register.rs      # Register management
│   ├── instruction.rs   # Instruction encoding
│   ├── operand.rs       # Operand handling
│   └── abi/            # ABI implementations
│       ├── mod.rs
│       ├── windows_x64.rs
│       └── system_v.rs
```

### 2. CLI Integration

**Command Line Interface**:
```rust
#[derive(clap::Parser)]
pub struct AssemblyArgs {
    #[arg(short, long)]
    output_format: OutputFormat,
    
    #[arg(short, long)]
    target_os: TargetOS,
    
    #[arg(short, long)]
    optimization_level: OptimizationLevel,
}
```

### 3. Build System Integration

**Cargo.toml Updates**:
```toml
[dependencies]
iced-x86 = "1.19"
thiserror = "1.0"

[dev-dependencies]
criterion = "0.5"
insta = "1.31"
```

## Risk Assessment and Mitigation

### 1. Technical Risks

**Risk**: iced-x86 API changes
**Mitigation**: Pin to specific version, maintain compatibility layer

**Risk**: ABI specification complexity
**Mitigation**: Comprehensive test suite, reference implementation validation

**Risk**: Performance regression
**Mitigation**: Continuous benchmarking, performance gates in CI

### 2. Integration Risks

**Risk**: Breaking existing IR interfaces
**Mitigation**: Extensive integration testing, gradual rollout

**Risk**: Cross-platform testing complexity
**Mitigation**: CI testing on all target platforms, docker-based testing

## Future Extension Points

### 1. Advanced Instruction Sets

**SSE/AVX Support**: Foundation for SIMD instruction generation
**Architecture**: Extensible register enum design supports future register classes

### 2. Optimization Passes

**Peephole Optimization**: Local instruction sequence improvements
**Register Allocation**: Advanced algorithms (graph coloring, linear scan)

### 3. Debug Information

**DWARF Generation**: Source-level debugging support
**Symbol Table**: Enhanced symbol information for debuggers

## Conclusion

This research establishes a comprehensive foundation for implementing a robust x86-64 assembly code generator within the jsavrs compiler framework. The technology choices prioritize safety, performance, and maintainability while providing clear extension paths for future enhancements. The proposed architecture aligns with jsavrs constitutional principles and leverages Rust's strengths to create a reliable, cross-platform code generation solution.

The implementation strategy emphasizes incremental development with comprehensive testing to ensure semantic correctness and performance targets. The modular design enables parallel development of different components while maintaining clear interfaces and separation of concerns.

## References

- [Intel 64 and IA-32 Architectures Software Developer's Manual](https://software.intel.com/en-us/articles/intel-sdm)
- [System V Application Binary Interface](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf)
- [Microsoft x64 Calling Convention](https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention)
- [iced-x86 Documentation](https://docs.rs/iced-x86/)
- [NASM Documentation](https://www.nasm.us/xdoc/2.15.05/html/nasmdoc0.html)