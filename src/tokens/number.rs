use std::fmt;
//src/tokens/number.rs
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Integer(i64),
    UnsignedInteger(u64),
    Float32(f32),
    Float64(f64),
    Scientific32(f32, i32), // Base (f32), exponent (i32)
    Scientific64(f64, i32), // Base (f64), exponent (i32)
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{i}"),
            Number::UnsignedInteger(u) => write!(f, "{u}"),
            Number::Float32(flt) => write!(f, "{flt}"),
            Number::Float64(flt) => write!(f, "{flt}"),
            Number::Scientific32(base, exp) => write!(f, "{base}e{exp}"),
            Number::Scientific64(base, exp) => write!(f, "{base}e{exp}"),
        }
    }
}
