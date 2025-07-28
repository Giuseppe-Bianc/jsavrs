use jsavrs::nir::generator::NIrGenerator;
use jsavrs::nir::{
    InstructionKind, IrBinaryOp, IrConstantValue, IrLiteralValue, IrType, IrUnaryOp,
    TerminatorKind, ValueKind,
};
use jsavrs::parser::ast::{BinaryOp, Expr, Parameter, Stmt, Type, UnaryOp};
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::I32);
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.clone().instructions.len(), 0);
    // VERIFICA TERMINATOR
    match &entry_block.terminator.kind {
        TerminatorKind::Return { value, .. } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(42)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "void_func");
    assert_eq!(func.return_type, IrType::Void);
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_void_func");
    let entry_block = func.cfg.get_block("entry_void_func").unwrap();
    assert_eq!(entry_block.clone().instructions.len(), 0);
    // VERIFICA TERMINATOR
    match &entry_block.terminator.kind {
        TerminatorKind::Return { value, .. } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "main");
    assert_eq!(func.return_type, IrType::Void);
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_main");
    let entry_block = func.cfg.get_block("entry_main").unwrap();
    assert_eq!(entry_block.clone().instructions.len(), 0);
    // VERIFICA TERMINATOR
    match &entry_block.terminator.kind {
        TerminatorKind::Return { value, .. } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);

    // Verifica assenza errori
    assert_eq!(ir_errors.len(), 0);

    // Verifica struttura funzione
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::I32);

    // Verifica struttura CFG
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");

    // Verifica contenuto blocco entry
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 1);

    // VERIFICA ISTRUZIONE BINARY
    let instruction = &entry_block.instructions[0];
    match &instruction.kind {
        InstructionKind::Binary { op, left, right, ty } => {
            // Verifica operatore
            assert_eq!(*op, IrBinaryOp::Add);

            // Verifica tipo
            assert_eq!(*ty, IrType::I32);

            // Verifica operandi
            assert_eq!(left.kind, ValueKind::Literal(IrLiteralValue::I32(10)));
            assert_eq!(right.kind, ValueKind::Literal(IrLiteralValue::I32(20)));
        }
        other => panic!("Expected binary instruction, got {:?}", other),
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

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);

    // Verifica assenza errori
    assert_eq!(ir_errors.len(), 0);

    // Verifica struttura funzione
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::Void);

    // Verifica struttura CFG
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");

    // Verifica contenuto blocco entry
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 2);
    // VERIFICA ISTRUZIONE ALLOCA
    let alloca_instr = &entry_block.instructions[0];
    match &alloca_instr.kind {
        InstructionKind::Alloca { ty } => {
            assert_eq!(*ty, IrType::I32);
        }
        other => panic!("Expected alloca instruction, got {:?}", other),
    }
    // VERIFICA ISTRUZIONE STORE
    let store_instr = &entry_block.instructions[1];
    match &store_instr.kind {
        InstructionKind::Store { value, dest } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(10)));
            assert_eq!(dest.kind, ValueKind::Temporary(0));
        }
        other => panic!("Expected store instruction, got {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 4);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    match &entry_block.terminator.kind {
        TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
            assert_eq!(
                condition.kind,
                ValueKind::Literal(IrLiteralValue::Bool(true))
            );
            assert_eq!(true_label, "then_1");
            assert_eq!(false_label, "else_2");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }

    let then_block = func.cfg.blocks.get("then_1").unwrap();
    assert_eq!(then_block.instructions.len(), 0);
    match &then_block.terminator.kind {
        TerminatorKind::Return { value, .. } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
    let else_block = func.cfg.blocks.get("else_2").unwrap();
    assert_eq!(else_block.instructions.len(), 0);
    match &else_block.terminator.kind {
        TerminatorKind::Branch { label } => {
            // Successo: blocco else ha un branch verso il merge
            assert_eq!(label, "merge_3");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
    let merge_block = func.cfg.blocks.get("merge_3").unwrap();
    assert_eq!(merge_block.instructions.len(), 0);
    match &merge_block.terminator.kind {
        TerminatorKind::Return { value, .. } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 3);

    let first_instruction = &entry_block.instructions[0];
    match first_instruction.clone().kind {
        InstructionKind::Unary { op, operand, ty } => {
            assert_eq!(op, IrUnaryOp::Negate);
            assert_eq!(ty, IrType::I32);
            assert_eq!(operand.kind, ValueKind::Literal(IrLiteralValue::I32(5)));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }
    let second_instruction = &entry_block.instructions[1];
    match second_instruction.clone().kind {
        InstructionKind::Binary {
            op,
            left,
            right,
            ty,
        } => {
            assert_eq!(op, IrBinaryOp::Add);
            assert_eq!(ty, IrType::I32);
            assert_eq!(left.kind, ValueKind::Literal(IrLiteralValue::I32(3)));
            assert_eq!(right.kind, ValueKind::Literal(IrLiteralValue::I32(2)));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }

    let third_instruction = &entry_block.instructions[2];
    match third_instruction.clone().kind {
        InstructionKind::Binary {
            op,
            left,
            right,
            ty,
        } => {
            assert_eq!(op, IrBinaryOp::Multiply);
            assert_eq!(ty, IrType::I32);
            assert_eq!(left.kind, ValueKind::Temporary(0));
            assert_eq!(right.kind, ValueKind::Temporary(1));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(
        func.parameters[0].ty,
        IrType::Custom("MyType".to_string(), dummy_span())
    );
    assert_eq!(
        func.return_type,
        IrType::Custom("MyType".to_string(), dummy_span())
    );
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 1);
    // VERIFICA ISTRUZIONE ALLOCA
    let instruction = &entry_block.instructions[0];
    match &instruction.kind {
        InstructionKind::Alloca { ty } => {
            assert_eq!(*ty, IrType::Array(Box::new(IrType::I32), 10));
        }
        other => panic!("Expected alloca instruction, got {:?}", other),
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

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    match &entry_block.terminator.kind {
        TerminatorKind::Return { value, ty } => {
            assert_eq!(*ty, IrType::I32);
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    match &entry_block.terminator.kind {
        TerminatorKind::Return { value, ty } => {
            assert_eq!(*ty, IrType::String);
            assert_eq!(
                value.kind,
                ValueKind::Constant(IrConstantValue::String {
                    string: "hello".to_string()
                })
            );
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    match &entry_block.terminator.kind {
        TerminatorKind::Return { value, ty } => {
            assert_eq!(*ty, IrType::Pointer(Box::new(IrType::I8)));
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I64(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 3);
    // Verifica istruzioni all'interno del blocco
    match &entry_block.instructions[0].kind {
        InstructionKind::Alloca { ty } => {
            assert_eq!(*ty, IrType::I32);
        }
        other => panic!("Expected alloca instruction, got {:?}", other),
    }

    match &entry_block.instructions[1].kind {
        InstructionKind::Store { value, dest } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(5)));
            assert_eq!(dest.kind, ValueKind::Temporary(0));
        }
        other => panic!("Expected store instruction, got {:?}", other),
    }


    match &entry_block.instructions[2].kind {
        InstructionKind::Store { value, dest } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(10)));
            assert_eq!(dest.kind, ValueKind::Temporary(0));
        }
        other => panic!("Expected store instruction, got {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.cfg.blocks.len(), 4);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 2);
    match &entry_block.instructions[0].kind {
        InstructionKind::Alloca { ty } => {
            assert_eq!(*ty, IrType::I32);
        }
        other => panic!("Expected alloca instruction, got {:?}", other),
    }
    match &entry_block.instructions[1].kind {
        InstructionKind::Store { value, dest } => {
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
            assert_eq!(dest.kind, ValueKind::Temporary(0));
        }
        other => panic!("Expected store instruction, got {:?}", other),
    }

    match &entry_block.terminator.kind {
        TerminatorKind::Branch { label } => {
            assert_eq!(label, "loop_start_1");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }

    let loop_start = func.cfg.get_block("loop_start_1").unwrap();
    assert_eq!(loop_start.instructions.len(), 1);
    match loop_start.instructions[0].clone().kind {
        InstructionKind::Binary {
            op,
            left,
            right,
            ty,
        } => {
            assert_eq!(op, IrBinaryOp::Less);
            assert_eq!(ty, IrType::Pointer(Box::new(IrType::I32)));
            assert_eq!(left.kind, ValueKind::Temporary(0));
            assert_eq!(right.kind, ValueKind::Literal(IrLiteralValue::I32(5)));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }

    match &loop_start.terminator.kind {
        TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
            assert_eq!(
                condition.kind,
                ValueKind::Temporary(1)
            );
            assert_eq!(true_label, "loop_body_2");
            assert_eq!(false_label, "loop_end_3");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }

    let loop_body = func.cfg.get_block("loop_body_2").unwrap();
    assert_eq!(loop_body.instructions.len(), 2);

    match loop_body.instructions[0].clone().kind {
        InstructionKind::Binary {
            op,
            left,
            right,
            ty,
        } => {
            assert_eq!(op, IrBinaryOp::Add);
            assert_eq!(ty, IrType::Pointer(Box::new(IrType::I32)));
            assert_eq!(left.kind, ValueKind::Temporary(0));
            assert_eq!(right.kind, ValueKind::Literal(IrLiteralValue::I32(1)));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }

    match loop_body.instructions[1].clone().kind {
        InstructionKind::Store { value, dest } => {
            assert_eq!(value.kind, ValueKind::Temporary(2));
            assert_eq!(dest.kind, ValueKind::Temporary(0));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }

    match &loop_body.terminator.kind {
        TerminatorKind::Branch { label } => {
            assert_eq!(label, "loop_start_1");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }

    let loop_end = func.cfg.get_block("loop_end_3").unwrap();
    assert_eq!(loop_end.instructions.len(), 0);
    match &loop_end.terminator.kind {
        TerminatorKind::Return { value, ty } => {
            assert_eq!(*ty, IrType::Void);
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
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
    let (functions, ir_errors) = generator.generate(ast);
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    // Verify block structure: entry, for_start, for_body, for_inc, for_end
    assert_eq!(func.cfg.blocks.len(), 5);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();

    match &entry_block.terminator.kind {
        TerminatorKind::Branch { label } => {
            assert_eq!(label, "for_start_1");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
    let for_start = func.cfg.get_block("for_start_1").unwrap();
    assert_eq!(for_start.instructions.len(), 1);
    match &for_start.terminator.kind {
        TerminatorKind::ConditionalBranch { condition, true_label, false_label } =>{
            assert_eq!(
                condition.kind,
                ValueKind::Temporary(1) // Initial value of i
            );
            assert_eq!(true_label, "for_body_2");
            assert_eq!(false_label, "for_end_4");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }

    let for_body = func.cfg.get_block("for_body_2").unwrap();
    assert_eq!(for_body.instructions.len(), 0);
    match &for_body.terminator.kind {
        TerminatorKind::Branch { label } => {
            assert_eq!(label, "for_inc_3");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
    let for_inc = func.cfg.get_block("for_inc_3").unwrap();
    assert_eq!(for_inc.instructions.len(), 2);
    match &for_inc.terminator.kind {
        TerminatorKind::Branch { label } => {
            assert_eq!(label, "for_start_1");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
    let for_end = func.cfg.get_block("for_end_4").unwrap();
    assert_eq!(for_end.instructions.len(), 0);
    match &for_end.terminator.kind {
        TerminatorKind::Return { value, ty } => {
            assert_eq!(*ty, IrType::Void);
            assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I32(0)));
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
}