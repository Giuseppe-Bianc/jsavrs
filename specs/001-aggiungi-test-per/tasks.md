# Implementation Tasks: Add Comprehensive Tests for Assembly Module

## Feature Overview
This document outlines the implementation tasks for adding comprehensive tests for the assembly module in the jsavrs compiler. The implementation will achieve 100% statement coverage across all assembly module components including registers, operands, instructions, and code generators for multiple target operating systems. The tests will include detailed edge cases, corner cases, and cross-platform consistency validation using Rust's testing framework with additional tools like insta for snapshot testing and cargo-llvm-cov for coverage reports.

## Dependencies & Prerequisites
- Rust 1.75+ installed
- Cargo for dependency management and testing
- Required: `cargo-llvm-cov` for coverage reports (`cargo install cargo-llvm-cov`)
- Note: Ensure plan.md is updated to reference "cargo-llvm-cov" instead of "tarpaulin" to maintain consistency across documentation

## Task List

### T001: Setup Testing Dependencies and Configuration
**Description**: Install and configure the required testing tools and dependencies for comprehensive assembly module tests. Ensure consistency with plan.md by using cargo-llvm-cov for coverage.
**Files to modify**: `Cargo.toml`
**Implementation**:
1. Add `insta` crate as a development dependency for snapshot testing
2. Add `cargo-llvm-cov` as the tool for coverage reports
3. Ensure existing testing dependencies are properly configured
4. Set up configuration for snapshot testing in `Cargo.toml`
5. Update documentation to reflect consistent usage of cargo-llvm-cov instead of tarpaulin
**Parallel**: No [P]

### T002: Create Register Module Test File with Robustness Validation
**Description**: Create a dedicated test file for register implementations to validate all register sizes (8-bit, 16-bit, 32-bit, 64-bit) achieving 100% statement coverage. Tests MUST also verify correct value formatting, boundary conditions, and proper handling of invalid inputs to ensure robustness across all supported architectures as specified in FR-001.
**Files to modify**: `src/asm/registers_test.rs`
**Implementation**:
1. Create a new test file specifically for register tests
2. Implement tests for register display implementations for each register size (8-bit, 16-bit, 32-bit, 64-bit)
3. Add tests for register creation with boundary conditions
4. Test proper value formatting for each register type
5. Test handling of invalid inputs to ensure robustness across all supported architectures
6. Include negative tests for invalid register operations
7. Use snapshot testing for output verification
8. Add architecture-specific validation tests to ensure robustness across different x86-64 implementations
9. Test register display with various formatting scenarios to ensure correct value formatting
10. Validate robustness with boundary value testing for all register sizes
**Parallel**: Yes [P]

### T003: Implement Register Conversion Tests with Architecture Robustness
**Description**: Test register conversion functions (to_64bit, to_32bit, etc.) for all register types achieving 100% statement coverage, including edge cases, invalid inputs, and boundary conditions as specified in FR-010. Enhance to ensure robustness across all supported architectures as specified in FR-001.
**Files to modify**: `src/asm/registers_test.rs`
**Implementation**:
1. Add tests for all register conversion methods (to_64bit, to_32bit, etc.)
2. Test conversion with boundary values and edge cases
3. Validate behavior with invalid inputs
4. Test all possible register size combinations
5. Ensure 100% statement coverage of conversion functions
6. Add architecture-specific robustness tests to verify conversion behavior across different x86-64 implementations
7. Validate conversion edge cases that could affect architectural differences
8. Test conversion functions with boundary conditions that might affect different architectures differently
**Parallel**: Yes [P]
**Dependency**: T002

### T004: Implement ABI-Specific Register Classification Tests
**Description**: Validate ABI-specific register classification (parameter, caller-saved, callee-saved) for all supported operating systems achieving 100% statement coverage.
**Files to modify**: `src/asm/registers_test.rs`
**Implementation**:
1. Test parameter register classification for all supported OSes
2. Test caller-saved register classification
3. Test callee-saved register classification
4. Verify correct behavior on each target OS (Linux, Windows, macOS)
5. Include tests for edge cases and invalid inputs
**Parallel**: Yes [P]
**Dependency**: T002

### T005: Create Operand Module Test File
**Description**: Create a dedicated test file for operand implementations to validate all operand types achieving 100% statement coverage.
**Files to modify**: `src/asm/operands_test.rs`
**Implementation**:
1. Create a new test file specifically for operand tests
2. Implement tests for all operand type variants (Register, Immediate, Label, MemoryRef)
3. Test operand display implementations
4. Add boundary value tests for immediate operands (i64::MIN, i64::MAX, 0, ±1)
5. Test complex addressing modes including RIP-relative addressing
6. Test operand formatting with edge cases
7. Use snapshot testing for output verification
**Parallel**: Yes [P]

### T006: Implement Immediate Operand Boundary Tests
**Description**: Validate edge cases for immediate operand boundary values (i64::MIN, i64::MAX, 0, ±1) achieving 100% statement coverage, including combinations of these values in arithmetic and logical operations, overflow/underflow scenarios, and sign transitions.
**Files to modify**: `src/asm/operands_test.rs`
**Implementation**:
1. Test immediate operands with i64::MIN and i64::MAX values
2. Test immediate operands with 0 and ±1 values
3. Test arithmetic operations with boundary values
4. Test logical operations with boundary values
5. Test combination operations with boundary value operands
6. Test overflow/underflow scenarios
7. Test sign transitions at boundary values
**Parallel**: Yes [P]
**Dependency**: T005

### T007: Implement Memory Reference Operand Tests
**Description**: Validate memory reference operand creation with various combinations of base, index, scale, and displacement, achieving 100% statement coverage, including edge cases such as null or zero registers, maximum and minimum displacement values, and unsupported scale factors.
**Files to modify**: `src/asm/operands_test.rs`
**Implementation**:
1. Test all combinations of base, index, scale, and displacement
2. Test with null or zero registers
3. Test with maximum and minimum displacement values
4. Test with unsupported scale factors
5. Test complex addressing with all components present
6. Test RIP-relative addressing modes
7. Test error handling for invalid combinations
**Parallel**: Yes [P]
**Dependency**: T005

### T008: Implement Operand Utility Method Tests
**Description**: Validate operand utility methods (is_register, as_immediate, etc.) achieving 100% statement coverage, including edge cases, error handling, and type validation.
**Files to modify**: `src/asm/operands_test.rs`
**Implementation**:
1. Test `is_register` method with all operand types
2. Test `as_immediate` method with all operand types
3. Test all other operand utility methods
4. Include error handling tests for invalid conversions
5. Test type validation for all operand types
6. Verify behavior with edge cases and invalid inputs
**Parallel**: Yes [P]
**Dependency**: T005

### T009: Create Instruction Module Test File
**Description**: Create a dedicated test file for instruction implementations to validate all instruction types and operand combinations achieving 100% statement coverage.
**Files to modify**: `src/asm/instructions_test.rs`
**Implementation**:
1. Create a new test file specifically for instruction tests
2. Implement tests for all instruction types
3. Test all possible operand combinations
4. Test instruction display implementations
5. Test instruction-specific operand constraints (e.g., div, idiv require single operand)
6. Test instruction formatting with complex operand combinations
7. Use snapshot testing for output verification
**Parallel**: Yes [P]

### T010: Implement Instruction-Specific Constraint Tests
**Description**: Validate instruction-specific operand constraints (e.g., div, idiv require a single operand), achieving 100% statement coverage, and enforce correct operand types, ranges, and combinations for all supported instructions.
**Files to modify**: `src/asm/instructions_test.rs`
**Implementation**:
1. Test div instruction with single operand requirement
2. Test idiv instruction with single operand requirement
3. Test other instructions with specific operand count requirements
4. Test correct operand types for each instruction
5. Test operand ranges for each instruction
6. Test invalid combinations and verify proper error handling
**Parallel**: Yes [P]
**Dependency**: T009

### T011: Implement Instruction Utility Method Tests
**Description**: Validate instruction utility methods (as_instruction, is_jump, etc.) achieving 100% statement coverage, ensuring all possible execution paths are exercised.
**Files to modify**: `src/asm/instructions_test.rs`
**Implementation**:
1. Test `as_instruction` method with all instruction types
2. Test `is_jump` method with all instruction types
3. Test all other instruction utility methods
4. Include tests for edge cases, invalid inputs, and boundary conditions
5. Verify all possible execution paths are covered
6. Test error handling for invalid operations
**Parallel**: Yes [P]
**Dependency**: T009

### T012: Create NASM Generator Test File
**Description**: Create a dedicated test file for NASM generator functionality to validate section handling, label generation, and program generation achieving 100% statement coverage.
**Files to modify**: `src/asm/nasm_generator_test.rs`
**Implementation**:
1. Create a new test file specifically for NASM generator tests
2. Test section handling functionality to prevent duplicate sections
3. Test correct section ordering
4. Test detection of empty or invalid sections
5. Test enforcement of consistency across generated outputs
6. Use snapshot testing for output verification
**Parallel**: Yes [P]

### T013: Implement Label Generation Tests
**Description**: Validate the label generation functionality to ensure unique label names achieving 100% statement coverage, including handling edge cases, duplicate detection, and proper error reporting.
**Files to modify**: `src/asm/nasm_generator_test.rs`
**Implementation**:
1. Test unique label generation functionality
2. Test duplicate detection mechanisms
3. Test proper error reporting for invalid inputs
4. Test edge cases for label generation
5. Verify sequential label naming works correctly
6. Test behavior under high label count scenarios
**Parallel**: Yes [P]
**Dependency**: T012

### T014: Implement Hello World Program Generation Tests
**Description**: Validate the hello world program generation for all currently implemented target operating systems, achieving 100% statement coverage, and report any compilation or runtime errors with clear diagnostics. Validation results should be logged for audit and traceability purposes. Tests MUST include comprehensive testing including edge cases, error handling, and cross-platform consistency as specified in FR-006.
**Files to modify**: `src/asm/hello_world_generation_test.rs`
**Implementation**:
1. Test hello world program generation for Linux
2. Test hello world program generation for Windows
3. Test hello world program generation for macOS
4. Test cross-platform consistency
5. Test error handling and diagnostics for generation failures
6. Validate generated assembly syntax for each platform
7. Test logging of validation results for audit and traceability
8. Test error handling and recovery procedures
9. Use snapshot testing for output verification
**Parallel**: Yes [P]

### T015: Implement Function Prologue/Epilogue Generation Tests
**Description**: Validate the function prologue and epilogue generation for different target operating systems, ensuring correct stack setup, register preservation, and cleanup, achieving 100% statement coverage across all supported platforms.
**Files to modify**: `src/asm/function_generation_test.rs`
**Implementation**:
1. Test function prologue generation for Linux
2. Test function prologue generation for Windows
3. Test function prologue generation for macOS
4. Test function epilogue generation for all platforms
5. Verify correct stack setup for each platform
6. Test register preservation for each platform
7. Test cleanup procedures for each platform
**Parallel**: Yes [P]
**Dependency**: T014

### T016: Implement Factorial Function Generation Tests
**Description**: Validate the factorial function generation with recursive calls achieving 100% statement coverage, ensuring correct handling of base cases, negative inputs, and large numbers.
**Files to modify**: `src/asm/factorial_function_test.rs`
**Implementation**:
1. Test factorial function generation with recursive calls
2. Test base case handling (factorial of 0 and 1)
3. Test handling of negative inputs
4. Test handling of large numbers within computational limits
5. Validate generated assembly for correctness
6. Test edge cases and error conditions
**Parallel**: Yes [P]

### T017: Create Assembly Element Test File
**Description**: Create a dedicated test file for assembly element implementations to validate all element types achieving 100% statement coverage.
**Files to modify**: `src/asm/assembly_elements_test.rs`
**Implementation**:
1. Create a new test file specifically for assembly element tests
2. Test all AssemblyElement type variants (Section, Label, Instruction, Directive, Comment)
3. Test assembly element manipulation methods (add_element, add_elements)
4. Test section handling to prevent duplicates
5. Test proper ordering of sections
6. Validate nested elements within sections
**Parallel**: Yes [P]

### T018: Implement Assembly Element Manipulation Tests
**Description**: Validate assembly element manipulation methods (add_element, add_elements, etc.) achieving 100% statement coverage. Validation MUST include error handling, boundary conditions, and input type verification.
**Files to modify**: `src/asm/assembly_elements_test.rs`
**Implementation**:
1. Test `add_element` method with various element types
2. Test `add_elements` method with various element collections
3. Test error handling for invalid operations
4. Test boundary conditions for element addition
5. Test input type verification
6. Verify robust and predictable behavior for all operations
**Parallel**: Yes [P]
**Dependency**: T017

### T019: Create TargetOS Test File
**Description**: Create a dedicated test file for TargetOS implementations to validate all OS-specific methods achieving 100% statement coverage. Include validation logging for traceability as specified in FR-023.
**Files to modify**: `src/asm/target_os_test.rs`
**Implementation**:
1. Create a new test file specifically for TargetOS tests
2. Test all TargetOS methods (param_register, callee_saved_registers)
3. Test OS-specific parameter register retrieval for Linux, Windows, and macOS
4. Test callee-saved register retrieval for all platforms
5. Test error handling for invalid or edge-case inputs
6. Test consistency across different OS targets
7. Test validation results logging for traceability and debugging
8. Verify backward compatibility in the API even if internals change
9. Use snapshot testing for output verification
**Parallel**: Yes [P]

### T020: Implement Cross-Platform Consistency Tests
**Description**: Validate cross-platform consistency in assembly generation and ensure mocked OS-specific functionality works correctly across platforms.
**Files to modify**: `src/asm/cross_platform_test.rs`
**Implementation**:
1. Test mocked OS-specific functionality for consistent testing
2. Validate cross-platform compatibility
3. Test calling convention differences between OSes
4. Verify ABI compliance across platforms
5. Test error handling for platform-specific behaviors
6. Test that API maintains backward compatibility across changes
**Parallel**: Yes [P]
**Dependency**: T019

### T021: Implement Error Handling Tests for Invalid Combinations
**Description**: Validate error handling for invalid register combinations achieving 100% statement coverage, ensuring all edge cases and boundary conditions are tested and properly logged.
**Files to modify**: `src/asm/error_handling_test.rs`
**Implementation**:
1. Test invalid register combinations
2. Test invalid operand type combinations
3. Test error logging and reporting
4. Test boundary conditions for invalid inputs
5. Test edge cases for invalid combinations
6. Verify proper error handling throughout the module
**Parallel**: Yes [P]

### T022: Implement Complex Addressing Mode Tests
**Description**: Validate all memory reference addressing modes, including RIP-relative, achieving 100% statement coverage, and ensure correct handling of edge cases, alignment constraints, and exception conditions. Validation results MUST be logged for traceability and debugging as specified in FR-020.
**Files to modify**: `src/asm/addressing_modes_test.rs`
**Implementation**:
1. Test all memory reference addressing modes
2. Test RIP-relative addressing specifically
3. Test alignment constraints handling
4. Test exception conditions during memory access
5. Test edge cases for complex addressing modes
6. Validate generated assembly for correctness
7. Test logging of validation results for traceability and debugging
**Parallel**: Yes [P]
**Dependency**: T007

### T023: Implement Edge Case Tests for Operand Formatting
**Description**: Validate edge cases for operand formatting, including negative displacements, zero and maximum scale factors, and other boundary values. Handle and report error conditions gracefully for all invalid or out-of-range inputs.
**Files to modify**: `src/asm/operand_formatting_test.rs`
**Implementation**:
1. Test negative displacement formatting
2. Test zero scale factor formatting
3. Test maximum scale factor formatting
4. Test other boundary values for formatting
5. Test graceful error handling for invalid inputs
6. Test formatting with various combinations of boundary values
**Parallel**: Yes [P]
**Dependency**: T008

### T024: Set Up Coverage Verification with cargo-llvm-cov
**Description**: Configure and run cargo-llvm-cov to ensure all tests achieve 100% statement coverage as required. This addresses the inconsistency between plan.md (which mentions "tarpaulin") and tasks.md by explicitly using cargo-llvm-cov.
**Files to modify**: `scripts/verify_coverage.sh`
**Implementation**:
1. Create a script to run `cargo-llvm-cov` with the correct settings
2. Generate coverage reports for all assembly module components
3. Verify that statement coverage reaches 100% for all components
4. Identify any remaining uncovered code paths
5. Create additional targeted tests if coverage is below 100%
6. Generate HTML coverage reports for review
7. Document the usage of cargo-llvm-cov instead of tarpaulin for consistency
**Parallel**: No [P]
**Dependency**: All previous tests completed

### T025: Run Full Assembly Module Test Suite
**Description**: Execute the complete test suite to validate all implemented tests work together and achieve the required performance goals.
**Files to modify**: `scripts/run_tests.sh`
**Implementation**:
1. Create a script to run all assembly module tests
2. Verify all tests pass (0 failures)
3. Measure test suite execution time (should be under 1 minute)
4. Verify coverage remains at 100% with all tests running using cargo-llvm-cov
5. Document performance metrics
6. Run cross-platform tests to ensure consistency
**Parallel**: No [P]
**Dependency**: T024

### T026: Document Test Organization and Maintenance
**Description**: Create documentation for the test organization and maintenance procedures as specified in the requirements. Include documentation of tooling choice consistency.
**Files to modify**: `docs/assembly_tests.md`
**Implementation**:
1. Document the modular test organization with separate test files for each component
2. Document the maintenance procedures for backward compatibility
3. Document the snapshot testing procedures for detecting changes
4. Provide examples of how to add new tests for new functionality
5. Include guidelines for manual review of snapshot differences
6. Document cross-platform testing procedures
7. Explicitly document the choice of cargo-llvm-cov over tarpaulin for consistency
**Parallel**: No [P]
**Dependency**: T025

## Task Execution Notes

### Parallel Execution Groups
- Group 1 [T002, T005, T009, T012, T014, T016, T017, T019]: All main test file creation tasks can run in parallel
- Group 2 [T003-T004, T006-T008, T010-T011, T013, T015, T018, T020-T023]: Sub-tests can run in parallel after their dependencies are satisfied
- Group 3 [T024-T026]: Final verification and documentation tasks run sequentially

### Tooling Consistency Note
This tasks.md file explicitly uses `cargo-llvm-cov` throughout to maintain consistency. The plan.md file should be updated to also reference `cargo-llvm-cov` instead of `tarpaulin` to align with this implementation approach. This resolves the inconsistency identified in the analysis report.

### Test Organization Strategy
Following requirement FR-016, tests are organized in separate files per module component:
- `registers_test.rs` - Tests for register implementations
- `operands_test.rs` - Tests for operand implementations
- `instructions_test.rs` - Tests for instruction implementations
- `nasm_generator_test.rs` - Tests for NASM generator functionality
- `target_os_test.rs` - Tests for target OS implementations
- `hello_world_generation_test.rs` - Tests for cross-platform hello world generation
- `function_generation_test.rs` - Tests for function generation
- `factorial_function_test.rs` - Tests for factorial function generation
- `assembly_elements_test.rs` - Tests for assembly element implementations
- `cross_platform_test.rs` - Tests for cross-platform consistency
- `error_handling_test.rs` - Tests for error handling
- `addressing_modes_test.rs` - Tests for complex addressing modes
- `operand_formatting_test.rs` - Tests for operand formatting edge cases

### Performance Expectations
The test suite should complete within 1 minute on standard development hardware as specified in FR-017. Parallel execution of independent test files will help achieve this goal.