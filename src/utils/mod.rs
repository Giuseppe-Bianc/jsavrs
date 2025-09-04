use crate::location::source_location::SourceLocation;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type, UnaryOp};
use crate::semantic::symbol_table::{FunctionSymbol, Symbol, VariableSymbol};
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;
use regex::Regex;
use std::sync::Arc;

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

/// Creates a numeric literal expression from an `i8` value.
pub fn num_lit_i8(n: i8) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::I8(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from an `i16` value.
pub fn num_lit_i16(n: i16) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::I16(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from an `i32` value.
pub fn num_lit_i32(n: i32) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::I32(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from an `i64` value (alias for `num_lit`).
pub fn num_lit_i64(n: i64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Integer(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from a `u8` value.
pub fn num_lit_u8(n: u8) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::U8(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from a `u16` value.
pub fn num_lit_u16(n: u16) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::U16(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from a `u32` value.
pub fn num_lit_u32(n: u32) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::U32(n)),
        span: dummy_span(),
    }
}

/// Creates a numeric literal expression from a `u64` value.
pub fn num_lit_unsigned(n: u64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::UnsignedInteger(n)),
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
        value: LiteralValue::StringLit(s.into()),
        span: dummy_span(),
    }
}

pub fn char_lit(c: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::CharLit(c.into()),
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

pub fn assign_expr(target: Expr, value: Expr) -> Expr {
    Expr::Assign {
        target: Box::new(target),
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
pub fn var_declaration(
    variables: Vec<Arc<str>>,
    type_annotation: Type,
    is_mutable: bool,
    initializers: Vec<Expr>,
) -> Stmt {
    Stmt::VarDeclaration {
        variables,
        type_annotation,
        initializers,
        span: dummy_span(),
        is_mutable,
    }
}
pub fn function_declaration(
    name: Arc<str>,
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

// Test di merging
pub fn create_span(
    file_path: &str,
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
) -> SourceSpan {
    SourceSpan::new(
        Arc::from(file_path),
        SourceLocation::new(start_line, start_col, 0),
        SourceLocation::new(end_line, end_col, 1),
    )
}

/// Helper function to create a `SourceSpan` for a given line.
pub fn t_span(line: usize) -> SourceSpan {
    create_span("test_file", line, 1, line, 2)
}

/// Helper macro to construct a `CompileError::<Variant>` instance, optionally mutable, with a default message but no span.
#[macro_export]
macro_rules! make_error {
    // Immutable binding
    ($var:ident, $error_type:ident, $line:expr) => {
        let $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
            span: t_span($line),
            help: None,
        };
    };
    ($var:ident, $error_type:ident, $line:expr, $help:expr) => {
        let $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
            span: t_span($line),
            help: $help,
        };
    };
    // Mutable binding
    (mut $var:ident, $error_type:ident, $line:expr) => {
        let mut $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
            span: t_span($line),
            help: None,
        };
    };
    (mut $var:ident, $error_type:ident, $line:expr, $help:expr) => {
        let mut $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
            span: t_span($line),
            help: $help,
        };
    };
}

/// Helper macro to construct a `CompileError::<Variant>` instance, optionally mutable, with a default message and span.
#[macro_export]
macro_rules! make_error_lineless {
    // Immutable binding
    ($var:ident, $error_type:ident) => {
        let $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
        };
    };
    // Mutable binding
    (mut $var:ident, $error_type:ident) => {
        let mut $var = CompileError::$error_type {
            message: "Unexpected token \"@\"".to_string(),
        };
    };
}

pub fn int_type() -> Type {
    Type::I32
}

pub fn create_var_symbol(name: &str, mutable: bool) -> Symbol {
    Symbol::Variable(VariableSymbol {
        name: name.into(),
        ty: int_type(),
        mutable,
        defined_at: dummy_span(),
        last_assignment: None,
    })
}

pub fn create_function_symbol(name: &str) -> FunctionSymbol {
    FunctionSymbol {
        name: name.to_string(),
        parameters: Vec::new(),
        return_type: Type::Void,
        defined_at: dummy_span(),
    }
}
pub fn create_func_symbol(name: &str) -> Symbol {
    Symbol::Function(create_function_symbol(name))
}

// Helper to extract inner symbol values for comparison
pub fn var_from_symbol(sym: Symbol) -> Option<VariableSymbol> {
    match sym {
        Symbol::Variable(v) => Some(v),
        _ => None,
    }
}

pub fn func_from_symbol(sym: Symbol) -> Option<FunctionSymbol> {
    match sym {
        Symbol::Function(f) => Some(f),
        _ => None,
    }
}
