// src/ir/ir.rs
use crate::ir::symbol_table::{SymbolTable, /*Symbol,*/ Parameter};
use crate::ir::types::Type;
use crate::ir::values::{Value, Constant, BinaryOperator, ConversionOp, /*InstructionRef*/};
//use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub global_variables: Vec<GlobalVariable>,
    pub type_definitions: Vec<TypeDefinition>,
    pub symbol_table: SymbolTable,
    pub data_layout: String,
    pub target_triple: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub basic_blocks: Vec<BasicBlock>,
    pub linkage: LinkageType,
    pub visibility: Visibility,
    pub attributes: Vec<FunctionAttribute>,
    pub calling_conv: CallingConvention,
    pub is_variadic: bool,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub predecessors: Vec<String>,
    pub successors: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Alloca {
        dest: Value,
        ty: Type,
        align: Option<u32>
    },
    Load {
        dest: Value,
        ptr: Value,
        ty: Type,
        align: Option<u32>
    },
    Store {
        value: Value,
        ptr: Value,
        align: Option<u32>
    },
    BinaryOp {
        op: BinaryOperator,
        dest: Value,
        left: Value,
        right: Value,
        flags: Vec<InstructionFlag>
    },
    UnaryOp {
        op: UnaryOperator,
        dest: Value,
        operand: Value
    },
    Call {
        dest: Option<Value>,
        callee: Value,
        arguments: Vec<(Value, Type)>,
        calling_conv: CallingConvention
    },
    GetElementPtr {
        dest: Value,
        ptr: Value,
        indices: Vec<(Value, Type)>,
        inbounds: bool
    },
    Conversion {
        op: ConversionOp,
        dest: Value,
        src: Value,
        src_ty: Type,
        dest_ty: Type
    },
    Phi {
        dest: Value,
        ty: Type,
        incoming: Vec<(Value, String)>
    },
    ExtractValue {
        dest: Value,
        aggregate: Value,
        indices: Vec<u32>
    },
    InsertValue {
        dest: Value,
        aggregate: Value,
        element: Value,
        indices: Vec<u32>
    },
    Select {
        dest: Value,
        condition: Value,
        true_value: Value,
        false_value: Value
    },
    ICmp {
        dest: Value,
        predicate: IntPredicate,
        left: Value,
        right: Value
    },
    FCmp {
        dest: Value,
        predicate: FloatPredicate,
        left: Value,
        right: Value
    },
    VAArg {
        dest: Value,
        va_list: Value,
        ty: Type
    },
    LandingPad {
        dest: Value,
        ty: Type,
        clauses: Vec<LandingPadClause>,
        cleanup: bool
    },
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret { value: Option<Value> },
    Br { dest: String },
    CondBr { condition: Value, true_dest: String, false_dest: String },
    Switch { value: Value, default_dest: String, cases: Vec<(Constant, String)> },
    IndirectBr { address: Value, possible_dests: Vec<String> },
    Invoke {
        callee: Value,
        arguments: Vec<(Value, Type)>,
        normal_dest: String,
        unwind_dest: String,
        dest: Option<Value>
    },
    Resume { value: Value },
    CatchSwitch {
        parent_pad: Value,
        catch_handlers: Vec<String>,
        default_unwind_dest: Option<String>
    },
    CatchRet {
        from: Value,
        to: String
    },
    CleanupRet {
        from: Value,
        unwind_dest: Option<String>
    },
    Unreachable,
}

#[derive(Debug, Clone)]
pub struct GlobalVariable {
    pub name: String,
    pub ty: Type,
    pub initializer: Option<Constant>,
    pub linkage: LinkageType,
    pub visibility: Visibility,
    pub is_constant: bool,
    pub alignment: Option<u32>,
    pub section: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub ty: Type,
    pub is_opaque: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageType {
    External,
    Private,
    Internal,
    LinkOnce,
    Weak,
    Common,
    Appending,
    ExternalWeak,
    LinkOnceODR,
    WeakODR,
    AvailableExternally,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Default,
    Hidden,
    Protected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    C,
    Fast,
    Cold,
    GHC,
    HiPE,
    WebKitJS,
    AnyReg,
    Win64,
    X86_64SysV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionAttribute {
    AlwaysInline,
    Builtin,
    Cold,
    Convergent,
    InlineHint,
    InAlloca,
    Naked,
    NoBuiltin,
    NoDuplicate,
    NoImplicitFloat,
    NoInline,
    NonLazyBind,
    NoRedZone,
    NoReturn,
    NoUnwind,
    OptimizeForSize,
    OptimizeNone,
    ReadNone,
    ReadOnly,
    ReturnsTwice,
    SExt,
    StackProtect,
    StackProtectReq,
    StackProtectStrong,
    SafeStack,
    SanitizeAddress,
    SanitizeThread,
    SanitizeMemory,
    StrictFP,
    UWTable,
    ZExt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionFlag {
    NoUnsignedWrap,
    NoSignedWrap,
    Exact,
    FastMath,
    NoNaNs,
    NoInfs,
    NoSignedZeros,
    NoReciprocal,
    AllowContract,
    ApproxFunc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    FNeg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntPredicate {
    EQ,
    NE,
    UGT,
    UGE,
    ULT,
    ULE,
    SGT,
    SGE,
    SLT,
    SLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatPredicate {
    False,
    OEQ,
    OGT,
    OGE,
    OLT,
    OLE,
    ONE,
    ORD,
    UNO,
    UEQ,
    UGT,
    UGE,
    ULT,
    ULE,
    UNE,
    True,
}

#[derive(Debug, Clone)]
pub enum LandingPadClause {
    Catch(Type),
    Filter(Type),
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            global_variables: Vec::new(),
            type_definitions: Vec::new(),
            symbol_table: SymbolTable::new(),
            data_layout: "".to_string(),
            target_triple: "".to_string(),
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn add_global_variable(&mut self, global: GlobalVariable) {
        self.global_variables.push(global);
    }

    pub fn add_type_definition(&mut self, type_def: TypeDefinition) {
        self.type_definitions.push(type_def);
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn get_global_variable(&self, name: &str) -> Option<&GlobalVariable> {
        self.global_variables.iter().find(|g| g.name == name)
    }

    pub fn get_type_definition(&self, name: &str) -> Option<&TypeDefinition> {
        self.type_definitions.iter().find(|t| t.name == name)
    }
}

impl Function {
    pub fn new(name: String, return_type: Type) -> Self {
        Self {
            name,
            parameters: Vec::new(),
            return_type,
            basic_blocks: Vec::new(),
            linkage: LinkageType::External,
            visibility: Visibility::Default,
            attributes: Vec::new(),
            calling_conv: CallingConvention::C,
            is_variadic: false,
        }
    }

    pub fn add_parameter(&mut self, name: String, ty: Type) {
        self.parameters.push(Parameter {
            name,
            ty,
            attributes: Vec::new(),
        });
    }

    pub fn add_basic_block(&mut self, block: BasicBlock) {
        self.basic_blocks.push(block);
    }

    pub fn get_basic_block(&self, name: &str) -> Option<&BasicBlock> {
        self.basic_blocks.iter().find(|b| b.name == name)
    }

    pub fn entry_block(&self) -> Option<&BasicBlock> {
        self.basic_blocks.first()
    }
}

impl BasicBlock {
    pub fn new(name: String) -> Self {
        Self {
            name,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn set_terminator(&mut self, terminator: Terminator) {
        self.terminator = terminator;
    }
}

impl Instruction {
    pub fn get_dest(&self) -> Option<&Value> {
        match self {
            Instruction::Alloca { dest, .. } => Some(dest),
            Instruction::Load { dest, .. } => Some(dest),
            Instruction::BinaryOp { dest, .. } => Some(dest),
            Instruction::UnaryOp { dest, .. } => Some(dest),
            Instruction::Call { dest, .. } => dest.as_ref(),
            Instruction::GetElementPtr { dest, .. } => Some(dest),
            Instruction::Conversion { dest, .. } => Some(dest),
            Instruction::Phi { dest, .. } => Some(dest),
            Instruction::ExtractValue { dest, .. } => Some(dest),
            Instruction::InsertValue { dest, .. } => Some(dest),
            Instruction::Select { dest, .. } => Some(dest),
            Instruction::ICmp { dest, .. } => Some(dest),
            Instruction::FCmp { dest, .. } => Some(dest),
            Instruction::VAArg { dest, .. } => Some(dest),
            Instruction::LandingPad { dest, .. } => Some(dest),
            Instruction::Store { .. } => None,
        }
    }

    pub fn get_type(&self) -> Option<Type> {
        match self {
            Instruction::Alloca { ty, .. } => Some(Type::pointer_to(ty.clone())),
            Instruction::Load { ty, .. } => Some(ty.clone()),
            Instruction::BinaryOp { dest, .. } => Some(dest.get_type()),
            Instruction::UnaryOp { dest, .. } => Some(dest.get_type()),
            Instruction::Call { dest, .. } => dest.as_ref().map(|v| v.get_type()),
            Instruction::GetElementPtr { dest, .. } => Some(dest.get_type()),
            Instruction::Conversion { dest_ty, .. } => Some(dest_ty.clone()),
            Instruction::Phi { ty, .. } => Some(ty.clone()),
            Instruction::ExtractValue { dest, .. } => Some(dest.get_type()),
            Instruction::InsertValue { dest, .. } => Some(dest.get_type()),
            Instruction::Select { dest, .. } => Some(dest.get_type()),
            Instruction::ICmp { dest, .. } => Some(dest.get_type()),
            Instruction::FCmp { dest, .. } => Some(dest.get_type()),
            Instruction::VAArg { ty, .. } => Some(ty.clone()),
            Instruction::LandingPad { ty, .. } => Some(ty.clone()),
            Instruction::Store { .. } => None,
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "; ModuleID = '{}'", self.name)?;
        if !self.target_triple.is_empty() {
            writeln!(f, "target triple = \"{}\"", self.target_triple)?;
        }
        if !self.data_layout.is_empty() {
            writeln!(f, "target datalayout = \"{}\"", self.data_layout)?;
        }

        // Print type definitions
        for type_def in &self.type_definitions {
            writeln!(f, "{}", type_def)?;
        }

        // Print global variables
        for global in &self.global_variables {
            writeln!(f, "{}", global)?;
        }

        // Print functions
        for function in &self.functions {
            writeln!(f, "{}", function)?;
        }

        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Function signature
        write!(f, "define {} {}(", self.linkage, self.return_type)?;
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} %{}", param.ty, param.name)?;
        }
        if self.is_variadic {
            if !self.parameters.is_empty() {
                write!(f, ", ")?;
            }
            write!(f, "...")?;
        }
        write!(f, ") {{")?;

        // Function attributes
        if !self.attributes.is_empty() {
            write!(f, " #{{")?;
            for (i, attr) in self.attributes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", attr)?;
            }
            write!(f, "}}")?;
        }

        writeln!(f)?;

        // Basic blocks
        for block in &self.basic_blocks {
            writeln!(f, "{}", block)?;
        }

        writeln!(f, "}}")
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;

        // Instructions
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction)?;
        }

        // Terminator
        writeln!(f, "  {}", self.terminator)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Alloca { dest, ty, align } => {
                write!(f, "{} = alloca {}", dest, ty)?;
                if let Some(a) = align {
                    write!(f, ", align {}", a)?;
                }
                Ok(())
            }
            Instruction::Load { dest, ptr, ty, align } => {
                write!(f, "{} = load {}, ptr {}", dest, ty, ptr)?;
                if let Some(a) = align {
                    write!(f, ", align {}", a)?;
                }
                Ok(())
            }
            Instruction::Store { value, ptr, align } => {
                write!(f, "store {}, ptr {}", value, ptr)?;
                if let Some(a) = align {
                    write!(f, ", align {}", a)?;
                }
                Ok(())
            }
            Instruction::BinaryOp { op, dest, left, right, flags } => {
                write!(f, "{} = {} {} {}", dest, op, left, right)?;
                for flag in flags {
                    write!(f, " {}", flag)?;
                }
                Ok(())
            }
            Instruction::UnaryOp { op, dest, operand } => {
                write!(f, "{} = {} {}", dest, op, operand)
            }
            Instruction::Call { dest, callee, arguments, calling_conv } => {
                if let Some(d) = dest {
                    write!(f, "{} = call {} {}(", d, calling_conv, callee)?;
                } else {
                    write!(f, "call {} {}(", calling_conv, callee)?;
                }
                for (i, (arg, ty)) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", ty, arg)?;
                }
                write!(f, ")")
            }
            Instruction::GetElementPtr { dest, ptr, indices, inbounds } => {
                if *inbounds {
                    write!(f, "{} = getelementptr inbounds ", dest)?;
                } else {
                    write!(f, "{} = getelementptr ", dest)?;
                }
                write!(f, "{}, ptr {}", ptr.get_type().get_pointer_element_type().unwrap_or(&Type::void()), ptr)?;
                for (idx, idx_ty) in indices {
                    write!(f, ", {} {}", idx_ty, idx)?;
                }
                Ok(())
            }
            Instruction::Conversion { op, dest, src, src_ty, dest_ty } => {
                write!(f, "{} = {} {} {} to {}", dest, op, src_ty, src, dest_ty)
            }
            Instruction::Phi { dest, ty, incoming } => {
                write!(f, "{} = phi {}", dest, ty)?;
                for (i, (value, block)) in incoming.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "[ {}, %{} ]", value, block)?;
                }
                Ok(())
            }
            Instruction::ExtractValue { dest, aggregate, indices } => {
                write!(f, "{} = extractvalue {}, ", dest, aggregate)?;
                for (i, &idx) in indices.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", idx)?;
                }
                Ok(())
            }
            Instruction::InsertValue { dest, aggregate, element, indices } => {
                write!(f, "{} = insertvalue {}, {}, ", dest, aggregate, element)?;
                for (i, &idx) in indices.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", idx)?;
                }
                Ok(())
            }
            Instruction::Select { dest, condition, true_value, false_value } => {
                write!(f, "{} = select {}, {}, {}", dest, condition, true_value, false_value)
            }
            Instruction::ICmp { dest, predicate, left, right } => {
                write!(f, "{} = icmp {} {}, {}", dest, predicate, left, right)
            }
            Instruction::FCmp { dest, predicate, left, right } => {
                write!(f, "{} = fcmp {} {}, {}", dest, predicate, left, right)
            }
            Instruction::VAArg { dest, va_list, ty } => {
                write!(f, "{} = va_arg {}, {}", dest, va_list, ty)
            }
            Instruction::LandingPad { dest, ty, clauses, cleanup } => {
                write!(f, "{} = landingpad {}", dest, ty)?;
                if *cleanup {
                    write!(f, " cleanup")?;
                }
                for clause in clauses {
                    write!(f, " {}", clause)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminator::Ret { value } => {
                if let Some(v) = value {
                    write!(f, "ret {}", v)
                } else {
                    write!(f, "ret void")
                }
            }
            Terminator::Br { dest } => {
                write!(f, "br label %{}", dest)
            }
            Terminator::CondBr { condition, true_dest, false_dest } => {
                write!(f, "br {}, label %{}, label %{}", condition, true_dest, false_dest)
            }
            Terminator::Switch { value, default_dest, cases } => {
                write!(f, "switch {}, label %{} [", value, default_dest)?;
                for (case_val, case_dest) in cases {
                    write!(f, " {}, label %{}", case_val, case_dest)?;
                }
                write!(f, " ]")
            }
            Terminator::IndirectBr { address, possible_dests } => {
                write!(f, "indirectbr {}, [", address)?;
                for dest in possible_dests {
                    write!(f, " label %{}", dest)?;
                }
                write!(f, " ]")
            }
            Terminator::Invoke { callee, arguments, normal_dest, unwind_dest, dest } => {
                if let Some(d) = dest {
                    write!(f, "{} = invoke ", d)?;
                } else {
                    write!(f, "invoke ")?;
                }
                write!(f, "{}(", callee)?;
                for (i, (arg, ty)) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", ty, arg)?;
                }
                write!(f, ") to label %{} unwind label %{}", normal_dest, unwind_dest)
            }
            Terminator::Resume { value } => {
                write!(f, "resume {}", value)
            }
            Terminator::CatchSwitch { parent_pad, catch_handlers, default_unwind_dest } => {
                write!(f, "catchswitch within {} [", parent_pad)?;
                for handler in catch_handlers {
                    write!(f, " label %{}", handler)?;
                }
                write!(f, " ]")?;
                if let Some(dest) = default_unwind_dest {
                    write!(f, " unwind label %{}", dest)?;
                }
                Ok(())
            }
            Terminator::CatchRet { from, to } => {
                write!(f, "catchret from {} to label %{}", from, to)
            }
            Terminator::CleanupRet { from, unwind_dest } => {
                write!(f, "cleanupret from {}", from)?;
                if let Some(dest) = unwind_dest {
                    write!(f, " unwind label %{}", dest)?;
                }
                Ok(())
            }
            Terminator::Unreachable => {
                write!(f, "unreachable")
            }
        }
    }
}

impl fmt::Display for GlobalVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} = ", self.linkage, self.name)?;
        if self.is_constant {
            write!(f, "constant ")?;
        } else {
            write!(f, "global ")?;
        }
        write!(f, "{}", self.ty)?;
        if let Some(init) = &self.initializer {
            write!(f, " {}", init)?;
        }
        if let Some(align) = self.alignment {
            write!(f, ", align {}", align)?;
        }
        if let Some(section) = &self.section {
            write!(f, ", section \"{}\"", section)?;
        }
        Ok(())
    }
}

impl fmt::Display for TypeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_opaque {
            write!(f, "%{} = type opaque", self.name)
        } else {
            write!(f, "%{} = type {}", self.name, self.ty)
        }
    }
}

impl fmt::Display for LinkageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkageType::External => write!(f, "external"),
            LinkageType::Private => write!(f, "private"),
            LinkageType::Internal => write!(f, "internal"),
            LinkageType::LinkOnce => write!(f, "linkonce"),
            LinkageType::Weak => write!(f, "weak"),
            LinkageType::Common => write!(f, "common"),
            LinkageType::Appending => write!(f, "appending"),
            LinkageType::ExternalWeak => write!(f, "external_weak"),
            LinkageType::LinkOnceODR => write!(f, "linkonce_odr"),
            LinkageType::WeakODR => write!(f, "weak_odr"),
            LinkageType::AvailableExternally => write!(f, "available_externally"),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Default => write!(f, "default"),
            Visibility::Hidden => write!(f, "hidden"),
            Visibility::Protected => write!(f, "protected"),
        }
    }
}

impl fmt::Display for CallingConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallingConvention::C => write!(f, "ccc"),
            CallingConvention::Fast => write!(f, "fastcc"),
            CallingConvention::Cold => write!(f, "coldcc"),
            CallingConvention::GHC => write!(f, "ghccc"),
            CallingConvention::HiPE => write!(f, "cc 10"),
            CallingConvention::WebKitJS => write!(f, "webkit_jscc"),
            CallingConvention::AnyReg => write!(f, "anyregcc"),
            CallingConvention::Win64 => write!(f, "win64cc"),
            CallingConvention::X86_64SysV => write!(f, "x86_64_sysvcc"),
        }
    }
}

impl fmt::Display for FunctionAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionAttribute::AlwaysInline => write!(f, "alwaysinline"),
            FunctionAttribute::Builtin => write!(f, "builtin"),
            FunctionAttribute::Cold => write!(f, "cold"),
            FunctionAttribute::Convergent => write!(f, "convergent"),
            FunctionAttribute::InlineHint => write!(f, "inlinehint"),
            FunctionAttribute::InAlloca => write!(f, "inalloca"),
            FunctionAttribute::Naked => write!(f, "naked"),
            FunctionAttribute::NoBuiltin => write!(f, "nobuiltin"),
            FunctionAttribute::NoDuplicate => write!(f, "noduplicate"),
            FunctionAttribute::NoImplicitFloat => write!(f, "noimplicitfloat"),
            FunctionAttribute::NoInline => write!(f, "noinline"),
            FunctionAttribute::NonLazyBind => write!(f, "nonlazybind"),
            FunctionAttribute::NoRedZone => write!(f, "noredzone"),
            FunctionAttribute::NoReturn => write!(f, "noreturn"),
            FunctionAttribute::NoUnwind => write!(f, "nounwind"),
            FunctionAttribute::OptimizeForSize => write!(f, "optsize"),
            FunctionAttribute::OptimizeNone => write!(f, "optnone"),
            FunctionAttribute::ReadNone => write!(f, "readnone"),
            FunctionAttribute::ReadOnly => write!(f, "readonly"),
            FunctionAttribute::ReturnsTwice => write!(f, "returnstwice"),
            FunctionAttribute::SExt => write!(f, "signext"),
            FunctionAttribute::StackProtect => write!(f, "ssp"),
            FunctionAttribute::StackProtectReq => write!(f, "sspreq"),
            FunctionAttribute::StackProtectStrong => write!(f, "sspstrong"),
            FunctionAttribute::SafeStack => write!(f, "safestack"),
            FunctionAttribute::SanitizeAddress => write!(f, "sanitize_address"),
            FunctionAttribute::SanitizeThread => write!(f, "sanitize_thread"),
            FunctionAttribute::SanitizeMemory => write!(f, "sanitize_memory"),
            FunctionAttribute::StrictFP => write!(f, "strictfp"),
            FunctionAttribute::UWTable => write!(f, "uwtable"),
            FunctionAttribute::ZExt => write!(f, "zeroext"),
        }
    }
}

impl fmt::Display for InstructionFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionFlag::NoUnsignedWrap => write!(f, "nuw"),
            InstructionFlag::NoSignedWrap => write!(f, "nsw"),
            InstructionFlag::Exact => write!(f, "exact"),
            InstructionFlag::FastMath => write!(f, "fast"),
            InstructionFlag::NoNaNs => write!(f, "nnan"),
            InstructionFlag::NoInfs => write!(f, "ninf"),
            InstructionFlag::NoSignedZeros => write!(f, "nsz"),
            InstructionFlag::NoReciprocal => write!(f, "arcp"),
            InstructionFlag::AllowContract => write!(f, "contract"),
            InstructionFlag::ApproxFunc => write!(f, "afn"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::FNeg => write!(f, "fneg"),
        }
    }
}

impl fmt::Display for IntPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntPredicate::EQ => write!(f, "eq"),
            IntPredicate::NE => write!(f, "ne"),
            IntPredicate::UGT => write!(f, "ugt"),
            IntPredicate::UGE => write!(f, "uge"),
            IntPredicate::ULT => write!(f, "ult"),
            IntPredicate::ULE => write!(f, "ule"),
            IntPredicate::SGT => write!(f, "sgt"),
            IntPredicate::SGE => write!(f, "sge"),
            IntPredicate::SLT => write!(f, "slt"),
            IntPredicate::SLE => write!(f, "sle"),
        }
    }
}

impl fmt::Display for FloatPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatPredicate::False => write!(f, "false"),
            FloatPredicate::OEQ => write!(f, "oeq"),
            FloatPredicate::OGT => write!(f, "ogt"),
            FloatPredicate::OGE => write!(f, "oge"),
            FloatPredicate::OLT => write!(f, "olt"),
            FloatPredicate::OLE => write!(f, "ole"),
            FloatPredicate::ONE => write!(f, "one"),
            FloatPredicate::ORD => write!(f, "ord"),
            FloatPredicate::UNO => write!(f, "uno"),
            FloatPredicate::UEQ => write!(f, "ueq"),
            FloatPredicate::UGT => write!(f, "ugt"),
            FloatPredicate::UGE => write!(f, "uge"),
            FloatPredicate::ULT => write!(f, "ult"),
            FloatPredicate::ULE => write!(f, "ule"),
            FloatPredicate::UNE => write!(f, "une"),
            FloatPredicate::True => write!(f, "true"),
        }
    }
}

impl fmt::Display for LandingPadClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LandingPadClause::Catch(ty) => write!(f, "catch {}", ty),
            LandingPadClause::Filter(ty) => write!(f, "filter {}", ty),
        }
    }
}