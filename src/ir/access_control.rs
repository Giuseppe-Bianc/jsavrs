// src/ir/access_control.rs
use super::types::{ResourceId, ScopeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Read,
    Write,
    Execute,
    Allocate,
    Deallocate,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AccessRules {
    read: bool,
    write: bool,
    execute: bool,
    allocate: bool,
    deallocate: bool,
}

impl AccessRules {
    // Add a public constructor
    pub const fn new(read: bool, write: bool, execute: bool, allocate: bool, deallocate: bool) -> Self {
        AccessRules { read, write, execute, allocate, deallocate }
    }

    pub const fn allows(&self, operation: Operation) -> bool {
        match operation {
            Operation::Read => self.read,
            Operation::Write => self.write,
            Operation::Execute => self.execute,
            Operation::Allocate => self.allocate,
            Operation::Deallocate => self.deallocate,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccessController {
    scopes: std::collections::HashMap<ScopeId, super::scope::Scope>,
    current_scope: ScopeId,
}

impl AccessController {
    pub fn new(scope_manager: &super::scope_manager::ScopeManager) -> Self {
        AccessController { scopes: scope_manager.get_scopes().clone(), current_scope: scope_manager.current_scope() }
    }
    pub fn check_access(&self, _resource: ResourceId, _operation: Operation) -> bool {
        // TODO: implement real access evaluation.
        // For safety, deny by default (e.g., require explicit allow in the current or ancestor scope).
        false
    }
}
