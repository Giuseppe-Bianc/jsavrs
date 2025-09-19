//! IR to ASM generator for x86-64 architecture using NASM syntax
use super::generator::{NasmGenerator, Section, TargetOS};
use super::instruction::Instruction as AsmInstruction;
use super::operand::Operand;
use super::register::Register;
use crate::ir::module::Module;
use crate::ir::function::Function;
use crate::ir::basic_block::BasicBlock;
use crate::ir::instruction::{Instruction, InstructionKind, IrBinaryOp, IrUnaryOp, CastKind, VectorOp};
use crate::ir::terminator::{Terminator, TerminatorKind};
use crate::ir::types::IrType;
use crate::ir::value::{Value, ValueKind, IrLiteralValue};
use std::collections::HashMap;

/// Maps IR types to appropriate x86-64 registers
fn map_type_to_register(ty: &IrType) -> Register {
    match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => Register::AL,
        IrType::I16 | IrType::U16 => Register::AX,
        IrType::I32 | IrType::U32 => Register::EAX,
        IrType::I64 | IrType::U64 => Register::RAX,
        IrType::F32 => Register::EAX, // Use integer register for floating point values
        IrType::F64 => Register::RAX,
        IrType::Pointer(_) => Register::RAX, // Pointers are 64-bit on x86-64
        _ => Register::RAX, // Default to 64-bit register
    }
}

/// Maps IR types to register size in bits
fn map_type_to_size(ty: &IrType) -> u8 {
    match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => 8,
        IrType::I16 | IrType::U16 => 16,
        IrType::I32 | IrType::U32 => 32,
        IrType::I64 | IrType::U64 | IrType::Pointer(_) => 64,
        IrType::F32 => 32,
        IrType::F64 => 64,
        IrType::Array(_, _) => 64, // Arrays are passed as pointers
        _ => 64, // Default to 64 bits
    }
}

/// Maps IR types to stack slot size in bytes (aligned to 8 bytes)
fn map_type_to_stack_size(ty: &IrType) -> u32 {
    let size = match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => 1,
        IrType::I16 | IrType::U16 => 2,
        IrType::I32 | IrType::U32 => 4,
        IrType::I64 | IrType::U64 | IrType::Pointer(_) => 8,
        IrType::F32 => 4,
        IrType::F64 => 8,
        IrType::Array(_, _) => 8, // Arrays are passed as pointers
        IrType::Char => 1,
        IrType::String => 8, // String is typically a pointer
        _ => 8, // Default to 8 bytes
    };
    
    // Align to 8-byte boundary
    ((size + 7) / 8) * 8
}

/// Converts a CastKind to a string representation
fn cast_kind_to_string(kind: &CastKind) -> &'static str {
    match kind {
        CastKind::IntToPtr => "inttoptr",
        CastKind::PtrToInt => "ptrtoint",
        CastKind::FloatToInt => "fptosi",
        CastKind::IntToFloat => "sitofp",
        CastKind::FloatTruncate => "fptrunc",
        CastKind::FloatExtend => "fpext",
        CastKind::IntTruncate => "trunc",
        CastKind::IntSignExtend => "sext",
        CastKind::IntZeroExtend => "zext",
        CastKind::Bitcast => "bitcast",
    }
}

/// Converts an IR value to an ASM operand
fn value_to_operand(value: &Value) -> Operand {
    match &value.kind {
        ValueKind::Literal(literal) => {
            match literal {
                IrLiteralValue::I8(val) => Operand::imm(*val as i64),
                IrLiteralValue::I16(val) => Operand::imm(*val as i64),
                IrLiteralValue::I32(val) => Operand::imm(*val as i64),
                IrLiteralValue::I64(val) => Operand::imm(*val),
                IrLiteralValue::U8(val) => Operand::imm(*val as i64),
                IrLiteralValue::U16(val) => Operand::imm(*val as i64),
                IrLiteralValue::U32(val) => Operand::imm(*val as i64),
                IrLiteralValue::U64(val) => Operand::imm(*val as i64),
                IrLiteralValue::F32(val) => Operand::imm(*val as i64),
                IrLiteralValue::F64(val) => Operand::imm(*val as i64),
                IrLiteralValue::Bool(val) => Operand::imm(if *val { 1 } else { 0 }),
                IrLiteralValue::Char(val) => Operand::imm(*val as i64),
            }
        }
        ValueKind::Local(name) => Operand::mem(name),
        ValueKind::Global(name) => Operand::label(name),
        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
        ValueKind::Constant(_) => {
            // For constants, we might need to handle them differently
            // For now, we'll just use a placeholder
            Operand::imm(0)
        }
    }
}

/// IR to ASM generator
pub struct IrToAsmGenerator {
    nasm_gen: NasmGenerator,
    /// Maps local variables/temporaries to their stack offsets
    local_vars: HashMap<String, i32>,
    /// Current stack offset (negative values for local variables)
    stack_offset: i32,
    /// Current function name
    current_function: String,
}

impl IrToAsmGenerator {
    /// Create a new IR to ASM generator
    pub fn new(target_os: TargetOS) -> Self {
        Self {
            nasm_gen: NasmGenerator::new(target_os),
            local_vars: HashMap::new(),
            stack_offset: 0,
            current_function: String::new(),
        }
    }

    /// Generate assembly code from an IR module
    pub fn generate_from_module(&mut self, module: &Module) -> String {
        self.nasm_gen.add_standard_prelude();
        
        // Generate all functions
        for function in &module.functions {
            self.generate_function(function);
        }
        
        self.nasm_gen.generate()
    }

    /// Generate assembly code for a function
    fn generate_function(&mut self, function: &Function) {
        self.current_function = function.name.clone();
        self.local_vars.clear();
        self.stack_offset = 0;
        
        self.nasm_gen.add_section(Section::Text);
        self.nasm_gen.add_global(&function.name);
        self.nasm_gen.add_empty_line();
        
        self.nasm_gen.add_label(&function.name);
        
        // Generate prologue
        self.generate_function_prologue(function);
        
        // Generate all basic blocks
        for block in function.cfg.blocks() {
            self.generate_basic_block(block);
        }
        
        // Generate epilogue
        self.generate_function_epilogue(function);
        
        self.nasm_gen.add_empty_line();
    }

    /// Generate function prologue
    fn generate_function_prologue(&mut self, _function: &Function) {
        self.nasm_gen.add_instruction(AsmInstruction::Push(Operand::reg(Register::RBP)));
        self.nasm_gen.add_instruction(AsmInstruction::Mov(
            Operand::reg(Register::RBP),
            Operand::reg(Register::RSP)
        ));
        // TODO: Reserve stack space for local variables
    }

    /// Generate function epilogue
    fn generate_function_epilogue(&mut self, _function: &Function) {
        self.nasm_gen.add_instruction(AsmInstruction::Pop(Operand::reg(Register::RBP)));
        self.nasm_gen.add_instruction(AsmInstruction::Ret);
    }

    /// Generate assembly code for a basic block
    fn generate_basic_block(&mut self, block: &BasicBlock) {
        self.nasm_gen.add_label(&block.label);
        
        // Generate all instructions in the block
        for instruction in &block.instructions {
            self.generate_instruction(instruction);
        }
        
        // Generate terminator
        self.generate_terminator(&block.terminator);
    }

    /// Generate assembly code for an IR instruction
    fn generate_instruction(&mut self, instruction: &Instruction) {
        match &instruction.kind {
            InstructionKind::Alloca { ty } => {
                self.generate_alloca(instruction, ty);
            }
            InstructionKind::Store { value, dest } => {
                self.generate_store(value, dest);
            }
            InstructionKind::Load { src, ty } => {
                self.generate_load(src, ty, instruction);
            }
            InstructionKind::Binary { op, left, right, ty } => {
                self.generate_binary(op, left, right, ty, instruction);
            }
            InstructionKind::Unary { op, operand, ty } => {
                self.generate_unary(op, operand, ty, instruction);
            }
            InstructionKind::Call { func, args, ty } => {
                self.generate_call(func, args, ty, instruction);
            }
            InstructionKind::GetElementPtr { base, index, element_ty } => {
                self.generate_gep(base, index, element_ty, instruction);
            }
            InstructionKind::Cast { kind, value, from_ty, to_ty } => {
                self.generate_cast(kind, value, from_ty, to_ty, instruction);
            }
            InstructionKind::Phi { ty, incoming } => {
                self.generate_phi(ty, incoming, instruction);
            }
            InstructionKind::Vector { op, operands, ty } => {
                self.generate_vector(op, operands, ty, instruction);
            }
        }
    }

    /// Generate assembly for alloca instruction
    fn generate_alloca(&mut self, instruction: &Instruction, ty: &IrType) {
        if let Some(result) = &instruction.result {
            // Calculate size of type
            let size = map_type_to_stack_size(ty);
            
            // Update stack offset
            self.stack_offset -= size as i32;
            
            // Map the result to its stack offset
            let var_name = match &result.kind {
                ValueKind::Temporary(id) => format!("t{}", id),
                ValueKind::Local(name) => name.to_string(),
                _ => format!("alloca_{}", result.id),
            };
            
            self.local_vars.insert(var_name.clone(), self.stack_offset);
            
            // Generate assembly to allocate stack space
            self.nasm_gen.add_instruction(AsmInstruction::Sub(
                Operand::reg(Register::RSP),
                Operand::imm(size as i64)
            ));
            
            // Store the address of the allocated space in the result
            // We need to store RSP (current stack top) in the result location
            let dest_addr = match &result.kind {
                ValueKind::Temporary(id) => {
                    Operand::mem(&format!("t{}", id))
                }
                ValueKind::Local(name) => {
                    Operand::mem(name)
                }
                _ => {
                    // For other cases, we'll use a temporary name
                    Operand::mem(&format!("alloca_{}", result.id))
                }
            };
            
            // Store the current stack pointer (which points to our allocated space) in the result
            self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_addr, Operand::reg(Register::RSP)));
            
            self.nasm_gen.add_comment(&format!("allocated {} bytes for {} -> {} at RBP{}", size, ty, result, self.stack_offset));
        }
    }

    /// Generate assembly for store instruction
    fn generate_store(&mut self, value: &Value, dest: &Value) {
        let value_operand = value_to_operand(value);
        
        // Get the destination address
        let dest_addr = match &dest.kind {
            ValueKind::Local(name) => {
                if let Some(offset) = self.local_vars.get(name.as_ref()) {
                    Operand::mem_ref(Some(Register::RBP), None, 1, *offset)
                } else {
                    Operand::mem(name)
                }
            }
            ValueKind::Temporary(id) => {
                let name = format!("t{}", id);
                if let Some(offset) = self.local_vars.get(&name) {
                    Operand::mem_ref(Some(Register::RBP), None, 1, *offset)
                } else {
                    Operand::mem(&name)
                }
            }
            _ => value_to_operand(dest),
        };
        
        self.nasm_gen.add_comment(&format!("store {} to {}", value, dest));
        self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_addr, value_operand));
    }

    /// Generate assembly for load instruction
    fn generate_load(&mut self, src: &Value, ty: &IrType, instruction: &Instruction) {
        if let Some(result) = &instruction.result {
            // Get the source address
            let src_addr = match &src.kind {
                ValueKind::Local(name) => {
                    if let Some(offset) = self.local_vars.get(name.as_ref()) {
                        Operand::mem_ref(Some(Register::RBP), None, 1, *offset)
                    } else {
                        Operand::mem(name)
                    }
                }
                ValueKind::Temporary(id) => {
                    let name = format!("t{}", id);
                    if let Some(offset) = self.local_vars.get(&name) {
                        Operand::mem_ref(Some(Register::RBP), None, 1, *offset)
                    } else {
                        Operand::mem(&name)
                    }
                }
                _ => value_to_operand(src),
            };
            
            // Determine the appropriate register based on type
            let dest_reg = map_type_to_register(ty);
            let dest_operand = Operand::reg(dest_reg);
            
            self.nasm_gen.add_comment(&format!("load {} from {} -> {}", ty, src, result));
            self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, src_addr));
        }
    }

    /// Generate assembly for binary operation
    fn generate_binary(
        &mut self,
        op: &IrBinaryOp,
        left: &Value,
        right: &Value,
        ty: &IrType,
        instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let left_operand = value_to_operand(left);
            let right_operand = value_to_operand(right);
            let dest_reg = map_type_to_register(ty);
            let dest_operand = Operand::reg(dest_reg.clone());
            
            // Move left operand to destination register
            self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), left_operand));
            
            // Apply the binary operation
            match op {
                IrBinaryOp::Add => {
                    self.nasm_gen.add_instruction(AsmInstruction::Add(dest_operand, right_operand));
                }
                IrBinaryOp::Subtract => {
                    self.nasm_gen.add_instruction(AsmInstruction::Sub(dest_operand, right_operand));
                }
                IrBinaryOp::Multiply => {
                    self.nasm_gen.add_instruction(AsmInstruction::Imul(dest_operand, Some(right_operand), None));
                }
                IrBinaryOp::Divide => {
                    // For division, we need to set up registers appropriately
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Cqo); // Sign extend RAX to RDX:RAX
                    self.nasm_gen.add_instruction(AsmInstruction::Idiv(Operand::reg(Register::RBX)));
                    // Result is in RAX, move it to destination if needed
                    let is_rax = dest_reg == Register::RAX;
                    if !is_rax {
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, Operand::reg(Register::RAX)));
                    }
                }
                IrBinaryOp::Modulo => {
                    // For modulo, we need to set up registers appropriately
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Cqo); // Sign extend RAX to RDX:RAX
                    self.nasm_gen.add_instruction(AsmInstruction::Idiv(Operand::reg(Register::RBX)));
                    // Remainder is in RDX, move it to result
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, Operand::reg(Register::RDX)));
                }
                IrBinaryOp::Equal => {
                    self.nasm_gen.add_instruction(AsmInstruction::Cmp(dest_operand.clone(), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), Operand::imm(0)));
                    self.nasm_gen.add_instruction(AsmInstruction::Sete(dest_operand));
                }
                IrBinaryOp::NotEqual => {
                    self.nasm_gen.add_instruction(AsmInstruction::Cmp(dest_operand.clone(), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), Operand::imm(0)));
                    self.nasm_gen.add_instruction(AsmInstruction::Setne(dest_operand));
                }
                IrBinaryOp::Less => {
                    self.nasm_gen.add_instruction(AsmInstruction::Cmp(dest_operand.clone(), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), Operand::imm(0)));
                    self.nasm_gen.add_instruction(AsmInstruction::Setl(dest_operand));
                }
                IrBinaryOp::LessEqual => {
                    self.nasm_gen.add_instruction(AsmInstruction::Cmp(dest_operand.clone(), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), Operand::imm(0)));
                    self.nasm_gen.add_instruction(AsmInstruction::Setle(dest_operand));
                }
                IrBinaryOp::Greater => {
                    self.nasm_gen.add_instruction(AsmInstruction::Cmp(dest_operand.clone(), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), Operand::imm(0)));
                    self.nasm_gen.add_instruction(AsmInstruction::Setg(dest_operand));
                }
                IrBinaryOp::GreaterEqual => {
                    self.nasm_gen.add_instruction(AsmInstruction::Cmp(dest_operand.clone(), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), Operand::imm(0)));
                    self.nasm_gen.add_instruction(AsmInstruction::Setge(dest_operand));
                }
                IrBinaryOp::And => {
                    self.nasm_gen.add_instruction(AsmInstruction::And(dest_operand, right_operand));
                }
                IrBinaryOp::Or => {
                    self.nasm_gen.add_instruction(AsmInstruction::Or(dest_operand, right_operand));
                }
                IrBinaryOp::BitwiseAnd => {
                    self.nasm_gen.add_instruction(AsmInstruction::And(dest_operand, right_operand));
                }
                IrBinaryOp::BitwiseOr => {
                    self.nasm_gen.add_instruction(AsmInstruction::Or(dest_operand, right_operand));
                }
                IrBinaryOp::BitwiseXor => {
                    self.nasm_gen.add_instruction(AsmInstruction::Xor(dest_operand, right_operand));
                }
                IrBinaryOp::ShiftLeft => {
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RCX), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Shl(dest_operand, Operand::reg(Register::RCX)));
                }
                IrBinaryOp::ShiftRight => {
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RCX), right_operand));
                    self.nasm_gen.add_instruction(AsmInstruction::Shr(dest_operand, Operand::reg(Register::RCX)));
                }
            }
            
            self.nasm_gen.add_comment(&format!("binary {} {} {} {} -> {}", op, left, right, ty, result));
        }
    }

    /// Generate assembly for unary operation
    fn generate_unary(
        &mut self,
        op: &IrUnaryOp,
        operand: &Value,
        ty: &IrType,
        instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let operand_val = value_to_operand(operand);
            let dest_reg = map_type_to_register(ty);
            let dest_operand = Operand::reg(dest_reg.clone());
            
            // Move operand to destination register
            self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand.clone(), operand_val));
            
            // Apply the unary operation
            match op {
                IrUnaryOp::Negate => {
                    self.nasm_gen.add_instruction(AsmInstruction::Neg(dest_operand));
                }
                IrUnaryOp::Not => {
                    self.nasm_gen.add_instruction(AsmInstruction::Not(dest_operand));
                }
            }
            
            self.nasm_gen.add_comment(&format!("unary {} {} {} -> {}", op, operand, ty, result));
        }
    }

    /// Generate assembly for function call
    fn generate_call(
        &mut self,
        func: &Value,
        args: &[Value],
        _ty: &IrType,
        instruction: &Instruction,
    ) {
        // In the System V AMD64 ABI (used on Linux), the first 6 integer/pointer arguments
        // are passed in registers: RDI, RSI, RDX, RCX, R8, R9
        let arg_registers = [
            Register::RDI,
            Register::RSI,
            Register::RDX,
            Register::RCX,
            Register::R8,
            Register::R9,
        ];
        
        // Push arguments to registers (simplified - real implementation would be more complex)
        for (i, arg) in args.iter().enumerate() {
            if i < arg_registers.len() {
                let arg_operand = value_to_operand(arg);
                let reg_operand = Operand::reg(arg_registers[i].clone()); // Clone the register
                self.nasm_gen.add_instruction(AsmInstruction::Mov(reg_operand, arg_operand));
            } else {
                // For more than 6 arguments, they would be pushed on the stack
                // This is a simplified implementation
                self.nasm_gen.add_comment(&format!("  arg {} (stack): {}", i, arg));
            }
        }
        
        // Get function name
        let func_name = match &func.kind {
            ValueKind::Global(name) => name.to_string(),
            ValueKind::Local(name) => name.to_string(),
            _ => format!("func_{}", func.id),
        };
        
        // Call the function
        self.nasm_gen.add_instruction(AsmInstruction::Call(func_name.clone()));
        
        // Handle result if needed
        if let Some(result) = &instruction.result {
            // Move result from RAX to destination
            // In a real implementation, we would handle the result properly
            self.nasm_gen.add_comment(&format!("  result {} stored in RAX", result));
        }
        
        self.nasm_gen.add_comment(&format!("call {} with {} args -> {:?}", func_name, args.len(), instruction.result));
    }

    /// Generate assembly for getelementptr instruction
    fn generate_gep(
        &mut self,
        base: &Value,
        index: &Value,
        element_ty: &IrType,
        instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let base_operand = value_to_operand(base);
            let index_operand = value_to_operand(index);
            
            // Calculate element size in bytes
            let element_size = map_type_to_stack_size(element_ty);
            
            // Generate actual assembly code for GEP operation:
            // 1. Load base address into RAX
            // 2. Load index into RBX
            // 3. Multiply index by element size
            // 4. Add offset to base
            // 5. Store result
            
            self.nasm_gen.add_comment(&format!("gep {} {} -> {}", base, index, result));
            self.nasm_gen.add_comment(&format!("  base: {}, index: {}", base_operand, index_operand));
            self.nasm_gen.add_comment(&format!("  element size: {} bytes", element_size));
            
            // Load base address into RAX
            self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), base_operand));
            
            // Load index into RBX
            self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), index_operand));
            
            // Multiply index by element size
            if element_size > 1 {
                self.nasm_gen.add_instruction(AsmInstruction::Imul(Operand::reg(Register::RBX), Some(Operand::imm(element_size as i64)), None));
            }
            
            // Add offset to base
            self.nasm_gen.add_instruction(AsmInstruction::Add(Operand::reg(Register::RAX), Operand::reg(Register::RBX)));
            
            // Store result
            let result_operand = match &result.kind {
                ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                ValueKind::Local(name) => Operand::mem(name),
                _ => Operand::mem(&format!("gep_{}", result.id)),
            };
            
            self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::RAX)));
        }
    }

    /// Generate assembly for cast instruction
    fn generate_cast(
        &mut self,
        kind: &CastKind,
        value: &Value,
        from_ty: &IrType,
        to_ty: &IrType,
        instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let value_operand = value_to_operand(value);
            let from_size = map_type_to_size(from_ty);
            let to_size = map_type_to_size(to_ty);
            let kind_str = cast_kind_to_string(kind);
            
            self.nasm_gen.add_comment(&format!("cast {} {} from {} ({} bits) to {} ({} bits) -> {}", 
                kind_str, value, from_ty, from_size, to_ty, to_size, result));
            self.nasm_gen.add_comment(&format!("  value: {}", value_operand));
            
            // Generate actual assembly code based on the cast kind
            match kind {
                CastKind::IntToPtr => {
                    self.nasm_gen.add_comment("  inttoptr: move integer to pointer register");
                    // For int to ptr, we typically just move the value
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::PtrToInt => {
                    self.nasm_gen.add_comment("  ptrtoint: move pointer to integer register");
                    // For ptr to int, we typically just move the value
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::FloatToInt => {
                    self.nasm_gen.add_comment("  fptosi: convert floating point to signed integer");
                    // This would typically use cvttss2si or similar SSE instructions
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::IntToFloat => {
                    self.nasm_gen.add_comment("  sitofp: convert signed integer to floating point");
                    // This would typically use cvtsi2ss or similar SSE instructions
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::FloatTruncate => {
                    self.nasm_gen.add_comment("  fptrunc: truncate floating point value");
                    // This would typically use cvtss2sd or similar SSE instructions
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::FloatExtend => {
                    self.nasm_gen.add_comment("  fpext: extend floating point value");
                    // This would typically use cvtsd2ss or similar SSE instructions
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::IntTruncate => {
                    self.nasm_gen.add_comment("  trunc: truncate integer value");
                    // For truncation, we move the value and it gets truncated automatically
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::IntSignExtend => {
                    self.nasm_gen.add_comment("  sext: sign extend integer value");
                    // For sign extension, we use movsx
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    // We'll use a generic move for now - in a real implementation we'd use movsx with proper sizing
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::IntZeroExtend => {
                    self.nasm_gen.add_comment("  zext: zero extend integer value");
                    // For zero extension, we use movzx
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    // We'll use a generic move for now - in a real implementation we'd use movzx with proper sizing
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
                CastKind::Bitcast => {
                    self.nasm_gen.add_comment("  bitcast: reinterpret bits without changing value");
                    // For bitcast, we just move the bits
                    let dest_operand = match &result.kind {
                        ValueKind::Temporary(id) => Operand::mem(&format!("t{}", id)),
                        ValueKind::Local(name) => Operand::mem(name),
                        _ => Operand::mem(&format!("cast_{}", result.id)),
                    };
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                }
            }
        }
    }

    /// Generate assembly for phi instruction
    fn generate_phi(
        &mut self,
        ty: &IrType,
        incoming: &[(Value, String)],
        instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let dest_reg = map_type_to_register(ty);
            let dest_operand = Operand::reg(dest_reg);
            
            // Phi nodes are resolved during SSA destruction or during code generation
            // In a real implementation, we would need to handle the control flow
            
            // For now, we'll generate a comment showing what the phi node represents
            // and generate placeholder code that would be filled in during SSA resolution
            self.nasm_gen.add_comment(&format!("phi {} -> {}", ty, result));
            self.nasm_gen.add_comment(&format!("  destination register: {}", dest_operand));
            self.nasm_gen.add_comment(&format!("  incoming values: {}", incoming.len()));
            
            // In a real implementation, phi nodes are typically handled by:
            // 1. Having each predecessor block set the phi value in a common register
            // 2. Using that register directly at the join point
            // 
            // For example:
            // Block A: mov rax, value1 ; jmp join
            // Block B: mov rax, value2 ; jmp join
            // Join: ; rax now contains the correct phi value
            
            // We'll generate a placeholder that indicates where the phi value would be computed
            self.nasm_gen.add_comment("  ; PHI node resolved during SSA destruction");
            self.nasm_gen.add_comment("  ; Predecessor blocks should set the value in the destination register");
            
            // Show what the incoming values are
            for (value, label) in incoming {
                self.nasm_gen.add_comment(&format!("    [{} <- {}]", value, label));
            }
            
            // In a real implementation, we might generate a selector or 
            // rely on predecessor blocks to set the value in the destination register
            self.nasm_gen.add_comment("; Placeholder for phi resolution - actual implementation depends on control flow");
        }
    }

    /// Generate assembly for vector instruction
    fn generate_vector(
        &mut self,
        op: &crate::ir::instruction::VectorOp,
        operands: &[Value],
        ty: &IrType,
        instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let dest_reg = map_type_to_register(ty);
            let dest_operand = Operand::reg(dest_reg);
            
            // Vector operations would typically use SIMD instructions
            // In a real implementation, we would need to:
            // 1. Load operands into vector registers
            // 2. Apply the vector operation
            // 3. Store the result
            
            self.nasm_gen.add_comment(&format!("vector.{} {} -> {}", op, ty, result));
            self.nasm_gen.add_comment(&format!("  destination register: {}", dest_operand));
            self.nasm_gen.add_comment(&format!("  operands: {}", operands.len()));
            
            // Show what the operands are
            for (i, operand) in operands.iter().enumerate() {
                let operand_val = value_to_operand(operand);
                self.nasm_gen.add_comment(&format!("    operand {}: {} = {}", i, operand, operand_val));
            }
            
            // Generate actual assembly code for vector operations
            match op {
                VectorOp::Add => {
                    self.nasm_gen.add_comment("  vadd: vector addition");
                    // Example: vaddps xmm0, xmm1, xmm2  ; Add packed single precision floats
                    // For now, we'll just generate a comment indicating what would be done
                    self.nasm_gen.add_comment("  ; vaddps xmm0, xmm1, xmm2  ; Add packed single precision floats");
                }
                VectorOp::Sub => {
                    self.nasm_gen.add_comment("  vsub: vector subtraction");
                    // Example: vsubps xmm0, xmm1, xmm2  ; Subtract packed single precision floats
                    self.nasm_gen.add_comment("  ; vsubps xmm0, xmm1, xmm2  ; Subtract packed single precision floats");
                }
                VectorOp::Mul => {
                    self.nasm_gen.add_comment("  vmul: vector multiplication");
                    // Example: vmulps xmm0, xmm1, xmm2  ; Multiply packed single precision floats
                    self.nasm_gen.add_comment("  ; vmulps xmm0, xmm1, xmm2  ; Multiply packed single precision floats");
                }
                VectorOp::Div => {
                    self.nasm_gen.add_comment("  vdiv: vector division");
                    // Example: vdivps xmm0, xmm1, xmm2  ; Divide packed single precision floats
                    self.nasm_gen.add_comment("  ; vdivps xmm0, xmm1, xmm2  ; Divide packed single precision floats");
                }
                VectorOp::DotProduct => {
                    self.nasm_gen.add_comment("  vdot: vector dot product");
                    // This would depend on specific implementation
                    self.nasm_gen.add_comment("  ; Vector dot product implementation depends on specific vector type");
                }
                VectorOp::Shuffle => {
                    self.nasm_gen.add_comment("  vshuffle: vector shuffle");
                    // Example: vpshufd xmm0, xmm1, imm8  ; Shuffle packed doublewords
                    self.nasm_gen.add_comment("  ; vpshufd xmm0, xmm1, imm8  ; Shuffle packed doublewords");
                }
            }
            
            // For a real implementation, we would generate appropriate assembly:
            // Example for vector addition:
            // movups xmm0, [operand1]   ; Load first operand into XMM register
            // movups xmm1, [operand2]   ; Load second operand into XMM register
            // vaddps xmm0, xmm0, xmm1   ; Add vectors
            // movups [result], xmm0     ; Store result
            
            self.nasm_gen.add_comment("; Vector operation implementation would generate appropriate SIMD instructions");
        }
    }

    /// Generate assembly for terminator
    fn generate_terminator(&mut self, terminator: &Terminator) {
        match &terminator.kind {
            TerminatorKind::Return { value, ty } => {
                self.generate_return(value, ty);
            }
            TerminatorKind::Branch { label } => {
                self.generate_branch(label);
            }
            TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
                self.generate_conditional_branch(condition, true_label, false_label);
            }
            TerminatorKind::IndirectBranch { address, possible_labels } => {
                self.generate_indirect_branch(address, possible_labels);
            }
            TerminatorKind::Switch { value, ty, default_label, cases } => {
                self.generate_switch(value, ty, default_label, cases);
            }
            TerminatorKind::Unreachable => {
                self.generate_unreachable();
            }
        }
    }

    /// Generate assembly for return terminator
    fn generate_return(&mut self, value: &Value, _ty: &IrType) {
        let value_operand = value_to_operand(value);
        // Move return value to RAX (or appropriate register based on calling convention)
        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), value_operand));
        
        // Jump to function epilogue
        // In a real implementation, we would jump to the epilogue or generate epilogue code here
        self.nasm_gen.add_comment(&format!("return {}", value));
    }

    /// Generate assembly for unconditional branch
    fn generate_branch(&mut self, label: &str) {
        self.nasm_gen.add_comment(&format!("branch to {}", label));
        self.nasm_gen.add_instruction(AsmInstruction::Jmp(label.to_string()));
    }

    /// Generate assembly for conditional branch
    fn generate_conditional_branch(&mut self, condition: &Value, true_label: &str, false_label: &str) {
        self.nasm_gen.add_comment(&format!("branch {} ? {} : {}", condition, true_label, false_label));
        let condition_operand = value_to_operand(condition);
        
        // Compare condition with 0 (false)
        self.nasm_gen.add_instruction(AsmInstruction::Cmp(condition_operand, Operand::imm(0)));
        
        // Jump to true label if not equal to 0 (true)
        self.nasm_gen.add_instruction(AsmInstruction::Jne(true_label.to_string()));
        
        // Jump to false label
        self.nasm_gen.add_instruction(AsmInstruction::Jmp(false_label.to_string()));
    }

    /// Generate assembly for indirect branch terminator
    fn generate_indirect_branch(&mut self, address: &Value, possible_labels: &[String]) {
        let addr_operand = value_to_operand(address);
        self.nasm_gen.add_comment(&format!("indirect branch to {} with possible labels: {:?}", address, possible_labels));
        self.nasm_gen.add_comment(&format!("  address: {}", addr_operand));
    }

    /// Generate assembly for switch terminator
    fn generate_switch(&mut self, value: &Value, ty: &IrType, default_label: &str, cases: &[(Value, String)]) {
        let value_operand = value_to_operand(value);
        self.nasm_gen.add_comment(&format!("switch on {} ({}) with default to {} and {} cases", 
            value, ty, default_label, cases.len()));
        self.nasm_gen.add_comment(&format!("  value: {}", value_operand));
        for (case_value, label) in cases {
            self.nasm_gen.add_comment(&format!("  case {} -> {}", case_value, label));
        }
    }

    /// Generate assembly for unreachable terminator
    fn generate_unreachable(&mut self) {
        // In a real implementation, we might use an int3 instruction or similar
        self.nasm_gen.add_instruction(AsmInstruction::Hlt);
    }
}