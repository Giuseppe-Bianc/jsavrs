# Analysis API Contracts

## Reaching Definitions Analysis

### `ReachingDefinitions` struct
```rust
pub struct ReachingDefinitions {
    def_blocks: HashMap<ValueId, NodeIndex>,           // Which block defines each value
    reaching_defs: HashMap<NodeIndex, BitVec>,        // Definitions reaching each block
    gen_sets: HashMap<NodeIndex, BitVec>,             // Definitions generated in each block
    kill_sets: HashMap<NodeIndex, BitVec>,            // Definitions killed in each block
}

impl Analysis<Function> for ReachingDefinitions {
    type Context = ();
    
    fn compute(&self, function: &Function, _context: &()) -> Self
    fn invalidate(&mut self)
}

impl ReachingDefinitions {
    pub fn get_reaching_defs_at_block(&self, block_idx: NodeIndex) -> &BitVec
    pub fn get_reaching_defs_at_instruction(&self, block_label: &str, instr_idx: usize) -> BitVec
    pub fn has_reaching_def(&self, block_idx: NodeIndex, value_id: ValueId) -> bool
}
```

## Live Variables Analysis

### `LiveVariables` struct
```rust
pub struct LiveVariables {
    live_in: HashMap<NodeIndex, HashSet<ValueId>>,    // Variables live at block entrance
    live_out: HashMap<NodeIndex, HashSet<ValueId>>,   // Variables live at block exit
    use_sets: HashMap<NodeIndex, HashSet<ValueId>>,   // Variables used in each block
    def_sets: HashMap<NodeIndex, HashSet<ValueId>>,   // Variables defined in each block
}

impl Analysis<Function> for LiveVariables {
    type Context = ();
    
    fn compute(&self, function: &Function, _context: &()) -> Self
    fn invalidate(&mut self)
}

impl LiveVariables {
    pub fn is_live_at_block_start(&self, block_idx: NodeIndex, value_id: ValueId) -> bool
    pub fn is_live_at_block_end(&self, block_idx: NodeIndex, value_id: ValueId) -> bool
    pub fn get_live_in(&self, block_idx: NodeIndex) -> &HashSet<ValueId>
    pub fn get_live_out(&self, block_idx: NodeIndex) -> &HashSet<ValueId>
}
```

## Constant Lattice Analysis

### `ConstantLattice` enum
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantLattice {
    Top,                    // Unknown value
    Constant(IrLiteralValue), // Concrete constant value
    Bottom,                 // Contradiction/unreachable
}
```

### `ConstantPropagation` struct
```rust
pub struct ConstantPropagation {
    constants: HashMap<ValueId, ConstantLattice>,
    executable_edges: HashSet<(String, String)>,  // Reachable CFG edges
}

impl Analysis<Function> for ConstantPropagation {
    type Context = ();
    
    fn compute(&self, function: &Function, _context: &()) -> Self
    fn invalidate(&mut self)
}

impl ConstantPropagation {
    pub fn get_constant_value(&self, value_id: ValueId) -> Option<&ConstantLattice>
    pub fn is_executable_edge(&self, from: &str, to: &str) -> bool
    pub fn is_constant(&self, value_id: ValueId) -> bool
    pub fn get_constant_int_value(&self, value_id: ValueId) -> Option<i64>
    pub fn get_constant_bool_value(&self, value_id: ValueId) -> Option<bool>
    pub fn get_constant_float_value(&self, value_id: ValueId) -> Option<f64>
}
```

## Loop Analysis

### `LoopInfo` struct
```rust
pub struct LoopInfo {
    loops: HashMap<String, LoopMetadata>,           // Map from header to loop metadata
    loop_depths: HashMap<String, usize>,            // Nesting depth of each block
    block_to_loop: HashMap<String, String>,         // Which loop contains each block
}

#[derive(Debug, Clone)]
pub struct LoopMetadata {
    pub header: String,                    // Header block label
    pub members: Vec<String>,             // All blocks in the loop
    pub depth: usize,                     // Nesting depth
    pub parent: Option<String>,           // Parent loop if nested
    pub children: Vec<String>,            // Nested child loops
    pub is_reducible: bool,               // Whether the loop is reducible
}

impl Analysis<Function> for LoopInfo {
    type Context = DominanceInfo;
    
    fn compute(&self, function: &Function, dominance_info: &DominanceInfo) -> Self
    fn invalidate(&mut self)
}

impl LoopInfo {
    pub fn get_loop(&self, block: &str) -> Option<&LoopMetadata>
    pub fn get_loop_depth(&self, block: &str) -> usize
    pub fn is_in_loop(&self, block: &str) -> bool
    pub fn get_innermost_loop(&self, block: &str) -> Option<&LoopMetadata>
    pub fn get_all_loops(&self) -> &HashMap<String, LoopMetadata>
    pub fn is_loop_invariant(&self, block: &str, value: ValueId, function: &Function) -> bool
}
```

## Global Value Numbering Analysis

### `GlobalValueNumbering` struct
```rust
pub struct GlobalValueNumbering {
    expression_map: HashMap<ExpressionHash, ValueId>,  // Map from expression to its canonical value
    available_expressions: HashMap<NodeIndex, HashSet<ExpressionHash>>,  // Available exprs at block exits
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionHash {
    kind: InstructionKind,
    operands: Vec<ValueId>,
    ty: IrType,
}

impl ExpressionHash {
    // Custom Hash implementation that ignores result values
    // Commutative operations have operands sorted by ValueId
}

impl Analysis<Function> for GlobalValueNumbering {
    type Context = ();
    
    fn compute(&self, function: &Function, _context: &()) -> Self
    fn invalidate(&mut self)
}

impl GlobalValueNumbering {
    pub fn get_or_assign_value_number(&mut self, expr: &Instruction) -> ValueId
    pub fn has_equivalent_expression(&self, expr: &Instruction) -> Option<ValueId>
    pub fn get_canonical_value(&self, expr_hash: &ExpressionHash) -> Option<ValueId>
    pub fn is_commutative(&self, kind: InstructionKind) -> bool
}
```

## Alias Analysis Implementations

### `AndersenAnalysis` struct
```rust
pub struct AndersenAnalysis {
    constraint_graph: HashMap<ValueId, HashSet<Constraint>>,  // Points-to graph
    points_to_sets: HashMap<ValueId, HashSet<AbstractLocation>>,  // Final points-to info
    processed: HashSet<Constraint>,  // Constraints already processed
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {
    Copy(ValueId, ValueId),        // v1 = v2
    Store(ValueId, ValueId),       // *v1 = v2
    Load(ValueId, ValueId),        // v1 = *v2
    AddrOf(ValueId, AllocationSite), // v = &allocation
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbstractLocation {
    Stack(AllocationSite),
    Heap(AllocationSite),
    Global(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AllocationSite {
    pub function_name: String,
    pub block_label: String,
    pub instruction_index: usize,
}

impl Analysis<Function> for AndersenAnalysis {
    type Context = ();
    
    fn compute(&self, function: &Function, _context: &()) -> Self
    fn invalidate(&mut self)
}

impl AliasAnalysis for AndersenAnalysis {
    fn may_alias(&self, value1: &Value, value2: &Value) -> Result<bool, AnalysisError>
    fn points_to_set(&self, value: &Value) -> Result<&HashSet<AbstractLocation>, AnalysisError>
}

impl AndersenAnalysis {
    pub fn add_constraint(&mut self, constraint: Constraint)
    pub fn solve(&mut self)  // Runs until fixed point
    pub fn get_points_to_set(&self, value_id: ValueId) -> Option<&HashSet<AbstractLocation>>
}
```

### `ConservativeAnalysis` struct
```rust
pub struct ConservativeAnalysis {
    external_symbols: HashSet<String>,  // Known external functions
}

impl Analysis<Function> for ConservativeAnalysis {
    type Context = ();
    
    fn compute(&self, function: &Function, _context: &()) -> Self
    fn invalidate(&mut self)
}

impl AliasAnalysis for ConservativeAnalysis {
    fn may_alias(&self, value1: &Value, value2: &Value) -> Result<bool, AnalysisError> {
        // Conservative: assume may alias unless proven otherwise
        if Self::proven_distinct(value1, value2) {
            Ok(false)
        } else {
            Ok(true)
        }
    }
    
    fn points_to_set(&self, value: &Value) -> Result<&HashSet<AbstractLocation>, AnalysisError> {
        // Conservative: return full set of possible locations
        // Implementation would return a conservative estimate
        unimplemented!() // Conservative analysis would return a default/unknown set
    }
}

impl ConservativeAnalysis {
    fn proven_distinct(value1: &Value, value2: &Value) -> bool {
        // Check if values are proven distinct via type or definition analysis
        // This is a simplified check - real implementation would be more sophisticated
        value1.id == value2.id  // Same value is not distinct
    }
}
```