# Quickstart: Assembly SSE and SSE2 Support

## Overview
This guide demonstrates how to use the new SSE and SSE2 support in the jsavrs compiler system. The implementation provides up to 50% performance improvement for vectorizable operations while maintaining backward compatibility with older processors.

## Prerequisites
- Rust 1.75+ installed
- A processor supporting SSE/SSE2 instructions (Pentium III+ or equivalent)
- The jsavrs compiler built with SSE support enabled

## Enabling SSE/SSE2 Support

### Compile-time Configuration
```bash
# Enable SSE/SSE2 optimizations during compilation
cargo build --features sse

# Or with release optimizations
cargo build --release --features sse
```

### Runtime Detection
The system automatically detects CPU capabilities using the CPUID instruction and selects the appropriate instruction set (SIMD or scalar).

## Basic Usage

### Compiling with SIMD Optimizations
```bash
# Compile a .vn file with SSE/SSE2 optimizations enabled
jsavrs -i input.vn --enable-simd

# Verify SIMD usage with verbose output
jsavrs -i input.vn --enable-simd --verbose
```

### Example Input
The following .vn code will benefit from SIMD optimizations:

```rust
// Vector arithmetic operations that will be optimized
fun vector_add(vec1: f32[4], vec2: f32[4]): f32[4] {
    var result: f32[4];
    result[0] = vec1[0] + vec2[0];  // These operations will use ADDPS
    result[1] = vec1[1] + vec2[1];
    result[2] = vec1[2] + vec2[2];
    result[3] = vec1[3] + vec2[3];
    return result;
}

// Loop with operations that will be vectorized
fun process_array(data: f32[16]) {
    var i: i32 = 0;
    while (i < 16) {
        data[i] = data[i] * 2.5f;  // This loop will use SIMD instructions
        i = i + 1;
    }
}
```

## Verification

### Confirm SIMD Usage
```bash
# Run with performance tracing enabled
jsavrs -i input.vn --enable-simd --trace-performance

# Check output for SIMD operation counts
# Look for messages like: "SIMD: 42 vectorized operations performed"
```

### Performance Comparison
```bash
# Compare performance with and without SIMD
time jsavrs -i input.vn --enable-simd
time jsavrs -i input.vn --disable-simd
```

## Validation of the Implementation

### Run SIMD-Targeted Tests
```bash
# Execute SIMD-specific tests
cargo test simd

# Run all tests including SIMD validation
cargo test

# Run performance benchmarks
cargo bench simd
```

### Expected Results
- Performance improvement: 20-50% for vectorizable operations
- Functional correctness: All results must match scalar implementation
- Fallback behavior: Scalar operations used on non-SIMD processors
- Memory alignment: Both aligned and unaligned data paths work correctly

## Troubleshooting

### CPU Not Supporting SSE/SSE2
If running on older hardware:
- The system should automatically fall back to scalar operations
- Verify with: `jsavrs --cpu-info` to see detected features
- Expect performance similar to the original non-SIMD version

### Alignment Issues
- Ensure data arrays have appropriate alignment for optimal performance
- Unaligned memory access paths are provided but may be slower
- Use alignment annotations in critical code paths if needed

## Feature Summary
- Automatic CPU feature detection via CPUID
- Trait-based dispatch for SIMD/scalar selection
- Both aligned and unaligned memory access patterns
- Fallback to scalar implementations on older processors
- Up to 50% performance improvement for vectorizable operations
- Full backward compatibility with existing interfaces