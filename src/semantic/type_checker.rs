use crate::error::compile_error::CompileError;
use crate::parser::ast::*;
use crate::semantic::symbol_table::{SymbolTable, Symbol, VariableSymbol, FunctionSymbol};
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
        // First pass: declare all top-level symbols
        for stmt in statements {
            self.declare_symbols(stmt);
        }
        
        // Second pass: type checking
        for stmt in statements {
            self.check_stmt(stmt);
        }
        
        std::mem::take(&mut self.errors)
    }

    fn declare_symbols(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, parameters, return_type, span, .. } => {
                let func_symbol = FunctionSymbol {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                    defined_at: span.clone(),
                };
                if let Err(e) = self.symbol_table.declare(name, Symbol::Function(func_symbol)) {
                    self.errors.push(e);
                }
            }
            Stmt::VarDeclaration { variables, type_annotation, is_mutable, span, .. } => {
                for var in variables {
                    let var_symbol = VariableSymbol {
                        name: var.clone(),
                        ty: type_annotation.clone(),
                        mutable: *is_mutable,
                        defined_at: span.clone(),
                        last_assignment: None,
                    };
                    if let Err(e) = self.symbol_table.declare(var, Symbol::Variable(var_symbol)) {
                        self.errors.push(e);
                    }
                }
            }
            _ => {}
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression { expr } => {
                self.check_expr(expr);
            }
            Stmt::VarDeclaration { variables, type_annotation, initializers, span, .. } => {
                for (i, var) in variables.iter().enumerate() {
                    if let Some(init) = initializers.get(i) {
                        let init_type = self.check_expr(init);
                        if !self.is_assignable(&init_type, type_annotation) {
                            self.errors.push(CompileError::TypeError {
                                message: format!(
                                    "Cannot assign {} to {} variable '{}'",
                                    self.type_name(&init_type),
                                    self.type_name(type_annotation),
                                    var
                                ),
                                span: init.span().clone(),
                            });
                        }
                    } else if let Type::Array(..) = type_annotation {
                        self.errors.push(CompileError::TypeError {
                            message: "Array type requires initializer".to_string(),
                            span: span.clone(),
                        });
                    }
                }
            }
            Stmt::Function { parameters, return_type, body, .. } => {
                self.symbol_table.push_scope();
                self.current_return_type = Some(return_type.clone());
                
                // Add parameters to scope
                for param in parameters {
                    let var_symbol = VariableSymbol {
                        name: param.name.clone(),
                        ty: param.type_annotation.clone(),
                        mutable: false,
                        defined_at: param.span.clone(),
                        last_assignment: None,
                    };
                    if let Err(e) = self.symbol_table.declare(
                        &param.name, 
                        Symbol::Variable(var_symbol)
                    ) {
                        self.errors.push(e);
                    }
                }
                
                // Check function body
                for stmt in body {
                    self.check_stmt(stmt);
                }
                
                self.symbol_table.pop_scope();
                self.current_return_type = None;
            }
            Stmt::If { condition, then_branch, else_branch, .. } => {
                let cond_type = self.check_expr(condition);
                if !self.is_bool(&cond_type) {
                    self.errors.push(CompileError::TypeError {
                        message: format!("Condition must be bool, found {}", 
                            self.type_name(&cond_type)),
                        span: condition.span().clone(),
                    });
                }
                
                // Then branch
                self.symbol_table.push_scope();
                for stmt in then_branch {
                    self.check_stmt(stmt);
                }
                self.symbol_table.pop_scope();
                
                // Else branch
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
                // Clone return type to avoid borrow issues
                let return_type = match self.current_return_type.clone() {
                    Some(ty) => ty,
                    None => {
                        self.errors.push(CompileError::TypeError {
                            message: "Return outside function".to_string(),
                            span: span.clone(),
                        });
                        return;
                    }
                };
                
                let value_type = value.as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(Type::Void);
                
                if !self.is_assignable(&value_type, &return_type) {
                    self.errors.push(CompileError::TypeError {
                        message: format!(
                            "Return type mismatch: expected {}, found {}",
                            self.type_name(&return_type),
                            self.type_name(&value_type)
                        ),
                        span: value.as_ref().map(|e| e.span().clone()).unwrap_or(span.clone()),
                    });
                }
            }
            Stmt::Break { span } | Stmt::Continue { span } => {
                if !self.in_loop {
                    self.errors.push(CompileError::TypeError {
                        message: "Break/continue outside loop".to_string(),
                        span: span.clone(),
                    });
                }
            }
            Stmt::MainFunction { body, .. } => {
                self.symbol_table.push_scope();
                for stmt in body {
                    self.check_stmt(stmt);
                }
                self.symbol_table.pop_scope();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Binary { left, op, right, span } => {
                let left_type = self.check_expr(left);
                let right_type = self.check_expr(right);
                
                match op {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply |
                    BinaryOp::Divide | BinaryOp::Modulo => {
                        if !self.is_numeric(&left_type) || !self.is_numeric(&right_type) {
                            self.errors.push(CompileError::TypeError {
                                message: format!(
                                    "Arithmetic operands must be numeric, found {} and {}",
                                    self.type_name(&left_type),
                                    self.type_name(&right_type)
                                ),
                                span: span.clone(),
                            });
                            return Type::Void;
                        }
                        if left_type != right_type {
                            self.errors.push(CompileError::TypeError {
                                message: format!(
                                    "Operand type mismatch: {} and {}",
                                    self.type_name(&left_type),
                                    self.type_name(&right_type)
                                ),
                                span: span.clone(),
                            });
                        }
                        left_type
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Less |
                    BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                        if !self.is_comparable(&left_type, &right_type) {
                            self.errors.push(CompileError::TypeError {
                                message: format!(
                                    "Cannot compare {} and {}",
                                    self.type_name(&left_type),
                                    self.type_name(&right_type)
                                ),
                                span: span.clone(),
                            });
                        }
                        Type::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if !self.is_bool(&left_type) || !self.is_bool(&right_type) {
                            self.errors.push(CompileError::TypeError {
                                message: "Logical operands must be bool".to_string(),
                                span: span.clone(),
                            });
                        }
                        Type::Bool
                    }
                    _ => right_type, // For bitwise ops, return left type
                }
            }
            Expr::Unary { op, expr, span } => {
                let expr_type = self.check_expr(expr);
                match op {
                    UnaryOp::Negate => {
                        if !self.is_numeric(&expr_type) {
                            self.errors.push(CompileError::TypeError {
                                message: "Negation requires numeric operand".to_string(),
                                span: span.clone(),
                            });
                        }
                        expr_type
                    }
                    UnaryOp::Not => {
                        if !self.is_bool(&expr_type) {
                            self.errors.push(CompileError::TypeError {
                                message: "Logical not requires bool operand".to_string(),
                                span: span.clone(),
                            });
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
            Expr::Variable { name, span } => {
                match self.symbol_table.lookup_variable(name) {
                    Some(var) => var.ty.clone(),
                    None => {
                        self.errors.push(CompileError::TypeError {
                            message: format!("Undefined variable '{}'", name),
                            span: span.clone(),
                        });
                        Type::Void
                    }
                }
            }
            Expr::Assign { target, value, span } => {
                let value_type = self.check_expr(value);
                let target_type = self.check_expr(target);
                
                if !self.is_assignable(&value_type, &target_type) {
                    self.errors.push(CompileError::TypeError {
                        message: format!(
                            "Cannot assign {} to {}",
                            self.type_name(&value_type),
                            self.type_name(&target_type)
                        ),
                        span: span.clone(),
                    });
                }
                
                // Check mutability
                if let Expr::Variable { name, .. } = target.as_ref() {
                    if let Some(var) = self.symbol_table.lookup_variable(name) {
                        if !var.mutable {
                            self.errors.push(CompileError::TypeError {
                                message: format!("Cannot assign to immutable variable '{}'", name),
                                span: span.clone(),
                            });
                        }
                    }
                }
                
                target_type
            }
            Expr::Call { callee, arguments, span } => {
                // Lookup function directly by name
                if let Expr::Variable { name, .. } = callee.as_ref() {
                    if let Some(func) = self.symbol_table.lookup_function(name) {
                        // Check argument count
                        if func.parameters.len() != arguments.len() {
                            self.errors.push(CompileError::TypeError {
                                message: format!(
                                    "Expected {} arguments, found {}",
                                    func.parameters.len(),
                                    arguments.len()
                                ),
                                span: span.clone(),
                            });
                            return func.return_type.clone();
                        }
                        
                        // Check argument types
                        for (i, (arg, param)) in arguments.iter().zip(&func.parameters).enumerate() {
                            let arg_type = self.check_expr(arg);
                            if !self.is_assignable(&arg_type, &param.type_annotation) {
                                self.errors.push(CompileError::TypeError {
                                    message: format!(
                                        "Argument {}: expected {}, found {}",
                                        i + 1,
                                        self.type_name(&param.type_annotation),
                                        self.type_name(&arg_type)
                                    ),
                                    span: arg.span().clone(),
                                });
                            }
                        }
                        
                        return func.return_type.clone();
                    }
                }
                
                self.errors.push(CompileError::TypeError {
                    message: "Invalid function call".to_string(),
                    span: span.clone(),
                });
                Type::Void
            }
            Expr::ArrayAccess { array, index, .. } => {
                let array_type = self.check_expr(array);
                let index_type = self.check_expr(index);
                
                if !self.is_integer(&index_type) {
                    self.errors.push(CompileError::TypeError {
                        message: "Array index must be integer".to_string(),
                        span: index.span().clone(),
                    });
                }
                
                match array_type {
                    Type::Array(element_type, _) => *element_type,
                    Type::Vector(element_type) => *element_type,
                    _ => {
                        self.errors.push(CompileError::TypeError {
                            message: "Cannot index non-array type".to_string(),
                            span: array.span().clone(),
                        });
                        Type::Void
                    }
                }
            }
            Expr::ArrayLiteral { elements, span } => {
                if elements.is_empty() {
                    self.errors.push(CompileError::TypeError {
                        message: "Array literal must have at least one element".to_string(),
                        span: span.clone(),
                    });
                    return Type::Array(Box::new(Type::Void), Box::new(Expr::null_expr(span.clone())));
                }
                
                let first_type = self.check_expr(&elements[0]);
                for element in elements.iter().skip(1) {
                    let element_type = self.check_expr(element);
                    if element_type != first_type {
                        self.errors.push(CompileError::TypeError {
                            message: format!(
                                "Array element type mismatch: expected {}, found {}",
                                self.type_name(&first_type),
                                self.type_name(&element_type)
                            ),
                            span: element.span().clone(),
                        });
                    }
                }
                
                Type::Array(
                    Box::new(first_type),
                    Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(elements.len() as i64)),
                        span: span.clone(),
                    })
                )
            }
            _ => Type::Void, // Placeholder for other expressions
        }
    }

    // Helper functions
    fn is_assignable(&self, from: &Type, to: &Type) -> bool {
        match (from, to) {
            // Numeric types can be assigned if compatible
            (Type::I8, Type::I16 | Type::I32 | Type::I64) => true,
            (Type::I16, Type::I32 | Type::I64) => true,
            (Type::I32, Type::I64) => true,
            (Type::U8, Type::U16 | Type::U32 | Type::U64) => true,
            (Type::U16, Type::U32 | Type::U64) => true,
            (Type::U32, Type::U64) => true,
            (Type::F32, Type::F64) => true,
            
            // All other cases require exact match
            _ => from == to,
        }
    }

    fn is_numeric(&self, ty: &Type) -> bool {
        matches!(ty, 
            Type::I8 | Type::I16 | Type::I32 | Type::I64 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64 |
            Type::F32 | Type::F64
        )
    }

    fn is_integer(&self, ty: &Type) -> bool {
        matches!(ty, 
            Type::I8 | Type::I16 | Type::I32 | Type::I64 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64
        )
    }

    fn is_bool(&self, ty: &Type) -> bool {
        ty == &Type::Bool
    }

    fn is_comparable(&self, left: &Type, right: &Type) -> bool {
        // Allow comparing numeric types with each other
        if self.is_numeric(left) && self.is_numeric(right) {
            return true;
        }
        
        // Otherwise require exact type match
        left == right
    }

    fn type_name(&self, ty: &Type) -> String {
        match ty {
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Custom(name) => name.clone(),
            Type::Array(inner, _) => format!("[{}]", self.type_name(inner)),
            Type::Vector(inner) => format!("Vector<{}>", self.type_name(inner)),
            Type::Void => "void".to_string(),
        }
    }
}