# Research: ASM Generation Components Improvement

## Current State Analysis

### ASM Module Structure
The assembly-code generation components are located in `src/asm/` and comprise the following files:

- `generator.rs` — Implements the primary assembly-code generator. Responsibilities include traversing intermediate representations, coordinating instruction emission, and producing formatted assembly output suitable for printing or file emission.

- `instruction.rs` — Declares the types and data structures that represent assembly instructions. This file captures instruction metadata such as mnemonics, operand arity, encoding information, and printing/formatting rules.

- `mod.rs` — Defines the `asm` module structure. It declares and organizes submodules, re-exports the module's public API, and contains module-level documentation describing the assembly-generation interface.

- `operand.rs` — Defines operand types (for example: immediate, register, and memory operands) and implements formatting and serialization routines used when emitting instructions.

- `register.rs` — Enumerates architecture registers and records associated properties, such as size, encoding index, alias names, and register-class utilities (helpers for classification, masks, or allocation hints).


### Key Findings from Code Review

1. **Modularity Issues**:

   - The file `generator.rs` implements multiple responsibilities within a single module, producing a monolithic design that hinders readability, unit testing, and reuse. 
      Recommendation: Refactor `generator.rs` by extracting distinct responsibilities (for example, AST traversal, code emission, and helper utilities) into separate modules (e.g., `generator::ast`, `generator::emitter`, `generator::utils`) and define narrow public interfaces for each module.

   - There is limited separation of concerns among components (for example, parsing, AST transformation, and code generation), which blurs responsibility boundaries and increases coupling.
      Recommendation: Explicitly document component responsibilities, define clear interfaces or traits between components, and enforce separation via module boundaries and tests.

   - Functionality is duplicated across multiple files, increasing maintenance effort and the risk of inconsistent behavior.
      Recommendation: Identify duplicated code, consolidate it into shared utility modules or generic abstractions, and add unit tests to the consolidated code to prevent regression.


2. **Documentation Deficiencies**:

   - Insufficient inline documentation (comments and docstrings are sparse or inconsistent).
   - Lack of comprehensive module-level documentation (module purpose, public API, usage examples, configuration, dependencies, and design rationale).
   - Poorly defined API boundaries between modules and components (no explicit public/private delineation, export lists, or interface contracts).


3. Extensibility concerns

   - Adding a new instruction requires coordinated modifications across multiple components (for example: instruction encoder/decoder, assembler, compiler back end, simulator, test suites, and documentation). This coupling increases implementation effort and the risk of inconsistencies.

   - Extending the register set and introducing new operand types is nontrivial because such changes affect instruction encodings, calling conventions, assembler and disassembler behavior, and compiler register allocation. These dependencies make incremental extension costly and error-prone.

   - Operating-system-specific behavior is insufficiently abstracted from core functionality, which reduces portability and complicates testing. Implementing a well-defined OS abstraction layer (HAL) or explicit interface boundary would isolate OS-dependent code and simplify cross-platform support.


## Best Practices Research

### Rust Code Organization
1. **Module Structure**:
   - Use separate modules for distinct responsibilities
   - Apply the single responsibility principle to modules
   - Leverage Rust's visibility controls (pub, pub(crate))

2. **Documentation**:
   - Follow rustdoc conventions for all public APIs
   - Include examples in documentation
   - Document design decisions in module-level comments

3. **Extensibility Patterns**:
   - Use traits for defining extensible interfaces
   - Apply the builder pattern for complex object construction
   - Leverage enums for closed sets of values (instructions, registers)

## Technology Research

### NASM Assembly Generation
1. **Compatibility Requirements**:
   - Ensure compatibility with the x86-64 instruction-set architecture (ISA), including standard 64-bit encodings, register set, and applicable ABIs.
   - Support generation of object and executable files in multiple formats: Executable and Linkable Format (ELF), Portable Executable / COFF (PE/COFF), and Mach-O.
   - Generate platform-agnostic assembly from a common intermediate representation while allowing insertion of OS- and format-specific sections, directives, and metadata.

2. **Best Practices**:
   - Use consistent naming conventions
   - Separate data and code sections appropriately
   - Handle alignment requirements properly

## Design Decisions

### Decision: Refactor Existing Components
- **Rationale**: Refactoring preserves existing functionality while improving the codebase’s internal structure, modularity, readability, and maintainability. This approach facilitates targeted improvements, simplifies future feature development, and reduces the accumulation of technical debt without disrupting validated system behavior.
- **Alternative Considered**: A complete system rewrite (full replacement of the existing codebase) was considered. A rewrite would allow a fundamental redesign of the architecture and could address accumulated design limitations. However, it entails substantially higher short-term cost, longer development and validation time, increased risk of regressions, potential loss of institutional knowledge embedded in existing behavior, and greater disruption to dependent systems and users.
- **Why Chosen**: Refactoring was selected as the preferred approach because it reduces implementation and integration risk by preserving proven behaviors and existing interfaces, thereby maintaining backward compatibility with current users and dependent systems. Refactoring also enables incremental delivery, continuous testing, and early stakeholder feedback, which together shorten time-to-value relative to a complete rewrite. We recommend executing the refactoring in staged increments with explicit scope definitions, automated regression tests, continuous integration, and stakeholder acceptance criteria to monitor and limit risk.

### Decision: Improve Documentation
- **Rationale**: Comprehensive, up-to-date, and accessible documentation enhances the maintainability of the codebase and the usability of the software for end users.
- **Approach**: Implement detailed Rustdoc comments throughout the codebase. Ensure that all functions, structs, and modules are thoroughly documented, providing clear explanations of functionality, parameters, return values, and usage examples. This approach enhances code readability, maintainability, and facilitates collaboration among developers.


### Decision: Enhance Modularity
- **Rationale**: Effective separation of concerns enhances system extensibility and maintainability by clearly delineating functional responsibilities, facilitating modular design, and reducing interdependencies.
- **Approach**: Redesign the software modules to ensure that each module possesses clearly defined responsibilities, promoting modularity, maintainability, and adherence to the single-responsibility principle.


## Research Summary

The ASM generation components in JSAVRS present challenges in modularity, documentation, and extensibility that necessitate systematic improvement. To address these challenges, the refactoring approach will concentrate on the following objectives:

1. Modularization: Decompose the monolithic generator into smaller, cohesive components that adhere to single-responsibility principles.
2. Comprehensive Documentation: Develop thorough documentation for all public APIs, including inline comments, usage guidelines, and illustrative examples.
3. Separation of Concerns: Clearly delineate distinct aspects of ASM generation, such as parsing, validation, and code generation, to enhance maintainability and clarity.
4. Extensibility: Implement design patterns and architectural provisions that facilitate future enhancements, including plug-in support and flexible API interfaces.

This research clarifies initial ambiguities and establishes a structured roadmap for the implementation phase, ensuring a more maintainable, extensible, and well-documented system.