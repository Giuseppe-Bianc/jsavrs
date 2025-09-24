# SIMD Instruction Generation Contract

## Purpose
Defines the interface for generating SIMD instructions from AST operations in the jsavrs compiler.

## Interface Definition
```rust
/// Trait for SIMD instruction generation
pub trait SimdInstructionGenerator {
    /// Generate SIMD addition instruction for packed single-precision floats
    /// Corresponds to ADDPS instruction
    fn generate_addps(&mut self, dest: XmmRegister, src1: XmmRegister, src2: XmmRegister) -> Result<AssemblyInstruction, SimdGenerationError>;
    
    /// Generate SIMD multiplication instruction for packed single-precision floats  
    /// Corresponds to MULPS instruction
    fn generate_mulps(&mut self, dest: XmmRegister, src1: XmmRegister, src2: XmmRegister) -> Result<AssemblyInstruction, SimdGenerationError>;
    
    /// Generate SIMD addition instruction for packed double-precision floats
    /// Corresponds to ADDPD instruction
    fn generate_addpd(&mut self, dest: XmmRegister, src1: XmmRegister, src2: XmmRegister) -> Result<AssemblyInstruction, SimdGenerationError>;
    
    /// Generate SIMD multiplication instruction for packed double-precision floats
    /// Corresponds to MULPD instruction
    fn generate_mulpd(&mut self, dest: XmmRegister, src1: XmmRegister, src2: XmmRegister) -> Result<AssemblyInstruction, SimdGenerationError>;
    
    /// Generate SIMD subtraction instruction for packed floats
    /// Corresponds to SUBPS or SUBPD depending on precision
    fn generate_subps(&mut self, dest: XmmRegister, src1: XmmRegister, src2: XmmRegister) -> Result<AssemblyInstruction, SimdGenerationError>;
    
    /// Check if SIMD instructions are supported on the target
    fn is_simd_supported(&self) -> bool;
}
```

## Expected Behavior
- When SIMD is supported, the implementation should generate the appropriate SSE/SSE2 instructions
- When SIMD is not supported, the implementation should either return an error or generate scalar equivalent operations
- All generated instructions should maintain functional equivalence with their scalar counterparts

## Validation Criteria
- SIMD operations must produce the same results as scalar operations (within precision tolerances)
- Generated assembly should be syntactically correct
- Performance should show improvement over scalar implementations for vectorizable operations
- Memory alignment requirements must be respected