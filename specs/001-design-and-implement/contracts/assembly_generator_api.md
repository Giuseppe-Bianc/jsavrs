# Assembly Generator API Contracts

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 2025-09-28  
**Status**: Phase 1 Design Complete

## Overview

This document defines the public API contracts for the x86-64 assembly code generator. These contracts serve as the interface specification between the generator and the rest of the jsavrs compiler infrastructure.

## Primary API Contracts

### 1. Main Assembly Generation Interface

```rust
/// Main entry point for x86-64 assembly generation
pub trait AssemblyCodeGenerator {
    /// Generate NASM-compatible assembly from IR module
    /// 
    /// # Arguments
    /// * `ir_module` - The IR module to translate
    /// * `target_platform` - Target platform configuration
    /// * `options` - Code generation options
    /// 
    /// # Returns
    /// * `Ok(String)` - Generated NASM assembly code
    /// * `Err(CodeGenError)` - Generation failure with details
    /// 
    /// # Requirements
    /// * Must complete within 5 seconds for modules ≤ 10,000 IR instructions
    /// * Memory usage must not exceed 2x input IR file size
    /// * Generated assembly must be NASM-assembleable on target platform
    fn generate_assembly(
        &mut self,
        ir_module: &IRModule,
        target_platform: TargetPlatform,
        options: CodeGenOptions,
    ) -> Result<String, CodeGenError>;

    /// Validate IR module compatibility
    /// 
    /// # Arguments
    /// * `ir_module` - The IR module to validate
    /// 
    /// # Returns
    /// * `Ok(())` - Module is supported
    /// * `Err(Vec<ValidationError>)` - List of unsupported constructs
    fn validate_ir_compatibility(
        &self,
        ir_module: &IRModule,
    ) -> Result<(), Vec<ValidationError>>;

    /// Get generator capabilities and limitations
    fn get_capabilities(&self) -> GeneratorCapabilities;
}

/// Implementation of the main assembly generator
pub struct X86AssemblyGenerator {
    // Internal state
}

impl AssemblyCodeGenerator for X86AssemblyGenerator {
    // Implementation details
}
```

### 2. Target Platform Configuration Contract

```rust
/// Platform-specific configuration for code generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetPlatform {
    pub os: TargetOS,
    pub arch: TargetArch,
    pub abi: ABISpec,
}

/// Supported target operating systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
}

/// Supported processor architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetArch {
    X86_64,
}

/// ABI specification for function calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABISpec {
    /// Windows x64 calling convention
    WindowsX64,
    /// System V ABI (Linux/macOS)
    SystemV,
}

impl TargetPlatform {
    /// Create Windows x64 target configuration
    pub fn windows_x64() -> Self;
    
    /// Create Linux x64 target configuration
    pub fn linux_x64() -> Self;
    
    /// Create macOS x64 target configuration  
    pub fn macos_x64() -> Self;
    
    /// Validate platform configuration
    pub fn validate(&self) -> Result<(), PlatformError>;
}
```

### 3. Code Generation Options Contract

```rust
/// Options for controlling assembly code generation
#[derive(Debug, Clone)]
pub struct CodeGenOptions {
    /// Optimization level (0 = debug, 1 = basic, 2 = aggressive)
    pub optimization_level: u8,
    
    /// Include debug information in assembly
    pub debug_info: bool,
    
    /// Generate human-readable comments
    pub include_comments: bool,
    
    /// Symbol naming prefix
    pub symbol_prefix: Option<String>,
    
    /// Maximum stack frame size (bytes)
    pub max_stack_frame_size: u32,
    
    /// Enable/disable specific instruction sets
    pub instruction_sets: InstructionSetFlags,
}

/// Bit flags for enabling instruction set extensions
#[derive(Debug, Clone, Copy)]
pub struct InstructionSetFlags {
    pub sse: bool,
    pub sse2: bool,
    pub avx: bool,
    pub avx2: bool,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            optimization_level: 1,
            debug_info: false,
            include_comments: true,
            symbol_prefix: None,
            max_stack_frame_size: 1024 * 1024, // 1MB
            instruction_sets: InstructionSetFlags {
                sse: true,
                sse2: true,
                avx: false,
                avx2: false,
            },
        }
    }
}
```

### 4. Error Handling Contract

```rust
/// Comprehensive error type for assembly generation failures
#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported IR instruction: {instruction} at {location}")]
    UnsupportedInstruction {
        instruction: String,
        location: SourceLocation,
    },
    
    #[error("Register allocation failed: {reason}")]
    RegisterAllocationFailure {
        reason: String,
        function: String,
    },
    
    #[error("Target platform not supported: {platform:?}")]
    UnsupportedPlatform {
        platform: TargetPlatform,
    },
    
    #[error("Stack frame size {size} exceeds limit {limit}")]
    StackOverflow {
        size: u32,
        limit: u32,
        function: String,
    },
    
    #[error("Type conversion error: cannot convert {from:?} to {to:?}")]
    TypeConversionError {
        from: IRType,
        to: X86Type,
        location: SourceLocation,
    },
    
    #[error("Symbol resolution failed: {symbol}")]
    UnresolvedSymbol {
        symbol: String,
        location: SourceLocation,
    },
    
    #[error("Internal generator error: {message}")]
    InternalError {
        message: String,
        backtrace: Option<std::backtrace::Backtrace>,
    },
}

/// Validation error for IR compatibility checking
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Unsupported IR instruction type: {instruction_type}")]
    UnsupportedInstructionType {
        instruction_type: String,
    },
    
    #[error("Unsupported data type: {data_type}")]
    UnsupportedDataType {
        data_type: String,
    },
    
    #[error("Function signature incompatible with target ABI")]
    IncompatibleFunctionSignature {
        function_name: String,
        reason: String,
    },
}

/// Source location information for error reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}
```

### 5. Generator Capabilities Contract

```rust
/// Information about generator capabilities and limitations
#[derive(Debug, Clone)]
pub struct GeneratorCapabilities {
    /// Supported target platforms
    pub supported_platforms: Vec<TargetPlatform>,
    
    /// Supported IR instruction types
    pub supported_instructions: Vec<String>,
    
    /// Supported data types
    pub supported_types: Vec<String>,
    
    /// Maximum function parameters
    pub max_function_parameters: usize,
    
    /// Maximum stack frame size
    pub max_stack_frame_size: u32,
    
    /// Performance characteristics
    pub performance_specs: PerformanceSpecs,
}

/// Performance specifications and guarantees
#[derive(Debug, Clone)]
pub struct PerformanceSpecs {
    /// Maximum generation time for 10K instructions (seconds)
    pub max_generation_time_10k_instr: f64,
    
    /// Memory overhead factor (multiplier of IR size)
    pub memory_overhead_factor: f64,
    
    /// Estimated instructions per second throughput
    pub instructions_per_second: u64,
}

impl GeneratorCapabilities {
    /// Check if platform is supported
    pub fn supports_platform(&self, platform: &TargetPlatform) -> bool;
    
    /// Check if IR instruction type is supported
    pub fn supports_instruction(&self, instruction_type: &str) -> bool;
    
    /// Check if data type is supported
    pub fn supports_type(&self, type_name: &str) -> bool;
}
```

## Secondary API Contracts

### 6. Register Management Contract

```rust
/// Public interface for register allocation information
pub trait RegisterInfo {
    /// Get available general-purpose registers for allocation
    fn available_gp_registers(&self) -> Vec<GPRegister>;
    
    /// Get available XMM registers for allocation
    fn available_xmm_registers(&self) -> Vec<XMMRegister>;
    
    /// Check if register is caller-saved (volatile)
    fn is_caller_saved(&self, register: Register) -> bool;
    
    /// Check if register is callee-saved (non-volatile)
    fn is_callee_saved(&self, register: Register) -> bool;
}

/// Register allocation statistics for debugging
#[derive(Debug, Clone)]
pub struct AllocationStats {
    pub registers_used: usize,
    pub registers_spilled: usize,
    pub stack_bytes_used: u32,
    pub allocation_pressure: f64, // 0.0 to 1.0
}
```

### 7. Calling Convention Contract

```rust
/// Public interface for calling convention information
pub trait CallingConventionInfo {
    /// Get the calling convention type
    fn convention_type(&self) -> CallingConventionType;
    
    /// Get parameter passing rules
    fn parameter_rules(&self) -> ParameterRules;
    
    /// Get return value rules
    fn return_rules(&self) -> ReturnRules;
    
    /// Get stack alignment requirement
    fn stack_alignment(&self) -> u32;
}

/// Parameter passing configuration
#[derive(Debug, Clone)]
pub struct ParameterRules {
    pub max_register_params: usize,
    pub integer_registers: Vec<GPRegister>,
    pub float_registers: Vec<XMMRegister>,
    pub stack_param_alignment: u32,
}

/// Return value configuration
#[derive(Debug, Clone)]
pub struct ReturnRules {
    pub integer_return_register: GPRegister,
    pub float_return_register: XMMRegister,
    pub large_struct_handling: LargeStructHandling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeStructHandling {
    ByReference,
    ByValue,
    FirstEightBytes,
}
```

### 8. Instruction Encoding Contract

```rust
/// Public interface for instruction encoding utilities
pub trait InstructionEncoder {
    /// Encode instruction to bytes (for verification)
    fn encode_instruction(&self, instruction: &X86Instruction) -> Result<Vec<u8>, EncodeError>;
    
    /// Validate instruction operand compatibility
    fn validate_instruction(&self, instruction: &X86Instruction) -> Result<(), ValidationError>;
    
    /// Get instruction size in bytes
    fn instruction_size(&self, instruction: &X86Instruction) -> Result<usize, EncodeError>;
}

/// Instruction encoding error
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error("Invalid operand combination for instruction {instruction}")]
    InvalidOperands { instruction: String },
    
    #[error("Operand size mismatch: expected {expected}, got {actual}")]
    SizeMismatch { expected: usize, actual: usize },
    
    #[error("Unsupported addressing mode")]
    UnsupportedAddressing,
}
```

## Testing Contracts

### 9. Semantic Equivalence Testing Contract

```rust
/// Interface for semantic equivalence validation
pub trait SemanticValidator {
    /// Compare IR execution results with assembly execution results
    fn validate_semantic_equivalence(
        &self,
        ir_module: &IRModule,
        assembly_code: &str,
        test_inputs: &[TestInput],
    ) -> Result<ValidationResult, ValidationError>;
    
    /// Generate test cases for semantic validation
    fn generate_test_cases(&self, ir_module: &IRModule) -> Vec<TestInput>;
}

/// Test input for semantic validation
#[derive(Debug, Clone)]
pub struct TestInput {
    pub function_name: String,
    pub parameters: Vec<TestValue>,
    pub expected_output: TestValue,
}

/// Test values for semantic validation
#[derive(Debug, Clone, PartialEq)]
pub enum TestValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Pointer(usize),
    Array(Vec<TestValue>),
    Struct(HashMap<String, TestValue>),
}

/// Result of semantic equivalence validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub test_results: Vec<TestCaseResult>,
    pub performance_metrics: PerformanceMetrics,
}

/// Individual test case result
#[derive(Debug, Clone)]
pub struct TestCaseResult {
    pub test_name: String,
    pub passed: bool,
    pub ir_result: Option<TestValue>,
    pub assembly_result: Option<TestValue>,
    pub error_message: Option<String>,
}
```

### 10. Performance Testing Contract

```rust
/// Interface for performance benchmarking
pub trait PerformanceBenchmark {
    /// Benchmark assembly generation speed
    fn benchmark_generation_speed(
        &self,
        ir_modules: &[IRModule],
    ) -> Result<BenchmarkResult, BenchmarkError>;
    
    /// Benchmark memory usage during generation
    fn benchmark_memory_usage(
        &self,
        ir_module: &IRModule,
    ) -> Result<MemoryProfile, BenchmarkError>;
    
    /// Benchmark generated code quality
    fn benchmark_code_quality(
        &self,
        assembly_code: &str,
    ) -> Result<CodeQualityMetrics, BenchmarkError>;
}

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub instructions_per_second: f64,
    pub total_generation_time: Duration,
    pub memory_peak_usage: usize,
    pub memory_final_usage: usize,
}

/// Memory usage profiling information
#[derive(Debug, Clone)]
pub struct MemoryProfile {
    pub peak_memory_bytes: usize,
    pub final_memory_bytes: usize,
    pub allocation_count: usize,
    pub memory_overhead_factor: f64,
}

/// Code quality metrics
#[derive(Debug, Clone)]
pub struct CodeQualityMetrics {
    pub instruction_count: usize,
    pub code_size_bytes: usize,
    pub optimization_opportunities: Vec<String>,
    pub abi_compliance_score: f64,
}
```

## Contract Validation Rules

### API Contract Constraints

1. **Performance Contracts**:
   - `generate_assembly()` MUST complete within 5 seconds for ≤ 10,000 IR instructions
   - Memory usage MUST NOT exceed 2x input IR file size
   - Error reporting MUST include precise location information

2. **Correctness Contracts**:
   - Generated assembly MUST be assembleable by NASM on target platform
   - Semantic equivalence MUST be maintained for all supported IR constructs
   - ABI compliance MUST be verified through automated testing

3. **Extensibility Contracts**:
   - All traits MUST use associated types for future extension
   - Error types MUST be exhaustive and actionable
   - Platform support MUST be query-able at runtime

4. **Safety Contracts**:
   - All public APIs MUST be memory-safe (no unsafe code in public interface)
   - Error handling MUST be comprehensive (no panics in normal operation)
   - Input validation MUST prevent invalid state creation

### Contract Testing Requirements

1. **Unit Tests**: Each contract interface must have comprehensive unit tests
2. **Integration Tests**: Cross-contract interactions must be tested
3. **Property Tests**: Contract invariants must be verified with property-based testing
4. **Performance Tests**: Performance contracts must be validated under load
5. **Compatibility Tests**: Cross-platform contracts must be tested on all target platforms

These contracts provide a comprehensive public API specification for the x86-64 assembly code generator, ensuring clear interfaces, comprehensive error handling, and maintainable integration with the broader jsavrs compiler infrastructure.