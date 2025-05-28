use std::sync::Arc;
use regex::Regex;
use crate::location::source_location::SourceLocation;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type, UnaryOp};
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;

// Helper to create a dummy SourceSpan
pub fn dummy_span() -> SourceSpan {
    SourceSpan::default()
}

// Strips ANSI escape codes for easier comparison
pub fn strip_ansi_codes(s: &str) -> String {
    let re = Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").unwrap();
    re.replace_all(s, "").to_string()
}

// Helper functions per costruire AST
pub fn num_lit(n: i64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Integer(n)),
        span: dummy_span(),
    }
}

pub fn float_lit(n: f64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Float64(n)),
        span: dummy_span(),
    }
}

pub fn bool_lit(b: bool) -> Expr {
    Expr::Literal {
        value: LiteralValue::Bool(b),
        span: dummy_span(),
    }
}

pub fn nullptr_lit() -> Expr {
    Expr::Literal {
        value: LiteralValue::Nullptr,
        span: dummy_span(),
    }
}

pub fn string_lit(s: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::StringLit(s.to_string()),
        span: dummy_span(),
    }
}

pub fn char_lit(c: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::CharLit(c.to_string()),
        span: dummy_span(),
    }
}

pub fn binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
        span: dummy_span(),
    }
}

pub fn unary_expr(op: UnaryOp, expr: Expr) -> Expr {
    Expr::Unary {
        op,
        expr: Box::new(expr),
        span: dummy_span(),
    }
}

pub fn grouping_expr(expr: Expr) -> Expr {
    Expr::Grouping {
        expr: Box::new(expr),
        span: dummy_span(),
    }
}

pub fn var_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.to_string(),
        span: dummy_span(),
    }
}

pub fn assign_expr(name: &str, value: Expr) -> Expr {
    Expr::Assign {
        name: name.to_string(),
        value: Box::new(value),
        span: dummy_span(),
    }
}

pub fn variable_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.into(),
        span: dummy_span(),
    }
}

pub fn call_expr(callee: Expr, arguments: Vec<Expr>) -> Expr {
    Expr::Call {
        callee: Box::new(callee),
        arguments,
        span: dummy_span(),
    }
}
pub fn var_declaration(variables: Vec<String>, type_annotation: Type, initializers: Vec<Expr>) -> Stmt {
    Stmt::VarDeclaration {
        variables,
        type_annotation,
        initializers,
        span: dummy_span(),
    }
}
pub fn function_declaration(
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Vec<Stmt>,
) -> Stmt {
    Stmt::Function {
        name,
        parameters,
        return_type,
        body,
        span: dummy_span(),
    }
}

pub fn array_access_expr(array: Expr, index: Expr) -> Expr {
    Expr::ArrayAccess {
        array: Box::new(array),
        index: Box::new(index),
        span: dummy_span(),
    }
}

pub fn create_tokens(kinds: Vec<TokenKind>) -> Vec<Token> {
    kinds
        .into_iter()
        .map(|k| Token {
            kind: k,
            span: dummy_span(),
        })
        .collect()
}

pub fn num_token(n: f64) -> Token {
    Token {
        kind: TokenKind::Numeric(Number::Float64(n)),
        span: dummy_span(),
    }
}

/// Helper function to create a `SourceSpan` for a given line.
pub fn t_span(line: usize) -> SourceSpan {
    SourceSpan::new(
        Arc::from("test_file"),
        SourceLocation::new(line, 1, 0),
        SourceLocation::new(line, 2, 1),
    )
}

/// Helper macro to construct a `CompileError::<Variant>` instance, optionally mutable, with a default message and span.
#[macro_export]
macro_rules! make_error {
    // Immutable binding
    ($var:ident, $error_type:ident, $line:expr) => {
        let $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
            span: t_span($line),
        };
    };
    // Mutable binding
    (mut $var:ident, $error_type:ident, $line:expr) => {
        let mut $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
            span: t_span($line),
        };
    };
}
