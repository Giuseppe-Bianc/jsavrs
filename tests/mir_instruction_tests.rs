use jsavrs::nir::{CastKind, Instruction, InstructionKind, IrBinaryOp, IrConstantValue, IrLiteralValue, IrType, IrUnaryOp, Value, ValueKind, VectorOp};
use jsavrs::utils::dummy_span;
// tests/nir_instruction_tests.rs


fn temp_value(id: u64, ty: IrType) -> Value {
    Value {
        id,
        kind: ValueKind::Temporary(id),
        ty,
        debug_info: None,
    }
}

#[test]
fn test_alloca_instruction() {
    let ty = IrType::I32;
    let inst = Instruction::new(InstructionKind::Alloca { ty: ty.clone() }, dummy_span())
        .with_result(temp_value(1, ty.clone()));

    assert_eq!(format!("{}", inst), "t1 = alloca i32");
}

#[test]
fn test_store_instruction() {
    let value = Value::new_literal(IrLiteralValue::I32(42));
    let dest = Value::new_local("x".to_string(), IrType::I32);
    let inst = Instruction::new(
        InstructionKind::Store { value: value.clone(), dest: dest.clone() },
        dummy_span()
    );

    assert_eq!(format!("{}", inst), "store 42i32 to %x");
}

#[test]
fn test_load_instruction() {
    let src = Value::new_local("ptr".to_string(), IrType::Pointer(Box::new(IrType::I32)));
    let ty = IrType::I32;
    let inst = Instruction::new(
        InstructionKind::Load { src: src.clone(), ty: ty.clone() },
        dummy_span()
    ).with_result(temp_value(2, ty));

    assert_eq!(format!("{}", inst), "t2 = load i32 from %ptr");
}

#[test]
fn test_binary_instruction() {
    let left = Value::new_literal(IrLiteralValue::I32(10));
    let right = Value::new_literal(IrLiteralValue::I32(20));
    let ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::Binary {
            op: IrBinaryOp::Add,
            left: left.clone(),
            right: right.clone(),
            ty: ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(3, ty));

    assert_eq!(format!("{}", inst), "t3 = add 10i32 20i32, i32");
}

#[test]
fn test_unary_instruction() {
    let operand = Value::new_literal(IrLiteralValue::I32(100));
    let ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::Unary {
            op: IrUnaryOp::Negate,
            operand: operand.clone(),
            ty: ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(4, ty));

    assert_eq!(format!("{}", inst), "t4 = neg 100i32 i32");
}

#[test]
fn test_call_instruction() {
    let func = Value::new_local("my_func".to_string(), IrType::Custom("fn()".to_string(), dummy_span()));
    let arg1 = Value::new_literal(IrLiteralValue::I32(1));
    let arg2 = Value::new_literal(IrLiteralValue::I32(2));
    let ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::Call {
            func: func.clone(),
            args: vec![arg1.clone(), arg2.clone()],
            ty: ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(5, ty));

    assert_eq!(format!("{}", inst), "t5 = %my_func(1i32, 2i32) : i32");
}

#[test]
fn test_call_instruction_no_args() {
    let func = Value::new_local("void_func".to_string(), IrType::Void);
    let ty = IrType::Void;

    let inst = Instruction::new(
        InstructionKind::Call {
            func: func.clone(),
            args: vec![],
            ty: ty.clone(),
        },
        dummy_span()
    );

    assert_eq!(format!("{}", inst), "%void_func() : void");
}

#[test]
fn test_gep_instruction() {
    let base = Value::new_local("arr".to_string(), IrType::Array(Box::new(IrType::I32), 10));
    let index = Value::new_literal(IrLiteralValue::I32(5));
    let element_ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::GetElementPtr {
            base: base.clone(),
            index: index.clone(),
            element_ty: element_ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(6, IrType::Pointer(Box::new(element_ty.clone()))));

    assert_eq!(format!("{}", inst), "t6 =  getelementptr %arr, 5i32 : i32");
}

#[test]
fn test_cast_instruction() {
    let value = Value::new_literal(IrLiteralValue::F32(3.14));
    let from_ty = IrType::F32;
    let to_ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::Cast {
            kind: CastKind::FloatToInt,
            value: value.clone(),
            from_ty: from_ty.clone(),
            to_ty: to_ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(7, to_ty));

    assert_eq!(format!("{}", inst), "t7 =  cast 3.14f32 from f32 to i32");
}

#[test]
fn test_phi_instruction() {
    let val1 = Value::new_literal(IrLiteralValue::I32(10));
    let val2 = Value::new_literal(IrLiteralValue::I32(20));
    let ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::Phi {
            ty: ty.clone(),
            incoming: vec![
                (val1.clone(), "block1".to_string()),
                (val2.clone(), "block2".to_string()),
            ],
        },
        dummy_span()
    ).with_result(temp_value(8, ty));

    assert_eq!(
        format!("{}", inst),
        "t8 =  phi i32 [ [ 10i32, block1 ], [ 20i32, block2 ] ]"
    );
}

#[test]
fn test_vector_instruction() {
    let vec1 = temp_value(9, IrType::Custom("vec3".to_string(), dummy_span()));
    let vec2 = temp_value(10, IrType::Custom("vec3".to_string(), dummy_span()));
    let ty = IrType::Custom("vec3".to_string(), dummy_span());

    let inst = Instruction::new(
        InstructionKind::Vector {
            op: VectorOp::Add,
            operands: vec![vec1.clone(), vec2.clone()],
            ty: ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(11, ty));

    assert_eq!(format!("{}", inst), "t11 =  vector.vadd t9, t10 : vec3");
}

#[test]
fn test_instruction_without_result() {
    let value = Value::new_literal(IrLiteralValue::I32(42));
    let dest = Value::new_local("x".to_string(), IrType::I32);
    let inst = Instruction::new(
        InstructionKind::Store { value, dest },
        dummy_span()
    );

    assert_eq!(format!("{}", inst), "store 42i32 to %x");
}

#[test]
fn test_float_literal_display() {
    let whole = Value::new_literal(IrLiteralValue::F32(5.0));
    let fractional = Value::new_literal(IrLiteralValue::F64(3.14159));

    let inst_whole = Instruction::new(
        InstructionKind::Load {
            src: Value::new_local("ptr".to_string(), IrType::Pointer(Box::new(IrType::F32))),
            ty: IrType::F32,
        },
        dummy_span()
    ).with_result(whole);

    let inst_frac = Instruction::new(
        InstructionKind::Load {
            src: Value::new_local("ptr".to_string(), IrType::Pointer(Box::new(IrType::F64))),
            ty: IrType::F64,
        },
        dummy_span()
    ).with_result(fractional);

    assert_eq!(format!("{}", inst_whole), "5.0f32 = load f32 from %ptr");
    assert_eq!(format!("{}", inst_frac), "3.14159f64 = load f64 from %ptr");
}

#[test]
fn test_string_constant_display() {
    let string_val = Value::new_constant(
        IrConstantValue::String("hello\nworld".to_string()),
        IrType::String
    );

    let inst = Instruction::new(
        InstructionKind::Store {
            value: string_val,
            dest: Value::new_local("s".to_string(), IrType::String),
        },
        dummy_span()
    );

    assert_eq!(
        format!("{}", inst),
        r#"store "hello\nworld" to %s"#
    );
}

#[test]
fn test_array_constant_display() {
    let elements = vec![
        Value::new_literal(IrLiteralValue::I32(1)),
        Value::new_literal(IrLiteralValue::I32(2)),
        Value::new_literal(IrLiteralValue::I32(3)),
    ];
    let array_val = Value::new_constant(
        IrConstantValue::Array(elements),
        IrType::Array(Box::new(IrType::I32), 3)
    );

    let inst = Instruction::new(
        InstructionKind::Store {
            value: array_val,
            dest: Value::new_local("arr".to_string(), IrType::Array(Box::new(IrType::I32), 3)),
        },
        dummy_span()
    );

    assert_eq!(
        format!("{}", inst),
        "store [1i32, 2i32, 3i32] to %arr"
    );
}

#[test]
fn test_struct_constant_display() {
    let fields = vec![
        Value::new_literal(IrLiteralValue::I32(10)),
        Value::new_literal(IrLiteralValue::Bool(true)),
    ];
    let struct_val = Value::new_constant(
        IrConstantValue::Struct("Point".to_string(), fields),
        IrType::Struct("Point".to_string(), vec![IrType::I32, IrType::Bool], dummy_span())
    );

    let inst = Instruction::new(
        InstructionKind::Store {
            value: struct_val,
            dest: Value::new_local("pt".to_string(), IrType::Struct("Point".to_string(), vec![IrType::I32, IrType::Bool], dummy_span())),
        },
        dummy_span()
    );

    assert_eq!(
        format!("{}", inst),
        "store Point<10i32, true> to %pt"
    );
}

#[test]
fn test_pointer_type_display() {
    let ptr_type = IrType::Pointer(Box::new(IrType::I32));
    let inst = Instruction::new(
        InstructionKind::Alloca { ty: ptr_type.clone() },
        dummy_span()
    ).with_result(temp_value(12, ptr_type));

    assert_eq!(format!("{}", inst), "t12 = alloca *i32");
}

#[test]
fn test_nested_array_type_display() {
    let inner_array = IrType::Array(Box::new(IrType::I32), 5);
    let outer_array = IrType::Array(Box::new(inner_array), 10);

    let inst = Instruction::new(
        InstructionKind::Alloca { ty: outer_array.clone() },
        dummy_span()
    ).with_result(temp_value(13, outer_array));

    assert_eq!(format!("{}", inst), "t13 = alloca [[i32; 5]; 10]");
}

#[test]
fn test_struct_type_display() {
    let struct_ty = IrType::Struct(
        "Point".to_string(),
        vec![IrType::I32, IrType::I32],
        dummy_span()
    );

    let inst = Instruction::new(
        InstructionKind::Alloca { ty: struct_ty.clone() },
        dummy_span()
    ).with_result(temp_value(14, struct_ty));

    assert_eq!(
        format!("{}", inst),
        "t14 = alloca struct Point { i32, i32 }"
    );
}

#[test]
fn test_vector_shuffle_instruction() {
    let vec1 = temp_value(15, IrType::Custom("vec4".to_string(), dummy_span()));
    let vec2 = temp_value(16, IrType::Custom("vec4".to_string(), dummy_span()));
    let ty = IrType::Custom("vec4".to_string(), dummy_span());

    let inst = Instruction::new(
        InstructionKind::Vector {
            op: VectorOp::Shuffle,
            operands: vec![vec1.clone(), vec2.clone()],
            ty: ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(17, ty));

    assert_eq!(
        format!("{}", inst),
        "t17 =  vector.vshuffle t15, t16 : vec4"
    );
}

#[test]
fn test_char_literal_display() {
    let char_val = Value::new_literal(IrLiteralValue::Char('\n'));
    let inst = Instruction::new(
        InstructionKind::Store {
            value: char_val,
            dest: Value::new_local("c".to_string(), IrType::Char),
        },
        dummy_span()
    );

    assert_eq!(
        format!("{}", inst),
        "store '\\n' to %c"
    );
}

#[test]
fn test_bitcast_instruction() {
    let value = Value::new_literal(IrLiteralValue::I32(0x41424344));
    let inst = Instruction::new(
        InstructionKind::Cast {
            kind: CastKind::Bitcast,
            value: value.clone(),
            from_ty: IrType::I32,
            to_ty: IrType::Pointer(Box::new(IrType::I8)),
        },
        dummy_span()
    ).with_result(temp_value(18, IrType::Pointer(Box::new(IrType::I8))));

    assert_eq!(
        format!("{}", inst),
        "t18 =  cast 1094861636i32 from i32 to *i8"
    );
}

#[test]
fn test_phi_single_incoming() {
    let val = Value::new_literal(IrLiteralValue::I32(42));
    let ty = IrType::I32;

    let inst = Instruction::new(
        InstructionKind::Phi {
            ty: ty.clone(),
            incoming: vec![(val.clone(), "entry".to_string())],
        },
        dummy_span()
    ).with_result(temp_value(19, ty));

    assert_eq!(
        format!("{}", inst),
        "t19 =  phi i32 [ [ 42i32, entry ] ]"
    );
}

#[test]
fn test_vector_dot_product() {
    let vec1 = temp_value(20, IrType::Custom("vec3".to_string(), dummy_span()));
    let vec2 = temp_value(21, IrType::Custom("vec3".to_string(), dummy_span()));
    let ty = IrType::F32;

    let inst = Instruction::new(
        InstructionKind::Vector {
            op: VectorOp::DotProduct,
            operands: vec![vec1.clone(), vec2.clone()],
            ty: ty.clone(),
        },
        dummy_span()
    ).with_result(temp_value(22, ty));

    assert_eq!(
        format!("{}", inst),
        "t22 =  vector.vdot t20, t21 : f32"
    );
}

#[test]
fn test_all_binary_ops() {
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

    let ty = IrType::I32;
    for (idx, (op, op_str)) in ops.iter().enumerate() {
        let left = Value::new_literal(IrLiteralValue::I32(100 + idx as i32));
        let right = Value::new_literal(IrLiteralValue::I32(200 + idx as i32));

        let inst = Instruction::new(
            InstructionKind::Binary {
                op: op.clone(),
                left: left.clone(),
                right: right.clone(),
                ty: ty.clone(),
            },
            dummy_span()
        ).with_result(temp_value(1000 + idx as u64, ty.clone()));

        assert_eq!(
            format!("{}", inst),
            format!("t{} = {} {} {}, i32", 1000 + idx, op_str, left, right)
        );
    }
}

#[test]
fn test_all_unary_ops() {
    let ops = vec![
        (IrUnaryOp::Negate, "neg"),
        (IrUnaryOp::Not, "not"),
    ];

    let ty = IrType::I32;
    for (idx, (op, op_str)) in ops.iter().enumerate() {
        let operand = Value::new_literal(IrLiteralValue::I32(100 + idx as i32));
        let res_idx = 5000 + idx as u64;

        let inst = Instruction::new(
            InstructionKind::Unary {
                op: op.clone(),
                operand: operand.clone(),
                ty: ty.clone(),
            },
            dummy_span()
        ).with_result(temp_value(res_idx, ty.clone()));

        assert_eq!(
            format!("{}", inst),
            format!("t{} = {} {} i32", res_idx, op_str, operand)
        );
    }
}