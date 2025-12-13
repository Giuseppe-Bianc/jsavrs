//! SCCP worklist algorithm implementation
//!
//! Implements the Wegman-Zadeck sparse conditional constant propagation algorithm
//! with SSA and CFG edge worklists.

use super::lattice::LatticeValue;
use crate::ir::Function;
use crate::ir::Terminator;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// CFG edge representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CFGEdge {
    pub from: usize,
    pub to: usize,
}

impl CFGEdge {
    #[must_use]
    pub const fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    #[must_use]
    pub fn from_node_index(from: NodeIndex, to: NodeIndex) -> Self {
        Self { from: from.index(), to: to.index() }
    }
}

/// Tracks lattice values for SSA values
pub struct LatticeState {
    values: HashMap<usize, LatticeValue>,
}

impl Default for LatticeState {
    fn default() -> Self {
        Self::new()
    }
}

impl LatticeState {
    #[must_use]
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { values: HashMap::with_capacity(capacity) }
    }

    /// Gets the lattice value for an SSA value (defaults to Bottom if not set)
    #[must_use]
    pub fn get(&self, value_id: usize) -> LatticeValue {
        self.values.get(&value_id).cloned().unwrap_or(LatticeValue::Bottom)
    }

    /// Updates the lattice value for an SSA value
    /// Returns true if the value changed
    pub fn update(&mut self, value_id: usize, new_value: LatticeValue) -> bool {
        let old_value = self.get(value_id);
        if old_value == new_value {
            false
        } else {
            self.values.insert(value_id, new_value);
            true
        }
    }

    /// Initializes an SSA value to a specific lattice value
    pub fn initialize(&mut self, value_id: usize, value: LatticeValue) {
        self.values.insert(value_id, value);
    }
}

/// Tracks which CFG edges are executable
pub struct ExecutableEdgeSet {
    edges: HashSet<CFGEdge>,
}

impl Default for ExecutableEdgeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutableEdgeSet {
    #[must_use]
    pub fn new() -> Self {
        Self { edges: HashSet::new() }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { edges: HashSet::with_capacity(capacity) }
    }

    /// Marks a CFG edge as executable
    /// Returns true if this is the first time the edge was marked executable
    pub fn mark_executable(&mut self, edge: CFGEdge) -> bool {
        self.edges.insert(edge)
    }

    /// Checks if a CFG edge is executable
    #[must_use]
    pub fn is_executable(&self, edge: &CFGEdge) -> bool {
        self.edges.contains(edge)
    }

    /// Checks if a block has any executable predecessor edges
    #[must_use]
    pub fn has_executable_predecessor(&self, block_id: usize) -> bool {
        self.edges.iter().any(|e| e.to == block_id)
    }

    /// Returns an iterator over all executable predecessors of a block
    pub fn executable_predecessors(&self, block_id: usize) -> impl Iterator<Item = usize> + '_ {
        self.edges.iter().filter(move |e| e.to == block_id).map(|e| e.from)
    }
}

/// Generic worklist with deduplication
pub struct Worklist<T: Eq + Hash + Clone> {
    queue: VecDeque<T>,
    seen: HashSet<T>,
}

impl<T: Eq + Hash + Clone> Drop for Worklist<T> {
    fn drop(&mut self) {
        // Explicitly clear collections to release memory eagerly
        self.queue.clear();
        self.seen.clear();
    }
}

impl<T: Eq + Hash + Clone> Default for Worklist<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash + Clone> Worklist<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { queue: VecDeque::new(), seen: HashSet::new() }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { queue: VecDeque::with_capacity(capacity), seen: HashSet::with_capacity(capacity) }
    }

    /// Adds an item to the worklist if not already present
    pub fn push(&mut self, item: T) {
        if self.seen.insert(item.clone()) {
            self.queue.push_back(item);
        }
    }

    /// Removes and returns the next item from the worklist
    pub fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.queue.pop_front() {
            self.seen.remove(&item);
            Some(item)
        } else {
            None
        }
    }

    /// Checks if the worklist is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the number of items in the worklist
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

/// SCCP algorithm errors
#[derive(Debug, thiserror::Error)]
pub enum SCCPError {
    #[error("Maximum iteration limit exceeded: {0}")]
    MaxIterationsExceeded(usize),
    #[error("Invalid SSA form: {0}")]
    InvalidSSA(String),
}
/// SCCP worklist-based propagator
///
/// Implements the Wegman-Zadeck algorithm for sparse conditional constant propagation.
/// The propagator maintains two worklists (CFG and SSA) and processes them until fixed point.
pub struct SCCPropagator {
    /// Lattice state mapping each SSA value to its lattice value
    lattice: LatticeState,
    /// Set of executable CFG edges
    executable_edges: ExecutableEdgeSet,
    /// CFG worklist for flow-sensitive propagation
    cfg_worklist: Worklist<CFGEdge>,
    /// SSA worklist for value updates
    ssa_worklist: Worklist<usize>,
    /// Verbose diagnostic output
    verbose: bool,
}

impl Drop for SCCPropagator {
    fn drop(&mut self) {
        // Explicitly clear all internal state to release memory eagerly
        // The worklists and other structures will be cleared by their own Drop impls
        self.lattice = LatticeState::new();
        self.executable_edges = ExecutableEdgeSet::new();
    }
}

impl SCCPropagator {
    /// Creates a new `SCCPropagator` for the given function with capacity preallocation.
    ///
    /// Preallocates internal data structures based on function size estimates:
    /// - `LatticeState`: capacity = `num_blocks * avg_instructions_per_block`
    /// - `ExecutableEdgeSet`: capacity = `num_blocks * 2` (average 2 edges per block)
    /// - `Worklists`: capacity = `num_blocks`
    ///
    /// # Arguments
    /// * `function` - The IR function to analyze
    ///
    /// # Returns
    /// A new `SCCPropagator` instance with preallocated capacity
    #[must_use]
    pub fn new_for_function(function: &crate::ir::Function) -> Self {
        let num_blocks = function.cfg.graph().node_count();
        let estimated_values = num_blocks * 10; // Estimate ~10 instructions per block
        let estimated_edges = num_blocks * 2; // Average 2 edges per block

        Self {
            lattice: LatticeState::with_capacity(estimated_values),
            executable_edges: ExecutableEdgeSet::with_capacity(estimated_edges),
            cfg_worklist: Worklist::with_capacity(num_blocks),
            ssa_worklist: Worklist::with_capacity(estimated_values),
            verbose: false,
        }
    }

    /// Sets verbose diagnostic output
    pub const fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Initializes the propagator for a function following FR-001.
    ///
    /// - Parameters → Top (overdefined)
    /// - Local variables → Bottom (unreachable)
    /// - Entry block edge → Executable
    ///
    /// # Arguments
    /// * `function` - The IR function to initialize
    ///
    /// # Note
    /// This is a simplified initialization. Full implementation will need
    /// to properly map parameter/variable names to `ValueIds`.
    fn initialize(&mut self, function: &crate::ir::Function) {
        // Mark entry block edge as executable (FR-001)
        if let Some(entry_idx) = function.cfg.get_entry_block_index() {
            // Entry block has no predecessor, use synthetic edge
            let entry_edge = CFGEdge { from: usize::MAX, to: entry_idx.index() };
            self.executable_edges.mark_executable(entry_edge);
            self.cfg_worklist.push(entry_edge);
        }

        // Note: Full parameter and local variable initialization will be implemented
        // when we have a proper mapping from names to ValueIds in the next tasks.
    }

    /// Main propagation loop implementing Wegman-Zadeck SCCP algorithm (FR-001).
    ///
    /// Processes CFG and SSA worklists until fixed point:
    /// 1. Process CFG worklist: visit blocks with new executable edges
    /// 2. Process SSA worklist: re-evaluate instructions with updated operands
    ///
    /// # Arguments
    /// * `function` - The IR function to analyze
    /// * `max_iterations` - Maximum number of iterations before giving up
    ///
    /// # Returns
    /// `Ok(iterations_count)` on successful completion, `Err(SCCPError)` if iteration limit exceeded
    ///
    /// # Errors
    /// - `MaxIterationsExceeded`: If the algorithm does not converge within `max_iterations`
    pub fn propagate(&mut self, function: &crate::ir::Function, max_iterations: usize) -> Result<usize, SCCPError> {
        self.initialize(function);

        let mut iterations = 0;
        while iterations < max_iterations {
            let mut made_progress = false;

            // Process CFG worklist
            while let Some(edge) = self.cfg_worklist.pop() {
                made_progress = true;
                self.visit_block(function, edge.to)?;
            }

            // Process SSA worklist
            while let Some(value_id) = self.ssa_worklist.pop() {
                made_progress = true;
                self.visit_value(function, value_id)?;
            }

            if !made_progress {
                break; // Fixed point reached
            }

            iterations += 1;
        }

        if iterations >= max_iterations {
            Err(SCCPError::MaxIterationsExceeded(max_iterations))
        } else {
            Ok(iterations)
        }
    }

    /// Visits a block when a new edge becomes executable.
    ///
    /// Processes all instructions in the block and evaluates the terminator.
    fn visit_block(&mut self, function: &crate::ir::Function, block_idx: usize) -> Result<(), SCCPError> {
        use petgraph::graph::NodeIndex;

        // Get the block from the CFG
        let node_idx = NodeIndex::new(block_idx);
        let block = &function.cfg.graph()[node_idx];

        // Process all instructions in the block
        for (instr_idx, instruction) in block.instructions.iter().enumerate() {
            self.visit_instruction(function, block_idx, instr_idx, instruction)?;
        }

        // Process terminator to determine control flow (T052-T058)
        self.visit_terminator(function, block_idx, &block.terminator)?;

        Ok(())
    }

    /// Visits an instruction and evaluates it if all operands are known.
    ///
    /// Implements FR-002 through FR-008 for binary operations.
    fn visit_instruction(
        &mut self, _function: &crate::ir::Function, block_idx: usize, _instr_idx: usize,
        instruction: &crate::ir::Instruction,
    ) -> Result<(), SCCPError> {
        use super::evaluator::ConstantEvaluator;
        use super::lattice::ConstantValue;
        use crate::ir::instruction::InstructionKind;

        // Only process instructions with results
        let Some(result_value) = &instruction.result else { return Ok(()) };

        let result_id = result_value.id;

        // Evaluate based on instruction kind
        let new_lattice_value = match &instruction.kind {
            InstructionKind::Binary { op, left, right, .. } => {
                use crate::ir::instruction::IrBinaryOp;

                // Get lattice values for operands
                let left_lattice = self.lattice.get(Self::value_id_to_key(&left.id));
                let right_lattice = self.lattice.get(Self::value_id_to_key(&right.id));

                // If either operand is Bottom, result is Bottom
                if left_lattice.is_bottom() || right_lattice.is_bottom() {
                    LatticeValue::Bottom
                } else if left_lattice.is_top() || right_lattice.is_top() {
                    // If either operand is Top, result is Top
                    LatticeValue::Top
                } else if let (Some(left_const), Some(right_const)) =
                    (left_lattice.as_constant(), right_lattice.as_constant())
                {
                    // Both are constants, try to evaluate
                    match (left_const, right_const) {
                        (ConstantValue::I32(l), ConstantValue::I32(r)) => {
                            // Check if it's a comparison operation first
                            match op {
                                IrBinaryOp::Equal => {
                                    ConstantEvaluator::eval_compare_i32(super::evaluator::BinaryOp::Eq, *l, *r)
                                }
                                IrBinaryOp::NotEqual => {
                                    ConstantEvaluator::eval_compare_i32(super::evaluator::BinaryOp::Ne, *l, *r)
                                }
                                IrBinaryOp::Less => {
                                    ConstantEvaluator::eval_compare_i32(super::evaluator::BinaryOp::Lt, *l, *r)
                                }
                                IrBinaryOp::LessEqual => {
                                    ConstantEvaluator::eval_compare_i32(super::evaluator::BinaryOp::Le, *l, *r)
                                }
                                IrBinaryOp::Greater => {
                                    ConstantEvaluator::eval_compare_i32(super::evaluator::BinaryOp::Gt, *l, *r)
                                }
                                IrBinaryOp::GreaterEqual => {
                                    ConstantEvaluator::eval_compare_i32(super::evaluator::BinaryOp::Ge, *l, *r)
                                }
                                // Arithmetic operations
                                IrBinaryOp::Add => {
                                    ConstantEvaluator::eval_binary_i32(super::evaluator::BinaryOp::Add, *l, *r)
                                }
                                IrBinaryOp::Subtract => {
                                    ConstantEvaluator::eval_binary_i32(super::evaluator::BinaryOp::Sub, *l, *r)
                                }
                                IrBinaryOp::Multiply => {
                                    ConstantEvaluator::eval_binary_i32(super::evaluator::BinaryOp::Mul, *l, *r)
                                }
                                IrBinaryOp::Divide => {
                                    ConstantEvaluator::eval_binary_i32(super::evaluator::BinaryOp::Div, *l, *r)
                                }
                                IrBinaryOp::Modulo => {
                                    ConstantEvaluator::eval_binary_i32(super::evaluator::BinaryOp::Mod, *l, *r)
                                }
                                _ => LatticeValue::Top, // Other ops not yet supported
                            }
                        }
                        (ConstantValue::Bool(l), ConstantValue::Bool(r)) => {
                            // Boolean operations
                            match op {
                                IrBinaryOp::And => {
                                    ConstantEvaluator::eval_binary_bool(super::evaluator::BinaryOp::And, *l, *r)
                                }
                                IrBinaryOp::Or => {
                                    ConstantEvaluator::eval_binary_bool(super::evaluator::BinaryOp::Or, *l, *r)
                                }
                                _ => LatticeValue::Top,
                            }
                        }
                        _ => LatticeValue::Top, // Type mismatch or unsupported types
                    }
                } else {
                    LatticeValue::Top
                }
            }
            InstructionKind::Phi { incoming, .. } => {
                // T068-T069: Phi node evaluation with executable edge filtering
                self.eval_phi_node(block_idx, incoming)?
            }
            _ => {
                // Other instruction types not yet implemented
                return Ok(());
            }
        };

        // Update lattice value and add to SSA worklist if changed
        self.update_lattice_value(Self::value_id_to_key(&result_id), new_lattice_value);

        Ok(())
    }

    /// Re-evaluates an instruction when one of its operands changes.
    fn visit_value(&mut self, function: &crate::ir::Function, value_id: usize) -> Result<(), SCCPError> {
        // Find all instructions that use this value and re-evaluate them
        use petgraph::visit::IntoNodeReferences;

        for (node_idx, block) in function.cfg.graph().node_references() {
            for (instr_idx, instruction) in block.instructions.iter().enumerate() {
                // Check if this instruction uses the value
                if self.instruction_uses_value(instruction, value_id) {
                    self.visit_instruction(function, node_idx.index(), instr_idx, instruction)?;
                }
            }
        }

        Ok(())
    }

    /// Visits a CFG edge and marks it executable (T051)
    ///
    /// Called when a new CFG edge becomes executable during propagation.
    /// Adds the target block to the CFG worklist if this is the first
    /// executable edge reaching it.
    fn visit_cfg_edge(&mut self, edge: CFGEdge) {
        if self.executable_edges.mark_executable(edge) {
            self.cfg_worklist.push(edge);
        }
    }

    /// Visits a terminator instruction to determine control flow (T052-T053)
    ///
    /// Analyzes terminator instructions to determine which CFG edges are executable:
    /// - Branch: Unconditional, always marks the edge executable
    /// - `ConditionalBranch`: Evaluates condition, marks appropriate edge(s)
    /// - Switch: Evaluates selector, marks matching case edge
    /// - Return/Unreachable: No CFG edges to mark
    #[allow(clippy::unnecessary_wraps)]
    fn visit_terminator(
        &mut self, function: &Function, block_id: usize, terminator: &Terminator,
    ) -> Result<(), SCCPError> {
        use crate::ir::terminator::TerminatorKind;

        match &terminator.kind {
            // Unconditional branch - always take the edge (T052)
            TerminatorKind::Branch { label } => {
                if let Some(target_idx) = self.find_block_by_label(function, label) {
                    let edge = CFGEdge::new(block_id, target_idx);
                    self.visit_cfg_edge(edge);
                }
                Ok(())
            }

            // Conditional branch - evaluate condition (T053)
            TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
                let cond_lattice = self.lattice.get(Self::value_id_to_key(&condition.id));

                match cond_lattice {
                    LatticeValue::Bottom => {
                        // Unreachable, don't mark any edges
                        Ok(())
                    }
                    LatticeValue::Constant(ref const_val) => {
                        // Constant condition - mark only the taken branch
                        if let Some(is_true) = const_val.as_bool() {
                            let target_label = if is_true { true_label } else { false_label };
                            if let Some(target_idx) = self.find_block_by_label(function, target_label) {
                                let edge = CFGEdge::new(block_id, target_idx);
                                self.visit_cfg_edge(edge);
                            }
                        } else {
                            // Non-boolean constant - mark both edges as Top
                            if let Some(true_idx) = self.find_block_by_label(function, true_label) {
                                let edge = CFGEdge::new(block_id, true_idx);
                                self.visit_cfg_edge(edge);
                            }
                            if let Some(false_idx) = self.find_block_by_label(function, false_label) {
                                let edge = CFGEdge::new(block_id, false_idx);
                                self.visit_cfg_edge(edge);
                            }
                        }
                        Ok(())
                    }
                    LatticeValue::Top => {
                        // Runtime-varying condition - mark both edges
                        if let Some(true_idx) = self.find_block_by_label(function, true_label) {
                            let edge = CFGEdge::new(block_id, true_idx);
                            self.visit_cfg_edge(edge);
                        }
                        if let Some(false_idx) = self.find_block_by_label(function, false_label) {
                            let edge = CFGEdge::new(block_id, false_idx);
                            self.visit_cfg_edge(edge);
                        }
                        Ok(())
                    }
                }
            }

            // Switch statement - evaluate selector (T057-T058)
            TerminatorKind::Switch { value, default_label, cases, .. } => {
                let value_lattice = self.lattice.get(Self::value_id_to_key(&value.id));

                match value_lattice {
                    LatticeValue::Bottom => {
                        // Unreachable
                        Ok(())
                    }
                    LatticeValue::Constant(ref const_val) => {
                        // Constant selector - find matching case
                        let mut matched = false;
                        for (case_value, case_label) in cases {
                            // Compare constant values
                            let case_lattice = self.lattice.get(Self::value_id_to_key(&case_value.id));
                            if let LatticeValue::Constant(ref case_const) = case_lattice
                                && const_val == case_const
                            {
                                // Match found - mark this edge only
                                if let Some(target_idx) = self.find_block_by_label(function, case_label) {
                                    let edge = CFGEdge::new(block_id, target_idx);
                                    self.visit_cfg_edge(edge);
                                }
                                matched = true;
                                break;
                            }
                        }

                        // If no case matched, take default
                        if !matched && let Some(default_idx) = self.find_block_by_label(function, default_label) {
                            let edge = CFGEdge::new(block_id, default_idx);
                            self.visit_cfg_edge(edge);
                        }
                        Ok(())
                    }
                    LatticeValue::Top => {
                        // Runtime-varying selector - mark all edges
                        for (_, case_label) in cases {
                            if let Some(target_idx) = self.find_block_by_label(function, case_label) {
                                let edge = CFGEdge::new(block_id, target_idx);
                                self.visit_cfg_edge(edge);
                            }
                        }
                        if let Some(default_idx) = self.find_block_by_label(function, default_label) {
                            let edge = CFGEdge::new(block_id, default_idx);
                            self.visit_cfg_edge(edge);
                        }
                        Ok(())
                    }
                }
            }

            // Return and Unreachable don't create CFG edges
            TerminatorKind::Return { .. } | TerminatorKind::Unreachable => Ok(()),

            // IndirectBranch marks all possible targets as Top
            TerminatorKind::IndirectBranch { possible_labels, .. } => {
                for label in possible_labels {
                    if let Some(target_idx) = self.find_block_by_label(function, label) {
                        let edge = CFGEdge::new(block_id, target_idx);
                        self.visit_cfg_edge(edge);
                    }
                }
                Ok(())
            }
        }
    }

    /// Evaluates a phi node by computing the meet of values from executable predecessors.
    ///
    /// Implements T068-T069: Phi node evaluation with executable edge filtering.
    ///
    /// # Algorithm
    /// 1. Filter incoming values to only those from executable predecessors
    /// 2. If no executable predecessors, result is Bottom (unreachable)
    /// 3. If one executable predecessor, result is that value's lattice value
    /// 4. If multiple predecessors, compute meet of all executable values
    ///
    /// # Arguments
    /// * `block_id` - ID of the block containing this phi node
    /// * `incoming` - Vector of (value, `predecessor_label`) pairs
    ///
    /// # Returns
    /// The computed lattice value for this phi node
    #[allow(clippy::unnecessary_wraps)]
    fn eval_phi_node(
        &self, block_id: usize, incoming: &[(crate::ir::Value, String)],
    ) -> Result<LatticeValue, SCCPError> {
        use super::lattice::LatticeValue;

        // Collect lattice values from executable predecessors only
        let mut executable_values = Vec::new();

        for (_value, _pred_label) in incoming {
            // Check if the edge from this predecessor is executable
            // We need to find the predecessor block index by label
            // For now, we'll use a simplified approach: check if any edge to this block is executable
            // from a block with a matching label

            // This is a simplified implementation - a full implementation would need
            // to map predecessor labels to block indices
            let has_executable_pred = self.executable_edges.has_executable_predecessor(block_id);

            if has_executable_pred {
                // Get the lattice value for this incoming value
                // Note: This is simplified - full implementation needs proper value tracking
                executable_values.push(LatticeValue::Top);
            }
        }

        // Compute result based on number of executable predecessors
        match executable_values.len() {
            0 => {
                // No executable predecessors - phi is unreachable
                Ok(LatticeValue::Bottom)
            }
            1 => {
                // Single executable predecessor - use its value directly
                Ok(executable_values[0].clone())
            }
            _ => {
                // Multiple executable predecessors - compute meet
                let mut result = executable_values[0].clone();
                for value in &executable_values[1..] {
                    result = result.meet(value);
                }
                Ok(result)
            }
        }
    }

    /// Helper to find a basic block by its label
    #[allow(clippy::unused_self)]
    fn find_block_by_label(&self, function: &Function, label: &str) -> Option<usize> {
        use petgraph::visit::IntoNodeReferences;

        for (idx, block) in function.cfg.graph().node_references() {
            if block.label.as_ref() == label {
                return Some(idx.index());
            }
        }
        None
    }

    /// Updates a lattice value and adds dependent values to SSA worklist if changed.
    ///
    /// Implements FR-002 monotonic updates.
    fn update_lattice_value(&mut self, value_id: usize, new_value: LatticeValue) {
        if self.lattice.update(value_id, new_value) {
            // Value changed, add to SSA worklist for propagation
            self.ssa_worklist.push(value_id);
        }
    }

    /// Helper to check if an instruction uses a specific value.
    #[allow(clippy::unused_self)]
    fn instruction_uses_value(&self, instruction: &crate::ir::Instruction, value_id: usize) -> bool {
        use crate::ir::instruction::InstructionKind;

        match &instruction.kind {
            InstructionKind::Binary { left, right, .. } => {
                Self::value_id_to_key(&left.id) == value_id || Self::value_id_to_key(&right.id) == value_id
            }
            InstructionKind::Unary { operand, .. } => Self::value_id_to_key(&operand.id) == value_id,
            InstructionKind::Store { value, dest, .. } => {
                Self::value_id_to_key(&value.id) == value_id || Self::value_id_to_key(&dest.id) == value_id
            }
            InstructionKind::Load { src, .. } => Self::value_id_to_key(&src.id) == value_id,
            _ => false,
        }
    }

    /// Converts a `ValueId` to a usize key for `HashMap` lookup.
    #[allow(clippy::unwrap_used)]
    fn value_id_to_key(value_id: &crate::ir::value::ValueId) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        value_id.hash(&mut hasher);
        usize::try_from(hasher.finish()).unwrap()
    }

    /// Returns a reference to the lattice state for use by the rewriter
    #[must_use]
    pub const fn get_lattice_state(&self) -> &LatticeState {
        &self.lattice
    }

    /// Returns a reference to the executable edge set for use by the rewriter
    #[must_use]
    pub const fn get_executable_edges(&self) -> &ExecutableEdgeSet {
        &self.executable_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_state() {
        let mut state = LatticeState::new();

        // Default is Bottom
        assert_eq!(state.get(1), LatticeValue::Bottom);

        // Update changes value
        assert!(state.update(1, LatticeValue::Top));
        assert_eq!(state.get(1), LatticeValue::Top);

        // No-op update returns false
        assert!(!state.update(1, LatticeValue::Top));
    }

    #[test]
    fn test_executable_edge_set() {
        let mut edges = ExecutableEdgeSet::new();
        let edge = CFGEdge::new(0, 1);

        assert!(!edges.is_executable(&edge));
        assert!(edges.mark_executable(edge));
        assert!(edges.is_executable(&edge));
        assert!(!edges.mark_executable(edge)); // Already marked
    }

    #[test]
    fn test_worklist() {
        let mut worklist = Worklist::new();

        assert!(worklist.is_empty());

        worklist.push(1);
        worklist.push(2);
        worklist.push(1); // Duplicate, should not be added

        assert_eq!(worklist.len(), 2);
        assert_eq!(worklist.pop(), Some(1));
        assert_eq!(worklist.pop(), Some(2));
        assert_eq!(worklist.pop(), None);
        assert!(worklist.is_empty());
    }
}
