use jsavrs::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type};
use jsavrs::semantic::type_checker::TypeChecker;
use jsavrs::tokens::number::Number;
use jsavrs::utils::{bool_lit, dummy_span, num_lit, string_lit};

#[test]
fn variable_declaration_and_assignment() {
    // Valid: mutable variable assignment
    let stmts = vec![
        Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I32,
            initializers: vec![num_lit(42)],
            is_mutable: true,
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::Assign {
                target: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    span: dummy_span(),
                }),
                value: Box::new(num_lit(100)),
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    assert!(checker.check(&stmts).is_empty());

    // Invalid: immutable assignment
    let stmts = vec![
        Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I32,
            initializers: vec![num_lit(42)],
            is_mutable: false,
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::Assign {
                target: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    span: dummy_span(),
                }),
                value: Box::new(num_lit(100)),
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);
}

#[test]
fn function_declaration_and_call() {
    // Valid function call
    let stmts = vec![
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
            body: vec![Stmt::Return {
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
            }],
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "add".to_string(),
                    span: dummy_span(),
                }),
                arguments: vec![num_lit(2), num_lit(3)],
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    assert!(checker.check(&stmts).is_empty());

    // Invalid: argument type mismatch
    let stmts = vec![
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
            body: vec![],
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::Call {
                callee: Box::new(Expr::Variable {
                    name: "add".to_string(),
                    span: dummy_span(),
                }),
                arguments: vec![bool_lit(true), num_lit(3)],
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);
}

#[test]
fn control_flow_statements() {
    // Valid if statement
    let stmts = vec![Stmt::If {
        condition: bool_lit(true),
        then_branch: vec![Stmt::Expression {
            expr: num_lit(42),
        }],
        else_branch: Some(vec![Stmt::Expression {
            expr: num_lit(0),
        }]),
        span: dummy_span(),
    }];

    let mut checker = TypeChecker::new();
    assert!(checker.check(&stmts).is_empty());

    // Invalid condition type
    let stmts = vec![Stmt::If {
        condition: num_lit(42),
        then_branch: vec![],
        else_branch: None,
        span: dummy_span(),
    }];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);
}

#[test]
fn expression_type_checking() {
    // Test through full check flow
    // Valid arithmetic
    let stmts = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(num_lit(10)),
            op: BinaryOp::Add,
            right: Box::new(num_lit(20)),
            span: dummy_span(),
        },
    }];

    let mut checker = TypeChecker::new();
    assert!(checker.check(&stmts).is_empty());

    // Invalid string addition
    let stmts = vec![Stmt::Expression {
        expr: Expr::Binary {
            left: Box::new(string_lit("hello")),
            op: BinaryOp::Add,
            right: Box::new(string_lit("word")),
            span: dummy_span(),
        },
    }];

    let mut checker = TypeChecker::new();
    assert!(!checker.check(&stmts).is_empty());
}

#[test]
fn array_operations() {
    // Valid array literal
    let stmts = vec![Stmt::Expression {
        expr: Expr::ArrayLiteral {
            elements: vec![num_lit(1), num_lit(2), num_lit(3)],
            span: dummy_span(),
        },
    }];

    let mut checker = TypeChecker::new();
    assert!(checker.check(&stmts).is_empty());

    // Invalid: mixed element types
    let stmts = vec![Stmt::Expression {
        expr: Expr::ArrayLiteral {
            elements: vec![num_lit(1), bool_lit(true)],
            span: dummy_span(),
        },
    }];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);

    // Valid array access
    let stmts = vec![
        Stmt::VarDeclaration {
            variables: vec!["arr".to_string()],
            type_annotation: Type::Array(
                Box::new(Type::I32),
                Box::new(num_lit(3)),
            ),
            initializers: vec![],
            is_mutable: true,
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::ArrayAccess {
                array: Box::new(Expr::Variable {
                    name: "arr".to_string(),
                    span: dummy_span(),
                }),
                index: Box::new(num_lit(0)),
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    let errors = checker.check(&stmts);
    assert!(!errors.is_empty());
    

    // Invalid: non-array access
    let stmts = vec![
        Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I32,
            initializers: vec![num_lit(42)],
            is_mutable: true,
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::ArrayAccess {
                array: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    span: dummy_span(),
                }),
                index: Box::new(num_lit(0)),
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);
}

#[test]
fn edge_cases() {
    // Shadowing variables
    let stmts = vec![
        Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I32,
            initializers: vec![num_lit(42)],
            is_mutable: true,
            span: dummy_span(),
        },
        Stmt::Block {
            statements: vec![
                Stmt::VarDeclaration {
                    variables: vec!["x".to_string()],
                    type_annotation: Type::String,
                    initializers: vec![string_lit("shadowed")],
                    is_mutable: true,
                    span: dummy_span(),
                },
                Stmt::Expression {
                    expr: Expr::Assign {
                        target: Box::new(Expr::Variable {
                            name: "x".to_string(),
                            span: dummy_span(),
                        }),
                        value: Box::new(string_lit("new")),
                        span: dummy_span(),
                    },
                },
            ],
            span: dummy_span(),
        },
    ];

    let mut checker = TypeChecker::new();
    let errrors = checker.check(&stmts);
    assert!(!errrors.is_empty());

    // Break outside loop
    let stmts = vec![Stmt::Break {
        span: dummy_span(),
    }];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);

    // Empty array declaration without initializer
    let stmts = vec![Stmt::VarDeclaration {
        variables: vec!["arr".to_string()],
        type_annotation: Type::Array(
            Box::new(Type::I32),
            Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(3)),
                span: dummy_span(),
            }),
        ),
        initializers: vec![],
        is_mutable: true,
        span: dummy_span(),
    }];

    let mut checker = TypeChecker::new();
    assert_eq!(checker.check(&stmts).len(), 1);

    // Numeric type compatibility
    let stmts = vec![
        Stmt::VarDeclaration {
            variables: vec!["x".to_string()],
            type_annotation: Type::I64,
            initializers: vec![num_lit(42)],
            is_mutable: true,
            span: dummy_span(),
        },
        Stmt::Expression {
            expr: Expr::Assign {
                target: Box::new(Expr::Variable {
                    name: "x".to_string(),
                    span: dummy_span(),
                }),
                value: Box::new(num_lit(100)),
                span: dummy_span(),
            },
        },
    ];

    let mut checker = TypeChecker::new();
    assert!(checker.check(&stmts).is_empty());
}