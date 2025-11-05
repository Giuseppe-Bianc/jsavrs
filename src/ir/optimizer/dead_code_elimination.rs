//! Dead Code Elimination (DCE) Optimization Phase
//!
//! Removes unreachable basic blocks and unused instructions from IR functions
//! while preserving all observable program behavior. Uses reachability analysis,
//! liveness analysis, and escape analysis to safely identify removable code.
//!
//! # Algorithm
//!
//! The optimization runs in a fixed-point loop:
//! 1. Reachability analysis: Mark blocks reachable from entry
//! 2. Block removal: Remove unreachable blocks, update CFG edges
//! 3. Liveness analysis: Compute live values via backward dataflow
//! 4. Escape analysis: Determine which allocations escape their scope
//! 5. Instruction removal: Remove dead instructions based on liveness/effects
//! 6. Repeat until no changes (fixed-point reached)
//!
//! # Example
//!
//! ```rust
//! use jsavrs::ir::{Module, optimizer::{Phase, DeadCodeElimination}};
//!
//! let mut module = Module::new("test", None);
//! // ... add functions ...
//!
//! let mut dce = DeadCodeElimination::default();
//! dce.run(&mut module);
//! ```
use crate::ir::{Function, Module, Phase, Value};
use console::style;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;

// ============================================================================
// Core Data Structures
// ============================================================================

/// Dead Code Elimination optimization phase.
///
/// Removes unreachable basic blocks and unused instructions from IR functions
/// while preserving all observable program behavior.
///
/// # Configuration
///
/// - `max_iterations`: Maximum fixed-point iterations (default 10)
/// - `enable_statistics`: Whether to collect and report optimization metrics
/// - `verbose_warnings`: Whether to emit detailed conservative decision warnings
#[derive(Debug, Clone)]
pub struct DeadCodeElimination {
    /// Maximum number of fixed-point iterations before stopping.
    /// Prevents infinite loops in case of algorithm bugs.
    /// Default: 10
    pub max_iterations: usize,

    /// Whether to collect and report detailed optimization statistics.
    /// Default: true
    pub enable_statistics: bool,

    /// Whether to emit warnings for conservative decisions that prevent removal.
    /// Useful for debugging missed optimization opportunities.
    /// Default: false
    pub verbose_warnings: bool,

    /// Statistics from the last optimization run.
    /// Updated after each call to optimize_function or run.
    last_stats: OptimizationStats,
}

/// Statistics collected during Dead Code Elimination optimization.
///
/// Provides metrics about the optimization's effectiveness including
/// the number of instructions and blocks removed, iterations required
/// to reach fixed-point, and any conservative decisions that prevented
/// more aggressive optimization.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OptimizationStats {
    /// Total number of instructions removed across all iterations.
    pub instructions_removed: usize,

    /// Total number of basic blocks removed across all iterations.
    pub blocks_removed: usize,

    /// Number of fixed-point iterations performed.
    /// 1 means no changes were made (optimization was no-op).
    /// Higher values indicate cascading dead code removal.
    pub iterations: usize,

    /// Warnings about conservative decisions that prevented removal.
    /// Empty if `verbose_warnings` is disabled on the optimizer.
    pub conservative_warnings: Vec<ConservativeWarning>,
}

/// Warning about a conservative decision that prevented code removal.
///
/// Used for diagnostics and debugging to understand why certain
/// instructions or blocks were not removed despite appearing unused.
///
/// # Example
///
/// An instruction that computes an unused value but calls a function
/// of unknown purity will generate a warning with reason `UnknownCallPurity`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConservativeWarning {
    /// Human-readable description of the instruction.
    /// Example: "call @unknown_func(42) in block 'entry'"
    pub instruction_debug: String,

    /// The specific reason this instruction was conservatively kept.
    pub reason: ConservativeReason,

    /// Optional: The basic block label where this instruction appears.
    pub block_label: Option<String>,
}

/// Reasons why an instruction was conservatively preserved.
///
/// Each variant represents a specific limitation in the analysis
/// that prevented proving the instruction was safe to remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConservativeReason {
    /// The instruction may alias with other memory locations.
    /// Unable to prove the store is to a purely local allocation.
    MayAlias,

    /// The function being called has unknown purity.
    /// Cannot determine if it has side effects.
    UnknownCallPurity,

    /// The pointer operand escapes the current function.
    /// Store may be observable by caller or other code.
    EscapedPointer,

    /// The instruction may have other side effects.
    /// Includes: I/O operations, volatile access, atomic operations.
    PotentialSideEffect,
}

/// Unique identifier for an instruction within a function.
///
/// Combines block index and instruction offset within that block
/// to provide a stable, comparable identifier for instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstructionIndex {
    /// The basic block containing this instruction.
    pub block_idx: NodeIndex,

    /// The offset of this instruction within the block's instruction list.
    pub inst_offset: usize,
}

// ============================================================================
// Liveness Analysis Data Structures (T036-T037)
// ============================================================================

/// Definition-Use chains for tracking value dependencies.
///
/// Maps each value to the instructions that use it, and each instruction
/// to the values it defines and uses. Essential for liveness analysis.
///
/// # Purpose
///
/// - Identify which instructions produce values that are never used
/// - Track dataflow dependencies between instructions
/// - Support backward dataflow analysis for liveness
///
/// # Example
///
/// ```text
/// t1 = add 10, 20    // defines t1
/// t2 = mul t1, 2     // uses t1, defines t2
/// ret t2             // uses t2
/// ```
///
/// DefUse chains would record:
/// - `t1 -> [mul instruction]` (t1 is used by the mul)
/// - `t2 -> [ret terminator]` (t2 is used by the return)
/// - `mul instruction -> uses: [t1], defines: t2`
#[derive(Debug, Clone)]
struct DefUseChains {
    /// Maps each value to the set of instruction indices that use it.
    /// Key: Value (temporary ID or global), Value: Set of InstructionIndex
    value_to_uses: std::collections::HashMap<Value, HashSet<InstructionIndex>>,

    /// Maps each instruction to the values it uses.
    /// Key: InstructionIndex, Value: Set of Values
    instruction_to_used_values: std::collections::HashMap<InstructionIndex, HashSet<Value>>,

    /// Maps each instruction to the value it defines (if any).
    /// Key: InstructionIndex, Value: Value (typically a temporary)
    instruction_to_defined_value: std::collections::HashMap<InstructionIndex, Value>,
}

/// Liveness information for a value.
///
/// Tracks where a value is first defined, last used, and which blocks
/// reference it. Used to determine if a value's defining instruction
/// can be eliminated.
///
/// # Purpose
///
/// - Determine instruction liveness via backward dataflow analysis
/// - Identify dead code (instructions whose results are never used)
/// - Support aggressive dead store elimination
///
/// # Invariants
///
/// - `first_use <= last_use` (if both are Some)
/// - If `used_in_blocks` is non-empty, `last_use` must be Some
#[derive(Debug, Clone)]
#[allow(dead_code)] // Reserved for future use in advanced liveness analysis
struct LivenessInfo {
    /// The instruction index where this value is first used (earliest in program order).
    first_use: Option<InstructionIndex>,

    /// The instruction index where this value is last used (latest in program order).
    last_use: Option<InstructionIndex>,

    /// Set of basic blocks where this value is referenced.
    /// Used for inter-procedural analysis and phi node handling.
    used_in_blocks: HashSet<NodeIndex>,
}

/// Liveness analyzer using backward dataflow analysis.
///
/// Computes which values are live (will be used in the future) at each
/// program point. Uses def-use chains and backward propagation through
/// the control flow graph.
///
/// # Algorithm
///
/// 1. Build def-use chains for all values
/// 2. Compute gen/kill sets for each block
/// 3. Iterate backward through CFG until fixed point:
///    - live_in[B] = gen[B] ∪ (live_out[B] - kill[B])
///    - live_out[B] = ∪ live_in[S] for all successors S
///
/// # Example
///
/// ```text
/// entry:
///   t1 = add 10, 20    // gen: {}, kill: {t1}
///   t2 = mul t1, 2     // gen: {t1}, kill: {t2}
///   ret t2             // gen: {t2}, kill: {}
/// ```
///
/// After analysis:
/// - live_out[entry] = {t2} (needed by return)
/// - live_in[entry] = {t1, t2} (t1 feeds t2, t2 used by return)
#[derive(Debug)]
struct LivenessAnalyzer {
    /// Def-use chains for tracking value dependencies.
    def_use_chains: DefUseChains,

    /// Gen sets: values used before being defined in each block.
    gen_sets: std::collections::HashMap<NodeIndex, HashSet<Value>>,

    /// Kill sets: values defined in each block.
    kill_sets: std::collections::HashMap<NodeIndex, HashSet<Value>>,

    /// Live-in sets: values live at the start of each block.
    live_in: std::collections::HashMap<NodeIndex, HashSet<Value>>,

    /// Live-out sets: values live at the end of each block.
    live_out: std::collections::HashMap<NodeIndex, HashSet<Value>>,
}

// ============================================================================
// Implementation: LivenessInfo (T039)
// ============================================================================
#[allow(dead_code)]
impl LivenessInfo {
    /// Creates a new LivenessInfo for a dead value (never used).
    fn dead() -> Self {
        Self { first_use: None, last_use: None, used_in_blocks: HashSet::new() }
    }

    /// Creates a new LivenessInfo with the given use information.
    ///
    /// # Arguments
    ///
    /// * `first_use` - The instruction where this value is first used
    /// * `last_use` - The instruction where this value is last used
    /// * `used_in_blocks` - Set of blocks where this value is referenced
    fn with_uses(
        first_use: Option<InstructionIndex>, last_use: Option<InstructionIndex>, used_in_blocks: HashSet<NodeIndex>,
    ) -> Self {
        Self { first_use, last_use, used_in_blocks }
    }

    /// Returns whether this value is live (has at least one use).
    fn is_live(&self) -> bool {
        self.last_use.is_some()
    }
}

// ============================================================================
// Implementation: DefUseChains (T038)
// ============================================================================

impl DefUseChains {
    /// Creates an empty DefUseChains structure.
    fn new() -> Self {
        Self {
            value_to_uses: std::collections::HashMap::new(),
            instruction_to_used_values: std::collections::HashMap::new(),
            instruction_to_defined_value: std::collections::HashMap::new(),
        }
    }

    /// Records that an instruction defines a value.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The instruction that defines the value
    /// * `value` - The value being defined (typically a temporary)
    fn add_definition(&mut self, inst_idx: InstructionIndex, value: crate::ir::Value) {
        self.instruction_to_defined_value.insert(inst_idx, value);
    }

    /// Records that an instruction uses a value.
    ///
    /// Updates both the value-to-uses mapping and the instruction-to-used-values mapping.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The instruction that uses the value
    /// * `value` - The value being used
    fn add_use(&mut self, inst_idx: InstructionIndex, value: crate::ir::Value) {
        // Add to value_to_uses map
        self.value_to_uses.entry(value.clone()).or_default().insert(inst_idx);

        // Add to instruction_to_used_values map
        self.instruction_to_used_values.entry(inst_idx).or_default().insert(value);
    }

    /// Returns the set of instructions that use the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to query
    ///
    /// # Returns
    ///
    /// An empty set if the value is never used, otherwise the set of instruction indices
    /// that reference this value.
    #[allow(dead_code)]
    fn get_uses(&self, value: &crate::ir::Value) -> HashSet<InstructionIndex> {
        self.value_to_uses.get(value).cloned().unwrap_or_default()
    }

    /// Returns the set of values used by the given instruction.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The instruction to query
    ///
    /// # Returns
    ///
    /// An empty set if the instruction uses no values, otherwise the set of values
    /// referenced by this instruction.
    fn get_used_values(&self, inst_idx: &InstructionIndex) -> HashSet<crate::ir::Value> {
        self.instruction_to_used_values.get(inst_idx).cloned().unwrap_or_default()
    }

    /// Returns the value defined by the given instruction, if any.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The instruction to query
    ///
    /// # Returns
    ///
    /// `Some(value)` if the instruction defines a value, `None` otherwise.
    fn get_defined_value(&self, inst_idx: &InstructionIndex) -> Option<&crate::ir::Value> {
        self.instruction_to_defined_value.get(inst_idx)
    }

    /// Checks if the given value has any uses.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check
    ///
    /// # Returns
    ///
    /// `true` if at least one instruction uses this value, `false` otherwise.
    fn has_uses(&self, value: &crate::ir::Value) -> bool {
        self.value_to_uses.get(value).is_some_and(|uses| !uses.is_empty())
    }
}

// ============================================================================
// Escape Analysis Data Structures (T056-T064) - User Story 3
// ============================================================================

/// Escape status of a value (typically an allocation).
///
/// Determines whether a value's address may be observed outside
/// the current function, which affects the safety of removing
/// stores and loads involving that value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EscapeStatus {
    /// The value is purely local - address never leaves the function.
    /// Safe to remove dead stores/loads to this allocation.
    Local,

    /// The value's address is taken (e.g., via GetElementPtr) but may not escape.
    /// Conservative: preserve stores in case of aliasing.
    AddressTaken,

    /// The value escapes the function (returned, passed to calls, stored to memory).
    /// Must preserve all stores - may be observable by callers.
    Escaped,
}

/// Side effect classification for instructions.
///
/// Determines whether an instruction can be safely removed if its
/// result is unused, based on observable effects it may have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SideEffectClass {
    /// Pure computation with no side effects.
    /// Safe to remove if result unused.
    /// Examples: arithmetic, logical operations, GEP
    Pure,

    /// Memory read operation.
    /// Conservative: may have observable effects in concurrent contexts.
    /// Safe to remove if result unused AND source is provably local.
    MemoryRead,

    /// Memory write operation.
    /// Must preserve unless target is provably dead local allocation.
    MemoryWrite,

    /// Has observable side effects.
    /// Never safe to remove.
    /// Examples: I/O, function calls, volatile/atomic operations
    EffectFul,
}

/// Escape analyzer for flow-insensitive escape analysis.
///
/// Identifies which allocations escape the current function,
/// enabling safe removal of stores/loads to provably local allocations.
#[derive(Debug)]
struct EscapeAnalyzer {
    /// Escape status for each value (typically alloca results).
    escape_map: std::collections::HashMap<crate::ir::Value, EscapeStatus>,
}

// ============================================================================
// Implementation: EscapeAnalyzer (T057-T063)
// ============================================================================

impl EscapeAnalyzer {
    /// Creates a new escape analyzer.
    fn new() -> Self {
        Self { escape_map: std::collections::HashMap::new() }
    }

    /// Performs flow-insensitive escape analysis on a function.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    ///
    /// # Algorithm (FR-019, research.md)
    ///
    /// 1. Initialize all allocas as Local
    /// 2. Scan all instructions for escape conditions:
    ///    - Store of alloca pointer → Escaped
    ///    - Call with alloca as argument → Escaped
    ///    - Return of alloca pointer → Escaped
    ///    - GetElementPtr of alloca → AddressTaken
    /// 3. Conservative defaults:
    ///    - Function parameters → Escaped
    ///    - Loaded pointers → Escaped
    fn analyze(&mut self, function: &Function) {
        use crate::ir::InstructionKind;

        // Step 1: Initialize allocas as Local
        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                if let InstructionKind::Alloca { .. } = instruction.kind
                    && let Some(result) = &instruction.result
                {
                    self.escape_map.insert(result.clone(), EscapeStatus::Local);
                }
            }
        }

        // Step 2: Scan for escape conditions
        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                match &instruction.kind {
                    // T059: Store of alloca pointer marks it as Escaped
                    InstructionKind::Store { value, .. } => {
                        if self.is_alloca_value(value) {
                            self.mark_escaped(value);
                        }
                    }

                    // T060: Call with alloca as argument marks it as Escaped
                    InstructionKind::Call { args, .. } => {
                        for arg in args {
                            if self.is_alloca_value(arg) {
                                self.mark_escaped(arg);
                            }
                        }
                    }

                    // T062: GetElementPtr of alloca marks it as AddressTaken
                    InstructionKind::GetElementPtr { base, .. } => {
                        if self.is_alloca_value(base) {
                            self.mark_address_taken(base);
                        }
                    }

                    _ => {}
                }
            }

            // T061: Return of alloca pointer marks it as Escaped
            let terminator = &block.terminator;
            if let crate::ir::TerminatorKind::Return { value, .. } = &terminator.kind
                && self.is_alloca_value(value)
            {
                self.mark_escaped(value);
            }
        }

        // T063: Conservative defaults - function parameters assumed escaped
        // (Already handled: parameters aren't in escape_map, so get_status returns Escaped)
    }

    /// Checks if a value is an alloca result we're tracking.
    fn is_alloca_value(&self, value: &crate::ir::Value) -> bool {
        self.escape_map.contains_key(value)
    }

    /// Marks a value as having its address taken.
    fn mark_address_taken(&mut self, value: &crate::ir::Value) {
        if let Some(status) = self.escape_map.get_mut(value)
            && *status == EscapeStatus::Local
        {
            *status = EscapeStatus::AddressTaken;
        }
    }

    /// Marks a value as escaped.
    fn mark_escaped(&mut self, value: &crate::ir::Value) {
        self.escape_map.insert(value.clone(), EscapeStatus::Escaped);
    }

    /// Gets the escape status of a value.
    ///
    /// # Returns
    ///
    /// - `Local` if provably local allocation
    /// - `AddressTaken` if address computed but may not escape
    /// - `Escaped` if unknown or provably escaped (conservative default)
    fn get_status(&self, value: &crate::ir::Value) -> EscapeStatus {
        self.escape_map.get(value).copied().unwrap_or(EscapeStatus::Escaped)
    }
}

// ============================================================================
// Implementation: SideEffectClass (T064-T065)
// ============================================================================

impl SideEffectClass {
    /// Classifies an instruction based on its side effects.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The instruction to classify
    /// * `escape_analyzer` - Escape analysis results for context
    ///
    /// # Returns
    ///
    /// The side effect classification of the instruction.
    ///
    /// # Classification Rules (FR-006, research.md)
    ///
    /// - **Pure**: Binary ops, unary ops, comparisons, casts, GEP
    /// - **MemoryRead**: Load (may be observable in concurrent contexts)
    /// - **MemoryWrite**: Store, AtomicRMW, AtomicCmpXchg
    /// - **EffectFul**: Calls, I/O, volatile/atomic operations, fences
    #[allow(dead_code)]
    fn classify(instruction: &crate::ir::Instruction, _escape_analyzer: &EscapeAnalyzer) -> Self {
        use crate::ir::InstructionKind;

        match &instruction.kind {
            // Pure instructions
            InstructionKind::Binary { .. }
            | InstructionKind::Unary { .. }
            | InstructionKind::Cast { .. }
            | InstructionKind::GetElementPtr { .. }
            | InstructionKind::Vector { .. }
            | InstructionKind::Alloca { .. } => SideEffectClass::Pure,

            // Memory reads
            InstructionKind::Load { .. } => SideEffectClass::MemoryRead,

            // Memory writes
            InstructionKind::Store { .. } => SideEffectClass::MemoryWrite,

            // Effectful operations
            InstructionKind::Call { .. } | InstructionKind::Phi { .. } => SideEffectClass::EffectFul,
        }
    }
}

// ============================================================================
// Implementation: LivenessAnalyzer (T041-T046)
// ============================================================================

impl LivenessAnalyzer {
    /// Creates a new liveness analyzer.
    fn new() -> Self {
        Self {
            def_use_chains: DefUseChains::new(),
            gen_sets: std::collections::HashMap::new(),
            kill_sets: std::collections::HashMap::new(),
            live_in: std::collections::HashMap::new(),
            live_out: std::collections::HashMap::new(),
        }
    }

    /// Builds def-use chains by scanning all instructions in the function.
    ///
    /// For each instruction:
    /// - Records the value it defines (if any)
    /// - Records all values it uses
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    ///
    /// # Implementation Note (T041)
    ///
    /// This performs a forward pass through all blocks and instructions,
    /// building bidirectional mappings between values and their definitions/uses.
    fn build_def_use_chains(&mut self, function: &Function) {
        // Iterate through all blocks in the CFG
        for block_idx in function.cfg.graph().node_indices() {
            let block = &function.cfg.graph()[block_idx];

            // Process each instruction in the block
            for (inst_offset, instruction) in block.instructions.iter().enumerate() {
                let inst_idx = InstructionIndex { block_idx, inst_offset };

                // Record the value this instruction defines
                if let Some(ref result) = instruction.result {
                    self.def_use_chains.add_definition(inst_idx, result.clone());
                }

                // Record all values this instruction uses
                let used_values = self.extract_used_values(instruction);
                for value in used_values {
                    self.def_use_chains.add_use(inst_idx, value);
                }
            }

            // Also process terminator uses
            let terminator_uses = self.extract_terminator_uses(&block.terminator);
            // Terminator is conceptually at the end of instruction list
            let term_idx = InstructionIndex { block_idx, inst_offset: block.instructions.len() };
            for value in terminator_uses {
                self.def_use_chains.add_use(term_idx, value);
            }
        }
    }

    /// Extracts all values used by an instruction.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The instruction to analyze
    ///
    /// # Returns
    ///
    /// A vector of all values referenced by this instruction.
    fn extract_used_values(&self, instruction: &crate::ir::Instruction) -> Vec<crate::ir::Value> {
        use crate::ir::InstructionKind;
        let mut values = Vec::new();

        match &instruction.kind {
            InstructionKind::Binary { left, right, .. } => {
                values.push(left.clone());
                values.push(right.clone());
            }
            InstructionKind::Unary { operand, .. } => {
                values.push(operand.clone());
            }
            InstructionKind::Load { src, .. } => {
                values.push(src.clone());
            }
            InstructionKind::Store { value, dest } => {
                values.push(value.clone());
                values.push(dest.clone());
            }
            InstructionKind::Call { func, args, .. } => {
                values.push(func.clone());
                values.extend(args.iter().cloned());
            }
            InstructionKind::GetElementPtr { base, index, .. } => {
                values.push(base.clone());
                values.push(index.clone());
            }
            InstructionKind::Cast { value, .. } => {
                values.push(value.clone());
            }
            InstructionKind::Phi { incoming, .. } => {
                values.extend(incoming.iter().map(|(v, _)| v.clone()));
            }
            InstructionKind::Vector { operands, .. } => {
                values.extend(operands.iter().cloned());
            }
            InstructionKind::Alloca { .. } => {
                // Alloca doesn't use any values
            }
        }

        values
    }

    /// Extracts all values used by a terminator.
    ///
    /// # Arguments
    ///
    /// * `terminator` - The terminator to analyze
    ///
    /// # Returns
    ///
    /// A vector of all values referenced by this terminator.
    fn extract_terminator_uses(&self, terminator: &crate::ir::Terminator) -> Vec<crate::ir::Value> {
        use crate::ir::TerminatorKind;
        let mut values = Vec::new();

        match &terminator.kind {
            TerminatorKind::Return { value, .. } => {
                values.push(value.clone());
            }
            TerminatorKind::ConditionalBranch { condition, .. } => {
                values.push(condition.clone());
            }
            TerminatorKind::IndirectBranch { address, .. } => {
                values.push(address.clone());
            }
            TerminatorKind::Switch { value, cases, .. } => {
                values.push(value.clone());
                values.extend(cases.iter().map(|(v, _)| v.clone()));
            }
            TerminatorKind::Branch { .. } | TerminatorKind::Unreachable => {
                // These don't use any values
            }
        }

        values
    }

    /// Computes gen and kill sets for each basic block.
    ///
    /// - **gen[B]**: Values used before being defined in block B
    /// - **kill[B]**: Values defined in block B
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    ///
    /// # Implementation Note (T042)
    ///
    /// For each block, we process instructions in order:
    /// - If a value is used and not yet in kill set, add to gen set
    /// - If a value is defined, add to kill set
    fn compute_gen_kill_sets(&mut self, function: &Function) {
        for block_idx in function.cfg.graph().node_indices() {
            let block = &function.cfg.graph()[block_idx];

            let mut gen_set = HashSet::new();
            let mut kill_set = HashSet::new();

            // Process instructions in forward order
            for (inst_offset, _instruction) in block.instructions.iter().enumerate() {
                let inst_idx = InstructionIndex { block_idx, inst_offset };

                // Add used values to gen (if not already killed)
                let used_values = self.def_use_chains.get_used_values(&inst_idx);
                for value in used_values {
                    if !kill_set.contains(&value) {
                        gen_set.insert(value);
                    }
                }

                // Add defined value to kill
                if let Some(defined_value) = self.def_use_chains.get_defined_value(&inst_idx) {
                    kill_set.insert(defined_value.clone());
                }
            }

            // Process terminator uses
            let term_idx = InstructionIndex { block_idx, inst_offset: block.instructions.len() };
            let used_values = self.def_use_chains.get_used_values(&term_idx);
            for value in used_values {
                if !kill_set.contains(&value) {
                    gen_set.insert(value);
                }
            }

            self.gen_sets.insert(block_idx, gen_set);
            self.kill_sets.insert(block_idx, kill_set);
        }
    }

    /// Performs backward dataflow analysis to compute live variable sets.
    ///
    /// Iterates until fixed point or maximum iterations reached:
    /// - live_out[B] = ∪ live_in[S] for all successors S of B
    /// - live_in[B] = gen[B] ∪ (live_out[B] - kill[B])
    ///
    /// # Arguments
    ///
    /// * `function` - The function to analyze
    ///
    /// # Returns
    ///
    /// `true` if convergence was reached, `false` if max iterations exceeded.
    ///
    /// # Implementation Notes (T043-T046)
    ///
    /// - Uses reverse post-order for faster convergence (T044)
    /// - Handles phi nodes specially (T045)
    /// - Maximum 10 iterations with warning (T046)
    fn analyze(&mut self, function: &Function) -> bool {
        const MAX_ITERATIONS: usize = 10;

        // Initialize all live sets to empty
        for block_idx in function.cfg.graph().node_indices() {
            self.live_in.insert(block_idx, HashSet::new());
            self.live_out.insert(block_idx, HashSet::new());
        }

        // Compute reverse post-order for better convergence (T044)
        let rpo = self.compute_reverse_post_order(function);

        // Fixed-point iteration
        let mut iteration = 0;
        let mut changed = true;

        while changed && iteration < MAX_ITERATIONS {
            changed = false;
            iteration += 1;

            // Process blocks in reverse post-order
            for &block_idx in rpo.iter().rev() {
                // Compute live_out[B] = ∪ live_in[S] for all successors S
                let mut new_live_out = HashSet::new();
                for successor in function.cfg.graph().neighbors(block_idx) {
                    if let Some(succ_live_in) = self.live_in.get(&successor) {
                        new_live_out.extend(succ_live_in.iter().cloned());
                    }
                }

                // Compute live_in[B] = gen[B] ∪ (live_out[B] - kill[B])
                let gen_set = self.gen_sets.get(&block_idx).cloned().unwrap_or_default();
                let kill_set = self.kill_sets.get(&block_idx).cloned().unwrap_or_default();

                let mut new_live_in = gen_set.clone();
                for value in &new_live_out {
                    if !kill_set.contains(value) {
                        new_live_in.insert(value.clone());
                    }
                }

                // Check for changes
                let old_live_in = self.live_in.get(&block_idx).cloned().unwrap_or_default();
                let old_live_out = self.live_out.get(&block_idx).cloned().unwrap_or_default();

                if new_live_in != old_live_in || new_live_out != old_live_out {
                    changed = true;
                    self.live_in.insert(block_idx, new_live_in);
                    self.live_out.insert(block_idx, new_live_out);
                }
            }
        }

        // Warn if we didn't converge (T046)
        if iteration >= MAX_ITERATIONS {
            eprintln!(
                "Warning: Liveness analysis did not converge after {} iterations for function '{}'",
                MAX_ITERATIONS, function.name
            );
            return false;
        }

        true
    }

    /// Computes reverse post-order traversal of the CFG.
    ///
    /// # Arguments
    ///
    /// * `function` - The function whose CFG to traverse
    ///
    /// # Returns
    ///
    /// A vector of block indices in reverse post-order.
    ///
    /// # Implementation Note (T044)
    ///
    /// Reverse post-order processes predecessors before successors,
    /// leading to faster convergence in backward dataflow analysis.
    fn compute_reverse_post_order(&self, function: &Function) -> Vec<NodeIndex> {
        use petgraph::visit::{DfsEvent, depth_first_search};

        let mut post_order = Vec::new();
        let entry_node = function.cfg.get_entry_block_index().expect("Function should have entry node");

        depth_first_search(function.cfg.graph(), Some(entry_node), |event| {
            if let DfsEvent::Finish(node, _) = event {
                post_order.push(node);
            }
        });

        // Reverse to get reverse post-order
        post_order.reverse();
        post_order
    }

    /// Checks if an instruction is dead (its result is never used).
    ///
    /// An instruction is dead if it defines a value that has no uses anywhere.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - The instruction to check
    ///
    /// # Returns
    ///
    /// `true` if the instruction defines a value that has no uses.
    fn is_instruction_dead(&self, inst_idx: &InstructionIndex) -> bool {
        if let Some(defined_value) = self.def_use_chains.get_defined_value(inst_idx) {
            !self.def_use_chains.has_uses(defined_value)
        } else {
            // Instructions without results are never considered dead
            // (they may have side effects)
            false
        }
    }
}

// ============================================================================
// Implementation: DeadCodeElimination
// ============================================================================

impl DeadCodeElimination {
    /// Creates a new DCE optimizer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new DCE optimizer with custom settings.
    ///
    /// # Arguments
    ///
    /// * `max_iterations` - Maximum fixed-point iterations (must be > 0)
    /// * `enable_statistics` - Whether to collect optimization metrics
    /// * `verbose_warnings` - Whether to emit conservative decision warnings
    ///
    /// # Panics
    ///
    /// Panics if `max_iterations` is 0.
    pub fn with_config(max_iterations: usize, enable_statistics: bool, verbose_warnings: bool) -> Self {
        assert!(max_iterations > 0, "max_iterations must be > 0");
        Self { max_iterations, enable_statistics, verbose_warnings, last_stats: OptimizationStats::default() }
    }

    /// Returns a reference to the statistics from the last optimization run.
    ///
    /// # Returns
    ///
    /// Statistics including iterations, instructions removed, blocks removed, and warnings.
    pub fn get_statistics(&self) -> &OptimizationStats {
        &self.last_stats
    }

    /// Optimizes a single function with DCE using fixed-point iteration.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to optimize (modified in-place)
    ///
    /// # Returns
    ///
    /// Statistics about the optimization (removals, iterations, warnings)
    ///
    /// # Implementation Notes (T077-T081)
    ///
    /// - Implements fixed-point iteration loop (FR-010)
    /// - Each iteration: reachability → block removal → liveness → instruction removal
    /// - Tracks changes (blocks/instructions removed) for convergence detection
    /// - Continues until no changes occur (fixed-point reached) or max iterations hit
    /// - Emits warning if max_iterations exceeded without convergence
    /// - Updates phi node incoming lists when removing blocks (T026)
    /// - Verifies SSA form preservation after block removal (T027)
    /// - Preserves debug information and source location metadata (T028)
    fn optimize_function(&mut self, function: &mut Function) -> OptimizationStats {
        let mut stats = OptimizationStats::new();

        // T077-T078: Fixed-point iteration loop
        for iteration in 1..=self.max_iterations {
            stats.iterations = iteration;

            let mut changed = false;

            // Phase 1: Reachability analysis and block removal
            let reachable_blocks = ReachabilityAnalyzer::analyze(&function.cfg);

            let blocks_to_remove: Vec<String> = function
                .cfg
                .blocks()
                .filter(|block| {
                    let block_idx = function.cfg.find_block_by_label(&block.label);
                    if let Some(idx) = block_idx { !reachable_blocks.contains(&idx) } else { false }
                })
                .map(|block| block.label.to_string())
                .collect();

            // T079: Track changes - blocks removed
            if !blocks_to_remove.is_empty() {
                changed = true;
            }

            // Update phi nodes before removing blocks
            self.update_phi_nodes_for_removed_blocks(&mut function.cfg, &blocks_to_remove);

            // Remove unreachable blocks
            for label in &blocks_to_remove {
                if self.verbose_warnings
                    && let Some(block) = function.cfg.get_block(label)
                {
                    self.log_block_removal_debug_info(block);
                }

                if function.cfg.remove_block(label) {
                    stats.blocks_removed += 1;
                }
            }

            // Verify SSA form preservation (T027)
            if cfg!(debug_assertions) {
                self.verify_ssa_form_preservation(&function.cfg);
            }

            // Phase 2: Dead instruction elimination using liveness analysis
            let dead_insts_removed = self.remove_dead_instructions(function);

            // T079: Track changes - instructions removed
            if dead_insts_removed > 0 {
                changed = true;
                stats.instructions_removed += dead_insts_removed;
            }

            // T080: Convergence detection - fixed-point reached when no changes
            if !changed {
                break;
            }

            // T081: Check if max_iterations reached without convergence
            if iteration == self.max_iterations && self.verbose_warnings {
                eprintln!(
                    "Warning: DCE did not converge for function '{}' after {} iterations",
                    function.name, self.max_iterations
                );
            }
        }

        // Store statistics for get_statistics()
        self.last_stats = stats.clone();

        stats
    }

    /// Removes dead instructions using liveness analysis and escape analysis.
    ///
    /// Performs backward dataflow analysis to identify instructions whose
    /// results are never used, then removes them if they have no side effects.
    /// For memory operations, uses escape analysis to determine safety.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to optimize (modified in-place)
    ///
    /// # Returns
    ///
    /// The number of instructions removed.
    ///
    /// # Implementation (T048, T066-T070)
    ///
    /// 1. Build def-use chains
    /// 2. Compute gen/kill sets
    /// 3. Run backward dataflow analysis
    /// 4. Run escape analysis for memory operations
    /// 5. Remove instructions with unused results (if pure/safe)
    /// 6. Remove orphaned allocations (second pass)
    /// 7. Generate conservative warnings for preserved operations
    fn remove_dead_instructions(&mut self, function: &mut Function) -> usize {
        use crate::ir::InstructionKind;

        let mut total_removed = 0;

        // Iterative removal: keep removing until no more changes
        loop {
            // Create and run liveness analyzer
            let mut analyzer = LivenessAnalyzer::new();
            analyzer.build_def_use_chains(function);
            analyzer.compute_gen_kill_sets(function);

            if !analyzer.analyze(function) && self.verbose_warnings {
                eprintln!("Warning: Liveness analysis did not converge for function '{}'", function.name);
            }

            // Create and run escape analyzer (T066)
            let mut escape_analyzer = EscapeAnalyzer::new();
            escape_analyzer.analyze(function);

            // Collect dead instructions to remove
            let mut dead_instructions = Vec::new();

            for block_idx in function.cfg.graph().node_indices() {
                let block = &function.cfg.graph()[block_idx];

                for (inst_offset, instruction) in block.instructions.iter().enumerate() {
                    let inst_idx = InstructionIndex { block_idx, inst_offset };

                    // Check if instruction result is unused
                    let is_dead = analyzer.is_instruction_dead(&inst_idx);

                    // Determine if instruction can be safely removed
                    let can_remove = match &instruction.kind {
                        // T066: Remove stores to Local allocations if dead
                        // A store is removable if destination is local and there are no loads
                        InstructionKind::Store { dest, .. } => {
                            match escape_analyzer.get_status(dest) {
                                EscapeStatus::Local => {
                                    // Check if there are any loads from this alloca
                                    let has_loads = self.has_loads_from(function, dest);
                                    !has_loads
                                }
                                // T068: Conservatively preserve stores to escaped/address-taken allocations
                                EscapeStatus::AddressTaken | EscapeStatus::Escaped => {
                                    if self.verbose_warnings {
                                        // T070: Generate warning for conservatively preserved store
                                        self.log_conservative_warning(
                                            &inst_idx,
                                            ConservativeReason::EscapedPointer,
                                            "Store to escaped or address-taken allocation preserved",
                                        );
                                    }
                                    false
                                }
                            }
                        }

                        // T067: Remove loads if result is unused and source is local
                        InstructionKind::Load { src, .. } => {
                            if is_dead {
                                match escape_analyzer.get_status(src) {
                                    EscapeStatus::Local => true,
                                    EscapeStatus::AddressTaken | EscapeStatus::Escaped => {
                                        if self.verbose_warnings {
                                            // T070: Generate warning for conservatively preserved load
                                            self.log_conservative_warning(
                                                &inst_idx,
                                                ConservativeReason::EscapedPointer,
                                                "Load from escaped or address-taken allocation preserved",
                                            );
                                        }
                                        false
                                    }
                                }
                            } else {
                                false
                            }
                        }

                        // Remove Alloca if the allocated memory is never used
                        InstructionKind::Alloca { .. } => {
                            if let Some(result) = &instruction.result {
                                !analyzer.def_use_chains.has_uses(result)
                            } else {
                                false
                            }
                        }

                        // T069: Never remove calls (unknown purity/side effects)
                        InstructionKind::Call { .. } => {
                            if is_dead && self.verbose_warnings {
                                self.log_conservative_warning(
                                    &inst_idx,
                                    ConservativeReason::UnknownCallPurity,
                                    "Call instruction preserved due to potential side effects",
                                );
                            }
                            false
                        }

                        // Other instructions: use existing pure instruction check
                        _ => is_dead && self.is_pure_instruction(instruction),
                    };

                    if can_remove {
                        dead_instructions.push((block.label.to_string(), inst_offset));
                    }
                }
            }

            // If no instructions to remove, we're done
            if dead_instructions.is_empty() {
                break;
            }

            // Remove dead instructions (in reverse order to maintain indices)
            for (block_label, _) in dead_instructions.iter().rev() {
                // Group by block and remove all at once
                let block_dead_offsets: Vec<usize> = dead_instructions
                    .iter()
                    .filter(|(label, _)| label == block_label)
                    .map(|(_, offset)| *offset)
                    .collect();

                if let Some(block) = function.cfg.get_block_mut(block_label) {
                    // Remove in reverse order to maintain indices
                    for &offset in block_dead_offsets.iter().rev() {
                        if offset < block.instructions.len() {
                            block.instructions.remove(offset);
                            total_removed += 1;
                        }
                    }
                }
            }
        }

        total_removed
    }

    /// Checks if there are any Load instructions from the given pointer.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to search
    /// * `ptr` - The pointer value to check
    ///
    /// # Returns
    ///
    /// `true` if at least one Load instruction reads from this pointer.
    fn has_loads_from(&self, function: &Function, ptr: &crate::ir::Value) -> bool {
        use crate::ir::InstructionKind;

        for block in function.cfg.blocks() {
            for instruction in &block.instructions {
                if let InstructionKind::Load { src, .. } = &instruction.kind
                    && src == ptr
                {
                    return true;
                }
            }
        }
        false
    }

    /// Logs a conservative warning for a preserved instruction.
    ///
    /// # Arguments
    ///
    /// * `inst_idx` - Index of the instruction
    /// * `reason` - Reason for conservative preservation
    /// * `message` - Human-readable explanation
    ///
    /// # Implementation (T070)
    ///
    /// Emits warnings when verbose_warnings is enabled to aid debugging.
    fn log_conservative_warning(&self, inst_idx: &InstructionIndex, _reason: ConservativeReason, message: &str) {
        if self.verbose_warnings {
            eprintln!(
                "Conservative DCE: Block {}, Instruction {}: {}",
                inst_idx.block_idx.index(),
                inst_idx.inst_offset,
                message
            );
        }
    }

    /// Checks if an instruction is pure (has no side effects).
    ///
    /// Pure instructions can be safely removed if their results are unused.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The instruction to check
    ///
    /// # Returns
    ///
    /// `true` if the instruction is pure, `false` if it may have side effects.
    ///
    /// # Implementation (T049)
    ///
    /// Conservative classification:
    /// - Pure: arithmetic, logical, comparison operations
    /// - Impure: stores, calls, I/O operations
    fn is_pure_instruction(&self, instruction: &crate::ir::Instruction) -> bool {
        use crate::ir::InstructionKind;

        matches!(
            instruction.kind,
            InstructionKind::Binary { .. }
                | InstructionKind::Unary { .. }
                | InstructionKind::Cast { .. }
                | InstructionKind::GetElementPtr { .. }
                | InstructionKind::Load { .. } // Conservative: treat loads as pure (may need refinement)
        )
    }

    /// Updates phi node incoming lists to remove references to blocks being removed.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The control-flow graph containing phi nodes
    /// * `removed_labels` - Labels of blocks being removed
    ///
    /// # Implementation
    ///
    /// Iterates through all blocks and their instructions, filtering out incoming
    /// phi entries that reference blocks in `removed_labels`.
    fn update_phi_nodes_for_removed_blocks(
        &self, cfg: &mut crate::ir::cfg::ControlFlowGraph, removed_labels: &[String],
    ) {
        use crate::ir::instruction::InstructionKind;

        // Iterate over all blocks in the CFG
        for block in cfg.blocks_mut() {
            // Iterate over all instructions in the block
            for instruction in &mut block.instructions {
                // Check if this is a Phi instruction
                if let InstructionKind::Phi { incoming, .. } = &mut instruction.kind {
                    // Filter out incoming edges from removed blocks
                    incoming.retain(|(_, predecessor_label)| !removed_labels.contains(predecessor_label));
                }
            }
        }
    }

    /// Verifies that SSA form is preserved after block removal (T027).
    ///
    /// # Arguments
    ///
    /// * `cfg` - The control-flow graph to verify
    ///
    /// # Verification Checks
    ///
    /// 1. All phi node incoming edges reference existing blocks
    /// 2. All phi node incoming edges reference actual CFG predecessors
    /// 3. No phi nodes have zero incoming edges (degenerate case)
    ///
    /// # Panics
    ///
    /// Panics in debug builds if SSA form invariants are violated.
    fn verify_ssa_form_preservation(&self, cfg: &crate::ir::cfg::ControlFlowGraph) {
        use crate::ir::instruction::InstructionKind;
        use petgraph::Direction;

        // Build set of all block labels for quick lookup
        let all_labels: std::collections::HashSet<_> = cfg.blocks().map(|b| b.label.as_ref()).collect();

        // Verify each block
        for block in cfg.blocks() {
            let block_idx = cfg.find_block_by_label(&block.label);

            // Get actual CFG predecessors for this block
            let predecessors: std::collections::HashSet<String> = if let Some(idx) = block_idx {
                cfg.graph()
                    .neighbors_directed(idx, Direction::Incoming)
                    .map(|pred_idx| cfg.graph()[pred_idx].label.to_string())
                    .collect()
            } else {
                std::collections::HashSet::new()
            };

            // Check each phi instruction
            for instruction in &block.instructions {
                if let InstructionKind::Phi { incoming, .. } = &instruction.kind {
                    // Check 1: No phi node should have zero incoming edges
                    assert!(
                        !incoming.is_empty(),
                        "SSA violation: Phi node in block '{}' has zero incoming edges",
                        block.label
                    );

                    // Check 2: All incoming edges reference existing blocks
                    for (_, pred_label) in incoming {
                        assert!(
                            all_labels.contains(pred_label.as_str()),
                            "SSA violation: Phi node in block '{}' references non-existent block '{}'",
                            block.label,
                            pred_label
                        );

                        // Check 3: All incoming edges correspond to actual CFG predecessors
                        assert!(
                            predecessors.contains(pred_label),
                            "SSA violation: Phi node in block '{}' references '{}' which is not a CFG predecessor",
                            block.label,
                            pred_label
                        );
                    }
                }
            }
        }
    }

    /// Logs debug information when removing a block (T028).
    ///
    /// # Arguments
    ///
    /// * `block` - The block being removed
    ///
    /// # Implementation
    ///
    /// Outputs source location information for the removed block and its instructions
    /// to aid in debugging and understanding optimization decisions.
    fn log_block_removal_debug_info(&self, block: &crate::ir::basic_block::BasicBlock) {
        eprintln!("DCE: Removing unreachable block '{}'", block.label);
        eprintln!("     Source location: {}", block.source_span);

        if !block.instructions.is_empty() {
            eprintln!("     Contained {} instruction(s):", block.instructions.len());
            for (idx, instr) in block.instructions.iter().enumerate() {
                eprintln!("       [{}] {:?} at {}", idx, instr.kind, instr.debug_info.source_span);
            }
        }

        if block.terminator.is_terminator() {
            eprintln!("     Terminator: {:?} at {}", block.terminator.kind, block.terminator.debug_info.source_span);
        }
    }
}

// ============================================================================
// Private Analysis Components
// ============================================================================

/// Analyzer for identifying reachable basic blocks via CFG traversal.
///
/// Uses depth-first search starting from the function entry block to
/// mark all blocks that can be reached through control-flow edges.
struct ReachabilityAnalyzer;

impl ReachabilityAnalyzer {
    /// Analyzes reachability starting from the function entry block.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The control-flow graph to analyze
    ///
    /// # Returns
    ///
    /// A `HashSet` of `NodeIndex` values for all reachable blocks.
    ///
    /// # Panics
    ///
    /// Returns empty set if the CFG has no entry block.
    fn analyze(cfg: &crate::ir::cfg::ControlFlowGraph) -> HashSet<NodeIndex> {
        let mut reachable = HashSet::new();

        // Get the entry block index
        if let Some(entry_idx) = cfg.get_entry_block_index() {
            // Use DFS to traverse all reachable blocks
            let mut dfs = petgraph::visit::Dfs::new(cfg.graph(), entry_idx);
            while let Some(node_idx) = dfs.next(cfg.graph()) {
                reachable.insert(node_idx);
            }
        }

        reachable
    }
}

impl Default for DeadCodeElimination {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            enable_statistics: true,
            verbose_warnings: false,
            last_stats: OptimizationStats::default(),
        }
    }
}

impl Phase for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "Dead Code Elimination"
    }

    fn run(&mut self, module: &mut Module) {
        // T087: Iterate over all functions in the module
        // Collect function names first to avoid borrowing issues
        let function_names: Vec<String> = module.functions().iter().map(|f| f.name.clone()).collect();

        // T089: Initialize aggregated statistics for all functions
        let mut aggregated_stats = OptimizationStats::default();

        // Iterate over all functions in the module and optimize each one
        for name in function_names {
            if let Some(function) = module.get_function_mut(&name) {
                // T088: Skip external function declarations (functions without body)
                // External functions have no basic blocks in their CFG
                if function.cfg.graph().node_count() == 0 {
                    continue; // Skip declarations without implementation
                }

                // Optimize function and collect statistics
                let stats = self.optimize_function(function);

                // T089: Aggregate statistics across all functions
                aggregated_stats.instructions_removed += stats.instructions_removed;
                aggregated_stats.blocks_removed += stats.blocks_removed;
                aggregated_stats.iterations += stats.iterations;
                aggregated_stats.conservative_warnings.extend(stats.conservative_warnings);
            }
        }

        // Store aggregated statistics (T090: module-level reporting)
        self.last_stats = aggregated_stats;
        let dce_stats = self.get_statistics();
        if dce_stats.had_effect() {
            let mut output = String::with_capacity(256);
            // Use write! macro to write directly into the buffer
            writeln!(output, "\n{}", style("Dead Code Elimination Statistics:").cyan().bold()).unwrap();
            writeln!(output, "✂️  Instructions removed: {}", dce_stats.instructions_removed).unwrap();
            writeln!(output, "🗑️  Blocks removed: {}", dce_stats.blocks_removed).unwrap();
            writeln!(output, "🔄  Iterations to convergence: {}", dce_stats.iterations).unwrap();

            if !dce_stats.conservative_warnings.is_empty() {
                writeln!(output, "⚠️  Conservative warnings: {}", dce_stats.conservative_warnings.len()).unwrap();

                // Reserve additional space for warnings
                output.reserve(dce_stats.conservative_warnings.len() * 50);

                for warning in &dce_stats.conservative_warnings {
                    writeln!(output, "    - {}", style(&warning.reason).yellow()).unwrap();
                }
            }

            print!("{}", output);
        } else {
            println!("{}", style("No dead code found - module already optimal").green());
        }
    }
}

// ============================================================================
// Implementation: OptimizationStats
// ============================================================================

impl OptimizationStats {
    /// Creates empty statistics (no removals).
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if any code was removed.
    pub fn had_effect(&self) -> bool {
        self.instructions_removed > 0 || self.blocks_removed > 0
    }

    /// Formats statistics for human-readable display.
    pub fn format_report(&self, function_name: &str) -> String {
        format!(
            "📊 DCE Statistics for '{}':\n\
             ✂️  Instructions removed: {}\n\
             🗑️  Blocks removed: {}\n\
             🔄 Iterations: {}\n\
             ⚠️  Conservative warnings: {}",
            function_name,
            self.instructions_removed,
            self.blocks_removed,
            self.iterations,
            self.conservative_warnings.len()
        )
    }
}

impl fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OptimizationStats {{ instructions: {}, blocks: {}, iterations: {}, warnings: {} }}",
            self.instructions_removed,
            self.blocks_removed,
            self.iterations,
            self.conservative_warnings.len()
        )
    }
}

// ============================================================================
// Implementation: ConservativeWarning
// ============================================================================

impl ConservativeWarning {
    /// Creates a new warning.
    ///
    /// # Arguments
    ///
    /// * `instruction_debug` - Human-readable instruction description
    /// * `reason` - The conservative reason for keeping the instruction
    /// * `block_label` - Optional block location for context
    pub fn new(instruction_debug: String, reason: ConservativeReason, block_label: Option<String>) -> Self {
        Self { instruction_debug, reason, block_label }
    }
}

impl fmt::Display for ConservativeWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref label) = self.block_label {
            write!(f, "⚠️  Conservative: {} in block '{}' (reason: {})", self.instruction_debug, label, self.reason)
        } else {
            write!(f, "⚠️  Conservative: {} (reason: {})", self.instruction_debug, self.reason)
        }
    }
}

// ============================================================================
// Implementation: ConservativeReason
// ============================================================================

impl ConservativeReason {
    /// Returns a human-readable explanation of this reason.
    pub fn explanation(&self) -> &'static str {
        match self {
            ConservativeReason::MayAlias => "instruction may alias with other memory locations",
            ConservativeReason::UnknownCallPurity => "function call has unknown purity (may have side effects)",
            ConservativeReason::EscapedPointer => "pointer operand escapes the current function",
            ConservativeReason::PotentialSideEffect => "instruction may have other observable side effects",
        }
    }
}

impl fmt::Display for ConservativeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.explanation())
    }
}

// ============================================================================
// Implementation: InstructionIndex
// ============================================================================

impl InstructionIndex {
    /// Creates a new instruction index.
    ///
    /// # Arguments
    ///
    /// * `block_idx` - The block's NodeIndex in the CFG
    /// * `inst_offset` - The instruction's offset within the block
    pub fn new(block_idx: NodeIndex, inst_offset: usize) -> Self {
        Self { block_idx, inst_offset }
    }
}

impl fmt::Display for InstructionIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block[{}].inst[{}]", self.block_idx.index(), self.inst_offset)
    }
}
