# Project Context for jsavrs

## Project Overview

**jsavrs** is a Rust-based compiler/transpiler designed to be OS-independent. It's built to compile a custom language (with .vn file extension) into various target formats. The project emphasizes high performance, cross-platform compatibility, and extensibility.

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
│   ├── mlir/           # Multi-level intermediate representation
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

- **Primary Language**: Rust 2024 edition
- **Lexer**: Logos crate for efficient tokenization
- **CLI**: Clap for command-line argument parsing
- **Testing**: Built-in Rust testing framework with insta for snapshot testing
- **Benchmarking**: Criterion.rs
- **Dependencies**: 
  - clap (CLI parsing)
  - console (terminal styling)
  - logos (lexer)
  - thiserror (error handling)
  - regex, lazy_static, uuid, petgraph

## Language Features (.vn files)

The compiler supports a custom language with these features:
- Functions with typed parameters and return types
- Variables with explicit typing (var/const declarations)
- Multiple numeric types (i8, i16, i32, i64, u8, u16, u32, u64, f32, f64)
- Character and string literals
- Boolean values and operations
- Control structures (if/else, while, for loops)
- Arrays and multidimensional arrays
- Comments (single-line // and multi-line /* */)
- Binary, octal, and hexadecimal literals

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
4. **IR Generation**: Creates multiple intermediate representations (NIR, HIR)
5. **Code Generation**: (Planned) Assembly or other target code generation

## Development Workflow

### Prerequisites
- Rust toolchain (rustup recommended)
- Cargo package manager

### Building
```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Running
```bash
# Run with input file
cargo run -- -i input.vn

# Run with verbose output
cargo run -- -i input.vn -v
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Update snapshot tests
cargo insta test --accept
```

### Code Quality
```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy --all-features --verbose -- -D warnings

# Run benchmarks
cargo bench
```

## CI/CD

The project uses GitHub Actions for continuous integration with:
- Cross-platform testing (Windows, macOS, Linux)
- Multiple Rust versions (stable, beta, nightly)
- Code coverage reporting
- Clippy linting
- Build verification

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes following the coding standards
4. Add tests for new functionality
5. Run the full test suite
6. Submit a pull request

Code should be formatted with `cargo fmt` and pass `cargo clippy` checks.