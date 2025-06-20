use jsavrs::ir::IrType;

#[test]
fn test_ir_type_display() {
    let types = vec![
        (IrType::I8, "i8"),
        (IrType::I16, "i16"),
        (IrType::I32, "i32"),
        (IrType::I64, "i64"),
        (IrType::U8, "u8"),
        (IrType::U16, "u16"),
        (IrType::U32, "u32"),
        (IrType::U64, "u64"),
        (IrType::F32, "f32"),
        (IrType::F64, "f64"),
        (IrType::Bool, "bool"),
        (IrType::Char, "char"),
        (IrType::String, "string"),
        (IrType::Void, "void"),
        (IrType::Pointer(Box::new(IrType::I32)), "*i32"),
        (IrType::Array(Box::new(IrType::I32), 10), "[i32; 10]"),
        (IrType::Custom("MyCustomType".to_string()), "MyCustomType"),
    ];

    for (ty, expected) in types {
        let output = format!("{}", ty);
        assert_eq!(output, expected);
    }
}
