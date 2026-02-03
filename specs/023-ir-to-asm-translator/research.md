# Research Findings: IR to x86-64 Assembly Translator

## Overview
This document captures the research findings for implementing the IR to x86-64 Assembly Translator module in the jsavrs compiler. The translator will convert IR structures from `src/ir/` into NASM-compatible x86-64 assembly using existing `src/asm/` infrastructure.

## Technology Decisions

### Decision: Use Rust 1.93.0 for Implementation
**Rationale**: The project already uses Rust extensively, and Rust 1.93.0 provides the necessary features for systems programming, memory safety, and performance required for a compiler backend. Leveraging the existing codebase ensures consistency and reduces dependencies.

**Alternatives considered**: 
- C++ for performance: Would introduce memory safety concerns and require significant rework of existing infrastructure
- Go for simplicity: Would not integrate well with existing Rust codebase
- Zig for modern systems programming: Would add another language dependency to the project

### Decision: Direct Integration with Existing src/asm Infrastructure
**Rationale**: The existing `src/asm/` module already provides foundational assembly generation capabilities. Reusing this infrastructure avoids code duplication and ensures consistency with existing assembly generation patterns.

**Alternatives considered**:
- Building a separate assembly generation system: Would duplicate functionality and create maintenance burden
- Using external assembly generation libraries: Would add dependencies and potentially create compatibility issues

### Decision: Support Both System V AMD64 and Windows x64 ABIs
**Rationale**: Supporting both major x86-64 ABIs ensures cross-platform compatibility as required by the feature specification. The existing `src/asm/abi.rs` module already provides ABI abstractions that can be leveraged.

**Alternatives considered**:
- Supporting only one ABI: Would limit platform compatibility against project requirements
- Implementing ABIs separately: Would duplicate effort when existing infrastructure is available

### Decision: Fail-Fast Approach for Unsupported IR Constructs
**Rationale**: Emitting clear diagnostic errors for unsupported IR constructs ensures semantic consistency and prevents generation of incorrect assembly code. This aligns with the safety-first principle of the project.

**Alternatives considered**:
- Generating best-effort code: Could produce incorrect assembly and violate semantic consistency
- Inserting runtime helpers: Would complicate the translator beyond its scope of direct IR-to-assembly mapping

### Decision: Symbolic Register Names for Initial Implementation
**Rationale**: Using symbolic temporary names (t0, t1, t2) simplifies the initial implementation by deferring the complex register allocation problem to a later phase, as specified in the requirements.

**Alternatives considered**:
- Immediate register allocation: Would significantly increase complexity of initial implementation
- Virtual registers: Would require additional infrastructure for register management

## Unknowns Resolved

### ABI Selection Mechanism
**Previously unknown**: How the ABI selection would be implemented
**Resolution**: Use a configuration flag `--target-abi` with platform default fallback via `#[cfg(target_os)]` conditional compilation

### IR Instruction Mapping Strategy
**Previously unknown**: How to map IR instructions to assembly equivalents
**Resolution**: Direct mapping approach: `IrBinaryOp::Add` → `Instruction::Add {dest, src}`, `InstructionKind::Load` → `Instruction::Mov`, etc.

### Assembly Output Format
**Previously unknown**: Whether to use inline comments or separate mapping files for source mapping
**Resolution**: Separate `.map` file with format "IR_LINE:COL → ASM_LINE:LABEL" when `--emit-mapping` flag is enabled

### Error Handling Approach
**Previously unknown**: How to handle translation errors
**Resolution**: Use `TranslationError` with `ErrorCode::E4001` and fail-fast with detailed diagnostic including IR source location

## Architecture Patterns Identified

### Translation Pipeline Pattern
The translator follows a strict pipeline: IR Function → Translation Context → Basic Block traversal in reverse post-order → Instruction-by-instruction mapping → Final assembly emission. This pattern ensures systematic and predictable translation.

### Context Propagation Pattern
Translation Context holds ABI configuration and state during translation, allowing consistent handling of platform-specific requirements throughout the process.

### Error Propagation Pattern
Using `Result<T, TranslationError>` throughout the translation process ensures errors are properly propagated and handled with detailed diagnostics.

## Best Practices Applied

### Memory Safety
Following Rust's ownership model to ensure memory safety during translation without garbage collection overhead.

### Performance Optimization
Targeting <100ms per-function translation with <1GB memory usage as specified in requirements, validated through Criterion benchmarks.

### Error Handling
Implementing comprehensive error handling with detailed diagnostics for production use, following the project's safety-first principle.

### Testing Strategy
Using `cargo test` with `insta` for assembly snapshot regression tests and `criterion` for performance benchmarks to ensure quality and performance.

## Integration Points Identified

### IR Module Integration
Direct consumption of IR structures from `src/ir/` module without modification to existing IR code.

### ASM Module Integration
Leveraging existing `src/asm/instruction::Instruction` enum variants and `AssemblyFile::text_sec_add_instruction()` for final assembly emission.

### ABI Module Integration
Using existing `src/asm/abi.rs` for parameter passing, prologue/epilogue generation, and platform-specific stack allocation.

### Error Handling Integration
Using existing error infrastructure with `ErrorCode::E4001` for translation-specific errors.

## Performance Considerations

### Translation Speed
Maintaining <100ms average translation time per function through efficient algorithms and data structures.

### Memory Usage
Keeping memory usage under 1GB through efficient IR traversal and assembly generation.

### Scalability
Ensuring full module translation completes in <30 seconds for typical compilation units.

## Testing Approach

### Snapshot Testing
Using `insta` crate for assembly output regression testing to catch unintended changes in generated assembly.

### Performance Benchmarking
Using `criterion` crate to validate performance targets and track improvements over time.

### Unit Testing
Comprehensive unit tests for each translation pattern (arithmetic, control flow, function calls) to ensure correctness.

## Code Organization Strategy

### Modular Design
Separating concerns into distinct modules: `mod.rs` (public API), `context.rs` (translation state), `function_translator.rs`, `block_translator.rs`, `instruction_translator.rs`, `terminator_translator.rs`, and `codegen/abi_adapter.rs`.

### Documentation Standards
Following existing codebase patterns with extensive documentation, clippy pedantic lints, rustfmt formatting, and comprehensive error handling.

## Risk Mitigation

### ABI Complexity
Leveraging existing ABI infrastructure to minimize implementation complexity and ensure correctness.

### IR Evolution
Designing extensible architecture to accommodate future IR changes without substantial rewrites.

### Performance Issues
Implementing benchmarking infrastructure early to monitor and address performance concerns.