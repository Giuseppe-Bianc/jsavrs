# API Contract: SSA-based IR Validator

## Overview
This document specifies the API contracts for the SSA-based IR validator with Control Flow Graph (CFG) validation. The validator ensures structural invariants (variable usage, loops, reachability), semantic invariants (type consistency, valid operands), and CFG integrity (proper construction, entry/exit nodes).

## Module Structure
```
src/
└── ir/
    └── validator/
        ├── mod.rs
        ├── structural.rs
        ├── semantic.rs
        ├── cfg.rs
        └── diagnostics.rs
```

## Public API

### 1. `IrValidator` struct

#### Definition
```rust
pub struct IrValidator {
    config: ValidationConfig,
    diagnostics: Vec<Diagnostic>,
}
```

#### Constructor
```rust
impl IrValidator {
    pub fn new(config: ValidationConfig) -> Self
}
```

**Parameters:**
- `config: ValidationConfig` - Configuration for the validator

**Returns:**
- `IrValidator` - A new instance of the validator

**Errors:**
- None

#### Validation Method
```rust
impl IrValidator {
    pub fn validate(&mut self, function: &Function) -> Result<ValidationResult, CompileError>
}
```

**Parameters:**
- `function: &Function` - The IR function to validate

**Returns:**
- `Result<ValidationResult, CompileError>` - Validation result or error if validation fails

**Errors:**
- `CompileError::IrGeneratorError` - If the validation process encounters an unexpected issue

**Side Effects:**
- Updates internal diagnostics collection
- May apply automatic fixes if enabled in config

### 2. `ValidationConfig` struct

#### Definition
```rust
pub struct ValidationConfig {
    pub enabled_checks: HashSet<ValidationErrorType>,
    pub collect_all_errors: bool,
    pub auto_fix_enabled: bool,
    pub precision_target: f64,
    pub max_lines_to_process: usize,
    pub max_errors_to_report: usize,
}
```

**Fields:**
- `enabled_checks: HashSet<ValidationErrorType>` - Set of validation checks to perform
- `collect_all_errors: bool` - Whether to collect all errors before stopping
- `auto_fix_enabled: bool` - Whether to attempt automatic fixes
- `precision_target: f64` - Target precision percentage for validation (default 95%)
- `max_lines_to_process: usize` - Maximum lines of IR to process
- `max_errors_to_report: usize` - Maximum number of errors to report

#### Default Implementation
```rust
impl Default for ValidationConfig {
    fn default() -> Self
}
```

### 3. `ValidationErrorType` enum

#### Definition
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationErrorType {
    StructuralVariableUseBeforeDefinition,
    StructuralUnreachableCode,
    StructuralLoopIntegrity,
    SemanticTypeMismatch,
    SemanticInvalidOperand,
    CfgInvalidNode,
    CfgInvalidEdge,
    CfgMissingEntryExit,
}
```

### 4. `ValidationResult` struct

#### Definition
```rust
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
    pub infos: Vec<ValidationError>,
    pub total_instructions_processed: usize,
    pub processing_time: Duration,
    pub validation_config: ValidationConfig,
    pub auto_fixes_performed: Vec<AutoFixInfo>,
}
```

**Fields:**
- `errors: Vec<ValidationError>` - List of validation errors found
- `warnings: Vec<ValidationError>` - List of validation warnings
- `infos: Vec<ValidationError>` - List of informational messages
- `total_instructions_processed: usize` - Number of IR instructions processed
- `processing_time: Duration` - Time taken for validation
- `validation_config: ValidationConfig` - Configuration used for this validation
- `auto_fixes_performed: Vec<AutoFixInfo>` - List of automatic fixes applied

#### Methods
```rust
impl ValidationResult {
    pub fn has_errors(&self) -> bool
    pub fn has_warnings(&self) -> bool
    pub fn total_issues(&self) -> usize
    pub fn errors(&self) -> &[ValidationError]
    pub fn warnings(&self) -> &[ValidationError]
    pub fn infos(&self) -> &[ValidationError]
    pub fn auto_fixes_performed(&self) -> &[AutoFixInfo]
}
```

### 5. `ValidationError` struct

#### Definition
```rust
pub struct ValidationError {
    pub id: String,
    pub error_type: ValidationErrorType,
    pub message: String,
    pub location: SourceSpan,
    pub severity: SeverityLevel,
    pub suggested_fix: Option<String>,
    pub help_text: Option<String>,
    pub related_locations: Vec<SourceSpan>,
}
```

**Fields:**
- `id: String` - Unique identifier for the validation error
- `error_type: ValidationErrorType` - Categorizes the type of validation error
- `message: String` - Human-readable description of the error
- `location: SourceSpan` - Location information (file, line, column) where error occurs
- `severity: SeverityLevel` - Severity of the error (Error, Warning, Info)
- `suggested_fix: Option<String>` - Suggested correction for the error
- `help_text: Option<String>` - Additional help and explanation
- `related_locations: Vec<SourceSpan>` - Additional locations relevant to the error

### 6. `SeverityLevel` enum

#### Definition
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeverityLevel {
    Error,
    Warning,
    Info,
}
```

### 7. `AutoFixInfo` struct

#### Definition
```rust
pub struct AutoFixInfo {
    pub fix_type: AutoFixType,
    pub description: String,
    pub original_location: SourceSpan,
    pub new_content: String,
    pub applied: bool,
}
```

**Fields:**
- `fix_type: AutoFixType` - Type of automatic fix applied
- `description: String` - Description of what was fixed
- `original_location: SourceSpan` - Location before the fix
- `new_content: String` - New content after the fix
- `applied: bool` - Whether the fix was actually applied

### 8. `AutoFixType` enum

#### Definition
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoFixType {
    VariableRename,
    ControlFlowAdjustment,
    TypeCoercion,
    PhiFunctionInsertion,
}
```

## Submodule APIs

### 1. Structural Validator (`structural.rs`)

#### Function Definition
```rust
pub fn validate_structural_invariants(
    function: &Function, 
    config: &ValidationConfig
) -> Result<Vec<ValidationError>, CompileError>
```

**Parameters:**
- `function: &Function` - The function to validate
- `config: &ValidationConfig` - Configuration controlling validation behavior

**Returns:**
- `Result<Vec<ValidationError>, CompileError>` - List of structural validation errors found

### 2. Semantic Validator (`semantic.rs`)

#### Function Definition
```rust
pub fn validate_semantic_invariants(
    function: &Function, 
    config: &ValidationConfig
) -> Result<Vec<ValidationError>, CompileError>
```

**Parameters:**
- `function: &Function` - The function to validate
- `config: &ValidationConfig` - Configuration controlling validation behavior

**Returns:**
- `Result<Vec<ValidationError>, CompileError>` - List of semantic validation errors found

### 3. CFG Validator (`cfg.rs`)

#### Function Definition
```rust
pub fn validate_cfg_integrity(
    cfg: &ControlFlowGraph, 
    config: &ValidationConfig
) -> Result<Vec<ValidationError>, CompileError>
```

**Parameters:**
- `cfg: &ControlFlowGraph` - The control flow graph to validate
- `config: &ValidationConfig` - Configuration controlling validation behavior

**Returns:**
- `Result<Vec<ValidationError>, CompileError>` - List of CFG validation errors found

### 4. Diagnostic Utilities (`diagnostics.rs`)

#### Function Definition
```rust
pub fn generate_suggested_fix(
    error_type: &ValidationErrorType,
    context: &DiagnosticContext
) -> Option<String>
```

**Parameters:**
- `error_type: &ValidationErrorType` - The type of validation error
- `context: &DiagnosticContext` - Context information for the error

**Returns:**
- `Option<String>` - A suggested fix for the error, if applicable

## CLI Interface Contract

### Command Line Arguments
The validator integrates with the existing CLI interface and adds the following:

- `--validate-only` - Run only validation without compilation
- `--validate-and-fix` - Run validation and apply safe automatic fixes
- `--checks <check-list>` - Enable specific validation checks (comma-separated)
- `--disable-checks <check-list>` - Disable specific validation checks (comma-separated)
- `--output-format <format>` - Specify output format (text, json, structured)
- `--show-fixes` - Show suggested automatic fixes

### Output Format
When using `--output-format json`, the validator outputs a JSON object with this structure:

```json
{
  "errors": [
    {
      "id": "string",
      "error_type": "ValidationErrorType",
      "message": "string",
      "location": {
        "file": "string",
        "start_line": "number",
        "start_column": "number",
        "end_line": "number",
        "end_column": "number"
      },
      "severity": "Error|Warning|Info",
      "suggested_fix": "string|undefined",
      "help_text": "string|undefined",
      "related_locations": ["SourceSpan"]
    }
  ],
  "warnings": ["ValidationError"],
  "infos": ["ValidationError"],
  "total_instructions_processed": "number",
  "processing_time_ms": "number",
  "auto_fixes_performed": [
    {
      "fix_type": "AutoFixType",
      "description": "string",
      "original_location": "SourceSpan",
      "new_content": "string",
      "applied": "boolean"
    }
  ]
}
```

## Error Handling Contract

All public functions in the validator module follow the error handling convention established in the jsavrs project:

1. Functions that can fail return `Result<T, CompileError>`
2. Functions that shouldn't fail panic in debug builds and return safe defaults in release builds
3. All errors contain relevant source location information when applicable

## Performance Contract

The validator must meet these performance requirements:
- Process up to 10,000 lines of IR code within 5 minutes
- Maintain 95% precision in validation (minimize false positives to 5% or less)
- Use memory efficiently without requiring excessive allocations
- Allow configuration to adjust performance vs thoroughness trade-offs

## Backward Compatibility Contract

This validator module is designed to be backward compatible:
- The public API will maintain stable method signatures
- New validation checks will be opt-in by default
- Configuration options will have sensible defaults
- New error types will be added as variants to existing enums