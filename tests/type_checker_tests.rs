use jsavrs::error::compile_error::CompileError;
use jsavrs::parser::ast::*;
use jsavrs::semantic::type_checker::TypeChecker;
use jsavrs::tokens::number::Number;
use jsavrs::utils::*;
use std::vec;

// Test helper
fn typecheck(ast: Vec<Stmt>) -> Vec<CompileError> {
    let mut checker = TypeChecker::new();
    checker.check(&ast)
}

fn typecheckd(ast: Vec<Stmt>) -> Vec<CompileError> {
    let mut checker = TypeChecker::default();
    checker.check(&ast)
}

#[test]
fn test_var_declaration_in_main() {
    let ast = vec![Stmt::MainFunction {
        body: vec![var_declaration(
            vec!["x".to_string()],
            Type::I32,
            true,
            vec![Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }],
        )],
        span: dummy_span(),
    }];
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_var_declaration_in_main_using_typecheck_default() {
    let ast = vec![Stmt::MainFunction {
        body: vec![var_declaration(
            vec!["x".to_string()],
            Type::I32,
            true,
            vec![Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }],
        )],
        span: dummy_span(),
    }];
    let errors = typecheckd(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_valid() {
    let ast = vec![var_declaration(
        vec!["x".to_string()],
        Type::I32,
        true,
        vec![Expr::Literal {
            value: LiteralValue::Number(Number::I32(42)),
            span: dummy_span(),
        }],
    )];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_in_block_valid() {
    let ast = vec![Stmt::Block {
        statements: vec![var_declaration(
            vec!["x".to_string()],
            Type::I32,
            true,
            vec![Expr::Literal {
                value: LiteralValue::Number(Number::I32(42)),
                span: dummy_span(),
            }],
        )],
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_type_mismatch() {
    let ast = vec![var_declaration(
        vec!["x".to_string()],
        Type::I32,
        true,
        vec![Expr::Literal {
            value: LiteralValue::StringLit("test".to_string()),
            span: dummy_span(),
        }],
    )];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Cannot assign string to i32 for variable 'x'")
    );
}

#[test]
fn test_function_call_valid() {
    let ast = vec![
        // Function declaration
        function_declaration(
            "add".to_string(),
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
            vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
        ),
        // Function call
        Stmt::Expression {
            expr: call_expr(
                variable_expr("add"),
                vec![
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(1)),
                        span: dummy_span(),
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::I32(2)),
                        span: dummy_span(),
                    },
                ],
            ),
        },
    ];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_call_not_using_variable() {
    let ast = vec![
        // Function call
        Stmt::Expression {
            expr: call_expr(
                array_access_expr(variable_expr("num"), num_lit_i32(0)),
                vec![
                    num_lit_i32(1),
                    num_lit_i32(2),
                ],
            ),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Callee must be a function name")
    );
}

#[test]
fn test_function_call_argument_mismatch() {
    let ast = vec![
        function_declaration(
            "add".to_string(),
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
            vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
        ),
        Stmt::Expression {
            expr: call_expr(
                variable_expr("add"),
                vec![
                    num_lit_i32(1),
                    // Wrong type argument
                    string_lit("two"),
                ],
            ),
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
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(bool_lit(true)),
            span: dummy_span(),
        }],
    )];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Return type mismatch: expected i32, found bool")
    );
}

#[test]
fn test_return_type_void() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: None,
            span: dummy_span(),
        }],
    )];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Function requires return type i32, found void")
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
            expr: array_access_expr(variable_expr("arr"), num_lit_i32(0)),
        },
    ];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_empty_array_literal() {
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
                elements: vec![],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Array access
        Stmt::Expression {
            expr: array_access_expr(variable_expr("arr"), num_lit_i32(0)),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array literal must have at least one element")
    );
}

#[test]
fn test_mismatched_types_in_array_literal() {
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
                elements: vec![num_lit_i32(1), char_lit("s")],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Array access
        Stmt::Expression {
            expr: array_access_expr(variable_expr("arr"), num_lit_i32(0)),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array elements must be of the same type, found i32 and char")
    );
}

#[test]
fn test_array_invalid_index_access() {
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
            expr: array_access_expr(variable_expr("arr"), char_lit("a")),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array index must be integer, found char")
    );
}

#[test]
fn test_numeric_promotion() {
    let ast = vec![Stmt::Expression {
        expr: binary_expr(num_lit_i32(42), BinaryOp::Add, float_lit(3.14)),
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
        expr: variable_expr("undefined"),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Undefined variable 'undefined'"));
}

#[test]
fn test_assign_to_undefined_variable() {
    let ast = vec![Stmt::Expression {
        expr: assign_expr(variable_expr("undefined"), num_lit_i32(43)),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Undefined variable 'undefined'"));
}

#[test]
fn test_immutable_assignment() {
    let ast = vec![
        var_declaration(
            // Constant declaration
            vec!["x".to_string()],
            Type::I32,
            false,
            vec![num_lit_i32(42)],
        ),
        Stmt::Expression {
            expr: assign_expr(
                variable_expr("x"),
                Expr::Literal {
                    value: LiteralValue::Number(Number::I32(43)),
                    span: dummy_span(),
                },
            ),
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
fn test_assign_f64_to_i32() {
    let ast = vec![
        var_declaration(
            vec!["x".to_string()],
            Type::I32,
            true,
            vec![num_lit_i32(42)],
        ),
        Stmt::Expression {
            expr: assign_expr(variable_expr("x"), float_lit(3.222)),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Cannot assign f64 to i32"));
}

#[test]
fn test_indexing_a_non_array_type() {
    let ast = vec![
        var_declaration(
            // Constant declaration
            vec!["x".to_string()],
            Type::I32,
            false,
            vec![num_lit_i32(42)],
        ),
        // Array access
        Stmt::Expression {
            expr: array_access_expr(variable_expr("x"), num_lit_i32(0)),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Indexing non-array type i32"));
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
        expr: binary_expr(num_lit_i32(10), BinaryOp::Add, num_lit_i32(20)),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_arithmetic_in_grouping_valid() {
    let ast = vec![Stmt::Expression {
        expr: grouping_expr(binary_expr(num_lit_i32(10), BinaryOp::Add, num_lit_i32(20))),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_arithmetic_invalid() {
    let ast = vec![Stmt::Expression {
        expr: binary_expr(bool_lit(true), BinaryOp::Add, num_lit_i32(20)),
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
        expr: binary_expr(num_lit_i32(10), BinaryOp::Less, num_lit_i32(20)),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_comparison_invalid() {
    let ast = vec![Stmt::Expression {
        expr: binary_expr(bool_lit(true), BinaryOp::Less, string_lit("test")),
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
        expr: binary_expr(bool_lit(true), BinaryOp::And, bool_lit(false)),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_logical_operations_invalid() {
    let ast = vec![Stmt::Expression {
        expr: binary_expr(num_lit_i32(1), BinaryOp::Or, bool_lit(false)),
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
        expr: binary_expr(num_lit_i32(10), BinaryOp::BitwiseAnd, num_lit_i32(20)),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_operations_invalid() {
    let ast = vec![Stmt::Expression {
        expr: binary_expr(bool_lit(true), BinaryOp::BitwiseOr, num_lit_i32(20)),
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
        expr: unary_expr(UnaryOp::Negate, num_lit_i32(10)),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_unary_negate_invalid() {
    let ast = vec![Stmt::Expression {
        expr: unary_expr(UnaryOp::Negate, bool_lit(true)),
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
        expr: unary_expr(UnaryOp::Not, bool_lit(true)),
    }];

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_unary_not_invalid() {
    let ast = vec![Stmt::Expression {
        expr: unary_expr(UnaryOp::Not, num_lit_i32(10)),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Logical not requires boolean operand, found i32")
    );
}

#[test]
fn test_if() {
    let ast = vec![Stmt::If {
        condition: bool_lit(true),
        then_branch: vec![Stmt::Expression {
            expr: num_lit_i32(42),
        }],
        else_branch: None,
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_if_invalid_condition() {
    let ast = vec![Stmt::If {
        condition: num_lit(32),
        then_branch: vec![Stmt::Expression {
            expr: num_lit_i32(42),
        }],
        else_branch: None,
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("If condition must be bool, found i64")
    );
}

#[test]
fn test_if_else() {
    let ast = vec![Stmt::If {
        condition: bool_lit(true),
        then_branch: vec![Stmt::Expression {
            expr: num_lit_i32(42),
        }],
        else_branch: Some(vec![Stmt::Block {
            statements: vec![],
            span: dummy_span(),
        }]),
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_return_outside_of_function() {
    let ast = vec![Stmt::Return {
        value: Some(num_lit_i32(42)),
        span: dummy_span(),
    }];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Return statement outside function")
    );
}

#[test]
fn test_function_arguments_numbers_mismatch() {
    let ast = vec![
        function_declaration(
            "add".to_string(),
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
            vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
        ),
        Stmt::Expression {
            expr: call_expr(
                variable_expr("add"),
                vec![num_lit_i32(2), num_lit_i32(3), num_lit_i32(4)],
            ),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Function 'add' expects 2 arguments, found 3")
    );
}
#[test]
fn test_invalid_assignment_target() {
    let ast = vec![
        function_declaration(
            "add".to_string(),
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
            vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
        ),
        Stmt::Expression {
            expr: assign_expr(
                call_expr(
                    variable_expr("add"),
                    vec![num_lit_i32(2), num_lit_i32(3), num_lit_i32(4)],
                ),
                num_lit_i32(43),
            ),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Invalid assignment target"));
}

#[test]
fn test_assign_wrong_type_to_array_access() {
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
                    num_lit_i32(1),
                    num_lit_i32(2),
                ],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Array access
        Stmt::Expression {
            expr: assign_expr(
                array_access_expr(variable_expr("arr"), num_lit_i32(0)),
                float_lit(3.12),
            ),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Cannot assign f64 to array element of type i32")
    );
}
#[test]
fn test_assign_to_array_access_whit_nullptr_index() {
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
                    num_lit_i32(1),
                    num_lit_i32(2),
                ],
                span: dummy_span(),
            }],
            span: dummy_span(),
        },
        // Array access
        Stmt::Expression {
            expr: assign_expr(
                array_access_expr(variable_expr("arr"), nullptr_lit()),
                num_lit_i32(33),
            ),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array index must be integer, found nullptr")
    );
}
#[test]
fn test_assign_to_a_non_array() {
    let ast = vec![
        // Array declaration
        Stmt::VarDeclaration {
            variables: vec!["arr".to_string()],
            type_annotation: Type::I32,
            is_mutable: true,
            initializers: vec![num_lit_i32(4)],
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: assign_expr(
                array_access_expr(variable_expr("arr"),num_lit_i32(2) ),
                num_lit_i32(33),
            ),
        },
    ];

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Indexing non-array type i32")
    );
}
