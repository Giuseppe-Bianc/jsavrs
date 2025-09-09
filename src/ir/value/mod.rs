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

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub id: ValueId,
    pub kind: ValueKind,
    pub ty: IrType,
    pub debug_info: Option<ValueDebugInfo>,
    pub scope: Option<ScopeId>,
}

impl Value {
    pub fn new_literal(imm: IrLiteralValue) -> Self {
        let ty: IrType = (&imm).into();
        Value { id: ValueId::new(), kind: ValueKind::Literal(imm), ty, debug_info: None, scope: None }
    }

    pub fn new_constant(imm: IrConstantValue, ty: IrType) -> Self {
        Value { id: ValueId::new(), kind: ValueKind::Constant(imm), ty, debug_info: None, scope: None }
    }

    pub fn new_local(name: Arc<str>, ty: IrType) -> Self {
        Value { id: ValueId::new(), kind: ValueKind::Local(name), ty, debug_info: None, scope: None }
    }

    pub fn new_global(name: Arc<str>, ty: IrType) -> Self {
        Value { id: ValueId::new(), kind: ValueKind::Global(name), ty, debug_info: None, scope: None }
    }

    pub fn new_temporary(tmp_id: u64, ty: IrType) -> Self {
        Value { id: ValueId::new(), kind: ValueKind::Temporary(tmp_id), ty, debug_info: None, scope: None }
    }

    pub fn with_debug_info(mut self, name: Option<Arc<str>>, span: SourceSpan) -> Self {
        self.debug_info = Some(ValueDebugInfo { name, source_span: span });
        self
    }

    pub fn with_scope(mut self, scope: ScopeId) -> Self {
        self.scope = Some(scope);
        self
    }

    /*fn next_id() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }*/
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ValueKind::Literal(imm) => write!(f, "{imm}")?,
            ValueKind::Constant(imm) => write!(f, "{imm}")?,
            ValueKind::Local(name) => write!(f, "%{name}")?,
            ValueKind::Global(name) => write!(f, "@{name}")?,
            ValueKind::Temporary(id) => write!(f, "t{id}")?,
        }
        if let Some(name) = self.debug_info.as_ref().and_then(|di| di.name.as_ref()) {
            write!(f, " ({name})")?;
        }
        Ok(())
    }
}
