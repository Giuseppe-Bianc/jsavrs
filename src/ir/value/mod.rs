// src/ir/value/mod.rs
pub mod constant;
pub mod debug_info;
pub mod kind;
pub mod literal;

pub use self::{constant::IrConstantValue, debug_info::ValueDebugInfo, kind::ValueKind, literal::IrLiteralValue};

use super::types::{IrType, ScopeId};
use crate::location::source_span::SourceSpan;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ValueId(Uuid);

impl ValueId {
    #[inline]
    pub fn new() -> Self {
        ValueId(Uuid::new_v4())
    }
}

impl Default for ValueId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value {
    pub id: ValueId,
    pub kind: ValueKind,
    pub ty: IrType,
    pub debug_info: Option<ValueDebugInfo>,
    pub scope: Option<ScopeId>,
}

impl Value {
    // Helper function to create a new Value with common fields
    fn new_value(kind: ValueKind, ty: IrType) -> Self {
        Value { id: ValueId::new(), kind, ty, debug_info: None, scope: None }
    }

    /// Creates a new literal value.
    pub fn new_literal(imm: IrLiteralValue) -> Self {
        let ty: IrType = (&imm).into();
        Self::new_value(ValueKind::Literal(imm), ty)
    }

    /// Creates a new constant value.
    pub fn new_constant(imm: IrConstantValue, ty: IrType) -> Self {
        Self::new_value(ValueKind::Constant(imm), ty)
    }

    /// Creates a new local value.
    pub fn new_local(name: Arc<str>, ty: IrType) -> Self {
        Self::new_value(ValueKind::Local(name), ty)
    }

    /// Creates a new global value.
    pub fn new_global(name: Arc<str>, ty: IrType) -> Self {
        Self::new_value(ValueKind::Global(name), ty)
    }

    /// Creates a new temporary value.
    pub fn new_temporary(tmp_id: u64, ty: IrType) -> Self {
        Self::new_value(ValueKind::Temporary(tmp_id), ty)
    }

    /// Creates a new temporary value (alias for `new_temporary`).
    /// Convenient for test code.
    pub fn new_temp(tmp_id: u64, ty: IrType) -> Self {
        Self::new_temporary(tmp_id, ty)
    }

    pub fn with_debug_info(mut self, name: Option<Arc<str>>, span: SourceSpan) -> Self {
        self.debug_info = Some(ValueDebugInfo { name, source_span: span });
        self
    }

    pub fn with_scope(mut self, scope: ScopeId) -> Self {
        self.scope = Some(scope);
        self
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ValueKind::Literal(imm) => imm.fmt(f)?,
            ValueKind::Constant(imm) => imm.fmt(f)?,
            ValueKind::Local(name) => {
                f.write_str("%")?;
                name.fmt(f)?;
            }
            ValueKind::Global(name) => {
                f.write_str("@")?;
                name.fmt(f)?;
            }
            ValueKind::Temporary(id) => f.write_fmt(format_args!("t{id}"))?,
        }

        // Only access debug_info if it's likely to be present
        if let Some(debug_info) = &self.debug_info
            && let Some(name) = &debug_info.name
        {
            f.write_str(" (")?;
            name.fmt(f)?;
            f.write_str(")")?;
        }
        Ok(())
    }
}
