# Project Context for jsavrs

## Project Overview

**jsavrs** is a sophisticated, high-performance Rust-based OS-independent compiler/transpiler written in Rust. It's designed to take source code written in a custom language (with `.vn` file extension) and compile it through multiple phases: lexical analysis, parsing, semantic analysis, intermediate representation (IR) generation, and finally assembly code generation. The project leverages Rust's safety and performance features to provide a robust and efficient compilation framework.

### Architecture & Components

The project follows a classic compiler architecture with distinct phases:

1. **Lexer** (`src/lexer.rs`): Uses the `logos` crate to tokenize input source code. Handles character scanning, token recognition, and error reporting with line/column tracking.

2. **Parser** (`src/parser/`): Implements a Pratt parser for handling expressions with proper precedence. Converts tokens into an Abstract Syntax Tree (AST) with error recovery capabilities and recursion depth limits.

3. **Semantic Analysis** (`src/semantic/type_checker.rs`): Performs type checking, symbol table management, and validation of the AST. Implements scope management, type promotion, and ensures type safety across the entire program.

4. **Intermediate Representation (IR)** (`src/ir/`): Generates a Control Flow Graph (CFG) based IR with basic blocks, instructions, and SSA (Static Single Assignment) transformation.

5. **Assembly Generation** (`src/asm/`): Translates IR into x86 assembly code with support for multiple platforms and ABIs.

6. **Command Line Interface** (`src/cli.rs`): Built with `clap` for parsing command-line arguments, enforcing `.vn` file extension, and providing verbose output options.

### Key Features

- High-performance implementation in Rust
- Cross-platform compatibility (Windows, macOS, Linux)
- Modular and extensible architecture
- Complete safety through Rust's memory and thread safety guarantees
- Support for multiple numeric types, arrays, functions, control flow structures

## Project Structure

```text
jsavrs/
├── src/                  # Source code
│   ├── cli.rs            # Command-line interface
│   ├── lexer.rs          # Lexer implementation
│   ├── lib.rs            # Library exports
│   ├── main.rs           # Main entry point
│   ├── asm/              # Assembly generation helpers
│   ├── error/            # Error handling types & helpers
│   ├── ir/               # Intermediate representation (IR)
│   ├── location/         # Source location tracking
│   ├── parser/           # Parser and AST
│   ├── printers/         # AST / IR printers
│   ├── semantic/         # Semantic analysis (type checking)
│   ├── time/             # Timing utilities
│   ├── tokens/           # Token definitions
│   └── utils/            # Misc utilities
├── tests/                # Unit & integration tests (many files + snapshots)
│   └── snapshots/        # insta snapshots used by tests
├── benches/              # Criterion benchmarks (e.g. jsavrs_benchmark.rs)
├── asm_output/           # Example / generated .asm outputs (not source)
├── vn_files/             # Example .vn source files used by tests/examples
├── specs/                # Design notes, RFCs and proposal directories
├── .github/              # CI / GitHub workflows
│   └── workflows/
├── Cargo.toml            # Rust package manifest
├── README.md             # Project documentation
├── QWEN.md               # This file (project context & docs)
├── AGENTS.md             # Agent-based dev notes (project-specific)
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── LICENSE
├── rustfmt.toml
└──  sonar-project.properties
```

## Technology Stack

- **Primary Language**: Rust 2024 edition (requires Rust 1.85+)
- **Lexer**: Logos crate for efficient tokenization with regex-based pattern matching
- **CLI**: Clap for sophisticated command-line argument parsing with custom styling
- **Testing**: Built-in Rust testing framework with insta for snapshot testing
- **Benchmarking**: Criterion.rs for performance benchmarking
- **Error Handling**: Thiserror for ergonomic error type definitions
- **Dependencies**:

    - clap (CLI parsing with derive macros)
    - console (terminal styling and formatting)
    - logos (lexer with high-performance tokenization)
    - thiserror (error handling with automatic implementation)
    - regex (regular expression support)
    - lazy_static (lazy initialization of static values)
    - uuid (universally unique identifier generation)
    - petgraph (graph data structures for IR representation)
    - insta (snapshot testing for output validation)
    - criterion (performance benchmarking and optimization validation)

## Language Features (.vn files)

The compiler supports a rich custom language with these features:

### Functions

- Typed parameters and return types with explicit type annotations
- Support for main function as program entry point
- Function declarations with `fun` keyword

### Variables and Constants

- Explicit typing with `var` (mutable) and `const` (immutable) declarations
- Type inference for initializers
- Support for multiple variable declarations in a single statement

### Numeric Types

- Signed integers: i8, i16, i32, i64
- Unsigned integers: u8, u16, u32, u64
- Floating-point: f32, f64
- Literal suffixes for explicit typing (e.g., 42u, 3.14f)
- Scientific notation support (e.g., 6.022e23)
- Base-specific literals: binary (#b1010), octal (#o755), hexadecimal (#xdeadbeef)

### Data Types

- Character literals ('A', '\n', '\u{1F600}')
- String literals ("Hello, World!")
- Boolean values (true, false)
- Null pointer literal (nullptr)

### Control Structures

- Conditional statements (if/else)
- Loop constructs (while, for)
- Break and continue statements for loop control
- Block scoping with curly braces

### Arrays and Collections

- Fixed-size arrays with explicit sizing (var arr: i64[5] = {1, 2, 3, 4, 5})
- Multi-dimensional arrays
- Array access with bracket notation

### Comments

- Single-line comments (// This is a comment)
- Multi-line comments (/*This is a multi-line comment*/)

Example syntax:

```rust
fun add(num1: i8, num2: i8): i8 {
    return num1 + num2
}

main {
    var x: i64 = 1 + 4 - (12 + 3) / 3
    var y: i8 = 12i8
    var arr: i64[5] = {1, 2, 3, 4, 5}
    
    if (x >= 10) {
        var result: i8 = add(5i8, 3i8)
    } else {
        x = factorial(5)
    }
}
```

## Architecture

1. **Lexical Analysis**: Tokenizes source code using Logos
2. **Parsing**: Builds AST from tokens using a recursive descent parser
3. **Semantic Analysis**: Type checking and symbol resolution
4. **IR Generation**: Creates multiple intermediate representations
5. **Code Generation**: (Planned) Assembly or other target code generation

## Development Workflow

### Prerequisites

- Rust toolchain (rustup recommended for version management)
- Cargo package manager (included with Rust)
- Git for version control

### Building

```bash
# Development build with debug symbols
cargo build

# Release build with optimizations
cargo build --release

# Build with all features
cargo build --all-features
```

### Running

```bash
# Run with input file
cargo run -- -i input.vn

# Run with verbose output for detailed compilation information
cargo run -- -i input.vn -v

# Run release build for maximum performance
cargo run --release -- -i input.vn
```

### Testing

```bash
# Run all tests (unit, integration, and documentation)
cargo test

# Run tests with output capture disabled to see println! output
cargo test -- --nocapture

# Run specific test suite
cargo test lexer

# Update snapshot tests when output changes are expected
cargo insta test --accept

# Review snapshot test differences
cargo insta review
```

### Code Quality

```bash
# Format code according to rustfmt standards
cargo fmt

# Run clippy lints to catch common mistakes and improve code quality
cargo clippy --all-features --verbose -- -D warnings

# Run benchmarks to measure performance
cargo bench

# Generate documentation
cargo doc --open
```

### Profiling and Performance Analysis

```bash
# Run with time tracking for performance analysis
cargo run --release -- -i large_toy_program.vn -v

# Profile with external tools like DHAT or FlameGraph
```

## CI/CD

The project uses GitHub Actions for comprehensive continuous integration with:

- **Cross-platform Testing**: Automated testing on Windows, macOS, and Linux
- **Multiple Rust Versions**: Testing against stable, beta, and nightly Rust versions
- **Code Coverage**: Integration with Codecov for coverage reporting
- **Linting**: Automated clippy linting with strict warning policies
- **Build Verification**: Ensuring successful compilation across all targets
- **Security Scanning**: Automated security scanning for dependencies
- **Documentation Generation**: Automated documentation building and deployment

## Contributing

We welcome contributions from the community to improve jsavrs. To ensure a smooth contribution process:

1. **Fork the Repository**
   - Create your own fork of the jsavrs repository on GitHub

2. **Create a Feature Branch**
   - Use descriptive branch names (e.g., `feature/add-array-support`, `fix/type-checker-bug`)

3. **Implement Changes**
   - Follow the existing coding standards and architectural patterns
   - Add comprehensive documentation for new features
   - Ensure all existing tests pass

4. **Add Tests**
   - Write unit tests for new functionality
   - Add integration tests where appropriate
   - Update snapshot tests if output changes are expected

5. **Run Quality Checks**
   - Format code with `cargo fmt` mandatory
   - Run clippy lints with `cargo clippy --all-features -- -D warnings` mandatory
   - Execute full test suite with `cargo test`

6. **Submit a Pull Request**
   - Provide a clear description of changes
   - Reference any related issues
   - Ensure CI checks pass

### Code Style Standards

- Follow Rust community standards and idioms
- Use `cargo fmt` to enforce consistent formatting.
- Resolve all Clippy warnings; if a lint is intentionally suppressed, document the justification.
- Write clear, descriptive commit messages
- Document public APIs using rustdoc comments, providing examples and testable snippets where appropriate.

### Testing Guidelines

- Write unit tests for individual functions and modules
- Use insta snapshot testing for output validation
- Include edge cases and error conditions in tests
- Maintain high test coverage for critical components
- Use property-based testing where appropriate and applicable

All code should be formatted with `cargo fmt` and pass `cargo clippy` checks before submission.
