use std::sync::Arc;
use jsavrs::location::source_span::SourceSpan;
use regex::Regex;
use jsavrs::location::source_location::SourceLocation;
use jsavrs::parser::ast::*;
use jsavrs::parser::ast_printer::pretty_print;
use jsavrs::tokens::number::Number;

// src/parser/ast_test.rs
// Helper to create a dummy SourceSpan
fn dummy_span() -> SourceSpan {
    SourceSpan::default()
}

// Strips ANSI escape codes for easier comparison
fn strip_ansi_codes(s: &str) -> String {
    let re = Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").unwrap();
    re.replace_all(s, "").to_string()
}

#[test]
fn test_simple_binary_expr() {
    let expr = Expr::Binary {
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
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── BinaryOp Add
    ├── Left:
    │   └── Literal 1
    └── Right:
        └── Literal 2";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_nested_binary_expr() {
    let inner = Expr::Binary {
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
    };
    let expr = Expr::Binary {
        left: Box::new(inner),
        op: BinaryOp::Multiply,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(3)),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── BinaryOp Multiply
    ├── Left:
    │   └── BinaryOp Add
    │       ├── Left:
    │       │   └── Literal 1
    │       └── Right:
    │           └── Literal 2
    └── Right:
        └── Literal 3";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_unary_negate() {
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        expr: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(5)),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── UnaryOp Negate
    └── Expr:
        └── Literal 5";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_grouping_expr() {
    let inner = Expr::Binary {
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
    };
    let expr = Expr::Grouping {
        expr: Box::new(inner),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Grouping
    └── Expr:
        └── BinaryOp Add
            ├── Left:
            │   └── Literal 1
            └── Right:
                └── Literal 2";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_literal_values() {
    let test_cases = vec![
        (
            Expr::Literal {
                value: LiteralValue::StringLit("test".to_string()),
                span: dummy_span(),
            },
            "└── Literal \"test\"",
        ),
        (
            Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            },
            "└── Literal true",
        ),
        (
            Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            },
            "└── Literal nullptr",
        ),
    ];

    for (expr, expected) in test_cases {
        let output = pretty_print(&expr);
        let stripped = strip_ansi_codes(&output);
        assert_eq!(stripped.trim(), expected);
    }
}

#[test]
fn test_variable_assignment() {
    let expr = Expr::Assign {
        name: "x".to_string(),
        value: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(3)),
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Assign to 'x'
    └── Value:
        └── Literal 3";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_function_call() {
    let callee = Expr::Variable {
        name: "foo".to_string(),
        span: dummy_span(),
    };
    let args = vec![
        Expr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: dummy_span(),
        },
        Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(2)),
                span: dummy_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(3)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    ];
    let expr = Expr::Call {
        callee: Box::new(callee),
        arguments: args,
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Function Call
    ├── Callee:
    │   └── Variable 'foo'
    └── Arguments:
            ├── Arg:
            │   └── Literal 1
            └── Arg:
                └── BinaryOp Add
                    ├── Left:
                    │   └── Literal 2
                    └── Right:
                        └── Literal 3";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── Array Access
    ├── Array:
    │   └── Variable 'arr'
    └── Index:
        └── BinaryOp Add
            ├── Left:
            │   └── Variable 'i'
            └── Right:
                └── Literal 1";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── BinaryOp Add
    ├── Left:
    │   └── BinaryOp Add
    │       ├── Left:
    │       │   └── BinaryOp Add
    │       │       ├── Left:
    │       │       │   └── Literal 1
    │       │       └── Right:
    │       │           └── Literal 2
    │       └── Right:
    │           └── Literal 3
    └── Right:
        └── Literal 4";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── UnaryOp Not
    └── Expr:
        └── UnaryOp Not
            └── Expr:
                └── Literal true";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_for_char_literal() {
    let expr = Expr::Literal {
        value: LiteralValue::CharLit("\'".to_string()),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Literal '''");
}

#[test]
fn test_edge_case_special_chars() {
    let expr = Expr::Literal {
        value: LiteralValue::StringLit("hello\nworld".to_string()),
        span: dummy_span(),
    };

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Literal \"hello\nworld\"");
}

#[test]
fn test_expr_binary_span() {
    let span = dummy_span();
    let expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: dummy_span(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(2)),
            span: dummy_span(),
        }),
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_unary_span() {
    let span = dummy_span();
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        expr: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(5)),
            span: dummy_span(),
        }),
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_grouping_span() {
    let span = dummy_span();
    let expr = Expr::Grouping {
        expr: Box::new(Expr::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span(),
        }),
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_literal_span() {
    let span = dummy_span();
    let expr = Expr::Literal {
        value: LiteralValue::Nullptr,
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_variable_span() {
    let span = dummy_span();
    let expr = Expr::Variable {
        name: "x".to_string(),
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_assign_span() {
    let span = dummy_span();
    let expr = Expr::Assign {
        name: "x".to_string(),
        value: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(3)),
            span: dummy_span(),
        }),
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_call_span() {
    let span = dummy_span();
    let callee = Expr::Variable {
        name: "func".to_string(),
        span: dummy_span(),
    };
    let expr = Expr::Call {
        callee: Box::new(callee),
        arguments: vec![],
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_expr_array_access_span() {
    let span = dummy_span();
    let array = Expr::Variable {
        name: "arr".to_string(),
        span: dummy_span(),
    };
    let index = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(0)),
        span: dummy_span(),
    };
    let expr = Expr::ArrayAccess {
        array: Box::new(array),
        index: Box::new(index),
        span: span.clone(),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_stmt_expression_span() {
    let expr_span = dummy_span();
    let expr = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(42)),
        span: expr_span.clone(),
    };
    let stmt = Stmt::Expression { expr };
    assert_eq!(stmt.span(), &expr_span);
}

#[test]
fn test_stmt_var_declaration_span() {
    let span = dummy_span();
    let stmt = Stmt::VarDeclaration {
        variables: vec!["x".to_string()],
        type_annotation: Type::I32,
        initializers: vec![],
        span: span.clone(),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_stmt_function_span() {
    let span = dummy_span();
    let stmt = Stmt::Function {
        name: "foo".to_string(),
        parameters: vec![],
        return_type: Type::Void,
        body: vec![],
        span: span.clone(),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_stmt_if_span() {
    let span = dummy_span();
    let condition = Expr::Literal {
        value: LiteralValue::Bool(true),
        span: dummy_span(),
    };
    let stmt = Stmt::If {
        condition,
        then_branch: vec![],
        else_branch: None,
        span: span.clone(),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_stmt_block_span() {
    let span = dummy_span();
    let stmt = Stmt::Block {
        statements: vec![],
        span: span.clone(),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_stmt_return_span() {
    let span = dummy_span();
    let stmt = Stmt::Return {
        value: None,
        span: span.clone(),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_zero_length_span() {
    // Assuming SourceSpan can be constructed with specific positions
    let zero_span = SourceSpan::new(
        Arc::from("test_file"),
        SourceLocation::new(1, 1, 0),
        SourceLocation::new(1, 1, 0),
    );
    let expr = Expr::Literal {
        value: LiteralValue::Nullptr,
        span: zero_span.clone(),
    };
    assert_eq!(expr.span(), &zero_span);
}

#[test]
fn test_nested_expr_spans() {
    let outer_span = dummy_span();
    let inner_span = dummy_span();

    let inner_expr = Expr::Binary {
        left: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: inner_span.clone(),
        }),
        op: BinaryOp::Add,
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(2)),
            span: inner_span.clone(),
        }),
        span: inner_span.clone(),
    };

    let outer_expr = Expr::Grouping {
        expr: Box::new(inner_expr),
        span: outer_span.clone(),
    };

    assert_eq!(outer_expr.span(), &outer_span);
}