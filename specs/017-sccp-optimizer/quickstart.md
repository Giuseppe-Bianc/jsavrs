# Quick Start Guide: SCCP Optimizer Development

**Feature**: Sparse Conditional Constant Propagation  
**Branch**: `017-sccp-optimizer`  
**Date**: 2025-11-19

## Overview

This guide provides a streamlined path to implementing the SCCP optimizer in the jsavrs compiler. Follow these steps to build a production-quality constant propagation optimization phase using lattice-based dataflow analysis.

## Prerequisites

- Rust 2024 edition environment
- Familiarity with SSA form and control flow graphs
- jsavrs repository cloned and building successfully
- Understanding of basic compiler optimization concepts

## Development Roadmap

### Phase 1: Foundation (Days 1-2)

**Goal**: Establish core data structures and lattice theory implementation

**Files to Create**:
1. `src/ir/optimizer/constant_folding/mod.rs` - Module declaration and public API
2. `src/ir/optimizer/constant_folding/lattice.rs` - LatticeValue enum and meet operation

**Step 1.1: Create Module Structure**

```bash
# Create directory
mkdir src/ir/optimizer/constant_folding

# Create initial files
touch src/ir/optimizer/constant_folding/mod.rs
touch src/ir/optimizer/constant_folding/lattice.rs
```

**Step 1.2: Implement Lattice (lattice.rs)**

```rust
use crate::ir::IrLiteralValue;

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
    
    pub fn is_constant(&self) -> bool {
        matches!(self, LatticeValue::Constant(_))
    }
    
    pub fn as_constant(&self) -> Option<&IrLiteralValue> {
        if let LatticeValue::Constant(c) = self {
            Some(c)
        } else {
            None
        }
    }
}
```

**Step 1.3: Add Unit Tests**

Create `src/ir/optimizer/constant_folding/lattice.rs` tests module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IrLiteralValue;
    
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
    fn test_different_constants_to_bottom() {
        let a = LatticeValue::Constant(IrLiteralValue::I32(42));
        let b = LatticeValue::Constant(IrLiteralValue::I32(17));
        assert_eq!(a.meet(&b), LatticeValue::Bottom);
    }
}
```

**Verification**: `cargo test lattice` should pass all tests

---

### Phase 2: Constant Folding (Days 3-4)

**Goal**: Implement pattern-matched constant evaluation for all supported operations

**File to Create**: `src/ir/optimizer/constant_folding/constant_folder.rs`

**Step 2.1: Basic Structure**

```rust
use crate::ir::{IrLiteralValue, IrBinaryOp, IrUnaryOp};

pub fn fold_binary(
    op: IrBinaryOp, 
    left: IrLiteralValue, 
    right: IrLiteralValue
) -> Option<IrLiteralValue> {
    use IrLiteralValue::*;
    
    match (op, left, right) {
        // Integer addition
        (IrBinaryOp::Add, I8(a), I8(b)) => Some(I8(a.wrapping_add(b))),
        (IrBinaryOp::Add, I16(a), I16(b)) => Some(I16(a.wrapping_add(b))),
        (IrBinaryOp::Add, I32(a), I32(b)) => Some(I32(a.wrapping_add(b))),
        (IrBinaryOp::Add, I64(a), I64(b)) => Some(I64(a.wrapping_add(b))),
        
        (IrBinaryOp::Add, U8(a), U8(b)) => Some(U8(a.wrapping_add(b))),
        (IrBinaryOp::Add, U16(a), U16(b)) => Some(U16(a.wrapping_add(b))),
        (IrBinaryOp::Add, U32(a), U32(b)) => Some(U32(a.wrapping_add(b))),
        (IrBinaryOp::Add, U64(a), U64(b)) => Some(U64(a.wrapping_add(b))),
        
        // Float addition
        (IrBinaryOp::Add, F32(a), F32(b)) => Some(F32(a + b)),
        (IrBinaryOp::Add, F64(a), F64(b)) => Some(F64(a + b)),
        
        // Division by zero → None
        (IrBinaryOp::Divide, I32(_), I32(0)) => None,
        (IrBinaryOp::Divide, I32(a), I32(b)) => Some(I32(a.wrapping_div(b))),
        
        // Add all other operations similarly...
        _ => None, // Type mismatch or unsupported operation
    }
}

pub fn fold_unary(op: IrUnaryOp, operand: IrLiteralValue) -> Option<IrLiteralValue> {
    use IrLiteralValue::*;
    
    match (op, operand) {
        (IrUnaryOp::Negate, I32(v)) => Some(I32(v.wrapping_neg())),
        (IrUnaryOp::Negate, F64(v)) => Some(F64(-v)),
        (IrUnaryOp::Not, Bool(v)) => Some(Bool(!v)),
        // Add all other types...
        _ => None,
    }
}
```

**Step 2.2: Comprehensive Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wrapping_overflow() {
        assert_eq!(
            fold_binary(IrBinaryOp::Add, I8(127), I8(1)),
            Some(I8(-128))
        );
    }
    
    #[test]
    fn test_division_by_zero() {
        assert_eq!(
            fold_binary(IrBinaryOp::Divide, I32(42), I32(0)),
            None
        );
    }
}
```

**Verification**: `cargo test constant_folder` should pass

---

### Phase 3: Worklist Management (Day 5)

**Goal**: Implement efficient work item queue with deduplication

**File to Create**: `src/ir/optimizer/constant_folding/worklist.rs`

**Step 3.1: Implementation**

```rust
use crate::ir::Value;
use petgraph::graph::NodeIndex;
use std::collections::{VecDeque, HashSet};

pub struct Worklist {
    ssa_worklist: VecDeque<Value>,
    cfg_worklist: VecDeque<(NodeIndex, NodeIndex)>,
    ssa_seen: HashSet<Value>,
    cfg_seen: HashSet<(NodeIndex, NodeIndex)>,
}

impl Worklist {
    pub fn new() -> Self {
        Self {
            ssa_worklist: VecDeque::new(),
            cfg_worklist: VecDeque::new(),
            ssa_seen: HashSet::new(),
            cfg_seen: HashSet::new(),
        }
    }
    
    pub fn add_ssa(&mut self, value: Value) {
        if self.ssa_seen.insert(value) {
            self.ssa_worklist.push_back(value);
        }
    }
    
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        let edge = (from, to);
        if self.cfg_seen.insert(edge) {
            self.cfg_worklist.push_back(edge);
        }
    }
    
    pub fn pop_ssa(&mut self) -> Option<Value> {
        let value = self.ssa_worklist.pop_front()?;
        self.ssa_seen.remove(&value);
        Some(value)
    }
    
    pub fn pop_cfg(&mut self) -> Option<(NodeIndex, NodeIndex)> {
        self.cfg_worklist.pop_front()
        // Note: cfg_seen NOT cleared (edges stay executable)
    }
    
    pub fn is_empty(&self) -> bool {
        self.ssa_worklist.is_empty() && self.cfg_worklist.is_empty()
    }
}
```

**Verification**: Test deduplication behavior

---

### Phase 4: Core Analysis Algorithm (Days 6-8)

**Goal**: Implement main SCCP dataflow analysis with worklist iteration

**File to Create**: `src/ir/optimizer/constant_folding/analyzer.rs`

**Step 4.1: Analyzer Structure**

```rust
use super::{LatticeValue, Worklist, constant_folder};
use crate::ir::{Value, Function};
use std::collections::{HashMap, HashSet};
use petgraph::graph::NodeIndex;

pub struct SccpAnalyzer {
    lattice_values: HashMap<Value, LatticeValue>,
    executable_blocks: HashSet<NodeIndex>,
    worklist: Worklist,
    max_iterations: usize,
}

impl SccpAnalyzer {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            lattice_values: HashMap::new(),
            executable_blocks: HashSet::new(),
            worklist: Worklist::new(),
            max_iterations,
        }
    }
    
    pub fn analyze(&mut self, function: &Function) -> AnalysisResult {
        self.initialize(function);
        self.run_worklist_algorithm(function);
        
        AnalysisResult {
            lattice_values: self.lattice_values.clone(),
            executable_blocks: self.executable_blocks.clone(),
            iterations: /* track this */,
            converged: /* check this */,
        }
    }
    
    fn initialize(&mut self, function: &Function) {
        // Mark entry block executable
        let entry_block = function.entry_block();
        self.executable_blocks.insert(entry_block);
        
        // Initialize all SSA values to Top
        for block in function.blocks() {
            for inst in block.instructions() {
                if let Some(result) = inst.result {
                    self.lattice_values.insert(result, LatticeValue::Top);
                }
            }
        }
        
        // Add entry block edges to worklist
        for succ in function.cfg().successors(entry_block) {
            self.worklist.add_edge(entry_block, succ);
        }
    }
    
    fn run_worklist_algorithm(&mut self, function: &Function) {
        let mut iterations = 0;
        
        while !self.worklist.is_empty() && iterations < self.max_iterations {
            iterations += 1;
            
            // Process CFG edges
            while let Some((from, to)) = self.worklist.pop_cfg() {
                self.process_cfg_edge(function, from, to);
            }
            
            // Process SSA values
            while let Some(value) = self.worklist.pop_ssa() {
                self.process_ssa_value(function, value);
            }
        }
    }
}
```

**Step 4.2: Instruction Evaluation**

```rust
impl SccpAnalyzer {
    fn evaluate_instruction(&mut self, inst: &Instruction) -> LatticeValue {
        match &inst.kind {
            InstructionKind::Binary { op, left, right, .. } => {
                let left_lat = self.lattice_values.get(left).unwrap_or(&LatticeValue::Bottom);
                let right_lat = self.lattice_values.get(right).unwrap_or(&LatticeValue::Bottom);
                
                match (left_lat, right_lat) {
                    (LatticeValue::Constant(l), LatticeValue::Constant(r)) => {
                        if let Some(result) = constant_folder::fold_binary(*op, *l, *r) {
                            LatticeValue::Constant(result)
                        } else {
                            LatticeValue::Bottom
                        }
                    }
                    (LatticeValue::Top, _) | (_, LatticeValue::Top) => LatticeValue::Top,
                    _ => LatticeValue::Bottom,
                }
            }
            
            InstructionKind::Phi { incoming, .. } => {
                self.evaluate_phi(incoming)
            }
            
            InstructionKind::Call { .. } => LatticeValue::Bottom,
            InstructionKind::Load { .. } => LatticeValue::Bottom,
            
            // Add other instruction kinds...
        }
    }
    
    fn evaluate_phi(&self, incoming: &[(Value, String)]) -> LatticeValue {
        let mut result = LatticeValue::Top;
        
        for (value, pred_label) in incoming {
            let pred_block = /* lookup block by label */;
            
            if self.executable_blocks.contains(&pred_block) {
                let value_lat = self.lattice_values.get(value).unwrap_or(&LatticeValue::Bottom);
                result = result.meet(value_lat);
            }
        }
        
        result
    }
}
```

**Verification**: Test on small hand-crafted IR functions

---

### Phase 5: IR Transformation (Days 9-10)

**Goal**: Mutate IR based on analysis results

**File to Create**: `src/ir/optimizer/constant_folding/transformer.rs`

**Step 5.1: Transformer Structure**

```rust
use super::AnalysisResult;
use crate::ir::Function;

pub struct SccpTransformer {
    stats: TransformStats,
    verbose: bool,
}

impl SccpTransformer {
    pub fn new(verbose: bool) -> Self {
        Self {
            stats: TransformStats::default(),
            verbose,
        }
    }
    
    pub fn transform(&mut self, function: &mut Function, result: &AnalysisResult) -> bool {
        self.replace_constants(function, result);
        self.simplify_branches(function, result);
        self.clean_phi_nodes(function, result);
        
        self.stats.has_changes()
    }
    
    fn replace_constants(&mut self, function: &mut Function, result: &AnalysisResult) {
        for (value, lattice) in &result.lattice_values {
            if let LatticeValue::Constant(c) = lattice {
                // Replace all uses of `value` with literal `c`
                // Mark defining instruction dead
                self.stats.constants_propagated += 1;
            }
        }
    }
    
    fn simplify_branches(&mut self, function: &mut Function, result: &AnalysisResult) {
        // Convert ConditionalBranch with constant condition to Branch
        self.stats.branches_simplified += /* count */;
    }
    
    fn clean_phi_nodes(&mut self, function: &mut Function, result: &AnalysisResult) {
        // Remove dead predecessor entries from phi nodes
        self.stats.phi_nodes_cleaned += /* count */;
    }
}
```

**Verification**: Test IR mutation correctness

---

### Phase 6: Public API and Integration (Day 11)

**Goal**: Implement Phase trait and integrate with optimizer pipeline

**File to Update**: `src/ir/optimizer/constant_folding/mod.rs`

**Step 6.1: Public API**

```rust
mod lattice;
mod constant_folder;
mod worklist;
mod analyzer;
mod transformer;

pub use lattice::LatticeValue;
use analyzer::SccpAnalyzer;
use transformer::SccpTransformer;

use crate::ir::{Module, optimizer::Phase};

pub struct SccpOptimizer {
    pub verbose: bool,
    pub max_iterations: usize,
    pub enabled: bool,
}

impl SccpOptimizer {
    pub fn new() -> Self {
        Self {
            verbose: false,
            max_iterations: 10_000,
            enabled: true,
        }
    }
    
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    
    fn run_on_function(&mut self, function: &mut Function) -> bool {
        let mut analyzer = SccpAnalyzer::new(self.max_iterations);
        let result = analyzer.analyze(function);
        
        let mut transformer = SccpTransformer::new(self.verbose);
        transformer.transform(function, &result)
    }
}

impl Phase for SccpOptimizer {
    fn name(&self) -> &'static str {
        "SCCP"
    }
    
    fn run(&mut self, module: &mut Module) -> bool {
        if !self.enabled {
            return false;
        }
        
        let mut changed = false;
        for function in module.functions_mut() {
            changed |= self.run_on_function(function);
        }
        changed
    }
}
```

**Step 6.2: Register in Optimizer Module**

Update `src/ir/optimizer/mod.rs`:

```rust
pub mod phase;
pub mod dead_code_elimination;
pub mod constant_folding;
pub mod sccp;  // Add this line

pub use phase::Phase;
```

**Verification**: `cargo build` should succeed

---

### Phase 7: Testing and Validation (Days 12-14)

**Goal**: Comprehensive test coverage at all levels

**Step 7.1: Integration Tests**

Create `tests/sccp_integration_tests.rs`:

```rust
use jsavrs::ir::optimizer::{Phase, sccp::SccpOptimizer};

#[test]
fn test_constant_propagation() {
    let mut module = build_test_ir(/* source code */);
    
    let mut sccp = SccpOptimizer::new();
    assert!(sccp.run(&mut module));
    
    // Verify constants propagated
    // ...
}

#[test]
fn test_branch_simplification() {
    // Test constant condition branches
}

#[test]
fn test_phi_resolution() {
    // Test phi nodes with constant values
}
```

**Step 7.2: Snapshot Tests**

Create `tests/sccp_snapshot_tests.rs`:

```rust
use insta::assert_snapshot;

#[test]
fn test_sccp_constant_chain() {
    let mut module = /* build IR */;
    
    let before = format!("{:?}", module);
    
    SccpOptimizer::new().run(&mut module);
    
    let after = format!("{:?}", module);
    
    assert_snapshot!("constant_chain_before", before);
    assert_snapshot!("constant_chain_after", after);
}
```

**Step 7.3: Performance Benchmarks**

Create `benches/sccp_benchmark.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsavrs::ir::optimizer::{Phase, sccp::SccpOptimizer};

fn bench_sccp(c: &mut Criterion) {
    let module = generate_large_function(10_000);
    
    c.bench_function("sccp_10k_instructions", |b| {
        b.iter(|| {
            let mut m = module.clone();
            SccpOptimizer::new().run(black_box(&mut m))
        });
    });
}

criterion_group!(benches, bench_sccp);
criterion_main!(benches);
```

**Verification**: All tests pass, benchmarks meet performance targets

---

## Common Pitfalls and Solutions

### Pitfall 1: Phi Node Incorrect Analysis

**Problem**: Considering values from non-executable predecessors

**Solution**: Always check `executable_blocks.contains(pred_block)` before using phi incoming values

### Pitfall 2: Lattice Non-Monotonicity

**Problem**: Values moving upward in lattice (Bottom → Constant)

**Solution**: Ensure `meet` operation only moves downward; never overwrite Bottom with Constant

### Pitfall 3: Infinite Loops

**Problem**: Worklist never empties

**Solution**: Implement `max_iterations` safety limit and degrade remaining Top values to Bottom

### Pitfall 4: Type Mismatches in Folding

**Problem**: Attempting to add i32 and f64

**Solution**: Return `None` from `fold_binary` for type mismatches, resulting in Bottom

---

## Performance Optimization Tips

1. **Pre-allocate HashMaps**: Use `with_capacity(estimated_size)`
2. **Value Deduplication**: Check `seen` sets before worklist insertion
3. **Early Termination**: Skip processing if lattice value unchanged
4. **Avoid Cloning**: Use references where possible, clone only IrLiteralValue

---

## Debugging Strategies

### Enable Trace Logging

```rust
if std::env::var("SCCP_TRACE").is_ok() {
    eprintln!("SCCP: {} changed from {:?} to {:?}", value, old, new);
}
```

### Visualize Lattice States

```rust
fn print_lattice_state(&self) {
    for (value, lattice) in &self.lattice_values {
        println!("{:?}: {:?}", value, lattice);
    }
}
```

### Assert Invariants

```rust
debug_assert!(
    self.iteration_count <= self.max_iterations,
    "Iteration count exceeded limit"
);
```

---

## Integration with Compiler Pipeline

### Recommended Pipeline Order

```rust
fn optimize_module(module: &mut Module) {
    let phases: Vec<Box<dyn Phase>> = vec![
        Box::new(SccpOptimizer::new()),           // 1. Discover constants
        Box::new(DeadCodeElimination::new()),     // 2. Remove dead code
        Box::new(SccpOptimizer::new()),           // 3. Re-run SCCP
        Box::new(DeadCodeElimination::new()),     // 4. Final cleanup
    ];
    
    run_pipeline(module, phases);
}
```

---

## Next Steps After Completion

1. **Interprocedural Analysis**: Extend to analyze across function calls
2. **Alias Analysis Integration**: Handle memory operations more precisely
3. **Loop-Aware Optimization**: Special handling for induction variables
4. **Profile-Guided Optimization**: Use runtime profiles for branch prediction

---

## Success Criteria Checklist

- [ ] All unit tests pass (`cargo test`)
- [ ] Integration tests demonstrate correct constant propagation
- [ ] Snapshot tests catch regressions
- [ ] Benchmarks meet O(V+E) complexity targets
- [ ] Phase trait integration works with existing pipeline
- [ ] Verbose logging provides useful optimization insights
- [ ] Documentation complete (rustdoc comments on public APIs)
- [ ] Code review feedback addressed
- [ ] Constitution compliance verified
- [ ] Architectural patterns match existing optimizer phases (see dead_code_elimination/)
- [ ] Public APIs follow crate conventions (visibility, re-exports, documentation)
- [ ] Module structure follows src/ir/optimizer/ patterns

---

## Resources

- **Research Document**: `specs/017-sccp-optimizer/research.md` (detailed algorithm analysis)
- **Data Model**: `specs/017-sccp-optimizer/data-model.md` (entity specifications)
- **API Contract**: `specs/017-sccp-optimizer/contracts/api.md` (public interface)
- **Wegman-Zadeck Paper**: Original SCCP algorithm publication
- **SSA Book**: "SSA-based Compiler Design" for SSA form details

---

## Getting Help

- Review existing optimizer phases: `src/ir/optimizer/dead_code_elimination/`
- Check IR data structures: `src/ir/` module documentation
- Ask questions in pull request reviews
- Reference AGENTS.md for AI-assisted development patterns

---

**Estimated Timeline**: 14 days for full implementation and testing

**Good luck building the SCCP optimizer!**
