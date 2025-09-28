//! Windows x64 ABI implementation
//!
//! Implements the Windows x64 calling convention as specified in the Microsoft
//! x64 ABI documentation.

use crate::asm::abi::{CallingConvention, /*CallingConventionType*/};
use crate::asm::register::{GPRegister, XMMRegister, Register};
use crate::asm::instruction::X86Instruction;
use crate::asm::generator::FunctionContext;

/// Windows x64 ABI implementation
#[derive(Debug, Clone)]
pub struct WindowsX64ABI;

impl WindowsX64ABI {
    pub fn new() -> Self {
        WindowsX64ABI
    }
}

impl CallingConvention for WindowsX64ABI {
    fn integer_param_registers(&self) -> &[GPRegister] {
        // Windows x64 ABI: RCX, RDX, R8, R9 for first 4 integer parameters
        use GPRegister::*;
        static REGS: &[GPRegister] = &[RCX, RDX, R8, R9];
        REGS
    }

    fn float_param_registers(&self) -> &[XMMRegister] {
        // Windows x64 ABI: XMM0, XMM1, XMM2, XMM3 for first 4 float parameters
        use XMMRegister::*;
        static REGS: &[XMMRegister] = &[XMM0, XMM1, XMM2, XMM3];
        REGS
    }

    fn return_registers(&self) -> (Option<GPRegister>, Option<XMMRegister>) {
        // Windows x64 ABI: RAX for integer returns, XMM0 for float returns
        (Some(GPRegister::RAX), Some(XMMRegister::XMM0))
    }

    fn caller_saved_registers(&self) -> &[Register] {
        // Windows x64 ABI caller-saved (volatile) registers
        use GPRegister::*;
        use XMMRegister::*;
        static REGS: &[Register] = &[
            Register::GP(RAX), Register::GP(RCX), Register::GP(RDX), 
            Register::GP(R8), Register::GP(R9), Register::GP(R10), Register::GP(R11),
            // XMM registers 0-15 are caller-saved
            Register::XMM(XMM0), Register::XMM(XMM1), Register::XMM(XMM2),
            Register::XMM(XMM3), Register::XMM(XMM4), Register::XMM(XMM5),
        ];
        REGS
    }

    fn callee_saved_registers(&self) -> &[Register] {
        // Windows x64 ABI callee-saved (non-volatile) registers
        use GPRegister::*;
        use XMMRegister::*;
        static REGS: &[Register] = &[
            Register::GP(RBX), Register::GP(RSI), Register::GP(RDI),
            Register::GP(RBP), Register::GP(R12), Register::GP(R13),
            Register::GP(R14), Register::GP(R15),
            // XMM6-XMM15 are callee-saved
            Register::XMM(XMM6), Register::XMM(XMM7), Register::XMM(XMM8),
            Register::XMM(XMM9), Register::XMM(XMM10), Register::XMM(XMM11),
            Register::XMM(XMM12), Register::XMM(XMM13), Register::XMM(XMM14),
            Register::XMM(XMM15),
        ];
        REGS
    }

    fn stack_alignment(&self) -> u32 {
        // Windows x64 ABI requires 16-byte stack alignment
        16
    }

    fn shadow_space_size(&self) -> u32 {
        // Windows x64 ABI requires 32 bytes of shadow space for the first 4 parameters
        32
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