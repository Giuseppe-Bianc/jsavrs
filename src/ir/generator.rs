// src/ir/generator.rs
use super::ssa::SsaTransformer;
use super::*;
use crate::error::compile_error::CompileError;
use crate::location::source_span::{HasSpan, SourceSpan};
use crate::parser::ast::*;
use crate::tokens::number::Number;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

/// Error message displayed when a break statement is encountered outside of a loop context.
const BREAK_OUTSIDE_LOOP: &str = "Break outside loop";

/// Error message displayed when a continue statement is encountered outside of a loop context.
const CONTINUE_OUTSIDE_LOOP: &str = "Continue outside loop";

/// Represents control flow operations within loops (break and continue statements).
///
/// This enum is used internally to distinguish between break and continue operations
/// when handling loop control flow during IR generation.
///
/// # Variants
///
/// * `Break` - Represents a break statement that exits the current loop
/// * `Continue` - Represents a continue statement that skips to the next iteration
#[repr(u8)]
enum LoopControl {
    /// Exit the current loop immediately
    Break,
    /// Skip to the next iteration of the current loop
    Continue,
}

/// Manages control flow labels for nested loops during IR generation.
///
/// This structure maintains separate stacks for break and continue target labels,
/// allowing proper handling of nested loop constructs. When entering a loop,
/// the appropriate labels are pushed onto the stacks. When exiting a loop,
/// they are popped.
///
/// # Fields
///
/// * `break_stack` - Stack of block labels where break statements should jump
/// * `continue_stack` - Stack of block labels where continue statements should jump
///
/// # Examples
///
/// ```ignore
/// let mut stack = ControlFlowStack::new();
/// stack.break_stack.push("loop_end_1".to_string());
/// stack.continue_stack.push("loop_start_1".to_string());
/// ```
#[derive(Default)]
pub struct ControlFlowStack {
    /// Stack of block labels for break operations in nested loops
    break_stack: Vec<String>,
    /// Stack of block labels for continue operations in nested loops
    continue_stack: Vec<String>,
}

impl ControlFlowStack {
    /// Creates a new empty control flow stack with pre-allocated capacity.
    ///
    /// Pre-allocates space for 64 nested loops to reduce allocations during
    /// typical IR generation.
    ///
    /// # Returns
    ///
    /// A new `ControlFlowStack` instance with empty stacks.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let stack = ControlFlowStack::new();
    /// assert_eq!(stack.break_stack.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { break_stack: Vec::with_capacity(64), continue_stack: Vec::with_capacity(64) }
    }

    /// Clears all control flow labels from both stacks.
    ///
    /// This should be called when beginning IR generation for a new function
    /// to ensure no state leaks between functions.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut stack = ControlFlowStack::new();
    /// stack.break_stack.push("label".to_string());
    /// stack.clear();
    /// assert_eq!(stack.break_stack.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.break_stack.clear();
        self.continue_stack.clear();
    }
}

/// The IR (Intermediate Representation) Generator transforms Abstract Syntax Tree (AST) nodes
/// into intermediate representation suitable for optimization and code generation.
///
/// This generator creates a control flow graph with basic blocks, handles variable scoping,
/// type mapping, and applies SSA (Static Single Assignment) transformation to optimize
/// the generated IR for further analysis and compilation.
pub struct IrGenerator {
    /// The currently active basic block being constructed
    current_block: Option<BasicBlock>,
    /// The label of the current basic block
    current_block_label: Option<String>,
    /// Manages variable scoping and symbol lookups during IR generation
    scope_manager: ScopeManager,
    /// Counter for generating unique temporary variable IDs
    temp_counter: u64,
    /// Counter for generating unique basic block labels
    block_counter: usize,
    /// Collection of errors encountered during IR generation
    errors: Vec<CompileError>,

    control_flow_stack: ControlFlowStack,

    /// Context for type information including struct definitions and type aliases
    type_context: TypeContext,
    /// Access controller for enforcing access rules during IR generation
    //_access_controller: AccessController,
    /// The root scope ID for the generator's scope hierarchy
    root_scope: Option<ScopeId>,
    /// Whether to apply SSA transformation to generated IR
    apply_ssa: bool,
    /// Reusable string buffer for formatting to reduce allocations
    format_buffer: String,
}

/// Context for managing type information during IR generation.
///
/// Maintains mappings of struct definitions and type aliases that are referenced
/// during the IR generation process. This allows the generator to resolve custom
/// types and struct field information.
///
/// # Fields
///
/// * `structs` - Map of struct names to their field lists and source locations
/// * `aliases` - Map of type alias names to their underlying IR types
///
/// # Examples
///
/// ```ignore
/// let mut context = TypeContext::default();
/// context.structs.insert(
///     "Point".to_string(),
///     (vec![("x".to_string(), IrType::I32), ("y".to_string(), IrType::I32)], span)
/// );
/// ```
#[allow(dead_code)]
#[derive(Debug, Default)]
struct TypeContext {
    /// Map of struct names to field definitions and source spans
    structs: HashMap<String, (Vec<(String, IrType)>, SourceSpan)>,
    /// Map of type alias names to their underlying IR types
    aliases: HashMap<String, IrType>,
}

#[allow(clippy::collapsible_if, clippy::collapsible_else_if)]
impl IrGenerator {
    /// Creates a new NIR generator instance with default settings
    ///
    /// # Returns
    /// A new instance of NIrGenerator with:
    /// - Initialized scope manager
    /// - Default access controller  
    /// - Empty error collection
    /// - SSA transformation enabled by default
    pub fn new() -> Self {
        let scope_manager = ScopeManager::new();
        //let access_controller = AccessController::new(&scope_manager);
        Self {
            current_block: None,
            current_block_label: None,
            scope_manager: scope_manager.clone(),
            temp_counter: 0,
            block_counter: 0,
            errors: Vec::new(),
            control_flow_stack: ControlFlowStack::new(),
            type_context: TypeContext::default(),
            root_scope: scope_manager.root_scope(),
            apply_ssa: true,                          // Enable SSA by default
            format_buffer: String::with_capacity(64), // Pre-allocate buffer for labels
        }
    }

    /// Creates a new generator with SSA transformation disabled
    ///
    /// This is useful when you want to see the raw IR without SSA transformations applied.
    ///
    /// # Returns
    /// A new instance of NIrGenerator with SSA transformation disabled
    pub fn new_without_ssa() -> Self {
        let mut generator = Self::new();
        generator.apply_ssa = false;
        generator
    }

    /// Checks if the current basic block needs a terminator instruction.
    ///
    /// A block needs a terminator if it exists and doesn't already have a valid
    /// terminator (branch, conditional branch, or return).
    ///
    /// # Returns
    ///
    /// `true` if the current block exists and lacks a terminator, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// if generator.block_needs_terminator() {
    ///     generator.add_terminator(func, Terminator::new(/* ... */));
    /// }
    /// ```
    fn block_needs_terminator(&self) -> bool {
        self.current_block.as_ref().is_some_and(|b| !b.terminator.is_terminator())
    }

    /// Generates intermediate representation for a module of statements
    ///
    /// This is the main entry point for IR generation. It processes the AST in two passes:
    /// 1. Declaration pass: Creates function declarations and adds them to the symbol table
    /// 2. Generation pass: Generates code for function bodies and other statements
    ///
    /// # Parameters
    /// * `stmts` - Vector of AST statements to convert to IR
    /// * `module_name` - Name of the module being generated
    ///
    /// # Returns
    /// A tuple containing:
    /// * The generated Module with all functions and global variables
    /// * A vector of compilation errors encountered during generation
    pub fn generate(&mut self, stmts: Vec<Stmt>, module_name: &str) -> (Module, Vec<CompileError>) {
        let mut module = Module::new(module_name, self.root_scope);

        // First pass: create all functions and add them to the symbol table
        for stmt in &stmts {
            match stmt {
                Stmt::Function { name, parameters: _, return_type, body: _, span } => {
                    let ir_return_type = self.map_type(return_type);
                    let func_ptr_type = IrType::Pointer(Box::new(ir_return_type));
                    let func_value = Value::new_global(name.clone(), func_ptr_type)
                        .with_debug_info(Some(name.clone()), span.clone());
                    self.scope_manager.add_symbol(name.as_ref(), func_value);
                }
                Stmt::MainFunction { body: _, span } => {
                    let ir_return_type = IrType::Void;
                    let func_ptr_type = IrType::Pointer(Box::new(ir_return_type));
                    let func_value = Value::new_global("main".into(), func_ptr_type)
                        .with_debug_info(Some("main".into()), span.clone());
                    self.scope_manager.add_symbol("main", func_value);
                }
                _ => {}
            }
        }

        // Second pass: generate function bodies
        for stmt in stmts {
            match stmt {
                Stmt::Function { name, parameters, return_type, body, span } => {
                    let mut func = self.create_function(&name, &parameters, return_type, span.clone());
                    self.generate_function_body(&mut func, body, span);
                    module.add_function(func);
                }
                Stmt::MainFunction { body, span } => {
                    let mut func = self.create_function("main", &[], Type::Void, span.clone());
                    self.generate_function_body(&mut func, body, span);
                    module.add_function(func);
                }
                other => {
                    self.new_error(Arc::from("Unsupported top-level statement"), other.span().clone());
                }
            }
        }

        // Apply SSA transformation to all functions in the module if enabled
        if self.apply_ssa {
            self.apply_ssa_transformation(&mut module);
        }

        (module, std::mem::take(&mut self.errors))
    }

    /// Applies SSA transformation to all functions in the module.
    ///
    /// Static Single Assignment (SSA) form is a property of IR where each variable is assigned
    /// exactly once, and every variable is defined before it is used. This form enables
    /// more efficient optimization passes.
    ///
    /// # Parameters
    /// * `module` - The module containing functions to transform to SSA form
    fn apply_ssa_transformation(&mut self, module: &mut Module) {
        // Use a single transformer for all functions to ensure unique temporary IDs
        let mut transformer = SsaTransformer::new(Some(self.temp_counter));
        // Transform each function in the module
        for func in &mut module.functions {
            if let Err(e) = transformer.transform_function(func) {
                self.new_error(Arc::from(format!("SSA transformation failed: {}", e)), SourceSpan::default());
            }
        }
    }

    /// Adds a new compilation error to the generator's error collection
    ///
    /// # Parameters
    /// * `message` - Human-readable description of the error
    /// * `span` - Source location where the error occurred
    fn new_error(&mut self, message: Arc<str>, span: SourceSpan) {
        self.errors.push(CompileError::IrGeneratorError { message, span, help: None });
    }

    /// Adds a branch terminator to the current block if it doesn't already have one.
    ///
    /// This ensures that control flow is properly maintained when exiting blocks.
    /// If the current block already has a terminator, this method does nothing.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the block
    /// * `target_label` - The label of the target block to branch to
    /// * `span` - Source location information for debugging and error reporting
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Ensure the current block branches to the merge point
    /// generator.add_branch_if_needed(func, "merge_1", span);
    /// ```
    fn add_branch_if_needed(&mut self, func: &mut Function, target_label: &str, span: SourceSpan) {
        if self.block_needs_terminator() {
            self.add_terminator(func, Terminator::new(TerminatorKind::Branch { label: target_label.into() }, span));
        }
    }

    /// Creates a new function with mapped parameter and return types.
    ///
    /// Converts AST-level function parameters and return type to IR types,
    /// creating a fully initialized [`Function`] instance with proper attributes
    /// and source span information.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function being created
    /// * `params` - Slice of AST parameters to map to IR parameters
    /// * `return_type` - The AST return type to map to an IR type
    /// * `span` - Source location of the function declaration for debugging
    ///
    /// # Returns
    ///
    /// A new [`Function`] instance with properly mapped types and attributes.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = vec![Parameter { name: "x".into(), type_annotation: Type::I32, span }];
    /// let func = generator.create_function("add_one", &params, Type::I32, span);
    /// ```
    fn create_function(&mut self, name: &str, params: &[Parameter], return_type: Type, span: SourceSpan) -> Function {
        let ir_params = params
            .iter()
            .map(|param| {
                let ty = self.map_type(&param.type_annotation);
                IrParameter {
                    name: param.name.clone(),
                    ty: ty.clone(),
                    attributes: ParamAttributes { source_span: Some(param.span.clone()), ..Default::default() },
                }
            })
            .collect();

        let ir_return_type = self.map_type(&return_type);

        let mut func = Function::new(name, ir_params, ir_return_type);
        func.attributes.source_span = Some(span);
        func
    }

    /// Maps an AST type to its corresponding IR type representation.
    ///
    /// This function converts high-level language types to their IR equivalents,
    /// handling primitive types, arrays, vectors, custom types, and structs.
    /// Custom types are resolved using the type context if available.
    ///
    /// # Arguments
    ///
    /// * `ty` - The AST [`Type`] to map to an IR type
    ///
    /// # Returns
    ///
    /// The corresponding [`IrType`] for the given AST type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let ast_type = Type::I32;
    /// let ir_type = generator.map_type(&ast_type);
    /// assert!(matches!(ir_type, IrType::I32));
    /// ```
    fn map_type(&self, ty: &Type) -> IrType {
        match ty {
            Type::I8 => IrType::I8,
            Type::I16 => IrType::I16,
            Type::I32 => IrType::I32,
            Type::I64 => IrType::I64,
            Type::U8 => IrType::U8,
            Type::U16 => IrType::U16,
            Type::U32 => IrType::U32,
            Type::U64 => IrType::U64,
            Type::F32 => IrType::F32,
            Type::F64 => IrType::F64,
            Type::Char => IrType::Char,
            Type::String => IrType::String,
            Type::Bool => IrType::Bool,
            Type::Custom(name) => {
                if let Some((fields, span)) = self.type_context.structs.get(name.as_ref()) {
                    IrType::Struct(name.clone(), fields.clone(), span.clone())
                } else {
                    IrType::Custom(name.clone(), SourceSpan::default())
                }
            }
            Type::Array(element_type, size_expr) => {
                let mapped_element = self.map_type(element_type);
                if let Expr::Literal { value: LiteralValue::Number(Number::Integer(size)), .. } = **size_expr {
                    IrType::Array(Box::new(mapped_element), size as usize)
                } else {
                    IrType::Pointer(Box::new(mapped_element))
                }
            }
            Type::Vector(element_type) => IrType::Pointer(Box::new(self.map_type(element_type))),
            Type::Void => IrType::Void,
            Type::NullPtr => IrType::Pointer(Box::new(IrType::I8)),
        }
    }

    /// Finalizes the current basic block and transfers it to the function's CFG.
    ///
    /// Takes the current block being constructed, transfers its instructions and
    /// terminator to the corresponding block in the function's control flow graph,
    /// and updates the current block label.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the control flow graph
    ///
    /// # Examples
    ///
    /// ```ignore
    /// generator.finalize_current_block(func);
    /// // Current block is now committed to the CFG
    /// ```
    fn finalize_current_block(&mut self, func: &mut Function) {
        if let Some(mut current_block) = self.current_block.take() {
            let label = current_block.label.clone();

            // Get the block in the CFG and update its instructions and terminator
            if let Some(cfg_block) = func.cfg.get_block_mut(&label) {
                // Transfer instructions and terminator to the CFG block
                cfg_block.instructions = std::mem::take(&mut current_block.instructions);
                cfg_block.terminator = current_block.terminator.clone();
                cfg_block.scope = current_block.scope;
            }

            self.current_block_label = Some(label.to_string());
        }
    }

    /// Establishes all control flow edges between basic blocks in the CFG.
    ///
    /// After all blocks have been finalized, this method connects them based on
    /// their terminator instructions. It collects all necessary connections first
    /// to avoid borrow checker issues, then applies them to the function's CFG.
    ///
    /// # Arguments
    ///
    /// * `func` - The function whose blocks need to be connected
    ///
    /// # Examples
    ///
    /// ```ignore
    /// generator.finalize_block_connections(func);
    /// // All blocks are now properly connected in the CFG
    /// ```
    fn finalize_block_connections(&mut self, func: &mut Function) {
        // First, collect all the connections we need to make
        let mut connections = Vec::new();
        for block in func.cfg.blocks() {
            let label = block.label.clone();
            for target in block.terminator.get_targets() {
                connections.push((label.clone(), target));
            }
        }

        // Now apply all the connections without holding the immutable borrow
        for (from_label, to_label) in connections {
            func.connect_blocks(&from_label, &to_label);
        }
    }
    /// Generates IR code for a complete function body.
    ///
    /// Creates the function's entry block, processes all function parameters,
    /// generates code for each statement in the body, ensures proper termination,
    /// and finalizes all blocks and their connections.
    ///
    /// # Arguments
    ///
    /// * `func` - The function being generated
    /// * `body` - Vector of statements comprising the function body
    /// * `span` - Source span of the function for debugging
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let statements = vec![/* AST statements */];
    /// generator.generate_function_body(&mut func, statements, span);
    /// ```
    fn generate_function_body(&mut self, func: &mut Function, body: Vec<Stmt>, span: SourceSpan) {
        self.control_flow_stack.clear();

        // Save the current generator scope manager
        let saved_scope_manager = self.scope_manager.clone();

        // Establish function scope before creating the entry block
        func.enter_scope();

        // Create the entry block using start_block
        self.format_buffer.clear();
        self.format_buffer.push_str("entry_");
        self.format_buffer.push_str(&func.name);
        let entry_label = self.format_buffer.clone();
        self.start_block(func, &entry_label, span.clone());

        // Add function parameters to symbol table
        for param in &func.parameters {
            let value = Value::new_local(param.name.clone(), param.ty.clone())
                .with_debug_info(Some(param.name.clone()), param.attributes.source_span.clone().unwrap_or_default());
            self.scope_manager.add_symbol(param.name.clone(), value.clone());
        }

        // Process all statements
        for stmt in body {
            self.generate_stmt(func, stmt);
        }

        // Ensure the last block has a terminator if needed
        if let Some(block) = &self.current_block {
            if matches!(block.terminator().kind, TerminatorKind::Unreachable) {
                let return_value = match func.return_type {
                    IrType::Void => Value::new_literal(IrLiteralValue::I32(0)),
                    _ => Value::new_literal(IrLiteralValue::I32(0)),
                };
                self.add_terminator(
                    func,
                    Terminator::new(
                        TerminatorKind::Return { value: return_value, ty: func.return_type.clone() },
                        SourceSpan::default(),
                    ),
                );
            }
        }

        // Finalize the last block
        self.finalize_current_block(func);

        // Now connect all blocks
        self.finalize_block_connections(func);

        // Update the function's scope manager with the scopes we created during generation
        func.scope_manager = self.scope_manager.clone();

        // Restore the generator's scope manager and append the function's scope manager
        self.scope_manager = saved_scope_manager;

        func.exit_scope();
        self.scope_manager.append_manager(&func.scope_manager);
    }

    /// Generates IR code for a single statement.
    ///
    /// Dispatches to specialized generation methods based on the statement type.
    /// Handles expressions, variable declarations, returns, blocks, control flow
    /// (if/while/for), and loop control statements (break/continue).
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing this statement
    /// * `stmt` - The AST statement to generate IR for
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for stmt in body_statements {
    ///     generator.generate_stmt(&mut func, stmt);
    /// }
    /// ```
    fn generate_stmt(&mut self, func: &mut Function, stmt: Stmt) {
        match stmt {
            Stmt::Expression { expr } => {
                self.generate_expr(func, expr);
            }
            Stmt::VarDeclaration { variables, type_annotation, initializers, span, is_mutable } => {
                self.generate_var_declaration(func, variables, type_annotation, initializers, is_mutable, span);
            }
            Stmt::Return { value, span } => {
                self.generate_return(func, value, span);
            }
            Stmt::Block { statements, span: _ } => {
                self.scope_manager.enter_scope();
                for stmt in statements {
                    self.generate_stmt(func, stmt);
                }
                self.scope_manager.exit_scope();
            }
            Stmt::If { condition, then_branch, else_branch, span } => {
                self.generate_if(func, condition, then_branch, else_branch, span);
            }
            Stmt::While { condition, body, span } => {
                self.generate_while(func, condition, body, span);
            }
            Stmt::For { initializer, condition, increment, body, span } => {
                self.scope_manager.enter_scope();
                self.generate_for(func, initializer, condition, increment, body, span);
                self.scope_manager.exit_scope();
            }
            Stmt::Break { span } => {
                self.handle_loop_control(func, span, LoopControl::Break);
            }
            Stmt::Continue { span } => {
                self.handle_loop_control(func, span, LoopControl::Continue);
            }
            other => self.new_error(Arc::from(format!("Unsupported statement: {:?}", other)), other.span().clone()),
        }
    }

    /// Generates IR code for variable declarations.
    ///
    /// Handles both mutable and immutable variable declarations. Mutable variables
    /// are allocated on the stack using alloca, while immutable variables are stored
    /// directly in the symbol table. Supports multiple variable declarations with
    /// optional initializers.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the declaration
    /// * `variables` - Names of the variables being declared
    /// * `type_annotation` - The declared type for all variables
    /// * `initializers` - Optional initial values for each variable
    /// * `is_mutable` - Whether the variables are mutable
    /// * `span` - Source location for error reporting
    ///
    /// # Errors
    ///
    /// Generates an error if an immutable variable lacks an initializer.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // let mut x: i32 = 42;
    /// generator.generate_var_declaration(
    ///     func,
    ///     vec!["x".into()],
    ///     Type::I32,
    ///     vec![expr_42],
    ///     true,
    ///     span
    /// );
    /// ```
    fn generate_var_declaration(
        &mut self, func: &mut Function, variables: Vec<Arc<str>>, type_annotation: Type, initializers: Vec<Expr>,
        is_mutable: bool, span: SourceSpan,
    ) {
        let ty: IrType = self.map_type(&type_annotation);

        for (i, var) in variables.iter().enumerate() {
            if is_mutable {
                let temp_id = self.new_temp();
                let ptr_ty = IrType::Pointer(Box::new(ty.clone()));
                let ptr_value = Value::new_temporary(temp_id, ptr_ty).with_debug_info(Some(var.clone()), span.clone());

                let alloca_inst = Instruction::new(InstructionKind::Alloca { ty: ty.clone() }, span.clone())
                    .with_result(ptr_value.clone());

                self.add_instruction(alloca_inst);

                if let Some(init) = initializers.get(i) {
                    let value_val = self.generate_expr(func, init.clone());
                    let store_inst = Instruction::new(
                        InstructionKind::Store { value: value_val, dest: ptr_value.clone() },
                        span.clone(),
                    );
                    self.add_instruction(store_inst);
                }

                self.scope_manager.add_symbol(var.clone(), ptr_value);
            } else {
                if let Some(init) = initializers.get(i) {
                    let value = self.generate_expr(func, init.clone());
                    self.scope_manager.add_symbol(var.clone(), value.with_debug_info(Some(var.clone()), span.clone()));
                } else {
                    self.new_error(Arc::from(format!("Constant '{var}' must be initialized")), span.clone());
                }
            }
        }
    }

    /// Generates IR code for a return statement.
    ///
    /// Creates a return terminator with the specified value (or a default value
    /// if none provided) and adds it to the current basic block.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the return statement
    /// * `value` - Optional expression to return
    /// * `span` - Source location for debugging
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // return 42;
    /// generator.generate_return(func, Some(expr_42), span);
    /// // return;
    /// generator.generate_return(func, None, span);
    /// ```
    fn generate_return(&mut self, func: &mut Function, value: Option<Expr>, span: SourceSpan) {
        let return_value =
            value.map_or_else(|| Value::new_literal(IrLiteralValue::I32(0)), |expr| self.generate_expr(func, expr));

        self.add_terminator(
            func,
            Terminator::new(TerminatorKind::Return { value: return_value, ty: func.return_type.clone() }, span),
        );
    }

    /// Generates IR code for an if-else conditional statement.
    ///
    /// Creates a control flow graph with separate blocks for the condition,
    /// then branch, else branch, and merge point. Ensures all paths properly
    /// converge at the merge block.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the if statement
    /// * `condition` - The boolean condition expression
    /// * `then_branch` - Statements to execute if condition is true
    /// * `else_branch` - Optional statements to execute if condition is false
    /// * `span` - Source location for debugging
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // if (x > 0) { ... } else { ... }
    /// generator.generate_if(func, condition, then_stmts, Some(else_stmts), span);
    /// ```
    fn generate_if(
        &mut self, func: &mut Function, condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>>,
        span: SourceSpan,
    ) {
        let cond_value = self.generate_expr(func, condition);

        let then_label = self.new_block_label("then");
        let else_label = self.new_block_label("else");
        let merge_label = self.new_block_label("merge");

        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: then_label.clone().into(),
                    false_label: else_label.clone().into(),
                },
                span.clone(),
            ),
        );

        self.start_block(func, &then_label, span.clone());
        self.scope_manager.enter_scope();
        for stmt in then_branch {
            self.generate_stmt(func, stmt);
        }
        self.scope_manager.exit_scope();
        self.add_branch_if_needed(func, &merge_label, span.clone());

        self.start_block(func, &else_label, span.clone());
        if let Some(else_stmts) = else_branch {
            self.scope_manager.enter_scope();
            for stmt in else_stmts {
                self.generate_stmt(func, stmt);
            }
            self.scope_manager.exit_scope();
        }

        self.add_branch_if_needed(func, &merge_label, span.clone());

        self.start_block(func, &merge_label, span);
    }

    /// Generates IR code for a while loop.
    ///
    /// Creates a control flow graph with separate blocks for the loop condition,
    /// loop body, and exit point. Manages break/continue labels on the control
    /// flow stack for nested loop support.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the while loop
    /// * `condition` - The loop condition expression
    /// * `body` - Statements comprising the loop body
    /// * `span` - Source location for debugging
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // while (i < 10) { ... }
    /// generator.generate_while(func, condition, body_stmts, span);
    /// ```
    fn generate_while(&mut self, func: &mut Function, condition: Expr, body: Vec<Stmt>, span: SourceSpan) {
        let loop_start_label = self.new_block_label("loop_start");
        let loop_body_label = self.new_block_label("loop_body");
        let loop_end_label = self.new_block_label("loop_end");

        self.add_terminator(
            func,
            Terminator::new(TerminatorKind::Branch { label: loop_start_label.clone().into() }, span.clone()),
        );

        self.start_block(func, &loop_start_label, span.clone());
        let cond_value = self.generate_expr(func, condition);
        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: loop_body_label.clone().into(),
                    false_label: loop_end_label.clone().into(),
                },
                span.clone(),
            ),
        );

        self.control_flow_stack.break_stack.push(loop_end_label.clone());
        self.control_flow_stack.continue_stack.push(loop_start_label.clone());

        self.start_block(func, &loop_body_label, span.clone());
        self.scope_manager.enter_scope();
        for stmt in body {
            self.generate_stmt(func, stmt);
        }
        self.scope_manager.exit_scope();

        self.control_flow_stack.break_stack.pop();
        self.control_flow_stack.continue_stack.pop();

        self.add_branch_if_needed(func, &loop_start_label, span.clone());
        self.start_block(func, &loop_end_label, span);
    }

    /// Generates IR code for a for loop.
    ///
    /// Creates a control flow graph with blocks for the loop start (condition check),
    /// loop body, increment, and exit. Handles optional initializer, condition, and
    /// increment expressions. Manages break/continue labels for nested loop support.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the for loop
    /// * `initializer` - Optional initialization statement
    /// * `condition` - Optional loop condition (defaults to true if absent)
    /// * `increment` - Optional increment expression
    /// * `body` - Statements comprising the loop body
    /// * `span` - Source location for debugging
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // for (let i = 0; i < 10; i++) { ... }
    /// generator.generate_for(func, Some(init), Some(cond), Some(inc), body, span);
    /// ```
    fn generate_for(
        &mut self, func: &mut Function, initializer: Option<Box<Stmt>>, condition: Option<Expr>,
        increment: Option<Expr>, body: Vec<Stmt>, span: SourceSpan,
    ) {
        let loop_st_label = self.new_block_label("for_start");
        let loop_bd_label = self.new_block_label("for_body");
        let loop_inc_label = self.new_block_label("for_inc");
        let loop_end_label = self.new_block_label("for_end");

        if let Some(init) = initializer {
            self.generate_stmt(func, *init);
        }

        self.add_terminator(
            func,
            Terminator::new(TerminatorKind::Branch { label: loop_st_label.clone().into() }, span.clone()),
        );

        self.start_block(func, &loop_st_label, span.clone());

        let cond_value = if let Some(cond) = condition {
            self.generate_expr(func, cond)
        } else {
            Value::new_literal(IrLiteralValue::Bool(true))
        };

        self.add_terminator(
            func,
            Terminator::new(
                TerminatorKind::ConditionalBranch {
                    condition: cond_value,
                    true_label: loop_bd_label.clone().into(),
                    false_label: loop_end_label.clone().into(),
                },
                span.clone(),
            ),
        );

        self.control_flow_stack.break_stack.push(loop_end_label.clone());
        self.control_flow_stack.continue_stack.push(loop_inc_label.clone());

        self.start_block(func, &loop_bd_label, span.clone());
        self.scope_manager.enter_scope();
        for stmt in body {
            self.generate_stmt(func, stmt);
        }
        self.scope_manager.exit_scope();
        self.control_flow_stack.break_stack.pop();
        self.control_flow_stack.continue_stack.pop();

        self.add_branch_if_needed(func, &loop_inc_label, span.clone());

        self.start_block(func, &loop_inc_label, span.clone());
        if let Some(inc) = increment {
            self.generate_expr(func, inc);
        }

        self.add_branch_if_needed(func, &loop_st_label, span.clone());
        self.start_block(func, &loop_end_label, span);
    }

    /// Handles break and continue statements within loops.
    ///
    /// Generates a branch to the appropriate target label based on the control
    /// flow stack. Generates an error if the statement appears outside a loop.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the control statement
    /// * `span` - Source location for error reporting
    /// * `control` - Whether this is a break or continue statement
    ///
    /// # Errors
    ///
    /// Generates an error if break/continue appears outside a loop context.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// generator.handle_loop_control(func, span, LoopControl::Break);
    /// ```
    fn handle_loop_control(&mut self, func: &mut Function, span: SourceSpan, control: LoopControl) {
        let (stack, message) = match control {
            LoopControl::Break => (&self.control_flow_stack.break_stack, BREAK_OUTSIDE_LOOP),
            LoopControl::Continue => (&self.control_flow_stack.continue_stack, CONTINUE_OUTSIDE_LOOP),
        };

        if let Some(label) = stack.last() {
            self.add_terminator(func, Terminator::new(TerminatorKind::Branch { label: label.clone().into() }, span));
        } else {
            self.new_error(Arc::from(message), span);
        }
    }

    /// Generates IR code for an expression.
    ///
    /// Dispatches to specialized generation methods based on the expression type.
    /// Handles literals, binary operations, unary operations, variables, assignments,
    /// array operations, and function calls.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the expression
    /// * `expr` - The AST expression to generate IR for
    ///
    /// # Returns
    ///
    /// A [`Value`] representing the result of evaluating the expression.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let value = generator.generate_expr(func, expr);
    /// // Use value in subsequent IR instructions
    /// ```
    #[allow(unreachable_patterns)] // To handle any unexpected Expr variants 
    fn generate_expr(&mut self, func: &mut Function, expr: Expr) -> Value {
        match expr {
            Expr::Literal { value, span } => self.generate_literal(value, span),
            Expr::Binary { left, op, right, span } => self.generate_binary(func, *left, op, *right, span),
            Expr::Unary { op, expr, span } => self.generate_unary(func, op, *expr, span),
            Expr::Variable { name, span } => self.generate_variable(name, span),
            Expr::Assign { target, value, span } => self.generate_assign(func, *target, *value, span),
            Expr::Grouping { expr, span: _ } => self.generate_expr(func, *expr),
            Expr::ArrayLiteral { elements, span } => self.generate_array_literal(func, elements, span),
            Expr::ArrayAccess { array, index, span } => self.generate_array_access(func, *array, *index, span),
            Expr::Call { callee, arguments, span } => self.generate_call(func, *callee, arguments, span),
            other => {
                self.new_error(Arc::from("Unsupported expression type"), other.span().clone());
                Value::new_literal(IrLiteralValue::I32(0))
            }
        }
    }

    /// Generates IR code for array element access.
    ///
    /// Calculates the element address using GetElementPtr (GEP) instruction,
    /// then loads the value from that address. Handles both direct arrays and
    /// pointers to arrays.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the array access
    /// * `array` - Expression evaluating to the array base
    /// * `index` - Expression evaluating to the array index
    /// * `span` - Source location for error reporting
    ///
    /// # Returns
    ///
    /// A [`Value`] containing the loaded element value.
    ///
    /// # Errors
    ///
    /// Generates an error if the array expression is not an array type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // arr[5]
    /// let value = generator.generate_array_access(func, arr_expr, index_expr, span);
    /// ```
    fn generate_array_access(&mut self, func: &mut Function, array: Expr, index: Expr, span: SourceSpan) -> Value {
        let base_val = self.generate_expr(func, array);
        let index_val = self.generate_expr(func, index);

        // Determine element type: handle both pointer to array and direct array
        let element_ty = match &base_val.ty {
            IrType::Pointer(inner) => match inner.as_ref() {
                IrType::Array(elem_ty, _) => *elem_ty.clone(),
                other => other.clone(), // fallback: pointer to already pointed element
            },
            IrType::Array(elem_ty, _) => *elem_ty.clone(),
            other => {
                // Unexpected case: report but continue with safe fallback (i32)
                self.new_error(Arc::from(format!("Array access on non-array type: {other}")), span.clone());
                IrType::I32
            }
        };

        let tmp = self.new_temp();
        let gep = Instruction::new(
            InstructionKind::GetElementPtr { base: base_val, index: index_val, element_ty: element_ty.clone() },
            span.clone(),
        )
        .with_result(Value::new_temporary(tmp, IrType::Pointer(Box::new(element_ty.clone()))));

        self.add_instruction(gep.clone());
        let ptr_value = gep.result.unwrap();

        // Load the value from the pointer for use in expressions
        let load_tmp = self.new_temp();
        let load_inst =
            Instruction::new(InstructionKind::Load { src: ptr_value, ty: element_ty.clone() }, span.clone())
                .with_result(Value::new_temporary(load_tmp, element_ty));

        self.add_instruction(load_inst.clone());
        load_inst.result.unwrap()
    }

    /// Generates IR code for an array literal.
    ///
    /// Allocates space for the array on the stack, generates code for each element,
    /// and stores the elements at the appropriate indices using GetElementPtr and
    /// Store instructions.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the array literal
    /// * `elements` - Vector of expressions for each array element
    /// * `span` - Source location for debugging
    ///
    /// # Returns
    ///
    /// A [`Value`] representing a pointer to the allocated array.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // [1, 2, 3, 4, 5]
    /// let array_ptr = generator.generate_array_literal(func, elements, span);
    /// ```
    fn generate_array_literal(&mut self, func: &mut Function, elements: Vec<Expr>, span: SourceSpan) -> Value {
        if elements.is_empty() {
            return Value::new_literal(IrLiteralValue::I64(0)); // Null pointer
        }

        let mut element_vals = Vec::with_capacity(elements.len());
        for element in elements {
            element_vals.push(self.generate_expr(func, element));
        }

        let element_ty = element_vals[0].ty.clone();
        let array_size = element_vals.len();
        let array_temp = self.new_temp();
        let array_ty = IrType::Array(Box::new(element_ty.clone()), array_size);

        let alloca_inst = Instruction::new(InstructionKind::Alloca { ty: array_ty.clone() }, span.clone())
            .with_result(Value::new_temporary(array_temp, array_ty.clone()));

        self.add_instruction(alloca_inst.clone());
        let array_ptr = alloca_inst.result.unwrap();

        for (index, element_val) in element_vals.into_iter().enumerate() {
            let index_temp = self.new_temp();
            let index_val = Value::new_literal(IrLiteralValue::I32(index as i32));

            let gep_inst = Instruction::new(
                InstructionKind::GetElementPtr {
                    base: array_ptr.clone(),
                    index: index_val,
                    element_ty: element_ty.clone(),
                },
                span.clone(),
            )
            .with_result(Value::new_temporary(index_temp, IrType::Pointer(Box::new(element_ty.clone()))));
            self.add_instruction(gep_inst.clone());

            let element_ptr = gep_inst.result.unwrap();

            let store_inst =
                Instruction::new(InstructionKind::Store { value: element_val, dest: element_ptr }, span.clone());
            self.add_instruction(store_inst);
        }

        array_ptr
    }

    /// Generates IR code for a literal value.
    ///
    /// Converts AST literal values to IR literal values, handling all numeric types,
    /// booleans, strings, characters, and null pointers. Scientific notation is
    /// evaluated to standard floating point values.
    ///
    /// # Arguments
    ///
    /// * `value` - The AST literal value
    /// * `span` - Source location for debugging
    ///
    /// # Returns
    ///
    /// A [`Value`] containing the IR representation of the literal.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let value = generator.generate_literal(LiteralValue::Number(Number::I32(42)), span);
    /// ```
    fn generate_literal(&mut self, value: LiteralValue, span: SourceSpan) -> Value {
        match value {
            LiteralValue::Number(num) => match num {
                Number::I8(i) => Value::new_literal(IrLiteralValue::I8(i)).with_debug_info(None, span),
                Number::I16(i) => Value::new_literal(IrLiteralValue::I16(i)).with_debug_info(None, span),
                Number::I32(i) => Value::new_literal(IrLiteralValue::I32(i)).with_debug_info(None, span),
                Number::Integer(i) => Value::new_literal(IrLiteralValue::I64(i)).with_debug_info(None, span),
                Number::U8(u) => Value::new_literal(IrLiteralValue::U8(u)).with_debug_info(None, span),
                Number::U16(u) => Value::new_literal(IrLiteralValue::U16(u)).with_debug_info(None, span),
                Number::U32(u) => Value::new_literal(IrLiteralValue::U32(u)).with_debug_info(None, span),
                Number::UnsignedInteger(u) => Value::new_literal(IrLiteralValue::U64(u)).with_debug_info(None, span),
                Number::Float32(f) => Value::new_literal(IrLiteralValue::F32(f)).with_debug_info(None, span),
                Number::Float64(f) => Value::new_literal(IrLiteralValue::F64(f)).with_debug_info(None, span),
                Number::Scientific32(f, i) => {
                    let value = f.powi(i);
                    Value::new_literal(IrLiteralValue::F32(value)).with_debug_info(None, span)
                }
                Number::Scientific64(f, i) => {
                    let value = f.powi(i);
                    Value::new_literal(IrLiteralValue::F64(value)).with_debug_info(None, span)
                }
            },
            LiteralValue::Bool(b) => Value::new_literal(IrLiteralValue::Bool(b)).with_debug_info(None, span),
            LiteralValue::StringLit(s) => {
                Value::new_constant(IrConstantValue::String { string: s }, IrType::String).with_debug_info(None, span)
            }
            LiteralValue::CharLit(c) => {
                Value::new_literal(IrLiteralValue::Char(c.chars().next().unwrap_or('\0'))).with_debug_info(None, span)
            }
            LiteralValue::Nullptr => Value::new_literal(IrLiteralValue::I64(0)).with_debug_info(None, span),
        }
    }

    /// Generates IR code for a binary operation.
    ///
    /// Evaluates both operands, applies type promotion if necessary, and creates
    /// a binary instruction with the promoted operands. The type promotion engine
    /// ensures operands have compatible types for the operation.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the binary operation
    /// * `left` - The left-hand operand expression
    /// * `op` - The binary operator
    /// * `right` - The right-hand operand expression
    /// * `span` - Source location for debugging
    ///
    /// # Returns
    ///
    /// A [`Value`] containing the result of the binary operation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // x + y
    /// let result = generator.generate_binary(func, x_expr, BinaryOp::Add, y_expr, span);
    /// ```
    fn generate_binary(
        &mut self, func: &mut Function, left: Expr, op: BinaryOp, right: Expr, span: SourceSpan,
    ) -> Value {
        let ir_op: IrBinaryOp = op.into();
        let left_val = self.generate_expr(func, left);
        let right_val = self.generate_expr(func, right);

        // Initialize type promotion engine
        let promotion_engine = TypePromotionEngine::new();

        // Analyze the binary operation for type promotion
        let promotion_result =
            promotion_engine.analyze_binary_promotion(&left_val.ty, &right_val.ty, ir_op, span.clone());

        // Insert promotion casts if needed
        let (promoted_left_val, promoted_right_val) =
            promotion_engine.insert_promotion_casts(self, func, left_val, right_val, &promotion_result, span.clone());

        // Use the result type from promotion analysis
        let result_ty = promotion_result.result_type;
        let dest_id = self.new_temp();

        let bin_inst = Instruction::new(
            InstructionKind::Binary {
                op: ir_op,
                left: promoted_left_val,
                right: promoted_right_val,
                ty: result_ty.clone(),
            },
            span.clone(),
        )
        .with_result(Value::new_temporary(dest_id, result_ty.clone()));

        self.add_instruction(bin_inst.clone());
        bin_inst.result.unwrap()
    }

    /// Generates IR code for a unary operation.
    ///
    /// Evaluates the operand expression and creates a unary instruction with
    /// the appropriate operator (negation, logical not, etc.).
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the unary operation
    /// * `op` - The unary operator
    /// * `expr` - The operand expression
    /// * `span` - Source location for debugging
    ///
    /// # Returns
    ///
    /// A [`Value`] containing the result of the unary operation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // -x
    /// let result = generator.generate_unary(func, UnaryOp::Neg, x_expr, span);
    /// ```
    fn generate_unary(&mut self, func: &mut Function, op: UnaryOp, expr: Expr, span: SourceSpan) -> Value {
        let ir_op: IrUnaryOp = op.into();
        let operand = self.generate_expr(func, expr);
        let ty = operand.ty.clone();
        let dest_id = self.new_temp();

        let unary_inst = Instruction::new(InstructionKind::Unary { op: ir_op, operand, ty: ty.clone() }, span.clone())
            .with_result(Value::new_temporary(dest_id, ty.clone()));

        self.add_instruction(unary_inst.clone());
        unary_inst.result.unwrap()
    }

    /// Generates IR code for a variable reference.
    ///
    /// Looks up the variable in the current scope and returns its value.
    /// Generates an error if the variable is not found.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to look up
    /// * `span` - Source location for error reporting
    ///
    /// # Returns
    ///
    /// A [`Value`] representing the variable's current value.
    ///
    /// # Errors
    ///
    /// Generates an error if the variable is undefined in the current scope.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let var_value = generator.generate_variable("x".into(), span);
    /// ```
    fn generate_variable(&mut self, name: Arc<str>, span: SourceSpan) -> Value {
        self.scope_manager.lookup(&name).cloned().unwrap_or_else(|| {
            self.new_error(Arc::from(format!("Undefined variable '{name}'")), span.clone());
            Value::new_literal(IrLiteralValue::I32(0)).with_debug_info(None, span)
        })
    }

    /// Generates IR code for an assignment expression.
    ///
    /// Evaluates the target expression (variable or array element), evaluates
    /// the value expression, and creates a store instruction. Handles both
    /// simple variable assignments and array element assignments.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the assignment
    /// * `target` - The assignment target (variable or array access)
    /// * `value` - The value expression to assign
    /// * `span` - Source location for debugging
    ///
    /// # Returns
    ///
    /// A [`Value`] containing the assigned value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // x = 42
    /// let value = generator.generate_assign(func, x_expr, value_expr, span);
    /// ```
    fn generate_assign(&mut self, func: &mut Function, target: Expr, value: Expr, span: SourceSpan) -> Value {
        let target_val = match target {
            Expr::ArrayAccess { array, index, span: access_span } => {
                self.generate_array_access_target(func, *array, *index, access_span)
            }
            _ => self.generate_expr(func, target),
        };

        let value_val = self.generate_expr(func, value);

        let store_inst =
            Instruction::new(InstructionKind::Store { value: value_val.clone(), dest: target_val }, span.clone());
        self.add_instruction(store_inst);

        value_val
    }

    /// Generates a unique temporary variable ID.
    ///
    /// Increments the internal counter and returns a unique identifier for
    /// creating temporary values in IR code.
    ///
    /// # Returns
    ///
    /// A unique `u64` identifier for a temporary variable.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let temp_id = generator.new_temp();
    /// let temp_value = Value::new_temporary(temp_id, IrType::I32);
    /// ```
    pub fn new_temp(&mut self) -> u64 {
        let id = self.temp_counter;
        self.temp_counter += 1;
        id
    }

    /// Generates a unique basic block label.
    ///
    /// Creates a unique label for a basic block by combining the provided prefix
    /// with an incremented counter. Uses an internal buffer to avoid allocations.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix for the block label (e.g., "loop_start", "then")
    ///
    /// # Returns
    ///
    /// A unique string label for the basic block.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let label = generator.new_block_label("loop_body");
    /// // Returns something like "loop_body_1"
    /// ```
    fn new_block_label(&mut self, prefix: &str) -> String {
        self.block_counter += 1;
        self.format_buffer.clear();
        self.format_buffer.push_str(prefix);
        self.format_buffer.push('_');
        let _ = write!(&mut self.format_buffer, "{}", self.block_counter);
        self.format_buffer.clone()
    }

    /// Starts a new basic block for IR generation.
    ///
    /// Finalizes the current block (if any), creates a new block with the given
    /// label, adds it to the function's CFG, and sets it as the current block.
    ///
    /// # Arguments
    ///
    /// * `func` - The function to add the block to
    /// * `label` - The unique label for the new block
    /// * `span` - Source location for debugging
    ///
    /// # Examples
    ///
    /// ```ignore
    /// generator.start_block(func, "then_1", span);
    /// // Now generating code in the "then_1" block
    /// ```
    fn start_block(&mut self, func: &mut Function, label: &str, span: SourceSpan) {
        // Finalize the current block first
        self.finalize_current_block(func);

        // Create a new block
        let new_block = BasicBlock::new(label, span.clone()).with_scope(self.scope_manager.current_scope());

        // Add the block to the CFG
        func.add_block(label, span);

        // Set the new block as current
        self.current_block = Some(new_block);
        self.current_block_label = Some(label.to_string());
    }

    /// Adds an instruction to the current basic block.
    ///
    /// Appends the instruction to the current block's instruction list.
    /// Does nothing if there is no current block.
    ///
    /// # Arguments
    ///
    /// * `inst` - The instruction to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let add_inst = Instruction::new(InstructionKind::Binary { /* ... */ }, span);
    /// generator.add_instruction(add_inst);
    /// ```
    pub fn add_instruction(&mut self, inst: Instruction) {
        if let Some(block) = &mut self.current_block {
            block.instructions.push(inst);
        }
    }

    /// Adds a terminator instruction to the current basic block.
    ///
    /// Sets the terminator for the current block. Terminators control the flow
    /// of execution between blocks (branches, returns, etc.). Block connections
    /// are established later during finalization.
    ///
    /// # Arguments
    ///
    /// * `_func` - The function (unused, kept for consistency)
    /// * `term` - The terminator instruction to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let ret = Terminator::new(TerminatorKind::Return { value, ty }, span);
    /// generator.add_terminator(func, ret);
    /// ```
    fn add_terminator(&mut self, _func: &mut Function, term: Terminator) {
        if let Some(block) = &mut self.current_block {
            block.terminator = term.clone();
            // Don't connect blocks here - they'll be connected when the block is finalized
        }
    }

    /// Generates array access for assignment targets.
    ///
    /// Calculates the address of an array element using GetElementPtr (GEP)
    /// and returns the pointer without loading the value. This allows the
    /// pointer to be used as the target of a store instruction.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the array access
    /// * `array` - Expression evaluating to the array base
    /// * `index` - Expression evaluating to the array index
    /// * `span` - Source location for error reporting
    ///
    /// # Returns
    ///
    /// A [`Value`] representing a pointer to the array element.
    ///
    /// # Errors
    ///
    /// Generates an error if the array expression is not an array type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // arr[5] = 42; (target generation)
    /// let ptr = generator.generate_array_access_target(func, arr_expr, index_expr, span);
    /// ```
    fn generate_array_access_target(
        &mut self, func: &mut Function, array: Expr, index: Expr, span: SourceSpan,
    ) -> Value {
        let base_val = self.generate_expr(func, array);
        let index_val = self.generate_expr(func, index);

        // Determine element type: handle both pointer to array and direct array
        let element_ty = match &base_val.ty {
            IrType::Pointer(inner) => match inner.as_ref() {
                IrType::Array(elem_ty, _) => *elem_ty.clone(),
                other => other.clone(), // fallback: pointer to already pointed element
            },
            IrType::Array(elem_ty, _) => *elem_ty.clone(),
            other => {
                // Unexpected case: report but continue with safe fallback (i32)
                self.new_error(Arc::from(format!("Array access on non-array type: {other}")), span.clone());
                IrType::I32
            }
        };

        let tmp = self.new_temp();
        let gep = Instruction::new(
            InstructionKind::GetElementPtr { base: base_val, index: index_val, element_ty: element_ty.clone() },
            span.clone(),
        )
        .with_result(Value::new_temporary(tmp, IrType::Pointer(Box::new(element_ty))));

        self.add_instruction(gep.clone());
        gep.result.unwrap()
    }

    /// Generates IR code for a function call.
    ///
    /// Extracts the function name from the callee expression, generates values
    /// for all arguments, looks up the function signature in the symbol table,
    /// and creates a call instruction with the appropriate return type.
    ///
    /// # Arguments
    ///
    /// * `func` - The function containing the call
    /// * `callee` - Expression identifying the function to call
    /// * `arguments` - Vector of argument expressions
    /// * `span` - Source location for error reporting
    ///
    /// # Returns
    ///
    /// A [`Value`] containing the result of the function call.
    ///
    /// # Errors
    ///
    /// Generates an error if:
    /// - The callee is not a variable expression
    /// - The function is not found in the symbol table (uses default return type)
    /// - The function type is invalid
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // result = add(x, y)
    /// let result = generator.generate_call(func, callee_expr, arg_exprs, span);
    /// ```
    fn generate_call(&mut self, func: &mut Function, callee: Expr, arguments: Vec<Expr>, span: SourceSpan) -> Value {
        // Get the function name from the callee expression
        let func_name = match &callee {
            Expr::Variable { name, .. } => name.clone(),
            _ => {
                self.new_error(Arc::from("Unsupported callee expression type"), callee.span().clone());
                return Value::new_literal(IrLiteralValue::I32(0));
            }
        };

        // Generate values for all arguments
        let mut arg_values = Vec::with_capacity(arguments.len());
        for arg in arguments {
            arg_values.push(self.generate_expr(func, arg));
        }

        // Look up the function signature in the symbol table
        let (return_type, func_value) = if let Some(func_decl) = self.scope_manager.lookup(&func_name) {
            match &func_decl.ty {
                IrType::Pointer(inner) => {
                    // For function pointers, the return type is the pointed-to type
                    let return_type = inner.as_ref().clone();
                    (return_type, func_decl.clone())
                }
                _ => {
                    // If we can't determine the return type, fall back to default
                    self.new_error(
                        Arc::from(format!("Function '{}' does not have a valid function pointer type", func_name)),
                        span.clone(),
                    );
                    let return_type = IrType::I64; // Default assumption
                    let func_value = Value::new_global(func_name, IrType::Pointer(Box::new(return_type.clone())));
                    (return_type, func_value)
                }
            }
        } else {
            // If function is not in symbol table, we might be calling a function
            // that is defined later or externally. Use a default return type.
            self.new_error(
                Arc::from(format!("Function '{}' not found in symbol table, using default return type", func_name)),
                span.clone(),
            );
            let return_type = IrType::I64; // Default assumption
            let func_value = Value::new_global(func_name, IrType::Pointer(Box::new(return_type.clone())));
            (return_type, func_value)
        };

        // Create a temporary value for the result
        let dest_id = self.new_temp();
        let result_value = Value::new_temporary(dest_id, return_type.clone());

        // Create the call instruction
        let call_inst = Instruction::new(
            InstructionKind::Call { func: func_value, args: arg_values, ty: return_type },
            span.clone(),
        )
        .with_result(result_value);

        self.add_instruction(call_inst.clone());
        call_inst.result.unwrap()
    }
}

impl Default for IrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
