# API Contract: SCCP Worklist Algorithm

**Module**: `src/ir/optimizer/constant_folding/worklist.rs`  
**Purpose**: Define the contract for the Sparse Conditional Constant Propagation worklist algorithm

---

## Interface Definition

### Primary Function

```rust
/// Performs Sparse Conditional Constant Propagation analysis on a function.
/// 
/// # Arguments
/// * `function` - The IR function to analyze
/// * `verbose` - Enable detailed diagnostic output
/// 
/// # Returns
/// * `Ok(SCCPResult)` - Analysis results with lattice values and reachable blocks
/// * `Err(SCCPError::MemoryLimit)` - If lattice memory exceeds 100KB limit
/// 
/// # Algorithm
/// 1. Initialize lattice values (all Top except entry block)
/// 2. Mark entry block executable
/// 3. Process worklists (CFG edges and SSA values) until fixed point
/// 4. Return final lattice state and reachability information
/// 
/// # Complexity
/// O(n + e) where n = number of SSA values, e = number of CFG edges
pub fn sccp_analysis(
    function: &Function,
    verbose: bool
) -> Result<SCCPResult, SCCPError>;
```

---

## Data Structures

### SCCPResult

```rust
/// Results of SCCP analysis for a single function.
#[derive(Debug, Clone)]
pub struct SCCPResult {
    /// Final lattice value for each SSA value.
    pub lattice_values: HashMap<ValueId, LatticeValue>,
    
    /// Set of basic blocks proven reachable from entry.
    pub reachable_blocks: HashSet<NodeIndex>,
    
    /// Set of executable CFG edges (predecessor → successor).
    pub executable_edges: HashSet<(NodeIndex, NodeIndex)>,
    
    /// Number of instructions that can be folded based on analysis.
    pub foldable_count: usize,
    
    /// Number of branches that can be resolved.
    pub resolvable_branches: usize,
}
```

**Contract**:
- **Invariant 1**: Entry block always in `reachable_blocks`
- **Invariant 2**: If `(A, B)` in `executable_edges`, then `B` in `reachable_blocks`
- **Invariant 3**: Lattice values respect monotonicity (never increase in lattice)
- **Invariant 4**: All ValueIds in `lattice_values` are valid references in `function`

---

### SCCPError

```rust
#[derive(Debug, thiserror::Error)]
pub enum SCCPError {
    #[error("SCCP lattice memory limit (100KB) exceeded for function {function_name}")]
    MemoryLimit { function_name: String },
    
    #[error("Invalid CFG: missing successor block {block_id}")]
    InvalidCFG { block_id: NodeIndex },
    
    #[error("Invalid SSA reference: value {value_id:?} not found")]
    InvalidSSA { value_id: ValueId },
}
```

---

## Worklist Algorithm

### Initialization Phase

```rust
/// Initializes SCCP context for a function.
/// 
/// # Postconditions
/// - Entry block marked as reachable
/// - All lattice values initialized to Top
/// - Entry block instructions added to worklist
fn initialize_sccp(function: &Function) -> SCCPContext;
```

**Behavior**:
1. Create lattice map with capacity based on function value count
2. Create reachable blocks set, initially containing only entry block
3. Create executable edges set, initially empty
4. Create worklists (both empty initially)
5. Add entry block's instructions to value worklist
6. Return initialized context

**Complexity**: O(1) amortized (pre-allocation dominates)

---

### Worklist Processing

```rust
/// Processes worklists until fixed point (both worklists empty).
/// 
/// # Algorithm
/// Loop until both worklists empty:
///   1. Drain CFG edge worklist (discovers new blocks)
///   2. Drain SSA value worklist (propagates constants)
///   3. Check memory limit after each iteration
/// 
/// # Returns
/// Ok(()) if fixed point reached, Err if memory limit exceeded
fn process_worklists(context: &mut SCCPContext, function: &Function) -> Result<(), SCCPError>;
```

**Detailed Algorithm**:

#### Step 1: Process CFG Edges

```rust
while let Some((pred, succ)) = context.edge_worklist.pop_front() {
    // Mark edge executable
    if context.executable_edges.insert((pred, succ)) {
        // First time reaching this edge
        
        if context.reachable_blocks.insert(succ) {
            // First time reaching successor block: add all instructions to worklist
            for instruction in function.block(succ).instructions() {
                if let Some(result_value) = instruction.result_value() {
                    context.worklist.push_back(result_value);
                }
            }
        }
        
        // Re-evaluate phi nodes in successor (new incoming edge)
        for phi in function.block(succ).phi_nodes() {
            context.worklist.push_back(phi.result_value());
        }
    }
}
```

**Complexity**: O(e) where e = number of CFG edges (each edge processed once)

#### Step 2: Process SSA Values

```rust
while let Some(value_id) = context.worklist.pop_front() {
    // Compute new lattice value for this SSA value
    let new_lattice = evaluate_value(value_id, function, &context);
    
    // Get old lattice value (default to Top)
    let old_lattice = context.lattice_values
        .get(&value_id)
        .cloned()
        .unwrap_or(LatticeValue::Top);
    
    // Check if lattice value changed
    if new_lattice != old_lattice {
        // Update lattice map
        context.lattice_values.insert(value_id, new_lattice.clone());
        
        // Propagate change to all uses of this value
        for use_instruction in function.uses_of(value_id) {
            if let Some(result) = use_instruction.result_value() {
                context.worklist.push_back(result);
            }
        }
        
        // Special handling for conditional branches
        if let Some(branch) = get_conditional_branch(value_id, function) {
            if let LatticeValue::Constant(condition) = new_lattice {
                // Branch condition is constant: mark one successor executable
                let target_block = if condition.as_bool() {
                    branch.true_successor
                } else {
                    branch.false_successor
                };
                
                let parent_block = function.instruction_block(value_id);
                context.edge_worklist.push_back((parent_block, target_block));
            } else if matches!(new_lattice, LatticeValue::Bottom) {
                // Branch condition is non-constant: both successors executable
                let parent_block = function.instruction_block(value_id);
                context.edge_worklist.push_back((parent_block, branch.true_successor));
                context.edge_worklist.push_back((parent_block, branch.false_successor));
            }
        }
    }
}
```

**Complexity**: O(n) where n = number of SSA values (each value processed at most once per lattice level, 3 levels max)

---

### Value Evaluation

```rust
/// Evaluates the lattice value for a specific SSA value based on its defining instruction.
/// 
/// # Arguments
/// * `value_id` - The SSA value to evaluate
/// * `function` - The function containing the value
/// * `context` - Current SCCP analysis context
/// 
/// # Returns
/// The computed lattice value (Top, Constant, or Bottom)
fn evaluate_value(
    value_id: ValueId,
    function: &Function,
    context: &SCCPContext
) -> LatticeValue;
```

**Behavior by Instruction Type**:

1. **Constant Instructions**: Return `Constant(literal_value)`
2. **Phi Nodes**: Merge incoming values from executable predecessors
3. **Binary Operations**: Fold if both operands are Constant, else meet operands
4. **Unary Operations**: Fold if operand is Constant, else propagate operand
5. **Load Instructions**: Return Bottom (conservative, may be improved with alias analysis)
6. **Function Calls**: Return Bottom (side effects, unknown return value)
7. **Unknown Instructions**: Return Bottom (conservative)

**Phi Node Handling**:
```rust
fn evaluate_phi(
    phi: &PhiNode,
    context: &SCCPContext
) -> LatticeValue {
    let mut result = LatticeValue::Top;
    
    for (value, predecessor) in phi.incoming_values() {
        // Only consider incoming edges that are executable
        let edge = (predecessor, phi.parent_block());
        if context.executable_edges.contains(&edge) {
            let incoming_lattice = context.lattice_values
                .get(&value)
                .cloned()
                .unwrap_or(LatticeValue::Top);
            
            result = result.meet(&incoming_lattice);
            
            // Early exit if Bottom (absorbing element)
            if matches!(result, LatticeValue::Bottom) {
                return result;
            }
        }
    }
    
    result
}
```

**Complexity**: O(k) where k = number of operands/incoming values (bounded by small constant)

---

### Memory Limit Checking

```rust
/// Checks if the lattice map exceeds the 100KB memory limit.
/// 
/// # Returns
/// true if limit exceeded, false otherwise
fn check_memory_limit(context: &SCCPContext) -> bool {
    const LATTICE_ENTRY_SIZE: usize = 24; // Conservative estimate
    const MAX_BYTES: usize = 100_000;
    
    let estimated_bytes = context.lattice_values.len() * LATTICE_ENTRY_SIZE;
    estimated_bytes > MAX_BYTES
}
```

**Enforcement**:
- Called after each iteration of worklist processing
- If limit exceeded, immediately return `Err(SCCPError::MemoryLimit)`
- Caller should fall back to basic constant folding

---

## Transformation Application

```rust
/// Applies SCCP analysis results to transform the function.
/// 
/// # Arguments
/// * `function` - Function to transform (mutated in-place)
/// * `result` - SCCP analysis results
/// 
/// # Returns
/// Number of instructions transformed
/// 
/// # Behavior
/// 1. Replace instructions with Constant lattice values
/// 2. Resolve conditional branches with constant conditions
/// 3. Mark unreachable blocks for removal (actual removal deferred to CFG cleanup)
pub fn apply_sccp_transformations(
    function: &mut Function,
    result: &SCCPResult
) -> usize;
```

**Transformation Patterns**:

1. **Constant Replacement**:
   ```rust
   // Before: %3 = add i32 %1, %2  (where %1=5, %2=3 are constant)
   // After:  %3 = const i32 8
   if let LatticeValue::Constant(value) = lattice_values.get(&instruction.result()) {
       replace_with_constant(instruction, value);
       transformed_count += 1;
   }
   ```

2. **Branch Resolution**:
   ```rust
   // Before: br i1 %cond, label %true_bb, label %false_bb  (where %cond is constant true)
   // After:  br label %true_bb
   if let Instruction::CondBr { condition, true_target, false_target } = instruction {
       if let LatticeValue::Constant(cond_value) = lattice_values.get(condition) {
           let target = if cond_value.as_bool() { true_target } else { false_target };
           replace_with_unconditional_branch(instruction, target);
           transformed_count += 1;
       }
   }
   ```

3. **Phi Simplification**:
   ```rust
   // Phi nodes with single incoming value from reachable blocks
   if phi.incoming_values().all(|(val, _)| lattice_values.get(val) == Some(&constant)) {
       replace_phi_with_constant(phi, constant);
       transformed_count += 1;
   }
   ```

**Complexity**: O(n) where n = number of instructions in function

---

## Usage Example

```rust
use jsavrs::ir::optimizer::constant_folding::worklist::{sccp_analysis, apply_sccp_transformations};

fn optimize_with_sccp(function: &mut Function, verbose: bool) -> Result<usize, SCCPError> {
    // Perform SCCP analysis
    let sccp_result = sccp_analysis(function, verbose)?;
    
    if verbose {
        eprintln!("SCCP found {} foldable instructions, {} resolvable branches",
                  sccp_result.foldable_count,
                  sccp_result.resolvable_branches);
    }
    
    // Apply transformations
    let transformed = apply_sccp_transformations(function, &sccp_result);
    
    Ok(transformed)
}
```

---

## Performance Characteristics

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| Initialization | O(1) | O(n) | Pre-allocate maps |
| Edge processing | O(e) | O(e) | Each edge once |
| Value processing | O(n) | O(n) | Each value ≤3 times |
| Phi evaluation | O(k) | O(1) | k = incoming values |
| Transformation | O(n) | O(1) | Single pass |
| **Total** | **O(n + e)** | **O(n + e)** | Bounded to 100KB |

Where:
- n = number of SSA values
- e = number of CFG edges
- k = number of phi incoming values (typically ≤ 3)

---

## Error Handling Contract

### Memory Limit Exceeded

**Scenario**: Lattice map grows beyond 100KB during analysis

**Behavior**:
1. Return `Err(SCCPError::MemoryLimit { function_name })`
2. Caller emits warning to stderr
3. Caller falls back to basic constant folding
4. No partial results returned (all-or-nothing)

**Example**:
```rust
match sccp_analysis(function, verbose) {
    Ok(result) => apply_sccp_transformations(function, &result),
    Err(SCCPError::MemoryLimit { function_name }) => {
        eprintln!("Warning: SCCP memory limit exceeded for {}, falling back", function_name);
        basic_constant_folding(function)
    }
}
```

---

## Validation and Testing

### Required Unit Tests

1. **Simple Constant Propagation**: Linear CFG, all constants fold
2. **Phi Node Merging**: Multiple incoming values, same constant
3. **Phi Node Bottom**: Multiple incoming values, different constants
4. **Branch Resolution**: Constant condition, one successor unreachable
5. **Loop Handling**: Backedge handling, fixed-point convergence
6. **Nested Branches**: Complex control flow, multiple levels
7. **Memory Limit**: Large function triggers fallback

### Invariant Validation

```rust
#[cfg(debug_assertions)]
fn validate_sccp_result(result: &SCCPResult, function: &Function) {
    // Entry block must be reachable
    assert!(result.reachable_blocks.contains(&function.entry_block()));
    
    // Executable edges imply reachable successors
    for (_, succ) in &result.executable_edges {
        assert!(result.reachable_blocks.contains(succ));
    }
    
    // All lattice values reference valid SSA values
    for value_id in result.lattice_values.keys() {
        assert!(function.contains_value(value_id));
    }
}
```

---

## Thread Safety

**Not thread-safe**: `SCCPContext` is mutable and function-local. For parallel optimization, run separate `sccp_analysis()` calls per function (no shared state).

---

## Summary

| Aspect | Specification |
|--------|--------------|
| Algorithm | Worklist-based SCCP with 3-value lattice |
| Time Complexity | O(n + e) |
| Space Complexity | O(n + e), bounded to 100KB lattice |
| Error Handling | Memory limit → fallback |
| Phi Nodes | Reachability-filtered merge |
| Branches | Constant conditions resolved |
| Thread Safety | Function-local, parallelizable across functions |
