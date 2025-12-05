# Feature Specification: Constant Folding Optimizer with Sparse Conditional Constant Propagation

**Feature Branch**: `020-sccp-constant-folding`  
**Created**: 2025-12-05  
**Status**: Draft  
**Input**: User description: "Constant Folding Optimizer with Sparse Conditional Constant Propagation
## Core Objectives
Build a production-ready constant folding optimization phase that implements the Wegman-Zadeck Sparse Conditional Constant Propagation (SCCP) algorithm for the jsavrs compiler's intermediate representation. The optimizer must integrate seamlessly with the existing optimization pipeline and Dead Code Elimination phase while maintaining the SSA form invariants of the IR.
## Fundamental Requirements
### 1. Lattice-Based Value Analysis
- Implement a three-level lattice system for tracking value states during propagation:
  - **Bottom** (⊥): Represents uninitialized or unreachable values
  - **Constant**: Represents values proven to be compile-time constants
  - **Top** (⊤): Represents values that may vary at runtime (overdefined)
- The lattice must support proper meet operations ensuring monotonic progression from Bottom → Constant → Top
- Track lattice values for all SSA temporaries, local variables, and memory locations
### 2. Sparse Conditional Constant Propagation Algorithm
- Implement the Wegman-Zadeck SCCP algorithm with two key worklists:
  - **SSA Edge Worklist**: Tracks data flow dependencies requiring reprocessing
  - **CFG Edge Worklist**: Tracks control flow edges that become executable
- Process phi nodes correctly by computing the meet of all executable predecessor values
- Mark CFG edges as executable only when control flow definitively reaches them
- Propagate constants through the SSA def-use chains efficiently
### 3. Constant Evaluation Engine
- Evaluate binary operations on constant operands at compile time
- Evaluate unary operations on constant operands at compile time
- Handle type-specific constant folding (integer arithmetic, floating-point, boolean logic)
- Properly handle edge cases: division by zero, integer overflow, NaN propagation
- Support bitwise operations with proper semantics
### 4. Conditional Branch Resolution
- Identify branches with constant conditions that can be resolved at compile time
- Mark unreachable CFG edges when branch conditions are proven constant
- Update the executable edge set to reflect dead control flow paths
- Maintain soundness by conservative analysis when conditions cannot be proven
### 5. IR Transformation and Rewriting
- Replace instructions that compute constant values with direct constant assignments
- Simplify phi nodes when all incoming values from executable edges are constant
- Remove or mark unreachable basic blocks identified during SCCP
- Preserve SSA form throughout all transformations
- Maintain proper def-use chain integrity after constant propagation
### 6. Integration with Existing Infrastructure
- Implement the `Phase` trait with proper `run()` method signature
- Coordinate with the Dead Code Elimination phase to remove instructions made dead by constant propagation
- Respect the module's `count_instructions()` interface for statistics tracking
- Use the existing `ControlFlowGraph`, `DominanceInfo`, and SSA infrastructure
- Work correctly with the existing scope management system
### 7. Modular Architecture
- Split implementation across logically separated files within `src/ir/optimizer/constant_folding/`:
  - **lattice.rs**: Lattice value representation and meet operations
  - **evaluator.rs**: Constant expression evaluation logic
  - **propagator.rs**: SCCP worklist algorithm implementation
  - **rewriter.rs**: IR transformation and simplification logic
  - **optimizer.rs**: Phase trait implementation and orchestration
- Each module should have clear, well-defined responsibilities
- Avoid monolithic functions exceeding 150 lines
### 8. Correctness and Safety Guarantees
- Never violate SSA form invariants during transformation
- Maintain sound analysis—never assume a value is constant unless proven
- Preserve observable program semantics (no behavior changes for non-constant computations)
- Handle phi nodes correctly in the presence of unreachable predecessors
- Correctly propagate undefined/uninitialized values as Bottom in the lattice
### 9. Performance Characteristics
- Achieve sparse analysis by processing only reachable code and live values
- Use efficient data structures (HashMap for lattice values, HashSet for executable edges)
- Minimize CFG and instruction traversals through incremental worklist processing
- Preallocate collections with estimated capacities based on IR size
- Converge in a bounded number of iterations (typically 1-3 passes for most functions)
### 10. Diagnostic and Debugging Support
- Provide optional verbose output controlled by configuration flags
- Track optimization statistics: constants propagated, instructions simplified, branches resolved
- Support the existing `OptimizationStats` pattern used by DCE
- Emit warnings for numerical edge cases (overflow, division by zero) when detected
- Include clear comments explaining lattice state transitions and algorithm invariants
## Rationale
Constant folding with SCCP is a foundational optimization that enables numerous downstream transformations. By proving values constant at compile time, we enable:
- Dead code elimination of conditional branches with known outcomes
- Simplification of arithmetic that depends only on constants
- Better register allocation through reduced live ranges
- Improved code size and runtime performance
The Wegman-Zadeck algorithm is specifically chosen because it operates in a single unified pass, simultaneously discovering constant values and unreachable code through its sparse lattice-based analysis. This is more efficient than naive iterative dataflow analysis and naturally integrates with SSA form.
The modular architecture ensures maintainability and testability. Each component can be developed, tested, and reasoned about independently, while the orchestrator in `optimizer.rs` provides a clean interface to the rest of the compiler pipeline.
Integration with the existing DCE phase creates a powerful optimization sequence: SCCP identifies constants and unreachable paths, then DCE removes the provably dead code, which may expose additional constant folding opportunities in subsequent optimization rounds.
## Non-Requirements (Explicitly Out of Scope)
- Interprocedural constant propagation across function boundaries
- Symbolic execution or constraint solving beyond simple constant evaluation
- Optimization of memory operations (load/store forwarding)
- Alias analysis or pointer analysis
- Loop-aware optimizations or induction variable analysis
- Profile-guided optimization or speculation
- Generation of new LLVM-style intrinsics or target-specific instructions<>"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Constant Propagation and Folding (Priority: P1)

A compiler developer compiles source code containing variables assigned to constant literal values that are subsequently used in arithmetic or logical operations. The optimizer analyzes the SSA-form intermediate representation, identifies which values remain constant throughout their entire lifetime, and replaces computed expressions with their constant results.

**Why this priority**: This delivers the core value proposition of the SCCP algorithm - eliminating redundant runtime computations by evaluating constant expressions at compile time. This is the foundational capability that enables all other optimizations and provides immediate, measurable improvements in code performance and size.

**Independent Test**: Can be fully tested by compiling a function containing constant assignments and arithmetic operations (e.g., `let x = 5; let y = 10; let z = x + y; return z;`) and verifying that the optimizer replaces the addition operation with the constant value 15, demonstrating standalone constant evaluation without requiring control flow analysis.

**Acceptance Scenarios**:

1. **Given** an SSA function with a variable assigned a literal constant (e.g., `x = 42`), **When** the SCCP optimizer runs, **Then** all uses of that variable throughout the function are replaced with the literal constant value
2. **Given** an SSA function with binary arithmetic operations on two constant operands (e.g., `result = 10 + 5`), **When** the optimizer runs, **Then** the operation is evaluated at compile time and replaced with the constant result (15)
3. **Given** an SSA function with chained constant expressions (e.g., `a = 2; b = 3; c = a * b; d = c + 1`), **When** the optimizer runs, **Then** all intermediate computations are folded and the final value is replaced with the constant 7
4. **Given** an SSA function with phi nodes where all incoming values from all predecessors are the same constant, **When** the optimizer runs, **Then** the phi node is simplified to a direct assignment of that constant value

---

### User Story 2 - Conditional Branch Resolution and Dead Path Elimination (Priority: P2)

A compiler developer compiles source code with conditional branches (if-statements, switch-statements) where the branch condition can be proven to be constant at compile time. The optimizer determines which execution path is always taken, converts conditional branches to unconditional jumps, and marks unreachable code paths for elimination.

**Why this priority**: This extends basic constant propagation into control flow analysis, unlocking significant optimizations by eliminating entire code paths that can never execute. This reduces code size, improves cache locality, and enables further optimization opportunities in downstream compiler phases.

**Independent Test**: Can be tested independently by compiling a function with a condition based on constants (e.g., `if (true) { return 1; } else { return 2; }`) and verifying that the optimizer converts the conditional branch to an unconditional jump to the true branch, marks the false branch as unreachable, and ultimately produces code that only contains the true path.

**Acceptance Scenarios**:

1. **Given** a conditional branch where the condition evaluates to the constant value true, **When** the optimizer runs, **Then** the conditional branch is converted to an unconditional jump to the true successor block and the false successor is marked as unreachable
2. **Given** a conditional branch where the condition evaluates to the constant value false, **When** the optimizer runs, **Then** the conditional branch is converted to an unconditional jump to the false successor block and the true successor is marked as unreachable
3. **Given** a switch statement where the selector value is proven to be a specific constant, **When** the optimizer runs, **Then** only the matching case branch remains reachable and all other case branches are marked as unreachable
4. **Given** nested conditional branches where outer branch conditions are constant, **When** the optimizer runs, **Then** entire subtrees of the control flow graph are marked unreachable based on the proven-constant outer conditions

---

### User Story 3 - Phi Node Simplification in Control Flow (Priority: P3)

A compiler developer compiles source code that generates phi nodes in SSA form where some incoming edges come from unreachable basic blocks or where executable incoming edges all carry the same constant value. The optimizer identifies these simplified phi node scenarios and either removes the phi node entirely or replaces it with a constant value.

**Why this priority**: This complements the previous stories by cleaning up SSA artifacts after dead path elimination. While not as immediately impactful as basic constant folding or branch resolution, it ensures the IR remains in a clean, canonical form that facilitates further optimization passes and produces more efficient final code.

**Independent Test**: Can be tested by compiling a function with a phi node that merges values from multiple paths where some paths are proven unreachable (e.g., through constant branch conditions), and verifying that the phi node is simplified to only consider executable predecessor edges or is replaced entirely with a constant if all executable edges carry the same value.

**Acceptance Scenarios**:

1. **Given** a phi node with incoming edges from both executable and unreachable blocks, **When** the optimizer runs, **Then** the phi node is updated to only include incoming values from executable predecessor edges
2. **Given** a phi node where all remaining executable incoming edges carry the same constant value, **When** the optimizer runs, **Then** the phi node is replaced with a direct assignment of that constant value
3. **Given** a phi node in a block that becomes entirely unreachable due to constant branch resolution, **When** the optimizer runs, **Then** the phi node is marked as part of the unreachable code and scheduled for removal
4. **Given** a phi node with a mix of constant and non-constant incoming values from executable edges, **When** the optimizer runs, **Then** the phi node is preserved but its lattice value is set to Top (overdefined) reflecting the varying runtime values

---

### User Story 4 - Type-Safe Constant Evaluation Across Data Types (Priority: P2)

A compiler developer compiles source code containing constant expressions with various data types including signed integers, unsigned integers, floating-point numbers, booleans, and characters. The optimizer correctly evaluates each operation according to the semantic rules of the specific type, handling overflow, underflow, division by zero, and special floating-point values.

**Why this priority**: Type correctness is essential for soundness - incorrect constant evaluation could silently introduce bugs or undefined behavior. This ensures the optimizer maintains the same observable program semantics while optimizing, which is a fundamental requirement for any compiler optimization pass.

**Independent Test**: Can be tested independently by creating test functions for each data type (I8, I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Char) containing constant expressions specific to that type, and verifying that evaluation respects type-specific overflow behavior, precision, and special values (e.g., NaN propagation for floats).

**Acceptance Scenarios**:

1. **Given** constant integer arithmetic that would overflow the type's range (e.g., `I8::MAX + 1`), **When** the optimizer runs, **Then** the result is marked as overdefined (Bottom in lattice) rather than producing an incorrect wrapped value, maintaining conservative soundness
2. **Given** constant floating-point operations involving NaN operands, **When** the optimizer runs, **Then** the result correctly propagates NaN according to IEEE 754 semantics
3. **Given** constant division by zero in integer arithmetic, **When** the optimizer runs, **Then** the result is marked as overdefined and a diagnostic warning is emitted
4. **Given** constant boolean logic operations (AND, OR, NOT), **When** the optimizer runs, **Then** the operations are evaluated correctly following boolean algebra rules
5. **Given** constant character operations within the valid Unicode range, **When** the optimizer runs, **Then** character values are correctly propagated while maintaining Unicode validity

---

### Edge Cases

- **What happens when a phi node exists in the entry block?** The entry block has no predecessors by definition in SSA form, so a phi node in the entry block is invalid IR - the optimizer should detect this as a structural error and either reject the IR or treat it conservatively.

- **How does the system handle self-loops in the control flow graph?** Self-loops (a block that branches back to itself) are processed until the lattice values reach a fixed point. The worklist algorithm naturally handles this by re-enqueueing edges when values change, ensuring convergence even with cycles.

- **What happens when maximum iteration limits are reached without convergence?** If the worklist algorithm exceeds the configured maximum iteration count (default: 100 iterations per function), the optimizer emits a warning diagnostic and terminates with a conservative result, marking remaining uncertain values as overdefined to maintain soundness.

- **How does the optimizer handle undefined or uninitialized variable uses?** Uninitialized values are represented as Bottom (⊥) in the lattice. Any computation using a Bottom value produces Bottom, ensuring that undefined behavior is conservatively propagated rather than assumed to be any specific constant.

- **What happens when dead code contains malformed instructions?** Since unreachable blocks are marked but not immediately removed during SCCP (deferring to DCE phase), malformed instructions in dead code are ignored during constant propagation - they won't affect reachable code analysis.

- **How are function calls handled during constant propagation?** Function calls are treated conservatively - their return values are always marked as overdefined (Top) since we don't perform interprocedural analysis. This ensures soundness without requiring whole-program analysis.

- **What happens with memory operations (load/store)?** Load instructions have their results marked as overdefined unless advanced alias analysis (out of scope) proves the source is constant. Store instructions don't propagate values. This conservative treatment maintains correctness for any possible memory aliasing.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The optimizer MUST implement a three-level lattice system (Bottom ⊥, Constant, Top ⊤) to track the compile-time value state of every SSA value in the function, with Bottom representing unreachable/uninitialized values, Constant representing proven compile-time constants, and Top representing overdefined runtime-varying values.

- **FR-002**: The optimizer MUST maintain two distinct worklists during analysis: an SSA Edge Worklist tracking data flow dependencies when value states change, and a CFG Edge Worklist tracking control flow edges that become executable.

- **FR-003**: The optimizer MUST correctly compute the lattice meet operation for phi nodes by considering only incoming values from executable predecessor blocks, where the meet of two equal constants is that constant, the meet of two different constants is Bottom (overdefined), and the meet with Top preserves the other operand.

- **FR-004**: The optimizer MUST evaluate binary operations (addition, subtraction, multiplication, division, modulo, bitwise operations, comparisons) on constant operands at compile time, producing constant results when both operands are constant and properly handling error conditions (division by zero, overflow).

- **FR-005**: The optimizer MUST evaluate unary operations (negation, bitwise NOT, logical NOT, type casts) on constant operands at compile time, producing constant results when the operand is constant.

- **FR-006**: The optimizer MUST handle type-specific constant folding correctly for all supported IR types: I8, I16, I32, I64 (signed integers), U8, U16, U32, U64 (unsigned integers), F32, F64 (floating-point), Bool (boolean), Char (Unicode character), String (conservatively marked as non-constant).

- **FR-007**: The optimizer MUST detect when conditional branch conditions are proven to be constant values and mark the appropriate control flow edges as executable or unreachable accordingly (true constant marks true successor executable, false constant marks false successor executable).

- **FR-008**: The optimizer MUST preserve SSA form throughout all transformations, ensuring that every variable has exactly one static assignment point and that all uses are dominated by their definitions.

- **FR-009**: The optimizer MUST replace instructions computing constant values with direct constant assignments in the IR rewriting phase, maintaining proper def-use chain integrity.

- **FR-010**: The optimizer MUST simplify phi nodes when all executable incoming values are the same constant by replacing the phi node with a direct constant assignment.

- **FR-011**: The optimizer MUST mark basic blocks as unreachable when no executable control flow paths reach them, enabling subsequent dead code elimination.

- **FR-012**: The optimizer MUST integrate with the existing Phase trait by implementing the `run()` method that accepts a module and performs the complete SCCP analysis and IR transformation.

- **FR-013**: The optimizer MUST coordinate with the Dead Code Elimination (DCE) phase by either marking dead instructions for DCE to remove or directly removing them based on configuration.

- **FR-014**: The optimizer MUST use efficient sparse data structures (HashMap for lattice values, HashSet for executable edges) and process only reachable code and live values to achieve performance characteristics suitable for large functions.

- **FR-015**: The optimizer MUST converge to a fixed point in a bounded number of iterations, with a configurable maximum iteration limit (default 100) to prevent infinite loops on pathological input.

- **FR-016**: The optimizer MUST provide optional verbose diagnostic output controlled by configuration flags, including lattice state transitions and algorithm invariants for debugging purposes.

- **FR-017**: The optimizer MUST track optimization statistics including the number of constants propagated, instructions simplified, branches resolved, and blocks marked unreachable.

- **FR-018**: The optimizer MUST emit diagnostic warnings when detecting numerical edge cases such as overflow, division by zero, or NaN propagation during constant evaluation.

- **FR-019**: The optimizer MUST implement a modular architecture with clearly separated components for lattice value management, constant evaluation logic, propagation algorithm, IR rewriting, and orchestration, with no single component function exceeding 150 lines.

- **FR-020**: The optimizer MUST maintain conservative soundness by never assuming a value is constant unless proven through analysis, and marking uncertain values as overdefined (Top) rather than guessing.

### Key Entities

- **LatticeValue**: Represents the compile-time state of an SSA value with three possible states: Bottom (⊥) for unreachable/uninitialized values, Constant(value) for proven compile-time constants with the specific constant value, and Top (⊤) for overdefined runtime-varying values. Supports meet and join operations following lattice theory semantics.

- **SSA Edge**: Represents a data flow dependency from a value definition to an instruction that uses that value. When a value's lattice state changes (e.g., from Top to Constant), all outgoing SSA edges are added to the SSA worklist for reprocessing.

- **CFG Edge**: Represents a control flow transition from one basic block to another in the control flow graph. Edges are marked as executable when the SCCP algorithm proves that control flow can reach the destination block along that path.

- **ExecutableEdge Set**: A collection (HashSet) tracking which CFG edges have been proven executable during SCCP analysis. Initially empty except for edges from the entry block. Used to determine which phi node incoming values to consider and which blocks are reachable.

- **Worklist**: Abstract data structure (implemented as VecDeque) holding work items to process. SSA worklist contains value-to-use edges, CFG worklist contains block-to-block edges. Worklists drive the iterative sparse analysis algorithm.

- **BasicBlock**: A sequence of instructions with a single entry point and single exit point (terminator instruction). Blocks are marked as executable or unreachable based on whether any incoming CFG edge is in the executable set.

- **Phi Node**: SSA instruction that merges values from different control flow predecessors. During SCCP, phi nodes are evaluated by computing the lattice meet of all incoming values from executable predecessor edges only.

- **Terminator**: The final instruction in a basic block that determines control flow transitions (Branch, ConditionalBranch, Switch, Return). Terminators are evaluated during SCCP to determine which outgoing CFG edges should be marked executable.

- **Instruction**: A single operation in the IR (arithmetic, logical, memory access, control flow). Instructions are visited by SCCP to compute the lattice values of their result operands based on the lattice values of their input operands.

- **OptimizationStats**: A data structure tracking metrics about the optimization pass execution, including counts of constants found, branches eliminated, blocks removed, instructions replaced, and iterations required for convergence.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Constant arithmetic expressions in compiled code are evaluated at compile time, with the optimizer replacing 95% or more of compile-time-computable binary and unary operations with their constant results in representative test programs.

- **SC-002**: Conditional branches with constant conditions are resolved at compile time, with the optimizer converting 90% or more of such branches to unconditional jumps and marking unreachable paths in test cases specifically designed to contain constant conditions.

- **SC-003**: The optimizer converges to a fixed point within 3 iterations or fewer for 95% of real-world functions, demonstrating the efficiency of the sparse worklist algorithm on typical code patterns.

- **SC-004**: The optimizer processes functions with up to 10,000 instructions while maintaining linear time complexity characteristics, completing analysis and transformation in under 1 second per function on standard development hardware.

- **SC-005**: Integration with the Dead Code Elimination phase results in a combined optimization sequence that eliminates 80% or more of unreachable code in test programs containing constant-condition branches.

- **SC-006**: Type-specific constant evaluation maintains 100% correctness across all supported IR types (signed integers, unsigned integers, floating-point, boolean, character), with zero test failures in type-specific constant folding test suites.

- **SC-007**: SSA form integrity is preserved through all transformations, with 100% of optimized functions passing SSA verification checks including dominance relations and single-assignment invariants.

- **SC-008**: The optimizer produces measurable performance improvements in generated code, reducing executable code size by 5-15% and improving runtime performance by 2-8% in benchmark programs containing constant expressions and dead code paths.

- **SC-009**: Diagnostic warnings are correctly emitted for 100% of numerical edge cases (overflow, division by zero, NaN propagation) detected during constant evaluation in test programs specifically designed to trigger these conditions.

- **SC-010**: The modular architecture maintains clean separation of concerns, with each component (lattice value management, constant evaluation, propagation algorithm, IR rewriting, orchestration) independently testable and no function exceeding 150 lines in 95% of the implementation.

## Assumptions

- The input intermediate representation is in valid SSA form with proper dominance relationships and single assignment invariants already established by previous compiler phases.

- The existing IR infrastructure provides `ControlFlowGraph`, `DominanceInfo`, and def-use chain data structures that can be efficiently queried during SCCP analysis.

- The Dead Code Elimination phase is available in the optimization pipeline and can be sequenced after SCCP to remove unreachable code identified by constant propagation.

- The IR type system is sufficiently expressive to represent all constant values that can appear in the source language, including literals for integers, floats, booleans, and characters.

- The `Phase` trait interface is stable and provides the necessary hooks for integration into the optimization pipeline with module-level granularity.

- Performance characteristics assume functions with typical size distributions (majority under 1000 instructions, with some outliers up to 10,000 instructions) as found in real-world compiler workloads.

- The existing scope management system correctly tracks variable lifetimes and can be used to determine which values are in scope at each program point during constant propagation.

- Memory safety and overflow checking behavior for constant evaluation should match the semantics defined by the source language (conservative wrapping or trapping on overflow depending on configuration).

- The compiler infrastructure supports emitting diagnostic warnings during optimization passes, with appropriate integration into the overall diagnostic reporting system.

- Test infrastructure supports snapshot testing, unit testing, and integration testing patterns needed to validate SCCP correctness across diverse code patterns and edge cases.
