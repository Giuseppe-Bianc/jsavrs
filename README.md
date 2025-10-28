# jsavrs

[![Rust CI](https://github.com/Giuseppe-Bianc/jsavrs/actions/workflows/rust.yml/badge.svg)](https://github.com/Giuseppe-Bianc/jsavrs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Giuseppe-Bianc/jsavrs/graph/badge.svg?token=5EIG6IbpPa)](https://codecov.io/gh/Giuseppe-Bianc/jsavrs)
[![Lines of Code](https://sonarcloud.io/api/project_badges/measure?project=Giuseppe-Bianc_jsavrs&metric=ncloc)](https://sonarcloud.io/summary/new_code?id=Giuseppe-Bianc_jsavrs)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/Giuseppe-Bianc/jsavrs?utm_source=oss&utm_medium=github&utm_campaign=Giuseppe-Bianc%2Fjsavrs&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

## Introduction

`jsavrs` is a Rust-based project designed to be an OS-independent compiler. It aims to provide a robust,
high-performance solution for compiling code across multiple platforms. The project leverages Rust's safety and
concurrency features to ensure reliability and efficiency.

Whether you're a developer looking for a customizable compiler or someone interested in exploring Rust's capabilities,
`jsavrs` offers a modular and extensible framework to meet your needs.

## Features

- **High Performance**: The implementation in Rust enables `jsavrs` to achieve optimal compilation speeds through efficient memory management and processor utilization. The framework incorporates advanced optimization techniques that reduce compilation time while maintaining the integrity of the generated output. Performance benchmarks demonstrate significant improvements over traditional compiler implementations, particularly in projects with extensive codebases.

- **Cross-Platform Compatibility**: `jsavrs` operates seamlessly across major operating systems including Windows, macOS, and various Linux distributions. This cross-platform functionality is achieved through abstraction layers that isolate system-specific operations, ensuring consistent behavior regardless of the host environment. The compatibility extends to different processor architectures, supporting both x86 and ARM-based systems.

- **User-Friendly Interface**: The framework provides a well-structured application programming interface (API) that simplifies integration with existing development environments. The API design follows established software engineering principles, with clear separation of concerns and intuitive method signatures. Comprehensive documentation accompanies the framework, offering detailed explanations, practical examples, and best practices to facilitate implementation.

- **Extensible Architecture**: Built with a modular design philosophy, `jsavrs` allows for straightforward customization and extension. The component-based structure enables developers to modify specific compilation phases, add support for new programming languages, or integrate specialized optimization passes without requiring extensive reengineering of the core system. This extensibility is supported by well-defined interfaces and comprehensive documentation of the extension points.

- **Complete Rust Implementation**: The entire codebase is implemented in Rust, leveraging the language's stringent compile-time safety guarantees, including memory safety and thread safety. This implementation choice eliminates entire classes of vulnerabilities commonly found in compilers implemented in less safe languages. The use of Rust's ownership model and concurrency primitives ensures predictable performance characteristics, even under heavy workloads.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Basic Usage](#basic-usage)
    - [Advanced Usage](#advanced-usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation

### Prerequisites

- **Rust Programming Language**: A functional Rust development environment is required for building and utilizing `jsavrs`. Rust is a systems programming language that emphasizes performance, reliability, and productivity. The recommended installation method is through [rustup](https://rustup.rs/), the official Rust toolchain installer. Rustup manages Rust versions and associated components, ensuring compatibility and facilitating updates. The installation process establishes the Rust compiler, cargo (the package manager), and standard libraries.

- **Cargo Package Manager**: Cargo is included with Rust installations and serves as the primary build system and dependency manager for Rust projects. It automates the process of downloading, compiling, and managing dependencies, while also providing standardized workflows for building, testing, and documenting Rust applications. Proficiency with Cargo commands is essential for effective utilization of the `jsavrs` framework.

### Installation Procedure

The following procedure outlines the steps necessary to obtain, compile, and verify the `jsavrs` compiler framework. These instructions assume familiarity with command-line operations and basic software development practices.

1. **Repository Acquisition**
   
   The initial step involves obtaining a local copy of the source code repository. This is accomplished through the Git version control system, which facilitates tracking of changes and collaboration among developers. Execute the following commands in your terminal:
   ```bash
   git clone https://github.com/Giuseppe-Bianc/jsavrs.git
   cd jsavrs
   ```
   The first command creates a local copy of the repository in a directory named `jsavrs`, while the second command changes the current working directory to this newly created directory.

2. **Project Compilation**
   
   Once the repository has been obtained, the source code must be compiled into executable form. This process is managed by Cargo, which resolves dependencies, compiles source files, and links the resulting object files. For an optimized production build, execute:
   ```bash
   cargo build --release
   ```
   This command instructs Cargo to compile the project in release mode, which applies performance optimizations at the cost of increased compilation time. The resulting executable files are placed in the `target/release` directory within the project structure.

3. **Verification Through Testing**
   
   To ensure the correct functioning of the compiled framework, the included test suite should be executed. This comprehensive suite validates the behavior of individual components and their interactions, providing assurance of the system's reliability. Execute the following command:
   ```bash
   cargo test
   ```
   Cargo will compile and run all tests, reporting the results in a structured format. Successful completion of the test suite indicates that the framework is functioning as intended in the current environment.

## Usage

### Basic Usage

The `jsavrs` compiler operates through a command-line interface that accepts source files and various configuration parameters. The fundamental usage pattern involves specifying an input file for compilation:

```bash
./jsavrs -i input_file.vn
```

In this example, `input_file.vn` represents a source file written in a language supported by the compiler. The `-i` flag explicitly designates the input file, though this may be optional in some configurations. Upon execution, the compiler processes the source code through its various phases—including lexical analysis, syntax parsing, semantic analysis, optimization, and code generation—producing an output file in the designated location. The specific format and location of the output depend on the compiler's configuration and the target platform.

### Advanced Usage

The compiler framework provides numerous options for customizing the compilation process. These options enable users to control output generation, diagnostic reporting, optimization levels, and other aspects of compilation behavior.

* **Output Directory Specification**
  
  By default, the compiler places generated files in a predetermined location. Users may specify an alternative output directory using the `--output` parameter:
  ```bash
  ./jsavrs input_file.vn --output ./build
  ```
  This command directs the compiler to place all generated files in the `./build` directory, creating it if necessary. This functionality facilitates integration with complex build systems and project structures.

* **Verbose Logging**
  
  For diagnostic purposes or detailed understanding of the compilation process, the `--verbose` flag enables comprehensive logging:
  ```bash
  ./jsavrs input_file.vn --verbose
  ```
  When enabled, this option produces detailed information about each compilation phase, including intermediate representations, optimization decisions, and resource utilization metrics. This information is valuable for performance analysis, debugging, and educational purposes.

* **Multiple File Compilation**
  
  The compiler supports processing multiple source files in a single invocation:
  ```bash
  ./jsavrs file1.vn file2.vn file3.vn
  ```
  This capability is particularly useful for projects with modular codebases, ensuring consistent compilation settings across all components and potentially enabling cross-module optimizations.

* **Comprehensive Option Reference**
  
  A complete listing of available command-line options and their functions can be obtained using the built-in help system:
  ```bash
  ./jsavrs --help
  ```
  This command displays detailed documentation for all supported parameters, including syntax, default values, and usage examples. This reference serves as the definitive guide for compiler configuration.

## Testing

The `jsavrs` project incorporates a comprehensive testing methodology designed to ensure correctness, reliability, and performance. The test suite encompasses multiple levels of verification, including unit tests for individual functions, integration tests for component interactions, and regression tests for known issues. This multi-faceted approach to quality assurance provides confidence in the compiler's behavior across diverse scenarios and input conditions.

To execute the complete test suite, utilize the following command:
```bash
cargo test
```

This command compiles and runs all tests defined within the project, presenting a summary of results including the number of tests passed, failed, and ignored. Detailed output is available for failed tests, facilitating diagnosis and resolution of issues.

### Test Development Guidelines

When extending the compiler or addressing defects, corresponding tests should be developed to verify the correctness of the implementation. All test files reside within the `tests` directory and adhere to Rust's testing conventions. This systematic approach to test development ensures that new functionality operates as intended and that modifications do not introduce unintended side effects.

#### Example of a Unit Test

A unit test verifies the correctness of an individual function or module in isolation. The following example
demonstrates a simple unit test:

```rust
#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}
```

In this example, the test confirms that the arithmetic operation produces the expected result. Unit tests should be
concise, deterministic, and focused on verifying a single aspect of functionality.

By consistently integrating new tests alongside feature development and bug fixes, developers contribute to a robust,
maintainable, and reliable codebase.

### Snapshot Testing

The `jsavrs` project incorporates snapshot testing through the `insta` library to verify the correctness of compiler outputs and intermediate representations. Snapshot testing captures the output of a function or process and compares it against a previously approved reference, ensuring that changes to the codebase do not inadvertently alter expected behavior. This approach is particularly valuable for compiler development, where outputs such as abstract syntax trees, intermediate representations, and generated code must maintain consistency across modifications.

To execute snapshot tests alongside the standard test suite:
```bash
cargo test
```

When snapshot tests detect differences between current output and stored references, the `insta` tool provides facilities for reviewing and accepting changes:
```bash
cargo insta review
# or in alternative cargo insta test  --review
```

This command presents each detected difference, allowing developers to approve legitimate changes or reject unintended modifications. Accepted snapshots are automatically updated in the repository.

#### Example of a Snapshot Test

A snapshot test verifies that a function's output remains consistent with established expectations:

```rust
use insta::assert_snapshot;

#[test]
fn test_parser_output() {
    let input = "fn main() { return 42; }";
    let ast = parse(input);
    assert_snapshot!(ast);
}
```

In this example, the test captures the abstract syntax tree produced by the parser and compares it against the stored snapshot. Snapshot tests should focus on verifiable outputs that are expected to remain stable across routine maintenance activities.

## Contributing

The `jsavrs` project welcomes contributions from the development community and recognizes that collaborative efforts enhance the quality, functionality, and sustainability of the framework. Individuals interested in contributing are encouraged to participate through various channels, including issue reporting, feature suggestions, code contributions, and documentation improvements.

All submissions undergo a thorough review process to ensure they align with the project's technical standards, design principles, and long-term objectives. To facilitate efficient evaluation and integration, contributors should provide clear, comprehensive documentation of their submissions, including detailed descriptions of problems addressed, implementation approaches, and potential impacts on existing functionality.

### Contribution Process

1. **Repository Forking**
   
   Begin by creating a personal fork of the project repository. This establishes an independent development environment where changes can be implemented and tested without affecting the main codebase.

2. **Branch Creation**
   
   Create a dedicated branch for your contribution, employing a naming convention that reflects the nature of the work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
   This isolation prevents interference with other development efforts and simplifies the eventual integration process.

3. **Implementation**
   
   Develop your modifications following established coding standards, architectural patterns, and best practices. Ensure that your implementation addresses the intended requirements while maintaining compatibility with existing functionality.

4. **Testing**
   
   Develop appropriate tests to verify the correctness of your implementation. Execute the complete test suite to confirm that your changes do not introduce regressions:
   ```bash
   cargo test
   ```
   Comprehensive testing is essential to validate the functionality and reliability of contributions.

5. **Pull Request Submission**
   
   Once your implementation is complete and tested, submit a pull request for review. Provide a detailed description of your changes, including the rationale, approach, and any relevant considerations that may assist reviewers in evaluating your submission.

### Code Style Standards

To maintain consistency and readability throughout the codebase, all contributions must adhere to established formatting conventions. The project utilizes `rustfmt`, Rust's official formatting tool, to standardize code presentation:

```bash
cargo fmt
```

This command automatically applies formatting rules to the codebase, ensuring consistent indentation, spacing, line breaks, and other stylistic elements. Regular use of `rustfmt` during development and prior to submission simplifies the review process and reduces potential conflicts arising from stylistic differences.

## License

The `jsavrs` project is distributed under the terms of the Apache License, Version 2.0. This permissive open-source license permits use, modification, and distribution under specific conditions. For complete details regarding rights, limitations, and obligations, please refer to the [LICENSE](LICENSE) file included with the project.

## Contact

For inquiries regarding the use of the `jsavrs` framework, proposals for new features, or reports of technical issues, please contact the development team through the project's [GitHub repository](https://github.com/Giuseppe-Bianc/jsavrs/issues). When submitting inquiries or reports, please provide comprehensive information including detailed descriptions of the issue, steps to reproduce the problem, and any relevant system configuration details.