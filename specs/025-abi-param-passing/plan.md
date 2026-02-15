# Implementation Plan: ABI-Compliant Parameter Passing and Return Value Handling

**Branch**: `025-abi-param-passing` | **Date**: 2026-02-12 | **Spec**: [spec.md](specs/025-abi-param-passing/spec.md)
**Input**: Feature specification from `/specs/025-abi-param-passing/spec.md`

## Summary

Extend the `gen_function` method in `AsmGen` to perform ABI-compliant parameter passing and return value handling for x86_64 targets. System V (Linux/macOS) and Microsoft x64 (Windows) calling conventions are supported; unsupported targets emit a structured `CompileError`. The implementation introduces new modules (`param.rs`, `prologue.rs`, `epilogue.rs`, `ret.rs`, `stack.rs`) under `src/codegen/` with a clean separation of concerns, leveraging the existing `Abi`, `AbiKind`, `Platform`, `IrType`, `IrParameter`, and `CompileError` infrastructure already in the codebase.

## Technical Context

**Language/Version**: Rust 1.93.0 (edition 2024)  
**Primary Dependencies**: Standard library only for production code; `clap`, `logos`, `thiserror`, `uuid`, `petgraph`, `chrono` already in Cargo.toml but no new deps needed  
**Storage**: N/A  
**Testing**: `cargo test` + `insta` snapshot testing (already configured in dev-dependencies)  
**Target Platform**: x86_64 — Linux (System V), macOS (System V), Windows (Microsoft x64)  
**Project Type**: Single monolithic Rust crate (lib + bin)  
**Performance Goals**: Deterministic O(n) code generation per function where n = number of parameters  
**Constraints**: Zero external dependencies beyond existing Cargo.toml; no `unwrap()` in production paths; all failures via `Result<T, CompileError>`; 16-byte stack alignment at all times  
**Scale/Scope**: ~7 new source files under `src/codegen/`, ~12+ integration test files under `tests/`

### Existing Infrastructure (already present)

| Component | Location | Relevance |
|---|---|---|
| `Abi` / `AbiKind` struct/enum | `src/asm/abi.rs` | Full ABI metadata: register lists, shadow space, red zone, callee-saved sets |
| `Platform` enum | `src/asm/platform.rs` | Linux, macOS, Windows variants |
| `TargetTriple` enum | `src/ir/module.rs` | 9 variants including non-x86_64 targets |
| `IrType` enum | `src/ir/types.rs` | All type variants: I8–I64, U8–U64, F32/F64, Bool, Char, Pointer, String, Array, Struct, Void |
| `IrParameter` struct | `src/ir/function.rs` | `name`, `ty: IrType`, `attributes: ParamAttributes` (includes `by_val`) |
| `Function` struct | `src/ir/function.rs` | `parameters`, `return_type`, `local_vars`, `name` |
| `CompileError::AsmGeneratorError` | `src/error/compile_error.rs` | Error variant for codegen failures |
| `AsmGen` struct | `src/codegen/asmgen.rs` | Existing `gen_function` stub to be extended |
| `AssemblyFile` | `src/asm/assembly_file.rs` | Output artifact with text/data sections |
| GP/XMM register constants | `src/asm/register/x86_register.rs` | `INT_PARAM_REGS_SYSTEMV`, `FLOAT_PARAM_REGS_SYSTEMV`, etc. |

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Justification |
|---|---|---|
| **Safety First** | PASS | No `unsafe` code; all failures via `Result<T, CompileError>`; no `unwrap()` in production |
| **Performance Excellence** | PASS | O(n) parameter classification; no allocations beyond Vec for parameter mappings; zero-cost ABI dispatch via `const fn` methods already on `Abi` |
| **Cross-Platform Compatibility** | PASS | Explicit support for Linux, macOS (System V) and Windows (x64); non-x86_64 targets produce `CompileError` without panic |
| **Modular Extensibility** | PASS | Clean separation: `param.rs`, `prologue.rs`, `epilogue.rs`, `ret.rs`, `stack.rs` as independent, composable modules; new ABIs can be added by extending `AbiKind` and adding new register constant lists |
| **Test-Driven Reliability** | PASS | All tests as integration tests in `tests/`; exact string comparison of assembly output; coverage of all user stories and edge cases |
| **Snapshot Validation** | PASS | Insta snapshot tests for prologue/epilogue/full-function assembly output |
| **Documentation Rigor** | PASS | Comprehensive rustdoc on all public types/functions; research.md and data-model.md generated |
| **Collaboration/Governance** | PASS | Feature developed on dedicated branch `025-abi-param-passing`; PR-based review |

**Gate Result**: **PASS** — no violations, proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/025-abi-param-passing/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── codegen-api.md   # Internal API contracts for codegen modules
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── codegen/
│   ├── mod.rs           # (existing) add new submodule declarations
│   ├── asmgen.rs        # (existing) extend gen_function with ABI logic
│   ├── param.rs         # NEW — parameter classification & register/stack assignment
│   ├── prologue.rs      # NEW — function prologue generation
│   ├── epilogue.rs      # NEW — function epilogue generation
│   ├── ret.rs           # NEW — return value placement
│   ├── stack.rs         # NEW — stack frame layout & alignment
│   ├── target.rs        # NEW — TargetTriple → Abi resolution with CompileError
│   └── error.rs         # NEW — codegen-specific error construction helpers
├── asm/
│   ├── abi.rs           # (existing) already contains full Abi infra — no changes
│   ├── register/        # (existing) already has all register constants — no changes
│   └── ...
├── ir/
│   ├── types.rs         # (existing) IrType — no changes
│   ├── function.rs      # (existing) IrParameter — no changes
│   └── module.rs        # (existing) TargetTriple — no changes
└── error/
    └── compile_error.rs # (existing) may add new ErrorCode variants

tests/
├── abi_param_int_tests.rs             # US1: integer parameter mapping
├── abi_param_float_tests.rs           # US4: floating-point parameter mapping
├── abi_param_mixed_tests.rs           # US5: mixed integer/float parameters
├── abi_prologue_epilogue_tests.rs     # US2: prologue/epilogue correctness
├── abi_return_value_tests.rs          # US3: return value placement
├── abi_stack_spill_tests.rs           # Stack spill offset validation
├── abi_shadow_space_tests.rs          # Windows shadow space allocation
├── abi_red_zone_tests.rs             # System V red zone behavior
├── abi_edge_case_tests.rs            # Zero params, void returns, alignment
├── abi_unsupported_type_tests.rs     # String/Array/Struct → CompileError
├── abi_unsupported_target_tests.rs   # Non-x86_64 → CompileError
└── abi_snapshot_tests.rs             # Insta snapshots for full assembly output
```

**Structure Decision**: Single monolithic Rust crate. New codegen modules are added under `src/codegen/` following the existing pattern. All tests are integration tests under `tests/` per project convention. No new external dependencies.

## Complexity Tracking

> No constitution violations — table not required.
