# Feature Specification: Cross-Platform Assembly Code Generator

**Feature Branch**: `006-the-feature-to`  
**Created**: mercoledì 15 ottobre 2025  
**Status**: Draft  
**Input**: User description: "The feature to be implemented is an x86_64 asm code generator that uses the nasm syntax. This generator must be OS independent (Windows, Linux, MacOS) and must use the Windows and SystemV ABIs. The ABIs are automatically selected based on the OS. For example, Windows uses the Windows ABI, Linux and MacOS use the SystemV ABI. These components are currently defined in the src/asm folder, which contains the following files: - abi.rs contains the ABI enum and the functions: from_platform, alignment, red_zone, shadow_space, int_param_registers, float_param_registers, int_return_registers, float_return_registers, callee_saved_gp_registers, callee_saved_xmm_registers, caller_saved_gp_registers, caller_saved_xmm_registers, is_callee_saved, is_caller_saved, requires_frame_pointer, struct_return_pointer_register, max_struct_return_size, stack_param_order_is_left_to_right, variadic_info, scratch_register, name, and the VariadicInfo enum - data_directive.rs contains the DataDirective, AssemblyElement, AssemblySection enums and the AssemblySection-related functions add_label, add_data, add_instruction, add_comment, add_empty_line, text_section, data_section, bss_section, and rodata_section - instruction.rs Immediate enum and related functions - register.rs enum Platform, GPRegister64, GPRegister32, GPRegister16, GPRegister8, FPURegister, MMXRegister, XMMRegister, YMMRegister, ZMMRegister, MaskRegister, SegmentRegister, ControlRegister, DebugRegister, FlagsRegister, InstructionPointer, X86Register, and related functions - section.rs contains the Section enum and related functions. The generator must take the IR (intermediate representation) as input. Once it receives the IR, it must scan it. Each IR instruction must be processed with great precision and generate the appropriate instructions. These instructions must create an ASM program valid for all the ABIs treated. Specifically, the ASM code must be well-formed, with no logical or system errors."

## Clarifications

### Session 2025-10-15

- Q: What is the target rate for processing IR instructions during assembly code generation? → A: 1,000 IR instructions per second
- Q: What specific IR format will be used as input to the code generator? → A: Custom internal IR format specific to this project
- Q: What should be the system's error handling behavior when the generator encounters invalid IR input? → A: Generate detailed error messages with line numbers and validation
- Q: What should be the specific output format of the generated assembly code? → A: Text-based assembly file (.asm or .s) that can be assembled separately
- Q: What is the minimum x86_64 feature set that the generated assembly code should be compatible with? → A: Baseline x86_64 without optional extensions
- Q: What level of observability should the assembly code generator provide during execution? → A: Significant events (start/end generation, ABI selection, fatal errors)
- Q: When the generator encounters an IR operation that requires CPU functionality not available in the x86_64 baseline instruction set, what behavior should it adopt? → A: Reject the generation with a detailed error indicating the unsupported operation
- Q: How should the generator behave when it encounters complex data structures in the IR that exceed the maximum size for direct return by value defined by the ABI? → A: Automatically use hidden pointer return as specified by the ABI.
- Q: How should the generator behave when it encounters calls to deeply nested functions that might exceed resource allocation limits? → A: Do not apply limits, generate correct code, and delegate overflow to the operating system.
- Q: When the generated assembly code is passed to the external assembler and assembly fails, what error reporting strategy should the system adopt? → A: Analyze assembler output and enrich it with context by linking errors to the original IR

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate Target Assembly Code from IR (Priority: P1)

Developers need to convert intermediate representation (IR) code to target assembly code that is compatible with the operating system's application binary interface (ABI). The generated code must be platform-appropriate and follow the calling conventions of the target system.

**Why this priority**: This is the core functionality that enables the entire purpose of the feature - transforming IR to executable assembly code that can be used across multiple platforms.

**Independent Test**: The system can accept IR input and produce correct assembly code that compiles without errors and follows the appropriate ABI for the target platform.

**Acceptance Scenarios**:

1. **Given** a valid IR input, **When** the code generator processes it on Windows, **Then** the output assembly code follows Windows ABI conventions
2. **Given** a valid IR input, **When** the code generator processes it on Linux or MacOS, **Then** the output assembly code follows SystemV ABI conventions
3. **Given** complex IR with function calls, parameters, and return values, **When** the code generator processes it, **Then** the assembly code correctly handles parameter passing and calling conventions for the target ABI

---

### User Story 2 - Support Cross-Platform ABI Selection (Priority: P2)

The system must automatically detect the target platform and select the appropriate application binary interface (ABI) to ensure generated code adheres to platform-specific calling conventions and register usage.

**Why this priority**: This ensures compatibility across different operating systems without requiring user intervention to specify the ABI manually.

**Independent Test**: The system correctly identifies the target platform and generates assembly code that follows the appropriate ABI conventions without user needing to specify it.

**Acceptance Scenarios**:

1. **Given** a compilation target is specified as Windows, **When** the code generator runs, **Then** it selects Windows ABI and generates code accordingly
2. **Given** a compilation target is specified as Linux or MacOS, **When** the code generator runs, **Then** it selects SystemV ABI and generates code accordingly

---

### User Story 3 - Generate Well-Formed Assembly with Proper Resource Management (Priority: P3)

The generator must handle resource allocation properly according to ABI specifications, including parameter passing mechanisms, return value handling, and resource preservation requirements to ensure correct execution and system stability.

**Why this priority**: This ensures generated code doesn't violate ABI requirements that could lead to runtime errors or system instability.

**Independent Test**: Generated assembly code correctly follows resource usage conventions for the target ABI and maintains proper resource state across function calls.

**Acceptance Scenarios**:

1. **Given** IR with function parameters and local variables, **When** the code generator processes it, **Then** it correctly handles parameter passing as defined by the ABI
2. **Given** IR with function returns, **When** the code generator processes it, **Then** it correctly handles return values as defined by the ABI
3. **Given** IR with nested function calls, **When** the code generator processes it, **Then** it properly preserves required resources as required by the ABI

---

### Edge Cases

- **Unsupported operations**: When the IR contains operations not supported by the baseline x86_64 instruction set, the generator MUST reject generation and produce a detailed error message indicating the unsupported operation and required instruction set
- **Large structure returns**: When the IR contains complex data structures that exceed the ABI-defined maximum size for direct return by value, the generator MUST automatically use the hidden pointer return mechanism (caller allocates space, callee receives pointer as implicit first parameter and writes result there)
- **Deeply nested function calls**: The generator does not impose artificial limits on function call nesting depth; it generates ABI-compliant code and delegates stack overflow handling to the operating system's runtime mechanisms
- **Assembler failures**: When the external assembler (NASM) fails to assemble the generated code, the system MUST capture and analyze the assembler's output, enrich error messages with context linking assembly errors back to the original IR constructs when possible, and present actionable diagnostics to the user
- What happens when the IR contains operations that require specific processor features not available on all target systems?
- What happens when the IR contains code that would cause ABI violations?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept intermediate representation (IR) code as input and process it to generate corresponding target assembly instructions
- **FR-002**: System MUST generate text-based assembly files (.asm or .s) that are compatible with the target platform's assembler
- **FR-003**: System MUST automatically detect or accept target platform specification to select appropriate ABI (Windows ABI for Windows, SystemV ABI for Linux/MacOS) 
- **FR-004**: System MUST correctly map function parameters to appropriate resources according to the selected ABI
- **FR-005**: System MUST handle return values according to the calling convention of the selected ABI
- **FR-006**: System MUST properly manage resource usage according to ABI specifications
- **FR-007**: System MUST generate properly aligned execution frames according to ABI alignment requirements
- **FR-008**: System MUST handle variadic functions according to the appropriate ABI specification
- **FR-009**: System MUST generate well-formed assembly code with correct syntax that assembles without errors
- **FR-010**: System MUST correctly handle data sections according to target platform conventions
- **FR-011**: System MUST preserve the logical structure and behavior of the original IR in the generated assembly
- **FR-012**: System MUST generate assembly code that, when assembled and linked, produces functionally equivalent executable code to the original IR
- **FR-013**: System MUST handle floating-point operations according to the appropriate ABI specification for parameter passing and resource usage
- **FR-014**: System MUST correctly implement platform-specific resource allocation when required by the ABI
- **FR-015**: System MUST automatically handle structure return values according to ABI size limits: structures within size limits are returned in registers, while larger structures use the hidden pointer mechanism (caller allocates, callee receives pointer as implicit first parameter)
- **FR-016**: System MUST generate detailed error messages with line numbers and validation when encountering invalid IR input
- **FR-017**: System MUST handle basic optimization only (focus on correctness over performance)
- **FR-018**: System MUST NOT generate debug information
- **FR-019**: System MUST generate code compatible with baseline x86_64 without optional extensions (no AVX, AVX2, etc.)
- **FR-020**: System MUST log significant events including generation start/end, ABI selection, and fatal errors to enable debugging and monitoring
- **FR-021**: System MUST reject code generation and produce a detailed error message when encountering IR operations that require CPU features beyond baseline x86_64
- **FR-022**: When external assembler (NASM) fails, system MUST capture assembler output, analyze it, and provide enriched error messages that link assembly errors to original IR constructs when possible

**Cross-References**:
- Implementation details for all Functional Requirements: See plan.md for architectural approach and implementation phases
- ABI-specific requirements implementation: Refer to plan.md section "ABI Compliance Implementation"

### Key Entities

- **ABI (Application Binary Interface)**: Defines how functions receive parameters, return values, and manage resources based on the target platform
- **Intermediate Representation (IR)**: Custom internal IR format specific to this project, containing the logical operations to be converted to target assembly code
- **Assembly Code**: Text-based assembly output (.asm or .s) compatible with baseline x86_64 without optional extensions, representing the same logical operations as the IR
- **Platform**: Operating system environment (Windows, Linux, or MacOS) that determines which ABI to use
- **Resources**: Platform-specific allocations that must follow ABI conventions for parameter passing and value storage

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Generated assembly code successfully assembles with appropriate assembler without syntax errors 100% of the time
- **SC-002**: Generated assembly code preserves the original functionality of the IR input, with functional equivalence verified through testing 100% of the time
- **SC-003**: System supports both Windows ABI and SystemV ABI with correct parameter and return value handling for all primitive data types
- **SC-004**: Generated assembly code follows all platform-specific requirements (resource alignment, calling conventions, resource usage) as defined by the selected ABI
- **SC-005**: All generated assembly programs execute correctly without runtime errors related to ABI violations
- **SC-006**: Assembly code generation processes at least 1,000 IR instructions per second (as clarified in Session 2025-10-15)
- **SC-007**: Assembly code generation completes within acceptable time limits (less than 30 seconds for programs up to 10,000 IR instructions)