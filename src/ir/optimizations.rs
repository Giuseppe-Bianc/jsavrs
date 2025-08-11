// src/ir/optimizations.rs
use crate::ir::ir::*;
use crate::ir::values::{Value, Constant};
use crate::ir::{BinaryOperator, ConversionOp, Type};
use crate::ir::validator::ControlFlowGraph;
use std::collections::HashMap;
pub struct ConstantPropagation {
    changed: bool,
    constant_values: HashMap<String, Constant>,
}
impl ConstantPropagation {
    pub fn new() -> Self {
        Self {
            changed: false,
            constant_values: HashMap::new(),
        }
    }
    pub fn run_on_module(&mut self, module: &mut Module) -> bool {
        self.changed = false;
        for function in &mut module.functions {
            self.run_on_function(function);
        }
        self.changed
    }
    pub fn run_on_function(&mut self, function: &mut Function) -> bool {
        self.changed = false;
        self.constant_values.clear();
        // Process basic blocks in order
        for block in &mut function.basic_blocks {
            self.run_on_block(block);
        }
        self.changed
    }
    fn run_on_block(&mut self, block: &mut BasicBlock) {
        let mut new_instructions = Vec::new();
        for instruction in &block.instructions {
            if let Some(simplified) = self.simplify_instruction(instruction) {
                new_instructions.push(simplified);
                self.changed = true;
            } else {
                new_instructions.push(instruction.clone());
            }
        }
        block.instructions = new_instructions;
    }
    fn simplify_instruction(&self, instruction: &Instruction) -> Option<Instruction> {
        match instruction {
            Instruction::BinaryOp { op, dest, left, right, flags:_ } => {
                if let (Some(left_const), Some(right_const)) = (
                    self.get_constant_value(left),
                    self.get_constant_value(right),
                ) {
                    if let Some(result) = self.evaluate_constant_binary_op(*op, &left_const, &right_const) {
                        return Some(Instruction::Alloca {
                            dest: dest.clone(),
                            ty: result.get_type(),
                            align: None,
                        });
                    }
                }
                None
            }
            Instruction::Conversion { op, dest, src, src_ty, dest_ty } => {
                if let Some(src_const) = self.get_constant_value(src) {
                    if let Some(result) = self.evaluate_constant_conversion(*op, &src_const, src_ty, dest_ty) {
                        return Some(Instruction::Alloca {
                            dest: dest.clone(),
                            ty: result.get_type(),
                            align: None,
                        });
                    }
                }
                None
            }
            Instruction::ICmp { dest, predicate, left, right } => {
                if let (Some(left_const), Some(right_const)) = (
                    self.get_constant_value(left),
                    self.get_constant_value(right),
                ) {
                    if let Some(result) = self.evaluate_constant_icmp(*predicate, &left_const, &right_const) {
                        return Some(Instruction::Alloca {
                            dest: dest.clone(),
                            ty: result.get_type(),
                            align: None,
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }
    fn get_constant_value(&self, value: &Value) -> Option<Constant> {
        match value {
            Value::Constant(c) => Some(c.clone()),
            Value::Instruction(ir) => {
                // Check if we have a constant value for this instruction
                self.constant_values.get(&format!("%{}", ir.id)).cloned()
            }
            _ => None,
        }
    }
    fn evaluate_constant_binary_op(
        &self,
        op: BinaryOperator,
        left: &Constant,
        right: &Constant,
    ) -> Option<Constant> {
        match (left, right) {
            (Constant::Integer { value: left_val, ty: left_ty }, Constant::Integer { value: right_val, ty: right_ty }) => {
                if left_ty != right_ty {
                    return None;
                }
                let result = match op {
                    BinaryOperator::Add => left_val.wrapping_add(*right_val),
                    BinaryOperator::Sub => left_val.wrapping_sub(*right_val),
                    BinaryOperator::Mul => left_val.wrapping_mul(*right_val),
                    BinaryOperator::UDiv => {
                        if *right_val == 0 {
                            return None;
                        }
                        left_val / right_val
                    }
                    BinaryOperator::SDiv => {
                        if *right_val == 0 {
                            return None;
                        }
                        // Handle signed division
                        let left_signed = *left_val as i64;
                        let right_signed = *right_val as i64;
                        (left_signed / right_signed) as u64
                    }
                    BinaryOperator::URem => {
                        if *right_val == 0 {
                            return None;
                        }
                        left_val % right_val
                    }
                    BinaryOperator::SRem => {
                        if *right_val == 0 {
                            return None;
                        }
                        // Handle signed remainder
                        let left_signed = *left_val as i64;
                        let right_signed = *right_val as i64;
                        (left_signed % right_signed) as u64
                    }
                    BinaryOperator::And => left_val & right_val,
                    BinaryOperator::Or => left_val | right_val,
                    BinaryOperator::Xor => left_val ^ right_val,
                    BinaryOperator::Shl => left_val.wrapping_shl(*right_val as u32),
                    BinaryOperator::LShr => left_val.wrapping_shr(*right_val as u32),
                    BinaryOperator::AShr => {
                        // Handle arithmetic shift right
                        let left_signed = *left_val as i64;
                        (left_signed >> (*right_val as u32)) as u64
                    }
                    _ => return None,
                };
                Some(Constant::Integer { value: result, ty: left_ty.clone() })
            }
            (Constant::Float { value: left_val, ty: left_ty }, Constant::Float { value: right_val, ty: right_ty }) => {
                if left_ty != right_ty {
                    return None;
                }
                let result = match op {
                    BinaryOperator::FAdd => left_val + right_val,
                    BinaryOperator::FSub => left_val - right_val,
                    BinaryOperator::FMul => left_val * right_val,
                    BinaryOperator::FDiv => {
                        if *right_val == 0.0 {
                            return None;
                        }
                        left_val / right_val
                    }
                    BinaryOperator::FRem => left_val % right_val,
                    _ => return None,
                };
                Some(Constant::Float { value: result, ty: left_ty.clone() })
            }
            _ => None,
        }
    }
    fn evaluate_constant_conversion(
        &self,
        op: ConversionOp,
        src: &Constant,
        src_ty: &Type,
        dest_ty: &Type,
    ) -> Option<Constant> {
        match (src, src_ty, dest_ty) {
            (Constant::Integer { value, .. }, Type::Integer { bits: src_bits, .. }, Type::Integer { bits: dest_bits, .. }) => {
                match op {
                    ConversionOp::Trunc if *dest_bits < *src_bits => {
                        Some(Constant::Integer { value: value & ((1u64 << *dest_bits) - 1), ty: dest_ty.clone() })
                    }
                    ConversionOp::ZExt if *dest_bits > *src_bits => {
                        Some(Constant::Integer { value: *value, ty: dest_ty.clone() })
                    }
                    ConversionOp::SExt if *dest_bits > *src_bits => {
                        // Sign extend
                        let sign_bit = 1u64 << (*src_bits - 1);
                        let mask = (1u64 << *dest_bits) - 1;
                        let sign_extended = if *value & sign_bit != 0 {
                            *value | (mask ^ ((1u64 << *src_bits) - 1))
                        } else {
                            *value
                        };
                        Some(Constant::Integer { value: sign_extended, ty: dest_ty.clone() })
                    }
                    _ => None,
                }
            }
            (Constant::Float { value, .. }, Type::Float { bits: 32 }, Type::Float { bits: 64 }) => {
                if op == ConversionOp::FPExt {
                    Some(Constant::Float { value: *value, ty: dest_ty.clone() })
                } else {
                    None
                }
            }
            (Constant::Float { value, .. }, Type::Float { bits: 64 }, Type::Float { bits: 32 }) => {
                if op == ConversionOp::FPTrunc {
                    Some(Constant::Float { value: *value as f32 as f64, ty: dest_ty.clone() })
                } else {
                    None
                }
            }
            (Constant::Integer { value, .. }, Type::Integer { .. }, Type::Float { .. }) => {
                match op {
                    ConversionOp::SIToFP => {
                        Some(Constant::Float { value: *value as i64 as f64, ty: dest_ty.clone() })
                    }
                    ConversionOp::UIToFP => {
                        Some(Constant::Float { value: *value as f64, ty: dest_ty.clone() })
                    }
                    _ => None,
                }
            }
            (Constant::Float { value, .. }, Type::Float { .. }, Type::Integer { .. }) => {
                match op {
                    ConversionOp::FPToSI => {
                        Some(Constant::Integer { value: *value as i64 as u64, ty: dest_ty.clone() })
                    }
                    ConversionOp::FPToUI => {
                        Some(Constant::Integer { value: *value as u64, ty: dest_ty.clone() })
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
    fn evaluate_constant_icmp(
        &self,
        predicate: IntPredicate,
        left: &Constant,
        right: &Constant,
    ) -> Option<Constant> {
        match (left, right) {
            (Constant::Integer { value: left_val, .. }, Constant::Integer { value: right_val, .. }) => {
                let result = match predicate {
                    IntPredicate::EQ => *left_val == *right_val,
                    IntPredicate::NE => *left_val != *right_val,
                    IntPredicate::UGT => *left_val > *right_val,
                    IntPredicate::UGE => *left_val >= *right_val,
                    IntPredicate::ULT => *left_val < *right_val,
                    IntPredicate::ULE => *left_val <= *right_val,
                    IntPredicate::SGT => (*left_val as i64) > (*right_val as i64),
                    IntPredicate::SGE => (*left_val as i64) >= (*right_val as i64),
                    IntPredicate::SLT => (*left_val as i64) < (*right_val as i64),
                    IntPredicate::SLE => (*left_val as i64) <= (*right_val as i64),
                };
                Some(Constant::Bool(result))
            }
            _ => None,
        }
    }
}
pub struct DeadCodeElimination {
    changed: bool,
}
impl DeadCodeElimination {
    pub fn new() -> Self {
        Self { changed: false }
    }
    pub fn run_on_module(&mut self, module: &mut Module) -> bool {
        self.changed = false;
        for function in &mut module.functions {
            self.run_on_function(function);
        }
        self.changed
    }
    pub fn run_on_function(&mut self, function: &mut Function) -> bool {
        self.changed = false;
        // Build CFG
        let cfg = ControlFlowGraph::build(&function.basic_blocks);
        // Compute reachable blocks
        let reachable = cfg.compute_reachable_blocks();
        // Remove unreachable blocks
        let original_count = function.basic_blocks.len();
        function.basic_blocks.retain(|block| reachable.contains(&block.name));
        self.changed = function.basic_blocks.len() < original_count;
        // Remove dead instructions within blocks
        for block in &mut function.basic_blocks {
            self.remove_dead_instructions(block);
        }
        self.changed
    }
    fn remove_dead_instructions(&mut self, block: &mut BasicBlock) {
        let mut live_instructions = std::collections::HashSet::new();
        // Mark terminator as live
        live_instructions.insert("terminator".to_string());
        // Mark instructions used by terminator
        self.mark_terminator_uses(&block.terminator, &mut live_instructions);
        // Mark instructions used by live instructions (worklist algorithm)
        let mut worklist = Vec::new();
        for instruction in &block.instructions {
            worklist.push(instruction);
        }
        while let Some(instruction) = worklist.pop() {
            let instruction_id = match instruction {
                Instruction::Alloca { dest, .. } => format!("{}", dest),
                Instruction::Load { dest, .. } => format!("{}", dest),
                Instruction::BinaryOp { dest, .. } => format!("{}", dest),
                Instruction::UnaryOp { dest, .. } => format!("{}", dest),
                Instruction::Call { dest, .. } => {
                    if let Some(d) = dest {
                        format!("{}", d)
                    } else {
                        continue;
                    }
                }
                Instruction::GetElementPtr { dest, .. } => format!("{}", dest),
                Instruction::Conversion { dest, .. } => format!("{}", dest),
                Instruction::Phi { dest, .. } => format!("{}", dest),
                Instruction::ExtractValue { dest, .. } => format!("{}", dest),
                Instruction::InsertValue { dest, .. } => format!("{}", dest),
                Instruction::Select { dest, .. } => format!("{}", dest),
                Instruction::ICmp { dest, .. } => format!("{}", dest),
                Instruction::FCmp { dest, .. } => format!("{}", dest),
                Instruction::VAArg { dest, .. } => format!("{}", dest),
                Instruction::LandingPad { dest, .. } => format!("{}", dest),
                Instruction::Store { .. } => continue, // Store has no dest
            };
            if live_instructions.contains(&instruction_id) {
                self.mark_uses(instruction, &mut live_instructions);
            }
        }
        // Remove dead instructions
        let original_count = block.instructions.len();
        block.instructions.retain(|instruction| {
            let instruction_id = match instruction {
                Instruction::Alloca { dest, .. } => format!("{}", dest),
                Instruction::Load { dest, .. } => format!("{}", dest),
                Instruction::BinaryOp { dest, .. } => format!("{}", dest),
                Instruction::UnaryOp { dest, .. } => format!("{}", dest),
                Instruction::Call { dest, .. } => {
                    if let Some(d) = dest {
                        format!("{}", d)
                    } else {
                        return true; // Keep calls without dest
                    }
                }
                Instruction::GetElementPtr { dest, .. } => format!("{}", dest),
                Instruction::Conversion { dest, .. } => format!("{}", dest),
                Instruction::Phi { dest, .. } => format!("{}", dest),
                Instruction::ExtractValue { dest, .. } => format!("{}", dest),
                Instruction::InsertValue { dest, .. } => format!("{}", dest),
                Instruction::Select { dest, .. } => format!("{}", dest),
                Instruction::ICmp { dest, .. } => format!("{}", dest),
                Instruction::FCmp { dest, .. } => format!("{}", dest),
                Instruction::VAArg { dest, .. } => format!("{}", dest),
                Instruction::LandingPad { dest, .. } => format!("{}", dest),
                Instruction::Store { .. } => return true, // Keep stores
            };
            live_instructions.contains(&instruction_id)
        });
        self.changed = self.changed || block.instructions.len() < original_count;
    }

    // Nuovo metodo per gestire i Terminator
    fn mark_terminator_uses(&self, terminator: &Terminator, live_instructions: &mut std::collections::HashSet<String>) {
        match terminator {
            Terminator::Ret { value } => {
                if let Some(v) = value {
                    self.mark_value(v, live_instructions);
                }
            }
            Terminator::Br { .. } => {}
            Terminator::CondBr { condition, .. } => {
                self.mark_value(condition, live_instructions);
            }
            Terminator::Switch { value, .. } => {
                self.mark_value(value, live_instructions);
            }
            Terminator::IndirectBr { address, .. } => {
                self.mark_value(address, live_instructions);
            }
            Terminator::Invoke { callee, arguments, .. } => {
                self.mark_value(callee, live_instructions);
                for (arg, _) in arguments {
                    self.mark_value(arg, live_instructions);
                }
            }
            Terminator::Resume { value } => {
                self.mark_value(value, live_instructions);
            }
            Terminator::CatchSwitch { parent_pad, .. } => {
                self.mark_value(parent_pad, live_instructions);
            }
            Terminator::CatchRet { from, .. } => {
                self.mark_value(from, live_instructions);
            }
            Terminator::CleanupRet { from, .. } => {
                self.mark_value(from, live_instructions);
            }
            Terminator::Unreachable => {}
        }
    }

    fn mark_uses(&self, instruction: &Instruction, live_instructions: &mut std::collections::HashSet<String>) {
        match instruction {
            Instruction::Alloca { .. } => {}
            Instruction::Load { ptr, .. } => {
                self.mark_value(ptr, live_instructions);
            }
            Instruction::Store { value, ptr, .. } => {
                self.mark_value(value, live_instructions);
                self.mark_value(ptr, live_instructions);
            }
            Instruction::BinaryOp { left, right, .. } => {
                self.mark_value(left, live_instructions);
                self.mark_value(right, live_instructions);
            }
            Instruction::UnaryOp { operand, .. } => {
                self.mark_value(operand, live_instructions);
            }
            Instruction::Call { callee, arguments, .. } => {
                self.mark_value(callee, live_instructions);
                for (arg, _) in arguments {
                    self.mark_value(arg, live_instructions);
                }
            }
            Instruction::GetElementPtr { ptr, indices, .. } => {
                self.mark_value(ptr, live_instructions);
                for (idx, _) in indices {
                    self.mark_value(idx, live_instructions);
                }
            }
            Instruction::Conversion { src, .. } => {
                self.mark_value(src, live_instructions);
            }
            Instruction::Phi { incoming, .. } => {
                for (value, _) in incoming {
                    self.mark_value(value, live_instructions);
                }
            }
            Instruction::ExtractValue { aggregate, .. } => {
                self.mark_value(aggregate, live_instructions);
            }
            Instruction::InsertValue { aggregate, element, .. } => {
                self.mark_value(aggregate, live_instructions);
                self.mark_value(element, live_instructions);
            }
            Instruction::Select { condition, true_value, false_value, .. } => {
                self.mark_value(condition, live_instructions);
                self.mark_value(true_value, live_instructions);
                self.mark_value(false_value, live_instructions);
            }
            Instruction::ICmp { left, right, .. } => {
                self.mark_value(left, live_instructions);
                self.mark_value(right, live_instructions);
            }
            Instruction::FCmp { left, right, .. } => {
                self.mark_value(left, live_instructions);
                self.mark_value(right, live_instructions);
            }
            Instruction::VAArg { va_list, .. } => {
                self.mark_value(va_list, live_instructions);
            }
            Instruction::LandingPad { .. } => {}
        }
    }
    fn mark_value(&self, value: &Value, live_instructions: &mut std::collections::HashSet<String>) {
        match value {
            Value::Instruction(ir) => {
                live_instructions.insert(format!("%{}", ir.id));
            }
            _ => {}
        }
    }
}