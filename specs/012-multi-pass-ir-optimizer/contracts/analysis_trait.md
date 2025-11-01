# Contract: Analysis Trait

**Purpose**: Defines the interface for all IR analyses, enabling uniform caching, invalidation, and query mechanisms through the AnalysisManager.

## Trait Definition

```rust
/// Trait implemented by all IR analyses
/// 
/// Analyses compute information about functions (dataflow, control flow, aliasing)
/// without modifying IR. The AnalysisManager caches analysis results and invalidates
/// them when passes modify IR.
/// 
/// # Example
/// 
/// ```rust
/// pub struct UseDefManager {
///     pub use_def: HashMap<ValueId, InstructionRef>,
///     pub def_use: HashMap<ValueId, Vec<InstructionRef>>,
/// }
/// 
/// impl Analysis for UseDefManager {
///     fn compute(function: &Function) -> Self {
///         let mut use_def = HashMap::new();
///         let mut def_use = HashMap::new();
///         
///         for (block_label, block) in &function.cfg.blocks {
///             for (index, instruction) in block.instructions.iter().enumerate() {
///                 let inst_ref = InstructionRef {
///                     block_label: block_label.clone(),
///                     index,
///                 };
///                 
///                 // Register definition
///                 if let Some(result) = instruction.result_value() {
///                     def_use.entry(result.id).or_insert_with(Vec::new);
///                 }
///                 
///                 // Register uses
///                 for operand in instruction.operands() {
///                     use_def.insert(operand.id, inst_ref.clone());
///                     def_use.entry(operand.id)
///                         .or_insert_with(Vec::new)
///                         .push(inst_ref.clone());
///                 }
///             }
///         }
///         
///         UseDefManager { use_def, def_use }
///     }
///     
///     fn invalidate(&mut self) {
///         self.use_def.clear();
///         self.def_use.clear();
///     }
/// }
/// ```
pub trait Analysis: Sized {
    /// Computes analysis from function
    /// 
    /// This method is called by AnalysisManager when analysis is not cached or
    /// has been invalidated. Should scan function IR and build analysis data structures.
    /// 
    /// # Parameters
    /// 
    /// - `function`: Function to analyze (immutable reference, no IR modification)
    /// 
    /// # Returns
    /// 
    /// Self (analysis result with computed data)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// fn compute(function: &Function) -> Self {
    ///     let mut reaching_in = HashMap::new();
    ///     let mut reaching_out = HashMap::new();
    ///     
    ///     // Initialize all blocks
    ///     for block_label in function.cfg.blocks.keys() {
    ///         reaching_in.insert(block_label.clone(), BitVec::new());
    ///         reaching_out.insert(block_label.clone(), BitVec::new());
    ///     }
    ///     
    ///     // Worklist algorithm for fixed-point computation
    ///     let mut worklist = VecDeque::from_iter(function.cfg.blocks.keys().cloned());
    ///     while let Some(block) = worklist.pop_front() {
    ///         // Dataflow transfer function
    ///         // ... (see research.md for details)
    ///     }
    ///     
    ///     ReachingDefinitions { reaching_in, reaching_out, /* ... */ }
    /// }
    /// ```
    fn compute(function: &Function) -> Self;
    
    /// Invalidates cached analysis state
    /// 
    /// Called by AnalysisManager when pass modifies IR and declares this analysis
    /// in `invalidated_analyses()`. Should clear internal state to prevent stale data.
    /// 
    /// After invalidation, next call to `AnalysisManager::get_analysis()` will invoke
    /// `compute()` to rebuild analysis.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// fn invalidate(&mut self) {
    ///     self.reaching_in.clear();
    ///     self.reaching_out.clear();
    ///     self.def_index.clear();
    ///     self.value_to_index.clear();
    /// }
    /// ```
    fn invalidate(&mut self);
}
```

## Analysis Implementations

### 1. UseDefManager

```rust
/// Maintains use-def and def-use chains for O(1) queries
/// 
/// In SSA form:
/// - Each value has exactly one definition (use_def: ValueId → InstructionRef)
/// - Each value may have multiple uses (def_use: ValueId → Vec<InstructionRef>)
pub struct UseDefManager {
    /// Use → definition mapping
    pub use_def: HashMap<ValueId, InstructionRef>,
    
    /// Definition → uses mapping
    pub def_use: HashMap<ValueId, Vec<InstructionRef>>,
}

impl UseDefManager {
    /// Returns definition of value (O(1))
    pub fn get_definition(&self, value_id: ValueId) -> Option<&InstructionRef>;
    
    /// Returns uses of value (O(1) + O(k) where k = use count)
    pub fn get_uses(&self, value_id: ValueId) -> &[InstructionRef];
}
```

### 2. ReachingDefinitions

```rust
/// Dataflow analysis: which definitions reach each block
/// 
/// Used by constant propagation to determine if value is constant at use site.
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
    /// Returns definitions reaching block entry
    pub fn reaching_at_block_entry(&self, block: &str) -> impl Iterator<Item = ValueId> + '_;
}
```

### 3. LiveVariables

```rust
/// Backward dataflow analysis: which variables are live at each point
/// 
/// Used by dead code elimination to identify unused instructions.
pub struct LiveVariables {
    /// Variables live at block entry
    pub live_in: HashMap<String, HashSet<ValueId>>,
    
    /// Variables live at block exit
    pub live_out: HashMap<String, HashSet<ValueId>>,
}

impl LiveVariables {
    /// Checks if value is live at block entry
    pub fn is_live_at_entry(&self, block: &str, value_id: ValueId) -> bool;
    
    /// Checks if value is live at block exit
    pub fn is_live_at_exit(&self, block: &str, value_id: ValueId) -> bool;
}
```

### 4. LoopMetadata

```rust
/// Natural loop detection and metadata
/// 
/// Identifies loops via back-edge analysis, computes nesting depth.
pub struct LoopMetadata {
    /// Detected loops (header label → LoopInfo)
    pub loops: HashMap<String, LoopInfo>,
}

pub struct LoopInfo {
    /// Loop header block label
    pub header: String,
    
    /// All blocks in loop
    pub members: HashSet<String>,
    
    /// Exit blocks (have successors outside loop)
    pub exits: Vec<String>,
    
    /// Parent loop header (if nested)
    pub parent: Option<String>,
    
    /// Nesting depth (0 for outermost)
    pub depth: usize,
}

impl LoopMetadata {
    /// Returns loop containing block (if any)
    pub fn loop_containing_block(&self, block: &str) -> Option<&LoopInfo>;
}
```

### 5. GlobalValueNumbering

```rust
/// Expression hashing for common subexpression elimination
/// 
/// Assigns canonical ValueId to each unique expression.
pub struct GlobalValueNumbering {
    /// Expression hash → canonical ValueId
    pub expression_map: HashMap<ExpressionHash, ValueId>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ExpressionHash {
    /// Instruction kind
    pub kind: InstructionKind,
    
    /// Operand ValueIds (sorted for commutative ops)
    pub operands: Vec<ValueId>,
    
    /// Result type
    pub ty: IrType,
}

impl GlobalValueNumbering {
    /// Looks up canonical value for expression
    pub fn lookup(&self, hash: &ExpressionHash) -> Option<ValueId>;
}
```

## Contract Guarantees

### Implementer Responsibilities

1. **Correctness**:
   - `compute()` must produce accurate analysis results based on current IR state
   - Results must be consistent with IR semantics (SSA, CFG, dominance)

2. **Completeness**:
   - `compute()` must analyze entire function (all blocks, instructions)
   - No partial analysis (all query methods should have valid answers)

3. **Efficiency**:
   - `compute()` should complete in reasonable time (O(n) to O(n³) typical)
   - `invalidate()` should be fast (O(1) to O(n))

4. **Idempotence**:
   - Multiple calls to `compute()` on unchanged function produce identical results
   - `invalidate()` followed by `compute()` produces fresh valid state

### AnalysisManager Guarantees

1. **Caching**:
   - Analysis computed at most once per function between invalidations
   - Cached result returned on subsequent `get_analysis()` calls

2. **Invalidation**:
   - Analysis invalidated when declared in pass's `invalidated_analyses()`
   - Transitive invalidation: dependent analyses also invalidated

3. **Thread Safety**:
   - Single-threaded access guaranteed (no concurrent modification)
   - Future: may support parallel analyses with locking

## Usage Patterns

### 1. Simple Analysis (No Dependencies)

```rust
impl Analysis for UseDefManager {
    fn compute(function: &Function) -> Self {
        // Scan function once, build maps
        // No dependencies on other analyses
    }
    
    fn invalidate(&mut self) {
        // Clear maps
    }
}
```

### 2. Dependent Analysis

```rust
impl LoopMetadata {
    /// Requires dominance info (not in Analysis trait, but used internally)
    fn compute_with_dominance(function: &Function, dom_info: &DominanceInfo) -> Self {
        // Detect back edges using dominance
        // Build LoopInfo structures
    }
}

impl Analysis for LoopMetadata {
    fn compute(function: &Function) -> Self {
        // Must compute or assume dominance available
        let dom_info = DominanceInfo::compute(&function.cfg);
        Self::compute_with_dominance(function, &dom_info)
    }
    
    fn invalidate(&mut self) {
        self.loops.clear();
    }
}
```

### 3. Incremental Analysis (Future Extension)

```rust
// Not yet implemented, but trait extensible for incremental updates
trait IncrementalAnalysis: Analysis {
    fn update(&mut self, function: &Function, changes: &[Change]);
}
```

## Version History

- **1.0** (2025-11-01): Initial contract definition
