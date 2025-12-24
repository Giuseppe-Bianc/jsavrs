# Rewriter API Contract

**Module**: `src/ir/optimizer/constant_folding/rewriter.rs`  
**Version**: 1.0  
**Status**: Phase 1 Design

## Overview

This module transforms IR based on SCCP analysis results by replacing constant computations, simplifying control flow, and marking unreachable code. It preserves SSA form while applying optimizations discovered during propagation.

## Public Types

### `IRRewriter`

```rust
pub struct IRRewriter<'a> {
    // Internal fields (not public)
}
```

**Purpose**: Rewrites IR based on SCCP lattice values and executable edges.

**Lifetime**: `'a` - References lattice state and executable edges from propagator

---

### `RewriteError`

```rust
#[derive(Debug, thiserror::Error)]
pub enum RewriteError {
    #[error("SSA form violation: {0}")]
    SSAViolation(String),
    
    #[error("Type mismatch during constant replacement: expected {expected:?}, got {actual:?}")]
    TypeMismatch { expected: IRType, actual: IRType },
    
    #[error("Invalid block ID: {0:?}")]
    InvalidBlockId(BlockId),
}
```

**Purpose**: Errors that can occur during IR rewriting.

---

### `OptimizationStats`

```rust
#[derive(Debug, Default, Clone)]
pub struct OptimizationStats {
    pub constants_propagated: usize,
    pub branches_resolved: usize,
    pub phi_nodes_simplified: usize,
    pub blocks_marked_unreachable: usize,
    pub iterations: usize,
}
```

**Purpose**: Tracks optimization metrics.

**Fields**:

- `constants_propagated` - Number of instructions replaced with constants
- `branches_resolved` - Number of conditional branches converted to unconditional
- `phi_nodes_simplified` - Number of phi nodes replaced with constants
- `blocks_marked_unreachable` - Number of blocks marked for DCE
- `iterations` - SCCP iterations to convergence

---

## Public API

### Constructor

#### `new`

```rust
pub fn new(
    lattice: &'a LatticeState,
    executable_edges: &'a ExecutableEdgeSet,
) -> Self
```

**Description**: Create rewriter from SCCP analysis results.

**Parameters**:

- `lattice: &'a LatticeState` - Final lattice values from propagator
- `executable_edges: &'a ExecutableEdgeSet` - Executable CFG edges

**Returns**: `Self` - New rewriter instance

**Examples**:

```rust
let rewriter = IRRewriter::new(
    propagator.get_lattice_state(),
    propagator.get_executable_edges(),
);
```

**Complexity**: O(1)

---

### IR Transformation

#### `rewrite_function`

```rust
pub fn rewrite_function(&mut self, function: &mut Function) -> Result<(), RewriteError>
```

**Description**: Rewrite entire function based on SCCP results.

**Parameters**:

- `function: &mut Function` - Function to transform (mutated in-place)

**Returns**:

- `Ok(())` - Rewriting succeeded
- `Err(RewriteError)` - SSA violation, type mismatch, or invalid IR

**Transformations Applied**:

1. Mark unreachable blocks (no executable incoming edges)
2. Replace constant-valued instructions with constant assignments
3. Simplify phi nodes with constant incoming values
4. Convert conditional branches with constant conditions to unconditional jumps

**SSA Preservation**: Maintains single-assignment invariant and dominance relations

**Examples**:

```rust
let mut rewriter = IRRewriter::new(lattice, edges);
rewriter.rewrite_function(&mut function)?;

let stats = rewriter.get_stats();
println!("Propagated {} constants", stats.constants_propagated);
```

**Complexity**: O(n) for n instructions

---

### Statistics

#### `get_stats`

```rust
pub fn get_stats(&self) -> &OptimizationStats
```

**Description**: Get optimization statistics accumulated during rewriting.

**Returns**: `&OptimizationStats` - Reference to statistics

**Examples**:

```rust
let stats = rewriter.get_stats();
println!("{}", stats); // Uses Display impl
```

**Complexity**: O(1)

---

## Transformation Details

### Constant Instruction Replacement

**Trigger**: Instruction result value has lattice value `Constant(v)`

**Transformation**:

```text
Before:
  %result = add %x, %y    // lattice[%result] = Constant(I32(42))

After:
  %result = const 42
```

**Constraints**:

- Result value type must match constant type
- Only applied to reachable instructions
- Preserves SSA def-use chains

**Examples**:

```rust
// Before SCCP:
// %1 = const 10
// %2 = const 32
// %3 = add %1, %2

// After rewriter.rewrite_function():
// %1 = const 10
// %2 = const 32
// %3 = const 42
```

---

### Phi Node Simplification

**Trigger**: All executable incoming values are the same constant

**Transformation**:

```text
Before:
  %result = phi [bb1: %v1, bb2: %v2, bb3: %v3]
  // lattice[%v1] = Constant(I32(42))
  // lattice[%v2] = Constant(I32(42))
  // lattice[%v3] = Constant(I32(42))
  // edges (bb1→current), (bb2→current), (bb3→current) all executable

After:
  %result = const 42
```

**Constraints**:

- Only considers executable predecessor edges
- All executable incoming values must be identical constants
- Result type must match constant type

**Examples**:

```rust
// Before (after SCCP marks edges):
// bb1:
//   %v1 = const 42
//   br bb_merge
// bb2:
//   %v2 = const 42
//   br bb_merge
// bb_merge:
//   %result = phi [bb1: %v1, bb2: %v2]

// After rewriter.rewrite_function():
// bb_merge:
//   %result = const 42
```

---

### Branch Resolution

**Trigger**: Conditional branch condition has lattice value `Constant(Bool(b))`

**Transformation (condition = true)**:

```text
Before:
  br %cond, label %true_block, label %false_block
  // lattice[%cond] = Constant(Bool(true))

After:
  br label %true_block
```

**Transformation (condition = false)**:

```text
Before:
  br %cond, label %true_block, label %false_block
  // lattice[%cond] = Constant(Bool(false))

After:
  br label %false_block
```

**Constraints**:

- Only applied to reachable blocks
- Condition must be proven constant (not just likely)

**Examples**:

```rust
// Before:
// %cond = const true
// br %cond, label %true_block, label %false_block

// After rewriter.rewrite_function():
// %cond = const true
// br label %true_block
```

---

### Unreachable Block Marking

**Trigger**: Block has no executable incoming CFG edges (except entry block)

**Transformation**:

```text
Before:
  bb5:  // No executable predecessors
    %x = add %a, %b
    br label %exit

After:
  bb5:  // Marked unreachable (metadata)
    %x = add %a, %b
    br label %exit
```

**Marking Method**: Sets `unreachable` metadata flag on basic block

**DCE Coordination**: Marked blocks removed by subsequent DCE pass

**Constraints**:

- Entry block never marked unreachable
- Blocks are marked but NOT deleted by rewriter

**Examples**:

```rust
// Before:
// bb1:
//   br bb3
// bb2:  // No predecessors reach here
//   %x = const 99
//   br bb4

// After rewriter.rewrite_function():
// bb1:
//   br bb3
// bb2:  // Marked: metadata["unreachable"] = true
//   %x = const 99
//   br bb4
```

---

## SSA Form Preservation

### Single Assignment Invariant

**Guarantee**: Every value still has exactly one static definition after rewriting

**How**: Replacement operations modify the RHS of assignments, not LHS

```rust
// Valid transformation:
%result = add %x, %y  →  %result = const 42

// Invalid (would violate SSA):
%result = add %x, %y
// later...
%result = const 42  // ERROR: %result redefined
```

---

### Dominance Preservation

**Guarantee**: All uses still dominated by definitions

**How**: Transformations only change instruction bodies, not control flow structure (except unreachable marking)

**Verification**: Optional dominance check in debug builds

---

## Usage Examples

### Complete Optimization Pipeline

```rust
use jsavrs::ir::optimizer::constant_folding::{
    SCCPropagator, SCCPConfig, IRRewriter
};

// Step 1: Run SCCP analysis
let config = SCCPConfig::default();
let mut propagator = SCCPropagator::new_for_function(&function, config);
propagator.propagate(&function)?;

// Step 2: Rewrite IR based on results
let mut rewriter = IRRewriter::new(
    propagator.get_lattice_state(),
    propagator.get_executable_edges(),
);
rewriter.rewrite_function(&mut function)?;

// Step 3: Report statistics
let mut stats = rewriter.get_stats().clone();
stats.iterations = propagator.iteration_count();
println!("{}", stats);
```

### Incremental Rewriting

```rust
let mut rewriter = IRRewriter::new(lattice, edges);

// Rewrite specific blocks only
for block in function.basic_blocks_mut() {
    if should_optimize(block) {
        // Internal method (not public API)
        // rewriter.rewrite_block(block)?;
    }
}
```

---

## Error Handling

### SSA Violations

**Cause**: Internal rewriter bug (should not occur with correct implementation)

**Detection**: Verification checks in debug builds

**Recovery**: Return error immediately, abort optimization

### Type Mismatches

**Cause**: Constant value type doesn't match instruction result type

**Detection**: Type checking before replacement

**Recovery**: Skip transformation, log warning, continue

### Invalid Block IDs

**Cause**: Reference to non-existent block

**Detection**: Block lookup failure

**Recovery**: Return error immediately

---

## Performance Characteristics

- **Time Complexity**: O(n) for n instructions
- **Space Complexity**: O(1) additional (modifies in-place)
- **Memory**: No heap allocations during rewriting

---

## Testing Requirements

1. **Unit Tests**: Each transformation type in isolation
2. **SSA Verification**: Validate SSA form after each rewriting test
3. **Integration Tests**: Combined with SCCP propagator
4. **Snapshot Tests**: Before/after IR comparisons with insta
5. **Statistics Tests**: Verify counts match transformations

---

## Invariants

1. **SSA Form**: Single assignment preserved
2. **Dominance**: Use-def dominance maintained
3. **Type Safety**: All replacements type-correct
4. **Idempotence**: Rewriting twice produces same result as once
5. **Conservatism**: Never transforms unless proven safe

---

**API Contract Status**: ✅ Complete  
**Implementation Status**: Pending  
**Review Status**: Pending
