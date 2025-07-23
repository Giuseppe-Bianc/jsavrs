use jsavrs::utils::*;
use jsavrs::nir::{Value, IrLiteralValue, IrConstantValue, ValueKind, IrType, ValueDebugInfo};

#[test]
fn value_creation_and_properties() {
    // Literal value
    let literal_val = Value::new_literal(IrLiteralValue::I32(42));
    assert!(matches!(literal_val.kind, ValueKind::Literal(_)));
    assert_eq!(literal_val.ty, IrType::I32);
    assert!(literal_val.debug_info.is_none());

    // Constant value
    let const_val = Value::new_constant(
        IrConstantValue::String("test".to_string()),
        IrType::String,
    );
    assert!(matches!(const_val.kind, ValueKind::Constant(_)));
    assert_eq!(const_val.ty, IrType::String);

    // Local value
    let local_val = Value::new_local("var".to_string(), IrType::Bool);
    assert!(matches!(local_val.kind, ValueKind::Local(_)));
    assert_eq!(local_val.ty, IrType::Bool);

    // Temporary value
    let temp_val = Value::new_temporary(99, IrType::F64);
    assert!(matches!(temp_val.kind, ValueKind::Temporary(99)));
    assert_eq!(temp_val.ty, IrType::F64);
}

#[test]
fn value_with_debug_info() {
    let mut val = Value::new_literal(IrLiteralValue::Bool(true));
    val = val.with_debug_info(Some("debug_var".to_string()), dummy_span());

    let debug_info = val.debug_info.as_ref().unwrap();
    assert_eq!(debug_info.name, Some("debug_var".to_string()));
    assert_eq!(debug_info.source_span, dummy_span());
}

#[test]
fn value_display_formatting() {
    // Literal values
    assert_eq!(
        format!("{}", Value::new_literal(IrLiteralValue::I8(-10))),
        "-10i8"
    );
    assert_eq!(
        format!("{}", Value::new_literal(IrLiteralValue::U16(65535))),
        "65535u16"
    );
    assert_eq!(
        format!("{}", Value::new_literal(IrLiteralValue::F32(3.5))),
        "3.5f32"
    );
    assert_eq!(
        format!("{}", Value::new_literal(IrLiteralValue::Bool(true))),
        "true"
    );
    assert_eq!(
        format!("{}", Value::new_literal(IrLiteralValue::Char('"'))),
        "'\\\"'"
    );

    // Constant values
    let array_val = Value::new_constant(
        IrConstantValue::Array(vec![
            Value::new_literal(IrLiteralValue::I32(1)),
            Value::new_literal(IrLiteralValue::I32(2)),
        ]),
        IrType::Array(Box::new(IrType::I32), 2),
    );
    assert_eq!(format!("{}", array_val), "[1i32, 2i32]");

    let struct_val = Value::new_constant(
        IrConstantValue::Struct(
            "Point".to_string(),
            vec![
                Value::new_literal(IrLiteralValue::I32(10)),
                Value::new_literal(IrLiteralValue::I32(20)),
            ],
        ),
        IrType::Struct("Point".to_string(), vec![IrType::I32, IrType::I32], dummy_span()),
    );
    assert_eq!(format!("{}", struct_val), "Point<10i32, 20i32>");

    // Local/Global/Temporary
    assert_eq!(
        format!("{}", Value::new_local("foo".to_string(), IrType::I32)),
        "%foo"
    );
    assert_eq!(
        format!(
            "{}",
            Value {
                id: 1,
                kind: ValueKind::Global("bar".to_string()),
                ty: IrType::I32,
                debug_info: None,
            }
        ),
        "@bar"
    );
    assert_eq!(
        format!("{}", Value::new_temporary(123, IrType::F32)),
        "t123"
    );
}

// Tests for IrLiteralValue
#[test]
fn literal_value_type_conversion() {
    assert_eq!(
        IrType::from(&IrLiteralValue::I8(0)),
        IrType::I8
    );
    assert_eq!(
        IrType::from(&IrLiteralValue::U64(0)),
        IrType::U64
    );
    assert_eq!(
        IrType::from(&IrLiteralValue::F64(0.0)),
        IrType::F64
    );
    assert_eq!(
        IrType::from(&IrLiteralValue::Bool(false)),
        IrType::Bool
    );
    assert_eq!(
        IrType::from(&IrLiteralValue::Char('a')),
        IrType::Char
    );
}

#[test]
fn literal_value_display_edge_cases() {
    // Float edge cases
    assert_eq!(
        format!("{}", IrLiteralValue::F32(0.0)),
        "0.0f32"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::F32(-0.0)),
        "-0.0f32"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::F32(f32::NAN)),
        "NaNf32"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::F32(f32::INFINITY)),
        "inff32"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::F32(f32::NEG_INFINITY)),
        "-inff32"
    );

    // Integer bounds
    assert_eq!(
        format!("{}", IrLiteralValue::I8(-128)),
        "-128i8"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::U8(255)),
        "255u8"
    );

    // Character escaping
    assert_eq!(
        format!("{}", IrLiteralValue::Char('\n')),
        "'\\n'"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::Char('\'')),
        "'\\''"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::Char('\0')),
        "'\\u{0}'"
    );
    assert_eq!(
        format!("{}", IrLiteralValue::Char('\u{7}')),
        "'\\u{7}'"
    );
}

// Tests for IrConstantValue
#[test]
fn constant_value_display_edge_cases() {
    // Empty array
    let empty_array = IrConstantValue::Array(Vec::new());
    assert_eq!(format!("{}", empty_array), "[]");

    // Array with different types
    let mixed_array = IrConstantValue::Array(vec![
        Value::new_literal(IrLiteralValue::I32(1)),
        Value::new_literal(IrLiteralValue::Bool(true)),
    ]);
    assert_eq!(format!("{}", mixed_array), "[1i32, true]");

    // String with escapes
    let string_val = IrConstantValue::String("line1\nline2\"tab\t".to_string());
    assert_eq!(
        format!("{}", string_val),
        "\"line1\\nline2\\\"tab\\t\""
    );

    // Empty struct
    let empty_struct = IrConstantValue::Struct("Empty".to_string(), Vec::new());
    assert_eq!(format!("{}", empty_struct), "Empty<>");

    // Struct with special characters in name
    let struct_val = IrConstantValue::Struct("My$Struct".to_string(), Vec::new());
    assert_eq!(format!("{}", struct_val), "My$Struct<>");
}

// Tests for ValueKind
#[test]
fn value_kind_variants() {
    let literal = ValueKind::Literal(IrLiteralValue::I32(42));
    assert!(matches!(literal, ValueKind::Literal(_)));

    let constant = ValueKind::Constant(IrConstantValue::String("test".to_string()));
    assert!(matches!(constant, ValueKind::Constant(_)));

    let local = ValueKind::Local("var".to_string());
    assert!(matches!(local, ValueKind::Local(_)));

    let global = ValueKind::Global("global".to_string());
    assert!(matches!(global, ValueKind::Global(_)));

    let temp = ValueKind::Temporary(123);
    assert!(matches!(temp, ValueKind::Temporary(123)));
}

// Tests for ValueDebugInfo
#[test]
fn debug_info_creation() {
    let debug_info = ValueDebugInfo {
        name: Some("var".to_string()),
        source_span: dummy_span()
    };

    assert_eq!(debug_info.name, Some("var".to_string()));
    assert_eq!(debug_info.source_span, dummy_span());

    let no_name = ValueDebugInfo {
        name: None,
        source_span: dummy_span()
    };
    assert!(no_name.name.is_none());
}