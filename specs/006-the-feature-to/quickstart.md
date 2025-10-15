# Quickstart: Generating NASM from IR

This quickstart shows basic usage examples for the new code generator and how to validate output with NASM.

## 1) Generate assembly for a module

Assuming the `CodeGenerator` API is implemented and exposed in `src/asm/codegen/mod.rs`:

```rust
use jsavrs::asm::codegen::DefaultCodeGenerator;
use jsavrs::ir::Module;

let module: Module = /* build or parse IR module */;
let mut gen = DefaultCodeGenerator::new(target_platform);
let asm_text = gen.generate_module(&module)?;
std::fs::write("out.asm", asm_text)?;
```

## 2) Validate assembly with NASM (local)

On Windows (PowerShell):

```powershell
nasm -f win64 out.asm -o out.obj
```

On Linux/macOS (bash/pwsh):

```bash
nasm -f elf64 out.asm -o out.o
```

Link and run with the platform toolchain.

## 3) ABI selection

The generator auto-selects ABI by platform; to override, pass a platform option to `DefaultCodeGenerator::new(platform)`.

## 4) Debugging and trace

Use the existing `--verbose` CLI flag to enable detailed tracing. Trace output will include generation start/end, ABI selection, and fatal errors. Trace format follows the project's error reporting style and is human-readable.

## 5) Running tests

Run the unit and snapshot tests:

```powershell
cargo test --test codegen_tests
cargo test --test codegen_snapshot_tests
```

Use `INSTA_UPDATE=1 cargo test` to refresh snapshots when intentionally changing output.

_Last updated: 2025-10-15"}