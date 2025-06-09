use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::semantic::symbol_table::{FunctionSymbol, Symbol, SymbolTable, VariableSymbol};
use crate::tokens::number::Number;

#[derive(Debug, Default)]
pub struct TypeChecker {
    symbol_table: SymbolTable,
    errors: Vec<CompileError>,
    current_return_type: Option<Type>,
    in_loop: bool,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            current_return_type: None,
            in_loop: false,
        }
    }

    pub fn check(&mut self, statements: &[Stmt]) -> Vec<CompileError> {
        // First pass: dichiarazione simboli top-level
        for stmt in statements {
            self.declare_symbols(stmt);
        }

        // Second pass: type checking
        for stmt in statements {
            self.check_stmt(stmt);
        }

        std::mem::take(&mut self.errors)
    }

    fn push_error(&mut self, error: CompileError) {
        self.errors.push(error);
    }

    fn push_type_error(&mut self, message: String, span: SourceSpan) {
        self.push_error(CompileError::TypeError { message, span });
    }

    fn declare_symbols(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function {
                name,
                parameters,
                return_type,
                span,
                ..
            } => {
                let func_symbol = FunctionSymbol {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                    defined_at: span.clone(),
                };
                if let Err(e) = self
                    .symbol_table
                    .declare(name, Symbol::Function(func_symbol))
                {
                    self.push_error(e);
                }
            }
            Stmt::VarDeclaration {
                variables,
                type_annotation,
                is_mutable,
                span,
                ..
            } => {
                for var in variables {
                    let var_symbol = VariableSymbol {
                        name: var.clone(),
                        ty: type_annotation.clone(),
                        mutable: *is_mutable,
                        defined_at: span.clone(),
                        last_assignment: None,
                    };
                    if let Err(e) = self.symbol_table.declare(var, Symbol::Variable(var_symbol)) {
                        self.push_error(e);
                    }
                }
            }
            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression { expr } => {
                self.check_expr(expr);
            }

            Stmt::VarDeclaration {
                variables,
                type_annotation,
                initializers,
                is_mutable,
                span,
                ..
            } => {
                for (i, var) in variables.iter().enumerate() {
                    // Variabile già dichiarata in prima passata: verifichiamo che esista
                    if self.symbol_table.lookup_variable(var).is_none() {
                        self.push_type_error(
                            format!("Variable '{var}' not found in symbol table"),
                            span.clone(),
                        );
                        continue;
                    }

                    if let Some(init) = initializers.get(i) {
                        let init_type = self.check_expr(init);
                        if !self.is_assignable(&init_type, type_annotation) {
                            self.push_type_error(
                                format!(
                                    "Cannot assign {init_type} to {type_annotation} variable '{var}'"
                                ),
                                init.span().clone(),
                            );
                        }
                    } else if let Type::Array(..) = type_annotation {
                        self.push_type_error(
                            "Array type requires initializer".to_string(),
                            span.clone(),
                        );
                    }
                }
            }

            Stmt::Function {
                parameters,
                return_type,
                body,
                ..
            } => {
                self.symbol_table.push_scope();
                self.current_return_type = Some(return_type.clone());

                // Aggiungiamo i parametri nella nuova scope
                for param in parameters {
                    let var_symbol = VariableSymbol {
                        name: param.name.clone(),
                        ty: param.type_annotation.clone(),
                        mutable: false,
                        defined_at: param.span.clone(),
                        last_assignment: None,
                    };
                    if let Err(e) = self
                        .symbol_table
                        .declare(&param.name, Symbol::Variable(var_symbol))
                    {
                        self.push_error(e);
                    }
                }

                // Controlliamo il body della funzione
                for stmt in body {
                    self.check_stmt(stmt);
                }

                self.symbol_table.pop_scope();
                self.current_return_type = None;
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let cond_type = self.check_expr(condition);
                if !self.is_bool(&cond_type) {
                    self.push_type_error(
                        format!("Condition must be bool, found {cond_type}"),
                        condition.span().clone(),
                    );
                }

                // Then branch
                self.symbol_table.push_scope();
                for stmt in then_branch {
                    self.check_stmt(stmt);
                }
                self.symbol_table.pop_scope();

                if let Some(else_statements) = else_branch {
                    self.symbol_table.push_scope();
                    for stmt in else_statements {
                        self.check_stmt(stmt);
                    }
                    self.symbol_table.pop_scope();
                }
            }

            Stmt::Block { statements, .. } => {
                self.symbol_table.push_scope();
                for stmt in statements {
                    self.check_stmt(stmt);
                }
                self.symbol_table.pop_scope();
            }

            Stmt::Return { value, span } => {
                // Cloniamo il tipo di ritorno per evitare problemi di borrow
                let expected = match self.current_return_type.clone() {
                    Some(ty) => ty,
                    None => {
                        self.push_type_error("Return outside function".to_string(), span.clone());
                        return;
                    }
                };

                let value_type = value
                    .as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(Type::Void);

                if !self.is_assignable(&value_type, &expected) {
                    self.push_type_error(
                        format!("Return type mismatch: expected {expected}, found {value_type}"),
                        value
                            .as_ref()
                            .map(|e| e.span().clone())
                            .unwrap_or(span.clone()),
                    );
                }
            }

            // ======= BREAK / CONTINUE =======
            Stmt::Break { span } | Stmt::Continue { span } => {
                self.check_break_or_continue(stmt, span);
            }

            // ======= MAIN FUNCTION =======
            Stmt::MainFunction { body, .. } => {
                self.symbol_table.push_scope();
                for stmt in body {
                    self.check_stmt(stmt);
                }
                self.symbol_table.pop_scope();
            } // ======= STUB FUTURI LOOP =======
              // Quando verranno aggiunti i costrutti di loop (While, For, ecc.) nel parser,
              // basterà scommentare questi match arm e adattare la logica:
              //
              // Stmt::While { condition, body, span } => {
              //     let cond_type = self.check_expr(condition);
              //     if !self.is_bool(&cond_type) {
              //         self.push_type_error(
              //             format!("Condition must be bool, found {}", self.type_name(&cond_type)),
              //             condition.span().clone(),
              //         );
              //     }
              //
              //     // Entriamo nel loop
              //     let prev_in_loop = self.in_loop;
              //     self.in_loop = true;
              //
              //     self.symbol_table.push_scope();
              //     for stmt in body {
              //         self.check_stmt(stmt);
              //     }
              //     self.symbol_table.pop_scope();
              //
              //     // Uscita dal loop
              //     self.in_loop = prev_in_loop;
              // }
              //
              // Stmt::For { initializer, condition, increment, body, span } => {
              //     // Controlli analoghi a While, + inizializzatore e incremento
              //     // ...
              // }
        }
    }

    /// Estrae tutta la logica di controllo di `break` e `continue`.
    fn check_break_or_continue(&mut self, stmt: &Stmt, span: &SourceSpan) {
        if !self.in_loop {
            let msg = match stmt {
                Stmt::Break { .. } => "Break outside loop",
                Stmt::Continue { .. } => "Continue outside loop",
                _ => unreachable!(),
            };
            self.push_type_error(msg.to_string(), span.clone());
        }
    }

    #[allow(clippy::collapsible_if)]
    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let left_type = self.check_expr(left);
                let right_type = self.check_expr(right);

                match op {
                    BinaryOp::Add
                    | BinaryOp::Subtract
                    | BinaryOp::Multiply
                    | BinaryOp::Divide
                    | BinaryOp::Modulo => {
                        if !self.is_numeric(&left_type) || !self.is_numeric(&right_type) {
                            self.push_type_error(
                                format!(
                                    "Arithmetic operands must be numeric, found {left_type} and {right_type}"
                                ),
                                span.clone(),
                            );
                            return Type::Void;
                        }
                        if left_type != right_type {
                            self.push_type_error(
                                format!("Operand type mismatch: {left_type} and {right_type}"),
                                span.clone(),
                            );
                        }
                        left_type
                    }
                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual => {
                        if !self.is_comparable(&left_type, &right_type) {
                            self.push_type_error(
                                format!("Cannot compare {left_type} and {right_type}"),
                                span.clone(),
                            );
                        }
                        Type::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if !self.is_bool(&left_type) || !self.is_bool(&right_type) {
                            self.push_type_error(
                                "Logical operands must be bool".to_string(),
                                span.clone(),
                            );
                        }
                        Type::Bool
                    }
                    _ => right_type, // Per bitwise ops, ritorniamo il tipo di destra
                }
            }

            Expr::Unary { op, expr, span } => {
                let expr_type = self.check_expr(expr);
                match op {
                    UnaryOp::Negate => {
                        if !self.is_numeric(&expr_type) {
                            self.push_type_error(
                                "Negation requires numeric operand".to_string(),
                                span.clone(),
                            );
                        }
                        expr_type
                    }
                    UnaryOp::Not => {
                        if !self.is_bool(&expr_type) {
                            self.push_type_error(
                                "Logical not requires bool operand".to_string(),
                                span.clone(),
                            );
                        }
                        Type::Bool
                    }
                }
            }

            Expr::Literal { value, .. } => match value {
                LiteralValue::Number(_) => Type::I32, // Default number type
                LiteralValue::StringLit(_) => Type::String,
                LiteralValue::CharLit(_) => Type::Char,
                LiteralValue::Bool(_) => Type::Bool,
                LiteralValue::Nullptr => Type::Void,
            },

            Expr::Variable { name, span } => match self.symbol_table.lookup_variable(name) {
                Some(var) => var.ty.clone(),
                None => {
                    self.push_type_error(format!("Undefined variable '{name}'"), span.clone());
                    Type::Void
                }
            },

            Expr::Assign {
                target,
                value,
                span,
            } => {
                let value_type = self.check_expr(value);
                let target_type = self.check_expr(target);

                if !self.is_assignable(&value_type, &target_type) {
                    self.push_type_error(
                        format!("Cannot assign {value_type} to {target_type}"),
                        span.clone(),
                    );
                }

                // Controllo mutabilità
                if let Expr::Variable { name, .. } = target.as_ref() {
                    if let Some(var) = self.symbol_table.lookup_variable(name) {
                        if !var.mutable {
                            self.push_type_error(
                                format!("Cannot assign to immutable variable '{name}'"),
                                span.clone(),
                            );
                        }
                    }
                }

                target_type
            }

            Expr::Call {
                callee,
                arguments,
                span,
            } => {
                // Cerchiamo la funzione direttamente per nome
                if let Expr::Variable { name, .. } = callee.as_ref() {
                    if let Some(func) = self.symbol_table.lookup_function(name) {
                        // Controllo numero di argomenti
                        if func.parameters.len() != arguments.len() {
                            self.push_type_error(
                                format!(
                                    "Expected {} arguments, found {}",
                                    func.parameters.len(),
                                    arguments.len()
                                ),
                                span.clone(),
                            );
                            return func.return_type.clone();
                        }

                        // Controllo tipi degli argomenti
                        for (i, (arg, param)) in arguments.iter().zip(&func.parameters).enumerate()
                        {
                            let arg_type = self.check_expr(arg);
                            if !self.is_assignable(&arg_type, &param.type_annotation) {
                                self.push_type_error(
                                    format!(
                                        "Argument {}: expected {}, found {}",
                                        i + 1,
                                        &param.type_annotation,
                                        &arg_type
                                    ),
                                    span.clone(),
                                );
                            }
                        }

                        return func.return_type.clone();
                    }
                }

                self.push_type_error("Invalid function call".to_string(), span.clone());
                Type::Void
            }

            Expr::ArrayAccess { array, index, .. } => {
                let array_type = self.check_expr(array);
                let index_type = self.check_expr(index);

                if !self.is_integer(&index_type) {
                    self.push_type_error(
                        "Array index must be integer".to_string(),
                        index.span().clone(),
                    );
                }

                match array_type {
                    Type::Array(element_type, _) => *element_type.clone(),
                    Type::Vector(element_type) => *element_type.clone(),
                    _ => {
                        self.push_type_error(
                            "Cannot index non-array type".to_string(),
                            array.span().clone(),
                        );
                        Type::Void
                    }
                }
            }

            Expr::ArrayLiteral { elements, span } => {
                if elements.is_empty() {
                    self.push_type_error(
                        "Array literal must have at least one element".to_string(),
                        span.clone(),
                    );
                    return Type::Array(
                        Box::new(Type::Void),
                        Box::new(Expr::null_expr(span.clone())),
                    );
                }

                let first_type = self.check_expr(&elements[0]);
                for element in elements.iter().skip(1) {
                    let element_type = self.check_expr(element);
                    if element_type != first_type {
                        self.push_type_error(
                            format!(
                                "Array element type mismatch: expected {first_type}, found {element_type}"
                            ),
                            element.span().clone(),
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

            _ => Type::Void, // Placeholders per altri casi non ancora supportati
        }
    }

    // Helper functions
    fn is_assignable(&self, from: &Type, to: &Type) -> bool {
        match (from, to) {
            // Numeric widening
            (Type::I8, Type::I16 | Type::I32 | Type::I64) => true,
            (Type::I16, Type::I32 | Type::I64) => true,
            (Type::I32, Type::I64) => true,
            (Type::U8, Type::U16 | Type::U32 | Type::U64) => true,
            (Type::U16, Type::U32 | Type::U64) => true,
            (Type::U32, Type::U64) => true,
            (Type::F32, Type::F64) => true,

            // Altri casi richiedono esatta corrispondenza
            _ => from == to,
        }
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

    fn is_integer(&self, ty: &Type) -> bool {
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

    fn is_bool(&self, ty: &Type) -> bool {
        ty == &Type::Bool
    }

    fn is_comparable(&self, left: &Type, right: &Type) -> bool {
        // Permettiamo il confronto tra tipi numerici
        if self.is_numeric(left) && self.is_numeric(right) {
            return true;
        }
        // Altrimenti richiediamo esatta corrispondenza
        left == right
    }
}
