// tests/mlir_hir_ast_to_hir.rs
use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
use jsavrs::mlir::hir::ast_to_hir::AstToHirTransformer;
use jsavrs::mlir::hir::hirimp::{HIRExpr, HIRStmt, HIRType, HIRParameter};
use jsavrs::mlir::hir::node_metadata::{NodeMetadata, NodeId};
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
    let expr = binary_expr(num_lit_i64(1), BinaryOp::Add, num_lit_i64(2));
    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── BinaryOp Add [ID:UUID_0::-::P:-]
    ├── Left:
    │   └── Literal 1 [ID:UUID_1::-::P:UUID_0]
    └── Right:
        └── Literal 2 [ID:UUID_2::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_nested_binary_expr() {
    let inner = binary_expr(num_lit_i64(1), BinaryOp::Add, num_lit_i64(2));
    let expr = binary_expr(inner, BinaryOp::Multiply, num_lit_i64(3));

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
    let expr = unary_expr(UnaryOp::Negate, num_lit_i64(5));
    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── UnaryOp Negate [ID:UUID_0::-::P:-]
    └── Expr:
        └── Literal 5 [ID:UUID_1::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_grouping_expr() {
    let inner = binary_expr(num_lit_i64(1), BinaryOp::Add, num_lit_i64(2));
    let expr = grouping_expr(inner);

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Grouping [ID:UUID_0::-::P:-]
    └── Expr:
        └── BinaryOp Add [ID:UUID_1::-::P:UUID_0]
            ├── Left:
            │   └── Literal 1 [ID:UUID_2::-::P:UUID_1]
            └── Right:
                └── Literal 2 [ID:UUID_3::-::P:UUID_1]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_literal_values() {
    let test_cases = vec![
        (string_lit("test"), "└── Literal \"test\" [ID:UUID_0::-::P:-]"),
        (bool_lit(true), "└── Literal true [ID:UUID_0::-::P:-]"),
        (nullptr_lit(), "└── Literal nullptr [ID:UUID_0::-::P:-]"),
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
    let expr = assign_expr(variable_expr("x"), num_lit_i64(3));

    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Assignment [ID:UUID_0::-::P:-]
    ├── Target:
    │   └── Variable 'x' [ID:UUID_1::-::P:UUID_0]
    └── Value:
        └── Literal 3 [ID:UUID_2::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_function_call() {
    let callee = variable_expr("foo");
    let args = vec![num_lit_i64(1), binary_expr(num_lit_i64(2), BinaryOp::Add, num_lit_i64(3))];
    let expr = call_expr(callee, args);
    let mut transformer = AstToHirTransformer::new();
    let hirexpr = transformer.transform_expr(expr.clone()).unwrap();
    let output = pretty_print_hir(&hirexpr);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);
    let expected = "\
└── Function Call [ID:UUID_0::-::P:-]
    ├── Callee:
    │   └── Variable 'foo' [ID:UUID_1::-::P:UUID_0]
    └── Arguments:
            ├── Arg:
            │   └── Literal 1 [ID:UUID_2::-::P:UUID_0]
            └── Arg:
                └── BinaryOp Add [ID:UUID_3::-::P:UUID_0]
                    ├── Left:
                    │   └── Literal 2 [ID:UUID_4::-::P:UUID_3]
                    └── Right:
                        └── Literal 3 [ID:UUID_5::-::P:UUID_3]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_array_access() {
    let array = variable_expr("arr");
    let index = binary_expr(variable_expr("i"), BinaryOp::Add, num_lit_i64(1));
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
        binary_expr(binary_expr(num_lit_i64(1), BinaryOp::Add, num_lit_i64(2)), BinaryOp::Add, num_lit_i64(3)),
        BinaryOp::Add,
        num_lit_i64(4),
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
└── UnaryOp Not [ID:UUID_0::-::P:-]
    └── Expr:
        └── UnaryOp Not [ID:UUID_1::-::P:UUID_0]
            └── Expr:
                └── Literal true [ID:UUID_2::-::P:UUID_1]";
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
    left: Box::new(num_lit_i64(1)),
    op: BinaryOp::Add,
    right: Box::new(num_lit_i64(2)),
    span: s,
});

expr_span_test!(test_expr_array_literal_span, |s| Expr::ArrayLiteral {
    elements: vec![num_lit_i64(1), num_lit_i64(2),],
    span: s,
});

expr_span_test!(test_expr_unary_span, |s| Expr::Unary { op: UnaryOp::Negate, expr: Box::new(num_lit_i64(5)), span: s });

expr_span_test!(test_expr_grouping_span, |s| Expr::Grouping { expr: Box::new(bool_lit(true)), span: s });

expr_span_test!(test_expr_literal_span, |s| Expr::Literal { value: LiteralValue::Nullptr, span: s });

expr_span_test!(test_expr_variable_span, |s| Expr::Variable { name: "x".into(), span: s });
expr_span_test!(test_expr_assign_span, |s| Expr::Assign {
    target: Box::new(variable_expr("x")),
    value: Box::new(num_lit_i64(3)),
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
    let expr = num_lit_i64(42);
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
    let inner_expr = binary_expr(num_lit_i64(1), BinaryOp::Add, num_lit_i64(2));

    let outer_expr = grouping_expr(inner_expr);

    assert_eq!(outer_expr.span(), &dummy_span());
}

#[test]
fn test_stmt_expression() {
    let stmt = Stmt::Expression { expr: num_lit_i64(42) };
    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Expression [ID:UUID_0::-::P:-]
    └── Expr:
        └── Literal 42 [ID:UUID_1::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_var_declaration_multiple_vars() {
    let stmt = var_declaration(vec!["x".into(), "y".into()], Type::I32, true, vec![num_lit_i64(1), num_lit_i64(2)]);

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── VarDeclaration [ID:UUID_0::-::P:-]
    ├── Variables:
    │   ├── x
    │   └── y
    ├── Type:
    │   └── i32
    └── Initializers:
        ├── Literal 1 [ID:UUID_1::-::P:UUID_0]
        └── Literal 2 [ID:UUID_2::-::P:UUID_0]";
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
└── Function [ID:UUID_0::-::P:-]
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
        └── Return [ID:UUID_1::-::P:UUID_0]
            └── Value:
                └── BinaryOp Add [ID:UUID_2::-::P:UUID_1]
                    ├── Left:
                    │   └── Variable 'a' [ID:UUID_3::-::P:UUID_2]
                    └── Right:
                        └── Variable 'b' [ID:UUID_4::-::P:UUID_2]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_if_stmt_with_else() {
    let condition = bool_lit(true);
    let then_branch = vec![Stmt::Expression { expr: num_lit_i64(1) }];
    let else_branch = vec![Stmt::Expression { expr: num_lit_i64(2) }];

    let stmt = Stmt::If { condition, then_branch, else_branch: Some(else_branch), span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── If [ID:UUID_0::-::P:-]
    ├── Condition:
    │   └── Literal true [ID:UUID_1::-::P:UUID_0]
    ├── Then:
    │   └── Expression [ID:UUID_2::-::P:UUID_0]
    │       └── Expr:
    │           └── Literal 1 [ID:UUID_3::-::P:UUID_2]
    └── Else:
        └── Expression [ID:UUID_4::-::P:UUID_0]
            └── Expr:
                └── Literal 2 [ID:UUID_5::-::P:UUID_4]";
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
        statements: vec![Stmt::Block {
            statements: vec![Stmt::Expression { expr: num_lit_i64(42) }],
            span: dummy_span(),
        }],
        span: dummy_span(),
    };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Block [ID:UUID_0::-::P:-]
    └── Block [ID:UUID_1::-::P:UUID_0]
        └── Expression [ID:UUID_2::-::P:UUID_1]
            └── Expr:
                └── Literal 42 [ID:UUID_3::-::P:UUID_2]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_return_stmt_with_value() {
    let stmt = Stmt::Return { value: Some(num_lit_i64(42)), span: dummy_span() };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── Return [ID:UUID_0::-::P:-]
    └── Value:
        └── Literal 42 [ID:UUID_1::-::P:UUID_0]";
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
└── While [ID:UUID_0::-::P:-]
    ├── Condition:
    │   └── Literal true [ID:UUID_1::-::P:UUID_0]
    └── Body:";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_while_not_empty_body() {
    let stmt = Stmt::While {
        condition: bool_lit(true),
        body: vec![Stmt::Expression { expr: num_lit_i64(42) }],
        span: dummy_span(),
    };

    let mut transformer = AstToHirTransformer::new();
    let hirstmt = transformer.transform_stmt(stmt.clone()).unwrap();
    let output = pretty_print_stmt_hir(&hirstmt);
    let stripped = strip_ansi_codes(&output);
    let stripped_uuids = sanitize_mdata_uuids(&stripped);

    let expected = "\
└── While [ID:UUID_0::-::P:-]
    ├── Condition:
    │   └── Literal true [ID:UUID_1::-::P:UUID_0]
    └── Body:
        └── Expression [ID:UUID_2::-::P:UUID_0]
            └── Expr:
                └── Literal 42 [ID:UUID_3::-::P:UUID_2]";
    assert_eq!(stripped_uuids.trim(), expected);
}

#[test]
fn test_for() {
    let stmt = Stmt::For {
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit_i64(1)]))),
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
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit_i64(1)]))),
        condition: Some(binary_expr(variable_expr("x"), BinaryOp::Less, num_lit_i64(2))),
        increment: Some(assign_expr(
            variable_expr("x"),
            binary_expr(variable_expr("x"), BinaryOp::Add, num_lit_i64(1)),
        )),
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
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit_i64(1)]))),
        condition: None,
        increment: None,
        body: vec![Stmt::Expression { expr: num_lit_i64(42) }],
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
        initializer: Some(Box::from(var_declaration(vec!["x".into()], Type::I32, true, vec![num_lit_i64(1)]))),
        condition: Some(binary_expr(variable_expr("x"), BinaryOp::Less, num_lit_i64(2))),
        increment: Some(assign_expr(
            variable_expr("x"),
            binary_expr(variable_expr("x"), BinaryOp::Add, num_lit_i64(1)),
        )),
        body: vec![Stmt::Expression { expr: num_lit_i64(42) }],
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
        then_branch: vec![Stmt::Expression { expr: num_lit_i64(3) }],
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
└── MainFunction [ID:UUID_0::-::P:-]
    └── Block: (empty) [ID:UUID_1::-::P:UUID_0]";
    assert_eq!(stripped_uuids.trim(), expected);
}

// Additional comprehensive tests for HIR implementation

// Tests for HIRExpr::span() method
#[test]
fn test_hir_expr_span_binary() {
    let span = create_test_span();
    let expr = HIRExpr::Binary {
        left: Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        op: BinaryOp::Add,
        right: Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(2)),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_unary() {
    let span = create_test_span();
    let expr = HIRExpr::Unary {
        op: UnaryOp::Negate,
        expr: Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(5)),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_grouping() {
    let span = create_test_span();
    let expr = HIRExpr::Grouping {
        expr: Box::new(HIRExpr::Literal {
            value: LiteralValue::Bool(true),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_literal() {
    let span = create_test_span();
    let expr = HIRExpr::Literal {
        value: LiteralValue::Nullptr,
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_variable() {
    let span = create_test_span();
    let expr = HIRExpr::Variable {
        name: "x".into(),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_assign() {
    let span = create_test_span();
    let expr = HIRExpr::Assign {
        target: Box::new(HIRExpr::Variable {
            name: "x".into(),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        value: Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(3)),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_call() {
    let span = create_test_span();
    let expr = HIRExpr::Call {
        callee: Box::new(HIRExpr::Variable {
            name: "foo".into(),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        arguments: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_array_access() {
    let span = create_test_span();
    let expr = HIRExpr::ArrayAccess {
        array: Box::new(HIRExpr::Variable {
            name: "arr".into(),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        index: Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(0)),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

#[test]
fn test_hir_expr_span_array_literal() {
    let span = create_test_span();
    let expr = HIRExpr::ArrayLiteral {
        elements: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(expr.span(), &span);
}

// Tests for HIRStmt::span() method
#[test]
fn test_hir_stmt_span_expression() {
    let span = create_test_span();
    let stmt = HIRStmt::Expression {
        expr: HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(42)),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        },
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_var_declaration() {
    let span = create_test_span();
    let stmt = HIRStmt::VarDeclaration {
        variables: vec!["x".into()],
        type_annotation: HIRType::I32,
        is_mutable: true,
        initializers: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_function() {
    let span = create_test_span();
    let stmt = HIRStmt::Function {
        name: "foo".into(),
        parameters: vec![],
        return_type: HIRType::Void,
        body: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_if() {
    let span = create_test_span();
    let stmt = HIRStmt::If {
        condition: HIRExpr::Literal {
            value: LiteralValue::Bool(true),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        },
        then_branch: vec![],
        else_branch: None,
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_while() {
    let span = create_test_span();
    let stmt = HIRStmt::While {
        condition: HIRExpr::Literal {
            value: LiteralValue::Bool(true),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        },
        body: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_for() {
    let span = create_test_span();
    let stmt = HIRStmt::For {
        initializer: None,
        condition: None,
        increment: None,
        body: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_block() {
    let span = create_test_span();
    let stmt = HIRStmt::Block {
        statements: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_return() {
    let span = create_test_span();
    let stmt = HIRStmt::Return {
        value: None,
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_break() {
    let span = create_test_span();
    let stmt = HIRStmt::Break {
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_continue() {
    let span = create_test_span();
    let stmt = HIRStmt::Continue {
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

#[test]
fn test_hir_stmt_span_main_function() {
    let span = create_test_span();
    let stmt = HIRStmt::MainFunction {
        body: vec![],
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    assert_eq!(stmt.span(), &span);
}

// Tests for HIRExpr::node_id() method
#[test]
fn test_hir_expr_node_id() {
    let node_metadata = NodeMetadata::new(None);
    let expr = HIRExpr::Literal {
        value: LiteralValue::Number(Number::Integer(42)),
        span: create_test_span(),
        node_metadata,
    };
    assert_eq!(expr.node_id(), node_metadata.node_id());
}

// Tests for HIRStmt::node_id() method
#[test]
fn test_hir_stmt_node_id() {
    let node_metadata = NodeMetadata::new(None);
    let stmt = HIRStmt::Expression {
        expr: HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(42)),
            span: create_test_span(),
            node_metadata: NodeMetadata::new(None),
        },
        node_metadata,
    };
    assert_eq!(stmt.node_id(), node_metadata.node_id());
}

// Tests for HIRExpr helper methods
#[test]
fn test_hir_expr_null_expr() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    let expr = HIRExpr::null_expr(span.clone(), node_metadata);
    match expr {
        HIRExpr::Literal { value: LiteralValue::Nullptr, span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected null expression"),
    }
}

#[test]
fn test_hir_expr_new_number_literal() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    let expr = HIRExpr::new_number_literal(Number::Integer(42), span.clone(), node_metadata).unwrap();
    match expr {
        HIRExpr::Literal { value: LiteralValue::Number(Number::Integer(42)), span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected number literal expression"),
    }
}

#[test]
fn test_hir_expr_new_bool_literal() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    let expr = HIRExpr::new_bool_literal(true, span.clone(), node_metadata).unwrap();
    match expr {
        HIRExpr::Literal { value: LiteralValue::Bool(true), span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected boolean literal expression"),
    }
}

#[test]
fn test_hir_expr_new_nullptr_literal() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    let expr = HIRExpr::new_nullptr_literal(span.clone(), node_metadata).unwrap();
    match expr {
        HIRExpr::Literal { value: LiteralValue::Nullptr, span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected nullptr literal expression"),
    }
}

#[test]
fn test_hir_expr_new_string_literal() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    let value: Arc<str> = "test".into();
    let expr = HIRExpr::new_string_literal(value.clone(), span.clone(), node_metadata).unwrap();
    match expr {
        HIRExpr::Literal { value: LiteralValue::StringLit(expr_value), span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(expr_value, value);
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected string literal expression"),
    }
}

#[test]
fn test_hir_expr_new_char_literal() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    let value: Arc<str> = "'a'".into();
    let expr = HIRExpr::new_char_literal(value.clone(), span.clone(), node_metadata).unwrap();
    match expr {
        HIRExpr::Literal { value: LiteralValue::CharLit(expr_value), span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(expr_value, value);
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected char literal expression"),
    }
}

// Tests for HIRType Display implementation
#[test]
fn test_hir_type_display_i8() {
    let typ = HIRType::I8;
    assert_eq!(format!("{}", typ), "i8");
}

#[test]
fn test_hir_type_display_i16() {
    let typ = HIRType::I16;
    assert_eq!(format!("{}", typ), "i16");
}

#[test]
fn test_hir_type_display_i32() {
    let typ = HIRType::I32;
    assert_eq!(format!("{}", typ), "i32");
}

#[test]
fn test_hir_type_display_i64() {
    let typ = HIRType::I64;
    assert_eq!(format!("{}", typ), "i64");
}

#[test]
fn test_hir_type_display_u8() {
    let typ = HIRType::U8;
    assert_eq!(format!("{}", typ), "u8");
}

#[test]
fn test_hir_type_display_u16() {
    let typ = HIRType::U16;
    assert_eq!(format!("{}", typ), "u16");
}

#[test]
fn test_hir_type_display_u32() {
    let typ = HIRType::U32;
    assert_eq!(format!("{}", typ), "u32");
}

#[test]
fn test_hir_type_display_u64() {
    let typ = HIRType::U64;
    assert_eq!(format!("{}", typ), "u64");
}

#[test]
fn test_hir_type_display_f32() {
    let typ = HIRType::F32;
    assert_eq!(format!("{}", typ), "f32");
}

#[test]
fn test_hir_type_display_f64() {
    let typ = HIRType::F64;
    assert_eq!(format!("{}", typ), "f64");
}

#[test]
fn test_hir_type_display_char() {
    let typ = HIRType::Char;
    assert_eq!(format!("{}", typ), "char");
}

#[test]
fn test_hir_type_display_string() {
    let typ = HIRType::String;
    assert_eq!(format!("{}", typ), "string");
}

#[test]
fn test_hir_type_display_bool() {
    let typ = HIRType::Bool;
    assert_eq!(format!("{}", typ), "bool");
}

#[test]
fn test_hir_type_display_custom() {
    let typ = HIRType::Custom("MyType".into());
    assert_eq!(format!("{}", typ), "MyType");
}

#[test]
fn test_hir_type_display_array() {
    let element_type = Box::new(HIRType::I32);
    let size_expr = Box::new(HIRExpr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: create_test_span(),
        node_metadata: NodeMetadata::new(None),
    });
    let typ = HIRType::Array(element_type, size_expr);
    assert_eq!(format!("{}", typ), "[i32; 5]");
}

#[test]
fn test_hir_type_display_array_non_literal() {
    let element_type = Box::new(HIRType::I32);
    let size_expr = Box::new(HIRExpr::Variable {
        name: "size".into(),
        span: create_test_span(),
        node_metadata: NodeMetadata::new(None),
    });
    let typ = HIRType::Array(element_type, size_expr);
    assert_eq!(format!("{}", typ), "[i32; <expr>]");
}

#[test]
fn test_hir_type_display_vector() {
    let element_type = Box::new(HIRType::I32);
    let typ = HIRType::Vector(element_type);
    assert_eq!(format!("{}", typ), "Vector<i32>");
}

#[test]
fn test_hir_type_display_void() {
    let typ = HIRType::Void;
    assert_eq!(format!("{}", typ), "void");
}

#[test]
fn test_hir_type_display_nullptr() {
    let typ = HIRType::NullPtr;
    assert_eq!(format!("{}", typ), "nullptr");
}

// Edge case tests for complex expressions
#[test]
fn test_hir_expr_complex_nested() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    
    // Create a complex nested expression: ((a + b) * c) - d
    let expr = HIRExpr::Binary {
        left: Box::new(HIRExpr::Binary {
            left: Box::new(HIRExpr::Binary {
                left: Box::new(HIRExpr::Variable {
                    name: "a".into(),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                }),
                op: BinaryOp::Add,
                right: Box::new(HIRExpr::Variable {
                    name: "b".into(),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                }),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            op: BinaryOp::Multiply,
            right: Box::new(HIRExpr::Variable {
                name: "c".into(),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        op: BinaryOp::Subtract,
        right: Box::new(HIRExpr::Variable {
            name: "d".into(),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        span: span.clone(),
        node_metadata,
    };
    
    // Verify the structure
    match &expr {
        HIRExpr::Binary { op: BinaryOp::Subtract, left, right, .. } => {
            match (left.as_ref(), right.as_ref()) {
                (HIRExpr::Binary { op: BinaryOp::Multiply, left: mult_left, right: mult_right, .. }, 
                 HIRExpr::Variable { name: d_name, .. }) => {
                    assert_eq!(d_name.as_ref(), "d");
                    
                    match mult_left.as_ref() {
                        HIRExpr::Binary { op: BinaryOp::Add, left: add_left, right: add_right, .. } => {
                            match (add_left.as_ref(), add_right.as_ref()) {
                                (HIRExpr::Variable { name: a_name, .. }, 
                                 HIRExpr::Variable { name: b_name, .. }) => {
                                    assert_eq!(a_name.as_ref(), "a");
                                    assert_eq!(b_name.as_ref(), "b");
                                }
                                _ => panic!("Expected variables a and b"),
                            }
                        }
                        _ => panic!("Expected addition expression"),
                    }
                    
                    match mult_right.as_ref() {
                        HIRExpr::Variable { name: c_name, .. } => {
                            assert_eq!(c_name.as_ref(), "c");
                        }
                        _ => panic!("Expected variable c"),
                    }
                }
                _ => panic!("Expected multiplication and variable d"),
            }
        }
        _ => panic!("Expected subtraction expression"),
    }
}

// Edge case tests for complex statements
#[test]
fn test_hir_stmt_complex_function() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    
    // Create a complex function with nested control structures
    let stmt = HIRStmt::Function {
        name: "complex_func".into(),
        parameters: vec![
            HIRParameter {
                name: "x".into(),
                type_annotation: HIRType::I32,
                span: span.clone(),
            }
        ],
        return_type: HIRType::I32,
        body: vec![
            HIRStmt::If {
                condition: HIRExpr::Binary {
                    left: Box::new(HIRExpr::Variable {
                        name: "x".into(),
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }),
                    op: BinaryOp::Greater,
                    right: Box::new(HIRExpr::Literal {
                        value: LiteralValue::Number(Number::Integer(0)),
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                },
                then_branch: vec![
                    HIRStmt::While {
                        condition: HIRExpr::Binary {
                            left: Box::new(HIRExpr::Variable {
                                name: "x".into(),
                                span: span.clone(),
                                node_metadata: NodeMetadata::new(None),
                            }),
                            op: BinaryOp::Less,
                            right: Box::new(HIRExpr::Literal {
                                value: LiteralValue::Number(Number::Integer(10)),
                                span: span.clone(),
                                node_metadata: NodeMetadata::new(None),
                            }),
                            span: span.clone(),
                            node_metadata: NodeMetadata::new(None),
                        },
                        body: vec![
                            HIRStmt::Expression {
                                expr: HIRExpr::Assign {
                                    target: Box::new(HIRExpr::Variable {
                                        name: "x".into(),
                                        span: span.clone(),
                                        node_metadata: NodeMetadata::new(None),
                                    }),
                                    value: Box::new(HIRExpr::Binary {
                                        left: Box::new(HIRExpr::Variable {
                                            name: "x".into(),
                                            span: span.clone(),
                                            node_metadata: NodeMetadata::new(None),
                                        }),
                                        op: BinaryOp::Add,
                                        right: Box::new(HIRExpr::Literal {
                                            value: LiteralValue::Number(Number::Integer(1)),
                                            span: span.clone(),
                                            node_metadata: NodeMetadata::new(None),
                                        }),
                                        span: span.clone(),
                                        node_metadata: NodeMetadata::new(None),
                                    }),
                                    span: span.clone(),
                                    node_metadata: NodeMetadata::new(None),
                                },
                                node_metadata: NodeMetadata::new(None),
                            }
                        ],
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }
                ],
                else_branch: Some(vec![
                    HIRStmt::Return {
                        value: Some(HIRExpr::Literal {
                            value: LiteralValue::Number(Number::Integer(0)),
                            span: span.clone(),
                            node_metadata: NodeMetadata::new(None),
                        }),
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }
                ]),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            },
            HIRStmt::Return {
                value: Some(HIRExpr::Variable {
                    name: "x".into(),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                }),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }
        ],
        span: span.clone(),
        node_metadata,
    };
    
    // Verify the structure
    match &stmt {
        HIRStmt::Function { name, parameters, return_type, body, .. } => {
            assert_eq!(name.as_ref(), "complex_func");
            assert_eq!(parameters.len(), 1);
            assert_eq!(parameters[0].name.as_ref(), "x");
            assert_eq!(*return_type, HIRType::I32);
            assert_eq!(body.len(), 2);
            
            // Check the if statement
            match &body[0] {
                HIRStmt::If { condition, then_branch, else_branch, .. } => {
                    // Check condition
                    match condition {
                        HIRExpr::Binary { op: BinaryOp::Greater, .. } => {}
                        _ => panic!("Expected greater than condition"),
                    }
                    
                    // Check then branch
                    assert_eq!(then_branch.len(), 1);
                    match &then_branch[0] {
                        HIRStmt::While { condition, body, .. } => {
                            match condition {
                                HIRExpr::Binary { op: BinaryOp::Less, .. } => {}
                                _ => panic!("Expected less than condition"),
                            }
                            
                            assert_eq!(body.len(), 1);
                            match &body[0] {
                                HIRStmt::Expression { expr: HIRExpr::Assign { .. }, .. } => {}
                                _ => panic!("Expected assignment expression"),
                            }
                        }
                        _ => panic!("Expected while statement"),
                    }
                    
                    // Check else branch
                    assert!(else_branch.is_some());
                    let else_branch = else_branch.as_ref().unwrap();
                    assert_eq!(else_branch.len(), 1);
                    match &else_branch[0] {
                        HIRStmt::Return { value: Some(HIRExpr::Literal { value: LiteralValue::Number(Number::Integer(0)), .. }), .. } => {}
                        _ => panic!("Expected return 0"),
                    }
                }
                _ => panic!("Expected if statement"),
            }
            
            // Check the final return statement
            match &body[1] {
                HIRStmt::Return { value: Some(HIRExpr::Variable { name, .. }), .. } => {
                    assert_eq!(name.as_ref(), "x");
                }
                _ => panic!("Expected return x"),
            }
        }
        _ => panic!("Expected function statement"),
    }
}

// Edge case tests for array literals
#[test]
fn test_hir_expr_array_literal_empty() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    
    let expr = HIRExpr::ArrayLiteral {
        elements: vec![],
        span: span.clone(),
        node_metadata,
    };
    
    match expr {
        HIRExpr::ArrayLiteral { elements, span: expr_span, node_metadata: expr_metadata } => {
            assert_eq!(elements.len(), 0);
            assert_eq!(expr_span, span);
            assert_eq!(expr_metadata, node_metadata);
        }
        _ => panic!("Expected array literal"),
    }
}

#[test]
fn test_hir_expr_array_literal_with_elements() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    
    let expr = HIRExpr::ArrayLiteral {
        elements: vec![
            HIRExpr::Literal {
                value: LiteralValue::Number(Number::Integer(1)),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            },
            HIRExpr::Literal {
                value: LiteralValue::Number(Number::Integer(2)),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            },
            HIRExpr::Literal {
                value: LiteralValue::Number(Number::Integer(3)),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            },
        ],
        span: span.clone(),
        node_metadata,
    };
    
    match expr {
        HIRExpr::ArrayLiteral { elements, .. } => {
            assert_eq!(elements.len(), 3);
            
            for (i, element) in elements.iter().enumerate() {
                match element {
                    HIRExpr::Literal { value: LiteralValue::Number(Number::Integer(n)), .. } => {
                        assert_eq!(*n, (i + 1) as i64);
                    }
                    _ => panic!("Expected number literal"),
                }
            }
        }
        _ => panic!("Expected array literal"),
    }
}

// Edge case tests for complex types
#[test]
fn test_hir_type_nested_array() {
    // Create a nested array type: [[i32; 3]; 2]
    let inner_array = Box::new(HIRType::Array(
        Box::new(HIRType::I32),
        Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(3)),
            span: create_test_span(),
            node_metadata: NodeMetadata::new(None),
        }),
    ));
    
    let outer_array = HIRType::Array(
        inner_array,
        Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(2)),
            span: create_test_span(),
            node_metadata: NodeMetadata::new(None),
        }),
    );
    
    assert_eq!(format!("{}", outer_array), "[[i32; 3]; 2]");
}

#[test]
fn test_hir_type_vector_of_arrays() {
    // Create a vector of arrays: Vector<[i32; 5]>
    let array_type = Box::new(HIRType::Array(
        Box::new(HIRType::I32),
        Box::new(HIRExpr::Literal {
            value: LiteralValue::Number(Number::Integer(5)),
            span: create_test_span(),
            node_metadata: NodeMetadata::new(None),
        }),
    ));
    
    let vector_type = HIRType::Vector(array_type);
    
    assert_eq!(format!("{}", vector_type), "Vector<[i32; 5]>");
}

// Edge case tests for node metadata
#[test]
fn test_hir_node_metadata_with_parent() {
    let parent_id = NodeId::new();
    let node_metadata = NodeMetadata::new(Some(parent_id));
    
    assert_eq!(node_metadata.node_id(), node_metadata.node_id());
    // We can't directly access the parent field, but we can test the display implementation
    let display_string = format!("{}", node_metadata);
    assert!(display_string.contains("P:"));
    assert!(!display_string.contains("P:-"));
}

#[test]
fn test_hir_node_metadata_without_parent() {
    let node_metadata = NodeMetadata::new(None);
    
    assert_eq!(node_metadata.node_id(), node_metadata.node_id());
    // We can't directly access the parent field, but we can test the display implementation
    let display_string = format!("{}", node_metadata);
    assert!(display_string.contains("P:-"));
}

// Edge case tests for large expressions
#[test]
fn test_hir_expr_very_deeply_nested() {
    let span = create_test_span();
    let mut expr = HIRExpr::Literal {
        value: LiteralValue::Number(Number::Integer(1)),
        span: span.clone(),
        node_metadata: NodeMetadata::new(None),
    };
    
    // Create a very deeply nested expression
    for _ in 0..100 {
        expr = HIRExpr::Binary {
            left: Box::new(expr),
            op: BinaryOp::Add,
            right: Box::new(HIRExpr::Literal {
                value: LiteralValue::Number(Number::Integer(1)),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        };
    }
    
    // Verify we can traverse the deeply nested structure without stack overflow
    fn count_depth(expr: &HIRExpr) -> usize {
        match expr {
            HIRExpr::Binary { left, .. } => 1 + count_depth(left.as_ref()),
            _ => 0,
        }
    }
    
    assert_eq!(count_depth(&expr), 100);
}

// Edge case tests for complex control flow
#[test]
fn test_hir_stmt_nested_control_flow() {
    let span = create_test_span();
    let node_metadata = NodeMetadata::new(None);
    
    // Create nested control flow: for loop containing if/else containing while loop
    let stmt = HIRStmt::For {
        initializer: Some(Box::new(HIRStmt::VarDeclaration {
            variables: vec!["i".into()],
            type_annotation: HIRType::I32,
            is_mutable: true,
            initializers: vec![HIRExpr::Literal {
                value: LiteralValue::Number(Number::Integer(0)),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }],
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        })),
        condition: Some(HIRExpr::Binary {
            left: Box::new(HIRExpr::Variable {
                name: "i".into(),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            op: BinaryOp::Less,
            right: Box::new(HIRExpr::Literal {
                value: LiteralValue::Number(Number::Integer(10)),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        increment: Some(HIRExpr::Assign {
            target: Box::new(HIRExpr::Variable {
                name: "i".into(),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            value: Box::new(HIRExpr::Binary {
                left: Box::new(HIRExpr::Variable {
                    name: "i".into(),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                }),
                op: BinaryOp::Add,
                right: Box::new(HIRExpr::Literal {
                    value: LiteralValue::Number(Number::Integer(1)),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                }),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }),
            span: span.clone(),
            node_metadata: NodeMetadata::new(None),
        }),
        body: vec![
            HIRStmt::If {
                condition: HIRExpr::Binary {
                    left: Box::new(HIRExpr::Variable {
                        name: "i".into(),
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }),
                    op: BinaryOp::Equal,
                    right: Box::new(HIRExpr::Literal {
                        value: LiteralValue::Number(Number::Integer(5)),
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }),
                    span: span.clone(),
                    node_metadata: NodeMetadata::new(None),
                },
                then_branch: vec![
                    HIRStmt::While {
                        condition: HIRExpr::Binary {
                            left: Box::new(HIRExpr::Variable {
                                name: "i".into(),
                                span: span.clone(),
                                node_metadata: NodeMetadata::new(None),
                            }),
                            op: BinaryOp::Greater,
                            right: Box::new(HIRExpr::Literal {
                                value: LiteralValue::Number(Number::Integer(0)),
                                span: span.clone(),
                                node_metadata: NodeMetadata::new(None),
                            }),
                            span: span.clone(),
                            node_metadata: NodeMetadata::new(None),
                        },
                        body: vec![
                            HIRStmt::Expression {
                                expr: HIRExpr::Assign {
                                    target: Box::new(HIRExpr::Variable {
                                        name: "i".into(),
                                        span: span.clone(),
                                        node_metadata: NodeMetadata::new(None),
                                    }),
                                    value: Box::new(HIRExpr::Binary {
                                        left: Box::new(HIRExpr::Variable {
                                            name: "i".into(),
                                            span: span.clone(),
                                            node_metadata: NodeMetadata::new(None),
                                        }),
                                        op: BinaryOp::Subtract,
                                        right: Box::new(HIRExpr::Literal {
                                            value: LiteralValue::Number(Number::Integer(1)),
                                            span: span.clone(),
                                            node_metadata: NodeMetadata::new(None),
                                        }),
                                        span: span.clone(),
                                        node_metadata: NodeMetadata::new(None),
                                    }),
                                    span: span.clone(),
                                    node_metadata: NodeMetadata::new(None),
                                },
                                node_metadata: NodeMetadata::new(None),
                            }
                        ],
                        span: span.clone(),
                        node_metadata: NodeMetadata::new(None),
                    }
                ],
                else_branch: Some(vec![
                    HIRStmt::Expression {
                        expr: HIRExpr::Assign {
                            target: Box::new(HIRExpr::Variable {
                                name: "i".into(),
                                span: span.clone(),
                                node_metadata: NodeMetadata::new(None),
                            }),
                            value: Box::new(HIRExpr::Literal {
                                value: LiteralValue::Number(Number::Integer(0)),
                                span: span.clone(),
                                node_metadata: NodeMetadata::new(None),
                            }),
                            span: span.clone(),
                            node_metadata: NodeMetadata::new(None),
                        },
                        node_metadata: NodeMetadata::new(None),
                    }
                ]),
                span: span.clone(),
                node_metadata: NodeMetadata::new(None),
            }
        ],
        span: span.clone(),
        node_metadata,
    };
    
    // Verify the structure
    match &stmt {
        HIRStmt::For { initializer, condition, increment, body, .. } => {
            // Check initializer
            assert!(initializer.is_some());
            match initializer.as_ref().unwrap().as_ref() {
                HIRStmt::VarDeclaration { variables, initializers, .. } => {
                    assert_eq!(variables.len(), 1);
                    assert_eq!(variables[0].as_ref(), "i");
                    assert_eq!(initializers.len(), 1);
                }
                _ => panic!("Expected variable declaration"),
            }
            
            // Check condition
            assert!(condition.is_some());
            match condition.as_ref().unwrap() {
                HIRExpr::Binary { op: BinaryOp::Less, .. } => {}
                _ => panic!("Expected less than condition"),
            }
            
            // Check increment
            assert!(increment.is_some());
            match increment.as_ref().unwrap() {
                HIRExpr::Assign { .. } => {}
                _ => panic!("Expected assignment"),
            }
            
            // Check body
            assert_eq!(body.len(), 1);
            match &body[0] {
                HIRStmt::If { condition, then_branch, else_branch, .. } => {
                    match condition {
                        HIRExpr::Binary { op: BinaryOp::Equal, .. } => {}
                        _ => panic!("Expected equality condition"),
                    }
                    
                    assert_eq!(then_branch.len(), 1);
                    match &then_branch[0] {
                        HIRStmt::While { .. } => {}
                        _ => panic!("Expected while loop"),
                    }
                    
                    assert!(else_branch.is_some());
                    let else_branch = else_branch.as_ref().unwrap();
                    assert_eq!(else_branch.len(), 1);
                    match &else_branch[0] {
                        HIRStmt::Expression { expr: HIRExpr::Assign { .. }, .. } => {}
                        _ => panic!("Expected assignment"),
                    }
                }
                _ => panic!("Expected if statement"),
            }
        }
        _ => panic!("Expected for loop"),
    }
}