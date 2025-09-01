// src/rvir/value/mod.rs
pub mod constant;
pub mod debug_info;
pub mod kind;
pub mod literal;

pub use self::{
    constant::RIrConstantValue, debug_info::RValueDebugInfo, kind::RValueKind, literal::RIrLiteralValue,
};

use super::types::{RIrType, RScopeId};
use crate::location::source_span::SourceSpan;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RValueId(Uuid);

impl RValueId {
    pub fn new() -> Self {
        RValueId(Uuid::new_v4())
    }
}

impl Default for RValueId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct RValue {
    pub id: RValueId,
    pub kind: RValueKind,
    pub ty: RIrType,
    pub debug_info: Option<RValueDebugInfo>,
    pub scope: Option<RScopeId>,
}

impl RValue {
    pub fn new_literal(imm: RIrLiteralValue) -> Self {
        let ty: RIrType = (&imm).into();
        RValue {
            id: RValueId::new(),
            kind: RValueKind::Literal(imm),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_constant(imm: RIrConstantValue, ty: RIrType) -> Self {
        RValue {
            id: RValueId::new(),
            kind: RValueKind::Constant(imm),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_local(name: String, ty: RIrType) -> Self {
        RValue {
            id: RValueId::new(),
            kind: RValueKind::Local(name),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_global(name: String, ty: RIrType) -> Self {
        RValue {
            id: RValueId::new(),
            kind: RValueKind::Global(name),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn new_temporary(tmp_id: u64, ty: RIrType) -> Self {
        RValue {
            id: RValueId::new(),
            kind: RValueKind::Temporary(tmp_id),
            ty,
            debug_info: None,
            scope: None,
        }
    }

    pub fn with_debug_info(mut self, name: Option<String>, span: SourceSpan) -> Self {
        self.debug_info = Some(RValueDebugInfo {
            name,
            source_span: span,
        });
        self
    }

    pub fn with_scope(mut self, scope: RScopeId) -> Self {
        self.scope = Some(scope);
        self
    }

    /*fn next_id() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }*/
}

impl fmt::Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            RValueKind::Literal(imm) => write!(f, "{imm}")?,
            RValueKind::Constant(imm) => write!(f, "{imm}")?,
            RValueKind::Local(name) => write!(f, "%{name}")?,
            RValueKind::Global(name) => write!(f, "@{name}")?,
            RValueKind::Temporary(id) => write!(f, "t{id}")?,
        };
        if let Some(name) = self.debug_info.as_ref().and_then(|di| di.name.as_ref()) {
            write!(f, " ({name})")?;
        }
        Ok(())
    }
}