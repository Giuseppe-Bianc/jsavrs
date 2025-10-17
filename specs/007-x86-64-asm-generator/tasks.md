# Tasks: x86-64 NASM Assembly Code Generator

**Feature Branch**: `007-x86-64-asm-generator`  
**Input**: Design documents from `/specs/007-x86-64-asm-generator/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, quickstart.md ✅

**Tests**: Test tasks are NOT included in this implementation plan as testing requirements are specified in the feature specification under Success Criteria and will be validated through Rust's standard test infrastructure (`cargo test`, Insta snapshot tests, Criterion benchmarks).

**Organization**: Tasks are grouped by user story (P1-P9 from spec.md) to enable independent implementation and testing of each story increment.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1-US9)
- Includes exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, error types, and basic generator structure

- [ ] T001 Create error module in `src/asm/error.rs` with `CodeGenError` enum covering all error variants (UnsupportedType, UnknownInstruction, MalformedInstruction, RegisterAllocationFailed, InvalidCallingConvention, CfgVerificationFailed, PhiResolutionFailed, IoError) with `From<CodeGenError>` for `CompileError` trait implementation
- [ ] T002 [P] Create generator module skeleton in `src/asm/generator.rs` with `AsmGenerator` struct fields (target_triple, abi, errors, sections, label_counter)
- [ ] T003 [P] Create result types in `src/asm/generator.rs`: `CodeGenResult` (assembly, errors, stats) and `CodeGenStats` (functions_generated, functions_failed, instructions_translated, assembly_instructions, register_spills, total_stack_size)
- [ ] T004 Update `src/asm/mod.rs` to export new modules (error, generator)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure components needed by ALL user stories

**⚠️ CRITICAL**: No user story implementation can begin until this phase is complete

- [ ] T005 Implement `AsmGenerator::new(target: TargetTriple) -> Self` constructor in `src/asm/generator.rs` that selects appropriate ABI (System V for Linux/macOS, Microsoft x64 for Windows) based on target triple
- [ ] T006 [P] Create register allocator module in `src/asm/register_allocator.rs` with `LinearScanAllocator` struct (intervals, active, available_regs, spilled_values) and empty method stubs
- [ ] T007 [P] Create instruction selector module in `src/asm/instruction_selector.rs` with `InstructionSelector` struct and `select_instruction(inst: &Instruction) -> Result<Vec<AsmInstruction>, CodeGenError>` signature
- [ ] T008 [P] Create phi resolver module in `src/asm/phi_resolver.rs` with `PhiResolver` struct and `resolve_phis(func: &Function) -> Result<(), CodeGenError>` signature
- [ ] T009 [P] Create prologue/epilogue generator module in `src/asm/prologue_epilogue.rs` with `FrameGenerator` struct and method signatures for `generate_prologue` and `generate_epilogue`
- [ ] T010 Create `FunctionContext` struct in `src/asm/generator.rs` to track per-function state (register_assignment, stack_frame, used_callee_saved, block_labels, value_locations)
- [ ] T011 Implement label generation in `src/asm/generator.rs`: `new_label(&mut self, prefix: &str) -> String` using monotonic label_counter
- [ ] T012 Implement assembly emission in `src/asm/generator.rs`: `emit_assembly(&self) -> String` that formats sections (.text, .data, .bss, .rodata) into valid NASM syntax with proper directives

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Basic Function Translation (Priority: P1) 🎯 MVP

**Goal**: Translate simple IR functions with arithmetic operations, local variables, and return statements into correct x86-64 NASM assembly

**Independent Test**: Provide IR function with basic arithmetic (add, subtract, multiply) and local variables, generate assembly, assemble with NASM, link, execute, verify return value

### Implementation for User Story 1

- [ ] T013 [P] [US1] Implement basic instruction selection for integer binary operations in `src/asm/instruction_selector.rs`: Add, Subtract, Multiply for I32/I64 types mapping to `add`, `sub`, `imul` instructions
- [ ] T014 [P] [US1] Implement alloca instruction selection in `src/asm/instruction_selector.rs`: translate IR `Alloca` to stack offset calculation with proper alignment
- [ ] T015 [P] [US1] Implement load instruction selection in `src/asm/instruction_selector.rs`: translate IR `Load` to `mov` instruction with memory operand
- [ ] T016 [P] [US1] Implement store instruction selection in `src/asm/instruction_selector.rs`: translate IR `Store` to `mov` instruction with memory destination
- [ ] T017 [US1] Implement `StackFrame` struct in `src/asm/prologue_epilogue.rs` with fields (local_vars_size, spill_slots_size, shadow_space_size, alignment_padding, total_size) and `calculate_frame_size` method
- [ ] T018 [US1] Implement function prologue generation in `src/asm/prologue_epilogue.rs`: `generate_prologue` produces `push rbp`, `mov rbp, rsp`, `sub rsp, frame_size` instructions
- [ ] T019 [US1] Implement function epilogue generation in `src/asm/prologue_epilogue.rs`: `generate_epilogue` produces `mov rsp, rbp`, `pop rbp`, `ret` or `leave`, `ret` sequence
- [ ] T020 [US1] Implement return terminator translation in `src/asm/instruction_selector.rs`: handle `Return` with value (move to rax) and void return
- [ ] T021 [US1] Implement `generate_function` in `src/asm/generator.rs`: orchestrate prologue → block iteration → instruction selection → epilogue for single function
- [ ] T022 [US1] Implement `generate(&mut self, module: &Module) -> CodeGenResult` main entry point in `src/asm/generator.rs`: iterate functions, accumulate errors, collect sections, return result with statistics
- [ ] T023 [US1] Implement type-to-size mapping in `src/asm/instruction_selector.rs`: helper function mapping IR types (I8→byte, I16→word, I32→dword, I64→qword) to instruction sizes
- [ ] T024 [US1] Implement file output in `src/asm/generator.rs`: save generated assembly to `.asm` file (same directory as input, same base name if no explicit path provided)

**Checkpoint**: Basic function translation complete - can compile, assemble, and execute simple arithmetic functions

---

## Phase 4: User Story 2 - Control Flow Translation (Priority: P2)

**Goal**: Translate IR control flow constructs (branches, loops, switches) into assembly jump instructions and labels

**Independent Test**: Provide IR with conditional branches and loops, generate assembly, verify execution flow matches IR semantics through test cases with different inputs

### Implementation for User Story 2

- [ ] T025 [P] [US2] Implement basic block label generation in `src/asm/generator.rs`: create unique labels for each basic block using `new_label` with block ID
- [ ] T026 [P] [US2] Implement unconditional branch translation in `src/asm/instruction_selector.rs`: translate `Branch` terminator to `jmp target_label` instruction
- [ ] T027 [P] [US2] Implement conditional branch translation in `src/asm/instruction_selector.rs`: translate `ConditionalBranch` to comparison instruction (`cmp`/`test`) followed by conditional jump (`je`, `jne`, `jg`, `jl`, `ja`, `jb` based on condition type)
- [ ] T028 [US2] Implement comparison instruction selection in `src/asm/instruction_selector.rs`: translate IR comparison operations (Eq, Ne, Lt, Le, Gt, Ge) to `cmp` instruction with appropriate flags checking, handling signed vs unsigned comparisons
- [ ] T029 [US2] Implement switch terminator translation in `src/asm/instruction_selector.rs`: translate `Switch` to jump table or conditional chain based on number of cases (jump table for >4 cases, conditional chain otherwise)
- [ ] T030 [US2] Update `generate_function` in `src/asm/generator.rs` to handle terminators: call instruction selector for each block's terminator after block instructions
- [ ] T031 [US2] Implement CFG traversal ordering in `src/asm/generator.rs`: use existing `ControlFlowGraph` from `src/ir/cfg.rs` to traverse blocks in proper order for code layout

**Checkpoint**: Control flow translation complete - can compile functions with if statements, loops, and switches

---

## Phase 5: User Story 3 - Function Calls and ABI Compliance (Priority: P3)

**Goal**: Generate assembly for function calls following System V (Linux/macOS) or Microsoft x64 (Windows) calling conventions

**Independent Test**: Create IR with function calls passing various parameters, verify parameters placed in correct registers/stack per ABI, return values handled correctly

### Implementation for User Story 3

- [ ] T032 [P] [US3] Implement parameter passing in `src/asm/prologue_epilogue.rs`: `generate_parameter_setup` maps function parameters to registers/stack per ABI using existing `Abi::parameter_registers` from `src/asm/abi.rs`
- [ ] T033 [P] [US3] Implement call instruction selection in `src/asm/instruction_selector.rs`: translate IR `Call` instruction to parameter moves + `call` instruction + return value handling
- [ ] T034 [US3] Implement caller-saved register preservation in `src/asm/instruction_selector.rs`: generate `push` instructions for caller-saved registers (rax, rcx, rdx, r8-r11, xmm0-xmm5) before call, `pop` after call
- [ ] T035 [US3] Implement callee-saved register preservation in `src/asm/prologue_epilogue.rs`: prologue pushes callee-saved registers (rbx, rbp, r12-r15) if used, epilogue pops them
- [ ] T036 [US3] Implement stack alignment for calls in `src/asm/instruction_selector.rs`: ensure RSP is 16-byte aligned before `call` instruction per ABI requirements
- [ ] T037 [US3] Implement shadow space allocation for Windows x64 in `src/asm/prologue_epilogue.rs`: allocate 32 bytes in function prologue if function makes calls (only for Microsoft x64 ABI), update `StackFrame` calculation
- [ ] T038 [US3] Implement return value handling in `src/asm/instruction_selector.rs`: move return value from rax/xmm0 to destination after call instruction
- [ ] T039 [US3] Handle excess parameters (>6 integers, >8 floats) in `src/asm/instruction_selector.rs`: push excess parameters onto stack in reverse order before call per ABI

**Checkpoint**: Function calls working - can compile multi-function programs with proper ABI compliance

---

## Phase 6: User Story 4 - Memory Operations Translation (Priority: P4)

**Goal**: Translate IR memory operations (load, store, GetElementPtr) into correct assembly with proper addressing modes

**Independent Test**: Provide IR with alloca, load, store, GetElementPtr instructions, verify assembly correctly allocates stack space and accesses memory at correct addresses

### Implementation for User Story 4

- [ ] T040 [P] [US4] Implement GetElementPtr translation in `src/asm/instruction_selector.rs`: translate array indexing to `lea` instruction with indexed addressing mode `[base + index*scale + offset]`
- [ ] T041 [P] [US4] Implement pointer arithmetic in `src/asm/instruction_selector.rs`: handle pointer addition/subtraction using `add`/`sub` instructions with scaled offsets
- [ ] T042 [US4] Implement alignment calculation in `src/asm/prologue_epilogue.rs`: update `StackFrame::calculate_frame_size` to align local variables to natural alignment (1/2/4/8 bytes based on type)
- [ ] T043 [US4] Implement memory operand generation in `src/asm/instruction_selector.rs`: helper to create `MemoryOperand` with base register, optional index register, scale, and displacement for load/store instructions
- [ ] T044 [US4] Handle global variable references in `src/asm/generator.rs`: emit globals in `.data` or `.bss` sections with proper labels, generate RIP-relative addressing for access on Linux/macOS or absolute addressing on Windows

**Checkpoint**: Memory operations working - can compile programs with arrays, pointers, and complex data access

---

## Phase 7: User Story 5 - Type Conversions and Casts (Priority: P5)

**Goal**: Translate IR type conversion instructions (casts, extensions, truncations, float conversions) into correct assembly

**Independent Test**: Provide IR with various Cast instructions, execute tests, verify converted values are mathematically correct per cast semantics

### Implementation for User Story 5

- [ ] T045 [P] [US5] Implement sign extension casts in `src/asm/instruction_selector.rs`: translate SignExtend cast to `movsx` instruction (movsx r64, r32 for I32→I64, movsx r32, r16 for I16→I32, etc.)
- [ ] T046 [P] [US5] Implement zero extension casts in `src/asm/instruction_selector.rs`: translate ZeroExtend cast to `movzx` instruction (movzx r64, r32 for U32→U64, etc.) or simple 32-bit mov (which zero-extends to 64-bit automatically)
- [ ] T047 [P] [US5] Implement truncation casts in `src/asm/instruction_selector.rs`: translate Truncate cast using register size aliases (mov eax, ecx for I64→I32 truncation)
- [ ] T048 [P] [US5] Implement float-to-int conversions in `src/asm/instruction_selector.rs`: translate FloatToInt cast to `cvttss2si` (F32→int) or `cvttsd2si` (F64→int) instructions with truncation toward zero
- [ ] T049 [P] [US5] Implement int-to-float conversions in `src/asm/instruction_selector.rs`: translate IntToFloat cast to `cvtsi2ss` (int→F32) or `cvtsi2sd` (int→F64) instructions
- [ ] T050 [P] [US5] Implement float-to-float conversions in `src/asm/instruction_selector.rs`: translate F32↔F64 casts to `cvtss2sd` (F32→F64) or `cvtsd2ss` (F64→F32) instructions
- [ ] T051 [US5] Implement bitcast operations in `src/asm/instruction_selector.rs`: translate Bitcast to `movd`/`movq` for register-to-register reinterpretation (int↔float) or simple register copy for same-size integer types

**Checkpoint**: Type conversions working - can compile mixed-type expressions and conversions

---

## Phase 8: User Story 6 - Error Detection and Reporting (Priority: P6)

**Goal**: Provide clear error messages when encountering unsupported, malformed, or invalid IR constructs

**Independent Test**: Provide deliberately malformed IR, verify generator collects appropriate error messages without crashing, returns them with potentially incomplete code

### Implementation for User Story 6

- [ ] T052 [P] [US6] Implement unsupported type detection in `src/asm/instruction_selector.rs`: check IR types against supported list (I8-I64, U8-U64, F32, F64, Bool, Char, Pointer, Void), return `CodeGenError::UnsupportedType` with location for others (I128, Struct, Array as value)
- [ ] T053 [P] [US6] Implement unknown instruction detection in `src/asm/instruction_selector.rs`: add catch-all match arm returning `CodeGenError::UnknownInstruction` with instruction kind and location
- [ ] T054 [P] [US6] Implement malformed instruction detection in `src/asm/instruction_selector.rs`: validate operand count and types for each instruction, return `CodeGenError::MalformedInstruction` with reason if invalid
- [ ] T055 [US6] Implement error accumulation in `src/asm/generator.rs`: collect errors in `self.errors` vector, continue generation for valid functions/blocks, skip only problematic instructions
- [ ] T056 [US6] Implement partial code generation in `src/asm/generator.rs`: ensure `generate_function` returns partial assembly even when some blocks have errors, mark failed functions in statistics
- [ ] T057 [US6] Add error context to `CodeGenError` variants in `src/asm/error.rs`: ensure all error variants include `SourceSpan` location information from IR instruction locations
- [ ] T058 [US6] Implement error reporting in `CodeGenResult` in `src/asm/generator.rs`: populate `errors` field with all accumulated errors, set `assembly` to `Some` if any functions succeeded or `None` if all failed

**Checkpoint**: Error handling complete - generator provides actionable diagnostics for unsupported constructs

---

## Phase 9: User Story 7 - Cross-Platform Assembly Generation (Priority: P7)

**Goal**: Generate platform-specific assembly (Windows/Linux/macOS) with correct calling conventions and system requirements

**Independent Test**: Generate assembly for same IR on different platforms, assemble and execute on each, verify behavior correct per platform conventions

### Implementation for User Story 7

- [ ] T059 [P] [US7] Implement platform-specific symbol mangling in `src/asm/generator.rs`: add underscore prefix for macOS symbols (_main, _function), no prefix for Linux/Windows (main, function)
- [ ] T060 [P] [US7] Implement platform-specific directives in `src/asm/generator.rs`: emit `default rel` for RIP-relative addressing on Linux/macOS, absolute addressing on Windows
- [ ] T061 [US7] Implement ABI-specific parameter passing in `src/asm/prologue_epilogue.rs`: use `Abi::parameter_registers` to select register order (RDI, RSI, RDX, RCX, R8, R9 for System V vs RCX, RDX, R8, R9 for Microsoft x64)
- [ ] T062 [US7] Verify shadow space handling in `src/asm/prologue_epilogue.rs`: ensure shadow space only allocated for Microsoft x64 ABI (32 bytes), not for System V
- [ ] T063 [US7] Implement platform detection in `src/asm/generator.rs`: use `TargetTriple` methods to determine platform (is_linux(), is_windows(), is_macos()) and select appropriate conventions

**Checkpoint**: Cross-platform generation working - same IR produces correct platform-specific assembly

---

## Phase 10: User Story 8 - Floating-Point Operations (Priority: P8)

**Goal**: Translate IR floating-point operations into SSE2/AVX scalar instructions with accurate numerical results

**Independent Test**: Provide IR with float/double arithmetic, generate assembly with SSE instructions, execute, compare results against expected values within precision

### Implementation for User Story 8

- [ ] T064 [P] [US8] Implement float binary operations in `src/asm/instruction_selector.rs`: translate F32 binary ops (Add→addss, Sub→subss, Mul→mulss, Div→divss) using XMM registers
- [ ] T065 [P] [US8] Implement double binary operations in `src/asm/instruction_selector.rs`: translate F64 binary ops (Add→addsd, Sub→subsd, Mul→mulsd, Div→divsd) using XMM registers
- [ ] T066 [P] [US8] Implement float comparison operations in `src/asm/instruction_selector.rs`: translate float comparisons to `ucomiss` (F32) or `ucomisd` (F64) followed by conditional jump based on flags
- [ ] T067 [US8] Implement XMM register allocation in `src/asm/register_allocator.rs`: extend `LinearScanAllocator` to handle separate XMM register class (xmm0-xmm15) for floating-point values
- [ ] T068 [US8] Implement float parameter passing in `src/asm/prologue_epilogue.rs`: pass F32/F64 parameters in XMM registers (xmm0-xmm7 for System V, xmm0-xmm3 for Microsoft x64) per ABI
- [ ] T069 [US8] Implement float return value handling in `src/asm/instruction_selector.rs`: place F32/F64 return values in xmm0 register
- [ ] T070 [US8] Implement float load/store in `src/asm/instruction_selector.rs`: use `movss` (F32) or `movsd` (F64) for memory operations with XMM registers

**Checkpoint**: Floating-point operations working - can compile numerical programs with F32/F64 types

---

## Phase 11: User Story 9 - Output File Management (Priority: P9)

**Goal**: Automatically save generated assembly to .asm file in predictable location

**Independent Test**: Run generator with output path specification, verify .asm file created at expected location with valid NASM content

### Implementation for User Story 9

- [ ] T071 [US9] Implement output path resolution in `src/asm/generator.rs`: determine output path (explicit path if provided, else same directory as input .vn file with .asm extension)
- [ ] T072 [US9] Implement directory creation in `src/asm/generator.rs`: create output directory if it doesn't exist using `std::fs::create_dir_all`, handle `IoError`
- [ ] T073 [US9] Implement file writing in `src/asm/generator.rs`: write assembly string to file using `std::fs::write`, wrap I/O errors in `CodeGenError::IoError`
- [ ] T074 [US9] Handle partial assembly file output in `src/asm/generator.rs`: write partial assembly to file even when errors present, document which functions failed in assembly comments

**Checkpoint**: Output file management complete - assembly automatically saved to appropriate location

---

## Phase 12: Register Allocation Implementation (Cross-Cutting)

**Goal**: Implement linear scan register allocation with spilling to enable complex functions with high register pressure

**Independent Test**: Provide IR function with 20+ live values, verify generator allocates registers and spills to stack, produces correct assembly

### Implementation

- [ ] T075 [P] Create `LiveInterval` struct in `src/asm/register_allocator.rs`: fields (value_id, start, end, use_positions, reg_class) and methods (overlaps, next_use_after)
- [ ] T076 [P] Create `RegisterAssignment` struct in `src/asm/register_allocator.rs`: mappings (register_map, spill_map, used_registers, used_callee_saved)
- [ ] T077 Implement liveness analysis in `src/asm/register_allocator.rs`: `compute_liveness(&Function) -> Vec<LiveInterval>` using backward dataflow on CFG basic blocks
- [ ] T078 Implement linear scan allocation in `src/asm/register_allocator.rs`: `allocate(&mut self, intervals: Vec<LiveInterval>) -> Result<RegisterAssignment>` implementing Poletto & Sarkar algorithm
- [ ] T079 Implement spilling logic in `src/asm/register_allocator.rs`: `spill(&mut self, interval: LiveInterval) -> StackSlot` using furthest-use heuristic
- [ ] T080 Implement spill code generation in `src/asm/instruction_selector.rs`: insert `mov [rbp-offset], reg` stores and `mov reg, [rbp-offset]` loads around spilled value uses
- [ ] T081 Update `StackFrame` in `src/asm/prologue_epilogue.rs`: add spill_slots_size to frame size calculation based on number of spilled values
- [ ] T082 Integrate allocator in `src/asm/generator.rs`: call `register_allocator.allocate()` after phi resolution, use assignment throughout instruction selection

**Checkpoint**: Register allocation complete - can handle functions with arbitrary register pressure

---

## Phase 13: SSA Phi Resolution (Cross-Cutting)

**Goal**: Resolve SSA phi functions through critical edge splitting

**Independent Test**: Provide IR in SSA form with phi functions, verify generator splits critical edges and inserts move instructions, produces correct assembly

### Implementation

- [ ] T083 [P] Implement critical edge detection in `src/asm/phi_resolver.rs`: `find_critical_edges(&Function) -> Vec<(BlockId, BlockId)>` identifying edges from blocks with multiple successors to blocks with multiple predecessors
- [ ] T084 Implement edge splitting in `src/asm/phi_resolver.rs`: `split_edge(&mut Function, from: BlockId, to: BlockId) -> BlockId` creates new basic block with unconditional branch, updates CFG
- [ ] T085 Implement phi resolution in `src/asm/phi_resolver.rs`: `resolve_phis(&mut Function)` splits critical edges, then converts phi(val1 from B1, val2 from B2) to moves at end of predecessor blocks
- [ ] T086 Integrate phi resolver in `src/asm/generator.rs`: call `phi_resolver.resolve_phis()` at start of function generation before register allocation

**Checkpoint**: SSA phi resolution complete - can handle IR in SSA form with phi functions

---

## Phase 14: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements, documentation, and validation

- [ ] T087 [P] Add comprehensive rustdoc comments to all public APIs in `src/asm/generator.rs`, `src/asm/register_allocator.rs`, `src/asm/instruction_selector.rs`
- [ ] T088 [P] Add assembly comments in `src/asm/generator.rs`: emit comments showing original IR instruction above each translated assembly block
- [ ] T089 [P] Add statistics collection in `src/asm/generator.rs`: track register spills, stack frame sizes, instruction counts in `CodeGenStats`
- [ ] T090 [P] Create integration tests in `tests/codegen_integration_tests.rs`: end-to-end tests for each user story with IR input → assembly output → NASM assembly → execution → result verification
- [ ] T091 [P] Create snapshot tests in `tests/codegen_snapshot_tests.rs`: Insta snapshot tests for generated assembly from test IR examples
- [ ] T092 [P] Create benchmarks in `benches/codegen_benchmark.rs`: Criterion benchmarks for generation performance (IR instructions/second), verify <1s per 1000 instructions target
- [ ] T093 Code cleanup: run rustfmt, clippy, address all warnings
- [ ] T094 Update `README.md` or create `docs/assembly-generator.md` with usage examples, architecture overview, and extension guide
- [ ] T095 Validate quickstart.md examples: ensure all code examples compile and execute correctly
- [ ] T096 Final validation: run full test suite (`cargo test --all`), benchmarks (`cargo bench`), verify all user story acceptance criteria met

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) completion - BLOCKS all user stories
- **User Stories (Phase 3-11)**: All depend on Foundational (Phase 2) completion
  - Can proceed in parallel if team capacity allows
  - OR sequentially in priority order: US1 → US2 → US3 → US4 → US5 → US6 → US7 → US8 → US9
- **Register Allocation (Phase 12)**: Can be developed in parallel with US1-US6 (simple register usage only), MUST complete before US7+ (requires proper allocation)
- **Phi Resolution (Phase 13)**: Can be developed in parallel with US1-US3, MUST complete before any IR in SSA form is used
- **Polish (Phase 14)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (Basic Function Translation)**: Can start after Foundational - No dependencies on other stories - **MVP candidate**
- **US2 (Control Flow)**: Depends on US1 (needs basic instruction selection) - Extends US1
- **US3 (Function Calls)**: Depends on US1 (needs basic generation) - Can run parallel with US2 after US1
- **US4 (Memory Operations)**: Depends on US1 (needs alloca, load, store from US1) - Extends US1
- **US5 (Type Conversions)**: Depends on US1 (needs basic instruction selection) - Can run parallel with US2/US3/US4
- **US6 (Error Handling)**: Can start after Foundational - Runs parallel with any story
- **US7 (Cross-Platform)**: Depends on US1, US3 (needs basic generation and calls working) - Integration story
- **US8 (Floating-Point)**: Depends on US1 (needs basic instruction selection) - Can run parallel with US2/US3/US4/US5
- **US9 (File Output)**: Depends on US1 (needs assembly generation) - Can be done last

### Critical Path (Minimum for MVP)

1. Phase 1 (Setup) → 2. Phase 2 (Foundational) → 3. Phase 3 (US1 Basic Translation) → 4. Phase 12 (Register Allocation partial) → 5. Phase 14 (Testing/Validation)

**MVP Delivery**: User Story 1 alone provides a complete, testable increment (simple function compilation)

### Parallel Opportunities

**Within Setup (Phase 1)**:
- T002 (generator skeleton), T003 (result types) can run in parallel

**Within Foundational (Phase 2)**:
- T006 (register allocator), T007 (instruction selector), T008 (phi resolver), T009 (frame generator) can all run in parallel

**Within Each User Story**:
- Tasks marked [P] can run in parallel within the story
- Example US1: T013-T016 (instruction selection variants) can all run in parallel
- Example US5: T045-T050 (cast types) can all run in parallel

**Across User Stories**:
- After US1 completes: US2, US3, US4, US5, US6, US8 can all start in parallel
- US7 waits for US1+US3
- US9 waits for US1

**Parallelism Example**:
```
Setup (1 day) → Foundational (2 days) → US1 (3 days) →
  ├─ US2 (2 days) ────────────┐
  ├─ US3 (2 days) ──────┐     │
  ├─ US4 (2 days)       │     ├─ US7 (1 day) → US9 (1 day)
  ├─ US5 (2 days)       │     │
  ├─ US6 (1 day)  ──────┤─────┘
  └─ US8 (2 days) ──────┘
```

**Sequential Estimate**: 1+2+3+2+2+2+2+1+1+2+1 = 19 developer-days
**Parallel Estimate** (3 developers): 1+2+3+2+1+1 = 10 calendar-days

---

## Implementation Strategy

### MVP First (User Story 1 Only)

**Recommended initial target**: Implement only Phase 1, Phase 2, Phase 3 (US1), and partial Phase 12 (basic register allocation without spilling).

**Benefits**:
- Delivers working end-to-end compilation in ~5-6 days
- Validates architecture and design decisions early
- Provides foundation for all other stories
- Enables early testing and feedback
- Each subsequent story adds independent value

**MVP Definition**:
- ✅ Can compile simple functions with integer arithmetic
- ✅ Can allocate local variables on stack
- ✅ Can generate valid NASM assembly
- ✅ Can assemble and execute compiled code
- ✅ Produces correct results for basic computations

### Incremental Delivery

After MVP, deliver user stories in priority order (P2 → P3 → P4...), with each story as a independently deployable increment:
- US2: Adds control flow → enables if/while/for compilation
- US3: Adds function calls → enables multi-function programs
- US4: Adds memory ops → enables arrays and pointers
- US5: Adds type conversions → enables mixed-type expressions
- US6: Adds error handling → improves developer experience
- US7: Adds cross-platform → enables Windows support
- US8: Adds floating-point → enables numerical programs
- US9: Adds file management → improves automation

---

## Task Summary

**Total Tasks**: 96
- Setup: 4 tasks
- Foundational: 8 tasks (T005-T012)
- US1 (Basic Translation): 12 tasks (T013-T024)
- US2 (Control Flow): 7 tasks (T025-T031)
- US3 (Function Calls/ABI): 8 tasks (T032-T039)
- US4 (Memory Operations): 5 tasks (T040-T044)
- US5 (Type Conversions): 7 tasks (T045-T051)
- US6 (Error Handling): 7 tasks (T052-T058)
- US7 (Cross-Platform): 5 tasks (T059-T063)
- US8 (Floating-Point): 7 tasks (T064-T070)
- US9 (Output Files): 4 tasks (T071-T074)
- Register Allocation: 8 tasks (T075-T082)
- Phi Resolution: 4 tasks (T083-T086)
- Polish: 10 tasks (T087-T096)

**Parallel Opportunities**: 43 tasks marked [P] can run in parallel with others in same phase

**Independent Test Criteria**: Each user story phase includes clear verification criteria for independent testing

**MVP Scope**: Phase 1 (4) + Phase 2 (8) + Phase 3/US1 (12) + Phase 12 partial (3) = ~27 tasks for minimal viable product
