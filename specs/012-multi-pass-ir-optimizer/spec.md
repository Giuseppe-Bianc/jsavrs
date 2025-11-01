# Feature Specification: Multi-pass IR optimizer for jsavrs

**Feature Branch**: `012-multi-pass-ir-optimizer`  
**Created**: 28-10-2025  
**Status**: Draft  
**Input**: User description: "multi-pass IR optimizer for the jsavrs compiler that transforms SSA-form Modules through rigorous analysis and systematic transformations while guaranteeing semantic preservation. The optimizer accepts Modules containing Functions with complete CFGs, dominator trees from DominanceInfo, and valid phi nodes at control flow joins, then produces semantically equivalent optimized IR maintaining SSA invariants. It implements a layered architecture with three core subsystems: analysis framework, transformation passes, and verification infrastructure. The analysis framework computes forward and backward data flow analysis including reaching definitions using iterative worklist algorithms, available expressions via value numbering, live variable analysis through backward propagation, and complete use-def and def-use chains stored in hash maps for O(1) lookup. It performs points-to alias analysis using Andersen's algorithm to determine memory dependencies, identifies natural loops using dominator tree back-edge detection, computes loop nesting depths and invariant regions, detects induction variables through pattern matching on phi nodes and arithmetic operations, and tracks constant values through lattice-based abstract interpretation supporting top/bottom/constant states. The transformation subsystem executes passes in carefully ordered phases: early optimization with sparse conditional constant propagation that simultaneously performs constant folding and unreachable code detection by maintaining executable edge sets, aggressive dead code elimination using mark-and-sweep over the use-def graph to remove unused instructions and unreachable blocks while updating phi nodes, and copy propagation that replaces trivial assignments by rewriting the use-def chains. Middle-phase optimizations include global value numbering for common subexpression elimination that assigns unique identifiers to equivalent expressions and replaces duplicates while respecting memory dependencies and side effects, loop-invariant code motion that hoists computations outside loops when domination analysis proves safety and no memory dependencies exist, induction variable optimization that identifies linear and polynomial induction patterns and replaces expensive multiplications with additions, and loop unrolling with configurable thresholds that replicates loop bodies while updating phi nodes and maintaining CFG validity. Late-phase optimizations perform instruction combining through pattern matching that recognizes sequences like consecutive shifts or arithmetic operations that collapse into single instructions, algebraic simplification applying identity laws, strength reduction converting multiplications by powers of two into shifts and divisions into arithmetic right shifts, and reassociation of commutative operations to expose constants. Memory optimizations include store-to-load forwarding that eliminates loads immediately following stores to the same location verified through alias analysis, redundant load elimination tracking memory state across blocks using available expressions analysis extended to memory operations, and dead store elimination that removes stores to locations provably overwritten before next read using backward dataflow analysis. Type-aware optimizations eliminate redundant casts by analyzing the type promotion matrix, combine cascaded casts into single operations, and narrow types when value range analysis proves all values fit in smaller representations. The optimizer maintains CFG integrity through incremental updates: when merging blocks it combines instruction sequences and updates all phi nodes in successors to reference the merged block, when splitting blocks it creates new BasicBlock instances with proper scope inheritance and updates terminator targets, when removing edges it deletes corresponding phi node entries and recomputes dominator tree incrementally using the semi-NCA algorithm, and when eliminating blocks it redirects all incoming edges to successors and removes phi entries. Phi node maintenance includes removing trivial phi nodes where all incoming values are identical by replacing uses with the single value, coalescing phi nodes at the same join point defining equivalent values, and eliminating phi nodes with single predecessors by converting to simple assignments. The system uses efficient data structures: sparse use-def chains implemented as hash maps from Value IDs to instruction vectors enabling O(1) definition lookup and O(k) use enumeration where k is use count, def-use chains as the inverse mapping, a constant lattice map tracking the abstract value (top/constant/bottom) for each SSA temporary, memory state abstractions representing abstract locations and their potential aliases using points-to sets, and worklist queues for iterative dataflow using efficient double-ended queues that avoid redundant reprocessing. Pass ordering follows a strategic sequence: early passes expose opportunities through constant propagation, copy propagation, and branch folding; middle passes perform major transformations via CSE, DCE, LICM, and induction variable optimization; late passes clean up through instruction combining, algebraic simplification, and phi optimization; the system iterates until reaching a fixed point where no pass reports changes or hitting a configurable maximum iteration limit typically set to 10. Integration with the compilation pipeline occurs at a well-defined boundary: the optimizer receives Modules after SSA transformation with verified CFG structure and computed dominator trees, executes its pass sequence while incrementally updating analysis results, and outputs verified optimized Modules to the code generation phase. Each pass implements the OptimizationPass trait declaring required analyses like dominance or alias analysis, invalidated analyses that must be recomputed, and a run method accepting a Function reference and returning a boolean indicating whether modifications occurred. The verification subsystem validates SSA form by checking each temporary is defined exactly once, every use is dominated by its definition, phi nodes have exactly one entry per predecessor, and no uses of undefined values exist. It verifies CFG properties ensuring the entry block has no predecessors, all blocks are reachable from entry via depth-first search, all terminator targets reference valid blocks in the CFG, and all blocks end with valid terminators. Type consistency verification confirms operand types match instruction requirements, phi incoming values have identical types, and no illegal casts exist. Semantic preservation verification optionally compares execution traces when running test suites. Performance evaluation tracks per-pass metrics including instructions eliminated, constants propagated, expressions eliminated by CSE, and blocks removed, along with aggregate metrics like total instruction count reduction percentage, CFG complexity measured by block and edge counts, compilation time overhead per pass and total, and memory usage during optimization. The system supports configurable optimization levels: O0 disables optimization for fast compilation, O1 enables basic optimizations like constant propagation and DCE with single iteration, O2 adds loop optimizations and CSE with multiple iterations, and O3 enables aggressive optimizations including speculative transformations and higher unrolling thresholds. Error handling preserves correctness through conservative analysis defaulting to 'may alias' when pointer analysis is uncertain, verification after each pass with automatic rollback on failure, detailed diagnostic output showing which pass modified each instruction and why, and comprehensive logging of optimization decisions for debugging. The optimizer is extensible through a plugin architecture allowing custom passes to register with the pass manager, domain-specific optimizations to integrate via the standard pass interface, and configurable cost models for instruction selection based on target architecture. It preserves debug information by maintaining ValueDebugInfo through all transformations, tracking source locations in SourceSpan for every instruction, and recording optimization provenance showing the transformation history of each value."

## User Scenarios & Testing *(mandatory)*

This feature targets compiler developers and maintainers who need an IR-level optimizer that consumes SSA-form Modules and emits semantically equivalent, optimized Modules while preserving debug information and enabling configurable optimization levels.

### User Story 1 - Optimize Release Builds (Priority: P1)

As a compiler maintainer, I want the optimizer to minimize instruction count and enhance loop execution efficiency in production builds, ensuring that generated binaries achieve maximal runtime performance. These optimizations must preserve exact program semantics, guaranteeing that improvements in speed and code compactness do not alter observable behavior or computational results.

**Why this priority**: Improves runtime performance and is the primary reason to add an optimizer.

**Independent Test**: Run a benchmark suite (existing `benches` harnesses) on a representative set of inputs before and after optimization and verify behavior equivalence and measurable performance improvements.

**Acceptance Scenarios**:

1. **Given** an SSA Module with loops and repeated expressions, **When** the optimizer runs at O2 or O3, **Then** the Module contains fewer instructions (>= 5% reduction for representative suite) and preserved program outputs.
2. **Given** a Module with induction-variable patterns, **When** induction variable optimization runs, **Then** generated IR replaces multiplications in loops with cheaper additions where safe.

---

### User Story 2 - Fast Iteration During Development (Priority: P2)

As a developer, I want a fast, low-optimization mode so that compilation latency remains low during edit-compile-test cycles.

**Why this priority**: Developer productivity requires short turnaround times.

**Independent Test**: Build with O0 and O1 and measure compile time; O0 should add minimal overhead vs. baseline; O1 performs limited passes with a single iteration.

**Acceptance Scenarios**:

1. **Given** a non-performance-critical build, **When** optimization level O0 or O1 is selected, **Then** optimizer runs with minimal passes (O0: disabled; O1: sparse constant propagation + single DCE iteration) and total compile time overhead is within acceptable limits (see Success Criteria).

---

### User Story 3 - Reliable Semantic Preservation and Debuggability (Priority: P1)

As a QA engineer, I require each optimization pass to be fully verifiable and easily revertible, ensuring that performance improvements do not alter the intended semantics or introduce functional regressions. Every transformation must preserve traceable debugging metadata and maintain sufficient auditability to facilitate precise issue isolation, reproducibility, and rollback during validation or post-deployment analysis.

**Why this priority**: Correctness is mandatory; optimizations must not change observable behavior.

**Independent Test**: Run the project's test suite (`tests/`) with and without optimizations; optimizer must either pass verification checks or automatically roll back failing changes; debug info (SourceSpan) must be preserved for instructions not removed.

**Acceptance Scenarios**:

1. **Given** the full test suite, **When** the optimizer runs at any level (O1..O3), **Then** all tests pass or the optimizer reports verification failures and rolls back modifications for failing Functions/Modules.

---

### Edge Cases

- Modules where pointer/alias information is uncertain: optimizer must default to conservative (may-alias) decisions and avoid speculative memory-based transformations.
- Functions with hand-written inline assembly snippets or external calls which cannot be analyzed precisely: optimizer must treat memory and side-effecting calls conservatively and avoid moving or eliminating memory operations across such calls.
- Very large functions/loops: optimizer must respect configurable thresholds to avoid explosion of compile time or code size. Large functions are defined as those exceeding: (1) 5000 instructions, OR (2) 500 basic blocks, OR (3) loops with >1000 iterations (when statically determinable). For such functions, the optimizer applies reduced optimization strategies with essential passes only and limited iterations (max 2 per phase).

## Clarifications

### Session 2025-10-29

- Q: How should the optimizer handle recursive function calls or complex control flow? → A: Explicitly handle recursion with bounded iteration limits (default: 1000 recursion depth) and conservative assumptions for exception-like control flow (panic handlers, abort paths)
- Q: What should be the bailout threshold when optimization iterations approach limits? → A: Use max iterations fully
- Q: How should the optimizer handle external library calls or FFI functions? → A: Treat all external/FFI calls conservatively with full side-effect assumptions and no inlining
- Q: What is the scope of rollbacks when an optimization pass fails verification? → A: Function-level rollback only
- Q: What constitutes a "pass iteration" in the context of the maximum iteration limit? → A: One full pass-sequence execution

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The optimizer MUST accept SSA-form `Module` inputs where each `Function` has a complete CFG and valid `DominanceInfo` computed.
- **FR-002**: The optimizer MUST provide an analysis framework that exposes reaching-definition queries, available-expression/value-numbering interfaces, live-variable queries, use-def and def-use chain access, aliasing/points-to summaries, and loop/induction metadata; analyses must be incrementally invalidatable by passes. The analysis framework MUST handle recursive function calls with bounded iteration limits (default: 1000 recursion depth) and conservative assumptions for exceptions.
- **FR-003**: The optimizer MUST implement phased transformation passes (early, middle, late) that cover at minimum: constant propagation and branch simplification, dead code elimination, copy propagation, global redundancy elimination (CSE), loop transformations (invariant hoisting, induction-variable optimization, controlled unrolling), instruction combining and algebraic simplification, memory-redundancy elimination (store/load forwarding, redundant-load elimination, dead-store elimination), and type/cast simplifications.
- **FR-004**: Each pass MUST implement the `OptimizationPass` trait that declares required analyses, lists analyses invalidated by the pass, and exposes a `run(&mut Function) -> bool` method returning true when modifications occurred.
- **FR-005**: The optimizer MUST maintain SSA invariants: every SSA value has a single definition, every use is dominated by its definition, phi nodes contain exactly one entry per predecessor, and no uses of undefined values exist after verification; on violation a pass must be rolled back for the affected Function.
- **FR-006**: The optimizer MUST maintain CFG integrity when merging/splitting/removing blocks: update phi nodes, fix terminator targets, and recompute dominator information incrementally using the semi-NCA (Semi-Naive Common Ancestor) algorithm when required by CFG edits.
- **FR-007**: The system MUST provide a verification subsystem that validates SSA form, CFG well-formedness, and type consistency after each pass; verification failures must trigger an automatic rollback for affected changes at the Function level only and produce actionable diagnostics. When verification failures occur, the optimizer SHALL (1) restore the Function to its pre-pass state using FunctionSnapshot, (2) log the failure with pass name, function name, error kind, and diagnostic message, (3) mark the pass as failed for this function and skip it in subsequent iterations, (4) continue optimization with remaining passes for the same function unless critical errors accumulate (default: max 3 verification failures per function before skipping all remaining passes for that function), and (5) report all verification failures in the final OptimizerReport for user review.
- **FR-008**: The optimizer MUST collect per-pass and aggregate metrics (instructions eliminated, constants propagated, CSE replacements, phi removals, blocks removed, instruction-count delta, block/edge counts, per-pass time and memory) and make them available as structured reports.
- **FR-009**: The optimizer MUST support configurable optimization levels (O0, O1, O2, O3) with documented pass sets and iteration limits (default max iterations = 10) and provide knobs for unroll thresholds and speculative options.
- **FR-010**: The optimizer MUST be extensible via a plugin/pass manager allowing external passes to register with declared analysis dependencies and optional cost models.

### Key Entities *(include if feature involves data)*

**Core IR Entities** (from existing jsavrs infrastructure):
- **Module**: Compiler IR container containing multiple `Function`s, CFGs, and metadata required by the optimizer (dominance info, debug info). Includes `DataLayout` and `TargetTriple` for target-specific information.
- **Function**: Contains BasicBlocks, Instructions, Terminators, and Phi nodes; the unit of many optimization passes and verification. Includes `FunctionAttributes` and `IrParameter` for function metadata and parameters.
- **BasicBlock**: Sequence of instructions ending with a terminator; predecessor/successor lists updated as CFG changes. Contains `SourceSpan` for debug information and optional scoping via `ScopeId`.
- **Instruction**: Atomic operations that perform computations, memory access, or control flow. Includes various instruction kinds like binary/unary operations, memory access, calls, and casting through `CastKind` and `InstructionKind`.
- **Terminator**: Defines how control flow exits a basic block (e.g., return, branch, switch). Contains `TerminatorKind` variants and debug information (`DebugInfo`).
- **Value**: Represents values in the IR with different kinds (literal, constant, local, global, temporary). Includes `ValueId`, types (`IrType`), debug info (`ValueDebugInfo`), and scope information. Has subtypes like `IrLiteralValue` and `IrConstantValue`.
- **IrLiteralValue**: Enum representing literal values including integers (`I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`), floats (`F32`, `F64`), and other types (`Bool`, `Char`).
- **IrConstantValue**: Enum representing constant values including `String`, `Array`, and `Struct` constants for compile-time values.
- **ValueKind**: Enum representing different value types (literal, constant, local, global, temporary) that classify how a value is stored and accessed in the IR.
- **ValueId**: Unique identifier for values in the IR, implemented using UUID for global uniqueness across compilation units.
- **IrParameter**: Parameter definition including name, type, and attributes for function parameters.
- **ParamAttributes**: Attributes for function parameters such as `by_val` and `no_alias`.
- **FunctionAttributes**: Attributes for functions such as `is_entry`, `is_varargs`, and calling convention.
- **CastKind**: Enum specifying different kinds of type casting operations including integer widening/narrowing, integer/float conversions, and bit reinterpretations.
- **IrBinaryOp**: Enum for binary operations including arithmetic, comparison, logical, and bitwise operations.
- **IrUnaryOp**: Enum for unary operations such as negation and logical not.
- **VectorOp**: Enum for vector operations including arithmetic, dot product, and shuffling operations.
- **InstructionKind**: Enum representing different types of instructions such as `Alloca`, `Store`, `Load`, `Binary`, `Unary`, `Call`, `GetElementPtr`, `Cast`, `Phi`, and `Vector`.
- **TerminatorKind**: Enum representing different terminator types such as `Return`, `Branch`, `ConditionalBranch`, `IndirectBranch`, `Switch`, and `Unreachable`.
- **DebugInfo**: Debugging metadata for instructions and terminators, including source span information.
- **ScopeId**: Unique identifier representing a scope within the IR (e.g., function, block, module) implemented as UUID for global uniqueness.
- **ResourceId**: Globally unique identifier representing resources such as types, modules, or external symbols.
- **ValueDebugInfo**: Debugging information for values including name and source span mapping.
- **TypeGroup**: Classification of types for promotion purposes (integer, float, boolean, character, string).
- **PromotionMatrix**: Matrix containing promotion rules for each pair of types to determine valid type conversions.
- **PromotionRule**: Rule defining how to promote one type to another including direct and indirect conversion paths.
- **PromotionWarning**: Warnings generated for potentially unsafe type conversions such as precision loss or overflow.
- **OverflowBehavior**: Defines behavior for handling overflow conditions during type promotions.
- **PrecisionLossEstimate**: Estimate of precision loss during type conversions, particularly floating-point to integer conversions.
- **FloatSpecialValueType**: Special floating-point values like infinity, NaN, and negative zero that may require special handling during type promotion.
- **BinaryOperationPromotion**: Result type for binary operations that determines the common type for operand promotion.
- **PromotionResult**: Result of a type promotion operation including the target type and any warnings.
- **TypePromotion**: Comprehensive system for handling type promotions across all type pairs.
- **AccessController/AccessRules**: Access control system for managing visibility and permissions for IR elements.
- **Operation**: Enum representing different types of access operations that can be controlled.
- **Scope**: Representation of a lexical or semantic scope containing symbol mappings and hierarchical relationships.
- **ControlFlowGraph (CFG)**: Directed graph of BasicBlocks with edges representing possible control flow paths. Uses petgraph's DiGraph internally for analysis and transformations.
- **DominanceInfo**: Contains dominance relationships in the control flow graph, including immediate dominators (`idom`), dominance frontiers, and dominator tree children used for SSA construction and optimization passes.
- **IrType**: Enum representing all possible IR types including primitives (integers, floats, bool, char, string), compound types (pointers, arrays, structs), and custom types with source span information.
- **SsaTransformer**: Manages the Static Single Assignment transformation process, handling phi-node insertion and SSA form maintenance for optimization passes.
- **Scope/ScopeManager**: Hierarchical scoping system for managing variable lifetimes and symbol resolution. `Scope` contains symbol mappings and hierarchy information; `ScopeManager` manages the scope stack and provides scoping operations for IR generation and analysis.
- **TypePromotion**: Comprehensive type promotion system handling conversions between types with a 169-type-pair matrix. Includes `PromotionMatrix`, `PromotionRule`, `TypePromotion`, `PromotionResult`, and `PromotionWarning` for safe type conversions during analysis and optimization.
- **TypePromotionEngine**: Core logic for analyzing and implementing type promotions in binary operations, using the promotion matrix to determine target types and generate appropriate cast instructions with proper warnings for precision loss or overflow.
- **AnalysisResult**: Objects produced by analyses (ReachingDefs, LiveVars, ValueNumbering, PointsToSets, DominanceInfo, LoopInfo) with clear invalidation rules.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For the standard benchmark suite (defined by maintainers), optimization at O2 yields >= 5% median instruction-count reduction across benchmarks compared to unoptimized IR, while preserving test-suite outputs.
- **SC-002**: Optimizer verification must report zero unchecked SSA/CFG errors for >= 95% of Functions in typical code; functions failing verification must be automatically rolled back and logged.
- **SC-003**: Compile-time overhead introduced by optimization (total of all passes) must be < 30% for O1, < 100% for O2 relative to a baseline compilation (defined as: O0 optimization level with completely bypassed optimizer pipeline, measured from Module input to optimized Module output) in the representative project; thresholds configurable per project.
- **SC-004**: Debug information (SourceSpan) must be preserved for at least 90% of instructions that remain after optimization, and optimization provenance must be recordable for any value transformed.

