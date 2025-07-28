use jsavrs::nir::generator::NIrGenerator;
use jsavrs::nir::{
    InstructionKind, IrBinaryOp, IrConstantValue, IrLiteralValue, IrType, IrUnaryOp,
    TerminatorKind, ValueKind,
};
use jsavrs::parser::ast::{BinaryOp, Parameter, Stmt, Type, UnaryOp};
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
