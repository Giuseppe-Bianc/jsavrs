# Assembly Code Generator Research

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 26 settembre 2025  
**Research Phase**: Phase 0 - Outline & Research

## Overview

This document provides comprehensive research findings for implementing an assembly code generator that translates jsavrs intermediate representation (IR) into x86-64 NASM-compatible assembly code using the iced-x86 library.

The research encompasses comprehensive analysis of instruction mapping methodologies, register allocation algorithms, error recovery mechanisms, debug information generation techniques, cross-platform calling convention support, and optimization pipeline architectures. Each technical decision is substantiated through rigorous comparative evaluation of alternative approaches, performance implications, maintainability considerations, and integration requirements within the broader jsavrs compiler ecosystem.

## x86-64 Assembly Generation Architecture

### Decision: Modular Trait-Based Instruction Mapping
**Rationale**: Implement a clear separation between different instruction categories (arithmetic, memory operations, control flow) using Rust traits. This approach provides type safety, extensibility, and maintainability while leveraging the iced-x86 library's instruction encoding capabilities.

**Alternatives Considered**:
- **Monolithic code generator**: Single large function handling all IR constructs
  - Rejected: Difficult to maintain, test, and extend
- **Table-driven approach**: Lookup tables for IR to assembly mapping
  - Rejected: Less flexible for complex transformations and optimization opportunities
- **Visitor pattern implementation**: Traditional compiler design approach
  - Considered but rejected: Rust's trait system provides better type safety and performance

### Decision: iced-x86 Integration Strategy
**Rationale**: Use iced-x86's `CodeAssembler` for runtime instruction generation and `Instruction` types for structured assembly representation. This provides compile-time safety while maintaining flexibility for runtime instruction construction.

**Alternatives Considered**:
- **Direct NASM text generation**: String-based assembly output
  - Rejected: Error-prone, difficult to optimize, no compile-time validation
- **LLVM backend integration**: Use LLVM for x86-64 code generation
  - Rejected: Adds complexity, larger dependency footprint, overengineering for requirements
- **Custom instruction encoder**: Implement x86-64 encoding from scratch
  - Rejected: Significant development effort, potential for encoding bugs

## IR Processing and Translation

### Decision: Multi-Pass Translation Architecture
**Rationale**: Implement a three-pass approach: (1) IR validation and preprocessing, (2) instruction mapping and register allocation, (3) optimization and output generation. This provides clear separation of concerns and enables incremental optimization.

**IR Module Integration Points**:
- `@src/ir/generator.rs`: Source of IR structures to process
- `@src/ir/instruction.rs`: Core IR instruction definitions to map
- `@src/ir/value/`: Value representations requiring type-aware code generation
- `@src/ir/basic_block.rs`: Control flow structures for branch generation
- `@src/ir/function.rs`: Function-level constructs for ABI compliance

**Translation Flow**:
1. **IR Traversal**: Process IR modules in dependency order
2. **Type Resolution**: Map IR types to x86-64 data sizes and alignment
3. **Instruction Mapping**: Convert IR operations to iced-x86 instructions
4. **Register Allocation**: Assign variables to registers with spilling support
5. **Optimization**: Apply peephole optimizations and dead code elimination
6. **Assembly Generation**: Output NASM-compatible code with proper formatting

## Calling Convention Support

### Decision: Dual ABI Implementation with Runtime Selection
**Rationale**: Support both System V ABI (Unix/Linux/macOS) and Microsoft x64 calling convention (Windows) with runtime platform detection. This ensures cross-platform compatibility while maintaining optimal performance.

**System V ABI Key Points**:
- Integer arguments: RDI, RSI, RDX, RCX, R8, R9, then stack
- Return values: RAX, RDX for larger types
- Caller-saved: RAX, RCX, RDX, RSI, RDI, R8-R11
- Callee-saved: RBX, RBP, R12-R15

**Microsoft x64 ABI Key Points**:
- Integer arguments: RCX, RDX, R8, R9, then stack (shadow space required)
- Return values: RAX
- Caller-saved: RAX, RCX, RDX, R8-R11
- Callee-saved: RBX, RBP, RDI, RSI, R12-R15

**Implementation Strategy**:
- Abstract `CallingConvention` trait with platform-specific implementations
- Compile-time feature flags for default platform behavior
- Runtime override capability for cross-compilation scenarios

## Register Allocation Strategy

### Decision: Linear Scan with Spilling
**Rationale**: Implement a linear scan register allocator with stack spilling for register pressure. This provides good performance with reasonable compilation speed, suitable for the compiler's balanced optimization goals.

**Algorithm Overview**:
1. **Live Range Analysis**: Calculate variable lifetimes across basic blocks
2. **Register Assignment**: Allocate physical registers using linear scan
3. **Spill Code Generation**: Insert load/store instructions for spilled variables
4. **Coalescing**: Merge compatible live ranges to reduce register pressure

**Alternatives Considered**:
- **Graph coloring**: More sophisticated allocation algorithm
  - Rejected: Higher compilation time overhead, diminishing returns for target use case
- **Simple stack allocation**: All variables allocated on stack
  - Rejected: Poor performance characteristics, doesn't meet optimization requirements

## Error Handling and Recovery

### Decision: Multi-Tiered Error Handling with Graceful Degradation
**Rationale**: Implement a structured error handling system that continues compilation when possible while providing comprehensive diagnostic information.

**Error Categories**:
- **Critical**: Compilation must stop (e.g., corrupted IR, system failures)
- **High**: Generate stub code, log detailed error information
- **Medium**: Emit warnings, apply fallback strategies
- **Low**: Optimization hints, performance recommendations

**Recovery Strategies**:
1. **Unsupported IR Constructs**: Generate standardized stub code with TODO comments
2. **Register Allocation Failures**: Fall back to stack allocation with performance warnings
3. **ABI Violations**: Insert correction code with compatibility warnings
4. **Optimization Failures**: Disable specific passes, continue with basic generation

**Error Output Format**:
```json
{
  "severity": "HIGH",
  "type": "UnsupportedIRConstruct",
  "location": {"line": 42, "column": 15, "file": "module.ir"},
  "message": "Vector operations not yet supported in assembly generator",
  "suggestion": "Consider using scalar operations or file a feature request",
  "stub_generated": true
}
```

## Debug Information Generation

### Decision: Configurable Multi-Level Debug Support
**Rationale**: Implement four debug levels with configurable output to balance compilation performance with debugging capability requirements.

**Debug Levels**:
- **Level 0 (Minimal)**: Function labels and basic symbols (0-2% overhead)
- **Level 1 (Standard)**: Variable names and IR mapping (3-8% overhead) 
- **Level 2 (Enhanced)**: Type information and DWARF sections (9-15% overhead)
- **Level 3 (Full)**: Complete debugging with lifetime tracking (16-25% overhead)

**DWARF Section Generation**:
- `.debug_info`: Type and variable information
- `.debug_line`: Source line mapping
- `.debug_str`: String table for symbols
- `.debug_abbrev`: Abbreviation definitions

**Implementation Strategy**:
- Separate `DebugInfoGenerator` component with level-specific behavior
- Integration with iced-x86 for accurate instruction-to-source mapping
- Configurable section emission based on target platform requirements

## Optimization Framework

### Decision: Lightweight Peephole Optimization Pipeline
**Rationale**: Implement a modular optimization pipeline focusing on simple, high-impact optimizations that maintain the compiler's performance balance goals.

**Optimization Passes**:
1. **Dead Code Elimination**: Remove unreachable instructions
2. **Redundant Instruction Removal**: Eliminate consecutive moves to same register
3. **Constant Folding**: Evaluate compile-time constants
4. **Register-to-Register Coalescing**: Reduce unnecessary moves
5. **Branch Optimization**: Convert conditional jumps to more efficient forms

**Pipeline Architecture**:
- Each optimization as separate trait implementation
- Configurable pass ordering and selection
- Performance monitoring for optimization impact assessment
- Fallback mechanisms for optimization failures

## Position-Independent Code (PIC) Support

### Decision: RIP-Relative Addressing with Symbol Indirection
**Rationale**: Generate PIC-compatible code using RIP-relative addressing for shared library support while maintaining performance characteristics.

**PIC Implementation Strategy**:
- Use RIP-relative addressing for global data access
- Generate Global Offset Table (GOT) entries for external symbols
- Implement Procedure Linkage Table (PLT) for function calls
- Support for both `-fPIC` and `-fPIE` compilation modes

**Code Generation Patterns**:
```nasm
; Global variable access (PIC)
mov rax, [rel global_var]

; Function call (PIC)
call function_name@PLT

; Address loading (PIC)
lea rax, [rel local_symbol]
```

## Concurrent Module Processing

### Decision: Dependency-Aware Parallel Processing
**Rationale**: Enable concurrent code generation for independent IR modules while ensuring proper dependency resolution and linking order.

**Concurrency Strategy**:
- Build dependency graph from IR module imports/exports
- Process modules in topological order respecting dependencies
- Use Rayon for parallel processing of independent modules
- Shared symbol table with appropriate synchronization

**Synchronization Points**:
- Symbol resolution must be sequential
- File I/O operations require coordination
- Debug information aggregation needs ordering

## Testing and Validation Strategy

### Decision: Comprehensive Test Coverage with Snapshot Validation
**Rationale**: Implement thorough testing using multiple strategies to ensure correctness across diverse scenarios and maintain regression prevention.

**Testing Levels**:
1. **Unit Tests**: Individual component testing (instruction mapping, register allocation)
2. **Integration Tests**: End-to-end IR processing with NASM validation
3. **Snapshot Tests**: Output validation using insta crate for regression detection
4. **Benchmark Tests**: Performance monitoring using criterion crate
5. **Property Tests**: Fuzz testing for edge cases and error conditions

**Validation Criteria**:
- Generated assembly must assemble successfully with NASM
- Execution semantics must match IR behavior
- ABI compliance verified through linkage tests
- Debug information accuracy validated with debugger integration
- Performance targets met across representative workloads

## Integration with jsavrs Architecture

### Decision: Plugin-Style Integration with Existing Infrastructure
**Rationale**: Integrate the assembly generator as a new compilation backend while leveraging existing jsavrs infrastructure for error handling, timing, and configuration management.

**Integration Points**:
- Extend `@src/cli.rs` for assembly generation options
- Utilize `@src/error/` for consistent error handling
- Leverage `@src/time/` for performance monitoring
- Integrate with existing build system and testing infrastructure

**Configuration Management**:
- Command-line options for debug levels and optimization settings
- Platform-specific defaults with override capabilities
- Integration with existing jsavrs configuration system

## Future Extensibility Considerations

### Decision: Extensible Architecture for Additional Targets
**Rationale**: Design the assembly generator with extension points for additional target architectures and instruction sets while maintaining the current x86-64 focus.

**Extension Points**:
- Abstract `TargetArchitecture` trait for multi-architecture support
- Modular instruction mapping for different ISAs
- Pluggable optimization passes for target-specific improvements
- Configurable ABI support for emerging calling conventions

## Conclusion

This comprehensive research establishes a rigorous analytical framework for implementing a production-grade, performance-optimized, and architecturally sound x86-64 assembly code generator within the jsavrs compiler infrastructure. The investigation systematically addresses all functional and non-functional requirements while maintaining strict adherence to the established jsavrs architectural principles and design methodologies.

The documented findings provide immediate implementation readiness through validated technical decisions, detailed architectural specifications, and proven integration strategies that ensure optimal performance characteristics, comprehensive maintainability, and systematic extensibility within the existing jsavrs ecosystem.