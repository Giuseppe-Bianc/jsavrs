use jsavrs::ir::{Instruction, IrBinaryOp, IrType, IrUnaryOp};
use jsavrs::ir::{ImmediateValue, Value, ValueKind};

// Helper functions to create values
fn i32_val(n: i32) -> Value {
    Value::new_immediate(ImmediateValue::I32(n))
}

fn local_val(name: &str) -> Value {
    Value {
        kind: ValueKind::Local(name.to_string()),
        ty: IrType::I32,
    }
}

fn temp_val(id: &str) -> Value {
    Value {
        kind: ValueKind::Temporary(id.to_string()),
        ty: IrType::I32,
    }
}

fn string_val(s: &str) -> Value {
    Value::new_immediate(ImmediateValue::String(s.to_string()))
}

#[test]
fn test_alloca_instruction() {
    let instr = Instruction::Alloca {
        dest: "var".to_string(),
        ty: IrType::I32,
    };
    assert_eq!(instr.to_string(), "var = alloca i32");
}

#[test]
fn test_store_instruction() {
    let instr = Instruction::Store {
        value: i32_val(42),
        dest: local_val("ptr"),
    };
    assert_eq!(instr.to_string(), "store 42i32 to %ptr");

    let instr = Instruction::Store {
        value: string_val("hello"),
        dest: Value {
            kind: ValueKind::Global("global".to_string()),
            ty: IrType::String,
        },
    };
    assert_eq!(instr.to_string(), r#"store "hello" to @global"#);
}

#[test]
fn test_load_instruction() {
    let instr = Instruction::Load {
        dest: "result".to_string(),
        src: local_val("source"),
        ty: IrType::F64,
    };
    assert_eq!(instr.to_string(), "result = load f64 from %source");

    let instr = Instruction::Load {
        dest: "t1".to_string(),
        src: Value {
            kind: ValueKind::Global("const".to_string()),
            ty: IrType::I8,
        },
        ty: IrType::I8,
    };
    assert_eq!(instr.to_string(), "t1 = load i8 from @const");
}

#[test]
fn test_binary_instructions() {
    // Test all binary operations with various value combinations
    let ops = vec![
        (IrBinaryOp::Add, "add"),
        (IrBinaryOp::Subtract, "sub"),
        (IrBinaryOp::Multiply, "mul"),
        (IrBinaryOp::Divide, "div"),
        (IrBinaryOp::Modulo, "mod"),
        (IrBinaryOp::Equal, "eq"),
        (IrBinaryOp::NotEqual, "ne"),
        (IrBinaryOp::Less, "lt"),
        (IrBinaryOp::LessEqual, "le"),
        (IrBinaryOp::Greater, "gt"),
        (IrBinaryOp::GreaterEqual, "ge"),
        (IrBinaryOp::And, "and"),
        (IrBinaryOp::Or, "or"),
        (IrBinaryOp::BitwiseAnd, "bitand"),
        (IrBinaryOp::BitwiseOr, "bitor"),
        (IrBinaryOp::BitwiseXor, "bitxor"),
        (IrBinaryOp::ShiftLeft, "shl"),
        (IrBinaryOp::ShiftRight, "shr"),
    ];

    for (op, op_str) in ops {
        let instr = Instruction::Binary {
            op: op.clone(),
            dest: "res".to_string(),
            left: i32_val(10),
            right: temp_val("2"),
            ty: IrType::I32,
        };
        assert_eq!(
            instr.to_string(),
            format!("res = {op_str} 10i32 t2, i32")
        );

        // Test with different types
        let instr = Instruction::Binary {
            op,
            dest: "f_res".to_string(),
            left: Value::new_immediate(ImmediateValue::F32(1.5)),
            right: Value::new_immediate(ImmediateValue::F32(2.5)),
            ty: IrType::F32,
        };
        assert_eq!(
            instr.to_string(),
            format!("f_res = {op_str} 1.5f32 2.5f32, f32")
        );
    }
}

#[test]
fn test_unary_instructions() {
    let ops = vec![
        (IrUnaryOp::Negate, "neg"),
        (IrUnaryOp::Not, "not"),
    ];

    for (op, op_str) in ops {
        let instr = Instruction::Unary {
            op: op.clone(),
            dest: "res".to_string(),
            operand: i32_val(100),
            ty: IrType::I32,
        };
        assert_eq!(instr.to_string(), format!("res = {op_str} 100i32 i32"));

        let instr = Instruction::Unary {
            op,
            dest: "b_res".to_string(),
            operand: Value::new_immediate(ImmediateValue::Bool(true)),
            ty: IrType::Bool,
        };
        assert_eq!(instr.to_string(), format!("b_res = {op_str} true bool"));
    }
}

#[test]
fn test_call_instruction() {
    // With return value
    let instr = Instruction::Call {
        dest: Some("result".to_string()),
        func: "calculate".to_string(),
        args: vec![i32_val(1), local_val("x"), temp_val("3")],
        ty: IrType::I32,
    };
    assert_eq!(
        instr.to_string(),
        "result = calculate(1i32, %x, t3) : i32"
    );

    // Without return value
    let instr = Instruction::Call {
        dest: None,
        func: "print".to_string(),
        args: vec![string_val("test")],
        ty: IrType::Void,
    };
    assert_eq!(instr.to_string(), r#"print("test") : void"#);

    // Empty arguments
    let instr = Instruction::Call {
        dest: Some("v".to_string()),
        func: "get_value".to_string(),
        args: vec![],
        ty: IrType::F64,
    };
    assert_eq!(instr.to_string(), "v = get_value() : f64");

    // Special characters in function name
    let instr = Instruction::Call {
        dest: Some("v".to_string()),
        func: "function@with#special$chars".to_string(),
        args: vec![i32_val(42)],
        ty: IrType::I32,
    };
    assert_eq!(
        instr.to_string(),
        "v = function@with#special$chars(42i32) : i32"
    );
}

#[test]
fn test_getelementptr_instruction() {
    let instr = Instruction::GetElementPtr {
        dest: "ptr".to_string(),
        base: local_val("arr"),
        index: i32_val(5),
        element_ty: IrType::I32,
    };
    assert_eq!(instr.to_string(), "ptr = getelementptr %arr, 5i32 : i32");

    let instr = Instruction::GetElementPtr {
        dest: "elem".to_string(),
        base: Value {
            kind: ValueKind::Global("global_arr".to_string()),
            ty: IrType::Array(Box::new(IrType::I8), 100),
        },
        index: i32_val(99),
        element_ty: IrType::I8,
    };
    assert_eq!(
        instr.to_string(),
        "elem = getelementptr @global_arr, 99i32 : i8"
    );
}

#[test]
fn test_cast_instruction() {
    let instr = Instruction::Cast {
        dest: "casted".to_string(),
        value: i32_val(100),
        from_ty: IrType::I32,
        to_ty: IrType::F64,
    };
    assert_eq!(instr.to_string(), "casted = cast 100i32 from i32 to f64");

    let instr = Instruction::Cast {
        dest: "b".to_string(),
        value: Value::new_immediate(ImmediateValue::F32(3.14)),
        from_ty: IrType::F32,
        to_ty: IrType::I32,
    };
    assert_eq!(instr.to_string(), "b = cast 3.14f32 from f32 to i32");
}

#[test]
fn test_phi_instruction() {
    let instr = Instruction::Phi {
        dest: "val".to_string(),
        ty: IrType::I32,
        incoming: vec![
            (i32_val(1), "block1".to_string()),
            (local_val("x"), "block2".to_string()),
        ],
    };
    assert_eq!(
        instr.to_string(),
        "val = phi i32 [ [ 1i32, block1 ], [ %x, block2 ] ]"
    );

    // Single incoming value
    let instr = Instruction::Phi {
        dest: "v".to_string(),
        ty: IrType::Bool,
        incoming: vec![(Value::new_immediate(ImmediateValue::Bool(true)), "entry".to_string())],
    };
    assert_eq!(instr.to_string(), "v = phi bool [ [ true, entry ] ]");

    // Empty incoming
    let instr = Instruction::Phi {
        dest: "empty".to_string(),
        ty: IrType::I32,
        incoming: vec![],
    };
    assert_eq!(instr.to_string(), "empty = phi i32 [  ]");

    // Special characters in block names
    let instr = Instruction::Phi {
        dest: "p".to_string(),
        ty: IrType::F32,
        incoming: vec![
            (Value::new_immediate(ImmediateValue::F32(1.0)), "block$1".to_string()),
            (Value::new_immediate(ImmediateValue::F32(2.0)), "block#2".to_string()),
        ],
    };
    assert_eq!(
        instr.to_string(),
        "p = phi f32 [ [ 1f32, block$1 ], [ 2f32, block#2 ] ]"
    );
}

#[test]
fn test_edge_cases() {
    // Empty names
    let instr = Instruction::Alloca {
        dest: "".to_string(),
        ty: IrType::I8,
    };
    assert_eq!(instr.to_string(), " = alloca i8");

    let instr = Instruction::Load {
        dest: "".to_string(),
        src: Value {
            kind: ValueKind::Local("".to_string()),
            ty: IrType::I8,
        },
        ty: IrType::I8,
    };
    assert_eq!(instr.to_string(), " = load i8 from %");

    // Special characters in names
    let instr = Instruction::Alloca {
        dest: "var@name".to_string(),
        ty: IrType::String,
    };
    assert_eq!(instr.to_string(), "var@name = alloca string");

    let instr = Instruction::Call {
        dest: Some("result#1".to_string()),
        func: "func$name".to_string(),
        args: vec![Value {
            kind: ValueKind::Local("loc@l".to_string()),
            ty: IrType::I32,
        }],
        ty: IrType::I32,
    };
    assert_eq!(
        instr.to_string(),
        "result#1 = func$name(%loc@l) : i32"
    );

    // Extreme values
    let large_num = i64::MAX;
    let instr = Instruction::Binary {
        op: IrBinaryOp::Add,
        dest: "big".to_string(),
        left: Value::new_immediate(ImmediateValue::I64(large_num)),
        right: Value::new_immediate(ImmediateValue::I64(1)),
        ty: IrType::I64,
    };
    assert_eq!(
        instr.to_string(),
        format!("big = add {}i64 {}i64, i64", large_num, 1)
    );

    // NaN and Infinity
    let instr = Instruction::Unary {
        op: IrUnaryOp::Negate,
        dest: "nan".to_string(),
        operand: Value::new_immediate(ImmediateValue::F32(f32::NAN)),
        ty: IrType::F32,
    };
    assert_eq!(instr.to_string(), "nan = neg NaNf32 f32");
}