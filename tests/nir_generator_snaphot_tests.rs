use insta::assert_debug_snapshot;
use jsavrs::error::compile_error::CompileError;
use jsavrs::nir::generator::NIrGenerator;
use jsavrs::nir::Function;
use jsavrs::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type, UnaryOp};
use jsavrs::tokens::number::Number;
use jsavrs::utils::*;

#[test]
fn test_generate_function_with_return() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(num_lit_i32(42)),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
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

    let mut generator = NIrGenerator::new();
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

    let mut generator = NIrGenerator::new();
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
            value: Some(binary_expr(num_lit_i32(10), BinaryOp::Add, num_lit_i32(20))),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
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
            var_declaration(vec!["x".to_string()], Type::I32, true, vec![]),
            Stmt::Expression {
                expr: assign_expr(variable_expr("x"), num_lit_i32(10)),
            },
        ],
    )];

    let mut generator = NIrGenerator::new();
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
            condition: bool_lit(true),
            then_branch: vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
            else_branch: None,
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
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
            value: Some(binary_expr(
                unary_expr(UnaryOp::Negate, num_lit_i32(5)),
                BinaryOp::Multiply,
                binary_expr(num_lit_i32(3), BinaryOp::Add, num_lit_i32(2)),
            )),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
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
            value: Some(variable_expr("param")),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_array_type() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![var_declaration(
            vec!["arr".to_string()],
            Type::Array(Box::new(Type::I32), Box::new(num_lit(10))),
            true,
            vec![],
        )],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_missing_return() {
    let ast = vec![Stmt::Function {
        name: "test".to_string(),
        parameters: vec![],
        return_type: Type::I32,
        body: vec![],
        span: dummy_span(),
    }];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_multiple_functions() {
    let ast = vec![
        function_declaration(
            "func1".to_string(),
            vec![],
            Type::Void,
            vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
        ),
        function_declaration(
            "func2".to_string(),
            vec![],
            Type::Void,
            vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
        ),
    ];

    let mut generator = NIrGenerator::new();
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
            value: Some(string_lit("hello")),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = NIrGenerator::new();
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
            value: Some(nullptr_lit()),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_simple_block() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![
            Stmt::Block {
                statements: vec![
                    var_declaration(vec!["y".to_string()], Type::I32, true, vec![num_lit_i32(5)]),
                    Stmt::Expression {
                        expr: Expr::Assign {
                            target: Box::new(variable_expr("y")),
                            value: Box::new(num_lit_i32(10)),
                            span: dummy_span(),
                        },
                    },
                ],
                span: dummy_span(),
            },
            Stmt::Return {
                value: None,
                span: dummy_span(),
            },
        ],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_simple_while_loop() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![
            var_declaration(vec!["counter".to_string()], Type::I32, true, vec![num_lit_i32(0)]),
            Stmt::While {
                condition: binary_expr(variable_expr("counter"), BinaryOp::Less, num_lit_i32(5)),
                body: vec![Stmt::Expression {
                    expr: Expr::Assign {
                        target: Box::new(variable_expr("counter")),
                        value: Box::new(binary_expr(
                            variable_expr("counter"),
                            BinaryOp::Add,
                            num_lit_i32(1),
                        )),
                        span: dummy_span(),
                    },
                }],
                span: dummy_span(),
            },
            Stmt::Return {
                value: None,
                span: dummy_span(),
            },
        ],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_for_loop_basic() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![Stmt::For {
            initializer: Some(Box::new(Stmt::VarDeclaration {
                variables: vec!["i".to_string()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![num_lit_i32(0)],
                span: dummy_span(),
            })),
            condition: Some(binary_expr(
                variable_expr("i"),
                BinaryOp::Less,
                num_lit_i32(10),
            )),
            increment: Some(Expr::Assign {
                target: Box::new(variable_expr("i")),
                value: Box::new(binary_expr(
                    variable_expr("i"),
                    BinaryOp::Add,
                    num_lit_i32(1),
                )),
                span: dummy_span(),
            }),
            body: vec![],
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_for_loop_with_break() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![Stmt::For {
            initializer: Some(Box::new(Stmt::VarDeclaration {
                variables: vec!["i".to_string()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![num_lit_i32(0)],
                span: dummy_span(),
            })),
            condition: Some(binary_expr(
                variable_expr("i"),
                BinaryOp::Less,
                num_lit_i32(10),
            )),
            increment: None,
            body: vec![Stmt::Break { span: dummy_span() }],
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_for_loop_with_continue() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![Stmt::For {
            initializer: Some(Box::new(Stmt::VarDeclaration {
                variables: vec!["i".to_string()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![num_lit_i32(0)],
                span: dummy_span(),
            })),
            condition: Some(binary_expr(
                variable_expr("i"),
                BinaryOp::Less,
                num_lit_i32(10),
            )),
            increment: None,
            body: vec![Stmt::Continue { span: dummy_span() }],
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_grouping_expression() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(grouping_expr(binary_expr(
                num_lit_i32(10),
                BinaryOp::Add,
                num_lit_i32(20),
            ))),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_array_literal_with_elements() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Array(Box::new(Type::I32), Box::new(num_lit(3))),
        vec![Stmt::Return {
            value: Some(Expr::ArrayLiteral {
                elements: vec![num_lit_i32(10), num_lit_i32(20), num_lit_i32(30)],
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_default_implementation() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(num_lit_i32(42)),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::default();
    let functions = generator.generate(ast);

    assert_debug_snapshot!(functions);
}

#[test]
fn test_generate_binary_all_operations() {
    let test_cases = vec![
        BinaryOp::Add,
        BinaryOp::Subtract,
        BinaryOp::Multiply,
        BinaryOp::Divide,
        BinaryOp::Modulo,
        BinaryOp::Equal,
        BinaryOp::NotEqual,
        BinaryOp::Less,
        BinaryOp::LessEqual,
        BinaryOp::Greater,
        BinaryOp::GreaterEqual,
        BinaryOp::And,
        BinaryOp::Or,
        BinaryOp::BitwiseAnd,
        BinaryOp::BitwiseOr,
        BinaryOp::BitwiseXor,
        BinaryOp::ShiftLeft,
        BinaryOp::ShiftRight,
    ];

    let mut results: Vec<(Vec<Function>, Vec<CompileError>)> = Vec::new();

    for ast_op in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            Type::I32,
            vec![Stmt::Return {
                value: Some(binary_expr(num_lit_i32(10), ast_op, num_lit_i32(20))),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::new();
        results.push(generator.generate(ast));
    }
    assert_debug_snapshot!(results);
}

#[test]
fn test_generate_unary_expression() {
    let test_cases = vec![
        UnaryOp::Negate,
        UnaryOp::Not,
    ];

    let mut results: Vec<(Vec<Function>, Vec<CompileError>)> = Vec::new();

    for ast_op in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            Type::I32,
            vec![Stmt::Return {
                value: Some(unary_expr(ast_op, num_lit_i32(42))),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::new();
        results.push(generator.generate(ast));
    }
    assert_debug_snapshot!(results);
}

#[test]
fn test_generate_integer_literals() {
    let test_cases = vec![
        Number::I8(42),
        Number::I16(1000),
        Number::I32(32000),
        Number::Integer(2_000_000_000),
        Number::U8(255),
        Number::U16(65535),
        Number::U32(4_000_000_000),
        Number::UnsignedInteger(18_000_000_000_000_000_000),
    ];

    let mut results: Vec<(Vec<Function>, Vec<CompileError>)> = Vec::new();

    for num in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            match num {
                Number::I8(_) => Type::I8,
                Number::I16(_) => Type::I16,
                Number::I32(_) => Type::I32,
                Number::Integer(_) => Type::I64,
                Number::U8(_) => Type::U8,
                Number::U16(_) => Type::U16,
                Number::U32(_) => Type::U32,
                Number::UnsignedInteger(_) => Type::U64,
                _ => Type::I32,
            },
            vec![Stmt::Return {
                value: Some(Expr::Literal {
                    value: LiteralValue::Number(num),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::new();
        results.push(generator.generate(ast));
    }
    assert_debug_snapshot!(results);
}

#[test]
fn test_generate_float_literals() {
    let test_cases = vec![
        Number::Float32(3.14),
        Number::Float64(123.456),
        Number::Scientific32(2.0, 2),
        Number::Scientific64(10.0, 3),
    ];

    let mut results: Vec<(Vec<Function>, Vec<CompileError>)> = Vec::new();

    for num in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            match num {
                Number::Float32(_) => Type::F32,
                Number::Float64(_) => Type::F64,
                Number::Scientific32(_, _) => Type::F32,
                Number::Scientific64(_, _) => Type::F64,
                _ => Type::F32,
            },
            vec![Stmt::Return {
                value: Some(Expr::Literal {
                    value: LiteralValue::Number(num),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::new();
        results.push(generator.generate(ast));
    }
    assert_debug_snapshot!(results);
}

#[test]
fn test_generate_boolean_literals() {
    let test_cases = vec![
        true,
        false
    ];

    let mut results: Vec<(Vec<Function>, Vec<CompileError>)> = Vec::new();

    for b in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            Type::Bool,
            vec![Stmt::Return {
                value: Some(bool_lit(b)),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::default();
        results.push(generator.generate(ast));
    }
    assert_debug_snapshot!(results);
}

#[test]
fn test_generate_char_literal() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Char,
        vec![Stmt::Return {
            value: Some(char_lit("A")),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::default();
    assert_debug_snapshot!(generator.generate(ast));
}

#[test]
fn test_generate_nullptr_literal() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::NullPtr,
        vec![Stmt::Return {
            value: Some(nullptr_lit()),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::default();
    assert_debug_snapshot!(generator.generate(ast));
}
