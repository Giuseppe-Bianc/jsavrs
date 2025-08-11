// src/ir/values.rs
use crate::ir::types::Type;
use std::fmt;
//use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Constant(Constant),
    Instruction(InstructionRef),
    Argument(ArgumentRef),
    Global(GlobalRef),
    BlockAddress(String, String),
    Undef(Type),
    Poison(Type),
}

#[derive(Debug, Clone)]
pub enum Constant {
    Integer { value: u64, ty: Type },
    Float { value: f64, ty: Type },
    String(String),
    Bool(bool),
    Null(Type),
    Array(Vec<Constant>),
    Struct(Vec<Constant>),
    ZeroInitializer(Type),
    GlobalReference(String),
    Expression(Box<ConstantExpr>),  // Aggiunto Box per interrompere la ricorsione
}

#[derive(Debug, Clone)]
pub enum ConstantExpr {
    GetElementPtr {
        ty: Type,
        base: Box<Constant>,  // Aggiunto Box per interrompere la ricorsione
        indices: Vec<(Constant, Type)>
    },
    BinaryOp {
        op: BinaryOperator,
        left: Box<Constant>,  // Aggiunto Box per interrompere la ricorsione
        right: Box<Constant>, // Aggiunto Box per interrompere la ricorsione
        ty: Type
    },
    Conversion {
        op: ConversionOp,
        src: Box<Constant>,   // Aggiunto Box per interrompere la ricorsione
        src_ty: Type,
        dest_ty: Type
    },
    ExtractValue {
        aggregate: Box<Constant>,  // Aggiunto Box per interrompere la ricorsione
        indices: Vec<u32>
    },
    InsertValue {
        aggregate: Box<Constant>,  // Aggiunto Box per interrompere la ricorsione
        element: Box<Constant>,   // Aggiunto Box per interrompere la ricorsione
        indices: Vec<u32>
    },
    Select {
        condition: Box<Constant>,  // Aggiunto Box per interrompere la ricorsione
        true_value: Box<Constant>, // Aggiunto Box per interrompere la ricorsione
        false_value: Box<Constant> // Aggiunto Box per interrompere la ricorsione
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    UDiv,
    SDiv,
    URem,
    SRem,
    FAdd,
    FSub,
    FMul,
    FDiv,
    FRem,
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionOp {
    Trunc,
    ZExt,
    SExt,
    FPTrunc,
    FPExt,
    FPToUI,
    FPToSI,
    UIToFP,
    SIToFP,
    PtrToInt,
    IntToPtr,
    BitCast,
    AddrSpaceCast,
}

#[derive(Debug, Clone)]
pub struct InstructionRef {
    pub id: u32,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct ArgumentRef {
    pub index: u32,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct GlobalRef {
    pub name: String,
    pub ty: Type,
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Constant(c) => c.get_type(),
            Value::Instruction(ir) => ir.ty.clone(),
            Value::Argument(ar) => ar.ty.clone(),
            Value::Global(gr) => gr.ty.clone(),
            Value::BlockAddress(_, _) => Type::pointer_to(Type::i8()),
            Value::Undef(ty) => ty.clone(),
            Value::Poison(ty) => ty.clone(),
        }
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, Value::Constant(_))
    }

    pub fn is_instruction(&self) -> bool {
        matches!(self, Value::Instruction(_))
    }

    pub fn is_argument(&self) -> bool {
        matches!(self, Value::Argument(_))
    }

    pub fn is_global(&self) -> bool {
        matches!(self, Value::Global(_))
    }
}

impl Constant {
    pub fn get_type(&self) -> Type {
        match self {
            Constant::Integer { ty, .. } => ty.clone(),
            Constant::Float { ty, .. } => ty.clone(),
            Constant::String(_) => Type::array_of(Type::i8(), 0),
            Constant::Bool(_) => Type::bool(),
            Constant::Null(ty) => ty.clone(),
            Constant::Array(elems) => {
                if let Some(first) = elems.first() {
                    Type::array_of(first.get_type(), elems.len() as u64)
                } else {
                    Type::array_of(Type::i8(), 0)
                }
            }
            Constant::Struct(fields) => {
                let field_types: Vec<Type> = fields.iter().map(|f| f.get_type()).collect();
                Type::Struct {
                    name: format!("struct_{}", fields.len()),
                    fields: field_types.iter().enumerate().map(|(i, ty)| (format!("field_{}", i), ty.clone())).collect(),
                    packed: false,
                }
            }
            Constant::ZeroInitializer(ty) => ty.clone(),
            Constant::GlobalReference(_) => Type::pointer_to(Type::void()),
            Constant::Expression(expr) => expr.get_type(),
        }
    }
}

impl ConstantExpr {
    pub fn get_type(&self) -> Type {
        match self {
            ConstantExpr::GetElementPtr { ty, .. } => Type::pointer_to(ty.clone()),
            ConstantExpr::BinaryOp { ty, .. } => ty.clone(),
            ConstantExpr::Conversion { dest_ty, .. } => dest_ty.clone(),
            ConstantExpr::ExtractValue { aggregate, indices } => {
                let mut ty = aggregate.get_type();
                for &index in indices {
                    if let Type::Struct { fields, .. } = &ty {
                        if let Some((_, field_ty)) = fields.get(index as usize) {
                            ty = field_ty.clone();
                        } else {
                            break;
                        }
                    } else if let Type::Array { element, .. } = &ty {
                        ty = *element.clone();
                    } else {
                        break;
                    }
                }
                ty
            }
            ConstantExpr::InsertValue { aggregate, .. } => aggregate.get_type(),
            ConstantExpr::Select { true_value, .. } => true_value.get_type(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Constant(c) => write!(f, "{}", c),
            Value::Instruction(ir) => write!(f, "%{}", ir.id),
            Value::Argument(ar) => write!(f, "%{}", ar.index),
            Value::Global(gr) => write!(f, "@{}", gr.name),
            Value::BlockAddress(func, block) => write!(f, "blockaddress(@{}, %{})", func, block),
            Value::Undef(ty) => write!(f, "undef {}", ty),
            Value::Poison(ty) => write!(f, "poison {}", ty),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Integer { value, ty } => write!(f, "{} {}", value, ty),
            Constant::Float { value, ty } => write!(f, "{} {}", value, ty),
            Constant::String(s) => write!(f, "\"{}\"", s),
            Constant::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Constant::Null(ty) => write!(f, "null {}", ty),
            Constant::Array(elems) => {
                write!(f, "[")?;
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Constant::Struct(fields) => {
                write!(f, "{{")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, "}}")
            }
            Constant::ZeroInitializer(ty) => write!(f, "zeroinitializer {}", ty),
            Constant::GlobalReference(name) => write!(f, "@{}", name),
            Constant::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for ConstantExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantExpr::GetElementPtr { ty, base, indices } => {
                write!(f, "getelementptr inbounds ({}, {}, ", ty, base)?;
                for (i, (idx, idx_ty)) in indices.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", idx_ty, idx)?;
                }
                write!(f, ")")
            }
            ConstantExpr::BinaryOp { op, left, right, ty } => {
                write!(f, "({} {} {} {})", ty, op, left, right)
            }
            ConstantExpr::Conversion { op, src, src_ty, dest_ty } => {
                write!(f, "({} to {} {} {})", op, dest_ty, src_ty, src)
            }
            ConstantExpr::ExtractValue { aggregate, indices } => {
                write!(f, "extractvalue ({}, ", aggregate)?;
                for (i, &idx) in indices.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", idx)?;
                }
                write!(f, ")")
            }
            ConstantExpr::InsertValue { aggregate, element, indices } => {
                write!(f, "insertvalue ({}, {}, ", aggregate, element)?;
                for (i, &idx) in indices.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", idx)?;
                }
                write!(f, ")")
            }
            ConstantExpr::Select { condition, true_value, false_value } => {
                write!(f, "select ({}, {}, {})", condition, true_value, false_value)
            }
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "add"),
            BinaryOperator::Sub => write!(f, "sub"),
            BinaryOperator::Mul => write!(f, "mul"),
            BinaryOperator::UDiv => write!(f, "udiv"),
            BinaryOperator::SDiv => write!(f, "sdiv"),
            BinaryOperator::URem => write!(f, "urem"),
            BinaryOperator::SRem => write!(f, "srem"),
            BinaryOperator::FAdd => write!(f, "fadd"),
            BinaryOperator::FSub => write!(f, "fsub"),
            BinaryOperator::FMul => write!(f, "fmul"),
            BinaryOperator::FDiv => write!(f, "fdiv"),
            BinaryOperator::FRem => write!(f, "frem"),
            BinaryOperator::Shl => write!(f, "shl"),
            BinaryOperator::LShr => write!(f, "lshr"),
            BinaryOperator::AShr => write!(f, "ashr"),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::Xor => write!(f, "xor"),
        }
    }
}

impl fmt::Display for ConversionOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionOp::Trunc => write!(f, "trunc"),
            ConversionOp::ZExt => write!(f, "zext"),
            ConversionOp::SExt => write!(f, "sext"),
            ConversionOp::FPTrunc => write!(f, "fptrunc"),
            ConversionOp::FPExt => write!(f, "fpext"),
            ConversionOp::FPToUI => write!(f, "fptoui"),
            ConversionOp::FPToSI => write!(f, "fptosi"),
            ConversionOp::UIToFP => write!(f, "uitofp"),
            ConversionOp::SIToFP => write!(f, "sitofp"),
            ConversionOp::PtrToInt => write!(f, "ptrtoint"),
            ConversionOp::IntToPtr => write!(f, "inttoptr"),
            ConversionOp::BitCast => write!(f, "bitcast"),
            ConversionOp::AddrSpaceCast => write!(f, "addrspacecast"),
        }
    }
}