//src/parser/ast.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: SourceSpan,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: SourceSpan,
    },
    Grouping {
        expr: Box<Expr>,
        span: SourceSpan,
    },
    Literal {
        value: LiteralValue,
        span: SourceSpan,
    },

    ArrayLiteral {
        elements: Vec<Expr>,
        span: SourceSpan,
    },

    Variable {
        name: Arc<str>,
        span: SourceSpan,
    },
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: SourceSpan,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        span: SourceSpan,
    },
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>,
        span: SourceSpan,
    },
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
            LiteralValue::Number(num) => write!(f, "{num}"),
            LiteralValue::StringLit(s) => write!(f, "\"{s}\""),
            LiteralValue::CharLit(c) => write!(f, "'{c}'"),
            LiteralValue::Bool(b) => write!(f, "{b}"),
            LiteralValue::Nullptr => f.write_str("nullptr"),
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
        body: Vec<Stmt>,
        span: SourceSpan,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        span: SourceSpan,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        span: SourceSpan,
    },
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },
    Block {
        statements: Vec<Stmt>,
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
        body: Vec<Stmt>,
        span: SourceSpan,
    },
}

impl Expr {
    pub fn span(&self) -> &SourceSpan {
        match self {
            Expr::Binary { span, .. } => span,
            Expr::Unary { span, .. } => span,
            Expr::Grouping { span, .. } => span,
            Expr::Literal { span, .. } => span,
            Expr::Variable { span, .. } => span,
            Expr::Assign { span, .. } => span,
            Expr::Call { span, .. } => span,
            Expr::ArrayAccess { span, .. } => span,
            Expr::ArrayLiteral { span, .. } => span,
        }
    }

    pub fn null_expr(span: SourceSpan) -> Expr {
        Expr::Literal {
            value: LiteralValue::Nullptr,
            span,
        }
    }

    // Helper methods for literals
    pub fn new_number_literal(value: Number, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::Number(value),
            span,
        })
    }

    pub fn new_bool_literal(value: bool, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::Bool(value),
            span,
        })
    }

    pub fn new_nullptr_literal(span: SourceSpan) -> Option<Expr> {
        Some(Expr::null_expr(span))
    }

    pub fn new_string_literal(value: Arc<str>, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::StringLit(value),
            span,
        })
    }

    pub fn new_char_literal(value: Arc<str>, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::CharLit(value),
            span,
        })
    }
}

impl Stmt {
    pub fn span(&self) -> &SourceSpan {
        match self {
            Stmt::Expression { expr } => expr.span(),
            Stmt::VarDeclaration { span, .. } => span,
            Stmt::While { span, .. } => span,
            Stmt::For { span, .. } => span,
            Stmt::Function { span, .. } => span,
            Stmt::If { span, .. } => span,
            Stmt::Block { span, .. } => span,
            Stmt::Return { span, .. } => span,
            Stmt::Break { span, .. } => span,
            Stmt::Continue { span, .. } => span,
            Stmt::MainFunction { span, .. } => span,
        }
    }
}

impl BinaryOp {
    pub fn get_op(token: &Token) -> Result<BinaryOp, CompileError> {
        let op = match token.kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Subtract,
            TokenKind::Star => BinaryOp::Multiply,
            TokenKind::Slash => BinaryOp::Divide,
            TokenKind::Percent => BinaryOp::Modulo,
            TokenKind::EqualEqual => BinaryOp::Equal,
            TokenKind::NotEqual => BinaryOp::NotEqual,
            TokenKind::Less => BinaryOp::Less,
            TokenKind::LessEqual => BinaryOp::LessEqual,
            TokenKind::Greater => BinaryOp::Greater,
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
            TokenKind::AndAnd => BinaryOp::And,
            TokenKind::OrOr => BinaryOp::Or,
            TokenKind::And => BinaryOp::BitwiseAnd,
            TokenKind::Or => BinaryOp::BitwiseOr,
            TokenKind::Xor => BinaryOp::BitwiseXor,
            TokenKind::ShiftLeft => BinaryOp::ShiftLeft,
            TokenKind::ShiftRight => BinaryOp::ShiftRight,
            _ => {
                return Err(CompileError::SyntaxError {
                    message: format!("Invalid binary operator: {:?}", token.kind),
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
            Type::I8 => f.write_str( "i8"),
            Type::I16 => f.write_str( "i16"),
            Type::I32 => f.write_str( "i32"),
            Type::I64 => f.write_str( "i64"),
            Type::U8 => f.write_str( "u8"),
            Type::U16 => f.write_str( "u16"),
            Type::U32 => f.write_str( "u32"),
            Type::U64 => f.write_str( "u64"),
            Type::F32 => f.write_str( "f32"),
            Type::F64 => f.write_str( "f64"),
            Type::Char => f.write_str( "char"),
            Type::String => f.write_str( "string"),
            Type::Bool => f.write_str( "bool"),
            Type::Custom(name) =>  write!(f, "{name}"),
            Type::Array(element_type, size_expr) => {
                // Simplified representation since we can't evaluate expressions here
                if let Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(size)),
                    ..
                } = size_expr.as_ref()
                {
                    write!(f, "[{element_type}; {size}]")
                } else {
                    write!(f, "[{element_type}; <expr>]")
                }
            }
            Type::Vector(element_type) => write!(f, "Vector<{element_type}>"),
            Type::Void => f.write_str( "void"),
            Type::NullPtr => f.write_str( "nullptr"),
        }
    }
}
