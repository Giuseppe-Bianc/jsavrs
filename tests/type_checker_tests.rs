use jsavrs::error::compile_error::CompileError;
use jsavrs::parser::ast::*;
use jsavrs::semantic::type_checker::TypeChecker;
use jsavrs::tokens::number::Number;
use jsavrs::utils::dummy_span;

// Test helper
fn typecheck(ast: Vec<Stmt>) -> Vec<CompileError> {
    let mut checker = TypeChecker::new();
    checker.check(&ast)
}

#[test]
fn test_var_declaration_in_main(){
    let ast = vec![
        Stmt::MainFunction {
            body: vec![Stmt::VarDeclaration {
                variables: vec!["x".to_string()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![Expr::Literal {
                    value: LiteralValue::Number(Number::I32(42)),
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }],
            span: dummy_span(),
        }
    ];
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_valid() {
    let ast = vec![Stmt::VarDeclaration {
        variables: vec!["x".to_string()],
        type_annotation: Type::I32,
        is_mutable: true,
        initializers: vec![Expr::Literal {
            value: LiteralValue::Number(Number::I32(42)),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_in_block_valid() {
    let ast = vec![Stmt::Block {
        statements: vec![Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I32,
            is_mutable: true,
            initializers: vec![Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }],
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_type_mismatch() {
    let ast = vec![Stmt::VarDeclaration {
        variables: vec!["x".to_string()],
        type_annotation: Type::I32,
        is_mutable: true,
        initializers: vec![Expr::Literal {
            value: LiteralValue::StringLit("test".to_string()),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("Cannot assign"));
}

#[test]
fn test_function_call_valid() {
    let ast = vec![
        // Function declaration
        Stmt::Function {
            name: "add".to_string(),
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
            body: vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Function call
        Stmt::Expression {
            expr: Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "add".to_string(),
                    span: dummy_span(),
                }),
                arguments: vec![
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(1)),
                        span: dummy_span(),
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(2)),
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            },
        },
    ];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_call_argument_mismatch() {
    let ast = vec![
        Stmt::Function {
            name: "add".to_string(),
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
            body: vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "add".to_string(),
                    span: dummy_span(),
                }),
                arguments: vec![
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(1)),
                        span: dummy_span(),
                    },
                    // Wrong type argument
                    Expr::Literal {
                        value: LiteralValue::StringLit("two".to_string()),
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            },
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Argument 2: expected i32, found string")
    );
}

#[test]
fn test_return_type_mismatch() {
    let ast = vec![Stmt::Function {
        name: "test".to_string(),
        parameters: Vec::new(),
        return_type: Type::I32,
        body: vec![Stmt::Return {
            value: Some(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Return type mismatch: expected i32, found bool")
    );
}

#[test]
fn test_array_operations_valid() {
    let ast = vec![
        // Array declaration
        Stmt::VarDeclaration {
            variables: vec!["arr".to_string()],
            type_annotation: Type::Array(
                Box::new(Type::I32),
                Box::new(Expr::null_expr(dummy_span())),
            ),
            is_mutable: true,
            initializers: vec![Expr::ArrayLiteral {
                elements: vec![
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(1)),
                        span: dummy_span(),
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(2)),
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Array access
        Stmt::Expression {
            expr: Expr::ArrayAccess {
                array: Box::new(Expr::Variable {
                    name: "arr".to_string(),
                    span: dummy_span(),
                }),
                index: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::I32(0)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            },
        },
    ];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_numeric_promotion() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Float64(3.14)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_break_outside_loop() {
    let ast = vec![Stmt::Break { span: dummy_span() }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Break/continue outside loop"));
}

#[test]
fn test_continue_outside_loop() {
    let ast = vec![Stmt::Continue { span: dummy_span() }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Break/continue outside loop"));
}

#[test]
fn test_undefined_variable() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Variable {
            name: "undefined".to_string(),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("Undefined variable"));
}

#[test]
fn test_immutable_assignment() {
    let ast = vec![
        // Constant declaration
        Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I32,
            is_mutable: false,
            initializers: vec![Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Assignment attempt
        Stmt::Expression {
            expr: Expr::Assign {
                target: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    span: dummy_span(),
                }),
                value: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::I32(43)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            },
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Assignment to immutable variable 'x'")
    );
}

#[test]
fn test_main_function_signature() {
    let ast = vec![Stmt::MainFunction {
        body: vec![],
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

/*
#[test]
fn test_double_main_function_signature() {
    let ast = vec![
        Stmt::MainFunction {
            body: vec![],
            span: dummy_span(),
        },
        Stmt::MainFunction {
            body: vec![],
            span: dummy_span(),
        },
    ];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}*/

#[test]
fn test_binary_arithmetic_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
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
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_arithmetic_in_grouping_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Grouping {
            expr: Box::new(Expr::Binary {
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
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_arithmetic_invalid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(20)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Binary operator 'Add' requires numeric operands, found bool and i32")
    );
}

#[test]
fn test_binary_comparison_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(10)),
                span: dummy_span(),
            }),
            op: BinaryOp::Less,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(20)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_comparison_invalid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            op: BinaryOp::Less,
            right: Box::new(Expr::Literal {
                value: LiteralValue::StringLit("test".to_string()),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Comparison operator 'Less' requires compatible types, found bool and string")
    );
}

#[test]
fn test_logical_operations_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            op: BinaryOp::And,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Bool(false),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_logical_operations_invalid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(1)),
                span: dummy_span(),
            }),
            op: BinaryOp::Or,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Bool(false),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Logical operator 'Or' requires boolean operands, found i32 and bool")
    );
}

#[test]
fn test_bitwise_operations_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(10)),
                span: dummy_span(),
            }),
            op: BinaryOp::BitwiseAnd,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(20)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_operations_invalid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            op: BinaryOp::BitwiseOr,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(20)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Bitwise operator 'BitwiseOr' requires integer operands, found bool and i32")
    );
}

#[test]
fn test_unary_negate_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Unary {
            op: UnaryOp::Negate,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(10)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_unary_negate_invalid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Unary {
            op: UnaryOp::Negate,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Negation requires numeric operand, found bool")
    );
}

#[test]
fn test_unary_not_valid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_unary_not_invalid() {
    let ast = vec![Stmt::Expression {
        expr: Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::I32(10)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        },
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Logical not requires boolean operand, found i32")
    );
}
