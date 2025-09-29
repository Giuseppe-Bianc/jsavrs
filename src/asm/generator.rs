//! # Assembly Generator
//!
//! The main component responsible for translating IR to x86-64 assembly.

use crate::ir::{BasicBlock, Function, Module, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::asm::abi::{CallingConvention, system_v::SystemVABI, windows_x64::WindowsX64ABI};
use crate::asm::error::CodeGenError;
use crate::asm::instruction::X86Instruction;
use crate::asm::operand::{Operand, ImmediateValue, MemoryOperand};
use crate::asm::options::CodeGenOptions;
use crate::asm::platform::{TargetOS, TargetPlatform};
use crate::asm::register::{Register, GPRegister, XMMRegister, RegisterAllocator};
use crate::ir::instruction::Instruction as IrInstruction;
use crate::location::source_span::SourceSpan;

/// Function context for tracking local generation state
#[derive(Clone)]
pub struct FunctionContext {
    /// Function name in generated assembly
    pub name: String,
    /// Local variable allocation map
    pub locals: HashMap<String, LocalVariable>,
    /// Parameter allocation map (parameter name -> location)
    pub parameters: HashMap<Arc<str>, LocalLocation>,
    /// Current basic block being generated
    pub current_block: Option<String>,
    /// Label mapping for jump targets
    pub label_map: HashMap<String, String>,
    /// Stack frame size in bytes
    pub stack_frame_size: u32,
    /// Maximum number of function parameters
    pub max_params: usize,
    /// Current instruction offset for tracking
    pub instruction_offset: usize,
}

/// Local variable information
#[derive(Clone)]
pub struct LocalVariable {
    /// Name of the variable
    pub name: String,
    /// Location of the variable (register or stack)
    pub location: LocalLocation,
    /// The type of the variable
    pub var_type: crate::ir::types::IrType,
}

/// Location of a local variable
#[derive(Debug, Clone)]
pub enum LocalLocation {
    /// Variable is stored in a register
    Register(Register),
    /// Variable is stored on the stack with specific offset
    Stack { offset: i32 },
}

/// IR to assembly translator
pub struct IRTranslator {
    /// Register allocator for the current translation
    pub register_allocator: RegisterAllocator,
    /// Map of IR values to registers
    pub value_map: HashMap<String, Register>,
    /// Map of local variables
    pub local_map: HashMap<String, LocalVariable>,
    /// Map of functions to their contexts
    pub function_contexts: HashMap<String, FunctionContext>,
    /// Set of values that have been allocated registers but not yet used
    /// This helps track which registers need to be freed
    pub allocated_values: HashSet<String>,
}

impl IRTranslator {
    pub fn new() -> Self {
        IRTranslator {
            register_allocator: RegisterAllocator::new(),
            value_map: HashMap::new(),
            local_map: HashMap::new(),
            function_contexts: HashMap::new(),
            allocated_values: HashSet::new(),
        }
    }

    /// Translates an IR instruction to x86 instructions
    pub fn translate_instruction(
        &mut self, ir_instruction: &IrInstruction,
    ) -> Result<Vec<X86Instruction>, CodeGenError> {
        use crate::ir::instruction::InstructionKind;

        let mut instructions = Vec::new();

        match &ir_instruction.kind {
            InstructionKind::Alloca { ty } => {
                // For alloca, we need to track the local variable allocation on the stack
                if let Some(result_value) = &ir_instruction.result {
                    // Calculate stack offset based on current frame size - adjust for 16-byte alignment
                    let current_offset = (self.register_allocator.spill_location as i32 + 1) * 8; // 8-byte slots
                    self.register_allocator.spill_location += 1;
                    
                    // Use debug name if available, otherwise use full value representation
                    let var_name = if let Some(debug_info) = &result_value.debug_info
                        && let Some(name) = &debug_info.name {
                        format!("{}", name)
                    } else {
                        format!("{}", result_value)
                    };
                    
                    let local_var = LocalVariable {
                        name: var_name.clone(),
                        location: LocalLocation::Stack { offset: -current_offset }, // Negative offset from RBP
                        var_type: ty.clone(),
                    };
                    
                    self.local_map.insert(var_name.clone(), local_var);
                    
                    // Create a memory operand pointing to the allocated stack location
                    let mem_op = MemoryOperand::new(Register::GP(GPRegister::RBP), -current_offset);
                    // Map the result value to this memory location
                    self.value_map.insert(
                        var_name.clone(), 
                        Register::GP(GPRegister::RBP) // Use RBP as a placeholder for memory location
                    );
                    
                    // For alloca, we're creating a pointer to the stack location
                    // In x86-64, we would typically load the effective address
                    instructions.push(X86Instruction::Lea {
                        dest: Operand::Register(Register::GP(GPRegister::RAX)), // Use a temporary register
                        src: Operand::Memory(MemoryOperand::new(Register::GP(GPRegister::RBP), -current_offset))
                    });
                    
                    // Now map the result value to the register that holds the address
                    if let Some(debug_info) = &result_value.debug_info
                        && let Some(name) = &debug_info.name {
                        self.value_map.insert(format!("{}", name), Register::GP(GPRegister::RAX));
                    } else {
                        self.value_map.insert(format!("{}", result_value), Register::GP(GPRegister::RAX));
                    }
                }
            }
            InstructionKind::Store { value, dest } => {
                // Translate store: Store value to the memory location specified by dest
                // Use the type-hinted register allocation to get the right register type
                let src_reg = self.get_or_allocate_register_for_value_with_type_hint(value)?;
                
                // Then, get the destination memory location (where to store it)
                // The dest should be a pointer value (address)
                let dest_location = self.get_memory_location_for_pointer_value(dest)?;
                
                // Generate MOV instruction to store the value
                match dest_location {
                    LocalLocation::Stack { offset } => {
                        instructions.push(X86Instruction::Mov {
                            dest: Operand::Memory(MemoryOperand::new(
                                Register::GP(GPRegister::RBP),
                                offset,
                            )),
                            src: Operand::Register(src_reg),
                        });
                    },
                    LocalLocation::Register(base_reg) => {
                        // If it's in a register, we need to treat it as a memory address
                        // For simplicity, we'll assume it's a stack address stored in a register
                        // This is a simplification - a full implementation would handle indirect addressing properly
                        instructions.push(X86Instruction::Other {
                            mnemonic: "mov".to_string(),
                            operands: vec![
                                Operand::Memory(MemoryOperand::with_index(
                                    base_reg, 
                                    Register::GP(GPRegister::RAX), // Temporary placeholder 
                                    1, 
                                    0
                                )), // [base_reg + RAX*1 + 0]
                                Operand::Register(src_reg)
                            ]
                        });
                    }
                }
            }
            InstructionKind::Load { src, ty: _ } => {
                // Translate load: Load value from memory location specified by src to a register
                // First, get the source memory location
                let src_location = self.get_memory_location_for_pointer_value(src)?;
                
                // Allocate a register for the destination
                let dest_reg = self.register_allocator.allocate_register(&format!("load_{}", src))
                    .ok_or_else(|| CodeGenError::RegisterAllocationFailure {
                        reason: format!("Could not allocate register for load operation of {}", src),
                        function: "unknown".to_string(),
                        instruction_id: None,
                    })?;
                
                // Generate MOV instruction to load the value
                match src_location {
                    LocalLocation::Stack { offset } => {
                        instructions.push(X86Instruction::Mov {
                            dest: Operand::Register(dest_reg),
                            src: Operand::Memory(MemoryOperand::new(
                                Register::GP(GPRegister::RBP),
                                offset,
                            )),
                        });
                    },
                    LocalLocation::Register(src_reg) => {
                        // If source is already in a register, just copy it
                        instructions.push(X86Instruction::Mov {
                            dest: Operand::Register(dest_reg),
                            src: Operand::Register(src_reg),
                        });
                    }
                }
                
                // Map the result value to the destination register if result exists
                if let Some(result_value) = &ir_instruction.result {
                    // Use debug name if available, otherwise use full value representation
                    let result_key = if let Some(debug_info) = &result_value.debug_info
                        && let Some(name) = &debug_info.name {
                        format!("{}", name)
                    } else {
                        format!("{}", result_value)
                    };
                    self.value_map.insert(result_key, dest_reg);
                }
            }
            InstructionKind::Binary { op, left, right, ty } => {
                // Translate binary operations to corresponding x86 instructions
                instructions.extend(self.translate_binary_operation(*op, left, right, ty)?);
            }
            InstructionKind::Unary { op, operand, ty } => {
                // Translate unary operations to corresponding x86 instructions
                instructions.extend(self.translate_unary_operation(*op, operand, ty)?);
            }
            InstructionKind::Call { func, args, ty: _ } => {
                // Translate function call - handle arguments according to ABI
                // First, save caller-saved registers if needed
                // Then place arguments in appropriate registers or on stack according to ABI
                
                // For now, we'll determine registers based on the default calling convention
                // System V ABI (Linux/macOS): RDI, RSI, RDX, RCX, R8, R9 for first 6 integer args
                // Windows x64 ABI: RCX, RDX, R8, R9 for first 4 integer args
                use crate::asm::register::GPRegister::*;
                let sys_v_int_params = [RDI, RSI, RDX, RCX, R8, R9];
                let windows_int_params = [RCX, RDX, R8, R9];
                
                // For simplicity, using System V ABI by default
                // In a full implementation, we'd access the actual calling convention
                let int_params = &sys_v_int_params[..];
                
                let mut stack_args = Vec::new(); // For arguments that go on the stack
                
                // Process arguments according to calling convention
                for (i, arg) in args.iter().enumerate() {
                    let arg_reg = self.get_or_allocate_register_for_value(arg)?;
                    
                    if i < int_params.len() {
                        // According to System V ABI, arguments go in specific registers
                        let param_reg = Register::GP(int_params[i]);
                        
                        // Move argument to the appropriate parameter register
                        instructions.push(X86Instruction::Mov {
                            dest: Operand::Register(param_reg),
                            src: Operand::Register(arg_reg),
                        });
                    } else {
                        // For additional parameters beyond register capacity, they go on the stack
                        // Push them to the stack in reverse order (since stack grows down)
                        stack_args.push(arg_reg);
                    }
                }
                
                // Push stack arguments (in reverse order for System V ABI)
                for arg_reg in stack_args.iter().rev() {
                    instructions.push(X86Instruction::Push {
                        operand: Operand::Register(*arg_reg)
                    });
                }
                
                // Make the function call
                instructions.push(X86Instruction::Call { 
                    target: Operand::Label(func.to_string()) 
                });

                // Clean up stack arguments if any were pushed
                if !stack_args.is_empty() {
                    let cleanup_size = (stack_args.len() * 8) as i64; // 8 bytes per argument
                    instructions.push(X86Instruction::Add {
                        dest: Operand::Register(Register::GP(GPRegister::RSP)),
                        src: Operand::Immediate(ImmediateValue::Int64(cleanup_size)),
                    });
                }

                // If the call returns a value, map it to a register (typically RAX)
                if let Some(result_value) = &ir_instruction.result {
                    // Use RAX as the return register according to calling convention
                    let return_reg = Register::GP(GPRegister::RAX);
                    
                    let result_key = if let Some(debug_info) = &result_value.debug_info
                        && let Some(name) = &debug_info.name {
                        format!("{}", name)
                    } else {
                        format!("{}", result_value)
                    };
                    
                    self.value_map.insert(result_key, return_reg);
                }
            }
            InstructionKind::GetElementPtr { base: _, index: _, element_ty: _ } => {
                // Translate getelementptr (pointer arithmetic)
                // For now, add a placeholder for this complex operation
                instructions.push(X86Instruction::Nop);
            }
            InstructionKind::Cast { kind: _, value: _, from_ty: _, to_ty: _ } => {
                // Translate type casting operations
                // For now, just pass through (no actual casting implemented)
                instructions.push(X86Instruction::Nop);
            }
            InstructionKind::Phi { ty: _, incoming: _ } => {
                // Translate PHI nodes (for SSA form)
                // PHI nodes require special handling in basic block boundaries
                instructions.push(X86Instruction::Nop);
            }
            InstructionKind::Vector { op: _, operands: _, ty: _ } => {
                // Translate vector operations
                // Vector operations require SIMD instructions (SSE/AVX) - placeholder for now
                instructions.push(X86Instruction::Nop);
            }
        }

        Ok(instructions)
    }

    /// Translate a binary operation to x86 instructions
    fn translate_binary_operation(
        &mut self, op: crate::ir::instruction::IrBinaryOp, left: &crate::ir::value::Value,
        right: &crate::ir::value::Value, ty: &crate::ir::types::IrType,
    ) -> Result<Vec<X86Instruction>, CodeGenError> {
        let mut instructions = Vec::new();

        // Get or allocate registers for the operands based on their types
        let left_reg = self.get_or_allocate_register_for_value_typed(left, ty)?;
        let right_reg = self.get_or_allocate_register_for_value_typed(right, ty)?;

        // For binary operations, we'll use a new register to store the result
        // Allocate a register for the result based on the operation type
        let result_reg = self.get_or_allocate_register_for_value_typed(left, ty)?; // Using 'left' as a reference for naming, but type 'ty' for register type

        // Check if this is a floating-point operation based on the type
        let is_float = matches!(ty, 
            crate::ir::types::IrType::F32 | 
            crate::ir::types::IrType::F64
            // Note: Vector types would need to be handled separately if they exist in this IR
        );

        match op {
            crate::ir::instruction::IrBinaryOp::Add => {
                if is_float {
                    // For floating point addition, use XMM registers and appropriate instruction
                    instructions.push(X86Instruction::Other {
                        mnemonic: "addss".to_string(), // Add scalar single-precision
                        operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                    });
                } else {
                    // For integer addition
                    instructions.push(X86Instruction::Add {
                        dest: Operand::Register(result_reg),
                        src: Operand::Register(right_reg),
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::Subtract => {
                if is_float {
                    // For floating point subtraction
                    instructions.push(X86Instruction::Other {
                        mnemonic: "subss".to_string(), // Subtract scalar single-precision
                        operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                    });
                } else {
                    // For integer subtraction
                    instructions.push(X86Instruction::Sub {
                        dest: Operand::Register(result_reg),
                        src: Operand::Register(right_reg),
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::Multiply => {
                if is_float {
                    // For floating point multiplication
                    instructions.push(X86Instruction::Other {
                        mnemonic: "mulss".to_string(), // Multiply scalar single-precision
                        operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                    });
                } else {
                    // For integer multiplication
                    instructions.push(X86Instruction::Imul {
                        dest: Operand::Register(result_reg),
                        src: Operand::Register(right_reg),
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::Divide => {
                if is_float {
                    // For floating point division
                    instructions.push(X86Instruction::Other {
                        mnemonic: "divss".to_string(), // Divide scalar single-precision
                        operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                    });
                } else {
                    // For integer division
                    // Division is complex - requires RAX:RDX registers
                    // Save left operand to RAX
                    instructions.push(X86Instruction::Mov {
                        dest: Operand::Register(Register::GP(crate::asm::register::GPRegister::RAX)),
                        src: Operand::Register(left_reg),
                    });

                    // Sign-extend RAX into RDX (for signed division)
                    instructions.push(X86Instruction::Other { mnemonic: "cqo".to_string(), operands: vec![] });

                    // Perform signed division (RAX/RDX by right operand)
                    instructions.push(X86Instruction::Other {
                        mnemonic: "idiv".to_string(), // Signed division
                        operands: vec![Operand::Register(right_reg)],
                    });

                    // Move result (in RAX) to result register
                    instructions.push(X86Instruction::Mov {
                        dest: Operand::Register(result_reg),
                        src: Operand::Register(Register::GP(crate::asm::register::GPRegister::RAX)),
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::And => {
                // Integer bitwise AND - doesn't apply to floating point
                instructions.push(X86Instruction::Other {
                    mnemonic: "and".to_string(),
                    operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                });
            }
            crate::ir::instruction::IrBinaryOp::Or => {
                // Integer bitwise OR - doesn't apply to floating point
                instructions.push(X86Instruction::Other {
                    mnemonic: "or".to_string(),
                    operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                });
            }
            crate::ir::instruction::IrBinaryOp::BitwiseXor => {
                // Integer bitwise XOR - doesn't apply to floating point
                instructions.push(X86Instruction::Other {
                    mnemonic: "xor".to_string(),
                    operands: vec![Operand::Register(result_reg), Operand::Register(right_reg)],
                });
            }
            crate::ir::instruction::IrBinaryOp::Equal => {
                if is_float {
                    // For floating point comparison, use appropriate instruction
                    // This sets flags that can be used by conditional instructions
                    instructions.push(X86Instruction::Other {
                        mnemonic: "comiss".to_string(), // Compare scalar ordered single-precision
                        operands: vec![Operand::Register(left_reg), Operand::Register(right_reg)],
                    });
                    // Set result based on flags
                    instructions.push(X86Instruction::Other {
                        mnemonic: "sete".to_string(), // Set if equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                } else {
                    // For integer comparison
                    instructions.push(X86Instruction::Cmp { 
                        op1: Operand::Register(left_reg), 
                        op2: Operand::Register(right_reg) 
                    });
                    // Set result based on flags
                    instructions.push(X86Instruction::Other {
                        mnemonic: "sete".to_string(), // Set if equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::NotEqual => {
                if is_float {
                    instructions.push(X86Instruction::Other {
                        mnemonic: "comiss".to_string(), // Compare scalar ordered single-precision
                        operands: vec![Operand::Register(left_reg), Operand::Register(right_reg)],
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setne".to_string(), // Set if not equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                } else {
                    instructions.push(X86Instruction::Cmp { 
                        op1: Operand::Register(left_reg), 
                        op2: Operand::Register(right_reg) 
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setne".to_string(), // Set if not equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::Less => {
                if is_float {
                    instructions.push(X86Instruction::Other {
                        mnemonic: "comiss".to_string(), // Compare scalar ordered single-precision
                        operands: vec![Operand::Register(left_reg), Operand::Register(right_reg)],
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setl".to_string(), // Set if less
                        operands: vec![Operand::Register(result_reg)],
                    });
                } else {
                    instructions.push(X86Instruction::Cmp { 
                        op1: Operand::Register(left_reg), 
                        op2: Operand::Register(right_reg) 
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setl".to_string(), // Set if less
                        operands: vec![Operand::Register(result_reg)],
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::LessEqual => {
                if is_float {
                    instructions.push(X86Instruction::Other {
                        mnemonic: "comiss".to_string(), // Compare scalar ordered single-precision
                        operands: vec![Operand::Register(left_reg), Operand::Register(right_reg)],
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setle".to_string(), // Set if less or equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                } else {
                    instructions.push(X86Instruction::Cmp { 
                        op1: Operand::Register(left_reg), 
                        op2: Operand::Register(right_reg) 
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setle".to_string(), // Set if less or equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::Greater => {
                if is_float {
                    instructions.push(X86Instruction::Other {
                        mnemonic: "comiss".to_string(), // Compare scalar ordered single-precision
                        operands: vec![Operand::Register(left_reg), Operand::Register(right_reg)],
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setg".to_string(), // Set if greater
                        operands: vec![Operand::Register(result_reg)],
                    });
                } else {
                    instructions.push(X86Instruction::Cmp { 
                        op1: Operand::Register(left_reg), 
                        op2: Operand::Register(right_reg) 
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setg".to_string(), // Set if greater
                        operands: vec![Operand::Register(result_reg)],
                    });
                }
            }
            crate::ir::instruction::IrBinaryOp::GreaterEqual => {
                if is_float {
                    instructions.push(X86Instruction::Other {
                        mnemonic: "comiss".to_string(), // Compare scalar ordered single-precision
                        operands: vec![Operand::Register(left_reg), Operand::Register(right_reg)],
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setge".to_string(), // Set if greater or equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                } else {
                    instructions.push(X86Instruction::Cmp { 
                        op1: Operand::Register(left_reg), 
                        op2: Operand::Register(right_reg) 
                    });
                    instructions.push(X86Instruction::Other {
                        mnemonic: "setge".to_string(), // Set if greater or equal
                        operands: vec![Operand::Register(result_reg)],
                    });
                }
            }
            _ => {
                // Handle other operations or return an error
                return Err(CodeGenError::UnsupportedInstruction {
                    instruction: format!("{:?}", op),
                    span: crate::location::source_span::SourceSpan::default(),
                });
            }
        }

        Ok(instructions)
    }

    /// Gets or allocates a register for an IR value based on type
    fn get_or_allocate_register_for_value_typed(&mut self, value: &crate::ir::value::Value, ty: &crate::ir::types::IrType) -> Result<crate::asm::register::Register, CodeGenError> {
        // Create a string representation of the value using Display trait or debug name
        let value_str = if let Some(debug_info) = &value.debug_info
            && let Some(name) = &debug_info.name {
            // If there's a debug name, use that as the key
            format!("{}", name)
        } else {
            // Otherwise, use the full value representation
            format!("{}", value)
        };
        
        // Check if value is already mapped to a register
        if let Some(reg) = self.value_map.get(&value_str) {
            return Ok(*reg);
        }
        
        // Determine register type based on IR type
        let reg = match ty {
            crate::ir::types::IrType::F32 | crate::ir::types::IrType::F64 => {
                // Allocate XMM register for floating-point values
                if let Some(xmm_reg) = self.register_allocator.allocate_xmm_register(&value_str) {
                    let reg = crate::asm::register::Register::XMM(xmm_reg);
                    self.value_map.insert(value_str.clone(), reg);
                    self.allocated_values.insert(value_str);
                    reg
                } else {
                    // Fallback to GP if no XMM available
                    return Err(CodeGenError::RegisterAllocationFailure {
                        reason: format!("Could not allocate XMM register for floating-point value: {}", value_str),
                        function: "unknown".to_string(),
                        instruction_id: None,
                    });
                }
            }
            _ => {
                // Allocate GP register for integer and other types
                if let Some(gp_reg) = self.register_allocator.allocate_gp_register(&value_str) {
                    let reg = crate::asm::register::Register::GP(gp_reg);
                    self.value_map.insert(value_str.clone(), reg);
                    self.allocated_values.insert(value_str);
                    reg
                } else {
                    // Fallback if no GP register available
                    return Err(CodeGenError::RegisterAllocationFailure {
                        reason: format!("Could not allocate GP register for integer value: {}", value_str),
                        function: "unknown".to_string(),
                        instruction_id: None,
                    });
                }
            }
        };
        
        Ok(reg)
    }
    
    /// Gets or allocates a register for an IR value, trying to infer type from the value itself
    fn get_or_allocate_register_for_value_with_type_hint(&mut self, value: &crate::ir::value::Value) -> Result<crate::asm::register::Register, CodeGenError> {
        // Create a string representation of the value using Display trait or debug name
        let value_str = if let Some(debug_info) = &value.debug_info
            && let Some(name) = &debug_info.name {
            // If there's a debug name, use that as the key
            format!("{}", name)
        } else {
            // Otherwise, use the full value representation
            format!("{}", value)
        };
        
        // Check if value is already mapped to a register
        if let Some(reg) = self.value_map.get(&value_str) {
            return Ok(*reg);
        }
        
        // Try to determine the type from the value representation for literals
        let is_float_literal = value_str.contains("f32") || value_str.contains("f64");
        let is_integer_literal = value_str.contains("i8") || value_str.contains("i16") || 
                                  value_str.contains("i32") || value_str.contains("i64") ||
                                  value_str.contains("u8") || value_str.contains("u16") || 
                                  value_str.contains("u32") || value_str.contains("u64");
        
        // Allocate register based on inferred type
        let reg = if is_float_literal {
            // Allocate XMM register for floating-point literals
            if let Some(xmm_reg) = self.register_allocator.allocate_xmm_register(&value_str) {
                crate::asm::register::Register::XMM(xmm_reg)
            } else {
                // Fallback to GP if no XMM available
                return Err(CodeGenError::RegisterAllocationFailure {
                    reason: format!("Could not allocate XMM register for floating-point literal: {}", value_str),
                    function: "unknown".to_string(),
                    instruction_id: None,
                });
            }
        } else if is_integer_literal {
            // Allocate GP register for integer literals
            if let Some(gp_reg) = self.register_allocator.allocate_gp_register(&value_str) {
                crate::asm::register::Register::GP(gp_reg)
            } else {
                // Fallback if no GP register available
                return Err(CodeGenError::RegisterAllocationFailure {
                    reason: format!("Could not allocate GP register for integer literal: {}", value_str),
                    function: "unknown".to_string(),
                    instruction_id: None,
                });
            }
        } else {
            // For other values, use default GP register allocation
            if let Some(gp_reg) = self.register_allocator.allocate_gp_register(&value_str) {
                crate::asm::register::Register::GP(gp_reg)
            } else {
                // If no registers available for allocation, check if it's a parameter
                // Parameters might already have been assigned to specific registers by the ABI
                if let Some(debug_info) = &value.debug_info {
                    if let Some(name) = &debug_info.name {
                        // Check if it's a parameter in the current function context
                        if let Some(current_func) = self.function_contexts.values().last() {
                            if let Some(param_loc) = current_func.parameters.get(name.as_ref()) {
                                if let LocalLocation::Register(reg) = param_loc {
                                    return Ok(reg.clone());
                                }
                            }
                        }
                    }
                }
                
                // If no registers available, raise an error
                return Err(CodeGenError::RegisterAllocationFailure {
                    reason: format!("Could not allocate register for value: {}", value_str),
                    function: "unknown".to_string(),
                    instruction_id: None,
                });
            }
        };
        
        // Store the mapping
        self.value_map.insert(value_str.clone(), reg);
        self.allocated_values.insert(value_str);
        Ok(reg)
    }
    
    /// Gets or allocates a register for an IR value (with default to GP for backward compatibility)
    fn get_or_allocate_register_for_value(&mut self, value: &crate::ir::value::Value) -> Result<crate::asm::register::Register, CodeGenError> {
        // Try the type-hinted version first
        match self.get_or_allocate_register_for_value_with_type_hint(value) {
            Ok(reg) => Ok(reg),
            Err(_) => {
                // Fall back to the original method if the hint-based approach fails
                // Create a string representation of the value using Display trait or debug name
                let value_str = if let Some(debug_info) = &value.debug_info
                    && let Some(name) = &debug_info.name {
                    // If there's a debug name, use that as the key
                    format!("{}", name)
                } else {
                    // Otherwise, use the full value representation
                    format!("{}", value)
                };
                
                // Check if value is already mapped to a register
                if let Some(reg) = self.value_map.get(&value_str) {
                    return Ok(*reg);
                }
                
                // Try to allocate a GP register (default for backward compatibility)
                if let Some(gp_reg) = self.register_allocator.allocate_gp_register(&value_str) {
                    let reg = crate::asm::register::Register::GP(gp_reg);
                    self.value_map.insert(value_str.clone(), reg);
                    self.allocated_values.insert(value_str);
                    Ok(reg)
                } else {
                    // If no registers available for allocation, check if it's a parameter
                    // Parameters might already have been assigned to specific registers by the ABI
                    if let Some(debug_info) = &value.debug_info {
                        if let Some(name) = &debug_info.name {
                            // Check if it's a parameter in the current function context
                            if let Some(current_func) = self.function_contexts.values().last() {
                                if let Some(param_loc) = current_func.parameters.get(name.as_ref()) {
                                    if let LocalLocation::Register(reg) = param_loc {
                                        return Ok(reg.clone());
                                    }
                                }
                            }
                        }
                    }
                    
                    // If no registers available, raise an error
                    Err(CodeGenError::RegisterAllocationFailure {
                        reason: format!("Could not allocate register for value: {}", value_str),
                        function: "unknown".to_string(),
                        instruction_id: None,
                    })
                }
            }
        }
    }
    
    /// Gets the memory location for a pointer value (register or stack offset)
    /// This is different from get_memory_location_for_value because it handles pointer dereferencing
    fn get_memory_location_for_pointer_value(&mut self, value: &crate::ir::value::Value) -> Result<LocalLocation, CodeGenError> {
        // Create a string representation of the value using Display trait or debug name
        let value_str = if let Some(debug_info) = &value.debug_info
            && let Some(name) = &debug_info.name {
            // If there's a debug name, use that as the key
            format!("{}", name)
        } else {
            // Otherwise, use the full value representation
            format!("{}", value)
        };
        
        // Check if it's in local variables (this should be the case for pointer values from alloca)
        if let Some(local_var) = self.local_map.get(&value_str) {
            return Ok(local_var.location.clone());
        }
        
        // Check if it's in the value map
        if let Some(reg) = self.value_map.get(&value_str) {
            return Ok(LocalLocation::Register(*reg));
        }
        
        // If not found, return an error
        Err(CodeGenError::SymbolResolutionFailure {
            symbol: value_str,
            span: crate::location::source_span::SourceSpan::default(), // Placeholder
        })
    }

    /// Translate a unary operation to x86 instructions
    fn translate_unary_operation(
        &mut self, op: crate::ir::instruction::IrUnaryOp, operand: &crate::ir::value::Value,
        _ty: &crate::ir::types::IrType,
    ) -> Result<Vec<X86Instruction>, CodeGenError> {
        let mut instructions = Vec::new();

        // Get the register for the operand
        let operand_reg = self.get_or_allocate_register_for_value(operand)?;

        match op {
            crate::ir::instruction::IrUnaryOp::Negate => {
                // Apply NEG instruction to the operand
                instructions.push(X86Instruction::Other {
                    mnemonic: "neg".to_string(),
                    operands: vec![Operand::Register(operand_reg)],
                });
            }
            crate::ir::instruction::IrUnaryOp::Not => {
                // Apply NOT instruction to the operand
                instructions.push(X86Instruction::Other {
                    mnemonic: "not".to_string(),
                    operands: vec![Operand::Register(operand_reg)],
                });
            }
        }

        Ok(instructions)
    }
}

/// The main component responsible for translating IR to x86-64 assembly.
pub struct AssemblyGenerator {
    /// Platform for which assembly is generated
    pub target_platform: TargetPlatform,
    /// Implementation of calling convention for target platform
    pub calling_convention: Box<dyn CallingConvention>,
    /// Current function being processed with its local variables and metadata
    _function_context: Option<FunctionContext>,
    /// IR translator for converting IR to x86 instructions
    pub ir_translator: IRTranslator,
    /// Code generation options
    pub options: CodeGenOptions,
}

impl AssemblyGenerator {
    /// Creates a new AssemblyGenerator for the specified target platform.
    pub fn new(target_platform: TargetPlatform) -> Result<Self, CodeGenError> {
        let calling_convention: Box<dyn CallingConvention> = match target_platform.os {
            TargetOS::Windows => Box::new(WindowsX64ABI::new()),
            TargetOS::Linux | TargetOS::MacOS => Box::new(SystemVABI::new()),
        };

        Ok(AssemblyGenerator {
            target_platform,
            calling_convention,
            _function_context: None,
            ir_translator: IRTranslator::new(),
            options: CodeGenOptions::default(),
        })
    }

    /// Generates assembly code from the provided IR module.
    pub fn generate_assembly(&mut self, ir_module: Module) -> Result<String, CodeGenError> {
        let mut assembly_code = String::new();

        // DEBUG: Print the IR module for debugging purposes
        // println!("Debug IR module: {:#?}", ir_module);

        // Add NASM section headers
        assembly_code.push_str("section .text\n");

        // Add global declarations
        for function in &ir_module.functions {
            assembly_code.push_str(&format!("    global {}\n", function.name));
        }

        // Process each function
        for mut function in ir_module.functions {
            let function_name = &function.name;

            // Create function context with parameter mappings according to ABI
            let mut parameters = HashMap::new();
            
            // Map parameters to specific registers according to calling convention
            let int_params = self.calling_convention.integer_param_registers();
            
            // Process integer parameters
            for (i, param) in function.parameters.iter().enumerate() {
                if i < int_params.len() {
                    // Map parameter to the appropriate register according to ABI
                    let param_reg = int_params[i];
                    let param_name = param.name.to_string(); // Convert Arc<str> to String
                    parameters.insert(param_name.into(), LocalLocation::Register(Register::GP(param_reg)));
                } else {
                    // For parameters that don't fit in registers, they go on the stack
                    // According to the ABI, the offset is calculated differently for each platform
                    // For System V ABI, stack parameters start at RSP + 16 (after return address and old RBP)
                    // For Windows x64, shadow space is allocated but parameters start at RSP + 40 (32 bytes shadow + 8 for return addr)
                    let param_name = param.name.to_string(); // Convert Arc<str> to String
                    let stack_offset = match self.target_platform.os {
                        TargetOS::Windows => 40 + (i - int_params.len()) as i32 * 8, // Windows: 32 shadow + 8 return addr
                        _ => 16 + (i - int_params.len()) as i32 * 8, // System V: 8 return addr + 8 old RBP
                    };
                    parameters.insert(param_name.into(), LocalLocation::Stack { offset: stack_offset });
                }
            }

            // Calculate stack frame size needed for local variables
            // This is a simplified calculation - in a real implementation, we'd analyze the function
            // to determine actual local variable requirements
            let mut locals = HashMap::new();
            let mut local_offset = 0i32; // Start at -8 from RBP (locals go below the saved RBP)
            
            // In a complete implementation, we would analyze the function's instructions
            // to determine actual local variable needs, but for now we'll just use what's
            // already in the translator's local_map
            let stack_frame_size = 0; // Placeholder - should calculate actual size

            let mut ctx = FunctionContext {
                name: function_name.clone(),
                locals, // Initialize locals map
                parameters, // Add parameter mappings
                current_block: None,
                label_map: HashMap::new(),
                stack_frame_size: stack_frame_size,
                max_params: function.parameters.len(),
                instruction_offset: 0,
            };

            // Add function label
            assembly_code.push_str(&format!("\n{}:\n", function_name));

            // Generate prologue
            let prologue = self.calling_convention.generate_prologue(&ctx);
            for instr in prologue {
                assembly_code.push_str(&format!("    {}\n", self.instruction_to_asm(&instr)?));
            }

            // Process function instructions through the CFG
            // Update the translator with the current function context
            self.ir_translator.function_contexts.insert(function_name.clone(), ctx.clone());

            for block in function.cfg.blocks() {
                for instruction in &block.instructions {
                    let x86_instructions = self.ir_translator.translate_instruction(instruction)?;
                    for x86_instr in x86_instructions {
                        assembly_code.push_str(&format!("    {}\n", self.instruction_to_asm(&x86_instr)?));
                    }
                }
            }

            // Update context after translation to get accurate local variable info
            if let Some(updated_ctx) = self.ir_translator.function_contexts.get_mut(function_name) {
                // Calculate stack frame size based on local variables
                // Each local variable needs space on the stack
                let mut max_offset = 0i32;
                for local_var in updated_ctx.locals.values() {
                    if let LocalLocation::Stack { offset } = local_var.location {
                        if offset.abs() > max_offset {
                            max_offset = offset.abs();
                        }
                    }
                }
                
                // Calculate total stack frame size, aligned to 16-byte boundary for ABI compliance
                let total_size = max_offset + 8; // Add 8 bytes as buffer
                // Align to 16-byte boundary for proper ABI compliance
                let aligned_size = ((total_size + 15) / 16) * 16;
                updated_ctx.stack_frame_size = aligned_size as u32;
                
                ctx = updated_ctx.clone(); // Update the context for epilogue generation
            }

            // Generate epilogue
            let epilogue = self.calling_convention.generate_epilogue(&ctx);
            for instr in epilogue {
                assembly_code.push_str(&format!("    {}\n", self.instruction_to_asm(&instr)?));
            }
            
            // Update function context in translator after processing
            if let Some(updated_ctx) = self.ir_translator.function_contexts.get_mut(function_name) {
                *updated_ctx = ctx;
            }
        }

        // Add data section if needed
        assembly_code.push_str("\nsection .data\n");

        Ok(assembly_code)
    }

    /// Converts an x86 instruction to assembly string representation
    fn instruction_to_asm(&self, instruction: &X86Instruction) -> Result<String, CodeGenError> {
        match instruction {
            X86Instruction::Mov { dest, src } => {
                Ok(format!("mov {}, {}", self.operand_to_asm(dest)?, self.operand_to_asm(src)?))
            }
            X86Instruction::Add { dest, src } => {
                Ok(format!("add {}, {}", self.operand_to_asm(dest)?, self.operand_to_asm(src)?))
            }
            X86Instruction::Sub { dest, src } => {
                Ok(format!("sub {}, {}", self.operand_to_asm(dest)?, self.operand_to_asm(src)?))
            }
            X86Instruction::Imul { dest, src } => {
                Ok(format!("imul {}, {}", self.operand_to_asm(dest)?, self.operand_to_asm(src)?))
            }
            X86Instruction::Cmp { op1, op2 } => {
                Ok(format!("cmp {}, {}", self.operand_to_asm(op1)?, self.operand_to_asm(op2)?))
            }
            X86Instruction::Lea { dest, src } => {
                Ok(format!("lea {}, {}", self.operand_to_asm(dest)?, self.operand_to_asm(src)?))
            }
            X86Instruction::Jmp { target } => Ok(format!("jmp {}", self.operand_to_asm(target)?)),
            X86Instruction::ConditionalJump { mnemonic, target } => {
                Ok(format!("{} {}", mnemonic, self.operand_to_asm(target)?))
            }
            X86Instruction::Call { target } => Ok(format!("call {}", self.operand_to_asm(target)?)),
            X86Instruction::Ret => Ok("ret".to_string()),
            X86Instruction::Push { operand } => Ok(format!("push {}", self.operand_to_asm(operand)?)),
            X86Instruction::Pop { operand } => Ok(format!("pop {}", self.operand_to_asm(operand)?)),
            X86Instruction::Prologue => Ok("prologue".to_string()),
            X86Instruction::Epilogue => Ok("epilogue".to_string()),
            X86Instruction::Nop => Ok("nop".to_string()),
            X86Instruction::Other { mnemonic, operands } => {
                let mut result = mnemonic.clone();
                for (i, operand) in operands.iter().enumerate() {
                    if i == 0 {
                        result.push_str(&format!(" {}", self.operand_to_asm(operand)?));
                    } else {
                        result.push_str(&format!(", {}", self.operand_to_asm(operand)?));
                    }
                }
                Ok(result)
            }
        }
    }

    /// Converts an operand to assembly string representation
    fn operand_to_asm(&self, operand: &Operand) -> Result<String, CodeGenError> {
        match operand {
            Operand::Register(reg) => {
                Ok(format!("{}", reg)) // This will use our Display implementation
            }
            Operand::Immediate(value) => {
                Ok(format!(
                    "{}",
                    match value {
                        crate::asm::operand::ImmediateValue::Int8(v) => *v as i64,
                        crate::asm::operand::ImmediateValue::Int16(v) => *v as i64,
                        crate::asm::operand::ImmediateValue::Int32(v) => *v as i64,
                        crate::asm::operand::ImmediateValue::Int64(v) => *v,
                        crate::asm::operand::ImmediateValue::UInt8(v) => *v as i64,
                        crate::asm::operand::ImmediateValue::UInt16(v) => *v as i64,
                        crate::asm::operand::ImmediateValue::UInt32(v) => *v as i64,
                        crate::asm::operand::ImmediateValue::UInt64(v) => *v as i64,
                        _ => 0, // Handle float values appropriately
                    }
                ))
            }
            Operand::Memory(mem_op) => {
                let mut result = "[".to_string();

                if let Some(base) = mem_op.base {
                    result.push_str(&format!("{}", base));
                }

                if let Some(index) = mem_op.index {
                    if mem_op.base.is_some() {
                        result.push_str(&format!(" + {}", index));
                    } else {
                        result.push_str(&format!("{}", index));
                    }
                    if mem_op.scale != 1 {
                        result.push_str(&format!(" * {}", mem_op.scale));
                    }
                }

                if mem_op.displacement != 0 {
                    if mem_op.base.is_some() || mem_op.index.is_some() {
                        if mem_op.displacement > 0 {
                            result.push_str(&format!(" + {}", mem_op.displacement));
                        } else {
                            result.push_str(&format!(" - {}", -mem_op.displacement));
                        }
                    } else {
                        result.push_str(&format!("{}", mem_op.displacement));
                    }
                }

                result.push(']');
                Ok(result)
            }
            Operand::Label(label) => Ok(label.clone()),
            Operand::Address(addr) => Ok(format!("0x{:x}", addr)),
        }
    }

    /// Verifies semantic equivalence between IR and generated assembly.
    pub fn verify_equivalence(&self, _ir_module: &Module, _assembly_code: &str) -> Result<bool, CodeGenError> {
        // For now, return true as a placeholder (incomplete implementation)
        // A full implementation would execute both the IR and assembly code
        // and compare the results
        Ok(true)
    }
}
