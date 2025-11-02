# Feature Specification: Dead Code Elimination (DCE) Optimization

**Feature Branch**: `013-dce`  
**Created**: 2025-11-02  
**Status**: Draft  
**Input**: User description: "Build a Dead Code Elimination (DCE) optimization phase for an intermediate representation (IR) compiler that removes unused code and unreachable instructions from functions."

## Clarifications

### Session 2025-11-02

- Q: Logging and Diagnostics Detail Level - What level of diagnostic information should be provided when optimization encounters limitations? → A: Balanced approach - Report summary statistics plus structured warnings when conservative decisions prevent removal (aliasing uncertainty, unknown call purity) with decision rationale included.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Remove Unreachable Code Blocks (Priority: P1)

A compiler developer compiles a program containing unreachable code blocks (e.g., code after return statements, impossible conditional branches, or dead switch cases). The optimization phase identifies and removes these unreachable blocks, reducing the final compiled output size and improving execution efficiency.

**Why this priority**: This is the most fundamental and visible optimization that DCE provides. Unreachable code removal has the highest impact on code quality and is the easiest to reason about and verify.

**Independent Test**: Can be fully tested by compiling functions with obvious unreachable blocks (code after return, impossible branches) and verifying that those blocks are removed from the final IR while preserving program correctness.

**Acceptance Scenarios**:

1. **Given** a function with code after an unconditional return statement, **When** the DCE phase runs, **Then** all instructions after the return are removed from the IR
2. **Given** a function with an if-statement where the condition is a compile-time constant false, **When** the DCE phase runs, **Then** the entire false branch is removed from the CFG
3. **Given** a function with a switch statement with impossible cases, **When** the DCE phase runs, **Then** the unreachable case blocks are removed
4. **Given** a function with an infinite loop that never returns, **When** the DCE phase runs, **Then** any code following the loop is identified as unreachable and removed

---

### User Story 2 - Eliminate Dead Instructions (Priority: P2)

A compiler developer compiles a program containing computed values that are never used (e.g., temporary variables assigned but never read, arithmetic operations whose results are discarded). The optimization phase identifies these dead instructions through liveness analysis and removes them, reducing both compilation time and runtime overhead.

**Why this priority**: Dead instruction elimination is the second most important optimization as it directly reduces the instruction count and can cascade to enable further optimizations. It requires liveness analysis infrastructure established in P1.

**Independent Test**: Can be fully tested by compiling functions with unused temporary variables and verifying that the instructions computing those values are removed while all actually-used values are preserved.

**Acceptance Scenarios**:

1. **Given** a function that computes a temporary value but never uses it, **When** the DCE phase runs, **Then** the instruction computing that value is removed
2. **Given** a function with a chain of computations where only the final result is used, **When** the DCE phase runs, **Then** all intermediate computations in the chain remain (as they contribute to the final result)
3. **Given** a function with multiple definitions of the same variable where only the last one is read, **When** the DCE phase runs, **Then** earlier dead definitions are removed while the live definition is preserved
4. **Given** a phi node whose result is never used by any successor block, **When** the DCE phase runs, **Then** the phi node is removed

---

### User Story 3 - Optimize Memory Operations Safely (Priority: P3)

A compiler developer compiles a program containing unnecessary memory operations (e.g., stores to local variables that are never read, loads from addresses whose values are never used). The optimization phase uses side-effect analysis to safely remove these operations while preserving all observable program behavior, including maintaining any operations that could affect visible memory state.

**Why this priority**: Memory operation optimization is more complex and requires sophisticated alias analysis to ensure correctness. It builds on the infrastructure from P1 and P2 but has more conservative safety requirements.

**Independent Test**: Can be fully tested by compiling functions with dead stores/loads to local variables and verifying their removal, while ensuring that any potentially-observable memory operations (stores to globals, escaped pointers) are preserved.

**Acceptance Scenarios**:

1. **Given** a function that stores to a local variable but never reads from it before return, **When** the DCE phase runs, **Then** the store instruction is removed
2. **Given** a function that loads from memory but never uses the loaded value, **When** the DCE phase runs, **Then** the load instruction is removed
3. **Given** a function that stores to a pointer that may alias with other memory, **When** the DCE phase runs, **Then** the store is conservatively kept
4. **Given** a function with an alloca whose address never escapes and has no loads, **When** the DCE phase runs, **Then** both the alloca and any stores to it are removed

---

### User Story 4 - Iterative Fixed-Point Optimization (Priority: P4)

A compiler developer compiles a program where removing dead code creates additional optimization opportunities (e.g., removing a use makes its definition dead, removing an instruction makes a block empty and unreachable). The optimization phase iteratively applies analysis and removal steps until no further improvements are possible, maximizing the code reduction.

**Why this priority**: Fixed-point iteration is important for achieving maximum optimization but depends on having the basic analysis and removal capabilities from P1-P3 working correctly first.

**Independent Test**: Can be fully tested by compiling functions with cascading dead code (where removing one instruction makes others dead) and verifying that all transitively-dead code is eventually removed.

**Acceptance Scenarios**:

1. **Given** a function where removing a dead instruction makes another instruction's result unused, **When** the DCE phase runs, **Then** both instructions are removed through iterative analysis
2. **Given** a function where removing all instructions from a block makes it effectively empty, **When** the DCE phase runs, **Then** predecessor branches to that block are updated to skip it
3. **Given** a function where removing a phi node's use makes the phi itself dead, **When** the DCE phase runs, **Then** the phi node is removed in a subsequent iteration
4. **Given** a function where no dead code exists, **When** the DCE phase runs, **Then** analysis reaches fixed-point in a single iteration with no changes

---

### Edge Cases

- What happens when a function consists entirely of dead code (no reachable instructions)?
- How does the system handle circular dependencies between phi nodes in unreachable blocks?
- What happens when removing instructions would make a basic block empty - should the block itself be removed?
- How are function calls handled when the return value is unused but the function may have side effects?
- What happens when a store's target address comes from a complex pointer computation - can we prove non-aliasing?
- How does the system handle indirect branches or computed gotos where targets are determined at runtime?
- What happens when debug information or source location metadata references removed instructions?
- How are scope boundaries preserved when removing instructions that may be scope-defining?
- What happens when all predecessors of a block are removed - how are phi nodes in that block updated?
- How does the system handle volatile or atomic operations that must not be removed despite appearing unused?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST perform reachability analysis starting from the function entry block and identify all blocks reachable through control flow edges
- **FR-002**: System MUST remove all basic blocks that are unreachable from the function entry point while maintaining CFG integrity
- **FR-003**: System MUST perform backward liveness analysis starting from function return points to determine which values are live at each program point
- **FR-004**: System MUST build def-use chains tracking which instructions define values and which instructions use those values
- **FR-005**: System MUST remove instructions whose results are never used (dead) and which have no observable side effects
- **FR-006**: System MUST classify each instruction type as pure (no side effects), memory-dependent, or effect-ful to determine removal safety
- **FR-007**: System MUST preserve store instructions when ANY of these conditions are true: (a) target address has EscapeStatus::Escaped (stored to memory, passed to function, or returned), (b) target address has EscapeStatus::AddressTaken AND alias analysis cannot prove non-aliasing with other accessed memory, (c) escape analysis is inconclusive (conservative default); System MAY remove store ONLY when ALL of these conditions are true: (1) target is EscapeStatus::Local allocation (alloca), (2) no subsequent load instructions read from that allocation, (3) address never escapes the function
- **FR-008**: System MUST preserve function call instructions when ANY of these conditions are true: (a) callee is external function (declaration without body), (b) callee analysis indicates potential side effects (I/O, global state modification, memory writes through pointers), (c) callee purity cannot be determined (conservative default); System MAY remove call ONLY when ALL of these conditions are true: (1) callee is known-pure function (no side effects, deterministic result), (2) return value is unused (dead), (3) no exception/error paths exist that could alter control flow
- **FR-009**: System MUST handle phi nodes specially, considering them live if any successor block uses their result
- **FR-010**: System MUST iterate analysis and removal until reaching fixed-point (no further changes possible)
- **FR-011**: System MUST update CFG edges when removing blocks or changing control flow
- **FR-012**: System MUST update phi node incoming value lists when predecessor blocks are removed
- **FR-013**: System MUST preserve SSA form properties - no uses of undefined values after optimization
- **FR-014**: System MUST preserve debug information and source location metadata where possible
- **FR-015**: System MUST respect scope boundaries when removing instructions
- **FR-016**: System MUST track and report statistics including number of instructions removed, blocks removed, and iterations to fixed-point; system MUST emit structured warnings with rationale when conservative decisions prevent removal (e.g., aliasing uncertainty, unknown call purity, potential side effects)
- **FR-017**: System MUST operate on entire modules, iterating over all functions
- **FR-018**: System MUST skip external function declarations (functions without bodies)
- **FR-019**: System MUST perform escape analysis to track which allocated objects may be accessed outside their defining function
- **FR-020**: System MUST handle all terminator kinds correctly: Return, Branch, ConditionalBranch, Switch, IndirectBranch, Unreachable

### Key Entities

- **LivenessInfo**: Tracks first-use and last-use instruction indices for each value, enabling backward propagation of liveness information through the CFG

- **DefUseChain**: Maps each value identifier to the list of instruction indices that use that value, enabling quick identification of dead definitions

- **ReachabilitySet**: Set of basic block indices that are reachable from the function entry point, used to identify and remove unreachable blocks

- **SideEffectClassification**: Categorization of each instruction as Pure (no side effects, removable if unused), MemoryDependent (affects memory, conditionally removable), or EffectFul (must preserve)

- **AliasAnalysisState**: Tracks which pointer values may alias with each other, enabling safe removal of stores/loads to non-aliasing local memory

- **EscapeAnalysisState**: Tracks which allocated objects escape their defining function through calls or stores, determining if local allocations can be removed

- **OptimizationStatistics**: Per-function counters for instructions removed, blocks removed, and iterations required to reach fixed-point

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For functions with obvious unreachable code (code after return), system removes 100% of unreachable instructions
- **SC-002**: For functions with unused temporary computations, system removes at least 90% of provably-dead instructions
- **SC-003**: System reaches fixed-point within 5 iterations for typical programs (functions with <1000 instructions)
- **SC-004**: Optimization phase completes analysis and removal for a 10,000-instruction function in under 1 second on modern hardware
- **SC-005**: After optimization, no valid program exhibits different observable behavior compared to pre-optimization (correctness preserved at 100%)
- **SC-006**: For benchmark programs with significant dead code, compiled output size is reduced by 15-30% after DCE
- **SC-007**: System reports accurate statistics matching actual changes: reported instruction counts equal actual removal counts with 100% accuracy
- **SC-008**: For programs with no dead code, system completes in a single iteration with zero removals reported
- **SC-009**: SSA form validation passes 100% after optimization (no undefined value uses created)
- **SC-010**: All existing compiler test suites continue to pass after enabling DCE optimization phase
- **SC-011**: When optimization is limited by conservative analysis decisions, system emits structured diagnostic warnings identifying the reason (aliasing, call purity, etc.) for at least 95% of preservation decisions
