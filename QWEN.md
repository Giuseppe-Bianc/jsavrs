# Project Context for jsavrs

## Project Overview

**jsavrs** is a sophisticated, high-performance compiler implemented in Rust, designed to be completely OS-independent. It's engineered to compile a custom programming language (with `.vn` file extension) into multiple target formats, including intermediate representations and potentially assembly code. The project emphasizes performance, cross-platform compatibility, extensibility, and leverages Rust's safety guarantees to ensure reliability.

### Key Features
- High-performance implementation in Rust
- Cross-platform compatibility (Windows, macOS, Linux)
- Modular and extensible architecture
- Complete safety through Rust's memory and thread safety guarantees
- Support for multiple numeric types, arrays, functions, control flow structures

## Project Structure

```
jsavrs/
├── src/                 # Source code
│   ├── cli.rs          # Command-line interface
│   ├── lexer.rs        # Lexer implementation using Logos
│   ├── lib.rs          # Library exports
│   ├── main.rs         # Main entry point
│   ├── error/          # Error handling
│   ├── ir/             # Intermediate representation
│   ├── location/       # Source location tracking
│   ├── parser/         # Parser and AST
│   ├── printers/       # AST/HIR printers
│   ├── semantic/       # Semantic analysis (type checking)
│   ├── time/           # Timing utilities
│   ├── tokens/         # Token definitions
│   └── utils/          # Utility functions
├── tests/              # Test suite
├── benches/            # Benchmarking
├── Cargo.toml          # Rust package manifest
├── README.md           # Project documentation
└── .github/workflows/  # CI/CD workflows
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
- Multi-line comments (/* This is a multi-line comment */)

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
