// src/ir/generator.rs
use super::ssa::SsaTransformer;
use super::*;
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::tokens::number::Number;
use std::collections::HashMap;
use std::sync::Arc;
#[repr(u8)]
enum LoopControl {
    Break,
    Continue,
}

pub struct NIrGenerator {
    current_block: Option<BasicBlock>,
    current_block_label: Option<String>,
    scope_manager: ScopeManager,
    temp_counter: u64,
    block_counter: usize,
    errors: Vec<CompileError>,
    break_stack: Vec<String>,
    continue_stack: Vec<String>,
    type_context: TypeContext,
    _access_controller: AccessController,
    root_scope: Option<ScopeId>,
    /// Whether to apply SSA transformation to generated IR
    apply_ssa: bool,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct TypeContext {
    structs: HashMap<String, (Vec<(String, IrType)>, SourceSpan)>,
    aliases: HashMap<String, IrType>,
}

#[allow(clippy::collapsible_if, clippy::collapsible_else_if)]
impl NIrGenerator {
    pub fn new() -> Self {
        let scope_manager = ScopeManager::new();
        let access_controller = AccessController::new(&scope_manager);
        Self {
            current_block: None,
            current_block_label: None,
            scope_manager: scope_manager.clone(),
            temp_counter: 0,
            block_counter: 0,
            errors: Vec::new(),
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            _access_controller: access_controller,
            type_context: TypeContext::default(),
            root_scope: scope_manager.root_scope(),
            apply_ssa: true, // Enable SSA by default
        }
    }

    /// Creates a new generator with SSA transformation disabled
    pub fn new_without_ssa() -> Self {
        let mut generator = Self::new();
        generator.apply_ssa = false;
        generator
    }

    fn block_needs_terminator(&self) -> bool {
        self.current_block.as_ref().is_some_and(|b| !b.terminator.is_terminator())
    }

    pub fn generate(&mut self, stmts: Vec<Stmt>, module_name: &str) -> (Module, Vec<CompileError>) {
        //let mut functions = Vec::new();
        let mut module = Module::new(module_name, self.root_scope);

        // First pass: create all functions and add them to the symbol table
        for stmt in &stmts {
            match stmt {
                Stmt::Function { name, parameters: _, return_type, body: _, span } => {
                    let ir_return_type = self.map_type(return_type);
                    let func_ptr_type = IrType::Pointer(Box::new(ir_return_type));
                    let func_value = Value::new_global(name.clone(), func_ptr_type)
                        .with_debug_info(Some(name.clone()), span.clone());
                    self.scope_manager.add_symbol(name.as_ref(), func_value);
                }
                Stmt::MainFunction { body: _, span } => {
                    let ir_return_type = IrType::Void;
                    let func_ptr_type = IrType::Pointer(Box::new(ir_return_type));
                    let func_value = Value::new_global("main".into(), func_ptr_type)
                        .with_debug_info(Some("main".into()), span.clone());
                    self.scope_manager.add_symbol("main", func_value);
                }
                _ => {}
            }
        }

        // Second pass: generate function bodies
        for stmt in stmts {
            match stmt {
                Stmt::Function { name, parameters, return_type, body, span } => {
                    let mut func = self.create_function(&name, &parameters, return_type, span.clone());
                    self.generate_function_body(&mut func, body, span);
                    module.add_function(func);
                }
                Stmt::MainFunction { body, span } => {
                    let mut func = self.create_function("main", &[], Type::Void, span.clone());
                    self.generate_function_body(&mut func, body, span);
                    module.add_function(func);
                }
                other => {
                    self.new_error("Unsupported top-level statement".to_string(), other.span().clone());
                }
            }
        }

        // Apply SSA transformation to all functions in the module if enabled
        if self.apply_ssa {
            self.apply_ssa_transformation(&mut module);
        }

        (module, std::mem::take(&mut self.errors))
    }

    /// Applies SSA transformation to all functions in the module.
    fn apply_ssa_transformation(&mut self, module: &mut Module) {
        // Use a single transformer for all functions to ensure unique temporary IDs
        let mut transformer = SsaTransformer::new(Some(self.temp_counter));
        // Transform each function in the module
        for func in &mut module.functions {
            if let Err(e) = transformer.transform_function(func) {
                self.new_error(format!("SSA transformation failed: {}", e), SourceSpan::default());
            }
        }
    }

    fn new_error(&mut self, message: String, span: SourceSpan) {
        self.errors.push(CompileError::IrGeneratorError { message, span, help: None });
    }

    fn add_branch_if_needed(&mut self, func: &mut Function, target_label: &str, span: SourceSpan) {
        if self.block_needs_terminator() {
            self.add_terminator(func, Terminator::new(TerminatorKind::Branch { label: target_label.into() }, span));
        }
    }

    fn create_function(&mut self, name: &str, params: &[Parameter], return_type: Type, span: SourceSpan) -> Function {
        let ir_params = params
            .iter()
            .map(|param| {
                let ty = self.map_type(&param.type_annotation);
                IrParameter {
                    name: param.name.clone(),
                    ty: ty.clone(),
                    attributes: ParamAttributes { source_span: Some(param.span.clone()), ..Default::default() },
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
                if let Some((fields, span)) = self.type_context.structs.get(name.as_ref()) {
                    IrType::Struct(name.clone(), fields.clone(), span.clone())
                } else {
                    IrType::Custom(name.clone(), SourceSpan::default())
                }
            }
            Type::Array(element_type, size_expr) => {
                if let Expr::Literal { value: LiteralValue::Number(Number::Integer(size)), .. } = **size_expr {
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
        if let Some(mut current_block) = self.current_block.take() {
            let label = current_block.label.clone();

            // Get the block in the CFG and update its instructions and terminator
            if let Some(cfg_block) = func.cfg.get_block_mut(&label) {
                // Transfer instructions and terminator to the CFG block
                cfg_block.instructions = std::mem::take(&mut current_block.instructions);
                cfg_block.terminator = current_block.terminator.clone();
                cfg_block.scope = current_block.scope;
            }

            self.current_block_label = Some(label.to_string());
        }
    }

    // Handle block connections at the end
    fn finalize_block_connections(&mut self, func: &mut Function) {
        // First, collect all the connections we need to make
        let mut connections = Vec::new();
        for block in func.cfg.blocks() {
            let label = block.label.clone();
            for target in block.terminator.get_targets() {
                connections.push((label.clone(), target));
            }
        }

        // Now apply all the connections without holding the immutable borrow
        for (from_label, to_label) in connections {
            func.connect_blocks(&from_label, &to_label);
        }
    }
    fn generate_function_body(&mut self, func: &mut Function, body: Vec<Stmt>, span: SourceSpan) {
        self.break_stack.clear();
        self.continue_stack.clear();

        // Save the current generator scope manager
        let saved_scope_manager = self.scope_manager.clone();

        // Establish function scope before creating the entry block
        func.enter_scope();

        // Create the entry block using start_block
        let entry_label = format!("entry_{}", func.name);
        self.start_block(func, &entry_label, span.clone());

        // Add function parameters to symbol table
        for param in &func.parameters {
            let value = Value::new_local(param.name.clone(), param.ty.clone())
                .with_debug_info(Some(param.name.clone()), param.attributes.source_span.clone().unwrap_or_default());
            self.scope_manager.add_symbol(param.name.clone(), value.clone());
        }

        // Process all statements
        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        // Ensure the last block has a terminator if needed
        if let Some(block) = &self.current_block {
            if matches!(block.terminator().kind, TerminatorKind::Unreachable) {
                let return_value = match func.return_type {
                    IrType::Void => Value::new_literal(IrLiteralValue::I32(0)),
                    _ => Value::new_literal(IrLiteralValue::I32(0)),
                };
                self.add_terminator(
                    func,
                    Terminator::new(
                        TerminatorKind::Return { value: return_value, ty: func.return_type.clone() },
                        SourceSpan::default(),
                    ),
                );
            }
        }

        // Finalize the last block
        self.finalize_current_block(func);

        // Now connect all blocks
        self.finalize_block_connections(func);

        // Update the function's scope manager with the scopes we created during generation
        func.scope_manager = self.scope_manager.clone();

        // Restore the generator's scope manager and append the function's scope manager
        self.scope_manager = saved_scope_manager;

        func.exit_scope();
        self.scope_manager.append_manager(&func.scope_manager);
    }

    fn generate_stmt(&mut self, func: &mut Function, stmt: Stmt) {
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
                self.scope_manager.enter_scope();
                for stmt in statements {
                    self.generate_stmt(func, stmt);
                }
                self.scope_manager.exit_scope();
            }
            Stmt::If { condition, then_branch, else_branch, span } => {
                self.generate_if(func, condition, then_branch, else_branch, span);
            }
            Stmt::While { condition, body, span } => {
                self.generate_while(func, condition, body, span);
            }
            Stmt::For { initializer, condition, increment, body, span } => {
                self.scope_manager.enter_scope();
                self.generate_for(func, initializer, condition, increment, body, span);
                self.scope_manager.exit_scope();
            }
            Stmt::Break { span } => {
                self.handle_loop_control(func, span, LoopControl::Break);
            }
            Stmt::Continue { span } => {
                self.handle_loop_control(func, span, LoopControl::Continue);
            }
            other => self.new_error(format!("Unsupported statement: {:?}", other), other.span().clone()),
        }
    }

    fn generate_var_declaration(
        &mut self, func: &mut Function, variables: Vec<Arc<str>>, type_annotation: Type, initializers: Vec<Expr>,
        is_mutable: bool, span: SourceSpan,
    ) {
        let ty: IrType = self.map_type(&type_annotation);

        for (i, var) in variables.iter().enumerate() {
            if is_mutable {
                let temp_id = self.new_temp();
                let ptr_ty = IrType::Pointer(Box::new(ty.clone()));
                let ptr_value = Value::new_temporary(temp_id, ptr_ty).with_debug_info(Some(var.clone()), span.clone());

                let alloca_inst = Instruction::new(InstructionKind::Alloca { ty: ty.clone() }, span.clone())
                    .with_result(ptr_value.clone());

                self.add_instruction(alloca_inst);

                if let Some(init) = initializers.get(i) {
                    let value_val = self.generate_expr(func, init.clone());
                    let store_inst = Instruction::new(
                        InstructionKind::Store { value: value_val, dest: ptr_value.clone() },
                        span.clone(),
                    );
                    self.add_instruction(store_inst);
                }

                self.scope_manager.add_symbol(var.clone(), ptr_value);
            } else {
                if let Some(init) = initializers.get(i) {
                    let value = self.generate_expr(func, init.clone());
                    self.scope_manager.add_symbol(var.clone(), value.with_debug_info(Some(var.clone()), span.clone()));
                } else {
                    self.new_error(format!("Constant '{var}' must be initialized"), span.clone());
                }
            }
        }
    }

    fn generate_return(&mut self, func: &mut Function, value: Option<Expr>, span: SourceSpan) {
        let return_value =
            value.map_or_else(|| Value::new_literal(IrLiteralValue::I32(0)), |expr| self.generate_expr(func, expr));

        self.add_terminator(
            func,
            Terminator::new(TerminatorKind::Return { value: return_value, ty: func.return_type.clone() }, span),
        );
    }

    fn generate_if(
        &mut self, func: &mut Function, condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>>,
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
                    true_label: then_label.clone().into(),
                    false_label: else_label.clone().into(),
                },
                span.clone(),
            ),
        );

        self.start_block(func, &then_label, span.clone());
        self.scope_manager.enter_scope();
        for stmt in then_branch {
            self.generate_stmt(func, stmt);
        }
        self.scope_manager.exit_scope();
        self.add_branch_if_needed(func, &merge_label, span.clone());

        self.start_block(func, &else_label, span.clone());
        if let Some(else_stmts) = else_branch {
            self.scope_manager.enter_scope();
            for stmt in else_stmts {
                self.generate_stmt(func, stmt);
            }
            self.scope_manager.exit_scope();
        }

        self.add_branch_if_needed(func, &merge_label, span.clone());

        self.start_block(func, &merge_label, span);
    }

    fn generate_while(&mut self, func: &mut Function, condition: Expr, body: Vec<Stmt>, span: SourceSpan) {
        let loop_start_label = self.new_block_label("loop_start");
        let loop_body_label = self.new_block_label("loop_body");
        let loop_end_label = self.new_block_label("loop_end");

        self.add_terminator(
            func,
            Terminator::new(TerminatorKind::Branch { label: loop_start_label.clone().into() }, span.clone()),
        );

        self.start_block(func, &loop_start_label, span.clone());
        let cond_value = self.generate_expr(func, condition);
        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: loop_body_label.clone().into(),
                    false_label: loop_end_label.clone().into(),
                },
                span.clone(),
            ),
        );

        self.break_stack.push(loop_end_label.clone());
        self.continue_stack.push(loop_start_label.clone());

        self.start_block(func, &loop_body_label, span.clone());
        self.scope_manager.enter_scope();
        for stmt in body {
            self.generate_stmt(func, stmt);
        }
        self.scope_manager.exit_scope();

        self.break_stack.pop();
        self.continue_stack.pop();

        self.add_branch_if_needed(func, &loop_start_label, span.clone());
        self.start_block(func, &loop_end_label, span);
    }

    fn generate_for(
        &mut self, func: &mut Function, initializer: Option<Box<Stmt>>, condition: Option<Expr>,
        increment: Option<Expr>, body: Vec<Stmt>, span: SourceSpan,
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
            Terminator::new(TerminatorKind::Branch { label: loop_st_label.clone().into() }, span.clone()),
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
                    true_label: loop_bd_label.clone().into(),
                    false_label: loop_end_label.clone().into(),
                },
                span.clone(),
            ),
        );

        self.break_stack.push(loop_end_label.clone());
        self.continue_stack.push(loop_inc_label.clone());

        self.start_block(func, &loop_bd_label, span.clone());
        self.scope_manager.enter_scope();
        for stmt in body {
            self.generate_stmt(func, stmt);
        }
        self.scope_manager.exit_scope();
        self.break_stack.pop();
        self.continue_stack.pop();

        self.add_branch_if_needed(func, &loop_inc_label, span.clone());

        self.start_block(func, &loop_inc_label, span.clone());
        if let Some(inc) = increment {
            self.generate_expr(func, inc);
        }

        self.add_branch_if_needed(func, &loop_st_label, span.clone());
        self.start_block(func, &loop_end_label, span);
    }

    fn handle_loop_control(&mut self, func: &mut Function, span: SourceSpan, control: LoopControl) {
        let (stack, message) = match control {
            LoopControl::Break => (&self.break_stack, "Break outside loop"),
            LoopControl::Continue => (&self.continue_stack, "Continue outside loop"),
        };

        if let Some(label) = stack.last() {
            self.add_terminator(func, Terminator::new(TerminatorKind::Branch { label: label.clone().into() }, span));
        } else {
            self.new_error(message.to_string(), span);
        }
    }

    #[allow(unreachable_patterns)] // To handle any unexpected Expr variants 
    fn generate_expr(&mut self, func: &mut Function, expr: Expr) -> Value {
        match expr {
            Expr::Literal { value, span } => self.generate_literal(value, span),
            Expr::Binary { left, op, right, span } => self.generate_binary(func, *left, op, *right, span),
            Expr::Unary { op, expr, span } => self.generate_unary(func, op, *expr, span),
            Expr::Variable { name, span } => self.generate_variable(name, span),
            Expr::Assign { target, value, span } => self.generate_assign(func, *target, *value, span),
            Expr::Grouping { expr, span: _ } => self.generate_expr(func, *expr),
            Expr::ArrayLiteral { elements, span } => self.generate_array_literal(func, elements, span),
            Expr::ArrayAccess { array, index, span } => self.generate_array_access(func, *array, *index, span),
            Expr::Call { callee, arguments, span } => self.generate_call(func, *callee, arguments, span),
            other => {
                self.new_error("Unsupported expression type".to_string(), other.span().clone());
                Value::new_literal(IrLiteralValue::I32(0))
            }
        }
    }

    /// Generates array access: calculates the element address with GEP and returns a pointer to the element.
    fn generate_array_access(&mut self, func: &mut Function, array: Expr, index: Expr, span: SourceSpan) -> Value {
        let base_val = self.generate_expr(func, array);
        let index_val = self.generate_expr(func, index);

        // Determine element type: handle both pointer to array and direct array
        let element_ty = match &base_val.ty {
            IrType::Pointer(inner) => match inner.as_ref() {
                IrType::Array(elem_ty, _) => *elem_ty.clone(),
                other => other.clone(), // fallback: pointer to already pointed element
            },
            IrType::Array(elem_ty, _) => *elem_ty.clone(),
            other => {
                // Unexpected case: report but continue with safe fallback (i32)
                self.new_error(format!("Array access on non-array type: {other}"), span.clone());
                IrType::I32
            }
        };

        let tmp = self.new_temp();
        let gep = Instruction::new(
            InstructionKind::GetElementPtr { base: base_val, index: index_val, element_ty: element_ty.clone() },
            span.clone(),
        )
        .with_result(Value::new_temporary(tmp, IrType::Pointer(Box::new(element_ty.clone()))));

        self.add_instruction(gep.clone());
        let ptr_value = gep.result.unwrap();

        // Load the value from the pointer for use in expressions
        let load_tmp = self.new_temp();
        let load_inst =
            Instruction::new(InstructionKind::Load { src: ptr_value, ty: element_ty.clone() }, span.clone())
                .with_result(Value::new_temporary(load_tmp, element_ty));

        self.add_instruction(load_inst.clone());
        load_inst.result.unwrap()
    }

    fn generate_array_literal(&mut self, func: &mut Function, elements: Vec<Expr>, span: SourceSpan) -> Value {
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

        let alloca_inst = Instruction::new(InstructionKind::Alloca { ty: array_ty.clone() }, span.clone())
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
            .with_result(Value::new_temporary(index_temp, IrType::Pointer(Box::new(element_ty.clone()))));
            self.add_instruction(gep_inst.clone());

            let element_ptr = gep_inst.result.unwrap();

            let store_inst =
                Instruction::new(InstructionKind::Store { value: element_val, dest: element_ptr }, span.clone());
            self.add_instruction(store_inst);
        }

        array_ptr
    }

    fn generate_literal(&mut self, value: LiteralValue, span: SourceSpan) -> Value {
        match value {
            LiteralValue::Number(num) => match num {
                Number::I8(i) => Value::new_literal(IrLiteralValue::I8(i)).with_debug_info(None, span),
                Number::I16(i) => Value::new_literal(IrLiteralValue::I16(i)).with_debug_info(None, span),
                Number::I32(i) => Value::new_literal(IrLiteralValue::I32(i)).with_debug_info(None, span),
                Number::Integer(i) => Value::new_literal(IrLiteralValue::I64(i)).with_debug_info(None, span),
                Number::U8(u) => Value::new_literal(IrLiteralValue::U8(u)).with_debug_info(None, span),
                Number::U16(u) => Value::new_literal(IrLiteralValue::U16(u)).with_debug_info(None, span),
                Number::U32(u) => Value::new_literal(IrLiteralValue::U32(u)).with_debug_info(None, span),
                Number::UnsignedInteger(u) => Value::new_literal(IrLiteralValue::U64(u)).with_debug_info(None, span),
                Number::Float32(f) => Value::new_literal(IrLiteralValue::F32(f)).with_debug_info(None, span),
                Number::Float64(f) => Value::new_literal(IrLiteralValue::F64(f)).with_debug_info(None, span),
                Number::Scientific32(f, i) => {
                    let value = f.powi(i);
                    Value::new_literal(IrLiteralValue::F32(value)).with_debug_info(None, span)
                }
                Number::Scientific64(f, i) => {
                    let value = f.powi(i);
                    Value::new_literal(IrLiteralValue::F64(value)).with_debug_info(None, span)
                }
            },
            LiteralValue::Bool(b) => Value::new_literal(IrLiteralValue::Bool(b)).with_debug_info(None, span),
            LiteralValue::StringLit(s) => {
                Value::new_constant(IrConstantValue::String { string: s }, IrType::String).with_debug_info(None, span)
            }
            LiteralValue::CharLit(c) => {
                Value::new_literal(IrLiteralValue::Char(c.chars().next().unwrap_or('\0'))).with_debug_info(None, span)
            }
            LiteralValue::Nullptr => Value::new_literal(IrLiteralValue::I64(0)).with_debug_info(None, span),
        }
    }

    fn generate_binary(
        &mut self, func: &mut Function, left: Expr, op: BinaryOp, right: Expr, span: SourceSpan,
    ) -> Value {
        let ir_op: IrBinaryOp = op.into();
        let left_val = self.generate_expr(func, left);
        let right_val = self.generate_expr(func, right);
        let ty = left_val.ty.clone();
        let dest_id = self.new_temp();

        let bin_inst = Instruction::new(
            InstructionKind::Binary { op: ir_op, left: left_val, right: right_val, ty: ty.clone() },
            span.clone(),
        )
        .with_result(Value::new_temporary(dest_id, ty.clone()));

        self.add_instruction(bin_inst.clone());
        bin_inst.result.unwrap()
    }

    fn generate_unary(&mut self, func: &mut Function, op: UnaryOp, expr: Expr, span: SourceSpan) -> Value {
        let ir_op: IrUnaryOp = op.into();
        let operand = self.generate_expr(func, expr);
        let ty = operand.ty.clone();
        let dest_id = self.new_temp();

        let unary_inst = Instruction::new(InstructionKind::Unary { op: ir_op, operand, ty: ty.clone() }, span.clone())
            .with_result(Value::new_temporary(dest_id, ty.clone()));

        self.add_instruction(unary_inst.clone());
        unary_inst.result.unwrap()
    }

    fn generate_variable(&mut self, name: Arc<str>, span: SourceSpan) -> Value {
        self.scope_manager.lookup(&name).cloned().unwrap_or_else(|| {
            self.new_error(format!("Undefined variable '{name}'"), span.clone());
            Value::new_literal(IrLiteralValue::I32(0)).with_debug_info(None, span)
        })
    }

    fn generate_assign(&mut self, func: &mut Function, target: Expr, value: Expr, span: SourceSpan) -> Value {
        let target_val = match target {
            Expr::ArrayAccess { array, index, span: access_span } => {
                self.generate_array_access_target(func, *array, *index, access_span)
            }
            _ => self.generate_expr(func, target),
        };

        let value_val = self.generate_expr(func, value);

        let store_inst =
            Instruction::new(InstructionKind::Store { value: value_val.clone(), dest: target_val }, span.clone());
        self.add_instruction(store_inst);

        value_val
    }

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
        // Finalize the current block first
        self.finalize_current_block(func);

        // Create a new block
        let new_block = BasicBlock::new(label, span.clone()).with_scope(self.scope_manager.current_scope());

        // Add the block to the CFG
        func.add_block(label, span);

        // Set the new block as current
        self.current_block = Some(new_block);
        self.current_block_label = Some(label.to_string());
    }

    fn add_instruction(&mut self, inst: Instruction) {
        if let Some(block) = &mut self.current_block {
            block.instructions.push(inst);
        }
    }

    // Add a terminator to the current block
    fn add_terminator(&mut self, _func: &mut Function, term: Terminator) {
        if let Some(block) = &mut self.current_block {
            block.terminator = term.clone();
            // Don't connect blocks here - they'll be connected when the block is finalized
        }
    }

    /// Generate array access for assignment target: calculate the address of the element with GEP
    /// and return the pointer (without loading the value).
    fn generate_array_access_target(
        &mut self, func: &mut Function, array: Expr, index: Expr, span: SourceSpan,
    ) -> Value {
        let base_val = self.generate_expr(func, array);
        let index_val = self.generate_expr(func, index);

        // Determine element type: handle both pointer to array and direct array
        let element_ty = match &base_val.ty {
            IrType::Pointer(inner) => match inner.as_ref() {
                IrType::Array(elem_ty, _) => *elem_ty.clone(),
                other => other.clone(), // fallback: pointer to already pointed element
            },
            IrType::Array(elem_ty, _) => *elem_ty.clone(),
            other => {
                // Unexpected case: report but continue with safe fallback (i32)
                self.new_error(format!("Array access on non-array type: {other}"), span.clone());
                IrType::I32
            }
        };

        let tmp = self.new_temp();
        let gep = Instruction::new(
            InstructionKind::GetElementPtr { base: base_val, index: index_val, element_ty: element_ty.clone() },
            span.clone(),
        )
        .with_result(Value::new_temporary(tmp, IrType::Pointer(Box::new(element_ty))));

        self.add_instruction(gep.clone());
        gep.result.unwrap()
    }

    /// Generate a function call instruction
    fn generate_call(&mut self, func: &mut Function, callee: Expr, arguments: Vec<Expr>, span: SourceSpan) -> Value {
        // Get the function name from the callee expression
        let func_name = match &callee {
            Expr::Variable { name, .. } => name.clone(),
            _ => {
                self.new_error("Unsupported callee expression type".to_string(), callee.span().clone());
                return Value::new_literal(IrLiteralValue::I32(0));
            }
        };

        // Generate values for all arguments
        let mut arg_values = Vec::with_capacity(arguments.len());
        for arg in arguments {
            arg_values.push(self.generate_expr(func, arg));
        }

        // Look up the function signature in the symbol table
        let (return_type, func_value) = if let Some(func_decl) = self.scope_manager.lookup(&func_name) {
            match &func_decl.ty {
                IrType::Pointer(inner) => {
                    // For function pointers, the return type is the pointed-to type
                    let return_type = inner.as_ref().clone();
                    (return_type, func_decl.clone())
                }
                _ => {
                    // If we can't determine the return type, fall back to default
                    self.new_error(
                        format!("Function '{}' does not have a valid function pointer type", func_name),
                        span.clone(),
                    );
                    let return_type = IrType::I64; // Default assumption
                    let func_value = Value::new_global(func_name, IrType::Pointer(Box::new(return_type.clone())));
                    (return_type, func_value)
                }
            }
        } else {
            // If function is not in symbol table, we might be calling a function
            // that is defined later or externally. Use a default return type.
            self.new_error(
                format!("Function '{}' not found in symbol table, using default return type", func_name),
                span.clone(),
            );
            let return_type = IrType::I64; // Default assumption
            let func_value = Value::new_global(func_name, IrType::Pointer(Box::new(return_type.clone())));
            (return_type, func_value)
        };

        // Create a temporary value for the result
        let dest_id = self.new_temp();
        let result_value = Value::new_temporary(dest_id, return_type.clone());

        // Create the call instruction
        let call_inst = Instruction::new(
            InstructionKind::Call { func: func_value, args: arg_values, ty: return_type },
            span.clone(),
        )
        .with_result(result_value);

        self.add_instruction(call_inst.clone());
        call_inst.result.unwrap()
    }
}

impl Default for NIrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
