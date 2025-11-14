# Feature Specification: Constant Folding and Propagation Optimizer

**Feature Branch**: `015-constant-folding-sccp`  
**Created**: 2025-11-14  
**Status**: Draft  
**Input**: User description: "Build a comprehensive constant folding and propagation optimizer for an intermediate representation (IR) compiler phase that eliminates compile-time evaluable operations and propagates constant values through the control flow graph. The optimizer must integrate seamlessly with an existing SSA-based IR system that includes control flow graphs, basic blocks, phi nodes, and dominance analysis.

## Clarifications

### Session 2025-11-14

- Q: When verbose mode is enabled, where should detailed optimization statistics be output? → A: Structured to stderr with instruction counts to stdout
- Q: When the optimizer encounters malformed IR (invalid SSA references, missing CFG information), how should it behave? → A: Fail conservatively: skip optimization, preserve original instruction
- Q: What should the integer overflow behavior be for constant folding arithmetic? → A: Wrapping (two's complement)
- Q: What memory usage constraint should apply to the SCCP lattice value tracking? → A: Maximum 100KB memory for lattice values per function
- Q: What data structure should be used to map SSA values to their lattice states in SCCP? → A: Hash map with SSA value ID keys
- Q: When the optimizer encounters floating-point operations that could produce NaN through operations like `0.0 / 0.0` or `sqrt(-1.0)`, should it fold them to NaN or preserve the original instruction to maintain portability across different floating-point implementations? → A: Fold to NaN following IEEE 754 rules deterministically
- Q: How should the optimizer handle test execution and validation during development - should it include a built-in test mode that validates transformations, or rely entirely on external test harnesses? → A: External test harness only (standard Cargo test integration)
- Q: When SCCP determines a function has no constant-foldable operations after analysis, should it skip subsequent transformation passes entirely or still perform a CFG cleanup pass to ensure consistency? → A: Always perform CFG cleanup pass for consistency
- Q: Should the optimizer maintain detailed metrics per-function (enabling fine-grained profiling but using more memory) or aggregate metrics globally across all functions (using less memory but providing less diagnostic granularity)? → A: Per-function metrics for detailed profiling
- Q: When the optimizer encounters phi nodes with incoming values from unreachable predecessors during SCCP, should it remove those incoming edges from the phi node immediately during analysis or defer edge removal to the final CFG cleanup pass? → A: Defer edge removal to final CFG cleanup pass
## Core Functionality
The optimizer analyzes IR functions to identify opportunities for constant folding (evaluating constant expressions at compile time) and constant propagation (replacing variable uses with their known constant values). It operates on an IR module containing multiple functions, each with a CFG of basic blocks containing instructions in SSA form.
### Constant Folding Requirements
The system must evaluate binary operations, unary operations, and cast operations when all operands are compile-time constants. This includes:
- **Arithmetic operations**: Addition, subtraction, multiplication, division, modulo for integer and floating-point types
- **Comparison operations**: Equality, inequality, less than, greater than, and their variants
- **Logical operations**: AND, OR for booleans
- **Bitwise operations**: Bitwise AND, OR, XOR, shift left, shift right for integer types
- **Unary operations**: Negation and logical NOT
- **Type conversions**: All cast kinds including integer widening/narrowing, signed/unsigned conversions, float conversions, and conversions between numeric types and booleans
When folding operations, the system must respect type semantics including overflow behavior for integers, NaN and infinity handling for floating-point operations, and proper handling of division by zero scenarios.
### Constant Propagation Requirements
The system must track constant values across the program by analyzing variable definitions and uses. When a variable is assigned a constant value through a store instruction, subsequent load instructions from that same variable location should be replaced with the constant value directly, eliminating the redundant load.
The propagation must respect SSA form and properly handle:
- Simple assignment chains where constants flow from definitions to uses
- Variables that are defined once with a constant value and used multiple times
- Local allocations that never escape their defining function
### Sparse Conditional Constant Propagation (SCCP)
When SCCP mode is enabled, the optimizer performs a more sophisticated interprocedural constant propagation that operates on the SSA graph structure. SCCP uses a worklist algorithm to simultaneously discover executable code paths and propagate constant values.
The SCCP algorithm must:
- Initialize with an assumption that all blocks except the entry block are unreachable
- Use a worklist of SSA edges to explore, starting from function parameters and entry block instructions
- Maintain a lattice value for each SSA value representing Top (unknown), Constant (known constant), or Bottom (not constant/unknown)
- Process phi nodes specially by merging incoming values based on which predecessor blocks are reachable
- Mark conditional branches as resolved when the condition becomes constant, making one successor reachable and the other unreachable
- Iterate until reaching a fixed point where no more values change and no new blocks become reachable
- Replace instructions with constant results when all operands have reached constant lattice values
- Remove unreachable blocks and dead branches after propagation completes
SCCP must handle complex control flow including loops, multiple entry points into blocks, and nested conditionals. When a phi node has multiple incoming edges but only some predecessors are reachable, only those incoming values participate in the lattice merge operation.
### Integration Requirements
The optimizer implements the existing `Phase` trait which defines a `name()` method returning the phase identifier and a `run()` method that mutates the IR module. The implementation must preserve SSA form throughout all transformations and maintain CFG validity.
After optimization, the system prints the total instruction count remaining in the module. When verbose mode is enabled, detailed statistics should be available including the number of instructions folded, values propagated, and branches resolved.
### Safety and Correctness Constraints
The optimizer must never change program semantics. All transformations must be semantically preserving, meaning the optimized program produces identical observable behavior to the original program for all valid inputs.
Specific safety requirements:
- Never fold operations that could produce undefined behavior (like division by zero in contexts where it's not already guaranteed safe)
- Respect floating-point semantics including signed zeros, infinities, and NaN propagation
- Preserve evaluation order for operations with potential side effects
- Never remove or reorder instructions that could affect memory visible to other functions
- Maintain the validity of phi node incoming value lists
- Ensure all value uses reference valid definitions after transformation
- Preserve debug information and source span metadata when possible
The optimizer should be conservative when uncertain about transformation safety. When the escape analysis or alias analysis cannot prove that a memory operation is safe to optimize, the original instruction should be preserved.
### Performance Considerations
The implementation should favor linear or near-linear time complexity relative to the number of instructions. Avoid algorithms that require repeated traversal of the entire function for each instruction. Use efficient data structures like hash maps for constant value lookups and worklist-based algorithms to process only instructions that could be affected by changes.

The SCCP implementation must use a hash map with SSA value ID keys to track lattice states, providing O(1) lookup performance for constant status checks during worklist processing.

Memory usage for SCCP lattice value tracking must not exceed 100KB per function. If this limit would be exceeded, the optimizer should fall back to basic constant folding without SCCP for that function and emit a warning.
### Error Handling
The optimizer should gracefully handle malformed IR by logging warnings and continuing with conservative transformations rather than panicking. When encountering unexpected conditions like missing CFG information or invalid SSA references, emit diagnostic messages that help identify the source of the issue.
### Modularity
Given the complexity of constant folding logic, the implementation should be split across multiple files:
- A main optimizer file containing the `Phase` implementation and high-level orchestration
- A lattice module defining the value lattice used for SCCP
- An evaluator module containing the constant folding logic for all operation types
- A worklist module implementing the SCCP worklist algorithm
- A statistics module for tracking optimization metrics
- A utilities module for common helper functions
This separation ensures maintainability and testability of individual components while keeping the codebase organized and comprehensible."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Constant Folding (Priority: P1)

A compiler developer compiles a program containing simple constant expressions like `2 + 3` or `10 * 5`. The optimizer analyzes the IR and replaces these compile-time evaluable operations with their constant results, eliminating unnecessary computation at runtime. This reduces the instruction count in the generated code and improves execution performance.

**Why this priority**: This is the foundational capability that delivers immediate value. Even without advanced propagation, folding constant expressions provides measurable compilation improvements and serves as the basis for more sophisticated optimizations.

**Independent Test**: Can be fully tested by compiling programs with arithmetic constant expressions and verifying that the optimized IR contains constant values instead of operation instructions, delivering reduced instruction count and faster execution.

**Acceptance Scenarios**:

1. **Given** an IR function containing `add i32 2, 3`, **When** the optimizer runs, **Then** the instruction is replaced with the constant value `5`
2. **Given** an IR function with nested constant operations `mul (add 2, 3), 4`, **When** the optimizer runs, **Then** both operations are folded to the constant value `20`
3. **Given** an IR function with floating-point constant arithmetic `fadd 1.5, 2.5`, **When** the optimizer runs, **Then** the instruction is replaced with the constant `4.0`
4. **Given** an IR function with comparison operations `icmp eq 10, 10`, **When** the optimizer runs, **Then** the instruction is replaced with boolean constant `true`
5. **Given** an IR function with type conversions between constant values `sext i8 127 to i32`, **When** the optimizer runs, **Then** the instruction is replaced with the constant `127` of type i32

---

### User Story 2 - Simple Constant Propagation (Priority: P2)

A compiler developer compiles a program where variables are initialized with constant values and then used in subsequent operations. The optimizer tracks which variables hold constant values and replaces load operations with those constants, eliminating redundant memory accesses.

**Why this priority**: This extends the basic folding capability to handle common programming patterns where constants are stored in variables. It provides significant optimization opportunities in typical code while remaining relatively straightforward to implement and verify.

**Independent Test**: Can be tested independently by compiling programs with constant variable assignments and verifying that subsequent uses of those variables are replaced with direct constant values, eliminating load instructions.

**Acceptance Scenarios**:

1. **Given** an IR sequence where a local variable is stored with constant `42` then loaded, **When** the optimizer runs, **Then** the load is replaced with the constant `42`
2. **Given** a local variable assigned once with a constant and loaded multiple times, **When** the optimizer runs, **Then** all loads are replaced with the constant value
3. **Given** an operation that uses the result of a constant load `add %loaded_const, 10`, **When** the optimizer runs, **Then** the load is eliminated and the add operation is folded to a constant
4. **Given** a variable that is reassigned with different values, **When** the optimizer runs, **Then** only the loads that are guaranteed to see constant values are optimized

---

### User Story 3 - Advanced SCCP Analysis (Priority: P3)

A compiler developer compiles a program with conditional branches where the condition can be determined at compile time. The SCCP optimizer analyzes control flow to identify unreachable code paths, propagate constants through complex flow including phi nodes, and eliminate dead branches.

**Why this priority**: This represents the most sophisticated optimization capability. While it provides powerful optimization for complex control flow patterns, it requires the foundational capabilities from P1 and P2 and is more complex to implement and validate correctly.

**Independent Test**: Can be tested by compiling programs with constant conditional branches and verifying that unreachable blocks are removed, phi nodes are simplified, and constants propagate through complex control flow, delivering maximal code size reduction.

**Acceptance Scenarios**:

1. **Given** a conditional branch where the condition is constant `br i1 true, label %then, label %else`, **When** SCCP runs, **Then** the else block is marked unreachable and removed
2. **Given** a phi node `phi [5, %block1], [5, %block2]` where all incoming values are the same constant, **When** SCCP runs, **Then** the phi is replaced with the constant `5`
3. **Given** a phi node with multiple incoming values but some predecessor blocks are unreachable, **When** SCCP runs, **Then** only values from reachable predecessors are considered in the merge
4. **Given** a loop where the iteration count can be determined at compile time, **When** SCCP runs, **Then** loop-carried constants are propagated and the loop may be partially or fully unrolled by subsequent phases
5. **Given** nested conditional branches with constant conditions, **When** SCCP runs, **Then** only reachable paths are preserved and all unreachable blocks are eliminated

---

### Edge Cases

- What happens when dividing by zero in a constant expression (e.g., `div i32 10, 0`)? The optimizer must detect this and avoid folding to prevent introducing undefined behavior.
- How does the system handle floating-point special values like NaN, infinity, and signed zeros? All floating-point constant folding must preserve IEEE 754 semantics.
- What happens when a phi node has incoming edges from both reachable and unreachable blocks? SCCP must only merge values from blocks that are proven reachable.
- How does the optimizer handle integer overflow in constant arithmetic? The folding must respect the wrapping semantics of the target type.
- What happens when a variable might escape its function (address taken and passed to another function)? The optimizer must conservatively avoid propagating constants that might be modified externally.
- How does the system handle casts between incompatible types or precision-losing conversions? All type conversions must follow the IR's defined semantics.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Optimizer MUST evaluate binary arithmetic operations (add, subtract, multiply, divide, modulo) when both operands are constant values
- **FR-002**: Optimizer MUST evaluate comparison operations (equal, not equal, less than, greater than, etc.) when both operands are constant values
- **FR-003**: Optimizer MUST evaluate logical operations (AND, OR) on constant boolean values
- **FR-004**: Optimizer MUST evaluate bitwise operations (AND, OR, XOR, shift left, shift right) on constant integer values
- **FR-005**: Optimizer MUST evaluate unary operations (negation, logical NOT) on constant values
- **FR-006**: Optimizer MUST evaluate type conversion operations (casts) when the source value is constant
- **FR-007**: Optimizer MUST use wrapping (two's complement) semantics for integer overflow during constant folding, matching the behavior of the target architecture
- **FR-008**: Optimizer MUST preserve IEEE 754 semantics for floating-point operations including NaN, infinity, and signed zero; operations that produce NaN (such as `0.0 / 0.0` or `sqrt(-1.0)`) MUST be folded to NaN deterministically following IEEE 754 rules
- **FR-009**: Optimizer MUST NOT fold operations that would introduce undefined behavior such as division by zero
- **FR-010**: Optimizer MUST track constant values assigned to local variables through store instructions
- **FR-011**: Optimizer MUST replace load instructions with constant values when the loaded location is known to hold a constant
- **FR-012**: Optimizer MUST preserve SSA form throughout all transformations
- **FR-013**: Optimizer MUST maintain control flow graph validity after removing unreachable blocks
- **FR-014**: When SCCP mode is enabled, optimizer MUST analyze control flow to identify reachable and unreachable blocks
- **FR-015**: When SCCP mode is enabled, optimizer MUST maintain a lattice value (Top/Constant/Bottom) for each SSA value using a hash map indexed by SSA value IDs
- **FR-016**: When SCCP mode is enabled, optimizer MUST process phi nodes by merging incoming values only from reachable predecessors; removal of unreachable incoming edges MUST be deferred to the final CFG cleanup pass rather than performed during SCCP analysis
- **FR-017**: When SCCP mode is enabled, optimizer MUST mark conditional branches as resolved when the condition becomes constant
- **FR-018**: When SCCP mode is enabled, optimizer MUST remove blocks proven unreachable by constant branch resolution
- **FR-019**: Optimizer MUST implement the existing Phase trait with name() and run() methods
- **FR-020**: Optimizer MUST preserve debug information and source span metadata when possible
- **FR-021**: Optimizer MUST be conservative when escape analysis cannot prove a memory location is local
- **FR-022**: Optimizer MUST NOT reorder instructions that could affect externally visible memory
- **FR-023**: Optimizer MUST ensure all value uses reference valid definitions after transformation
- **FR-024**: Optimizer MUST maintain validity of phi node incoming value lists after block removal
- **FR-025**: Optimizer MUST emit diagnostic warning messages when encountering malformed IR (invalid SSA references, missing CFG information), skip the specific optimization for that instruction, and preserve the original instruction rather than panicking or aborting the entire pass
- **FR-026**: Optimizer MUST print total instruction count to stdout after optimization completes
- **FR-027**: When verbose mode is enabled, optimizer MUST output detailed statistics per-function (folded instruction count, propagated value count, resolved branch count) to stderr in structured format to enable fine-grained profiling and diagnostics
- **FR-028**: Optimizer MUST limit SCCP lattice value tracking memory to maximum 100KB per function, falling back to basic constant folding if limit would be exceeded
- **FR-029**: Optimizer MUST always perform CFG cleanup pass after SCCP analysis completes, regardless of whether constant-foldable operations were found, to ensure CFG consistency for downstream compiler passes

### Non-Functional Requirements

- **NFR-001**: Optimizer MUST complete processing of functions with 1000+ instructions in under 1 second
- **NFR-002**: Optimizer MUST use linear or near-linear time complexity algorithms relative to instruction count
- **NFR-003**: Optimizer validation MUST rely on external test harness using standard Cargo test framework rather than built-in self-validation modes

### Key Entities

- **Lattice Value**: Represents the compile-time knowledge about an SSA value during SCCP analysis. Can be Top (not yet analyzed), Constant (proven to be a specific constant), or Bottom (not constant or multiple possible values).
- **Constant Expression**: An IR operation where all operands are known constant values, making the result computable at compile time.
- **Reachable Block**: A basic block in the control flow graph that can be reached from the function entry through executable branches.
- **Executable Edge**: A control flow edge from one block to another that is proven to be traversable during SCCP analysis.
- **Constant Store**: A store instruction that writes a known constant value to a memory location.
- **Foldable Operation**: An instruction that can be replaced with a constant result because all its operands are constants.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Programs with constant arithmetic expressions show reduced instruction count in optimized IR compared to unoptimized IR (measurable via instruction count difference)
- **SC-002**: Programs with constant variable assignments show eliminated load instructions in optimized IR (measurable via load instruction count reduction)
- **SC-003**: Programs with constant conditional branches show removed unreachable blocks when SCCP is enabled (measurable via basic block count reduction)
- **SC-004**: Optimized programs produce identical output to unoptimized programs for all test cases (semantic preservation verified through test suite)
- **SC-005**: Optimizer completes processing of functions with 1000+ instructions in under 1 second (performance target)
- **SC-006**: Optimizer successfully handles complex control flow including nested loops and multiple conditional branches without errors
- **SC-007**: Test suite demonstrates correct handling of edge cases including division by zero, NaN propagation, and integer overflow with 100% pass rate
- **SC-008**: Optimizer maintains SSA form validity as verified by SSA validation checks after optimization
- **SC-009**: Optimizer preserves CFG validity as verified by CFG validation checks after optimization
