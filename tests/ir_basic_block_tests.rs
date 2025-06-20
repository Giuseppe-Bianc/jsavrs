use jsavrs::ir::{
    BasicBlock, ImmediateValue, Instruction, IrBinaryOp, IrType, IrUnaryOp, Terminator, Value,
    ValueKind,
};

// Helper functions
fn i32_val(n: i32) -> Value {
    Value::new_immediate(ImmediateValue::I32(n))
}

fn create_instruction() -> Instruction {
    Instruction::Binary {
        op: IrBinaryOp::Add,
        dest: "sum".to_string(),
        left: i32_val(10),
        right: i32_val(20),
        ty: IrType::I32,
    }
}

fn create_terminator() -> Terminator {
    Terminator::Return(i32_val(42), IrType::I32)
}

#[test]
fn test_new_block() {
    let block = BasicBlock::new("entry");
    assert_eq!(block.label, "entry");
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator, Terminator::Unreachable);
}

#[test]
fn test_block_display_empty() {
    let block = BasicBlock {
        label: "start".to_string(),
        instructions: Vec::new(),
        terminator: Terminator::Unreachable,
    };
    assert_eq!(block.to_string(), "start:\n  unreachable");
}

#[test]
fn test_block_display_single_instruction() {
    let mut block = BasicBlock::new("block1");
    block.instructions.push(Instruction::Unary {
        op: IrUnaryOp::Negate,
        dest: "neg".to_string(),
        operand: i32_val(100),
        ty: IrType::I32,
    });
    block.terminator = Terminator::Branch("next".to_string());

    let expected = "block1:\n  neg = neg 100i32 i32\n  br next";
    assert_eq!(block.to_string(), expected);
}

#[test]
fn test_block_display_multiple_instructions() {
    let mut block = BasicBlock::new("compute");

    // Add several instructions
    block.instructions.push(Instruction::Alloca {
        dest: "var".to_string(),
        ty: IrType::I32,
    });
    block.instructions.push(Instruction::Store {
        value: i32_val(5),
        dest: Value {
            kind: ValueKind::Local("var".to_string()),
            ty: IrType::I32,
        },
    });
    block.instructions.push(Instruction::Load {
        dest: "val".to_string(),
        src: Value {
            kind: ValueKind::Local("var".to_string()),
            ty: IrType::I32,
        },
        ty: IrType::I32,
    });

    block.terminator = create_terminator();

    let expected = r#"compute:
  var = alloca i32
  store 5i32 to %var
  val = load i32 from %var
  ret 42i32 i32"#;
    assert_eq!(block.to_string(), expected);
}

#[test]
fn test_block_display_edge_cases() {
    // Empty label
    let mut block = BasicBlock::new("");
    block.instructions.push(create_instruction());
    block.terminator = Terminator::Unreachable;

    assert_eq!(
        block.to_string(),
        ":\n  sum = add 10i32 20i32, i32\n  unreachable"
    );

    // Special characters in label
    let mut block = BasicBlock::new("block$@1");
    block.terminator = Terminator::Branch("next#block".to_string());

    assert_eq!(block.to_string(), "block$@1:\n  br next#block");

    // Long instruction list
    let mut block = BasicBlock::new("long_block");
    for _ in 0..100 {
        block.instructions.push(create_instruction());
    }
    block.terminator = Terminator::Unreachable;

    let output = block.to_string();
    assert!(output.starts_with("long_block:"));
    assert!(output.ends_with("unreachable"));
    assert_eq!(output.matches("sum = add").count(), 100);

    // Multi-line instructions
    let mut block = BasicBlock::new("multi_line");
    block.instructions.push(Instruction::Call {
        dest: Some("result".to_string()),
        func: "func".to_string(),
        args: vec![
            Value::new_immediate(ImmediateValue::String("line1\nline2".to_string())),
            i32_val(42),
        ],
        ty: IrType::Void,
    });
    block.terminator = Terminator::Unreachable;

    assert_eq!(
        block.to_string(),
        r#"multi_line:
  result = func("line1\nline2", 42i32) : void
  unreachable"#
    );
}

#[test]
fn test_mixed_instructions_and_terminators() {
    let mut block = BasicBlock::new("mixed");

    // Add different instruction types
    block.instructions.push(Instruction::Cast {
        dest: "casted".to_string(),
        value: i32_val(100),
        from_ty: IrType::I32,
        to_ty: IrType::F64,
    });
    block.instructions.push(Instruction::Phi {
        dest: "phi_val".to_string(),
        ty: IrType::I32,
        incoming: vec![
            (i32_val(1), "block1".to_string()),
            (i32_val(2), "block2".to_string()),
        ],
    });

    block.terminator = Terminator::ConditionalBranch {
        condition: Value::new_immediate(ImmediateValue::Bool(true)),
        true_label: "true_block".to_string(),
        false_label: "false_block".to_string(),
    };

    let expected = r#"mixed:
  casted = cast 100i32 from i32 to f64
  phi_val = phi i32 [ [ 1i32, block1 ], [ 2i32, block2 ] ]
  br true ? true_block : false_block"#;
    assert_eq!(block.to_string(), expected);
}

#[test]
fn test_large_block() {
    let mut block = BasicBlock::new("large_block");
    let mut expected = String::from("large_block:\n");

    // Add 1000 instructions
    for i in 0..1000 {
        let inst = Instruction::Store {
            value: i32_val(i),
            dest: Value {
                kind: ValueKind::Temporary(i.to_string()),
                ty: IrType::I32,
            },
        };
        block.instructions.push(inst.clone());
        expected.push_str(&format!("  {}\n", inst));
    }

    block.terminator = Terminator::Return(i32_val(-1), IrType::I32);
    expected.push_str("  ret -1i32 i32");

    assert_eq!(block.to_string(), expected);
}
