# API Contracts: IR to x86-64 Code Generator

**Feature**: 021-ir-x86-codegen  
**Date**: 2025-12-16  
**Status**: Complete

## Overview

This document defines the public API contracts for the IR to x86-64 code generator module.

---

## Module Structure

```rust
// src/asm/codegen/mod.rs
pub mod context;
pub mod emitter;
pub mod error;
pub mod stats;

// Re-exports
pub use context::GenerationContext;
pub use error::CodeGenError;
pub use stats::CodeGenStats;
```

---

## Primary Entry Points

### `generate`

Generates x86-64 assembly from an IR module using default options.

```rust
/// Generates x86-64 assembly code from an IR module.
///
/// # Arguments
///
/// * `module` - The validated IR module to generate code for
/// * `platform` - Target platform (Linux, macOS, or Windows)
///
/// # Returns
///
/// * `Ok(AssemblyFile)` - Generated assembly ready for NASM
/// * `Err(CodeGenError)` - If generation fails
///
/// # Example
///
/// ```rust
/// use jsavrs::ir::Module;
/// use jsavrs::asm::{Platform, codegen};
///
/// let module = Module::new("example", DataLayout::LinuxX86_64, TargetTriple::X86_64UnknownLinuxGnu);
/// // ... add functions to module ...
///
/// let assembly = codegen::generate(&module, Platform::Linux)?;
/// println!("{}", assembly);
/// ```
///
/// # Errors
///
/// Returns `CodeGenError` if:
/// - IR contains unsupported constructs
/// - Register allocation fails
/// - Stack frame exceeds maximum size
pub fn generate(module: &ir::Module, platform: Platform) -> Result<AssemblyFile, CodeGenError>;
```

### `generate_with_options`

Generates x86-64 assembly with custom options.

```rust
/// Generates x86-64 assembly code with custom options.
///
/// # Arguments
///
/// * `module` - The validated IR module to generate code for
/// * `options` - Code generation options
///
/// # Returns
///
/// * `Ok((AssemblyFile, CodeGenStats))` - Generated assembly and statistics
/// * `Err(CodeGenError)` - If generation fails
///
/// # Example
///
/// ```rust
/// use jsavrs::asm::codegen::{self, CodeGenOptions};
///
/// let options = CodeGenOptions {
///     platform: Platform::Windows,
///     emit_debug_comments: true,
///     optimize_fall_through: true,
///     collect_stats: true,
/// };
///
/// let (assembly, stats) = codegen::generate_with_options(&module, options)?;
/// println!("Generated {} instructions", stats.instructions);
/// ```
pub fn generate_with_options(
    module: &ir::Module,
    options: CodeGenOptions,
) -> Result<(AssemblyFile, CodeGenStats), CodeGenError>;
```

---

## Configuration Types

### `CodeGenOptions`

```rust
/// Configuration options for code generation.
#[derive(Debug, Clone)]
pub struct CodeGenOptions {
    /// Target platform for code generation.
    /// Default: Platform::Linux
    pub platform: Platform,
    
    /// Include debug comments in output showing:
    /// - Original variable names
    /// - Source locations
    /// - Basic block boundaries
    /// Default: false
    pub emit_debug_comments: bool,
    
    /// Optimize fall-through blocks by eliminating
    /// unnecessary jump instructions.
    /// Default: true
    pub optimize_fall_through: bool,
    
    /// Collect and return generation statistics.
    /// Default: false
    pub collect_stats: bool,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            platform: Platform::Linux,
            emit_debug_comments: false,
            optimize_fall_through: true,
            collect_stats: false,
        }
    }
}
```

---

## Error Types

### `CodeGenError`

```rust
/// Errors that can occur during code generation.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CodeGenError {
    /// IR construct has no valid x86-64 translation.
    ///
    /// This occurs when the IR contains operations that cannot
    /// be represented in x86-64 assembly.
    #[error("unsupported IR construct: {construct} - {reason}")]
    UnsupportedConstruct {
        construct: String,
        reason: String,
    },
    
    /// Register allocation failed due to excessive pressure.
    ///
    /// This should rarely occur with Linear Scan allocation
    /// but may happen with extremely complex control flow.
    #[error("register allocation failed for function '{function}': {reason}")]
    AllocationFailed {
        function: String,
        reason: String,
    },
    
    /// Stack frame exceeds maximum allowed size.
    ///
    /// x86-64 addressing modes limit stack offsets.
    #[error("stack frame for function '{function}' exceeds maximum: {size} bytes > {max} bytes")]
    StackOverflow {
        function: String,
        size: u64,
        max: u64,
    },
    
    /// IR is invalid (should have been caught by IR validator).
    ///
    /// This indicates a bug in an earlier compilation phase.
    #[error("invalid IR in function '{function}': {reason}")]
    InvalidIr {
        function: String,
        reason: String,
    },
    
    /// Internal code generator error.
    ///
    /// This indicates a bug in the code generator itself.
    #[error("internal codegen error: {0}")]
    Internal(String),
}

impl CodeGenError {
    /// Returns true if this error is recoverable.
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::UnsupportedConstruct { .. })
    }
    
    /// Returns the function name where the error occurred, if known.
    pub fn function(&self) -> Option<&str> {
        match self {
            Self::AllocationFailed { function, .. } => Some(function),
            Self::StackOverflow { function, .. } => Some(function),
            Self::InvalidIr { function, .. } => Some(function),
            _ => None,
        }
    }
}
```

---

## Statistics Types

### `CodeGenStats`

```rust
/// Statistics about code generation.
#[derive(Debug, Clone, Default)]
pub struct CodeGenStats {
    /// Number of functions generated.
    pub functions: u32,
    
    /// Total instructions emitted across all functions.
    pub instructions: u32,
    
    /// Instruction breakdown by category.
    pub instructions_by_kind: HashMap<InstructionKind, u32>,
    
    /// Total register spills.
    pub total_spills: u32,
    
    /// Maximum spills in any single function.
    pub max_spills_per_function: u32,
    
    /// Average spills per function.
    pub avg_spills_per_function: f32,
    
    /// Register usage frequency.
    pub register_usage: HashMap<String, u32>,
    
    /// Total generation time in milliseconds.
    pub generation_time_ms: u64,
}

/// Categories of generated instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionKind {
    Arithmetic,
    Memory,
    ControlFlow,
    Conversion,
    Comparison,
    Move,
    Push,
    Pop,
    Call,
    Return,
    Other,
}

impl CodeGenStats {
    /// Format statistics as a human-readable report.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Code Generation Statistics
    /// ==========================
    /// Functions:     42
    /// Instructions:  1,234
    /// Total spills:  56
    /// Max spills:    8 (in function 'complex_calc')
    /// Time:          45ms
    ///
    /// Instruction Breakdown:
    ///   Arithmetic:    456 (37%)
    ///   Memory:        234 (19%)
    ///   ControlFlow:   123 (10%)
    ///   ...
    /// ```
    pub fn report(&self) -> String;
    
    /// Returns true if spill count exceeds threshold.
    pub fn has_excessive_spills(&self, threshold: u32) -> bool {
        self.max_spills_per_function > threshold
    }
}
```

---

## Register Allocation API

### `regalloc::allocate`

```rust
/// Performs register allocation for a function.
///
/// # Arguments
///
/// * `function` - The IR function to allocate registers for
/// * `abi` - Target ABI configuration
///
/// # Returns
///
/// * `Ok(RegisterMapping)` - Successful allocation
/// * `Err(CodeGenError)` - If allocation fails
pub fn allocate(
    function: &ir::Function,
    abi: &Abi,
) -> Result<RegisterMapping, CodeGenError>;
```

### `RegisterMapping`

```rust
/// Register allocation result for a function.
#[derive(Debug, Clone, Default)]
pub struct RegisterMapping {
    /// Value to physical register assignments.
    reg_assignments: HashMap<ValueId, PhysicalRegister>,
    
    /// Value to spill slot assignments.
    spill_assignments: HashMap<ValueId, SpillSlotId>,
    
    /// Spill slot metadata.
    spill_slots: HashMap<SpillSlotId, SpillSlot>,
}

impl RegisterMapping {
    /// Get the physical location for a value.
    ///
    /// Returns `Some(PhysicalLocation)` if the value has been allocated,
    /// `None` if the value is unknown.
    pub fn get(&self, value: ValueId) -> Option<PhysicalLocation>;
    
    /// Returns true if the value is spilled to stack.
    pub fn is_spilled(&self, value: ValueId) -> bool;
    
    /// Get spill slot details.
    pub fn spill_slot(&self, id: SpillSlotId) -> Option<&SpillSlot>;
    
    /// Total number of spills.
    pub fn spill_count(&self) -> u32;
    
    /// Iterator over all register assignments.
    pub fn registers(&self) -> impl Iterator<Item = (ValueId, PhysicalRegister)> + '_;
    
    /// Iterator over all spill assignments.
    pub fn spills(&self) -> impl Iterator<Item = (ValueId, &SpillSlot)> + '_;
}
```

---

## Phi Resolution API

### `phi::resolve`

```rust
/// Resolves phi nodes for a function.
///
/// # Arguments
///
/// * `function` - The IR function with phi nodes
/// * `mapping` - Register allocation results
///
/// # Returns
///
/// Map from predecessor block label to sequentialized moves.
pub fn resolve(
    function: &ir::Function,
    mapping: &RegisterMapping,
) -> HashMap<Arc<str>, Vec<PhiMove>>;
```

### `PhiMove`

```rust
/// A single move operation for phi resolution.
#[derive(Debug, Clone)]
pub struct PhiMove {
    /// Source physical location.
    pub src: PhysicalLocation,
    
    /// Destination physical location.
    pub dst: PhysicalLocation,
    
    /// Size in bytes.
    pub size: u32,
    
    /// Optional debug info (original IR values).
    pub debug_src: Option<String>,
    pub debug_dst: Option<String>,
}
```

---

## Instruction Lowering API

### `lowering::lower_instruction`

```rust
/// Lowers a single IR instruction to x86-64 instructions.
///
/// # Arguments
///
/// * `inst` - The IR instruction to lower
/// * `ctx` - Generation context with register mapping
///
/// # Returns
///
/// Vector of x86-64 instructions (may be 1:1 or 1:many).
pub fn lower_instruction(
    inst: &ir::Instruction,
    ctx: &GenerationContext,
) -> Result<Vec<Instruction>, CodeGenError>;
```

### `lowering::lower_terminator`

```rust
/// Lowers a basic block terminator to x86-64 instructions.
///
/// # Arguments
///
/// * `term` - The block terminator
/// * `ctx` - Generation context
/// * `next_block` - Label of the next block (for fall-through optimization)
///
/// # Returns
///
/// Vector of x86-64 instructions (jumps, returns, etc.).
pub fn lower_terminator(
    term: &ir::Terminator,
    ctx: &GenerationContext,
    next_block: Option<&str>,
) -> Result<Vec<Instruction>, CodeGenError>;
```

---

## Output Format

### Generated Assembly Structure

```nasm
; Assembly File - ABI: System V AMD64 ABI on Linux
; Generated by jsavrs code generator

section .data
    ; Initialized global variables
    global_var: dd 42

section .rodata
    ; String literals and constants
    str_0: db "Hello, World!", 0

section .bss
    ; Uninitialized global variables
    buffer: resb 1024

section .text
    global main
    extern printf

main:
    ; Function prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32
    
    ; Block: entry
    ; ... instructions ...
    
    ; Function epilogue
    mov rsp, rbp
    pop rbp
    ret
```

---

## Thread Safety

All types in the codegen module are `Send + Sync` where appropriate:

- `CodeGenerator` - Not `Send` (holds mutable state during generation)
- `CodeGenOptions` - `Send + Sync` (immutable configuration)
- `CodeGenStats` - `Send + Sync` (value type)
- `CodeGenError` - `Send + Sync` (value type)
- `RegisterMapping` - `Send + Sync` (value type)

For parallel compilation of multiple modules, create separate `CodeGenerator` instances per thread.
