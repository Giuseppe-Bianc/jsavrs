# jsavrs

[![Rust CI](https://github.com/Giuseppe-Bianc/jsavrs/actions/workflows/rust.yml/badge.svg)](https://github.com/Giuseppe-Bianc/jsavrs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Giuseppe-Bianc/jsavrs/graph/badge.svg?token=5EIG6IbpPa)](https://codecov.io/gh/Giuseppe-Bianc/jsavrs)
[![Lines of Code](https://sonarcloud.io/api/project_badges/measure?project=Giuseppe-Bianc_jsavrs&metric=ncloc)](https://sonarcloud.io/summary/new_code?id=Giuseppe-Bianc_jsavrs)
![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/Giuseppe-Bianc/jsavrs?utm_source=oss&utm_medium=github&utm_campaign=Giuseppe-Bianc%2Fjsavrs&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

## Introduction

`jsavrs` is a Rust-based project designed to be an OS-independent compiler. It aims to provide a robust, high-performance solution for compiling code across multiple platforms. The project leverages Rust's safety and concurrency features to ensure reliability and efficiency.

Whether you're a developer looking for a customizable compiler or someone interested in exploring Rust's capabilities, `jsavrs` offers a modular and extensible framework to meet your needs.

## Features

- **High Performance**: Built with Rust, `jsavrs` ensures fast compilation times and efficient resource usage.
- **Cross-Platform**: Designed to work seamlessly across different operating systems.
- **Ease of Use**: Simple APIs and clear documentation make it accessible to developers of all skill levels.
- **Extensibility**: Modular design allows for easy customization and integration with other tools.
- **100% Rust**: Fully implemented in Rust, ensuring safety, concurrency, and modern language features.

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

- **Rust**: Ensure you have Rust installed. You can install it using [rustup](https://rustup.rs/).
- **Cargo**: Rust's package manager (installed with Rust).

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/Giuseppe-Bianc/jsavrs.git
   cd jsavrs
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the tests:
   ```bash
   cargo test
   ```

## Usage

### Basic Usage

To use `jsavrs`, you can run the compiled binary with the following command:

```bash
./jsavrs input_file.rs
```

This will compile the specified Rust file and output the result to the default location.

### Advanced Usage

You can customize the behavior of `jsavrs` using command-line flags:

- Specify an output directory:
  ```bash
  ./jsavrs input_file.rs --output ./build
  ```
- Enable verbose logging:
  ```bash
  ./jsavrs input_file.rs --verbose
  ```
- Compile multiple files:
  ```bash
  ./jsavrs file1.rs file2.rs file3.rs
  ```

For a full list of options, run:
```bash
./jsavrs --help
```

## Testing

`jsavrs` includes a comprehensive suite of tests to ensure reliability and correctness. To run the tests, use the following command:

```bash
cargo test
```

### Adding New Tests

If you add new features or fix bugs, ensure you write corresponding tests. Tests are located in the `tests` directory and follow Rust's standard testing conventions.

Example of a unit test:
```rust
#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}
```

## Contributing

Contributions are welcome! If you have suggestions or improvements, please open an issue or submit a pull request.

### Steps to Contribute

1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes and write appropriate tests.
4. Run the tests:
   ```bash
   cargo test
   ```
5. Submit a pull request.

### Code Style

Please follow Rust's standard formatting guidelines. You can format your code using `rustfmt`:
```bash
cargo fmt
```

## License

This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions, feature requests, or to report bugs, open an issue on [GitHub](https://github.com/Giuseppe-Bianc/jsavrs/issues).
