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
