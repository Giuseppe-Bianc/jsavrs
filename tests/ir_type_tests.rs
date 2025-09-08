// tests/nir_type_tests.rs
use jsavrs::location::source_span::SourceSpan;
use jsavrs::ir::{IrType, ResourceId, ScopeId};
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
        let output = format!("{}", ty);
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
