//src/mlir/hir/hirimp.rs
use crate::location::source_span::SourceSpan;
use crate::mlir::hir::node_metadata::{NodeId, NodeMetadata};
use crate::parser::ast::{BinaryOp, LiteralValue, UnaryOp};
use crate::tokens::number::Number;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HIRExpr {
    Binary { left: Box<HIRExpr>, op: BinaryOp, right: Box<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },
    Unary { op: UnaryOp, expr: Box<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },
    Grouping { expr: Box<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },
    Literal { value: LiteralValue, span: SourceSpan, node_metadata: NodeMetadata },

    ArrayLiteral { elements: Vec<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },

    Variable { name: Arc<str>, span: SourceSpan, node_metadata: NodeMetadata },
    Assign { target: Box<HIRExpr>, value: Box<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },
    Call { callee: Box<HIRExpr>, arguments: Vec<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },
    ArrayAccess { array: Box<HIRExpr>, index: Box<HIRExpr>, span: SourceSpan, node_metadata: NodeMetadata },
    // Additional expressions as needed
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HIRStmt {
    Expression {
        expr: HIRExpr,
        node_metadata: NodeMetadata,
    },
    VarDeclaration {
        variables: Vec<Arc<str>>,
        type_annotation: HIRType,
        is_mutable: bool,
        initializers: Vec<HIRExpr>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    Function {
        name: Arc<str>,
        parameters: Vec<HIRParameter>,
        return_type: HIRType,
        body: Vec<HIRStmt>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    If {
        condition: HIRExpr,
        then_branch: Vec<HIRStmt>,
        else_branch: Option<Vec<HIRStmt>>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    While {
        condition: HIRExpr,
        body: Vec<HIRStmt>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    For {
        initializer: Option<Box<HIRStmt>>,
        condition: Option<HIRExpr>,
        increment: Option<HIRExpr>,
        body: Vec<HIRStmt>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    Block {
        statements: Vec<HIRStmt>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    Return {
        value: Option<HIRExpr>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    Break {
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    Continue {
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
    MainFunction {
        body: Vec<HIRStmt>,
        span: SourceSpan,
        node_metadata: NodeMetadata,
    },
}

impl HIRExpr {
    /// Returns the source span for this expression.
    pub fn span(&self) -> &SourceSpan {
        match self {
            HIRExpr::Binary { span, .. } |
            HIRExpr::Unary { span, .. } |
            HIRExpr::Grouping { span, .. } |
            HIRExpr::Literal { span, .. } |
            HIRExpr::Variable { span, .. } |
            HIRExpr::Assign { span, .. } |
            HIRExpr::Call { span, .. } |
            HIRExpr::ArrayAccess { span, .. } |
            HIRExpr::ArrayLiteral { span, .. } => span,
        }
    }

    /// Returns the node ID for this expression.
    pub fn node_id(&self) -> NodeId {
        match self {
            HIRExpr::Binary { node_metadata, .. } |
            HIRExpr::Unary { node_metadata, .. } |
            HIRExpr::Grouping { node_metadata, .. } |
            HIRExpr::Literal { node_metadata, .. } |
            HIRExpr::Variable { node_metadata, .. } |
            HIRExpr::Assign { node_metadata, .. } |
            HIRExpr::Call { node_metadata, .. } |
            HIRExpr::ArrayAccess { node_metadata, .. } |
            HIRExpr::ArrayLiteral { node_metadata, .. } => node_metadata.node_id(),
        }
    }

    pub fn null_expr(span: SourceSpan, node_metadata: NodeMetadata) -> HIRExpr {
        HIRExpr::Literal { value: LiteralValue::Nullptr, span, node_metadata }
    }

    /// Helper methods for creating literal expressions.
    
    /// Generic helper function to create literal expressions.
    fn new_literal<T>(value: T, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr>
    where
        T: Into<LiteralValue>,
    {
        Some(HIRExpr::Literal { value: value.into(), span, node_metadata })
    }
    
    /// Creates a new number literal expression.
    pub fn new_number_literal(value: Number, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Self::new_literal(LiteralValue::Number(value), span, node_metadata)
    }

    /// Creates a new boolean literal expression.
    pub fn new_bool_literal(value: bool, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Self::new_literal(LiteralValue::Bool(value), span, node_metadata)
    }

    /// Creates a new nullptr literal expression.
    pub fn new_nullptr_literal(span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Some(HIRExpr::null_expr(span, node_metadata))
    }

    /// Creates a new string literal expression.
    pub fn new_string_literal(value: Arc<str>, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Self::new_literal(LiteralValue::StringLit(value), span, node_metadata)
    }

    /// Creates a new character literal expression.
    pub fn new_char_literal(value: Arc<str>, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Self::new_literal(LiteralValue::CharLit(value), span, node_metadata)
    }
}

impl HIRStmt {
    /// Returns the source span for this statement.
    pub fn span(&self) -> &SourceSpan {
        match self {
            HIRStmt::Expression { expr, .. } => expr.span(),
            HIRStmt::VarDeclaration { span, .. } |
            HIRStmt::While { span, .. } |
            HIRStmt::For { span, .. } |
            HIRStmt::Function { span, .. } |
            HIRStmt::If { span, .. } |
            HIRStmt::Block { span, .. } |
            HIRStmt::Return { span, .. } |
            HIRStmt::Break { span, .. } |
            HIRStmt::Continue { span, .. } |
            HIRStmt::MainFunction { span, .. } => span,
        }
    }

    /// Returns the node ID for this statement.
    pub fn node_id(&self) -> NodeId {
        match self {
            HIRStmt::Expression { node_metadata, .. } |
            HIRStmt::VarDeclaration { node_metadata, .. } |
            HIRStmt::While { node_metadata, .. } |
            HIRStmt::For { node_metadata, .. } |
            HIRStmt::Function { node_metadata, .. } |
            HIRStmt::If { node_metadata, .. } |
            HIRStmt::Block { node_metadata, .. } |
            HIRStmt::Return { node_metadata, .. } |
            HIRStmt::Break { node_metadata, .. } |
            HIRStmt::Continue { node_metadata, .. } |
            HIRStmt::MainFunction { node_metadata, .. } => node_metadata.node_id(),
        }
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct HIRParameter {
    pub name: Arc<str>,
    pub type_annotation: HIRType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HIRType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char,
    String,
    Bool,
    Custom(Arc<str>),
    Array(Box<HIRType>, Box<HIRExpr>),
    Vector(Box<HIRType>),
    Void,
    NullPtr,
}

// Add this at the end of the file
impl std::fmt::Display for HIRType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HIRType::I8 => f.write_str("i8"),
            HIRType::I16 => f.write_str("i16"),
            HIRType::I32 => f.write_str("i32"),
            HIRType::I64 => f.write_str("i64"),
            HIRType::U8 => f.write_str("u8"),
            HIRType::U16 => f.write_str("u16"),
            HIRType::U32 => f.write_str("u32"),
            HIRType::U64 => f.write_str("u64"),
            HIRType::F32 => f.write_str("f32"),
            HIRType::F64 => f.write_str("f64"),
            HIRType::Char => f.write_str("char"),
            HIRType::String => f.write_str("string"),
            HIRType::Bool => f.write_str("bool"),
            HIRType::Custom(name) => f.write_str(name),
            HIRType::Array(element_type, size_expr) => {
                // Simplified representation since we can't evaluate expressions here
                if let HIRExpr::Literal { value: LiteralValue::Number(Number::Integer(size)), .. } = size_expr.as_ref() {
                    write!(f, "[{element_type}; {size}]")
                } else {
                    write!(f, "[{element_type}; <expr>]")
                }
            }
            HIRType::Vector(element_type) => write!(f, "Vector<{element_type}>"),
            HIRType::Void => f.write_str("void"),
            HIRType::NullPtr => f.write_str("nullptr"),
        }
    }
}
