use jsavrs::ir::generator::IrGenerator;
use jsavrs::ir::{
    ImmediateValue, Instruction, IrBinaryOp, IrType, IrUnaryOp, Terminator, Value, ValueKind,
};
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
            value: Some(num_lit_i32(42)),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
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
            value: Some(binary_expr(num_lit_i32(10), BinaryOp::Add, num_lit_i32(20))),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
            condition: bool_lit(true),
            then_branch: vec![Stmt::Return {
                value: None,
                span: dummy_span(),
            }],
            else_branch: None,
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
            value: Some(binary_expr(
                unary_expr(UnaryOp::Negate, num_lit_i32(5)),
                BinaryOp::Multiply,
                binary_expr(num_lit_i32(3), BinaryOp::Add, num_lit_i32(2)),
            )),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 3);

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
            value: Some(variable_expr("param")),
            span: dummy_span(),
        }],
    )];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
        vec![var_declaration(
            vec!["arr".to_string()],
            Type::Array(Box::new(Type::I32), Box::new(num_lit(10))),
            true,
            vec![],
        )],
    )];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
        body: vec![],
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
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
            value: Some(string_lit("hello")),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
            value: Some(nullptr_lit()),
            span: dummy_span(),
        }],
        span: dummy_span(),
    }];

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
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
                            target: Box::new(variable_expr("x")),
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];

    // Should have 1 block with 2 instructions (alloca + store)
    let entry_block = &func.basic_blocks[0];
    assert_eq!(entry_block.instructions.len(), 2);
    assert!(matches!(
        entry_block.instructions[0],
        Instruction::Alloca { .. }
    ));
    assert!(matches!(
        entry_block.instructions[1],
        Instruction::Store { .. }
    ));
}

#[test]
fn test_generate_simple_while_loop() {
    let ast = vec![function_declaration(
        "test".to_string(),
        vec![],
        Type::Void,
        vec![
            var_declaration(vec!["x".to_string()], Type::I32, true, vec![num_lit_i32(0)]),
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert!(ir_errors.is_empty(),);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.basic_blocks.len(), 4);

    // Entry block checks
    let entry_block = &func.basic_blocks[0];
    assert!(
        matches!(entry_block.instructions[0], Instruction::Alloca { .. }),
        "Expected Alloca instruction, got {:?}",
        entry_block.instructions[0]
    );
    assert!(
        matches!(entry_block.instructions[1], Instruction::Store { .. }),
        "Expected Store instruction, got {:?}",
        entry_block.instructions[1]
    );
    assert!(
        matches!(&entry_block.terminator, Terminator::Branch(_)),
        "Expected Branch terminator, got {:?}",
        entry_block.terminator
    );

    // Loop start block checks
    let loop_start = &func.basic_blocks[1];
    assert!(
        loop_start.instructions.len() >= 1,
        "Expected at least 1 instruction, got {}",
        loop_start.instructions.len()
    );
    assert!(
        matches!(
            &loop_start.instructions[0],
            Instruction::Binary {
                op: IrBinaryOp::Less,
                ..
            }
        ),
        "Expected Less comparison, got {:?}",
        loop_start.instructions[0]
    );
    assert!(
        matches!(&loop_start.terminator, Terminator::ConditionalBranch { .. }),
        "Expected ConditionalBranch terminator, got {:?}",
        loop_start.terminator
    );

    // Loop body block checks
    let loop_body = &func.basic_blocks[2];
    assert!(
        loop_body.instructions.len() >= 2,
        "Expected at least 3 instructions, got {}",
        loop_body.instructions.len()
    );
    assert!(
        matches!(loop_body.instructions[0], Instruction::Binary { .. }),
        "Expected Load instruction, got {:?}",
        loop_body.instructions[0]
    );
    assert!(
        matches!(loop_body.instructions[1], Instruction::Store { .. }),
        "Expected Add operation, got {:?}",
        loop_body.instructions[1]
    );
    assert!(
        matches!(&loop_body.terminator, Terminator::Branch(_)),
        "Expected Branch terminator, got {:?}",
        loop_body.terminator
    );

    // Loop end block checks
    let loop_end = &func.basic_blocks[3];
    assert!(
        matches!(&loop_end.terminator, Terminator::Return(..)),
        "Expected Return terminator, got {:?}",
        loop_end.terminator
    );
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];

    // Verify block structure: entry, for_start, for_body, for_inc, for_end
    assert_eq!(func.basic_blocks.len(), 5);

    // Check terminators in each block
    assert!(matches!(
        func.basic_blocks[0].terminator,
        Terminator::Branch(_) // to for_start
    ));
    assert!(matches!(
        func.basic_blocks[1].terminator,
        Terminator::ConditionalBranch { .. }
    ));
    assert!(matches!(
        func.basic_blocks[2].terminator,
        Terminator::Branch(_) // to for_inc
    ));
    assert!(matches!(
        func.basic_blocks[3].terminator,
        Terminator::Branch(_) // to for_start
    ));
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];

    // Body block (index 2) should break to for_end
    assert!(matches!(
        func.basic_blocks[2].terminator,
        Terminator::Branch(_) // to for_end
    ));
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];

    // Body block (index 2) should continue to for_inc
    assert!(matches!(
        func.basic_blocks[2].terminator,
        Terminator::Branch(_) // to for_inc
    ));
}

// Add to the existing test module
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    let block = &func.basic_blocks[0];
    assert_eq!(block.instructions.len(), 1);

    // Should have a single binary instruction with the grouped values
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    let block = &func.basic_blocks[0];

    assert_eq!(block.instructions.len(), 7);

    // First instruction should be alloca for the array
    if let Instruction::Alloca { ty, .. } = &block.instructions[0] {
        assert!(matches!(ty, IrType::Array(..)));
        if let IrType::Array(element_type, size) = ty {
            assert_eq!(**element_type, IrType::I32);
            assert_eq!(*size, 3);
        }
    } else {
        panic!("Expected alloca instruction for array");
    }
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

    let mut default_generator = IrGenerator::default();
    let (functions, errors) = default_generator.generate(ast);

    // Verify default instance works correctly
    assert_eq!(errors.len(), 0);
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "test");

    // Verify terminator in the generated function
    let block = &functions[0].basic_blocks[0];
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
fn test_generate_binary_all_operations() {
    let test_cases = vec![
        (BinaryOp::Add, IrBinaryOp::Add),
        (BinaryOp::Subtract, IrBinaryOp::Subtract),
        (BinaryOp::Multiply, IrBinaryOp::Multiply),
        (BinaryOp::Divide, IrBinaryOp::Divide),
        (BinaryOp::Modulo, IrBinaryOp::Modulo),
        (BinaryOp::Equal, IrBinaryOp::Equal),
        (BinaryOp::NotEqual, IrBinaryOp::NotEqual),
        (BinaryOp::Less, IrBinaryOp::Less),
        (BinaryOp::LessEqual, IrBinaryOp::LessEqual),
        (BinaryOp::Greater, IrBinaryOp::Greater),
        (BinaryOp::GreaterEqual, IrBinaryOp::GreaterEqual),
        (BinaryOp::And, IrBinaryOp::And),
        (BinaryOp::Or, IrBinaryOp::Or),
        (BinaryOp::BitwiseAnd, IrBinaryOp::BitwiseAnd),
        (BinaryOp::BitwiseOr, IrBinaryOp::BitwiseOr),
        (BinaryOp::BitwiseXor, IrBinaryOp::BitwiseXor),
        (BinaryOp::ShiftLeft, IrBinaryOp::ShiftLeft),
        (BinaryOp::ShiftRight, IrBinaryOp::ShiftRight),
    ];

    for (ast_op, expected_ir_op) in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            Type::I32,
            vec![Stmt::Return {
                value: Some(binary_expr(num_lit_i32(10), ast_op, num_lit_i32(20))),
                span: dummy_span(),
            }],
        )];

        let mut generator = IrGenerator::new();
        let (functions, ir_errors) = generator.generate(ast);
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.len(), 1);

        let func = &functions[0];
        let block = &func.basic_blocks[0];
        assert_eq!(block.instructions.len(), 1);

        match &block.instructions[0] {
            Instruction::Binary {
                op,
                left,
                right,
                ty,
                ..
            } => {
                assert_eq!(*op, expected_ir_op);
                assert_eq!(*ty, IrType::I32);
                assert_eq!(left.kind, ValueKind::Immediate(ImmediateValue::I32(10)));
                assert_eq!(right.kind, ValueKind::Immediate(ImmediateValue::I32(20)));
            }
            _ => panic!("Expected binary instruction"),
        }
    }
}

#[test]
fn test_generate_unary_expression() {
    let test_cases = vec![
        (UnaryOp::Negate, IrUnaryOp::Negate),
        (UnaryOp::Not, IrUnaryOp::Not),
    ];

    for (ast_op, expected_ir_op) in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            Type::I32,
            vec![Stmt::Return {
                value: Some(unary_expr(ast_op, num_lit_i32(42))),
                span: dummy_span(),
            }],
        )];

        let mut generator = IrGenerator::new();
        let (functions, ir_errors) = generator.generate(ast);
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.len(), 1);

        let func = &functions[0];
        let block = &func.basic_blocks[0];
        assert_eq!(block.instructions.len(), 1);

        match &block.instructions[0] {
            Instruction::Unary {
                op, operand, ty, ..
            } => {
                assert_eq!(*op, expected_ir_op);
                assert_eq!(*ty, IrType::I32);
                assert_eq!(operand.kind, ValueKind::Immediate(ImmediateValue::I32(42)));
            }
            _ => panic!("Expected unary instruction"),
        }
    }
}

#[test]
fn test_generate_integer_literals() {
    let test_cases = vec![
        (Number::I8(42), ImmediateValue::I8(42), IrType::I8),
        (Number::I16(1000), ImmediateValue::I16(1000), IrType::I16),
        (Number::I32(32000), ImmediateValue::I32(32000), IrType::I32),
        (
            Number::Integer(2_000_000_000),
            ImmediateValue::I64(2_000_000_000),
            IrType::I64,
        ),
        (Number::U8(255), ImmediateValue::U8(255), IrType::U8),
        (Number::U16(65535), ImmediateValue::U16(65535), IrType::U16),
        (
            Number::U32(4_000_000_000),
            ImmediateValue::U32(4_000_000_000),
            IrType::U32,
        ),
        (
            Number::UnsignedInteger(18_000_000_000_000_000_000),
            ImmediateValue::U64(18_000_000_000_000_000_000),
            IrType::U64,
        ),
    ];

    for (num, expected_value, expected_type) in test_cases {
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

        let mut generator = IrGenerator::new();
        let (functions, ir_errors) = generator.generate(ast);
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.len(), 1);

        let func = &functions[0];
        let block = &func.basic_blocks[0];

        match &block.terminator {
            Terminator::Return(value, ty) => {
                assert_eq!(*ty, expected_type);
                match &value.kind {
                    ValueKind::Immediate(imm) => assert_eq!(imm, &expected_value),
                    _ => panic!("Expected immediate value"),
                }
            }
            _ => panic!("Expected return terminator"),
        }
    }
}

#[test]
fn test_generate_float_literals() {
    let test_cases = vec![
        (
            Number::Float32(3.14),
            ImmediateValue::F32(3.14),
            IrType::F32,
        ),
        (
            Number::Float64(123.456),
            ImmediateValue::F64(123.456),
            IrType::F64,
        ),
        (
            Number::Scientific32(2.0, 2),
            ImmediateValue::F32(4.0),
            IrType::F32,
        ),
        (
            Number::Scientific64(10.0, 3),
            ImmediateValue::F64(1000.0),
            IrType::F64,
        ),
    ];

    for (num, expected_value, expected_type) in test_cases {
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

        let mut generator = IrGenerator::new();
        let (functions, ir_errors) = generator.generate(ast);
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.len(), 1);

        let func = &functions[0];
        let block = &func.basic_blocks[0];

        match &block.terminator {
            Terminator::Return(value, ty) => {
                assert_eq!(*ty, expected_type);
                match &value.kind {
                    ValueKind::Immediate(imm) => assert_eq!(imm, &expected_value),
                    _ => panic!("Expected immediate value"),
                }
            }
            _ => panic!("Expected return terminator"),
        }
    }
}

#[test]
fn test_generate_boolean_literals() {
    let test_cases = vec![
        (true, ImmediateValue::Bool(true)),
        (false, ImmediateValue::Bool(false)),
    ];

    for (b, expected_value) in test_cases {
        let ast = vec![function_declaration(
            "test".to_string(),
            vec![],
            Type::Bool,
            vec![Stmt::Return {
                value: Some(bool_lit(b)),
                span: dummy_span(),
            }],
        )];

        let mut generator = IrGenerator::new();
        let (functions, ir_errors) = generator.generate(ast);
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.len(), 1);

        let func = &functions[0];
        let block = &func.basic_blocks[0];

        match &block.terminator {
            Terminator::Return(value, ty) => {
                assert_eq!(*ty, IrType::Bool);
                match &value.kind {
                    ValueKind::Immediate(imm) => assert_eq!(imm, &expected_value),
                    _ => panic!("Expected immediate value"),
                }
            }
            _ => panic!("Expected return terminator"),
        }
    }
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);

    let func = &functions[0];
    let block = &func.basic_blocks[0];

    match &block.terminator {
        Terminator::Return(value, ty) => {
            assert_eq!(*ty, IrType::Char);
            match &value.kind {
                ValueKind::Immediate(ImmediateValue::Char(c)) => assert_eq!(*c, 'A'),
                _ => panic!("Expected char immediate value"),
            }
        }
        _ => panic!("Expected return terminator"),
    }
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

    let mut generator = IrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);

    let func = &functions[0];
    let block = &func.basic_blocks[0];

    match &block.terminator {
        Terminator::Return(value, ty) => {
            assert!(matches!(ty, IrType::Pointer(_)));
            match &value.kind {
                ValueKind::Immediate(ImmediateValue::I64(0)) => (), // Expected
                _ => panic!("Expected i64(0) for nullptr"),
            }
        }
        _ => panic!("Expected return terminator"),
    }
}
