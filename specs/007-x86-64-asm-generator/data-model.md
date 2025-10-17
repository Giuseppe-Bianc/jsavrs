# Data Model: x86-64 NASM Assembly Code Generator

**Feature**: 007-x86-64-asm-generator  
**Date**: 2025-10-17  
**Status**: Phase 1 Design Complete

## Overview

This document provides a comprehensive, detailed specification of all internal data structures, state machines, and IR-to-assembly mappings used by the x86-64 NASM assembly code generator. Every field, relationship, invariant, and state transition is documented with precision to ensure complete understanding of the generator's internal workings.

## Core Generator Data Structures

### 1. AsmGenerator

**Purpose**: Main orchestrator for code generation, coordinating register allocation, instruction selection, and assembly output.

**Struct Definition**:

```rust
pub struct AsmGenerator {
    /// Target architecture triple (determines ABI and conventions)
    target_triple: TargetTriple,
    
    /// Selected ABI based on target (System V or Microsoft x64)
    abi: Abi,
    
    /// Register allocator instance (linear scan)
    register_allocator: LinearScanAllocator,
    
    /// Instruction selector for IR → assembly translation
    instruction_selector: InstructionSelector,
    
    /// Phi function resolver for SSA elimination
    phi_resolver: PhiResolver,
    
    /// Prologue/epilogue generator for function frames
    frame_generator: FrameGenerator,
    
    /// Accumulated errors during generation
    errors: Vec<CodeGenError>,
    
    /// Generated assembly sections
    text_section: AssemblySection,
    data_section: AssemblySection,
    rodata_section: AssemblySection,
    bss_section: AssemblySection,
    
    /// Label counter for unique label generation
    label_counter: u64,
    
    /// Current function being generated (context)
    current_function: Option<FunctionContext>,
}
```

**Invariants**:
- `abi` must match `target_triple` (System V for Linux/macOS, Microsoft x64 for Windows)
- `current_function` is `Some` during function generation, `None` between functions
- `label_counter` is monotonically increasing (ensures unique labels)
- All sections start empty and are populated during generation

**Lifecycle States**:
1. **Initialized**: Created with target triple, ABI selected, all sections empty
2. **Generating**: Processing IR module, `current_function` context active
3. **Complete**: All functions processed, sections finalized, ready to emit assembly string

**Methods**:

```rust
impl AsmGenerator {
    /// Create new generator for specific target
    pub fn new(target: TargetTriple) -> Self;
    
    /// Generate assembly from IR module (main entry point)
    pub fn generate(&mut self, module: &Module) -> CodeGenResult;
    
    /// Generate assembly for single function
    fn generate_function(&mut self, func: &Function) -> Result<(), CodeGenError>;
    
    /// Generate assembly for single basic block
    fn generate_block(&mut self, block: &BasicBlock) -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Generate assembly for single IR instruction
    fn generate_instruction(&mut self, inst: &Instruction) -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Generate unique label
    fn new_label(&mut self, prefix: &str) -> String;
    
    /// Emit final assembly string
    fn emit_assembly(&self) -> String;
}
```

### 2. FunctionContext

**Purpose**: Maintains per-function state during code generation, including stack frame layout, used registers, and local variable mappings.

**Struct Definition**:

```rust
pub struct FunctionContext {
    /// Function being generated
    function: Function,
    
    /// Register assignment for all IR values
    register_assignment: RegisterAssignment,
    
    /// Stack frame layout
    stack_frame: StackFrame,
    
    /// Set of callee-saved registers used (for prologue/epilogue)
    used_callee_saved: HashSet<Register>,
    
    /// Label mapping: IR block label → assembly label
    block_labels: HashMap<String, String>,
    
    /// Value-to-location mapping (registers or stack slots)
    value_locations: HashMap<ValueId, ValueLocation>,
    
    /// Whether function makes any calls (determines shadow space need)
    makes_calls: bool,
}
```

**ValueLocation Enum**:

```rust
pub enum ValueLocation {
    /// Value is in a physical register
    Register(Register),
    
    /// Value is spilled to stack
    Stack { offset: i32, size: OperandSize },
    
    /// Value is an immediate constant
    Immediate(Immediate),
    
    /// Value is in memory (for loads/stores)
    Memory(MemoryOperand),
}
```

**Relationships**:
- Each `Function` has exactly one `FunctionContext` during generation
- `register_assignment` maps IR `Value` IDs to physical registers
- `stack_frame` layout includes local variables, spill slots, and shadow space
- `block_labels` ensures consistent label naming across jumps and block definitions

### 3. StackFrame

**Purpose**: Describes the complete layout of a function's stack frame, including all allocated regions.

**Struct Definition**:

```rust
pub struct StackFrame {
    /// Size of local variables region (from IR alloca instructions)
    local_vars_size: u32,
    
    /// Offsets of each local variable from RBP
    local_var_offsets: HashMap<ValueId, i32>,
    
    /// Size of spill slots region (for register spilling)
    spill_slots_size: u32,
    
    /// Offsets of each spill slot from RBP
    spill_slot_offsets: HashMap<ValueId, i32>,
    
    /// Shadow space size (32 bytes for Windows, 0 for System V)
    shadow_space_size: u32,
    
    /// Alignment padding to ensure 16-byte alignment
    alignment_padding: u32,
    
    /// Total stack frame size (sum of all above + alignment)
    total_size: u32,
    
    /// List of callee-saved registers pushed in prologue (affects stack offsets)
    saved_registers: Vec<Register>,
}
```

**Stack Frame Layout** (Windows x64 example):

```
High addresses
+-----------------------+
|  Return address       |  [RBP + 8]
+-----------------------+
|  Saved RBP            |  [RBP]  <- RBP points here after prologue
+-----------------------+
|  Saved R15            |  [RBP - 8]
|  Saved R14            |  [RBP - 16]
|  Saved R13            |  [RBP - 24]
|  ...                  |  (Callee-saved registers)
+-----------------------+
|  Local variable 1     |  [RBP - offset1]
|  Local variable 2     |  [RBP - offset2]
|  ...                  |
+-----------------------+
|  Spill slot 1         |  [RBP - offset3]
|  Spill slot 2         |  [RBP - offset4]
|  ...                  |
+-----------------------+
|  Shadow space (32B)   |  [RSP + 0] to [RSP + 31] (Windows only)
+-----------------------+
|  Alignment padding    |  (0-15 bytes to align RSP to 16)
+-----------------------+ <- RSP points here after prologue
Low addresses
```

**Invariants**:
- `total_size` = `local_vars_size` + `spill_slots_size` + `shadow_space_size` + `alignment_padding`
- All local variable offsets are negative (relative to RBP)
- `local_var_offsets` keys match `alloca` instruction result values
- `spill_slot_offsets` keys match spilled value IDs
- `shadow_space_size` is 32 if ABI is Windows and function makes calls, 0 otherwise
- `alignment_padding` ensures `(total_size + 8 * saved_registers.len()) % 16 == 0` (16-byte alignment)

**Calculation Algorithm**:

```rust
impl StackFrame {
    pub fn calculate(
        func: &Function,
        register_assignment: &RegisterAssignment,
        abi: &Abi,
        makes_calls: bool,
    ) -> Self {
        let mut frame = StackFrame::default();
        
        // 1. Calculate local variables size
        for inst in func.all_instructions() {
            if let InstructionKind::Alloca { result_type, .. } = &inst.kind {
                let size = type_size(result_type);
                let alignment = type_alignment(result_type);
                
                // Align offset
                let offset = align_down(frame.local_vars_size, alignment);
                frame.local_var_offsets.insert(inst.result.id, -(offset as i32));
                frame.local_vars_size = offset + size;
            }
        }
        
        // 2. Calculate spill slots size
        for (value_id, location) in register_assignment.locations() {
            if let ValueLocation::Stack { size, .. } = location {
                let slot_size = size.bytes();
                let offset = frame.local_vars_size + frame.spill_slots_size;
                frame.spill_slot_offsets.insert(value_id, -(offset as i32));
                frame.spill_slots_size += slot_size;
            }
        }
        
        // 3. Add shadow space (Windows only)
        if abi.is_windows() && makes_calls {
            frame.shadow_space_size = 32;
        }
        
        // 4. Calculate alignment padding
        let callee_saved_size = register_assignment.used_callee_saved().len() * 8;
        let unaligned_total = frame.local_vars_size + frame.spill_slots_size + frame.shadow_space_size;
        let stack_before_alignment = unaligned_total + callee_saved_size as u32;
        
        // We need (stack_before_alignment + 8) % 16 == 0 (accounting for return address)
        frame.alignment_padding = (16 - ((stack_before_alignment + 8) % 16)) % 16;
        
        // 5. Calculate total
        frame.total_size = unaligned_total + frame.alignment_padding;
        
        frame
    }
}
```

## Register Allocation Data Structures

### 4. LinearScanAllocator

**Purpose**: Implements linear scan register allocation algorithm with furthest-use spilling.

**Struct Definition**:

```rust
pub struct LinearScanAllocator {
    /// Live intervals for all IR values, sorted by start point
    intervals: Vec<LiveInterval>,
    
    /// Currently allocated intervals (active set)
    active: Vec<(LiveInterval, Register)>,
    
    /// Available physical registers (not yet allocated)
    available_gp_regs: Vec<GPRegister>,
    available_xmm_regs: Vec<XMMRegister>,
    
    /// Spill decisions (value → stack slot)
    spilled_values: HashMap<ValueId, SpillSlot>,
    
    /// Next available spill slot offset
    next_spill_offset: i32,
    
    /// Final register assignment result
    assignment: RegisterAssignment,
}
```

**Invariants**:
- `intervals` is sorted by `start` field (ascending)
- `active` contains only intervals that overlap the current position
- Each interval in `active` is assigned to a unique register (no conflicts)
- `available_*_regs` contains only registers not in `active`
- `spilled_values` keys are disjoint from `assignment.register_map` keys

**Algorithm State Machine**:

```
[Initial State]
    ↓
    • intervals = compute_liveness()
    • Sort intervals by start
    • available_regs = all allocatable registers
    • active = []
    ↓
[Processing Intervals]
    ↓
    For each interval i:
        ↓
        1. Expire old intervals (end < i.start)
           → Move registers back to available_regs
        ↓
        2. Check if register available
           ↓
           YES → Allocate register to i
           ↓           ↓
           NO          Add (i, reg) to active
           ↓
           3. Spill decision
              ↓
              • Find furthest-use interval
              • If furthest > i.end: spill furthest, allocate to i
              • Else: spill i, continue
    ↓
[Complete]
    • Build RegisterAssignment from active + spilled_values
    • Return assignment
```

### 5. LiveInterval

**Purpose**: Represents the live range of a single IR value (from definition to last use).

**Struct Definition**:

```rust
pub struct LiveInterval {
    /// IR value this interval represents
    value: Value,
    
    /// Start position (instruction index where value is defined)
    start: u32,
    
    /// End position (instruction index of last use)
    end: u32,
    
    /// Register class required (GP or XMM)
    reg_class: RegisterClass,
    
    /// List of use positions (for furthest-use heuristic)
    use_positions: Vec<u32>,
}
```

**Invariants**:
- `start` ≤ `end`
- `start` is in `use_positions` (definition is a use)
- `use_positions` is sorted ascending
- `use_positions.last()` == `end` (last use defines interval end)
- `reg_class` matches `value.ty` (float types → XMM, others → GP)

**Methods**:

```rust
impl LiveInterval {
    /// Check if this interval overlaps another
    pub fn overlaps(&self, other: &LiveInterval) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
    
    /// Find next use after given position (for furthest-use heuristic)
    pub fn next_use_after(&self, pos: u32) -> Option<u32> {
        self.use_positions.iter()
            .find(|&&use_pos| use_pos > pos)
            .copied()
    }
    
    /// Get furthest use position (for spilling priority)
    pub fn furthest_use(&self) -> u32 {
        self.use_positions.last().copied().unwrap_or(self.end)
    }
}
```

**Liveness Computation**:

```rust
pub fn compute_liveness(func: &Function) -> Vec<LiveInterval> {
    let mut intervals = Vec::new();
    let cfg = &func.cfg;
    
    // 1. Compute live-in/live-out for each block (backward dataflow)
    let liveness_info = compute_block_liveness(cfg);
    
    // 2. For each IR value, find definition and all uses
    let mut value_info: HashMap<ValueId, (u32, Vec<u32>)> = HashMap::new();
    
    for (block_id, block) in cfg.blocks().enumerate() {
        let base_pos = block_id * 1000;  // Give each block 1000 instruction slots
        
        for (inst_offset, inst) in block.instructions.iter().enumerate() {
            let pos = base_pos + inst_offset as u32;
            
            // Record definition
            if let Some(result) = &inst.result {
                value_info.entry(result.id)
                    .or_insert((pos, vec![]))
                    .1.push(pos);  // Definition is also a use
            }
            
            // Record uses
            for operand in inst.operands() {
                if let Some(value_id) = operand.value_id() {
                    value_info.entry(value_id)
                        .or_insert((pos, vec![]))
                        .1.push(pos);
                }
            }
        }
        
        // Handle terminator uses
        if let Some(term) = &block.terminator {
            let pos = base_pos + block.instructions.len() as u32;
            for operand in term.operands() {
                if let Some(value_id) = operand.value_id() {
                    value_info.entry(value_id)
                        .or_insert((pos, vec![]))
                        .1.push(pos);
                }
            }
        }
    }
    
    // 3. Create LiveInterval for each value
    for (value_id, (def_pos, mut use_positions)) in value_info {
        use_positions.sort_unstable();
        
        let start = def_pos;
        let end = *use_positions.last().unwrap();
        let value = func.get_value(value_id).unwrap();
        
        let reg_class = match &value.ty {
            IrType::F32 | IrType::F64 => RegisterClass::FloatingPoint,
            _ => RegisterClass::GeneralPurpose,
        };
        
        intervals.push(LiveInterval {
            value: value.clone(),
            start,
            end,
            reg_class,
            use_positions,
        });
    }
    
    // 4. Sort by start position
    intervals.sort_by_key(|i| i.start);
    
    intervals
}
```

### 6. RegisterAssignment

**Purpose**: Maps IR values to their allocated physical registers or spill locations.

**Struct Definition**:

```rust
pub struct RegisterAssignment {
    /// Value → Register mapping for allocated values
    register_map: HashMap<ValueId, Register>,
    
    /// Value → Stack slot mapping for spilled values
    spill_map: HashMap<ValueId, SpillSlot>,
    
    /// Set of all registers used (for prologue/epilogue generation)
    used_registers: HashSet<Register>,
    
    /// Set of callee-saved registers used (must be saved/restored)
    used_callee_saved: HashSet<Register>,
}
```

**Invariants**:
- `register_map` and `spill_map` keys are disjoint (value is either in register OR spilled)
- `used_registers` = `register_map.values().collect()`
- `used_callee_saved` ⊆ `used_registers`
- All registers in `register_map` are allocatable (not reserved like RSP, RBP)

**Methods**:

```rust
impl RegisterAssignment {
    /// Get location of a value (register or spill)
    pub fn get_location(&self, value_id: &ValueId) -> Option<ValueLocation> {
        if let Some(reg) = self.register_map.get(value_id) {
            Some(ValueLocation::Register(reg.clone()))
        } else if let Some(slot) = self.spill_map.get(value_id) {
            Some(ValueLocation::Stack {
                offset: slot.offset,
                size: slot.size,
            })
        } else {
            None
        }
    }
    
    /// Check if value is spilled
    pub fn is_spilled(&self, value_id: &ValueId) -> bool {
        self.spill_map.contains_key(value_id)
    }
    
    /// Get all used callee-saved registers (for prologue/epilogue)
    pub fn used_callee_saved(&self) -> &HashSet<Register> {
        &self.used_callee_saved
    }
}
```

### 7. SpillSlot

**Purpose**: Represents a stack location for a spilled value.

**Struct Definition**:

```rust
pub struct SpillSlot {
    /// Offset from RBP (negative)
    offset: i32,
    
    /// Size of the slot
    size: OperandSize,
    
    /// IR type of the spilled value (for reloading)
    value_type: IrType,
}
```

**Example Stack Layout with Spills**:

```
RBP
  ↓
  [Local var 1]    RBP - 8
  [Local var 2]    RBP - 16
  [Spill slot 1]   RBP - 24  (for %t5: i64)
  [Spill slot 2]   RBP - 32  (for %t12: f64)
  [Spill slot 3]   RBP - 40  (for %t20: i32, only uses 4 bytes)
  ...
```

## Instruction Selection Data Structures

### 8. InstructionSelector

**Purpose**: Translates IR instructions to x86-64 assembly instructions with type-driven size selection.

**Struct Definition**:

```rust
pub struct InstructionSelector {
    /// ABI for calling convention decisions
    abi: Abi,
    
    /// Register assignment for value lookup
    register_assignment: RegisterAssignment,
    
    /// Type-to-size mapping cache
    type_size_cache: HashMap<IrType, OperandSize>,
}
```

**Methods**:

```rust
impl InstructionSelector {
    /// Select instructions for an IR instruction
    pub fn select(&self, ir_inst: &Instruction) -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Translate binary operation
    fn select_binary(&self, op: IrBinaryOp, left: &Value, right: &Value, ty: &IrType) 
        -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Translate unary operation
    fn select_unary(&self, op: IrUnaryOp, operand: &Value, ty: &IrType)
        -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Translate memory load
    fn select_load(&self, address: &Value, ty: &IrType)
        -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Translate memory store
    fn select_store(&self, address: &Value, value: &Value)
        -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Translate cast operation
    fn select_cast(&self, from: &Value, to_type: &IrType, kind: CastKind)
        -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Translate function call
    fn select_call(&self, func: &Value, args: &[Value], ret_type: &IrType)
        -> Result<Vec<AsmInstruction>, CodeGenError>;
    
    /// Get operand size for IR type
    fn type_to_size(&self, ty: &IrType) -> Result<OperandSize, CodeGenError>;
}
```

### 9. IR-to-Assembly Instruction Mappings

**Complete mapping table for all supported IR instructions**:

#### Binary Operations (Integer)

| IR Operation | IR Types | x86-64 Instruction | Notes |
|--------------|----------|-------------------|--------|
| `add` | i8-i64, u8-u64 | `add dest, src` | Two-operand form: `dest = dest + src` |
| `subtract` | i8-i64, u8-u64 | `sub dest, src` | Two-operand form: `dest = dest - src` |
| `multiply` | i8-i64 (signed) | `imul dest, src` | Signed multiply, result in dest |
| `multiply` | u8-u64 (unsigned) | `mul src` | Unsigned multiply, result in RDX:RAX |
| `divide` | i8-i64 (signed) | `idiv src` | Signed divide, quotient in RAX, remainder in RDX. Requires `cqo` before to sign-extend RAX into RDX:RAX |
| `divide` | u8-u64 (unsigned) | `div src` | Unsigned divide, quotient in RAX, remainder in RDX. Requires `xor rdx, rdx` before to clear RDX |
| `modulo` | i8-i64 (signed) | `idiv src` | Remainder in RDX after `idiv` |
| `modulo` | u8-u64 (unsigned) | `div src` | Remainder in RDX after `div` |
| `bitwise_and` | All integer types | `and dest, src` | Bitwise AND |
| `bitwise_or` | All integer types | `or dest, src` | Bitwise OR |
| `bitwise_xor` | All integer types | `xor dest, src` | Bitwise XOR |
| `shift_left` | All integer types | `shl dest, count` | Logical left shift |
| `shift_right` | u8-u64 (unsigned) | `shr dest, count` | Logical right shift (zero fill) |
| `shift_right` | i8-i64 (signed) | `sar dest, count` | Arithmetic right shift (sign fill) |

#### Binary Operations (Floating-Point)

| IR Operation | IR Types | x86-64 Instruction | Notes |
|--------------|----------|-------------------|--------|
| `add` | f32 | `addss dest, src` | SSE scalar single-precision add |
| `add` | f64 | `addsd dest, src` | SSE scalar double-precision add |
| `subtract` | f32 | `subss dest, src` | SSE scalar single-precision subtract |
| `subtract` | f64 | `subsd dest, src` | SSE scalar double-precision subtract |
| `multiply` | f32 | `mulss dest, src` | SSE scalar single-precision multiply |
| `multiply` | f64 | `mulsd dest, src` | SSE scalar double-precision multiply |
| `divide` | f32 | `divss dest, src` | SSE scalar single-precision divide |
| `divide` | f64 | `divsd dest, src` | SSE scalar double-precision divide |

#### Comparison Operations

| IR Operation | IR Types | x86-64 Instructions | Notes |
|--------------|----------|---------------------|--------|
| `equal` | Integer | `cmp left, right` + `je label` | Sets ZF flag, jump if equal |
| `not_equal` | Integer | `cmp left, right` + `jne label` | Sets ZF flag, jump if not equal |
| `less` | Signed int | `cmp left, right` + `jl label` | Sets SF, OF flags, jump if less (signed) |
| `less` | Unsigned int | `cmp left, right` + `jb label` | Sets CF flag, jump if below (unsigned) |
| `less_equal` | Signed int | `cmp left, right` + `jle label` | Jump if less or equal (signed) |
| `less_equal` | Unsigned int | `cmp left, right` + `jbe label` | Jump if below or equal (unsigned) |
| `greater` | Signed int | `cmp left, right` + `jg label` | Jump if greater (signed) |
| `greater` | Unsigned int | `cmp left, right` + `ja label` | Jump if above (unsigned) |
| `greater_equal` | Signed int | `cmp left, right` + `jge label` | Jump if greater or equal (signed) |
| `greater_equal` | Unsigned int | `cmp left, right` + `jae label` | Jump if above or equal (unsigned) |
| `equal` | Float | `ucomiss` (f32) or `ucomisd` (f64) + `je label` | Unordered compare (handles NaN), sets flags |
| `not_equal` | Float | `ucomiss/ucomisd` + `jne label` | Unordered compare, jump if not equal |
| `less` | Float | `ucomiss/ucomisd` + `jb label` | Jump if less (below) |
| `less_equal` | Float | `ucomiss/ucomisd` + `jbe label` | Jump if less or equal |
| `greater` | Float | `ucomiss/ucomisd` + `ja label` | Jump if greater (above) |
| `greater_equal` | Float | `ucomiss/ucomisd` + `jae label` | Jump if greater or equal |

#### Unary Operations

| IR Operation | IR Types | x86-64 Instruction | Notes |
|--------------|----------|-------------------|--------|
| `negate` | Integer | `neg dest` | Two's complement negation |
| `not` | Integer | `not dest` | Bitwise NOT (one's complement) |
| `negate` | Float | `xorps dest, [sign_bit_mask]` | XOR with sign bit to flip (requires constant pool) |

#### Memory Operations

| IR Operation | x86-64 Instructions | Notes |
|--------------|---------------------|--------|
| `alloca` | `sub rsp, size` (in prologue) | Allocates stack space; actual offset calculated in stack frame |
| `load` | `mov dest, [address]` | Load from memory address |
| `store` | `mov [address], src` | Store to memory address |
| `get_element_ptr` (constant offset) | `lea dest, [base + offset]` | Load effective address with constant displacement |
| `get_element_ptr` (indexed) | `lea dest, [base + index*scale + offset]` | Scaled index addressing (scale = 1, 2, 4, or 8) |

#### Type Conversions (Casts)

| Cast Kind | From Type | To Type | x86-64 Instruction | Notes |
|-----------|-----------|---------|-------------------|--------|
| `IntSignExtend` | i8 | i16/i32/i64 | `movsx dest, src` | Sign-extend (e.g., `movsx eax, bl`) |
| `IntSignExtend` | i16 | i32/i64 | `movsx dest, src` | Sign-extend |
| `IntSignExtend` | i32 | i64 | `movsxd dest, src` | Sign-extend 32→64 (special instruction) |
| `IntZeroExtend` | u8 | u16/u32/u64 | `movzx dest, src` | Zero-extend (e.g., `movzx eax, bl`) |
| `IntZeroExtend` | u16 | u32/u64 | `movzx dest, src` | Zero-extend |
| `IntZeroExtend` | u32 | u64 | `mov dest_32bit, src` | Automatic zero-extension in x86-64 (moving to 32-bit register zeros upper 32 bits) |
| `IntTruncate` | i64/i32 | i16/i8 | `mov dest_small, src_small` | Use smaller register alias (e.g., `al` for low byte of `rax`) |
| `FloatToInt` | f32 | i32/i64 | `cvttss2si dest, src` | Convert float to int with truncation toward zero |
| `FloatToInt` | f64 | i32/i64 | `cvttsd2si dest, src` | Convert double to int with truncation toward zero |
| `IntToFloat` | i32/i64 | f32 | `cvtsi2ss dest, src` | Convert int to single-precision float |
| `IntToFloat` | i32/i64 | f64 | `cvtsi2sd dest, src` | Convert int to double-precision float |
| `FloatExtend` | f32 | f64 | `cvtss2sd dest, src` | Extend single to double precision |
| `FloatTruncate` | f64 | f32 | `cvtsd2ss dest, src` | Truncate double to single precision |
| `BoolToInt` | bool | i8-i64 | `movzx dest, src` | Zero-extend (bool is 1 byte) |
| `IntToBool` | i8-i64 | bool | `cmp src, 0` + `setne dest` | Set byte to 1 if non-zero, 0 otherwise |

#### Control Flow (Terminators)

| IR Terminator | x86-64 Instructions | Notes |
|---------------|---------------------|--------|
| `return void` | Epilogue + `ret` | Tear down stack frame, restore callee-saved, return |
| `return value` | `mov rax, value` (or `movss xmm0, value` for float) + Epilogue + `ret` | Move return value to appropriate register, then return |
| `branch label` | `jmp label` | Unconditional jump |
| `conditional_branch cond, true_label, false_label` | (Previous `cmp` sets flags) + `je/jne/jl/etc. true_label` + `jmp false_label` | Conditional jump based on flags, fall through to unconditional jump |
| `switch value, default, cases` | Series of `cmp value, case_val` + `je case_label` + final `jmp default` | Chain of comparisons (jump table optimization future work) |
| `unreachable` | `ud2` | Undefined instruction (triggers exception) |

## Phi Resolution Data Structures

### 10. PhiResolver

**Purpose**: Eliminates SSA phi functions by splitting critical edges and inserting move instructions.

**Struct Definition**:

```rust
pub struct PhiResolver {
    /// CFG being modified (owned during resolution)
    cfg: ControlFlowGraph,
    
    /// Counter for generating unique split block labels
    split_block_counter: u64,
    
    /// Map of (from_block, to_block) → split_block for critical edges
    split_edges: HashMap<(BlockId, BlockId), BlockId>,
    
    /// Collected move instructions: (block_id, phi_target, phi_source)
    moves: Vec<(BlockId, Value, Value)>,
}
```

**Critical Edge**:
An edge (A → B) is critical if:
- Block A has multiple successors (e.g., conditional branch), AND
- Block B has multiple predecessors (e.g., merge point)

**Example Critical Edge**:

```
     [A]
     / \
    /   \
  [C]   [D]    <- A has 2 successors
    \   /
     \ /
     [B]       <- B has 2 predecessors
     
The edges A→B via C and A→B via D are critical.
```

**Resolution Algorithm**:

```rust
impl PhiResolver {
    /// Resolve all phi functions in the CFG
    pub fn resolve(&mut self) -> Result<(), CodeGenError> {
        // 1. Identify all phi instructions
        let phi_instructions = self.collect_phi_instructions();
        
        // 2. For each phi, process all incoming edges
        for phi in phi_instructions {
            for (pred_label, value) in phi.incoming_values {
                let pred_block = self.cfg.find_block_by_label(&pred_label)?;
                let phi_block = self.cfg.find_block_by_label(&phi.block_label)?;
                
                // 3. Check if edge is critical
                if self.is_critical_edge(pred_block.id, phi_block.id) {
                    // Split the edge
                    let split_block = self.split_edge(pred_block.id, phi_block.id)?;
                    
                    // Insert move in the split block
                    self.moves.push((split_block, phi.target.clone(), value.clone()));
                } else {
                    // Insert move at end of predecessor (before terminator)
                    self.moves.push((pred_block.id, phi.target.clone(), value.clone()));
                }
            }
        }
        
        // 4. Apply all move instructions
        for (block_id, target, source) in &self.moves {
            self.insert_move_before_terminator(*block_id, target, source)?;
        }
        
        // 5. Remove all phi instructions
        self.remove_phi_instructions();
        
        Ok(())
    }
    
    fn is_critical_edge(&self, from: BlockId, to: BlockId) -> bool {
        let from_successors = self.cfg.successors(from).count();
        let to_predecessors = self.cfg.predecessors(to).count();
        from_successors > 1 && to_predecessors > 1
    }
    
    fn split_edge(&mut self, from: BlockId, to: BlockId) -> Result<BlockId, CodeGenError> {
        // Check cache
        if let Some(&split_id) = self.split_edges.get(&(from, to)) {
            return Ok(split_id);
        }
        
        // Create new block
        let split_label = format!(".Lsplit_{}", self.split_block_counter);
        self.split_block_counter += 1;
        
        let split_block = BasicBlock::new(split_label.clone());
        let split_id = self.cfg.add_block(split_block);
        
        // Update edges: from → split → to
        self.cfg.remove_edge(from, to);
        self.cfg.add_edge(from, split_id);
        self.cfg.add_edge(split_id, to);
        
        // Update terminator of 'from' block to jump to split instead of to
        let to_label = self.cfg.get_block(to).label.clone();
        self.update_terminator_target(from, &to_label, &split_label)?;
        
        // Add unconditional branch in split block to 'to'
        self.cfg.set_block_terminator(split_id, Terminator::branch(to_label));
        
        // Cache the split
        self.split_edges.insert((from, to), split_id);
        
        Ok(split_id)
    }
}
```

## ABI and Calling Convention Data

### 11. Calling Convention Parameter Passing

**System V AMD64 ABI** (Linux, macOS):

| Parameter Index | Integer/Pointer Register | Float Register | Stack Offset (if >6 int or >8 float) |
|----------------|-------------------------|---------------|--------------------------------------|
| 1 | RDI | XMM0 | [RSP + 0] |
| 2 | RSI | XMM1 | [RSP + 8] |
| 3 | RDX | XMM2 | [RSP + 16] |
| 4 | RCX | XMM3 | [RSP + 24] |
| 5 | R8 | XMM4 | [RSP + 32] |
| 6 | R9 | XMM5 | [RSP + 40] |
| 7+ | — | XMM6, XMM7, Stack | [RSP + 48], [RSP + 56], ... |

**Microsoft x64 ABI** (Windows):

| Parameter Index | Integer/Pointer Register | Float Register | Stack Offset (always reserved) |
|----------------|-------------------------|---------------|------------------------------|
| 1 | RCX | XMM0 | [RSP + 8] (shadow space) |
| 2 | RDX | XMM1 | [RSP + 16] (shadow space) |
| 3 | R8 | XMM2 | [RSP + 24] (shadow space) |
| 4 | R9 | XMM3 | [RSP + 32] (shadow space) |
| 5+ | — | — | [RSP + 40], [RSP + 48], ... |

**Key Difference**: Microsoft x64 always reserves 32 bytes of "shadow space" on the stack for the first 4 parameters, even if they're passed in registers. The callee can use this space to spill register parameters if needed.

### 12. Function Call Generation

**Algorithm**:

```rust
fn generate_call(&self, func: &Value, args: &[Value], ret_type: &IrType) 
    -> Result<Vec<AsmInstruction>, CodeGenError> {
    let mut instructions = Vec::new();
    
    let int_param_regs = self.abi.integer_parameter_registers();
    let float_param_regs = self.abi.float_parameter_registers();
    
    let mut int_param_idx = 0;
    let mut float_param_idx = 0;
    let mut stack_args = Vec::new();
    
    // 1. Classify arguments and assign to registers or stack
    for arg in args {
        match &arg.ty {
            IrType::F32 | IrType::F64 => {
                if float_param_idx < float_param_regs.len() {
                    // Pass in XMM register
                    let reg = float_param_regs[float_param_idx];
                    instructions.push(AsmInstruction::Movss {
                        dest: Operand::Register(Register::XMM(reg)),
                        src: self.value_to_operand(arg),
                        size: self.type_to_size(&arg.ty)?,
                    });
                    float_param_idx += 1;
                } else {
                    // Pass on stack
                    stack_args.push(arg);
                }
            }
            _ => {
                if int_param_idx < int_param_regs.len() {
                    // Pass in integer register
                    let reg = int_param_regs[int_param_idx];
                    instructions.push(AsmInstruction::Mov {
                        dest: Operand::Register(Register::GP(reg)),
                        src: self.value_to_operand(arg),
                        size: self.type_to_size(&arg.ty)?,
                    });
                    int_param_idx += 1;
                } else {
                    // Pass on stack
                    stack_args.push(arg);
                }
            }
        }
    }
    
    // 2. Push stack arguments (right-to-left for x86-64)
    for arg in stack_args.iter().rev() {
        instructions.push(AsmInstruction::Push(self.value_to_register(arg)?));
    }
    
    // 3. Ensure stack alignment (RSP % 16 == 0 before call)
    if self.needs_stack_alignment(&stack_args) {
        instructions.push(AsmInstruction::Sub {
            dest: Operand::Register(Register::RSP),
            src: Operand::Immediate(Immediate::I32(8)),
            size: OperandSize::Qword,
        });
    }
    
    // 4. Generate the call instruction
    let func_operand = self.value_to_operand(func);
    instructions.push(AsmInstruction::Call(func_operand));
    
    // 5. Clean up stack (pop arguments + alignment padding)
    let stack_bytes = stack_args.len() * 8 + if self.needs_stack_alignment(&stack_args) { 8 } else { 0 };
    if stack_bytes > 0 {
        instructions.push(AsmInstruction::Add {
            dest: Operand::Register(Register::RSP),
            src: Operand::Immediate(Immediate::I32(stack_bytes as i32)),
            size: OperandSize::Qword,
        });
    }
    
    // 6. Move return value to destination
    if !matches!(ret_type, IrType::Void) {
        let ret_reg = self.abi.return_register(ret_type);
        // Return value is already in ret_reg (RAX or XMM0), caller will use it
    }
    
    Ok(instructions)
}
```

## Assembly Output Data Structures

### 13. AsmFunction

**Purpose**: Represents a fully generated assembly function ready for emission.

**Struct Definition**:

```rust
pub struct AsmFunction {
    /// Function name (symbol)
    name: String,
    
    /// Whether function is entry point (generates `global` directive)
    is_entry: bool,
    
    /// Function signature (for comment generation)
    params: Vec<(String, IrType)>,
    return_type: IrType,
    
    /// Generated prologue instructions
    prologue: Vec<AsmInstruction>,
    
    /// Generated body instructions (grouped by basic block)
    body: Vec<(String, Vec<AsmInstruction>)>,  // (label, instructions)
    
    /// Generated epilogue instructions
    epilogue: Vec<AsmInstruction>,
}
```

**Output Format**:

```nasm
; Function: add_numbers
; Signature: (i32, i32) -> i32
add_numbers:
    ; Prologue
    push rbp
    mov rbp, rsp
    sub rsp, 16
    
    ; Body
.Lentry:
    ; IR: %result = add i32 %a, %b
    mov eax, edi    ; Parameter 1 from RDI
    add eax, esi    ; Add parameter 2 from RSI
    
    ; Epilogue
    mov rsp, rbp
    pop rbp
    ret
```

## Type System Mappings

### 14. IR Type to x86-64 Type Mapping

| IR Type | Size (bytes) | x86-64 Register Class | Operand Size | Alignment |
|---------|-------------|----------------------|--------------|-----------|
| `i8` | 1 | GP (8-bit: al, bl, cl, ...) | Byte | 1 |
| `i16` | 2 | GP (16-bit: ax, bx, cx, ...) | Word | 2 |
| `i32` | 4 | GP (32-bit: eax, ebx, ecx, ...) | Dword | 4 |
| `i64` | 8 | GP (64-bit: rax, rbx, rcx, ...) | Qword | 8 |
| `u8` | 1 | GP (8-bit) | Byte | 1 |
| `u16` | 2 | GP (16-bit) | Word | 2 |
| `u32` | 4 | GP (32-bit) | Dword | 4 |
| `u64` | 8 | GP (64-bit) | Qword | 8 |
| `f32` | 4 | XMM (single-precision) | Dword | 4 |
| `f64` | 8 | XMM (double-precision) | Qword | 8 |
| `bool` | 1 | GP (8-bit) | Byte | 1 |
| `char` | 2 | GP (16-bit) | Word | 2 |
| `ptr<T>` | 8 | GP (64-bit) | Qword | 8 |
| `void` | 0 | — | — | — |

## Error Handling Data Structures

### 15. CodeGenError

**Complete error taxonomy with CompileError integration**:

```rust
use crate::error::compile_error::CompileError;

#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    /// IR contains unsupported type (e.g., i128, struct, first-class array)
    #[error("Unsupported type {ty} at {location}")]
    UnsupportedType {
        ty: IrType,
        location: SourceSpan,
    },
    
    /// Unknown instruction kind (shouldn't happen with valid IR)
    #[error("Unknown instruction kind {kind} at {location}")]
    UnknownInstruction {
        kind: String,
        location: SourceSpan,
    },
    
    /// Malformed IR instruction (missing operands, invalid types)
    #[error("Malformed instruction: {reason} at {location}")]
    MalformedInstruction {
        reason: String,
        location: SourceSpan,
    },
    
    /// Register allocation failed (ran out of registers and spill slots)
    #[error("Register allocation failed: {0}")]
    RegisterAllocationFailed(String),
    
    /// Invalid calling convention for target
    #[error("Invalid calling convention {convention} for target {target}")]
    InvalidCallingConvention {
        convention: String,
        target: String,
    },
    
    /// CFG verification failed (missing terminators, broken edges)
    #[error("CFG verification failed: {0}")]
    CfgVerificationFailed(String),
    
    /// Phi resolution failed (critical edge split error)
    #[error("Phi resolution failed: {0}")]
    PhiResolutionFailed(String),
    
    /// File I/O error (writing .asm file)
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<CodeGenError> for CompileError {
    /// Converts code generation errors to the unified CompileError type.
    ///
    /// This enables seamless integration with the compiler's error reporting system.
    /// Each CodeGenError variant is mapped to the appropriate CompileError variant
    /// with contextual information preserved.
    ///
    /// # Examples
    /// ```
    /// use jsavrs::asm::error::CodeGenError;
    /// use jsavrs::error::compile_error::CompileError;
    /// 
    /// let codegen_err = CodeGenError::UnsupportedType {
    ///     ty: IrType::I128,
    ///     location: span.clone(),
    /// };
    /// 
    /// let compile_err: CompileError = codegen_err.into();
    /// // Now can be handled by unified error reporting
    /// ```
    fn from(err: CodeGenError) -> Self {
        match err {
            CodeGenError::UnsupportedType { ty, location } => {
                CompileError::AsmGeneratorError {
                    message: format!("Unsupported type: {}", ty),
                    span: Some(location),
                    help: Some(
                        "Only I8-I64, U8-U64, F32, F64, Bool, Char, Pointer, and Void types are supported. \
                         Consider using a supported type or refactoring the code.".to_string()
                    ),
                }
            }
            
            CodeGenError::UnknownInstruction { kind, location } => {
                CompileError::AsmGeneratorError {
                    message: format!("Unknown IR instruction: {}", kind),
                    span: Some(location),
                    help: Some(
                        "This instruction is not recognized by the code generator. \
                         This may indicate a bug in the IR generator.".to_string()
                    ),
                }
            }
            
            CodeGenError::MalformedInstruction { reason, location } => {
                CompileError::AsmGeneratorError {
                    message: format!("Malformed IR instruction: {}", reason),
                    span: Some(location),
                    help: Some(
                        "The IR instruction is structurally invalid. \
                         This may indicate a bug in the IR generator or validator.".to_string()
                    ),
                }
            }
            
            CodeGenError::RegisterAllocationFailed(msg) => {
                CompileError::AsmGeneratorError {
                    message: format!("Register allocation failed: {}", msg),
                    span: None,
                    help: Some(
                        "The function may have too many live values simultaneously (>20). \
                         Consider simplifying the function or reducing temporary variables.".to_string()
                    ),
                }
            }
            
            CodeGenError::InvalidCallingConvention { convention, target } => {
                CompileError::AsmGeneratorError {
                    message: format!(
                        "Invalid calling convention '{}' for target '{}'", 
                        convention, target
                    ),
                    span: None,
                    help: Some(
                        "Verify that the target platform supports the specified calling convention. \
                         Use System V for Linux/macOS, or Microsoft x64 for Windows.".to_string()
                    ),
                }
            }
            
            CodeGenError::CfgVerificationFailed(msg) => {
                CompileError::AsmGeneratorError {
                    message: format!("Control flow graph verification failed: {}", msg),
                    span: None,
                    help: Some(
                        "The IR control flow graph is invalid. \
                         Ensure all basic blocks have terminators and all branch targets exist.".to_string()
                    ),
                }
            }
            
            CodeGenError::PhiResolutionFailed(msg) => {
                CompileError::AsmGeneratorError {
                    message: format!("SSA phi function resolution failed: {}", msg),
                    span: None,
                    help: Some(
                        "Failed to eliminate SSA phi functions through critical edge splitting. \
                         This may indicate a bug in the phi resolver.".to_string()
                    ),
                }
            }
            
            CodeGenError::IoError(io_err) => {
                CompileError::IoError(io_err)
            }
        }
    }
}
```

### 16. CodeGenResult

**Result type for generation**:

```rust
pub struct CodeGenResult {
    /// Generated assembly code (if any portion succeeded)
    pub assembly: Option<String>,
    
    /// List of all errors encountered
    pub errors: Vec<CodeGenError>,
    
    /// Statistics about generation
    pub stats: CodeGenStats,
}

pub struct CodeGenStats {
    /// Number of functions successfully generated
    pub functions_generated: usize,
    
    /// Number of functions that failed
    pub functions_failed: usize,
    
    /// Number of IR instructions translated
    pub instructions_translated: usize,
    
    /// Number of assembly instructions emitted
    pub assembly_instructions: usize,
    
    /// Number of register spills
    pub register_spills: usize,
    
    /// Total stack frame size across all functions
    pub total_stack_size: usize,
}
```

## Summary

This data model provides a complete, precise, and meticulous specification of all internal data structures used by the x86-64 NASM assembly code generator. Every struct, field, invariant, algorithm, and mapping has been documented with the level of detail necessary to implement the generator correctly and maintainably, in full compliance with the jsavrs constitution's Documentation Rigor principle.

Key relationships:
- `AsmGenerator` orchestrates all components
- `FunctionContext` maintains per-function state during generation
- `LinearScanAllocator` produces `RegisterAssignment` for value-to-register mapping
- `InstructionSelector` uses `RegisterAssignment` to translate IR instructions
- `PhiResolver` modifies CFG to eliminate SSA phi functions
- `StackFrame` calculates complete stack layout including spills and shadow space
- All components respect ABI conventions selected from `TargetTriple`

---

**Document Completed**: 2025-10-17  
**Reviewed By**: AI Assistant (detailed, precise, meticulous, in-depth documentation per Documentation Rigor principle)
