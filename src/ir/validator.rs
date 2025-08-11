// src/ir/validator.rs
use crate::ir::ir::*;
use crate::ir::types::Type;
//use crate::ir::values::Value;
use std::collections::{HashSet, HashMap};
pub struct IrValidator {
    errors: Vec<CompileError>,
    warnings: Vec<CompileWarning>,
}
#[derive(Debug, Clone)]
pub struct CompileError {
    pub message: String,
    pub location: Option<String>,
    pub help: Option<String>,
}
#[derive(Debug, Clone)]
pub struct CompileWarning {
    pub message: String,
    pub location: Option<String>,
    pub help: Option<String>,
}
impl IrValidator {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn validate(&mut self, module: &Module) -> bool {
        self.validate_functions(module);
        self.validate_global_variables(module);
        self.validate_type_definitions(module);
        self.validate_control_flow(module);
        self.errors.is_empty()
    }
    fn validate_functions(&mut self, module: &Module) {
        for function in &module.functions {
            self.validate_function(function);
        }
    }
    fn validate_function(&mut self, function: &Function) {
        // Check that function has at least one basic block
        if function.basic_blocks.is_empty() {
            self.errors.push(CompileError {
                message: format!("Function '{}' has no basic blocks", function.name),
                location: Some(function.name.clone()),
                help: Some("Add at least one basic block to the function".to_string()),
            });
            return;
        }
        // Check that entry block exists
        if function.entry_block().is_none() {
            self.errors.push(CompileError {
                message: format!("Function '{}' has no entry block", function.name),
                location: Some(function.name.clone()),
                help: Some("Add an entry block to the function".to_string()),
            });
        }
        // Validate basic blocks
        for block in &function.basic_blocks {
            self.validate_basic_block(function, block);
        }
    }
    fn validate_basic_block(&mut self, function: &Function, block: &BasicBlock) {
        // Check that block has a terminator
        if matches!(block.terminator, Terminator::Unreachable) {
            self.errors.push(CompileError {
                message: format!("Block '{}' in function '{}' has no terminator", block.name, function.name),
                location: Some(format!("{}::{}", function.name, block.name)),
                help: Some("Add a terminator instruction to the block".to_string()),
            });
        }
        // Validate instructions
        for instruction in &block.instructions {
            self.validate_instruction(function, block, instruction);
        }
        // Validate terminator
        self.validate_terminator(function, block, &block.terminator);
    }
    fn validate_instruction(&mut self, function: &Function, block: &BasicBlock, instruction: &Instruction) {
        match instruction {
            Instruction::Alloca { dest, ty, .. } => {
                if !ty.is_sized() {
                    self.errors.push(CompileError {
                        message: format!("Alloca instruction in block '{}' has unsized type '{}'", block.name, ty),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Use a sized type for alloca".to_string()),
                    });
                }
                if !dest.get_type().is_pointer() {
                    self.errors.push(CompileError {
                        message: format!("Alloca destination in block '{}' is not a pointer type", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Alloca should produce a pointer type".to_string()),
                    });
                }
            }
            Instruction::Load { dest, ptr, ty, .. } => {
                if !ptr.get_type().is_pointer() {
                    self.errors.push(CompileError {
                        message: format!("Load instruction in block '{}' has non-pointer operand", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Load requires a pointer operand".to_string()),
                    });
                }
                if dest.get_type() != *ty {
                    self.errors.push(CompileError {
                        message: format!("Load instruction in block '{}' has mismatched types", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Destination type must match loaded type".to_string()),
                    });
                }
            }
            Instruction::Store { value, ptr, .. } => {
                if !ptr.get_type().is_pointer() {
                    self.errors.push(CompileError {
                        message: format!("Store instruction in block '{}' has non-pointer pointer operand", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Store requires a pointer operand".to_string()),
                    });
                }
                // Correzione: clona il tipo invece di mantenere un riferimento
                let pointee_type = ptr.get_type().get_pointer_element_type()
                    .cloned()
                    .unwrap_or(Type::void());
                if value.get_type() != pointee_type {
                    self.errors.push(CompileError {
                        message: format!("Store instruction in block '{}' has mismatched types", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Value type must match pointer element type".to_string()),
                    });
                }
            }
            Instruction::BinaryOp { op, dest, left, right, .. } => {
                if left.get_type() != right.get_type() {
                    self.errors.push(CompileError {
                        message: format!("Binary instruction '{}' in block '{}' has mismatched operand types", op, block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Binary operands must have the same type".to_string()),
                    });
                }
                if dest.get_type() != left.get_type() {
                    self.errors.push(CompileError {
                        message: format!("Binary instruction '{}' in block '{}' has mismatched destination type", op, block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Destination type must match operand type".to_string()),
                    });
                }
            }
            Instruction::Call { dest, callee, arguments, .. } => {
                if let Type::Function { return_type, parameters, .. } = callee.get_type() {
                    if arguments.len() != parameters.len() {
                        self.errors.push(CompileError {
                            message: format!("Call instruction in block '{}' has wrong number of arguments", block.name),
                            location: Some(format!("{}::{}", function.name, block.name)),
                            help: Some(format!("Expected {} arguments, got {}", parameters.len(), arguments.len())),
                        });
                    }
                    for (i, (arg, expected_ty)) in arguments.iter().zip(parameters.iter()).enumerate() {
                        // Correzione: accedi al primo elemento della tupla (il Value) e poi chiama get_type()
                        if arg.0.get_type() != *expected_ty {
                            self.errors.push(CompileError {
                                message: format!("Call instruction in block '{}' has mismatched argument type at position {}", block.name, i),
                                location: Some(format!("{}::{}", function.name, block.name)),
                                help: Some(format!("Expected type '{}', got '{}'", expected_ty, arg.0.get_type())),
                            });
                        }
                    }
                    if let Some(d) = dest {
                        if d.get_type() != *return_type {
                            self.errors.push(CompileError {
                                message: format!("Call instruction in block '{}' has mismatched destination type", block.name),
                                location: Some(format!("{}::{}", function.name, block.name)),
                                help: Some(format!("Expected type '{}', got '{}'", return_type, d.get_type())),
                            });
                        }
                    }
                } else {
                    self.errors.push(CompileError {
                        message: format!("Call instruction in block '{}' has non-function callee", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Callee must be a function type".to_string()),
                    });
                }
            }
            Instruction::GetElementPtr { dest:_, ptr, indices, .. } => {
                if !ptr.get_type().is_pointer() {
                    self.errors.push(CompileError {
                        message: format!("GetElementPtr instruction in block '{}' has non-pointer base", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("GetElementPtr requires a pointer base".to_string()),
                    });
                }
                // Validate index types
                for (i, (index, _index_ty)) in indices.iter().enumerate() {
                    // Correzione: index è già un Value, non una tupla
                    if !index.get_type().is_integer() {
                        self.errors.push(CompileError {
                            message: format!("GetElementPtr instruction in block '{}' has non-integer index at position {}", block.name, i),
                            location: Some(format!("{}::{}", function.name, block.name)),
                            help: Some("Indices must be integers".to_string()),
                        });
                    }
                }
            }
            Instruction::Phi { dest, ty, incoming } => {
                if incoming.is_empty() {
                    self.errors.push(CompileError {
                        message: format!("Phi instruction in block '{}' has no incoming values", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Phi nodes must have at least one incoming value".to_string()),
                    });
                }
                for (value, block_name) in incoming {
                    if value.get_type() != *ty {
                        self.errors.push(CompileError {
                            message: format!("Phi instruction in block '{}' has mismatched incoming value type from block '{}'", block.name, block_name),
                            location: Some(format!("{}::{}", function.name, block.name)),
                            help: Some(format!("Expected type '{}', got '{}'", ty, value.get_type())),
                        });
                    }
                }
                if dest.get_type() != *ty {
                    self.errors.push(CompileError {
                        message: format!("Phi instruction in block '{}' has mismatched destination type", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Destination type must match incoming value type".to_string()),
                    });
                }
            }
            _ => {
                // Other instructions not validated yet
            }
        }
    }
    fn validate_terminator(&mut self, function: &Function, block: &BasicBlock, terminator: &Terminator) {
        match terminator {
            Terminator::Ret { value } => {
                if let Some(v) = value {
                    if v.get_type() != function.return_type {
                        self.errors.push(CompileError {
                            message: format!("Return instruction in block '{}' has mismatched return type", block.name),
                            location: Some(format!("{}::{}", function.name, block.name)),
                            help: Some(format!("Expected type '{}', got '{}'", function.return_type, v.get_type())),
                        });
                    }
                } else if !function.return_type.is_void() {
                    self.errors.push(CompileError {
                        message: format!("Return instruction in block '{}' returns void but function expects non-void", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Add a return value or change function return type to void".to_string()),
                    });
                }
            }
            Terminator::Br { dest } => {
                if function.get_basic_block(dest).is_none() {
                    self.errors.push(CompileError {
                        message: format!("Branch instruction in block '{}' references non-existent block '{}'", block.name, dest),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Check that the destination block exists".to_string()),
                    });
                }
            }
            Terminator::CondBr { condition, true_dest, false_dest } => {
                if condition.get_type() != Type::bool() {
                    self.errors.push(CompileError {
                        message: format!("Conditional branch instruction in block '{}' has non-boolean condition", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Condition must be a boolean value".to_string()),
                    });
                }
                if function.get_basic_block(true_dest).is_none() {
                    self.errors.push(CompileError {
                        message: format!("Conditional branch instruction in block '{}' references non-existent true block '{}'", block.name, true_dest),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Check that the true destination block exists".to_string()),
                    });
                }
                if function.get_basic_block(false_dest).is_none() {
                    self.errors.push(CompileError {
                        message: format!("Conditional branch instruction in block '{}' references non-existent false block '{}'", block.name, false_dest),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Check that the false destination block exists".to_string()),
                    });
                }
            }
            Terminator::Switch { value, default_dest, cases } => {
                if !value.get_type().is_integer() {
                    self.errors.push(CompileError {
                        message: format!("Switch instruction in block '{}' has non-integer condition", block.name),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Switch condition must be an integer value".to_string()),
                    });
                }
                if function.get_basic_block(default_dest).is_none() {
                    self.errors.push(CompileError {
                        message: format!("Switch instruction in block '{}' references non-existent default block '{}'", block.name, default_dest),
                        location: Some(format!("{}::{}", function.name, block.name)),
                        help: Some("Check that the default destination block exists".to_string()),
                    });
                }
                for (_case_value, case_dest) in cases {
                    if function.get_basic_block(case_dest).is_none() {
                        self.errors.push(CompileError {
                            message: format!("Switch instruction in block '{}' references non-existent case block '{}'", block.name, case_dest),
                            location: Some(format!("{}::{}", function.name, block.name)),
                            help: Some("Check that the case destination block exists".to_string()),
                        });
                    }
                }
            }
            _ => {
                // Other terminators not validated yet
            }
        }
    }
    fn validate_global_variables(&mut self, module: &Module) {
        for global in &module.global_variables {
            if global.initializer.is_none() && !global.is_constant {
                self.warnings.push(CompileWarning {
                    message: format!("Global variable '{}' has no initializer", global.name),
                    location: Some(global.name.clone()),
                    help: Some("Consider adding an initializer or marking it as constant".to_string()),
                });
            }
        }
    }
    fn validate_type_definitions(&mut self, module: &Module) {
        // Check for duplicate type definitions
        let mut type_names = HashSet::new();
        for type_def in &module.type_definitions {
            if type_names.contains(&type_def.name) {
                self.errors.push(CompileError {
                    message: format!("Duplicate type definition for '{}'", type_def.name),
                    location: Some(type_def.name.clone()),
                    help: Some("Use a unique name for each type definition".to_string()),
                });
            }
            type_names.insert(type_def.name.clone());
        }
    }
    fn validate_control_flow(&mut self, module: &Module) {
        for function in &module.functions {
            let cfg = ControlFlowGraph::build(&function.basic_blocks);
            self.validate_cfg(&cfg, function);
        }
    }
    fn validate_cfg(&mut self, cfg: &ControlFlowGraph, function: &Function) {
        // Check that all blocks are reachable from entry
        let reachable = cfg.compute_reachable_blocks();
        for block in &function.basic_blocks {
            if !reachable.contains(&block.name) {
                self.warnings.push(CompileWarning {
                    message: format!("Block '{}' in function '{}' is unreachable", block.name, function.name),
                    location: Some(format!("{}::{}", function.name, block.name)),
                    help: Some("Consider removing this block or adding a branch to it".to_string()),
                });
            }
        }
        // Check for infinite loops
        let has_loop = cfg.has_loop();
        if has_loop {
            // Check for loops without exit
            for block in &function.basic_blocks {
                if cfg.is_loop_header(&block.name) {
                    let has_exit = cfg.has_loop_exit(&block.name);
                    if !has_exit {
                        self.warnings.push(CompileWarning {
                            message: format!("Loop with header '{}' in function '{}' may not have an exit", block.name, function.name),
                            location: Some(format!("{}::{}", function.name, block.name)),
                            help: Some("Ensure the loop has a conditional exit".to_string()),
                        });
                    }
                }
            }
        }
    }
}
// Control Flow Graph for validation
#[derive(Debug)]
pub struct ControlFlowGraph {
    pub blocks: HashMap<String, BasicBlockNode>,
}
#[derive(Debug)]
pub struct BasicBlockNode {
    pub name: String,
    pub successors: Vec<String>,
    pub predecessors: Vec<String>,
}
impl ControlFlowGraph {
    pub fn build(blocks: &[BasicBlock]) -> Self {
        let mut graph = ControlFlowGraph {
            blocks: HashMap::new(),
        };
        // Add all blocks
        for block in blocks {
            let node = BasicBlockNode {
                name: block.name.clone(),
                successors: Vec::new(),
                predecessors: Vec::new(),
            };
            graph.blocks.insert(block.name.clone(), node);
        }
        // Add edges based on terminators
        for block in blocks {
            let successors = match &block.terminator {
                Terminator::Br { dest } => vec![dest.clone()],
                Terminator::CondBr { true_dest, false_dest, .. } => {
                    vec![true_dest.clone(), false_dest.clone()]
                }
                Terminator::Switch { default_dest, cases, .. } => {
                    let mut succs = vec![default_dest.clone()];
                    for (_, dest) in cases {
                        succs.push(dest.clone());
                    }
                    succs
                }
                Terminator::Ret { .. } | Terminator::Unreachable => Vec::new(),
                _ => Vec::new(), // Other terminators not handled yet
            };
            if let Some(node) = graph.blocks.get_mut(&block.name) {
                node.successors = successors.clone();
                // Update predecessors
                for succ in &successors {
                    if let Some(succ_node) = graph.blocks.get_mut(succ) {
                        succ_node.predecessors.push(block.name.clone());
                    }
                }
            }
        }
        graph
    }
    pub fn compute_reachable_blocks(&self) -> HashSet<String> {
        let mut reachable = HashSet::new();
        let mut worklist = Vec::new();
        // Start with entry block (first block)
        if let Some(entry) = self.blocks.keys().next() {
            worklist.push(entry.clone());
        }
        while let Some(block) = worklist.pop() {
            if reachable.insert(block.clone()) {
                if let Some(node) = self.blocks.get(&block) {
                    for succ in &node.successors {
                        worklist.push(succ.clone());
                    }
                }
            }
        }
        reachable
    }
    pub fn has_loop(&self) -> bool {
        // Simple loop detection using DFS
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        for block_name in self.blocks.keys() {
            if !visited.contains(block_name) {
                if self.dfs_has_loop(block_name, &mut visited, &mut recursion_stack) {
                    return true;
                }
            }
        }
        false
    }
    fn dfs_has_loop(
        &self,
        block_name: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(block_name.to_string());
        recursion_stack.insert(block_name.to_string());
        if let Some(node) = self.blocks.get(block_name) {
            for succ in &node.successors {
                if !visited.contains(succ) {
                    if self.dfs_has_loop(succ, visited, recursion_stack) {
                        return true;
                    }
                } else if recursion_stack.contains(succ) {
                    return true;
                }
            }
        }
        recursion_stack.remove(block_name);
        false
    }
    pub fn is_loop_header(&self, block_name: &str) -> bool {
        if let Some(node) = self.blocks.get(block_name) {
            // A block is a loop header if it has a back edge
            for pred in &node.predecessors {
                if self.dominates(block_name, pred) {
                    return true;
                }
            }
        }
        false
    }
    pub fn has_loop_exit(&self, loop_header: &str) -> bool {
        if let Some(node) = self.blocks.get(loop_header) {
            // Check if any successor is outside the loop
            for succ in &node.successors {
                if !self.dominates(loop_header, succ) {
                    return true;
                }
            }
        }
        false
    }
    fn dominates(&self, dominator: &str, dominated: &str) -> bool {
        // Simple dominance check (not fully accurate)
        if dominator == dominated {
            return true;
        }
        // In a real implementation, we would compute the dominator tree
        // For now, we'll use a simple heuristic
        dominator < dominated
    }
}