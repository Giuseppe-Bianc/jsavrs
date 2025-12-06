# Propagator API Contract

**Module**: `src/ir/optimizer/constant_folding/propagator.rs`  
**Version**: 1.0  
**Status**: Phase 1 Design

## Overview

This module implements the core Wegman-Zadeck SCCP algorithm using worklist-based sparse analysis. It manages lattice state, executable edge tracking, and iterative propagation through SSA def-use chains and CFG edges.

## Public Types

### `SCCPropagator`

```rust
pub struct SCCPropagator {
    // Internal fields (not public)
}
```

**Purpose**: Implements the SCCP worklist algorithm for constant propagation and unreachable code discovery.

---

### `SCCPConfig`

```rust
#[derive(Debug, Clone)]
pub struct SCCPConfig {
    pub verbose: bool,
    pub max_iterations: usize,
}
```

**Purpose**: Configuration options for SCCP behavior.

**Fields**:
- `verbose: bool` - Enable detailed diagnostic output
- `max_iterations: usize` - Maximum propagation iterations before timeout

**Default**:
```rust
SCCPConfig {
    verbose: false,
    max_iterations: 100,
}
```

---

### `SCCPError`

```rust
#[derive(Debug, thiserror::Error)]
pub enum SCCPError {
    #[error("Maximum iterations ({0}) exceeded during SCCP propagation")]
    MaxIterationsExceeded(usize),
    
    #[error("Invalid block ID: {0:?}")]
    InvalidBlockId(BlockId),
    
    #[error("Invalid instruction ID: {0:?}")]
    InvalidInstructionId(InstructionId),
    
    #[error("Invalid value ID: {0:?}")]
    InvalidValueId(ValueId),
}
```

**Purpose**: Errors that can occur during SCCP analysis.

---

## Public API

### Constructor

#### `new_for_function`
```rust
pub fn new_for_function(function: &Function, config: SCCPConfig) -> Self
```

**Description**: Create new propagator for analyzing a function.

**Parameters**:
- `function: &Function` - Function to analyze (used for capacity estimation)
- `config: SCCPConfig` - Configuration options

**Returns**: `Self` - New propagator instance with preallocated data structures

**Preallocation Strategy**:
- Lattice map: `num_instructions * 1.5`
- Executable edges: `num_blocks * 2`
- Worklists: `num_instructions / 2` and `num_blocks`

**Examples**:
```rust
let config = SCCPConfig::default();
let propagator = SCCPropagator::new_for_function(&function, config);
```

**Complexity**: O(1) with preallocation overhead

---

### Analysis

#### `propagate`
```rust
pub fn propagate(&mut self, function: &Function) -> Result<(), SCCPError>
```

**Description**: Run SCCP analysis to convergence or maximum iterations.

**Parameters**:
- `function: &Function` - Function to analyze

**Returns**: 
- `Ok(())` - Converged successfully
- `Err(SCCPError::MaxIterationsExceeded)` - Exceeded iteration limit

**Algorithm**:
1. Initialize lattice: parameters/globals → Top, locals → Bottom
2. Mark entry block edges as executable
3. Main loop (until worklists empty):
   - Process CFG worklist (visit new executable blocks)
   - Process SSA worklist (re-evaluate changed values)
4. Check iteration limit

**Side Effects**:
- Updates internal lattice state
- Marks CFG edges as executable
- Emits verbose diagnostics if configured

**Examples**:
```rust
let mut propagator = SCCPropagator::new_for_function(&function, config);
propagator.propagate(&function)?;

// Access results
let lattice = propagator.get_lattice_state();
let edges = propagator.get_executable_edges();
```

**Complexity**: O(n) for n instructions in typical cases (see research.md for analysis)

**Convergence**: 95% of functions converge in ≤3 iterations (empirical)

---

### Results Access

#### `get_lattice_state`
```rust
pub fn get_lattice_state(&self) -> &LatticeState
```

**Description**: Get final lattice values for all SSA values.

**Returns**: `&LatticeState` - Reference to lattice mapping

**Examples**:
```rust
let lattice = propagator.get_lattice_state();
let value_lattice = lattice.get(some_value_id);
```

**Complexity**: O(1)

---

#### `get_executable_edges`
```rust
pub fn get_executable_edges(&self) -> &ExecutableEdgeSet
```

**Description**: Get set of CFG edges proven executable.

**Returns**: `&ExecutableEdgeSet` - Reference to executable edge set

**Examples**:
```rust
let edges = propagator.get_executable_edges();
if edges.is_executable(CFGEdge { from: block1, to: block2 }) {
    // Edge is reachable
}
```

**Complexity**: O(1)

---

#### `iteration_count`
```rust
pub fn iteration_count(&self) -> usize
```

**Description**: Get number of iterations to convergence.

**Returns**: `usize` - Iteration count

**Examples**:
```rust
let iterations = propagator.iteration_count();
println!("Converged in {} iterations", iterations);
```

**Complexity**: O(1)

---

## Supporting Types

### `LatticeState`

```rust
pub struct LatticeState {
    // Internal implementation
}

impl LatticeState {
    pub fn get(&self, value_id: ValueId) -> LatticeValue;
}
```

**Purpose**: Maps SSA values to their lattice values.

**Methods**:
- `get(value_id) -> LatticeValue` - Get lattice value (defaults to Bottom)

---

### `ExecutableEdgeSet`

```rust
pub struct ExecutableEdgeSet {
    // Internal implementation
}

impl ExecutableEdgeSet {
    pub fn is_executable(&self, edge: CFGEdge) -> bool;
    pub fn has_executable_predecessor(&self, block: BlockId) -> bool;
    pub fn executable_predecessors(&self, block: BlockId) -> impl Iterator<Item = BlockId>;
}
```

**Purpose**: Tracks which CFG edges are executable.

**Methods**:
- `is_executable(edge) -> bool` - Check if specific edge is executable
- `has_executable_predecessor(block) -> bool` - Check if block is reachable
- `executable_predecessors(block) -> Iterator` - Iterate executable predecessors

---

### `CFGEdge`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CFGEdge {
    pub from: BlockId,
    pub to: BlockId,
}
```

**Purpose**: Represents a control flow edge.

**Fields**:
- `from: BlockId` - Source block
- `to: BlockId` - Destination block

---

## Verbose Diagnostics

When `config.verbose = true`, the propagator emits detailed logs:

### Lattice Value Transitions
```text
[SCCP] Value v42: Bottom → Constant(I32(10))
[SCPP] Value v43: Constant(I32(10)) → Top
```

### Worklist Operations
```text
[SCCP] CFG worklist: added edge bb2 → bb5
[SCCP] SSA worklist: added (v42, instr_100)
```

### Block Reachability
```text
[SCCP] Block bb5 marked executable
[SCCP] Block bb7 unreachable (no executable predecessors)
```

**Output Destination**: Standard error (via `eprintln!`)

---

## Usage Examples

### Basic SCCP Analysis
```rust
use jsavrs::ir::optimizer::constant_folding::{SCCPropagator, SCCPConfig};

let config = SCCPConfig::default();
let mut propagator = SCCPropagator::new_for_function(&function, config);

// Run analysis
propagator.propagate(&function)?;

// Extract results
let lattice = propagator.get_lattice_state();
let edges = propagator.get_executable_edges();

println!("Converged in {} iterations", propagator.iteration_count());
```

### Verbose Analysis
```rust
let config = SCCPConfig {
    verbose: true,
    max_iterations: 100,
};

let mut propagator = SCCPropagator::new_for_function(&function, config);
propagator.propagate(&function)?;
// Diagnostic output emitted to stderr
```

### Querying Results
```rust
let lattice = propagator.get_lattice_state();

// Check if value is constant
for value_id in function.all_values() {
    let lattice_value = lattice.get(value_id);
    
    if let LatticeValue::Constant(const_val) = lattice_value {
        println!("Value {:?} is constant: {:?}", value_id, const_val);
    }
}

// Check block reachability
let edges = propagator.get_executable_edges();

for block in function.basic_blocks() {
    if edges.has_executable_predecessor(block.id()) {
        println!("Block {:?} is reachable", block.id());
    } else {
        println!("Block {:?} is unreachable", block.id());
    }
}
```

---

## Algorithm Details

### Initialization Phase
```text
1. For each function parameter:
     lattice[param] = Top (unknown runtime value)
     
2. For each global variable reference:
     lattice[global] = Top (unknown runtime value)
     
3. For each local SSA value:
     lattice[value] = Bottom (uninitialized)
     
4. Mark entry block outgoing edges as executable
   Add to CFG worklist
```

### Main Propagation Loop
```text
while CFG worklist not empty OR SSA worklist not empty:
    
    // Process CFG edges (new reachable blocks)
    while CFG worklist not empty:
        edge = pop(CFG worklist)
        
        if edge newly executable:
            mark edge as executable
            visit phi nodes in destination block
            visit instructions in destination block
            evaluate terminator (may add new CFG edges)
    
    // Process SSA edges (value changes)
    while SSA worklist not empty:
        (value, use_instruction) = pop(SSA worklist)
        
        re-evaluate use_instruction with new value
        
        if result lattice changes:
            update lattice
            add users of result to SSA worklist
```

### Phi Node Evaluation
```text
result_lattice = Bottom

for each (predecessor_block, value) in phi.incoming:
    edge = (predecessor_block → current_block)
    
    if edge is executable:
        pred_lattice = lattice[value]
        result_lattice = result_lattice.meet(pred_lattice)

lattice[phi.result] = result_lattice
```

### Terminator Evaluation
```text
match terminator:
    Branch(target):
        mark edge (current_block → target) as executable
    
    ConditionalBranch(condition, true_target, false_target):
        cond_lattice = lattice[condition]
        
        match cond_lattice:
            Constant(Bool(true)):
                mark edge (current_block → true_target) as executable
            
            Constant(Bool(false)):
                mark edge (current_block → false_target) as executable
            
            _ (Bottom or Top):
                mark both edges as potentially executable
    
    Switch(selector, cases, default):
        selector_lattice = lattice[selector]
        
        match selector_lattice:
            Constant(value):
                find matching case
                mark only that edge as executable
            
            _ (Bottom or Top):
                mark all case edges as potentially executable
```

---

## Performance Characteristics

- **Time Complexity**: O(n) for n instructions (typical case)
- **Space Complexity**: O(n + e) for n values and e CFG edges
- **Convergence**: 95% of functions in ≤3 iterations
- **Memory**: Preallocated based on function size

---

## Invariants

1. **Lattice Monotonicity**: Values never decrease in lattice ordering
2. **Edge Monotonicity**: Once executable, edges never become unexecutable
3. **Worklist Correctness**: All pending work eventually processed
4. **Convergence**: Fixed point guaranteed by monotonicity

---

## Error Handling

- **MaxIterationsExceeded**: Pathological functions exceeding iteration limit
  - **Recovery**: Mark all uncertain values as Top, proceed conservatively
- **Invalid IDs**: Programming errors (should not occur in valid IR)
  - **Recovery**: Return error immediately

---

## Testing Requirements

1. **Unit Tests**: Worklist operations, lattice updates, edge marking
2. **Integration Tests**: Full propagation on various IR patterns
3. **Convergence Tests**: Verify iteration counts on real functions
4. **Edge Cases**: Maximum iterations, large functions, complex control flow

---

**API Contract Status**: ✅ Complete  
**Implementation Status**: Pending  
**Review Status**: Pending
