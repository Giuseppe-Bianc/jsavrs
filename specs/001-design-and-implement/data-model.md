# Data Model: x86-64 Assembly Code Generator

## Core Entities

### AssemblyGenerator
**Description**: Main component responsible for translating IR to x86-64 assembly
- **Fields**:
  - `ir_module`: Reference to the IR module being translated
  - `target_platform`: Enum specifying target platform (Windows x86-64, Linux x86-64, macOS x86-64)
  - `register_allocator`: Component managing register allocation and stack spilling
  - `calling_convention`: ABI-specific calling convention implementation
  - `assembly_buffer`: Temporary storage for generated assembly text
- **Relationships**: 
  - Composes RegisterAllocator
  - Composes CallingConvention
  - Reads from IRModule
- **State transitions**: Initialized → Generating → Finalized

### RegisterAllocator
**Description**: Manages allocation of x86-64 registers with stack overflow capability
- **Fields**:
  - `available_registers`: List of currently available registers
  - `allocated_registers`: Map of IR values to physical registers
  - `stack_offset`: Current offset for stack-allocated values
  - `register_map`: Mapping of register names to x86-64 register IDs
- **Relationships**: Used by AssemblyGenerator
- **Validation rules**: 
  - Must not exceed physical register count
  - Stack offsets must be properly aligned

### CallingConvention
**Description**: Handles platform-specific calling conventions and ABI compliance
- **Fields**:
  - `platform`: Target platform enum
  - `caller_saved_registers`: List of registers preserved by caller
  - `callee_saved_registers`: List of registers preserved by callee
  - `parameter_registers`: Order of registers for parameter passing
  - `stack_alignment`: Required stack alignment value
- **Relationships**: Used by AssemblyGenerator
- **Validation rules**: 
  - Must follow platform-specific ABI specifications

## IR-to-Assembly Mapping

### IRInstruction → x86-64 Instruction Mapping
- **Arithmetic Operations**:
  - `Add(lhs, rhs)` → `add rax, rbx` (or appropriate registers)
  - `Sub(lhs, rhs)` → `sub rax, rbx`
  - `Mul(lhs, rhs)` → `imul rax, rbx`
- **Memory Operations**:
  - `Load(address)` → `mov rax, [rbx]`
  - `Store(value, address)` → `mov [rbx], rax`
- **Control Flow**:
  - `Jump(target)` → `jmp label`
  - `ConditionalJump(condition, target1, target2)` → `cmp rax, rbx; je label1; jmp label2`
- **Function Operations**:
  - `Call(function, args)` → Proper calling convention setup and `call` instruction
  - `Return(value)` → `mov rax, value; ret`

## Function Prologue/Epilogue Generation

### FunctionPrologue
**Fields**:
- `function_signature`: IR representation of function signature
- `local_variables_size`: Total size of local variables requiring stack space
- `saved_registers`: List of registers that need to be preserved
- `stack_frame_size`: Size of complete stack frame

### FunctionEpilogue
**Fields**:
- `function_signature`: IR representation of function signature
- `saved_registers`: List of registers that need to be restored
- `return_value`: Register or memory location of return value

## Assembly Output Structure

### AssemblyModule
**Fields**:
- `section_text`: Executable code section
- `section_data`: Initialized data section
- `section_bss`: Uninitialized data section
- `global_symbols`: List of global symbols for linking
- `exported_functions`: List of functions to be exported

### AssemblyFunction
**Fields**:
- `name`: Function name
- `parameters`: List of function parameters
- `locals`: List of local variables
- `instructions`: List of assembled x86-64 instructions
- `labels`: Map of IR labels to assembly labels
- `prologue`: Generated function prologue
- `epilogue`: Generated function epilogue

## Platform-Specific Data

### PlatformABI
**Fields**:
- `platform`: Target platform enum
- `register_convention`: How registers are used
- `stack_convention`: How stack is managed
- `parameter_convention`: How parameters are passed
- `return_convention`: How return values are handled
- `alignment_requirements`: Memory alignment requirements

### PlatformABI Implementations
- **Windows x64 ABI**:
  - Uses RCX, RDX, R8, R9 for first 4 integer parameters
  - Uses XMM0-XMM3 for first 4 floating-point parameters
  - Maintains 16-byte stack alignment
  - Caller saves RAX, RCX, RDX, R8-R11, XMM0-XMM5

- **System V ABI (Linux/macOS)**:
  - Uses RDI, RSI, RDX, RCX, R8, R9 for first 6 integer parameters
  - Uses XMM0-XMM7 for first 8 floating-point parameters
  - Maintains 16-byte stack alignment
  - Caller saves RAX, RCX, RDX, RSI, RDI, R8-R11, R15

## Error Handling

### AssemblyGenerationError
**Fields**:
- `error_type`: Type of error (UnsupportedIR, RegisterOverflow, etc.)
- `ir_location`: Location in IR where error occurred
- `description`: Human-readable error description
- `suggestion`: Suggested resolution if applicable