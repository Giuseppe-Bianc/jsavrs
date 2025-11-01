# Data Model: Multi-pass IR Optimizer

**Feature**: 012-multi-pass-ir-optimizer  
**Date**: 2025-11-01  
**Purpose**: Comprehensive specification of all data structures, their relationships, and invariants

## 1. Core Optimizer Structures

### 1.1 OptimizerConfig

**Purpose**: Configuration for optimization pipeline, including optimization level, pass selection, and thresholds.

**Structure**:
```rust
pub struct OptimizerConfig {
    /// Optimization level (O0, O1, O2, O3)
    pub opt_level: OptLevel,
    
    /// Maximum iterations for fixed-point convergence (default: 10 for O2/O3, 1 for O1)
    pub max_iterations: usize,
    
    /// Loop unrolling threshold (iterations, default: 4 for O2, 8 for O3)
    pub loop_unroll_threshold: usize,
    
    /// Alias analysis implementation
    pub alias_analysis_kind: AliasAnalysisKind,
    
    /// Early-phase passes (constant propagation, DCE)
    pub early_passes: Vec<Box<dyn OptimizationPass>>,
    
    /// Middle-phase passes (GVN, LICM, loop optimizations)
    pub middle_passes: Vec<Box<dyn OptimizationPass>>,
    
    /// Late-phase passes (instruction combining, phi optimization)
    pub late_passes: Vec<Box<dyn OptimizationPass>>,
    
    /// Enable optimization provenance tracking (for debugging)
    pub record_provenance: bool,
}

pub enum OptLevel {
    O0,  // No optimization (fast compilation)
    O1,  // Basic optimization (single iteration, conservative alias analysis)
    O2,  // Full optimization (multi-iteration, Andersen alias analysis)
    O3,  // Aggressive optimization (increased thresholds, speculative transformations)
}

pub enum AliasAnalysisKind {
    Conservative,                           // Always returns may-alias (O0/O1)
    Andersen,                               // Inclusion-based points-to analysis (O2/O3)
    Custom(Box<dyn AliasAnalysis>),         // User-provided implementation
}
```

**Factory Method**:
```rust
impl OptimizerConfig {
    /// Creates configuration for given optimization level with default passes
    pub fn config_for_level(level: OptLevel) -> Self;
}
```

**Relationships**:
- Contains `Vec<Box<dyn OptimizationPass>>` for each phase
- References `AliasAnalysisKind` determining memory dependency precision
- Used by `PassManager` to orchestrate optimization pipeline

**Invariants**:
- `max_iterations >= 1`
- `loop_unroll_threshold >= 1`
- `early_passes`, `middle_passes`, `late_passes` may be empty (e.g., O0) but not null
- At O0: all pass vectors empty, max_iterations = 0
- At O1: only SCCP and ADCE in early_passes, max_iterations = 1, Conservative alias analysis
- At O2/O3: full pass set, max_iterations = 10, Andersen alias analysis

---

### 1.2 PassManager

**Purpose**: Orchestrates execution of optimization passes, manages analysis cache, handles fixed-point iteration.

**Structure**:
```rust
pub struct PassManager {
    config: OptimizerConfig,
    analysis_manager: AnalysisManager,
}

impl PassManager {
    /// Creates new pass manager with given configuration
    pub fn new(config: OptimizerConfig) -> Self;
    
    /// Runs optimization pipeline on module
    /// Returns aggregate metrics for all passes
    pub fn run_optimization_pipeline(&mut self, module: &mut Module) -> OptimizerReport;
    
    /// Runs optimization pipeline on single function
    /// Returns whether any changes occurred
    fn optimize_function(&mut self, function: &mut Function) -> bool;
}
```

**Algorithm** (optimize_function):
```
for iteration in 0..config.max_iterations:
    any_changes_this_iteration = false
    
    for pass in config.early_passes:
        result = run_pass_with_verification(pass, function, analysis_manager)
        any_changes_this_iteration |= result.changed
    
    for pass in config.middle_passes:
        result = run_pass_with_verification(pass, function, analysis_manager)
        any_changes_this_iteration |= result.changed
    
    for pass in config.late_passes:
        result = run_pass_with_verification(pass, function, analysis_manager)
        any_changes_this_iteration |= result.changed
    
    if !any_changes_this_iteration:
        break  // Fixed point reached
```

**Relationships**:
- Contains `OptimizerConfig` determining pass sequence
- Contains `AnalysisManager` providing cached analysis results to passes
- Invokes `OptimizationPass::run()` for each pass
- Produces `OptimizerReport` with aggregate metrics

**Invariants**:
- At most `config.max_iterations` iterations executed
- Fixed point detection ensures early exit when no pass reports changes
- Verification runs after each pass that reports changes

---

### 1.3 AnalysisManager

**Purpose**: Caches analysis results, invalidates stale analyses when IR changes, provides query interface to passes.

**Structure**:
```rust
pub struct AnalysisManager {
    /// Cache: (function_name, analysis_kind) → boxed analysis result
    cache: HashMap<(String, AnalysisKind), Box<dyn Any>>,
    
    /// Analysis dependencies: which analyses depend on which others
    dependencies: HashMap<AnalysisKind, Vec<AnalysisKind>>,
}

pub enum AnalysisKind {
    ReachingDefs,
    LiveVars,
    UseDef,
    Dominance,
    Loops,
    Constants,
    Alias,
    ValueNumbering,
}

impl AnalysisManager {
    /// Retrieves or computes analysis result
    /// Returns reference to cached result (downcasts from Box<dyn Any>)
    pub fn get_analysis<T: Analysis + 'static>(
        &mut self,
        function: &Function,
        kind: AnalysisKind,
    ) -> &T;
    
    /// Invalidates specific analysis and its dependents
    pub fn invalidate(&mut self, kind: AnalysisKind);
    
    /// Invalidates all analyses for function (called after modification)
    pub fn invalidate_all_for_function(&mut self, function_name: &str);
}
```

**Relationships**:
- Used by `PassManager` to provide analysis results to passes
- Caches `Analysis` trait objects (implementing `ReachingDefinitions`, `LiveVariables`, etc.)
- Tracks dependencies between analyses (e.g., `LoopInfo` depends on `Dominance`)

**Invariants**:
- Cache entries are always valid (invalidate removes stale entries)
- Transitive invalidation: invalidating A invalidates all analyses depending on A
- Thread-safe: uses interior mutability if needed for parallel passes (future extension)

---

## 2. Analysis Framework

### 2.1 Analysis Trait

**Purpose**: Common interface for all analyses, enabling uniform caching and invalidation.

**Structure**:
```rust
pub trait Analysis {
    /// Computes analysis from function
    fn compute(function: &Function) -> Self where Self: Sized;
    
    /// Invalidates cached state (called when IR changes)
    fn invalidate(&mut self);
}
```

**Implementations**:
- `UseDefManager`
- `ReachingDefinitions`
- `LiveVariables`
- `ConstantLattice`
- `LoopInfo`
- `GlobalValueNumbering`
- Alias analysis types (`ConservativeAnalysis`, `AndersenAnalysis`)

---

### 2.2 UseDefManager

**Purpose**: Maintains use-def and def-use chains for O(1) definition lookup and O(k) use enumeration.

**Structure**:
```rust
pub struct UseDefManager {
    /// Use → definition
    pub use_def: HashMap<ValueId, InstructionRef>,
    
    /// Definition → uses
    pub def_use: HashMap<ValueId, Vec<InstructionRef>>,
}

pub struct InstructionRef {
    pub block_label: String,
    pub index: usize,  // Index in block's instruction vector (or len for terminator)
}

impl UseDefManager {
    /// Builds use-def chains from function
    pub fn build(function: &Function) -> Self;
    
    /// Returns definition of value (O(1))
    pub fn get_definition(&self, value_id: ValueId) -> Option<&InstructionRef>;
    
    /// Returns uses of value (O(1) + O(k) iteration)
    pub fn get_uses(&self, value_id: ValueId) -> &[InstructionRef];
    
    /// Updates chains after replacing use (old_value → new_value at instruction)
    pub fn replace_use(&mut self, inst_ref: &InstructionRef, old_value: ValueId, new_value: ValueId);
    
    /// Removes instruction from chains
    pub fn remove_instruction(&mut self, inst_ref: &InstructionRef, instruction: &Instruction);
}
```

**Invariants**:
- In SSA: each ValueId appears as key in `def_use` at most once (single definition)
- Each InstructionRef in `def_use[vid]` corresponds to instruction actually using `vid`
- Each ValueId key in `use_def` corresponds to operand of instruction at `use_def[vid]`

---

### 2.3 ReachingDefinitions

**Purpose**: Dataflow analysis tracking which definitions reach each block (used by SCCP, constant propagation).

**Structure**:
```rust
pub struct ReachingDefinitions {
    /// Definitions reaching block entry
    pub reaching_in: HashMap<String, BitVec>,
    
    /// Definitions reaching block exit
    pub reaching_out: HashMap<String, BitVec>,
    
    /// Mapping: definition index → ValueId
    pub def_index: Vec<ValueId>,
    
    /// Mapping: ValueId → definition index
    pub value_to_index: HashMap<ValueId, usize>,
}

impl ReachingDefinitions {
    /// Computes reaching definitions via worklist algorithm
    pub fn compute(function: &Function) -> Self;
    
    /// Returns definitions reaching block entry
    pub fn reaching_at_block_entry(&self, block: &str) -> impl Iterator<Item = ValueId> + '_;
}
```

**Algorithm**: Forward dataflow with worklist (see research.md section 5).

**Invariants**:
- `def_index.len() == value_to_index.len()` (bijection)
- All BitVec have length `def_index.len()`
- `reaching_in[entry_block]` is empty (no definitions reach entry)

---

### 2.4 LiveVariables

**Purpose**: Backward dataflow analysis determining which variables are live at each program point (used by DCE).

**Structure**:
```rust
pub struct LiveVariables {
    /// Variables live at block entry
    pub live_in: HashMap<String, HashSet<ValueId>>,
    
    /// Variables live at block exit
    pub live_out: HashMap<String, HashSet<ValueId>>,
}

impl LiveVariables {
    /// Computes live variables via backward worklist algorithm
    pub fn compute(function: &Function) -> Self;
    
    /// Checks if value is live at block entry
    pub fn is_live_at_entry(&self, block: &str, value_id: ValueId) -> bool;
}
```

**Algorithm**: Backward dataflow with worklist (see research.md section 5).

**Invariants**:
- `live_out[exit_block]` contains only return value (if any)
- All ValueId in `live_in`/`live_out` are defined in function

---

### 2.5 ConstantLattice

**Purpose**: Tracks constant values via abstract interpretation (used by SCCP).

**Structure**:
```rust
pub enum ConstantLattice {
    Top,                          // Uninitialized or unknown
    Constant(IrLiteralValue),     // Known constant
    Bottom,                       // Overdefined (multiple values)
}

pub struct ConstantLatticeMap {
    /// Value → lattice state
    pub lattice_values: HashMap<ValueId, ConstantLattice>,
}

impl ConstantLattice {
    /// Lattice meet operation
    pub fn meet(&self, other: &Self) -> Self;
}
```

**Lattice Properties**:
- Partial order: Top ⊑ Constant(c) ⊑ Bottom
- Meet operation: `Top ⊔ x = x`, `Constant(c1) ⊔ Constant(c2) = if c1 == c2 then Constant(c1) else Bottom`, `Bottom ⊔ x = Bottom`
- Monotonic: repeated meet operations move down lattice (Top → Constant → Bottom)

**Invariants**:
- Once a value reaches Bottom, it cannot go back to Constant or Top
- Constants in lattice must be valid IrLiteralValue

---

### 2.6 AliasAnalysis Trait

**Purpose**: Interface for querying pointer aliasing relationships (may two pointers refer to same memory?).

**Structure**:
```rust
pub trait AliasAnalysis {
    /// Returns true if v1 and v2 may alias
    fn may_alias(&self, v1: &Value, v2: &Value) -> bool;
    
    /// Returns set of abstract locations value may point to
    fn points_to_set(&self, value: &Value) -> &HashSet<AbstractLocation>;
}

pub enum AbstractLocation {
    Stack(AllocationSite),     // Stack allocation (alloca)
    Heap(AllocationSite),      // Heap allocation (malloc/new)
    Global(String),            // Global variable
    Unknown,                   // External pointer or unknown origin
}

pub struct AllocationSite {
    pub function_name: String,
    pub block_label: String,
    pub instruction_index: usize,
}
```

**Implementations**:

**ConservativeAnalysis** (O0/O1):
```rust
pub struct ConservativeAnalysis;

impl AliasAnalysis for ConservativeAnalysis {
    fn may_alias(&self, v1: &Value, v2: &Value) -> bool {
        // Returns true unless provably distinct (e.g., different allocas)
    }
}
```

**AndersenAnalysis** (O2/O3):
```rust
pub struct AndersenAnalysis {
    /// Value → set of locations it may point to
    pub points_to_sets: HashMap<ValueId, HashSet<AbstractLocation>>,
    
    /// Constraints extracted from IR
    pub constraints: Vec<Constraint>,
}

pub enum Constraint {
    Copy(ValueId, ValueId),              // a = b
    Store(ValueId, ValueId),              // *a = b
    Load(ValueId, ValueId),               // a = *b
}

impl AliasAnalysis for AndersenAnalysis {
    fn may_alias(&self, v1: &Value, v2: &Value) -> bool {
        !self.points_to_sets[&v1.id].is_disjoint(&self.points_to_sets[&v2.id])
    }
}
```

**Invariants**:
- `may_alias` is symmetric: `may_alias(v1, v2) == may_alias(v2, v1)`
- `may_alias` is reflexive: `may_alias(v, v) == true`
- Conservative: false negatives OK (saying may-alias when doesn't), false positives NEVER (saying no-alias when does alias would break correctness)

---

### 2.7 LoopInfo

**Purpose**: Identifies natural loops, computes nesting depth, detects induction variables.

**Structure**:
```rust
pub struct LoopInfo {
    /// Header block label
    pub header: String,
    
    /// All blocks in loop (including header)
    pub members: HashSet<String>,
    
    /// Blocks that exit loop (have successors outside loop)
    pub exits: Vec<String>,
    
    /// Parent loop header (if nested)
    pub parent: Option<String>,
    
    /// Nesting depth (0 for outermost)
    pub depth: usize,
}

pub struct LoopMetadata {
    /// Loops detected in function
    pub loops: HashMap<String, LoopInfo>,  // Header label → LoopInfo
}

impl LoopMetadata {
    /// Detects natural loops via back-edge identification
    pub fn compute(function: &Function) -> Self;
    
    /// Returns loop containing block (if any)
    pub fn loop_containing_block(&self, block: &str) -> Option<&LoopInfo>;
}
```

**Algorithm**: Back-edge detection (see research.md section 8).

**Invariants**:
- Header dominates all members (guaranteed by back-edge definition)
- Members form connected subgraph containing header
- No cycles between loops (parent relation forms forest)

---

### 2.8 GlobalValueNumbering

**Purpose**: Assigns unique identifiers to equivalent expressions for CSE.

**Structure**:
```rust
pub struct GlobalValueNumbering {
    /// Expression hash → canonical ValueId
    pub expression_map: HashMap<ExpressionHash, ValueId>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ExpressionHash {
    /// Instruction kind (Add, Sub, Load, etc.)
    pub kind: InstructionKind,
    
    /// Operand ValueIds (sorted for commutative ops)
    pub operands: Vec<ValueId>,
    
    /// Result type
    pub ty: IrType,
}

impl ExpressionHash {
    /// Creates hash from instruction (None if side effects)
    pub fn from_instruction(inst: &Instruction) -> Option<Self>;
}

impl GlobalValueNumbering {
    /// Computes value numbering for function
    pub fn compute(function: &Function) -> Self;
    
    /// Looks up existing computation with same hash
    pub fn lookup(&self, hash: &ExpressionHash) -> Option<ValueId>;
}
```

**Invariants**:
- Commutative operations have sorted operands (canonical form)
- Only side-effect-free instructions in `expression_map`
- All ValueIds in `expression_map` are valid definitions in function

---

## 3. Transformation Passes

### 3.1 OptimizationPass Trait

**Purpose**: Common interface for all transformation passes.

**Structure**:
```rust
pub trait OptimizationPass {
    /// Name of pass (for logging and metrics)
    fn name(&self) -> &'static str;
    
    /// Required analyses (computed if not cached)
    fn required_analyses(&self) -> &'static [AnalysisKind];
    
    /// Analyses invalidated by this pass (removed from cache)
    fn invalidated_analyses(&self) -> &'static [AnalysisKind];
    
    /// Runs pass on function, returns result with metrics
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult;
}

pub struct PassResult {
    /// Whether IR was modified
    pub changed: bool,
    
    /// Performance metrics
    pub metrics: PassMetrics,
}
```

**Example Implementation** (SCCP):
```rust
pub struct Sccp;

impl OptimizationPass for Sccp {
    fn name(&self) -> &'static str { "SCCP" }
    
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::Dominance, AnalysisKind::UseDef]
    }
    
    fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::ReachingDefs, AnalysisKind::LiveVars, AnalysisKind::Constants]
    }
    
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
        // SCCP algorithm implementation
    }
}
```

---

### 3.2 PassMetrics

**Purpose**: Tracks performance and effectiveness metrics for each pass.

**Structure**:
```rust
pub struct PassMetrics {
    /// Instructions eliminated
    pub instructions_eliminated: usize,
    
    /// Constants propagated
    pub constants_propagated: usize,
    
    /// CSE hits (expressions replaced with existing values)
    pub cse_hits: usize,
    
    /// Phi nodes removed
    pub phi_nodes_removed: usize,
    
    /// Basic blocks removed
    pub blocks_removed: usize,
    
    /// Pass execution time
    pub elapsed: Duration,
}
```

**Relationships**:
- Returned by `OptimizationPass::run()` in `PassResult`
- Aggregated by `PassManager` into `OptimizerReport`

---

## 4. Verification and Error Handling

### 4.1 VerificationError

**Purpose**: Represents SSA/CFG/type consistency violations detected by verification.

**Structure**:
```rust
pub enum VerificationError {
    DuplicateDefinition {
        value_id: ValueId,
        block: String,
    },
    UseBeforeDefinition {
        value_id: ValueId,
        block: String,
    },
    PhiPredecessorMismatch {
        block: String,
        expected: usize,
        actual: usize,
    },
    InvalidPhiPredecessor {
        block: String,
        pred: String,
    },
    EntryBlockHasPredecessors,
    UnreachableBlocks {
        count: usize,
    },
    InvalidTerminatorTarget {
        block: String,
        target: String,
    },
    UnreachableTerminator {
        block: String,
    },
    BinaryOperandTypeMismatch {
        block: String,
        lhs_ty: IrType,
        rhs_ty: IrType,
    },
    BinaryResultTypeMismatch {
        block: String,
        operand_ty: IrType,
        result_ty: IrType,
    },
    LoadFromNonPointer {
        block: String,
        ty: IrType,
    },
    PhiTypeMismatch {
        block: String,
        expected: IrType,
        actual: IrType,
    },
}
```

**Usage**: Returned by verification functions, triggers rollback in `run_pass_with_verification`.

---

### 4.2 FunctionSnapshot

**Purpose**: Captures function state for rollback on verification failure.

**Structure**:
```rust
pub struct FunctionSnapshot {
    /// Cloned basic blocks
    pub blocks: Vec<BasicBlock>,
    
    /// CFG edges (src, dst)
    pub edges: Vec<(String, String)>,
    
    /// Use-def chains
    pub use_def_chains: HashMap<ValueId, InstructionRef>,
    
    /// Def-use chains
    pub def_use_chains: HashMap<ValueId, Vec<InstructionRef>>,
}

impl FunctionSnapshot {
    /// Captures current function state
    pub fn capture(function: &Function, use_def_mgr: &UseDefManager) -> Self;
    
    /// Restores function to captured state
    pub fn restore(&self, function: &mut Function, use_def_mgr: &mut UseDefManager);
}
```

**Invariants**:
- Snapshot is immutable after capture
- Restore produces exact state at time of capture
- Edges reference blocks in `blocks` vector

---

### 4.3 OptimizerError

**Purpose**: Top-level error type for optimizer failures.

**Structure**:
```rust
pub enum OptimizerError {
    VerificationFailed {
        pass: String,
        message: String,
    },
    AnalysisFailed {
        analysis: String,
        message: String,
    },
    PassError {
        pass: String,
        message: String,
    },
}
```

**Usage**: Returned by `run_pass_with_verification`, logged by `PassManager`.

---

## 5. Reporting and Metrics

### 5.1 OptimizerReport

**Purpose**: Summarizes optimization results for entire module.

**Structure**:
```rust
pub struct OptimizerReport {
    /// Per-pass metrics
    pub per_pass_metrics: Vec<(String, PassMetrics)>,
    
    /// Aggregate metrics across all passes
    pub aggregate_metrics: AggregateMetrics,
}

pub struct AggregateMetrics {
    /// Total instructions eliminated
    pub total_instructions_eliminated: usize,
    
    /// Instruction count before optimization
    pub instruction_count_before: usize,
    
    /// Instruction count after optimization
    pub instruction_count_after: usize,
    
    /// Reduction percentage
    pub reduction_percentage: f64,
    
    /// CFG complexity (block count, edge count)
    pub cfg_complexity: (usize, usize),
    
    /// Total elapsed time
    pub total_elapsed: Duration,
    
    /// Memory usage (if tracked)
    pub memory_usage: Option<usize>,
}
```

**Relationships**:
- Produced by `PassManager::run_optimization_pipeline()`
- Consumed by main compiler driver for logging/reporting

---

## 6. Integration with Existing IR

### 6.1 Module (Existing)

**Relevant Fields**:
```rust
pub struct Module {
    pub functions: Vec<Function>,
    // ... other fields
}
```

**Optimizer Integration**:
```rust
impl Module {
    /// Optimizes all functions in module
    pub fn optimize(&mut self, config: OptimizerConfig) -> OptimizerReport {
        let mut pass_mgr = PassManager::new(config);
        pass_mgr.run_optimization_pipeline(self)
    }
}
```

---

### 6.2 Function (Existing)

**Relevant Fields**:
```rust
pub struct Function {
    pub name: String,
    pub cfg: ControlFlowGraph,
    pub dominance: DominanceInfo,
    // ... other fields
}
```

**Optimizer Requirements**:
- `cfg` must be valid (entry block, reachable blocks, valid terminators)
- `dominance` must be computed (via `DominanceInfo::compute(&cfg)`)
- SSA form must be established (via `SsaTransformer::transform()`)

---

### 6.3 Value (Existing)

**Relevant Fields**:
```rust
pub struct Value {
    pub id: ValueId,
    pub kind: ValueKind,
    pub ty: IrType,
    pub debug_info: Option<ValueDebugInfo>,
    // ... other fields
}

pub type ValueId = Uuid;
```

**Optimizer Usage**:
- `id` used as key in use-def chains, lattice maps, points-to sets
- `kind` distinguishes constants, temporaries, locals for analysis
- `ty` used for type consistency verification

---

## 7. Entity Relationships Diagram

```
┌─────────────────┐
│ OptimizerConfig │
│  - opt_level    │
│  - passes       │
└────────┬────────┘
         │ contains
         ▼
┌──────────────────┐      uses      ┌───────────────────┐
│   PassManager    │◄──────────────►│ AnalysisManager   │
│  - config        │                │  - cache          │
│  - analysis_mgr  │                │  - dependencies   │
└────────┬─────────┘                └─────────┬─────────┘
         │ runs                               │ provides
         ▼                                    ▼
┌──────────────────┐      requires    ┌──────────────────┐
│ OptimizationPass │◄─────────────────│    Analysis      │
│  - run()         │                  │  - compute()     │
│  - metrics       │                  │  - invalidate()  │
└────────┬─────────┘                  └──────────────────┘
         │ produces                            △
         ▼                                     │
┌──────────────────┐                          │ implements
│   PassResult     │                          │
│  - changed       │                 ┌────────┴────────┐
│  - metrics       │                 │                 │
└──────────────────┘           ┌─────┴─────┐   ┌──────┴──────┐
                               │  UseDef   │   │ LiveVars    │
                               │  Manager  │   │             │
                               └───────────┘   └─────────────┘
                               
                               ┌─────────────┐   ┌─────────────┐
                               │ LoopInfo    │   │ AliasAnalysis│
                               │             │   │             │
                               └─────────────┘   └─────────────┘
```

---

## 8. State Transitions

### 8.1 Pass Execution State

```
┌─────────────┐
│   READY     │ Initial state: pass registered, analyses may be cached
└──────┬──────┘
       │ PassManager calls run()
       ▼
┌─────────────┐
│  CAPTURING  │ FunctionSnapshot captures pre-pass state
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  RUNNING    │ Pass modifies IR, collects metrics
└──────┬──────┘
       │
       ├───────────────┐
       │               │ changed = true
       ▼               ▼
┌─────────────┐  ┌────────────┐
│ VERIFYING   │  │  COMPLETE  │ No changes, skip verification
└──────┬──────┘  └────────────┘
       │
       ├─────────────┐
       │ valid       │ invalid
       ▼             ▼
┌─────────────┐  ┌────────────┐
│  COMPLETE   │  │ ROLLBACK   │ Restore snapshot, report error
└─────────────┘  └──────┬─────┘
                        │
                        ▼
                 ┌────────────┐
                 │   ERROR    │ OptimizerError logged
                 └────────────┘
```

---

## 9. Invariants and Constraints

### 9.1 Global Invariants (maintained across all passes)

1. **SSA Form**:
   - Each temporary (ValueKind::Temporary) defined exactly once
   - All uses dominated by definition
   - Phi nodes have exactly one incoming value per predecessor

2. **CFG Consistency**:
   - Entry block has no predecessors
   - All blocks reachable from entry
   - All terminator targets are valid block labels
   - No unreachable terminators (should be removed by DCE)

3. **Type Consistency**:
   - Binary operation operands have matching types
   - Phi incoming values have type matching phi result type
   - Load operations have pointer-typed operands

4. **Debug Information Preservation**:
   - SourceSpan preserved for >=90% of instructions that survive optimization
   - Instruction::debug_info copied when creating derived instructions

### 9.2 Analysis Invariants

1. **Use-Def Chains**:
   - Each use has at most one definition (SSA guarantee)
   - InstructionRef references valid instructions

2. **Dataflow Analysis**:
   - Reaching definitions: monotonic (sets only grow) until fixed point
   - Live variables: monotonic (sets only grow backwards) until fixed point

3. **Alias Analysis**:
   - Conservative: no false negatives (may say may-alias when doesn't, never says no-alias when does)
   - Symmetric: may_alias(v1, v2) = may_alias(v2, v1)

4. **Loop Detection**:
   - Headers dominate all loop members
   - Back edges: target dominates source

### 9.3 Pass-Specific Invariants

**SCCP**:
- Lattice values monotonically descend (Top → Constant → Bottom)
- Executable edges subset grows monotonically

**ADCE**:
- Marked instructions form transitive closure from anchors
- Unmarked instructions have no live uses

**GVN/CSE**:
- Expression map entries reference dominating definitions
- Replaced values dominated by replacement values

**LICM**:
- Hoisted instructions have no loop-variant operands
- Hoisted instructions have no memory dependencies on loop body

---

## Conclusion

This data model provides complete specification of all structures, relationships, and invariants for the multi-pass IR optimizer. All entities are designed for:

1. **Correctness**: Invariants ensure semantic preservation
2. **Efficiency**: Data structures optimized for common operations (O(1) lookups, cache-friendly iteration)
3. **Maintainability**: Clear separation of concerns, trait-based extensibility
4. **Safety**: Rust ownership prevents data races, use-after-free, null pointer dereferences

Next steps: Implement contracts (trait definitions) and quickstart guide showing integration with compilation pipeline.
