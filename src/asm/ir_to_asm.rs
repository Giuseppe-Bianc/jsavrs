//! IR to ASM generator for x86-64 architecture using NASM syntax - FIXED VERSION
use super::generator::{NasmGenerator, Section, TargetOS};
use super::instruction::Instruction as AsmInstruction;
use super::operand::Operand;
use super::register::Register;
use crate::ir::basic_block::BasicBlock;
use crate::ir::function::Function;
use crate::ir::instruction::{CastKind, Instruction, InstructionKind, IrBinaryOp, IrUnaryOp, VectorOp};
use crate::ir::module::Module;
use crate::ir::terminator::{Terminator, TerminatorKind};
use crate::ir::types::IrType;
use crate::ir::value::{IrLiteralValue, Value, ValueKind};
use std::collections::HashMap;
use std::sync::Arc;

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
        _ => Register::RAX,                  // Default to 64-bit register
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
        _ => 64,                   // Default to 64 bits
    }
}

/// Maps IR types to stack slot size in bytes (aligned to 8 bytes)
fn map_type_to_stack_size(ty: &IrType) -> u32 {
    let size = match ty {
        IrType::I8 | IrType::U8 | IrType::Bool => 4, // Minimum 4 bytes for integers
        IrType::I16 | IrType::U16 => 4,
        IrType::I32 | IrType::U32 => 4,
        IrType::I64 | IrType::U64 | IrType::Pointer(_) => 8,
        IrType::F32 => 4,
        IrType::F64 => 8,
        IrType::Array(_, _) => 8, // Arrays are passed as pointers
        IrType::Char => 4,
        IrType::String => 8, // String is typically a pointer
        _ => 8,              // Default to 8 bytes
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

/// IR to ASM generator
pub struct IrToAsmGenerator {
    nasm_gen: NasmGenerator,
    /// Maps local variables/temporaries to their stack offsets
    local_vars: HashMap<Arc<str>, i32>,
    /// Current stack offset (negative values for local variables)
    stack_offset: i32,
    /// Current function name
    current_function: String,
    /// Function parameters (for System V ABI)
    function_params: Vec<Arc<str>>,
}

impl IrToAsmGenerator {
    /// Create a new IR to ASM generator
    pub fn new(target_os: TargetOS) -> Self {
        Self {
            nasm_gen: NasmGenerator::new(target_os),
            local_vars: HashMap::new(),
            stack_offset: 0,
            current_function: String::new(),
            function_params: Vec::new(),
        }
    }

    /// Converts an IR value to an ASM operand
    fn value_to_operand(&self, value: &Value) -> Operand {
        match &value.kind {
            ValueKind::Literal(literal) => match literal {
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
            },
            ValueKind::Local(name) => {
                // For local variables, we need to check if they're parameters or allocated variables
                if let Some(param_index) = self.function_params.iter().position(|p| **p == **name) {
                    // Return appropriate parameter register based on System V ABI
                    // Use 64-bit registers for pointers to ensure full 64-bit addresses
                    match param_index {
                        0 => Operand::reg(Register::RDI), // First parameter
                        1 => Operand::reg(Register::RSI), // Second parameter
                        2 => Operand::reg(Register::RDX), // Third parameter
                        3 => Operand::reg(Register::RCX), // Fourth parameter
                        4 => Operand::reg(Register::R8),  // Fifth parameter
                        5 => Operand::reg(Register::R9),  // Sixth parameter
                        _ => {
                            // Parameters beyond 6 are on the stack
                            let offset = 16 + (param_index - 6) * 8; // 16 for return addr + rbp
                            Operand::mem_ref(Some(Register::RBP), None, 1, offset as i32)
                        }
                    }
                } else if let Some(&offset) = self.local_vars.get(name.as_ref()) {
                    // It's a local variable on the stack - this is a pointer to the data
                    Operand::mem_ref(Some(Register::RBP), None, 1, offset)
                } else {
                    // Global variable or undefined - treat as label
                    Operand::label(name)
                }
            }
            ValueKind::Global(name) => Operand::label(name),
            ValueKind::Temporary(id) => {
                let temp_name = format!("t{}", id);
                // For temporaries, check if we have a specific offset
                // Temporaries that are results of alloca are pointers to stack locations
                if let Some(&offset) = self.local_vars.get(temp_name.as_str()) {
                    Operand::mem_ref(Some(Register::RBP), None, 1, offset)
                } else {
                    // Check if this temporary represents a variable with debug info
                    if let Some(debug_info) = &value.debug_info {
                        if let Some(name) = &debug_info.name {
                            // Try to look up by variable name
                            if let Some(&offset) = self.local_vars.get(name.as_ref()) {
                                return Operand::mem_ref(Some(Register::RBP), None, 1, offset);
                            }
                        }
                    }
                    // Temporary not allocated yet - this shouldn't happen in well-formed IR
                    // For now, we'll just use a placeholder
                    Operand::mem_ref(Some(Register::RBP), None, 1, -8) // Default offset
                }
            }
            ValueKind::Constant(_) => {
                // For constants, we might need to handle them differently
                // For now, we'll just use a placeholder
                Operand::imm(0)
            }
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
        self.function_params.clear();

        // Store function parameters for later reference
        for param in &function.parameters {
            self.function_params.push(Arc::clone(&param.name));
        }

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

        self.nasm_gen.add_empty_line();
    }

    /// Generate function prologue
    fn generate_function_prologue(&mut self, function: &Function) {
        self.nasm_gen.add_comment(&format!("Function: {} prologue", function.name));
        self.nasm_gen.add_instruction(AsmInstruction::Push(Operand::reg(Register::RBP)));
        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBP), Operand::reg(Register::RSP)));

        // Pre-scan for local variables and allocate stack space
        self.prescan_for_locals(function);

        // Reserve stack space if needed
        if self.stack_offset < 0 {
            let stack_space = (-self.stack_offset) as i64;
            // Align to 16 bytes
            let aligned_space = ((stack_space + 15) / 16) * 16;
            self.nasm_gen
                .add_instruction(AsmInstruction::Sub(Operand::reg(Register::RSP), Operand::imm(aligned_space)));
            self.nasm_gen.add_comment(&format!("Reserved {} bytes for local variables", aligned_space));
        }

        // Save parameters to their stack locations (if they need to be stored)
        self.save_parameters_to_stack(&function.parameters);
    }

    /// Pre-scan function to find all local variables and allocate stack space
    fn prescan_for_locals(&mut self, function: &Function) {
        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                if let InstructionKind::Alloca { ty } = &instruction.kind {
                    if let Some(result) = &instruction.result {
                        // For alloca, we need to allocate space for both:
                        // 1. The actual data (what the pointer points to)
                        // 2. The pointer itself (the result of the alloca)

                        // Allocate space for the data
                        let size = map_type_to_stack_size(ty) as i32;
                        self.stack_offset -= size;
                        let data_offset = self.stack_offset; // Store the data offset

                        // Allocate space for the pointer (8 bytes on x64)
                        self.stack_offset -= 8;
                        let pointer_offset = self.stack_offset;

                        let var_name = match &result.kind {
                            ValueKind::Temporary(id) => format!("t{}", id),
                            ValueKind::Local(name) => name.to_string(),
                            _ => format!("alloca_{}", result.id),
                        };

                        // Map the temporary to where we store the pointer
                        let var_name_clone = var_name.clone();
                        self.local_vars.insert(var_name_clone.into(), pointer_offset);

                        // Also store the data offset for later use in store/load operations
                        let data_var_name = format!("{}_data", var_name);
                        self.local_vars.insert(data_var_name.into(), data_offset);

                        // If this value has debug info with a variable name, also map the variable name
                        if let Some(debug_info) = &result.debug_info {
                            if let Some(name) = &debug_info.name {
                                // Map the variable name to the same pointer offset
                                self.local_vars.insert(name.clone(), pointer_offset);
                            }
                        }
                    }
                }

                // Also allocate space for other temporaries that might be needed
                if let Some(result) = &instruction.result {
                    if let ValueKind::Temporary(id) = &result.kind {
                        let temp_name = format!("t{}", id);
                        if !self.local_vars.contains_key(temp_name.as_str()) {
                            self.stack_offset -= 8; // Default 8 bytes for temporaries
                            self.local_vars.insert(temp_name.into(), self.stack_offset);
                        }
                    }
                }
            }
        }
    }

    /// Save function parameters to stack if needed
    fn save_parameters_to_stack(&mut self, parameters: &[crate::ir::function::IrParameter]) {
        // For System V ABI, first 6 integer parameters are in registers
        // We might need to save them if they're used as local variables
        for (i, param) in parameters.iter().enumerate() {
            // Check if this parameter is used as a local variable
            if self.local_vars.contains_key(param.name.as_ref()) {
                // Save the parameter register to its stack location
                if i < 6 {
                    let src_reg = match i {
                        0 => Register::RDI, // Use 64-bit register
                        1 => Register::RSI, // Use 64-bit register
                        2 => Register::RDX, // Use 64-bit register
                        3 => Register::RCX, // Use 64-bit register
                        4 => Register::R8,  // Use 64-bit register
                        5 => Register::R9,  // Use 64-bit register
                        _ => unreachable!(),
                    };

                    let offset = self.local_vars[param.name.as_ref()];
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(
                        Operand::mem_ref(Some(Register::RBP), None, 1, offset),
                        Operand::reg(src_reg),
                    ));
                    self.nasm_gen.add_comment(&format!("Save parameter {} to stack", param.name));
                }
            }
        }
    }

    /// Generate function epilogue
    fn generate_function_epilogue(&mut self) {
        self.nasm_gen.add_comment("Function epilogue");
        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RSP), Operand::reg(Register::RBP)));
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
            // The space was already allocated in prescan_for_locals
            // We just need to compute the address and store it in the result
            let var_name = match &result.kind {
                ValueKind::Temporary(id) => format!("t{}", id),
                ValueKind::Local(name) => name.to_string(),
                _ => format!("alloca_{}", result.id),
            };

            let var_name_clone = var_name.clone();
            if let Some(&pointer_offset) = self.local_vars.get(var_name_clone.as_str()) {
                // Get the data offset that was stored during prescan
                let data_var_name = format!("{}_data", var_name);
                let data_offset = *self.local_vars.get(data_var_name.as_str()).unwrap_or(&0);

                // Calculate the address of the allocated data space
                // LEA is perfect for this: lea rax, [rbp + data_offset]
                self.nasm_gen.add_instruction(AsmInstruction::Lea(
                    Operand::reg(Register::RAX),
                    Operand::mem_ref(Some(Register::RBP), None, 1, data_offset),
                ));

                // Store the address in the pointer location
                self.nasm_gen.add_instruction(AsmInstruction::Mov(
                    Operand::mem_ref(Some(Register::RBP), None, 1, pointer_offset),
                    Operand::reg(Register::RAX),
                ));

                self.nasm_gen.add_comment(&format!(
                    "alloca {} at RBP{} -> {} (pointer at RBP{})",
                    ty, data_offset, result, pointer_offset
                ));
            }
        }
    }

    /// Generate assembly for store instruction
    // Versione corretta di generate_store
    fn generate_store(&mut self, value: &Value, dest: &Value) {
        let value_operand = self.value_to_operand(value);
        let dest_operand = self.value_to_operand(dest);

        match &dest.kind {
            ValueKind::Temporary(_) | ValueKind::Local(_) => {
                // Carica il puntatore di destinazione in RAX
                self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), dest_operand));

                // Gestisci il valore in modo sicuro
                match &value.kind {
                    ValueKind::Temporary(_) | ValueKind::Local(_) => {
                        // Se il valore è anche un riferimento a memoria, caricalo in un registro prima
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), value_operand));
                        // Poi salva il registro in memoria
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::mem_ref(Some(Register::RAX), None, 1, 0),
                            Operand::reg(Register::RBX),
                        ));
                    }
                    _ => {
                        // Valore immediato o globale, salva direttamente
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::mem_ref(Some(Register::RAX), None, 1, 0),
                            value_operand,
                        ));
                    }
                }
            }
            _ => {
                // Store diretto - ma evita movimenti memoria-a-memoria
                match &value.kind {
                    ValueKind::Temporary(_) | ValueKind::Local(_) => {
                        // Carica il valore in un registro prima
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), value_operand));
                        // Poi salva il registro nella destinazione
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, Operand::reg(Register::RAX)));
                    }
                    _ => {
                        // Valore immediato o globale, salva direttamente
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(dest_operand, value_operand));
                    }
                }
            }
        }
    }

    /// Generate assembly for load instruction
    fn generate_load(&mut self, src: &Value, _ty: &IrType, instruction: &Instruction) {
        if let Some(result) = &instruction.result {
            // For load, src is usually a pointer (from alloca or parameter)
            // We need to dereference it to get the value at the memory location it points to
            match &src.kind {
                ValueKind::Temporary(id) => {
                    let temp_name = format!("t{}", id);
                    if let Some(&pointer_offset) = self.local_vars.get(temp_name.as_str()) {
                        // This temporary contains a pointer, load the pointer value first
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::reg(Register::RAX),
                            Operand::mem_ref(Some(Register::RBP), None, 1, pointer_offset),
                        ));
                        // Then load the value at the address pointed to by RAX
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::reg(Register::EAX),
                            Operand::mem_ref(Some(Register::RAX), None, 1, 0),
                        ));

                        // Store result
                        let result_operand = self.value_to_operand(result);
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::EAX)));
                    } else {
                        // Direct load from memory - use the default offset approach from value_to_operand
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::reg(Register::EAX),
                            Operand::mem_ref(Some(Register::RBP), None, 1, -8),
                        ));

                        // Store result
                        let result_operand = self.value_to_operand(result);
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::EAX)));
                    }
                }
                ValueKind::Local(name) => {
                    // Check if this is a parameter or an allocated variable
                    if self.function_params.iter().any(|p| **p == **name) {
                        // This is a parameter, load directly from its register
                        let src_operand = self.value_to_operand(src);
                        let result_operand = self.value_to_operand(result);
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, src_operand));
                    } else if let Some(&pointer_offset) = self.local_vars.get(name.as_ref()) {
                        // This local contains a pointer, load the pointer value first
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::reg(Register::RAX),
                            Operand::mem_ref(Some(Register::RBP), None, 1, pointer_offset),
                        ));
                        // Then load the value at the address pointed to by RAX
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(
                            Operand::reg(Register::EAX),
                            Operand::mem_ref(Some(Register::RAX), None, 1, 0),
                        ));

                        // Store result
                        let result_operand = self.value_to_operand(result);
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::EAX)));
                    } else {
                        // Direct load from memory
                        let src_operand = self.value_to_operand(src);
                        let result_operand = self.value_to_operand(result);
                        self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, src_operand));
                    }
                }
                _ => {
                    let src_operand = self.value_to_operand(src);
                    let result_operand = self.value_to_operand(result);
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, src_operand));
                }
            }

            self.nasm_gen.add_comment(&format!("load from {} -> {}", src, result));
        }
    }

    /// Generate assembly for binary operation
    fn generate_binary(
        &mut self, op: &IrBinaryOp, left: &Value, right: &Value, _ty: &IrType, instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let left_operand = self.value_to_operand(left);
            let right_operand = self.value_to_operand(right);

            // For binary operations, we need to load the actual values, not pointers
            // Check if the operands are pointers that need to be dereferenced
            match &left.kind {
                ValueKind::Temporary(_) | ValueKind::Local(_) => {
                    // Left operand is a pointer, load the pointer value first
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), left_operand));
                    // Then load the value at the address pointed to by RAX
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(
                        Operand::reg(Register::EAX),
                        Operand::mem_ref(Some(Register::RAX), None, 1, 0),
                    ));
                }
                _ => {
                    // Direct load
                    self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::EAX), left_operand));
                }
            }

            // Apply the binary operation
            match op {
                IrBinaryOp::Add => {
                    match &right.kind {
                        ValueKind::Temporary(_) | ValueKind::Local(_) => {
                            // Right operand is a pointer, load the pointer value first
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), right_operand));
                            // Then load the value at the address pointed to by RBX
                            self.nasm_gen.add_instruction(AsmInstruction::Mov(
                                Operand::reg(Register::EBX),
                                Operand::mem_ref(Some(Register::RBX), None, 1, 0),
                            ));
                            self.nasm_gen.add_instruction(AsmInstruction::Add(
                                Operand::reg(Register::EAX),
                                Operand::reg(Register::EBX),
                            ));
                        }
                        _ => {
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Add(Operand::reg(Register::EAX), right_operand));
                        }
                    }
                }
                IrBinaryOp::Subtract => {
                    match &right.kind {
                        ValueKind::Temporary(_) | ValueKind::Local(_) => {
                            // Right operand is a pointer, load the pointer value first
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), right_operand));
                            // Then load the value at the address pointed to by RBX
                            self.nasm_gen.add_instruction(AsmInstruction::Mov(
                                Operand::reg(Register::EBX),
                                Operand::mem_ref(Some(Register::RBX), None, 1, 0),
                            ));
                            self.nasm_gen.add_instruction(AsmInstruction::Sub(
                                Operand::reg(Register::EAX),
                                Operand::reg(Register::EBX),
                            ));
                        }
                        _ => {
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Sub(Operand::reg(Register::EAX), right_operand));
                        }
                    }
                }
                IrBinaryOp::Multiply => {
                    match &right.kind {
                        ValueKind::Temporary(_) | ValueKind::Local(_) => {
                            // Right operand is a pointer, load the pointer value first
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), right_operand));
                            // Then load the value at the address pointed to by RBX
                            self.nasm_gen.add_instruction(AsmInstruction::Mov(
                                Operand::reg(Register::EBX),
                                Operand::mem_ref(Some(Register::RBX), None, 1, 0),
                            ));
                            self.nasm_gen.add_instruction(AsmInstruction::Imul(
                                Operand::reg(Register::EAX),
                                Some(Operand::reg(Register::EBX)),
                                None,
                            ));
                        }
                        _ => {
                            self.nasm_gen.add_instruction(AsmInstruction::Imul(
                                Operand::reg(Register::EAX),
                                Some(right_operand),
                                None,
                            ));
                        }
                    }
                }
                IrBinaryOp::Divide => {
                    // For division, we need to set up registers appropriately
                    match &right.kind {
                        ValueKind::Temporary(_) | ValueKind::Local(_) => {
                            // Right operand is a pointer, load the pointer value first
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), right_operand));
                            // Then load the value at the address pointed to by RBX
                            self.nasm_gen.add_instruction(AsmInstruction::Mov(
                                Operand::reg(Register::EBX),
                                Operand::mem_ref(Some(Register::RBX), None, 1, 0),
                            ));
                        }
                        _ => {
                            self.nasm_gen
                                .add_instruction(AsmInstruction::Mov(Operand::reg(Register::EBX), right_operand));
                        }
                    }
                    self.nasm_gen.add_instruction(AsmInstruction::Cdq); // Sign extend EAX to EDX:EAX
                    self.nasm_gen.add_instruction(AsmInstruction::Idiv(Operand::reg(Register::EBX)));
                    // Result is in EAX
                }
                // ... other operations remain the same
                _ => {
                    self.nasm_gen.add_comment(&format!("Binary operation {} not fully implemented", op));
                }
            }

            // Store result
            let result_operand = self.value_to_operand(result);
            self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::EAX)));

            self.nasm_gen.add_comment(&format!("binary {} {} {} -> {}", op, left, right, result));
        }
    }

    /// Generate assembly for unary operation
    fn generate_unary(&mut self, op: &IrUnaryOp, operand: &Value, _ty: &IrType, instruction: &Instruction) {
        if let Some(result) = &instruction.result {
            let operand_val = self.value_to_operand(operand);

            // Load operand into EAX
            self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::EAX), operand_val));

            // Apply the unary operation
            match op {
                IrUnaryOp::Negate => {
                    self.nasm_gen.add_instruction(AsmInstruction::Neg(Operand::reg(Register::EAX)));
                }
                IrUnaryOp::Not => {
                    self.nasm_gen.add_instruction(AsmInstruction::Not(Operand::reg(Register::EAX)));
                }
            }

            // Store result
            let result_operand = self.value_to_operand(result);
            self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::EAX)));

            self.nasm_gen.add_comment(&format!("unary {} {} -> {}", op, operand, result));
        }
    }

    /// Generate assembly for function call
    fn generate_call(&mut self, func: &Value, args: &[Value], _ty: &IrType, instruction: &Instruction) {
        // System V ABI: First 6 integer/pointer arguments are passed in registers
        // Use 64-bit registers to ensure full 64-bit pointers
        let arg_registers = [Register::RDI, Register::RSI, Register::RDX, Register::RCX, Register::R8, Register::R9];

        // Load arguments into registers
        self.nasm_gen.add_comment("Load arguments into registers");

        // Load arguments into registers
        for (i, arg) in args.iter().enumerate() {
            if i < arg_registers.len() {
                let arg_operand = self.value_to_operand(arg);
                self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(arg_registers[i].clone()), arg_operand));
            } else {
                // Push additional arguments onto stack
                let arg_operand = self.value_to_operand(arg);
                self.nasm_gen.add_instruction(AsmInstruction::Push(arg_operand));
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

        // Clean up stack if we pushed arguments
        if args.len() > 6 {
            let stack_cleanup = (args.len() - 6) * 8;
            self.nasm_gen
                .add_instruction(AsmInstruction::Add(Operand::reg(Register::RSP), Operand::imm(stack_cleanup as i64)));
        }

        // Handle result if needed
        if let Some(result) = &instruction.result {
            // Result is in EAX, store it in the correct location for this temporary
            let temp_name = match &result.kind {
                ValueKind::Temporary(id) => format!("t{}", id),
                _ => format!("result_{}", result.id),
            };

            // Check if this result needs to be stored in a specific location
            if let Some(&offset) = self.local_vars.get(temp_name.as_str()) {
                // Store result in the allocated stack location
                self.nasm_gen.add_instruction(AsmInstruction::Mov(
                    Operand::mem_ref(Some(Register::RBP), None, 1, offset),
                    Operand::reg(Register::EAX),
                ));
            } else {
                // For temporaries that don't have a specific stack location,
                // we still need to handle them correctly in subsequent uses
                self.nasm_gen.add_comment(&format!("Result of call stored in EAX for temporary {}", temp_name));
            }
        }

        self.nasm_gen.add_comment(&format!("call {} with {} args -> {:?}", func_name, args.len(), instruction.result));
    }

    /// Generate assembly for getelementptr instruction
    fn generate_gep(&mut self, base: &Value, index: &Value, element_ty: &IrType, instruction: &Instruction) {
        if let Some(result) = &instruction.result {
            let base_operand = self.value_to_operand(base);
            let index_operand = self.value_to_operand(index);

            // Calculate element size in bytes
            let element_size = map_type_to_stack_size(element_ty);

            self.nasm_gen.add_comment(&format!("gep {} + {} * {} -> {}", base, index, element_size, result));

            // Load base address into RAX
            self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), base_operand));

            // Load index into RBX
            self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RBX), index_operand));

            // Multiply index by element size
            if element_size > 1 {
                self.nasm_gen.add_instruction(AsmInstruction::Imul(
                    Operand::reg(Register::RBX),
                    Some(Operand::imm(element_size as i64)),
                    None,
                ));
            }

            // Add offset to base
            self.nasm_gen
                .add_instruction(AsmInstruction::Add(Operand::reg(Register::RAX), Operand::reg(Register::RBX)));

            // Store result
            let result_operand = self.value_to_operand(result);
            self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::RAX)));
        }
    }

    /// Generate assembly for cast instruction
    fn generate_cast(
        &mut self, kind: &CastKind, value: &Value, from_ty: &IrType, to_ty: &IrType, instruction: &Instruction,
    ) {
        if let Some(result) = &instruction.result {
            let value_operand = self.value_to_operand(value);
            let from_size = map_type_to_size(from_ty);
            let to_size = map_type_to_size(to_ty);
            let kind_str = cast_kind_to_string(kind);

            self.nasm_gen.add_comment(&format!(
                "cast {} {} from {} ({} bits) to {} ({} bits) -> {}",
                kind_str, value, from_ty, from_size, to_ty, to_size, result
            ));

            // Load value into appropriate register
            self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::EAX), value_operand));

            // Generate actual assembly code based on the cast kind
            match kind {
                CastKind::IntToPtr | CastKind::PtrToInt | CastKind::Bitcast => {
                    // Simple bit reinterpretation - no conversion needed
                    self.nasm_gen.add_comment("  bitwise copy");
                }
                CastKind::IntSignExtend => {
                    if from_size < to_size {
                        self.nasm_gen.add_comment("  sign extend");
                        // Use movsx for sign extension
                        match (from_size, to_size) {
                            (8, 32) => self.nasm_gen.add_instruction(AsmInstruction::Movsx(
                                Operand::reg(Register::EAX),
                                Operand::reg(Register::AL),
                            )),
                            (16, 32) => self.nasm_gen.add_instruction(AsmInstruction::Movsx(
                                Operand::reg(Register::EAX),
                                Operand::reg(Register::AX),
                            )),
                            _ => self.nasm_gen.add_comment("  sign extension case not handled"),
                        }
                    }
                }
                CastKind::IntZeroExtend => {
                    if from_size < to_size {
                        self.nasm_gen.add_comment("  zero extend");
                        // Use movzx for zero extension
                        match (from_size, to_size) {
                            (8, 32) => self.nasm_gen.add_instruction(AsmInstruction::Movzx(
                                Operand::reg(Register::EAX),
                                Operand::reg(Register::AL),
                            )),
                            (16, 32) => self.nasm_gen.add_instruction(AsmInstruction::Movzx(
                                Operand::reg(Register::EAX),
                                Operand::reg(Register::AX),
                            )),
                            _ => self.nasm_gen.add_comment("  zero extension case not handled"),
                        }
                    }
                }
                CastKind::IntTruncate => {
                    self.nasm_gen.add_comment("  truncate by using smaller register");
                    // Truncation happens automatically when using smaller registers
                }
                _ => {
                    self.nasm_gen.add_comment(&format!("  cast kind {} not fully implemented", kind_str));
                }
            }

            // Store result
            let result_operand = self.value_to_operand(result);
            self.nasm_gen.add_instruction(AsmInstruction::Mov(result_operand, Operand::reg(Register::EAX)));
        }
    }

    /// Generate assembly for phi instruction
    fn generate_phi(&mut self, _ty: &IrType, incoming: &[(Value, String)], instruction: &Instruction) {
        if let Some(result) = &instruction.result {
            self.nasm_gen.add_comment(&format!("phi -> {}", result));

            // Phi nodes should be resolved during SSA destruction
            // For now, we generate a placeholder comment
            for (value, label) in incoming {
                self.nasm_gen.add_comment(&format!("  [{} <- {}]", value, label));
            }

            self.nasm_gen.add_comment("  ; PHI node resolved during SSA destruction");
        }
    }

    /// Generate assembly for vector instruction
    fn generate_vector(&mut self, op: &VectorOp, operands: &[Value], _ty: &IrType, instruction: &Instruction) {
        if let Some(result) = &instruction.result {
            self.nasm_gen.add_comment(&format!("vector.{} -> {}", op, result));

            // Vector operations would use SIMD instructions
            match op {
                VectorOp::Add => self.nasm_gen.add_comment("  ; vaddps/vaddpd for SIMD addition"),
                VectorOp::Sub => self.nasm_gen.add_comment("  ; vsubps/vsubpd for SIMD subtraction"),
                VectorOp::Mul => self.nasm_gen.add_comment("  ; vmulps/vmulpd for SIMD multiplication"),
                VectorOp::Div => self.nasm_gen.add_comment("  ; vdivps/vdivpd for SIMD division"),
                VectorOp::DotProduct => self.nasm_gen.add_comment("  ; vdpps for dot product"),
                VectorOp::Shuffle => self.nasm_gen.add_comment("  ; vpshufd/vpshufb for shuffle"),
            }

            // Show operands
            for (i, operand) in operands.iter().enumerate() {
                let operand_val = self.value_to_operand(operand);
                self.nasm_gen.add_comment(&format!("    operand {}: {}", i, operand_val));
            }

            self.nasm_gen.add_comment("  ; Vector operation implementation would generate SIMD instructions");
        }
    }

    /// Generate assembly for terminator
    fn generate_terminator(&mut self, terminator: &Terminator) {
        match &terminator.kind {
            TerminatorKind::Return { value, ty: _ } => {
                self.generate_return(value);
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
    fn generate_return(&mut self, value: &Value) {
        let value_operand = self.value_to_operand(value);

        // Move return value to EAX (System V ABI)
        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::EAX), value_operand));

        self.nasm_gen.add_comment(&format!("return {}", value));

        // Generate function epilogue and return
        self.generate_function_epilogue();
    }

    /// Generate assembly for unconditional branch
    fn generate_branch(&mut self, label: &str) {
        self.nasm_gen.add_comment(&format!("branch to {}", label));
        self.nasm_gen.add_instruction(AsmInstruction::Jmp(label.to_string()));
    }

    /// Generate assembly for conditional branch
    fn generate_conditional_branch(&mut self, condition: &Value, true_label: &str, false_label: &str) {
        let condition_operand = self.value_to_operand(condition);

        self.nasm_gen.add_comment(&format!("conditional branch: {} ? {} : {}", condition, true_label, false_label));

        // Compare condition with 0 (false)
        self.nasm_gen.add_instruction(AsmInstruction::Cmp(condition_operand, Operand::imm(0)));

        // Jump to true label if not equal to 0 (true)
        self.nasm_gen.add_instruction(AsmInstruction::Jne(true_label.to_string()));

        // Jump to false label
        self.nasm_gen.add_instruction(AsmInstruction::Jmp(false_label.to_string()));
    }

    /// Generate assembly for indirect branch terminator
    fn generate_indirect_branch(&mut self, address: &Value, possible_labels: &[String]) {
        let addr_operand = self.value_to_operand(address);

        self.nasm_gen
            .add_comment(&format!("indirect branch to {} with possible labels: {:?}", address, possible_labels));

        // Load address and jump
        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::RAX), addr_operand));
        self.nasm_gen.add_instruction(AsmInstruction::Jmp("rax".to_string()));
    }

    /// Generate assembly for switch terminator
    fn generate_switch(&mut self, value: &Value, _ty: &IrType, default_label: &str, cases: &[(Value, String)]) {
        let value_operand = self.value_to_operand(value);

        self.nasm_gen.add_comment(&format!(
            "switch on {} with default to {} and {} cases",
            value,
            default_label,
            cases.len()
        ));

        // Load switch value into register
        self.nasm_gen.add_instruction(AsmInstruction::Mov(Operand::reg(Register::EAX), value_operand));

        // Generate comparisons for each case
        for (case_value, label) in cases {
            let case_operand = self.value_to_operand(case_value);
            self.nasm_gen.add_instruction(AsmInstruction::Cmp(Operand::reg(Register::EAX), case_operand));
            self.nasm_gen.add_instruction(AsmInstruction::Je(label.clone()));
        }

        // Default case
        self.nasm_gen.add_instruction(AsmInstruction::Jmp(default_label.to_string()));
    }

    /// Generate assembly for unreachable terminator
    fn generate_unreachable(&mut self) {
        self.nasm_gen.add_comment("unreachable code");
        self.nasm_gen.add_instruction(AsmInstruction::Hlt);
    }

    /// Generate the complete assembly code
    pub fn generate(&self) -> String {
        self.nasm_gen.generate()
    }

    /// Save the generated code to a file
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        self.nasm_gen.save_to_file(filename)
    }
}
