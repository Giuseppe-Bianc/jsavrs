// src/ir/types.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Void,
    Integer { bits: u32, is_signed: bool },
    Float { bits: u32 },
    Pointer { pointee: Box<Type>, address_space: Option<u32> },
    Array { element: Box<Type>, count: u64 },
    Vector { element: Box<Type>, count: u64 },
    Struct { name: String, fields: Vec<(String, Type)>, packed: bool },
    Function {
        return_type: Box<Type>,
        parameters: Vec<Type>,
        is_variadic: bool
    },
    Named(String),
}

impl Type {
    pub fn is_void(&self) -> bool {
        matches!(self, Type::Void)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Integer { .. })
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::Float { .. })
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::Pointer { .. })
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array { .. })
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Type::Struct { .. })
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Type::Function { .. })
    }

    pub fn is_sized(&self) -> bool {
        match self {
            Type::Void => false,
            Type::Integer { .. } => true,
            Type::Float { .. } => true,
            Type::Pointer { .. } => true,
            Type::Array { element, count } => {
                // Array is sized if element is sized and count is not zero
                element.is_sized() && *count > 0
            }
            Type::Vector { element, count } => {
                // Vector is sized if element is sized and count is not zero
                element.is_sized() && *count > 0
            }
            Type::Struct { fields, .. } => {
                // Struct is sized if all fields are sized
                fields.iter().all(|(_, ty)| ty.is_sized())
            }
            Type::Function { .. } => false,
            Type::Named(_) => {
                // Named types could be sized or not, but we'll assume they are
                // In a real implementation, we would look up the actual type
                true
            }
        }
    }

    pub fn get_pointer_element_type(&self) -> Option<&Type> {
        if let Type::Pointer { pointee, .. } = self {
            Some(pointee)
        } else {
            None
        }
    }

    pub fn get_array_element_type(&self) -> Option<&Type> {
        if let Type::Array { element, .. } = self {
            Some(element)
        } else {
            None
        }
    }

    pub fn get_struct_fields(&self) -> Option<&Vec<(String, Type)>> {
        if let Type::Struct { fields, .. } = self {
            Some(fields)
        } else {
            None
        }
    }

    pub fn size_in_bits(&self) -> Option<u64> {
        match self {
            Type::Integer { bits, .. } => Some(*bits as u64),
            Type::Float { bits } => Some(*bits as u64),
            Type::Pointer { .. } => Some(64), // Assuming 64-bit pointers
            Type::Array { element, count } => {
                element.size_in_bits().map(|size| size * count)
            }
            Type::Struct { fields, .. } => {
                fields.iter().try_fold(0, |acc, (_, ty)| {
                    ty.size_in_bits().map(|size| acc + size)
                })
            }
            _ => None,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Integer { bits, is_signed } => {
                if *is_signed {
                    write!(f, "i{}", bits)
                } else {
                    write!(f, "u{}", bits)
                }
            }
            Type::Float { bits } => write!(f, "f{}", bits),
            Type::Pointer { pointee, address_space } => {
                if let Some(addr_space) = address_space {
                    write!(f, "{} addrspace({})", pointee, addr_space)
                } else {
                    write!(f, "{}*", pointee)
                }
            }
            Type::Array { element, count } => write!(f, "[{} x {}]", element, count),
            Type::Vector { element, count } => write!(f, "<{} x {}>", element, count),
            Type::Struct { name, fields:_, packed } => {
                if *packed {
                    write!(f, "<{{ {} }}>", name)
                } else {
                    write!(f, "{{ {} }}", name)
                }
            }
            Type::Function { return_type, parameters, is_variadic } => {
                write!(f, "{} (", return_type)?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                if *is_variadic {
                    write!(f, ", ...")?;
                }
                write!(f, ")")
            }
            Type::Named(name) => write!(f, "%{}", name),
        }
    }
}

// Common type constructors
impl Type {
    pub fn i8() -> Self { Type::Integer { bits: 8, is_signed: true } }
    pub fn i16() -> Self { Type::Integer { bits: 16, is_signed: true } }
    pub fn i32() -> Self { Type::Integer { bits: 32, is_signed: true } }
    pub fn i64() -> Self { Type::Integer { bits: 64, is_signed: true } }
    pub fn u8() -> Self { Type::Integer { bits: 8, is_signed: false } }
    pub fn u16() -> Self { Type::Integer { bits: 16, is_signed: false } }
    pub fn u32() -> Self { Type::Integer { bits: 32, is_signed: false } }
    pub fn u64() -> Self { Type::Integer { bits: 64, is_signed: false } }
    pub fn f32() -> Self { Type::Float { bits: 32 } }
    pub fn f64() -> Self { Type::Float { bits: 64 } }
    pub fn bool() -> Self { Type::Integer { bits: 1, is_signed: false } }
    pub fn void() -> Self { Type::Void }

    pub fn pointer_to(ty: Type) -> Self {
        Type::Pointer {
            pointee: Box::new(ty),
            address_space: None
        }
    }

    pub fn array_of(ty: Type, count: u64) -> Self {
        Type::Array {
            element: Box::new(ty),
            count
        }
    }

    pub fn vector_of(ty: Type, count: u64) -> Self {
        Type::Vector {
            element: Box::new(ty),
            count
        }
    }
}