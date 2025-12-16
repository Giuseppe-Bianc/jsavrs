//src/parser/ast.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::{HasSpan, SourceSpan};
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Expr {
    Binary { left: Box<Self>, op: BinaryOp, right: Box<Self>, span: SourceSpan },
    Unary { op: UnaryOp, expr: Box<Self>, span: SourceSpan },
    Grouping { expr: Box<Self>, span: SourceSpan },
    Literal { value: LiteralValue, span: SourceSpan },

    ArrayLiteral { elements: Vec<Self>, span: SourceSpan },

    Variable { name: Arc<str>, span: SourceSpan },
    Assign { target: Box<Self>, value: Box<Self>, span: SourceSpan },
    Call { callee: Box<Self>, arguments: Vec<Self>, span: SourceSpan },
    ArrayAccess { array: Box<Self>, index: Box<Self>, span: SourceSpan },
    // Additional expressions as needed
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

#[repr(u8)]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum LiteralValue {
    Number(Number),
    StringLit(Arc<str>),
    CharLit(Arc<str>),
    Bool(bool),
    Nullptr,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::StringLit(s) => write!(f, "\"{s}\""),
            Self::CharLit(c) => write!(f, "'{c}'"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Nullptr => f.write_str("nullptr"),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Stmt {
    Expression {
        expr: Expr,
    },
    VarDeclaration {
        variables: Vec<Arc<str>>,
        type_annotation: Type,
        is_mutable: bool,
        initializers: Vec<Expr>,
        span: SourceSpan,
    },
    Function {
        name: Arc<str>,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Self>,
        span: SourceSpan,
    },
    If {
        condition: Expr,
        then_branch: Vec<Self>,
        else_branch: Option<Vec<Self>>,
        span: SourceSpan,
    },
    While {
        condition: Expr,
        body: Vec<Self>,
        span: SourceSpan,
    },
    For {
        initializer: Option<Box<Self>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Vec<Self>,
        span: SourceSpan,
    },
    Block {
        statements: Vec<Self>,
        span: SourceSpan,
    },
    Return {
        value: Option<Expr>,
        span: SourceSpan,
    },
    Break {
        span: SourceSpan,
    },
    Continue {
        span: SourceSpan,
    },
    MainFunction {
        body: Vec<Self>,
        span: SourceSpan,
    },
}

impl Expr {
    #[must_use]
    pub const fn null_expr(span: SourceSpan) -> Self {
        Self::Literal { value: LiteralValue::Nullptr, span }
    }

    // Helper methods for literals
    #[must_use]
    pub const fn new_number_literal(value: Number, span: SourceSpan) -> Option<Self> {
        Some(Self::Literal { value: LiteralValue::Number(value), span })
    }

    #[must_use]
    pub const fn new_bool_literal(value: bool, span: SourceSpan) -> Option<Self> {
        Some(Self::Literal { value: LiteralValue::Bool(value), span })
    }

    #[must_use]
    pub const fn new_nullptr_literal(span: SourceSpan) -> Option<Self> {
        Some(Self::null_expr(span))
    }

    #[must_use]
    pub const fn new_string_literal(value: Arc<str>, span: SourceSpan) -> Option<Self> {
        Some(Self::Literal { value: LiteralValue::StringLit(value), span })
    }

    #[must_use]
    pub const fn new_char_literal(value: Arc<str>, span: SourceSpan) -> Option<Self> {
        Some(Self::Literal { value: LiteralValue::CharLit(value), span })
    }
}

impl HasSpan for Expr {
    fn span(&self) -> &SourceSpan {
        match self {
            Self::Binary { span, .. }
            | Self::Unary { span, .. }
            | Self::Grouping { span, .. }
            | Self::Literal { span, .. }
            | Self::ArrayLiteral { span, .. }
            | Self::Variable { span, .. }
            | Self::Assign { span, .. }
            | Self::Call { span, .. }
            | Self::ArrayAccess { span, .. } => span,
        }
    }
}

impl HasSpan for Stmt {
    fn span(&self) -> &SourceSpan {
        match self {
            Self::Expression { expr } => expr.span(),
            Self::VarDeclaration { span, .. }
            | Self::While { span, .. }
            | Self::For { span, .. }
            | Self::Function { span, .. }
            | Self::If { span, .. }
            | Self::Block { span, .. }
            | Self::Return { span, .. }
            | Self::Break { span, .. }
            | Self::Continue { span, .. }
            | Self::MainFunction { span, .. } => span,
        }
    }
}

impl BinaryOp {
    /// Converts a token into its corresponding binary operator.
    ///
    /// # Errors
    ///
    /// Returns a `CompileError::SyntaxError` if the token kind is not a valid binary operator.
    pub fn get_op(token: &Token) -> Result<Self, CompileError> {
        let op = match token.kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Subtract,
            TokenKind::Star => Self::Multiply,
            TokenKind::Slash => Self::Divide,
            TokenKind::Percent => Self::Modulo,
            TokenKind::EqualEqual => Self::Equal,
            TokenKind::NotEqual => Self::NotEqual,
            TokenKind::Less => Self::Less,
            TokenKind::LessEqual => Self::LessEqual,
            TokenKind::Greater => Self::Greater,
            TokenKind::GreaterEqual => Self::GreaterEqual,
            TokenKind::AndAnd => Self::And,
            TokenKind::OrOr => Self::Or,
            TokenKind::And => Self::BitwiseAnd,
            TokenKind::Or => Self::BitwiseOr,
            TokenKind::Xor => Self::BitwiseXor,
            TokenKind::ShiftLeft => Self::ShiftLeft,
            TokenKind::ShiftRight => Self::ShiftRight,
            _ => {
                return Err(CompileError::SyntaxError {
                    message: Arc::from(format!("Invalid binary operator: {:?}", token.kind)),
                    span: token.clone().span,
                    help: None,
                });
            }
        };
        Ok(op)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Parameter {
    pub name: Arc<str>,
    pub type_annotation: Type,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Type {
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
    Array(Box<Type>, Box<Expr>),
    Vector(Box<Type>),
    Void,
    NullPtr,
}

// Add this at the end of the file
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8 => f.write_str("i8"),
            Self::I16 => f.write_str("i16"),
            Self::I32 => f.write_str("i32"),
            Self::I64 => f.write_str("i64"),
            Self::U8 => f.write_str("u8"),
            Self::U16 => f.write_str("u16"),
            Self::U32 => f.write_str("u32"),
            Self::U64 => f.write_str("u64"),
            Self::F32 => f.write_str("f32"),
            Self::F64 => f.write_str("f64"),
            Self::Char => f.write_str("char"),
            Self::String => f.write_str("string"),
            Self::Bool => f.write_str("bool"),
            Self::Custom(name) => f.write_str(name),
            Self::Array(element_type, size_expr) => {
                // Simplified representation since we can't evaluate expressions here
                if let Expr::Literal { value: LiteralValue::Number(Number::Integer(size)), .. } = size_expr.as_ref() {
                    write!(f, "[{element_type}; {size}]")
                } else {
                    write!(f, "[{element_type}; <expr>]")
                }
            }
            Self::Vector(element_type) => write!(f, "Vector<{element_type}>"),
            Self::Void => f.write_str("void"),
            Self::NullPtr => f.write_str("nullptr"),
        }
    }
}
