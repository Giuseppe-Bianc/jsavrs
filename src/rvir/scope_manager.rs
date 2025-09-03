// src/rvir/scope_manager.rs
use super::scope::RScope;
use super::types::RScopeId;
use crate::rvir::RValue;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RScopeManager {
    scopes: HashMap<RScopeId, RScope>,
    current_scope: RScopeId,
    root_scope: RScopeId,
}

impl RScopeManager {
    pub fn new() -> Self {
        let root_id = RScopeId::new();
        let root_scope = RScope::new(None, 0);

        let mut scopes = HashMap::new();
        scopes.insert(root_id, root_scope);

        RScopeManager {
            scopes,
            current_scope: root_id,
            root_scope: root_id,
        }
    }

    pub fn enter_scope(&mut self) -> RScopeId {
        let new_id = RScopeId::new();
        let depth = self.scopes[&self.current_scope].depth + 1;
        let new_scope = RScope::new(Some(self.current_scope), depth);

        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .children
            .push(new_id);

        self.scopes.insert(new_id, new_scope);
        self.current_scope = new_id;
        new_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[&self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    // Nuovo metodo pubblico per ottenere una copia della mappa degli scope
    pub fn get_scopes(&self) -> &HashMap<RScopeId, RScope> {
        &self.scopes
    }

    pub fn current_scope(&self) -> RScopeId {
        self.current_scope
    }

    pub fn add_symbol(&mut self, name: impl Into<String>, mut value: RValue) {
        value.scope = Some(self.current_scope);
        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&RValue> {
        let mut current = self.current_scope;

        loop {
            let scope = self.scopes.get(&current).expect("scope id must exist");
            if let Some(value) = scope.get(name) {
                return Some(value);
            }

            if let Some(parent) = scope.parent {
                current = parent;
            } else {
                return None;
            }
        }
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut RValue> {
        let mut current = self.current_scope;

        loop {
            if self.scopes[&current].symbols.contains_key(name) {
                return self.scopes.get_mut(&current).unwrap().symbols.get_mut(name);
            }

            if let Some(parent) = self.scopes[&current].parent {
                current = parent;
            } else {
                return None;
            }
        }
    }

    pub fn append_manager(&mut self, other: &RScopeManager) {
        let root_id = self.root_scope;

        for (scope_id, scope) in other.scopes.iter() {
            // Salta lo scope se è la root del manager da accodare
            if *scope_id == other.root_scope {
                continue;
            }

            let mut new_scope = scope.clone();

            // Se lo scope era figlio della root dell'altro manager, ora diventa figlio della root del nostro
            if let Some(parent_id) = new_scope.parent {
                if parent_id == other.root_scope {
                    new_scope.parent = Some(root_id);
                    new_scope.depth = self.scopes[&root_id].depth + 1;
                    self.scopes
                        .get_mut(&root_id)
                        .unwrap()
                        .children
                        .push(*scope_id);
                } else {
                    // Mantieni lo stesso parent per gli altri scope
                    new_scope.depth = self
                        .scopes
                        .get(&parent_id)
                        .map_or(new_scope.depth, |p| p.depth + 1);
                }
            } else {
                // Scope senza parent: lo colleghiamo alla root
                new_scope.parent = Some(root_id);
                new_scope.depth = self.scopes[&root_id].depth + 1;
                self.scopes
                    .get_mut(&root_id)
                    .unwrap()
                    .children
                    .push(*scope_id);
            }

            self.scopes.insert(*scope_id, new_scope);
        }

        // Aggiorniamo current_scope al last scope del manager accodato se esiste
        self.current_scope = other.current_scope;
    }
}

impl Default for RScopeManager {
    fn default() -> Self {
        Self::new()
    }
}
