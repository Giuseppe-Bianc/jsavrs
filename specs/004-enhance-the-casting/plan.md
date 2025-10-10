# Implementation Plan: Comprehensive Type Casting System Enhancement

**Branch**: `004-enhance-the-casting` | **Date**: 2025-10-08 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-enhance-the-casting/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Enhance the jsavrs compiler's type casting system to support comprehensive conversions among all 13 fundamental data types (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, String, bool). The implementation will extend the existing PromotionMatrix in `src/ir/type_promotion.rs` to define all 169 possible type conversion pairs, implement precise CastKind variant mapping for each conversion type, create a comprehensive warning system for precision loss and overflow scenarios, and ensure O(1) lookup performance for common conversion cases. The enhancement maintains backward compatibility with existing type promotion code while following established code style and documentation conventions.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: 
- Core: std collections (HashMap for O(1) promotion rule lookup)
- Existing modules: crate::ir::types (IrType enum), crate::ir::instruction (CastKind enum), crate::location::source_span (SourceSpan for error reporting)
- Testing: insta (snapshot testing), criterion (performance benchmarks)

**Storage**: N/A (in-memory data structures only)  
**Testing**: Rust's built-in testing framework (`cargo test`) with custom test harness in `tests/ir_type_promotion_tests.rs`, insta for snapshot testing, criterion for benchmarking  
**Target Platform**: Cross-platform (Windows, macOS, Linux) - compiler infrastructure
**Project Type**: Single project (Rust library/binary)  
**Performance Goals**: 
- O(1) lookup time for promotion rule retrieval from HashMap
- <1ms for complete type promotion analysis of binary operations
- Minimal compilation time overhead (<5% increase for programs with extensive type conversions)

**Constraints**: 
- Backward compatibility: Cannot change public API of TypePromotion or PromotionMatrix structures
- Warning generation must not cause compilation failures unless overflow behavior is CompileError
- Must handle all 24 CastKind variants defined in src/ir/instruction.rs
- Must support all 4 overflow behavior configurations (Wrap, Saturate, Trap, CompileError)
- IEEE 754 floating-point representation for f32/f64
- Rust Unicode scalar values for char type (excluding surrogates U+D800 to U+DFFF)

**Scale/Scope**: 
- 169 type conversion pairs (13 types × 13 types) to define
- 24 CastKind variants to map correctly
- Target 95% code coverage for type_promotion module
- At least 50 distinct test scenarios covering edge cases

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Safety First ✅
The implementation leverages Rust's type system to prevent invalid type conversions at compile time. All CastKind variants are strongly typed enums, and the PromotionMatrix uses HashMap for safe, checked access to promotion rules. No unsafe code blocks are required for this feature.

### Performance Excellence ✅
The design uses HashMap-based O(1) lookup for promotion rules, ensuring minimal performance overhead. Benchmark tests with criterion will verify that the enhancement adds <5% compilation time overhead for programs with extensive type conversions.

### Cross-Platform Compatibility ✅
Type promotion logic is platform-agnostic and operates purely on abstract IR types. The implementation does not introduce platform-specific dependencies or behaviors.

### Modular Extensibility ✅
The enhancement extends the existing PromotionMatrix infrastructure without breaking changes. New promotion rules are added through the existing `add_promotion_rule` method, and the public API remains unchanged, allowing future extensions.

### Test-Driven Reliability ✅
Comprehensive test suite in `tests/ir_type_promotion_tests.rs` will cover all 169 type conversion pairs with at least 50 distinct scenarios including edge cases (overflow, precision loss, invalid conversions). Target 95% code coverage.

### Snapshot Validation ✅
Insta snapshot testing will validate warning messages and error output consistency across all conversion scenarios, ensuring no regressions in diagnostic quality.

### Documentation Rigor ✅
All new promotion rules and CastKind mappings will be documented with rustdoc comments following established conventions. The module-level documentation in `type_promotion.rs` will be updated to reflect the comprehensive casting capabilities.

**Gate Status**: PASS - All constitution principles are upheld by this design.

## Project Structure

### Documentation (this feature)

```
specs/004-enhance-the-casting/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
src/
├── ir/
│   ├── types.rs                    # IrType enum (no changes expected)
│   ├── instruction.rs              # CastKind enum (already complete with 24 variants)
│   ├── type_promotion.rs           # PRIMARY MODIFICATION TARGET
│   │                               # - Extend PromotionMatrix initialization
│   │                               # - Add boolean conversion rules
│   │                               # - Add character conversion rules  
│   │                               # - Add string conversion rules
│   │                               # - Add integer narrowing rules
│   │                               # - Enhance warning generation
│   └── value.rs                    # Value type (reference only)
└── location/
    └── source_span.rs              # SourceSpan for error reporting (reference only)

tests/
├── ir_type_promotion_tests.rs      # PRIMARY TEST TARGET
│                                   # - Add tests for all 169 conversion pairs
│                                   # - Add edge case tests (overflow, precision loss)
│                                   # - Add boolean conversion tests
│                                   # - Add character/string conversion tests
│                                   # - Add validation tests for invalid conversions
└── snapshots/                      # Insta snapshot files
    └── ir_type_promotion_tests__*.snap

benches/
└── jsavrs_benchmark.rs             # Add type promotion benchmarks
```

**Structure Decision**: Single project (default) structure is appropriate for this compiler infrastructure enhancement. All modifications are confined to the existing `src/ir/` module structure, specifically the `type_promotion.rs` file and its corresponding test file. No new modules or directories are required.

## Complexity Tracking

*No constitution violations detected - this section is intentionally left empty.*

---

## Phase 0: Outline & Research ✅ COMPLETE

**Status**: ✅ All research tasks completed  
**Output**: `research.md` generated with comprehensive findings  
**Date**: 2025-10-08

### Research Tasks Completed

1. ✅ **CastKind Enum Completeness**: Verified all 24 variants exist in `src/ir/instruction.rs`
2. ✅ **Integer Narrowing Strategy**: Defined rules for all narrowing conversions with overflow detection
3. ✅ **Boolean Conversion Semantics**: Established canonical 0/1 mappings and zero-test semantics
4. ✅ **Character and Unicode Handling**: Documented valid Unicode scalar value ranges and validation rules
5. ✅ **String Conversion Runtime Requirements**: Identified compile-time vs. runtime conversion needs
6. ✅ **Floating-Point Special Values**: Defined behavior for NaN, infinity, and subnormal numbers
7. ✅ **Precision Loss Estimation**: Designed quantification system for precision loss types
8. ✅ **Cross-Signedness Conversion Rules**: Established bit reinterpretation semantics for same-width types
9. ✅ **Performance Optimization Strategy**: Verified O(1) HashMap lookup with optimization opportunities
10. ✅ **Overflow Behavior Configuration**: Documented implementation for Wrap/Saturate/Trap/CompileError

### Key Decisions Summary

| Decision Area | Choice | Rationale |
|--------------|--------|-----------|
| Integer Narrowing | IntTruncate with may_overflow=true | Follows Rust's explicit casting semantics |
| Boolean Conversions | 0/1 mapping for integers, zero-test for bool | Standard C/Rust conventions |
| Unicode Validation | Reject surrogates and >0x10FFFF | Rust char safety guarantees |
| String Conversions | Runtime support + validation flags | Parsing requires runtime, may fail |
| Float Special Values | Warning-based with deterministic OverflowBehavior control: **Wrap** (NaN→0, +∞→INT_MAX/UINT_MAX, -∞→INT_MIN/0), **Saturate** (clamps to type bounds), **Trap** (runtime panic), **CompileError** (const evaluation fails) | Balances safety, cross-platform consistency, and flexibility per FR-017 |
| Performance | HashMap O(1) lookup | Optimal for 169-entry matrix |

---

## Phase 1: Design & Contracts ✅ COMPLETE

**Status**: ✅ All design artifacts generated  
**Output**: `data-model.md`, `contracts/promotion_matrix_api.md`, `quickstart.md`  
**Date**: 2025-10-08

### Generated Artifacts

1. ✅ **Data Model** (`data-model.md`):
   - Defined all 8 core entities with relationships
   - Documented 169 promotion rules across 9 categories
   - Specified validation rules for compile-time and runtime checks
   - Provided implementation checklist

2. ✅ **API Contract** (`contracts/promotion_matrix_api.md`):
   - Documented public API with backward compatibility guarantees
   - Specified internal API enhancements (4 new helper methods)
   - Defined new fields: `requires_runtime_support`, `requires_validation`
   - Provided usage examples and testing contracts

3. ✅ **Quick Start Guide** (`quickstart.md`):
   - Created 5-minute quick start for developers
   - Documented common use cases with code examples
   - Explained type lattice and promotion precedence
   - Provided FAQ and troubleshooting guide

### Agent Context Update

✅ **Updated**: `.github/copilot-instructions.md`
- Added: Rust 1.75+ technology
- Added: N/A (in-memory data structures only) for database
- Status: Successfully updated via `update-agent-context.ps1`

### Constitution Re-Check (Post-Design)

**Re-evaluation Status**: ✅ PASS - All principles still upheld after detailed design

| Principle | Status | Verification |
|-----------|--------|--------------|
| Safety First | ✅ | No unsafe code, strongly typed enums, validation flags for runtime checks |
| Performance Excellence | ✅ | HashMap O(1) lookup confirmed, <5% overhead target set |
| Cross-Platform Compatibility | ✅ | Platform-agnostic IR operations, no OS-specific dependencies |
| Modular Extensibility | ✅ | Public API unchanged, new rules via existing methods |
| Test-Driven Reliability | ✅ | 169 test cases + edge cases + benchmarks planned |
| Snapshot Validation | ✅ | Insta snapshot tests for all warning messages |
| Documentation Rigor | ✅ | Rustdoc comments, module-level docs, quick start guide |

**Design Review**: No architectural concerns identified. Implementation can proceed to Phase 2 (Tasks).

---

## Next Steps: Phase 2 (NOT INCLUDED IN THIS COMMAND)

**Command**: `/speckit.tasks` - Generate implementation tasks from this plan  
**Input**: This plan file (`plan.md`) and design artifacts  
**Output**: `tasks.md` with granular implementation tasks

**Scope**: Phase 2 will break down the implementation into:
1. Code modification tasks for `src/ir/type_promotion.rs`
2. Test implementation tasks for `tests/ir_type_promotion_tests.rs`
3. Benchmark implementation tasks for `benches/jsavrs_benchmark.rs`
4. Documentation update tasks
5. Validation and verification tasks

**Note**: The `/speckit.plan` command stops here as per the workflow specification. Phase 2 requires a separate command invocation.

---

## Plan Summary

### Artifacts Generated

| Artifact | Status | Purpose |
|----------|--------|---------|
| `plan.md` | ✅ | This file - comprehensive implementation plan |
| `research.md` | ✅ | Phase 0 - All technical clarifications resolved |
| `data-model.md` | ✅ | Phase 1 - Entity definitions and 169 promotion rules |
| `contracts/promotion_matrix_api.md` | ✅ | Phase 1 - API contracts and guarantees |
| `quickstart.md` | ✅ | Phase 1 - Developer quick start guide |
| `.github/copilot-instructions.md` | ✅ | Agent context updated with new technologies |

### Implementation Readiness

| Category | Readiness | Details |
|----------|-----------|---------|
| **Requirements** | ✅ 100% | All 20 functional requirements defined |
| **Research** | ✅ 100% | All 10 clarifications resolved |
| **Design** | ✅ 100% | All entities, rules, and contracts defined |
| **Constitution** | ✅ PASS | All 7 core principles upheld |
| **Testing Strategy** | ✅ Defined | 169 test cases + edge cases + benchmarks |
| **Performance Target** | ✅ Set | O(1) lookup, <5% overhead, <1ms analysis |

### Key Metrics

- **Type Conversion Pairs**: 169 (13 types × 13 types)
- **CastKind Variants**: 24 (all utilized)
- **Promotion Rule Categories**: 9 (identity, widening, narrowing, etc.)
- **Test Coverage Target**: 95%
- **Performance Target**: O(1) lookup, <1ms analysis

---

## Branch Status

**Current Branch**: `004-enhance-the-casting`  
**Based On**: `main`  
**Status**: Planning phase complete, ready for implementation

### Files Modified (Planning Phase)

```
specs/004-enhance-the-casting/
├── plan.md                            # ✅ This file
├── research.md                        # ✅ Generated
├── data-model.md                      # ✅ Generated
├── quickstart.md                      # ✅ Generated
└── contracts/
    └── promotion_matrix_api.md        # ✅ Generated

.github/
└── copilot-instructions.md            # ✅ Updated
```

### Files to be Modified (Implementation Phase)

```
src/ir/type_promotion.rs               # 🔨 Primary implementation target
tests/ir_type_promotion_tests.rs       # 🔨 Primary test target
benches/jsavrs_benchmark.rs            # 🔨 Benchmark additions
```

---

**Plan Status**: ✅ COMPLETE  
**Next Command**: `/speckit.tasks` to generate implementation tasks  
**Estimated Implementation Time**: 3-5 days for core implementation + testing
