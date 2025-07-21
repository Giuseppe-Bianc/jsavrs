// src/nir/generator.rs
use super::*;
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::nir::function::{IrParameter, ParamAttributes};
use crate::nir::instruction::{Instruction, InstructionKind, IrBinaryOp, IrUnaryOp};
use crate::parser::ast::*;
use crate::tokens::number::Number;
use std::collections::HashMap;

pub struct NIrGenerator {
    current_function: Option<String>,
    current_block: Option<BasicBlock>,
    current_block_label: Option<String>,
    symbol_table: HashMap<String, Value>,
    temp_counter: u64,
    block_counter: usize,
    errors: Vec<CompileError>,
    value_types: HashMap<String, IrType>,
    break_stack: Vec<String>,
    continue_stack: Vec<String>,
    type_context: TypeContext,
    _next_value_id: u64,
}

#[derive(Debug, Default)]
struct TypeContext {
    structs: HashMap<String, (Vec<IrType>, SourceSpan)>,
    _aliases: HashMap<String, IrType>,
}

#[allow(clippy::collapsible_if, clippy::collapsible_else_if)]
impl NIrGenerator {
    pub fn new() -> Self {
        Self {
            current_function: None,
            current_block: None,
            current_block_label: None,
            symbol_table: HashMap::new(),
            temp_counter: 0,
            block_counter: 0,
            errors: Vec::new(),
            value_types: HashMap::new(),
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            type_context: TypeContext::default(),
            _next_value_id: 1,
        }
    }

    /*fn next_value_id(&mut self) -> u64 {
        let id = self.next_value_id;
        self.next_value_id += 1;
        id
    }*/

    fn block_needs_terminator(&self) -> bool {
        self.current_block
            .as_ref()
            .is_some_and(|b| !b.terminator.is_terminator())
    }

    pub fn generate(&mut self, stmts: Vec<Stmt>) -> (Vec<Function>, Vec<CompileError>) {
        let mut functions = Vec::new();

        for stmt in stmts {
            match stmt {
                Stmt::Function {
                    name,
                    parameters,
                    return_type,
                    body,
                    span,
                } => {
                    let mut func =
                        self.create_function(&name, &parameters, return_type, span.clone());
                    self.generate_function_body(&mut func, body, span);
                    functions.push(func);
                }
                Stmt::MainFunction { body, span } => {
                    let mut func = self.create_function("main", &[], Type::Void, span.clone());
                    self.generate_function_body(&mut func, body, span);
                    functions.push(func);
                }
                /*Stmt::Struct { name, fields, span } => {
                    self.register_struct(name, fields, span);
                }*/
                other => {
                    self.new_error(
                        "Unsupported top-level statement".to_string(),
                        other.span().clone(),
                    );
                }
            }
        }

        (functions, std::mem::take(&mut self.errors))
    }

    /*fn register_struct(&mut self, name: String, fields: Vec<(String, Type)>, span: SourceSpan) {
        let field_types = fields.into_iter()
            .map(|(_, ty)| self.map_type(&ty))
            .collect();

        self.type_context.structs.insert(name.clone(), (field_types, span));
    }*/

    fn new_error(&mut self, message: String, span: SourceSpan) {
        self.errors
            .push(CompileError::IrGeneratorError { message, span });
    }

    fn create_function(
        &mut self,
        name: &str,
        params: &[Parameter],
        return_type: Type,
        span: SourceSpan,
    ) -> Function {
        let ir_params = params
            .iter()
            .map(|param| {
                let ty = self.map_type(&param.type_annotation);
                IrParameter {
                    name: param.name.clone(),
                    ty: ty.clone(),
                    attributes: ParamAttributes {
                        source_span: Some(param.span.clone()),
                        ..Default::default()
                    },
                }
            })
            .collect();

        let ir_return_type = self.map_type(&return_type);

        let mut func = Function::new(name, ir_params, ir_return_type);
        func.attributes.source_span = Some(span);
        func
    }

    fn map_type(&self, ty: &Type) -> IrType {
        match ty {
            Type::I8 => IrType::I8,
            Type::I16 => IrType::I16,
            Type::I32 => IrType::I32,
            Type::I64 => IrType::I64,
            Type::U8 => IrType::U8,
            Type::U16 => IrType::U16,
            Type::U32 => IrType::U32,
            Type::U64 => IrType::U64,
            Type::F32 => IrType::F32,
            Type::F64 => IrType::F64,
            Type::Char => IrType::Char,
            Type::String => IrType::String,
            Type::Bool => IrType::Bool,
            Type::Custom(name) => {
                if let Some((fields, span)) = self.type_context.structs.get(name) {
                    IrType::Struct(name.clone(), fields.clone(), span.clone())
                } else {
                    IrType::Custom(name.clone(), SourceSpan::default())
                }
            }
            Type::Array(element_type, size_expr) => {
                if let Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(size)),
                    ..
                } = **size_expr
                {
                    IrType::Array(Box::new(self.map_type(element_type)), size as usize)
                } else {
                    IrType::Pointer(Box::new(self.map_type(element_type)))
                }
            }
            Type::Vector(element_type) => IrType::Pointer(Box::new(self.map_type(element_type))),
            Type::Void => IrType::Void,
            Type::NullPtr => IrType::Pointer(Box::new(IrType::I8)),
        }
    }

    fn finalize_current_block(&mut self, func: &mut Function) {
        if let Some(block) = self.current_block.take() {
            let label = block.label.clone();
            func.add_block(block);
            self.current_block_label = Some(label);
        }
    }

    fn generate_function_body(&mut self, func: &mut Function, body: Vec<Stmt>, span: SourceSpan) {
        self.break_stack.clear();
        self.continue_stack.clear();
        self.start_block(func, format!("entry_{}", func.name).as_str(), span);

        // Add function parameters to symbol table
        for param in &func.parameters {
            let temp = self.new_temp();
            let value = Value::new_local(param.name.clone(), param.ty.clone()).with_debug_info(
                Some(param.name.clone()),
                param.attributes.source_span.clone().unwrap_or_default(),
            );

            self.symbol_table.insert(param.name.clone(), value.clone());
            self.value_types
                .insert(temp.clone().to_string(), param.ty.clone());
        }

        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        if let Some(block) = &self.current_block {
            if matches!(block.terminator.kind, TerminatorKind::Unreachable) {
                let return_value = match func.return_type {
                    IrType::Void => Value::new_literal(IrLiteralValue::I32(0)),
                    _ => Value::new_literal(IrLiteralValue::I32(0)),
                };
                self.add_terminator(
                    func,
                    Terminator::new(
                        TerminatorKind::Return(return_value, func.return_type.clone()),
                        SourceSpan::default(),
                    ),
                );
            }
        }

        self.finalize_current_block(func);

        self.symbol_table.clear();
        self.value_types.clear();
        self.current_function = None;
    }

    fn generate_stmt(&mut self, func: &mut Function, stmt: Stmt) {
        //let span = stmt.span().clone();
        match stmt {
            Stmt::Expression { expr } => {
                self.generate_expr(func, expr);
            }
            Stmt::VarDeclaration { variables, type_annotation, initializers, span, is_mutable } => {
                self.generate_var_declaration(func, variables, type_annotation, initializers, is_mutable, span);
            }
            Stmt::Return { value, span } => {
                self.generate_return(func, value, span);
            }
            Stmt::Block { statements, span: _ } => {
                for stmt in statements {
                    self.generate_stmt(func, stmt);
                }
            }
            Stmt::If { condition, then_branch, else_branch, span } => {
                self.generate_if(func, condition, then_branch, else_branch, span);
            }
            Stmt::While { condition, body, span } => {
                self.generate_while(func, condition, body, span);
            }
            Stmt::For { initializer, condition, increment, body, span } => {
                self.generate_for(func, initializer, condition, increment, body, span);
            }
            Stmt::Break { span } => {
                self.handle_break(func, span);
            }
            Stmt::Continue { span } => {
                self.handle_continue(func, span);
            }
            other => self.new_error(
                "Unsupported statement type".to_string(),
                other.span().clone(),
            ),
        }
    }

    fn generate_var_declaration(
        &mut self,
        func: &mut Function,
        variables: Vec<String>,
        type_annotation: Type,
        initializers: Vec<Expr>,
        is_mutable: bool,
        span: SourceSpan,
    ) {
        let ty: IrType = self.map_type(&type_annotation);

        for (i, var) in variables.iter().enumerate() {
            if is_mutable {
                let temp_id = self.new_temp();
                let temp_name = format!("t{temp_id}");

                let alloca_inst =
                    Instruction::new(InstructionKind::Alloca { ty: ty.clone() }, span.clone())
                        .with_result(Value::new_temporary(temp_id, ty.clone()));

                self.add_instruction(alloca_inst);

                let value = Value::new_local(temp_name.clone(), ty.clone())
                    .with_debug_info(Some(var.clone()), span.clone());

                self.symbol_table.insert(var.clone(), value.clone());
                self.value_types.insert(temp_name, ty.clone());

                if let Some(init) = initializers.get(i) {
                    let value = self.generate_expr(func, init.clone());
                    let store_inst = Instruction::new(
                        InstructionKind::Store {
                            value: value.clone(),
                            dest: value.clone(),
                        },
                        span.clone(),
                    );
                    self.add_instruction(store_inst);
                }
            } else {
                if let Some(init) = initializers.get(i) {
                    let value = self.generate_expr(func, init.clone());
                    self.symbol_table.insert(
                        var.clone(),
                        value.with_debug_info(Some(var.clone()), span.clone()),
                    );
                } else {
                    self.new_error(
                        format!("Constant '{var}' must be initialized"),
                        span.clone(),
                    );
                }
            }
        }
    }

    fn generate_return(&mut self, func: &mut Function, value: Option<Expr>, span: SourceSpan) {
        let return_value = value.map_or_else(
            || Value::new_literal(IrLiteralValue::I32(0)),
            |expr| self.generate_expr(func, expr),
        );

        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::Return(return_value, func.return_type.clone()),
                span,
            ),
        );
    }

    fn generate_if(
        &mut self,
        func: &mut Function,
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        span: SourceSpan,
    ) {
        let cond_value = self.generate_expr(func, condition);

        let then_label = self.new_block_label("then");
        let else_label = self.new_block_label("else");
        let merge_label = self.new_block_label("merge");

        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: then_label.clone(),
                    false_label: else_label.clone(),
                },
                span.clone(),
            ),
        );

        self.start_block(func, &then_label, span.clone());
        for stmt in then_branch {
            self.generate_stmt(func, stmt);
        }

        if self.block_needs_terminator() {
            self.add_terminator(
                func,
                Terminator::new(TerminatorKind::Branch(merge_label.clone()), span.clone()),
            );
        }

        self.start_block(func, &else_label, span.clone());
        if let Some(else_stmts) = else_branch {
            for stmt in else_stmts {
                self.generate_stmt(func, stmt);
            }
        }

        if self.block_needs_terminator() {
            self.add_terminator(
                func,
                Terminator::new(TerminatorKind::Branch(merge_label.clone()), span.clone()),
            );
        }

        self.start_block(func, &merge_label, span);
    }

    fn generate_while(
        &mut self,
        func: &mut Function,
        condition: Expr,
        body: Vec<Stmt>,
        span: SourceSpan,
    ) {
        let loop_start_label = self.new_block_label("loop_start");
        let loop_body_label = self.new_block_label("loop_body");
        let loop_end_label = self.new_block_label("loop_end");

        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::Branch(loop_start_label.clone()),
                span.clone(),
            ),
        );

        self.start_block(func, &loop_start_label, span.clone());
        let cond_value = self.generate_expr(func, condition);
        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: loop_body_label.clone(),
                    false_label: loop_end_label.clone(),
                },
                span.clone(),
            ),
        );

        self.break_stack.push(loop_end_label.clone());
        self.continue_stack.push(loop_start_label.clone());

        self.start_block(func, &loop_body_label, span.clone());
        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        self.break_stack.pop();
        self.continue_stack.pop();

        if self.block_needs_terminator() {
            self.add_terminator(
                func,
                Terminator::new(
                    TerminatorKind::Branch(loop_start_label.clone()),
                    span.clone(),
                ),
            );
        }

        self.start_block(func, &loop_end_label, span);
    }

    fn generate_for(
        &mut self,
        func: &mut Function,
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    ) {
        let loop_st_label = self.new_block_label("for_start");
        let loop_bd_label = self.new_block_label("for_body");
        let loop_inc_label = self.new_block_label("for_inc");
        let loop_end_label = self.new_block_label("for_end");

        if let Some(init) = initializer {
            self.generate_stmt(func, *init);
        }

        self.add_terminator(
            func,
            Terminator::new(TerminatorKind::Branch(loop_st_label.clone()), span.clone()),
        );

        self.start_block(func, &loop_st_label, span.clone());

        let cond_value = if let Some(cond) = condition {
            self.generate_expr(func, cond)
        } else {
            Value::new_literal(IrLiteralValue::Bool(true))
        };

        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: loop_bd_label.clone(),
                    false_label: loop_end_label.clone(),
                },
                span.clone(),
            ),
        );

        self.break_stack.push(loop_end_label.clone());
        self.continue_stack.push(loop_inc_label.clone());

        self.start_block(func, &loop_bd_label, span.clone());
        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        self.break_stack.pop();
        self.continue_stack.pop();

        if self.block_needs_terminator() {
            self.add_terminator(
                func,
                Terminator::new(TerminatorKind::Branch(loop_inc_label.clone()), span.clone()),
            );
        }

        self.start_block(func, &loop_inc_label, span.clone());
        if let Some(inc) = increment {
            self.generate_expr(func, inc);
        }

        if self.block_needs_terminator() {
            self.add_terminator(
                func,
                Terminator::new(TerminatorKind::Branch(loop_st_label.clone()), span.clone()),
            );
        }

        self.start_block(func, &loop_end_label, span);
    }

    fn handle_break(&mut self, func: &mut Function, span: SourceSpan) {
        if let Some(label) = self.break_stack.last() {
            self.add_terminator(
                func,
                Terminator::new(TerminatorKind::Branch(label.clone()), span),
            );
        } else {
            self.new_error("Break outside loop".to_string(), span);
        }
    }

    fn handle_continue(&mut self, func: &mut Function, span: SourceSpan) {
        if let Some(label) = self.continue_stack.last() {
            self.add_terminator(
                func,
                Terminator::new(TerminatorKind::Branch(label.clone()), span),
            );
        } else {
            self.new_error("Continue outside loop".to_string(), span);
        }
    }

    fn generate_expr(&mut self, func: &mut Function, expr: Expr) -> Value {
        //let span = expr.span().clone();
        match expr {
            Expr::Literal { value, span } => self.generate_literal(value, span),
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => self.generate_binary(func, *left, op, *right, span),
            Expr::Unary { op, expr, span } => self.generate_unary(func, op, *expr, span),
            Expr::Variable { name, span } => self.generate_variable(name, span),
            Expr::Assign {
                target,
                value,
                span,
            } => self.generate_assign(func, *target, *value, span),
            Expr::Grouping { expr, span: _ } => self.generate_expr(func, *expr),
            Expr::ArrayLiteral { elements, span } => {
                self.generate_array_literal(func, elements, span)
            }
            other => {
                self.new_error(
                    "Unsupported expression type".to_string(),
                    other.span().clone(),
                );
                Value::new_literal(IrLiteralValue::I32(0))
            }
        }
    }

    fn generate_array_literal(
        &mut self,
        func: &mut Function,
        elements: Vec<Expr>,
        span: SourceSpan,
    ) -> Value {
        if elements.is_empty() {
            return Value::new_literal(IrLiteralValue::I64(0)); // Null pointer
        }

        let mut element_vals = Vec::with_capacity(elements.len());
        for element in elements {
            element_vals.push(self.generate_expr(func, element));
        }

        let element_ty = element_vals[0].ty.clone();
        let array_size = element_vals.len();
        let array_temp = self.new_temp();
        let array_ty = IrType::Array(Box::new(element_ty.clone()), array_size);

        let alloca_inst = Instruction::new(
            InstructionKind::Alloca {
                ty: array_ty.clone(),
            },
            span.clone(),
        )
        .with_result(Value::new_temporary(array_temp, array_ty.clone()));

        self.add_instruction(alloca_inst.clone());
        let array_ptr = alloca_inst.result.unwrap();

        for (index, element_val) in element_vals.into_iter().enumerate() {
            let index_temp = self.new_temp();
            let index_val = Value::new_literal(IrLiteralValue::I32(index as i32));

            let gep_inst = Instruction::new(
                InstructionKind::GetElementPtr {
                    base: array_ptr.clone(),
                    index: index_val,
                    element_ty: element_ty.clone(),
                },
                span.clone(),
            )
            .with_result(Value::new_temporary(
                index_temp,
                IrType::Pointer(Box::new(element_ty.clone())),
            ));
            self.add_instruction(gep_inst.clone());

            let element_ptr = gep_inst.result.unwrap();

            let store_inst = Instruction::new(
                InstructionKind::Store {
                    value: element_val,
                    dest: element_ptr,
                },
                span.clone(),
            );
            self.add_instruction(store_inst);
        }

        array_ptr
    }

    fn generate_literal(&mut self, value: LiteralValue, span: SourceSpan) -> Value {
        match value {
            LiteralValue::Number(num) => match num {
                Number::I8(i) => {
                    Value::new_literal(IrLiteralValue::I8(i)).with_debug_info(None, span)
                }
                Number::I16(i) => {
                    Value::new_literal(IrLiteralValue::I16(i)).with_debug_info(None, span)
                }
                Number::I32(i) => {
                    Value::new_literal(IrLiteralValue::I32(i)).with_debug_info(None, span)
                }
                Number::Integer(i) => {
                    Value::new_literal(IrLiteralValue::I64(i)).with_debug_info(None, span)
                }
                Number::U8(u) => {
                    Value::new_literal(IrLiteralValue::U8(u)).with_debug_info(None, span)
                }
                Number::U16(u) => {
                    Value::new_literal(IrLiteralValue::U16(u)).with_debug_info(None, span)
                }
                Number::U32(u) => {
                    Value::new_literal(IrLiteralValue::U32(u)).with_debug_info(None, span)
                }
                Number::UnsignedInteger(u) => {
                    Value::new_literal(IrLiteralValue::U64(u)).with_debug_info(None, span)
                }
                Number::Float32(f) => {
                    Value::new_literal(IrLiteralValue::F32(f)).with_debug_info(None, span)
                }
                Number::Float64(f) => {
                    Value::new_literal(IrLiteralValue::F64(f)).with_debug_info(None, span)
                }
                Number::Scientific32(f, i) => {
                    let value = f.powi(i);
                    Value::new_literal(IrLiteralValue::F32(value)).with_debug_info(None, span)
                }
                Number::Scientific64(f, i) => {
                    let value = f.powi(i);
                    Value::new_literal(IrLiteralValue::F64(value)).with_debug_info(None, span)
                }
            },
            LiteralValue::Bool(b) => {
                Value::new_literal(IrLiteralValue::Bool(b)).with_debug_info(None, span)
            }
            LiteralValue::StringLit(s) => {
                Value::new_constant(IrConstantValue::String(s), IrType::String)
                    .with_debug_info(None, span)
            }
            LiteralValue::CharLit(c) => {
                Value::new_literal(IrLiteralValue::Char(c.chars().next().unwrap_or('\0')))
                    .with_debug_info(None, span)
            }
            LiteralValue::Nullptr => {
                Value::new_literal(IrLiteralValue::I64(0)).with_debug_info(None, span)
            }
        }
    }

    fn generate_binary(
        &mut self,
        func: &mut Function,
        left: Expr,
        op: BinaryOp,
        right: Expr,
        span: SourceSpan,
    ) -> Value {
        let ir_op: IrBinaryOp = op.into();
        let left_val = self.generate_expr(func, left);
        let right_val = self.generate_expr(func, right);
        let ty = left_val.ty.clone();
        let dest_id = self.new_temp();

        let bin_inst = Instruction::new(
            InstructionKind::Binary {
                op: ir_op,
                left: left_val,
                right: right_val,
                ty: ty.clone(),
            },
            span.clone(),
        )
        .with_result(Value::new_temporary(dest_id, ty.clone()));

        self.add_instruction(bin_inst.clone());
        bin_inst.result.unwrap()
    }

    fn generate_unary(
        &mut self,
        func: &mut Function,
        op: UnaryOp,
        expr: Expr,
        span: SourceSpan,
    ) -> Value {
        let ir_op: IrUnaryOp = op.into();
        let operand = self.generate_expr(func, expr);
        let ty = operand.ty.clone();
        let dest_id = self.new_temp();

        let unary_inst = Instruction::new(
            InstructionKind::Unary {
                op: ir_op,
                operand,
                ty: ty.clone(),
            },
            span.clone(),
        )
        .with_result(Value::new_temporary(dest_id, ty.clone()));

        self.add_instruction(unary_inst.clone());
        unary_inst.result.unwrap()
    }

    fn generate_variable(&mut self, name: String, span: SourceSpan) -> Value {
        self.symbol_table.get(&name).cloned().unwrap_or_else(|| {
            self.new_error(format!("Undefined variable '{name}'"), span.clone());
            Value::new_literal(IrLiteralValue::I32(0)).with_debug_info(None, span)
        })
    }

    fn generate_assign(
        &mut self,
        func: &mut Function,
        target: Expr,
        value: Expr,
        span: SourceSpan,
    ) -> Value {
        let target_val = self.generate_expr(func, target);
        let value_val = self.generate_expr(func, value);

        let store_inst = Instruction::new(
            InstructionKind::Store {
                value: value_val.clone(),
                dest: target_val,
            },
            span.clone(),
        );
        self.add_instruction(store_inst);

        value_val
    }

    // Helper methods
    fn new_temp(&mut self) -> u64 {
        let id = self.temp_counter;
        self.temp_counter += 1;
        id
    }

    fn new_block_label(&mut self, prefix: &str) -> String {
        self.block_counter += 1;
        format!("{}_{}", prefix, self.block_counter)
    }

    fn start_block(&mut self, func: &mut Function, label: &str, span: SourceSpan) {
        self.finalize_current_block(func);

        let mut new_block = BasicBlock::new(label, span);

        // Preserve predecessors from CFG
        if let Some(preds) = func.cfg.predecessors.get(label) {
            for pred in preds {
                new_block.add_predecessor(pred.clone());
            }
        }

        self.current_block = Some(new_block);
        self.current_block_label = Some(label.to_string());
    }

    fn add_instruction(&mut self, inst: Instruction) {
        if let Some(block) = &mut self.current_block {
            block.instructions.push(inst);
        }
    }

    fn add_terminator(&mut self, func: &mut Function, term: Terminator) {
        if let Some(block) = &mut self.current_block {
            block.terminator = term.clone();

            // Add CFG edges
            if let Some(current_label) = &self.current_block_label {
                for target in term.get_targets() {
                    func.add_edge(current_label, &target);
                }
            }
        }
    }
}

impl Default for NIrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
