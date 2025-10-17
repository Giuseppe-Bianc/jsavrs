# API Contract: AsmGenerator

**Component**: Core Assembly Code Generator  
**Module**: `src/asm/generator.rs`  
**Status**: Public API Contract  
**Version**: 1.0.0

## Overview

The `AsmGenerator` is the main entry point for x86-64 NASM assembly code generation from jsavrs IR. It coordinates register allocation, instruction selection, phi resolution, and assembly output formatting to produce valid, platform-appropriate assembly code.

## Public Interface

### Constructor

```rust
pub fn new(target: TargetTriple) -> Self
```

**Purpose**: Creates a new assembly generator for the specified target platform.

**Parameters**:
- `target: TargetTriple` - Target platform (determines ABI and calling conventions)
  - Valid values: `X86_64UnknownLinuxGnu`, `X86_64PcWindowsMsvc`, `X86_64AppleDarwin`, etc.

**Returns**: Initialized `AsmGenerator` instance

**Postconditions**:
- ABI is selected based on target (System V for Linux/macOS, Microsoft x64 for Windows)
- All internal components initialized (register allocator, instruction selector, phi resolver)
- Error list is empty
- Label counter starts at 0

**Example**:
```rust
let generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);
```

---

### Main Generation Method

```rust
pub fn generate(&mut self, module: &Module) -> CodeGenResult
```

**Purpose**: Generates x86-64 NASM assembly code from an IR module.

**Parameters**:
- `module: &Module` - IR module containing functions to compile

**Returns**: `CodeGenResult` containing:
- `assembly: Option<String>` - Generated assembly code (None if all functions failed)
- `errors: Vec<CodeGenError>` - List of all errors encountered
- `stats: CodeGenStats` - Generation statistics

**Preconditions**:
- `module` must contain at least one function
- IR must be well-formed (validated, all blocks have terminators)
- All IR types must be supported (I8-I64, U8-U64, F32, F64, Bool, Char, Pointer, Void)

**Postconditions**:
- If `errors.is_empty()`: all functions successfully generated, `assembly.is_some()`
- If `!errors.is_empty()` and `assembly.is_some()`: partial generation (some functions succeeded)
- If `!errors.is_empty()` and `assembly.is_none()`: complete failure (no functions generated)

**Behavior**:
1. Validates target platform compatibility
2. For each function in module:
   - Resolves SSA phi functions (critical edge splitting)
   - Performs liveness analysis
   - Allocates registers (linear scan with spilling)
   - Selects instructions (IR → x86-64)
   - Generates prologue and epilogue
   - Accumulates errors (does not stop on first error)
3. Organizes assembly into sections (.text, .data, .bss, .rodata)
4. Formats output with NASM syntax and comments

**Error Handling**:
- Errors are accumulated in `CodeGenResult.errors`
- Generation continues for valid functions even after errors
- Partial assembly is returned if any functions succeed

**Example**:
```rust
let mut generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);
let result = generator.generate(&ir_module);

if result.errors.is_empty() {
    println!("Success! Generated {} instructions", result.stats.assembly_instructions);
    std::fs::write("output.asm", result.assembly.unwrap())?;
} else {
    eprintln!("Errors encountered:");
    for error in &result.errors {
        eprintln!("  - {}", error);
    }
    
    if let Some(asm) = result.assembly {
        println!("Partial assembly available ({} functions succeeded)", 
            result.stats.functions_generated);
        std::fs::write("output.asm", asm)?;
    }
}
```

---

## Result Types

### CodeGenResult

```rust
pub struct CodeGenResult {
    pub assembly: Option<String>,
    pub errors: Vec<CodeGenError>,
    pub stats: CodeGenStats,
}
```

**Fields**:
- `assembly`: Generated NASM assembly code (None if all functions failed)
- `errors`: All errors encountered during generation
- `stats`: Statistics about the generation process

**Invariants**:
- If `errors.is_empty()`, then `assembly.is_some()`
- `stats.functions_generated + stats.functions_failed` equals total function count

---

### CodeGenStats

```rust
pub struct CodeGenStats {
    pub functions_generated: usize,
    pub functions_failed: usize,
    pub instructions_translated: usize,
    pub assembly_instructions: usize,
    pub register_spills: usize,
    pub total_stack_size: usize,
}
```

**Purpose**: Provides metrics about the generation process for debugging and optimization.

---

### CodeGenError

```rust
use crate::error::compile_error::CompileError;

#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported type {ty} at {location}")]
    UnsupportedType { ty: IrType, location: SourceSpan },
    
    #[error("Unknown instruction kind {kind} at {location}")]
    UnknownInstruction { kind: String, location: SourceSpan },
    
    #[error("Malformed instruction: {reason} at {location}")]
    MalformedInstruction { reason: String, location: SourceSpan },
    
    #[error("Register allocation failed: {0}")]
    RegisterAllocationFailed(String),
    
    #[error("Invalid calling convention {convention} for target {target}")]
    InvalidCallingConvention { convention: String, target: String },
    
    #[error("CFG verification failed: {0}")]
    CfgVerificationFailed(String),
    
    #[error("Phi resolution failed: {0}")]
    PhiResolutionFailed(String),
    
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<CodeGenError> for CompileError {
    /// Converts CodeGenError to unified CompileError for integration with compiler pipeline.
    ///
    /// This enables seamless error reporting across all compilation phases.
    /// Each variant is mapped with appropriate help text for user guidance.
    fn from(err: CodeGenError) -> Self {
        match err {
            CodeGenError::UnsupportedType { ty, location } => {
                CompileError::AsmGeneratorError {
                    message: format!("Unsupported type: {}", ty),
                    span: Some(location),
                    help: Some(
                        "Only I8-I64, U8-U64, F32, F64, Bool, Char, Pointer, and Void types are supported. \
                         Consider using a supported type or refactoring the code.".to_string()
                    ),
                }
            }
            CodeGenError::UnknownInstruction { kind, location } => {
                CompileError::AsmGeneratorError {
                    message: format!("Unknown IR instruction: {}", kind),
                    span: Some(location),
                    help: Some(
                        "This instruction is not recognized by the code generator. \
                         This may indicate a bug in the IR generator.".to_string()
                    ),
                }
            }
            CodeGenError::MalformedInstruction { reason, location } => {
                CompileError::AsmGeneratorError {
                    message: format!("Malformed IR instruction: {}", reason),
                    span: Some(location),
                    help: Some(
                        "The IR instruction is structurally invalid. \
                         This may indicate a bug in the IR generator or validator.".to_string()
                    ),
                }
            }
            CodeGenError::RegisterAllocationFailed(msg) => {
                CompileError::AsmGeneratorError {
                    message: format!("Register allocation failed: {}", msg),
                    span: None,
                    help: Some(
                        "The function may have too many live values simultaneously (>20). \
                         Consider simplifying the function or reducing temporary variables.".to_string()
                    ),
                }
            }
            CodeGenError::InvalidCallingConvention { convention, target } => {
                CompileError::AsmGeneratorError {
                    message: format!(
                        "Invalid calling convention '{}' for target '{}'", 
                        convention, target
                    ),
                    span: None,
                    help: Some(
                        "Verify that the target platform supports the specified calling convention. \
                         Use System V for Linux/macOS, or Microsoft x64 for Windows.".to_string()
                    ),
                }
            }
            CodeGenError::CfgVerificationFailed(msg) => {
                CompileError::AsmGeneratorError {
                    message: format!("Control flow graph verification failed: {}", msg),
                    span: None,
                    help: Some(
                        "The IR control flow graph is invalid. \
                         Ensure all basic blocks have terminators and all branch targets exist.".to_string()
                    ),
                }
            }
            CodeGenError::PhiResolutionFailed(msg) => {
                CompileError::AsmGeneratorError {
                    message: format!("SSA phi function resolution failed: {}", msg),
                    span: None,
                    help: Some(
                        "Failed to eliminate SSA phi functions through critical edge splitting. \
                         This may indicate a bug in the phi resolver.".to_string()
                    ),
                }
            }
            CodeGenError::IoError(io_err) => {
                CompileError::IoError(io_err)
            }
        }
    }
}
```

**Purpose**: Comprehensive error taxonomy for all failure modes in code generation, with automatic conversion to unified `CompileError` type for seamless compiler integration.

---

## Usage Patterns

### Basic Usage

```rust
use jsavrs::asm::generator::AsmGenerator;
use jsavrs::ir::{Module, TargetTriple};

// Create generator for target platform
let mut generator = AsmGenerator::new(TargetTriple::X86_64UnknownLinuxGnu);

// Generate assembly from IR module
let result = generator.generate(&ir_module);

// Check results
match (result.assembly, result.errors.is_empty()) {
    (Some(asm), true) => {
        // Complete success
        std::fs::write("output.asm", asm)?;
    }
    (Some(asm), false) => {
        // Partial success
        eprintln!("Warnings: {:?}", result.errors);
        std::fs::write("output.asm", asm)?;
    }
    (None, _) => {
        // Complete failure
        eprintln!("Generation failed: {:?}", result.errors);
    }
}
```

### Cross-Platform Generation

```rust
// Generate for multiple platforms from same IR
let targets = vec![
    TargetTriple::X86_64UnknownLinuxGnu,
    TargetTriple::X86_64PcWindowsMsvc,
    TargetTriple::X86_64AppleDarwin,
];

for target in targets {
    let mut generator = AsmGenerator::new(target);
    let result = generator.generate(&ir_module);
    
    if let Some(asm) = result.assembly {
        let filename = format!("output_{}.asm", target.name());
        std::fs::write(filename, asm)?;
    }
}
```

### Error Analysis

```rust
let result = generator.generate(&ir_module);

// Categorize errors by type
let mut unsupported_types = vec![];
let mut malformed_instructions = vec![];
let mut register_failures = vec![];

for error in &result.errors {
    match error {
        CodeGenError::UnsupportedType { ty, location } => {
            unsupported_types.push((ty, location));
        }
        CodeGenError::MalformedInstruction { reason, location } => {
            malformed_instructions.push((reason, location));
        }
        CodeGenError::RegisterAllocationFailed(msg) => {
            register_failures.push(msg);
        }
        _ => {}
    }
}

// Report grouped errors
if !unsupported_types.is_empty() {
    eprintln!("Unsupported types:");
    for (ty, loc) in unsupported_types {
        eprintln!("  - {} at {}", ty, loc);
    }
}
```

---

## Performance Characteristics

- **Time Complexity**: O(n) where n = number of IR instructions
  - Liveness analysis: O(n * e) where e = CFG edges (typically 2-3 passes)
  - Register allocation: O(n) for linear scan
  - Instruction selection: O(n) for pattern matching
  - Overall: O(n) amortized

- **Space Complexity**: O(n + r) where r = number of registers
  - IR representation: O(n)
  - Live intervals: O(n)
  - Register assignment: O(r)
  - Assembly output: O(n) (approximately 1.5-3x IR instruction count)

- **Performance Target**: <1 second per 1000 IR instructions on standard hardware (best-effort)

---

## Thread Safety

- `AsmGenerator` is **not** thread-safe (contains mutable state)
- To generate for multiple modules concurrently, create separate `AsmGenerator` instances per thread
- IR `Module` can be shared (read-only) across generators

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-10-17 | Initial API specification |

---

**Contract Reviewed**: 2025-10-17  
**Stability**: Stable - Breaking changes will increment major version
