// tests/nir_type_tests.rs
use jsavrs::ir::{IrType, ResourceId, ScopeId};
use jsavrs::location::source_span::SourceSpan;
use std::fmt::{Debug, Display};
use uuid::Uuid;

fn assert_default_uuid_is_unique_and_valid<T>()
where
    T: Default + Display + Eq + Debug,
{
    let id1 = T::default();
    let id2 = T::default();

    assert_ne!(id1, id2, "Default values should be unique across calls");
    assert!(Uuid::parse_str(&id1.to_string()).is_ok(), "Default value must format as a valid UUID");
    assert!(Uuid::parse_str(&id2.to_string()).is_ok(), "Default value must format as a valid UUID");
}

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
        (IrType::Custom("MyCustomType".into(), SourceSpan::default()), "MyCustomType"),
        (
            IrType::Struct(
                "MyStruct".into(),
                vec![("field1".into(), IrType::I32), ("field2".into(), IrType::F64)],
                SourceSpan::default(),
            ),
            "struct MyStruct { field1: i32, field2: f64 }",
        ),
    ];

    for (ty, expected) in types {
        let output = format!("{ty}");
        assert_eq!(output, expected);
    }
}

#[test]
fn test_scope_id_default() {
    // Ensure default values are unique and format as valid UUIDs
    assert_default_uuid_is_unique_and_valid::<ScopeId>();
}

#[test]
fn test_resource_id_default() {
    // Ensure default values are unique and format as valid UUIDs
    assert_default_uuid_is_unique_and_valid::<ResourceId>();
}

#[test]
fn test_get_bit_width_for_all_integer_types() {
    // Test all signed integer types
    assert_eq!(IrType::I8.get_bit_width(), 8);
    assert_eq!(IrType::I16.get_bit_width(), 16);
    assert_eq!(IrType::I32.get_bit_width(), 32);
    assert_eq!(IrType::I64.get_bit_width(), 64);

    // Test all unsigned integer types
    assert_eq!(IrType::U8.get_bit_width(), 8);
    assert_eq!(IrType::U16.get_bit_width(), 16);
    assert_eq!(IrType::U32.get_bit_width(), 32);
    assert_eq!(IrType::U64.get_bit_width(), 64);
}

#[test]
fn test_get_bit_width_for_float_types() {
    // Test all float types
    assert_eq!(IrType::F32.get_bit_width(), 32);
    assert_eq!(IrType::F64.get_bit_width(), 64);
}

#[test]
fn test_get_bit_width_for_special_types() {
    // Test types that should return default bit width (32)
    assert_eq!(IrType::Bool.get_bit_width(), 32);
    assert_eq!(IrType::Char.get_bit_width(), 32);
    assert_eq!(IrType::String.get_bit_width(), 32);
    assert_eq!(IrType::Void.get_bit_width(), 32);
}

#[test]
fn test_get_bit_width_for_complex_types() {
    // Test pointer type - should return default bit width (32)
    assert_eq!(IrType::Pointer(Box::new(IrType::I32)).get_bit_width(), 32);

    // Test array type - should return default bit width (32)
    assert_eq!(IrType::Array(Box::new(IrType::I32), 10).get_bit_width(), 32);

    // Test custom type - should return default bit width (32)
    let custom_type = IrType::Custom("MyCustomType".into(), SourceSpan::default());
    assert_eq!(custom_type.get_bit_width(), 32);

    // Test struct type - should return default bit width (32)
    let struct_type = IrType::Struct(
        "MyStruct".into(),
        vec![("field1".into(), IrType::I32), ("field2".into(), IrType::F64)],
        SourceSpan::default(),
    );
    assert_eq!(struct_type.get_bit_width(), 32);
}

#[test]
fn test_get_bit_width_consistency() {
    // Verify that integer and unsigned integer pairs have the same bit widths
    assert_eq!(IrType::I8.get_bit_width(), IrType::U8.get_bit_width());
    assert_eq!(IrType::I16.get_bit_width(), IrType::U16.get_bit_width());
    assert_eq!(IrType::I32.get_bit_width(), IrType::U32.get_bit_width());
    assert_eq!(IrType::I64.get_bit_width(), IrType::U64.get_bit_width());

    // Verify that float sizes match expected bit widths
    assert_eq!(IrType::F32.get_bit_width(), 32);
    assert_eq!(IrType::F64.get_bit_width(), 64);
}

#[test]
fn test_get_bit_width_edge_cases() {
    // Test with deeply nested types to ensure default behavior
    let nested_pointer = IrType::Pointer(Box::new(IrType::Pointer(Box::new(IrType::I32))));
    assert_eq!(nested_pointer.get_bit_width(), 32);

    // Test with a complex nested array
    let nested_array = IrType::Array(Box::new(IrType::Array(Box::new(IrType::I8), 5)), 10);
    assert_eq!(nested_array.get_bit_width(), 32);
}
