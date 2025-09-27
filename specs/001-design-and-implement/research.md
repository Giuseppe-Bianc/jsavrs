# Research: x86-64 Assembly Code Generator

## Decision: Using iced-x86 for x86-64 instruction encoding
**Rationale**: The iced-x86 crate is a mature, well-maintained Rust library specifically designed for x86/x64 instruction encoding and decoding. It provides safe, efficient access to x86-64 instructions and handles the complexities of the x86-64 instruction set including variable-length encoding, operand encoding, and proper instruction formatting.

**Alternatives considered**:
- Writing raw bytes directly: Risky and error-prone due to x86-64 instruction complexity
- Using the `x86` crate: Less maintained and fewer features than iced-x86
- Generating assembly text directly: Would require implementing complex encoding logic

## Decision: NASM syntax for assembly output
**Rationale**: NASM (Netwide Assembler) is the most widely used assembler for x86-64 on Windows, Linux, and macOS. It has excellent documentation and community support, making it the ideal choice for cross-platform compatibility.

**Alternatives considered**:
- GAS (GNU Assembler) syntax: More common on Linux but less portable to Windows
- MASM (Microsoft Assembler): Windows-specific, not suitable for cross-platform approach
- Direct machine code generation: Would lose readability and maintainability

## Decision: Integration with existing IR modules
**Rationale**: The existing IR modules in @src/ir and @src/ir/value already provide a well-structured intermediate representation. Building the assembly generator to work directly with these modules ensures consistency and leverages existing infrastructure.

**Alternatives considered**:
- Creating a separate IR for code generation: Would duplicate effort and create maintenance overhead
- Using external IR formats: Would break the established architecture pattern

## Decision: Register allocation strategy
**Rationale**: Based on the feature specification, a simple round-robin allocation with stack overflow is specified as the initial approach. This is pragmatic for the initial implementation while allowing for more sophisticated algorithms in the future.

**Alternatives considered**:
- Linear scan allocation: More sophisticated but more complex to implement initially
- Graph coloring allocation: Very sophisticated but overkill for initial implementation
- Pre-colored allocation: Would limit optimization opportunities

## Decision: Platform ABI handling
**Rationale**: Implementing all three major platform ABIs (Windows x64, System V for Linux, and macOS) from the start ensures cross-platform compatibility as required by the feature specification.

**Alternatives considered**:
- Single-platform implementation first: Would delay cross-platform compatibility
- Abstract ABI interface: Already planned as a future requirement in the spec

## Decision: Semantics preservation verification
**Rationale**: Using snapshot testing with insta to verify assembly output against known test cases is the most practical way to ensure semantic equivalence between IR and assembly as specified in the requirements.

**Alternatives considered**:
- Formal verification: Too complex for initial implementation
- Runtime execution comparison: Complex testing infrastructure needed
- Manual verification: Not scalable or reliable