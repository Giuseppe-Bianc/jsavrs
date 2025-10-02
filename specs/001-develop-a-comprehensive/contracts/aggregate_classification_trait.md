# API Contract: AggregateClassification Trait

**Feature**: 001-develop-a-comprehensive  
**Contract Type**: Trait Definition  
**Status**: Design Complete

## Trait Definition

```rust
/// Trait for classifying structure/union parameter passing
pub trait AggregateClassification {
    /// Classifies how an aggregate type should be passed
    ///
    /// # Parameters
    /// - `size`: Size of the aggregate in bytes
    /// - `fields`: Slice of field types in the aggregate
    ///
    /// # Returns
    /// AggregateClass indicating passing mechanism
    ///
    /// # Platform Behavior
    /// - **Windows**: size ≤ 8 → ByValue, otherwise ByReference
    /// - **System V**: size ≤ 16 and simple → Decomposed, otherwise ByReference
    fn classify_aggregate(size: usize, fields: &[FieldType]) -> AggregateClass;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateClass {
    /// Pass entire value in a single register
    ByValue(GPRegister64),
    
    /// Pass pointer to aggregate (hidden pointer parameter)
    ByReference,
    
    /// Decompose into multiple registers
    Decomposed(Vec<X86Register>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Integer,
    Float,
    Pointer,
}
```

## Implementations

```rust
impl AggregateClassification for WindowsX64 {
    fn classify_aggregate(size: usize, _fields: &[FieldType]) -> AggregateClass {
        if size <= 8 {
            // Small aggregates passed by value
            AggregateClass::ByValue(GPRegister64::Rcx)
        } else {
            // Large aggregates passed by reference
            AggregateClass::ByReference
        }
    }
}

impl AggregateClassification for SystemV {
    fn classify_aggregate(size: usize, fields: &[FieldType]) -> AggregateClass {
        if size > 16 {
            return AggregateClass::ByReference;
        }
        
        // Simplified classification (full implementation defers to reference compiler)
        if size <= 8 && fields.len() == 1 {
            match fields[0] {
                FieldType::Integer | FieldType::Pointer => 
                    AggregateClass::ByValue(GPRegister64::Rdi),
                FieldType::Float => 
                    AggregateClass::Decomposed(vec![X86Register::Xmm(XMMRegister::Xmm0)]),
            }
        } else if size <= 16 {
            // Complex structures may decompose into multiple registers
            // Full implementation matches GCC/Clang behavior
            AggregateClass::Decomposed(vec![
                X86Register::GP64(GPRegister64::Rdi),
                X86Register::GP64(GPRegister64::Rsi)
            ])
        } else {
            AggregateClass::ByReference
        }
    }
}
```

## Contract Tests

```rust
#[cfg(test)]
mod aggregate_classification_tests {
    #[test]
    fn test_windows_small_aggregate() {
        let result = WindowsX64::classify_aggregate(8, &[FieldType::Integer, FieldType::Integer]);
        assert_eq!(result, AggregateClass::ByValue(GPRegister64::Rcx));
    }

    #[test]
    fn test_windows_large_aggregate() {
        let result = WindowsX64::classify_aggregate(16, &[]);
        assert_eq!(result, AggregateClass::ByReference);
    }

    #[test]
    fn test_systemv_decomposition() {
        let result = SystemV::classify_aggregate(16, &[FieldType::Integer, FieldType::Integer]);
        assert!(matches!(result, AggregateClass::Decomposed(_)));
    }

    #[test]
    fn test_systemv_large_aggregate() {
        let result = SystemV::classify_aggregate(32, &[]);
        assert_eq!(result, AggregateClass::ByReference);
    }
}
```

## Notes

Full implementation of System V aggregate classification follows the complex algorithm specified in the System V AMD64 ABI. The simplified version shown here demonstrates the concept; production code defers to reference compiler behavior (GCC/Clang) as specified in clarifications.
