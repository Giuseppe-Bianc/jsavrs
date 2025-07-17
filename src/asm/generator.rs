// src/asm/generator.rs
use crate::error::compile_error::CompileError;
use crate::ir::{
    BasicBlock, Function, ImmediateValue, Instruction, IrBinaryOp, IrType, Terminator,
    Value, ValueKind,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetOS {
    Linux,
    Windows,
}

pub struct AsmGenerator {
    target_os: TargetOS,
    asm_code: String,
    data_section: String,
    current_fn: String,
    stack_offset: i32,
    param_count: usize,
    label_counter: usize,
    temp_offset_map: HashMap<String, i32>,
    string_constants: HashMap<String, String>,
    next_str_id: usize,
    errors: Vec<CompileError>,
}

impl AsmGenerator {
    pub fn new(target_os: TargetOS) -> Self {
        Self {
            target_os,
            asm_code: String::new(),
            data_section: String::new(),
            current_fn: String::new(),
            stack_offset: 0,
            param_count: 0,
            label_counter: 0,
            temp_offset_map: HashMap::new(),
            string_constants: HashMap::new(),
            next_str_id: 0,
            errors: Vec::new(),
        }
    }

    pub fn generate(&mut self, functions: Vec<Function>) -> (String, Vec<CompileError>) {
        self.emit_header();

        for func in functions {
            if let Err(e) = self.generate_function(&func) {
                self.errors.push(e);
            }
        }

        // Add Windows-specific directives at the top
        let mut full_asm = String::new();
        if self.target_os == TargetOS::Windows {
            self.generate_windows_entry_point();
            full_asm.push_str("BITS 64\n");
            full_asm.push_str("DEFAULT REL\n\n");
        }
        full_asm.push_str(&format!("{}\n\n{}", self.data_section, self.asm_code));

        (full_asm, std::mem::take(&mut self.errors))
    }

    fn error(&self, message: &str) -> CompileError {
        CompileError::AsmGeneratorError(message.to_string())
    }

    fn emit(&mut self, code: &str) {
        self.asm_code.push_str(code);
        self.asm_code.push('\n');
    }

    fn emit_header(&mut self) {
        self.data_section.push_str("section .data\n");
        self.asm_code.push_str("section .text\n");

        match self.target_os {
            TargetOS::Linux => {
                self.asm_code.push_str("global main\n");
                self.asm_code.push_str("extern printf, exit\n");
            }
            TargetOS::Windows => {
                self.asm_code.push_str("global mainCRTStartup\n");
                self.asm_code.push_str("extern printf, ExitProcess\n");
            }
        }
    }

    fn generate_windows_entry_point(&mut self) {
        self.asm_code.push_str(
            r#"
mainCRTStartup:
    call    main
    mov     rcx, rax
    call    ExitProcess
"#,
        );
    }

    fn generate_function(&mut self, func: &Function) -> Result<(), CompileError> {
        self.current_fn = func.name.clone();
        self.stack_offset = 0;
        self.param_count = 0;
        self.temp_offset_map.clear();

        // Function prologue
        self.emit(&format!("{}:", func.name));
        self.emit("push rbp");
        self.emit("mov rbp, rsp");

        // Calculate stack size and allocate space
        let stack_size = self.calculate_stack_size(func);
        if stack_size > 0 {
            self.emit(&format!("sub rsp, {}", align_to(stack_size, 16)));
        }

        // Store parameters to stack
        for (i, (name, ty)) in func.parameters.iter().enumerate() {
            let reg = self.get_param_reg(i, ty)?.to_string();
            let size = self.type_size(ty) as i32;
            let offset = self.stack_offset + size;
            self.temp_offset_map.insert(name.clone(), offset);
            self.emit_store(&reg, "rbp", -offset, ty)?;
            self.stack_offset = offset;
        }

        // Generate basic blocks
        for block in &func.basic_blocks {
            self.generate_block(block)?;
        }

        // Function epilogue
        self.emit(".return:");
        self.emit("mov rsp, rbp");
        self.emit("pop rbp");
        self.emit("ret");

        Ok(())
    }

    fn generate_block(&mut self, block: &BasicBlock) -> Result<(), CompileError> {
        self.emit(&format!(".{}:", block.label));

        for inst in &block.instructions {
            self.generate_instruction(inst)?;
        }

        self.generate_terminator(&block.terminator)?;
        Ok(())
    }

    fn generate_instruction(&mut self, inst: &Instruction) -> Result<(), CompileError> {
        match inst {
            Instruction::Alloca { dest, ty } => {
                let size = self.type_size(ty) as i32;
                self.stack_offset += size;
                self.temp_offset_map.insert(dest.clone(), self.stack_offset);
                Ok(())
            }
            Instruction::Store { value, dest } => {
                let src_reg = self.load_value(value, "rax", "xmm0")?;
                let dest_offset = self.get_value_offset(dest)?;
                self.emit_store(&src_reg, "rbp", -dest_offset, &dest.ty)
            }
            Instruction::Load { dest, src, ty } => {
                let src_offset = self.get_value_offset(src)?;
                let tmp_reg = if self.is_float(ty) { "xmm0" } else { "rax" };
                self.emit_load("rbp", -src_offset, tmp_reg, ty)?;

                let dest_offset = self.stack_offset + self.type_size(ty) as i32;
                self.temp_offset_map.insert(dest.clone(), dest_offset);
                self.emit_store(tmp_reg, "rbp", -dest_offset, ty)?;
                self.stack_offset = dest_offset;
                Ok(())
            }
            Instruction::Binary {
                op,
                dest,
                left,
                right,
                ty,
            } => self.generate_binary_op(op, left, right, dest, ty),
            Instruction::Call {
                dest,
                func,
                args,
                ty,
            } => self.generate_call(func, args, dest, ty),
            Instruction::Unary { .. } => Err(self.error("Unary operations not implemented")),
            _ => Err(self.error(&format!(
                "Unsupported instruction: {:?}",
                inst
            ))),
        }
    }

    fn generate_terminator(&mut self, term: &Terminator) -> Result<(), CompileError> {
        match term {
            Terminator::Return(value, ty) => {
                let reg = self.load_value(value, "rax", "xmm0")?;
                self.emit("jmp .return");
                Ok(())
            }
            Terminator::Branch(label) => {
                self.emit(&format!("jmp .{}", label));
                Ok(())
            }
            Terminator::ConditionalBranch {
                condition,
                true_label,
                false_label,
            } => {
                let cond_reg = self.load_value(condition, "rax", "xmm0")?;
                self.emit(&format!("cmp {}, 0", cond_reg));
                self.emit(&format!("jne .{}", true_label));
                self.emit(&format!("jmp .{}", false_label));
                Ok(())
            }
            _ => Err(self.error("Unsupported terminator")),
        }
    }

    fn get_param_reg(&self, index: usize, ty: &IrType) -> Result<&str, CompileError> {
        let is_float = self.is_float(ty);
        match (self.target_os, index, is_float) {
            (TargetOS::Linux, 0, false) => Ok("rdi"),
            (TargetOS::Linux, 1, false) => Ok("rsi"),
            (TargetOS::Linux, 2, false) => Ok("rdx"),
            (TargetOS::Linux, 3, false) => Ok("rcx"),
            (TargetOS::Linux, 4, false) => Ok("r8"),
            (TargetOS::Linux, 5, false) => Ok("r9"),
            (TargetOS::Linux, 0, true) => Ok("xmm0"),
            (TargetOS::Linux, 1, true) => Ok("xmm1"),
            (TargetOS::Linux, 2, true) => Ok("xmm2"),
            (TargetOS::Linux, 3, true) => Ok("xmm3"),
            (TargetOS::Windows, 0, false) => Ok("rcx"),
            (TargetOS::Windows, 1, false) => Ok("rdx"),
            (TargetOS::Windows, 2, false) => Ok("r8"),
            (TargetOS::Windows, 3, false) => Ok("r9"),
            (TargetOS::Windows, 0, true) => Ok("xmm0"),
            (TargetOS::Windows, 1, true) => Ok("xmm1"),
            (TargetOS::Windows, 2, true) => Ok("xmm2"),
            (TargetOS::Windows, 3, true) => Ok("xmm3"),
            _ => Err(self.error(&format!(
                "Unsupported parameter index {} for type {:?}",
                index, ty
            ))),
        }
    }

    fn type_size(&self, ty: &IrType) -> usize {
        match ty {
            IrType::I8 | IrType::U8 | IrType::Bool => 1,
            IrType::I16 | IrType::U16 => 2,
            IrType::I32 | IrType::U32 | IrType::F32 => 4,
            IrType::I64 | IrType::U64 | IrType::F64 | IrType::Pointer(_) => 8,
            IrType::Array(element_ty, size) => self.type_size(element_ty) * size,
            _ => 8,
        }
    }

    fn is_float(&self, ty: &IrType) -> bool {
        matches!(ty, IrType::F32 | IrType::F64)
    }

    fn calculate_stack_size(&self, func: &Function) -> i32 {
        let mut size = 0;
        for (_, ty) in &func.parameters {
            size += self.type_size(ty) as i32;
        }
        for (_, ty) in &func.local_vars {
            size += self.type_size(ty) as i32;
        }
        size += func.basic_blocks.iter().map(|bb| bb.instructions.len()).sum::<usize>() as i32 * 8;
        size
    }

    fn load_value(
        &mut self,
        value: &Value,
        int_reg: &str,
        float_reg: &str,
    ) -> Result<String, CompileError> {
        match &value.kind {
            ValueKind::Immediate(imm) => {
                let reg = if self.is_float(&value.ty) {
                    float_reg
                } else {
                    int_reg
                };
                let imm_str = self.imm_to_string(imm)?;
                self.emit(&format!("mov {}, {}", reg, imm_str));
                Ok(reg.to_string())
            }
            _ => {
                let offset = self.get_value_offset(value)?;
                let reg = if self.is_float(&value.ty) {
                    float_reg
                } else {
                    int_reg
                };
                self.emit_load("rbp", -offset, reg, &value.ty)?;
                Ok(reg.to_string())
            }
        }
    }

    fn get_value_offset(&self, value: &Value) -> Result<i32, CompileError> {
        match &value.kind {
            ValueKind::Local(name) | ValueKind::Temporary(name) => {
                self.temp_offset_map
                    .get(name)
                    .copied()
                    .ok_or_else(|| self.error(&format!("Undefined variable: {}", name)))
            }
            _ => Err(self.error("Value offset not available for immediate")),
        }
    }

    fn emit_load(
        &mut self,
        base: &str,
        offset: i32,
        dest: &str,
        ty: &IrType,
    ) -> Result<(), CompileError> {
        match ty {
            IrType::F32 => {
                self.emit(&format!("movss {}, [{} + {}]", dest, base, offset));
                Ok(())
            }
            IrType::F64 => {
                self.emit(&format!("movsd {}, [{} + {}]", dest, base, offset));
                Ok(())
            }
            _ => {
                let size = self.type_size(ty);
                let suffix = match size {
                    1 => "BYTE",
                    2 => "WORD",
                    4 => "DWORD",
                    8 => "QWORD",
                    _ => "QWORD",
                };
                self.emit(&format!(
                    "mov {}, {} [{} + {}]",
                    self.get_subreg(dest, size),
                    suffix,
                    base,
                    offset
                ));
                Ok(())
            }
        }
    }

    fn emit_store(
        &mut self,
        src: &str,
        base: &str,
        offset: i32,
        ty: &IrType,
    ) -> Result<(), CompileError> {
        match ty {
            IrType::F32 => {
                self.emit(&format!("movss [{} + {}], {}", base, offset, src));
                Ok(())
            }
            IrType::F64 => {
                self.emit(&format!("movsd [{} + {}], {}", base, offset, src));
                Ok(())
            }
            _ => {
                let size = self.type_size(ty);
                let suffix = match size {
                    1 => "BYTE",
                    2 => "WORD",
                    4 => "DWORD",
                    8 => "QWORD",
                    _ => "QWORD",
                };
                self.emit(&format!(
                    "mov {} [{} + {}], {}",
                    suffix,
                    base,
                    offset,
                    self.get_subreg(src, size)
                ));
                Ok(())
            }
        }
    }

    fn get_subreg(&self, reg: &str, size: usize) -> String {
        if reg.starts_with("xmm") {
            return reg.to_string();
        }
        match size {
            1 => match reg {
                "rax" => "al",
                "rbx" => "bl",
                "rcx" => "cl",
                "rdx" => "dl",
                _ => reg,
            },
            2 => match reg {
                "rax" => "ax",
                "rbx" => "bx",
                "rcx" => "cx",
                "rdx" => "dx",
                _ => reg,
            },
            4 => match reg {
                "rax" => "eax",
                "rbx" => "ebx",
                "rcx" => "ecx",
                "rdx" => "edx",
                _ => reg,
            },
            _ => reg,
        }
            .to_string()
    }

    fn imm_to_string(&mut self, imm: &ImmediateValue) -> Result<String, CompileError> {
        match imm {
            ImmediateValue::I8(v) => Ok(format!("{}", v)),
            ImmediateValue::I16(v) => Ok(format!("{}", v)),
            ImmediateValue::I32(v) => Ok(format!("{}", v)),
            ImmediateValue::I64(v) => Ok(format!("{}", v)),
            ImmediateValue::U8(v) => Ok(format!("{}", v)),
            ImmediateValue::U16(v) => Ok(format!("{}", v)),
            ImmediateValue::U32(v) => Ok(format!("{}", v)),
            ImmediateValue::U64(v) => Ok(format!("{}", v)),
            ImmediateValue::F32(v) => Ok(format!("__float32__({})", v)),
            ImmediateValue::F64(v) => Ok(format!("__float64__({})", v)),
            ImmediateValue::Bool(v) => Ok(format!("{}", if *v { 1 } else { 0 })),
            ImmediateValue::Char(v) => Ok(format!("'{}'", v)),
            ImmediateValue::String(s) => Ok(self.get_string_constant(s)),
        }
    }

    fn get_string_constant(&mut self, s: &str) -> String {
        if let Some(label) = self.string_constants.get(s) {
            return label.clone();
        }
        let label = format!("str_{}", self.next_str_id);
        self.next_str_id += 1;
        self.data_section.push_str(&format!(
            "{} db '{}', 0\n",
            label,
            s.replace('\n', "\\n").replace('\t', "\\t")
        ));
        self.string_constants.insert(s.to_string(), label.clone());
        label
    }

    fn generate_binary_op(
        &mut self,
        op: &IrBinaryOp,
        left: &Value,
        right: &Value,
        dest: &str,
        ty: &IrType,
    ) -> Result<(), CompileError> {
        let is_float = self.is_float(ty);
        let left_reg = self.load_value(left, "rax", "xmm0")?;
        let right_reg = self.load_value(right, "rdx", "xmm1")?;

        let op_asm = match (op, is_float) {
            (IrBinaryOp::Add, false) => "add",
            (IrBinaryOp::Add, true) => "addss",
            (IrBinaryOp::Subtract, false) => "sub",
            (IrBinaryOp::Subtract, true) => "subss",
            (IrBinaryOp::Multiply, false) => "imul",
            (IrBinaryOp::Multiply, true) => "mulss",
            (IrBinaryOp::Equal, false) => "cmp",
            (IrBinaryOp::Equal, true) => "comiss",
            _ => return Err(self.error("Unsupported binary operation")),
        };

        self.emit(&format!("{} {}, {}", op_asm, left_reg, right_reg));

        if matches!(op, IrBinaryOp::Equal) {
            self.emit("sete al");
            self.emit("movzx rax, al");
        }

        let dest_offset = self.stack_offset + self.type_size(ty) as i32;
        self.temp_offset_map.insert(dest.to_string(), dest_offset);
        self.emit_store(&left_reg, "rbp", -dest_offset, ty)?;
        self.stack_offset = dest_offset;
        Ok(())
    }

    fn generate_call(
        &mut self,
        func: &str,
        args: &[Value],
        dest: &Option<String>,
        ty: &IrType,
    ) -> Result<(), CompileError> {
        // Setup arguments
        for (i, arg) in args.iter().enumerate() {
            let reg = self.get_param_reg(i, &arg.ty)?.to_string();
            let value_reg = self.load_value(arg, "rax", "xmm0")?;
            if self.is_float(&arg.ty) {
                self.emit(&format!("movss {}, {}", reg, value_reg));
            } else {
                self.emit(&format!("mov {}, {}", reg, value_reg));
            }
        }

        // Call function
        self.emit(&format!("call {}", func));

        // Handle result
        if let Some(dest) = dest {
            let dest_offset = self.stack_offset + self.type_size(ty) as i32;
            self.temp_offset_map.insert(dest.clone(), dest_offset);

            let reg = if self.is_float(ty) { "xmm0" } else { "rax" };
            self.emit_store(reg, "rbp", -dest_offset, ty)?;
            self.stack_offset = dest_offset;
        }

        Ok(())
    }
}

fn align_to(size: i32, align: i32) -> i32 {
    (size + align - 1) / align * align
}