# Quickstart: ABI-Compliant Parameter Passing

**Feature**: 025-abi-param-passing  
**Date**: 2026-02-12

---

## Overview

This feature extends the `gen_function` method in `AsmGen` to generate ABI-compliant parameter passing, prologue/epilogue, and return value handling for x86_64 targets (System V and Windows x64).

## Prerequisites

- Rust 1.93.0+ (edition 2024)
- Existing jsavrs codebase on branch `025-abi-param-passing`

## Quick Verification

```bash
# Run all tests
cargo test

# Run only ABI-related tests
cargo test abi_param
cargo test abi_prologue
cargo test abi_return
cargo test abi_stack
cargo test abi_shadow
cargo test abi_red_zone
cargo test abi_edge
cargo test abi_unsupported
cargo test abi_snapshot

# Check formatting and lints
cargo fmt --check
cargo clippy -- -D warnings
```

## Architecture Summary

```
AsmGen::gen_function(func)
  │
  ├─ target.rs::resolve_abi(triple)         → Abi or CompileError
  ├─ param.rs::classify_parameters(params)  → Vec<ParamAssignment>
  ├─ stack.rs::compute_stack_frame(func)    → StackFrame
  ├─ prologue.rs::gen_prologue(abi, frame)  → assembly instructions
  ├─ [function body IR translation]
  ├─ ret.rs::gen_return(ret_type, abi)      → return register placement
  └─ epilogue.rs::gen_epilogue(abi, frame)  → assembly instructions
```

## New Modules

| Module | Purpose |
|--------|---------|
| `src/codegen/target.rs` | `TargetTriple` → `Abi` resolution with `CompileError` for unsupported targets |
| `src/codegen/param.rs` | Parameter classification (`ParamLocation`, `ParamAssignment`, `classify_parameters`) |
| `src/codegen/stack.rs` | Stack frame layout computation (`StackFrame`, alignment) |
| `src/codegen/prologue.rs` | Function prologue generation (frame pointer, callee-saved, stack alloc) |
| `src/codegen/epilogue.rs` | Function epilogue generation (reverse of prologue + `ret`) |
| `src/codegen/ret.rs` | Return value register placement |
| `src/codegen/error.rs` | Error construction helpers for codegen |

## Key Design Decisions

1. **No new external dependencies** — uses only existing crate infrastructure
2. **`by_val` attribute ignored** — parameter classification is based solely on `IrType`
3. **Frame pointer always emitted** — `push rbp; mov rbp, rsp` in every function
4. **RBP-relative stack offsets** — stable regardless of local variable allocation
5. **Independent counters (System V)** vs **shared positional slots (Windows)**
6. **Red zone optimization** — System V leaf functions with locals ≤ 128 bytes skip `sub rsp`
7. **All errors via `Result<T, CompileError>`** — no `unwrap()`, no panics

## Example: System V with 3 Integer Parameters

Given IR function:
```
function add(a: i32, b: i32, c: i32) -> i32
```

Generated assembly (System V):
```asm
add:
    push rbp
    mov rbp, rsp
    ; a in edi (lower 32 bits of rdi)
    ; b in esi (lower 32 bits of rsi)
    ; c in edx (lower 32 bits of rdx)
    ; ... function body ...
    ; result in eax
    pop rbp
    ret
```

## Example: Windows x64 with Mixed Parameters

Given IR function:
```
function calc(x: i64, y: f64, z: i64, w: f64) -> f64
```

Generated assembly (Windows x64):
```asm
calc:
    push rbp
    mov rbp, rsp
    sub rsp, 32          ; shadow space
    ; x in rcx  (slot 1, integer)
    ; y in xmm1 (slot 2, float)
    ; z in r8   (slot 3, integer)
    ; w in xmm3 (slot 4, float)
    ; ... function body ...
    ; result in xmm0
    add rsp, 32
    pop rbp
    ret
```

## Test Coverage Matrix

| Test File | User Story | What's Validated |
|-----------|-----------|------------------|
| `abi_param_int_tests.rs` | US1 | Integer register mapping (1–10 params, both ABIs) |
| `abi_param_float_tests.rs` | US4 | FP register mapping (1–10 params, both ABIs) |
| `abi_param_mixed_tests.rs` | US5 | Mixed int/float with independent vs positional slots |
| `abi_prologue_epilogue_tests.rs` | US2 | Frame pointer, callee-saved push/pop, stack alloc |
| `abi_return_value_tests.rs` | US3 | RAX, EAX, RAX:RDX, XMM0, XMM0:XMM1 |
| `abi_stack_spill_tests.rs` | US1,4 | RBP-relative offsets for excess parameters |
| `abi_shadow_space_tests.rs` | US2 | 32-byte shadow space on Windows |
| `abi_red_zone_tests.rs` | US2 | 128-byte red zone on System V leaf functions |
| `abi_edge_case_tests.rs` | Edge | Zero params, void returns, full callee-saved, alignment |
| `abi_unsupported_type_tests.rs` | Edge | String/Array/Struct → CompileError |
| `abi_unsupported_target_tests.rs` | Edge | Non-x86_64 → CompileError, no panic |
| `abi_snapshot_tests.rs` | All | Insta snapshots for complete function assembly output |
