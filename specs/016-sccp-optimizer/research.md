# SCCP Optimizer Research Document

**Feature**: Sparse Conditional Constant Propagation Optimizer  
**Branch**: 016-sccp-optimizer  
**Date**: 2025-11-17

## Executive Summary

This document provides comprehensive research and analysis for implementing the Sparse Conditional Constant Propagation (SCCP) optimization algorithm in the jsavrs Rust compiler. SCCP is a powerful dataflow analysis technique that simultaneously performs constant propagation and dead code elimination by leveraging SSA form to efficiently track constant values through the program while respecting control flow. This research synthesizes the foundational Wegman-Zadeck algorithm with modern compiler implementation practices and Rust-specific design patterns.

## 1. SCCP Algorithm Fundamentals

### 1.1 Historical Context and Motivation

Sparse Conditional Constant Propagation was introduced by Wegman and Zadeck in their seminal 1991 paper "Constant Propagation with Conditional Branches". The algorithm addresses two key limitations of traditional constant propagation:

1. **Interprocedural precision**: Traditional algorithms struggle with conditional branches that guard constant values
2. **Efficiency**: Dense dataflow analysis requires O(n²) or worse complexity for large functions

By leveraging the sparse SSA form, SCCP achieves linear O(edges) complexity while simultaneously performing constant propagation, unreachable code elimination, and dead branch elimination in a single unified pass.

**Key Innovation**: SCCP operates on the insight that in SSA form, constant information flows along def-use chains (SSA edges) and control-flow edges simultaneously. By maintaining both types of edges in separate worklists, the algorithm can efficiently propagate constants while discovering which code paths are actually executable.

### 1.2 Three-Level Lattice Theory

SCCP uses a flat three-level lattice for each SSA value:

```text
         Top (⊤)
        /   |   \
Constant(c₁) Constant(c₂) ... Constant(cₙ)
        \   |   /
       Bottom (⊥)
```

**Lattice Properties**:

- **Top (⊤)**: Optimistically unknown - value not yet determined, potentially constant
- **Constant(c)**: Proven to be constant value `c` on all executable paths
- **Bottom (⊥)**: Pessimistically varying - multiple different values or unknown at compile time

**Partial Order**: Top ⊑ Constant(c) ⊑ Bottom for any constant c

**Meet Operation** (greatest lower bound):
- meet(⊤, x) = x (Top is top element, meet with anything yields that thing)
- meet(⊥, x) = ⊥ (Bottom is bottom element, meet with anything yields Bottom)
- meet(Constant(c), Constant(c)) = Constant(c) (same constants)
- meet(Constant(c₁), Constant(c₂)) = ⊥ if c₁ ≠ c₂ (different constants are varying)

**Monotonicity**: The lattice must only descend (Top → Constant → Bottom), never ascend. This ensures termination since there are only two possible state transitions per value (Top→Constant, Constant→Bottom or Top→Bottom).

**Soundness**: If a value reaches Bottom, it may take on multiple values at runtime. If it remains at Constant(c), it is proven to always equal c. Top values in unreachable code are acceptable but should be handled during validation.

### 1.3 Dual Worklist Algorithm

SCCP maintains two worklists to efficiently process dataflow information:

**SSAWorkList**: Queue of (definition → use) edges
- Triggered when a value's lattice state changes from Top→Constant or Constant→Bottom
- Contains SSA edges that need re-evaluation because the source value changed
- Ensures uses are updated when definitions become more precise

**FlowWorkList**: Queue of (predecessor → successor) control-flow edges
- Triggered when a new CFG edge becomes executable
- Contains control-flow edges whose destination blocks need visiting
- Ensures newly reachable code is analyzed

**Processing Algorithm**:
```text
1. Initialize:
   - Set all SSA values to Top (except parameters/globals → Bottom)
   - Mark entry block edges as executable
   - Add entry block to FlowWorkList

2. While SSAWorkList ∪ FlowWorkList ≠ ∅:
   
   a. Process SSAWorkList:
      - Pop (def_value → use_instruction) edge
      - If use_instruction's block is executable:
        - Call VisitInstruction(use_instruction)
   
   b. Process FlowWorkList:
      - Pop (pred_block → succ_block) edge
      - If succ_block not yet marked executable:
        - Mark succ_block as executable
        - Visit all instructions in succ_block (including phi nodes)
      - Evaluate terminator of pred_block to update outgoing edges

3. Rewrite IR based on lattice results
```

### 1.4 Executable Edge Tracking

**Executable Edges Set**: HashSet<(BlockId, BlockId)> tracking which CFG edges are proven reachable
- Initially contains only (Entry, FirstSuccessor) edges
- Grown as conditional branches with constant conditions determine single paths
- Critical for correct phi node evaluation (only executable predecessor values contribute)

**Executable Blocks Set**: HashSet<BlockId> tracking which blocks are proven reachable
- Initially contains only entry block
- A block becomes executable the first time any incoming edge is marked executable
- Used during IR rewrite to remove unreachable blocks

**Edge Processing Invariant**: Each CFG edge is processed at most once (when first marked executable). This is proven by maintaining a processed_edges HashSet and asserting before marking any edge.

## 2. Instruction Evaluation (Abstract Interpretation)

### 2.1 Binary and Unary Operations

**Abstract Interpretation Rules**:

For `result = op(operand₁, operand₂, ...)`:

1. **If any operand is Bottom** → result is Bottom (conservative)
2. **If all operands are Constant** → compute result:
   - Use checked arithmetic (checked_add, checked_mul, etc.)
   - If operation succeeds → result is Constant(computed_value)
   - If operation fails (overflow, division by zero) → result is Bottom
3. **If any operand is Top and none are Bottom** → result remains Top (optimistic)

**Type-Specific Evaluation**:

**Integer Types** (I8, I16, I32, I64, U8, U16, U32, U64):
```rust
match (left_lattice, right_lattice) {
    (Constant(IrLiteralValue::I32(a)), Constant(IrLiteralValue::I32(b))) => {
        match instruction.kind {
            InstructionKind::Binary(BinaryOp::Add) => {
                match a.checked_add(b) {
                    Some(result) => Constant(IrLiteralValue::I32(result)),
                    None => Bottom  // Overflow
                }
            }
            InstructionKind::Binary(BinaryOp::Div) => {
                if b == 0 {
                    Bottom  // Division by zero
                } else {
                    match a.checked_div(b) {
                        Some(result) => Constant(IrLiteralValue::I32(result)),
                        None => Bottom  // Overflow (MIN / -1)
                    }
                }
            }
            // ... other operations
        }
    }
    (Bottom, _) | (_, Bottom) => Bottom,
    _ => Top  // At least one operand is Top
}
```

**Floating-Point Types** (F32, F64):
```rust
match (left_lattice, right_lattice) {
    (Constant(IrLiteralValue::F64(a)), Constant(IrLiteralValue::F64(b))) => {
        let result = match instruction.kind {
            InstructionKind::Binary(BinaryOp::Add) => a + b,
            InstructionKind::Binary(BinaryOp::Mul) => a * b,
            InstructionKind::Binary(BinaryOp::Div) => a / b,
            // ... other operations
        };
        
        // IEEE 754 special value handling
        if result.is_nan() || result.is_infinite() {
            // Decision: Propagate NaN/Inf as constants or mark Bottom?
            // Conservative: Bottom to avoid incorrect optimizations
            // Aggressive: Constant(NaN/Inf) if semantics are well-defined
            Bottom  // Conservative choice
        } else {
            Constant(IrLiteralValue::F64(result))
        }
    }
    (Bottom, _) | (_, Bottom) => Bottom,
    _ => Top
}
```

**Boolean Type**:
```rust
match (left_lattice, right_lattice) {
    (Constant(IrLiteralValue::Bool(a)), Constant(IrLiteralValue::Bool(b))) => {
        let result = match instruction.kind {
            InstructionKind::Binary(BinaryOp::And) => a && b,
            InstructionKind::Binary(BinaryOp::Or) => a || b,
            InstructionKind::Binary(BinaryOp::Eq) => a == b,
            InstructionKind::Binary(BinaryOp::Ne) => a != b,
            // ... other logical operations
        };
        Constant(IrLiteralValue::Bool(result))
    }
    (Bottom, _) | (_, Bottom) => Bottom,
    _ => Top
}
```

### 2.2 Phi Node Evaluation

Phi nodes require special handling because they merge values from multiple control-flow paths:

```rust
fn evaluate_phi(phi: &PhiNode, lattice: &HashMap<Value, LatticeValue>, 
                executable_edges: &HashSet<(BlockId, BlockId)>) -> LatticeValue {
    let mut result = Top;
    
    for (incoming_value, predecessor_block) in phi.incoming.iter() {
        let edge = (predecessor_block, phi.block);
        
        // Only consider values from executable predecessor edges
        if !executable_edges.contains(&edge) {
            continue;
        }
        
        let incoming_lattice = lattice.get(incoming_value).unwrap_or(&Bottom);
        result = meet(result, *incoming_lattice);
        
        // Early termination: if we reach Bottom, no need to check more
        if result == Bottom {
            break;
        }
    }
    
    result
}
```

**Key Invariants**:
1. Only executable predecessor edges contribute to the phi's value
2. If no predecessors are executable, phi remains Top (block is unreachable)
3. If all executable predecessors provide the same Constant(c), phi becomes Constant(c)
4. If executable predecessors provide different constants, phi becomes Bottom
5. If any executable predecessor provides Bottom, phi becomes Bottom

### 2.3 Memory Operations

**Load Instructions**:
- **Conservative approach** (required): Mark all Load results as Bottom
- **Optimistic approach** (future enhancement): If alias analysis proves the loaded address points to a compile-time constant (e.g., constant global), propagate that constant
- **Rationale**: Without alias analysis, we cannot prove that memory hasn't been modified between store and load

**Store Instructions**:
- Do not propagate constants through Store operations
- Store side effects prevent constant propagation across stores
- Mark any dependent computations as Bottom

**Call Instructions**:
- Mark all Call results as Bottom (functions may return non-constant values)
- Side effects of calls prevent constant propagation
- Exception: Pure functions with constant arguments could theoretically be evaluated, but requires interprocedural analysis (out of scope)

### 2.4 Type Casts

**Safe Casts** (preserve value):
```rust
match (source_lattice, target_type) {
    (Constant(IrLiteralValue::I32(v)), IrType::I64) => {
        // Sign extension: i32 → i64 is safe
        Constant(IrLiteralValue::I64(v as i64))
    }
    (Constant(IrLiteralValue::U32(v)), IrType::U64) => {
        // Zero extension: u32 → u64 is safe
        Constant(IrLiteralValue::U64(v as u64))
    }
    (Constant(IrLiteralValue::I32(v)), IrType::F64) => {
        // i32 → f64 is safe (f64 can represent all i32 values exactly)
        Constant(IrLiteralValue::F64(v as f64))
    }
    // ... other safe casts
}
```

**Potentially Lossy Casts** (mark as Bottom):
```rust
match (source_lattice, target_type) {
    (Constant(IrLiteralValue::I64(_)), IrType::I32) => {
        // Truncation: may lose information
        Bottom
    }
    (Constant(IrLiteralValue::F64(_)), IrType::I32) => {
        // Float to int: may truncate fractional part, may overflow
        Bottom
    }
    // ... other potentially lossy casts
}
```

**Type Mismatches** (mark as Bottom):
```rust
match (source_lattice, target_type) {
    (Constant(IrLiteralValue::Bool(_)), IrType::I32) => {
        // Type mismatch: bool → i32 depends on language semantics
        Bottom
    }
    (Constant(IrLiteralValue::String(_)), _) => {
        // Strings are always Bottom (too complex to analyze statically)
        Bottom
    }
    // ... other type mismatches
}
```

## 3. Terminator Evaluation (Branch Analysis)

### 3.1 Conditional Branch Evaluation

```rust
fn evaluate_conditional_branch(condition_value: Value, 
                                 lattice: &HashMap<Value, LatticeValue>,
                                 true_target: BlockId, 
                                 false_target: BlockId,
                                 executable_edges: &mut HashSet<(BlockId, BlockId)>,
                                 flow_worklist: &mut VecDeque<(BlockId, BlockId)>,
                                 current_block: BlockId) {
    let condition_lattice = lattice.get(&condition_value).unwrap_or(&Bottom);
    
    match condition_lattice {
        Constant(IrLiteralValue::Bool(true)) => {
            // Only true branch is reachable
            let edge = (current_block, true_target);
            if executable_edges.insert(edge) {
                flow_worklist.push_back(edge);
            }
        }
        Constant(IrLiteralValue::Bool(false)) => {
            // Only false branch is reachable
            let edge = (current_block, false_target);
            if executable_edges.insert(edge) {
                flow_worklist.push_back(edge);
            }
        }
        Top | Bottom | _ => {
            // Condition is unknown, both branches may be reachable
            let true_edge = (current_block, true_target);
            let false_edge = (current_block, false_target);
            
            if executable_edges.insert(true_edge) {
                flow_worklist.push_back(true_edge);
            }
            if executable_edges.insert(false_edge) {
                flow_worklist.push_back(false_edge);
            }
        }
    }
}
```

**Optimization Opportunity**: When condition is Constant(true) or Constant(false), only one edge becomes executable. This is the key to dead branch elimination.

### 3.2 Switch Statement Evaluation

```rust
fn evaluate_switch(selector_value: Value, 
                   lattice: &HashMap<Value, LatticeValue>,
                   cases: &[(IrLiteralValue, BlockId)],
                   default: BlockId,
                   executable_edges: &mut HashSet<(BlockId, BlockId)>,
                   flow_worklist: &mut VecDeque<(BlockId, BlockId)>,
                   current_block: BlockId) {
    let selector_lattice = lattice.get(&selector_value).unwrap_or(&Bottom);
    
    match selector_lattice {
        Constant(selector_const) => {
            // Find matching case
            let target = cases.iter()
                .find(|(case_value, _)| case_value == selector_const)
                .map(|(_, target)| *target)
                .unwrap_or(default);
            
            let edge = (current_block, target);
            if executable_edges.insert(edge) {
                flow_worklist.push_back(edge);
            }
        }
        Top | Bottom | _ => {
            // Unknown selector, all cases may be reachable
            for (_, target) in cases.iter() {
                let edge = (current_block, *target);
                if executable_edges.insert(edge) {
                    flow_worklist.push_back(edge);
                }
            }
            
            let default_edge = (current_block, default);
            if executable_edges.insert(default_edge) {
                flow_worklist.push_back(default_edge);
            }
        }
    }
}
```

### 3.3 Unconditional Terminators

**Branch** (unconditional jump):
```rust
fn evaluate_branch(target: BlockId, 
                   executable_edges: &mut HashSet<(BlockId, BlockId)>,
                   flow_worklist: &mut VecDeque<(BlockId, BlockId)>,
                   current_block: BlockId) {
    let edge = (current_block, target);
    if executable_edges.insert(edge) {
        flow_worklist.push_back(edge);
    }
}
```

**Return** and **Unreachable**:
- Do not enqueue any new edges (function exit)
- Unreachable explicitly marks the end of a non-returning path

## 4. IR Rewriting Phase

### 4.1 Constant Replacement

After fixed-point convergence, instructions with Constant lattice values can be replaced:

```rust
fn replace_with_constant(instruction: &mut Instruction, 
                         constant_value: IrLiteralValue,
                         new_instructions: &mut Vec<Instruction>) {
    // Replace the instruction with a direct constant assignment
    let constant_instruction = Instruction {
        kind: InstructionKind::Constant(constant_value.clone()),
        result: instruction.result.clone(),
        debug_info: instruction.debug_info.clone(),
    };
    
    new_instructions.push(constant_instruction);
}
```

**Preservation Requirements**:
- Maintain source_span for error reporting
- Preserve debug information
- Update def-use chains if maintained explicitly

### 4.2 Branch Elimination

Convert conditional branches with constant conditions to unconditional branches:

```rust
fn eliminate_constant_branch(block: &mut BasicBlock, 
                              condition_value: &IrLiteralValue,
                              true_target: BlockId,
                              false_target: BlockId) {
    match condition_value {
        IrLiteralValue::Bool(true) => {
            block.terminator = Terminator::Branch(true_target);
        }
        IrLiteralValue::Bool(false) => {
            block.terminator = Terminator::Branch(false_target);
        }
        _ => {
            // Should not happen if lattice is correct
            panic!("Non-boolean constant in conditional branch");
        }
    }
}
```

### 4.3 Unreachable Code Removal

Remove blocks that are not in the executable_blocks set:

```rust
fn remove_unreachable_blocks(function: &mut Function, 
                              executable_blocks: &HashSet<BlockId>) {
    // Remove blocks not in executable set
    function.blocks.retain(|block| executable_blocks.contains(&block.id));
    
    // Update phi nodes to remove incoming edges from removed predecessors
    for block in function.blocks.iter_mut() {
        for instruction in block.instructions.iter_mut() {
            if let InstructionKind::Phi(phi_node) = &mut instruction.kind {
                phi_node.incoming.retain(|(_, pred_block)| {
                    executable_blocks.contains(pred_block)
                });
            }
        }
    }
}
```

### 4.4 Phi Node Simplification

Simplify phi nodes with single incoming value:

```rust
fn simplify_phi_nodes(block: &mut BasicBlock) {
    let mut replacements = Vec::new();
    
    for (idx, instruction) in block.instructions.iter().enumerate() {
        if let InstructionKind::Phi(phi_node) = &instruction.kind {
            if phi_node.incoming.len() == 1 {
                // Single incoming value, replace phi with direct assignment
                let (single_value, _) = phi_node.incoming[0];
                replacements.push((idx, single_value));
            } else if phi_node.incoming.is_empty() {
                // No incoming values (unreachable block or error)
                // Should be caught by validation
            } else {
                // Check if all incoming values are the same
                let first_value = phi_node.incoming[0].0;
                if phi_node.incoming.iter().all(|(v, _)| *v == first_value) {
                    replacements.push((idx, first_value));
                }
            }
        }
    }
    
    // Apply replacements
    for (idx, replacement_value) in replacements.iter().rev() {
        let instruction = block.instructions.remove(*idx);
        
        // Create copy instruction: result = replacement_value
        let copy_instruction = Instruction {
            kind: InstructionKind::Copy(replacement_value.clone()),
            result: instruction.result,
            debug_info: instruction.debug_info,
        };
        
        block.instructions.insert(*idx, copy_instruction);
    }
}
```

### 4.5 SSA and CFG Preservation

**SSA Invariants to Maintain**:
1. Each value has exactly one definition point
2. All uses dominate their definitions (or are phi nodes)
3. Phi nodes only appear at the start of blocks with multiple predecessors

**CFG Invariants to Maintain**:
1. All branch targets exist in the function's block list
2. Entry block is reachable
3. All blocks have valid terminators
4. No dangling edges (edges to non-existent blocks)

**Validation After Rewrite**:
```rust
fn validate_transformations(function: &Function) -> Result<(), String> {
    // 1. Verify SSA form
    verify_ssa_form(function)?;
    
    // 2. Verify CFG validity
    function.cfg.verify()?;
    
    // 3. Verify no Top values remain in executable regions
    // (Top values in unreachable code are acceptable but should have been removed)
    
    // 4. Verify lattice monotonicity (no upward movement)
    // This should be enforced during analysis, but can be re-checked
    
    Ok(())
}
```

## 5. Complexity Analysis and Performance

### 5.1 Time Complexity Proof

**Theorem**: SCCP runs in O(E) time, where E is the total number of SSA edges and CFG edges.

**Proof**:

**SSA Edge Processing**:
- Each SSA value can change lattice state at most twice: Top→Constant, Constant→Bottom (or Top→Bottom directly)
- Each state change triggers at most one enqueue of each outgoing use edge
- Therefore, each SSA edge is processed at most 2 times
- Total SSA edge processing: O(2 * |SSA_edges|) = O(|SSA_edges|)

**CFG Edge Processing**:
- Each CFG edge is marked executable at most once (tracked via HashSet)
- Marking an edge executable triggers visiting its destination block
- Visiting a block processes its instructions and terminator (constant time per instruction)
- Total CFG edge processing: O(|CFG_edges|)

**Total Complexity**: O(|SSA_edges| + |CFG_edges|) = O(E)

**Space Complexity**: O(N) where N is the number of SSA values + basic blocks (for lattice map and executable sets)

### 5.2 Worklist Management Strategies

**VecDeque for Worklists**:
- O(1) push_back and pop_front
- FIFO order ensures breadth-first traversal of dataflow

**HashSet for Duplicate Prevention**:
- O(1) average-case insert and contains
- Prevents redundant work by tracking which edges have been enqueued

**Processing Order**:
- Process FlowWorkList before SSAWorkList in each iteration (or interleave)
- Ensures newly executable blocks are discovered before processing their SSA edges
- Both orders are correct, but FlowWorkList-first may converge faster

### 5.3 Performance Benchmarking Strategy

**Synthetic Test Cases**:
1. Linear chain of 1,000 / 5,000 / 10,000 instructions with constant propagation opportunities
2. Wide CFG (many branches) vs. Deep CFG (nested loops) to test different graph shapes
3. Worst case: All values remain Top (no constants) to test overhead

**Real-World Test Cases**:
1. Compiler bootstrap code (lexer, parser, optimizer itself)
2. Numerical computation kernels (matrix multiplication, FFT)
3. String processing algorithms

**Metrics to Track**:
- Wall-clock time per function size
- Number of worklist iterations to convergence
- Number of constants found vs. total SSA values
- Number of branches eliminated vs. total branches
- Memory usage (peak heap allocation)

**Expected Results**:
- Linear scaling: doubling function size should approximately double runtime
- Convergence iterations should remain constant (typically <100 regardless of function size)
- Memory usage should grow linearly with function size

## 6. Integration with Existing Infrastructure

### 6.1 Phase Trait Implementation

```rust
pub trait Phase {
    fn name(&self) -> &str;
    fn run(&mut self, module: &mut Module) -> Result<(), String>;
}

pub struct ConstantFoldingOptimizer {
    pub verbose: bool,
    pub max_iterations: usize,
    pub sccp_enabled: bool,
}

impl Default for ConstantFoldingOptimizer {
    fn default() -> Self {
        Self {
            verbose: false,
            max_iterations: 100,
            sccp_enabled: true,
        }
    }
}

impl Phase for ConstantFoldingOptimizer {
    fn name(&self) -> &str {
        "Constant Folding (SCCP)"
    }
    
    fn run(&mut self, module: &mut Module) -> Result<(), String> {
        if !self.sccp_enabled {
            return Ok(());
        }
        
        let mut total_stats = OptimizationStatistics::default();
        
        for function in module.functions.iter_mut() {
            let stats = self.transform_function(function)?;
            total_stats.merge(stats);
        }
        
        if self.verbose {
            eprintln!("{}", total_stats);
        }
        
        Ok(())
    }
}

impl ConstantFoldingOptimizer {
    fn transform_function(&mut self, function: &mut Function) -> Result<OptimizationStatistics, String> {
        // 1. Pre-optimization validation
        self.validate_preconditions(function)?;
        
        // 2. Initialize lattice and worklists
        let mut analyzer = SCCPAnalyzer::new(function, self.max_iterations);
        
        // 3. Run fixed-point analysis
        let stats = analyzer.analyze()?;
        
        // 4. Rewrite IR based on analysis results
        analyzer.rewrite(function)?;
        
        // 5. Post-optimization validation
        self.validate_postconditions(function, &analyzer.lattice)?;
        
        Ok(stats)
    }
}
```

### 6.2 Coordination with Dead Code Elimination

**Option 1: SCCP marks dead code, DCE removes it**
- SCCP identifies constant values and unreachable blocks
- Marks dead instructions and blocks with metadata
- DCE pass removes marked code in separate phase
- **Advantage**: Clear separation of concerns, easier to test independently
- **Disadvantage**: Two passes required

**Option 2: SCCP performs complete cleanup**
- SCCP removes unreachable blocks and dead instructions immediately
- No need for separate DCE pass (or DCE only handles other dead code patterns)
- **Advantage**: Single-pass optimization, potentially faster
- **Disadvantage**: More complex SCCP implementation

**Recommended**: Option 1 for initial implementation (cleaner separation), with Option 2 as future optimization if profiling shows significant overhead.

### 6.3 Error Handling and Diagnostics

```rust
#[derive(Debug, thiserror::Error)]
pub enum SCCPError {
    #[error("Pre-optimization validation failed: {0}")]
    PreValidationFailed(String),
    
    #[error("Post-optimization validation failed: {0}")]
    PostValidationFailed(String),
    
    #[error("Maximum iterations ({0}) exceeded without convergence")]
    MaxIterationsExceeded(usize),
    
    #[error("SSA form violation: {0}")]
    SSAViolation(String),
    
    #[error("CFG integrity violation: {0}")]
    CFGViolation(String),
    
    #[error("Lattice invariant violation: {0}")]
    LatticeInvariantViolation(String),
}
```

**Logging Strategy**:
- Use `eprintln!` for warnings (max iterations, unexpected patterns)
- Use `Result` return type for hard errors (validation failures)
- Verbose mode logs each worklist operation and lattice state change
- Statistics always collected, printed only in verbose mode

## 7. Testing Strategy

### 7.1 Unit Tests (Per Module)

**lattice.rs**:
```rust
#[test]
fn test_meet_operation() {
    assert_eq!(meet(Top, Constant(5)), Constant(5));
    assert_eq!(meet(Constant(5), Constant(5)), Constant(5));
    assert_eq!(meet(Constant(5), Constant(6)), Bottom);
    assert_eq!(meet(Bottom, Constant(5)), Bottom);
    assert_eq!(meet(Bottom, Bottom), Bottom);
}

#[test]
fn test_lattice_order() {
    assert!(Top < Constant(42));
    assert!(Constant(42) < Bottom);
    assert!(Top < Bottom);
}

#[test]
fn test_integer_arithmetic() {
    // Test all integer types and operations
    // Test overflow behavior (checked_add returns None → Bottom)
    // Test division by zero → Bottom
}

#[test]
fn test_float_arithmetic() {
    // Test NaN and infinity handling
    // Test IEEE 754 special cases
}
```

**worklist.rs**:
```rust
#[test]
fn test_ssa_worklist_duplicate_prevention() {
    let mut worklist = SSAWorkList::new();
    let edge = (Value::Temp(1), InstructionId::new(1));
    
    assert!(worklist.enqueue(edge));
    assert!(!worklist.enqueue(edge));  // Duplicate prevented
    
    assert_eq!(worklist.dequeue(), Some(edge));
    assert_eq!(worklist.dequeue(), None);  // Empty
}

#[test]
fn test_flow_worklist_fifo_order() {
    let mut worklist = FlowWorkList::new();
    worklist.enqueue((BlockId(1), BlockId(2)));
    worklist.enqueue((BlockId(2), BlockId(3)));
    
    assert_eq!(worklist.dequeue(), Some((BlockId(1), BlockId(2))));
    assert_eq!(worklist.dequeue(), Some((BlockId(2), BlockId(3))));
    assert_eq!(worklist.dequeue(), None);
}
```

**evaluator.rs**:
```rust
#[test]
fn test_binary_op_evaluation() {
    // Test all combinations of Top/Constant/Bottom for each operand
    // Test all BinaryOp variants (Add, Sub, Mul, Div, Mod, And, Or, Eq, Ne, Lt, Le, Gt, Ge)
    // Test all IrType variants
}

#[test]
fn test_phi_evaluation() {
    // Test phi with no executable predecessors → Top
    // Test phi with one executable predecessor → value from that predecessor
    // Test phi with multiple executable predecessors, all same constant → that constant
    // Test phi with multiple executable predecessors, different constants → Bottom
}
```

### 7.2 Integration Tests (Complete Functions)

```rust
#[test]
fn test_constant_propagation() {
    let ir = r#"
        function test():
            %1 = const 5
            %2 = const 10
            %3 = add %1, %2
            return %3
    "#;
    
    let optimized = optimize_with_sccp(ir);
    
    // Verify %3 is replaced with constant 15
    assert!(optimized.contains("const 15"));
    assert!(!optimized.contains("add"));
}

#[test]
fn test_branch_elimination() {
    let ir = r#"
        function test():
            %1 = const true
            br %1, then_block, else_block
        
        then_block:
            %2 = const 42
            return %2
        
        else_block:
            %3 = const 99
            return %3
    "#;
    
    let optimized = optimize_with_sccp(ir);
    
    // Verify conditional branch converted to unconditional
    // Verify else_block removed as unreachable
    assert!(!optimized.contains("else_block"));
    assert!(optimized.contains("br then_block"));
}

#[test]
fn test_phi_simplification() {
    let ir = r#"
        function test():
            %1 = const 5
            br merge_block
        
        merge_block:
            %2 = phi [%1, entry_block]
            return %2
    "#;
    
    let optimized = optimize_with_sccp(ir);
    
    // Verify phi node simplified to direct value
    assert!(!optimized.contains("phi"));
}
```

### 7.3 Regression Tests

```rust
#[test]
fn test_no_constants_unchanged() {
    let ir = r#"
        function test(%arg):
            %1 = add %arg, 1
            return %1
    "#;
    
    let optimized = optimize_with_sccp(ir);
    
    // Verify IR unchanged when no constant propagation possible
    assert_eq!(ir, optimized);
}

#[test]
fn test_side_effects_preserved() {
    let ir = r#"
        function test():
            %1 = const 5
            call print(%1)
            return %1
    "#;
    
    let optimized = optimize_with_sccp(ir);
    
    // Verify call to print() is preserved (side effect)
    assert!(optimized.contains("call print"));
}
```

### 7.4 Snapshot Tests (using insta)

```rust
#[test]
fn test_complex_optimization_snapshot() {
    let ir = load_test_ir("complex_function.ir");
    let optimized = optimize_with_sccp(ir);
    
    insta::assert_snapshot!(optimized);
}
```

### 7.5 Performance Tests (using criterion)

```rust
fn benchmark_sccp_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("sccp_scaling");
    
    for size in [1000, 5000, 10000].iter() {
        let ir = generate_synthetic_ir(*size);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &ir,
            |b, ir| b.iter(|| optimize_with_sccp(ir))
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_sccp_scaling);
criterion_main!(benches);
```

## 8. Edge Cases and Error Conditions

### 8.1 Entry Block Edge Cases

**Entry block with no successors**:
- Valid (function immediately returns)
- Entry block marked executable, no outgoing edges to process
- Analysis completes immediately

**Entry block with self-loop**:
- Entry block marked executable
- Loop edge (entry → entry) enqueued
- Phi nodes in entry block are invalid (entry has no predecessors by definition)
- Should be caught by pre-validation

**Phi nodes in entry block**:
- Invalid SSA form (entry block has no predecessors)
- Pre-validation should reject this

### 8.2 Infinite Loops

**Simple infinite loop**:
```text
entry:
    br loop_head

loop_head:
    br loop_head  # Infinite loop
```

- Entry block marked executable
- Edge (entry → loop_head) enqueued
- loop_head marked executable
- Edge (loop_head → loop_head) enqueued
- Self-loop edge processes, no new information
- Analysis converges (no lattice values change)

**Loop with phi node**:
```text
entry:
    %1 = const 0
    br loop_head

loop_head:
    %2 = phi [%1, entry], [%3, loop_head]
    %3 = add %2, 1
    br loop_head
```

- %2 initially Top
- First iteration: %2 = meet(Top, Constant(0)) = Constant(0)
- %3 = Constant(0) + Constant(1) = Constant(1)
- Second iteration: %2 = meet(Constant(0), Constant(1)) = Bottom (different constants)
- %3 = Bottom (Bottom + Constant = Bottom)
- Third iteration: %2 = meet(Bottom, Bottom) = Bottom (no change)
- Converges with %2 and %3 as Bottom

**Maximum Iteration Safety**:
- If analysis exceeds max_iterations, log warning and mark all remaining Top values as Bottom
- Ensures termination even in pathological cases

### 8.3 Division by Zero and Overflow

**Division by zero**:
```rust
%1 = const 5
%2 = const 0
%3 = div %1, %2  # Division by zero
```
- Evaluator detects `b == 0` and returns Bottom
- Conservative: does not optimize this to a trap or undefined behavior

**Integer overflow**:
```rust
%1 = const 2147483647  # i32::MAX
%2 = const 1
%3 = add %1, %2  # Overflow
```
- `checked_add` returns None
- Evaluator returns Bottom
- Does not assume wrapping or trapping behavior

**Floating-point special values**:
```rust
%1 = const 1.0
%2 = const 0.0
%3 = div %1, %2  # Produces Infinity
```
- Conservative: mark result as Bottom (to avoid incorrect optimizations)
- Aggressive alternative: propagate Infinity as Constant if semantics are well-defined

### 8.4 Type Mismatches

**Mismatched operand types**:
```rust
%1 = const 5      # I32
%2 = const true   # Bool
%3 = add %1, %2   # Type mismatch
```
- Pre-validation should catch this (type checker runs before optimizer)
- If not caught, evaluator returns Bottom conservatively

**Phi with different types**:
```rust
%1 = const 5      # I32
%2 = const 3.14   # F64
%3 = phi [%1, bb1], [%2, bb2]  # Type mismatch
```
- Invalid IR (should be caught by type checker)
- If encountered, evaluator returns Bottom

### 8.5 Unreachable Code Validation

**Top values in unreachable code**:
- After analysis, some values in unreachable blocks may remain Top
- Post-validation should only check that Top values are in unreachable blocks
- OR: rewrite phase marks all Top values as Bottom before validation

**Unreachable entry block**:
- Impossible by definition (entry is always reachable)
- Should not occur

## 9. Rust-Specific Implementation Details

### 9.1 Data Structures

**LatticeValue Enum**:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatticeValue {
    Top,
    Constant(IrLiteralValue),
    Bottom,
}

impl PartialOrd for LatticeValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LatticeValue::Top, LatticeValue::Top) => Some(Ordering::Equal),
            (LatticeValue::Top, _) => Some(Ordering::Less),
            (LatticeValue::Bottom, LatticeValue::Bottom) => Some(Ordering::Equal),
            (LatticeValue::Bottom, _) => Some(Ordering::Greater),
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) if c1 == c2 => Some(Ordering::Equal),
            (LatticeValue::Constant(_), LatticeValue::Top) => Some(Ordering::Greater),
            (LatticeValue::Constant(_), LatticeValue::Bottom) => Some(Ordering::Less),
            _ => None,  // Different constants are incomparable
        }
    }
}

impl LatticeValue {
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (LatticeValue::Top, x) | (x, LatticeValue::Top) => x.clone(),
            (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) if c1 == c2 => LatticeValue::Constant(c1.clone()),
            (LatticeValue::Constant(_), LatticeValue::Constant(_)) => LatticeValue::Bottom,
        }
    }
}
```

**Worklist Types**:
```rust
pub struct SSAWorkList {
    queue: VecDeque<(Value, InstructionId)>,
    seen: HashSet<(Value, InstructionId)>,
}

impl SSAWorkList {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            seen: HashSet::new(),
        }
    }
    
    pub fn enqueue(&mut self, edge: (Value, InstructionId)) -> bool {
        if self.seen.insert(edge) {
            self.queue.push_back(edge);
            true
        } else {
            false
        }
    }
    
    pub fn dequeue(&mut self) -> Option<(Value, InstructionId)> {
        self.queue.pop_front()
    }
    
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

pub struct FlowWorkList {
    queue: VecDeque<(BlockId, BlockId)>,
    seen: HashSet<(BlockId, BlockId)>,
}

impl FlowWorkList {
    // Similar implementation to SSAWorkList
}
```

**Executable Edge Tracking**:
```rust
pub struct ExecutableEdges {
    edges: HashSet<(BlockId, BlockId)>,
    blocks: HashSet<BlockId>,
}

impl ExecutableEdges {
    pub fn new() -> Self {
        Self {
            edges: HashSet::new(),
            blocks: HashSet::new(),
        }
    }
    
    pub fn mark_edge_executable(&mut self, pred: BlockId, succ: BlockId) -> bool {
        let newly_inserted = self.edges.insert((pred, succ));
        if newly_inserted {
            self.blocks.insert(succ);
        }
        newly_inserted
    }
    
    pub fn is_block_executable(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }
    
    pub fn is_edge_executable(&self, pred: BlockId, succ: BlockId) -> bool {
        self.edges.contains(&(pred, succ))
    }
}
```

### 9.2 Ownership and Borrowing

**Lattice State Management**:
```rust
pub struct SCCPAnalyzer<'a> {
    function: &'a Function,
    lattice: HashMap<Value, LatticeValue>,
    ssa_worklist: SSAWorkList,
    flow_worklist: FlowWorkList,
    executable: ExecutableEdges,
    max_iterations: usize,
    stats: OptimizationStatistics,
}

impl<'a> SCCPAnalyzer<'a> {
    pub fn new(function: &'a Function, max_iterations: usize) -> Self {
        // Initialize with all values at Top, except parameters and globals
        let mut lattice = HashMap::new();
        for value in function.all_values() {
            let initial_state = match value {
                Value::Parameter(_) | Value::Global(_) => LatticeValue::Bottom,
                _ => LatticeValue::Top,
            };
            lattice.insert(value, initial_state);
        }
        
        Self {
            function,
            lattice,
            ssa_worklist: SSAWorkList::new(),
            flow_worklist: FlowWorkList::new(),
            executable: ExecutableEdges::new(),
            max_iterations,
            stats: OptimizationStatistics::default(),
        }
    }
    
    pub fn analyze(&mut self) -> Result<OptimizationStatistics, SCCPError> {
        // Initialize entry block
        let entry_block = self.function.entry_block();
        for successor in self.function.successors(entry_block) {
            self.executable.mark_edge_executable(entry_block, successor);
            self.flow_worklist.enqueue((entry_block, successor));
        }
        
        let mut iterations = 0;
        while !self.ssa_worklist.is_empty() || !self.flow_worklist.is_empty() {
            iterations += 1;
            if iterations > self.max_iterations {
                return Err(SCCPError::MaxIterationsExceeded(self.max_iterations));
            }
            
            // Process flow worklist
            while let Some((pred, succ)) = self.flow_worklist.dequeue() {
                self.visit_block(succ)?;
            }
            
            // Process SSA worklist
            while let Some((def_value, use_instruction)) = self.ssa_worklist.dequeue() {
                if self.executable.is_block_executable(use_instruction.block) {
                    self.visit_instruction(use_instruction)?;
                }
            }
        }
        
        self.stats.iterations_to_convergence = iterations;
        Ok(self.stats.clone())
    }
}
```

### 9.3 Error Handling with thiserror

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SCCPError {
    #[error("Pre-optimization validation failed: {0}")]
    PreValidationFailed(String),
    
    #[error("Post-optimization validation failed: {0}")]
    PostValidationFailed(String),
    
    #[error("Maximum iterations ({0}) exceeded without convergence")]
    MaxIterationsExceeded(usize),
    
    #[error("SSA form violation: {0}")]
    SSAViolation(String),
    
    #[error("CFG integrity violation: {0}")]
    CFGViolation(String),
    
    #[error("Lattice invariant violation: value {0} moved upward from {1:?} to {2:?}")]
    LatticeInvariantViolation(String, LatticeValue, LatticeValue),
}
```

## 10. Future Enhancements

### 10.1 Interprocedural SCCP

Extend SCCP across function boundaries:
- Analyze callers to determine constant arguments
- Propagate constant return values
- Requires call graph analysis and function summaries

### 10.2 Advanced Alias Analysis Integration

Improve precision for memory operations:
- Use alias analysis to determine when loads access compile-time constants
- Propagate constants through non-aliasing stores
- Requires integration with points-to analysis

### 10.3 Symbolic Execution Integration

Handle complex conditions:
- Use constraint solving for branch conditions involving multiple variables
- Detect infeasible paths even when individual values are not constant
- Requires SMT solver integration (e.g., Z3)

### 10.4 Profile-Guided Optimization

Use runtime profiling information:
- Prioritize optimization of hot paths
- Speculate on likely constant values based on profiling data
- Requires profiling infrastructure and speculative optimization support

## 11. Decision Log

### Decision 1: Three-Level Lattice vs. More Complex Lattices

**Decision**: Use flat three-level lattice (Top/Constant/Bottom)

**Rationale**:
- Simpler to implement and reason about
- Sufficient for constant propagation (no need for interval or symbolic domains)
- Wegman-Zadeck paper uses three-level lattice successfully

**Alternatives Considered**:
- Interval lattice (e.g., [0, 10]) for range analysis
- Symbolic lattice (e.g., x+5) for symbolic execution
- Both add significant complexity without clear benefit for initial implementation

### Decision 2: Conservative Floating-Point Handling

**Decision**: Mark NaN and Infinity results as Bottom

**Rationale**:
- IEEE 754 semantics are complex (NaN != NaN, Infinity arithmetic has special rules)
- Conservative approach avoids incorrect optimizations
- Can be relaxed in future if profiling shows benefit

**Alternatives Considered**:
- Propagate NaN and Infinity as Constant values
- Requires careful handling of all floating-point operations and comparisons
- Higher risk of incorrect optimizations

### Decision 3: Separate DCE Pass vs. Integrated Cleanup

**Decision**: Initial implementation marks dead code, separate DCE removes it

**Rationale**:
- Clearer separation of concerns
- Easier to test SCCP and DCE independently
- Allows DCE to handle other dead code patterns (unused values, etc.)

**Alternatives Considered**:
- SCCP performs complete dead code removal
- Simpler for end users (single-pass optimization)
- More complex implementation, harder to test
- Can be implemented as future optimization if profiling shows overhead

### Decision 4: Maximum Iteration Limit

**Decision**: Default max_iterations = 100, configurable

**Rationale**:
- Pathological cases (complex loops, large functions) may not converge quickly
- Safety mechanism to prevent infinite loops in optimizer
- 100 iterations sufficient for 99%+ of real-world code (based on literature)

**Alternatives Considered**:
- No iteration limit (rely on termination proof)
- Risk of infinite loops in buggy implementation
- Iteration limit provides defense in depth

### Decision 5: String Type Handling

**Decision**: Always mark String values as Bottom

**Rationale**:
- String operations are complex (concatenation, slicing, encoding)
- Constant string folding requires significant infrastructure
- Low priority for initial implementation (can be added later)

**Alternatives Considered**:
- Propagate constant strings
- Requires string interning, complex operation evaluation
- High implementation cost for modest benefit

## 12. References and Further Reading

### Foundational Papers

1. **Wegman, M. N., & Zadeck, F. K. (1991)**. "Constant propagation with conditional branches." *ACM Transactions on Programming Languages and Systems (TOPLAS)*, 13(2), 181-210.
   - Original SCCP algorithm paper
   - Describes three-level lattice and dual worklist approach
   - Proves O(edges) time complexity

2. **Cytron, R., Ferrante, J., Rosen, B. K., Wegman, M. N., & Zadeck, F. K. (1991)**. "Efficiently computing static single assignment form and the control dependence graph." *ACM Transactions on Programming Languages and Systems (TOPLAS)*, 13(4), 451-490.
   - Foundational SSA paper
   - Describes SSA construction and properties
   - Essential background for understanding SCCP

### Compiler Textbooks

3. **Appel, A. W., & Palsberg, J. (2002)**. *Modern Compiler Implementation in Java* (2nd ed.). Cambridge University Press.
   - Chapter 19: Dataflow Analysis
   - Chapter 19.2: Constant Propagation
   - Practical implementation guidance

4. **Cooper, K. D., & Torczon, L. (2011)**. *Engineering a Compiler* (2nd ed.). Morgan Kaufmann.
   - Chapter 9: Data-Flow Analysis
   - Section 9.3: Constant Propagation
   - SSA-based optimization techniques

### Online Resources

5. **LLVM Documentation**: "SCCP - Sparse Conditional Constant Propagation"
   - https://llvm.org/docs/Passes.html#sccp-sparse-conditional-constant-propagation
   - Real-world implementation reference
   - Integration with other optimization passes

6. **GCC Wiki**: "Tree SSA Passes"
   - https://gcc.gnu.org/wiki/Tree_SSA_Passes
   - GCC's constant propagation implementation
   - Practical considerations for production compilers

### Rust-Specific Resources

7. **Rust Compiler (rustc)**: MIR Constant Propagation
   - https://github.com/rust-lang/rust/tree/master/compiler/rustc_mir_transform/src/const_prop
   - Rust's own constant propagation on MIR (Mid-level IR)
   - Practical Rust implementation patterns

8. **Cranelift Code Generator**: Constant Folding
   - https://github.com/bytecodealliance/wasmtime/tree/main/cranelift/codegen/src/constant_folding
   - Rust-based JIT compiler's constant folding
   - Performance-oriented implementation

---

**End of Research Document**

This document provides the comprehensive foundation for implementing SCCP in the jsavrs compiler. All technical decisions, algorithms, and implementation strategies are research-based and aligned with industry best practices.
