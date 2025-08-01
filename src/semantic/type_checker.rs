// src/semantic/type_checker.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::semantic::symbol_table::*;
use crate::tokens::number::Number;

pub struct TypeChecker {
    symbol_table: SymbolTable,
    errors: Vec<CompileError>,
    in_loop: usize,
}

// Promotion hierarchy
const HIERARCHY: [Type; 10] = [
    Type::F64,
    Type::F32,
    Type::U64,
    Type::I64,
    Type::U32,
    Type::I32,
    Type::U16,
    Type::I16,
    Type::U8,
    Type::I8,
];

#[allow(clippy::borrowed_box, clippy::only_used_in_recursion)]
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            in_loop: 0,
        }
    }

    pub fn check(&mut self, ast: &[Stmt]) -> Vec<CompileError> {
        // First pass: declare functions and global variables
        for stmt in ast {
            self.visit_stmt_first_pass(stmt);
        }

        // Second pass: type checking
        for stmt in ast {
            self.visit_stmt(stmt);
        }

        std::mem::take(&mut self.errors)
    }

    fn ptype_error(&mut self, message: String, span: SourceSpan) {
        self.errors.push(CompileError::TypeError { message, span });
    }

    fn undefined_var_error(&mut self, name: &String, span: SourceSpan) {
        self.errors.push(CompileError::TypeError {
            message: format!("Undefined variable '{name}'"),
            span,
        });
    }
    fn array_access_index_error(&mut self, index_type: &Type, span: SourceSpan) {
        self.errors.push(CompileError::TypeError {
            message: format!("Array index must be integer, found {index_type}"),
            span,
        });
    }

    // Helper method per dichiarare simboli
    fn declare_symbol(&mut self, name: &str, symbol: Symbol) {
        if let Err(e) = self.symbol_table.declare(name, symbol) {
            self.errors.push(e);
        }
    }

    fn visit_stmt_first_pass(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements, .. } => self.process_block(statements),
            Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable,
                initializers: _,
                span,
                ..
            } => self.process_var_declaration(variables, type_annotation, *is_mutable, span),
            Stmt::Function {
                name,
                parameters,
                return_type,
                body,
                span,
            } => self.process_function(name, parameters, return_type, body, span),
            Stmt::MainFunction { body, span } => self.process_main_function(body, span),
            Stmt::For {
                initializer, body, ..
            } => self.process_for_loop(initializer, body),
            Stmt::While { body, .. } => self.process_while_loop(body),
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => self.process_if_statement(then_branch, else_branch),
            _ => {} // Other statements don't declare symbols in first pass
        }
    }

    fn process_block(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.visit_stmt_first_pass(stmt);
        }
    }

    fn process_var_declaration(
        &mut self,
        variables: &[String],
        type_annotation: &Type,
        is_mutable: bool,
        span: &SourceSpan,
    ) {
        for var in variables {
            let symbol = Symbol::Variable(VariableSymbol {
                name: var.clone(),
                ty: type_annotation.clone(),
                mutable: is_mutable,
                defined_at: span.clone(),
                last_assignment: None,
            });
            self.declare_symbol(var, symbol);
        }
    }

    fn process_function(
        &mut self,
        name: &str,
        parameters: &[Parameter],
        return_type: &Type,
        body: &[Stmt],
        span: &SourceSpan,
    ) {
        // Declare function symbol
        let func_symbol = FunctionSymbol {
            name: name.to_string(),
            parameters: parameters.to_vec(),
            return_type: return_type.clone(),
            defined_at: span.clone(),
        };
        self.declare_symbol(name, Symbol::Function(func_symbol.clone()));

        self.symbol_table.enter_function(func_symbol);
        self.symbol_table.push_scope(ScopeKind::Function, Some(span.clone()));
        self.declare_parameters(parameters);
        self.process_block(body);
        // Note: Scope not popped to retain symbols for second pass
    }

    fn process_main_function(&mut self, body: &[Stmt], span: &SourceSpan) {
        // Main is treated as special case function
        self.process_function("main", &[], &Type::Void, body, span);
    }

    fn declare_parameters(&mut self, parameters: &[Parameter]) {
        for param in parameters {
            let symbol = Symbol::Variable(VariableSymbol {
                name: param.name.clone(),
                ty: param.type_annotation.clone(),
                mutable: false,
                defined_at: param.span.clone(),
                last_assignment: None,
            });
            self.declare_symbol(&param.name, symbol);
        }
    }

    fn process_for_loop(&mut self, initializer: &Option<Box<Stmt>>, body: &[Stmt]) {
        // Process initializer if present
        if let Some(init) = initializer {
            self.visit_stmt_first_pass(init);
        }

        // Process loop body
        self.process_block(body);
    }

    fn process_while_loop(&mut self, body: &[Stmt]) {
        self.process_block(body);
    }

    fn process_if_statement(&mut self, then_branch: &[Stmt], else_branch: &Option<Vec<Stmt>>) {
        // Process main branch
        self.process_block(then_branch);

        // Process else branch if exists
        if let Some(else_statements) = else_branch {
            self.process_block(else_statements);
        }
    }

    //#[allow(unreachable_patterns)]
    #[allow(clippy::collapsible_if)]
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements, span } => {
                self.symbol_table.push_scope(ScopeKind::Block, Some(span.clone()));
                for stmt in statements {
                    self.visit_stmt(stmt);
                }
                self.symbol_table.pop_scope();
            }
            Stmt::Function {
                name: _,
                parameters: _,
                return_type: _,
                body,
                span: _,
            } => {
                // Process function body
                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.symbol_table.pop_scope();
                self.symbol_table.exit_function();
            }
            Stmt::MainFunction { body, .. } => {
                for stmt in body {
                    self.visit_stmt(stmt);
                }
                self.symbol_table.pop_scope();
                self.symbol_table.exit_function();
            }
            Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable: _,
                initializers,
                span: _,
            } => {
                for (var, init) in variables.iter().zip(initializers) {
                    if let Some(init_type) = self.visit_expr(init) {
                        if !self.is_assignable(&init_type, type_annotation) {
                            self.ptype_error(
                                format!("Cannot assign {init_type} to {type_annotation} for variable '{var}'"),
                                init.span().clone(),
                            )
                        }
                    }
                }
            }
            Stmt::Expression { expr } => {
                self.visit_expr(expr);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                span: _,
            } => {
                if let Some(cond_type) = self.visit_expr(condition) {
                    if cond_type != Type::Bool {
                        self.ptype_error(
                            format!("If condition must be bool, found {cond_type}"),
                            condition.span().clone(),
                        );
                    }
                }

                for stmt in then_branch {
                    self.visit_stmt(stmt);
                }

                if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.visit_stmt(stmt);
                    }
                }
            }
            Stmt::Return { value, span } => {
                self.check_return_statement(value, span);
            }
            Stmt::Break { span } | Stmt::Continue { span } => {
                if self.in_loop == 0 {
                    self.ptype_error("Break/continue outside loop".to_string(), span.clone());
                }
            }
            Stmt::While {
                condition,
                body,
                span: _,
            } => {
                self.in_loop += 1;
                if let Some(cond_type) = self.visit_expr(condition) {
                    if cond_type != Type::Bool {
                        self.ptype_error(
                            format!("While condition must be bool, found {cond_type}"),
                            condition.span().clone(),
                        );
                    }
                }
                for stmt in body {
                    self.visit_stmt(stmt);
                }
                self.in_loop -= 1;
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                span: _,
            } => {
                self.in_loop += 1;
                // Process initializer
                if let Some(init) = initializer {
                    self.visit_stmt(init);
                }

                // Process condition
                if let Some(cond) = condition {
                    if let Some(cond_type) = self.visit_expr(cond) {
                        if cond_type != Type::Bool {
                            self.ptype_error(
                                format!("For loop condition must be bool, found {cond_type}"),
                                cond.span().clone(),
                            );
                        }
                    }
                }

                // Process increment
                if let Some(inc) = increment {
                    self.visit_expr(inc);
                }

                // Process body
                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.in_loop -= 1;
            } //_ => unimplemented!("Unsupported statement in type checker"),
        }
    }

    //#[allow(unreachable_patterns)]
    #[allow(clippy::collapsible_if)]
    fn visit_expr(&mut self, expr: &Expr) -> Option<Type> {
        match expr {
            Expr::Variable { name, span } => self
                .symbol_table
                .lookup_variable(name)
                .map(|sym| sym.ty.clone())
                .or_else(|| {
                    self.undefined_var_error(name, span.clone());
                    None
                }),
            Expr::Literal { value, span: _ } => match value {
                LiteralValue::Number(n) => Some(self.type_of_number(n)),
                LiteralValue::StringLit(_) => Some(Type::String),
                LiteralValue::CharLit(_) => Some(Type::Char),
                LiteralValue::Bool(_) => Some(Type::Bool),
                LiteralValue::Nullptr => Some(Type::NullPtr),
            },
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let left_type = self.visit_expr(left)?;
                let right_type = self.visit_expr(right)?;
                self.check_binary_op(&left_type, op, &right_type, span)
            }
            Expr::Unary { op, expr, span } => {
                let expr_type = self.visit_expr(expr)?;
                self.check_unary_op(op, &expr_type, span)
            }
            Expr::Grouping { expr, span: _ } => self.visit_expr(expr),
            Expr::Assign {
                target,
                value,
                span: _,
            } => {
                let value_type = self.visit_expr(value)?;

                // Check valid lvalue
                match &**target {
                    Expr::Variable { name, span } => {
                        if let Some(sym) = self.symbol_table.lookup_variable(name) {
                            if !sym.mutable {
                                self.ptype_error(
                                    format!("Assignment to immutable variable '{name}'"),
                                    span.clone(),
                                );
                            }
                            if !self.is_assignable(&value_type, &sym.ty) {
                                self.ptype_error(
                                    format!("Cannot assign {} to {}", value_type, sym.ty),
                                    span.clone(),
                                );
                            }
                            Some(sym.ty.clone())
                        } else {
                            self.undefined_var_error(name, span.clone());
                            None
                        }
                    }
                    Expr::ArrayAccess { array, index, span } => {
                        let index_type = self.visit_expr(index)?;
                        if !self.is_integer_type(&index_type) {
                            self.array_access_index_error(&index_type, index.span().clone());
                        }

                        let array_type = self.visit_expr(array)?;
                        if let Type::Array(elem_type, _) = &array_type {
                            if !self.is_assignable(&value_type, elem_type) {
                                self.ptype_error(
                                    format!("Cannot assign {value_type} to array element of type {elem_type}"),
                                    span.clone(),
                                );
                            }
                            Some(*elem_type.clone())
                        } else {
                            self.ptype_error(
                                format!("Indexing non-array type {array_type}"),
                                array.span().clone(),
                            );
                            None
                        }
                    }
                    _ => {
                        self.ptype_error(
                            "Invalid assignment target".to_string(),
                            target.span().clone(),
                        );
                        None
                    }
                }
            }
            Expr::Call {
                callee,
                arguments,
                span,
            } => {
                // Handle function calls by name lookup
                if let Expr::Variable {
                    name,
                    span: var_span,
                } = &**callee
                {
                    if let Some(func) = self.symbol_table.lookup_function(name) {
                        // Valida argomenti
                        if arguments.len() != func.parameters.len() {
                            self.ptype_error(
                                format!(
                                    "Function '{name}' expects {} arguments, found {}",
                                    func.parameters.len(),
                                    arguments.len()
                                ),
                                span.clone(),
                            );
                        }

                        // Validate argument types
                        for (i, (arg, param)) in arguments.iter().zip(&func.parameters).enumerate()
                        {
                            if let Some(arg_type) = self.visit_expr(arg) {
                                if !self.is_assignable(&arg_type, &param.type_annotation) {
                                    self.ptype_error(
                                        format!(
                                            "Argument {}: expected {}, found {}",
                                            i + 1,
                                            param.type_annotation,
                                            arg_type
                                        ),
                                        arg.span().clone(),
                                    );
                                }
                            }
                        }
                        return Some(func.return_type.clone());
                    } else {
                        self.ptype_error(format!("Undefined function '{name}'"), var_span.clone());
                    }
                } else {
                    self.ptype_error(
                        "Callee must be a function name".to_string(),
                        callee.span().clone(),
                    );
                }

                // Visit arguments even for invalid calls to catch nested errors
                for arg in arguments {
                    self.visit_expr(arg);
                }
                None
            }
            Expr::ArrayAccess {
                array,
                index,
                span: _,
            } => {
                let array_type = self.visit_expr(array)?;
                let index_type = self.visit_expr(index)?;

                if !self.is_integer_type(&index_type) {
                    self.array_access_index_error(&index_type, index.span().clone());
                }

                match array_type {
                    Type::Array(elem_type, _) => Some(*elem_type),
                    _ => {
                        self.ptype_error(
                            format!("Indexing non-array type {array_type}"),
                            array.span().clone(),
                        );
                        None
                    }
                }
            }
            // In the visit_expr method for ArrayLiteral
            Expr::ArrayLiteral { elements, span } => {
                if elements.is_empty() {
                    self.ptype_error(
                        "Array literal must have at least one element".to_string(),
                        span.clone(),
                    );
                    return None;
                }

                let len = elements.len();
                let mut element_type = None;
                for element in elements {
                    if let Some(ty) = self.visit_expr(element) {
                        if let Some(prev) = &element_type {
                            if !self.is_same_type(prev, &ty) {
                                self.ptype_error(
                                    format!("Array elements must be of the same type, found {prev} and {ty}"),
                                    element.span().clone(),
                                )
                            }
                        } else {
                            element_type = Some(ty);
                        }
                    }
                }

                element_type.map(|ty| {
                    // Create proper size expression with actual length
                    let size_expr = Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(len as i64)),
                        span: span.clone(),
                    };
                    Type::Array(Box::new(ty), Box::new(size_expr))
                })
            }
        }
    }

    // Helper functions
    pub fn type_of_number(&self, n: &Number) -> Type {
        match n {
            Number::I8(_) => Type::I8,
            Number::I16(_) => Type::I16,
            Number::I32(_) => Type::I32,
            Number::Integer(_) => Type::I64,
            Number::U8(_) => Type::U8,
            Number::U16(_) => Type::U16,
            Number::U32(_) => Type::U32,
            Number::UnsignedInteger(_) => Type::U64,
            Number::Float32(_) | Number::Scientific32(_, _) => Type::F32,
            Number::Float64(_) | Number::Scientific64(_, _) => Type::F64,
        }
    }

    pub fn is_assignable(&self, source: &Type, target: &Type) -> bool {
        match (source, target) {
            // Numeric promotions
            (Type::I8, Type::I16 | Type::I32 | Type::I64 | Type::F32 | Type::F64) => true,
            (Type::I16, Type::I32 | Type::I64 | Type::F32 | Type::F64) => true,
            (Type::I32, Type::I64 | Type::F64) => true,
            (Type::U8, Type::U16 | Type::U32 | Type::U64 | Type::F32 | Type::F64) => true,
            (Type::U16, Type::U32 | Type::U64 | Type::F32 | Type::F64) => true,
            (Type::U32, Type::U64 | Type::F64) => true,
            (Type::F32, Type::F64) => true,

            // Null pointer can be assigned to pointer types
            (Type::NullPtr, Type::Array(_, _) | Type::Vector(_)) => true,

            // Array assignment handling
            (Type::Array(source_elem, source_size), Type::Array(target_elem, target_size)) => {
                // Check if element types are compatible
                if !self.is_assignable(source_elem, target_elem) {
                    return false;
                }

                // Try to extract size values from expressions
                let get_size = |expr: &Box<Expr>| {
                    if let Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(n)),
                        ..
                    } = &**expr
                    {
                        Some(*n)
                    } else {
                        None
                    }
                };

                if let (Some(source_size_val), Some(target_size_val)) =
                    (get_size(source_size), get_size(target_size))
                {
                    // Compare sizes if both are integer literals
                    source_size_val == target_size_val
                } else {
                    // Fallback to expression comparison for non-literals
                    source_size == target_size
                }
            }

            // Exact matches for other types
            _ => source == target,
        }
    }

    fn is_integer_type(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
        )
    }

    fn is_same_type(&self, t1: &Type, t2: &Type) -> bool {
        t1 == t2
    }

    fn check_binary_op(
        &mut self,
        left: &Type,
        op: &BinaryOp,
        right: &Type,
        span: &SourceSpan,
    ) -> Option<Type> {
        match op {
            // Arithmetic operations
            BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo => {
                if self.is_numeric_type(left) && self.is_numeric_type(right) {
                    // Numeric promotion
                    Some(self.promote_numeric_types(left, right))
                } else {
                    self.ptype_error(
                        format!("Binary operator '{op:?}' requires numeric operands, found {left} and {right}"),
                        span.clone(),
                    );
                    None
                }
            }

            // Comparison operations
            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual => {
                if self.is_comparable(left, right) {
                    Some(Type::Bool)
                } else {
                    self.ptype_error(
                        format!("Comparison operator '{op:?}' requires compatible types, found {left} and {right}"),
                        span.clone(),
                    );
                    None
                }
            }

            // Logical operations
            BinaryOp::And | BinaryOp::Or => {
                if *left == Type::Bool && *right == Type::Bool {
                    Some(Type::Bool)
                } else {
                    self.ptype_error(
                        format!("Logical operator '{op:?}' requires boolean operands, found {left} and {right}"),
                        span.clone(),
                    );
                    None
                }
            }

            // Bitwise operations
            BinaryOp::BitwiseAnd
            | BinaryOp::BitwiseOr
            | BinaryOp::BitwiseXor
            | BinaryOp::ShiftLeft
            | BinaryOp::ShiftRight => {
                if self.is_integer_type(left) && self.is_integer_type(right) {
                    // Use the left operand's type as result type
                    Some(left.clone())
                } else {
                    self.ptype_error(
                        format!("Bitwise operator '{op:?}' requires integer operands, found {left} and {right}"),
                        span.clone(),
                    );

                    None
                }
            }
        }
    }

    fn check_unary_op(
        &mut self,
        op: &UnaryOp,
        expr_type: &Type,
        span: &SourceSpan,
    ) -> Option<Type> {
        match op {
            UnaryOp::Negate => {
                if self.is_numeric_type(expr_type) {
                    Some(expr_type.clone())
                } else {
                    self.ptype_error(
                        format!("Negation requires numeric operand, found {expr_type}"),
                        span.clone(),
                    );
                    None
                }
            }
            UnaryOp::Not => {
                if *expr_type == Type::Bool {
                    Some(Type::Bool)
                } else {
                    self.ptype_error(
                        format!("Logical not requires boolean operand, found {expr_type}"),
                        span.clone(),
                    );
                    None
                }
            }
        }
    }

    fn is_numeric_type(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::F32
                | Type::F64
        )
    }

    fn is_comparable(&self, t1: &Type, t2: &Type) -> bool {
        // Allow comparison between numeric types
        if self.is_numeric_type(t1) && self.is_numeric_type(t2) {
            return true;
        }

        // Allow comparison between same non-numeric types
        t1 == t2
    }

    pub fn promote_numeric_types(&self, t1: &Type, t2: &Type) -> Type {
        // Find the highest ranked type
        for ty in &HIERARCHY {
            if t1 == ty || t2 == ty {
                return ty.clone();
            }
        }

        // Fallback to I64
        Type::I64
    }

    /// Controlla la validità di un'istruzione return con errori dettagliati
    fn check_return_statement(&mut self, value: &Option<Expr>, span: &SourceSpan) {
        // Clona il tipo di ritorno per evitare conflitti di prestito
        let expected_type = self.symbol_table.current_function_return_type();

        let Some(expected) = expected_type else {
            self.ptype_error("Return statement outside function".to_string(), span.clone());
            return;
        };

        match value {
            Some(expr) => self.check_non_void_return(expr, &expected),
            None => self.check_void_return(&expected, span),
        }
    }

    /// Controlla return con valore (non void)
    fn check_non_void_return(&mut self, expr: &Expr, expected_type: &Type) {
        let Some(actual_type) = self.visit_expr(expr) else { return };

        if !self.is_assignable(&actual_type, expected_type) {
            self.ptype_error(
                format!(
                    "Return type mismatch: expected {expected_type}, found {actual_type}"
                ),
                expr.span().clone(),
            );
        }
    }

    /// Controlla return senza valore (void)
    fn check_void_return(&mut self, expected_type: &Type, span: &SourceSpan) {
        if *expected_type != Type::Void {
            self.ptype_error(
                format!(
                    "Function requires return type {expected_type}, found void",
                ),
                span.clone(),
            );
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
