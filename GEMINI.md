
# Project Overview

`jsavrs` is a compiler written in Rust. It takes `.vn` files as input and compiles them. The project uses several external libraries, including:

*   `clap` for parsing command-line arguments.
*   `logos` for lexical analysis.
*   `petgraph` for creating and managing graph data structures, likely for an Intermediate Representation (IR).
*   `insta` for snapshot testing.

The compiler pipeline consists of the following stages:
1.  **Lexical Analysis:** The input file is read and converted into a stream of tokens by the `Lexer`.
2.  **Parsing:** The tokens are parsed into an Abstract Syntax Tree (AST) by the `JsavParser`.
3.  **Type Checking:** The `TypeChecker` traverses the AST to ensure type correctness.
4.  **Intermediate Representation (IR) Generation:** The AST is transformed into a custom IR by the `NIrGenerator`.
5.  **High-level Intermediate Representation (HIR) Transformation:** The AST is also transformed into a HIR by the `AstToHirTransformer`.

# Building and Running

## Building the Project

To build the project, run the following command:

```sh
cargo build
```

For an optimized release build, use:

```sh
cargo build --release
```

## Running the Compiler

To run the compiler, provide an input file with the `.vn` extension:

```sh
cargo run -- -i <path/to/your/file.vn>
```

You can also enable verbose output to see more details about the compilation process:

```sh
cargo run -- -i <path/to/your/file.vn> --verbose
```

## Running Tests

The project has a comprehensive test suite. To run all tests, use:

```sh
cargo test
```

The project uses `insta` for snapshot testing. If a snapshot test fails, you can review the changes and accept them with:

```sh
cargo insta review
```

## Formatting the Code

The project uses `rustfmt` to maintain a consistent code style. To format the code, run:

```sh
cargo fmt
```

# Development Conventions

## Code Style

The project follows the standard Rust code style, enforced by `rustfmt`.

## Testing

The project uses a combination of unit tests and snapshot tests.

*   **Unit Tests:** These are located in the `tests` directory and test individual components of the compiler.
*   **Snapshot Tests:** These use the `insta` crate to capture the output of the compiler and compare it against a stored snapshot. This is useful for ensuring that the output of the lexer, parser, and IR generator remains consistent.

When adding new features or fixing bugs, it is important to add corresponding tests to ensure the correctness of the implementation.

## Error Handling

The compiler uses a custom `CompileError` enum to represent different types of errors that can occur during compilation. The `ErrorReporter` is used to display these errors to the user in a user-friendly format.
