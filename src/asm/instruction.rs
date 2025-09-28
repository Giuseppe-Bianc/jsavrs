//! # Instruction Representation
//!
//! Defines x86-64 instruction types and encoding mechanisms.

use std::fmt;
use crate::asm::operand::Operand;
//use iced_x86::{Instruction as IcedInstruction, Mnemonic, OpKind};

/// X86-64 instruction enum with all instruction variants
#[derive(Debug, Clone, PartialEq)]
pub enum X86Instruction {
    /// Move instruction: MOV destination, source
    Mov { dest: Operand, src: Operand },
    /// Add instruction: ADD destination, source
    Add { dest: Operand, src: Operand },
    /// Subtract instruction: SUB destination, source
    Sub { dest: Operand, src: Operand },
    /// Multiply instruction: IMUL destination, source
    Imul { dest: Operand, src: Operand },
    /// Compare instruction: CMP operand1, operand2
    Cmp { op1: Operand, op2: Operand },
    /// Jump instruction: JMP target
    Jmp { target: Operand },
    /// Conditional jump: JE/JNE/JL/JG/etc
    ConditionalJump { mnemonic: String, target: Operand },
    /// Call instruction: CALL function
    Call { target: Operand },
    /// Return instruction: RET
    Ret,
    /// Push instruction: PUSH operand
    Push { operand: Operand },
    /// Pop instruction: POP operand
    Pop { operand: Operand },
    /// Function prologue instructions
    Prologue,
    /// Function epilogue instructions
    Epilogue,
    /// No operation: NOP
    Nop,
    /// Other instructions can be added as needed
    Other { mnemonic: String, operands: Vec<Operand> },
}

impl fmt::Display for X86Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X86Instruction::Mov { dest, src } => write!(f, "mov {}, {}", dest, src),
            X86Instruction::Add { dest, src } => write!(f, "add {}, {}", dest, src),
            X86Instruction::Sub { dest, src } => write!(f, "sub {}, {}", dest, src),
            X86Instruction::Imul { dest, src } => write!(f, "imul {}, {}", dest, src),
            X86Instruction::Cmp { op1, op2 } => write!(f, "cmp {}, {}", op1, op2),
            X86Instruction::Jmp { target } => write!(f, "jmp {}", target),
            X86Instruction::ConditionalJump { mnemonic, target } => write!(f, "{} {}", mnemonic, target),
            X86Instruction::Call { target } => write!(f, "call {}", target),
            X86Instruction::Ret => write!(f, "ret"),
            X86Instruction::Push { operand } => write!(f, "push {}", operand),
            X86Instruction::Pop { operand } => write!(f, "pop {}", operand),
            X86Instruction::Prologue => write!(f, "prologue"),
            X86Instruction::Epilogue => write!(f, "epilogue"),
            X86Instruction::Nop => write!(f, "nop"),
            X86Instruction::Other { mnemonic, operands } => {
                write!(f, "{}", mnemonic)?;
                for (i, op) in operands.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {}", op)?;
                    } else {
                        write!(f, " {}", op)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl X86Instruction {
    /// Creates a MOV instruction
    pub fn mov(dest: Operand, src: Operand) -> Self {
        X86Instruction::Mov { dest, src }
    }

    /// Creates an ADD instruction
    pub fn add(dest: Operand, src: Operand) -> Self {
        X86Instruction::Add { dest, src }
    }

    /// Creates a SUB instruction
    pub fn sub(dest: Operand, src: Operand) -> Self {
        X86Instruction::Sub { dest, src }
    }

    /// Creates a RET instruction
    pub fn ret() -> Self {
        X86Instruction::Ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::register::{Register, GPRegister};

    #[test]
    fn test_mov_instruction() {
        let reg_rax = Register::GP(GPRegister::RAX);
        let reg_rbx = Register::GP(GPRegister::RBX);
        let mov_inst = X86Instruction::mov(
            Operand::Register(reg_rax),
            Operand::Register(reg_rbx),
        );
        
        match mov_inst {
            X86Instruction::Mov { dest, src } => {
                assert_eq!(dest, Operand::Register(reg_rax));
                assert_eq!(src, Operand::Register(reg_rbx));
            }
            _ => panic!("Expected MOV instruction"),
        }
    }
}