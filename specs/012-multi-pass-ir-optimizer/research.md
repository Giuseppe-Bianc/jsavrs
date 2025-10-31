# Research Summary: Multi-pass IR Optimizer for jsavrs

## Overview
This document provides comprehensive research for implementing a multi-pass IR optimizer in the jsavrs compiler. The optimizer will transform SSA-form IR Modules through rigorous analysis and systematic transformations while guaranteeing semantic preservation.

## Decision: Implementation Architecture
**Rationale**: The optimizer follows a three-layer architecture (analysis framework, transformation passes, verification infrastructure) to ensure modularity, maintainability, and extensibility. This approach allows for independent development of analyses and passes while maintaining clear separation of concerns.

**Alternatives considered**:
1. Monolithic design - rejected due to poor maintainability and difficulty in extending with new passes
2. Plugin-based architecture from the start - rejected as overly complex for initial implementation
3. LLVM-style pass manager - rejected due to complexity and our need for tight integration with existing jsavrs IR

## Core Components Research

### Analysis Framework
The analysis framework will implement forward and backward data flow analysis using a trait-based system:

**UseDefManager**: Builds use-def chains as HashMap<ValueId, InstructionRef> for O(1) lookup of definitions and def-use chains as HashMap<ValueId, Vec<InstructionRef>> for tracking all uses in O(1) time. This is critical for efficient dead code elimination and copy propagation passes.

**ReachingDefinitions**: Uses iterative worklist algorithm with BitVec per basic block tracking reaching definitions, propagated until fixed point is reached. The implementation ensures optimal performance via sparse representation of the kill set.

**LiveVariables**: Implements backward dataflow analysis with HashSet<ValueId> per basic block, propagated from successors to predecessors. This analysis is crucial for dead code elimination and register allocation.

**ConstantLattice**: Tracks abstract values using constant lattice theory (Top, Constant(value), Bottom) stored in HashMap<ValueId, ConstantLattice>, updated via sparse conditional constant propagation for optimal precision.

**AliasAnalysis**: Critical for memory optimization passes. Two implementations planned:
- AndersenAnalysis for O2/O3 using constraint graphs with worklist solving until fixed point
- ConservativeAnalysis for O0/O1 that assumes all pointers may alias (safe but less optimal)

### Transformation Passes
The passes are organized in three phases to ensure correct application order:

**Early Passes**:
- Sparse Conditional Constant Propagation (SCCP): Combines constant propagation with unreachable code elimination by maintaining executable CFG edges
- Aggressive Dead Code Elimination (DCE): Uses mark-and-sweep over use-def chains starting from anchor instructions
- Copy Propagation: Replaces trivial assignments by rewriting use-def chains to reference sources directly

**Middle Passes**:
- Global Value Numbering (GVN) + Common Subexpression Elimination (CSE): Prevents duplicate computations by identifying and eliminating equivalent expressions
- Loop Invariant Code Motion (LICM): Moves loop-invariant computations outside loops
- Induction Variable Optimization: Recognizes induction variables and optimizes derived variables

**Late Passes**:
- Instruction Combining: Pattern-matches sequences of instructions to replace with more efficient equivalents
- Algebraic Simplification: Applies mathematical identities (x*1→x, x+0→x, etc.)
- Strength Reduction: Converts expensive operations to cheaper equivalents (x*8→x<<3)

### Verification Infrastructure
Ensures semantic preservation with multiple verification passes:
- SSA form verification ensuring definition-use domination and phi node correctness
- CFG consistency checking for connectivity and terminator validity
- Type consistency verification ensuring operand types match instruction expectations
- Verification rollback to restore function state on failure

## Performance Considerations
The optimizer implements several performance optimizations:
1. Cache-friendly data structures (Vec for instruction sequences)
2. Fast hashing (FxHasher for small keys like ValueId)
3. Dense boolean sets (BitVec for dataflow analysis)
4. Incremental analysis invalidation to avoid recomputation

## Integration with Existing Infrastructure
The optimizer leverages existing jsavrs components:
- ControlFlowGraph using petgraph DiGraph (already used in the codebase)
- DominanceInfo for loop and SSA transformation
- SsaTransformer for maintaining SSA form during transformations
- Existing IR types (Module, Function, BasicBlock, Instruction, Value)

## Safety and Error Handling
The optimizer uses proper Result<T, OptimizerError> for error handling, with OptimizerError containing:
- VerificationFailed: When SSA/CFG invariants are violated
- AnalysisFailed: When analysis computation encounters errors
- PassError: When specific optimization passes fail

## Testing Strategy
Comprehensive testing includes:
- Unit tests for individual analysis and passes using hand-built CFGs
- Integration tests loading source files, optimizing, and comparing outputs
- Property tests using proptest to validate SSA/CFG invariants
- Benchmark tests using criterion to measure performance characteristics

## Memory Management
Optimizations include:
- Using InstructionRef as lightweight struct (block_label, index) to avoid pointer instability
- AbstractLocation enum for precise alias analysis
- FunctionSnapshot for verification rollback without excessive memory allocation

## Debug Information Preservation
The optimizer maintains SourceSpan information through transformations and optionally records provenance information for debugging purposes, ensuring debuggability is preserved even after optimizations.

## Complexity Analysis
Time complexity considerations:
- Reaching definitions: O(E * |Values|) where E is number of edges in CFG
- Live variables: O(E * |Values|) 
- SCCP: O(N * |Values|) where N is number of instructions
- GVN: O(N * log(N)) for hash table operations
- Loop detection: O(E) using dominator tree analysis

## Future Extensibility
The design supports:
- Plugin-style pass registration
- Custom analysis implementations
- Configuration of optimization levels
- Integration of domain-specific optimizations