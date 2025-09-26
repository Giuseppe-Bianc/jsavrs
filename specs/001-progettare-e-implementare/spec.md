# Feature Specification: Generatore di Codice Assembly x86-64 per IR

**Feature Branch**: `001-progettare-e-implementare`  
**Created**: 26 settembre 2025  
**Status**: Draft  
**Input**: User description: "Progettare e implementare un generatore di codice assembly per l'architettura x86-64 (a 64 bit) che, utilizzando la sintassi NASM, trasformi una rappresentazione intermedia (IR) in codice assembly corretto e assemblabile con NASM"

## Execution Flow (main)
```
1. Parse user description from Input
   → Feature: Code generator for x86-64 assembly from IR
2. Extract key concepts from description
   → Actors: compiler system, developers
   → Actions: transform IR to assembly, generate NASM-compatible code
   → Data: intermediate representation (IR), assembly code
   → Constraints: x86-64 architecture, NASM syntax compliance
3. Unclear aspects identified and marked
4. Fill User Scenarios & Testing section
   → User flow: compile source → IR → assembly generation
5. Generate Functional Requirements
   → All requirements are testable and specific
6. Identify Key Entities (IR structures, assembly output)
7. Run Review Checklist
   → No implementation details included
   → Focus on user needs and business value
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## Clarifications

### Session 2025-09-26
- Q: Quale calling convention x86-64 dovrebbe essere supportata dal generatore? → A: Entrambe le convenzioni (System V + Microsoft)
- Q: Quali tipi di dati deve supportare il generatore per le operazioni e l'allocazione memoria? → A: Interi + float + puntatori + stringhe
- Q: Quale strategia di gestione degli errori dovrebbe adottare il generatore quando incontra IR non supportate o problematiche? → A: Multi-tiered error handling: (1) Generate stub code with standardized TODO comments using format `; TODO: Unsupported IR construct [type] - [reason]`, (2) Log structured error data to JSON output with severity levels, source locations, and resolution suggestions, (3) Continue compilation for recoverable errors while maintaining error count statistics, (4) Provide graceful degradation strategies for resource conflicts (e.g., stack spilling for register pressure)
- Q: Quale livello di ottimizzazione del codice assembly dovrebbe implementare il generatore? → A: Ottimizzazioni di base (peephole, istruzioni ridondanti)
- Q: Quale formato di output assembly dovrebbe produrre il generatore per massimizzare la compatibilità? → A: Output modulare con sezioni separate (data, text, bss)
- Q: What is the expected performance target for the assembly generation process? Should the generator prioritize compilation speed, output code quality, or strike a balance between them? → A: Balance speed and code quality
- Q: Should the generator handle multi-threaded or concurrent code generation for different modules? → A: Yes, if modules are independent
- Q: How should the generator handle dependencies between different IR modules? → A: Resolve dependencies before code generation
- Q: What level of debug information should the generator produce in the assembly output? → A: Configurable multi-level debug information: (Level 0) Basic symbols and labels, (Level 1) IR mapping and variable names, (Level 2) DWARF-compatible sections with type information, (Level 3) Full source-level debugging with variable lifetime tracking. Default to Level 1 with command-line option to specify debug level. Must ensure debugger compatibility (GDB, LLDB) for Level 2+ and maintain performance impact < 15% compilation time overhead for any debug level
- Q: Should the generator emit position-independent code (PIC) for shared libraries? → A: Yes, for shared library compatibility

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
As a developer using the jsavrs compiler, I want to generate executable assembly code from my compiled program's intermediate representation so that I can produce native x86-64 binaries that can be assembled and executed on target systems.

### Acceptance Scenarios
1. **Given** a valid IR representation from the jsavrs compiler, **When** the assembly generator is invoked, **Then** syntactically correct NASM assembly code is produced for x86-64 architecture
2. **Given** NASM assembly output from the generator, **When** processed by the NASM assembler, **Then** the assembly process completes successfully without syntax errors
3. **Given** complex IR with function calls and control flow, **When** assembly is generated, **Then** the output maintains correct program semantics and execution order
4. **Given** IR with variable declarations and operations, **When** assembly is generated, **Then** proper register allocation and memory management instructions are produced
5. **Given** jsavrs IR containing unsupported or invalid constructs, **When** the generator encounters these constructs, **Then** the system generates stub code with standardized TODO comments, logs structured error information, continues processing remaining valid IR, and produces a compilation report with error summary and actionable recommendations
6. **Given** jsavrs IR with memory constraint violations or impossible register allocation scenarios, **When** the generator attempts code generation, **Then** the system gracefully degrades performance (using stack spilling), emits appropriate warnings with specific resource conflict details, and completes assembly generation with alternative strategies
7. **Given** jsavrs IR with source location metadata and debug level 2+ configuration, **When** assembly generation occurs, **Then** the system produces DWARF-compatible debug sections, maintains accurate IR-to-assembly mapping, generates complete symbol tables, and ensures debugger compatibility with GDB/LLDB for step-through debugging
8. **Given** jsavrs assembly output with debug information level 1+, **When** loaded by debugging tools, **Then** variable names are correctly resolved, function boundaries are accurately identified, and source line mapping provides ±2 line accuracy for debugging navigation

### Edge Cases
- What happens when IR contains unsupported operations for x86-64 architecture? → Generate stub code with standardized TODO comments (format: `; TODO: Unsupported IR construct [type] - [reason]`), log error details to JSON output, continue compilation
- How does the system handle memory alignment requirements for different data types? → Apply x86-64 alignment rules (1, 2, 4, 8 bytes), emit padding instructions, warn about performance implications
- What occurs when register pressure exceeds available x86-64 registers? → Implement stack spilling with minimal performance impact, emit optimization suggestions, maintain register allocation statistics
- How are calling conventions properly implemented for function interoperability? → Support System V ABI and Microsoft x64 calling conventions, validate parameter passing, handle return value management
- What happens when IR contains invalid memory references or null pointer dereferences? → Generate safe assembly with bounds checking where possible, emit runtime safety comments, log potential issues
- How does the system handle compilation errors that prevent assembly generation? → Maintain partial assembly output, provide detailed error location mapping, suggest corrective actions
- What occurs during concurrent module processing when dependencies are circular or unresolvable? → Detect dependency cycles, generate appropriate error messages, provide dependency resolution suggestions
- How does the system handle debug information generation when IR lacks source location metadata? → Generate synthetic debug markers based on IR structure, maintain function boundaries, provide best-effort symbol information, warn about limited debugging capability
- What happens when debug information generation conflicts with optimization passes? → Prioritize debugging accuracy over optimization when debug level ≥ 2, maintain optimization annotations for debug level 3, provide compilation flags to control debug vs optimization trade-offs
- How are debug symbols managed for inlined functions and optimized-away variables? → Track inlining decisions in debug metadata, maintain variable lifetime information, generate DWARF inline info sections, provide symbolic debugger compatibility for optimized code
- What occurs when DWARF section generation fails or exceeds memory limits? → Gracefully degrade to lower debug level, emit warning with specific resource constraints, maintain partial debug information, continue assembly generation with reduced debug fidelity

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST transform jsavrs intermediate representation into valid x86-64 assembly code
- **FR-002**: System MUST generate assembly using NASM-compatible syntax and directives with modular output organized into separate sections (data, text, bss)
- **FR-003**: System MUST produce assembly that successfully assembles with NASM without syntax errors
- **FR-004**: System MUST preserve program semantics and execution flow from the original IR
- **FR-005**: System MUST handle basic arithmetic operations (add, subtract, multiply, divide) for integer types (8, 16, 32, 64 bit) and floating-point types (32, 64 bit)
- **FR-006**: System MUST implement both System V ABI (Unix/Linux) and Microsoft x64 calling conventions for cross-platform compatibility
- **FR-007**: System MUST generate appropriate memory management instructions for variable storage including integers, floats, pointers, and string data
- **FR-008**: System MUST handle control flow structures (loops, conditionals, jumps) correctly
- **FR-009**: System MUST allocate registers efficiently for temporary and local variables across all supported data types, applying basic optimization techniques to minimize register spilling
- **FR-010**: System MUST generate proper program entry points and exit sequences
- **FR-011**: System MUST produce human-readable assembly output with appropriate comments and labels
- **FR-012**: System MUST handle data type conversions and casting operations between integers, floating-point numbers, pointers, and string types
- **FR-013**: System MUST support basic I/O operations for program input and output
- **FR-014**: System MUST implement comprehensive error handling with the following specific behaviors:
  - Generate stub code with standardized TODO comments (format: `; TODO: Unsupported IR construct [construct_type] - [reason]`) when encountering unsupported IR operations
  - Continue compilation process without termination when non-critical errors occur
  - Maintain error counter with maximum threshold of 100 warnings before suggesting review
  - Log all error conditions to structured output (JSON format) for automated tooling integration
  - Provide clear error messages with IR source location, error type classification, and suggested resolution steps
- **FR-015**: System MUST implement basic code optimizations including peephole optimizations and removal of redundant instructions to improve generated assembly quality
- **FR-016**: System MUST organize generated assembly code into clearly separated sections: .text for executable code, .data for initialized data, and .bss for uninitialized data
- **FR-017**: System MUST balance compilation speed with output code quality during assembly generation
- **FR-018**: System MUST support concurrent code generation for independent modules when applicable
- **FR-019**: System MUST resolve dependencies between IR modules before code generation
- **FR-020**: System MUST implement comprehensive debug information generation with multiple configurable levels:
  - **Level 0 (Minimal)**: Function labels and basic symbol table with external symbol visibility
  - **Level 1 (Standard)**: Function labels, local variable names, basic type information, and IR-to-assembly line mapping
  - **Level 2 (Enhanced)**: Complete symbol table, detailed type information, register allocation tracking, and optimization decision annotations
  - **Level 3 (Full Debug)**: Source-level debugging support with DWARF-compatible metadata, variable lifetime tracking, and inlined function information
- **FR-021**: System MUST generate position-independent code (PIC) for shared library compatibility
- **FR-022**: System MUST implement standardized error message format with the following components:
  - Error classification (CRITICAL, HIGH, MEDIUM, LOW severity levels)
  - IR source location with line and column numbers where available
  - Specific error type identification (UnsupportedOperation, InvalidMemoryRef, RegisterAllocationFailure, etc.)
  - Context description explaining the problematic IR construct
  - Suggested resolution steps or alternative approaches
  - Machine-readable JSON output for integration with development tools
- **FR-023**: System MUST generate DWARF-compatible debug sections (.debug_info, .debug_line, .debug_str) when debug level ≥ 2, ensuring compatibility with standard debuggers (GDB, LLDB)
- **FR-024**: System MUST provide debug information mapping with the following precision requirements:
  - IR instruction to assembly instruction correspondence with 100% accuracy
  - Source line to assembly line mapping within ±2 line accuracy when source information available
  - Variable name to register/memory location mapping with real-time updates during register allocation
  - Function boundary markers with accurate stack frame size information
- **FR-025**: System MUST provide comprehensive debug configuration control with the following capabilities:
  - Command-line debug level selection (--debug-level 0|1|2|3) with level validation
  - Selective debug section generation (--debug-sections symbols,lines,types,dwarf) for custom debugging needs
  - Debug output format selection (--debug-format nasm-comments|dwarf|json) for different toolchain compatibility
  - Source location preservation mode (--preserve-locations strict|best-effort|off) for optimization vs debugging trade-offs
  - Debug symbol filtering (--debug-filter functions|variables|types|all) for reduced debug information size
  - Cross-compilation debug target specification (--debug-target-platform linux|windows|macos) for platform-specific debug formats

### Non-Functional Requirements
- **NFR-001**: Assembly generation process MUST balance compilation speed with output code quality
- **NFR-002**: System MUST support concurrent processing of independent modules during code generation
- **NFR-003**: Error handling system MUST provide diagnostic information with response time < 100ms for error classification and reporting
- **NFR-004**: System MUST handle error conditions gracefully without memory leaks or resource corruption, maintaining compilation process stability across 10,000+ IR nodes
- **NFR-005**: Error messages MUST be internationalization-ready and include context-sensitive help with minimum 90% user comprehension rate based on developer feedback
- **NFR-006**: Error recovery mechanisms MUST allow compilation to continue for at least 95% of recoverable error conditions without losing subsequent valid code generation
- **NFR-007**: Debug information generation MUST maintain compilation performance overhead within specified limits: Level 0 (0-2%), Level 1 (3-8%), Level 2 (9-15%), Level 3 (16-25%) compared to no-debug compilation
- **NFR-008**: Generated DWARF debug sections MUST be compatible with industry-standard debuggers (GDB 7.0+, LLDB 6.0+) with 100% symbol resolution accuracy for Level 2+ debug information
- **NFR-009**: Debug symbol table generation MUST scale efficiently with program size, maintaining sub-linear memory overhead (O(n log n)) for programs up to 1M+ IR nodes
- **NFR-010**: IR-to-assembly debug mapping MUST preserve source location accuracy within ±2 lines for 95% of mappable instructions when source metadata is available

### Key Entities *(include if feature involves data)*
- **Intermediate Representation (IR)**: The compiler's internal representation containing program structure, operations, control flow, and data declarations
- **Assembly Code**: The generated x86-64 NASM-compatible assembly instructions organized in modular sections (.text, .data, .bss) representing the executable program
- **Instruction Mapping**: The correspondence between IR operations and their x86-64 assembly equivalents
- **Register Allocation**: The assignment of program variables and temporaries to available x86-64 processor registers
- **Memory Layout**: The organization of program data, stack frames, and heap allocations in the generated assembly
- **Symbol Table**: The mapping of program identifiers to their assembly labels and memory locations
- **Control Flow Graph**: The representation of program execution paths and branching logic in assembly form
- **Optimization Engine**: The component responsible for applying peephole optimizations and removing redundant instructions from the generated assembly code
- **Error Handler**: The system component responsible for detecting, classifying, logging, and recovering from IR processing errors while maintaining compilation continuity and providing actionable diagnostic information
- **Diagnostic Reporter**: The structured output system that generates JSON-formatted error reports, maintains error statistics, and provides context-sensitive recommendations for error resolution
- **Debug Information Generator**: The system component responsible for producing multi-level debug metadata, DWARF section generation, symbol table construction, and IR-to-assembly mapping for debugger compatibility
- **Symbol Table Manager**: The centralized registry maintaining program identifiers, their assembly labels, memory locations, type information, and scope visibility for debug information generation
- **Source Location Mapper**: The precision mapping system that correlates IR instructions with original source positions, tracks variable lifetimes, and maintains debugging accuracy across optimization passes

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous  
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

### Community Guidelines
- [x] Specifications promote collaboration and respect among contributors
- [x] Requirements consider shared learning opportunities
- [x] Community impact is considered in feature design

---

## Execution Status
*Updated by main() during processing*

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked (none found)
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

---
