//src/mlir/hir/hir.rs
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
    pub fn span(&self) -> &SourceSpan {
        match self {
            HIRExpr::Binary { span, .. } => span,
            HIRExpr::Unary { span, .. } => span,
            HIRExpr::Grouping { span, .. } => span,
            HIRExpr::Literal { span, .. } => span,
            HIRExpr::Variable { span, .. } => span,
            HIRExpr::Assign { span, .. } => span,
            HIRExpr::Call { span, .. } => span,
            HIRExpr::ArrayAccess { span, .. } => span,
            HIRExpr::ArrayLiteral { span, .. } => span,
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            HIRExpr::Binary { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::Unary { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::Grouping { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::Literal { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::Variable { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::Assign { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::Call { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::ArrayAccess { node_metadata, .. } => node_metadata.node_id(),
            HIRExpr::ArrayLiteral { node_metadata, .. } => node_metadata.node_id(),
        }
    }

    pub fn null_expr(span: SourceSpan, node_metadata: NodeMetadata) -> HIRExpr {
        HIRExpr::Literal { value: LiteralValue::Nullptr, span, node_metadata }
    }

    // Helper methods for literals
    pub fn new_number_literal(value: Number, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Some(HIRExpr::Literal { value: LiteralValue::Number(value), span, node_metadata })
    }

    pub fn new_bool_literal(value: bool, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Some(HIRExpr::Literal { value: LiteralValue::Bool(value), span, node_metadata })
    }

    pub fn new_nullptr_literal(span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Some(HIRExpr::null_expr(span, node_metadata))
    }

    pub fn new_string_literal(value: Arc<str>, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Some(HIRExpr::Literal { value: LiteralValue::StringLit(value), span, node_metadata })
    }

    pub fn new_char_literal(value: Arc<str>, span: SourceSpan, node_metadata: NodeMetadata) -> Option<HIRExpr> {
        Some(HIRExpr::Literal { value: LiteralValue::CharLit(value), span, node_metadata })
    }
}

impl HIRStmt {
    pub fn span(&self) -> &SourceSpan {
        match self {
            HIRStmt::Expression { expr, .. } => expr.span(),
            HIRStmt::VarDeclaration { span, .. } => span,
            HIRStmt::While { span, .. } => span,
            HIRStmt::For { span, .. } => span,
            HIRStmt::Function { span, .. } => span,
            HIRStmt::If { span, .. } => span,
            HIRStmt::Block { span, .. } => span,
            HIRStmt::Return { span, .. } => span,
            HIRStmt::Break { span, .. } => span,
            HIRStmt::Continue { span, .. } => span,
            HIRStmt::MainFunction { span, .. } => span,
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            HIRStmt::Expression { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::VarDeclaration { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::While { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::For { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::Function { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::If { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::Block { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::Return { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::Break { node_metadata, .. } => node_metadata.node_id(),
            HIRStmt::Continue { node_metadata, .. } => node_metadata.node_id(),
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
