// src/semantic/type_checker.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::semantic::symbol_table::*;
use crate::tokens::number::Number;

pub struct TypeChecker {
    symbol_table: SymbolTable,
    errors: Vec<CompileError>,
    current_function_return_type: Option<Type>,
    in_loop: bool,
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
            current_function_return_type: None,
            in_loop: false,
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
            Stmt::Block { statements, .. } => {
                for stmt in statements {
                    self.visit_stmt_first_pass(stmt);
                }
            }
            Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable,
                initializers: _,
                span,
            } => {
                for var in variables {
                    let symbol = Symbol::Variable(VariableSymbol {
                        name: var.clone(),
                        ty: type_annotation.clone(),
                        mutable: *is_mutable,
                        defined_at: span.clone(),
                        last_assignment: None,
                    });
                    self.declare_symbol(var, symbol);
                }
            }
            Stmt::Function {
                name,
                parameters,
                return_type,
                body,
                span,
            } => {
                // Declare function symbol
                let func_symbol = Symbol::Function(FunctionSymbol {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                    defined_at: span.clone(),
                });
                self.declare_symbol(name, func_symbol);

                // Push scope and declare parameters
                self.symbol_table.push_scope();
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

                // Process body in first pass (without nested scope changes)
                for stmt in body {
                    self.visit_stmt_first_pass(stmt);
                }
                // Do NOT pop scope here - retain for second pass
            }
            Stmt::MainFunction { body, span } => {
                // Declare main symbol
                let func_symbol = Symbol::Function(FunctionSymbol {
                    name: "main".to_string(),
                    parameters: Vec::new(),
                    return_type: Type::Void,
                    defined_at: span.clone(),
                });
                self.declare_symbol("main", func_symbol);

                // Push scope and process body
                self.symbol_table.push_scope();
                for stmt in body {
                    self.visit_stmt_first_pass(stmt);
                }
                // Do NOT pop scope
            }
            _ => {} // Other statements don't declare symbols in first pass
        }
    }

    //#[allow(unreachable_patterns)]
    #[allow(clippy::collapsible_if)]
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block {
                statements,
                span: _,
            } => {
                self.symbol_table.push_scope();
                for stmt in statements {
                    self.visit_stmt(stmt);
                }
                self.symbol_table.pop_scope();
            }
            Stmt::Function {
                name: _,
                parameters: _,
                return_type,
                body,
                span: _,
            } => {
                // Set current function context
                let old_return_type = self
                    .current_function_return_type
                    .replace(return_type.clone());

                // Process function body
                for stmt in body {
                    self.visit_stmt(stmt);
                }

                self.current_function_return_type = old_return_type;
            }
            Stmt::MainFunction { body, span: _ } => {
                let old_return_type = self.current_function_return_type.replace(Type::Void);
                for stmt in body {
                    self.visit_stmt(stmt);
                }
                self.current_function_return_type = old_return_type;
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
                if let Some(expected) = &self.current_function_return_type.clone() {
                    match value {
                        Some(expr) => {
                            if let Some(actual) = self.visit_expr(expr) {
                                if !self.is_assignable(&actual, expected) {
                                    self.ptype_error(format!(
                                        "Return type mismatch: expected {expected}, found {actual}",
                                    ), expr.span().clone());
                                }
                            }
                        }
                        None if *expected != Type::Void => self.ptype_error(
                            format!("Function requires return type {expected}, found void"),
                            span.clone(),
                        ),
                        _ => {}
                    }
                } else {
                    self.ptype_error(
                        "Return statement outside function".to_string(),
                        span.clone(),
                    );
                }
            }
            Stmt::Break { span } | Stmt::Continue { span } => {
                if !self.in_loop {
                    self.ptype_error("Break/continue outside loop".to_string(), span.clone());
                }
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
                    if let Some(symbol) = self.symbol_table.lookup(name) {
                        match symbol {
                            Symbol::Function(func) => {
                                // Validate argument count
                                if arguments.len() != func.parameters.len() {
                                    self.ptype_error(
                                        format!(
                                            "Function '{name}' expects {} arguments, found {}",
                                            func.parameters.len(),
                                            arguments.len()
                                        ),
                                        span.clone(),
                                    )
                                }

                                // Validate argument types
                                for (i, (arg, param)) in
                                    arguments.iter().zip(&func.parameters).enumerate()
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
                                            )
                                        }
                                    }
                                }
                                return Some(func.return_type.clone());
                            }
                            _ => {
                                self.ptype_error(
                                    format!("'{name}' is not a function"),
                                    var_span.clone(),
                                );
                            }
                        }
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
            } //_ => unimplemented!("Unsupported expression in type checker"),
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
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
