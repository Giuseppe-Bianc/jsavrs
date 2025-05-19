use crate::{
    location::source_span::SourceSpan,
    tokens::{
        number::Number,
        token_kind::TokenKind,
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal, SourceSpan),
    Variable(String, SourceSpan),
    BinaryOp {
        lhs: Box<Expr>,
        op: TokenKind,
        rhs: Box<Expr>,
        span: SourceSpan,
    },
    UnaryOp {
        op: TokenKind,
        expr: Box<Expr>,
        span: SourceSpan,
    },
    Grouping(Box<Expr>, SourceSpan),
    Assignment {
        name: String,
        value: Box<Expr>,
        span: SourceSpan,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Numeric(Number),
    String(String),
    Bool(bool),
    Nullptr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    VarDecl {
        name: String,
        type_annotation: Option<TokenKind>,
        initializer: Expr,
        span: SourceSpan,
    },
    Block(Vec<Stmt>, SourceSpan),
    Function {
        name: String,
        params: Vec<(String, Option<TokenKind>)>,
        return_type: Option<TokenKind>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },
}

pub fn pratt_binding_power(op: &TokenKind) -> Option<(u8, u8)> {
    match op {
        // Exponentiation (right-associative)
        TokenKind::Xor => Some((30, 29)),
        // Multiplicative operators (left-associative)
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some((20, 21)),
        // Additive operators (left-associative)
        TokenKind::Plus | TokenKind::Minus => Some((10, 11)),
        // Bitwise shifts (left-associative)
        TokenKind::ShiftLeft | TokenKind::ShiftRight => Some((18, 19)),
        // Relational comparisons (left-associative)
        TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual => Some((12, 13)),
        // Equality comparisons (left-associative)
        TokenKind::EqualEqual | TokenKind::NotEqual => Some((11, 12)),
        // Bitwise AND (left-associative)
        TokenKind::And => Some((9, 10)),
        // Bitwise OR (left-associative)
        TokenKind::Or => Some((8, 9)),
        // Logical AND (left-associative)
        TokenKind::AndAnd => Some((7, 8)),
        // Logical OR (left-associative)
        TokenKind::OrOr => Some((6, 7)),
        // Assignment operators (right-associative)
        TokenKind::Equal | TokenKind::PlusEqual | TokenKind::MinusEqual | TokenKind::PercentEqual | TokenKind::XorEqual => Some((2, 1)),
        _ => None,
    }
}