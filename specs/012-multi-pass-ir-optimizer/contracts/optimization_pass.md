# Contract: OptimizationPass Trait

**Purpose**: Defines the interface that all optimization passes must implement, enabling uniform integration with the pass manager and analysis framework.

## Trait Definition

```rust
/// Trait implemented by all optimization passes
/// 
/// Optimization passes transform IR while preserving semantics. Each pass
/// declares its analysis dependencies and reports which analyses it invalidates.
/// 
/// # Example
/// 
/// ```rust
/// pub struct Sccp;
/// 
/// impl OptimizationPass for Sccp {
///     fn name(&self) -> &'static str {
///         "SCCP"
///     }
///     
///     fn required_analyses(&self) -> &'static [AnalysisKind] {
///         &[AnalysisKind::Dominance, AnalysisKind::UseDef]
///     }
///     
///     fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
///         &[AnalysisKind::ReachingDefs, AnalysisKind::Constants]
///     }
///     
///     fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
///         let start = Instant::now();
///         let dom_info = analysis_mgr.get_analysis::<DominanceInfo>(function, AnalysisKind::Dominance);
///         let use_def = analysis_mgr.get_analysis::<UseDefManager>(function, AnalysisKind::UseDef);
///         
///         // SCCP algorithm implementation
///         let (changed, metrics) = self.run_sccp(function, dom_info, use_def);
///         
///         PassResult {
///             changed,
///             metrics: PassMetrics {
///                 constants_propagated: metrics.constants_propagated,
///                 instructions_eliminated: metrics.instructions_eliminated,
///                 blocks_removed: metrics.blocks_removed,
///                 elapsed: start.elapsed(),
///                 ..Default::default()
///             },
///         }
///     }
/// }
/// ```
pub trait OptimizationPass {
    /// Returns the name of the pass (for logging and metrics reporting)
    /// 
    /// # Returns
    /// 
    /// A static string identifying this pass (e.g., "SCCP", "ADCE", "GVN")
    fn name(&self) -> &'static str;
    
    /// Returns the list of analyses required by this pass
    /// 
    /// The pass manager ensures these analyses are computed (or retrieved from
    /// cache) before calling `run()`. Analyses are provided via `AnalysisManager`.
    /// 
    /// # Returns
    /// 
    /// Slice of `AnalysisKind` values representing required analyses
    /// 
    /// # Example
    /// 
    /// ```rust
    /// fn required_analyses(&self) -> &'static [AnalysisKind] {
    ///     &[AnalysisKind::Dominance, AnalysisKind::UseDef, AnalysisKind::Alias]
    /// }
    /// ```
    fn required_analyses(&self) -> &'static [AnalysisKind];
    
    /// Returns the list of analyses invalidated by this pass
    /// 
    /// When the pass modifies IR (`PassResult::changed = true`), the pass manager
    /// removes these analyses from the cache, forcing recomputation on next access.
    /// 
    /// # Returns
    /// 
    /// Slice of `AnalysisKind` values representing invalidated analyses
    /// 
    /// # Example
    /// 
    /// ```rust
    /// fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
    ///     // SCCP changes constants and may remove blocks
    ///     &[AnalysisKind::ReachingDefs, AnalysisKind::Constants, AnalysisKind::LiveVars]
    /// }
    /// ```
    fn invalidated_analyses(&self) -> &'static [AnalysisKind];
    
    /// Runs the optimization pass on the given function
    /// 
    /// This is the main entry point for pass execution. The pass may:
    /// - Query required analyses via `analysis_mgr`
    /// - Modify the function's IR (instructions, blocks, CFG)
    /// - Track metrics (instructions eliminated, constants propagated, etc.)
    /// 
    /// # Parameters
    /// 
    /// - `function`: Mutable reference to function to optimize
    /// - `analysis_mgr`: Analysis manager providing access to required analyses
    /// 
    /// # Returns
    /// 
    /// `PassResult` indicating whether IR was modified and performance metrics
    /// 
    /// # Panics
    /// 
    /// Should not panic. Return errors via `PassResult` or log warnings.
    /// 
    /// # Safety
    /// 
    /// Must preserve SSA form, CFG consistency, and type correctness. Verification
    /// will check invariants after pass execution; failures trigger rollback.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
    ///     let start = Instant::now();
    ///     let mut changed = false;
    ///     let mut metrics = PassMetrics::default();
    ///     
    ///     // Get required analyses
    ///     let use_def = analysis_mgr.get_analysis::<UseDefManager>(function, AnalysisKind::UseDef);
    ///     
    ///     // Perform optimization
    ///     for block in function.cfg.blocks.values_mut() {
    ///         for instruction in &mut block.instructions {
    ///             if self.can_optimize(instruction, use_def) {
    ///                 self.optimize_instruction(instruction);
    ///                 changed = true;
    ///                 metrics.instructions_eliminated += 1;
    ///             }
    ///         }
    ///     }
    ///     
    ///     metrics.elapsed = start.elapsed();
    ///     PassResult { changed, metrics }
    /// }
    /// ```
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult;
}
```

## Supporting Types

### PassResult

```rust
/// Result of optimization pass execution
/// 
/// Contains boolean flag indicating whether IR was modified and performance
/// metrics tracking pass effectiveness.
#[derive(Debug, Clone)]
pub struct PassResult {
    /// Whether the pass modified the IR
    /// 
    /// If `true`, the pass manager:
    /// - Runs verification to ensure correctness
    /// - Invalidates analyses declared in `invalidated_analyses()`
    /// - Continues iterating toward fixed point
    /// 
    /// If `false`, the pass manager:
    /// - Skips verification (no changes to verify)
    /// - Keeps analyses cached (no invalidation needed)
    pub changed: bool,
    
    /// Performance and effectiveness metrics
    pub metrics: PassMetrics,
}
```

### PassMetrics

```rust
/// Performance metrics for optimization pass
/// 
/// Tracks effectiveness (instructions eliminated, constants propagated) and
/// performance (execution time, memory usage).
#[derive(Debug, Clone, Default)]
pub struct PassMetrics {
    /// Number of instructions eliminated by this pass
    pub instructions_eliminated: usize,
    
    /// Number of constants propagated (SCCP, constant folding)
    pub constants_propagated: usize,
    
    /// Number of common subexpressions eliminated (GVN/CSE)
    pub cse_hits: usize,
    
    /// Number of phi nodes removed (phi optimization)
    pub phi_nodes_removed: usize,
    
    /// Number of basic blocks removed (DCE, unreachable code elimination)
    pub blocks_removed: usize,
    
    /// Pass execution time
    pub elapsed: Duration,
}
```

### AnalysisKind

```rust
/// Enumeration of available analyses
/// 
/// Used by passes to declare dependencies and invalidation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisKind {
    /// Reaching definitions dataflow analysis
    ReachingDefs,
    
    /// Live variables dataflow analysis
    LiveVars,
    
    /// Use-def and def-use chains
    UseDef,
    
    /// Dominator tree and dominance frontiers
    Dominance,
    
    /// Natural loop detection and metadata
    Loops,
    
    /// Constant lattice (SCCP)
    Constants,
    
    /// Alias analysis (points-to sets)
    Alias,
    
    /// Global value numbering (expression hashing)
    ValueNumbering,
}
```

## Contract Guarantees

### Implementer Responsibilities

1. **Correctness**:
   - Must preserve SSA form (no duplicate definitions, all uses dominated by defs)
   - Must maintain CFG consistency (valid terminators, reachable blocks)
   - Must preserve type correctness (operand types match instruction requirements)

2. **Honesty**:
   - Must accurately report `changed = true` when IR is modified
   - Must declare all required analyses (no undeclared dependencies)
   - Must declare all invalidated analyses (no stale cache entries)

3. **Efficiency**:
   - Should complete in reasonable time (no infinite loops)
   - Should track metrics accurately for performance evaluation

4. **Error Handling**:
   - Should not panic (use Result or log warnings)
   - Should handle malformed IR gracefully (defensive programming)

### Pass Manager Guarantees

1. **Analysis Availability**:
   - All analyses in `required_analyses()` are computed before `run()`
   - Analyses are retrieved from cache if valid, recomputed if invalidated

2. **Verification**:
   - SSA form, CFG consistency, and type correctness verified after pass if `changed = true`
   - Verification failures trigger automatic rollback to pre-pass state

3. **Invalidation**:
   - Analyses in `invalidated_analyses()` removed from cache after pass if `changed = true`
   - Dependent analyses transitively invalidated

4. **Metrics**:
   - PassMetrics aggregated into OptimizerReport
   - Per-pass metrics available for debugging and tuning

## Common Pass Patterns

### 1. Simple Transformation Pass

```rust
pub struct CopyPropagation;

impl OptimizationPass for CopyPropagation {
    fn name(&self) -> &'static str { "CopyPropagation" }
    
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::UseDef]
    }
    
    fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::UseDef]  // We modify use-def chains
    }
    
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
        let use_def = analysis_mgr.get_analysis::<UseDefManager>(function, AnalysisKind::UseDef);
        // ... implementation
    }
}
```

### 2. Analysis-Heavy Pass

```rust
pub struct Licm;

impl OptimizationPass for Licm {
    fn name(&self) -> &'static str { "LICM" }
    
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        &[
            AnalysisKind::Dominance,
            AnalysisKind::Loops,
            AnalysisKind::UseDef,
            AnalysisKind::Alias,  // Memory dependency analysis
        ]
    }
    
    fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
        &[AnalysisKind::UseDef, AnalysisKind::Dominance]  // CFG changes
    }
    
    fn run(&self, function: &mut Function, analysis_mgr: &AnalysisManager) -> PassResult {
        let loops = analysis_mgr.get_analysis::<LoopMetadata>(function, AnalysisKind::Loops);
        let alias = analysis_mgr.get_analysis::<dyn AliasAnalysis>(function, AnalysisKind::Alias);
        // ... implementation
    }
}
```

### 3. Conservative Pass (No Invalidation)

```rust
pub struct AlgebraicSimplification;

impl OptimizationPass for AlgebraicSimplification {
    fn name(&self) -> &'static str { "AlgebraicSimplification" }
    
    fn required_analyses(&self) -> &'static [AnalysisKind] {
        &[]  // No analyses required
    }
    
    fn invalidated_analyses(&self) -> &'static [AnalysisKind] {
        &[]  // Replaces instructions in-place, doesn't change structure
    }
    
    fn run(&self, function: &mut Function, _analysis_mgr: &AnalysisManager) -> PassResult {
        // Apply identity laws: x+0→x, x*1→x, etc.
        // No CFG changes, no use-def changes (just value replacement)
    }
}
```

## Version History

- **1.0** (2025-11-01): Initial contract definition
