# Research Document: Sparse Conditional Constant Propagation (SCCP) Optimizer

**Feature**: SCCP Optimizer  
**Branch**: `017-sccp-optimizer`  
**Date**: 2025-11-19  
**Status**: Research Complete

## Executive Summary

This document provides comprehensive research findings for implementing a Sparse Conditional Constant Propagation (SCCP) optimization phase in the jsavrs compiler. SCCP is a powerful optimization technique that simultaneously performs constant propagation and dead code elimination by analyzing control flow and data flow together. The implementation will follow the Wegman-Zadeck algorithm using lattice-based dataflow analysis on SSA form IR.

## Research Objectives

The research phase addresses the following questions identified during Technical Context analysis:

1. **Lattice Theory Foundation**: How to implement a three-value lattice (Unknown ⊤, Constant, Variable ⊥) that guarantees monotonic convergence?
2. **Constant Folding Semantics**: What are the correct semantics for constant folding all supported operations (arithmetic, bitwise, comparison) across all primitive types?
3. **Worklist Algorithm**: How to implement efficient sparse dataflow analysis using worklists to avoid full-function scans?
4. **SSA Integration**: How to correctly handle phi nodes in the analysis and transformation phases?
5. **IR Transformation**: What is the optimal strategy for in-place IR mutation after analysis completes?
6. **Integration with DCE**: How to coordinate SCCP with existing Dead Code Elimination for maximum effectiveness?

## 1. Lattice Theory Foundation

### Decision: Three-Value Lattice with Meet Operation

**Rationale**: The lattice provides a mathematical foundation that guarantees analysis termination and correctness. The three values represent increasing precision of knowledge about SSA values.

**Lattice Structure**:
```
    ⊤ (Top/Unknown)
   / \
  /   \
Constant Bottom
  \   /
   \ /
    ⊥ (Bottom/Variable)
```

**Ordering**: Top > Constant > Bottom (where > means "less precise than")

**Meet Operation (⊓)**: Combines two lattice values to produce their greatest lower bound:
- Top ⊓ Top = Top
- Top ⊓ Constant(c) = Constant(c)
- Top ⊓ Bottom = Bottom
- Constant(c) ⊓ Constant(c) = Constant(c)
- Constant(c₁) ⊓ Constant(c₂) = Bottom (if c₁ ≠ c₂)
- Constant(c) ⊓ Bottom = Bottom
- Bottom ⊓ Bottom = Bottom

**Properties Guaranteed**:
- **Commutativity**: a ⊓ b = b ⊓ a
- **Associativity**: (a ⊓ b) ⊓ c = a ⊓ (b ⊓ c)
- **Idempotence**: a ⊓ a = a
- **Monotonicity**: Values only move down the lattice (never upward)

**Implementation Strategy**:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    Top,
    Constant(IrLiteralValue),
    Bottom,
}

impl LatticeValue {
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (LatticeValue::Top, v) | (v, LatticeValue::Top) => v.clone(),
            (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) => {
                if c1 == c2 {
                    LatticeValue::Constant(c1.clone())
                } else {
                    LatticeValue::Bottom
                }
            }
        }
    }
}
```

**Alternatives Considered**:
- **Two-value lattice (Constant/Variable)**: Rejected because it cannot represent "not yet analyzed" state, requiring pessimistic initialization that misses optimization opportunities.
- **Four-value lattice (Top/Constant/NonConstant/Bottom)**: Rejected as unnecessarily complex for our use case; three values provide sufficient precision.

## 2. Constant Folding Semantics

### Decision: Wrapping Arithmetic with Pattern Matching

**Rationale**: Rust's wrapping semantics match release-mode behavior, ensuring SCCP-optimized code produces identical results to unoptimized code. Pattern matching provides type-safe, exhaustive constant evaluation.

**Arithmetic Operations**:

For integer types (i8-i64, u8-u64), use Rust's wrapping methods:
```rust
pub fn fold_binary(op: IrBinaryOp, left: IrLiteralValue, right: IrLiteralValue) 
    -> Option<IrLiteralValue> 
{
    use IrLiteralValue::*;
    match (op, left, right) {
        // Signed integer addition
        (IrBinaryOp::Add, I8(a), I8(b)) => Some(I8(a.wrapping_add(b))),
        (IrBinaryOp::Add, I16(a), I16(b)) => Some(I16(a.wrapping_add(b))),
        (IrBinaryOp::Add, I32(a), I32(b)) => Some(I32(a.wrapping_add(b))),
        (IrBinaryOp::Add, I64(a), I64(b)) => Some(I64(a.wrapping_add(b))),
        
        // Unsigned integer addition
        (IrBinaryOp::Add, U8(a), U8(b)) => Some(U8(a.wrapping_add(b))),
        (IrBinaryOp::Add, U16(a), U16(b)) => Some(U16(a.wrapping_add(b))),
        (IrBinaryOp::Add, U32(a), U32(b)) => Some(U32(a.wrapping_add(b))),
        (IrBinaryOp::Add, U64(a), U64(b)) => Some(U64(a.wrapping_add(b))),
        
        // Floating-point addition (no wrapping needed)
        (IrBinaryOp::Add, F32(a), F32(b)) => Some(F32(a + b)),
        (IrBinaryOp::Add, F64(a), F64(b)) => Some(F64(a + b)),
        
        // Division by zero → None (conservative)
        (IrBinaryOp::Divide, I32(_), I32(0)) => None,
        (IrBinaryOp::Divide, I32(a), I32(b)) => Some(I32(a.wrapping_div(b))),
        
        // Type mismatch → None
        _ => None,
    }
}
```

**Overflow/Underflow Handling**:
- All integer operations use `wrapping_*` methods (wrapping_add, wrapping_sub, wrapping_mul, wrapping_div)
- This matches Rust's release-mode behavior where overflow checks are disabled
- Ensures SCCP-optimized code produces identical runtime results

**Division by Zero**:
- Return `None` → lattice value becomes Bottom (conservative)
- Prevents incorrect constant propagation that would change runtime behavior
- Consistent with Rust's release-mode division-by-zero behavior (undefined, but we're conservative)

**Floating-Point Special Cases**:
- NaN propagation: Any operation with NaN produces NaN
- Infinity: Handled by Rust's f32/f64 semantics
- Precision: Accept potential floating-point precision differences (deterministic within a platform)

**Bitwise Operations**:
```rust
(IrBinaryOp::BitwiseAnd, I32(a), I32(b)) => Some(I32(a & b)),
(IrBinaryOp::BitwiseOr, U64(a), U64(b)) => Some(U64(a | b)),
(IrBinaryOp::BitwiseXor, I16(a), I16(b)) => Some(I16(a ^ b)),
(IrBinaryOp::ShiftLeft, U32(a), U32(b)) => Some(U32(a.wrapping_shl(b))),
(IrBinaryOp::ShiftRight, I64(a), I64(b)) => Some(I64(a.wrapping_shr(b))),
```

**Comparison Operations**:
```rust
(IrBinaryOp::Equal, I32(a), I32(b)) => Some(Bool(a == b)),
(IrBinaryOp::Less, F64(a), F64(b)) => Some(Bool(a < b)),
(IrBinaryOp::GreaterEqual, U8(a), U8(b)) => Some(Bool(a >= b)),
```

**Logical Operations**:
```rust
(IrBinaryOp::And, Bool(a), Bool(b)) => Some(Bool(a && b)),
(IrBinaryOp::Or, Bool(a), Bool(b)) => Some(Bool(a || b)),
```

**Unary Operations**:
```rust
pub fn fold_unary(op: IrUnaryOp, operand: IrLiteralValue) 
    -> Option<IrLiteralValue> 
{
    match (op, operand) {
        (IrUnaryOp::Negate, I32(v)) => Some(I32(v.wrapping_neg())),
        (IrUnaryOp::Negate, F64(v)) => Some(F64(-v)),
        (IrUnaryOp::Not, Bool(v)) => Some(Bool(!v)),
        _ => None,
    }
}
```

**Alternatives Considered**:
- **Checked arithmetic with Result**: Rejected because it would diverge from release-mode semantics.
- **Separate debug/release folding logic**: Rejected as unnecessarily complex; always use wrapping.
- **Interpreter-based evaluation**: Rejected as too heavyweight and difficult to prove correct.

## 3. Worklist Algorithm

### Decision: Two-Phase Worklist with Sparse Propagation

**Rationale**: Sparse dataflow analysis processes only affected parts of the code when values change, achieving O(n) complexity instead of O(n²) for iterative full-function scans.

**Data Structures**:
```rust
pub struct Worklist {
    ssa_worklist: VecDeque<Value>,           // SSA values needing re-evaluation
    cfg_worklist: VecDeque<(NodeIndex, NodeIndex)>, // Executable CFG edges
}

impl Worklist {
    pub fn add_ssa(&mut self, value: Value) {
        if !self.ssa_seen.contains(&value) {
            self.ssa_worklist.push_back(value);
            self.ssa_seen.insert(value);
        }
    }
    
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        let edge = (from, to);
        if !self.cfg_seen.contains(&edge) {
            self.cfg_worklist.push_back(edge);
            self.cfg_seen.insert(edge);
        }
    }
}
```

**Algorithm Flow**:

1. **Initialization**:
   - Mark entry block's outgoing edges as executable (add to cfg_worklist)
   - All SSA values start at Top (unknown)
   - All CFG edges start as non-executable

2. **Iteration Loop** (until worklists empty or 10,000 iterations):
   ```rust
   while let Some(edge) = worklist.pop_cfg_edge() {
       let (from_block, to_block) = edge;
       
       // Mark destination block as executable
       if !executable_blocks.contains(&to_block) {
           executable_blocks.insert(to_block);
           
           // Process all instructions in newly reachable block
           for inst in to_block.instructions() {
               evaluate_instruction(inst);
           }
           
           // Add block's outgoing edges to worklist
           for succ in to_block.successors() {
               worklist.add_edge(to_block, succ);
           }
       }
       
       // Update phi nodes in destination with values from this edge
       for phi in to_block.phis() {
           update_phi(phi, from_block);
       }
   }
   
   while let Some(value) = worklist.pop_ssa_value() {
       // Re-evaluate all uses of this value
       for use_inst in def_use_chain.get_uses(value) {
           evaluate_instruction(use_inst);
       }
   }
   ```

3. **Instruction Evaluation**:
   ```rust
   fn evaluate_instruction(inst: &Instruction) {
       let old_lattice = lattice_values.get(inst.result);
       let new_lattice = match &inst.kind {
           Binary { op, left, right, .. } => {
               evaluate_binary(*op, 
                   lattice_values.get(*left), 
                   lattice_values.get(*right))
           }
           Call { .. } => LatticeValue::Bottom, // Conservative
           Phi { incoming, .. } => evaluate_phi(incoming),
           // ... other instruction kinds
       };
       
       if new_lattice != old_lattice {
           lattice_values.set(inst.result, new_lattice);
           worklist.add_ssa(inst.result); // Propagate change
       }
   }
   ```

**Complexity Analysis**:
- Each SSA value can change lattice value at most twice (Top → Constant → Bottom)
- Each CFG edge is processed at most once when marked executable
- Total iterations: O(V + E) where V = SSA values, E = CFG edges
- Each iteration processes constant work
- **Overall**: O(V + E) time complexity

**Memory Usage**:
- HashMap<Value, LatticeValue>: O(V) space
- HashSet<NodeIndex>: O(B) space (B = basic blocks)
- Worklists with deduplication: O(V + E) space
- **Overall**: O(V + E) space complexity

**Alternatives Considered**:
- **Full-function iterative dataflow**: Rejected due to O(n²) complexity and redundant work.
- **Priority queue worklist**: Rejected as unnecessary; FIFO is sufficient for correctness.
- **Concurrent worklist processing**: Rejected due to complexity and lack of parallelism benefit for single functions.

## 4. SSA Integration and Phi Node Handling

### Decision: Executable-Edge-Only Phi Analysis

**Rationale**: Phi nodes select values based on which predecessor was executed. We must only consider values from executable predecessors to avoid unsound constant propagation.

**Phi Node Structure**:
```rust
Phi { 
    ty: IrType, 
    incoming: Vec<(Value, String)>  // (value, predecessor_block_label)
}
```

**Analysis Algorithm**:
```rust
fn evaluate_phi(phi: &Instruction, executable_blocks: &HashSet<NodeIndex>) 
    -> LatticeValue 
{
    let mut result = LatticeValue::Top;
    
    for (value, pred_label) in &phi.incoming {
        let pred_block = find_block_by_label(pred_label);
        
        // Only consider values from executable predecessors
        if executable_blocks.contains(&pred_block) {
            let value_lattice = lattice_values.get(*value);
            result = result.meet(&value_lattice);
        }
    }
    
    result
}
```

**Key Cases**:

1. **All executable predecessors provide same constant**:
   - Result: Constant(c)
   - Example: `phi [42, block1], [42, block2]` with both blocks executable → Constant(42)

2. **Executable predecessors provide different constants**:
   - Result: Bottom (variable)
   - Example: `phi [42, block1], [17, block2]` with both blocks executable → Bottom

3. **Some predecessors have Top (not yet analyzed)**:
   - Result: Top ⊓ Constant(c) = Constant(c)
   - Phi will be re-evaluated when Top values are refined

4. **No executable predecessors**:
   - Result: Top (stays Top forever, block is unreachable)
   - This is correct because the phi value is never used

**Transformation Phase**:

During IR transformation, physically remove dead predecessor entries:
```rust
fn transform_phi(phi: &mut Instruction, executable_blocks: &HashSet<NodeIndex>) {
    if let InstructionKind::Phi { incoming, .. } = &mut phi.kind {
        incoming.retain(|(_, pred_label)| {
            let pred_block = find_block_by_label(pred_label);
            executable_blocks.contains(&pred_block)
        });
        
        // If only one predecessor remains, the phi can be replaced by that value
        // (this is done by marking the phi as dead and letting DCE handle it)
    }
}
```

**Alternatives Considered**:
- **Consider all predecessors**: Rejected because it would propagate constants through dead paths, causing unsoundness.
- **Lazy phi evaluation**: Rejected because it complicates fixed-point detection.
- **Immediate phi replacement during analysis**: Rejected to keep analysis and transformation phases separate.

## 5. IR Transformation Strategy

### Decision: In-Place Mutation with Dead Instruction Marking

**Rationale**: Directly mutate the IR during transformation for efficiency, while marking dead instructions for DCE to remove. This avoids creating intermediate IR copies and leverages existing DCE infrastructure.

**Transformation Operations**:

1. **Constant Replacement**:
   ```rust
   for (ssa_value, lattice_value) in &lattice_values {
       if let LatticeValue::Constant(literal) = lattice_value {
           // Replace all uses of ssa_value with literal constant
           for use_inst in def_use_chain.get_uses(ssa_value) {
               replace_operand(use_inst, ssa_value, literal);
           }
           
           // Mark the defining instruction as dead
           if let Some(def_inst) = def_use_chain.get_def(ssa_value) {
               mark_instruction_dead(def_inst);
           }
       }
   }
   ```

2. **Branch Simplification**:
   ```rust
   for block in function.blocks() {
       if let Some(terminator) = block.terminator() {
           match terminator {
               ConditionalBranch { condition, true_dest, false_dest } => {
                   if let LatticeValue::Constant(Bool(true)) = lattice_values.get(condition) {
                       // Replace with unconditional branch to true_dest
                       block.set_terminator(Branch { dest: true_dest });
                   } else if let LatticeValue::Constant(Bool(false)) = lattice_values.get(condition) {
                       // Replace with unconditional branch to false_dest
                       block.set_terminator(Branch { dest: false_dest });
                   }
               }
               _ => {}
           }
       }
   }
   ```

3. **Phi Node Cleanup**:
   ```rust
   for block in function.blocks() {
       for phi in block.phis_mut() {
           // Remove entries from non-executable predecessors
           phi.incoming.retain(|(_, pred_label)| {
               let pred_block = find_block_by_label(pred_label);
               executable_blocks.contains(&pred_block)
           });
           
           // If phi now has constant value, replace all uses
           if let LatticeValue::Constant(literal) = lattice_values.get(phi.result) {
               replace_all_uses(phi.result, literal);
               mark_instruction_dead(phi);
           }
       }
   }
   ```

4. **Unreachable Block Marking**:
   ```rust
   for block in function.blocks() {
       if !executable_blocks.contains(&block) {
           // Don't physically remove - let DCE handle it
           // Just ensure it's not in the executable set
           // DCE's reachability analysis will remove it
       }
   }
   ```

**Dead Instruction Marking**:

We leverage the existing IR infrastructure:
```rust
// Add a flag to Instruction struct (or use existing metadata)
impl Instruction {
    pub fn mark_dead(&mut self) {
        self.is_dead = true;
    }
}
```

DCE will remove all instructions marked dead during its sweep phase.

**Alternatives Considered**:
- **Immediate instruction removal**: Rejected because it complicates iteration and requires careful index management.
- **Create new optimized IR**: Rejected due to memory overhead and complexity of copying entire functions.
- **Lazy transformation on demand**: Rejected because it complicates downstream phases that need consistent IR state.

## 6. Integration with Dead Code Elimination

### Decision: Alternating Pipeline with Fixed-Point Detection

**Rationale**: SCCP discovers unreachable blocks and dead values, but DCE physically removes them. Running these phases alternately allows each to enable further optimizations by the other.

**Pipeline Structure**:
```rust
pub fn optimize_function(function: &mut Function) -> bool {
    let mut changed = true;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 3;
    
    while changed && iterations < MAX_ITERATIONS {
        changed = false;
        iterations += 1;
        
        // Phase 1: SCCP discovers constants and unreachable code
        let sccp = SccpOptimizer::new();
        if sccp.run(function) {
            changed = true;
        }
        
        // Phase 2: DCE removes dead instructions and unreachable blocks
        let dce = DeadCodeElimination::new();
        if dce.run(function) {
            changed = true;
        }
    }
    
    changed
}
```

**Why Alternation is Necessary**:

1. **SCCP enables DCE**: SCCP marks branches as unconditional and blocks as unreachable, creating dead code for DCE to remove.

2. **DCE enables SCCP**: DCE removes dead instructions, potentially exposing new constant propagation opportunities (e.g., removing a store that made a value appear non-constant).

3. **Example**:
   ```
   Initial IR:
   x = 42
   if (x > 40) {  // SCCP: condition is constant true
       y = x + 1  // SCCP: y is constant 43
   } else {
       y = 0      // Unreachable after SCCP
   }
   z = y + 1      // SCCP: z is constant 44 after iteration 2
   
   After SCCP Iteration 1:
   - x = 42 marked dead (replaced with literal)
   - condition simplified to unconditional branch
   - else block marked unreachable
   - y is still a phi (not yet removed)
   
   After DCE Iteration 1:
   - else block physically removed
   - x = 42 instruction removed
   - phi for y now has single predecessor
   
   After SCCP Iteration 2:
   - y = 43 propagated (phi with single pred eliminated by DCE)
   - z = 44 discovered
   ```

**Fixed-Point Detection**:

Both phases return `bool` indicating whether they made changes:
```rust
impl Phase for SccpOptimizer {
    fn run(&mut self, module: &mut Module) -> bool {
        let mut changed = false;
        for function in module.functions_mut() {
            changed |= self.run_on_function(function);
        }
        changed
    }
}
```

Loop terminates when:
- No changes made by either phase (`changed = false`)
- Maximum iteration count reached (3 iterations is typical)

**Performance Considerations**:

- Most functions converge in 1-2 iterations
- 3 iterations is a conservative upper bound
- Each iteration is still O(n) so total is O(n) with small constant factor

**Alternatives Considered**:
- **Single combined SCCP+DCE phase**: Rejected due to implementation complexity and tight coupling.
- **SCCP only, no DCE integration**: Rejected because it leaves dead code in IR.
- **Unlimited iterations until fixed-point**: Rejected due to unpredictable performance; 3 iterations is sufficient in practice.

## 7. Implementation Architecture

### Module Structure

**File Organization**:
```
src/ir/optimizer/sccp/
├── mod.rs                # Public API and SccpOptimizer struct
├── lattice.rs            # LatticeValue enum and meet operation
├── constant_folder.rs    # Constant folding for all operations
├── analyzer.rs           # Worklist-driven dataflow analysis
├── transformer.rs        # IR transformation after analysis
└── worklist.rs           # Worklist data structure
```

**Module Responsibilities**:

1. **mod.rs**: Public interface and high-level orchestration
   ```rust
   pub struct SccpOptimizer {
       verbose: bool,
       max_iterations: usize,
   }
   
   impl Phase for SccpOptimizer {
       fn name(&self) -> &'static str { "SCCP" }
       fn run(&mut self, module: &mut Module) -> bool;
   }
   ```

2. **lattice.rs**: Lattice value abstraction
   ```rust
   pub enum LatticeValue {
       Top,
       Constant(IrLiteralValue),
       Bottom,
   }
   
   impl LatticeValue {
       pub fn meet(&self, other: &Self) -> Self;
       pub fn is_constant(&self) -> bool;
       pub fn as_constant(&self) -> Option<&IrLiteralValue>;
   }
   ```

3. **constant_folder.rs**: Pure constant evaluation
   ```rust
   pub fn fold_binary(op: IrBinaryOp, left: IrLiteralValue, right: IrLiteralValue) 
       -> Option<IrLiteralValue>;
   
   pub fn fold_unary(op: IrUnaryOp, operand: IrLiteralValue) 
       -> Option<IrLiteralValue>;
   ```

4. **analyzer.rs**: Core SCCP algorithm
   ```rust
   pub struct SccpAnalyzer {
       lattice_values: HashMap<Value, LatticeValue>,
       executable_blocks: HashSet<NodeIndex>,
       worklist: Worklist,
   }
   
   impl SccpAnalyzer {
       pub fn analyze(&mut self, function: &Function) -> AnalysisResult;
   }
   ```

5. **transformer.rs**: IR mutation
   ```rust
   pub struct SccpTransformer {
       stats: TransformStats,
   }
   
   impl SccpTransformer {
       pub fn transform(&mut self, function: &mut Function, result: &AnalysisResult);
   }
   ```

6. **worklist.rs**: Worklist management
   ```rust
   pub struct Worklist {
       ssa_values: VecDeque<Value>,
       cfg_edges: VecDeque<(NodeIndex, NodeIndex)>,
       ssa_seen: HashSet<Value>,
       cfg_seen: HashSet<(NodeIndex, NodeIndex)>,
   }
   ```

**Data Flow**:
```
SccpOptimizer::run(module)
  ├── for each function in module
  │   ├── SccpAnalyzer::new()
  │   ├── analyzer.analyze(function)
  │   │   ├── Initialize worklists
  │   │   ├── While worklists not empty
  │   │   │   ├── Process CFG edges
  │   │   │   ├── Process SSA values
  │   │   │   └── Use constant_folder for evaluation
  │   │   └── Return AnalysisResult
  │   ├── SccpTransformer::new()
  │   └── transformer.transform(function, analysis_result)
  │       ├── Replace constant uses
  │       ├── Simplify branches
  │       ├── Clean phi nodes
  │       └── Update stats
  └── Return changed flag
```

## 8. Testing Strategy

### Unit Tests

**Lattice Operations** (`lattice.rs`):
```rust
#[test]
fn test_meet_commutativity() {
    let a = LatticeValue::Constant(IrLiteralValue::I32(42));
    let b = LatticeValue::Top;
    assert_eq!(a.meet(&b), b.meet(&a));
}

#[test]
fn test_meet_idempotence() {
    let a = LatticeValue::Constant(IrLiteralValue::Bool(true));
    assert_eq!(a.meet(&a), a);
}

#[test]
fn test_different_constants_meet_to_bottom() {
    let a = LatticeValue::Constant(IrLiteralValue::I32(42));
    let b = LatticeValue::Constant(IrLiteralValue::I32(17));
    assert_eq!(a.meet(&b), LatticeValue::Bottom);
}
```

**Constant Folding** (`constant_folder.rs`):
```rust
#[test]
fn test_wrapping_add_overflow() {
    let result = fold_binary(
        IrBinaryOp::Add,
        IrLiteralValue::I8(127),
        IrLiteralValue::I8(1)
    );
    assert_eq!(result, Some(IrLiteralValue::I8(-128)));
}

#[test]
fn test_division_by_zero() {
    let result = fold_binary(
        IrBinaryOp::Divide,
        IrLiteralValue::I32(42),
        IrLiteralValue::I32(0)
    );
    assert_eq!(result, None);
}

#[test]
fn test_comparison_folding() {
    let result = fold_binary(
        IrBinaryOp::Less,
        IrLiteralValue::U64(10),
        IrLiteralValue::U64(20)
    );
    assert_eq!(result, Some(IrLiteralValue::Bool(true)));
}
```

### Integration Tests

**End-to-End SCCP** (`tests/sccp_integration_tests.rs`):
```rust
#[test]
fn test_constant_propagation() {
    let mut module = build_test_module(r#"
        function test() {
            x = 42
            y = x + 1
            return y
        }
    "#);
    
    let mut sccp = SccpOptimizer::new();
    sccp.run(&mut module);
    
    // Verify y is propagated to 43
    let function = &module.functions[0];
    assert!(returns_constant(function, IrLiteralValue::I32(43)));
}

#[test]
fn test_unreachable_elimination() {
    let mut module = build_test_module(r#"
        function test() {
            x = 42
            if (x > 40) {
                return 1
            } else {
                return 0  # Should be marked unreachable
            }
        }
    "#);
    
    let mut sccp = SccpOptimizer::new();
    sccp.run(&mut module);
    
    let function = &module.functions[0];
    assert_eq!(count_basic_blocks(function), 2); // entry + true branch only
}
```

### Snapshot Tests

**IR Comparison** (`tests/sccp_snapshot_tests.rs`):
```rust
#[test]
fn test_sccp_snapshot() {
    let mut module = build_test_module(include_str!("test_cases/constant_chain.js"));
    
    let before = format!("{:?}", module);
    
    let mut sccp = SccpOptimizer::new();
    sccp.run(&mut module);
    
    let after = format!("{:?}", module);
    
    insta::assert_snapshot!("constant_chain_before", before);
    insta::assert_snapshot!("constant_chain_after", after);
}
```

### Performance Benchmarks

**Criterion Benchmarks** (`benches/sccp_benchmark.rs`):
```rust
fn bench_sccp_large_function(c: &mut Criterion) {
    let module = generate_function_with_n_instructions(10_000);
    
    c.bench_function("sccp_10k_instructions", |b| {
        b.iter(|| {
            let mut m = module.clone();
            let mut sccp = SccpOptimizer::new();
            sccp.run(&mut m);
        });
    });
}
```

## 9. Configuration and Observability

### Configuration Options

```rust
pub struct SccpOptimizer {
    pub verbose: bool,           // Enable detailed logging
    pub max_iterations: usize,   // Safety limit (default 10,000)
    pub enable: bool,            // Master enable/disable switch
}

impl SccpOptimizer {
    pub fn new() -> Self {
        Self {
            verbose: false,
            max_iterations: 10_000,
            enable: true,
        }
    }
    
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}
```

### Logging Output

**Verbose Mode**:
```rust
if self.verbose {
    println!("SCCP: Discovered {} constants", stats.constants_found);
    println!("SCCP: Simplified {} branches", stats.branches_simplified);
    println!("SCCP: Marked {} blocks unreachable", stats.unreachable_blocks);
    println!("SCCP: Converged in {} iterations", stats.iterations);
}
```

**Debug Tracing** (via environment variable):
```rust
if std::env::var("SCCP_TRACE").is_ok() {
    eprintln!("SCCP: {} changed from {:?} to {:?}", 
        value, old_lattice, new_lattice);
}
```

### Statistics Collection

```rust
pub struct SccpStats {
    pub constants_found: usize,
    pub branches_simplified: usize,
    pub unreachable_blocks: usize,
    pub iterations: usize,
    pub time_elapsed: Duration,
}
```

## 10. Error Handling and Safety

### Conservative Fallbacks

**Iteration Limit Exceeded**:
```rust
if iterations > self.max_iterations {
    if self.verbose {
        eprintln!("SCCP: WARNING: Iteration limit exceeded, degrading all Top values to Bottom");
    }
    
    // Conservative: mark all remaining Top values as Bottom
    for (_, lattice_value) in lattice_values.iter_mut() {
        if *lattice_value == LatticeValue::Top {
            *lattice_value = LatticeValue::Bottom;
        }
    }
    
    // Analysis completes without crashing
    break;
}
```

**Invalid IR Detection**:
```rust
if entry_block_unreachable {
    eprintln!("SCCP: WARNING: Entry block appears unreachable (invalid IR)");
    // Force entry block to be executable
    executable_blocks.insert(entry_block);
}
```

**Type Mismatches in Constant Folding**:
```rust
match (left, right) {
    (I32(a), I32(b)) => Some(I32(a.wrapping_add(b))),
    (I32(_), _) | (_, I32(_)) => {
        // Type mismatch → None → Bottom
        None
    }
    // ... other cases
}
```

### Soundness Guarantees

**Never claim constant unless provable**:
- Function calls → Bottom (no interprocedural analysis)
- Memory loads → Bottom (no alias analysis)
- Division by zero → Bottom (conservative)
- Unknown operations → Bottom (conservative)

**Monotonicity enforced**:
- Lattice values only move down (Top → Constant → Bottom)
- Once Bottom, stays Bottom forever
- Ensures termination and correctness

## 11. Performance Optimization Techniques

### Memory Efficiency

**Pre-allocation**:
```rust
let estimated_values = function.instruction_count();
let lattice_values = HashMap::with_capacity(estimated_values);
let executable_blocks = HashSet::with_capacity(function.block_count());
```

**Lightweight Handles**:
- Use `NodeIndex` (u32) instead of `BasicBlock` references
- Use `Value` IDs instead of cloning instruction data

**Value Deduplication**:
```rust
fn add_ssa_to_worklist(&mut self, value: Value) {
    if self.ssa_seen.insert(value) {  // Only add if not seen before
        self.ssa_worklist.push_back(value);
    }
}
```

### Computational Efficiency

**Early Termination**:
```rust
if old_lattice == new_lattice {
    continue;  // No change, skip propagation
}
```

**Sparse Processing**:
- Only process instructions when operands change
- Only process blocks when edges become executable
- Avoid full-function scans

**Constant Folding Cache** (optional future optimization):
```rust
let mut fold_cache: HashMap<(IrBinaryOp, IrLiteralValue, IrLiteralValue), Option<IrLiteralValue>> 
    = HashMap::new();
```

## Conclusion

This research provides a comprehensive foundation for implementing SCCP in jsavrs. The key decisions are:

1. **Three-value lattice with meet operation** ensures monotonic convergence and correctness
2. **Wrapping arithmetic semantics** match Rust release-mode behavior for soundness
3. **Sparse worklist algorithm** achieves O(n) complexity with efficient memory usage
4. **Executable-edge-only phi analysis** correctly handles SSA merge points
5. **In-place transformation with dead marking** leverages existing DCE infrastructure
6. **Alternating SCCP-DCE pipeline** maximizes optimization opportunities through synergy

All technical unknowns from the planning phase have been resolved. The implementation can proceed with confidence in these architectural decisions.

## References

- Wegman, M. N., & Zadeck, F. K. (1991). "Constant propagation with conditional branches". ACM Transactions on Programming Languages and Systems.
- Cytron, R., et al. (1991). "Efficiently computing static single assignment form and the control dependence graph". ACM TOPLAS.
- Cooper, K. D., & Torczon, L. (2011). "Engineering a Compiler" (2nd ed.), Chapter 9: Data-Flow Analysis.
- The Rust Programming Language Book - Release vs Debug mode semantics
- jsavrs project documentation: QWEN.md, AGENTS.md
