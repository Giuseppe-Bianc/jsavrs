# Research: x86-64 NASM Assembly Code Generator

**Feature**: 007-x86-64-asm-generator  
**Date**: 2025-10-17  
**Status**: Phase 0 Research Complete

## Executive Summary

This research document analyzes the technical approaches, algorithms, and design trade-offs for implementing a complete x86-64 NASM assembly code generator that translates the jsavrs Intermediate Representation (IR) into executable assembly code. The generator must handle register allocation, instruction selection, calling convention compliance, SSA phi function resolution, and cross-platform ABI differences while maintaining correctness, performance, and extensibility.

## Research Topics

### 1. Register Allocation Strategies

**Decision**: Linear Scan Register Allocation with Furthest-Use Spilling

**Rationale**:
- **Simplicity**: Linear scan is significantly simpler to implement and debug than graph coloring algorithms, reducing initial development time and bug surface
- **Performance**: O(n) complexity where n = number of IR instructions, meeting the <1s per 1000 instructions performance target
- **Effectiveness**: Produces near-optimal register assignments for most programs (within 10-15% of graph coloring in register usage)
- **Extensibility**: Clear upgrade path to more sophisticated algorithms (graph coloring with coalescing) without architectural changes
- **Proven**: Used successfully in production compilers (Java HotSpot C1, Mono JIT, LuaJIT)

**Alternatives Considered**:

1. **Graph Coloring with Chaitin-Briggs Algorithm**
   - **Pros**: Optimal register allocation for most cases, better handling of complex interference patterns
   - **Cons**: O(n²) to O(n³) complexity, significantly more complex implementation, harder to debug
   - **Rejected Because**: Complexity outweighs benefits for initial implementation. Can be added later as optimization pass.

2. **Simple Allocation (Greedy Without Spilling)**
   - **Pros**: Trivial to implement, very fast
   - **Cons**: Cannot handle register pressure, fails on functions with >16 live values (x86-64 GP registers)
   - **Rejected Because**: Does not meet requirement to handle functions with 20+ live values (FR-038)

3. **SSA-Based Register Allocation (Hack et al.)**
   - **Pros**: Leverages SSA form for better allocation, eliminates move instructions more aggressively
   - **Cons**: Requires SSA form to be maintained during code generation, more complex phi handling
   - **Rejected Because**: jsavrs resolves SSA phi functions before code generation (critical edge splitting approach), making SSA-based allocation incompatible

**Implementation Details**:

```rust
// Linear Scan Register Allocation Algorithm (Poletto & Sarkar, 1999)
struct LinearScanAllocator {
    intervals: Vec<LiveInterval>,      // Sorted by start point
    active: Vec<LiveInterval>,         // Currently allocated intervals
    available_regs: Vec<Register>,     // Free physical registers
    spill_stack: HashMap<ValueId, StackSlot>,  // Spilled values
}

impl LinearScanAllocator {
    fn allocate(&mut self) -> Result<RegisterAssignment> {
        // 1. Compute live intervals for all IR values
        self.compute_liveness()?;
        
        // 2. Sort intervals by start point (already in IR instruction order)
        self.intervals.sort_by_key(|i| i.start);
        
        // 3. For each interval i:
        for interval in self.intervals.drain(..) {
            // Expire old intervals (no longer live)
            self.expire_old_intervals(interval.start);
            
            if self.available_regs.is_empty() {
                // Spill: choose furthest-use interval
                let spill_candidate = self.find_furthest_use(interval.start);
                if spill_candidate.end > interval.end {
                    // Spill the candidate, allocate register to current interval
                    self.spill(spill_candidate)?;
                } else {
                    // Spill current interval (it lives longer)
                    self.spill(interval)?;
                    continue;
                }
            }
            
            // Allocate register to interval
            let reg = self.available_regs.pop().unwrap();
            self.active.push((interval, reg));
        }
        
        Ok(self.build_assignment())
    }
    
    fn find_furthest_use(&self, current_pos: u32) -> &LiveInterval {
        // Furthest-use heuristic: spill the value with the furthest next use
        self.active.iter()
            .max_by_key(|interval| interval.next_use_after(current_pos))
            .unwrap()
    }
}
```

**Liveness Analysis**:
- **Approach**: Backward dataflow analysis on CFG basic blocks
- **Representation**: Bit vectors for live-in/live-out sets per block
- **Complexity**: O(n * e) where n = instructions, e = CFG edges (typically 2-3 passes for convergence)
- **Optimization**: Leverage existing `DominanceInfo` to reduce iterations

**Spilling Strategy**:
- **Trigger**: When all physical registers are allocated and a new value needs a register
- **Selection**: Furthest-use heuristic (spill the value that won't be used for the longest time)
- **Location**: Stack slots allocated in function prologue (included in stack frame calculation)
- **Reloading**: Insert load instructions immediately before each use of spilled value

### 2. Instruction Selection

**Decision**: Direct Pattern Matching with Type-Driven Size Selection

**Rationale**:
- **Correctness**: One-to-one mapping from IR instructions to x86-64 instructions ensures semantic preservation
- **Simplicity**: Pattern matching on `InstructionKind` enum is idiomatic Rust with exhaustive checking
- **Type Safety**: IR type information directly determines instruction size (byte/word/dword/qword)
- **Debugging**: Straightforward correspondence between IR and assembly aids debugging and verification

**Alternatives Considered**:

1. **Tree Pattern Matching (BURS - Bottom-Up Rewrite System)**
   - **Pros**: Can match complex expression trees to single instructions (e.g., `lea` for multiply+add)
   - **Cons**: Significantly more complex, requires tree construction phase, overkill for unoptimized code
   - **Rejected Because**: Out of scope for initial implementation (OS-001: optimization passes)

2. **Table-Driven Selection (LLVM TableGen style)**
   - **Pros**: Declarative specification, machine-readable, easy to extend to new architectures
   - **Cons**: Requires external DSL and code generation tooling, adds build complexity
   - **Rejected Because**: x86-64 is the only target architecture (A-004), table-driven approach is overkill

**Implementation Pattern**:

```rust
fn select_instruction(&self, ir_inst: &Instruction) -> Result<Vec<AsmInstruction>> {
    match &ir_inst.kind {
        InstructionKind::Binary { op, left, right, result_type } => {
            match op {
                IrBinaryOp::Add => self.select_add(left, right, result_type),
                IrBinaryOp::Multiply => self.select_mul(left, right, result_type),
                // ... other operations
            }
        }
        InstructionKind::Load { address, result_type } => {
            self.select_load(address, result_type)
        }
        InstructionKind::Store { address, value } => {
            self.select_store(address, value)
        }
        // ... other instruction kinds
    }
}

fn select_add(&self, left: &Value, right: &Value, ty: &IrType) -> Result<Vec<AsmInstruction>> {
    let size = self.type_to_size(ty)?;
    let dest = self.allocate_register(ty)?;
    
    // x86-64 add is two-operand: add dest, src (dest += src)
    vec![
        AsmInstruction::Mov { dest: dest.clone(), src: left.into(), size },
        AsmInstruction::Add { dest, src: right.into(), size },
    ]
}
```

**Type-to-Instruction Size Mapping**:
```rust
fn type_to_size(&self, ty: &IrType) -> OperandSize {
    match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => OperandSize::Byte,
        IrType::I16 | IrType::U16 | IrType::Char => OperandSize::Word,
        IrType::I32 | IrType::U32 | IrType::F32 => OperandSize::Dword,
        IrType::I64 | IrType::U64 | IrType::F64 | IrType::Pointer(_) => OperandSize::Qword,
        _ => return Err(UnsupportedType(ty.clone())),
    }
}
```

**Signed vs Unsigned Semantics**:
- **Arithmetic**: Most operations (add, sub, bitwise) are identical for signed/unsigned at bit level
- **Multiplication**: Use `imul` (signed) or `mul` (unsigned) based on IR type signedness
- **Division**: Use `idiv` (signed) or `div` (unsigned) based on IR type signedness
- **Comparisons**: Use signed jumps (jl, jg, jle, jge) or unsigned jumps (jb, ja, jbe, jae)
- **Shifts**: Logical shifts (shl, shr) for unsigned, arithmetic shift right (sar) for signed

### 3. SSA Phi Function Resolution

**Decision**: Critical Edge Splitting with Move Instruction Insertion at Predecessor Block Ends

**Rationale**:
- **Correctness**: Proven technique from SSA literature (Cytron et al., 1991; Briggs et al., 1998)
- **Simplicity**: Transforms phi functions into conventional assignments without complex register allocation interactions
- **Compatibility**: Works with any register allocation algorithm (linear scan, graph coloring)
- **Clear Semantics**: Each predecessor block explicitly sets the phi variable to its corresponding value

**Alternatives Considered**:

1. **Register Coalescing During Allocation**
   - **Pros**: Can eliminate some move instructions by allocating phi sources and target to same register
   - **Cons**: Significantly complicates register allocation, may not eliminate all moves anyway
   - **Rejected Because**: Added complexity for marginal benefit; move elimination is an optimization (out of scope per OS-001)

2. **Parallel Copy Semantics with Temporary Variables**
   - **Pros**: Preserves SSA semantics more directly, avoids critical edge splitting
   - **Cons**: Requires careful ordering to avoid overwriting values, complex when multiple phis exist
   - **Rejected Because**: Critical edge splitting is simpler and more robust

**Critical Edge Definition**:
An edge in the CFG is *critical* if it connects a block with multiple successors to a block with multiple predecessors. Phi functions on critical edges create ambiguity about where to insert move instructions.

**Algorithm**:

```rust
fn resolve_phi_functions(&mut self, function: &Function) -> Result<()> {
    for block in function.cfg.blocks() {
        if let Some(phis) = self.extract_phi_instructions(block) {
            for phi in phis {
                for (pred_label, value) in phi.incoming_values {
                    let pred_block = function.cfg.find_block_by_label(&pred_label)?;
                    
                    // Check if edge is critical
                    if self.is_critical_edge(pred_block, block) {
                        // Split the edge by inserting a new block
                        let split_block = self.split_edge(pred_block, block)?;
                        // Insert move in the new block
                        self.insert_move(split_block, &phi.result, &value)?;
                    } else {
                        // Insert move at end of predecessor (before terminator)
                        self.insert_move_before_terminator(pred_block, &phi.result, &value)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_critical_edge(&self, from: &BasicBlock, to: &BasicBlock) -> bool {
    let from_successors = self.cfg.successors(from).count();
    let to_predecessors = self.cfg.predecessors(to).count();
    from_successors > 1 && to_predecessors > 1
}

fn split_edge(&mut self, from: &BasicBlock, to: &BasicBlock) -> Result<BasicBlockId> {
    // Create new block: from -> new_block -> to
    let new_block = BasicBlock::new(self.new_label());
    let new_block_id = self.cfg.add_block(new_block);
    
    // Update edges
    self.cfg.remove_edge(from.id, to.id);
    self.cfg.add_edge(from.id, new_block_id);
    self.cfg.add_edge(new_block_id, to.id);
    
    // Update terminator of 'from' to target new_block
    self.update_terminator_target(from.id, to.label, new_block.label);
    
    // Add unconditional branch in new_block to 'to'
    self.set_block_terminator(new_block_id, Terminator::branch(to.label));
    
    Ok(new_block_id)
}
```

**Example Transformation**:

```
Before (IR with phi):
  entry:
    br cond, label %then, label %else
  
  then:
    %x1 = ...
    br label %merge
  
  else:
    %x2 = ...
    br label %merge
  
  merge:
    %x = phi i32 [%x1, %then], [%x2, %else]
    ...

After (critical edge split + moves):
  entry:
    br cond, label %then, label %else
  
  then:
    %x1 = ...
    mov %x, %x1        ; Move inserted before branch
    br label %merge
  
  else:
    %x2 = ...
    mov %x, %x2        ; Move inserted before branch
    br label %merge
  
  merge:
    ; Phi function removed, %x is now a regular variable
    ...
```

### 4. Calling Convention (ABI) Implementation

**Decision**: Use Existing `Abi` Infrastructure with Platform-Based Selection

**Rationale**:
- **Reuse**: Existing `src/asm/abi.rs` already encodes System V and Microsoft x64 calling conventions
- **Correctness**: ABI specs are complex (32+ pages of specification); existing implementation is tested
- **Maintainability**: Centralized ABI logic prevents duplication and inconsistencies

**Platform Detection**:
```rust
fn select_abi(target: &TargetTriple) -> Abi {
    match target {
        TargetTriple::X86_64UnknownLinuxGnu | 
        TargetTriple::X86_64AppleDarwin => Abi::system_v(),
        TargetTriple::X86_64PcWindowsGnu |
        TargetTriple::X86_64PcWindowsMsvc => Abi::windows(),
    }
}
```

**System V AMD64 ABI Key Points**:
- **Integer Parameters**: RDI, RSI, RDX, RCX, R8, R9 (first 6), then stack (right-to-left)
- **Float Parameters**: XMM0-XMM7 (first 8), then stack
- **Return Values**: RAX (integer), XMM0 (float), RDX:RAX (128-bit)
- **Callee-Saved**: RBX, RBP, R12-R15
- **Caller-Saved**: RAX, RCX, RDX, RSI, RDI, R8-R11
- **Stack Alignment**: 16-byte before call (RSP % 16 == 0 before call instruction)
- **Red Zone**: 128 bytes below RSP available for scratch (leaf functions)

**Microsoft x64 ABI Key Points**:
- **Integer Parameters**: RCX, RDX, R8, R9 (first 4), then stack (right-to-left)
- **Float Parameters**: XMM0-XMM3 (first 4), then stack
- **Return Values**: RAX (integer), XMM0 (float)
- **Callee-Saved**: RBX, RBP, RDI, RSI, RSP, R12-R15
- **Caller-Saved**: RAX, RCX, RDX, R8-R11
- **Stack Alignment**: 16-byte before call
- **Shadow Space**: 32 bytes (4 register slots) allocated by caller above return address
- **No Red Zone**: Must allocate stack space explicitly

**Function Prologue Generation**:

```rust
fn generate_prologue(&self, func: &Function, abi: &Abi) -> Vec<AsmInstruction> {
    let mut prologue = vec![
        // Standard prologue: save frame pointer, set up new frame
        AsmInstruction::Push(Register::RBP),
        AsmInstruction::Mov {
            dest: Operand::Register(Register::RBP),
            src: Operand::Register(Register::RSP),
            size: OperandSize::Qword,
        },
    ];
    
    // Calculate stack frame size
    let local_vars_size = self.calculate_local_vars_size(func);
    let spill_slots_size = self.register_allocator.spill_slots_size();
    let shadow_space = if abi.is_windows() && self.function_makes_calls(func) {
        32  // Microsoft x64 shadow space
    } else {
        0
    };
    let alignment_padding = self.calculate_alignment_padding(
        local_vars_size + spill_slots_size + shadow_space, 16
    );
    
    let total_stack = local_vars_size + spill_slots_size + shadow_space + alignment_padding;
    
    if total_stack > 0 {
        prologue.push(AsmInstruction::Sub {
            dest: Operand::Register(Register::RSP),
            src: Operand::Immediate(Immediate::I32(total_stack as i32)),
            size: OperandSize::Qword,
        });
    }
    
    // Save callee-saved registers used by this function
    for reg in self.used_callee_saved_registers(func, abi) {
        prologue.push(AsmInstruction::Push(reg));
    }
    
    prologue
}
```

**Function Epilogue Generation**:

```rust
fn generate_epilogue(&self, func: &Function, abi: &Abi) -> Vec<AsmInstruction> {
    let mut epilogue = vec![];
    
    // Restore callee-saved registers (reverse order of prologue)
    for reg in self.used_callee_saved_registers(func, abi).iter().rev() {
        epilogue.push(AsmInstruction::Pop(reg.clone()));
    }
    
    // Tear down stack frame
    epilogue.push(AsmInstruction::Mov {
        dest: Operand::Register(Register::RSP),
        src: Operand::Register(Register::RBP),
        size: OperandSize::Qword,
    });
    epilogue.push(AsmInstruction::Pop(Register::RBP));
    
    // Return
    epilogue.push(AsmInstruction::Ret);
    
    epilogue
}
```

### 5. Control Flow Translation

**Decision**: Direct CFG Traversal with Label Generation

**Rationale**:
- **Simplicity**: One-to-one mapping from CFG basic blocks to assembly labels
- **Correctness**: Preserves all control flow edges from IR
- **Efficiency**: Single-pass traversal in CFG order (post-order or topological sort)

**Label Generation**:
```rust
fn generate_label(&self, block: &BasicBlock) -> String {
    // Use IR block label directly, prefixed for uniqueness
    format!(".L{}", block.label)
}
```

**Terminator Translation**:

```rust
fn translate_terminator(&self, term: &Terminator) -> Vec<AsmInstruction> {
    match &term.kind {
        TerminatorKind::Return { value, .. } => {
            let mut insns = vec![];
            if let Some(val) = value {
                // Move return value to appropriate register (RAX for int, XMM0 for float)
                let ret_reg = self.abi.return_register(val.ty);
                insns.push(self.move_to_register(val, ret_reg));
            }
            insns.extend(self.generate_epilogue());
            insns
        }
        
        TerminatorKind::Branch { target } => {
            vec![AsmInstruction::Jmp(self.generate_label(target))]
        }
        
        TerminatorKind::ConditionalBranch { condition, true_target, false_target } => {
            // Assume condition is already in flags from previous cmp instruction
            vec![
                AsmInstruction::Je(self.generate_label(true_target)),  // Jump if equal (condition true)
                AsmInstruction::Jmp(self.generate_label(false_target)),  // Fall through to false
            ]
        }
        
        TerminatorKind::Switch { value, default, cases } => {
            // Simple implementation: chain of comparisons (jump table optimization future work)
            let mut insns = vec![];
            let val_reg = self.get_value_register(value)?;
            
            for (case_val, case_label) in cases {
                insns.push(AsmInstruction::Cmp {
                    left: Operand::Register(val_reg.clone()),
                    right: Operand::Immediate(case_val.clone()),
                    size: self.type_to_size(value.ty)?,
                });
                insns.push(AsmInstruction::Je(self.generate_label(case_label)));
            }
            
            // Default case
            insns.push(AsmInstruction::Jmp(self.generate_label(default)));
            insns
        }
        
        TerminatorKind::Unreachable => {
            // Insert undefined instruction (triggers exception if reached)
            vec![AsmInstruction::Ud2]
        }
    }
}
```

**Comparison Instruction Generation**:
For conditional branches, comparisons generate flag-setting instructions:

```rust
fn translate_comparison(&self, op: IrBinaryOp, left: &Value, right: &Value) 
    -> (Vec<AsmInstruction>, ConditionCode) {
    let size = self.type_to_size(left.ty)?;
    let left_reg = self.get_value_register(left)?;
    
    let cmp = AsmInstruction::Cmp {
        left: Operand::Register(left_reg),
        right: self.value_to_operand(right),
        size,
    };
    
    let condition = match op {
        IrBinaryOp::Equal => ConditionCode::Equal,
        IrBinaryOp::NotEqual => ConditionCode::NotEqual,
        IrBinaryOp::Less if left.ty.is_signed() => ConditionCode::Less,
        IrBinaryOp::Less => ConditionCode::Below,  // Unsigned
        IrBinaryOp::Greater if left.ty.is_signed() => ConditionCode::Greater,
        IrBinaryOp::Greater => ConditionCode::Above,  // Unsigned
        // ... other comparisons
    };
    
    (vec![cmp], condition)
}
```

### 6. Error Handling and Recovery

**Decision**: Error Accumulation with Partial Code Generation

**Rationale**:
- **Robustness**: Generator does not crash on unsupported IR constructs
- **Developer Experience**: All errors collected and reported together (no "whack-a-mole" debugging)
- **Partial Generation**: Valid code is generated for successfully translated portions, aiding debugging

**Error Types**:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported IR type: {0}")]
    UnsupportedType(IrType),
    
    #[error("Unknown IR instruction kind at {location}")]
    UnknownInstruction { location: SourceSpan },
    
    #[error("Malformed IR instruction: {reason} at {location}")]
    MalformedInstruction { reason: String, location: SourceSpan },
    
    #[error("Register allocation failed: {0}")]
    RegisterAllocationFailed(String),
    
    #[error("Invalid calling convention for target {0}")]
    InvalidCallingConvention(String),
    
    #[error("CFG verification failed: {0}")]
    CfgVerificationFailed(String),
}

pub struct CodeGenResult {
    pub assembly: Option<String>,
    pub errors: Vec<CodeGenError>,
}
```

**Error Handling Pattern**:

```rust
impl AsmGenerator {
    fn generate(&mut self, module: &Module) -> CodeGenResult {
        let mut errors = Vec::new();
        let mut functions = Vec::new();
        
        for func in module.functions() {
            match self.generate_function(func) {
                Ok(asm_func) => functions.push(asm_func),
                Err(err) => {
                    errors.push(err);
                    // Continue with next function
                }
            }
        }
        
        let assembly = if functions.is_empty() {
            None
        } else {
            Some(self.assemble_output(functions))
        };
        
        CodeGenResult { assembly, errors }
    }
    
    fn generate_instruction(&mut self, inst: &Instruction) -> Result<Vec<AsmInstruction>, CodeGenError> {
        match &inst.kind {
            InstructionKind::Alloca { .. } => self.translate_alloca(inst),
            InstructionKind::Store { .. } => self.translate_store(inst),
            // ... handle all supported instructions
            
            InstructionKind::Vector { .. } => {
                // Unsupported: SIMD operations out of scope (OS-014)
                Err(CodeGenError::UnsupportedInstruction {
                    kind: "Vector",
                    location: inst.debug_info.source_span.clone(),
                })
            }
        }
    }
}
```

### 7. Memory Addressing Modes

**Decision**: Leverage x86-64 Addressing Capabilities with `GetElementPtr` Translation

**Rationale**:
- **Hardware Support**: x86-64 supports complex addressing: `[base + index*scale + displacement]`
- **Efficiency**: Single `lea` or indexed `mov` instruction for array access calculations
- **IR Compatibility**: `GetElementPtr` IR instruction maps naturally to x86-64 addressing

**Addressing Mode Selection**:

```rust
fn translate_get_element_ptr(&self, gep: &GetElementPtrInstruction) -> Result<MemoryOperand> {
    let base_reg = self.get_value_register(&gep.base)?;
    
    match &gep.indices[..] {
        // Simple offset: base + constant
        [Index::Constant(offset)] => {
            MemoryOperand {
                base: Some(base_reg),
                index: None,
                scale: 1,
                displacement: *offset as i32,
                size: self.type_to_size(&gep.result_type)?,
            }
        }
        
        // Array indexing: base + index * element_size
        [Index::Value(idx_val)] => {
            let idx_reg = self.get_value_register(idx_val)?;
            let element_size = self.element_size(&gep.element_type)?;
            
            MemoryOperand {
                base: Some(base_reg),
                index: Some(idx_reg),
                scale: element_size,  // 1, 2, 4, or 8
                displacement: 0,
                size: self.type_to_size(&gep.result_type)?,
            }
        }
        
        // Nested indexing: translate to multiple lea/add instructions
        _ => self.translate_complex_gep(gep),
    }
}

fn element_size(&self, ty: &IrType) -> Result<u8> {
    match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => Ok(1),
        IrType::I16 | IrType::U16 | IrType::Char => Ok(2),
        IrType::I32 | IrType::U32 | IrType::F32 => Ok(4),
        IrType::I64 | IrType::U64 | IrType::F64 | IrType::Pointer(_) => Ok(8),
        _ => Err(CodeGenError::UnsupportedType(ty.clone())),
    }
}
```

**LEA Instruction Usage**:
```rust
// For address calculation without memory access
let address_calc = AsmInstruction::Lea {
    dest: Operand::Register(dest_reg),
    src: MemoryOperand {
        base: Some(base_reg),
        index: Some(index_reg),
        scale: 8,  // For i64 array
        displacement: 0,
        size: OperandSize::Qword,
    },
};

// Then use the calculated address for load/store
let load = AsmInstruction::Mov {
    dest: Operand::Register(result_reg),
    src: Operand::Memory(MemoryOperand {
        base: Some(dest_reg),
        index: None,
        scale: 1,
        displacement: 0,
        size: OperandSize::Qword,
    }),
    size: OperandSize::Qword,
};
```

### 8. Floating-Point Operations

**Decision**: SSE2 Scalar Instructions (addss/addsd, mulss/mulsd, etc.)

**Rationale**:
- **Modern Standard**: SSE2 is baseline requirement for x86-64 (all x86-64 CPUs have SSE2)
- **Simplicity**: Scalar SSE instructions map directly to IR float operations
- **Register Model**: XMM registers are straightforward (no stack management like x87 FPU)
- **ABI Compliance**: All modern ABIs use XMM registers for float parameter passing and returns

**Alternatives Considered**:

1. **x87 FPU Instructions**
   - **Pros**: Higher precision (80-bit extended precision), available on all x86 CPUs
   - **Cons**: Stack-based architecture is complex, not used by modern ABIs, legacy technology
   - **Rejected Because**: SSE2 is simpler and matches ABI requirements (FR-037)

2. **AVX Instructions (vaddss, vaddsd, etc.)**
   - **Pros**: Three-operand form (dest, src1, src2), avoids move instructions
   - **Cons**: Not available on all x86-64 CPUs (requires CPU feature detection), more complex
   - **Rejected Because**: Out of scope for initial implementation (OS-014: advanced SIMD)

**Float Operation Translation**:

```rust
fn translate_float_binary(&self, op: IrBinaryOp, left: &Value, right: &Value, ty: &IrType) 
    -> Result<Vec<AsmInstruction>> {
    let dest_reg = self.allocate_xmm_register()?;
    let left_reg = self.get_value_xmm_register(left)?;
    let right_operand = self.value_to_operand(right);
    
    let instruction = match (op, ty) {
        (IrBinaryOp::Add, IrType::F32) => AsmInstruction::Addss { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Add, IrType::F64) => AsmInstruction::Addsd { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Multiply, IrType::F32) => AsmInstruction::Mulss { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Multiply, IrType::F64) => AsmInstruction::Mulsd { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Divide, IrType::F32) => AsmInstruction::Divss { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Divide, IrType::F64) => AsmInstruction::Divsd { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Subtract, IrType::F32) => AsmInstruction::Subss { dest: dest_reg, src1: left_reg, src2: right_operand },
        (IrBinaryOp::Subtract, IrType::F64) => AsmInstruction::Subsd { dest: dest_reg, src1: left_reg, src2: right_operand },
        _ => return Err(CodeGenError::UnsupportedOperation),
    };
    
    Ok(vec![instruction])
}
```

**Float Comparison Translation**:
```rust
fn translate_float_comparison(&self, op: IrBinaryOp, left: &Value, right: &Value, ty: &IrType) 
    -> Result<Vec<AsmInstruction>> {
    let left_reg = self.get_value_xmm_register(left)?;
    let right_operand = self.value_to_operand(right);
    
    let cmp_instruction = match ty {
        IrType::F32 => AsmInstruction::Ucomiss { src1: left_reg, src2: right_operand },
        IrType::F64 => AsmInstruction::Ucomisd { src1: left_reg, src2: right_operand },
        _ => return Err(CodeGenError::InvalidType),
    };
    
    // Set flags, then use conditional jump based on comparison type
    Ok(vec![cmp_instruction])
}
```

**Float-Int Conversions**:
```rust
fn translate_cast(&self, cast: &CastInstruction) -> Result<Vec<AsmInstruction>> {
    match (&cast.from_type, &cast.to_type, &cast.kind) {
        // Float to integer (truncation toward zero)
        (IrType::F32, IrType::I32, CastKind::FloatToInt) => {
            let src_xmm = self.get_value_xmm_register(&cast.value)?;
            let dest_gpr = self.allocate_gp_register()?;
            Ok(vec![AsmInstruction::Cvttss2si { dest: dest_gpr, src: src_xmm }])
        }
        (IrType::F64, IrType::I64, CastKind::FloatToInt) => {
            let src_xmm = self.get_value_xmm_register(&cast.value)?;
            let dest_gpr = self.allocate_gp_register()?;
            Ok(vec![AsmInstruction::Cvttsd2si { dest: dest_gpr, src: src_xmm }])
        }
        
        // Integer to float
        (IrType::I32, IrType::F32, CastKind::IntToFloat) => {
            let src_gpr = self.get_value_register(&cast.value)?;
            let dest_xmm = self.allocate_xmm_register()?;
            Ok(vec![AsmInstruction::Cvtsi2ss { dest: dest_xmm, src: src_gpr }])
        }
        (IrType::I64, IrType::F64, CastKind::IntToFloat) => {
            let src_gpr = self.get_value_register(&cast.value)?;
            let dest_xmm = self.allocate_xmm_register()?;
            Ok(vec![AsmInstruction::Cvtsi2sd { dest: dest_xmm, src: src_gpr }])
        }
        
        // Float precision conversions
        (IrType::F32, IrType::F64, CastKind::FloatExtend) => {
            let src_xmm = self.get_value_xmm_register(&cast.value)?;
            let dest_xmm = self.allocate_xmm_register()?;
            Ok(vec![AsmInstruction::Cvtss2sd { dest: dest_xmm, src: src_xmm }])
        }
        (IrType::F64, IrType::F32, CastKind::FloatTruncate) => {
            let src_xmm = self.get_value_xmm_register(&cast.value)?;
            let dest_xmm = self.allocate_xmm_register()?;
            Ok(vec![AsmInstruction::Cvtsd2ss { dest: dest_xmm, src: src_xmm }])
        }
        
        _ => Err(CodeGenError::UnsupportedCast),
    }
}
```

### 9. Assembly Output Format

**Decision**: NASM Syntax with Section Directives and Comment Annotations

**Rationale**:
- **Requirement**: Spec explicitly mandates NASM (Netwide Assembler) syntax (FR-004)
- **Simplicity**: NASM syntax is straightforward and widely documented
- **Debugging**: Comments preserve IR instruction information for debugging

**Output Structure**:

```nasm
; Generated by jsavrs compiler
; Source: input.vn
; Target: x86_64-unknown-linux-gnu
; ABI: System V AMD64

section .text
    global main

; Function: main
; Signature: () -> i32
main:
    ; Prologue
    push rbp
    mov rbp, rsp
    sub rsp, 16          ; Local variables: 16 bytes
    
    ; IR: %t0 = add i32 42, 58
    mov eax, 42
    add eax, 58
    
    ; IR: ret i32 %t0
    ; Return value already in eax
    mov rsp, rbp
    pop rbp
    ret

section .data
    ; Global constants
    .str0: db "Hello, world!", 0

section .rodata
    ; Read-only data

section .bss
    ; Uninitialized data
```

**Comment Generation**:

```rust
fn generate_assembly_output(&self, functions: Vec<AsmFunction>) -> String {
    let mut output = String::new();
    
    // Header comments
    output.push_str("; Generated by jsavrs compiler\n");
    output.push_str(&format!("; Date: {}\n", chrono::Local::now()));
    output.push_str(&format!("; Target: {}\n", self.target_triple));
    output.push_str(&format!("; ABI: {}\n\n", self.abi.name()));
    
    // Text section (code)
    output.push_str("section .text\n");
    for func in &functions {
        if func.is_entry {
            output.push_str(&format!("    global {}\n", func.name));
        }
        output.push_str("\n");
        output.push_str(&format!("; Function: {}\n", func.name));
        output.push_str(&format!("; Signature: {} -> {}\n", 
            self.format_params(&func.params), self.format_type(&func.return_type)));
        output.push_str(&format!("{}:\n", func.name));
        
        for (ir_inst, asm_insts) in func.instructions {
            // Add IR instruction as comment
            output.push_str(&format!("    ; IR: {}\n", ir_inst));
            
            // Add assembly instructions
            for asm in asm_insts {
                output.push_str(&format!("    {}\n", asm));
            }
        }
    }
    
    // Data sections
    output.push_str("\nsection .data\n");
    for data in &self.data_section {
        output.push_str(&format!("    {}\n", data));
    }
    
    output.push_str("\nsection .rodata\n");
    for data in &self.rodata_section {
        output.push_str(&format!("    {}\n", data));
    }
    
    output.push_str("\nsection .bss\n");
    for data in &self.bss_section {
        output.push_str(&format!("    {}\n", data));
    }
    
    output
}
```

### 10. Testing Strategy

**Decision**: Multi-Level Testing with Insta Snapshot Validation

**Rationale**:
- **Coverage**: Unit tests for each component, integration tests for end-to-end flows
- **Regression Prevention**: Insta snapshots catch unintended changes in assembly output
- **ABI Compliance**: Dedicated tests verify calling convention correctness
- **Performance Tracking**: Criterion benchmarks ensure performance remains acceptable

**Test Levels**:

1. **Unit Tests** (per component):
   - Register allocator: liveness analysis, spilling decisions, register assignment
   - Instruction selector: IR instruction → assembly mapping for all `InstructionKind`
   - Phi resolver: critical edge detection, edge splitting, move insertion
   - Prologue/epilogue generator: stack frame calculation, callee-saved register handling

2. **Integration Tests** (end-to-end):
   - Complete IR modules → assembly generation
   - Cross-platform: same IR on Linux/Windows/macOS targets
   - Error handling: malformed IR, unsupported types, missing terminators

3. **Snapshot Tests** (Insta):
   - Generated assembly for standard test cases (arithmetic, control flow, function calls)
   - Error messages for invalid IR constructs
   - Assembly comments and formatting

4. **ABI Compliance Tests**:
   - Parameter passing: verify correct registers used for different parameter counts/types
   - Return values: verify RAX/XMM0 usage
   - Stack alignment: verify 16-byte alignment before calls
   - Shadow space: verify 32-byte allocation for Windows

5. **Property-Based Tests** (if time permits):
   - Round-trip: IR → assembly → assemble → execute → verify
   - Register allocation: no live range conflicts, all values have registers or spill slots

**Example Unit Test**:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linear_scan_basic_allocation() {
        let mut allocator = LinearScanAllocator::new();
        
        // Create simple IR: two values with non-overlapping lifetimes
        let v1 = Value::new_temporary(1, IrType::I32);
        let v2 = Value::new_temporary(2, IrType::I32);
        
        allocator.add_interval(LiveInterval {
            value: v1.clone(),
            start: 0,
            end: 10,
            reg_class: RegisterClass::GeneralPurpose,
        });
        
        allocator.add_interval(LiveInterval {
            value: v2.clone(),
            start: 15,
            end: 25,
            reg_class: RegisterClass::GeneralPurpose,
        });
        
        let assignment = allocator.allocate().unwrap();
        
        // Both values should get registers (no spilling)
        assert!(assignment.get_register(&v1).is_some());
        assert!(assignment.get_register(&v2).is_some());
        
        // Can even reuse the same register (non-overlapping)
        assert_eq!(assignment.get_register(&v1), assignment.get_register(&v2));
    }
    
    #[test]
    fn test_linear_scan_spilling() {
        let mut allocator = LinearScanAllocator::with_register_count(2); // Only 2 registers
        
        // Create 3 overlapping values (forces spilling)
        for i in 1..=3 {
            allocator.add_interval(LiveInterval {
                value: Value::new_temporary(i, IrType::I32),
                start: 0,
                end: 100,
                reg_class: RegisterClass::GeneralPurpose,
            });
        }
        
        let assignment = allocator.allocate().unwrap();
        
        // One value must be spilled
        let spilled_count = (1..=3).filter(|&i| {
            let v = Value::new_temporary(i, IrType::I32);
            assignment.get_register(&v).is_none()
        }).count();
        
        assert_eq!(spilled_count, 1);
    }
}
```

**Example Snapshot Test**:

```rust
#[cfg(test)]
mod snapshot_tests {
    use insta::assert_snapshot;
    
    #[test]
    fn test_simple_arithmetic_codegen() {
        let ir = r#"
            define i32 @add(i32 %a, i32 %b) {
            entry:
                %result = add i32 %a, %b
                ret i32 %result
            }
        "#;
        
        let module = parse_ir(ir).unwrap();
        let generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);
        let result = generator.generate(&module);
        
        assert!(result.errors.is_empty());
        assert_snapshot!("simple_add_linux", result.assembly.unwrap());
    }
    
    #[test]
    fn test_function_call_codegen_windows() {
        let ir = r#"
            declare void @external_func(i32, i32)
            
            define void @caller() {
            entry:
                call void @external_func(i32 42, i32 58)
                ret void
            }
        "#;
        
        let module = parse_ir(ir).unwrap();
        let generator = AsmGenerator::new(TargetTriple::X86_64PcWindowsMsvc);
        let result = generator.generate(&module);
        
        assert!(result.errors.is_empty());
        
        let asm = result.assembly.unwrap();
        // Verify Microsoft x64 calling convention: RCX, RDX for params
        assert!(asm.contains("mov ecx, 42"));
        assert!(asm.contains("mov edx, 58"));
        // Verify shadow space allocation
        assert!(asm.contains("sub rsp, 32"));
        
        assert_snapshot!("function_call_windows", asm);
    }
}
```

## Research Summary

This comprehensive research establishes the technical foundation for implementing the x86-64 NASM assembly code generator. Key decisions include:

1. **Linear Scan Register Allocation**: Proven O(n) algorithm with furthest-use spilling, balancing simplicity and effectiveness
2. **Direct Pattern Matching Instruction Selection**: Type-driven, correct, and maintainable approach for unoptimized code generation
3. **Critical Edge Splitting for Phi Resolution**: Standard SSA resolution technique with clear semantics
4. **ABI Reuse**: Leverages existing `src/asm/abi.rs` infrastructure for calling convention correctness
5. **SSE2 Floating-Point**: Modern, ABI-compliant approach avoiding x87 complexity
6. **Error Accumulation**: Robust error handling enabling partial code generation and comprehensive diagnostics
7. **Insta Snapshot Testing**: Comprehensive regression prevention for assembly output validation

All decisions align with the jsavrs constitution principles (Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, Documentation Rigor) and satisfy the requirements specified in the feature spec.

## Next Steps

With research complete, proceed to **Phase 1: Design & Contracts** to create:
1. `data-model.md`: Detailed internal data structures (generator state, register allocator state, IR-to-assembly mappings)
2. `contracts/`: Public API specifications for `AsmGenerator`, `RegisterAllocator`, `InstructionSelector` traits
3. `quickstart.md`: Usage examples and integration guide for the generator
4. Update AI agent context with technologies: Linear Scan Register Allocation, x86-64 instruction set, NASM syntax, System V/Microsoft x64 ABIs, SSE2 floating-point

---

**Research Completed**: 2025-10-17  
**Reviewed By**: AI Assistant (detailed, precise, meticulous, in-depth analysis per Documentation Rigor principle)
