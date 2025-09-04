// src/nir/access_control.rs
use super::types::{ResourceId, ScopeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    Read,
    Write,
    Execute,
    Allocate,
    Deallocate,
}

pub struct AccessRules {
    read: bool,
    write: bool,
    execute: bool,
    allocate: bool,
    deallocate: bool,
}

impl AccessRules {
    // Add a public constructor
    pub fn new(read: bool, write: bool, execute: bool, allocate: bool, deallocate: bool) -> Self {
        AccessRules {
            read,
            write,
            execute,
            allocate,
            deallocate,
        }
    }

    pub fn allows(&self, operation: Operation) -> bool {
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
pub struct AccessController {
    scopes: std::collections::HashMap<ScopeId, super::scope::Scope>,
    current_scope: ScopeId,
}

impl AccessController {
    pub fn new(scope_manager: &super::scope_manager::ScopeManager) -> Self {
        AccessController {
            scopes: scope_manager.get_scopes(),
            current_scope: scope_manager.current_scope(),
        }
    }

    pub fn check_access(&self, _resource: ResourceId, _operation: Operation) -> bool {
        // Implementazione completa dipende dalla logica specifica
        // Esempio: verifica se il current scope o un suo parent possiede la risorsa
        true // Placeholder
    }
}