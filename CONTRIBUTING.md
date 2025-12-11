# Contributing to jsavrs

## Introduction

jsavrs is an open-source, Rust-based compiler toolchain and learning project that implements a small language frontend, intermediate representations (IR), SSA-based transformations, and assembly emission. The repository contains the compiler implementation, tests, benchmarks, assembly examples and developer utilities. The project's goals are:

- Provide a clear, maintainable Rust implementation of compiler concepts for education and experimentation.
- Produce a working backend that emits x86-64 assembly and supports small benchmarks and test programs.
- Maintain a high-quality codebase with strong testing and documentation so contributors can learn and iterate safely.

This document explains how to contribute code, tests, documentation, and reports to jsavrs.

## How Can You Contribute?

There are many ways to help:

- Report issues: open an issue with a reproducible description, expected behavior, and steps to reproduce. Attach small examples or test cases when possible (use the `asm_output/` and `vn_files/` folders for sample inputs).
- Suggest features: open an issue labeled `enhancement` and describe the motivation, a proposal for the API or user-visible changes, and backward-compatibility considerations.
- Submit pull requests (PRs): implement features, bug fixes, tests, or documentation improvements following the guidelines below.
- Improve tests: add unit/integration tests under `tests/`, update snapshots under `tests/snapshots/`, and add benches under `benches/` (use Criterion where applicable).
- Improve docs: update `README.md`, add more `rustdoc` comments, or improve RFC/design docs under `specs/`.
- Review PRs: provide constructive reviews and feedback in PR discussions.

Before working on larger changes, open an issue first to discuss the design—this avoids duplicated effort and helps maintainers coordinate.

## Coding Standards

The repository is written in Rust. Follow these guidelines to keep the codebase consistent and maintainable.

- Rust edition and tooling
  - Target Rust stable. Ensure your toolchain is up-to-date and compatible with the project.
  - Use `rustfmt` for formatting and `clippy` for lints. The project includes a `rustfmt.toml` file with preferred formatting options.

- Style
  - Aim for idiomatic Rust: prefer safe code, explicit error handling using `Result`/`Option`, and avoid unnecessary `unsafe` blocks. If `unsafe` is required, document why and keep it minimal.
  - Keep functions small and focused. Prefer composition to duplication.
  - Use descriptive names for functions, structs, enums, and variables. Add `///` rustdoc comments for public types and functions.

- Modules and organization
  - Group related code in appropriate modules under `src/` (e.g., `ir/`, `parser/`, `semantic/`).
  - Expose a minimal public API from `lib.rs`. Keep internal details private when possible.

- Tests
  - Add unit tests next to modules where convenient (use `#[cfg(test)] mod tests { ... }`), and integration tests in the `tests/` directory.
  - Snapshot tests are used in this project—follow existing snapshot patterns in `tests/` when adding new tests.
  - Benchmarks live in `benches/` and use Criterion. Keep microbenchmarks stable and document what they measure.

- Commit messages
  - Use clear, imperative commit messages (e.g., "Add SSA optimization for constant folding").
  - For larger changes, squash or organize commits logically before opening a PR.

Automated checks (recommended before PR):

- rustfmt: cargo fmt --all
- clippy: cargo clippy --all-targets --all-features -- -D warnings
- tests: cargo test --all

If your changes are large or affect build/test infrastructure, mention this in the PR description so maintainers can give focused attention.

## Development Environment Setup

This section explains how to set up a development environment on Windows (PowerShell) and other platforms.

Prerequisites

- Rust toolchain (stable): install from https://rustup.rs and ensure `cargo` and `rustc` are on PATH.
- LLVM/clang (optional): required only for certain integrations or if you want to use local LLVM tools. Most build/test tasks use Rust only.

Quick start (PowerShell)

1. Clone the repository:

```powershell
git clone https://github.com/Giuseppe-Bianc/jsavrs.git
cd jsavrs
```

2. Install recommended Rust components (optional but recommended):

```powershell
rustup component add rustfmt clippy
```

3. Build the project:

```powershell
cargo build --all
```

4. Run the tests:

```powershell
cargo test --all
```

5. Run benchmarks (optional):

```powershell
cargo bench
```

Notes

- If you add native dependencies or tools that require a system-level dependency (for example, a specific LLVM version), document them in the PR and update this file.
- The repository contains `asm_output/`, `vn_files/`, and `tests/` which house sample inputs and expected outputs—use them for developing and testing.

## Submitting Changes

Follow these steps to submit code changes via pull requests.

1. Fork the repository and create a feature branch from `main`:

```powershell
git checkout -b feature/brief-description
```

2. Implement your changes. Ensure the code builds and tests pass locally.

3. Run formatting and linting, and fix issues:

```powershell
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

4. Run the test suite and add tests for your change:

```powershell
cargo test --all
```

5. Commit your work with clear messages, and push to your fork:

```powershell
git add -A
git commit -m "Short, imperative description"
git push origin feature/brief-description
```

6. Open a pull request against `Giuseppe-Bianc/jsavrs:main` and include:

- A clear description of what the change does and why (motivation).
- The testing you performed and any new tests added.
- If the change affects performance, include before/after measurements or microbenchmarks (use `benches/`).
- Any migration or compatibility notes.

PR Review process

- Maintainers will review the PR, request changes when needed, and may suggest alternate implementations.
- Please respond to review comments promptly and update the PR with fixes. Keep the discussion focused and constructive.
- For larger features, the reviewer may ask you to split the work into smaller PRs.

Merging policy

- PRs should pass CI (if configured), formatting, lints, and tests before merging.
- A maintainer or repository owner will merge changes after approvals. The project uses `main` as the default branch.

## Community Engagement

We want the jsavrs community to be welcoming, inclusive, and productive. Please follow these guidelines:

- Be respectful and constructive in issue discussions and PR reviews.
- Use clear, non-confrontational language. Assume good intent. Focus feedback on code and design, not on individuals.
- Quote discussion context where helpful. Link to relevant code or tests when referencing specifics.
- If you disagree with design choices, present alternative solutions and explain trade-offs.

Code of Conduct

This project follows the `CODE_OF_CONDUCT.md` in the repository. All contributors must adhere to it. If you witness or experience unacceptable behavior, follow the reporting instructions in the code of conduct.

## Security and Responsible Disclosure

- Do not include secrets (API keys, credentials) in commits or issues. Use environment variables or secure vaults for secrets.
- If you discover a security vulnerability, do not open a public issue. Instead, contact the maintainers privately. See `SECURITY.md` if present, or open an email to the project owner if needed.

## Legal and Licensing

- This project is licensed under the terms found in `LICENSE` at the repository root. By contributing, you agree that your contributions are made under the same license.
- If your contribution includes third-party code, ensure it is compatible with the project's license and include attribution and license text as needed.

## Additional Notes and Tips

- Use smaller, focused PRs when possible. They are easier to review and faster to land.
- Run tests locally before pushing. CI may be configured to run tests and linting; fixing issues locally speeds up the merge process.
- Keep the public API stable; when breaking changes are necessary, coordinate via an issue or RFC and document migration steps.
- When touching code that affects many modules (IR, SSA, parser), add comprehensive tests and consider adding benchmark comparisons.

## Where to Ask for Help

- Open an issue describing your problem with logs, error messages, and steps to reproduce.
- For design questions, open an issue and tag it `design` or `enhancement`.

Thank you for contributing to jsavrs! Your help keeps the project healthy and useful for learners and contributors.
