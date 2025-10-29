//src/ir/scope.rs

use super::{ScopeId, Value};
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a lexical or semantic scope within an intermediate representation (IR).
///
/// A `Scope` holds a collection of symbols (`HashMap<Arc<str>, Value>`) that map identifiers
/// (e.g., variable names) to their corresponding IR values. Scopes can form a hierarchy through
/// parent and child relationships, which enables nested scoping (e.g., functions, blocks, etc.).
///
/// # Fields
///
/// * `symbols` - A mapping of symbol names to their `Value`s within this scope.
/// * `parent` - The optional `ScopeId` of the parent scope (if any).
/// * `children` - A list of `ScopeId`s representing nested child scopes.
/// * `depth` - The depth level of this scope in the hierarchy (root scope = 0).
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    /// A map of symbol names to their corresponding values within the current scope.
    pub symbols: HashMap<Arc<str>, Value>,
    /// An optional reference to the parent scope, if this is not the root scope.
    pub parent: Option<ScopeId>,
    /// A list of child scope identifiers that originate from this scope.
    pub children: Vec<ScopeId>,
    /// The nesting depth of this scope relative to the global or root scope.
    pub depth: usize,
}

impl Scope {
    /// Creates a new `Scope` instance.
    ///
    /// # Parameters
    /// * `parent` - An optional `ScopeId` indicating the parent scope (or `None` if this is the root).
    /// * `depth` - The hierarchical depth of this scope (e.g., `0` for the global scope).
    ///
    /// # Returns
    /// A newly initialized `Scope` with an empty symbol table and no children.
    pub fn new(parent: Option<ScopeId>, depth: usize) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
            children: Vec::new(),
            depth,
        }
    }

    /// Inserts a new symbol and its corresponding value into the scope.
    ///
    /// If a symbol with the same name already exists, its value will be overwritten.
    ///
    /// # Parameters
    /// * `name` - The symbol name to insert, wrapped in an `Arc<str>` for efficient sharing.
    /// * `value` - The `Value` associated with the symbol.
    ///
    /// # Side Effects
    /// This operation mutates the internal `symbols` map of the scope.
    pub fn insert(&mut self, name: Arc<str>, value: Value) {
        self.symbols.insert(name, value);
    }

    /// Retrieves a reference to a symbol’s associated `Value` within the current scope.
    ///
    /// This lookup only checks the current scope; it does **not** search parent scopes.
    ///
    /// # Parameters
    /// * `name` - The name of the symbol to look up.
    ///
    /// # Returns
    /// An `Option` containing a reference to the `Value` if found, or `None` if not present.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.symbols.get(name)
    }
}
