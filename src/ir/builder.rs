// src/ir/builder.rs
use crate::ir::ir::*;
//use crate::ir::symbol_table::{SymbolTable, Symbol, Parameter};
use crate::ir::types::Type;
use crate::ir::values::{Value, Constant, InstructionRef, ArgumentRef, GlobalRef};
use crate::ir::{BinaryOperator, IntPredicate};  // Aggiungi questa importazione
//use std::collections::HashMap;
pub struct IrBuilder {
    module: Module,
    current_function: Option<Function>,
    current_block: Option<BasicBlock>,
    value_counter: u32,
    block_counter: u32,
    loop_stack: Vec<(String, String)>,
    function_stack: Vec<String>,
}
impl IrBuilder {
    pub fn new(module_name: String) -> Self {
        Self {
            module: Module::new(module_name),
            current_function: None,
            current_block: None,
            value_counter: 0,
            block_counter: 0,
            loop_stack: Vec::new(),
            function_stack: Vec::new(),
        }
    }
    pub fn build(mut self, ast: Vec<crate::parser::ast::Stmt>) -> Module {
        // Process top-level statements
        for stmt in ast {
            self.process_top_level_stmt(stmt);
        }
        self.module
    }
    fn process_top_level_stmt(&mut self, stmt: crate::parser::ast::Stmt) {
        match stmt {
            crate::parser::ast::Stmt::Function {
                name,
                parameters,
                return_type,
                body,
                span: _
            } => {
                self.process_function(name, parameters, return_type, body);
            }
            crate::parser::ast::Stmt::MainFunction { body, span: _ } => {
                self.process_main_function(body);
            }
            crate::parser::ast::Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable,
                initializers,
                span: _
            } => {
                self.process_global_var_declaration(variables, type_annotation, is_mutable, initializers);
            }
            _ => {
                // Other top-level statements not implemented yet
            }
        }
    }
    fn process_function(
        &mut self,
        name: String,
        parameters: Vec<crate::parser::ast::Parameter>,
        return_type: crate::parser::ast::Type,
        body: Vec<crate::parser::ast::Stmt>
    ) {
        let ir_return_type = self.convert_ast_type(&return_type);
        let mut function = Function::new(name.clone(), ir_return_type);
        // Convert parameters
        for param in parameters {
            let ir_param_ty = self.convert_ast_type(&param.type_annotation);
            function.add_parameter(param.name, ir_param_ty);
        }
        // Add function to module
        self.module.add_function(function.clone());
        self.current_function = Some(function);
        self.function_stack.push(name.clone());
        // Enter function scope
        self.module.symbol_table.enter_scope(crate::ir::symbol_table::ScopeKind::Function);
        // Add parameters to symbol table
        for (i, param) in self.current_function.as_ref().unwrap().parameters.iter().enumerate() {
            let value = Value::Argument(ArgumentRef {
                index: i as u32,
                ty: param.ty.clone(),
            });
            self.module.symbol_table.add_variable(
                param.name.clone(),
                param.ty.clone(),
                value,
                false, // Parameters are immutable
                false,
            ).unwrap();
        }
        // Create entry block
        let entry_block = self.create_basic_block("entry");
        self.set_insert_point(entry_block);
        // Process function body
        self.process_block(body);
        // If no return instruction, add void return
        if let Some(block) = &mut self.current_block {
            if matches!(block.terminator, Terminator::Unreachable) {
                block.set_terminator(Terminator::Ret { value: None });
            }
        }
        // Exit function scope
        self.module.symbol_table.exit_scope();
        self.function_stack.pop();
        self.current_function = None;
    }
    fn process_main_function(&mut self, body: Vec<crate::parser::ast::Stmt>) {
        self.process_function(
            "main".to_string(),
            Vec::new(),
            crate::parser::ast::Type::I32,
            body,
        );
    }
    fn process_global_var_declaration(
        &mut self,
        variables: Vec<String>,
        type_annotation: crate::parser::ast::Type,
        is_mutable: bool,
        initializers: Vec<crate::parser::ast::Expr>
    ) {
        let ir_type = self.convert_ast_type(&type_annotation);
        for (i, var_name) in variables.iter().enumerate() {
            let initializer = if i < initializers.len() {
                // Convert Value to Constant
                if let Some(value) = self.evaluate_constant_expr(&initializers[i]) {
                    self.value_to_constant(value)
                } else {
                    None
                }
            } else {
                None
            };
            let global = GlobalVariable {
                name: var_name.clone(),
                ty: ir_type.clone(), // Fixed: Clone the type
                initializer,
                linkage: LinkageType::Internal,
                visibility: Visibility::Default,
                is_constant: !is_mutable,
                alignment: None,
                section: None,
            };
            self.module.add_global_variable(global.clone());
            // Add to symbol table
            let value = Value::Global(GlobalRef {
                name: var_name.clone(),
                ty: ir_type.clone(), // Fixed: Clone the type
            });
            self.module.symbol_table.add_variable(
                var_name.clone(),
                ir_type.clone(), // Fixed: Clone the type
                value,
                is_mutable,
                true,
            ).unwrap();
        }
    }
    fn process_block(&mut self, stmts: Vec<crate::parser::ast::Stmt>) {
        self.module.symbol_table.enter_scope(crate::ir::symbol_table::ScopeKind::Block);
        for stmt in stmts {
            self.process_stmt(stmt);
        }
        self.module.symbol_table.exit_scope();
    }
    fn process_stmt(&mut self, stmt: crate::parser::ast::Stmt) {
        match stmt {
            crate::parser::ast::Stmt::Expression { expr } => {
                self.process_expr(expr);
            }
            crate::parser::ast::Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable,
                initializers,
                span: _
            } => {
                self.process_local_var_declaration(variables, type_annotation, is_mutable, initializers);
            }
            crate::parser::ast::Stmt::Return { value, span: _ } => {
                let ir_value = value.map(|v| self.process_expr(v));
                self.build_return(ir_value);
            }
            crate::parser::ast::Stmt::If {
                condition,
                then_branch,
                else_branch,
                span: _
            } => {
                self.process_if(condition, then_branch, else_branch);
            }
            crate::parser::ast::Stmt::While { condition, body, span: _ } => {
                self.process_while(condition, body);
            }
            crate::parser::ast::Stmt::For {
                initializer,
                condition,
                increment,
                body,
                span: _
            } => {
                self.process_for(initializer, condition, increment, body);
            }
            crate::parser::ast::Stmt::Block { statements, span: _ } => {
                self.process_block(statements);
            }
            crate::parser::ast::Stmt::Break { span: _ } => {
                self.build_break();
            }
            crate::parser::ast::Stmt::Continue { span: _ } => {
                self.build_continue();
            }
            _ => {
                // Other statements not implemented yet
            }
        }
    }
    fn process_local_var_declaration(
        &mut self,
        variables: Vec<String>,
        type_annotation: crate::parser::ast::Type,
        is_mutable: bool,
        initializers: Vec<crate::parser::ast::Expr>
    ) {
        let ir_type = self.convert_ast_type(&type_annotation);
        for (i, var_name) in variables.iter().enumerate() {
            // Allocate space on the stack
            let alloca = self.build_alloca(ir_type.clone()); // Fixed: Clone the type
            // Initialize if provided
            if i < initializers.len() {
                let init_value = self.process_expr(initializers[i].clone());
                self.build_store(init_value, alloca.clone());
            }
            // Add to symbol table
            self.module.symbol_table.add_variable(
                var_name.clone(),
                ir_type.clone(), // Fixed: Clone the type
                alloca,
                is_mutable,
                false,
            ).unwrap();
        }
    }
    fn process_if(
        &mut self,
        condition: crate::parser::ast::Expr,
        then_branch: Vec<crate::parser::ast::Stmt>,
        else_branch: Option<Vec<crate::parser::ast::Stmt>>
    ) {
        let condition_value = self.process_expr(condition);
        // Create blocks
        let then_block = self.create_basic_block("then");
        let else_block = if else_branch.is_some() {
            Some(self.create_basic_block("else"))
        } else {
            None
        };
        let merge_block = self.create_basic_block("ifmerge");
        // Build conditional branch
        if let Some(ref else_block) = else_block {
            self.build_cond_br(condition_value, then_block.name.clone(), else_block.name.clone());
        } else {
            self.build_cond_br(condition_value, then_block.name.clone(), merge_block.name.clone());
        }
        // Process then block
        self.set_insert_point(then_block);
        self.process_block(then_branch);
        if !self.current_block_terminated() {
            self.build_br(merge_block.name.clone());
        }
        // Process else block if present
        if let Some(ref else_block) = else_block {
            self.set_insert_point(else_block.clone());
            if let Some(else_stmts) = else_branch {
                self.process_block(else_stmts);
            }
            if !self.current_block_terminated() {
                self.build_br(merge_block.name.clone());
            }
        }
        // Set insert point to merge block
        self.set_insert_point(merge_block);
    }
    fn process_while(&mut self, condition: crate::parser::ast::Expr, body: Vec<crate::parser::ast::Stmt>) {
        let condition_block = self.create_basic_block("while.cond");
        let body_block = self.create_basic_block("while.body");
        let end_block = self.create_basic_block("while.end");
        // Branch to condition block
        self.build_br(condition_block.name.clone());
        // Process condition block
        self.set_insert_point(condition_block.clone());
        let condition_value = self.process_expr(condition);
        self.build_cond_br(condition_value, body_block.name.clone(), end_block.name.clone());
        // Process body block
        self.set_insert_point(body_block);
        self.loop_stack.push((condition_block.name.clone(), end_block.name.clone()));
        self.process_block(body);
        self.loop_stack.pop();
        if !self.current_block_terminated() {
            self.build_br(condition_block.name.clone());
        }
        // Set insert point to end block
        self.set_insert_point(end_block);
    }
    fn process_for(
        &mut self,
        initializer: Option<Box<crate::parser::ast::Stmt>>,
        condition: Option<crate::parser::ast::Expr>,
        increment: Option<crate::parser::ast::Expr>,
        body: Vec<crate::parser::ast::Stmt>
    ) {
        let condition_block = self.create_basic_block("for.cond");
        let body_block = self.create_basic_block("for.body");
        let increment_block = self.create_basic_block("for.inc");
        let end_block = self.create_basic_block("for.end");
        // Process initializer
        if let Some(init) = initializer {
            self.process_stmt(*init);
        }
        // Branch to condition block
        self.build_br(condition_block.name.clone());
        // Process condition block
        self.set_insert_point(condition_block.clone());
        let condition_value = if let Some(cond) = condition {
            self.process_expr(cond)
        } else {
            self.build_constant_bool(true)
        };
        self.build_cond_br(condition_value, body_block.name.clone(), end_block.name.clone());
        // Process body block
        self.set_insert_point(body_block);
        self.loop_stack.push((increment_block.name.clone(), end_block.name.clone()));
        self.process_block(body);
        self.loop_stack.pop();
        if !self.current_block_terminated() {
            self.build_br(increment_block.name.clone());
        }
        // Process increment block
        self.set_insert_point(increment_block);
        if let Some(inc) = increment {
            self.process_expr(inc);
        }
        self.build_br(condition_block.name.clone());
        // Set insert point to end block
        self.set_insert_point(end_block);
    }
    fn process_expr(&mut self, expr: crate::parser::ast::Expr) -> Value {
        match expr {
            crate::parser::ast::Expr::Literal { value, span: _ } => {
                self.process_literal(value)
            }
            crate::parser::ast::Expr::Variable { name, span: _ } => {
                self.process_variable(name)
            }
            crate::parser::ast::Expr::Binary { left, op, right, span: _ } => {
                let left_value = self.process_expr(*left);
                let right_value = self.process_expr(*right);
                self.process_binary_op(left_value, op, right_value)
            }
            crate::parser::ast::Expr::Unary { op, expr, span: _ } => {
                let operand = self.process_expr(*expr);
                self.process_unary_op(op, operand)
            }
            crate::parser::ast::Expr::Assign { target, value, span: _ } => {
                let target_value = self.process_expr(*target);
                let value_value = self.process_expr(*value);
                self.process_assignment(target_value, value_value)
            }
            crate::parser::ast::Expr::Call { callee, arguments, span: _ } => {
                let callee_value = self.process_expr(*callee);
                let arg_values = arguments.into_iter().map(|arg| self.process_expr(arg)).collect();
                self.process_call(callee_value, arg_values)
            }
            crate::parser::ast::Expr::ArrayAccess { array, index, span: _ } => {
                let array_value = self.process_expr(*array);
                let index_value = self.process_expr(*index);
                self.process_array_access(array_value, index_value)
            }
            crate::parser::ast::Expr::ArrayLiteral { elements, span: _ } => {
                let element_values = elements.into_iter().map(|elem| self.process_expr(elem)).collect();
                self.process_array_literal(element_values)
            }
            crate::parser::ast::Expr::Grouping { expr, span: _ } => {
                self.process_expr(*expr)
            }
            _ => {
                // Other expressions not implemented yet
                self.build_undef(Type::void())
            }
        }
    }
    fn process_literal(&mut self, literal: crate::parser::ast::LiteralValue) -> Value {
        match literal {
            crate::parser::ast::LiteralValue::Number(num) => {
                self.process_number_literal(num)
            }
            crate::parser::ast::LiteralValue::StringLit(s) => {
                self.build_constant_string(s)
            }
            crate::parser::ast::LiteralValue::CharLit(c) => {
                self.build_constant_char(c)
            }
            crate::parser::ast::LiteralValue::Bool(b) => {
                self.build_constant_bool(b)
            }
            crate::parser::ast::LiteralValue::Nullptr => {
                self.build_constant_null(Type::pointer_to(Type::i8()))
            }
        }
    }
    fn process_number_literal(&mut self, num: crate::tokens::number::Number) -> Value {
        match num {
            crate::tokens::number::Number::I8(v) => self.build_constant_i8(v),
            crate::tokens::number::Number::I16(v) => self.build_constant_i16(v),
            crate::tokens::number::Number::I32(v) => self.build_constant_i32(v),
            crate::tokens::number::Number::Integer(v) => self.build_constant_i64(v),
            crate::tokens::number::Number::U8(v) => self.build_constant_u8(v),
            crate::tokens::number::Number::U16(v) => self.build_constant_u16(v),
            crate::tokens::number::Number::U32(v) => self.build_constant_u32(v),
            crate::tokens::number::Number::UnsignedInteger(v) => self.build_constant_u64(v),
            crate::tokens::number::Number::Float32(v) => self.build_constant_f32(v),
            crate::tokens::number::Number::Float64(v) => self.build_constant_f64(v),
            crate::tokens::number::Number::Scientific32(base, exp) => {
                let value = base * 10f32.powi(exp);
                self.build_constant_f32(value)
            }
            crate::tokens::number::Number::Scientific64(base, exp) => {
                let value = base * 10f64.powi(exp);
                self.build_constant_f64(value)
            }
        }
    }
    fn process_variable(&mut self, name: String) -> Value {
        if let Some((ty, value, _)) = self.module.symbol_table.get_variable(&name) {
            if ty.is_pointer() {
                // Load the value if it's a pointer (stack allocation)
                self.build_load(value, ty.get_pointer_element_type().unwrap().clone())
            } else {
                value
            }
        } else {
            // Variable not found
            self.build_undef(Type::void())
        }
    }
    fn process_binary_op(&mut self, left: Value, op: crate::parser::ast::BinaryOp, right: Value) -> Value {
        match op {
            crate::parser::ast::BinaryOp::Add => self.build_add(left, right),
            crate::parser::ast::BinaryOp::Subtract => self.build_sub(left, right),
            crate::parser::ast::BinaryOp::Multiply => self.build_mul(left, right),
            crate::parser::ast::BinaryOp::Divide => self.build_div(left, right),
            crate::parser::ast::BinaryOp::Modulo => self.build_rem(left, right),
            crate::parser::ast::BinaryOp::Equal => self.build_icmp(IntPredicate::EQ, left, right),
            crate::parser::ast::BinaryOp::NotEqual => self.build_icmp(IntPredicate::NE, left, right),
            crate::parser::ast::BinaryOp::Less => self.build_icmp(IntPredicate::SLT, left, right),
            crate::parser::ast::BinaryOp::LessEqual => self.build_icmp(IntPredicate::SLE, left, right),
            crate::parser::ast::BinaryOp::Greater => self.build_icmp(IntPredicate::SGT, left, right),
            crate::parser::ast::BinaryOp::GreaterEqual => self.build_icmp(IntPredicate::SGE, left, right),
            crate::parser::ast::BinaryOp::And => self.build_and(left, right),
            crate::parser::ast::BinaryOp::Or => self.build_or(left, right),
            crate::parser::ast::BinaryOp::BitwiseAnd => self.build_and(left, right),
            crate::parser::ast::BinaryOp::BitwiseOr => self.build_or(left, right),
            crate::parser::ast::BinaryOp::BitwiseXor => self.build_xor(left, right),
            crate::parser::ast::BinaryOp::ShiftLeft => self.build_shl(left, right),
            crate::parser::ast::BinaryOp::ShiftRight => self.build_ashr(left, right),
        }
    }
    fn process_unary_op(&mut self, op: crate::parser::ast::UnaryOp, operand: Value) -> Value {
        match op {
            crate::parser::ast::UnaryOp::Negate => self.build_neg(operand),
            crate::parser::ast::UnaryOp::Not => self.build_not(operand),
        }
    }
    fn process_assignment(&mut self, target: Value, value: Value) -> Value {
        self.build_store(value.clone(), target);
        value
    }
    fn process_call(&mut self, callee: Value, arguments: Vec<Value>) -> Value {
        // Convert arguments to (Value, Type) pairs
        let arg_pairs: Vec<(Value, Type)> = arguments
            .into_iter()
            .map(|arg| (arg.clone(), arg.get_type()))
            .collect();
        self.build_call(callee, arg_pairs)
    }
    fn process_array_access(&mut self, array: Value, index: Value) -> Value {
        let ptr_type = array.get_type();
        let element_type = ptr_type.get_pointer_element_type().unwrap().clone();
        // Get element pointer
        let gep = self.build_gep(array, vec![(self.build_constant_i32(0), Type::i32()), (index, Type::i32())]);
        // Load the element
        self.build_load(gep, element_type)
    }
    fn process_array_literal(&mut self, elements: Vec<Value>) -> Value {
        if elements.is_empty() {
            return self.build_undef(Type::array_of(Type::i8(), 0));
        }
        let element_type = elements[0].get_type();
        let array_type = Type::array_of(element_type, elements.len() as u64);
        // Create constant array
        let constants: Vec<Constant> = elements
            .into_iter()
            .filter_map(|v| {
                if let Value::Constant(c) = v {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        self.build_constant_array(array_type, constants)
    }
    fn convert_ast_type(&self, ast_type: &crate::parser::ast::Type) -> Type {
        match ast_type {
            crate::parser::ast::Type::I8 => Type::i8(),
            crate::parser::ast::Type::I16 => Type::i16(),
            crate::parser::ast::Type::I32 => Type::i32(),
            crate::parser::ast::Type::I64 => Type::i64(),
            crate::parser::ast::Type::U8 => Type::u8(),
            crate::parser::ast::Type::U16 => Type::u16(),
            crate::parser::ast::Type::U32 => Type::u32(),
            crate::parser::ast::Type::U64 => Type::u64(),
            crate::parser::ast::Type::F32 => Type::f32(),
            crate::parser::ast::Type::F64 => Type::f64(),
            crate::parser::ast::Type::Char => Type::i8(),
            crate::parser::ast::Type::String => Type::pointer_to(Type::i8()),
            crate::parser::ast::Type::Bool => Type::bool(),
            crate::parser::ast::Type::Custom(name) => Type::Named(name.clone()),
            crate::parser::ast::Type::Array(element_type, size_expr) => {
                let ir_element_type = self.convert_ast_type(element_type);
                // Try to evaluate size as constant
                if let Some(size) = self.evaluate_constant_expr(size_expr) {
                    if let Value::Constant(Constant::Integer { value, .. }) = size {
                        Type::array_of(ir_element_type, value)
                    } else {
                        Type::array_of(ir_element_type, 0) // Unknown size
                    }
                } else {
                    Type::array_of(ir_element_type, 0) // Unknown size
                }
            }
            crate::parser::ast::Type::Vector(element_type) => {
                let ir_element_type = self.convert_ast_type(element_type);
                Type::vector_of(ir_element_type, 0) // Unknown size
            }
            crate::parser::ast::Type::Void => Type::void(),
            crate::parser::ast::Type::NullPtr => Type::pointer_to(Type::i8()),
        }
    }
    fn evaluate_constant_expr(&self, expr: &crate::parser::ast::Expr) -> Option<Value> {
        match expr {
            crate::parser::ast::Expr::Literal { value, .. } => {
                match value {
                    crate::parser::ast::LiteralValue::Number(num) => {
                        match num {
                            crate::tokens::number::Number::I8(v) => Some(self.build_constant_i8(*v)),
                            crate::tokens::number::Number::I16(v) => Some(self.build_constant_i16(*v)),
                            crate::tokens::number::Number::I32(v) => Some(self.build_constant_i32(*v)),
                            crate::tokens::number::Number::Integer(v) => Some(self.build_constant_i64(*v)),
                            crate::tokens::number::Number::U8(v) => Some(self.build_constant_u8(*v)),
                            crate::tokens::number::Number::U16(v) => Some(self.build_constant_u16(*v)),
                            crate::tokens::number::Number::U32(v) => Some(self.build_constant_u32(*v)),
                            crate::tokens::number::Number::UnsignedInteger(v) => Some(self.build_constant_u64(*v)),
                            crate::tokens::number::Number::Float32(v) => Some(self.build_constant_f32(*v)),
                            crate::tokens::number::Number::Float64(v) => Some(self.build_constant_f64(*v)),
                            _ => None,
                        }
                    }
                    crate::parser::ast::LiteralValue::Bool(b) => Some(self.build_constant_bool(*b)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
    // Helper method to convert a Value to a Constant
    fn value_to_constant(&self, value: Value) -> Option<Constant> {
        match value {
            Value::Constant(c) => Some(c),
            _ => None,
        }
    }
    // Helper methods for building IR
    fn create_basic_block(&mut self, name: &str) -> BasicBlock {
        let block_name = if name.is_empty() {
            format!("bb{}", self.block_counter)
        } else {
            format!("{}.{}", name, self.block_counter)
        };
        self.block_counter += 1;
        let block = BasicBlock::new(block_name.clone());
        if let Some(ref mut function) = self.current_function {
            function.add_basic_block(block.clone());
        }
        block
    }
    fn set_insert_point(&mut self, block: BasicBlock) {
        self.current_block = Some(block);
    }
    fn current_block_terminated(&self) -> bool {
        if let Some(ref block) = self.current_block {
            !matches!(block.terminator, Terminator::Unreachable)
        } else {
            false
        }
    }
    fn next_value_id(&mut self) -> u32 {
        self.value_counter += 1;
        self.value_counter
    }
    fn build_alloca(&mut self, ty: Type) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: Type::pointer_to(ty.clone()),
        });
        let instruction = Instruction::Alloca {
            dest: dest.clone(),
            ty,
            align: None,
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_load(&mut self, ptr: Value, ty: Type) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: ty.clone(),
        });
        let instruction = Instruction::Load {
            dest: dest.clone(),
            ptr,
            ty,
            align: None,
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_store(&mut self, value: Value, ptr: Value) {
        let instruction = Instruction::Store {
            value,
            ptr,
            align: None,
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
    }
    fn build_add(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Add,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_sub(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Sub,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_mul(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Mul,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_div(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::SDiv,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_rem(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::SRem,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_and(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::And,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_or(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Or,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_xor(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Xor,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_shl(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Shl,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_ashr(&mut self, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: left.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::AShr,
            dest: dest.clone(),
            left,
            right,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_neg(&mut self, operand: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: operand.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Sub,
            dest: dest.clone(),
            left: self.build_constant_i32(0),
            right: operand,
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_not(&mut self, operand: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: operand.get_type(),
        });
        let instruction = Instruction::BinaryOp {
            op: BinaryOperator::Xor,
            dest: dest.clone(),
            left: operand,
            right: self.build_constant_i32(-1),
            flags: Vec::new(),
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_icmp(&mut self, predicate: IntPredicate, left: Value, right: Value) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: Type::bool(),
        });
        let instruction = Instruction::ICmp {
            dest: dest.clone(),
            predicate,
            left,
            right,
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_call(&mut self, callee: Value, arguments: Vec<(Value, Type)>) -> Value {
        let dest = if let Type::Function { return_type, .. } = callee.get_type() {
            if return_type.is_void() {
                None
            } else {
                Some(Value::Instruction(InstructionRef {
                    id: self.next_value_id(),
                    ty: *return_type,
                }))
            }
        } else {
            None
        };
        let instruction = Instruction::Call {
            dest: dest.clone(),
            callee,
            arguments,
            calling_conv: CallingConvention::C,
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest.unwrap_or_else(|| self.build_undef(Type::void()))
    }
    fn build_gep(&mut self, ptr: Value, indices: Vec<(Value, Type)>) -> Value {
        let dest = Value::Instruction(InstructionRef {
            id: self.next_value_id(),
            ty: ptr.get_type().get_pointer_element_type()
                .unwrap_or(&Type::void())
                .clone(),
        });
        let instruction = Instruction::GetElementPtr {
            dest: dest.clone(),
            ptr,
            indices,
            inbounds: true,
        };
        if let Some(ref mut block) = self.current_block {
            block.add_instruction(instruction);
        }
        dest
    }
    fn build_br(&mut self, dest: String) {
        let terminator = Terminator::Br { dest };
        if let Some(ref mut block) = self.current_block {
            block.set_terminator(terminator);
        }
    }
    fn build_cond_br(&mut self, condition: Value, true_dest: String, false_dest: String) {
        let terminator = Terminator::CondBr {
            condition,
            true_dest,
            false_dest,
        };
        if let Some(ref mut block) = self.current_block {
            block.set_terminator(terminator);
        }
    }
    fn build_return(&mut self, value: Option<Value>) {
        let terminator = Terminator::Ret { value };
        if let Some(ref mut block) = self.current_block {
            block.set_terminator(terminator);
        }
    }
    fn build_break(&mut self) {
        if let Some((_, end_block)) = self.loop_stack.last() {
            self.build_br(end_block.clone());
        }
    }
    fn build_continue(&mut self) {
        if let Some((cond_block, _)) = self.loop_stack.last() {
            self.build_br(cond_block.clone());
        }
    }
    fn build_constant_i8(&self, value: i8) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::i8(),
        })
    }
    fn build_constant_i16(&self, value: i16) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::i16(),
        })
    }
    fn build_constant_i32(&self, value: i32) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::i32(),
        })
    }
    fn build_constant_i64(&self, value: i64) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::i64(),
        })
    }
    fn build_constant_u8(&self, value: u8) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::u8(),
        })
    }
    fn build_constant_u16(&self, value: u16) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::u16(),
        })
    }
    fn build_constant_u32(&self, value: u32) -> Value {
        Value::Constant(Constant::Integer {
            value: value as u64,
            ty: Type::u32(),
        })
    }
    fn build_constant_u64(&self, value: u64) -> Value {
        Value::Constant(Constant::Integer {
            value,
            ty: Type::u64(),
        })
    }
    fn build_constant_f32(&self, value: f32) -> Value {
        Value::Constant(Constant::Float {
            value: value as f64,
            ty: Type::f32(),
        })
    }
    fn build_constant_f64(&self, value: f64) -> Value {
        Value::Constant(Constant::Float {
            value,
            ty: Type::f64(),
        })
    }
    fn build_constant_bool(&self, value: bool) -> Value {
        Value::Constant(Constant::Bool(value))
    }
    fn build_constant_string(&self, value: String) -> Value {
        Value::Constant(Constant::String(value))
    }
    fn build_constant_char(&self, value: String) -> Value {
        if let Some(c) = value.chars().next() {
            Value::Constant(Constant::Integer {
                value: c as u64,
                ty: Type::i8(),
            })
        } else {
            self.build_constant_i8(0)
        }
    }
    fn build_constant_null(&self, ty: Type) -> Value {
        Value::Constant(Constant::Null(ty))
    }
    fn build_constant_array(&self, _ty: Type, elements: Vec<Constant>) -> Value {
        Value::Constant(Constant::Array(elements))
    }
    fn build_undef(&self, ty: Type) -> Value {
        Value::Undef(ty)
    }
}