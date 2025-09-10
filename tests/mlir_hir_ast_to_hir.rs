/*// tets/ast_test.rs
use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
use jsavrs::mlir::hir::ast_to_hir::AstToHirTransformer;
use jsavrs::mlir::hir::hirimp::{HIRExpr, HIRStmt, HIRType};
use jsavrs::parser::ast::*;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::printers::hir_printer::{pretty_print_hir, pretty_print_stmt_hir};
use jsavrs::tokens::number::Number;
use jsavrs::utils::*;
use std::sync::Arc;
fn create_test_span() -> SourceSpan {
    let start = SourceLocation::new(1, 1, 0);
    let end = SourceLocation::new(1, 10, 9);
    SourceSpan::new(Arc::from("test.rs"), start, end)
}

#[test]
fn test_transform_simple_literal() {
    let mut transformer = AstToHirTransformer::new();
    let span = create_test_span();

    let ast_expr = Expr::Literal { value: LiteralValue::Number(Number::Integer(42)), span: span.clone() };

    let hir_expr = transformer.transform_expr(ast_expr).unwrap();

    match hir_expr {
        HIRExpr::Literal { value: LiteralValue::Number(Number::Integer(42)), .. } => {}
        _ => panic!("Expected HIR literal with number 42"),
    }
}

#[test]
fn test_transform_binary_expression() {
    let mut transformer = AstToHirTransformer::new();
    let span = create_test_span();

    let left = Box::new(Expr::Literal { value: LiteralValue::Number(Number::Integer(1)), span: span.clone() });
    let right = Box::new(Expr::Literal { value: LiteralValue::Number(Number::Integer(2)), span: span.clone() });

    let ast_expr = Expr::Binary { left, op: BinaryOp::Add, right, span: span.clone() };

    let hir_expr = transformer.transform_expr(ast_expr).unwrap();

    match hir_expr {
        HIRExpr::Binary { op: BinaryOp::Add, .. } => {}
        _ => panic!("Expected HIR binary expression with Add operator"),
    }
}

#[test]
fn test_transform_variable_declaration() {
    let mut transformer = AstToHirTransformer::new();
    let span = create_test_span();

    let ast_stmt = Stmt::VarDeclaration {
        variables: vec![Arc::from("x")],
        type_annotation: Type::I32,
        is_mutable: false,
        initializers: vec![Expr::Literal { value: LiteralValue::Number(Number::Integer(10)), span: span.clone() }],
        span: span.clone(),
    };

    let hir_stmt = transformer.transform_stmt(ast_stmt).unwrap();

    match hir_stmt {
        HIRStmt::VarDeclaration { variables, type_annotation: HIRType::I32, is_mutable: false, .. } => {
            assert_eq!(variables.len(), 1);
            assert_eq!(variables[0].as_ref(), "x");
        }
        _ => panic!("Expected HIR variable declaration"),
    }
}

#[test]
fn test_parent_child_relationships() {
    let mut transformer = AstToHirTransformer::new();
    let span = create_test_span();

    // Create a binary expression to test parent-child relationships
    let left = Box::new(Expr::Variable { name: Arc::from("a"), span: span.clone() });
    let right = Box::new(Expr::Variable { name: Arc::from("b"), span: span.clone() });

    let ast_expr = Expr::Binary { left, op: BinaryOp::Add, right, span: span.clone() };

    let hir_expr = transformer.transform_expr(ast_expr).unwrap();

    // Verify that the transformation succeeded and has proper structure
    match hir_expr {
        HIRExpr::Binary { left, right, .. } => {
            // Verify left and right operands are transformed correctly
            match (left.as_ref(), right.as_ref()) {
                (HIRExpr::Variable { name: left_name, .. }, HIRExpr::Variable { name: right_name, .. }) => {
                    assert_eq!(left_name.as_ref(), "a");
                    assert_eq!(right_name.as_ref(), "b");
                }
                _ => panic!("Expected left and right to be variables"),
            }
        }
        _ => panic!("Expected binary expression"),
    }
}

#[test]
fn test_transform_function_with_body() {
    let mut transformer = AstToHirTransformer::new();
    let span = create_test_span();

    let param = Parameter { name: Arc::from("x"), type_annotation: Type::I32, span: span.clone() };

    let return_stmt =
        Stmt::Return { value: Some(Expr::Variable { name: Arc::from("x"), span: span.clone() }), span: span.clone() };

    let ast_stmt = Stmt::Function {
        name: Arc::from("identity"),
        parameters: vec![param],
        return_type: Type::I32,
        body: vec![return_stmt],
        span: span.clone(),
    };

    let hir_stmt = transformer.transform_stmt(ast_stmt).unwrap();

    match hir_stmt {
        HIRStmt::Function { name, parameters, return_type, body, .. } => {
            assert_eq!(name.as_ref(), "identity");
            assert_eq!(parameters.len(), 1);
            assert_eq!(parameters[0].name.as_ref(), "x");
            assert_eq!(return_type, HIRType::I32);
            assert_eq!(body.len(), 1);

            // Verify the return statement was transformed correctly
            match &body[0] {
                HIRStmt::Return { value: Some(HIRExpr::Variable { name, .. }), .. } => {
                    assert_eq!(name.as_ref(), "x");
                }
                _ => panic!("Expected return statement with variable"),
            }
        }
        _ => panic!("Expected function declaration"),
    }
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
    let expr = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── BinaryOp Add
    ├── Left:
    │   └── Literal 1
    └── Right:
        └── Literal 2";
    assert_eq!(stripped_uuids.trim(), expected);
}
#[test]
fn test_nested_binary_expr() {
    let inner = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let expr = binary_expr(inner, BinaryOp::Multiply, num_lit(3));

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── BinaryOp Multiply [ID:UUID_0::-::P:-]
    ├── Left:
    │   └── BinaryOp Add [ID:UUID_1::-::P:UUID_0]
    │       ├── Left:
    │       │   └── Literal 1 [ID:UUID_2::-::P:UUID_1]
    │       └── Right:
    │           └── Literal 2 [ID:UUID_3::-::P:UUID_1]
    └── Right:
        └── Literal 3 [ID:UUID_4::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}
#[test]
fn test_unary_negate() {
    let expr = unary_expr(UnaryOp::Negate, num_lit(5));
    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── UnaryOp Negate
    └── Expr:
        └── Literal 5";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_grouping_expr() {
    let inner = binary_expr(num_lit(1), BinaryOp::Add, num_lit(2));
    let expr = grouping_expr(inner);

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Grouping
    └── Expr:
        └── BinaryOp Add
            ├── Left:
            │   └── Literal 1
            └── Right:
                └── Literal 2";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_literal_values() {
    let test_cases = vec![
        (string_lit("test"), "└── Literal \"test\""),
        (bool_lit(true), "└── Literal true"),
        (nullptr_lit(), "└── Literal nullptr"),
    ];

    for (expr, expected) in test_cases {
        let mut transformer = AstToHirTransformer::new();
        let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
        let output = pretty_print_hir(&hirexpr);
        let stripped = strip_ansi_codes(&output);
        let stripped_uuids = sanitize_mdata_uuids(&stripped);
        assert_eq!(stripped_uuids.trim(), expected);
    }
}

#[test]
fn test_variable_assignment() {
    let expr = assign_expr(variable_expr("x"), num_lit(3));

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Assignment
    ├── Target:
    │   └── Variable 'x'
    └── Value:
        └── Literal 3";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_function_call() {
    let callee = variable_expr("foo");

    let args = vec![num_lit(1), binary_expr(num_lit(2), BinaryOp::Add, num_lit(3))];
    let expr = call_expr(callee, args);

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

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
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_array_access() {
    let array = variable_expr("arr");
    let index = binary_expr(variable_expr("i"), BinaryOp::Add, num_lit(1));
    let expr = array_access_expr(array, index);

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Array Access [ID:UUID_0::-::P:-]
    ├── Array:
    │   └── Variable 'arr' [ID:UUID_1::-::P:UUID_0]
    └── Index:
        └── BinaryOp Add [ID:UUID_2::-::P:UUID_0]
            ├── Left:
            │   └── Variable 'i' [ID:UUID_3::-::P:UUID_2]
            └── Right:
                └── Literal 1 [ID:UUID_4::-::P:UUID_2]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_deeply_nested_binary() {
    let expr = binary_expr(
        binary_expr(binary_expr(num_lit(1), BinaryOp::Add, num_lit(2)), BinaryOp::Add, num_lit(3)),
        BinaryOp::Add,
        num_lit(4),
    );
    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── BinaryOp Add [ID:UUID_0::-::P:-]
    ├── Left:
    │   └── BinaryOp Add [ID:UUID_1::-::P:UUID_0]
    │       ├── Left:
    │       │   └── BinaryOp Add [ID:UUID_2::-::P:UUID_1]
    │       │       ├── Left:
    │       │       │   └── Literal 1 [ID:UUID_3::-::P:UUID_2]
    │       │       └── Right:
    │       │           └── Literal 2 [ID:UUID_4::-::P:UUID_2]
    │       └── Right:
    │           └── Literal 3 [ID:UUID_5::-::P:UUID_1]
    └── Right:
        └── Literal 4 [ID:UUID_6::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}
#[test]
fn test_multiple_unary_ops() {
    let expr = unary_expr(UnaryOp::Not, unary_expr(UnaryOp::Not, bool_lit(true)));

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── UnaryOp Not
    └── Expr:
        └── UnaryOp Not
            └── Expr:
                └── Literal true";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_for_char_literal() {
    let expr = char_lit("\'");

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    assert_eq!(stripped_uuids.trim(), "└── Literal ''' [ID:UUID_0::-::P:-]");
}

#[test]
fn test_edge_case_special_chars() {
    let expr = string_lit("hello\nworld");

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    assert_eq!(stripped_uuids.trim(), "└── Literal \"hello\nworld\" [ID:UUID_0::-::P:-]");
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

expr_span_test!(test_expr_unary_span, |s| Expr::Unary { op: UnaryOp::Negate, expr: Box::new(num_lit(5)), span: s });

expr_span_test!(test_expr_grouping_span, |s| Expr::Grouping { expr: Box::new(bool_lit(true)), span: s });

expr_span_test!(test_expr_literal_span, |s| Expr::Literal { value: LiteralValue::Nullptr, span: s });

expr_span_test!(test_expr_variable_span, |s| Expr::Variable { name: "x".into(), span: s });
expr_span_test!(test_expr_assign_span, |s| Expr::Assign {
    target: Box::new(variable_expr("x")),
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
    index: Box::new(Expr::Literal { value: LiteralValue::Number(Number::Integer(0)), span: dummy_span() }),
    span: s,
});

stmt_span_test!(test_stmt_main_function_span, |s| Stmt::MainFunction { body: vec![], span: s });

#[test]
fn test_stmt_expression_span() {
    let expr = num_lit(42);
    let stmt = Stmt::Expression { expr };
    assert_eq!(stmt.span(), &dummy_span());
}

stmt_span_test!(test_stmt_var_declaration_span, |s| Stmt::VarDeclaration {
    variables: vec!["x".into()],
    type_annotation: Type::I32,
    initializers: vec![],
    is_mutable: true,
    span: s,
});

stmt_span_test!(test_stmt_const_declaration_span, |s| Stmt::VarDeclaration {
    variables: vec!["x".into()],
    type_annotation: Type::I32,
    initializers: vec![],
    is_mutable: false,
    span: s,
});

stmt_span_test!(test_stmt_function_span, |s| Stmt::Function {
    name: "foo".into(),
    parameters: vec![],
    return_type: Type::Void,
    body: vec![],
    span: s,
});

stmt_span_test!(test_stmt_if_span, |s| Stmt::If {
    condition: Expr::Literal { value: LiteralValue::Bool(true), span: dummy_span() },
    then_branch: vec![],
    else_branch: None,
    span: s,
});

stmt_span_test!(test_stmt_while_span, |s| Stmt::While {
    condition: Expr::Literal { value: LiteralValue::Bool(true), span: dummy_span() },
    body: vec![],
    span: s,
});
stmt_span_test!(test_stmt_for_span, |s| Stmt::For {
    initializer: None,
    condition: None,
    increment: None,
    body: vec![],
    span: s,
});

stmt_span_test!(test_stmt_block_span, |s| Stmt::Block { statements: vec![], span: s });

stmt_span_test!(test_stmt_return_span, |s| Stmt::Return { value: None, span: s });

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
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Expression
    └── Expr:
        └── Literal 42";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_var_declaration_multiple_vars() {
    let stmt = var_declaration(vec!["x".into(), "y".into()], Type::I32, true, vec![num_lit(1), num_lit(2)]);

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

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
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_function_with_parameters() {
    let stmt = function_declaration(
        "sum".into(),
        vec![
            Parameter { name: "a".into(), type_annotation: Type::I32, span: dummy_span() },
            Parameter { name: "b".into(), type_annotation: Type::I32, span: dummy_span() },
        ],
        Type::I32,
        vec![Stmt::Return {
            value: Some(binary_expr(variable_expr("a"), BinaryOp::Add, variable_expr("b"))),
            span: dummy_span(),
        }],
    );

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

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
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_if_stmt_with_else() {
    let condition = bool_lit(true);
    let then_branch = vec![Stmt::Expression { expr: num_lit(1) }];
    let else_branch = vec![Stmt::Expression { expr: num_lit(2) }];

    let stmt = Stmt::If { condition, then_branch, else_branch: Some(else_branch), span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

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
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_empty_block_stmt() {
    let stmt = Stmt::Block { statements: vec![], span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    assert_eq!(stripped_uuids.trim(), "└── Block: (empty) [ID:UUID_0::-::P:-]");
}

#[test]
fn test_nested_block_stmt() {
    let stmt = Stmt::Block {
        statements: vec![Stmt::Block { statements: vec![Stmt::Expression { expr: num_lit(42) }], span: dummy_span() }],
        span: dummy_span(),
    };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Block
    └── Block
        └── Expression
            └── Expr:
                └── Literal 42";
    assert_eq!(stripped_uuids.trim(), expected);
}
#[test]
fn test_return_stmt_with_value() {
    let stmt = Stmt::Return { value: Some(num_lit(42)), span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Return
    └── Value:
        └── Literal 42";
    assert_eq!(stripped_uuids.trim(), expected);
}
#[test]
fn test_complex_type_declaration() {
    let stmt =
        var_declaration(vec!["matrix".into()], Type::Array(Box::new(Type::F64), Box::new(nullptr_lit())), true, vec![]);

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── VarDeclaration [ID:UUID_0::-::P:-]
    ├── Variables:
    │   └── matrix
    ├── Type:
    │   └── [f64; <expr>]
    └── Initializers:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_complex_type_const_declaration() {
    let stmt = var_declaration(
        vec!["matrix".into()],
        Type::Array(Box::new(Type::F64), Box::new(nullptr_lit())),
        false,
        vec![],
    );

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── VarDeclaration [ID:UUID_0::-::P:-]
    ├── Constants:
    │   └── matrix
    ├── Type:
    │   └── [f64; <expr>]
    └── Initializers:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_edge_case_empty_then_branch() {
    let stmt = Stmt::If { condition: bool_lit(true), then_branch: vec![], else_branch: None, span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── If [ID:UUID_0::-::P:-]
    ├── Condition:
    │   └── Literal true [ID:UUID_1::-::P:UUID_0]
    └── Then: (empty)";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_while() {
    let stmt = Stmt::While { condition: bool_lit(true), body: vec![], span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── While
    ├── Condition:
    │   └── Literal true
    └── Body:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_while_not_empty_body() {
    let stmt = Stmt::While {
        condition: bool_lit(true),
        body: vec![Stmt::Expression { expr: num_lit(42) }],
        span: dummy_span(),
    };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── While
    ├── Condition:
    │   └── Literal true
    └── Body:
        └── Expression
            └── Expr:
                └── Literal 42";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_for() {
    let stmt = Stmt::For {
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit(1)]))),
        condition: None,
        increment: None,
        body: vec![],
        span: dummy_span(),
    };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── For [ID:UUID_0::-::P:-]
    ├── Initializer:
    │   └── VarDeclaration [ID:UUID_1::-::P:UUID_0]
    │       ├── Variables:
    │       │   └── x
    │       ├── Type:
    │       │   └── i32
    │       └── Initializers:
    │           └── Literal 1 [ID:UUID_2::-::P:UUID_1]
    └── Body:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_for_complete() {
    let stmt = Stmt::For {
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit(1)]))),
        condition: Some(binary_expr(variable_expr("x"), BinaryOp::Less, num_lit(2))),
        increment: Some(assign_expr(variable_expr("x"), binary_expr(variable_expr("x"), BinaryOp::Add, num_lit(1)))),
        body: vec![],
        span: dummy_span(),
    };
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── For [ID:UUID_0::-::P:-]
    ├── Initializer:
    │   └── VarDeclaration [ID:UUID_1::-::P:UUID_0]
    │       ├── Variables:
    │       │   └── x
    │       ├── Type:
    │       │   └── i32
    │       └── Initializers:
    │           └── Literal 1 [ID:UUID_2::-::P:UUID_1]
    ├── Condition:
    │   └── BinaryOp Less [ID:UUID_3::-::P:UUID_0]
    │       ├── Left:
    │       │   └── Variable 'x' [ID:UUID_4::-::P:UUID_3]
    │       └── Right:
    │           └── Literal 2 [ID:UUID_5::-::P:UUID_3]
    ├── Increment:
    │   └── Assignment [ID:UUID_6::-::P:UUID_0]
    │       ├── Target:
    │       │   └── Variable 'x' [ID:UUID_7::-::P:UUID_6]
    │       └── Value:
    │           └── BinaryOp Add [ID:UUID_8::-::P:UUID_6]
    │               ├── Left:
    │               │   └── Variable 'x' [ID:UUID_9::-::P:UUID_8]
    │               └── Right:
    │                   └── Literal 1 [ID:UUID_10::-::P:UUID_8]
    └── Body:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_for_not_empty_body() {
    let stmt = Stmt::For {
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit(1)]))),
        condition: None,
        increment: None,
        body: vec![Stmt::Expression { expr: num_lit(42) }],
        span: dummy_span(),
    };
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── For [ID:UUID_0::-::P:-]
    ├── Initializer:
    │   └── VarDeclaration [ID:UUID_1::-::P:UUID_0]
    │       ├── Variables:
    │       │   └── x
    │       ├── Type:
    │       │   └── i32
    │       └── Initializers:
    │           └── Literal 1 [ID:UUID_2::-::P:UUID_1]
    └── Body:
        └── Expression [ID:UUID_3::-::P:UUID_0]
            └── Expr:
                └── Literal 42 [ID:UUID_4::-::P:UUID_3]";
    assert_eq!(stripped_uuids.trim(), expected);
}
#[test]
fn test_for_complete_not_empty_body() {
    let stmt = Stmt::For {
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit(1)]))),
        condition: Some(binary_expr(variable_expr("x"), BinaryOp::Less, num_lit(2))),
        increment: Some(assign_expr(variable_expr("x"), binary_expr(variable_expr("x"), BinaryOp::Add, num_lit(1)))),
        body: vec![Stmt::Expression { expr: num_lit(42) }],
        span: dummy_span(),
    };
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── For [ID:UUID_0::-::P:-]
    ├── Initializer:
    │   └── VarDeclaration [ID:UUID_1::-::P:UUID_0]
    │       ├── Variables:
    │       │   └── x
    │       ├── Type:
    │       │   └── i32
    │       └── Initializers:
    │           └── Literal 1 [ID:UUID_2::-::P:UUID_1]
    ├── Condition:
    │   └── BinaryOp Less [ID:UUID_3::-::P:UUID_0]
    │       ├── Left:
    │       │   └── Variable 'x' [ID:UUID_4::-::P:UUID_3]
    │       └── Right:
    │           └── Literal 2 [ID:UUID_5::-::P:UUID_3]
    ├── Increment:
    │   └── Assignment [ID:UUID_6::-::P:UUID_0]
    │       ├── Target:
    │       │   └── Variable 'x' [ID:UUID_7::-::P:UUID_6]
    │       └── Value:
    │           └── BinaryOp Add [ID:UUID_8::-::P:UUID_6]
    │               ├── Left:
    │               │   └── Variable 'x' [ID:UUID_9::-::P:UUID_8]
    │               └── Right:
    │                   └── Literal 1 [ID:UUID_10::-::P:UUID_8]
    └── Body:
        └── Expression [ID:UUID_11::-::P:UUID_0]
            └── Expr:
                └── Literal 42 [ID:UUID_12::-::P:UUID_11]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_edge_case_multiple_parameters() {
    let stmt = function_declaration(
        "func".into(),
        vec![
            Parameter { name: "a".into(), type_annotation: Type::I32, span: dummy_span() },
            Parameter { name: "b".into(), type_annotation: Type::I32, span: dummy_span() },
            Parameter { name: "c".into(), type_annotation: Type::I32, span: dummy_span() },
        ],
        Type::Void,
        vec![],
    );

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Function [ID:UUID_0::-::P:-]
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
    assert_eq!(stripped_uuids.trim(), expected);
}

macro_rules! test_type_output {
    ($name:ident, $typ:expr, $type_str:expr) => {
        #[test]
        fn $name() {
            let stmt = function_declaration("func".into(), vec![], $typ, vec![]);

            let mut transformer = AstToHirTransformer::new();
            let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
            let output = pretty_print_stmt_hir(&hirstmt);
            let stripped = strip_ansi_codes(&output);
            let stripped_uuids = sanitize_mdata_uuids(&stripped);

            let expected = format!(
                "└── Function [ID:UUID_0::-::P:-]
    ├── Name:
    │   └── func
    ├── Parameters:
    ├── Return Type:
    │   └── {}
    └── Body:",
                $type_str
            );
            assert_eq!(stripped_uuids.trim(), expected);
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
test_type_output!(test_custom_output, Type::Custom("inin".into()), "inin");

#[test]
fn test_corner_case_deeply_nested_if() {
    let inner_if = Stmt::If {
        condition: bool_lit(false),
        then_branch: vec![Stmt::Expression { expr: num_lit(3) }],
        else_branch: None,
        span: dummy_span(),
    };
    let stmt =
        Stmt::If { condition: bool_lit(true), then_branch: vec![inner_if], else_branch: None, span: dummy_span() };
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── If [ID:UUID_0::-::P:-]
    ├── Condition:
    │   └── Literal true [ID:UUID_1::-::P:UUID_0]
    └── Then:
        └── If [ID:UUID_2::-::P:UUID_0]
            ├── Condition:
            │   └── Literal false [ID:UUID_3::-::P:UUID_2]
            └── Then:
                └── Expression [ID:UUID_4::-::P:UUID_2]
                    └── Expr:
                        └── Literal 3 [ID:UUID_5::-::P:UUID_4]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_corner_case_complex_return_type() {
    let stmt = function_declaration(
        "getVector".into(),
        vec![],
        Type::Vector(Box::new(Type::Array(
            Box::new(Type::I32),
            Box::new(Expr::Literal { value: LiteralValue::Nullptr, span: dummy_span() }),
        ))),
        vec![],
    );

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Function [ID:UUID_0::-::P:-]
    ├── Name:
    │   └── getVector
    ├── Parameters:
    ├── Return Type:
    │   └── Vector<[i32; <expr>]>
    └── Body:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_break_stmt() {
    let stmt = Stmt::Break { span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    assert_eq!(stripped_uuids.trim(), "└── Break [ID:UUID_0::-::P:-]");
}
#[test]
fn test_continue_stmt() {
    let stmt = Stmt::Continue { span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    assert_eq!(stripped_uuids.trim(), "└── Continue [ID:UUID_0::-::P:-]");
}

#[test]
fn test_array_literal_output() {
    let input = "var arr: i8[5] = {1, 2, 3, 4, 5}";
    let mut lexer = Lexer::new("test.vn", input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(expr[0].clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── VarDeclaration [ID:UUID_0::-::P:-]
    ├── Variables:
    │   └── arr
    ├── Type:
    │   └── [i8; 5]
    └── Initializers:
        └── Array Literal [ID:UUID_1::-::P:UUID_0]
            └── Elements:
                ├── Literal 1 [ID:UUID_2::-::P:UUID_1]
                ├── Literal 2 [ID:UUID_3::-::P:UUID_1]
                ├── Literal 3 [ID:UUID_4::-::P:UUID_1]
                ├── Literal 4 [ID:UUID_5::-::P:UUID_1]
                └── Literal 5 [ID:UUID_6::-::P:UUID_1]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_main() {
    let input = "main { }";
    let mut lexer = Lexer::new("test.vn", input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(expr[0].clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── MainFunction
    └── Block: (empty)";
    assert_eq!(stripped_uuids.trim(), expected);
}
*/