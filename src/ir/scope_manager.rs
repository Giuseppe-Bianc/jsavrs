// src/ir/scope_manager.rs

use super::scope::Scope;
use super::types::ScopeId;
use crate::ir::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Manages a hierarchy of [`Scope`]s within the Intermediate Representation (IR).
///
/// A `ScopeManager` acts as the central authority for handling nested scopes,
/// symbol insertion, and lookup operations. It keeps track of:
///
/// - A root scope (global scope)
/// - The current active scope
/// - A collection of all scopes (`HashMap<ScopeId, Scope>`)
///
/// It provides functionality for:
/// - Entering and exiting nested scopes
/// - Adding and resolving symbols
/// - Combining (`append_manager`) multiple `ScopeManager`s while maintaining
///   structural integrity and avoiding `ScopeId` collisions.
#[derive(Debug, Clone, PartialEq)]
pub struct ScopeManager {
    /// A mapping from `ScopeId` to the corresponding [`Scope`] instance.
    scopes: HashMap<ScopeId, Scope>,
    /// The identifier of the currently active scope.
    current_scope: ScopeId,
    /// The identifier of the root (global) scope.
    root_scope: ScopeId,
}

impl ScopeManager {
    /// Creates a new `ScopeManager` containing a single, empty root scope.
    ///
    /// # Returns
    /// A fully initialized [`ScopeManager`] with one root scope.
    pub fn new() -> Self {
        let root_id = ScopeId::new();
        let root_scope = Scope::new(None, 0);

        let mut scopes = HashMap::new();
        scopes.insert(root_id, root_scope);

        ScopeManager {
            scopes,
            current_scope: root_id,
            root_scope: root_id,
        }
    }

    /// Enters a new nested scope, creating and switching to it.
    ///
    /// # Returns
    /// The [`ScopeId`] of the newly created scope.
    ///
    /// # Behavior
    /// - A new scope is created with its parent set to the current scope.
    /// - The current scope becomes this new scope.
    /// - The depth is incremented relative to its parent.
    pub fn enter_scope(&mut self) -> ScopeId {
        let new_id = ScopeId::new();
        let depth = self.scopes[&self.current_scope].depth + 1;
        let new_scope = Scope::new(Some(self.current_scope), depth);

        // Register new scope as a child of the current one.
        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .children
            .push(new_id);

        self.scopes.insert(new_id, new_scope);
        self.current_scope = new_id;
        new_id
    }

    /// Exits the current scope, returning to its parent (if one exists).
    ///
    /// # Behavior
    /// - If the current scope has a parent, the active scope is changed to that parent.
    /// - If the current scope is the root, no change occurs.
    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[&self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    /// Returns a shared reference to all known scopes.
    ///
    /// # Returns
    /// A reference to the internal scope map (`HashMap<ScopeId, Scope>`).
    pub fn get_scopes(&self) -> &HashMap<ScopeId, Scope> {
        &self.scopes
    }

    /// Returns the identifier of the current active scope.
    ///
    /// # Returns
    /// The current [`ScopeId`].
    pub fn current_scope(&self) -> ScopeId {
        self.current_scope
    }

    /// Adds a new symbol to the current scope, binding it to the given value.
    ///
    /// # Parameters
    /// * `name` - The name of the symbol to add (converted into `Arc<str>`).
    /// * `value` - The [`Value`] to associate with the symbol.
    ///
    /// # Behavior
    /// - The value’s `scope` field is automatically set to the current scope.
    /// - If a symbol with the same name already exists in this scope, it is overwritten.
    pub fn add_symbol(&mut self, name: impl Into<Arc<str>>, mut value: Value) {
        value.scope = Some(self.current_scope);
        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .insert(name.into(), value);
    }

    /// Performs a hierarchical lookup for a symbol by name.
    ///
    /// Searches the current scope and recursively ascends through parent scopes
    /// until the symbol is found or the root is reached.
    ///
    /// # Parameters
    /// * `name` - The name of the symbol to resolve.
    ///
    /// # Returns
    /// An `Option<&Value>` if the symbol exists, otherwise `None`.
    pub fn lookup(&self, name: &str) -> Option<&Value> {
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

    /// Performs a mutable hierarchical lookup for a symbol by name.
    ///
    /// Similar to [`lookup`], but returns a mutable reference to allow in-place modification
    /// of the symbol’s associated value.
    ///
    /// # Parameters
    /// * `name` - The name of the symbol to look up.
    ///
    /// # Returns
    /// An `Option<&mut Value>` pointing to the symbol if found, otherwise `None`.
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Value> {
        let mut current = self.current_scope;

        loop {
            if self.scopes[&current].symbols.contains_key(name) {
                return self.scopes
                    .get_mut(&current)
                    .unwrap()
                    .symbols
                    .get_mut(name);
            }

            if let Some(parent) = self.scopes[&current].parent {
                current = parent;
            } else {
                return None;
            }
        }
    }

    /// Returns the identifier of the root scope.
    ///
    /// This method always returns `Some(root_scope)`, but is provided for
    /// interface symmetry with other scope navigation methods.
    pub fn root_scope(&self) -> Option<ScopeId> {
        Some(self.root_scope)
    }

    /// Appends another `ScopeManager`’s scopes into the current one.
    ///
    /// This operation merges two independent scope hierarchies by:
    /// - Reassigning all of the other manager’s `ScopeId`s to new ones to avoid collisions.
    /// - Linking the other manager’s root children under this manager’s root.
    /// - Maintaining depth and parent-child relationships.
    ///
    /// # Parameters
    /// * `other` - The other [`ScopeManager`] whose scopes should be merged.
    ///
    /// # Side Effects
    /// - Modifies the current manager’s `scopes` map.
    /// - May update the current active scope to match that of `other`.
    ///
    /// # Panics
    /// This method will panic if an internal scope reference is missing (should not occur under normal operation).
    pub fn append_manager(&mut self, other: &ScopeManager) {
        let root_id = self.root_scope;

        // Create a mapping from old to new IDs to prevent collisions.
        let mut id_mapping: HashMap<ScopeId, ScopeId> = HashMap::new();

        // Allocate new IDs for all scopes except the other root.
        for (scope_id, _) in other.scopes.iter() {
            if *scope_id != other.root_scope {
                id_mapping.insert(*scope_id, ScopeId::new());
            }
        }

        // Clone and remap each scope.
        for (old_scope_id, scope) in other.scopes.iter() {
            if *old_scope_id == other.root_scope {
                continue;
            }

            let new_scope_id = *id_mapping.get(old_scope_id).unwrap();
            let mut new_scope = scope.clone();

            // Remap parent IDs.
            if let Some(old_parent_id) = new_scope.parent {
                if old_parent_id == other.root_scope {
                    new_scope.parent = Some(root_id);
                    new_scope.depth = self.scopes[&root_id].depth + 1;
                } else if let Some(new_parent_id) = id_mapping.get(&old_parent_id) {
                    new_scope.parent = Some(*new_parent_id);
                    new_scope.depth = self.scopes.get(new_parent_id)
                        .map(|p| p.depth + 1)
                        .unwrap_or(self.scopes[&root_id].depth + 1);
                } else {
                    new_scope.parent = Some(root_id);
                    new_scope.depth = self.scopes[&root_id].depth + 1;
                }
            } else {
                new_scope.parent = Some(root_id);
                new_scope.depth = self.scopes[&root_id].depth + 1;
            }

            // Remap children.
            let mut new_children = Vec::new();
            for child_id in &new_scope.children {
                if let Some(new_child_id) = id_mapping.get(child_id) {
                    new_children.push(*new_child_id);
                }
            }
            new_scope.children = new_children;

            self.scopes.insert(new_scope_id, new_scope);
        }

        // Connect the merged scopes to our root.
        for (old_scope_id, scope) in other.scopes.iter() {
            if *old_scope_id == other.root_scope {
                continue;
            }

            if let Some(parent_id) = scope.parent
                && parent_id == other.root_scope
            {
                let new_scope_id = *id_mapping.get(old_scope_id).unwrap();
                self.scopes.get_mut(&root_id).unwrap().children.push(new_scope_id);
            }
        }

        // Update current scope to the other manager’s active scope.
        if other.current_scope != other.root_scope {
            if let Some(new_current_scope_id) = id_mapping.get(&other.current_scope) {
                self.current_scope = *new_current_scope_id;
            }
        }
    }
}

impl Default for ScopeManager {
    /// Creates a default [`ScopeManager`] with one empty root scope.
    fn default() -> Self {
        Self::new()
    }
}
