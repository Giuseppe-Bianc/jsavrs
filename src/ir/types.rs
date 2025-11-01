// src/ir/types.rs

use crate::location::source_span::SourceSpan;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

/// Represents all possible intermediate representation (IR) types
/// used within the compiler or analysis phase.
///
/// This enumeration includes primitive types (integers, floats, booleans),
/// compound types (arrays, pointers, structs), and custom user-defined types.
///
/// Each variant optionally carries metadata such as source span information,
/// which helps in diagnostics and error reporting.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum IrType {
    /// 8-bit signed integer type (`i8`).
    #[default]
    I8,
    /// 16-bit signed integer type (`i16`).
    I16,
    /// 32-bit signed integer type (`i32`).
    I32,
    /// 64-bit signed integer type (`i64`).
    I64,
    /// 8-bit unsigned integer type (`u8`).
    U8,
    /// 16-bit unsigned integer type (`u16`).
    U16,
    /// 32-bit unsigned integer type (`u32`).
    U32,
    /// 64-bit unsigned integer type (`u64`).
    U64,
    /// 32-bit floating-point type (`f32`).
    F32,
    /// 64-bit floating-point type (`f64`).
    F64,
    /// Boolean type (`bool`).
    Bool,
    /// Unicode scalar value type (`char`).
    Char,
    /// String type (`string`), typically a UTF-8 encoded sequence.
    String,
    /// Void type — represents the absence of a value (similar to `()` or `void`).
    Void,
    /// Pointer type — represents a pointer to another `IrType`.
    Pointer(Box<IrType>),
    /// Array type — represents a fixed-size array of a specific element type.
    Array(Box<IrType>, usize),
    /// Custom user-defined type — identified by name and source span.
    Custom(Arc<str>, SourceSpan), // Added source span
    /// Struct type — consists of named fields and a source span for debugging.
    Struct(Arc<str>, Vec<(String, IrType)>, SourceSpan), // New struct type
}

/// A unique identifier representing a scope within the IR (e.g., a function, block, or module).
///
/// This is implemented as a UUID to ensure global uniqueness across compilation units.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct ScopeId(Uuid);

impl ScopeId {
    /// Creates a new, globally unique `ScopeId` using a random UUID.
    ///
    /// # Returns
    /// A new `ScopeId` instance with a randomly generated UUID.
    pub fn new() -> Self {
        ScopeId(Uuid::new_v4())
    }
}

impl Default for ScopeId {
    /// Provides a default implementation that generates a new unique `ScopeId`.
    fn default() -> Self {
        ScopeId::new()
    }
}

/// A globally unique identifier representing a resource within the IR system.
///
/// This can refer to various high-level resources such as types, modules,
/// or external symbols. Implemented as a UUID for uniqueness.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct ResourceId(Uuid);

impl ResourceId {
    /// Creates a new, globally unique `ResourceId` using a random UUID.
    ///
    /// # Returns
    /// A new `ResourceId` instance with a unique identifier.
    pub fn new() -> Self {
        ResourceId(Uuid::new_v4())
    }
}

impl Default for ResourceId {
    /// Provides a default implementation that generates a new unique `ResourceId`.
    fn default() -> Self {
        ResourceId::new()
    }
}

// -------------------------------------------------------------------------------------------------
// Display trait implementations
// -------------------------------------------------------------------------------------------------

/// Implements the `Display` trait for `ScopeId`, allowing it to be formatted
/// as a string (UUID) when printed.
impl fmt::Display for ScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implements the `Display` trait for `ResourceId`, allowing it to be formatted
/// as a string (UUID) when printed.
impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// -------------------------------------------------------------------------------------------------
// IrType methods
// -------------------------------------------------------------------------------------------------

impl IrType {
    /// Determines whether the IR type represents a signed integer (`i8`, `i16`, `i32`, `i64`).
    ///
    /// # Returns
    /// `true` if the type is a signed integer; otherwise, `false`.
    pub fn is_signed_integer(&self) -> bool {
        matches!(self, IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64)
    }

    /// Determines whether the IR type represents an unsigned integer (`u8`, `u16`, `u32`, `u64`).
    ///
    /// # Returns
    /// `true` if the type is an unsigned integer; otherwise, `false`.
    pub fn is_unsigned_integer(&self) -> bool {
        matches!(self, IrType::U8 | IrType::U16 | IrType::U32 | IrType::U64)
    }

    /// Returns the bit-width associated with this IR type.
    ///
    /// For integer and floating-point types, this returns their precise bit-width.
    /// For other types (such as pointers, structs, etc.), a default width of `32` is returned.
    ///
    /// # Returns
    /// A `u32` representing the bit-width of the type.
    pub fn get_bit_width(&self) -> u32 {
        match self {
            IrType::I8 | IrType::U8 => 8,
            IrType::I16 | IrType::U16 => 16,
            IrType::I32 | IrType::U32 => 32,
            IrType::I64 | IrType::U64 => 64,
            IrType::F32 => 32,
            IrType::F64 => 64,
            _ => 32, // Default width for other types (heuristic)
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Display implementation for IrType
// -------------------------------------------------------------------------------------------------

/// Provides a human-readable string representation for each IR type.
/// This is primarily used for debugging, error messages, and IR dumps.
impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::I8 => write!(f, "i8"),
            IrType::I16 => write!(f, "i16"),
            IrType::I32 => write!(f, "i32"),
            IrType::I64 => write!(f, "i64"),
            IrType::U8 => write!(f, "u8"),
            IrType::U16 => write!(f, "u16"),
            IrType::U32 => write!(f, "u32"),
            IrType::U64 => write!(f, "u64"),
            IrType::F32 => write!(f, "f32"),
            IrType::F64 => write!(f, "f64"),
            IrType::Bool => write!(f, "bool"),
            IrType::Char => write!(f, "char"),
            IrType::String => write!(f, "string"),
            IrType::Void => write!(f, "void"),
            IrType::Pointer(inner) => write!(f, "*{inner}"),
            IrType::Array(element_type, size) => write!(f, "[{element_type}; {size}]"),
            IrType::Custom(name, _) => write!(f, "{name}"),
            IrType::Struct(name, fields, _) => {
                let fields_str =
                    fields.iter().map(|(field_name, ty)| format!("{field_name}: {ty}")).collect::<Vec<_>>().join(", ");
                write!(f, "struct {name} {{ {fields_str} }}")
            }
        }
    }
}
