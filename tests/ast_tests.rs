use std::sync::Arc;
use jsavrs::location::source_span::SourceSpan;
use regex::Regex;
use jsavrs::location::source_location::SourceLocation;
use jsavrs::parser::ast::*;
use jsavrs::parser::ast_printer::{pretty_print, pretty_print_stmt};
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

macro_rules! expr_span_test {
    ($test_name:ident, $expr_constructor:expr) => {
        #[test]
        fn $test_name() {
            let span = dummy_span();
            let expr = $expr_constructor(span.clone());
            assert_eq!(expr.span(), &span);
        }
    };
}

macro_rules! stmt_span_test {
    ($test_name:ident, $stmt_constructor:expr) => {
        #[test]
        fn $test_name() {
            let span = dummy_span();
            let stmt = $stmt_constructor(span.clone());
            assert_eq!(stmt.span(), &span);
        }
    };
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


////////////////
expr_span_test!(test_expr_binary_span, |s| Expr::Binary {
    left: Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(1)),
        span: dummy_span(),
    }),
    op: BinaryOp::Add,
    right: Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(2)),
        span: dummy_span(),
    }),
    span: s,
});

expr_span_test!(test_expr_unary_span, |s| Expr::Unary {
    op: UnaryOp::Negate,
    expr: Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: dummy_span(),
    }),
    span: s,
});

expr_span_test!(test_expr_grouping_span, |s| Expr::Grouping {
    expr: Box::new(Expr::Literal {
        value: LiteralValue::Bool(true),
        span: dummy_span(),
    }),
    span: s,
});

expr_span_test!(test_expr_literal_span, |s| Expr::Literal {
        value: LiteralValue::Nullptr,
        span: s,
    }
);


expr_span_test!(test_expr_variable_span, |s| Expr::Variable {
    name: "x".to_string(),
    span: s,
});
expr_span_test!(test_expr_assign_span, |s| Expr::Assign {
    name: "x".to_string(),
    value: Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(3)),
        span: dummy_span(),
    }),
    span: s,
});

expr_span_test!(test_expr_call_span, |s| Expr::Call {
    callee: Box::new(Expr::Variable {
        name: "foo".to_string(),
        span: dummy_span(),
    }),
    arguments: vec![],
    span: s,
});

expr_span_test!(test_expr_array_access_span, |s| Expr::ArrayAccess {
    array: Box::new(Expr::Variable {
        name: "arr".to_string(),
        span: dummy_span(),
    }),
    index: Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(0)),
        span: dummy_span(),
    }),
    span: s,
});

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

stmt_span_test!(test_stmt_var_declaration_span, |s| Stmt::VarDeclaration {
    variables: vec!["x".to_string()],
    type_annotation: Type::I32,
    initializers: vec![],
    span: s,
});

stmt_span_test!(test_stmt_function_span, |s| Stmt::Function {
    name: "foo".to_string(),
    parameters: vec![],
    return_type: Type::Void,
    body: vec![],
    span: s,
});


stmt_span_test!(test_stmt_if_span, |s| Stmt::If {
    condition: Expr::Literal {
        value: LiteralValue::Bool(true),
        span: dummy_span(),
    },
    then_branch: vec![],
    else_branch: None,
    span: s,
});

stmt_span_test!(test_stmt_block_span, |s| Stmt::Block {
    statements: vec![],
    span: s,
});

stmt_span_test!(test_stmt_while_span, |s| Stmt::While {
    condition: Expr::Literal {
        value: LiteralValue::Bool(true),
        span: dummy_span(),
    },
    body: vec![],
    span: s,
});

stmt_span_test!(test_stmt_return_span, |s| Stmt::Return {
    value: None,
    span: s,
});

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

// Add the following tests to your existing test module

#[test]
fn test_stmt_expression() {
    let expr = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(42)),
        span: dummy_span(),
    };
    let stmt = Stmt::Expression { expr };
    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Expression
    └── Expr:
        └── Literal 42";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── VarDeclaration
    ├── Variables:
    │   ├── x
    │   └── y
    ├── Type:
    │   └── i32
    └── Initializers:
        ├── Literal 1
        └── Literal 2";
    assert_eq!(stripped.trim(), expected);
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
        body: vec![
            Stmt::Return {
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
            },
        ],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Function
    ├── Name:
    │   └── sum
    ├── Parameters:
    │   ├── Parameter 'a'
    │   │   └── Type: i32
    │   └── Parameter 'b'
    │       └── Type: i32
    ├── Return Type:
    │   └── i32
    └── Body:
        └── Return
            └── Value:
                └── BinaryOp Add
                    ├── Left:
                    │   └── Variable 'a'
                    └── Right:
                        └── Variable 'b'";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── If
    ├── Condition:
    │   └── Literal true
    ├── Then:
    │   └── Expression
    │       └── Expr:
    │           └── Literal 1
    └── Else:
        └── Expression
            └── Expr:
                └── Literal 2";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_empty_block_stmt() {
    let stmt = Stmt::Block {
        statements: vec![],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Block");
}

#[test]
fn test_nested_block_stmt() {
    let stmt = Stmt::Block {
        statements: vec![
            Stmt::Block {
                statements: vec![
                    Stmt::Expression {
                        expr: Expr::Literal {
                            value: LiteralValue::Number(Number::Integer(42)),
                            span: dummy_span(),
                        },
                    },
                ],
                span: dummy_span(),
            },
        ],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Block
    └── Block
        └── Expression
            └── Expr:
                └── Literal 42";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── Return
    └── Value:
        └── Literal 42";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_while_stmt_with_body() {
    let stmt = Stmt::While {
        condition: Expr::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span(),
        },
        body: vec![
            Stmt::Expression {
                expr: Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(1)),
                    span: dummy_span(),
                },
            },
        ],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── While
    ├── Condition:
    │   └── Literal true
    └── Body:
        └── Expression
            └── Expr:
                └── Literal 1";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_complex_type_declaration() {
    let stmt = Stmt::VarDeclaration {
        variables: vec!["matrix".to_string()],
        type_annotation: Type::Array(Box::new(Type::F64), Box::new(
            Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            }
        )),
        initializers: vec![],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── VarDeclaration
    ├── Variables:
    │   └── matrix
    ├── Type:
    │   └── [f64; <expr>]
    └── Initializers:";
    assert_eq!(stripped.trim(), expected);
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

    let expected = "\
└── If
    ├── Condition:
    │   └── Literal true
    └── Then: (empty)";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_edge_case_multiple_parameters() {
    let stmt = Stmt::Function {
        name: "func".to_string(),
        parameters: vec![
            Parameter {
                name: "a".to_string(),
                type_annotation: Type::I32,
                span: dummy_span(),
            },
            Parameter  {
                name: "b".to_string(),
                type_annotation: Type::I32,
                span: dummy_span(),
            },
            Parameter {
                name: "c".to_string(),
                type_annotation: Type::I32,
                span: dummy_span(),
            },
        ],
        return_type: Type::Void,
        body: vec![],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Function
    ├── Name:
    │   └── func
    ├── Parameters:
    │   ├── Parameter 'a'
    │   │   └── Type: i32
    │   ├── Parameter 'b'
    │   │   └── Type: i32
    │   └── Parameter 'c'
    │       └── Type: i32
    ├── Return Type:
    │   └── void
    └── Body:";
    assert_eq!(stripped.trim(), expected);
}


macro_rules! test_type_output {
    ($name:ident, $typ:expr, $type_str:expr) => {
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

            let expected = format!(
"└── Function
    ├── Name:
    │   └── func
    ├── Parameters:
    ├── Return Type:
    │   └── {}
    └── Body:", $type_str);
    assert_eq!(stripped.trim(), expected);
}
    };
}

test_type_output!(test_i8_output, Type::I8, "i8");
test_type_output!(test_i16_output, Type::I16, "i16");
test_type_output!(test_i32_output, Type::I32, "i32");
test_type_output!(test_i64_output, Type::I64, "i64");
test_type_output!(test_u8_output, Type::U8, "u8");
test_type_output!(test_u16_output, Type::U16, "u16");
test_type_output!(test_u32_output, Type::U32, "u32");
test_type_output!(test_u64_output, Type::U64, "u64");
test_type_output!(test_f32_output, Type::F32, "f32");
test_type_output!(test_char_output, Type::Char, "char");
test_type_output!(test_string_output, Type::String, "string");
test_type_output!(test_bool_output, Type::Bool, "bool");

#[test]
fn test_corner_case_deeply_nested_if() {
    let inner_if = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Bool(false),
            span: dummy_span(),
        },
        then_branch: vec![Stmt::Expression {
            expr: Expr::Literal {
                value: LiteralValue::Number(Number::Integer(3)),
                span: dummy_span(),
            },
        }],
        else_branch: None,
        span: dummy_span(),
    };

    let stmt = Stmt::If {
        condition: Expr::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span(),
        },
        then_branch: vec![inner_if],
        else_branch: None,
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── If
    ├── Condition:
    │   └── Literal true
    └── Then:
        └── If
            ├── Condition:
            │   └── Literal false
            └── Then:
                └── Expression
                    └── Expr:
                        └── Literal 3";
    assert_eq!(stripped.trim(), expected);
}

#[test]
fn test_corner_case_complex_return_type() {
    let stmt = Stmt::Function {
        name: "getVector".to_string(),
        parameters: vec![],
        return_type: Type::Vector(Box::new(Type::Array(Box::new(Type::I32),
            Box::new(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            })
        ))),
        body: vec![],
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    let expected = "\
└── Function
    ├── Name:
    │   └── getVector
    ├── Parameters:
    ├── Return Type:
    │   └── Vector<[i32; <expr>]>
    └── Body:";
    assert_eq!(stripped.trim(), expected);
}