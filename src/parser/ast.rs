//src/parser/ast.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;

#[derive(Debug, Clone, PartialEq)]
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
        name: String,
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(Number),
    StringLit(String),
    CharLit(String),
    Bool(bool),
    Nullptr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expr: Expr,
    },
    VarDeclaration {
        variables: Vec<String>,
        type_annotation: Type,
        is_mutable: bool,
        initializers: Vec<Expr>,
        span: SourceSpan,
    },
    Function {
        name: String,
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

    pub fn new_string_literal(value: String, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::StringLit(value),
            span,
        })
    }

    pub fn new_char_literal(value: String, span: SourceSpan) -> Option<Expr> {
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
                });
            }
        };
        Ok(op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
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
    Custom(String),
    Array(Box<Type>, Box<Expr>),
    Vector(Box<Type>),
    Void,
}
