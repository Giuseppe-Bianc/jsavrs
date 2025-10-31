# Optimizer API Contract

## Module-level API

### `optimize_module`
```rust
pub fn optimize_module(module: &mut Module, config: OptimizerConfig) -> Result<(OptimizerReport, PassMetrics), OptimizerError>
```

**Description**: Main entry point for the optimizer that applies optimization passes to all functions in a module.

**Parameters**:
- `module: &mut Module` - The module to optimize (mutable reference since it's modified)
- `config: OptimizerConfig` - Configuration controlling optimization behavior

**Returns**: 
- `Ok((OptimizerReport, PassMetrics))` - On success, returns optimization report and metrics
- `Err(OptimizerError)` - On failure during optimization

**Pre-conditions**:
- Module must be in valid SSA form
- All functions must have valid CFG and dominance information
- Module must be well-typed

**Post-conditions**:
- Module is semantically equivalent to input (preserves observable behavior)
- Module may have fewer instructions than input
- Debug information is preserved for remaining instructions

## Configuration API

### `OptimizerConfig`
```rust
pub struct OptimizerConfig {
    pub opt_level: OptLevel,
    pub max_iterations: usize,
    pub loop_unroll_threshold: usize,
    pub alias_analysis_kind: AliasAnalysisKind,
    pub early_passes: Vec<Box<dyn OptimizationPass>>,
    pub middle_passes: Vec<Box<dyn OptimizationPass>>,
    pub late_passes: Vec<Box<dyn OptimizationPass>>,
    pub record_provenance: bool,
}
```

### `OptLevel` enum
```rust
pub enum OptLevel {
    O0,  // No optimizations
    O1,  // Basic optimizations
    O2,  // Most optimizations
    O3,  // Aggressive optimizations
}
```

### `config_for_level` function
```rust
impl OptimizerConfig {
    pub fn config_for_level(level: OptLevel) -> Self
}
```

## Analysis Framework API

### `Analysis` trait
```rust
pub trait Analysis<T> {
    type Context;
    
    fn compute(&self, input: &T, context: &Self::Context) -> Self;
    fn invalidate(&mut self);
    // Query methods specific to each analysis type
}
```

### `AnalysisManager`
```rust
pub struct AnalysisManager {
    cached_results: HashMap<(String, AnalysisKind), Box<dyn Any>>,
    dependencies: HashMap<AnalysisKind, Vec<AnalysisKind>>,
}

impl AnalysisManager {
    pub fn get_analysis<T>(&self, function: &Function, kind: AnalysisKind) -> Result<&T, AnalysisError>
    pub fn invalidate(&mut self, kind: AnalysisKind)
    pub fn invalidate_transitive(&mut self, kind: AnalysisKind)
}
```

### `AnalysisKind` enum
```rust
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
```

## Pass API

### `OptimizationPass` trait
```rust
pub trait OptimizationPass {
    fn name(&self) -> &'static str;
    fn required_analyses(&self) -> &'static [AnalysisKind];
    fn invalidated_analyses(&self) -> &'static [AnalysisKind];
    fn run(&mut self, function: &mut Function, analysis_mgr: &AnalysisManager) -> Result<PassResult, PassError>;
}
```

### `PassResult` struct
```rust
pub struct PassResult {
    pub changed: bool,           // Whether the pass modified the function
    pub metrics: PassMetrics,    // Metrics collected during the pass
}
```

### `PassMetrics` struct
```rust
pub struct PassMetrics {
    pub instructions_eliminated: usize,
    pub constants_propagated: usize,
    pub cse_hits: usize,
    pub phi_nodes_removed: usize,
    pub blocks_removed: usize,
    pub elapsed: Duration,
}
```

## Specific Pass APIs

### `PassManager`
```rust
pub struct PassManager {
    config: OptimizerConfig,
}

impl PassManager {
    pub fn run_optimization_pipeline(&mut self, module: &mut Module) -> Result<PassMetrics, OptimizerError>
    pub fn add_pass(&mut self, pass: Box<dyn OptimizationPass>)
    pub fn clear_passes(&mut self)
    pub fn set_config(&mut self, config: OptimizerConfig)
}
```

## Error Handling API

### `OptimizerError` enum
```rust
pub enum OptimizerError {
    VerificationFailed { pass: String, message: String },
    AnalysisFailed { analysis: String, message: String },
    PassError { pass: String, message: String },
    InternalError { message: String },
}
```

## Verification API

### `verify_ssa_form`
```rust
pub fn verify_ssa_form(function: &Function) -> Result<(), VerificationError>
```

### `verify_cfg_consistency`
```rust
pub fn verify_cfg_consistency(function: &Function) -> Result<(), VerificationError>
```

### `verify_type_consistency`
```rust
pub fn verify_type_consistency(function: &Function) -> Result<(), VerificationError>
```

### `verify_and_rollback`
```rust
pub fn verify_and_rollback(
    function: &mut Function, 
    snapshot: &FunctionSnapshot
) -> Result<bool, VerificationError>
```

## Snapshot API

### `FunctionSnapshot`
```rust
pub struct FunctionSnapshot {
    blocks: Vec<BasicBlock>,
    edges: Vec<(String, String)>,
    use_def_chains: HashMap<ValueId, InstructionRef>,
    def_use_chains: HashMap<ValueId, Vec<InstructionRef>>,
}

impl FunctionSnapshot {
    pub fn capture(function: &Function) -> Self
    pub fn restore(&self, function: &mut Function)
}
```

## Analysis-Specific APIs

### `UseDefManager`
```rust
pub struct UseDefManager {
    use_def_chains: HashMap<ValueId, InstructionRef>,
    def_use_chains: HashMap<ValueId, Vec<InstructionRef>>,
}

impl UseDefManager {
    pub fn get_def(&self, value: ValueId) -> Option<&InstructionRef>
    pub fn get_uses(&self, value: ValueId) -> Option<&[InstructionRef]>
    pub fn build_from_function(function: &Function) -> Self
    pub fn update_on_replacement(&mut self, old_inst: &InstructionRef, new_inst: &InstructionRef)
    pub fn update_on_removal(&mut self, inst_ref: &InstructionRef)
}
```

### `AliasAnalysis` trait
```rust
pub trait AliasAnalysis {
    fn may_alias(&self, value1: &Value, value2: &Value) -> Result<bool, AnalysisError>;
    fn points_to_set(&self, value: &Value) -> Result<&HashSet<AbstractLocation>, AnalysisError>;
}

pub enum AliasAnalysisKind {
    Andersen,
    Conservative,
    Custom(Box<dyn AliasAnalysis>),
}
```