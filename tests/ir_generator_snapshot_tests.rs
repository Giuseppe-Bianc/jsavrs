use insta::assert_debug_snapshot;
use jsavrs::ir::generator::IrGenerator;
use jsavrs::parser::ast::*;
use jsavrs::tokens::number::Number;
use jsavrs::utils::*;

#[test]
fn test_generate_function_with_return() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_void_function() {
    let ast = vec![function_declaration(
        "void_func".to_string(),
        vec![],
        Type::Void,
        vec![Stmt::Return {
            value: None,
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_main_function() {
    let ast = vec![Stmt::MainFunction {
        body: vec![Stmt::Return {
            value: None,
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_binary_expression() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::I32(10)),
                    span: dummy_span(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::I32(20)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_unary_expression() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(Expr::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::I32(42)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_variable_assignment() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![
            Stmt::VarDeclaration {
                variables: vec!["x".to_string()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![],
                span: dummy_span(),
            },
            Stmt::Expression {
                expr: Expr::Assign {
                    target: Box::new(Expr::Variable {
                        name: "x".to_string(),
                        span: dummy_span(),
                    }),
                    value: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::I32(10)),
                        span: dummy_span(),
                    }),
                    span: dummy_span(),
                },
            },
        ],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_if_statement() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![Stmt::If {
            condition: Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            },
            then_branch: vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
            else_branch: None,
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_nested_expressions() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(Expr::Binary {
                left: Box::new(Expr::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::I32(5)),
                        span: dummy_span(),
                    }),
                    span: dummy_span(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::I32(3)),
                        span: dummy_span(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::I32(2)),
                        span: dummy_span(),
                    }),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_custom_type() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![Parameter {
            name: "param".to_string(),
            type_annotation: Type::Custom("MyType".to_string()),
            span: dummy_span(),
        }],
        Type::Custom("MyType".to_string()),
        vec![Stmt::Return {
            value: Some(Expr::Variable {
                name: "param".to_string(),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_array_type() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![Stmt::VarDeclaration {
            variables: vec!["arr".to_string()],
            type_annotation: Type::Array(
                Box::new(Type::I32),
                Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(10)),
                    span: dummy_span(),
                }),
            ),
            is_mutable: true,
            initializers: vec![],
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_missing_return() {
    let ast = vec![Stmt::Function {
        name: "test".to_string(),
        parameters: vec![],
        return_type: Type::I32,
        body: vec![], // No return statement
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_multiple_functions() {
    let ast = vec![
        Stmt::Function {
            name: "func1".to_string(),
            parameters: vec![],
            return_type: Type::Void,
            body: vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        Stmt::Function {
            name: "func2".to_string(),
            parameters: vec![],
            return_type: Type::Void,
            body: vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
    ];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_string_literal() {
    let ast = vec![Stmt::Function {
        name: "test".to_string(),
        parameters: vec![],
        return_type: Type::String,
        body: vec![Stmt::Return {
            value: Some(Expr::Literal {
                value: LiteralValue::StringLit("hello".to_string()),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_nullptr() {
    let ast = vec![Stmt::Function {
        name: "test".to_string(),
        parameters: vec![],
        return_type: Type::NullPtr,
        body: vec![Stmt::Return {
            value: Some(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}
