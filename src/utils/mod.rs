//! # Utilities Module
//!
//! The utilities module provides helper functions, macros, and utilities used
//! throughout the compiler. It contains common functionality that doesn't belong
//! to a specific compilation phase.
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Provides helper functions for AST construction and testing
//! * Runtime: Offers utility functions for token processing, string manipulation, and more
//! * Termination: Provides formatting and sanitization utilities for output  
use crate::ir::Module;
use crate::location::source_location::SourceLocation;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type, UnaryOp};
use crate::semantic::symbol_table::{FunctionSymbol, Symbol, VariableSymbol};
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;
//use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;

use std::process::{Command, Stdio};

/// Funzione helper per recuperare l'hash SHA-1 del commit corrente.
/// Restituisce `Some(hash)` se riesce, altrimenti `None`.
pub fn get_git_commit_hash() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?; // ritorna None se il comando fallisce

    if !output.status.success() {
        return None;
    }

    let hash = String::from_utf8(output.stdout).ok()?;
    let hash = hash.trim();

    // Verifica formato SHA-1
    if hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(hash.to_string())
    } else {
        None
    }
}

static ANSI_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::expect_used)]
    Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").expect("ANSI regex pattern is valid")
});
static UUID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::expect_used)]
    Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
        .expect("UUID regex pattern is valid")
});

/// Creates a dummy source span for testing purposes.
///
/// Returns a default-initialized `SourceSpan` suitable for use in unit tests
/// where actual source location information is not needed.
///
/// # Returns
///
/// A default `SourceSpan` with no meaningful location data.
///
/// # Examples
///
/// ```ignore
/// let expr = Expr::Literal {
///     value: LiteralValue::Bool(true),
///     span: dummy_span(),
/// };
/// ```
#[must_use]
pub fn dummy_span() -> SourceSpan {
    SourceSpan::default()
}

/// Strips ANSI escape codes from a string for easier comparison in tests.
///
/// Removes all ANSI color and formatting codes, leaving only the plain text.
/// Useful for testing terminal output without dealing with formatting codes.
///
/// # Arguments
///
/// * `s` - String containing ANSI escape sequences
///
/// # Returns
///
/// A new string with all ANSI codes removed.
///
/// # Examples
///
/// ```ignore
/// let colored = "\x1B[31mError\x1B[0m";
/// let plain = strip_ansi_codes(colored);
/// assert_eq!(plain, "Error");
/// ```
#[must_use]
pub fn strip_ansi_codes(s: &str) -> String {
    ANSI_REGEX.replace_all(s, "").to_string()
}

#[macro_export]
// Helper functions per costruire AST
macro_rules! create_num_lit {
    ($variant:ident, $value:expr) => {
        Expr::Literal { value: LiteralValue::Number(Number::$variant($value)), span: dummy_span() }
    };
}

/// Creates an i8 numeric literal expression.
///
/// # Arguments
///
/// * `n` - The 8-bit signed integer value
///
/// # Returns
///
/// An `Expr::Literal` containing the i8 value with a dummy span.
#[must_use]
pub fn num_lit_i8(n: i8) -> Expr {
    create_num_lit!(I8, n)
}

/// Creates an i16 numeric literal expression.
///
/// # Arguments
///
/// * `n` - The 16-bit signed integer value
///
/// # Returns
///
/// An `Expr::Literal` containing the i16 value with a dummy span.
#[must_use]
pub fn num_lit_i16(n: i16) -> Expr {
    create_num_lit!(I16, n)
}

/// Creates an i32 numeric literal expression.
///
/// # Arguments
///
/// * `n` - The 32-bit signed integer value
///
/// # Returns
///
/// An `Expr::Literal` containing the i32 value with a dummy span.
#[must_use]
pub fn num_lit_i32(n: i32) -> Expr {
    create_num_lit!(I32, n)
}

/// Creates an i64 numeric literal expression.
///
/// # Arguments
///
/// * `n` - The 64-bit signed integer value
///
/// # Returns
///
/// An `Expr::Literal` containing the i64 value with a dummy span.
#[must_use]
pub fn num_lit_i64(n: i64) -> Expr {
    create_num_lit!(Integer, n)
}

#[must_use]
pub fn num_lit_u8(n: u8) -> Expr {
    create_num_lit!(U8, n)
}

#[must_use]
pub fn num_lit_u16(n: u16) -> Expr {
    create_num_lit!(U16, n)
}

#[must_use]
pub fn num_lit_u32(n: u32) -> Expr {
    create_num_lit!(U32, n)
}

#[must_use]
pub fn num_lit_unsigned(n: u64) -> Expr {
    create_num_lit!(UnsignedInteger, n)
}

#[must_use]
pub fn float_lit(n: f64) -> Expr {
    create_num_lit!(Float64, n)
}

fn create_literal_expr(value: LiteralValue) -> Expr {
    Expr::Literal { value, span: dummy_span() }
}
#[must_use]
pub fn bool_lit(b: bool) -> Expr {
    create_literal_expr(LiteralValue::Bool(b))
}

#[must_use]
pub fn nullptr_lit() -> Expr {
    create_literal_expr(LiteralValue::Nullptr)
}

#[must_use]
pub fn string_lit(s: &str) -> Expr {
    create_literal_expr(LiteralValue::StringLit(s.into()))
}

#[must_use]
pub fn char_lit(c: &str) -> Expr {
    create_literal_expr(LiteralValue::CharLit(c.into()))
}

/// Creates a binary operation expression.
///
/// # Arguments
///
/// * `left` - Left-hand side expression
/// * `op` - Binary operator (Add, Subtract, Multiply, etc.)
/// * `right` - Right-hand side expression
///
/// # Returns
///
/// An `Expr::Binary` with the given operands and operator.
///
/// # Examples
///
/// ```ignore
/// let expr = binary_expr(
///     num_lit_i32(2),
///     BinaryOp::Add,
///     num_lit_i32(3)
/// );
/// // Represents: 2 + 3
/// ```
#[must_use]
pub fn binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Binary { left: Box::new(left), op, right: Box::new(right), span: dummy_span() }
}

/// Creates a unary operation expression.
///
/// # Arguments
///
/// * `op` - Unary operator (Negate, Not)
/// * `expr` - The expression to apply the operator to
///
/// # Returns
///
/// An `Expr::Unary` with the given operator and operand.
///
/// # Examples
///
/// ```ignore
/// let expr = unary_expr(UnaryOp::Negate, num_lit_i32(5));
/// // Represents: -5
/// ```
#[must_use]
pub fn unary_expr(op: UnaryOp, expr: Expr) -> Expr {
    Expr::Unary { op, expr: Box::new(expr), span: dummy_span() }
}

/// Creates a grouping (parenthesized) expression.
///
/// # Arguments
///
/// * `expr` - The expression to group
///
/// # Returns
///
/// An `Expr::Grouping` containing the given expression.
///
/// # Examples
///
/// ```ignore
/// let expr = grouping_expr(binary_expr(
///     num_lit_i32(2),
///     BinaryOp::Add,
///     num_lit_i32(3)
/// ));
/// // Represents: (2 + 3)
/// ```
#[must_use]
pub fn grouping_expr(expr: Expr) -> Expr {
    Expr::Grouping { expr: Box::new(expr), span: dummy_span() }
}

// Generic helper function to create assignment expressions
#[must_use]
pub fn assign_expr(target: Expr, value: Expr) -> Expr {
    Expr::Assign { target: Box::new(target), value: Box::new(value), span: dummy_span() }
}

#[must_use]
pub fn variable_expr(name: &str) -> Expr {
    Expr::Variable { name: name.into(), span: dummy_span() }
}

// Generic helper function to create call expressions
#[must_use]
pub fn call_expr(callee: Expr, arguments: Vec<Expr>) -> Expr {
    Expr::Call { callee: Box::new(callee), arguments, span: dummy_span() }
}

// Generic helper function to create variable declarations
#[must_use]
pub fn var_declaration(
    variables: Vec<Arc<str>>, type_annotation: Type, is_mutable: bool, initializers: Vec<Expr>,
) -> Stmt {
    Stmt::VarDeclaration { variables, type_annotation, initializers, span: dummy_span(), is_mutable }
}

// Generic helper function to create function declarations
#[must_use]
pub fn function_declaration(name: Arc<str>, parameters: Vec<Parameter>, return_type: Type, body: Vec<Stmt>) -> Stmt {
    Stmt::Function { name, parameters, return_type, body, span: dummy_span() }
}

// Generic helper function to create array access expressions
#[must_use]
pub fn array_access_expr(array: Expr, index: Expr) -> Expr {
    Expr::ArrayAccess { array: Box::new(array), index: Box::new(index), span: dummy_span() }
}

// Generic helper function to create tokens from token kinds
#[must_use]
pub fn create_tokens(kinds: Vec<TokenKind>) -> Vec<Token> {
    kinds.into_iter().map(|k| Token { kind: k, span: dummy_span() }).collect()
}

// Generic helper function to create numeric tokens
#[must_use]
pub fn num_token(n: f64) -> Token {
    Token { kind: TokenKind::Numeric(Number::Float64(n)), span: dummy_span() }
}

// Test di merging
#[must_use]
pub fn create_span(
    file_path: &str, start_line: usize, start_col: usize, end_line: usize, end_col: usize,
) -> SourceSpan {
    SourceSpan::new(
        Arc::from(file_path),
        SourceLocation::new(start_line, start_col, 0),
        SourceLocation::new(end_line, end_col, 1),
    )
}

/// Helper function to create a `SourceSpan` for a given line.
#[must_use]
pub fn t_span(line: usize) -> SourceSpan {
    create_span("test_file", line, 1, line, 2)
}

/// Helper macro to construct a `CompileError::<Variant>` instance, optionally mutable, with a default message but no span.
#[macro_export]
macro_rules! make_error {
    // Immutable binding
    ($var:ident, $error_type:ident, $line:expr) => {
        let $var = CompileError::$error_type {
            code: None,
            message: "Unexpected token \"@\"".into(),
            span: t_span($line),
            help: None,
        };
    };
    ($var:ident, $error_type:ident, $line:expr, $help:expr) => {
        let $var = CompileError::$error_type {
            code: None,
            message: "Unexpected token \"@\"".into(),
            span: t_span($line),
            help: $help,
        };
    };
    // Mutable binding
    (mut $var:ident, $error_type:ident, $line:expr) => {
        let mut $var = CompileError::$error_type {
            code: None,
            message: "Unexpected token \"@\"".into(),
            span: t_span($line),
            help: None,
        };
    };
    (mut $var:ident, $error_type:ident, $line:expr, $help:expr) => {
        let mut $var = CompileError::$error_type {
            code: None,
            message: "Unexpected token \"@\"".into(),
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
        let $var = CompileError::$error_type { code: None, message: "Unexpected token \"@\"".into() };
    };
    // Mutable binding
    (mut $var:ident, $error_type:ident) => {
        let mut $var = CompileError::$error_type { code: None, message: "Unexpected token \"@\"".into() };
    };
}

#[must_use]
pub const fn int_type() -> Type {
    Type::I32
}

// Generic helper function to create variable symbols
#[must_use]
pub fn create_var_symbol(name: &str, mutable: bool) -> Symbol {
    Symbol::Variable(VariableSymbol {
        name: name.into(),
        ty: int_type(),
        mutable,
        defined_at: dummy_span(),
        last_assignment: None,
    })
}

// Generic helper function to create function symbols
#[must_use]
pub fn create_function_symbol(name: &str) -> FunctionSymbol {
    FunctionSymbol { name: name.into(), parameters: Vec::new(), return_type: Type::Void, defined_at: dummy_span() }
}

// Generic helper function to create function symbols wrapped in Symbol enum
#[must_use]
pub fn create_func_symbol(name: &str) -> Symbol {
    Symbol::Function(create_function_symbol(name))
}

#[macro_export]
macro_rules! from_symbol {
    ($sym:expr, $variant:ident) => {
        match $sym {
            Symbol::$variant(v) => Some(v),
            _ => None,
        }
    };
}

// Helper to extract inner symbol values for comparison
#[must_use]
pub fn var_from_symbol(sym: Symbol) -> Option<VariableSymbol> {
    from_symbol!(sym, Variable)
}

#[must_use]
pub fn func_from_symbol(sym: Symbol) -> Option<FunctionSymbol> {
    from_symbol!(sym, Function)
}

#[must_use]
pub fn sanitize_uuids(input: &str) -> String {
    sanitize_uuids_with_prefix(input, "SCOPE_")
}

#[must_use]
pub fn sanitize_mdata_uuids(input: &str) -> String {
    sanitize_uuids_with_prefix(input, "UUID_")
}

#[must_use]
fn sanitize_uuids_with_prefix(input: &str, prefix: &str) -> String {
    let mut counter = 0;
    let mut mapping = HashMap::new();

    UUID_REGEX
        .replace_all(input, |captures: &regex::Captures| {
            let uuid = captures.get(0).unwrap().as_str();
            let id = *mapping.entry(uuid.to_string()).or_insert_with(|| {
                let id = counter;
                counter += 1;
                id
            });
            format!("{prefix}{id}")
        })
        .to_string()
}

#[must_use]
pub fn vec_to_string<T: Display>(vec: Vec<T>) -> String {
    sanitize_uuids(vec.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ").as_str())
}

#[must_use]
pub fn module_redacted(module: Module) -> String {
    let mut redacted: String = String::new();
    writeln!(redacted, "module {} {{", module.name).unwrap();
    writeln!(redacted, "  data_layout = \"{}\";", module.data_layout).unwrap();
    writeln!(redacted, "  target_triple = \"{}\";", module.target_triple).unwrap();

    if module.functions.is_empty() {
        writeln!(redacted, "  // No functions").unwrap();
    } else {
        writeln!(redacted, "{}", vec_to_string(module.functions)).unwrap();
    }

    write!(redacted, "}}").unwrap();
    redacted
}

/// Thread-safe object pool for reusing frequently allocated objects.
///
/// This pool reduces allocation overhead by storing and reusing objects of type `T`.
/// All operations are thread-safe but may contend on a single mutex.
pub struct ObjectPool<T> {
    pool: Mutex<Vec<T>>,
}

impl<T> ObjectPool<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { pool: Mutex::new(Vec::new()) }
    }

    pub fn acquire(&self) -> Option<T> {
        match self.pool.lock() {
            Ok(mut guard) => guard.pop(),
            Err(poisoned) => {
                // Clear the poisoned state and continue
                poisoned.into_inner().pop()
            }
        }
    }
    pub fn release(&self, obj: T) {
        match self.pool.lock() {
            Ok(mut guard) => guard.push(obj),
            Err(poisoned) => {
                // Clear poisoned state and continue
                poisoned.into_inner().push(obj);
            }
        }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { pool: Mutex::new(Vec::with_capacity(capacity)) }
    }
}

impl<T> Default for ObjectPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
