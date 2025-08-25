// src/nir/value/mod.rs
pub mod constant;
pub mod debug_info;
pub mod kind;
pub mod literal;

pub use self::{
    constant::IrConstantValue, debug_info::ValueDebugInfo, kind::ValueKind, literal::IrLiteralValue,
};

use super::types::{IrType, ScopeId};
use crate::location::source_span::SourceSpan;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub id: u64,
    pub kind: ValueKind,
    pub ty: IrType,
    pub debug_info: Option<ValueDebugInfo>,
    pub scope: Option<ScopeId>,
}

impl Value {
    pub fn new_literal(imm: IrLiteralValue) -> Self {
        let ty: IrType = (&imm).into();
        Value {
            id: Self::next_id(),
            kind: ValueKind::Literal(imm),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_constant(imm: IrConstantValue, ty: IrType) -> Self {
        Value {
            id: Self::next_id(),
            kind: ValueKind::Constant(imm),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_local(name: String, ty: IrType) -> Self {
        Value {
            id: Self::next_id(),
            kind: ValueKind::Local(name),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_global(name: String, ty: IrType) -> Self {
        Value {
            id: Self::next_id(),
            kind: ValueKind::Global(name),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_temporary(id: u64, ty: IrType) -> Self {
        Value {
            id,
            kind: ValueKind::Temporary(id),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn with_debug_info(mut self, name: Option<String>, span: SourceSpan) -> Self {
        self.debug_info = Some(ValueDebugInfo {
            name,
            source_span: span,
        });
        self
    }

    pub fn with_scope(mut self, scope: ScopeId) -> Self {
        self.scope = Some(scope);
        self
    }

    fn next_id() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ValueKind::Literal(imm) => write!(f, "{imm}")?,
            ValueKind::Constant(imm) => write!(f, "{imm}")?,
            ValueKind::Local(name) => write!(f, "%{name}")?,
            ValueKind::Global(name) => write!(f, "@{name}")?,
            ValueKind::Temporary(id) => write!(f, "t{id}")?,
        };
        #[allow(clippy::collapsible_if)]
        if let Some(di) = &self.debug_info {
            if let Some(name) = &di.name {
                write!(f, " ({name})")?;
            }
        }
        Ok(())
    }
}