// src/semantic/type_checker.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::semantic::symbol_table::*;
use crate::tokens::number::Number;

pub struct TypeChecker {
    symbol_table: SymbolTable,
    errors: Vec<CompileError>,
    //current_function: Option<Type>,
    in_loop: bool,
    return_type_stack: Vec<Type>,
}

// Gerarchia per la promozione dei tipi numerici
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

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            //current_function: None,
            in_loop: false,
            return_type_stack: Vec::new(),
        }
    }

    // Helper method for type errors
    fn type_error(&mut self, message: impl Into<String>, span: &SourceSpan) {
        self.errors.push(CompileError::TypeError {
            message: message.into(),
            span: span.clone(),
        });
    }

    pub fn check(&mut self, statements: &[Stmt]) -> Vec<CompileError> {
        self.visit_statements(statements);
        std::mem::take(&mut self.errors)
    }

    // Helper method per dichiarare simboli
    fn declare_symbol(&mut self, name: &str, symbol: Symbol) {
        if let Err(e) = self.symbol_table.declare(name, symbol) {
            self.errors.push(e);
        }
    }

    fn visit_statements(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression { expr } => {
                self.visit_expr(expr);
            }
            Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable,
                initializers,
                span,
            } => self.visit_var_declaration(
                variables,
                type_annotation,
                *is_mutable,
                initializers,
                span,
            ),
            Stmt::Function {
                name,
                parameters,
                return_type,
                body,
                span,
            } => self.visit_function(name, parameters, return_type, body, span),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                span,
            } => self.visit_if(condition, then_branch, else_branch.as_deref(), span),
            Stmt::While {
                condition,
                body,
                span,
            } => self.visit_while(condition, body, span),
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                span,
            } => self.visit_for(initializer, condition, increment, body, span),
            Stmt::Block { statements, span } => self.visit_block(statements, span),
            Stmt::Return { value, span } => self.visit_return(value.as_ref(), span),
            Stmt::Break { span } => self.visit_break(span),
            Stmt::Continue { span } => self.visit_continue(span),
            Stmt::MainFunction { body, span } => self.visit_main_function(body, span),
        }
    }

    fn visit_var_declaration(
        &mut self,
        variables: &[String],
        type_annotation: &Type,
        is_mutable: bool,
        initializers: &[Expr],
        span: &SourceSpan,
    ) {
        if variables.len() != initializers.len() {
            self.type_error(
                format!(
                    "Expected {} initializers, found {}",
                    variables.len(),
                    initializers.len()
                ),
                span,
            );
            return;
        }

        for (var_name, init_expr) in variables.iter().zip(initializers) {
            let init_type = self.visit_expr(init_expr);

            if !self.is_assignable(&init_type, type_annotation) {
                self.type_error(
                    format!("Cannot assign {init_type} to {var_name} of type {type_annotation}"),
                    init_expr.span(),
                );
            }

            self.declare_symbol(
                var_name,
                Symbol::Variable(VariableSymbol {
                    name: var_name.clone(),
                    ty: type_annotation.clone(),
                    mutable: is_mutable,
                    defined_at: span.clone(),
                    last_assignment: None,
                }),
            );
        }
    }

    fn visit_function(
        &mut self,
        name: &str,
        parameters: &[Parameter],
        return_type: &Type,
        body: &[Stmt],
        span: &SourceSpan,
    ) {
        let func_symbol = FunctionSymbol {
            name: name.to_string(),
            parameters: parameters.to_vec(),
            return_type: return_type.clone(),
            defined_at: span.clone(),
        };

        self.declare_symbol(name, Symbol::Function(func_symbol.clone()));

        self.symbol_table
            .push_scope(ScopeKind::Function, Some(span.clone()));
        self.return_type_stack.push(return_type.clone());

        for param in parameters {
            self.declare_symbol(
                &param.name,
                Symbol::Variable(VariableSymbol {
                    name: param.name.clone(),
                    ty: param.type_annotation.clone(),
                    mutable: true,
                    defined_at: param.span.clone(),
                    last_assignment: None,
                }),
            );
        }

        self.visit_statements(body);

        if *return_type != Type::Void && !self.function_has_return(body) {
            self.type_error("Function may not return a value", span);
        }

        self.return_type_stack.pop();
        self.symbol_table.pop_scope();
    }

    fn visit_main_function(&mut self, body: &[Stmt], span: &SourceSpan) {
        self.visit_function("main", &[], &Type::Void, body, span);
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &[Stmt],
        else_branch: Option<&[Stmt]>,
        _span: &SourceSpan,
    ) {
        let cond_type = self.visit_expr(condition);
        if cond_type != Type::Bool {
            self.type_error(
                format!("If condition must be bool, found {cond_type}"),
                condition.span(),
            );
        }

        self.symbol_table
            .push_scope(ScopeKind::Block, Some(condition.span().clone()));
        self.visit_statements(then_branch);
        self.symbol_table.pop_scope();

        if let Some(else_branch) = else_branch {
            self.symbol_table
                .push_scope(ScopeKind::Block, Some(condition.span().clone()));
            self.visit_statements(else_branch);
            self.symbol_table.pop_scope();
        }
    }

    fn visit_while(&mut self, condition: &Expr, body: &[Stmt], _span: &SourceSpan) {
        let cond_type = self.visit_expr(condition);
        if cond_type != Type::Bool {
            self.type_error(
                format!("While condition must be bool, found {cond_type}"),
                condition.span(),
            );
        }

        let was_in_loop = self.in_loop;
        self.in_loop = true;

        self.symbol_table
            .push_scope(ScopeKind::Block, Some(condition.span().clone()));
        self.visit_statements(body);
        self.symbol_table.pop_scope();

        self.in_loop = was_in_loop;
    }

    fn visit_for(
        &mut self,
        initializer: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &[Stmt],
        span: &SourceSpan,
    ) {
        self.symbol_table
            .push_scope(ScopeKind::Block, Some(span.clone()));

        if let Some(init) = initializer {
            self.visit_stmt(init);
        }

        if let Some(cond) = condition {
            let cond_type = self.visit_expr(cond);
            if cond_type != Type::Bool {
                self.type_error(
                    format!("For condition must be bool, found {cond_type}"),
                    cond.span(),
                );
            }
        }

        if let Some(inc) = increment {
            self.visit_expr(inc);
        }

        let was_in_loop = self.in_loop;
        self.in_loop = true;

        self.visit_statements(body);

        self.in_loop = was_in_loop;
        self.symbol_table.pop_scope();
    }

    fn visit_block(&mut self, statements: &[Stmt], span: &SourceSpan) {
        self.symbol_table
            .push_scope(ScopeKind::Block, Some(span.clone()));
        self.visit_statements(statements);
        self.symbol_table.pop_scope();
    }

    fn visit_return(&mut self, value: Option<&Expr>, span: &SourceSpan) {
        let expected_type = self.return_type_stack.last().cloned().unwrap_or(Type::Void);

        match (value, &expected_type) {
            (Some(expr), Type::Void) => {
                self.type_error("Cannot return a value from void function", expr.span());
            }
            (Some(expr), _) => {
                let actual_type = self.visit_expr(expr);
                if !self.is_assignable(&actual_type, &expected_type) {
                    self.type_error(
                        format!(
                            "Return type mismatch, expected {expected_type} found {actual_type}"
                        ),
                        expr.span(),
                    );
                }
            }
            (None, Type::Void) => {}
            (None, _) => {
                self.type_error(
                    format!("Expected return value of type {expected_type}"),
                    span,
                );
            }
        }
    }

    fn visit_break(&mut self, span: &SourceSpan) {
        if !self.in_loop {
            self.type_error("Break statement outside loop", span);
        }
    }

    fn visit_continue(&mut self, span: &SourceSpan) {
        if !self.in_loop {
            self.type_error("Continue statement outside loop", span);
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => self.visit_binary_expr(left, op, right, span),
            Expr::Unary { op, expr, span } => self.visit_unary_expr(op, expr, span),
            Expr::Grouping { expr, span: _ } => self.visit_expr(expr),
            Expr::Literal { value, span } => self.visit_literal(value, span),
            Expr::ArrayLiteral { elements, span } => self.visit_array_literal(elements, span),
            Expr::Variable { name, span } => self.visit_variable(name, span),
            Expr::Assign {
                target,
                value,
                span,
            } => self.visit_assign(target, value, span),
            Expr::Call {
                callee,
                arguments,
                span,
            } => self.visit_call(callee, arguments, span),
            Expr::ArrayAccess { array, index, span } => self.visit_array_access(array, index, span),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
        span: &SourceSpan,
    ) -> Type {
        let mut left_type = self.visit_expr(left);
        let mut right_type = self.visit_expr(right);

        // Distinzione tra operatori bitwise e altri operatori numerici
        if matches!(
            op,
            BinaryOp::BitwiseAnd
                | BinaryOp::BitwiseOr
                | BinaryOp::BitwiseXor
                | BinaryOp::ShiftLeft
                | BinaryOp::ShiftRight
        ) {
            // Solo tipi interi sono ammessi per operatori bitwise
            if self.is_integer_type(&left_type) && self.is_integer_type(&right_type) {
                let common_type = self.promote_numeric_types(&left_type, &right_type);
                left_type = common_type.clone();
                right_type = common_type;
            } else {
                self.type_error(
                    format!("Bitwise operations require integer types, found {left_type} and {right_type}"),
                    span,
                );
                return Type::Void;
            }
        } else if matches!(
            op,
            BinaryOp::Add
                | BinaryOp::Subtract
                | BinaryOp::Multiply
                | BinaryOp::Divide
                | BinaryOp::Modulo
                | BinaryOp::Equal
                | BinaryOp::NotEqual
                | BinaryOp::Less
                | BinaryOp::LessEqual
                | BinaryOp::Greater
                | BinaryOp::GreaterEqual
        ) && self.is_numeric(&left_type)
            && self.is_numeric(&right_type)
        {
            // Promozione numerica standard per operatori aritmetici e di confronto
            let common_type = self.promote_numeric_types(&left_type, &right_type);
            left_type = common_type.clone();
            right_type = common_type;
        }

        if !self.are_compatible(&left_type, &right_type) {
            self.type_error(
                format!("Type mismatch in binary operation: {left_type} and {right_type}"),
                span,
            );
            return Type::Void;
        }

        match op {
            BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo => {
                if !self.is_numeric(&left_type) {
                    self.type_error(
                        format!("Arithmetic operation not supported for {left_type}"),
                        left.span(),
                    );
                }
                left_type
            }
            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual => Type::Bool,
            BinaryOp::And | BinaryOp::Or => {
                if left_type != Type::Bool {
                    self.type_error(
                        format!("Logical operation requires bool, found {left_type}"),
                        left.span(),
                    );
                }
                Type::Bool
            }
            BinaryOp::BitwiseAnd
            | BinaryOp::BitwiseOr
            | BinaryOp::BitwiseXor
            | BinaryOp::ShiftLeft
            | BinaryOp::ShiftRight => left_type,
        }
    }

    fn visit_unary_expr(&mut self, op: &UnaryOp, expr: &Expr, _span: &SourceSpan) -> Type {
        let expr_type = self.visit_expr(expr);

        match op {
            UnaryOp::Negate => {
                if !self.is_numeric(&expr_type) {
                    self.type_error(
                        format!("Negation requires numeric type, found {expr_type}"),
                        expr.span(),
                    );
                }
                expr_type
            }
            UnaryOp::Not => {
                if expr_type != Type::Bool {
                    self.type_error(
                        format!("Logical not requires bool, found {expr_type}"),
                        expr.span(),
                    );
                }
                Type::Bool
            }
        }
    }

    fn visit_literal(&mut self, value: &LiteralValue, _span: &SourceSpan) -> Type {
        match value {
            LiteralValue::Number(n) => self.type_of_number(n),
            LiteralValue::StringLit(_) => Type::String,
            LiteralValue::CharLit(_) => Type::Char,
            LiteralValue::Bool(_) => Type::Bool,
            LiteralValue::Nullptr => Type::NullPtr,
        }
    }

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

    fn visit_array_literal(&mut self, elements: &[Expr], span: &SourceSpan) -> Type {
        if elements.is_empty() {
            self.type_error(
                "Array literals must have at least one element for type inference",
                span,
            );
            return Type::Void;
        }

        let first_type = self.visit_expr(&elements[0]);
        for element in &elements[1..] {
            let element_type = self.visit_expr(element);
            if element_type != first_type {
                self.type_error(
                    format!(
                        "Array element type mismatch: expected {first_type}, found {element_type}"
                    ),
                    element.span(),
                );
            }
        }

        Type::Array(
            Box::new(first_type),
            Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(elements.len() as i64)),
                span: span.clone(),
            }),
        )
    }

    fn visit_variable(&mut self, name: &str, span: &SourceSpan) -> Type {
        match self.symbol_table.lookup_variable(name) {
            Some(var) => var.ty.clone(),
            None => {
                if self.symbol_table.lookup_function(name).is_some() {
                    self.type_error(format!("'{name}' is a function, not a variable"), span);
                } else {
                    self.type_error(format!("Undefined variable '{name}'"), span);
                }
                Type::Void
            }
        }
    }

    fn visit_assign(&mut self, target: &Expr, value: &Expr, _span: &SourceSpan) -> Type {
        let target_type = match target {
            Expr::Variable { name, span } => match self.symbol_table.lookup_variable(name) {
                Some(var) => {
                    if !var.mutable {
                        self.type_error(
                            format!("Cannot assign to immutable variable '{name}'"),
                            span,
                        );
                    }
                    var.ty.clone()
                }
                None => {
                    self.type_error(format!("Undefined variable '{name}'"), span);
                    Type::Void
                }
            },
            Expr::ArrayAccess { array, index: _ , span } => {
                let array_type = self.visit_expr(array);
                if let Type::Array(element_type, _) = array_type {
                    *element_type
                } else {
                    self.type_error(format!("Cannot index non-array type {array_type}"), span);
                    Type::Void
                }
            }
            _ => {
                self.type_error("Invalid assignment target", target.span());
                Type::Void
            }
        };

        let value_type = self.visit_expr(value);

        if !self.is_assignable(&value_type, &target_type) {
            self.type_error(
                format!("Cannot assign {value_type} to {target_type}"),
                value.span(),
            );
        }

        target_type
    }

    fn visit_call(&mut self, callee: &Expr, arguments: &[Expr], span: &SourceSpan) -> Type {
        let callee_name = if let Expr::Variable { name, .. } = callee {
            name
        } else {
            self.type_error("Callee must be a function name", callee.span());
            for arg in arguments {
                self.visit_expr(arg);
            }
            return Type::Void;
        };

        let func = match self.symbol_table.lookup_function(&callee_name) {
            Some(func) => func,
            None => {
                self.type_error(format!("Undefined function '{callee_name}'"), callee.span());
                for arg in arguments {
                    self.visit_expr(arg);
                }
                return Type::Void;
            }
        };

        if arguments.len() != func.parameters.len() {
            self.type_error(
                format!(
                    "Expected {} arguments, found {}",
                    func.parameters.len(),
                    arguments.len()
                ),
                span,
            );
        }

        for (i, (arg, param)) in arguments.iter().zip(&func.parameters).enumerate() {
            let arg_type = self.visit_expr(arg);
            if !self.is_assignable(&arg_type, &param.type_annotation) {
                self.type_error(
                    format!(
                        "Argument {}: expected {}, found {}",
                        i + 1,
                        param.type_annotation,
                        arg_type
                    ),
                    arg.span(),
                );
            }
        }

        func.return_type.clone()
    }

    fn visit_array_access(&mut self, array: &Expr, index: &Expr, _span: &SourceSpan) -> Type {
        let array_type = self.visit_expr(array);
        let index_type = self.visit_expr(index);

        if !self.is_integer_type(&index_type) {
            self.type_error(
                format!("Array index must be integer, found {index_type}"),
                index.span(),
            );
        }

        if let Type::Array(element_type, _) = array_type {
            *element_type
        } else {
            self.type_error(
                format!("Cannot index non-array type {array_type}"),
                array.span(),
            );
            Type::Void
        }
    }

    // Funzione per la promozione automatica dei tipi numerici
    pub fn promote_numeric_types(&self, t1: &Type, t2: &Type) -> Type {
        // Trova il tipo con rango più alto nella gerarchia
        for ty in &HIERARCHY {
            if t1 == ty || t2 == ty {
                return ty.clone();
            }
        }
        // This should never happen if HIERARCHY contains all numeric types
        // Return the first type as a fallback to maintain type safety
        t1.clone()
    }

    // FUNZIONI DI VERIFICA TIPI
    pub fn is_assignable(&self, source: &Type, target: &Type) -> bool {
        match (source, target) {
            // Promozioni numeriche
            (Type::I8, Type::I16 | Type::I32 | Type::I64 | Type::F32 | Type::F64) => true,
            (Type::I16, Type::I32 | Type::I64 | Type::F32 | Type::F64) => true,
            (Type::I32, Type::I64 | Type::F32 | Type::F64) => true,
            (Type::I64, Type::F64) => true,

            (Type::U8, Type::U16 | Type::U32 | Type::U64 | Type::F32 | Type::F64) => true,
            (Type::U16, Type::U32 | Type::U64 | Type::F32 | Type::F64) => true,
            (Type::U32, Type::U64 | Type::F32 | Type::F64) => true,
            (Type::U64, Type::F64) => true,

            (Type::F32, Type::F64) => true,

            // Nullptr assegnabile a tipi puntatore
            (Type::NullPtr, Type::Array(_, _) | Type::Vector(_) | Type::Custom(_)) => true,

            // Char assegnabile a String
            (Type::Char, Type::String) => true,

            // Array: richiede tipi compatibili e dimensioni uguali
            (Type::Array(source_elem, source_size), Type::Array(target_elem, target_size)) => {
                if !self.is_assignable(source_elem, target_elem) {
                    return false;
                }

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

                match (get_size(source_size), get_size(target_size)) {
                    (Some(source_val), Some(target_val)) => source_val == target_val,
                    _ => false,
                }
            }

            // Vector: richiede tipi elemento compatibili
            (Type::Vector(source_elem), Type::Vector(target_elem)) => {
                self.is_assignable(source_elem, target_elem)
            }

            // Tipi identici
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

    fn are_compatible(&self, t1: &Type, t2: &Type) -> bool {
        self.is_assignable(t1, t2) || self.is_assignable(t2, t1)
    }

    fn is_numeric(&self, ty: &Type) -> bool {
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

    fn function_has_return(&self, body: &[Stmt]) -> bool {
        for stmt in body {
            match stmt {
                Stmt::Return { .. } => return true,
                Stmt::If {
                    then_branch,
                    else_branch,
                    ..
                } => {
                    let then_has_return = self.function_has_return(then_branch);
                    let else_has_return = else_branch
                        .as_ref()
                        .map(|b| self.function_has_return(b))
                        .unwrap_or(false);

                    if then_has_return && else_has_return {
                        return true;
                    }
                }
                Stmt::Block { statements, .. } => {
                    if self.function_has_return(statements) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
