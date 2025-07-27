use jsavrs::nir::generator::NIrGenerator;
use jsavrs::nir::{InstructionKind, IrBinaryOp, IrLiteralValue, IrType, TerminatorKind, Value, ValueKind};
use jsavrs::parser::ast::{BinaryOp, Stmt, Type};
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
        TerminatorKind::Return{value, .. } => {
            if let Value {
                kind: ValueKind::Literal(IrLiteralValue::I32(42)),
                ..
            } = *value
            {
                // Successo: valore di ritorno corretto
            } else {
                panic!("Return value is not 42");
            }
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
        TerminatorKind::Return{value, .. } => {
            if let Value {
                kind: ValueKind::Literal(IrLiteralValue::I32(0)),
                ..
            } = *value
            {
                // Successo: valore di ritorno corretto
            } else {
                panic!("Return value is not 0 actual: {:?}", value);
            }
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
        TerminatorKind::Return{value, .. } => {
                if let Value {
                    kind: ValueKind::Literal(IrLiteralValue::I32(0)),
                    ..
                } = *value
                {
                    // Successo: valore di ritorno corretto
                } else {
                    panic!("Return value is not 0 actual: {:?}", value);
                }
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
        InstructionKind::Binary { op, left, right, ty} => {
            // Verifica operatore
            assert_eq!(*op, IrBinaryOp::Add);

            // Verifica tipo
            assert_eq!(*ty, IrType::I32);

            // Verifica operandi
            if let ValueKind::Literal(IrLiteralValue::I32(10)) = &left.kind {
                // Operando sinistro corretto
            } else {
                panic!("Left operand is not i32(10)");
            }

            if let ValueKind::Literal(IrLiteralValue::I32(20)) = &right.kind {
                // Operando destro corretto
            } else {
                panic!("Right operand is not i32(20)");
            }
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
    /*assert!(matches!(block.instructions[0], Instruction::Alloca { .. }));
    assert!(matches!(block.instructions[1], Instruction::Store { .. }));*/
    // VERIFICA ISTRUZIONE ALLOCA
    let alloca_instr = &entry_block.instructions[0];
    match &alloca_instr.kind {
        InstructionKind::Alloca { ty }  => {
            assert_eq!(*ty, IrType::I32);
        }
        other => panic!("Expected alloca instruction, got {:?}", other),
    }
    // VERIFICA ISTRUZIONE STORE
    let store_instr = &entry_block.instructions[1];
    match &store_instr.kind {
        InstructionKind::Store{ value, dest } => {
            if let Value {
                kind: ValueKind::Literal(IrLiteralValue::I32(10)),
                ..
            } = *value
            {
                // Successo: valore di ritorno corretto
            } else {
                panic!("Return value is not 10 actual: {:?}", value);
            }

            if let Value {
                kind: ValueKind::Temporary(0),
                ..
            } = *dest
            {
                // Successo: valore di ritorno corretto
            } else {
                panic!("Return value is not 10 actual: {:?}", dest);
            }
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
            if let Value {
                kind: ValueKind::Literal(IrLiteralValue::Bool(true)),
                ..
            } = *condition
            {
                // Successo: condizione corretta
            } else {
                panic!("Condition is not true actual: {:?}", condition);
            }
            assert_eq!(true_label, "then_1");
            assert_eq!(false_label, "else_2");
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }

    let then_block= func.cfg.blocks.get("then_1").unwrap();
    assert_eq!(then_block.instructions.len(), 0);
    match &then_block.terminator.kind {
        TerminatorKind::Return { value, .. } => {
            if let Value {
                kind: ValueKind::Literal(IrLiteralValue::I32(0)),
                ..
            } = *value
            {
                // Successo: valore di ritorno corretto
            } else {
                panic!("Return value is not 0 actual: {:?}", value);
            }
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
    let else_block= func.cfg.blocks.get("else_2").unwrap();
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
            if let Value {
                kind: ValueKind::Literal(IrLiteralValue::I32(0)),
                ..
            } = *value
            {
                // Successo: valore di ritorno corretto
            } else {
                panic!("Return value is not 0 actual: {:?}", value);
            }
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
}