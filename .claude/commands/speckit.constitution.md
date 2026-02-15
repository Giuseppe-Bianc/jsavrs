---
description: Create or update the project constitution from interactive or provided principle inputs, ensuring all dependent templates stay in sync.
handoffs: 
  - label: Build Specification
    agent: speckit.specify
    prompt: Implement the feature specification based on the updated constitution. I want to build...
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

You are updating the project constitution at `.specify/memory/constitution.md`. This file is a TEMPLATE containing placeholder tokens in square brackets (e.g. `[PROJECT_NAME]`, `[PRINCIPLE_1_NAME]`). Your job is to (a) collect/derive concrete values, (b) fill the template precisely, and (c) propagate any amendments across dependent artifacts.

**Note**: If `.specify/memory/constitution.md` does not exist yet, it should have been initialized from `.specify/templates/constitution-template.md` during project setup. If it's missing, copy the template first.

Follow this execution flow:

1. Load the existing constitution at `.specify/memory/constitution.md`.
   - Identify every placeholder token of the form `[ALL_CAPS_IDENTIFIER]`.
   **IMPORTANT**: The user might require less or more principles than the ones used in the template. If a number is specified, respect that - follow the general template. You will update the doc accordingly.

2. Collect/derive values for placeholders:
   - If user input (conversation) supplies a value, use it.
   - Otherwise infer from existing repo context (README, docs, prior constitution versions if embedded).
   - For governance dates: `RATIFICATION_DATE` is the original adoption date (if unknown ask or mark TODO), `LAST_AMENDED_DATE` is today if changes are made, otherwise keep previous.
   - `CONSTITUTION_VERSION` must increment according to semantic versioning rules:
     - MAJOR: Backward incompatible governance/principle removals or redefinitions.
     - MINOR: New principle/section added or materially expanded guidance.
     - PATCH: Clarifications, wording, typo fixes, non-semantic refinements.
   - If version bump type ambiguous, propose reasoning before finalizing.

3. Draft the updated constitution content:
   - Replace every placeholder with concrete text (no bracketed tokens left except intentionally retained template slots that the project has chosen not to define yet—explicitly justify any left).
   - Preserve heading hierarchy and comments can be removed once replaced unless they still add clarifying guidance.
   - Ensure each Principle section: succinct name line, paragraph (or bullet list) capturing non‑negotiable rules, explicit rationale if not obvious.
   - Ensure Governance section lists amendment procedure, versioning policy, and compliance review expectations.

4. Consistency propagation checklist (convert prior checklist into active validations):
   - Read `.specify/templates/plan-template.md` and ensure any "Constitution Check" or rules align with updated principles.
   - Read `.specify/templates/spec-template.md` for scope/requirements alignment—update if constitution adds/removes mandatory sections or constraints.
   - Read `.specify/templates/tasks-template.md` and ensure task categorization reflects new or removed principle-driven task types (e.g., observability, versioning, testing discipline).
   - Read each command file in `.specify/templates/commands/*.md` (including this one) to verify no outdated references (agent-specific names like CLAUDE only) remain when generic guidance is required.
   - Read any runtime guidance docs (e.g., `README.md`, `docs/quickstart.md`, or agent-specific guidance files if present). Update references to principles changed.

5. Produce a Sync Impact Report (prepend as an HTML comment at top of the constitution file after update):
   - Version change: old → new
   - List of modified principles (old title → new title if renamed)
   - Added sections
   - Removed sections
   - Templates requiring updates (✅ updated / ⚠ pending) with file paths
   - Follow-up TODOs if any placeholders intentionally deferred.

6. Validation before final output:
   - No remaining unexplained bracket tokens.
   - Version line matches report.
   - Dates ISO format YYYY-MM-DD.
   - Principles are declarative, testable, and free of vague language ("should" → replace with MUST/SHOULD rationale where appropriate).

7. Write the completed constitution back to `.specify/memory/constitution.md` (overwrite).

8. Output a final summary to the user with:
   - New version and bump rationale.
   - Any files flagged for manual follow-up.
   - Suggested commit message (e.g., `docs: amend constitution to vX.Y.Z (principle additions + governance update)`).

## Patterns: Best Practices for Constitution Management

### Pattern: Semantic Version-Driven Change Classification

**Objective:** Maintain clear governance evolution history and enable consumers to quickly assess backward compatibility of constitutional changes.

**Context of Application:** Every constitution update, regardless of scope, when incrementing the `CONSTITUTION_VERSION` field.

**Key Characteristics:**

- Version follows semantic versioning (MAJOR.MINOR.PATCH) with governance-specific semantics
- MAJOR increments signal breaking changes requiring dependent artifact review
- MINOR increments indicate additive changes expanding governance scope
- PATCH increments mark non-semantic refinements safe for immediate adoption
- Version bump rationale explicitly documented in Sync Impact Report

**Operational Guidance:**

1. Before any modification, load the current `CONSTITUTION_VERSION` and parse into components (e.g., "2.3.1" → MAJOR=2, MINOR=3, PATCH=1)
2. Analyze the nature of all proposed changes against classification criteria:
   - MAJOR: Principle removal, principle redefinition that invalidates prior interpretations, governance procedure change requiring new approval workflows, constraint removal that was previously mandatory
   - MINOR: New principle addition, new governance section, material expansion of existing principle guidance that adds enforceable rules, new mandatory template section
   - PATCH: Typo corrections, wording clarifications preserving original intent, formatting improvements, comment additions, example refinements
3. If multiple change types present, use highest severity level (MAJOR > MINOR > PATCH)
4. If classification ambiguous, document the ambiguity and propose reasoning: "Change X could be MINOR (additive constraint) or MAJOR (redefines scope) because Y. Recommending MAJOR for safety."
5. Increment the appropriate component: MAJOR increments reset MINOR and PATCH to 0; MINOR increments reset PATCH to 0; PATCH increments only rightmost digit
6. Record version change and rationale in Sync Impact Report header comment: `<!-- Version: 2.3.1 → 3.0.0 (MAJOR: Removed Principle 4, redefined versioning governance) -->`
7. Update `LAST_AMENDED_DATE` to current date in ISO format (YYYY-MM-DD)

### Pattern: Transitive Dependency Synchronization

**Objective:** Prevent constitution-template inconsistencies by systematically propagating constitutional changes to all dependent artifacts in a single atomic operation.

**Context of Application:** After completing constitution content updates, before finalizing the write operation.

**Key Characteristics:**

- Dependency graph explicitly enumerated: constitution → plan/spec/tasks templates → command files → documentation
- Each dependent artifact validated against updated principles
- Synchronization status tracked per artifact (✅ updated / ⚠ pending / ➖ not applicable)
- Sync Impact Report documents propagation outcomes

**Operational Guidance:**

1. After drafting updated constitution content, identify all dependent artifacts by reading file system: `.specify/templates/plan-template.md`, `.specify/templates/spec-template.md`, `.specify/templates/tasks-template.md`, `.specify/templates/commands/*.md`, `README.md`, `docs/**/*.md`
2. For each dependent artifact, perform targeted validation:
   - Plan template: Verify "Constitution Check" or "Compliance Validation" sections reference current principle names and constraints; add new validation steps for new principles; remove validation steps for deleted principles
   - Spec template: Ensure mandatory sections align with constitutional requirements (e.g., if constitution mandates observability, spec template must include "## Observability" section); update section descriptions if principle rationale changed
   - Tasks template: Verify task categorization aligns with principle-driven task types (e.g., new security principle requires "Security Tasks" category); update task acceptance criteria templates to reference relevant principles
   - Command files: Search for hardcoded principle references or outdated governance procedures; replace with current versions; verify no agent-specific names when generic guidance required
   - Documentation: Update quickstart guides, contribution guidelines, and architecture docs to reference amended principles; ensure examples align with new constraints
3. For each validation, determine outcome: ✅ (updated in this operation), ⚠ (requires manual follow-up due to complexity/ambiguity), ➖ (not affected by this change)
4. Write all ✅ artifacts atomically with constitution update (multi-file transaction)
5. In Sync Impact Report, list all artifacts with status: `Templates requiring updates: ✅ plan-template.md (added Principle 5 validation), ⚠ docs/architecture.md (complex diagram update needed), ➖ tasks-template.md (no affected sections)`
6. If any ⚠ items exist, add to Follow-up TODOs section with specific action items
7. Validate sync completeness: no orphaned principle references in dependent files, no missing mandatory sections introduced by new principles

### Pattern: Context-Driven Value Inference

**Objective:** Minimize user interaction burden by intelligently deriving placeholder values from existing repository artifacts while maintaining transparency and user override capability.

**Context of Application:** Step 2 (value collection) when placeholders lack explicit user-provided values.

**Key Characteristics:**

- Multi-source evidence gathering: user input > existing constitution > README > package metadata > git history
- Inference logic explicitly documented for user verification
- Ambiguous inferences surfaced as proposals requiring confirmation
- TODO markers used for genuinely unknowable values rather than guessing

**Operational Guidance:**

1. For each placeholder token identified in step 1, attempt value derivation in priority order:
   - **User input (highest priority):** Parse conversation history and current `$ARGUMENTS` for explicit values (e.g., "set PROJECT_NAME to SpecKit")
   - **Existing constitution:** If updating rather than creating, check if placeholder already filled in current version (preserve unless user explicitly overriding)
   - **README.md:** Extract project name from first heading, description from first paragraph, key principles from "Philosophy" or "Design Principles" sections
   - **Package metadata:** Read `package.json`, `pyproject.toml`, `Cargo.toml` for project name, description, version
   - **Git history:** Use `git log --reverse .specify/memory/constitution.md` to find RATIFICATION_DATE from first commit touching constitution
2. For governance dates, apply special logic:
   - `RATIFICATION_DATE`: If existing version has date, preserve it (never changes); if new constitution, use today's date or prompt user "When was this governance model originally adopted? Reply with YYYY-MM-DD or 'today'."
   - `LAST_AMENDED_DATE`: Always set to today's date (YYYY-MM-DD format) when any semantic change occurs; preserve previous date for PATCH-only typo fixes if desired
3. For `CONSTITUTION_VERSION`, apply Semantic Version-Driven Change Classification pattern (see above)
4. For principle-specific placeholders (e.g., `[PRINCIPLE_1_NAME]`, `[PRINCIPLE_1_BODY]`):
   - If user specifies count ("I want 3 principles"), adjust template to match count
   - If user provides principle content, use verbatim
   - If inferring from README/docs, extract declarative statements and reformat as principle structure
5. When inference uncertain, format as proposal: "Based on README heading, inferring PROJECT_NAME='SpecKit Framework'. Confirm or provide alternative?"
6. When value genuinely unknowable (e.g., future compliance review date not yet scheduled), insert: `TODO(FIELD_NAME): explanation` and document in Sync Impact Report deferred items
7. Never fabricate values; prefer TODO over guessing

### Pattern: Declarative Principle Encoding

**Objective:** Transform vague aspirational statements into testable, enforceable governance rules that enable automated compliance validation.

**Context of Application:** Step 3 (drafting constitution content) when converting user input or inferred principles into formal principle sections.

**Key Characteristics:**

- Principles use imperative mood (MUST, MUST NOT, SHOULD, SHOULD NOT, MAY) per RFC 2119
- Each principle includes observable success criteria or violation conditions
- Rationale explicitly stated when not immediately obvious
- Vague qualifiers ("robust," "intuitive," "reasonable") eliminated or quantified

**Operational Guidance:**

1. For each principle, structure content as: **Name** (single line, noun phrase), **Rules** (declarative statements), **Rationale** (optional, when non-obvious)
2. Convert aspirational language to imperative:
   - "We value security" → "All authentication tokens MUST expire within 24 hours."
   - "Code should be tested" → "Every public API MUST have corresponding unit test coverage ≥80%."
   - "Documentation is important" → "User-facing features MUST include usage examples in docs/ before merge."
3. Replace vague qualifiers with measurable criteria:
   - "Reasonable performance" → "API response time MUST NOT exceed 200ms at p95 for read operations under normal load."
   - "Intuitive UX" → "New users MUST complete primary workflow without documentation within 5 minutes (validated via usability testing)."
   - "Robust error handling" → "All external service calls MUST implement circuit breaker pattern with 3-failure threshold and 30s cooldown."
4. For each MUST/SHOULD statement, ensure it's verifiable: "Can this be tested in CI/code review/audit?" If no, refine until testable.
5. Add rationale when principle might seem arbitrary or overly strict: "Principle: All database queries MUST use parameterized statements. Rationale: Prevents SQL injection; non-parameterized queries rejected in code review."
6. Format as paragraph or concise bullets depending on complexity (single constraint = paragraph; multiple related constraints = bullet list)
7. Review final principle set for completeness: Do principles cover functional correctness, security, performance, observability, maintainability, and process governance? Fill gaps if constitutional scope demands it.

### Pattern: Atomic Multi-File Transaction

**Objective:** Ensure constitution and dependent templates remain synchronized even in failure scenarios by treating related updates as a single atomic operation.

**Context of Application:** Step 7 (write operation) when constitution changes require propagation to multiple template files.

**Key Characteristics:**

- All affected files updated in single operation or none updated at all
- Temporary files used for staging before atomic rename
- Write validation performed before finalizing
- Rollback capability on partial failure

**Operational Guidance:**

1. Before writing any files, prepare complete content for all artifacts requiring updates: constitution + affected templates
2. For each file to update, write content to temporary file in same directory: `.specify/memory/constitution.md.tmp`, `.specify/templates/plan-template.md.tmp`, etc.
3. Validate each temporary file: parse as valid Markdown, verify no unresolved placeholders (except documented TODOs), check file size reasonable (not empty, not unexpectedly large)
4. If all validations pass, perform atomic rename sequence: `mv constitution.md.tmp constitution.md && mv plan-template.md.tmp plan-template.md ...` (platform-appropriate atomic file replacement)
5. If any rename fails, immediately rollback all completed renames: restore original files from `.bak` copies created before operation
6. After successful atomic update, delete temporary files and backup copies
7. Log operation to `.specify/logs/constitution-updates.log` (append-only): `2024-02-06T14:32:15Z | v2.1.0 → v3.0.0 | Files: 4 | Status: SUCCESS`
8. If operation fails, preserve temporary files for debugging and log failure details
9. In user-facing output, report transaction status clearly: "✅ Constitution and 3 templates updated atomically" or "❌ Update failed, no files modified (see constitution.md.tmp for staged changes)"

## Anti-Patterns: Common Mistakes in Constitution Management

### Anti-Pattern: Vague Aspirational Principles

**Description:** Writing constitutional principles as broad, inspirational statements without concrete enforcement mechanisms or measurable success criteria (e.g., "We believe in code quality," "Security is a priority," "Performance matters").

**Reasons to Avoid:**

- Principles become unenforceable guidance rather than governance rules—no objective way to determine compliance or violation
- Different team members interpret vague principles inconsistently, leading to conflicting implementation decisions
- Automated validation impossible when criteria are subjective or unmeasurable
- Code reviews devolve into opinion debates rather than constitutional compliance checks

**Negative Consequences:**

- Constitution perceived as ceremonial documentation with no practical impact on development process
- Governance becomes dependent on individual judgment rather than shared standards, creating bottlenecks and inconsistency
- New team members cannot learn project norms from constitution; must rely on tribal knowledge
- Compliance audits impossible; no way to prove adherence to stated principles
- Technical debt accumulates in areas with vague principles because violations go undetected

**Correct Alternative:** Use the **Declarative Principle Encoding** pattern to transform aspirations into testable rules with imperative mood (MUST/SHOULD), quantified thresholds, and explicit success/failure criteria.

### Anti-Pattern: Version Increment Without Rationale

**Description:** Bumping the `CONSTITUTION_VERSION` number arbitrarily or following unclear logic (e.g., always incrementing PATCH, using date-based versions, or inconsistent MAJOR/MINOR decisions) without documenting the classification reasoning.

**Reasons to Avoid:**

- Consumers cannot assess breaking change risk by examining version number alone
- No clear signal for when dependent templates require urgent review vs. optional updates
- Version history becomes meaningless noise rather than meaningful change tracking
- Inconsistent versioning erodes trust in governance change management process

**Negative Consequences:**

- Teams miss critical breaking changes because version bump didn't signal MAJOR change appropriately
- Unnecessary review overhead when MINOR changes incorrectly marked as MAJOR, causing alert fatigue
- Inability to rollback to "last known good" constitution version because version semantics unreliable
- Downstream tools cannot automate compatibility checks or dependency updates
- Governance evolution narrative lost; cannot reconstruct decision timeline from version history

**Correct Alternative:** Apply the **Semantic Version-Driven Change Classification** pattern with explicit MAJOR/MINOR/PATCH criteria and document bump rationale in Sync Impact Report header comment.

### Anti-Pattern: Silent Template Desynchronization

**Description:** Updating the constitution without checking or updating dependent template files (plan, spec, tasks templates, commands, documentation), allowing constitutional requirements and template structures to drift apart over time.

**Reasons to Avoid:**

- Templates generate artifacts that violate constitutional principles, creating compliance gaps from the start
- Manual reconciliation burden placed on users who must cross-reference constitution and templates independently
- Confusion when template guidance contradicts constitutional mandates
- Cascading errors as outdated templates produce non-compliant specs, which produce non-compliant plans, which produce non-compliant code

**Negative Consequences:**

- Feature specifications created from outdated spec template lack mandatory sections introduced by new constitutional principles (e.g., new observability principle but spec template has no "Observability" section)
- Task templates omit principle-driven task categories, causing those concerns to be overlooked during implementation
- Command files reference removed or renamed principles, causing runtime errors or confusing guidance
- Documentation examples demonstrate patterns that violate current constitutional constraints
- Teams lose confidence in template system, abandoning it in favor of ad-hoc approaches

**Correct Alternative:** Use the **Transitive Dependency Synchronization** pattern to systematically validate and update all dependent artifacts when constitution changes, tracking sync status in Sync Impact Report.

### Anti-Pattern: Hardcoded Value Fabrication

**Description:** When constitution placeholders lack explicit user-provided values, inventing plausible-sounding but inaccurate values rather than inferring from repository context or marking as TODO (e.g., guessing "MyProject" as project name, fabricating ratification dates, inventing principle content).

**Reasons to Avoid:**

- Fabricated values embed inaccuracies into governance foundation, which then propagate to all dependent artifacts
- User cannot easily detect fabrications, leading to silent acceptance of incorrect information
- No transparency into inference logic; user cannot validate or correct agent reasoning
- Fabricated historical dates (ratification, last amended) corrupt governance timeline and audit trail

**Negative Consequences:**

- Project identity misrepresented in official governance document (wrong name, description, principles)
- Governance history falsified through fabricated dates, undermining compliance audits and legal review
- Principles that don't reflect actual project values or constraints encoded as canonical governance
- Time wasted correcting fabrications when discovered, often after they've propagated to multiple dependent files
- Trust erosion: if constitution contains obvious fabrications, credibility of entire governance system questioned

**Correct Alternative:** Apply the **Context-Driven Value Inference** pattern with explicit priority hierarchy (user input > existing constitution > README > package metadata > git history) and use TODO markers for unknowable values rather than guessing.

### Anti-Pattern: Lossy Partial Updates

**Description:** When user provides partial updates (e.g., "change Principle 2 name to 'Data Privacy'"), processing only the explicitly mentioned change without validating consistency of unmentioned sections, version increments, or dependent artifacts, then writing the partially updated constitution.

**Reasons to Avoid:**

- Version number may remain unchanged despite semantic modifications, breaking version semantics
- Last amended date not updated, hiding recent governance changes
- Modified principle may create inconsistencies with other principles or governance sections not reviewed
- Dependent templates not synchronized with the partial change, creating localized desynchronization

**Negative Consequences:**

- Constitution version number claims "v2.0.0" but content reflects undocumented v2.1.0 changes, causing confusion
- Governance timeline shows last amendment 6 months ago despite changes made today
- Principle 2 updated content contradicts Principle 4 constraint, but Principle 4 not reviewed for consistency
- Spec template still references old Principle 2 name while constitution uses new name, breaking references
- Incremental corruption: multiple partial updates compound inconsistencies until constitution requires complete rebuild

**Correct Alternative:** Treat every constitution update, even partial, as full validation cycle: analyze all changes for version bump classification, update governance dates, validate principle consistency, and propagate changes to dependent artifacts per **Transitive Dependency Synchronization** pattern.

### Anti-Pattern: Non-Atomic Multi-File Updates

**Description:** Writing constitution and dependent template files sequentially without transaction semantics, allowing partial success where some files update successfully while others fail, leaving the artifact set in inconsistent state.

**Reasons to Avoid:**

- Process interruption (crash, timeout, disk full) leaves half-updated artifact set in undefined state
- Constitution references principles that spec template hasn't incorporated yet (or vice versa)
- Debugging partial failures difficult because unclear which files successfully updated
- No rollback mechanism; manual recovery requires determining which files need reverting

**Negative Consequences:**

- Constitution v3.0.0 written successfully but spec template update failed; template still generates v2.0.0-compliant specs
- Plan template updated with new principle validation but constitution write failed; template validates against non-existent principle
- Corruption discovered hours later when user attempts to generate spec, by which time determining correct state requires forensic analysis
- Recovery requires manually examining each file's content, comparing against intended updates, and selectively reverting/reapplying changes
- User loses work if they cannot determine which partial state is recoverable vs. requires full re-run

**Correct Alternative:** Use the **Atomic Multi-File Transaction** pattern with temporary file staging, validation before finalization, and rollback capability on any individual file failure.

Formatting & Style Requirements:

- Use Markdown headings exactly as in the template (do not demote/promote levels).
- Wrap long rationale lines to keep readability (<100 chars ideally) but do not hard enforce with awkward breaks.
- Keep a single blank line between sections.
- Avoid trailing whitespace.

If the user supplies partial updates (e.g., only one principle revision), still perform validation and version decision steps.

If critical info missing (e.g., ratification date truly unknown), insert `TODO(<FIELD_NAME>): explanation` and include in the Sync Impact Report under deferred items.

Do not create a new template; always operate on the existing `.specify/memory/constitution.md` file.
