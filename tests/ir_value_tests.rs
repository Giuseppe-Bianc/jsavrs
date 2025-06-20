use jsavrs::ir::{ImmediateValue, IrType, Value, ValueKind};

#[test]
fn test_immediate_value_creation_and_display() {
    // Integer types
    assert_eq!(
        Value::new_immediate(ImmediateValue::I8(-42)).to_string(),
        "-42i8"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::I16(0)).to_string(),
        "0i16"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::I32(i32::MIN)).to_string(),
        "-2147483648i32"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::I64(9_223_372_036_854_775_807)).to_string(),
        "9223372036854775807i64"
    );

    // Unsigned integers
    assert_eq!(
        Value::new_immediate(ImmediateValue::U8(255)).to_string(),
        "255u8"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::U16(u16::MAX)).to_string(),
        "65535u16"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::U32(4_294_967_295)).to_string(),
        "4294967295u32"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::U64(18_446_744_073_709_551_615)).to_string(),
        "18446744073709551615u64"
    );

    // Floating point (test edge cases and formatting)
    assert_eq!(
        Value::new_immediate(ImmediateValue::F32(3.14)).to_string(),
        "3.14f32"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::F32(f32::NAN)).to_string(),
        "NaNf32"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::F32(f32::INFINITY)).to_string(),
        "inff32"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::F64(-0.0)).to_string(),
        "-0f64"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::F64(1e100)).to_string(),
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f64"
    );

    // Bool and char
    assert_eq!(
        Value::new_immediate(ImmediateValue::Bool(true)).to_string(),
        "true"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::Bool(false)).to_string(),
        "false"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::Char('a')).to_string(),
        "'a'"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::Char('\n')).to_string(),
        "'\n'"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::Char('\"')).to_string(),
        "'\"'"
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::Char('\u{1F600}')).to_string(),
        "'\u{1F600}'"
    );

    // String (test escaping)
    assert_eq!(
        Value::new_immediate(ImmediateValue::String("hello".into())).to_string(),
        "\"hello\""
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::String("\n\t\"\\".into())).to_string(),
        r#""\n\t\"\\""#
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::String("\x07\x1B".into())).to_string(),
        r#""\u{7}\u{1b}""#
    );
    assert_eq!(
        Value::new_immediate(ImmediateValue::String("".into())).to_string(),
        "\"\""
    );
}

#[test]
fn test_local_value_creation_and_display() {
    let val = Value::new_local("foo".into(), IrType::I32);
    assert_eq!(val.to_string(), "%foo");
    assert_eq!(val.ty, IrType::I32);

    // Edge cases: empty name
    let empty = Value::new_local("".into(), IrType::Bool);
    assert_eq!(empty.to_string(), "%");
    assert_eq!(empty.ty, IrType::Bool);
}

#[test]
fn test_global_value_display() {
    let val = Value {
        kind: ValueKind::Global("bar".into()),
        ty: IrType::String,
    };
    assert_eq!(val.to_string(), "@bar");
    assert_eq!(val.ty, IrType::String);

    // Special characters in name
    let special = Value {
        kind: ValueKind::Global("name@with!special#chars".into()),
        ty: IrType::I8,
    };
    assert_eq!(special.to_string(), "@name@with!special#chars");
}

#[test]
fn test_temporary_value_display() {
    let val = Value::new_temporary("123".into(), IrType::F64);
    assert_eq!(val.to_string(), "t123");
    assert_eq!(val.ty, IrType::F64);

    // Empty ID (should be allowed by the code)
    let empty = Value::new_temporary("".into(), IrType::Char);
    assert_eq!(empty.to_string(), "t");
    assert_eq!(empty.ty, IrType::Char);
}

#[test]
fn test_value_kind_display_edge_cases() {
    // Local with special characters
    let special_local = Value {
        kind: ValueKind::Local("name\n\t".into()),
        ty: IrType::I16,
    };
    assert_eq!(special_local.to_string(), "%name\n\t");

    // Global with empty name
    let empty_global = Value {
        kind: ValueKind::Global("".into()),
        ty: IrType::U8,
    };
    assert_eq!(empty_global.to_string(), "@");
}
