
# Implementation Plan: Assembly SSE and SSE2 Support

**Branch**: `003-aggiorna-tutto-il` | **Date**: 2025-09-24 | **Spec**: [/specs/003-aggiorna-tutto-il/spec.md](/specs/003-aggiorna-tutto-il/spec.md)
**Input**: Feature specification from `/specs/003-aggiorna-tutto-il/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
This feature adds SSE and SSE2 instruction support to the jsavrs compiler system's assembly generation capabilities. The implementation will replace scalar arithmetic operations (ADD, MUL, SUB) on floats/doubles with SIMD equivalents (ADDPS, MULPS, ADDPD, MULPD) where applicable, optimize loops operating on arrays/vectors using XMM registers, and include fallback scalar instructions when SSE/SSE2 is not available. The system must maintain backward compatibility with existing interfaces and provide graceful degradation on older processors. Implementation will be at the intermediate representation phase with compile-time flags and runtime detection.

## Technical Context
**Language/Version**: Rust 1.75+  
**Primary Dependencies**: Logos (lexer), Clap (CLI), Thiserror (error handling), Insta (snapshot testing), Criterion.rs (benchmarking)  
**Storage**: Files in src/asm/ directory including generator.rs, instruction.rs, operand.rs, register.rs  
**Testing**: Cargo test, Insta snapshot testing, custom SIMD validation harness, benchmarking infrastructure  
**Target Platform**: Windows, Linux, macOS supporting SSE/SSE2 instructions (Pentium III+ CPU)  
**Project Type**: Single project (compiler system)  
**Performance Goals**: 20-50% execution speedup for vectorizable operations with measurable benchmarks  
**Constraints**: Maintain backward compatibility with existing assembly interfaces, cross-platform compilation, graceful degradation on older CPUs  
**Scale/Scope**: Vectorizable operations with typically 8-16 elements per loop for optimal performance

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, and Documentation Rigor. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

## Scope Alignment

### Purpose and Relationship

This section establishes the scope boundaries for SSE3+ instruction research within the broader SIMD implementation framework. While the primary focus of this implementation plan centers on SSE and SSE2 instruction support, the relationship between advanced SIMD capabilities (SSE3+) and effective implementation design requires careful alignment to ensure architectural coherence and future extensibility.

### SSE3+ Instructions: Context vs. Implementation Scope

While SSE3+ instructions (including SSE3, SSSE3, SSE4.1, SSE4.2, and the AVX instruction family) are **technically outside the main research and implementation scope** of this feature, understanding their architectural evolution and capabilities is **crucial for effective SIMD deployment and system design**. This contextual knowledge directly impacts the quality and extensibility of the current SSE/SSE2 implementation.

The SSE3+ instruction sets represent the natural evolution path for SIMD optimization, and comprehending their design patterns, performance characteristics, and architectural improvements informs critical decisions in the current implementation phase. Without this broader context, the SSE/SSE2 implementation risks architectural limitations that would complicate future enhancements.

### Context and Understanding for Research Integration

Understanding SSE3+ capabilities ensures successful integration with the planned research activities in several critical dimensions:

**Architectural Design Context**: Knowledge of SSE3+ instruction patterns directly influences the design of trait abstractions, register allocation strategies, and code generation frameworks in the current SSE/SSE2 implementation. This ensures that architectural decisions made today support seamless extension to advanced instruction sets.

**Performance Optimization Context**: Comprehending the performance characteristics and optimization opportunities available in SSE3+ instructions provides essential baseline knowledge for validating current SSE/SSE2 optimization strategies. This context prevents implementation of patterns that would be suboptimal when extended to more advanced SIMD capabilities.

**Research Activity Coherence**: The Phase 0 research tasks, particularly those investigating "efficient SIMD implementation strategies" and "trait-based dispatch mechanisms," require SSE3+ contextual awareness to produce architecturally sound solutions. Research conducted in isolation from the broader SIMD landscape risks producing designs with inherent extensibility limitations.

### Recommendations and Considerations for Ongoing Research

**Contextual Research Integration**: Research activities should incorporate SSE3+ awareness through focused contextual analysis rather than implementation exploration. Specific recommendations include:

1. **Architectural Pattern Studies**: Research tasks investigating SIMD implementation strategies should analyze how industry-standard compilers (LLVM, GCC) handle multi-generation SIMD instruction support, focusing on extensibility patterns applicable to the current SSE/SSE2 implementation.

2. **Performance Baseline Establishment**: Benchmarking and performance analysis should consider SSE3+ capabilities as reference points for validating current optimization approaches, ensuring that SSE/SSE2 implementations follow patterns that scale effectively to more advanced instruction sets.

3. **Interface Design Validation**: API contract and trait design research should validate proposed abstractions against the broader SIMD instruction landscape, ensuring that current interface decisions support future instruction set integration without requiring architectural restructuring.

**Documentation and Knowledge Management**: All research findings related to SSE3+ contextual understanding should be documented as architectural notes and future enhancement guidance, providing a knowledge foundation for subsequent implementation phases while maintaining clear boundaries around current scope deliverables.

**Research Scope Boundaries**: While contextual understanding of SSE3+ is essential, research activities must maintain disciplined focus on SSE/SSE2 implementation requirements. SSE3+ investigation should be limited to architectural context gathering rather than detailed technical analysis that could compromise delivery timeline or scope clarity.

## Project Structure

### Documentation (this feature)
```
specs/003-aggiorna-tutto-il/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 1: Single project (DEFAULT) - jsavrs compiler
src/
├── cli.rs              # Command-line interface
├── lexer.rs            # Lexer implementation using Logos
├── lib.rs              # Library exports
├── main.rs             # Main entry point
├── error/              # Error handling with thiserror
├── ir/                 # Intermediate representation (NIR, HIR)
├── location/           # Source location tracking
├── parser/             # Parser and AST
├── printers/           # AST/HIR printers
├── semantic/           # Semantic analysis (type checking)
├── time/               # Timing utilities
├── tokens/             # Token definitions
├── utils/              # Utility functions
├── asm/                # Assembly code generation modules
│   ├── generator.rs    # Assembly generator
│   ├── instruction.rs  # Assembly instructions
│   ├── operand.rs      # Assembly operands
│   └── register.rs     # Assembly registers
└── [Additional modules as needed]

tests/
├── snapshots/          # Snapshot tests with insta
└── [Other test files]

benches/                # Benchmarking with Criterion.rs
```

**Structure Decision**: Option 1 (Single project compiler system) - The jsavrs project is a Rust-based compiler system with assembly generation capability

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   For SSE/SSE2 implementation:
     Task: "Investigate efficient SIMD implementation strategies in Rust, emphasizing performance optimization, maintainability, and code safety"
     Task: "Analyze the use of the CPUID instruction to detect processor capabilities, ensuring correct selection of SIMD features for different architectures."
     Task: "Research aligned vs unaligned memory access patterns for SIMD"
     Task: "Investigate trait-based dispatch mechanisms in Rust for selecting between SIMD and scalar operations, considering both runtime efficiency and code modularity."
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]
   - Performance impact: [expected performance improvement]

**Output**: research.md with all NEEDS CLARIFICATION resolved In great detail, precisely, meticulously, and in depth.

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Identify the name, attributes, and relationships of each entity to ensure a comprehensive understanding of the data model In great detail, precisely, meticulously, and in depth.
   - Define validation rules as specified in the requirements document, including constraints, formats, and mandatory fields
   - Document relevant state transitions for each entity, where applicable, to capture the entity lifecycle and behavior.

2. **Generate API contracts** from functional requirements:
   - For SIMD instruction generation → assembly generation function
   - For CPU detection → CPUID checking function
   - For SIMD/scalar selection → trait-based dispatch interface
   - Use appropriate Rust patterns for low-level operations
   - Output function signatures and interface specifications to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per SIMD operation type
   - Assert SIMD vs scalar result equivalency
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → SIMD operation validation scenario
   - Quickstart test = SIMD performance validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/powershell/update-agent-context.ps1 -AgentType qwen`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 40-50 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| SIMD Implementation | Performance optimization requirements (20-50% speedup) | Scalar-only implementation would not meet performance goals |
| Assembly Complexity | Low-level SSE/SSE2 instructions required for optimization | Higher-level abstractions would not provide necessary performance |

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
