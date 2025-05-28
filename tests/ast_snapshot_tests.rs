// tests/ast_snapshot_test.rs
use jsavrs::lexer::lexer_tokenize_with_errors;
use jsavrs::location::source_span::SourceSpan;
use jsavrs::parser::ast::*;
use jsavrs::parser::ast_printer::{pretty_print, pretty_print_stmt};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::tokens::number::Number;
use regex::Regex;
use insta::{assert_debug_snapshot, assert_snapshot};

// Helper to create a dummy SourceSpan
fn dummy_span() -> SourceSpan {
    SourceSpan::default()
}

// Strips ANSI escape codes for easier comparison
fn strip_ansi_codes(s: &str) -> String {
    let re = Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").unwrap();
    re.replace_all(s, "").to_string()
}


// Helper functions per costruire AST
fn num_lit(n: i64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Integer(n)),
        span: dummy_span(),
    }
}

fn float_lit(n: f64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Float64(n)),
        span: dummy_span(),
    }
}

fn bool_lit(b: bool) -> Expr {
    Expr::Literal {
        value: LiteralValue::Bool(b),
        span: dummy_span(),
    }
}

fn nullptr_lit() -> Expr {
    Expr::Literal {
        value: LiteralValue::Nullptr,
        span: dummy_span(),
    }
}

fn string_lit(s: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::StringLit(s.to_string()),
        span: dummy_span(),
    }
}

fn char_lit(c: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::CharLit(c.to_string()),
        span: dummy_span(),
    }
}

fn binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
        span: dummy_span(),
    }
}

fn unary_expr(op: UnaryOp, expr: Expr) -> Expr {
    Expr::Unary {
        op,
        expr: Box::new(expr),
        span: dummy_span(),
    }
}

fn grouping_expr(expr: Expr) -> Expr {
    Expr::Grouping {
        expr: Box::new(expr),
        span: dummy_span(),
    }
}

fn var_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.to_string(),
        span: dummy_span(),
    }
}

fn assign_expr(name: &str, value: Expr) -> Expr {
    Expr::Assign {
        name: name.to_string(),
        value: Box::new(value),
        span: dummy_span(),
    }
}

#[test]
fn test_simple_binary_expr() {
    let expr = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);
    assert_snapshot!(stripped.trim());
}

#[test]
fn test_nested_binary_expr() {
    let inner = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let expr = binary_expr(inner, BinaryOp::Multiply, num_lit(3));

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);
    assert_snapshot!(stripped.trim());
}

#[test]
fn test_unary_negate() {
    let expr = unary_expr(UnaryOp::Negate, num_lit(5));

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);
    assert_snapshot!(stripped.trim());
}

#[test]
fn test_grouping_expr() {
    let inner = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let expr = grouping_expr(inner);

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);
    assert_snapshot!(stripped.trim());
}

#[test]
fn test_literal_values() {
    let cases = vec![
        string_lit("test"),
        bool_lit(true),
        nullptr_lit()
    ];

    let mut snapshot_cases: Vec<(Expr, String)> = Vec::new();

    for expr in cases {
        let output = pretty_print(&expr);
        let stripped = strip_ansi_codes(&output);
        snapshot_cases.push((expr, stripped.trim().to_string()));
    }

    assert_debug_snapshot!(snapshot_cases);
}

#[test]
fn test_variable_assignment() {
    let expr = assign_expr("x", num_lit(3));


    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

fn variable_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.into(),
        span: dummy_span(),
    }
}

fn call_expr(callee: Expr, arguments: Vec<Expr>) -> Expr {
    Expr::Call {
        callee: Box::new(callee),
        arguments,
        span: dummy_span(),
    }
}

#[test]
fn test_function_call() {
    let callee = variable_expr("foo");

    let args = vec![
        num_lit(1),
        binary_expr(num_lit(2), BinaryOp::Add, num_lit(3)),
    ];
    let expr = call_expr(callee, args);

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_array_access() {
    let array = Expr::Variable {
        name: "arr".to_string(),
        span: dummy_span(),
    };
    let index = Expr::Binary {
        left: Box::new(Expr::Variable {
            name: "i".to_string(),
            span: dummy_span(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };
    let expr = Expr::ArrayAccess {
        array: Box::new(array),
        index: Box::new(index),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_deeply_nested_binary() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(1)),
                    span: dummy_span(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(2)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(3)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(4)),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_multiple_unary_ops() {
    let expr = Expr::Unary {
        op: UnaryOp::Not,
        expr: Box::new(Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_stmt_expression() {
    let expr = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(42)),
        span: dummy_span(),
    };
    let stmt = Stmt::Expression { expr };
    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_var_declaration_multiple_vars() {
    let stmt = Stmt::VarDeclaration {
        variables: vec!["x".to_string(), "y".to_string()],
        type_annotation: Type::I32,
        initializers: vec![
            Expr::Literal {
                value: LiteralValue::Number(Number::Integer(1)),
                span: dummy_span(),
            },
            Expr::Literal {
                value: LiteralValue::Number(Number::Integer(2)),
                span: dummy_span(),
            },
        ],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}
#[test]
fn test_function_with_parameters() {
    let stmt = Stmt::Function {
        name: "sum".to_string(),
        parameters: vec![
            Parameter {
                name: "a".to_string(),
                type_annotation: Type::I32,
                span: dummy_span(),
            },
            Parameter {
                name: "b".to_string(),
                type_annotation: Type::I32,
                span: dummy_span(),
            },
        ],
        return_type: Type::I32,
        body: vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "a".to_string(),
                    span: dummy_span(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Variable {
                    name: "b".to_string(),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_if_stmt_with_else() {
    let condition = Expr::Literal {
        value: LiteralValue::Bool(true),
        span: dummy_span(),
    };
    let then_branch = vec![Stmt::Expression {
        expr: Expr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: dummy_span(),
        },
    }];
    let else_branch = vec![Stmt::Expression {
        expr: Expr::Literal {
            value: LiteralValue::Number(Number::Integer(2)),
            span: dummy_span(),
        },
    }];

    let stmt = Stmt::If {
        condition,
        then_branch,
        else_branch: Some(else_branch),
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_empty_block_stmt() {
    let stmt = Stmt::Block {
        statements: vec![],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_nested_block_stmt() {
    let stmt = Stmt::Block {
        statements: vec![Stmt::Block {
            statements: vec![Stmt::Expression {
                expr: Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(42)),
                    span: dummy_span(),
                },
            }],
            span: dummy_span(),
        }],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_return_stmt_with_value() {
    let stmt = Stmt::Return {
        value: Some(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(42)),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_complex_type_declaration() {
    let stmt = Stmt::VarDeclaration {
        variables: vec!["matrix".to_string()],
        type_annotation: Type::Array(
            Box::new(Type::F64),
            Box::new(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            }),
        ),
        initializers: vec![],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_edge_case_empty_then_branch() {
    let stmt = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span(),
        },
        then_branch: vec![],
        else_branch: None,
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

macro_rules! test_type_output {
    ($name:ident, $typ:expr) => {
        #[test]
        fn $name() {
            let stmt = Stmt::Function {
                name: "func".to_string(),
                parameters: vec![],
                return_type: $typ,
                body: vec![],
                span: dummy_span(),
            };

            let output = pretty_print_stmt(&stmt);
            let stripped = strip_ansi_codes(&output);

            assert_snapshot!(stripped.trim());
        }
    };
}

test_type_output!(test_i8_output, Type::I8);
test_type_output!(test_i16_output, Type::I16);
test_type_output!(test_i32_output, Type::I32);
test_type_output!(test_i64_output, Type::I64);
test_type_output!(test_u8_output, Type::U8);
test_type_output!(test_u16_output, Type::U16);
test_type_output!(test_u32_output, Type::U32);
test_type_output!(test_u64_output, Type::U64);
test_type_output!(test_f32_output, Type::F32);
test_type_output!(test_char_output, Type::Char);
test_type_output!(test_string_output, Type::String);
test_type_output!(test_bool_output, Type::Bool);
test_type_output!(test_void_output, Type::Void);
test_type_output!(test_custom_output, Type::Custom("inin".to_string()));

#[test]
fn test_break_stmt() {
    let stmt = Stmt::Break { span: dummy_span() };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_continue_stmt() {
    let stmt = Stmt::Continue { span: dummy_span() };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_array_literal_output() {
    let input = "var arr: i8[5] = {1, 2, 3, 4, 5}";
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(input, "test.vn");
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);

    let output = pretty_print_stmt(&expr[0]);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}
