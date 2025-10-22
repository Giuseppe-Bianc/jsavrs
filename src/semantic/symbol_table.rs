// src/semantic/symbol_table.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::{Parameter, Type};
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a symbol in the symbol table.
///
/// Symbols can be variables, functions, or type aliases. Each symbol type
/// carries specific metadata relevant to semantic analysis and code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    /// A variable symbol with associated metadata
    Variable(VariableSymbol),
    /// A function symbol with signature information
    Function(FunctionSymbol),
    /// A type alias mapping to an underlying type
    TypeAlias(Type),
}

/// Metadata for a variable symbol.
///
/// Tracks all information necessary for type checking and mutability analysis,
/// including the variable's type, mutability status, and location information
/// for error reporting.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableSymbol {
    /// The name of the variable
    pub name: Arc<str>,
    /// The type of the variable
    pub ty: Type,
    /// Whether the variable is mutable
    pub mutable: bool,
    /// Source location where the variable was defined
    pub defined_at: SourceSpan,
    /// Source location of the last assignment (if any)
    pub last_assignment: Option<SourceSpan>,
}

/// Metadata for a function symbol.
///
/// Contains the function signature including parameters and return type,
/// along with location information for error reporting.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSymbol {
    /// The name of the function
    pub name: String,
    /// The function's parameters with their types
    pub parameters: Vec<Parameter>,
    /// The return type of the function
    pub return_type: Type,
    /// Source location where the function was defined
    pub defined_at: SourceSpan,
}

/// Represents the different kinds of scopes in the program.
///
/// The scope kind determines visibility rules and what operations are valid.
/// Using `#[repr(u8)]` ensures a compact memory representation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    /// The global scope containing top-level declarations
    Global,
    /// A function scope containing parameters and local variables
    Function,
    /// A block scope (e.g., inside if, while, for, or explicit blocks)
    Block,
    // Future: Struct scope for struct member access
    //Struct,
}

/// Represents a single scope in the symbol table hierarchy.
///
/// Each scope maintains its own symbol mappings and can be nested within
/// other scopes to implement lexical scoping rules.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    /// The kind of scope (global, function, block)
    pub kind: ScopeKind,
    /// Symbols defined in this scope
    pub symbols: HashMap<String, Symbol>,
    /// Optional source location where this scope was created
    pub defined_at: Option<SourceSpan>,
}

/// The symbol table manages lexical scoping and symbol resolution.
///
/// Implements a stack of scopes to support nested lexical scoping, with
/// symbols resolved by searching from the innermost scope outward. Also
/// tracks the current function context for return type checking.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SymbolTable {
    /// Stack of scopes, with the current scope at the end
    scopes: Vec<Scope>,
    /// The currently active function (if inside a function)
    current_function: Option<FunctionSymbol>,
}

impl SymbolTable {
    /// Creates a new symbol table with a global scope.
    ///
    /// # Returns
    ///
    /// A new `SymbolTable` initialized with an empty global scope.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::semantic::symbol_table::SymbolTable;
    /// let symbol_table = SymbolTable::new();
    /// assert_eq!(symbol_table.scope_count(), 1);
    /// ```
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope { kind: ScopeKind::Global, symbols: HashMap::new(), defined_at: None }],
            current_function: None,
        }
    }

    /// Pushes a new scope onto the scope stack.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of scope to create
    /// * `defined_at` - Optional source location where the scope begins
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::semantic::symbol_table::SymbolTable;
    /// use jsavrs::semantic::symbol_table::ScopeKind;
    /// let mut table = SymbolTable::new();
    /// table.push_scope(ScopeKind::Block, None);
    /// assert_eq!(table.scope_count(), 2);
    /// ```
    pub fn push_scope(&mut self, kind: ScopeKind, defined_at: Option<SourceSpan>) {
        self.scopes.push(Scope { kind, symbols: HashMap::new(), defined_at });
    }

    /// Pops the current scope from the scope stack.
    ///
    /// The global scope is never popped to maintain invariant that at least
    /// one scope always exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::semantic::symbol_table::SymbolTable;
    /// use jsavrs::semantic::symbol_table::ScopeKind;
    /// let mut table = SymbolTable::new();
    /// table.push_scope(ScopeKind::Block, None);
    /// table.pop_scope();
    /// assert_eq!(table.scope_count(), 1);
    /// ```
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Returns the total number of scopes currently active.
    ///
    /// # Returns
    ///
    /// The count of scopes in the stack (always at least 1 for global scope).
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    /// Returns a reference to the current (innermost) scope.
    ///
    /// # Returns
    ///
    /// An optional reference to the current scope, or `None` if no scopes exist
    /// (which should never happen in practice due to the global scope invariant).
    pub fn current_scope(&self) -> Option<&Scope> {
        self.scopes.last()
    }

    /// Returns a mutable reference to the current (innermost) scope.
    ///
    /// # Returns
    ///
    /// An optional mutable reference to the current scope, or `None` if no scopes
    /// exist (which should never happen in practice).
    pub fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.scopes.last_mut()
    }

    /// Returns the kind of the current scope.
    ///
    /// # Returns
    ///
    /// An optional `ScopeKind` indicating the type of the current scope.
    pub fn current_scope_kind(&self) -> Option<ScopeKind> {
        self.current_scope().map(|s| s.kind)
    }

    /// Declares a new symbol in the current scope.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol to declare
    /// * `symbol` - The symbol metadata to associate with the name
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the symbol was successfully declared
    /// * `Err(CompileError)` if a symbol with the same name already exists in the current scope
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the identifier is already declared in the current scope,
    /// including the location of the previous declaration for error reporting.
    pub fn declare(&mut self, name: &str, symbol: Symbol) -> Result<(), CompileError> {
        let current_scope = self.current_scope_mut().expect("At least one scope");

        if current_scope.symbols.contains_key(name) {
            return Err(CompileError::TypeError {
                message: format!("Identifier '{}' already declared in this {:?} scope", name, current_scope.kind),
                span: match current_scope.symbols.get(name) {
                    Some(Symbol::Variable(v)) => v.defined_at.clone(),
                    Some(Symbol::Function(f)) => f.defined_at.clone(),
                    _ => SourceSpan::default(),
                },
                help: None,
            });
        }

        current_scope.symbols.insert(name.to_string(), symbol);
        Ok(())
    }

    /// Generic helper method to find symbols with a custom filter.
    ///
    /// Searches through scopes from innermost to outermost, applying the provided
    /// filter function to each matching symbol name. This allows for type-specific
    /// lookups while maintaining a single search implementation.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Filter function that extracts desired data from a symbol
    /// * `T` - The type of data to return
    ///
    /// # Arguments
    ///
    /// * `name` - The symbol name to search for
    /// * `filter` - Function that converts a symbol to the desired result type
    ///
    /// # Returns
    ///
    /// The first successful result from applying the filter, or `None` if not found.
    #[allow(clippy::collapsible_if)]
    fn find_symbol<F, T>(&self, name: &str, filter: F) -> Option<T>
    where
        F: Fn(&Symbol) -> Option<T>,
    {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                if let Some(result) = filter(sym) {
                    return Some(result);
                }
            }
        }
        None
    }

    /// Looks up a symbol by name, searching through all scopes.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol to look up
    ///
    /// # Returns
    ///
    /// An optional clone of the symbol if found, or `None` if not found.
    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        self.find_symbol(name, |sym| Some(sym.clone()))
    }

    /// Looks up a function symbol by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to look up
    ///
    /// # Returns
    ///
    /// An optional clone of the function symbol if found and is a function,
    /// or `None` if not found or not a function.
    pub fn lookup_function(&self, name: &str) -> Option<FunctionSymbol> {
        self.find_symbol(name, |sym| match sym {
            Symbol::Function(f) => Some(f.clone()),
            _ => None,
        })
    }

    /// Looks up a variable symbol by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to look up
    ///
    /// # Returns
    ///
    /// An optional clone of the variable symbol if found and is a variable,
    /// or `None` if not found or not a variable.
    pub fn lookup_variable(&self, name: &str) -> Option<VariableSymbol> {
        self.find_symbol(name, |sym| match sym {
            Symbol::Variable(v) => Some(v.clone()),
            _ => None,
        })
    }

    /// Sets the current function context.
    ///
    /// This is used to track which function is currently being analyzed,
    /// enabling proper return type checking.
    ///
    /// # Arguments
    ///
    /// * `func` - The function symbol to set as current
    pub fn enter_function(&mut self, func: FunctionSymbol) {
        self.current_function = Some(func);
    }

    /// Clears the current function context.
    ///
    /// Called when exiting a function scope to reset the function context.
    pub fn exit_function(&mut self) {
        self.current_function = None;
    }

    /// Returns a reference to the current function being analyzed.
    ///
    /// # Returns
    ///
    /// An optional reference to the current function symbol, or `None` if
    /// not currently inside a function.
    pub fn current_function(&self) -> Option<&FunctionSymbol> {
        self.current_function.as_ref()
    }

    /// Returns the return type of the current function.
    ///
    /// # Returns
    ///
    /// An optional clone of the current function's return type, or `None` if
    /// not currently inside a function.
    pub fn current_function_return_type(&self) -> Option<Type> {
        self.current_function().map(|f| f.return_type.clone())
    }
}