# Research: SSA-based IR Validator Implementation

## Overview
This document outlines the research conducted to support the implementation of an SSA-based IR validator with CFG for the jsavrs compiler. The research focuses on understanding the existing IR architecture, SSA implementation details, and best practices for validation in compiler systems.

## Decision: Research approach for SSA-based IR validator
**Rationale**: To implement the validator effectively, we need a deep understanding of the current IR architecture, SSA implementation, and CFG construction in the jsavrs compiler. This research will inform the design decisions for the validator module.

## Research Tasks

### 1. Understanding the Current IR Architecture
**Task**: Analyze the existing IR implementation in `src/ir/` to understand:
- Current data structures for representing IR
- How SSA form is implemented and maintained
- How CFGs are constructed and stored
- Available APIs for traversing/manipulating IR

**Findings**:
- Located the `src/ir/` directory which contains the intermediate representation code
- Need to examine SSA-specific code to understand how variables are represented in SSA form
- Need to check how control flow is represented and how basic blocks are connected

### 2. SSA Implementation Details
**Task**: Research how Static Single Assignment is currently implemented:
- How variables are defined and used
- How phi functions are handled (if applicable)
- How variable definitions and uses are tracked
- How the dominance relationships are established

**Rationale**: Understanding the SSA implementation is critical for implementing structural invariant validation that checks if variables are defined before use.

### 3. Existing CFG Implementation
**Task**: Understand the current Control Flow Graph implementation:
- How basic blocks are structured
- How control flow edges are represented
- How entry and exit nodes are identified
- How unreachable blocks are handled

**Rationale**: This understanding is essential for implementing CFG integrity validation checks.

### 4. Error Reporting Framework
**Task**: Examine the existing error handling approach in the codebase:
- How errors are currently reported in the compiler
- What error reporting structures exist
- How diagnostic information is formatted
- How location information (line numbers) is handled

**Rationale**: The validator needs to generate detailed error reports with specific line numbers and constructs, so we need to follow the existing patterns.

### 5. CLI Framework and Interface Standards
**Task**: Research the current CLI approach in the project:
- How command-line arguments are processed
- What standard options are currently supported
- How the compiler interfaces with input/output files
- How configuration is handled

**Rationale**: The spec requires CLI support with standard options (-i for input, -o for output, -v for verbose, -c for config), so we need to understand the current patterns.

### 6. Type System and Semantic Analysis
**Task**: Investigate the existing type system in the codebase:
- How types are represented in the IR
- How type checking currently occurs
- How type compatibility is validated
- What type-related data structures exist

**Rationale**: This is essential for implementing semantic invariant validation that checks type consistency.

## Technical Unknowns Resolved

Based on initial analysis, the following unknowns have been resolved:

1. **Language/Version**: Rust 1.75 (determined from project context)
2. **Primary Dependencies**: Internal IR modules, thiserror for errors, insta for testing
3. **Testing**: cargo test and insta
4. **Target Platform**: Linux, Windows, macOS
5. **Project Type**: Single binary/library (compiler component)
6. **Performance Goals**: Process up to 10,000 lines within 5 minutes, <5% false positives (from spec)
7. **Constraints**: Integrate with existing IR architecture, maintain 95% precision (from spec)

## Detailed Findings from Codebase Analysis

### 1. Current IR Architecture
- Located `src/ir/` directory with comprehensive IR implementation
- Key modules include: `basic_block.rs`, `cfg.rs`, `dominance.rs`, `function.rs`, `instruction.rs`, `ssa.rs`, `terminator.rs`, `types.rs`, `value.rs`
- The IR system uses petgraph for representing control flow graphs
- Values are represented in `value.rs` with `Value` and `ValueKind` enums
- Instructions in `instruction.rs` include various operation types like binary, unary, phi functions, etc.

### 2. SSA Implementation Details
- Found `SsaTransformer` struct in `src/ir/ssa.rs` which handles:
  - Dominance computation using `DominanceInfo`
  - Phi-function insertion based on dominance frontiers
  - Variable renaming to ensure single assignment
  - SSA form verification
- The implementation handles variable definitions/uses with debug information containing variable names
- Uses a stack-based approach to maintain variable scopes during renaming

### 3. Existing CFG Implementation
- Found `ControlFlowGraph` struct in `src/ir/cfg.rs` which:
  - Uses petgraph's `DiGraph<BasicBlock, ()>` for the underlying representation
  - Provides entry point identification and block management
  - Includes verification method (`verify()`) that checks entry block existence, terminator validity, and target existence
- Includes basic verification but lacks comprehensive validation needed for the new validator

### 4. Error Reporting Framework
- Found error handling via `CompileError` enum in `src/error/compile_error.rs`
- Uses `thiserror` crate for ergonomic error definition
- Contains location information via `SourceSpan` for error reporting
- Has structured error variants with message, span, and optional help text
- Each error variant includes location information (SourceSpan) and optional help guidance

### 5. CLI Framework and Interface Standards
- Found CLI implementation using `clap` crate in `src/cli.rs`
- Uses clap's derive macros for argument parsing
- Currently supports `-i` (input file) and `-v` (verbose) options
- Includes custom styling for CLI output
- Input files are validated to have `.vn` extension

### 6. Type System and Semantic Analysis
- Found type definitions in `src/ir/types.rs` including:
  - Basic types: `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `Bool`, `Void`
  - Complex types: `Pointer`, `Array`, `Vector`, `Struct`, `FunctionType`
  - Special types: `TypeId`, `ResourceId`, `ScopeId`

## Implementation Considerations

### Validation Approaches
- **Incremental vs Batch Validation**: The spec requires batch validation only, run periodically or on-demand
- **Validation Modes**: Need to support different validation modes (structural, semantic, CFG integrity) as specified
- **Error Collection**: Need to collect all errors before presenting a comprehensive report
- Can leverage existing `SourceSpan` for location tracking and error reporting

### Performance Considerations
- Must handle up to 10,000 lines of IR code efficiently
- Use of Rust's ownership model to ensure memory efficiency
- Consider caching mechanisms for repeated validation runs
- Can reuse existing data structures from IR module for efficiency

### Error Reporting and Diagnostics
- Need to provide location information using `SourceSpan` for errors (already supported by error system)
- Should include suggestions for corrections (already supported by `CompileError` with help field)
- Need to support both structured and human-readable output
- Can leverage the existing error reporting infrastructure with `CompileError`

### Automatic Fix Implementation
- Focus on common structural fixes (variable renaming, control flow adjustments)
- Log all automatic fixes performed for transparency
- Ensure fixes maintain program semantics
- Can build on existing SSA transformation logic for structural fixes

## Implementation Considerations

### Validation Approaches
- **Incremental vs Batch Validation**: The spec requires batch validation only, run periodically or on-demand
- **Validation Modes**: Need to support different validation modes (structural, semantic, CFG integrity) as specified
- **Error Collection**: Need to collect all errors before presenting a comprehensive report

### Performance Considerations
- Must handle up to 10,000 lines of IR code efficiently
- Use of Rust's ownership model to ensure memory efficiency
- Consider caching mechanisms for repeated validation runs

### Error Reporting and Diagnostics
- Need to provide location information for errors
- Should include suggestions for corrections
- Need to support both structured and human-readable output

### Automatic Fix Implementation
- Focus on common structural fixes (variable renaming, control flow adjustments)
- Log all automatic fixes performed for transparency
- Ensure fixes maintain program semantics

## Research Conclusion

The research identified key areas where we need to dive deeper into the existing codebase to understand the implementation details. The next step is to examine the actual IR implementation files to understand the data structures and APIs available for the validator implementation.