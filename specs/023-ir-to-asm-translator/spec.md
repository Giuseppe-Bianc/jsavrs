# Feature Specification: IR to x86-64 Assembly Translator

**Feature Branch**: `023-ir-to-asm-translator`
**Created**: 2026-02-01
**Status**: Draft
**Input**: User description: "Progettare un traduttore che converta un IR proprietario, definito e mantenuto nella cartella `src/ir`, in codice assembly x86-64. L'obiettivo principale è consentire la generazione di file assembly corretti, completi e semanticamente coerenti a partire dalle strutture dell'IR, mantenendo una corrispondenza chiara tra le astrazioni di alto livello dell'IR e le istruzioni assembly prodotte. Il traduttore deve basarsi in modo integrato sul codice già presente nella cartella `src/asm`, che rappresenta il fondamento concettuale e strutturale per la generazione dell'assembly. Le specifiche del progetto devono garantire che tale codice venga riutilizzato e orchestrato, evitando duplicazioni logiche o bypass delle astrazioni già esistenti. Il risultato della traduzione deve essere codice assembly x86-64 sintatticamente valido e direttamente assemblabile con NASM, senza necessità di modifiche manuali successive. La generazione dell'output deve rispettare le convenzioni dell'architettura target (registri, chiamate di funzione, gestione dello stack, sezioni del file assembly), assicurando coerenza e prevedibilità del comportamento del codice prodotto. Dal punto di vista funzionale, il traduttore deve:

- Interpretare in modo deterministico le strutture dell'IR e trasformarle in istruzioni assembly equivalenti.
- Gestire il flusso di controllo (sequenze, salti, condizioni) e i dati (registri, memoria, costanti) definiti nell'IR.
- Produrre un output leggibile e strutturato, facilitando il debugging e l'analisi del codice assembly generato.
- Essere progettato per essere estendibile, così da supportare future evoluzioni dell'IR senza richiedere una riscrittura sostanziale. Poiché sia l'IR sia il generatore di assembly sono sviluppati in Rust, le specifiche devono riflettere uno stile progettuale idiomatico per questo linguaggio: separazione chiara delle responsabilità, uso coerente di strutture e astrazioni, e un'architettura che favorisca sicurezza, chiarezza e manutenibilità del codice."

## Clarifications

### Session 2026-02-02

- Q: What is explicitly out of scope for this translator? → A: D (All of the above: no optimizations, no register allocator, no linking/runtime)

- Q: Which ABI/calling convention do we want to assume as the target for assembly generation? → A: C (Support both: System V AMD64 and Windows x64, selectable via a target flag)

- Q: How should we handle IR constructs with no direct equivalent in x86-64? → A: B (Emit a compile-time error; fail fast)

- Q: Do you want the translator to generate maps/annotations between IR nodes and assembly lines (source maps)? → A: C (Make mapping optional via flag, default off)

- Q: How should error handling and logging be implemented in the translator? → A: A (Focus on error handling and logging for production use)

- Q: What type of logging approach should the translator use? → A: A (Structured logging with configurable levels (trace/debug/info/warn/error))

- Q: What are the performance requirements for the translator? → A: B (Target 100ms average translation time per function with 1GB RAM limit)

- Q: What level of input validation should be implemented? → A: A (Basic input validation with bounds checking and format verification)

- Q: Should the translator support debugging of generated assembly? → A: C (Generate assembly with standard debugging symbols (DWARF on Unix, PDB on Windows))

### Session 2026-02-05

- Q: Which InstructionKind constructs are supported in the MVP? → A: A (All InstructionKind defined in src/ir except Phi)
- Q: How should the translator handle type conversions/promotions between operands of different sizes? → A: A (Assume IR is type-consistent; emit error if incompatible operands)
- Q: Which component in src/asm should be the main integration point for the translator? → A: B (Defer to planning phase; document discovered components in plan)
- Q: How should the translator handle global variables and constants in assembly data sections? → A: A (Constants in `.rodata`, mutable globals in `.data`, uninitialized in `.bss`)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Translate IR to Valid Assembly (Priority: P1)

As a compiler developer, I want to convert IR structures from src/ir into valid x86-64 assembly code so that I can generate executable programs from high-level representations.

**Why this priority**: This is the core functionality of the translator - without this basic conversion capability, the entire system fails to deliver value.

**Independent Test**: Can be fully tested by providing a simple IR input and verifying that the output is syntactically correct x86-64 assembly that can be assembled with NASM without errors, delivering the fundamental translation capability.

**Acceptance Scenarios**:

1. **Given** a valid IR structure representing a simple function, **When** the translator processes it, **Then** it produces syntactically correct x86-64 assembly that assembles without errors
2. **Given** an IR structure with basic arithmetic operations, **When** the translator processes it, **Then** it produces equivalent x86-64 assembly instructions that perform the same operations

---

### User Story 2 - Maintain Semantic Consistency (Priority: P1)

As a compiler developer, I want the translator to maintain semantic consistency between IR and assembly so that the generated code behaves identically to the original high-level representation.

**Why this priority**: Semantic consistency is critical for correctness - if the translated code doesn't behave the same way as the IR, the compiler produces incorrect programs.

**Independent Test**: Can be tested by comparing the execution behavior of the original IR and the generated assembly code with identical inputs, delivering functional equivalence.

**Acceptance Scenarios**:

1. **Given** an IR structure with conditional logic, **When** the translator processes it, **Then** the resulting assembly produces the same logical branching behavior
2. **Given** an IR structure with function calls, **When** the translator processes it, **Then** the resulting assembly maintains the same calling conventions and parameter passing

---

### User Story 3 - Follow Architecture Conventions (Priority: P2)

As a compiler developer, I want the translator to follow x86-64 architecture conventions so that the generated assembly integrates properly with other system components and standard toolchains.

**Why this priority**: Following architecture conventions ensures compatibility with existing systems, debuggers, and development tools.

**Independent Test**: Can be tested by verifying that generated assembly follows x86-64 register usage, calling conventions, and stack management patterns, delivering compatibility with standard toolchains.

**Acceptance Scenarios**:

1. **Given** an IR structure requiring register allocation, **When** the translator processes it, **Then** it uses appropriate x86-64 registers following standard conventions
2. **Given** an IR structure with function calls, **When** the translator processes it, **Then** it follows x86-64 calling conventions for parameter passing and return values

---

### User Story 4 - Integrate with Existing Assembly Generator (Priority: P2)

As a compiler developer, I want the translator to integrate with existing code in src/asm so that I can leverage existing abstractions and avoid duplicating logic.

**Why this priority**: Integration with existing code promotes maintainability and reduces redundancy, making the system easier to maintain.

**Independent Test**: Can be tested by verifying that the translator utilizes existing src/asm components appropriately without reimplementing their functionality, delivering code reuse benefits.

**Acceptance Scenarios**:

1. **Given** IR structures that map to existing assembly generation patterns, **When** the translator processes them, **Then** it delegates to appropriate src/asm components
2. **Given** the translator needs to generate assembly code, **When** it encounters patterns already handled by src/asm, **Then** it reuses existing functionality rather than duplicating it

---

### User Story 5 - Support Extensibility for Future IR Changes (Priority: P3)

As a compiler developer, I want the translator to be designed for extensibility so that I can accommodate future evolutions of the IR without major rewrites.

**Why this priority**: Extensibility ensures long-term maintainability as the IR evolves, preventing costly rewrites.

**Independent Test**: Can be tested by extending the IR with a new construct and verifying that the translator can handle it with minimal changes, delivering future-proofing value.

**Acceptance Scenarios**:

1. **Given** a new IR construct is added, **When** the translator is extended to support it, **Then** it can be integrated with minimal changes to existing code
2. **Given** the translator architecture, **When** new IR features are introduced, **Then** they can be implemented following established patterns

---

### Edge Cases

- What happens when the IR contains constructs that don't have direct x86-64 equivalents?
- How does the system handle IR structures that exceed typical assembly complexity limits?
- What occurs when the IR contains invalid or malformed structures?
- How does the system handle extremely large IR inputs that might cause performance issues?

- Policy: For IR constructs with no direct equivalent on x86-64, the translator must abort the translation and issue a clear diagnostic error (fail-fast). No best-effort code will be generated, nor will runtime helper calls be inserted for these cases; any workarounds must be handled upstream or introduced via later extensions.

## Requirements *(mandatory)*

### Out of Scope

- This translator explicitly excludes the following tasks: IR or peephole optimizations, register allocation, Phi elimination, and linking/runtime integration. The component is solely responsible for deterministically transforming the IR into NASM-compatible assembly, leaving optimizations and register allocation to other stages of the toolchain.

- **Phi Elimination**: The IR supports SSA form with `Phi` instructions (`InstructionKind::Phi`). The translator assumes Phi nodes have been eliminated in a prior lowering pass (e.g., through parallel copy insertion or critical edge splitting). If a Phi instruction is encountered during translation, the translator MUST emit error `E4001` and abort. Phi elimination is the responsibility of the IR lowering pipeline, not the assembly translator.

### Target ABIs

- The translator must support both target ABIs: `System V AMD64` (Linux/macOS) and `Windows x64` (Microsoft calling convention). The target ABI must be selectable via a generator configuration/compilation flag; if not specified, the default behavior is the plantform default.

### IR→ASM Mapping (Diagnostics)

- The translator must offer the optional generation of maps/annotations that connect IR nodes to lines/labels in the produced assembly file. This feature must be enabled via an output flag (`--emit-mapping`), and is disabled by default. The format can be inline comments in the `.asm` file or a separate `.map` file; the initial implementation must document the chosen format.

### Functional Requirements

- **FR-000**: System MUST support translation of ALL `InstructionKind` variants defined in `src/ir` except `Phi` (which requires prior elimination per Phi Elimination policy). This includes arithmetic, logical, memory, control flow, comparison, conversion, and call instructions.
- **FR-001**: System MUST translate IR structures from src/ir into syntactically correct x86-64 assembly code
- **FR-002**: System MUST generate assembly code that is directly assemblable with NASM without requiring manual modifications
- **FR-003**: System MUST maintain semantic consistency between IR abstractions and generated assembly instructions
- **FR-004**: System MUST follow x86-64 architecture conventions for registers, function calls, and stack management
- **FR-005**: System MUST integrate with existing code in src/asm without duplicating logic or bypassing existing abstractions
- **FR-006**: System MUST handle control flow constructs (sequences, jumps, conditions) from the IR and translate them to equivalent assembly patterns
- **FR-007**: System MUST manage data representations (registers, memory, constants) as defined in the IR
- **FR-008**: System MUST produce readable and structured assembly output to facilitate debugging and analysis
- **FR-009**: System MUST be designed for extensibility to accommodate future IR evolutions without requiring substantial rewrites
- **FR-010**: System MUST interpret IR structures deterministically to ensure consistent translation results
- **FR-011**: System MUST generate appropriate assembly file sections that conform to NASM requirements
- **FR-012**: System MUST preserve the relationship between high-level IR concepts and their low-level assembly implementations
- **FR-013**: System MUST place global data in appropriate sections: constants in `.rodata` (read-only), mutable global variables in `.data`, and uninitialized variables in `.bss`

### Non-Functional Requirements

- **NFR-001**: System MUST implement comprehensive error handling with detailed diagnostics for production use
- **NFR-002**: System MUST use structured logging with configurable levels (trace/debug/info/warn/error)
- **NFR-003**: System MUST achieve target performance of 100ms average translation time per function with 1GB RAM limit
- **NFR-004**: System MUST perform input validation including:
    - **Structural validation**: All IR nodes are well-formed per the IR schema
    - **Reference validation**: All symbol references (functions, labels, variables) resolve to defined entities
    - **Type validation**: Operations are type-consistent (e.g., no arithmetic on function pointers). The translator assumes the IR is already type-consistent with explicit conversion instructions (ZExt, SExt, Trunc, etc.); operands of incompatible sizes without explicit conversion MUST trigger error `E4002` and abort translation.
    - **Bounds checking**:
        - Register indices are valid for x86-64:
            - General-purpose registers: 0-15 (RAX-R15, including legacy AL-DI as 0-7)
            - XMM registers (SSE): 0-15 (XMM0-XMM15)
            - YMM registers (AVX): 0-15 (YMM0-YMM15)
            - FPU registers (x87): 0-7 (ST0-ST7)
        - Immediate values fit within instruction-specific encoding limits:
            - 8-bit immediates: -128 to 255 (sign/zero extended depending on instruction)
            - 16-bit immediates: -32,768 to 65,535
            - 32-bit immediates: -2^31 to 2^31-1 (most arithmetic/logical operations)
            - 64-bit immediates: 0 to 2^64-1 (MOV instruction only; other instructions limited to 32-bit sign-extended)
        - Stack offsets are within x86-64 addressing limits:
            - Architectural limit: ±2^31 bytes (32-bit signed displacement)
            - Practical sanity threshold: ±16 MB (configurable; catches excessive stack usage early)
    - **ABI validation**: Function signatures are compatible with selected calling convention
    - Early failure: Invalid input must be rejected before translation begins
- **NFR-005**: System MUST generate assembly with debugging information to enable debugging of the generated code
    - **MVP scope**: Generate NASM-compatible debug directives (e.g., `%line` directives, symbolic labels) that work cross-platform and can be consumed by assemblers/linkers to produce debug info
    - **Post-MVP**: Native DWARF section generation on Unix; PDB generation on Windows is explicitly deferred due to toolchain complexity (requires cv2pdb or similar external tooling)

### Key Entities

- **IR Structures**: Represent the intermediate representation elements from src/ir that need to be translated to assembly, including expressions, statements, control flow constructs, and data types
- **Assembly Instructions**: The x86-64 assembly code elements that result from IR translation, including opcodes, operands, registers, and addressing modes
- **Translation Mapping**: Defines the correspondence between IR constructs and their equivalent assembly representations, ensuring semantic consistency
- **Architecture Conventions**: Represents x86-64 specific requirements including calling conventions, register usage, stack management, and section organization
- **Code Generation Components**: The existing assembly generation code in src/asm that the translator must integrate with and reuse
- **Assembly Output**: The final NASM-compatible assembly file that serves as the translator's output, organized into appropriate sections and following syntax requirements

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of valid IR inputs produce syntactically correct x86-64 assembly that assembles successfully with NASM
- **SC-002**: Generated assembly code maintains functional equivalence to the original IR with 99% accuracy in execution behavior
- **SC-003**: Translation process completes within acceptable timeframes (under 30 seconds for typical compilation units)
- **SC-004**: At least 95% of x86-64 architecture conventions are correctly implemented in generated assembly
- **SC-005**: The system successfully integrates with existing src/asm components without introducing redundant code
- **SC-006**: Generated assembly code is readable and debuggable, with clear correspondence to IR constructs
- **SC-007**: The system accommodates new IR features with minimal architectural changes (less than 20% of core translation logic modified per new IR construct)
- **SC-008**: Error handling provides detailed diagnostics for at least 95% of failure scenarios
- **SC-009**: Logging system captures appropriate information at all configurable levels without performance degradation
- **SC-010**: Translation performance stays within 100ms average per function with memory usage under 1GB
- **SC-011**: Input validation detects and handles at least 95% of malformed IR inputs appropriately
- **SC-012**: Generated assembly includes NASM debug directives (`%line`, symbolic labels) that enable basic source-level debugging; native DWARF/PDB generation is tracked as post-MVP enhancement
