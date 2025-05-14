#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Integer(i64),
    Float(f64),
    Scientific(f64, i32), // Value and exponent
}