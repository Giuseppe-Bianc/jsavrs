// src/rvir/access_control.rs
use super::types::{RResourceId, RScopeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ROperation {
    Read,
    Write,
    Execute,
    Allocate,
    Deallocate,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RAccessRules {
    read: bool,
    write: bool,
    execute: bool,
    allocate: bool,
    deallocate: bool,
}

impl RAccessRules {
    // Add a public constructor
    pub const fn new(
        read: bool,
        write: bool,
        execute: bool,
        allocate: bool,
        deallocate: bool,
    ) -> Self {
        RAccessRules {
            read,
            write,
            execute,
            allocate,
            deallocate,
        }
    }

    pub fn allows(&self, operation: ROperation) -> bool {
        match operation {
            ROperation::Read => self.read,
            ROperation::Write => self.write,
            ROperation::Execute => self.execute,
            ROperation::Allocate => self.allocate,
            ROperation::Deallocate => self.deallocate,
        }
    }
}

#[allow(dead_code)]
pub struct RAccessController {
    scopes: std::collections::HashMap<RScopeId, super::scope::RScope>,
    current_scope: RScopeId,
}

impl RAccessController {
    pub fn new(scope_manager: &super::scope_manager::RScopeManager) -> Self {
        RAccessController {
            scopes: scope_manager.get_scopes().clone(),
            current_scope: scope_manager.current_scope(),
        }
    }
    pub fn check_access(&self, _resource: RResourceId, _operation: ROperation) -> bool {
        // TODO: implement real access evaluation.
        // For safety, deny by default (e.g., require explicit allow in the current or ancestor scope).
        false
    }
}
