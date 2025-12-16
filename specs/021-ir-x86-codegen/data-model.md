# Data Model: IR to x86-64 Assembly Code Generator

**Feature**: 021-ir-x86-codegen  
**Date**: 2025-12-16  
**Status**: Complete

## Overview

This document defines the data structures and relationships for the IR to x86-64 code generator. The model builds upon existing IR and ASM infrastructure while introducing new types for register allocation, phi resolution, and code generation.

---

## Entity Relationship Diagram

```
┌─────────────────┐         ┌─────────────────┐
│   ir::Module    │────────▶│  ir::Function   │
│                 │   1:N   │                 │
└─────────────────┘         └────────┬────────┘
                                     │ 1:1
                                     ▼
                            ┌─────────────────┐
                            │ CodeGenerator   │
                            │                 │
                            └────────┬────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐         ┌─────────────────┐         ┌─────────────────┐
│ LivenessAnalysis│         │ LinearScan      │         │ PhiResolver     │
│                 │         │ Allocator       │         │                 │
└────────┬────────┘         └────────┬────────┘         └────────┬────────┘
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐         ┌─────────────────┐         ┌─────────────────┐
│  LiveInterval   │         │ RegisterMapping │         │  ParallelCopy   │
│                 │         │                 │         │                 │
└─────────────────┘         └─────────────────┘         └─────────────────┘
                                     │
                                     ▼
                            ┌─────────────────┐
                            │ InstructionEmitter│
                            │                 │
                            └────────┬────────┘
                                     │
                                     ▼
                            ┌─────────────────┐
                            │  AssemblyFile   │
                            │   (EXISTING)    │
                            └─────────────────┘
```

---

## Core Entities

### 1. CodeGenerator

The main orchestrator for the code generation pipeline.

```rust
/// Main code generator that transforms IR modules to assembly.
pub struct CodeGenerator {
    /// Target platform for code generation.
    platform: Platform,
    
    /// ABI configuration derived from platform.
    abi: Abi,
    
    /// Code generation options.
    options: CodeGenOptions,
    
    /// Statistics collector.
    stats: CodeGenStats,
}

/// Configuration options for code generation.
#[derive(Debug, Clone, Default)]
pub struct CodeGenOptions {
    /// Include debug comments in output.
    pub emit_debug_comments: bool,
    
    /// Optimize fall-through blocks.
    pub optimize_fall_through: bool,
    
    /// Collect generation statistics.
    pub collect_stats: bool,
}
```

**Relationships**:
- Uses `ir::Module` as input (1:1 per generation call)
- Produces `AssemblyFile` as output (1:1)
- Contains `CodeGenStats` for metrics (1:1)

**Validation Rules**:
- Platform must be one of: Linux, macOS, Windows
- IR module must be validated before generation

---

### 2. GenerationContext

Per-function state during code generation.

```rust
/// Context maintained during code generation for a single function.
pub struct GenerationContext<'a> {
    /// The IR function being generated.
    function: &'a ir::Function,
    
    /// Target ABI configuration.
    abi: &'a Abi,
    
    /// Register allocation results.
    reg_mapping: RegisterMapping,
    
    /// Current basic block being processed.
    current_block: Option<Arc<str>>,
    
    /// Block ordering for fall-through optimization.
    block_order: Vec<Arc<str>>,
    
    /// Label counter for unique label generation.
    label_counter: u32,
    
    /// Stack frame layout.
    frame: StackFrame,
}

/// Stack frame layout for a function.
#[derive(Debug, Clone, Default)]
pub struct StackFrame {
    /// Total size in bytes (16-byte aligned).
    pub size: u32,
    
    /// Offset for each spill slot.
    pub spill_slots: HashMap<SpillSlotId, i32>,
    
    /// Offset for each local variable.
    pub locals: HashMap<Arc<str>, i32>,
    
    /// Callee-saved registers that need preservation.
    pub callee_saved_used: Vec<GPRegister64>,
}
```

**Relationships**:
- References `ir::Function` (1:1)
- Contains `RegisterMapping` (1:1)
- Contains `StackFrame` (1:1)

---

### 3. LivenessAnalysis

Computes liveness information for register allocation.

```rust
/// Result of liveness analysis for a function.
pub struct LivenessInfo {
    /// Live-in sets for each basic block.
    pub live_in: HashMap<Arc<str>, HashSet<ValueId>>,
    
    /// Live-out sets for each basic block.
    pub live_out: HashMap<Arc<str>, HashSet<ValueId>>,
    
    /// Live intervals for each value.
    pub intervals: Vec<LiveInterval>,
    
    /// Instruction numbering (for interval positions).
    pub inst_numbers: HashMap<InstructionId, usize>,
}

/// Represents the lifetime of a value in terms of instruction positions.
#[derive(Debug, Clone)]
pub struct LiveInterval {
    /// The IR value this interval represents.
    pub value: ValueId,
    
    /// First use position (inclusive).
    pub start: usize,
    
    /// Last use position (inclusive).
    pub end: usize,
    
    /// Use positions within the interval.
    pub use_positions: Vec<usize>,
    
    /// Definition position.
    pub def_position: usize,
    
    /// Preferred register class (GP or XMM).
    pub reg_class: RegisterClass,
    
    /// Allocated physical register (set by allocator).
    pub allocated: Option<PhysicalRegister>,
    
    /// Spill slot (set by allocator if spilled).
    pub spill_slot: Option<SpillSlotId>,
}

/// Register class for allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterClass {
    /// General-purpose registers (RAX, RBX, etc.)
    GeneralPurpose,
    /// SSE/AVX registers (XMM0, XMM1, etc.)
    Simd,
}
```

**Relationships**:
- Computed from `ir::Function` (N:1)
- Produces `LiveInterval` for each IR value (1:N)

---

### 4. LinearScanAllocator

Implements the Linear Scan register allocation algorithm.

```rust
/// Linear Scan register allocator.
pub struct LinearScanAllocator<'a> {
    /// Target ABI for register availability.
    abi: &'a Abi,
    
    /// All live intervals sorted by start position.
    intervals: Vec<LiveInterval>,
    
    /// Currently active intervals (sorted by end position).
    active: BTreeSet<IntervalRef>,
    
    /// Available general-purpose registers.
    free_gp_regs: Vec<GPRegister64>,
    
    /// Available SIMD registers.
    free_xmm_regs: Vec<XMMRegister>,
    
    /// Next spill slot offset.
    next_spill_offset: u32,
    
    /// Allocation results.
    mapping: RegisterMapping,
}

/// Reference to an interval for ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalRef {
    pub index: usize,
    pub end: usize,
}

impl Ord for IntervalRef {
    fn cmp(&self, other: &Self) -> Ordering {
        self.end.cmp(&other.end)
    }
}
```

**Relationships**:
- Takes `LivenessInfo` as input (1:1)
- Produces `RegisterMapping` (1:1)

---

### 5. RegisterMapping

Maps IR values to physical locations.

```rust
/// Complete register allocation result for a function.
#[derive(Debug, Clone, Default)]
pub struct RegisterMapping {
    /// Value to physical register mapping.
    pub reg_assignments: HashMap<ValueId, PhysicalRegister>,
    
    /// Value to spill slot mapping.
    pub spill_assignments: HashMap<ValueId, SpillSlotId>,
    
    /// Spill slot details.
    pub spill_slots: HashMap<SpillSlotId, SpillSlot>,
    
    /// Total number of spills.
    pub spill_count: u32,
}

/// A physical register (GP or SIMD).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalRegister {
    GP(GPRegister64),
    Simd(XMMRegister),
}

/// Unique identifier for a spill slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpillSlotId(pub u32);

/// A spill slot on the stack.
#[derive(Debug, Clone)]
pub struct SpillSlot {
    /// Unique identifier.
    pub id: SpillSlotId,
    
    /// Offset from RBP (negative).
    pub offset: i32,
    
    /// Size in bytes.
    pub size: u32,
}
```

**Relationships**:
- Maps `ValueId` → `PhysicalRegister` or `SpillSlot` (N:1)

---

### 6. PhiResolver

Handles SSA phi node resolution.

```rust
/// Resolves phi nodes into explicit move instructions.
pub struct PhiResolver<'a> {
    /// The function's CFG.
    cfg: &'a ir::ControlFlowGraph,
    
    /// Register mapping for physical locations.
    reg_mapping: &'a RegisterMapping,
}

/// A move operation for phi resolution.
#[derive(Debug, Clone)]
pub struct PhiMove {
    /// Source value.
    pub src: ValueId,
    
    /// Destination value (phi result).
    pub dst: ValueId,
    
    /// Physical source location.
    pub src_loc: PhysicalLocation,
    
    /// Physical destination location.
    pub dst_loc: PhysicalLocation,
}

/// Physical location (register or memory).
#[derive(Debug, Clone, Copy)]
pub enum PhysicalLocation {
    Register(PhysicalRegister),
    Stack(i32), // Offset from RBP
}

/// A set of moves that must happen "simultaneously".
#[derive(Debug, Clone, Default)]
pub struct ParallelCopy {
    /// All moves in this parallel copy.
    pub moves: Vec<PhiMove>,
}

impl ParallelCopy {
    /// Sequentialize moves, handling cycles with temporaries.
    pub fn sequentialize(&self) -> Vec<PhiMove>;
    
    /// Detect cycles in the move graph.
    fn find_cycles(&self) -> Vec<Vec<usize>>;
}
```

**Relationships**:
- Uses `RegisterMapping` for physical locations (1:1)
- Produces `ParallelCopy` per predecessor block (N:1)

---

### 7. InstructionEmitter

Emits x86-64 instructions to the assembly file.

```rust
/// Emits x86-64 instructions from IR.
pub struct InstructionEmitter<'a> {
    /// Output assembly file.
    output: &'a mut AssemblyFile,
    
    /// Generation context.
    ctx: &'a GenerationContext<'a>,
    
    /// Current instruction being emitted.
    current_inst: Option<&'a ir::Instruction>,
}

impl InstructionEmitter<'_> {
    /// Emit a single IR instruction.
    pub fn emit_instruction(&mut self, inst: &ir::Instruction) -> Result<(), CodeGenError>;
    
    /// Emit function prologue.
    pub fn emit_prologue(&mut self, frame: &StackFrame) -> Result<(), CodeGenError>;
    
    /// Emit function epilogue.
    pub fn emit_epilogue(&mut self, frame: &StackFrame) -> Result<(), CodeGenError>;
    
    /// Emit a basic block label.
    pub fn emit_label(&mut self, label: &str);
    
    /// Emit a comment.
    pub fn emit_comment(&mut self, comment: &str);
}
```

**Relationships**:
- Writes to `AssemblyFile` (N:1)
- Reads from `GenerationContext` (1:1)

---

### 8. CodeGenError

Error types for code generation failures.

```rust
/// Errors that can occur during code generation.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CodeGenError {
    /// IR construct cannot be translated to x86-64.
    #[error("unsupported IR construct: {0}")]
    UnsupportedConstruct(String),
    
    /// Register allocation failed.
    #[error("register allocation failed: {0}")]
    AllocationFailed(String),
    
    /// Stack frame too large.
    #[error("stack frame exceeds maximum size: {size} bytes")]
    StackOverflow { size: u64 },
    
    /// Invalid IR (should have been caught by validator).
    #[error("invalid IR: {0}")]
    InvalidIr(String),
    
    /// Internal error (bug in codegen).
    #[error("internal codegen error: {0}")]
    Internal(String),
}
```

---

### 9. CodeGenStats

Statistics about generated code.

```rust
/// Statistics about code generation.
#[derive(Debug, Clone, Default)]
pub struct CodeGenStats {
    /// Number of functions generated.
    pub functions: u32,
    
    /// Total instructions emitted.
    pub instructions: u32,
    
    /// Instructions by category.
    pub instructions_by_kind: HashMap<String, u32>,
    
    /// Total spills across all functions.
    pub total_spills: u32,
    
    /// Maximum spills in a single function.
    pub max_spills_per_function: u32,
    
    /// Register usage counts.
    pub register_usage: HashMap<String, u32>,
    
    /// Generation time in milliseconds.
    pub generation_time_ms: u64,
}

impl CodeGenStats {
    /// Record a generated instruction.
    pub fn record_instruction(&mut self, kind: &str);
    
    /// Record register usage.
    pub fn record_register(&mut self, reg: PhysicalRegister);
    
    /// Record spill count for a function.
    pub fn record_spills(&mut self, count: u32);
    
    /// Format as human-readable report.
    pub fn report(&self) -> String;
}
```

---

## Identifier Types

```rust
/// Unique identifier for an IR value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

/// Unique identifier for an IR instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionId {
    pub block: u32,
    pub index: u32,
}

/// Unique identifier for a spill slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpillSlotId(pub u32);
```

---

## State Transitions

### Code Generation Pipeline States

```
┌─────────┐     ┌──────────┐     ┌───────────┐     ┌─────────┐
│  Init   │────▶│ Analyzing│────▶│ Allocating│────▶│Resolving│
└─────────┘     └──────────┘     └───────────┘     └─────────┘
                                                        │
                ┌──────────┐     ┌───────────┐          │
                │ Complete │◀────│  Emitting │◀─────────┘
                └──────────┘     └───────────┘
```

| State | Description | Outputs |
|-------|-------------|---------|
| Init | Load IR, validate, set up context | GenerationContext |
| Analyzing | Compute liveness information | LivenessInfo |
| Allocating | Run Linear Scan allocation | RegisterMapping |
| Resolving | Resolve phi nodes | ParallelCopy per block |
| Emitting | Generate x86-64 instructions | AssemblyFile |
| Complete | Finalize, collect stats | Final AssemblyFile + Stats |

---

## Validation Rules

### LiveInterval Invariants

1. `start <= end`
2. `def_position == start` for non-phi values
3. All `use_positions` within `[start, end]`
4. `reg_class` matches IR value type

### RegisterMapping Invariants

1. No value in both `reg_assignments` and `spill_assignments`
2. Every spilled value has corresponding entry in `spill_slots`
3. No two live values share the same register at any point

### StackFrame Invariants

1. `size` is 16-byte aligned
2. All spill slot offsets are negative (below RBP)
3. No overlapping spill slots

---

## Existing Types (from codebase)

The code generator reuses these existing types:

| Type | Module | Description |
|------|--------|-------------|
| `ir::Module` | `src/ir/module.rs` | IR module container |
| `ir::Function` | `src/ir/function.rs` | IR function with CFG |
| `ir::BasicBlock` | `src/ir/basic_block.rs` | Block with instructions |
| `ir::Instruction` | `src/ir/instruction.rs` | IR operations |
| `ir::IrType` | `src/ir/types.rs` | IR type system |
| `ir::Value` | `src/ir/value/` | IR values |
| `Abi` | `src/asm/abi.rs` | ABI configuration |
| `Platform` | `src/asm/platform.rs` | Target platform |
| `GPRegister64` | `src/asm/register/` | GP registers |
| `XMMRegister` | `src/asm/register/` | SIMD registers |
| `AssemblyFile` | `src/asm/assembly_file.rs` | Assembly output |
| `Instruction` | `src/asm/instruction/` | x86 instructions |
