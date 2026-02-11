# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`jsavrs` is a Rust-based compiler that translates a custom language (.vn files) into assembly code. The compiler follows a traditional multi-phase architecture:

1. **Lexical Analysis** - Tokenizes source code using the `logos` crate
2. **Parsing** - Builds Abstract Syntax Trees (AST) from tokens
3. **Semantic Analysis** - Type checking and variable scoping
4. **Intermediate Representation (IR)** - Generates a SSA-form IR for optimization
5. **Optimization** - Constant folding, dead code elimination
6. **Code Generation** - Translates IR to target-specific assembly

## Key Commands for Development

### Building the Project

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests and review snapshot changes
cargo insta test --review

# Run specific test
cargo test test_name
```

### Running the Compiler

```bash
# Compile a .vn file
cargo run -- -i path/to/file.vn

# Compile with verbose output
cargo run -- -i path/to/file.vn -v
```

### Code Formatting

```bash
# Format code according to project standards
cargo fmt 
```

### Linting

```bash
# Check for linting issues
cargo clippy --all-targets --all-features -- -D warnings
```

## Architecture Overview

### Core Components

1. **Lexer (`src/lexer.rs`)** - Uses `logos` for efficient tokenization
2. **Parser (`src/parser/`)** - Recursive descent parser generating AST nodes
3. **IR Generator (`src/ir/generator.rs`)** - Translates AST to SSA-form IR
4. **Optimizer (`src/ir/optimizer/`)** - Applies optimization passes like constant folding and dead code elimination
5. **Assembly Generator (`src/codegen/asmgen.rs`)** - Converts IR to assembly code
6. **Assembly Builder (`src/asm/`)** - Structures assembly output with sections, directives, and instructions

### Key Data Structures

- **Tokens** - Defined in `src/tokens/token_kind.rs`
- **AST Nodes** - Defined in `src/parser/ast.rs`
- **IR Types** - Defined in `src/ir/types.rs`
- **IR Values** - Defined in `src/ir/value/mod.rs`
- **Instructions** - Defined in `src/ir/instruction.rs`
- **Functions** - Defined in `src/ir/function.rs`
- **Modules** - Defined in `src/ir/module.rs`

### Language Features Supported

The .vn language supports:

- Primitive types (i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, string)
- Variable declarations (mutable and immutable)
- Functions with parameters and return types
- Arithmetic, logical, and comparison operations
- Control flow (if/else, while, for loops)
- Arrays (single and multi-dimensional)
- Structured programming constructs (blocks, return statements)
- Loop control (break, continue)

## Development Workflow

### Adding New Language Features

1. Extend the lexer by adding new token patterns in `src/tokens/token_kind.rs`
2. Update the parser to handle new syntax in `src/parser/jsav_parser.rs`
3. Add AST node definitions in `src/parser/ast.rs`
4. Implement IR generation in `src/ir/generator.rs`
5. Add assembly generation support in `src/codegen/asmgen.rs`
6. Write tests in the `tests/` directory
7. Update snapshot tests with `cargo insta test --review`

### Optimization Development

The optimizer consists of multiple passes:

- Constant Folding (`src/ir/optimizer/constant_folding/`)
- Dead Code Elimination (`src/ir/optimizer/dead_code_elimination/`)

To add new optimization passes:

1. Create a new module in `src/ir/optimizer/`
2. Implement the `Phase` trait for your optimization
3. Register it in the pipeline in `src/main.rs`

## Testing

The project uses comprehensive snapshot testing with `insta` for AST, IR, and error output verification. Tests are located in the `tests/` directory.

To update snapshots after making changes:

```bash
cargo insta test --review
```

## Target Platforms

The compiler supports multiple ABIs:

- System V (Linux)
- Windows
- macOS

Assembly generation adapts to target-specific conventions through the `Abi` type in `src/asm/abi.rs`.

## Performance Considerations

Key performance aspects:

- Uses `logos` for fast lexical analysis
- Implements SSA form for efficient optimization
- Uses arena allocation patterns where possible
- Employs efficient data structures (petgraph for CFGs)

## Error Handling

Errors are categorized by phase:

- Lexer errors (`E0001` series)
- Parser/Syntax errors (`E1000` series)
- Semantic/Type errors (`E2000` series)
- IR generation errors (`E3000` series)

Error reporting is handled by `src/error/` modules with detailed span information for precise error location.
See AGENTS.md for project context and guidelines.
