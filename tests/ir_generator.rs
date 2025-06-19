use jsavrs::ir::generator::IrGenerator;
use jsavrs::ir::IrType;
use jsavrs::ir::{ImmediateValue, Value, ValueKind};
use jsavrs::ir::{Instruction, IrBinaryOp, IrUnaryOp, Terminator};
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

    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::I32);
    assert_eq!(func.basic_blocks.len(), 1);

    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 0);
    assert!(matches!(
        &block.terminator,
        Terminator::Return(
            Value {
                kind: ValueKind::Immediate(ImmediateValue::I32(42)),
                ..
            },
            IrType::I32
        )
    ));
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert!(matches!(
        &block.terminator,
        Terminator::Return(
            Value {
                kind: ValueKind::Immediate(ImmediateValue::I32(0)),
                ..
            },
            IrType::Void
        )
    ));
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

    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "main");
    assert_eq!(func.return_type, IrType::Void);
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 1);

    if let Instruction::Binary {
        op,
        left,
        right,
        ty,
        ..
    } = &block.instructions[0]
    {
        assert_eq!(*op, IrBinaryOp::Add);
        assert_eq!(*ty, IrType::I32);
        assert!(matches!(
            &left.kind,
            ValueKind::Immediate(ImmediateValue::I32(10))
        ));
        assert!(matches!(
            &right.kind,
            ValueKind::Immediate(ImmediateValue::I32(20))
        ));
    } else {
        panic!("Expected binary instruction");
    }
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 1);

    if let Instruction::Unary {
        op, operand, ty, ..
    } = &block.instructions[0]
    {
        assert_eq!(*op, IrUnaryOp::Negate);
        assert_eq!(*ty, IrType::I32);
        assert!(matches!(
            &operand.kind,
            ValueKind::Immediate(ImmediateValue::I32(42))
        ));
    } else {
        panic!("Expected unary instruction");
    }
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

    let func = &functions[0];
    assert_eq!(func.basic_blocks.len(), 1);
    let block = &func.basic_blocks[0];

    // Should have: alloca, store (assignment)
    assert_eq!(block.instructions.len(), 2);
    assert!(matches!(block.instructions[0], Instruction::Alloca { .. }));
    assert!(matches!(block.instructions[1], Instruction::Store { .. }));
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

    let func = &functions[0];
    assert_eq!(func.basic_blocks.len(), 4); // entry, then, else, merge

    // Entry block should have conditional branch
    let entry_block = &func.basic_blocks[0];
    assert!(matches!(
        &entry_block.terminator,
        Terminator::ConditionalBranch { .. }
    ));

    // Then block should have return
    let then_block = &func.basic_blocks[1];
    assert!(matches!(&then_block.terminator, Terminator::Return(..)));

    // Else block should have branch to merge
    let else_block = &func.basic_blocks[2];
    assert!(matches!(&else_block.terminator, Terminator::Branch(_)));

    // Merge block should be unreachable or have content
    let merge_block = &func.basic_blocks[3];
    assert!(matches!(&merge_block.terminator, Terminator::Return(..)));
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 3); // unary, binary, binary

    // First instruction should be unary
    assert!(matches!(block.instructions[0], Instruction::Unary { .. }));

    // Second instruction should be binary (addition)
    assert!(matches!(
        block.instructions[1],
        Instruction::Binary {
            op: IrBinaryOp::Add,
            ..
        }
    ));

    // Third instruction should be binary (multiplication)
    assert!(matches!(
        block.instructions[2],
        Instruction::Binary {
            op: IrBinaryOp::Multiply,
            ..
        }
    ));
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

    let func = &functions[0];
    assert_eq!(func.parameters[0].1, IrType::Custom("MyType".to_string()));
    assert_eq!(func.return_type, IrType::Custom("MyType".to_string()));
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 1);

    if let Instruction::Alloca { ty, .. } = &block.instructions[0] {
        assert!(matches!(ty, IrType::Array(..)));
        if let IrType::Array(element_type, size) = ty {
            assert_eq!(**element_type, IrType::I32);
            assert_eq!(*size, 10);
        }
    } else {
        panic!("Expected alloca instruction for array");
    }
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert!(matches!(
        &block.terminator,
        Terminator::Return(
            Value {
                kind: ValueKind::Immediate(ImmediateValue::I32(0)),
                ..
            },
            IrType::I32
        )
    ));
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

    assert_eq!(functions.len(), 2);
    assert_eq!(functions[0].name, "func1");
    assert_eq!(functions[1].name, "func2");
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert!(matches!(
        &block.terminator,
        Terminator::Return(
            Value { kind: ValueKind::Immediate(ImmediateValue::String(s)), .. },
            IrType::String
        ) if s == "hello"
    ));
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

    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert!(matches!(
        &block.terminator,
        Terminator::Return(
            Value {
                kind: ValueKind::Immediate(ImmediateValue::I64(0)),
                ..
            },
            IrType::Pointer(..)
        )
    ));
}
