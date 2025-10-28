// src/ir/access_control.rs
use super::types::{ResourceId, ScopeId};

/// Represents different types of operations that can be performed on resources.
///
/// These operations form the basis of the access control system, allowing
/// fine-grained control over what actions can be performed on resources.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    /// Read access to a resource (e.g., loading a value).
    Read,
    /// Write access to a resource (e.g., storing a value).
    Write,
    /// Execute access to a resource (e.g., calling a function).
    Execute,
    /// Permission to allocate new resources.
    Allocate,
    /// Permission to deallocate existing resources.
    Deallocate,
}

/// Defines a set of permissions for different operations.
///
/// `AccessRules` is a lightweight structure that encodes which operations
/// are allowed. It uses boolean flags for each operation type, making it
/// efficient to check permissions at runtime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AccessRules {
    /// Whether read operations are permitted.
    read: bool,
    /// Whether write operations are permitted.
    write: bool,
    /// Whether execute operations are permitted.
    execute: bool,
    /// Whether allocation operations are permitted.
    allocate: bool,
    /// Whether deallocation operations are permitted.
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
