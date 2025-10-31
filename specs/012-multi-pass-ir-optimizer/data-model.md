# Data Model: Multi-pass IR Optimizer for jsavrs

## Entity: Module
**Description**: Compiler IR container containing multiple Functions, CFGs, and metadata required by the optimizer
**Fields**:
- functions: Vec<Function> - Collection of functions in the module
- data_layout: DataLayout - Target-specific data layout information
- target_triple: TargetTriple - Target architecture information
- metadata: HashMap<String, IrMetadata> - Additional metadata for optimization
**Relationships**: Contains multiple Function entities
**Validation rules**: Must contain at least one function; functions must have unique names
**State transitions**: Unoptimized → Optimized (via optimize_module function)

## Entity: Function
**Description**: Unit of optimization containing BasicBlocks, Instructions, and Phi nodes
**Fields**:
- name: String - Unique identifier for the function
- parameters: Vec<IrParameter> - Function parameters
- return_type: IrType - Type of value returned by the function
- basic_blocks: Vec<BasicBlock> - Basic blocks forming the control flow graph
- dominance_info: DominanceInfo - Dominator tree information
- attributes: FunctionAttributes - Function-specific attributes
- debug_info: Option<DebugInfo> - Debugging information
**Relationships**: Belongs to a Module; contains multiple BasicBlock entities
**Validation rules**: Must have exactly one entry block; all blocks must be reachable; SSA invariants must be maintained
**State transitions**: Unoptimized → Optimized (via optimization passes)

## Entity: BasicBlock
**Description**: Sequence of instructions ending with a terminator; predecessor/successor lists updated as CFG changes
**Fields**:
- label: String - Unique identifier for the basic block
- instructions: Vec<Instruction> - Instructions in the block (excluding terminator)
- terminator: Terminator - Control flow exit instruction
- predecessors: Vec<String> - Labels of predecessor blocks
- successors: Vec<String> - Labels of successor blocks
- debug_info: Option<DebugInfo> - Debugging information including SourceSpan
**Relationships**: Belongs to a Function; connected to other BasicBlock entities via CFG
**Validation rules**: Must end with a valid terminator; predecessors/successors must exist in the same function
**State transitions**: May be modified by optimization passes (instructions added/removed)

## Entity: Instruction
**Description**: Atomic operations that perform computations, memory access, or control flow
**Fields**:
- id: InstructionId - Unique identifier for the instruction
- kind: InstructionKind - Type of operation (Binary, Call, Load, etc.)
- result_type: IrType - Type of the result value
- operands: Vec<ValueId> - Operands for the instruction
- debug_info: Option<DebugInfo> - Debugging information
- result_value: Option<ValueId> - Optional result value identifier
**Relationships**: Belongs to a BasicBlock; references Value entities via operands
**Validation rules**: Operands must be valid ValueId references; instruction must match result_type
**State transitions**: May be removed (dead code elimination), modified, or replaced by passes

## Entity: Terminator
**Description**: Defines how control flow exits a basic block (return, branch, switch)
**Fields**:
- kind: TerminatorKind - Type of terminator (Return, Branch, ConditionalBranch, etc.)
- operands: Vec<ValueId> - Values used by the terminator
- targets: Vec<String> - Target block labels for control flow
- debug_info: Option<DebugInfo> - Debugging information
**Relationships**: Belongs to a BasicBlock; references other BasicBlock entities via targets
**Validation rules**: Target blocks must exist in the same function
**State transitions**: May be simplified by optimization passes (e.g., unconditional branch)

## Entity: Value
**Description**: Represents values in the IR with different kinds (literal, constant, local, global, temporary)
**Fields**:
- id: ValueId - Unique identifier for the value
- kind: ValueKind - Type of value (Literal, Constant, Local, Global, Temporary)
- ty: IrType - Type of the value
- value: ValueData - Actual value data (IrLiteralValue, IrConstantValue, etc.)
- debug_info: Option<ValueDebugInfo> - Debugging information
- provenance: Option<Vec<ProvenanceRecord>> - Optimization history (if recorded)
**Relationships**: Referenced by Instruction entities via operands
**Validation rules**: Type must match the value kind; literal values must be valid for their type
**State transitions**: May be replaced by constant values, optimized, or eliminated

## Entity: AnalysisResult
**Description**: Abstract entity representing results produced by analysis passes
**Fields**:
- function_name: String - Function this analysis result is for
- kind: AnalysisKind - Type of analysis (ReachingDefs, LiveVars, etc.)
- timestamp: SystemTime - Time when analysis was computed
- dependencies: Vec<AnalysisKind> - Other analyses this one depends on
**Validation rules**: Must be invalidated when dependent analyses change
**State transitions**: Valid → Invalid (when invalidate() is called on the AnalysisManager)

## Entity: AnalysisManager
**Description**: Caches analysis results and manages dependencies between them
**Fields**:
- cached_results: HashMap<(String, AnalysisKind), Box<dyn Any>> - Cached analysis results
- dependencies: HashMap<AnalysisKind, Vec<AnalysisKind>> - Analysis dependency graph
- last_computed: HashMap<(String, AnalysisKind), SystemTime> - Last computation time
**Relationships**: Manages multiple AnalysisResult entities
**Validation rules**: Must maintain consistent dependency information
**State transitions**: Results invalidated when dependencies change

## Entity: OptimizationPass
**Description**: Abstract entity representing a single optimization transformation
**Fields**:
- name: &'static str - Unique name for the pass
- required_analyses: &'static [AnalysisKind] - Analyses required to run this pass
- invalidated_analyses: &'static [AnalysisKind] - Analyses invalidated by this pass
- metrics: PassMetrics - Performance and transformation metrics
**Validation rules**: Required analyses must be available before running
**State transitions**: Not executed → Executed (may change the Function)

## Entity: PassMetrics
**Description**: Metrics collected during a single optimization pass
**Fields**:
- instructions_eliminated: usize - Number of instructions removed
- constants_propagated: usize - Number of constants propagated
- cse_hits: usize - Number of common subexpression eliminations
- phi_nodes_removed: usize - Number of phi nodes removed
- blocks_removed: usize - Number of basic blocks removed
- elapsed: Duration - Time taken for the pass
**Relationships**: Associated with an OptimizationPass execution
**Validation rules**: All values must be non-negative
**State transitions**: Initialized → Accumulating → Finalized

## Entity: OptimizerConfig
**Description**: Configuration for the optimization pipeline
**Fields**:
- opt_level: OptLevel - Optimization level (O0, O1, O2, O3)
- max_iterations: usize - Maximum number of optimization iterations (default 10)
- loop_unroll_threshold: usize - Maximum loop unroll factor (default 4)
- alias_analysis_kind: AliasAnalysisKind - Type of alias analysis to use
- early_passes: Vec<Box<dyn OptimizationPass>> - Early phase passes
- middle_passes: Vec<Box<dyn OptimizationPass>> - Middle phase passes
- late_passes: Vec<Box<dyn OptimizationPass>> - Late phase passes
- record_provenance: bool - Whether to track optimization provenance
**Validation rules**: max_iterations > 0; loop_unroll_threshold > 0
**State transitions**: Created → Applied to optimization run

## Entity: FunctionSnapshot
**Description**: Snapshot of a function state for verification rollback
**Fields**:
- blocks: Vec<BasicBlock> - Cloned basic blocks
- edges: Vec<(String, String)> - CFG edges
- use_def_chains: HashMap<ValueId, InstructionRef> - Cloned use-def chains
- def_use_chains: HashMap<ValueId, Vec<InstructionRef>> - Cloned def-use chains
- timestamp: SystemTime - Time when snapshot was taken
**Relationships**: Associated with a Function entity
**Validation rules**: Must represent a consistent state of the function
**State transitions**: Captured → Restored (or discarded)

## Entity: AbstractLocation
**Description**: Location abstraction for alias analysis
**Fields**:
- variant: AbstractLocationVariant - Either Stack, Heap, Global, or Unknown
- allocation_site: Option<AllocationSite> - Source location for Stack/Heap/Global
**Relationships**: Used by alias analysis to determine memory relationships
**Validation rules**: Allocation site must be provided for Stack/Heap/Global variants
**State transitions**: Created → Used in alias queries

## Entity: ProvenanceRecord
**Description**: Record of how a value was transformed during optimization
**Fields**:
- pass_name: String - Name of pass that created this transformation
- original_value: ValueId - Value that was transformed
- transformation_kind: TransformKind - Type of transformation (ConstantFolded, etc.)
- timestamp: SystemTime - When the transformation occurred
**Relationships**: Associated with a Value entity
**Validation rules**: Original value must be valid if referenced
**State transitions**: Created during transformation → Stored with Value

## Entity: ValueId
**Description**: Unique identifier for values in the IR
**Fields**:
- uuid: Uuid - Globally unique identifier
**Validation rules**: Must be unique across all values
**State transitions**: Created → Used → Potentially eliminated

## Entity: InstructionRef
**Description**: Lightweight reference to an instruction in a function
**Fields**:
- block_label: String - Label of the containing basic block
- index: usize - Index of the instruction in the block
**Relationships**: Points to an Instruction within a BasicBlock
**Validation rules**: Block label must exist; index must be valid
**State transitions**: Valid → Potentially invalid (if block or instruction is removed)