use jsavrs::nir::{IrBinaryOp, IrConstantValue, IrLiteralValue, IrType, IrUnaryOp, Value, ValueDebugInfo, ValueKind};
use jsavrs::parser::ast::{BinaryOp, UnaryOp};
use jsavrs::utils::*;

#[test]
fn value_creation_and_properties() {
    // Literal value
    let literal_val = Value::new_literal(IrLiteralValue::I32(42));
    assert!(matches!(literal_val.kind, ValueKind::Literal(_)));
    assert_eq!(literal_val.ty, IrType::I32);
    assert!(literal_val.debug_info.is_none());

    // Constant value
    let const_val = Value::new_constant(IrConstantValue::String { string: "test".into() }, IrType::String);
    assert!(matches!(const_val.kind, ValueKind::Constant(_)));
    assert_eq!(const_val.ty, IrType::String);

    // Local value
    let local_val = Value::new_local("var".into(), IrType::Bool);
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
    val = val.with_debug_info(Some("debug_var".into()), dummy_span());

    let debug_info = val.debug_info.as_ref().unwrap();
    assert_eq!(debug_info.name, Some("debug_var".into()));
    assert_eq!(debug_info.source_span, dummy_span());
}

#[test]
fn value_display_formatting() {
    // Literal values
    assert_eq!(format!("{}", Value::new_literal(IrLiteralValue::I8(-10))), "-10i8");
    assert_eq!(format!("{}", Value::new_literal(IrLiteralValue::U16(65535))), "65535u16");
    assert_eq!(format!("{}", Value::new_literal(IrLiteralValue::F32(3.5))), "3.5f32");
    assert_eq!(format!("{}", Value::new_literal(IrLiteralValue::Bool(true))), "true");
    assert_eq!(format!("{}", Value::new_literal(IrLiteralValue::Char('"'))), "'\\\"'");

    // Constant values
    let array_val = Value::new_constant(
        IrConstantValue::Array {
            elements: vec![Value::new_literal(IrLiteralValue::I32(1)), Value::new_literal(IrLiteralValue::I32(2))],
        },
        IrType::Array(Box::new(IrType::I32), 2),
    );
    assert_eq!(format!("{}", array_val), "[1i32, 2i32]");

    let struct_val = Value::new_constant(
        IrConstantValue::Struct {
            name: "Point".into(),
            elements: vec![Value::new_literal(IrLiteralValue::I32(10)), Value::new_literal(IrLiteralValue::I32(20))],
        },
        IrType::Struct(
            "Point".into(),
            vec![("p1".to_string(), IrType::I32), ("p2".to_string(), IrType::I32)],
            dummy_span(),
        ),
    );
    assert_eq!(format!("{}", struct_val), "Point<10i32, 20i32>");

    // Local/Global/Temporary
    assert_eq!(format!("{}", Value::new_local("foo".into(), IrType::I32)), "%foo");
    assert_eq!(format!("{}", Value::new_global("bar".into(), IrType::I32)), "@bar");
    assert_eq!(format!("{}", Value::new_temporary(123, IrType::F32)), "t123");
}

// Tests for IrLiteralValue
#[test]
fn literal_value_type_conversion() {
    assert_eq!(IrType::from(&IrLiteralValue::I8(0)), IrType::I8);
    assert_eq!(IrType::from(&IrLiteralValue::U64(0)), IrType::U64);
    assert_eq!(IrType::from(&IrLiteralValue::F64(0.0)), IrType::F64);
    assert_eq!(IrType::from(&IrLiteralValue::Bool(false)), IrType::Bool);
    assert_eq!(IrType::from(&IrLiteralValue::Char('a')), IrType::Char);
}

#[test]
fn literal_value_display_edge_cases() {
    // Float edge cases
    assert_eq!(format!("{}", IrLiteralValue::F32(0.0)), "0.0f32");
    assert_eq!(format!("{}", IrLiteralValue::F32(-0.0)), "-0.0f32");
    assert_eq!(format!("{}", IrLiteralValue::F32(f32::NAN)), "NaNf32");
    assert_eq!(format!("{}", IrLiteralValue::F32(f32::INFINITY)), "inff32");
    assert_eq!(format!("{}", IrLiteralValue::F32(f32::NEG_INFINITY)), "-inff32");

    // Integer bounds
    assert_eq!(format!("{}", IrLiteralValue::I8(-128)), "-128i8");
    assert_eq!(format!("{}", IrLiteralValue::U8(255)), "255u8");

    // Character escaping
    assert_eq!(format!("{}", IrLiteralValue::Char('\n')), "'\\n'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\'')), "'\\''");
    assert_eq!(format!("{}", IrLiteralValue::Char('\0')), "'\\u{0}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\u{7}')), "'\\u{7}'");
}

// Tests for IrConstantValue
#[test]
fn constant_value_display_edge_cases() {
    // Empty array
    let empty_array = IrConstantValue::Array { elements: Vec::new() };
    assert_eq!(format!("{}", empty_array), "[]");

    // Array with different types
    let mixed_array = IrConstantValue::Array {
        elements: vec![Value::new_literal(IrLiteralValue::I32(1)), Value::new_literal(IrLiteralValue::Bool(true))],
    };
    assert_eq!(format!("{}", mixed_array), "[1i32, true]");

    // String with escapes
    let string_val = IrConstantValue::String { string: "line1\nline2\"tab\t".into() };
    assert_eq!(format!("{}", string_val), "\"line1\\nline2\\\"tab\\t\"");

    // Empty struct
    let empty_struct = IrConstantValue::Struct { name: "Empty".into(), elements: Vec::new() };
    assert_eq!(format!("{}", empty_struct), "Empty<>");

    // Struct with special characters in name
    let struct_val = IrConstantValue::Struct { name: "My$Struct".into(), elements: Vec::new() };
    assert_eq!(format!("{}", struct_val), "My$Struct<>");
}

// Tests for ValueKind
#[test]
fn value_kind_variants() {
    let literal = ValueKind::Literal(IrLiteralValue::I32(42));
    assert!(matches!(literal, ValueKind::Literal(_)));

    let constant = ValueKind::Constant(IrConstantValue::String { string: "test".into() });
    assert!(matches!(constant, ValueKind::Constant(_)));

    let local = ValueKind::Local("var".into());
    assert!(matches!(local, ValueKind::Local(_)));

    let global = ValueKind::Global("global".into());
    assert!(matches!(global, ValueKind::Global(_)));

    let temp = ValueKind::Temporary(123);
    assert!(matches!(temp, ValueKind::Temporary(123)));
}

// Tests for ValueDebugInfo
#[test]
fn debug_info_creation() {
    let debug_info = ValueDebugInfo { name: Some("var".into()), source_span: dummy_span() };

    assert_eq!(debug_info.name, Some("var".into()));
    assert_eq!(debug_info.source_span, dummy_span());

    let no_name = ValueDebugInfo { name: None, source_span: dummy_span() };
    assert!(no_name.name.is_none());
}
#[test]
fn literal_to_type_conversion() {
    // Test di conversione per tutti i tipi
    assert_eq!(IrType::from(&IrLiteralValue::I8(0)), IrType::I8);
    assert_eq!(IrType::from(&IrLiteralValue::I16(0)), IrType::I16);
    assert_eq!(IrType::from(&IrLiteralValue::I32(0)), IrType::I32);
    assert_eq!(IrType::from(&IrLiteralValue::I64(0)), IrType::I64);

    assert_eq!(IrType::from(&IrLiteralValue::U8(0)), IrType::U8);
    assert_eq!(IrType::from(&IrLiteralValue::U16(0)), IrType::U16);
    assert_eq!(IrType::from(&IrLiteralValue::U32(0)), IrType::U32);
    assert_eq!(IrType::from(&IrLiteralValue::U64(0)), IrType::U64);

    assert_eq!(IrType::from(&IrLiteralValue::F32(0.0)), IrType::F32);
    assert_eq!(IrType::from(&IrLiteralValue::F64(0.0)), IrType::F64);

    assert_eq!(IrType::from(&IrLiteralValue::Bool(true)), IrType::Bool);
    assert_eq!(IrType::from(&IrLiteralValue::Char('a')), IrType::Char);
}

#[test]
fn integer_display_formatting() {
    // Limiti degli interi con segno
    assert_eq!(format!("{}", IrLiteralValue::I8(-128)), "-128i8");
    assert_eq!(format!("{}", IrLiteralValue::I8(127)), "127i8");
    assert_eq!(format!("{}", IrLiteralValue::I16(-32768)), "-32768i16");
    assert_eq!(format!("{}", IrLiteralValue::I16(32767)), "32767i16");
    assert_eq!(format!("{}", IrLiteralValue::I32(-2147483648)), "-2147483648i32");
    assert_eq!(format!("{}", IrLiteralValue::I32(2147483647)), "2147483647i32");
    assert_eq!(format!("{}", IrLiteralValue::I64(-9223372036854775808)), "-9223372036854775808i64");
    assert_eq!(format!("{}", IrLiteralValue::I64(9223372036854775807)), "9223372036854775807i64");

    // Limiti degli interi senza segno
    assert_eq!(format!("{}", IrLiteralValue::U8(0)), "0u8");
    assert_eq!(format!("{}", IrLiteralValue::U8(255)), "255u8");
    assert_eq!(format!("{}", IrLiteralValue::U16(0)), "0u16");
    assert_eq!(format!("{}", IrLiteralValue::U16(65535)), "65535u16");
    assert_eq!(format!("{}", IrLiteralValue::U32(0)), "0u32");
    assert_eq!(format!("{}", IrLiteralValue::U32(4294967295)), "4294967295u32");
    assert_eq!(format!("{}", IrLiteralValue::U64(0)), "0u64");
    assert_eq!(format!("{}", IrLiteralValue::U64(18446744073709551615)), "18446744073709551615u64");
}

#[allow(clippy::approx_constant)]
#[test]
fn float_display_formatting() {
    // Numeri interi rappresentati come float
    assert_eq!(format!("{}", IrLiteralValue::F32(42.0)), "42.0f32");
    assert_eq!(format!("{}", IrLiteralValue::F32(-100.0)), "-100.0f32");
    assert_eq!(format!("{}", IrLiteralValue::F64(123456.0)), "123456.0f64");

    // Numeri frazionari
    assert_eq!(format!("{}", IrLiteralValue::F32(3.14159)), "3.14159f32");
    assert_eq!(format!("{}", IrLiteralValue::F64(2.718281828459045)), "2.718281828459045f64");

    // Zero negativo
    assert_eq!(format!("{}", IrLiteralValue::F32(-0.0)), "-0.0f32");
    assert_eq!(format!("{}", IrLiteralValue::F64(-0.0)), "-0.0f64");

    // Valori speciali
    assert_eq!(format!("{}", IrLiteralValue::F32(f32::NAN)), "NaNf32");
    assert_eq!(format!("{}", IrLiteralValue::F32(f32::INFINITY)), "inff32");
    assert_eq!(format!("{}", IrLiteralValue::F32(f32::NEG_INFINITY)), "-inff32");
    assert_eq!(format!("{}", IrLiteralValue::F64(f64::NAN)), "NaNf64");
    assert_eq!(format!("{}", IrLiteralValue::F64(f64::INFINITY)), "inff64");
    assert_eq!(format!("{}", IrLiteralValue::F64(f64::NEG_INFINITY)), "-inff64");
}

#[test]
fn bool_display_formatting() {
    assert_eq!(format!("{}", IrLiteralValue::Bool(true)), "true");
    assert_eq!(format!("{}", IrLiteralValue::Bool(false)), "false");
}

#[test]
fn char_display_formatting() {
    // Caratteri ASCII stampabili
    assert_eq!(format!("{}", IrLiteralValue::Char('a')), "'a'");
    assert_eq!(format!("{}", IrLiteralValue::Char('Z')), "'Z'");
    assert_eq!(format!("{}", IrLiteralValue::Char(' ')), "' '");
    assert_eq!(format!("{}", IrLiteralValue::Char('@')), "'@'");

    // Caratteri speciali (escape necessari)
    assert_eq!(format!("{}", IrLiteralValue::Char('\'')), "'\\\''");
    assert_eq!(format!("{}", IrLiteralValue::Char('\"')), "'\\\"'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\\')), "'\\\\'");

    // Caratteri di controllo
    assert_eq!(format!("{}", IrLiteralValue::Char('\n')), "'\\n'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\r')), "'\\r'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\t')), "'\\t'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\0')), "'\\u{0}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\x07')), "'\\u{7}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\x1F')), "'\\u{1f}'");

    // Caratteri Unicode
    assert_eq!(format!("{}", IrLiteralValue::Char('é')), "'\\u{e9}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('ß')), "'\\u{df}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('あ')), "'\\u{3042}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('😂')), "'\\u{1f602}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\u{FFFF}')), "'\\u{ffff}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\u{10FFFF}')), "'\\u{10ffff}'");

    // Caratteri che richiedono escape Unicode
    assert_eq!(format!("{}", IrLiteralValue::Char('\u{0}')), "'\\u{0}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\u{1F}')), "'\\u{1f}'");
    assert_eq!(format!("{}", IrLiteralValue::Char('\u{7F}')), "'\\u{7f}'");
}

#[test]
fn display_precision_edge_cases() {
    // Numeri che sono esattamente interi
    assert_eq!(format!("{}", IrLiteralValue::F32(5.0)), "5.0f32");
    assert_eq!(format!("{}", IrLiteralValue::F64(-10.0)), "-10.0f64");

    // Numeri con frazioni molto piccole
    assert_eq!(format!("{}", IrLiteralValue::F32(0.000000001)), "0.000000001f32");
    assert_eq!(format!("{}", IrLiteralValue::F64(0.0000000000000001)), "0.0000000000000001f64");

    // Numeri che sembrano interi ma hanno parte frazionaria
    assert_eq!(format!("{}", IrLiteralValue::F32(1.0000001)), "1.0000001f32");
    assert_eq!(format!("{}", IrLiteralValue::F64(2.000000000000001)), "2.000000000000001f64");
}

#[test]
fn test_binary_op_conversion() {
    // Test all variants
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

    for (input, expected) in test_cases {
        let result: IrBinaryOp = input.clone().into();
        assert_eq!(result, expected, "Failed conversion for {:?}: expected {:?}, got {:?}", input, expected, result);
    }
}

#[test]
fn test_unary_op_conversion() {
    // Test all variants
    let test_cases = vec![(UnaryOp::Negate, IrUnaryOp::Negate), (UnaryOp::Not, IrUnaryOp::Not)];

    for (input, expected) in test_cases {
        let result: IrUnaryOp = input.clone().into();
        assert_eq!(result, expected, "Failed conversion for {:?}: expected {:?}, got {:?}", input, expected, result);
    }
}
