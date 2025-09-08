// src/mlir/hir/ast_to_hir.rs
use crate::mlir::hir::hirimp::{HIRExpr, HIRStmt, HIRType, HIRParameter};
use crate::mlir::hir::node_metadata::{NodeId, NodeMetadata};
use crate::parser::ast::{Expr, Stmt, Type, Parameter};
use crate::error::compile_error::CompileError;

/// Context for maintaining parent-child relationships during AST to HIR transformation
#[derive(Debug)]
struct TransformContext {
    /// Stack of parent node IDs for tracking hierarchical relationships
    parent_stack: Vec<NodeId>,
}

impl TransformContext {
    fn new() -> Self {
        Self {
            parent_stack: Vec::new(),
        }
    }

    /// Get the current parent node ID (top of stack)
    fn current_parent(&self) -> Option<NodeId> {
        self.parent_stack.last().copied()
    }

    /// Push a new parent onto the stack
    fn push_parent(&mut self, parent_id: NodeId) {
        self.parent_stack.push(parent_id);
    }

    /// Pop the current parent from the stack
    fn pop_parent(&mut self) -> Option<NodeId> {
        self.parent_stack.pop()
    }

    /// Create new node metadata with current parent context
    fn create_node_metadata(&self) -> NodeMetadata {
        NodeMetadata::new(self.current_parent())
    }
}

/// Main transformer struct for converting AST to HIR
pub struct AstToHirTransformer {
    context: TransformContext,
}

impl AstToHirTransformer {
    pub fn new() -> Self {
        Self {
            context: TransformContext::new(),
        }
    }

    /// Transform a complete AST program (vector of statements) to HIR
    pub fn transform_program(&mut self, ast: Vec<Stmt>) -> Result<Vec<HIRStmt>, CompileError> {
        let mut hir_stmts = Vec::new();
        
        for stmt in ast {
            let hir_stmt = self.transform_stmt(stmt)?;
            hir_stmts.push(hir_stmt);
        }
        
        Ok(hir_stmts)
    }

    /// Transform a single statement from AST to HIR
    pub fn transform_stmt(&mut self, stmt: Stmt) -> Result<HIRStmt, CompileError> {
        match stmt {
            Stmt::Expression { expr } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                // Push this statement as parent for the expression
                self.context.push_parent(node_id);
                let hir_expr = self.transform_expr(expr)?;
                self.context.pop_parent();
                
                Ok(HIRStmt::Expression {
                    expr: hir_expr,
                    node_metadata,
                })
            }

            Stmt::VarDeclaration { variables, type_annotation, is_mutable, initializers, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                // Push this statement as parent for initializer expressions
                self.context.push_parent(node_id);
                let mut hir_initializers = Vec::new();
                for init in initializers {
                    hir_initializers.push(self.transform_expr(init)?);
                }
                self.context.pop_parent();
                
                let hir_type = self.transform_type(type_annotation)?;
                
                Ok(HIRStmt::VarDeclaration {
                    variables,
                    type_annotation: hir_type,
                    is_mutable,
                    initializers: hir_initializers,
                    span,
                    node_metadata,
                })
            }

            Stmt::Function { name, parameters, return_type, body, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                // Transform parameters
                let mut hir_parameters = Vec::new();
                for param in parameters {
                    hir_parameters.push(self.transform_parameter(param)?);
                }
                
                let hir_return_type = self.transform_type(return_type)?;
                
                // Push this function as parent for body statements
                self.context.push_parent(node_id);
                let mut hir_body = Vec::new();
                for stmt in body {
                    hir_body.push(self.transform_stmt(stmt)?);
                }
                self.context.pop_parent();
                
                Ok(HIRStmt::Function {
                    name,
                    parameters: hir_parameters,
                    return_type: hir_return_type,
                    body: hir_body,
                    span,
                    node_metadata,
                })
            }

            Stmt::If { condition, then_branch, else_branch, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                // Push this statement as parent
                self.context.push_parent(node_id);
                
                let hir_condition = self.transform_expr(condition)?;
                
                let mut hir_then_branch = Vec::new();
                for stmt in then_branch {
                    hir_then_branch.push(self.transform_stmt(stmt)?);
                }
                
                let hir_else_branch = if let Some(else_stmts) = else_branch {
                    let mut hir_else_stmts = Vec::new();
                    for stmt in else_stmts {
                        hir_else_stmts.push(self.transform_stmt(stmt)?);
                    }
                    Some(hir_else_stmts)
                } else {
                    None
                };
                
                self.context.pop_parent();
                
                Ok(HIRStmt::If {
                    condition: hir_condition,
                    then_branch: hir_then_branch,
                    else_branch: hir_else_branch,
                    span,
                    node_metadata,
                })
            }

            Stmt::While { condition, body, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_condition = self.transform_expr(condition)?;
                
                let mut hir_body = Vec::new();
                for stmt in body {
                    hir_body.push(self.transform_stmt(stmt)?);
                }
                self.context.pop_parent();
                
                Ok(HIRStmt::While {
                    condition: hir_condition,
                    body: hir_body,
                    span,
                    node_metadata,
                })
            }

            Stmt::For { initializer, condition, increment, body, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                
                let hir_initializer = if let Some(init) = initializer {
                    Some(Box::new(self.transform_stmt(*init)?))
                } else {
                    None
                };
                
                let hir_condition = if let Some(cond) = condition {
                    Some(self.transform_expr(cond)?)
                } else {
                    None
                };
                
                let hir_increment = if let Some(inc) = increment {
                    Some(self.transform_expr(inc)?)
                } else {
                    None
                };
                
                let mut hir_body = Vec::new();
                for stmt in body {
                    hir_body.push(self.transform_stmt(stmt)?);
                }
                
                self.context.pop_parent();
                
                Ok(HIRStmt::For {
                    initializer: hir_initializer,
                    condition: hir_condition,
                    increment: hir_increment,
                    body: hir_body,
                    span,
                    node_metadata,
                })
            }

            Stmt::Block { statements, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let mut hir_statements = Vec::new();
                for stmt in statements {
                    hir_statements.push(self.transform_stmt(stmt)?);
                }
                self.context.pop_parent();
                
                Ok(HIRStmt::Block {
                    statements: hir_statements,
                    span,
                    node_metadata,
                })
            }

            Stmt::Return { value, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                let hir_value = if let Some(val) = value {
                    self.context.push_parent(node_id);
                    let result = Some(self.transform_expr(val)?);
                    self.context.pop_parent();
                    result
                } else {
                    None
                };
                
                Ok(HIRStmt::Return {
                    value: hir_value,
                    span,
                    node_metadata,
                })
            }

            Stmt::Break { span } => {
                Ok(HIRStmt::Break {
                    span,
                    node_metadata: self.context.create_node_metadata(),
                })
            }

            Stmt::Continue { span } => {
                Ok(HIRStmt::Continue {
                    span,
                    node_metadata: self.context.create_node_metadata(),
                })
            }

            Stmt::MainFunction { body, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let mut hir_body = Vec::new();
                for stmt in body {
                    hir_body.push(self.transform_stmt(stmt)?);
                }
                self.context.pop_parent();
                
                Ok(HIRStmt::MainFunction {
                    body: hir_body,
                    span,
                    node_metadata,
                })
            }
        }
    }

    /// Transform an expression from AST to HIR
    pub fn transform_expr(&mut self, expr: Expr) -> Result<HIRExpr, CompileError> {
        match expr {
            Expr::Binary { left, op, right, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_left = Box::new(self.transform_expr(*left)?);
                let hir_right = Box::new(self.transform_expr(*right)?);
                self.context.pop_parent();
                
                Ok(HIRExpr::Binary {
                    left: hir_left,
                    op,
                    right: hir_right,
                    span,
                    node_metadata,
                })
            }

            Expr::Unary { op, expr, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_expr = Box::new(self.transform_expr(*expr)?);
                self.context.pop_parent();
                
                Ok(HIRExpr::Unary {
                    op,
                    expr: hir_expr,
                    span,
                    node_metadata,
                })
            }

            Expr::Grouping { expr, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_expr = Box::new(self.transform_expr(*expr)?);
                self.context.pop_parent();
                
                Ok(HIRExpr::Grouping {
                    expr: hir_expr,
                    span,
                    node_metadata,
                })
            }

            Expr::Literal { value, span } => {
                Ok(HIRExpr::Literal {
                    value,
                    span,
                    node_metadata: self.context.create_node_metadata(),
                })
            }

            Expr::ArrayLiteral { elements, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let mut hir_elements = Vec::new();
                for element in elements {
                    hir_elements.push(self.transform_expr(element)?);
                }
                self.context.pop_parent();
                
                Ok(HIRExpr::ArrayLiteral {
                    elements: hir_elements,
                    span,
                    node_metadata,
                })
            }

            Expr::Variable { name, span } => {
                Ok(HIRExpr::Variable {
                    name,
                    span,
                    node_metadata: self.context.create_node_metadata(),
                })
            }

            Expr::Assign { target, value, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_target = Box::new(self.transform_expr(*target)?);
                let hir_value = Box::new(self.transform_expr(*value)?);
                self.context.pop_parent();
                
                Ok(HIRExpr::Assign {
                    target: hir_target,
                    value: hir_value,
                    span,
                    node_metadata,
                })
            }

            Expr::Call { callee, arguments, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_callee = Box::new(self.transform_expr(*callee)?);
                
                let mut hir_arguments = Vec::new();
                for arg in arguments {
                    hir_arguments.push(self.transform_expr(arg)?);
                }
                self.context.pop_parent();
                
                Ok(HIRExpr::Call {
                    callee: hir_callee,
                    arguments: hir_arguments,
                    span,
                    node_metadata,
                })
            }

            Expr::ArrayAccess { array, index, span } => {
                let node_metadata = self.context.create_node_metadata();
                let node_id = node_metadata.node_id();
                
                self.context.push_parent(node_id);
                let hir_array = Box::new(self.transform_expr(*array)?);
                let hir_index = Box::new(self.transform_expr(*index)?);
                self.context.pop_parent();
                
                Ok(HIRExpr::ArrayAccess {
                    array: hir_array,
                    index: hir_index,
                    span,
                    node_metadata,
                })
            }
        }
    }

    /// Transform a type from AST to HIR
    pub fn transform_type(&mut self, ast_type: Type) -> Result<HIRType, CompileError> {
        match ast_type {
            Type::I8 => Ok(HIRType::I8),
            Type::I16 => Ok(HIRType::I16),
            Type::I32 => Ok(HIRType::I32),
            Type::I64 => Ok(HIRType::I64),
            Type::U8 => Ok(HIRType::U8),
            Type::U16 => Ok(HIRType::U16),
            Type::U32 => Ok(HIRType::U32),
            Type::U64 => Ok(HIRType::U64),
            Type::F32 => Ok(HIRType::F32),
            Type::F64 => Ok(HIRType::F64),
            Type::Char => Ok(HIRType::Char),
            Type::String => Ok(HIRType::String),
            Type::Bool => Ok(HIRType::Bool),
            Type::Custom(name) => Ok(HIRType::Custom(name)),
            Type::Array(element_type, size_expr) => {
                let hir_element_type = Box::new(self.transform_type(*element_type)?);
                let hir_size_expr = Box::new(self.transform_expr(*size_expr)?);
                Ok(HIRType::Array(hir_element_type, hir_size_expr))
            }
            Type::Vector(element_type) => {
                let hir_element_type = Box::new(self.transform_type(*element_type)?);
                Ok(HIRType::Vector(hir_element_type))
            }
            Type::Void => Ok(HIRType::Void),
            Type::NullPtr => Ok(HIRType::NullPtr),
        }
    }

    /// Transform a parameter from AST to HIR
    pub fn transform_parameter(&mut self, param: Parameter) -> Result<HIRParameter, CompileError> {
        let hir_type = self.transform_type(param.type_annotation)?;
        Ok(HIRParameter {
            name: param.name,
            type_annotation: hir_type,
            span: param.span,
        })
    }
}

impl Default for AstToHirTransformer {
    fn default() -> Self {
        Self::new()
    }
}