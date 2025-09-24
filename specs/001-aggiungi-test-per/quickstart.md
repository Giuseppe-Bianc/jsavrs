# Quickstart Guide: Assembly Module Testing

## Overview
This guide helps you quickly set up and run comprehensive tests for the assembly module in the jsavrs compiler project. The tests target components in `src/asm/` and ensure 100% statement coverage with detailed edge cases and cross-platform validation.

## Prerequisites
- Rust 1.75+ installed
- Cargo for dependency management and testing
- Optional: `cargo-llvm-cov` for coverage reports (`cargo install cargo-llvm-cov`)

## Setup
1. Clone the jsavrs repository:
   ```bash
   git clone <repository-url>
   cd jsavrs
   ```

2. Install development dependencies:
   ```bash
   cargo build
   ```

3. Install coverage tool (optional but recommended):
   ```bash
   cargo install cargo-llvm-cov
   ```

## Running Tests

### Basic Test Execution
Run all assembly module tests:
```bash
cargo test --package jsavrs --lib -- asm --nocapture
```

### Running Specific Test Modules
Run tests for specific assembly components:

- Register tests: `cargo test register`
- Operand tests: `cargo test operand`
- Instruction tests: `cargo test instruction`
- NASM generator tests: `cargo test generator`
- Target OS tests: `cargo test target`

### Full Test Suite with Coverage
To run all tests and generate a coverage report:
```bash
cargo llvm-cov --all-features --html --output-path target/llvm-cov/html
```

To run with just command line output:
```bash
cargo llvm-cov --all-features --summary-only
```

## Key Test Scenarios

### Register Testing
- Tests for all register sizes (8-bit, 16-bit, 32-bit, 64-bit)
- Register display implementations
- Conversion functions (to_64bit, to_32bit, etc.)
- ABI-specific register classification

### Operand Testing
- Register, immediate, label, and memory reference operands
- Complex addressing modes including RIP-relative
- Immediate operand boundary values (i64::MIN, i64::MAX, 0, ±1)
- Operand formatting edge cases

### Instruction Testing
- All instruction types and operand combinations
- Instruction-specific operand constraints (e.g., div, idiv require single operand)
- Instruction formatting with complex operand combinations

### NASM Generator Testing
- Section handling to prevent duplicate sections
- Label generation with unique names
- Hello world program generation for all target operating systems
- Function prologue and epilogue generation
- Factorial function generation with recursive calls

### Cross-Platform Testing
- Mocked OS-specific functionality for consistent testing across platforms
- TargetOS methods testing for Linux, Windows, and MacOS
- ABI-specific parameter and callee-saved register validation

## Expected Output
- All tests should pass (0 failures)
- Statement coverage should be 100%
- Test suite execution should complete in under 1 minute
- No performance regressions detected

## Troubleshooting
- If tests timeout: Check for infinite loops or performance issues in implementation
- If coverage is below 100%: Identify untested branches and add corresponding test cases
- If cross-platform tests fail: Verify mocked OS-specific functionality