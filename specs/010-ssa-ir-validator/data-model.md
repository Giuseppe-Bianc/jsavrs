# Data Model: SSA-based IR Validator

## Type Definitions & Imports
- `SourceSpan` - Location information (file, line, column); assumed to be imported from `location::source_span::SourceSpan` or similar
- `Duration` - Time measurement; assumed to be from standard library


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
- Example: A type mismatch error may relate to a use-before-definition error in the same expression
- Related errors share a common conceptual issue but are reported separately
- Use `related_locations` to cross-reference; if more complex relationships are needed, add a `parent_error_id: Option<String>` field

**Validation Rules:**
- `id` must be unique within a validation session
	- IDs should be generated sequentially or use a UUID-based scheme (specify preference)
	- Duplicate IDs within a single ValidationResult are invalid and must be rejected
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
  - Used for structural/semantic violations that make IR invalid (e.g., undefined variable, type mismatch)
- `Warning` - Potential issue that may cause problems
  - Used for suboptimal patterns or edge cases that don't prevent compilation (e.g., unused variables, inefficient loops)
- `Info` - Informational message for awareness
  - Used for diagnostic information (e.g., function has no loops, basic block has single predecessor)

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
-  Consider adding `timeout: Option<Duration>` to ValidationConfig if processing time is a concern

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
- If auto-fixing is enabled, `auto_fixes_performed` must not be empty unless auto-fixing was disabled or no fixes were applicable
- Each AutoFixInfo in `auto_fixes_performed` must have a corresponding original location in the source IR
- Auto-fixes with `applied: false` must include diagnostic information in their description field

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

**Applicability:**
- `VariableRename` - Triggered by StructuralVariableUseBeforeDefinition when safe renaming resolves conflicts
- `ControlFlowAdjustment` - Triggered by StructuralUnreachableCode or StructuralLoopIntegrity issues
- `TypeCoercion` - Triggered by SemanticTypeMismatch when safe implicit coercion is available
- `PhiFunctionInsertion` - Triggered by SSA form violations in CFG validation

**Values:**
- `VariableRename` - Renaming a variable to avoid conflicts
- `ControlFlowAdjustment` - Adjusting control flow to ensure reachability
- `TypeCoercion` - Automatically coercing types where safe
- `PhiFunctionInsertion` - Adding phi functions where needed

## State Transitions

### ValidationSession
Describes the states of a validation session:

1. **Initialized** → **Running**: When validation starts
   - Guard: Configuration must be valid (precision_target in range, check set non-empty if specified)
2. **Running** → **Completed**: When all checks complete successfully
   - Guard: All errors have been processed and reported
3. **Running** → **Failed**: When validation encounters an irrecoverable error
   - Guard: E.g., IR file not found, CFG malformed beyond recovery
   - Transitions are irreversible from Failed/Completed/Stopped
4. **Running** → **Stopped**: When validation is stopped manually (e.g., timeout)
   - Guard: Timeout exceeded or external stop signal received

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

**Processing Model:**
- Checks run sequentially in the order above; later checks may skip blocks if earlier checks found critical errors (controlled by `collect_all_errors` flag)
- If auto-fixing is enabled, fixes are attempted after all validation checks complete
- Auto-fix failures are logged but do not block validation result generation

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