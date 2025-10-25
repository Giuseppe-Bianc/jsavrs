# Data Model: SSA-based IR Validator

## Overview
This document specifies the data model for the SSA-based IR validator with Control Flow Graph (CFG) validation. The validator checks structural invariants (variable usage, loops, reachability), semantic invariants (type consistency, valid operands), and CFG integrity (proper construction, entry/exit nodes).

## Core Entities

### 1. ValidationError
Represents an issue found in the IR or CFG during validation.

**Fields:**
- `id: String` - Unique identifier for the validation error
- `error_type: ValidationErrorType` - Categorizes the type of validation error
- `message: String` - Human-readable description of the error
- `location: SourceSpan` - Location information (file, line, column) where error occurs
- `severity: SeverityLevel` - Severity of the error (Error, Warning, Info)
- `suggested_fix: Option<String>` - Suggested correction for the error
- `help_text: Option<String>` - Additional help and explanation
- `related_locations: Vec<SourceSpan>` - Additional locations relevant to the error

**Relationships:**
- Belongs to a specific IR function or module
- May be related to other ValidationError instances for complex issues

**Validation Rules:**
- `id` must be unique within a validation session
- `error_type` must be one of the predefined validation error types
- `location` must be a valid SourceSpan if provided
- `severity` must be one of Error, Warning, or Info

### 2. ValidationErrorType
Enumeration that categorizes different types of validation errors.

**Values:**
- `StructuralVariableUseBeforeDefinition` - Variable used before it's defined
- `StructuralUnreachableCode` - Code that cannot be reached from entry point
- `StructuralLoopIntegrity` - Issues with loop entry/exit points
- `SemanticTypeMismatch` - Type mismatch in operations or assignments
- `SemanticInvalidOperand` - Invalid operand for an operation
- `CfgInvalidNode` - CFG node that doesn't exist or is malformed
- `CfgInvalidEdge` - Invalid edge in the control flow graph
- `CfgMissingEntryExit` - Missing entry or exit nodes in CFG

### 3. SeverityLevel
Enumeration representing the severity of validation errors.

**Values:**
- `Error` - Critical issue that prevents compilation
- `Warning` - Potential issue that may cause problems
- `Info` - Informational message for awareness

### 4. ValidationConfig
Configuration that controls validation behavior.

**Fields:**
- `enabled_checks: HashSet<ValidationErrorType>` - Set of validation checks to perform
- `collect_all_errors: bool` - Whether to collect all errors before stopping
- `auto_fix_enabled: bool` - Whether to attempt automatic fixes
- `precision_target: f64` - Target precision percentage for validation (default 95%)
- `max_lines_to_process: usize` - Maximum lines of IR to process
- `max_errors_to_report: usize` - Maximum number of errors to report

**Validation Rules:**
- `precision_target` must be between 0.0 and 100.0
- `max_lines_to_process` and `max_errors_to_report` must be positive values

### 5. ValidationResult
Result of a validation run that includes all errors found and metadata.

**Fields:**
- `errors: Vec<ValidationError>` - List of validation errors found
- `warnings: Vec<ValidationError>` - List of validation warnings
- `infos: Vec<ValidationError>` - List of informational messages
- `total_instructions_processed: usize` - Number of IR instructions processed
- `processing_time: Duration` - Time taken for validation
- `validation_config: ValidationConfig` - Configuration used for this validation
- `auto_fixes_performed: Vec<AutoFixInfo>` - List of automatic fixes applied

**Validation Rules:**
- All ValidationError items must have valid source locations
- Processing time should be reasonable (less than 5 minutes for 10,000 lines)

### 6. AutoFixInfo
Information about an automatic fix applied during validation.

**Fields:**
- `fix_type: AutoFixType` - Type of automatic fix applied
- `description: String` - Description of what was fixed
- `original_location: SourceSpan` - Location before the fix
- `new_content: String` - New content after the fix
- `applied: bool` - Whether the fix was actually applied

### 7. AutoFixType
Enumeration representing types of automatic fixes.

**Values:**
- `VariableRename` - Renaming a variable to avoid conflicts
- `ControlFlowAdjustment` - Adjusting control flow to ensure reachability
- `TypeCoercion` - Automatically coercing types where safe
- `PhiFunctionInsertion` - Adding phi functions where needed

## State Transitions

### ValidationSession
Describes the states of a validation session:

1. **Initialized** → **Running**: When validation starts
2. **Running** → **Completed**: When all checks complete successfully
3. **Running** → **Failed**: When validation encounters an irrecoverable error
4. **Running** → **Stopped**: When validation is stopped manually (e.g., timeout)

## Relationships and Dependencies

### Validator Module Dependencies
The validator module depends on:
- `ir::cfg::ControlFlowGraph` - For CFG integrity validation
- `ir::ssa::SsaTransformer` - For understanding SSA form in structural validation
- `ir::instruction::Instruction` - For semantic validation of operations
- `ir::types::IrType` - For type consistency validation
- `ir::value::Value` - For variable definition/use tracking
- `error::compile_error::CompileError` - For error representation

### Output Dependencies
The validator provides data to:
- CLI module for command-line output
- Testing modules for validation tests
- Logging infrastructure for fix logs

## Data Flow

### Input to Validator
1. `ir::function::Function` - The function containing the IR to validate
2. `ValidationConfig` - Configuration specifying which checks to run
3. Optional `PathBuf` - Input file path for location information

### Processing
1. Structural validation examines variable definitions and uses
2. Semantic validation checks type consistency and operand validity
3. CFG validation verifies graph integrity and reachability
4. Errors are collected and categorized

### Output from Validator
1. `ValidationResult` - Complete result of the validation run
2. Optional auto-fixed IR if auto-fixing is enabled
3. Log of all automatic fixes performed

## Validation Rules Summary

### Structural Invariant Rules
- Each variable must have a definition before any use
- All control flow paths must be reachable from the entry point
- Loops must have appropriate entry and exit points

### Semantic Invariant Rules
- Operations must be executed with valid operands of compatible types
- Values must have compatible types according to the operation requirements

### CFG Integrity Rules
- Entry and exit nodes must exist and be accessible
- All referenced nodes in the CFG must exist
- Control flow edges must connect valid basic blocks