# Feature Specification: Sparse Conditional Constant Propagation Optimizer

**Feature Branch**: `016-sccp-optimizer`  
**Created**: 2025-11-17  
**Status**: Draft  
**Input**: User description: "Build a Sparse Conditional Constant Propagation optimizer implementing the Wegman-Zadeck algorithm for a Rust compiler IR in SSA form. Use a flat three-level lattice where each SSA value starts at Top (unknown/optimistic), descends to Constant(literal_value) when proven constant, or to Bottom (overdefined/varying) when multiple different values reach it, with monotonic descent only ensuring termination. Maintain two worklists: SSAWorkList containing (definition_value → use_instruction) edges triggered when a value's lattice state changes, and FlowWorkList containing (predecessor_block → successor_block) CFG edges triggered when new control paths become executable. Track executable_edges as a HashSet of CFG edge pairs and executable_blocks as a HashSet of block indices, both initially empty except entry block edges. Initialize all SSA temporaries, locals, and phi results to Top in a HashMap<Value, LatticeValue>, except global values and function parameters which start at Bottom conservatively. Process SSAWorkList by popping edges, checking if the use instruction's block is executable, then calling VisitInstruction; process FlowWorkList by popping edges, marking destination blocks executable if first visit, visiting all instructions in program order including phi nodes, and enqueueing the block's outgoing edges based on terminator evaluation. In VisitInstruction for Binary/Unary ops: if any operand is Bottom produce Bottom, if all operands are Constant evaluate the operation producing Constant or Bottom on overflow/undefined behavior, otherwise keep Top; for Phi nodes compute the meet of only incoming values from executable predecessor edges where meet(Constant(c), Constant(c))=Constant(c), meet(Constant(c1), Constant(c2))=Bottom if c1≠c2, meet(Top, x)=x, meet(Bottom, x)=Bottom, and update result if changed; for Load mark result Bottom unless advanced alias analysis proves the source constant; for Store/Call mark as Bottom and don't propagate; for Cast propagate constant through safe conversions or Bottom; for GetElementPtr compute constant offset if base and index constant otherwise Bottom. Evaluate terminators: for Branch enqueue target unconditionally; for ConditionalBranch evaluate condition lattice value enqueueing true_target if Constant(true), false_target if Constant(false), both targets if Top or Bottom; for Switch enqueue matching case if Constant selector or all cases if Top/Bottom; for Return/Unreachable enqueue nothing. After fixed-point when both worklists empty, rewrite IR by replacing instruction results that are Constant in lattice with literal values, converting ConditionalBranch with constant condition to unconditional Branch to taken successor, removing unreachable blocks not in executable_blocks set, updating phi nodes to remove incoming edges from non-executable predecessors, simplifying phi nodes with single incoming value to direct value assignment, and deleting instructions whose results are constant. Preserve SSA by ensuring each phi node removal or simplification maintains the single-assignment property, verify no value moves upward in lattice during analysis, verify CFG structure remains valid with entry block reachable and all branch targets existing, assert worklists empty at termination, handle edge cases like entry block without predecessors, phi nodes in entry block as invalid, self-loops by processing until stable, infinite loops by safety iteration limit, division by zero as Bottom, type mismatches as Bottom. Structure as lattice.rs defining LatticeValue enum with meet/join operations and per-type constant evaluation, worklist.rs managing VecDeque-based SSAWorkList and FlowWorkList with duplicate prevention via HashSet, evaluator.rs implementing abstract interpretation for each InstructionKind using match exhaustive patterns, branch_analysis.rs evaluating TerminatorKind conditions, executable_edges.rs tracking CFG edge reachability with efficient lookup, rewriter.rs performing IR mutations while maintaining SSA/CFG invariants, stats.rs collecting constants_found, branches_eliminated, blocks_removed, instructions_replaced, iterations_to_convergence. Expose configuration via verbose:bool, max_iterations:usize defaulting to 100 with convergence checking each iteration. Integrate by implementing Phase trait's run method orchestrating analysis then rewrite then verification, coordinating with existing DeadCodeElimination by marking dead instructions but allowing DCE to remove them or removing directly based on configuration. Ensure type-safety by matching IrType discriminants during constant evaluation, handle I8/I16/I32/I64/U8/U16/U32/U64 with checked arithmetic returning Bottom on overflow, F32/F64 with NaN/infinity checks, Bool with logical operations, Char with valid Unicode, String as always Bottom. Track each CFG edge processing count asserting each processed at most once, each SSA edge at most twice when source value changes or target block becomes executable, proving O(edges) complexity. Validate before optimization that function has entry block, all phi incoming edges reference existing predecessors, all branch targets exist; validate after optimization that SSA form preserved via existing verify_ssa_form, CFG valid via cfg.verify(), no Top values remain in lattice for executable code regions. Handle errors by returning Result<(), String> from transform_function propagating verification failures, logging warnings for max_iterations exceeded or unexpected patterns but continuing conservatively with Bottom, maintaining source_span information through rewrites for error reporting. Test with unit tests per module verifying lattice arithmetic correctness for all types and operations, integration tests on complete functions with known constant propagation opportunities, regression tests ensuring no changes when no constants present, and performance tests asserting linear scaling to 10000+ instruction functions."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Constant Propagation (Priority: P1)

A compiler developer compiles a program containing variables assigned to constant values that are used in subsequent computations. The optimizer analyzes the SSA-form intermediate representation and identifies which values are provably constant throughout their lifetime, replacing those constant computations with their literal values.

**Why this priority**: This is the core value proposition - identifying and propagating constants is the fundamental capability that delivers immediate compilation performance benefits and produces smaller, faster executables.

**Independent Test**: Can be fully tested by compiling a function with constant assignments (e.g., `x = 5; y = 10; z = x + y;`) and verifying the optimizer replaces `z` with the constant `15`, demonstrating standalone constant propagation without requiring conditional branch elimination.

**Acceptance Scenarios**:

1. **Given** an SSA function with a variable assigned a literal constant value, **When** the optimizer runs, **Then** all uses of that variable are replaced with the literal constant value
2. **Given** an SSA function with arithmetic operations on constant values, **When** the optimizer runs, **Then** the operation results are computed at compile-time and replaced with constant literals
3. **Given** an SSA function with phi nodes where all incoming values are the same constant, **When** the optimizer runs, **Then** the phi node result is replaced with that constant value

---

### User Story 2 - Conditional Branch Elimination (Priority: P2)

A compiler developer compiles a program containing conditional branches where the condition can be proven constant at compile-time. The optimizer determines which branch path is always taken, eliminates the unreachable path, and converts the conditional branch to an unconditional jump.

**Why this priority**: Builds upon P1 constant propagation to enable control-flow optimization, significantly reducing code size and improving runtime performance by eliminating dead code paths. This is a key differentiator of SCCP over simpler constant folding.

**Independent Test**: Can be tested independently by compiling a function with a condition based on constants (e.g., `if (true) { A } else { B }`) and verifying the optimizer removes the false branch and converts to unconditional control flow, delivering measurable code size reduction.

**Acceptance Scenarios**:

1. **Given** a conditional branch where the condition evaluates to a constant true value, **When** the optimizer runs, **Then** the branch is converted to an unconditional jump to the true successor and the false successor becomes unreachable
2. **Given** a conditional branch where the condition evaluates to a constant false value, **When** the optimizer runs, **Then** the branch is converted to an unconditional jump to the false successor and the true successor becomes unreachable
3. **Given** a switch statement where the selector is a known constant, **When** the optimizer runs, **Then** only the matching case branch remains reachable and all other cases are marked unreachable

---

### User Story 3 - Unreachable Code Elimination (Priority: P3)

A compiler developer compiles a program where certain basic blocks become unreachable after constant propagation determines control flow paths. The optimizer identifies blocks that are never executed and removes them from the control-flow graph, simplifying the program structure.

**Why this priority**: Complements P2 by cleaning up the results of branch elimination, further reducing code size and compilation time for subsequent passes. This is an expected outcome but lower priority since dead code elimination may handle it separately.

**Independent Test**: Can be tested by compiling a function where a branch always goes one direction (making other blocks unreachable) and verifying those unreachable blocks are removed from the final IR, demonstrating standalone reachability analysis and cleanup.

**Acceptance Scenarios**:

1. **Given** basic blocks that have no executable predecessor edges, **When** the optimizer completes its analysis, **Then** those blocks are marked as unreachable and removed from the control-flow graph
2. **Given** phi nodes with incoming edges from unreachable predecessor blocks, **When** the optimizer rewrites the IR, **Then** those incoming edges are removed from the phi nodes
3. **Given** a phi node where only one incoming edge remains after removing unreachable predecessors, **When** the optimizer simplifies the IR, **Then** the phi node is replaced with a direct assignment of the single incoming value

---

### User Story 4 - Type-Safe Constant Evaluation (Priority: P4)

A compiler developer compiles programs using various data types (integers, floats, booleans, characters). The optimizer evaluates constant operations while respecting type semantics, handling overflow, underflow, and undefined behaviors according to language rules, ensuring correctness across all supported types.

**Why this priority**: Essential for correctness and safety, but builds on top of P1-P3 infrastructure. Type safety is non-negotiable but can be implemented incrementally per type as the core algorithm stabilizes.

**Independent Test**: Can be tested by compiling functions with type-specific constant operations (integer overflow, floating-point NaN, boolean logic) and verifying the optimizer produces correct results or conservatively marks values as non-constant when undefined behavior would occur.

**Acceptance Scenarios**:

1. **Given** integer arithmetic that would overflow, **When** the optimizer evaluates the operation, **Then** the result is marked as non-constant (Bottom) rather than producing an incorrect wrapped value
2. **Given** floating-point operations producing NaN or infinity, **When** the optimizer evaluates them, **Then** special float values are handled correctly according to IEEE 754 semantics
3. **Given** operations between incompatible types, **When** the optimizer encounters them, **Then** the result is conservatively marked as non-constant
4. **Given** safe type conversions between constant values, **When** the optimizer processes casts, **Then** the constant is propagated with the new type


---

### Edge Cases

- What happens when a phi node has circular dependencies (self-loops or cycles in the SSA graph)?
  - The optimizer processes the phi node iteratively until the lattice value stabilizes, with a maximum iteration limit to prevent infinite loops
- How does the system handle division or modulo by zero?
  - Marked as Bottom (non-constant) to avoid incorrect optimizations (see FR-017)
- What if the entry block has no reachable successors (infinite loop at entry)?
  - The entry block is always marked executable; if it loops infinitely, the algorithm converges when all reachable paths are processed
- How are memory operations (Load/Store) handled when addresses may alias?
  - Conservative approach: Load results marked Bottom unless alias analysis proves the source is constant; Stores do not propagate constants through memory
- What if function parameters or global variables are encountered?
  - Marked as Bottom (non-constant) conservatively since their values are unknown at optimization time
- How does the system handle unreachable entry blocks or malformed CFGs?
  - Pre-optimization validation ensures the entry block exists and CFG is well-formed; malformed IR results in optimization failure
- What happens when the maximum iteration limit is reached before convergence?
  - Optimizer logs a warning, marks remaining uncertain values as Bottom conservatively, and completes with partial results
- How are phi nodes in the entry block handled?
  - Treated as invalid IR since entry blocks have no predecessors; validation detects this as an error
- What if a block has multiple incoming edges from the same predecessor?
  - Handled by processing each edge independently in the phi node's incoming value list
- How does the optimizer maintain source location information for error reporting?
  - Source spans are preserved through all IR rewrites to maintain accurate error messages

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Optimizer MUST implement a three-level lattice for each SSA value with states Top (unknown), Constant (known literal value), and Bottom (varying/multiple values)
- **FR-002**: Optimizer MUST initialize all SSA temporaries and locals to Top, and function parameters and globals to Bottom
- **FR-003**: Optimizer MUST maintain two work lists: SSAWorkList for value-to-use edges and FlowWorkList for control-flow edges
- **FR-004**: Optimizer MUST track executable CFG edges and blocks, starting with only entry block edges marked executable
- **FR-005**: Optimizer MUST process SSAWorkList by visiting instructions when their defining values' lattice states change
- **FR-006**: Optimizer MUST process FlowWorkList by marking blocks executable and evaluating their instructions and terminators
- **FR-007**: Optimizer MUST evaluate binary and unary operations by computing constants when all operands are Constant, producing Bottom when any operand is Bottom, or keeping Top otherwise
- **FR-008**: Optimizer MUST evaluate phi nodes by computing the meet (greatest lower bound) of incoming values from executable predecessor edges only
- **FR-009**: Optimizer MUST evaluate conditional branches by determining which successor blocks are reachable based on the condition's lattice value
- **FR-010**: Optimizer MUST handle unconditional branches by always marking the target block as reachable
- **FR-011**: Optimizer MUST evaluate switch statements by determining reachable cases based on the selector's lattice value
- **FR-012**: Optimizer MUST mark Load instruction results as Bottom unless alias analysis proves the loaded location is constant
- **FR-013**: Optimizer MUST mark Store and Call instruction results as Bottom without propagating constants through them
- **FR-014**: Optimizer MUST perform checked arithmetic for integer types using checked_add/checked_sub/checked_mul/checked_div, marking results as Bottom when operations return None (overflow or underflow), never using wrapping arithmetic
- **FR-015**: Optimizer MUST handle floating-point special values (NaN, infinity) according to IEEE 754 semantics
- **FR-016**: Optimizer MUST mark type-mismatched operations as Bottom
- **FR-017**: Optimizer MUST mark division by zero and modulo by zero as Bottom
- **FR-018**: Optimizer MUST propagate constants through safe type casts or mark as Bottom for unsafe conversions
- **FR-019**: Optimizer MUST continue processing until both worklists are empty, indicating fixed-point convergence
- **FR-020**: Optimizer MUST enforce a configurable maximum iteration limit to prevent infinite loops
- **FR-021**: Optimizer MUST rewrite IR after analysis by replacing Constant values with literal constants in instructions
- **FR-022**: Optimizer MUST convert conditional branches with constant conditions to unconditional branches
- **FR-023**: Optimizer MUST remove unreachable blocks from the CFG
- **FR-024**: Optimizer MUST remove incoming edges from unreachable predecessors in phi nodes
- **FR-025**: Optimizer MUST simplify phi nodes with a single incoming value to direct value assignments
- **FR-026**: Optimizer MUST preserve SSA form throughout all transformations
- **FR-027**: Optimizer MUST verify the CFG remains valid after optimization
- **FR-028**: Optimizer MUST validate that no lattice values move upward (from Bottom to Constant/Top or Constant to Top) during analysis
- **FR-029**: Optimizer MUST perform pre-optimization validation ensuring entry block exists, all phi incoming edges reference existing predecessors, and all branch targets exist
- **FR-030**: Optimizer MUST perform post-optimization validation verifying SSA form is preserved and no Top values remain in executable regions
- **FR-031**: Optimizer MUST collect statistics including constants found, branches eliminated, blocks removed, instructions replaced, and iterations to convergence
- **FR-032**: Optimizer MUST provide configuration options for verbose logging and maximum iteration limit (defaulting to 100)
- **FR-033**: Optimizer MUST return Result type propagating validation failures as errors
- **FR-034**: Optimizer MUST log warnings for maximum iterations exceeded or unexpected patterns while continuing conservatively
- **FR-035**: Optimizer MUST preserve source span information through all IR rewrites for error reporting
- **FR-036**: Optimizer MUST handle string type values as always Bottom (non-constant)
- **FR-037**: Optimizer MUST handle character type values with Unicode validation
- **FR-038**: Optimizer MUST track each CFG edge processing count, asserting each is processed at most once
- **FR-039**: Optimizer MUST track each SSA edge processing count, asserting each is processed at most twice (when source value changes or target block becomes executable)

### Key Entities

- **Lattice Value**: Represents the constant state of an SSA value, with variants Top (optimistically unknown), Constant(literal) (proven constant with specific value), and Bottom (pessimistically varying). The lattice forms a partial order where Top ⊑ Constant ⊑ Bottom, and the meet operation computes the greatest lower bound
- **SSA Work List**: Queue of (definition value → use instruction) edges triggered when a value's lattice state changes, ensuring uses are re-evaluated when new constant information becomes available
- **Flow Work List**: Queue of (predecessor block → successor block) CFG edges triggered when new control-flow paths become executable, ensuring newly reachable code is analyzed
- **Executable Edges Set**: Set of CFG edge pairs (predecessor, successor) representing control-flow paths determined to be reachable during analysis
- **Executable Blocks Set**: Set of basic block indices marked as reachable during analysis, initially containing only the entry block
- **Lattice State Map**: Mapping from SSA values to their current lattice state, tracking constant information for all values in the function
- **Optimization Statistics**: Collection of metrics including number of constants found, branches eliminated, blocks removed, instructions replaced, and iterations required for convergence

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Optimizer successfully identifies and propagates at least 95% of statically determinable constants measured using LLVM test-suite constant-propagation benchmarks comparing optimized IR constant count to manually verified expected count
- **SC-002**: Optimizer eliminates all conditional branches with compile-time constant conditions in test programs
- **SC-003**: Optimizer completes analysis and rewriting of functions with up to 10,000 instructions within 1 second on standard hardware
- **SC-004**: Optimizer achieves fixed-point convergence within 100 iterations for 99% of real-world code patterns
- **SC-005**: Optimized programs produce identical runtime behavior to unoptimized versions, verified through comprehensive regression testing
- **SC-006**: Optimizer reduces code size by an average of 5-15% for programs with significant constant computations, measured as (unoptimized_instruction_count - optimized_instruction_count) / unoptimized_instruction_count on jsavrs example programs, measured as (unoptimized_instruction_count - optimized_instruction_count) / unoptimized_instruction_count on jsavrs example programs
- **SC-007**: Optimizer maintains linear O(edges) time complexity, scaling proportionally to the number of SSA edges and CFG edges
- **SC-008**: Optimizer produces zero false positives (incorrect constant values) across all type combinations and operations
- **SC-009**: Post-optimization IR passes SSA form validation and CFG integrity checks with 100% success rate
- **SC-010**: Optimizer handles all supported types (I8, I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Char, String) with correct semantics

## Assumptions

- The input IR is in valid SSA form with unique definitions for each value
- The IR has a well-formed control-flow graph with a single entry block
- All phi nodes appear only in basic blocks with multiple predecessors
- The entry block has no predecessors (no phi nodes in entry)
- Type information is available and accurate for all SSA values
- Existing verification functions (`verify_ssa_form`, `cfg.verify()`) are available and correct
- The IR uses standard instruction kinds for operations (Binary, Unary, Phi, Branch, ConditionalBranch, Switch, Load, Store, Call, Cast, Return, Unreachable)
- Integer arithmetic follows two's complement semantics with checked operations that produce Bottom on overflow/underflow (never wrapping)
- Floating-point arithmetic follows IEEE 754 semantics
- The optimization phase integrates with existing compiler infrastructure through a Phase trait with a run method
- Dead code elimination (DCE) may run as a separate pass, either before or after SCCP
- Performance benchmarking infrastructure using standard test suites is available
- Maximum iteration limit of 100 is sufficient for practical programs (configurable if needed)
- Alias analysis results, if available, can be conservatively ignored by treating all memory loads as Bottom
- The optimizer may coordinate with DCE by either marking dead instructions or removing them directly based on configuration

## Dependencies

- Existing SSA-form IR representation with instruction and value types
- Control-flow graph (CFG) data structure with block and edge representations
- SSA verification functionality to validate correctness
- Type system definitions for all supported IR types
- Error handling infrastructure for propagating validation failures
- Logging infrastructure for warnings and verbose output
- Statistics collection framework
- Testing infrastructure including unit test framework, integration test harness, and performance benchmarking tools
- Existing optimization phase infrastructure with Phase trait

## Out of Scope

- Interprocedural (cross-function) constant propagation
- Advanced alias analysis for memory operations (loads conservatively marked Bottom)
- Optimization of indirect calls or virtual dispatch
- Profile-guided optimization using runtime information
- Optimization of exception handling or unwinding paths (if present in IR)
- Concurrent or parallel execution of the optimization algorithm
- Incremental re-optimization when IR is modified
- Integration with specific backend code generation (IR-level optimization only)
- Optimization of platform-specific intrinsics or inline assembly
- Handling of debug information or metadata beyond source spans
- Symbolic execution or constraint solving for complex conditions
- Loop-specific optimizations (loop unrolling, induction variable analysis) - SCCP operates on general CFGs
