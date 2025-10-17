# Feature Specification: x86-64 NASM Assembly Code Generator

**Feature Branch**: `007-x86-64-asm-generator`  
**Created**: 2025-10-17  
**Status**: Draft  
**Input**: User description: "Create an Assembly code generator within the src/asm folder, where there are already many useful components and modules that can facilitate and support its implementation and creation. The generator must be able to produce Assembly code for the x86_64 architecture using NASM (Netwide Assembler) syntax. The generator receives as input the intermediate representation (IR), which is defined and structured in the src/ir folder of the project. Once the generator has received the input IR, it must proceed to translate it into corresponding Assembly instructions. The generated Assembly code must follow all established guidelines, programming conventions, and must be both logically and syntactically correct on all operating systems and platforms supported by the project. The generator must be extremely detailed, precise, accurate, and meticulous during the process of translating instructions from the intermediate IR representation into the corresponding Assembly instructions in order to minimize and minimize translation errors and inaccuracies. When the generator encounters a problem during the generation process-such as an unknown, unsupported, or malformed IR instruction-it must create an appropriate error message and add it to the list of accumulated errors. This complete list of errors will be returned along with the generated Assembly code, allowing any problems to be identified and corrected. Once the generation process is complete, the generated Assembly code should be saved in a file with the extension .asm on the file system."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Function Translation (Priority: P1)

Compiler developers need to translate simple IR functions containing arithmetic operations, local variables, and return statements into correct x86-64 NASM assembly that can be assembled and executed.

**Why this priority**: This is the foundational capability - without basic function translation, no compilation is possible. It demonstrates end-to-end working code generation and provides immediate value.

**Independent Test**: Can be fully tested by providing an IR function with basic arithmetic (add, subtract, multiply) and local variable assignments, generating assembly, assembling with NASM, linking, executing, and verifying the return value matches expected computation.

**Acceptance Scenarios**:

1. **Given** an IR function with integer arithmetic operations, **When** the generator processes the IR, **Then** valid x86-64 NASM assembly is produced that correctly performs the calculations
2. **Given** an IR function with local variable allocations and assignments, **When** the generator creates assembly, **Then** stack space is correctly allocated and variables are stored/loaded from proper stack offsets
3. **Given** generated assembly for a simple function, **When** assembled with NASM and executed, **Then** the function returns the correct computed value
4. **Given** an IR function using multiple data types (integers of different widths), **When** translated to assembly, **Then** correct register sizes and instructions are used for each type

---

### User Story 2 - Control Flow Translation (Priority: P2)

Compiler developers need to translate IR control flow constructs (conditional branches, loops, switches) into correct assembly jump instructions and labels that preserve program logic.

**Why this priority**: After basic translation works, control flow is essential for any real program. This enables compilation of if statements, loops, and complex branching logic.

**Independent Test**: Can be tested independently by providing IR with conditional branches and loops, generating assembly with proper labels and jumps, and verifying execution flow matches the IR semantics through test cases with different input values.

**Acceptance Scenarios**:

1. **Given** an IR function with conditional branches, **When** translated to assembly, **Then** correct conditional jump instructions (je, jne, jg, jl, etc.) are generated with proper label targets
2. **Given** an IR function with a loop construct, **When** converted to assembly, **Then** the loop structure is preserved with correct backward jumps and loop condition checks
3. **Given** an IR switch statement with multiple cases, **When** generated into assembly, **Then** an efficient jump table or conditional chain is created that correctly routes to each case
4. **Given** nested control flow structures in IR, **When** translated, **Then** unique labels are generated without conflicts and all jumps target the correct destinations

---

### User Story 3 - Function Calls and ABI Compliance (Priority: P3)

Compiler developers need to generate assembly that correctly implements function calls following the appropriate calling convention (System V for Linux/macOS, Microsoft x64 for Windows) with proper parameter passing and return value handling.

**Why this priority**: Function calls are necessary for modular programs, but basic single-function code generation must work first. This enables inter-procedural programs and library calls.

**Independent Test**: Can be tested by creating IR with function calls passing various parameters, generating assembly, and verifying parameters are placed in correct registers/stack locations per the ABI, and return values are handled correctly.

**Acceptance Scenarios**:

1. **Given** an IR function call with integer parameters, **When** translated to assembly, **Then** parameters are passed in the correct registers according to the target platform's ABI
2. **Given** an IR function call with more parameters than available parameter registers, **When** generated into assembly, **Then** excess parameters are correctly pushed onto the stack
3. **Given** an IR function call with floating-point parameters, **When** translated, **Then** float parameters are passed in appropriate XMM registers per the calling convention
4. **Given** an IR function that calls another function, **When** assembly is generated, **Then** caller-saved registers are preserved around the call and stack alignment is maintained

---

### User Story 4 - Memory Operations Translation (Priority: P4)

Compiler developers need to translate IR memory operations (load, store, allocate, address calculation) into correct assembly instructions that safely access memory with proper alignment and addressing modes.

**Why this priority**: Memory operations are essential for working with data structures and arrays, but basic computation and control flow are prerequisites.

**Independent Test**: Can be tested with IR containing alloca, load, store, and GetElementPtr instructions, verifying the generated assembly correctly allocates stack space, accesses memory at correct addresses, and handles pointer arithmetic.

**Acceptance Scenarios**:

1. **Given** IR alloca instructions for local variables, **When** translated to assembly, **Then** stack space is allocated with correct alignment for the data types
2. **Given** IR load and store instructions, **When** converted to assembly, **Then** correct mov instructions with appropriate memory operands access the specified addresses
3. **Given** IR GetElementPtr instructions for array indexing, **When** generated into assembly, **Then** address calculations using lea or indexed addressing modes correctly compute element addresses
4. **Given** IR pointer operations, **When** translated, **Then** assembly uses appropriate 64-bit registers and addressing modes for pointer manipulation

---

### User Story 5 - Type Conversions and Casts (Priority: P5)

Compiler developers need to translate IR type conversion instructions (cast operations like sign extension, zero extension, truncation, float-to-int conversions) into correct assembly instructions that preserve data semantics.

**Why this priority**: Type conversions are common but depend on basic arithmetic and memory operations working correctly first.

**Independent Test**: Can be tested with IR containing various Cast instructions, generating assembly, executing test cases, and verifying converted values are mathematically correct according to the cast semantics.

**Acceptance Scenarios**:

1. **Given** IR integer sign extension casts, **When** translated to assembly, **Then** movsx instructions correctly sign-extend smaller integers to larger sizes
2. **Given** IR integer zero extension casts, **When** converted, **Then** movzx instructions correctly zero-extend unsigned integers
3. **Given** IR integer truncation casts, **When** generated into assembly, **Then** appropriate register size aliases are used to access the lower bits
4. **Given** IR float-to-integer conversions, **When** translated, **Then** cvttss2si/cvttsd2si instructions correctly convert with truncation toward zero
5. **Given** IR integer-to-float conversions, **When** generated, **Then** cvtsi2ss/cvtsi2sd instructions correctly convert with proper precision

---

### User Story 6 - Error Detection and Reporting (Priority: P6)

Compiler developers need clear, actionable error messages when the generator encounters unsupported, malformed, or invalid IR constructs, allowing them to identify and fix issues.

**Why this priority**: Error handling is important for developer experience but should not block basic functionality from working.

**Independent Test**: Can be tested by providing deliberately malformed or unsupported IR constructs and verifying that the generator collects appropriate error messages without crashing and returns them with the (potentially incomplete) generated code.

**Acceptance Scenarios**:

1. **Given** IR containing an unknown instruction opcode, **When** the generator processes it, **Then** an error is recorded identifying the unknown instruction and its location
2. **Given** IR with a malformed instruction (missing required operands), **When** translation is attempted, **Then** a descriptive error message explains what is malformed and where
3. **Given** IR using an unsupported feature, **When** generation proceeds, **Then** an error message clearly states the feature is not yet supported
4. **Given** multiple errors during generation, **When** the process completes, **Then** all errors are collected and returned as a comprehensive list for debugging
5. **Given** partial IR translation with errors, **When** the generator finishes, **Then** valid assembly code is generated for successful portions while errors are reported for failed portions

---

### User Story 7 - Cross-Platform Assembly Generation (Priority: P7)

Compiler developers need to generate assembly code that is correct for different operating systems (Windows, Linux, macOS) by adapting calling conventions, system call interfaces, and platform-specific requirements.

**Why this priority**: Cross-platform support is valuable but can be addressed incrementally after core generation works on one platform.

**Independent Test**: Can be tested by generating assembly for the same IR on different target platforms, assembling and executing on each platform, and verifying behavior is correct for each platform's conventions.

**Acceptance Scenarios**:

1. **Given** IR compiled for Linux target, **When** assembly is generated, **Then** System V AMD64 ABI calling convention is used
2. **Given** IR compiled for Windows target, **When** assembly is generated, **Then** Microsoft x64 calling convention is used including shadow space
3. **Given** IR compiled for macOS target, **When** assembly is generated, **Then** System V calling convention is used with macOS-specific symbol mangling
4. **Given** IR with system calls, **When** translated for different platforms, **Then** correct syscall instructions or library calls are generated per platform

---

### User Story 8 - Floating-Point Operations (Priority: P8)

Compiler developers need to translate IR floating-point arithmetic operations into correct SSE2/AVX assembly instructions that produce accurate numerical results.

**Why this priority**: Floating-point is important for numerical programs but is a specialized feature that can be added after integer operations work.

**Independent Test**: Can be tested with IR containing float and double arithmetic operations, generating assembly using SSE/AVX instructions, executing, and comparing numerical results against expected values within acceptable precision.

**Acceptance Scenarios**:

1. **Given** IR with single-precision float arithmetic, **When** translated to assembly, **Then** appropriate SSE scalar instructions (addss, mulss, divss, subss) are generated
2. **Given** IR with double-precision float arithmetic, **When** converted, **Then** appropriate SSE scalar instructions (addsd, mulsd, divsd, subsd) are generated
3. **Given** IR float comparisons, **When** generated into assembly, **Then** ucomiss/ucomisd instructions followed by appropriate conditional jumps are used
4. **Given** IR mixing integer and float operations, **When** translated, **Then** correct register classes (GPR vs XMM) are used and conversions are properly inserted

---

### User Story 9 - Output File Management (Priority: P9)

Compiler developers need the generated assembly to be automatically saved to a file with .asm extension in a predictable location for subsequent assembly and linking steps.

**Why this priority**: File output is convenient but not essential for testing - assembly can initially be returned as a string and written separately.

**Independent Test**: Can be tested by running the generator with an output path specification, verifying the .asm file is created at the expected location, and confirming it contains valid NASM-formatted assembly text.

**Acceptance Scenarios**:

1. **Given** a specified output file path, **When** generation completes successfully, **Then** a .asm file is created at that path containing the generated assembly
2. **Given** no explicit output path is provided, **When** generation completes, **Then** a .asm file is created in the same directory as the input .vn file with the same base name (e.g., input.vn → input.asm)
3. **Given** an output directory that doesn't exist, **When** file writing is attempted, **Then** the directory is created automatically or an error is reported
4. **Given** generation with errors, **When** output is written, **Then** the partial assembly is still written to the file (in the same directory as the input .vn file by default) and errors are reported separately
5. **Given** multiple functions in the IR module, **When** saved to file, **Then** all functions are included in the output .asm file with proper section organization

---

### Edge Cases

- Phi functions from SSA form are resolved using critical edge splitting, with move instructions inserted at the end of predecessor blocks before the terminator jump
- How does the generator handle IR functions with no return statement (unreachable terminator)?
- IR types that don't directly map to x86-64 registers (e.g., i128, custom structs, arrays as values) generate errors; only the types listed in FR-031 are supported
- Pointer types are translated as 64-bit integer registers (rax, rbx, etc.) when used as values; when dereferenced, they serve as base addresses for memory operands
- What happens when IR contains indirect branches with unknown target labels?
- Deep expression nesting requiring many temporary registers is handled through the register spilling mechanism without imposing artificial nesting depth limits
- Register spilling is triggered when all physical registers are allocated, using furthest-use heuristic to select spill candidates
- How are IR string constants and array constants represented in the generated assembly's data section?
- What happens when IR function calls exceed the available parameter registers for the calling convention?
- How does the generator handle IR vector operations if SIMD support is limited or absent?
- What happens when generating assembly for unsupported target platforms specified in the IR module?
- How are IR debug information and source spans preserved or represented in assembly comments?
- What happens when IR contains recursive function calls requiring stack frame management?
- How does the generator handle IR with variadic function calls?
- What happens when the IR uses calling conventions that don't match the target platform's native convention?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Generator MUST accept an IR Module as input containing one or more functions with their control flow graphs
- **FR-002**: Generator MUST translate IR instructions from supported InstructionKind variants (Alloca, Store, Load, Binary, Unary, Call, GetElementPtr, Cast, Phi) into equivalent x86-64 assembly instructions; **Note**: Vector operations are NOT supported and MUST generate UnsupportedType errors per FR-031
- **FR-003**: Generator MUST translate IR terminators (Return, Branch, ConditionalBranch, Switch) into appropriate control flow assembly (ret, jmp, conditional jumps, jump tables)
- **FR-004**: Generator MUST emit assembly in valid NASM syntax that can be assembled without errors
- **FR-005**: Generator MUST respect the target platform's calling convention (System V AMD64 ABI for Linux/macOS, Microsoft x64 for Windows) when generating function prologues, epilogues, and call sites
- **FR-006**: Generator MUST allocate stack space for local variables based on IR alloca instructions with appropriate alignment
- **FR-007**: Generator MUST translate IR binary operations (Add, Subtract, Multiply, Divide, Modulo, comparison operations, logical operations, bitwise operations, shifts) into correct x86-64 arithmetic and logic instructions
- **FR-008**: Generator MUST translate IR unary operations (Negate, Not) into appropriate assembly instructions (neg, not, or equivalent sequences)
- **FR-009**: Generator MUST implement register allocation using linear scan algorithm to assign IR temporary values to x86-64 physical registers
- **FR-010**: Generator MUST generate correct memory addressing operands for IR Load and Store instructions based on the address value
- **FR-011**: Generator MUST translate IR GetElementPtr instructions into address calculations using lea, imul, and add instructions as needed
- **FR-012**: Generator MUST handle IR Cast instructions by emitting appropriate conversion instructions (movsx, movzx, cvt* family, register aliasing for truncations)
- **FR-013**: Generator MUST translate IR function calls into call instructions with parameters placed according to the calling convention
- **FR-014**: Generator MUST preserve caller-saved registers around function calls and restore callee-saved registers before returning
- **FR-015**: Generator MUST ensure stack alignment requirements of the target ABI are maintained (16-byte alignment at call sites)
- **FR-016**: Generator MUST generate unique labels for each IR basic block to serve as jump targets
- **FR-017**: Generator MUST emit assembly directives to define appropriate sections (section .text, section .data, section .bss, section .rodata)
- **FR-018**: Generator MUST place IR string constants and global constants in the appropriate data sections with proper labels and alignment: **String literals** are placed in `.rodata` section with null terminator (e.g., `.str0: db "hello", 0`), 1-byte aligned; **Numeric array constants** are placed in `.rodata` with natural element alignment (e.g., `align 4` for I32 arrays, `align 8` for I64/F64 arrays) using labels `.arr0:`, `.arr1:`, etc.; **Scalar global constants** use appropriate alignment (1/2/4/8 bytes per FR-021 type mapping); Generator MUST track emitted constants to avoid duplication (identical string/array values share the same label)
- **FR-019**: Generator MUST generate a function prologue that sets up the stack frame (push rbp, mov rbp rsp, sub rsp for locals); stack allocation MUST include space for local variables, spill slots, and shadow space per FR-039 (if Windows x64 and function makes calls)
- **FR-020**: Generator MUST generate a function epilogue that tears down the stack frame and returns (mov rsp rbp, pop rbp, ret or leave, ret)
- **FR-021**: Generator MUST select appropriate instruction sizes (byte, word, dword, qword) based on IR type information according to the following mapping: **I8/U8 → byte (8-bit)**, **I16/U16 → word (16-bit)**, **I32/U32 → dword (32-bit)**, **I64/U64 → qword (64-bit)**, **F32 → dword (32-bit single-precision)**, **F64 → qword (64-bit double-precision)**, **Bool → byte (1 byte, 0=false, 1=true)**, **Char → dword (4 bytes, UTF-32 code point)**, **Pointer → qword (8 bytes on x86-64)**
- **FR-022**: Generator MUST use appropriate register sizes (r8, r16, r32, r64) matching the IR operand types according to FR-021 size mapping (byte→r8 like al/bl, word→r16 like ax/bx, dword→r32 like eax/ebx, qword→r64 like rax/rbx)
- **FR-023**: Generator MUST translate IR floating-point operations into SSE2 scalar instructions (addss, addsd, mulss, mulsd, etc.)
- **FR-024**: Generator MUST use XMM registers for floating-point values and operations
- **FR-025**: Generator MUST handle IR conditional branches by generating comparison instructions followed by conditional jumps (cmp + je/jne/jg/jl/ja/jb)
- **FR-026**: Generator MUST implement IR phi functions by splitting critical edges in the control flow graph and generating appropriate move instructions at the end of predecessor blocks to resolve phi nodes correctly
- **FR-027**: Generator MUST collect error messages when encountering genuinely unsupported or malformed IR constructs without terminating the generation process; core supported features MUST produce correct assembly without errors; generation MUST continue for valid portions, skipping only the problematic instruction; if a terminator instruction fails, the generator MUST emit an unreachable/trap instruction as fallback to maintain valid block structure
- **FR-028**: Generator MUST return both the generated assembly code (including partial code from successful portions) and a list of all collected error messages
- **FR-029**: Error messages MUST include sufficient context to identify the problematic IR construct: **minimum required context** includes (1) instruction kind or construct type, (2) source location as SourceSpan (file path, line number, column number), (3) operand types involved, and (4) one-sentence explanation of why the error occurred
- **FR-030**: Generator MUST save the generated assembly output to a file with .asm extension; when no explicit output path is provided, the file MUST be written to the same directory as the input .vn source file with the same base name. Path resolution follows these rules: (1) if input path is absolute, output directory is the absolute directory of the input file; (2) if input path is relative, output directory is resolved relative to current working directory, then output file is placed in the resolved input file's directory; (3) if explicit output path is provided, it is used as-is (absolute or relative to current working directory)
- **FR-031**: Generator MUST support generating assembly for the IR types I8, I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Char, Pointer, and Void; types not in this list (e.g., I128, custom structs, arrays as first-class values) MUST generate errors
- **FR-032**: Generator MUST handle IR void-returning functions by generating return without a value (ret without operands)
- **FR-033**: Generator MUST generate assembly that implements the correct signed vs unsigned semantics for IR operations by checking the IR value type signedness (I8-I64 are signed, U8-U64 are unsigned) and selecting appropriate instructions: **signed types use imul, idiv, jg, jl, jge, jle**; **unsigned types use mul, div, ja, jb, jae, jbe**; this applies to multiplication, division, and comparison operations where signed/unsigned distinction affects results
- **FR-034**: Generator MUST respect the target platform specified in the IR Module metadata (Linux, Windows, macOS) when selecting conventions
- **FR-035**: Generator MUST emit proper symbol visibility and linkage directives (global, extern) for IR functions based on their attributes
- **FR-036**: Generator MUST handle IR comparison operations by generating comparison instructions and capturing results appropriately: if the comparison result is used by a conditional branch terminator, results remain in CPU flags (ZF, CF, SF, OF); if the comparison result is used by a non-branch instruction (e.g., assignment, arithmetic), the generator MUST use setcc instructions (sete, setne, setg, setl, seta, setb, etc.) to materialize the boolean result (0 or 1) into a register
- **FR-037**: Generator MUST manage the x87 FPU stack or avoid x87 instructions, preferring SSE for floating-point operations
- **FR-038**: Generator MUST handle register pressure by implementing register spilling to stack temporaries when all physical registers are allocated; spilling MUST select the value with the furthest next use (standard linear scan furthest-use heuristic)
- **FR-039**: Generator MUST generate correct shadow space (32 bytes) on the stack for Windows x64 calling convention; shadow space MUST be allocated once in the function prologue if the function makes any calls and reused for all call sites
- **FR-040**: Generator MUST translate IR GetElementPtr with array indices into scaled index addressing modes when possible (e.g., mov rax, [base + index*8])

### Key Entities

- **IR Module**: Input entity representing the complete program with functions, data layout, target triple, and root scope
- **IR Function**: A function within the module containing parameters, return type, control flow graph, local variables, and attributes (entry point, calling convention, varargs)
- **Control Flow Graph (CFG)**: Graph of basic blocks with edges representing control flow, used to traverse the function during generation
- **Basic Block**: Sequence of IR instructions with a label, ending in a terminator, serving as unit of control flow
- **IR Instruction**: Individual operation (alloca, load, store, binary op, unary op, call, cast, phi, GEP, vector op) with operands and result
- **IR Terminator**: Control flow operation (return, branch, conditional branch, switch, indirect branch, unreachable) ending a basic block
- **IR Value**: Represents a value (literal, constant, local variable, global, temporary) with a type, used as operand in instructions
- **IR Type**: Type information (primitives I8-I64/U8-U64/F32/F64/Bool/Char/String/Void, Pointer, Array, Custom, Struct) determining assembly representation
- **Assembly Instruction**: Generated x86-64 instruction (mov, add, jmp, call, etc.) with operands (registers, immediates, memory, labels)
- **Assembly Section**: Organizational unit (.text, .data, .bss, .rodata) containing related assembly elements
- **Register**: x86-64 physical register (GPR, XMM, YMM, etc.) allocated to IR values during generation
- **Calling Convention (ABI)**: Rules for parameter passing, return values, register preservation (System V, Microsoft x64) based on target platform
- **Stack Frame**: Per-function memory layout containing local variables, saved registers, return address, managed by prologue/epilogue
- **Label**: Named target for jumps, representing basic blocks or data locations in the generated assembly
- **Memory Operand**: Assembly addressing mode (base + index*scale + displacement) representing memory access
- **Error Message**: Diagnostic information about unsupported or malformed IR constructs encountered during generation, includes context and location
- **Symbol Directive**: NASM directive controlling symbol visibility and linkage per FR-035:
  - **`global <symbol>`**: Exports symbol (used for External linkage functions with `entry` attribute, or any External function with a body)
  - **`extern <symbol>`**: Imports symbol (used for External linkage functions without a body—forward declarations)
  - **Internal linkage**: No directive emitted (function is local to the module)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Generated assembly for a basic arithmetic function (addition, subtraction, multiplication of integers) can be assembled with NASM and executed to produce the correct return value
- **SC-002**: Generator successfully translates all IR instruction kinds present in the project's test suite IR examples without crashing or producing incorrect assembly for core supported features
- **SC-003**: Generated assembly for functions with conditional branches correctly implements control flow logic verified by test cases with different input values
- **SC-004**: Assembly generated for function calls with 1-6 integer parameters passes parameters in the correct registers according to System V or Microsoft x64 calling conventions
- **SC-005**: Generated assembly for functions allocating local variables correctly reserves stack space and accesses variables at proper offsets without corruption
- **SC-006**: Floating-point arithmetic functions generate assembly using SSE2 instructions that produce numerically correct results (within floating-point precision)
- **SC-007**: When generator encounters unsupported IR constructs, error messages clearly identify the construct type and location, enabling developer diagnosis
- **SC-008**: Generator completes translation of the project's test IR modules in under 1 second per 1000 IR instructions as a best-effort performance goal, measured relative to baseline benchmarks established during initial implementation. This is informational guidance tracked via Criterion benchmarks, not a hard acceptance criterion that would reject the implementation. Performance targets are relative to the development environment where benchmarks are run.
- **SC-009**: Generated assembly files are valid NASM syntax verified by assembling without errors using nasm -f elf64 (Linux) or nasm -f win64 (Windows)
- **SC-010**: Cross-platform IR module targeting Linux generates System V ABI-compliant assembly, while the same IR targeting Windows generates Microsoft x64-compliant assembly
- **SC-011**: 95% of IR instructions in typical compiler test cases translate to assembly without requiring manual intervention or error workarounds
- **SC-012**: Generated assembly code size is within 3x of hand-optimized assembly for equivalent functionality (allowing for unoptimized code generation)
- **SC-013**: All generated assembly includes properly organized sections (.text, .data, .bss) with correct directives and labels
- **SC-014**: Register allocation successfully handles functions with up to 20 live values without generating invalid assembly
- **SC-015**: Generator produces a non-empty list of errors when given deliberately malformed IR test cases, with zero false negatives on known-invalid IR

## Assumptions *(if applicable)*

### Technical Assumptions

- **A-001**: The IR input provided to the generator has already been validated and is structurally well-formed (basic blocks have terminators, control flow graph is properly constructed, types are consistent)
- **A-002**: The IR may be in SSA form with phi functions; the generator will resolve phi nodes into conventional move instructions using critical edge splitting and predecessor block resolution
- **A-003**: The existing `src/asm` infrastructure (register definitions, instruction enums, ABI implementations, section management) is stable and provides the necessary building blocks for the generator
- **A-004**: The target platforms supported are limited to x86-64 architecture (not x86-32, ARM, or other architectures)
- **A-005**: NASM assembler is available in the development and deployment environment for assembling the generated code
- **A-006**: The generator will produce unoptimized assembly initially; optimization passes are out of scope for the initial implementation
- **A-007**: Register allocation will use linear scan algorithm with simple spilling to stack when register pressure is high; advanced graph-coloring allocators are out of scope
- **A-008**: The IR does not contain inline assembly or architecture-specific intrinsics that bypass IR abstractions
- **A-009**: System calls and external library functions are represented as normal function calls in the IR, not special instructions
- **A-010**: Memory operations in the IR assume a flat memory model with explicit pointer types
- **A-011**: The generator has access to the platform-specific ABI information through the existing `Abi` struct in `src/asm/abi.rs`
- **A-012**: Error handling during generation accumulates errors rather than using exceptions or early termination, allowing partial code generation
- **A-013**: The file system is writable when the generator needs to save output to a .asm file
- **A-014**: The generator can assume standard C library conventions for external function calls (e.g., main returning int, printf following platform ABI)
- **A-015**: Alignment requirements for stack allocations follow standard platform conventions (local variables aligned to their natural alignment, stack 16-byte aligned at calls)

### Domain Assumptions

- **A-016**: Compiler developers using the generator understand basic x86-64 assembly concepts and can interpret the generated code
- **A-017**: The IR accurately represents the semantics of the source program; any source-level bugs are already present in the IR
- **A-018**: Performance of the generated code is acceptable for initial correctness testing; optimization is a separate concern
- **A-019**: The generator's primary users are developers working on the jsavrs compiler project itself, not end-users of compiled programs
- **A-020**: Standard development tools (assembler, linker, debugger) are available for working with the generated assembly code

## Dependencies *(if applicable)*

### Internal Dependencies

- **D-001**: IR module system (`src/ir/module.rs`) providing the Module, DataLayout, and TargetTriple types
- **D-002**: IR function representation (`src/ir/function.rs`) providing Function, CFG, and parameter information
- **D-003**: IR basic block and instruction structures (`src/ir/basic_block.rs`, `src/ir/instruction.rs`, `src/ir/terminator.rs`) defining the IR constructs to be translated
- **D-004**: IR type system (`src/ir/types.rs`) for determining assembly operand sizes and instruction selection
- **D-005**: IR value system (`src/ir/value/`) for understanding operand types (literals, constants, locals, globals, temporaries)
- **D-006**: Assembly infrastructure (`src/asm/register.rs`, `src/asm/instruction.rs`, `src/asm/data_directive.rs`, `src/asm/section.rs`) providing the target assembly representation
- **D-007**: ABI implementations (`src/asm/abi.rs`) for calling convention rules specific to each platform
- **D-008**: Error reporting system (`src/error/`) for creating structured error messages with source locations

### External Dependencies

- **D-009**: NASM assembler for validating and assembling the generated .asm files
- **D-010**: Operating system (Linux, Windows, macOS) providing the execution environment and system ABI
- **D-011**: System linker (ld, link.exe) for linking assembled object files into executables during testing
- **D-012**: File system for reading IR input and writing .asm output files

### Assumed Capabilities

- **D-013**: The IR generator (`src/ir/generator.rs`) correctly produces well-formed IR from the source AST
- **D-014**: The IR type system provides sufficient information to determine correct assembly instruction sizes
- **D-015**: The existing register definitions accurately reflect x86-64 architecture registers and their properties
- **D-016**: The ABI structs correctly encode the calling convention rules for all supported platforms
- **D-017**: The control flow graph provides accurate successor/predecessor information for basic blocks

## Clarifications

### Session 2025-10-17

- Q: The specification mentions generating assembly code with "minimal translation errors" but doesn't define what level of correctness is expected during the initial implementation phase. → A: All core functionality must produce correct assembly; errors only for genuinely unsupported features
- Q: The specification mentions register allocation but doesn't specify the concrete strategy to use initially. This impacts implementation complexity and testing approach. → A: Linear scan register allocation with simple spilling to stack
- Q: The specification mentions saving output to ".asm files" but doesn't specify where these files should be written by default when no explicit path is provided. → A: Write to same directory as input .vn file
- Q: The specification mentions phi function handling but doesn't specify the concrete approach for resolving SSA phi nodes into conventional assignments during code generation. → A: Critical edge splitting with phi resolution at predecessor block ends
- Q: The specification mentions performance expectations ("under 1 second per 1000 IR instructions") but doesn't specify how the generator should handle scenarios where this performance target cannot be met due to very large or complex functions. → A: No special handling; performance target is best-effort goal, not hard requirement
- Q: When the generator encounters register pressure (more live values than available physical registers), the spec mentions "register spilling to stack" but doesn't specify the spilling strategy's aggressiveness or when to trigger spilling. → A: Spill the furthest-use value when all registers are full
- Q: The specification describes error accumulation for "unsupported or malformed IR constructs" but doesn't specify whether generation should continue after encountering an error or halt immediately for that function. → A: Continue generating code for valid portions; skip only the problematic instruction/block
- Q: The specification mentions handling "IR with very deep expression nesting requiring many temporary registers" as an edge case but doesn't specify the maximum expression nesting depth the generator should support before failing or using alternative strategies. → A: No hard limit; rely on spilling
- Q: The specification lists several IR types that must be supported (I8-I64, U8-U64, F32, F64, Bool, Char, Pointer) but the edge cases mention "types that don't directly map to x86-64 registers (e.g., i128, custom structs)" without specifying how to handle them. → A: Generate error for unsupported types (i128, structs); support only listed types
- Q: The edge cases mention "How are IR string constants and array constants represented in the generated assembly's data section?" but don't provide an answer. → A: String constants are placed in .rodata section with unique labels (e.g., `.str0: db "hello", 0` with null terminator), aligned to 1-byte boundary. Numeric array constants are placed in .rodata with element-size alignment (e.g., `align 4` for i32 arrays, `align 8` for i64 arrays). Array labels use format `.arr0:`, `.arr1:`, etc. See FR-018 for data section placement requirements.

## Out of Scope *(clarify what this feature does NOT include)*

- **OS-001**: Optimization passes (instruction scheduling, dead code elimination, constant folding, strength reduction, loop optimizations, etc.) - the generator produces correct but unoptimized code
- **OS-002**: Advanced register allocation algorithms (graph coloring with coalescing, linear scan with advanced splitting and spilling heuristics) - basic linear scan allocation suffices initially
- **OS-003**: Instruction selection based on performance heuristics - straightforward one-to-one translation is used
- **OS-004**: Support for architectures other than x86-64 (ARM, RISC-V, etc.)
- **OS-005**: Support for assemblers other than NASM (GAS, MASM, etc.) - syntax is NASM-specific
- **OS-006**: Inline assembly or architecture-specific intrinsics in the IR
- **OS-007**: Debugging information generation (DWARF, PDB, etc.) beyond basic comments
- **OS-008**: Automatic linking or creation of executable files - the generator only produces .asm source
- **OS-009**: Runtime support library or standard library implementation
- **OS-010**: Exception handling or unwinding table generation
- **OS-011**: Thread-local storage (TLS) variable support
- **OS-012**: Position-independent code (PIC) generation for shared libraries
- **OS-013**: Profile-guided optimization or instrumentation
- **OS-014**: Advanced SIMD operations beyond basic SSE2 scalar floating-point (AVX-512, NEON equivalents, etc.)
- **OS-015**: Support for exotic calling conventions beyond System V and Microsoft x64
- **OS-016**: Automatic generation of object files (.o, .obj) - only .asm source is produced
- **OS-017**: Integration with build systems or build automation beyond file generation
- **OS-018**: Cross-compilation toolchain setup or management
- **OS-019**: Semantic verification that generated assembly matches IR semantics (this is assumed to be tested externally)
- **OS-020**: Performance benchmarking or profiling of generated code (testing concern, not generator concern)

