# Feature Specification: Automatic Target Configuration for Module

**Feature Branch**: `024-auto-target-config`  
**Created**: 2025-02-08  
**Status**: Draft  
**Input**: User description: "Realizzare un comportamento automatico che assegni i campi DataLayout e TargetTriple della struttura Module in base al sistema operativo rilevato al momento dell'esecuzione, così da garantire una configurazione coerente dell'ambiente di destinazione. Il sistema deve distinguere esplicitamente tra Windows, Linux e macOS su architettura x86_64 e produrre valori coerenti e prevedibili per ciascuna combinazione supportata. I test devono validare il comportamento senza dipendere dal sistema operativo su cui vengono eseguiti, consentendo di simulare o verificare tutti i sistemi supportati in modo deterministico. Il supporto per architetture diverse da x86_64 non rientra nell'ambito corrente e sarà considerato come estensione futura. Tutti i test relativi a questo comportamento devono essere collocati nella cartella tests."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Default Module reflects host OS (Priority: P1)

When a developer creates a new Module without specifying any target configuration, the module must automatically detect the host operating system and set DataLayout and TargetTriple to values consistent with that OS on x86_64 architecture. This eliminates the current behaviour where every Module defaults to Linux regardless of the actual host.

**Why this priority**: This is the core value proposition of the feature. Without this, modules compiled on Windows or macOS silently produce code targeting Linux, leading to subtle and hard-to-diagnose compilation output errors.

**Independent Test**: Can be fully tested by creating a Module with default settings and verifying that the resulting DataLayout and TargetTriple correspond to the expected values for each of the three supported operating systems (Windows, Linux, macOS).

**Acceptance Scenarios**:

1. **Given** the compiler runs on a Windows x86_64 host, **When** a new Module is created with default settings, **Then** the DataLayout is set to the Windows x86_64 layout and TargetTriple is set to `x86_64-pc-windows-gnu`.
2. **Given** the compiler runs on a Linux x86_64 host, **When** a new Module is created with default settings, **Then** the DataLayout is set to the Linux x86_64 layout and TargetTriple is set to `x86_64-unknown-linux-gnu`.
3. **Given** the compiler runs on a macOS x86_64 host, **When** a new Module is created with default settings, **Then** the DataLayout is set to the macOS x86_64 layout and TargetTriple is set to `x86_64-apple-darwin`.

---

### User Story 2 - Deterministic testing across all platforms (Priority: P1)

Developers must be able to test the automatic detection behaviour for all three supported operating systems (Windows, Linux, macOS) from any single development machine, without requiring access to the other platforms. The tests must be deterministic and must not depend on the OS on which they are executed.

**Why this priority**: Equally critical to US-1 because without deterministic cross-platform test coverage, the correctness of the detection logic cannot be validated in CI or on a single developer machine. This directly enables confidence in the feature's correctness.

**Independent Test**: Can be fully tested by invoking the platform detection logic for each supported OS and verifying the returned DataLayout/TargetTriple pair matches the expected mapping, all within a single test run on any OS.

**Acceptance Scenarios**:

1. **Given** a test executing on any OS, **When** the detection logic is invoked for the "Windows" platform, **Then** it returns the Windows-specific DataLayout and TargetTriple.
2. **Given** a test executing on any OS, **When** the detection logic is invoked for the "Linux" platform, **Then** it returns the Linux-specific DataLayout and TargetTriple.
3. **Given** a test executing on any OS, **When** the detection logic is invoked for the "macOS" platform, **Then** it returns the macOS-specific DataLayout and TargetTriple.

---

### User Story 3 - DataLayout and TargetTriple consistency (Priority: P2)

The DataLayout and TargetTriple selected by the automatic detection must always be internally consistent — i.e., a Windows TargetTriple must never be paired with a Linux DataLayout. The system must guarantee this pairing at the point of detection.

**Why this priority**: Important but secondary to core detection, as inconsistent pairing would produce invalid compilation output. This validates internal correctness rather than adding new capability.

**Independent Test**: Can be fully tested by asserting that for every supported platform, the returned DataLayout and TargetTriple form a valid, documented pair.

**Acceptance Scenarios**:

1. **Given** automatic detection returns a TargetTriple for Windows, **When** the paired DataLayout is inspected, **Then** it corresponds to the Windows x86_64 data layout.
2. **Given** automatic detection returns a TargetTriple for Linux, **When** the paired DataLayout is inspected, **Then** it corresponds to the Linux x86_64 data layout.
3. **Given** automatic detection returns a TargetTriple for macOS, **When** the paired DataLayout is inspected, **Then** it corresponds to the macOS x86_64 data layout.

---

### User Story 4 - Manual override remains available (Priority: P3)

Even after automatic detection is in place, developers must still be able to explicitly set DataLayout and TargetTriple on a Module to override the detected defaults. This preserves cross-compilation workflows.

**Why this priority**: Lower priority because the existing set_data_layout / set_target_triple methods already exist. This story validates that automatic detection does not break them.

**Independent Test**: Can be tested by creating a Module (which will have auto-detected defaults), then calling the manual setters and verifying the overridden values persist.

**Acceptance Scenarios**:

1. **Given** a Module created with auto-detected defaults on any OS, **When** `set_data_layout` and `set_target_triple` are called with Linux-specific values, **Then** the Module's fields reflect the overridden Linux values regardless of the host OS.

---

### Edge Cases

- What happens when the host OS is not one of the three supported platforms (e.g., FreeBSD)? The system falls back to the Linux x86_64 configuration and emits a warning-level message to stderr via `eprintln!` indicating that the host was not explicitly recognized.
- What happens when the host architecture is not x86_64 (e.g., aarch64 on Apple Silicon)? For the current scope, the system still selects the x86_64 variant for the detected OS and emits a warning-level message to stderr via `eprintln!` indicating that the host architecture is not natively supported, since non-x86_64 architectures are out of scope for this feature. A future extension will address native architecture detection.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST automatically detect the host operating system at runtime and assign the corresponding x86_64 DataLayout and TargetTriple to every newly created Module.
- **FR-002**: The system MUST support three distinct platform configurations: Windows x86_64, Linux x86_64, and macOS x86_64.
- **FR-003**: The system MUST guarantee that the DataLayout and TargetTriple assigned to a Module are always internally consistent (i.e., they correspond to the same platform).
- **FR-004**: The system MUST provide a mechanism to query the detected platform configuration independently of Module creation, enabling deterministic testing of all supported platforms from any single host.
- **FR-005**: The system MUST fall back to the Linux x86_64 configuration when the host operating system is not explicitly recognized, emitting a warning to stderr via `eprintln!`.
- **FR-006**: The system MUST preserve the ability to manually override DataLayout and TargetTriple after automatic assignment.
- **FR-007**: All tests validating this behaviour MUST be placed in the `tests/` directory and MUST be executable deterministically regardless of the host operating system.

### Key Entities

- **Module**: Top-level compilation unit that contains functions, data layout, and target triple. Automatic detection populates its DataLayout and TargetTriple at creation time.
- **DataLayout**: Platform-specific specification describing data type sizes, alignments, and memory layout conventions. One variant per supported platform.
- **TargetTriple**: Platform identifier in `<arch>-<vendor>-<os>-<environment>` format. One variant per supported platform.
- **Platform Configuration**: The paired combination of a DataLayout and a TargetTriple representing a single supported target environment. The system exposes a mapping from detected OS to platform configuration.

## Assumptions

- The host architecture is x86_64 for the purposes of this feature. Non-x86_64 architectures are out of scope and will be addressed in a future extension.
- The compiler is expected to run on one of the three supported platforms (Windows, Linux, macOS). For any other platform, a Linux x86_64 fallback is acceptable.
- Existing callers that rely on the previous behaviour (always defaulting to Linux) will need to be aware of this change. This is considered a beneficial breaking change since the previous default was incorrect for non-Linux hosts. Existing tests may be adapted as part of this feature to reflect the new default; no backward-compatibility shim is required.
- The existing `set_data_layout` and `set_target_triple` methods continue to work unchanged as manual overrides.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A Module created on Windows produces output targeting Windows x86_64 without any manual configuration.
- **SC-002**: A Module created on Linux produces output targeting Linux x86_64 without any manual configuration.
- **SC-003**: A Module created on macOS produces output targeting macOS x86_64 without any manual configuration.
- **SC-004**: 100% of the three supported platform configurations (Windows, Linux, macOS) are testable and tested from a single host OS in a single test run.
- **SC-005**: All existing tests that depend on Module creation continue to pass in the final state (after adapting them to the new default behaviour), with zero regressions. Test adaptation is part of this feature's scope.
- **SC-006**: Manual override of DataLayout and TargetTriple continues to function correctly after automatic detection is applied.

## Clarifications

### Session 2026-02-08

- Q: Does SC-005 ("zero regressions") apply to the final state (tests adapted) or require no test changes at all? → A: SC-005 means zero failures in the final state — existing tests can be adapted as part of this feature.
- Q: Should the system emit a warning when the host architecture is not x86_64 (e.g., aarch64 on Apple Silicon)? → A: Yes, emit a warning analogous to the one for unrecognized OS.
- Q: What mechanism should be used to emit warnings for unrecognized platform/architecture? → A: Direct output to stderr via eprintln!.
