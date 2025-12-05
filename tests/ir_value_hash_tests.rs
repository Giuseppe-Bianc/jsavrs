use jsavrs::ir::{IrConstantValue, IrLiteralValue, IrType, Value, ValueKind};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Helper function to compute hash of a value
fn compute_hash<T: Hash>(value: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn test_value_hash_excludes_id() {
    // Two values with same semantic content but different IDs should hash identically
    let val1 = Value::new_literal(IrLiteralValue::I32(42));
    let val2 = Value::new_literal(IrLiteralValue::I32(42));

    assert_ne!(val1.id, val2.id, "IDs should be different");
    assert_eq!(compute_hash(&val1), compute_hash(&val2), "Hash should be identical for same semantic content");
}

#[test]
fn test_value_hash_excludes_debug_info() {
    use jsavrs::location::source_location::SourceLocation;
    use jsavrs::location::source_span::SourceSpan;

    let val1 = Value::new_literal(IrLiteralValue::I32(42));

    let span = SourceSpan::new(Arc::from("test.js"), SourceLocation::new(1, 1, 0), SourceLocation::new(1, 5, 4));
    let val2 = Value::new_literal(IrLiteralValue::I32(42)).with_debug_info(Some(Arc::from("x")), span);

    assert_eq!(compute_hash(&val1), compute_hash(&val2), "Debug info should not affect hash");
}

#[test]
fn test_value_hash_excludes_scope() {
    use jsavrs::ir::ScopeId;

    let val1 = Value::new_literal(IrLiteralValue::I32(42));
    let val2 = Value::new_literal(IrLiteralValue::I32(42)).with_scope(ScopeId::new());

    assert_eq!(compute_hash(&val1), compute_hash(&val2), "Scope should not affect hash");
}

#[test]
fn test_value_hash_consistency_with_eq() {
    // This is the fundamental requirement: if a == b, then hash(a) == hash(b)
    let values = [
        Value::new_literal(IrLiteralValue::I32(42)),
        Value::new_literal(IrLiteralValue::I32(42)),
        Value::new_local(Arc::from("x"), IrType::I32),
        Value::new_local(Arc::from("x"), IrType::I32),
        Value::new_global(Arc::from("g"), IrType::F64),
        Value::new_global(Arc::from("g"), IrType::F64),
        Value::new_temporary(1, IrType::Bool),
        Value::new_temporary(1, IrType::Bool),
    ];

    for i in 0..values.len() {
        for j in 0..values.len() {
            if values[i] == values[j] {
                assert_eq!(
                    compute_hash(&values[i]),
                    compute_hash(&values[j]),
                    "Equal values must have equal hashes: {:?} == {:?}",
                    values[i],
                    values[j]
                );
            }
        }
    }
}

#[test]
fn test_value_hash_different_for_different_types() {
    let val1 = Value::new_literal(IrLiteralValue::I32(42));
    let val2 = Value::new_literal(IrLiteralValue::I64(42));

    assert_ne!(compute_hash(&val1), compute_hash(&val2), "Different types should have different hashes");
}

#[test]
fn test_literal_hash_float_special_values() {
    // Test that NaN, infinity, etc. are handled correctly
    let nan1 = IrLiteralValue::F32(f32::NAN);
    let nan2 = IrLiteralValue::F32(f32::NAN);
    let inf = IrLiteralValue::F32(f32::INFINITY);
    let neg_inf = IrLiteralValue::F32(f32::NEG_INFINITY);

    // NaN values with same bit pattern should hash identically
    assert_eq!(compute_hash(&nan1), compute_hash(&nan2));

    // Different special values should hash differently
    assert_ne!(compute_hash(&inf), compute_hash(&neg_inf));
}

#[test]
fn test_literal_hash_zero_values() {
    // -0.0 and +0.0 should hash differently (they're not equal in our impl)
    let pos_zero_f32 = IrLiteralValue::F32(0.0);
    let neg_zero_f32 = IrLiteralValue::F32(-0.0);

    assert_ne!(
        compute_hash(&pos_zero_f32),
        compute_hash(&neg_zero_f32),
        "Positive and negative zero should hash differently"
    );
}

#[test]
fn test_literal_hash_integer_types() {
    // Same numeric value but different types should hash differently
    let i32_val = IrLiteralValue::I32(42);
    let i64_val = IrLiteralValue::I64(42);
    let u32_val = IrLiteralValue::U32(42);

    assert_ne!(compute_hash(&i32_val), compute_hash(&i64_val));
    assert_ne!(compute_hash(&i32_val), compute_hash(&u32_val));
    assert_ne!(compute_hash(&i64_val), compute_hash(&u32_val));
}

#[test]
fn test_value_kind_hash_local_vs_global() {
    let local = ValueKind::Local(Arc::from("x"));
    let global = ValueKind::Global(Arc::from("x"));

    assert_ne!(compute_hash(&local), compute_hash(&global), "Local and global with same name should hash differently");
}

#[test]
fn test_value_kind_hash_string_content() {
    // Same string content should hash identically
    let local1 = ValueKind::Local(Arc::from("variable"));
    let local2 = ValueKind::Local(Arc::from("variable"));

    assert_eq!(compute_hash(&local1), compute_hash(&local2), "Same string content should produce same hash");

    // Different string content should hash differently
    let local3 = ValueKind::Local(Arc::from("other"));
    assert_ne!(compute_hash(&local1), compute_hash(&local3));
}

#[test]
fn test_constant_value_hash_strings() {
    let str1 = IrConstantValue::String { string: Arc::from("hello") };
    let str2 = IrConstantValue::String { string: Arc::from("hello") };
    let str3 = IrConstantValue::String { string: Arc::from("world") };

    assert_eq!(compute_hash(&str1), compute_hash(&str2), "Same strings should hash identically");
    assert_ne!(compute_hash(&str1), compute_hash(&str3), "Different strings should hash differently");
}

#[test]
fn test_constant_value_hash_arrays() {
    let elem1 = Value::new_literal(IrLiteralValue::I32(1));
    let elem2 = Value::new_literal(IrLiteralValue::I32(2));

    let arr1 = IrConstantValue::Array { elements: vec![elem1.clone(), elem2.clone()] };
    let arr2 = IrConstantValue::Array { elements: vec![elem1.clone(), elem2.clone()] };
    let arr3 = IrConstantValue::Array { elements: vec![elem2.clone(), elem1.clone()] };

    assert_eq!(compute_hash(&arr1), compute_hash(&arr2), "Same arrays should hash identically");
    assert_ne!(compute_hash(&arr1), compute_hash(&arr3), "Different arrays should hash differently");
}

#[test]
fn test_value_in_hashmap() {
    // Test that Value works correctly as HashMap key
    let mut map: HashMap<Value, String> = HashMap::new();

    let key1 = Value::new_literal(IrLiteralValue::I32(42));
    let key2 = Value::new_literal(IrLiteralValue::I32(42)); // Same semantic content, different ID

    map.insert(key1.clone(), "first".to_string());

    // Should be able to retrieve using semantically equal key
    assert_eq!(map.get(&key2), Some(&"first".to_string()));
    assert_eq!(map.len(), 1);

    // Inserting with key2 should replace the value
    map.insert(key2.clone(), "second".to_string());
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&key1), Some(&"second".to_string()));
}

#[test]
fn test_value_in_hashset() {
    // Test that Value works correctly in HashSet
    let mut set: HashSet<Value> = HashSet::new();

    let val1 = Value::new_literal(IrLiteralValue::I32(42));
    let val2 = Value::new_literal(IrLiteralValue::I32(42)); // Same semantic content
    let val3 = Value::new_literal(IrLiteralValue::I32(43)); // Different content

    set.insert(val1.clone());
    set.insert(val2.clone());
    set.insert(val3.clone());

    // Should only have 2 elements (val1 and val2 are semantically equal)
    assert_eq!(set.len(), 2);
    assert!(set.contains(&val1));
    assert!(set.contains(&val2));
    assert!(set.contains(&val3));
}

#[test]
fn test_hash_performance_characteristics() {
    // Test that hash computation is deterministic
    let val = Value::new_local(Arc::from("variable"), IrType::I32);

    let hash1 = compute_hash(&val);
    let hash2 = compute_hash(&val);
    let hash3 = compute_hash(&val);

    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
}

#[test]
fn test_complex_constant_hash() {
    // Test struct with nested values
    let field1 = Value::new_literal(IrLiteralValue::I32(1));
    let field2 = Value::new_literal(IrLiteralValue::F64(2.5));

    let struct1 = IrConstantValue::Struct { name: Arc::from("Point"), elements: vec![field1.clone(), field2.clone()] };

    let struct2 = IrConstantValue::Struct { name: Arc::from("Point"), elements: vec![field1.clone(), field2.clone()] };

    let struct3 = IrConstantValue::Struct { name: Arc::from("Vector"), elements: vec![field1.clone(), field2.clone()] };

    assert_eq!(compute_hash(&struct1), compute_hash(&struct2), "Same structs should hash identically");
    assert_ne!(compute_hash(&struct1), compute_hash(&struct3), "Structs with different names should hash differently");
}

#[test]
fn test_temporary_values_hash() {
    let temp1 = Value::new_temporary(1, IrType::I32);
    let temp2 = Value::new_temporary(1, IrType::I32);
    let temp3 = Value::new_temporary(2, IrType::I32);

    assert_eq!(compute_hash(&temp1), compute_hash(&temp2), "Same temporaries should hash identically");
    assert_ne!(compute_hash(&temp1), compute_hash(&temp3), "Different temporaries should hash differently");
}
