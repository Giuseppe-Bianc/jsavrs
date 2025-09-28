//! System V ABI implementation
//!
//! Implements the System V ABI calling convention used on Unix-like systems
//! (Linux, macOS) for x86-64 architecture.

use crate::asm::abi::{CallingConvention, /*CallingConventionType*/};
use crate::asm::register::{GPRegister, XMMRegister, Register};
use crate::asm::instruction::X86Instruction;
use crate::asm::generator::FunctionContext;

/// System V ABI implementation
#[derive(Debug, Clone)]
pub struct SystemVABI;

impl SystemVABI {
    pub fn new() -> Self {
        SystemVABI
    }
}

impl CallingConvention for SystemVABI {
    fn integer_param_registers(&self) -> &[GPRegister] {
        // System V ABI: RDI, RSI, RDX, RCX, R8, R9 for first 6 integer parameters
        use GPRegister::*;
        static REGS: &[GPRegister] = &[RDI, RSI, RDX, RCX, R8, R9];
        REGS
    }

    fn float_param_registers(&self) -> &[XMMRegister] {
        // System V ABI: XMM0-XMM7 for first 8 float parameters
        use XMMRegister::*;
        static REGS: &[XMMRegister] = &[XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7];
        REGS
    }

    fn return_registers(&self) -> (Option<GPRegister>, Option<XMMRegister>) {
        // System V ABI: RAX for integer returns, XMM0 for float returns
        (Some(GPRegister::RAX), Some(XMMRegister::XMM0))
    }

    fn caller_saved_registers(&self) -> &[Register] {
        // System V ABI caller-saved (volatile) registers
        use GPRegister::*;
        use XMMRegister::*;
        static REGS: &[Register] = &[
            Register::GP(RAX), Register::GP(RCX), Register::GP(RDX), 
            Register::GP(RSI), Register::GP(RDI), Register::GP(R8), 
            Register::GP(R9), Register::GP(R10), Register::GP(R11),
            // XMM registers 0-15 are caller-saved in System V
            Register::XMM(XMM0), Register::XMM(XMM1), Register::XMM(XMM2),
            Register::XMM(XMM3), Register::XMM(XMM4), Register::XMM(XMM5),
            Register::XMM(XMM6), Register::XMM(XMM7), Register::XMM(XMM8),
            Register::XMM(XMM9), Register::XMM(XMM10), Register::XMM(XMM11),
            Register::XMM(XMM12), Register::XMM(XMM13), Register::XMM(XMM14),
            Register::XMM(XMM15),
        ];
        REGS
    }

    fn callee_saved_registers(&self) -> &[Register] {
        // System V ABI callee-saved (non-volatile) registers
        use GPRegister::*;
        static REGS: &[Register] = &[
            Register::GP(RBX), Register::GP(RBP), Register::GP(R12),
            Register::GP(R13), Register::GP(R14), Register::GP(R15),
        ];
        REGS
    }

    fn stack_alignment(&self) -> u32 {
        // System V ABI requires 16-byte stack alignment
        16
    }

    fn shadow_space_size(&self) -> u32 {
        // System V ABI doesn't require shadow space (unlike Windows x64)
        0
    }

    fn generate_prologue(&self, ctx: &FunctionContext) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();
        
        // Push the frame pointer (RBP)
        instructions.push(X86Instruction::Push { 
            operand: crate::asm::operand::Operand::Register(Register::GP(GPRegister::RBP)) 
        });
        
        // Move stack pointer to frame pointer
        instructions.push(X86Instruction::Mov { 
            dest: crate::asm::operand::Operand::Register(Register::GP(GPRegister::RBP)),
            src: crate::asm::operand::Operand::Register(Register::GP(GPRegister::RSP))
        });
        
        // Allocate space for local variables if needed
        if ctx.stack_frame_size > 0 {
            // Adjust RSP to make space for locals (ensuring 16-byte alignment)
            let size = ctx.stack_frame_size;
            instructions.push(X86Instruction::Sub {
                dest: crate::asm::operand::Operand::Register(Register::GP(GPRegister::RSP)),
                src: crate::asm::operand::Operand::Immediate(crate::asm::operand::ImmediateValue::Int64(size as i64)),
            });
        }
        
        instructions
    }

    fn generate_epilogue(&self, ctx: &FunctionContext) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();
        
        // Restore the stack pointer if we allocated space for locals
        if ctx.stack_frame_size > 0 {
            let size = ctx.stack_frame_size;
            instructions.push(X86Instruction::Add {
                dest: crate::asm::operand::Operand::Register(Register::GP(GPRegister::RSP)),
                src: crate::asm::operand::Operand::Immediate(crate::asm::operand::ImmediateValue::Int64(size as i64)),
            });
        }
        
        // Restore the frame pointer (RBP)
        instructions.push(X86Instruction::Pop { 
            operand: crate::asm::operand::Operand::Register(Register::GP(GPRegister::RBP)) 
        });
        
        // Return
        instructions.push(X86Instruction::Ret);
        
        instructions
    }
}