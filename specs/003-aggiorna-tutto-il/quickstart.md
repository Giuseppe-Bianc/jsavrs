# Quickstart: Testing SSE/SSE2 Implementation

## Prerequisites
- Rust 1.75+ installed
- Cargo build system
- CPU with SSE/SSE2 support (Pentium III+ or equivalent)
- jsavrs compiler source code

## Setup
1. Clone the jsavrs repository
2. Navigate to the project directory
3. Ensure your CPU supports SSE/SSE2 instructions

## Basic Usage Test
1. Compile a simple program with floating-point operations:
   ```bash
   cargo run -- compile --enable-sse2 example_program.vn
   ```

2. Verify the generated assembly contains SSE instructions:
   ```bash
   # Check for SSE instructions in output
   grep -i "addps\|mulps\|addpd\|mulpd" output.asm
   ```

## Performance Validation
1. Create a test program with vectorizable operations:
   ```rust
   // Example array operations that should trigger SIMD optimizations
   let mut arr = [1.0f32; 16];
   for i in 0..16 {
       arr[i] = arr[i] * 2.0 + 1.0;  // Should use SIMD instructions
   }
   ```

2. Benchmark the performance:
   ```bash
   cargo run -- benchmark --with-sse example_program.vn
   ```

## Compatibility Test
1. Run on a system without SSE support (or use compatibility mode):
   ```bash
   cargo run -- compile --disable-sse example_program.vn
   ```

2. Verify the scalar fallback executes correctly:
   ```bash
   ./compiled_program
   # Should run without errors, possibly with reduced performance
   ```

## Validation Steps
- [ ] SSE/SSE2 instructions are generated when supported
- [ ] Scalar fallback works when SIMD is unavailable
- [ ] Generated code produces correct results for both paths
- [ ] Performance improves by 20-50% on vectorizable operations
- [ ] Memory alignment issues are properly handled
- [ ] No regression in existing functionality

## Expected Results
With SSE/SSE2 enabled, operations on vectors/arrays should:
1. Use XMM registers for parallel processing
2. Show improved performance metrics (20-50% speedup)
3. Maintain identical output to scalar implementations
4. Gracefully fall back to scalar code when needed