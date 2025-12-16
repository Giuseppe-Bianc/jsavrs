# Feature Specification: IR to x86-64 Assembly Code Generator

**Feature Branch**: `021-ir-x86-codegen`  
**Created**: 16 dicembre 2025  
**Status**: Draft  
**Input**: User description: "Build a code generator that transforms intermediate representation (IR) into x86-64 assembly code"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate Basic x86-64 Assembly from IR (Priority: P1)

As a compiler developer, I want to transform a validated IR module into x86-64 assembly code so that I can produce executable machine code from my intermediate representation.

**Why this priority**: This is the foundational capability - without basic IR-to-assembly translation, no other features are possible. Provides immediate value by completing the compilation pipeline.

**Independent Test**: Can be fully tested by providing a simple IR module with arithmetic operations and verifying the generated assembly file contains correct Intel syntax instructions.

**Acceptance Scenarios**:

1. **Given** an IR module with a single function containing arithmetic operations, **When** the code generator processes it, **Then** a valid assembly file is produced with correct .text section and Intel syntax instructions
2. **Given** an IR module with multiple data types (integers, floats), **When** the code generator processes it, **Then** appropriate sized registers and instructions are used (e.g., EAX for 32-bit, RAX for 64-bit)
3. **Given** an IR module with basic blocks and control flow, **When** the code generator processes it, **Then** unique labels are created for each block with correct jump instructions

---

### User Story 2 - Platform-Specific Code Generation (Priority: P1)

As a compiler developer targeting multiple operating systems, I want to generate assembly code that follows each platform's calling conventions so that my compiled code works correctly on Linux, macOS, and Windows.

**Why this priority**: Cross-platform support is essential for a production compiler. Without correct calling conventions, generated code will crash or produce incorrect results.

**Independent Test**: Can be tested by generating assembly for the same IR function targeting each platform and verifying the correct register usage for parameter passing.

**Acceptance Scenarios**:

1. **Given** an IR function with 4 parameters targeting Linux/macOS, **When** code is generated, **Then** parameters use RDI, RSI, RDX, RCX registers (System V AMD64)
2. **Given** an IR function with 4 parameters targeting Windows, **When** code is generated, **Then** parameters use RCX, RDX, R8, R9 registers (Microsoft x64)
3. **Given** an IR function targeting Windows, **When** code is generated, **Then** 32-byte shadow space is allocated on the stack
4. **Given** an IR function targeting Linux/macOS, **When** code is generated, **Then** 128-byte red zone optimization is respected

---

### User Story 3 - Function Prologue and Epilogue Generation (Priority: P1)

As a compiler developer, I want proper function prologues and epilogues generated so that stack frames are correctly managed and callee-saved registers are preserved.

**Why this priority**: Correct stack frame management is required for any function call to work properly. Incorrect register preservation causes program corruption.

**Independent Test**: Can be tested by generating a function that modifies callee-saved registers and verifying they are pushed/popped in prologue/epilogue.

**Acceptance Scenarios**:

1. **Given** an IR function, **When** code is generated, **Then** the prologue saves RBP and sets up the stack frame
2. **Given** an IR function that uses RBX register, **When** code is generated, **Then** RBX is saved in prologue and restored in epilogue
3. **Given** an IR function with local variables, **When** code is generated, **Then** stack space is allocated with 16-byte alignment

---

### User Story 4 - Register Allocation and Spilling (Priority: P2)

As a compiler developer, I want the code generator to allocate values to registers and spill to stack when necessary so that the generated code uses available registers efficiently.

**Why this priority**: Register allocation directly impacts performance. While basic allocation enables functional code, optimized allocation improves execution speed.

**Independent Test**: Can be tested by providing IR with more live values than available registers and verifying spill code is generated.

**Acceptance Scenarios**:

1. **Given** an IR function with few live values, **When** code is generated, **Then** values are kept in registers without spilling
2. **Given** an IR function with more live values than available registers, **When** code is generated, **Then** excess values are spilled to stack locations
3. **Given** an IR with SIMD operations, **When** code is generated, **Then** XMM0-XMM15 registers are used appropriately

---

### User Story 5 - SSA Phi Node Resolution (Priority: P2)

As a compiler developer using SSA form, I want phi nodes resolved into actual move instructions so that values are correctly transferred at control flow merge points.

**Why this priority**: Phi resolution is necessary for correct SSA handling, but depends on basic code generation working first.

**Independent Test**: Can be tested by providing IR with a simple if-else that merges into a phi node and verifying correct move instructions are placed.

**Acceptance Scenarios**:

1. **Given** an IR basic block with phi nodes, **When** code is generated, **Then** move instructions are inserted at predecessor block ends
2. **Given** a phi node with multiple predecessors, **When** code is generated, **Then** each predecessor has the correct source value moved
3. **Given** phi nodes that would create a swap, **When** code is generated, **Then** a temporary register or memory location is used to break the cycle

---

### User Story 6 - Data Section Generation (Priority: P2)

As a compiler developer, I want string literals, global variables, and constants placed in appropriate assembly sections so that data is correctly initialized and accessible.

**Why this priority**: Programs need data in addition to code. While not needed for pure computation, most real programs require global data.

**Independent Test**: Can be tested by generating IR with string literals and global variables, then verifying .data, .rodata, and .bss sections are correct.

**Acceptance Scenarios**:

1. **Given** IR with string literals, **When** code is generated, **Then** strings appear in .rodata with null termination
2. **Given** IR with initialized global variables, **When** code is generated, **Then** variables appear in .data with correct directives
3. **Given** IR with uninitialized global variables, **When** code is generated, **Then** variables appear in .bss section
4. **Given** an array global, **When** code is generated, **Then** proper alignment and size are specified

---

### User Story 7 - Function Call Generation (Priority: P2)

As a compiler developer, I want function calls to be correctly generated with proper argument passing and return value handling so that inter-procedural code works correctly.

**Why this priority**: Function calls are essential for modular programs. Depends on platform-specific calling conventions (P1) being implemented first.

**Independent Test**: Can be tested by generating IR that calls a function with various argument counts and types.

**Acceptance Scenarios**:

1. **Given** a function call with fewer arguments than register slots, **When** code is generated, **Then** all arguments are passed in registers
2. **Given** a function call with more arguments than register slots, **When** code is generated, **Then** excess arguments are pushed to stack
3. **Given** a function returning a float, **When** code is generated, **Then** return value is retrieved from XMM0
4. **Given** a function call in the middle of computations, **When** code is generated, **Then** caller-saved registers with live values are preserved

---

### User Story 8 - Debug Comments and Annotations (Priority: P3)

As a compiler developer debugging generated code, I want assembly output to include comments showing original variable names, source locations, and block boundaries so that I can trace issues back to source code.

**Why this priority**: Debug information improves developer experience but is not required for correct execution.

**Independent Test**: Can be tested by generating assembly and verifying comments contain original IR identifiers and source locations.

**Acceptance Scenarios**:

1. **Given** an IR instruction with source location, **When** code is generated, **Then** a comment shows the line number
2. **Given** an IR value with a named variable, **When** code is generated, **Then** a comment includes the variable name
3. **Given** a basic block boundary, **When** code is generated, **Then** a comment marks the block entry

---

### User Story 9 - Control Flow Optimization (Priority: P3)

As a compiler developer, I want unnecessary jumps eliminated when blocks fall through naturally so that the generated code is more efficient and readable.

**Why this priority**: Optimization improves code quality but is not required for correctness.

**Independent Test**: Can be tested by providing IR with consecutive blocks that can fall through and verifying no jump is generated between them.

**Acceptance Scenarios**:

1. **Given** two consecutive basic blocks where the first unconditionally jumps to the second, **When** code is generated, **Then** no jump instruction is emitted (fall-through)
2. **Given** a conditional branch where the false target is the next block, **When** code is generated, **Then** only the conditional jump for the true case is emitted

---

### User Story 10 - Code Generation Statistics (Priority: P3)

As a compiler developer analyzing code generation quality, I want statistics about the generated code so that I can measure and improve code generator performance.

**Why this priority**: Statistics aid optimization efforts but are not required for functional code generation.

**Independent Test**: Can be tested by generating code and verifying statistics output includes instruction counts and register usage.

**Acceptance Scenarios**:

1. **Given** code generation completes, **When** statistics are requested, **Then** total instruction count is reported
2. **Given** code generation completes, **When** statistics are requested, **Then** register usage breakdown is provided
3. **Given** code generation completes, **When** statistics are requested, **Then** spill count is reported

---

### Edge Cases

- What happens when an IR instruction has no valid x86-64 equivalent? (Error reported with clear message)
- How does the system handle IR with unreachable basic blocks? (Blocks may be omitted with a warning)
- What happens when stack frame exceeds maximum size? (Error reported before generation)
- How are very long switch statements handled? (Jump table generation when appropriate)
- What happens with recursive function calls? (Treated like any other call, proper frame setup)
- How are zero-sized types handled? (No storage allocated, operations optimized away)

## Requirements *(mandatory)*

### Functional Requirements

**Core Translation**

- **FR-001**: System MUST accept IR modules containing functions with control flow graphs, basic blocks, and instructions
- **FR-002**: System MUST convert arithmetic IR instructions (add, sub, mul, div, rem, and, or, xor, shl, shr) into equivalent x86-64 instructions
- **FR-003**: System MUST convert memory access instructions (load, store) into appropriate mov instructions with correct sizes
- **FR-004**: System MUST convert type conversion instructions (extend, truncate, float-to-int, int-to-float) into appropriate x86-64 conversion instructions
- **FR-005**: System MUST convert comparison instructions into cmp followed by appropriate conditional set or jump instructions
- **FR-006**: System MUST translate 8-bit, 16-bit, 32-bit, and 64-bit signed and unsigned integers to appropriate register sizes and instructions
- **FR-007**: System MUST translate 32-bit (single) and 64-bit (double) floating point values using SSE registers and instructions
- **FR-008**: System MUST translate booleans as byte values (0 for false, non-zero for true)
- **FR-009**: System MUST translate character types as byte values
- **FR-010**: System MUST translate arrays as contiguous memory regions with element size x count allocation
- **FR-011**: System MUST translate pointers as 64-bit values containing memory addresses
- **FR-012**: System MUST translate structs as memory regions with fields at computed offsets respecting alignment
- **FR-013**: System MUST resolve SSA phi nodes into move instructions placed at predecessor block ends

**Platform Support**

- **FR-014**: System MUST generate assembly for Linux x86-64 using System V AMD64 ABI
- **FR-015**: System MUST generate assembly for macOS x86-64 using System V AMD64 ABI with underscore name prefixing
- **FR-016**: System MUST generate assembly for Windows x86-64 using Microsoft x64 calling convention
- **FR-017**: System MUST use RDI, RSI, RDX, RCX, R8, R9 for integer parameters on System V
- **FR-017a**: System MUST use XMM0-XMM7 for floating-point parameters on System V (tracked separately from integer registers)
- **FR-018**: System MUST use RCX, RDX, R8, R9 for integer parameters on Windows x64
- **FR-018a**: System MUST use XMM0-XMM3 for floating-point parameters on Windows x64, where each XMM register corresponds to the same parameter position as the integer register (slot-based)
- **FR-019**: System MUST allocate 32-byte shadow space for Windows x64 function calls
- **FR-020**: System MUST respect 128-byte red zone on System V platforms (not use stack below RSP without allocation)
- **FR-021**: System MUST save and restore callee-saved registers (RBX, RBP, R12-R15 on all platforms; RDI, RSI additionally on Windows)

**Register and Memory Management**

- **FR-022**: System MUST allocate variables to general-purpose registers RAX, RBX, RCX, RDX, RSI, RDI, R8-R15
- **FR-023**: System MUST allocate floating-point values to SIMD registers XMM0-XMM15
- **FR-024**: System MUST spill values to stack when register pressure exceeds available registers, using Linear Scan allocation with liveness analysis
- **FR-025**: System MUST create stack frames with 16-byte alignment as required by x86-64 ABI
- **FR-026**: System MUST calculate array element addresses as base + (index x element_size)
- **FR-027**: System MUST calculate struct field addresses as base + field_offset

**Function Calls**

- **FR-028**: System MUST place function parameters in platform-designated registers
- **FR-029**: System MUST place excess parameters on stack in right-to-left order
- **FR-030**: System MUST retrieve integer/pointer return values from RAX
- **FR-031**: System MUST retrieve floating-point return values from XMM0
- **FR-031a**: System MUST handle large return values (struct/array >16 bytes) via hidden pointer: caller allocates space and passes pointer as implicit first argument, callee writes result there
- **FR-032**: System MUST generate function prologue saving RBP and callee-saved registers
- **FR-033**: System MUST generate function epilogue restoring callee-saved registers and RBP
- **FR-034**: System MUST support variadic functions with correct argument passing

**Output Generation**

- **FR-035**: System MUST produce .text section containing executable code
- **FR-036**: System MUST produce .data section for initialized global data
- **FR-037**: System MUST produce .bss section for uninitialized global data
- **FR-038**: System MUST produce .rodata section for read-only constants and string literals
- **FR-039**: System MUST use Intel assembly syntax
- **FR-040**: System MUST emit assembler directives for section declarations (section .text, etc.)
- **FR-041**: System MUST emit `global` directives for exported symbols and `extern` directives for each called function not defined in the module
- **FR-042**: System MUST generate output compatible with NASM assembler

**Control Flow**

- **FR-043**: System MUST create unique labels for each basic block
- **FR-044**: System MUST generate jmp instructions for unconditional branches
- **FR-045**: System MUST generate conditional jump instructions (je, jne, jl, jle, jg, jge, etc.) for conditional branches
- **FR-046**: System MUST support switch statement implementation using jump tables for ≥4 contiguous cases, cascaded comparisons otherwise
- **FR-047**: System MUST eliminate unnecessary jumps when execution falls through to next block

**Debug Information**

- **FR-048**: System MUST include comments with original variable names where available
- **FR-049**: System MUST include comments marking basic block boundaries
- **FR-050**: System SHOULD preserve source location information in comments when available in IR

**Data Handling**

- **FR-051**: System MUST emit string literals with null termination
- **FR-052**: System MUST emit appropriate data directives (db, dw, dd, dq) matching data sizes
- **FR-053**: System MUST respect alignment requirements for data types

**Integration and Error Handling**

- **FR-054**: System MUST accept IR that has been validated by previous compilation phases
- **FR-055**: System MUST report clear error messages when IR constructs cannot be translated
- **FR-056**: System MUST provide statistics including instruction count, register usage, and spill count

### Key Entities

- **IR Module**: Top-level container holding functions, global variables, and type definitions to be translated
- **IR Function**: Contains control flow graph of basic blocks representing a function's implementation
- **IR Basic Block**: Sequence of instructions ending in a terminator, represents a single-entry single-exit code region
- **IR Instruction**: Single operation in the IR (arithmetic, memory, control flow, etc.)
- **Assembly File**: Output artifact containing translated assembly code organized into sections
- **Stack Frame**: Per-function memory layout on stack for local variables and spilled registers
- **Register Mapping**: Association between IR values and physical x86-64 registers

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Generated assembly can be assembled by NASM without errors for all valid IR inputs
- **SC-002**: Programs compiled through the full pipeline produce correct output matching interpreter results
- **SC-003**: Code generation completes for modules with up to 1000 functions within 30 seconds
- **SC-004**: Generated code for each platform passes platform-specific ABI compliance tests
- **SC-005**: Stack frames maintain 16-byte alignment as verified by runtime checks
- **SC-006**: Function calls with up to 20 parameters execute correctly
- **SC-007**: All IR data types (integers, floats, booleans, arrays, structs, pointers) are correctly represented in assembly
- **SC-008**: Switch statements with up to 256 cases generate working code
- **SC-009**: Debug comments appear for at least 80% of emitted assembly instructions (measured as: instructions with associated comment / total instructions)
- **SC-010**: Code generation statistics are accurate within 1% of actual instruction counts

## Clarifications

### Session 2025-12-16

- Q: Which register allocation strategy to use? → A: Linear Scan allocation (liveness analysis + single-pass allocation)
- Q: When to use jump table vs cascaded comparisons for switch? → A: Jump table for ≥4 contiguous cases, otherwise cascaded comparisons
- Q: How to handle calls to external functions (not defined in module)? → A: Emit `extern symbol_name` for each called function not defined in the module
- Q: How to pass floating-point parameters on each platform? → A: System V: floats in XMM0-XMM7 (tracked separately from integers); Windows: floats in same-slot XMM register corresponding to parameter position
- Q: How to handle large return values (struct/array >16 bytes)? → A: Caller allocates space and passes hidden pointer as first argument; callee writes result there

## Assumptions

- IR input has been validated and type-checked by previous compiler phases
- IR is in valid SSA form with proper dominance properties
- Target platform is specified before code generation begins
- NASM assembler is available for final assembly step
- Standard C runtime library is available for linking external calls
- IR uses 64-bit pointers (x86-64 only, no 32-bit support required)
- Floating-point operations use IEEE 754 semantics
