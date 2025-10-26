# Quickstart Guide: SSA-based IR Validator

## Overview
This guide provides a quick introduction to using the SSA-based IR validator with Control Flow Graph (CFG) validation. The validator ensures structural invariants (variable usage, loops, reachability), semantic invariants (type consistency, valid operands), and CFG integrity (proper construction, entry/exit nodes).

## Prerequisites
- Rust 1.75 or higher
- Cargo package manager
- Access to the jsavrs source code

## Installation
The validator is built into the jsavrs compiler. No additional installation is required beyond building the compiler:

```bash
cd jsavrs
cargo build
```

## Building the Validator
To build the validator specifically:

```bash
cargo build --features validator
```

Or build the entire project:

```bash
cargo build --all
```

## Basic Usage

### Command Line Interface
The validator can be used directly through the command line:

```bash
# Basic validation of an IR file
cargo run -- -i path/to/input.ir --validate-only

# With verbose output showing all validation checks
cargo run -- -i path/to/input.ir --validate-only -v

# Validate with specific checks enabled
cargo run -- -i path/to/input.ir --validate-only --checks structural,semantic
```

### Library Usage
The validator can also be used as a library within Rust code:

```rust
use jsavrs::ir::validator::IrValidator;
use jsavrs::ir::function::Function;
use jsavrs::ir::validator::ValidationConfig;

// Create a validation configuration
let config = ValidationConfig::default();

// Load your IR function
let function = /* your function loading code */;

// Create and run the validator
let mut validator = IrValidator::new(config);
let result = validator.validate(&function)?;

// Check the results
if result.has_errors() {
    for error in result.errors() {
        println!("Error: {} at {}", error.message, error.location);
    }
}
```

## Configuration Options

### Validation Checks
You can enable/disable specific validation checks:

- `--checks structural,semantic,cfg` - Enable specific checks
- `--disable-checks <check-list>` - Disable specific checks
- `--validate-all` - Enable all validation checks (default)

### Output Options
- `-v, --verbose` - Enable verbose output
- `--output-format {text,json,structured}` - Specify output format
- `--show-fixes` - Show suggested automatic fixes

### Performance Options
- `--max-lines <number>` - Set maximum lines to process (default: 10000)
- `--precision-target <percentage>` - Set precision target (default: 95)

## Example Usage

### Validating a Simple Function
```bash
# Validate a function in SSA form
cargo run -- -i examples/simple_function.ir --validate-only
```

Example output:
```
Validation Result:
- Errors: 0
- Warnings: 2
- Processing time: 45ms

Warning: Variable 'x' used before definition at line 15
Suggested fix: Move the definition of 'x' before its use

Warning: Unreachable code detected at line 30
Suggested fix: Review control flow to remove unreachable block
```

### Using with Auto-Fix
```bash
# Validate and apply safe automatic fixes
cargo run -- -i examples/fixable_function.ir --validate-and-fix
```

### Library Integration Example
```rust
use jsavrs::ir::validator::{IrValidator, ValidationConfig, ValidationErrorType};

let config = ValidationConfig {
    enabled_checks: vec![
        ValidationErrorType::StructuralVariableUseBeforeDefinition,
        ValidationErrorType::CfgInvalidNode
    ],
    collect_all_errors: true,
    auto_fix_enabled: true,
    ..ValidationConfig::default()
};

let mut validator = IrValidator::new(config);
let result = validator.validate(&my_function)?;

// Process validation results
for error in result.errors() {
    println!("Found error: {}", error.message);
}

// Check if auto-fixes were applied
if !result.auto_fixes_performed().is_empty() {
    println!("Applied {} automatic fixes", result.auto_fixes_performed().len());
}
```

## Validation Modes

### Structural Validation
Checks variable definitions and uses, ensuring each variable is defined before use, and verifies control flow properties like loop entry/exit points.

### Semantic Validation  
Checks type consistency and ensures operations are executed with valid operands of compatible types.

### CFG Validation
Validates the proper construction of the Control Flow Graph, including the existence and accessibility of entry and exit nodes, and proper connectivity.

## Output Formats

The validator supports multiple output formats:

- **Text (default)**: Human-readable output with detailed error descriptions
- **JSON**: Structured output for programmatic processing
- **Structured**: Tabular format with error summaries

Use the `--output-format` flag to specify the desired format.

## Next Steps
1. Review the detailed data model in `data-model.md` for comprehensive understanding
2. Check the API contracts in `contracts/` for integration details
3. Look at the tests in `tests/ir/validator/` for usage examples
4. Consult the full API documentation for advanced usage patterns