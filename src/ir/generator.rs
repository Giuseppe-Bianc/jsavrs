// src/ir/generator.rs
use super::*;
use crate::error::compile_error::CompileError;
use crate::ir::instruction::{IrBinaryOp, IrUnaryOp};
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::tokens::number::Number;
use std::collections::HashMap;

/// Generates IR from AST
pub struct IrGenerator {
    current_block: Option<BasicBlock>,
    symbol_table: HashMap<String, Value>,
    temp_counter: usize,
    block_counter: usize,
    errors: Vec<CompileError>,
    value_types: HashMap<String, IrType>,
    break_stack: Vec<String>,    // Stack of loop end labels
    continue_stack: Vec<String>, // Stack of loop continue targets
}

#[allow(clippy::collapsible_if, clippy::only_used_in_recursion)]
impl IrGenerator {
    pub fn new() -> Self {
        Self {
            current_block: None,
            symbol_table: HashMap::new(),
            temp_counter: 0,
            block_counter: 0,
            errors: Vec::new(),
            value_types: HashMap::new(),
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
        }
    }

    fn block_needs_terminator(&self) -> bool {
        self.current_block
            .as_ref()
            .is_some_and(|b| !b.terminator.is_terminator())
    }
    /// Generate IR from AST statements
    pub fn generate(&mut self, stmts: Vec<Stmt>) -> (Vec<Function>, Vec<CompileError>) {
        let mut functions = Vec::new();

        for stmt in stmts {
            match stmt {
                Stmt::Function {
                    name,
                    parameters,
                    return_type,
                    body,
                    span: _,
                } => {
                    let mut func = self.create_function(&name, &parameters, return_type);
                    self.generate_function_body(&mut func, body);
                    functions.push(func);
                }
                Stmt::MainFunction { body, span: _ } => {
                    let mut func = self.create_function("main", &[], Type::Void);
                    self.generate_function_body(&mut func, body);
                    functions.push(func);
                }
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

    fn new_error(&mut self, message: String, span: SourceSpan) {
        self.errors
            .push(CompileError::IrGeneratorError { message, span });
    }

    fn create_function(&mut self, name: &str, params: &[Parameter], return_type: Type) -> Function {
        let ir_params = params
            .iter()
            .map(|param| {
                let ty = self.map_type(&param.type_annotation);
                (param.name.clone(), ty)
            })
            .collect();

        let ir_return_type = self.map_type(&return_type);

        Function::new(name, ir_params, ir_return_type)
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
            Type::Custom(name) => IrType::Custom(name.clone()),
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

    fn generate_function_body(&mut self, func: &mut Function, body: Vec<Stmt>) {
        // Clear loop stacks at start of function
        self.break_stack.clear();
        self.continue_stack.clear();
        self.start_block(func, "entry");

        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        // Add return if missing
        if let Some(block) = &self.current_block {
            if matches!(block.terminator, Terminator::Unreachable) {
                let return_value = match func.return_type {
                    IrType::Void => Value::new_immediate(ImmediateValue::I32(0)),
                    _ => Value::new_immediate(ImmediateValue::I32(0)),
                };
                self.add_terminator(Terminator::Return(return_value, func.return_type.clone()));
            }
        }

        if let Some(block) = self.current_block.take() {
            if block.terminator.is_terminator() || !block.instructions.is_empty() {
                func.add_block(block);
            }
        }

        self.symbol_table.clear();
        self.value_types.clear();
    }

    fn generate_stmt(&mut self, func: &mut Function, stmt: Stmt) {
        match stmt {
            Stmt::Expression { expr } => {
                self.generate_expr(func, expr);
            }
            Stmt::VarDeclaration {
                variables,
                type_annotation,
                initializers,
                span,
                is_mutable,
            } => {
                self.generate_var_declaration(
                    func,
                    variables,
                    type_annotation,
                    initializers,
                    is_mutable,
                    span
                );
            }
            Stmt::Return { value, span: _ } => {
                self.generate_return(func, value);
            }
            Stmt::Block {
                statements,
                span: _,
            } => {
                for stmt in statements {
                    self.generate_stmt(func, stmt);
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                span: _,
            } => {
                self.generate_if(func, condition, then_branch, else_branch);
            }
            Stmt::While {
                condition,
                body,
                span: _,
            } => {
                self.generate_while(func, condition, body);
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                span: _,
            } => {
                self.generate_for(func, initializer, condition, increment, body);
            }
            Stmt::Break { span } => {
                self.handle_break(span);
            }
            Stmt::Continue { span } => {
                self.handle_continue(span);
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
                // Variabile mutabile: comportamento originale
                let temp = self.new_temp();
                self.add_instruction(Instruction::Alloca {
                    dest: temp.clone(),
                    ty: ty.clone(),
                });

                let value = Value::new_local(temp.clone(), ty.clone());
                self.symbol_table.insert(var.clone(), value.clone());
                self.value_types.insert(temp.clone(), ty.clone());

                if let Some(init) = initializers.get(i) {
                    let value = self.generate_expr(func, init.clone());
                    self.add_instruction(Instruction::Store {
                        value,
                        dest: Value::new_local(temp.clone(), ty.clone()),
                    });
                }
            } else {
                // COSTANTE: nessuna allocazione, solo binding del valore
                if let Some(init) = initializers.get(i) {
                    let value = self.generate_expr(func, init.clone());
                    self.symbol_table.insert(var.clone(), value);
                } else {
                    // Gestione errore: costante non inizializzata
                    self.new_error(
                        format!("Constant '{var}' must be initialized"),
                        span.clone(), // Sostituisci con span appropriato
                    );
                }
            }
        }
    }

    fn generate_return(&mut self, func: &mut Function, value: Option<Expr>) {
        let return_value = value.map_or_else(
            || Value::new_immediate(ImmediateValue::I32(0)),
            |expr| self.generate_expr(func, expr),
        );

        self.add_terminator(Terminator::Return(return_value, func.return_type.clone()));
    }

    fn generate_if(
        &mut self,
        func: &mut Function,
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    ) {
        let cond_value = self.generate_expr(func, condition);

        let then_label = self.new_block_label("then");
        let else_label = self.new_block_label("else");
        let merge_label = self.new_block_label("merge");

        // Conditional branch
        self.add_terminator(Terminator::ConditionalBranch {
            condition: cond_value,
            true_label: then_label.clone(),
            false_label: else_label.clone(),
        });

        // End current block and start then block
        self.start_block(func, &then_label);
        for stmt in then_branch {
            self.generate_stmt(func, stmt);
        }

        // Add branch to merge if no terminator
        if self.block_needs_terminator() {
            self.add_terminator(Terminator::Branch(merge_label.clone()));
        }

        // Start else block
        self.start_block(func, &else_label);
        if let Some(else_stmts) = else_branch {
            for stmt in else_stmts {
                self.generate_stmt(func, stmt);
            }
        }

        // Add branch to merge if no terminator
        if self.block_needs_terminator() {
            self.add_terminator(Terminator::Branch(merge_label.clone()));
        }

        // Start merge block
        self.start_block(func, &merge_label);
    }

    fn generate_while(
        &mut self,
        func: &mut Function,
        condition: Expr,
        body: Vec<Stmt>,
        /*span: SourceSpan,*/
    ) {
        let loop_start_label = self.new_block_label("loop_start");
        let loop_body_label = self.new_block_label("loop_body");
        let loop_end_label = self.new_block_label("loop_end");

        // Branch to loop start
        self.add_terminator(Terminator::Branch(loop_start_label.clone()));

        // Loop start block (condition evaluation)
        self.start_block(func, &loop_start_label);
        let cond_value = self.generate_expr(func, condition);
        self.add_terminator(Terminator::ConditionalBranch {
            condition: cond_value,
            true_label: loop_body_label.clone(),
            false_label: loop_end_label.clone(),
        });

        // Push loop context
        self.break_stack.push(loop_end_label.clone());
        self.continue_stack.push(loop_start_label.clone());

        // Loop body block
        self.start_block(func, &loop_body_label);
        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        // Pop loop context
        self.break_stack.pop();
        self.continue_stack.pop();

        // After body, branch back to condition
        if self.block_needs_terminator() {
            self.add_terminator(Terminator::Branch(loop_start_label.clone()));
        }

        // Loop end block
        self.start_block(func, &loop_end_label);
    }

    fn generate_for(
        &mut self,
        func: &mut Function,
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Vec<Stmt>,
        /*span: SourceSpan,*/
    ) {
        let loop_start_label = self.new_block_label("for_start");
        //let loop_cond_label = self.new_block_label("for_cond");
        let loop_body_label = self.new_block_label("for_body");
        let loop_inc_label = self.new_block_label("for_inc");
        let loop_end_label = self.new_block_label("for_end");

        // Generate initializer in current block
        if let Some(init) = initializer {
            self.generate_stmt(func, *init);
        }

        // Branch to condition block
        self.add_terminator(Terminator::Branch(loop_start_label.clone()));

        // Start block (for condition evaluation)
        self.start_block(func, &loop_start_label);

        // Generate condition if present, otherwise default to true
        let cond_value = if let Some(cond) = condition {
            self.generate_expr(func, cond)
        } else {
            Value::new_immediate(ImmediateValue::Bool(true))
        };

        self.add_terminator(Terminator::ConditionalBranch {
            condition: cond_value,
            true_label: loop_body_label.clone(),
            false_label: loop_end_label.clone(),
        });

        // Push loop context
        self.break_stack.push(loop_end_label.clone());
        self.continue_stack.push(loop_inc_label.clone());

        // Body block
        self.start_block(func, &loop_body_label);
        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        // Pop loop context
        self.break_stack.pop();
        self.continue_stack.pop();

        // After body, branch to increment block
        if self.block_needs_terminator() {
            self.add_terminator(Terminator::Branch(loop_inc_label.clone()));
        }

        // Increment block
        self.start_block(func, &loop_inc_label);
        if let Some(inc) = increment {
            self.generate_expr(func, inc);
        }

        // After increment, branch back to condition
        if self.block_needs_terminator() {
            self.add_terminator(Terminator::Branch(loop_start_label.clone()));
        }

        // End block
        self.start_block(func, &loop_end_label);
    }

    // Break/Continue handlers
    fn handle_break(&mut self, span: SourceSpan) {
        if let Some(label) = self.break_stack.last() {
            self.add_terminator(Terminator::Branch(label.clone()));
        } else {
            self.new_error("Break outside loop".to_string(), span);
        }
    }

    fn handle_continue(&mut self, span: SourceSpan) {
        if let Some(label) = self.continue_stack.last() {
            self.add_terminator(Terminator::Branch(label.clone()));
        } else {
            self.new_error("Continue outside loop".to_string(), span);
        }
    }

    fn generate_expr(&mut self, func: &mut Function, expr: Expr) -> Value {
        //let span = expr.span().clone();
        match expr {
            Expr::Literal { value, .. } => self.generate_literal(value),
            Expr::Binary {
                left, op, right, ..
            } => self.generate_binary(func, *left, op, *right),
            Expr::Unary { op, expr, .. } => self.generate_unary(func, op, *expr),
            Expr::Variable { name, .. } => self.generate_variable(name),
            Expr::Assign { target, value, .. } => self.generate_assign(func, *target, *value),
            Expr::Grouping { expr, .. } => self.generate_expr(func, *expr),
            Expr::ArrayLiteral { elements, .. } => self.generate_array_literal(func, elements),
            other => {
                self.new_error(
                    "Unsupported expression type".to_string(),
                    other.span().clone(),
                );
                Value::new_immediate(ImmediateValue::I32(0))
            }
        }
    }

    fn generate_array_literal(
        &mut self,
        func: &mut Function,
        elements: Vec<Expr>,
        /*span: SourceSpan,*/
    ) -> Value {
        if elements.is_empty() {
            return Value::new_immediate(ImmediateValue::I64(0)); // Null pointer
        }

        // Generate all element values first
        let mut element_vals = Vec::with_capacity(elements.len());
        for element in elements {
            element_vals.push(self.generate_expr(func, element));
        }

        // Determine element type from first element
        let element_ty = element_vals[0].ty.clone();
        let array_size = element_vals.len();

        // Allocate array
        let array_temp = self.new_temp();
        let array_ty = IrType::Array(Box::new(element_ty.clone()), array_size);
        self.add_instruction(Instruction::Alloca {
            dest: array_temp.clone(),
            ty: array_ty.clone(),
        });
        let array_ptr = Value::new_local(array_temp.clone(), array_ty.clone());

        // Store each element
        for (index, element_val) in element_vals.into_iter().enumerate() {
            // Calculate element pointer
            let index_temp = self.new_temp();
            let index_val = Value::new_immediate(ImmediateValue::I32(index as i32));
            self.add_instruction(Instruction::GetElementPtr {
                dest: index_temp.clone(),
                base: array_ptr.clone(),
                index: index_val,
                element_ty: element_ty.clone(),
            });

            let element_ptr =
                Value::new_temporary(index_temp, IrType::Pointer(Box::new(element_ty.clone())));
            self.add_instruction(Instruction::Store {
                value: element_val,
                dest: element_ptr,
            });
        }

        array_ptr
    }

    fn generate_literal(&mut self, value: LiteralValue) -> Value {
        match value {
            LiteralValue::Number(num) => match num {
                Number::I8(i) => Value::new_immediate(ImmediateValue::I8(i)),
                Number::I16(i) => Value::new_immediate(ImmediateValue::I16(i)),
                Number::I32(i) => Value::new_immediate(ImmediateValue::I32(i)),
                Number::Integer(i) => Value::new_immediate(ImmediateValue::I64(i)),
                Number::U8(u) => Value::new_immediate(ImmediateValue::U8(u)),
                Number::U16(u) => Value::new_immediate(ImmediateValue::U16(u)),
                Number::U32(u) => Value::new_immediate(ImmediateValue::U32(u)),
                Number::UnsignedInteger(u) => Value::new_immediate(ImmediateValue::U64(u)),
                Number::Float32(f) => Value::new_immediate(ImmediateValue::F32(f)),
                Number::Float64(f) => Value::new_immediate(ImmediateValue::F64(f)),
                Number::Scientific32(f, i) => {
                    let value = f.powi(i);
                    Value::new_immediate(ImmediateValue::F32(value))
                }
                Number::Scientific64(f, i) => {
                    let value = f.powi(i);
                    Value::new_immediate(ImmediateValue::F64(value))
                } //_ => Value::new_immediate(ImmediateValue::I32(0)),
            },
            LiteralValue::Bool(b) => Value::new_immediate(ImmediateValue::Bool(b)),
            LiteralValue::StringLit(s) => Value::new_immediate(ImmediateValue::String(s)),
            LiteralValue::CharLit(c) => {
                Value::new_immediate(ImmediateValue::Char(c.chars().next().unwrap_or('\0')))
            }
            LiteralValue::Nullptr => Value::new_immediate(ImmediateValue::I64(0)),
        }
    }

    fn generate_binary(
        &mut self,
        func: &mut Function,
        left: Expr,
        op: BinaryOp,
        right: Expr,
    ) -> Value {
        let ir_op: IrBinaryOp = op.into();

        let left_val = self.generate_expr(func, left);
        let right_val = self.generate_expr(func, right);
        let ty = left_val.ty.clone();
        let dest = self.new_temp();

        self.add_instruction(Instruction::Binary {
            op: ir_op,
            dest: dest.clone(),
            left: left_val,
            right: right_val,
            ty: ty.clone(),
        });

        Value::new_temporary(dest, ty)
    }

    fn generate_unary(&mut self, func: &mut Function, op: UnaryOp, expr: Expr) -> Value {
        let ir_op: IrUnaryOp = op.into();

        let operand = self.generate_expr(func, expr);
        let ty = operand.ty.clone();
        let dest = self.new_temp();

        self.add_instruction(Instruction::Unary {
            op: ir_op,
            dest: dest.clone(),
            operand,
            ty: ty.clone(),
        });

        Value::new_temporary(dest, ty)
    }

    fn generate_variable(&mut self, name: String) -> Value {
        self.symbol_table
            .get(&name)
            .cloned()
            .unwrap_or_else(|| Value::new_immediate(ImmediateValue::I32(0)))
    }

    fn generate_assign(&mut self, func: &mut Function, target: Expr, value: Expr) -> Value {
        let target_val = self.generate_expr(func, target);
        let value_val = self.generate_expr(func, value);

        self.add_instruction(Instruction::Store {
            value: value_val.clone(),
            dest: target_val,
        });

        value_val
    }

    // Helper methods
    fn new_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("t{}", self.temp_counter)
    }

    fn new_block_label(&mut self, prefix: &str) -> String {
        self.block_counter += 1;
        format!("{}_{}", prefix, self.block_counter)
    }

    fn start_block(&mut self, func: &mut Function, label: &str) {
        if let Some(block) = self.current_block.take() {
            if !block.instructions.is_empty() || block.terminator.is_terminator() {
                func.add_block(block);
            }
        }
        self.current_block = Some(BasicBlock::new(label));
    }

    fn add_instruction(&mut self, inst: Instruction) {
        if let Some(block) = &mut self.current_block {
            block.instructions.push(inst);
        }
    }

    fn add_terminator(&mut self, term: Terminator) {
        if let Some(block) = &mut self.current_block {
            block.terminator = term;
        }
    }
}

impl Default for IrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
