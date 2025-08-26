// tests/nir_type_tests.rs
use jsavrs::location::source_span::SourceSpan;
use jsavrs::nir::IrType;
use jsavrs::nir::ScopeId;
use jsavrs::nir::ResourceId;
use uuid::Uuid;

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
        (IrType::Custom("MyCustomType".to_string(), SourceSpan::default()), "MyCustomType"),
        (IrType::Struct("MyStruct".to_string(), vec![IrType::I32, IrType::F64], SourceSpan::default()), "struct MyStruct { i32, f64 }"),
    ];

    for (ty, expected) in types {
        let output = format!("{}", ty);
        assert_eq!(output, expected);
    }
}

#[test]
fn test_scope_id_default() {
    // Genera due valori di default
    let id1 = ScopeId::default();
    let id2 = ScopeId::default();

    // Verifica che siano diversi
    assert_ne!(id1, id2, "I valori di default di ScopeId devono essere univoci");

    // Verifica che siano UUID validi
    assert!(Uuid::parse_str(&id1.to_string()).is_ok(), "ScopeId default deve essere un UUID valido");
    assert!(Uuid::parse_str(&id2.to_string()).is_ok(), "ScopeId default deve essere un UUID valido");
}

#[test]
fn test_resource_id_default() {
    // Genera due valori di default
    let id1 = ResourceId::default();
    let id2 = ResourceId::default();

    // Verifica che siano diversi
    assert_ne!(id1, id2, "I valori di default di ResourceId devono essere univoci");

    // Verifica che siano UUID validi
    assert!(Uuid::parse_str(&id1.to_string()).is_ok(), "ResourceId default deve essere un UUID valido");
    assert!(Uuid::parse_str(&id2.to_string()).is_ok(), "ResourceId default deve essere un UUID valido");
}