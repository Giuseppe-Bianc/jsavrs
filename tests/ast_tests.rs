// tets/ast_test.rs
use jsavrs::lexer::lexer_tokenize_with_errors;
use jsavrs::parser::ast::*;
use jsavrs::parser::ast_printer::{pretty_print, pretty_print_stmt};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::tokens::number::Number;
use jsavrs::utils::*;

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
    let expr = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));

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
    let inner = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let expr = binary_expr(inner, BinaryOp::Multiply, num_lit(3));

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
    let expr = unary_expr(UnaryOp::Negate, num_lit(5));
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
    let inner = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let expr = grouping_expr(inner);

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
        (string_lit("test"), "└── Literal \"test\""),
        (bool_lit(true), "└── Literal true"),
        (nullptr_lit(), "└── Literal nullptr"),
    ];

    for (expr, expected) in test_cases {
        let output = pretty_print(&expr);
        let stripped = strip_ansi_codes(&output);
        assert_eq!(stripped.trim(), expected);
    }
}

#[test]
fn test_variable_assignment() {
    let expr = assign_expr("x", num_lit(3));

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
    let callee = variable_expr("foo");

    let args = vec![
        num_lit(1),
        binary_expr(num_lit(2), BinaryOp::Add, num_lit(3)),
    ];
    let expr = call_expr(callee, args);

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
    let array = variable_expr("arr");
    let index = binary_expr(variable_expr("i"), BinaryOp::Add, num_lit(1));
    let expr = array_access_expr(array, index);

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
    let expr = binary_expr(
        binary_expr(
            binary_expr(num_lit(1), BinaryOp::Add, num_lit(2)),
            BinaryOp::Add,
            num_lit(3),
        ),
        BinaryOp::Add,
        num_lit(4),
    );

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
    let expr = unary_expr(UnaryOp::Not, unary_expr(UnaryOp::Not, bool_lit(true)));

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
    let expr = char_lit("\'");

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Literal '''");
}

#[test]
fn test_edge_case_special_chars() {
    let expr = string_lit("hello\nworld");

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Literal \"hello\nworld\"");
}

expr_span_test!(test_expr_binary_span, |s| Expr::Binary {
    left: Box::new(num_lit(1)),
    op: BinaryOp::Add,
    right: Box::new(num_lit(2)),
    span: s,
});

expr_span_test!(test_expr_array_literal_span, |s| Expr::ArrayLiteral {
    elements: vec![num_lit(1), num_lit(2),],
    span: s,
});

expr_span_test!(test_expr_unary_span, |s| Expr::Unary {
    op: UnaryOp::Negate,
    expr: Box::new(num_lit(5)),
    span: s,
});

expr_span_test!(test_expr_grouping_span, |s| Expr::Grouping {
    expr: Box::new(bool_lit(true)),
    span: s,
});

expr_span_test!(test_expr_literal_span, |s| Expr::Literal {
    value: LiteralValue::Nullptr,
    span: s,
});

expr_span_test!(test_expr_variable_span, |s| Expr::Variable {
    name: "x".to_string(),
    span: s,
});
expr_span_test!(test_expr_assign_span, |s| Expr::Assign {
    name: "x".to_string(),
    value: Box::new(num_lit(3)),
    span: s,
});

expr_span_test!(test_expr_call_span, |s| Expr::Call {
    callee: Box::new(variable_expr("foo")),
    arguments: vec![],
    span: s,
});

expr_span_test!(test_expr_array_access_span, |s| Expr::ArrayAccess {
    array: Box::new(variable_expr("arr")),
    index: Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(0)),
        span: dummy_span(),
    }),
    span: s,
});

#[test]
fn test_stmt_expression_span() {
    let expr = num_lit(42);
    let stmt = Stmt::Expression { expr };
    assert_eq!(stmt.span(), &dummy_span());
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

stmt_span_test!(test_stmt_return_span, |s| Stmt::Return {
    value: None,
    span: s,
});

stmt_span_test!(test_stmt_break_span, |s| Stmt::Break { span: s });
stmt_span_test!(test_stmt_continue_span, |s| Stmt::Continue { span: s });
#[test]
fn test_zero_length_span() {
    let expr = nullptr_lit();
    assert_eq!(expr.span(), &dummy_span());
}

#[test]
fn test_nested_expr_spans() {
    let inner_expr = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));

    let outer_expr = grouping_expr(inner_expr);

    assert_eq!(outer_expr.span(), &dummy_span());
}

#[test]
fn test_stmt_expression() {
    let stmt = Stmt::Expression { expr: num_lit(42) };
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
    let stmt = var_declaration(
        vec!["x".to_string(), "y".to_string()],
        Type::I32,
        vec![num_lit(1), num_lit(2)],
    );

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
    let stmt = function_declaration(
        "sum".to_string(),
        vec![
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
        Type::I32,
        vec![Stmt::Return {
            value: Some(binary_expr(
                variable_expr("a"),
                BinaryOp::Add,
                variable_expr("b"),
            )),
            span: dummy_span(),
        }],
    );

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
    let condition = bool_lit(true);
    let then_branch = vec![Stmt::Expression { expr: num_lit(1) }];
    let else_branch = vec![Stmt::Expression { expr: num_lit(2) }];

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
        statements: vec![Stmt::Block {
            statements: vec![Stmt::Expression { expr: num_lit(42) }],
            span: dummy_span(),
        }],
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
        value: Some(num_lit(42)),
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
fn test_complex_type_declaration() {
    let stmt = var_declaration(
        vec!["matrix".to_string()],
        Type::Array(Box::new(Type::F64), Box::new(nullptr_lit())),
        vec![],
    );

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
        condition: bool_lit(true),
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
    let stmt = function_declaration(
        "func".to_string(),
        vec![
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
            Parameter {
                name: "c".to_string(),
                type_annotation: Type::I32,
                span: dummy_span(),
            },
        ],
        Type::Void,
        vec![],
    );

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
            let stmt = function_declaration("func".to_string(), vec![], $typ, vec![]);

            let output = pretty_print_stmt(&stmt);
            let stripped = strip_ansi_codes(&output);

            let expected = format!(
                "└── Function
    ├── Name:
    │   └── func
    ├── Parameters:
    ├── Return Type:
    │   └── {}
    └── Body:",
                $type_str
            );
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
test_type_output!(test_void_output, Type::Void, "void");
test_type_output!(test_custom_output, Type::Custom("inin".to_string()), "inin");

#[test]
fn test_corner_case_deeply_nested_if() {
    let inner_if = Stmt::If {
        condition: bool_lit(false),
        then_branch: vec![Stmt::Expression { expr: num_lit(3) }],
        else_branch: None,
        span: dummy_span(),
    };

    let stmt = Stmt::If {
        condition: bool_lit(true),
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
    let stmt = function_declaration(
        "getVector".to_string(),
        vec![],
        Type::Vector(Box::new(Type::Array(
            Box::new(Type::I32),
            Box::new(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            }),
        ))),
        vec![],
    );

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

#[test]
fn test_break_stmt() {
    let stmt = Stmt::Break { span: dummy_span() };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Break");
}
#[test]
fn test_continue_stmt() {
    let stmt = Stmt::Continue { span: dummy_span() };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_eq!(stripped.trim(), "└── Continue");
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
    let expected = "\
└── VarDeclaration
    ├── Variables:
    │   └── arr
    ├── Type:
    │   └── [i8; <expr>]
    └── Initializers:
        └── Array Literal
            └── Elements:
                ├── Literal 1
                ├── Literal 2
                ├── Literal 3
                ├── Literal 4
                └── Literal 5";
    assert_eq!(stripped.trim(), expected);
}
