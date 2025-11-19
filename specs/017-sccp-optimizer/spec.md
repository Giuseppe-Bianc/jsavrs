# Feature Specification: Sparse Conditional Constant Propagation (SCCP) Optimizer

**Feature Branch**: `017-sccp-optimizer`  
**Created**: 19 November 2025  
**Status**: Draft  
**Input**: User description: Build a Sparse Conditional Constant Propagation (SCCP) optimization phase for the jsavrs compiler that discovers and propagates constant values throughout the intermediate representation while simultaneously eliminating unreachable code. The optimizer should identify variables that always hold the same constant value across all program executions, replace them with those constants, and remove code that can never be executed due to constant conditions.
## Core Capabilities
### Constant Discovery and Propagation
The optimizer must track every variable in the program and determine whether it's always a specific constant value. When a variable like `x = 5 + 3` is encountered, the optimizer should recognize that x is always 8 and replace all uses of x with 8. This propagation should flow through the entire program, including through phi nodes at control flow merge points.
### Dead Code Elimination Through Branch Analysis
When the optimizer encounters conditional branches with constant conditions (like `if (true) { A } else { B }`), it should recognize that the false branch can never execute. The optimizer marks the unreachable branch B and all code that can only be reached through B as dead code. This happens automatically through the SCCP algorithm rather than as a separate pass.
### Smart Value Tracking with Three States
Each variable is tracked in one of three states:
- **Unknown** (⊤): Haven't analyzed this variable yet, don't know what value it holds
- **Constant**: Determined this variable always holds a specific constant (like 42 or true)
- **Variable** (⊥): This variable can have different values at runtime, can't optimize it
Variables start as Unknown and move toward either Constant or Variable as the analysis progresses. This one-way flow ensures the analysis terminates.
### Phi Node Constant Resolution
At points where control flow merges (like after an if-statement), phi nodes select values from different predecessors. The optimizer should recognize when all executable paths feed the same constant into a phi node and resolve it to that constant. If different paths provide different constants, the phi result becomes Variable.
### Efficient Sparse Analysis
Rather than repeatedly scanning the entire program, the optimizer should use worklists to process only the parts of the code that need analysis. When a value changes, only the instructions that use that value are re-examined. When a branch condition becomes constant, only the newly reachable blocks are processed.
## Key Behaviors
### Variable Value Analysis
For each assignment and computation, determine if the result is constant:
- Direct constants: `x = 42` → x is constant 42
- Constant arithmetic: `y = 5 + 3` → y is constant 8  
- Constant propagation: If `x = 42` and `y = x + 1`, then y is constant 43
- Unknown operands: If any operand is Unknown, result is Unknown
- Variable operands: If any operand is Variable, result is Variable
### Branch Condition Evaluation
When analyzing branches:
- Constant true condition → only the true branch is reachable, mark false branch dead
- Constant false condition → only the false branch is reachable, mark true branch dead
- Variable or Unknown condition → both branches are potentially reachable
### Merge Point Analysis
At phi nodes where paths join:
- If all incoming values from executed paths are the same constant → phi resolves to that constant
- If incoming values differ or include Variable → phi becomes Variable
- Ignore values from unexecuted paths (paths marked dead)
### Conservative Safety Rules
The optimizer must be conservative when uncertain:
- Function calls always return Variable results (no interprocedural analysis)
- Memory loads always return Variable (no alias analysis to prove values are constant)
- Division by zero produces Variable (runtime behavior)
- Overflow/underflow in arithmetic follows Rust's wrapping semantics
### Code Transformation
After analysis completes:
- Replace instructions that compute constants with the constant value
- Replace conditional branches with constant conditions with unconditional jumps
- Remove blocks that were never marked as executable
- Update phi nodes to remove inputs from dead predecessors
## Operational Flow
### Initialization
Start with all variables marked Unknown and all control flow edges marked non-executable. Add the entry block's outgoing edges to a worklist for processing.
### Iterative Analysis
Process worklists until both are empty:
1. When a value definition changes, add all uses of that value to the worklist
2. When a block becomes reachable, add its instructions and outgoing edges to worklists
3. Process phi nodes considering only executed predecessor edges
4. Evaluate operations when operands become known
5. Resolve branch targets when conditions become constant
### Convergence
The analysis terminates when:
- No more values change their lattice state (Unknown → Constant or Unknown/Constant → Variable)
- No more blocks become reachable
- Both worklists are empty
- Or a safety iteration limit is reached
### Application of Results
Walk through the optimized function:
- Replace constant-valued SSA temporaries with their constant
- Simplify branches with constant conditions
- Mark unreachable blocks for removal (actual removal happens in DCE phase)
## Integration Points
### With Existing IR
The optimizer works on functions represented as control flow graphs with basic blocks in SSA form. It reads instruction kinds (Binary, Unary, Load, Store, Call, etc.) and SSA values. It must understand the existing Value, IrType, IrLiteralValue, and instruction structures.
### With Dead Code Elimination
SCCP runs before DCE in the optimization pipeline. SCCP marks blocks as unreachable by not adding them to the reachable set, but doesn't remove them. The existing DCE phase then removes these blocks and cleans up unused instructions. These two phases should be run alternately until a fixed point is reached.
### With Phase Infrastructure
Implement the Phase trait that the optimizer module defines. The run method receives a mutable module and processes each function. The optimizer should respect the existing configuration system (verbose logging, enable/disable flags).
## Supported Operations
### Arithmetic Operations
Constant fold addition, subtraction, multiplication, division, and modulo for all integer types (i8 through u64) and floating-point types (f32, f64). Handle division by zero conservatively.
### Bitwise Operations  
Constant fold AND, OR, XOR, shift left, and shift right for integer types.
### Comparison Operations
Constant fold equality, inequality, less than, less than or equal, greater than, and greater than or equal for numeric types.
### Logical Operations
Constant fold boolean AND and OR operations.
### Unary Operations
Constant fold negation for numeric types and logical NOT for booleans.
### Type Support
Handle constant propagation for i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, and char types. Conservatively mark strings, arrays, and pointers as Variable.
## Quality Requirements
### Correctness
Never transform code incorrectly. If unsure whether a value is constant, mark it Variable. If unsure whether a block is reachable, mark it reachable. The optimization should be sound—what it claims is constant must actually be constant across all executions.
### Performance
The analysis should complete in time proportional to the number of SSA values plus control flow edges. Each SSA edge should be processed at most twice. Don't repeatedly scan the entire function.
### Observability
When verbose mode is enabled, log:
- How many constants were discovered
- How many branches were simplified
- How many blocks became unreachable
- The number of worklist iterations required
- Any conservative decisions made
### Maintainability
Organize the implementation into logical modules (lattice values, constant evaluation, worklist management, main algorithm). Separate concerns so each module has a clear responsibility. Document the algorithm and implementation choices.
### Testability
Make the implementation testable at multiple levels:
- Unit test lattice operations and constant folding in isolation
- Integration test full SCCP analysis on small functions
- Verify correctness by comparing optimized vs unoptimized execution results
- Test edge cases like infinite loops, unreachable entry blocks, division by zero
## Why This Matters
SCCP is more powerful than running constant propagation and dead code elimination separately because it analyzes them together. By tracking which branches are actually taken, it can prove that values are constant along the executed paths even if they would be variable along dead paths. This discovers more optimization opportunities and results in smaller, faster compiled code. The Wegman-Zadeck algorithm accomplishes this efficiently using sparse dataflow analysis on SSA form, making it practical for real compilers.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Constant Expression Simplification (Priority: P1)

As a compiler user, when I write code with compile-time determinable values, the compiler should automatically simplify those expressions to their constant results, reducing executable size and improving runtime performance.

**Why this priority**: This is the core value proposition of SCCP - identifying and replacing values that are always constant. It's the foundation that enables all other optimizations.

**Independent Test**: Can be fully tested by compiling a program with constant arithmetic expressions (e.g., `x = 5 + 3; y = x * 2;`) and verifying the optimized IR replaces variable uses with constants (8 and 16).

**Acceptance Scenarios**:

1. **Given** a program with direct constant assignment `x = 42`, **When** SCCP analyzes the function, **Then** all uses of `x` are identified as constant value 42
2. **Given** a program with constant arithmetic `y = 5 + 3`, **When** SCCP evaluates the expression, **Then** `y` is determined to be constant 8
3. **Given** a program with chained constant propagation `x = 42; y = x + 1; z = y * 2`, **When** SCCP processes the chain, **Then** `x` becomes 42, `y` becomes 43, and `z` becomes 86
4. **Given** a program with all integer types (i8, i16, i32, i64, u8, u16, u32, u64), **When** SCCP performs constant folding, **Then** arithmetic operations on each type are correctly evaluated
5. **Given** a program with floating-point constants (f32, f64), **When** SCCP evaluates floating-point arithmetic, **Then** results are correctly computed with appropriate precision
6. **Given** a program with boolean constant expressions, **When** SCCP evaluates logical operations, **Then** boolean results are correctly determined

---

### User Story 2 - Unreachable Code Detection Through Constant Conditions (Priority: P1)

As a compiler user, when I have conditional branches with compile-time determinable conditions, the compiler should recognize which paths can never execute and mark them as unreachable, allowing dead code elimination to remove them.

**Why this priority**: This is the "conditional" and "sparse" part of SCCP - analyzing control flow based on constant conditions. Combined with P1, it forms the complete core algorithm.

**Independent Test**: Can be fully tested by compiling a program with constant conditions (e.g., `if (true) { A } else { B }`) and verifying the false branch is marked unreachable in the optimized IR.

**Acceptance Scenarios**:

1. **Given** a conditional branch with constant true condition `if (true) { A } else { B }`, **When** SCCP analyzes the branch, **Then** only branch A is marked reachable and branch B is marked unreachable
2. **Given** a conditional branch with constant false condition `if (false) { A } else { B }`, **When** SCCP analyzes the branch, **Then** only branch B is marked reachable and branch A is marked unreachable
3. **Given** nested conditionals where outer condition is constant true, **When** SCCP processes the control flow, **Then** only the true branch and its nested code are marked reachable
4. **Given** a loop with constant false entry condition, **When** SCCP analyzes the loop, **Then** the loop body is marked unreachable
5. **Given** multiple paths that merge at a common point, **When** SCCP determines some paths are unreachable, **Then** only values from reachable paths are considered at merge points

---

### User Story 3 - Phi Node Constant Resolution (Priority: P2)

As a compiler user, when control flow paths merge and all executable paths provide the same constant value, the compiler should recognize that the merged value is also constant, enabling further propagation.

**Why this priority**: This extends constant propagation across control flow merges, discovering additional optimization opportunities that wouldn't be found by analyzing basic blocks in isolation.

**Independent Test**: Can be fully tested by compiling a program with a phi node where both incoming values are the same constant (e.g., `x = condition ? 5 : 5`) and verifying the phi result is identified as constant 5.

**Acceptance Scenarios**:

1. **Given** a phi node with all incoming values being the same constant, **When** SCCP analyzes the phi, **Then** the phi result is determined to be that constant
2. **Given** a phi node with different constant values from different paths, **When** SCCP analyzes the phi, **Then** the phi result is marked as variable (non-constant)
3. **Given** a phi node where some predecessor edges are from unreachable blocks, **When** SCCP processes the phi, **Then** only values from reachable predecessors are considered
4. **Given** a phi node where all reachable predecessors provide the same constant but unreachable predecessors differ, **When** SCCP analyzes the phi, **Then** the phi result is the constant from reachable paths
5. **Given** a phi node in a loop where the value becomes constant after several iterations, **When** SCCP iterates to fixed point, **Then** the phi eventually resolves to constant if all paths converge

---

### User Story 4 - Bitwise and Comparison Operation Folding (Priority: P3)

As a compiler user, when I write code with bitwise operations or comparisons on constant values, the compiler should evaluate these at compile time, similar to arithmetic operations.

**Why this priority**: This extends constant folding to additional operation types beyond basic arithmetic. It's valuable but not essential for the core SCCP algorithm.

**Independent Test**: Can be fully tested by compiling programs with constant bitwise operations (e.g., `x = 0xFF & 0x0F`) and comparisons (e.g., `b = 5 > 3`) and verifying the optimized IR contains the constant results.

**Acceptance Scenarios**:

1. **Given** bitwise AND, OR, XOR operations on constant integers, **When** SCCP evaluates them, **Then** results are correctly computed
2. **Given** shift operations (left and right) on constant integers, **When** SCCP evaluates them, **Then** shift results are correctly computed
3. **Given** comparison operations (==, !=, <, <=, >, >=) on constant values, **When** SCCP evaluates them, **Then** boolean results are correctly determined
4. **Given** unary operations (negation, logical NOT) on constants, **When** SCCP evaluates them, **Then** results are correctly computed
5. **Given** complex expressions combining arithmetic, bitwise, and comparison operations, **When** SCCP evaluates the expression tree, **Then** the final constant result is correctly determined

---

### User Story 5 - Conservative Analysis for Uncertain Operations (Priority: P2)

As a compiler maintainer, when the optimizer encounters operations whose results cannot be determined at compile time, it should conservatively mark them as variable to ensure correctness.

**Why this priority**: Correctness is paramount in compiler optimization. This ensures the optimizer never produces incorrect code by making unsafe assumptions.

**Independent Test**: Can be fully tested by compiling programs with function calls, memory loads, and division by zero, and verifying these operations are conservatively marked as producing variable (non-constant) results.

**Acceptance Scenarios**:

1. **Given** a function call instruction, **When** SCCP analyzes it, **Then** the return value is marked as variable regardless of the function
2. **Given** a memory load instruction, **When** SCCP analyzes it, **Then** the loaded value is marked as variable
3. **Given** a division operation with zero denominator, **When** SCCP evaluates it, **Then** the result is conservatively marked as variable
4. **Given** operations on string, array, or pointer types, **When** SCCP encounters them, **Then** they are conservatively marked as variable
5. **Given** a conditional branch with non-constant condition, **When** SCCP analyzes the branch, **Then** both branches are marked as potentially reachable
6. **Given** integer overflow or underflow in constant arithmetic, **When** SCCP evaluates the operation, **Then** wrapping semantics are correctly applied according to Rust conventions

---

### Edge Cases

- What happens when a function contains an infinite loop with no exit? (The loop header becomes reachable, but SCCP should still terminate via iteration limit or worklist exhaustion)
- What happens when the entry block of a function is somehow marked unreachable? (This should not occur in valid IR, but if it does, the optimizer should conservatively mark it reachable)
- What happens when a phi node has zero executable predecessors? (The phi result remains Unknown initially, then becomes Variable if no reachable path provides a value)
- What happens when constant folding produces a value that changes the control flow graph structure? (The new edges are added to the worklist and analysis continues until fixed point)
- What happens when the worklist iterations exceed a safety limit? (Analysis terminates and all remaining Unknown values become Variable conservatively)
- What happens with recursive functions or mutually recursive call chains? (Function calls conservatively return Variable, preventing unsound interprocedural constant propagation)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The optimizer MUST track each SSA value in one of three lattice states: Unknown (⊤), Constant (specific value), or Variable (⊥)
- **FR-002**: The optimizer MUST initialize all SSA values to Unknown and all control flow edges to non-executable at the start of analysis
- **FR-003**: The optimizer MUST use worklist-based processing to analyze only affected parts of the code when values change
- **FR-004**: The optimizer MUST correctly evaluate constant folding for all binary arithmetic operations (add, subtract, multiply, divide, modulo) on integer types (i8, i16, i32, i64, u8, u16, u32, u64) and floating-point types (f32, f64)
- **FR-005**: The optimizer MUST correctly evaluate constant folding for bitwise operations (AND, OR, XOR, shift left, shift right) on integer types
- **FR-006**: The optimizer MUST correctly evaluate constant folding for comparison operations (==, !=, <, <=, >, >=) on numeric types
- **FR-007**: The optimizer MUST correctly evaluate constant folding for logical operations (AND, OR) on boolean values
- **FR-008**: The optimizer MUST correctly evaluate constant folding for unary operations (negation for numeric types, logical NOT for booleans)
- **FR-009**: The optimizer MUST handle char and bool constant values in addition to numeric types
- **FR-010**: The optimizer MUST conservatively mark function call results as Variable (no interprocedural analysis)
- **FR-011**: The optimizer MUST conservatively mark memory load results as Variable (no alias analysis)
- **FR-012**: The optimizer MUST conservatively mark division by zero results as Variable
- **FR-013**: The optimizer MUST conservatively mark string, array, and pointer values as Variable
- **FR-014**: The optimizer MUST apply Rust wrapping semantics for integer overflow and underflow in constant arithmetic
- **FR-015**: The optimizer MUST analyze conditional branches and mark only executable successors as reachable
- **FR-016**: The optimizer MUST recognize when a branch condition is constant true and mark only the true successor as reachable
- **FR-017**: The optimizer MUST recognize when a branch condition is constant false and mark only the false successor as reachable
- **FR-018**: The optimizer MUST mark both successors as potentially reachable when branch condition is Unknown or Variable
- **FR-019**: The optimizer MUST analyze phi nodes by considering only values from executable predecessor edges
- **FR-020**: The optimizer MUST resolve a phi node to a constant when all executable predecessors provide the same constant value
- **FR-021**: The optimizer MUST mark a phi node as Variable when executable predecessors provide different values (constant or variable)
- **FR-022**: The optimizer MUST ignore phi inputs from non-executable predecessor edges
- **FR-023**: The optimizer MUST continue analysis until both worklists are empty or iteration limit is reached
- **FR-024**: The optimizer MUST transform the IR by replacing constant-valued SSA temporaries with their constant values
- **FR-025**: The optimizer MUST transform conditional branches with constant conditions into unconditional jumps
- **FR-026**: The optimizer MUST mark unreachable blocks (not in the reachable set) for later removal by DCE
- **FR-027**: The optimizer MUST update phi nodes to remove inputs from dead predecessors during transformation
- **FR-028**: The optimizer MUST maintain correctness by never claiming a value is constant unless it is provably constant across all executions
- **FR-029**: The optimizer MUST integrate with the existing Phase trait infrastructure
- **FR-030**: The optimizer MUST process each function in the module independently
- **FR-031**: The optimizer MUST respect verbose logging configuration to output optimization statistics
- **FR-032**: The optimizer MUST respect enable/disable configuration flags

### Key Entities *(include if feature involves data)*

- **Lattice Value**: Represents the abstract state of an SSA value - Unknown (⊤, not yet analyzed), Constant (holds specific compile-time value), or Variable (⊥, runtime-determined value). The lattice ordering ensures monotonic convergence: Unknown → Constant or Unknown → Variable, and Constant → Variable, but never upward in the lattice.
  
- **SSA Value**: An intermediate representation value in static single assignment form. Each SSA value is assigned exactly once and tracked by the optimizer with a corresponding lattice value. SSA values include instruction results, function parameters, and phi node outputs.

- **Basic Block**: A sequence of instructions with a single entry point and single exit point. SCCP tracks whether each basic block is reachable (executable) based on which control flow edges are determined to be executable during analysis.

- **Control Flow Edge**: A directed edge in the control flow graph from one basic block to another, representing a possible execution path. Edges start as non-executable and become executable when the optimizer determines the source block is reachable and the branch condition (if any) permits flow to the target.

- **Phi Node**: An SSA construct at control flow merge points that selects a value based on which predecessor block was executed. SCCP analyzes phi nodes specially by considering only inputs from executable predecessor edges to determine if the phi result is constant.

- **Worklist**: A queue of work items requiring processing - either SSA values whose lattice state has changed or control flow edges that have become executable. The sparse analysis processes only items in worklists rather than scanning the entire program repeatedly.

- **Constant Value**: A compile-time determinable value of a specific type (integer, floating-point, boolean, character). The optimizer stores these concrete values when SSA values are determined to be constant, enabling constant folding and propagation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The optimizer successfully identifies and propagates at least 90% of compile-time constant values in standard test programs
- **SC-002**: The optimizer correctly marks unreachable code paths when branch conditions are compile-time constant, enabling DCE to remove them
- **SC-003**: The analysis completes in time proportional to the number of SSA values plus control flow edges (each edge processed at most twice)
- **SC-004**: The optimizer produces IR that, when compiled to assembly, results in measurably smaller executable size for constant-heavy programs (at least 10% reduction in typical cases)
- **SC-005**: The optimizer produces IR that, when compiled and executed, generates identical runtime behavior to unoptimized code for all test cases
- **SC-006**: Integration tests demonstrate that alternating SCCP and DCE passes reach a fixed point within 3 iterations for typical programs
- **SC-007**: When verbose logging is enabled, optimization statistics are reported including: number of constants discovered, branches simplified, blocks marked unreachable, and iteration count
- **SC-008**: The optimizer handles all supported numeric types (i8-u64, f32, f64), boolean, and char types correctly with zero type-related failures in test suite
- **SC-009**: The optimizer conservatively marks uncertain operations (function calls, memory loads, division by zero) as variable with zero soundness violations
- **SC-010**: Performance benchmarks show SCCP analysis completes in under 100ms for functions with 10,000 instructions on standard development hardware

### Assumptions

- The input IR is in valid SSA form with proper dominance properties
- The existing IR data structures (Value, IrType, IrLiteralValue, Instruction, BasicBlock, Function) provide sufficient information for analysis
- The existing DCE phase is available and will be run after SCCP to actually remove unreachable blocks
- The Phase trait infrastructure supports receiving and modifying IR modules
- Verbose logging and configuration systems are already established in the compiler
- The compiler follows Rust's wrapping semantics for integer arithmetic overflow
- No alias analysis or interprocedural analysis capabilities are available (conservative assumptions required)
- The compiler targets a single-threaded execution model (no concurrent constant propagation considerations)
- Test infrastructure supports both snapshot testing and execution-based validation
