// tests/ast_snapshot_test.rs
use insta::{assert_debug_snapshot, assert_snapshot};
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
use jsavrs::parser::ast::*;
use jsavrs::parser::ast_printer::{pretty_print, pretty_print_stmt};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::utils::*;

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
    let cases = vec![string_lit("test"), bool_lit(true), nullptr_lit()];

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
    let expr = assign_expr(variable_expr("x"), num_lit(3));

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
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
    let array = variable_expr("arr");
    let index = binary_expr(variable_expr("i"), BinaryOp::Add, num_lit(1));
    let expr = array_access_expr(array, index);

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
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

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_multiple_unary_ops() {
    let expr = unary_expr(UnaryOp::Not, unary_expr(UnaryOp::Not, bool_lit(true)));

    let output = pretty_print(&expr);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_stmt_expression() {
    let stmt = Stmt::Expression { expr: num_lit(42) };
    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
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

    assert_snapshot!(stripped.trim());
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

    assert_snapshot!(stripped.trim());
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
            statements: vec![Stmt::Expression { expr: num_lit(42) }],
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
        value: Some(num_lit(42)),
        span: dummy_span(),
    };

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}

#[test]
fn test_complex_type_declaration() {
    let stmt = var_declaration(
        vec!["matrix".to_string()],
        Type::Array(
            Box::new(Type::F64),
            Box::new(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            }),
        ),
        vec![],
    );

    let output = pretty_print_stmt(&stmt);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
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

    assert_snapshot!(stripped.trim());
}

macro_rules! test_type_output {
    ($name:ident, $typ:expr) => {
        #[test]
        fn $name() {
            let stmt = function_declaration("func".to_string(), vec![], $typ, vec![]);

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

#[allow(clippy::needless_borrow)]
#[test]
fn test_array_literal_output() {
    let input = "var arr: i8[5] = {1, 2, 3, 4, 5}";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);

    let output = pretty_print_stmt(&expr[0]);
    let stripped = strip_ansi_codes(&output);

    assert_snapshot!(stripped.trim());
}
