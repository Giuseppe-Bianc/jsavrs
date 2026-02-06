---
description: Perform a non-destructive cross-artifact consistency and quality analysis across spec.md, plan.md, and tasks.md after task generation.
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Goal

Identify inconsistencies, duplications, ambiguities, and underspecified items across the three core artifacts (`spec.md`, `plan.md`, `tasks.md`) before implementation. This command MUST run only after `/speckit.tasks` has successfully produced a complete `tasks.md`.

## Operating Constraints

**STRICTLY READ-ONLY**: Do **not** modify any files. Output a structured analysis report. Offer an optional remediation plan (user must explicitly approve before any follow-up editing commands would be invoked manually).

**Constitution Authority**: The project constitution (`.specify/memory/constitution.md`) is **non-negotiable** within this analysis scope. Constitution conflicts are automatically CRITICAL and require adjustment of the spec, plan, or tasks—not dilution, reinterpretation, or silent ignoring of the principle. If a principle itself needs to change, that must occur in a separate, explicit constitution update outside `/speckit.analyze`.

## Execution Steps

### 1. Initialize Analysis Context

Run `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/check-prerequisites.ps1 -Json -RequireTasks -IncludeTasks` once from repo root and parse JSON for FEATURE_DIR and AVAILABLE_DOCS. Derive absolute paths:

- SPEC = FEATURE_DIR/spec.md
- PLAN = FEATURE_DIR/plan.md
- TASKS = FEATURE_DIR/tasks.md

Abort with an error message if any required file is missing (instruct the user to run missing prerequisite command).
For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").

### 2. Load Artifacts (Progressive Disclosure)

Load only the minimal necessary context from each artifact:

**From spec.md:**

- Overview/Context
- Functional Requirements
- Non-Functional Requirements
- User Stories
- Edge Cases (if present)

**From plan.md:**

- Architecture/stack choices
- Data Model references
- Phases
- Technical constraints

**From tasks.md:**

- Task IDs
- Descriptions
- Phase grouping
- Parallel markers [P]
- Referenced file paths

**From constitution:**

- Load `.specify/memory/constitution.md` for principle validation

### 3. Build Semantic Models

Create internal representations (do not include raw artifacts in output):

- **Requirements inventory**: Each functional + non-functional requirement with a stable key (derive slug based on imperative phrase; e.g., "User can upload file" → `user-can-upload-file`)
- **User story/action inventory**: Discrete user actions with acceptance criteria
- **Task coverage mapping**: Map each task to one or more requirements or stories (inference by keyword / explicit reference patterns like IDs or key phrases)
- **Constitution rule set**: Extract principle names and MUST/SHOULD normative statements

### 4. Detection Passes (Token-Efficient Analysis)

Focus on high-signal findings. Limit to 50 findings total; aggregate remainder in overflow summary.

#### A. Duplication Detection

- Identify near-duplicate requirements
- Mark lower-quality phrasing for consolidation

#### B. Ambiguity Detection

- Flag vague adjectives lacking measurable criteria, including but not limited to:
    - **Performance**: fast, scalable, efficient, performant, responsive, optimized, lightweight, low-latency, real-time
    - **Reliability**: robust, reliable, stable, resilient, fault-tolerant, highly-available
    - **Security**: secure, safe, protected, hardened
    - **Usability**: intuitive, user-friendly, easy-to-use, simple, clean, seamless, polished
    - **Maintainability**: maintainable, extensible, flexible, modular, well-structured
    - **Quality**: high-quality, production-ready, enterprise-grade, world-class, best-in-class
    - **Comparatives without baseline**: better, faster, improved, enhanced, superior
    - Require numeric thresholds, SLOs, or testable acceptance criteria for each flagged term

- Flag unresolved placeholders (case-insensitive detection): BUG, FIXME, HACK, NOTE, OPTIMIZE, TODO, TBD, TKTK, WIP, XXX, ???, `<placeholder>`, etc.

#### C. Underspecification

- Requirements with verbs but missing object or measurable outcome
- User stories missing acceptance criteria alignment
- Tasks referencing files or components not defined in spec/plan

#### D. Constitution Alignment

- Any requirement or plan element conflicting with a MUST principle
- Missing mandated sections or quality gates from constitution

#### E. Coverage Gaps

- Requirements with zero associated tasks
- Tasks with no mapped requirement/story
- Non-functional requirements not reflected in tasks (e.g., performance, security)

#### F. Inconsistency

- Terminology drift (same concept named differently across files)
- Data entities referenced in plan but absent in spec (or vice versa)
- Task ordering contradictions (e.g., integration tasks before foundational setup tasks without dependency note)
- Conflicting requirements (e.g., one requires Next.js while other specifies Vue)

### 5. Severity Assignment

Use this heuristic to prioritize findings:

- **CRITICAL**: Violates constitution MUST, missing core spec artifact, or requirement with zero coverage that blocks baseline functionality
- **HIGH**: Duplicate or conflicting requirement, ambiguous security/performance attribute, untestable acceptance criterion
- **MEDIUM**: Terminology drift, missing non-functional task coverage, underspecified edge case
- **LOW**: Style/wording improvements, minor redundancy not affecting execution order

### 6. Produce Compact Analysis Report

Output a Markdown report (no file writes) with the following structure:

## Specification Analysis Report

| ID |   Category  | Severity |    Location(s)   |            Summary           |            Recommendation            |
|----|-------------|----------|------------------|------------------------------|--------------------------------------|
| A1 | Duplication |   HIGH   | spec.md:L120-134 | Two similar requirements ... | Merge phrasing; keep clearer version |

(Add one row per finding; generate stable IDs prefixed by category initial.)

**Coverage Summary Table:**

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|

**Constitution Alignment Issues:** (if any)

**Unmapped Tasks:** (if any)

**Metrics:**

- Total Requirements
- Total Tasks
- Coverage % (requirements with ≥1 task / total requirements)
    - Formula: `(count of requirements with at least one mapped task) / (count of all requirements)` × 100
    - Includes both functional and non-functional requirements in denominator
    - Format: Percentage rounded to 1 decimal place (e.g., `75.0%`)
    - Edge case: If total requirements = 0, display `N/A` and flag as CRITICAL issue
    - Must match count from Coverage Summary Table where "Has Task?" = YES
- Ambiguity Count
- Duplication Count
- Critical Issues Count

### 7. Provide Next Actions

At end of report, output a concise Next Actions block:

- If CRITICAL issues exist: Recommend resolving before `/speckit.implement`
- If only LOW/MEDIUM: User may proceed, but provide improvement suggestions
- Provide explicit command suggestions: e.g., "Run /speckit.specify with refinement", "Run /speckit.plan to adjust architecture", "Manually edit tasks.md to add coverage for 'performance-metrics'"

### 8. Offer Remediation

Ask the user: "Would you like me to suggest concrete remediation edits for the top N issues?" (Do NOT apply them automatically.)

## Operating Principles

### Context Efficiency

- **Minimal high-signal tokens**: Focus on actionable findings, not exhaustive documentation
- **Progressive disclosure**: Load artifacts incrementally; don't dump all content into analysis
- **Token-efficient output**: Limit findings table to 50 rows; summarize overflow
- **Deterministic results**: Rerunning without changes should produce consistent IDs and counts

### Analysis Guidelines

- **NEVER modify files** (this is read-only analysis)
- **NEVER hallucinate missing sections** (if absent, report them accurately)
- **Prioritize constitution violations** (these are always CRITICAL)
- **Use examples over exhaustive rules** (cite specific instances, not generic patterns)
- **Report zero issues gracefully** (emit success report with coverage statistics)

## Patterns: Best Practices for Specification Analysis

### Pattern 1: Progressive Artifact Loading

**Objective**: Minimize token consumption while maximizing analysis signal quality.

**Context of Application**: When analyzing multiple specification artifacts that may contain thousands of lines of documentation.

**Key Characteristics**:

- Extracts only semantically relevant sections from each artifact
- Builds internal semantic models rather than reprocessing raw text
- Defers full artifact loading until specific findings require deep inspection

**Operational Guidance**:

1. Parse prerequisite check output first to obtain absolute file paths
2. Load artifacts in dependency order: constitution → spec → plan → tasks
3. Extract only named sections (e.g., "Functional Requirements", "Architecture")
4. Skip boilerplate, examples, and commentary unless directly relevant to validation
5. Store extracted content in structured objects (requirement inventory, task mapping) rather than preserving original formatting

### Pattern 2: Stable Identifier Generation

**Objective**: Produce deterministic, reproducible analysis results across multiple runs.

**Context of Application**: When tracking findings across multiple analysis iterations or maintaining audit trails.

**Key Characteristics**:

- Finding IDs use category-based prefixes (A for Ambiguity, D for Duplication, etc.)
- Requirement keys derived from imperative phrases using consistent slug generation
- Sequential numbering within each category maintains insertion order

**Operational Guidance**:

1. Establish category prefix mapping at analysis initialization (A=Ambiguity, D=Duplication, U=Underspecification, etc.)
2. Generate requirement slugs by: extracting imperative phrase → lowercasing → replacing spaces with hyphens → removing special characters
3. Number findings sequentially within category in order of detection
4. Persist identifier generation rules across analysis runs to ensure reproducibility

### Pattern 3: Constitution-First Validation

**Objective**: Treat constitutional principles as non-negotiable constraints that supersede all other quality criteria.

**Context of Application**: When project governance requires adherence to established principles that must not be diluted or reinterpreted.

**Key Characteristics**:

- Constitution violations automatically receive CRITICAL severity
- Analysis framework validates spec/plan/tasks against constitution, never the reverse
- Clear separation between "constitution needs updating" (out of scope) and "artifacts violate constitution" (in scope)

**Operational Guidance**:

1. Load constitution first and parse all MUST/SHOULD normative statements
2. Build validation rule set from extracted principles before loading other artifacts
3. Flag any requirement, plan element, or task that contradicts a MUST principle as CRITICAL
4. Include explicit recommendation to update artifacts, not to "reconsider" the principle
5. If user questions a principle itself, recommend explicit constitution update process outside analysis scope

### Pattern 4: Coverage-Centric Metrics

**Objective**: Quantify requirement-to-task traceability as the primary health indicator.

**Context of Application**: When assessing specification completeness before implementation begins.

**Key Characteristics**:

- Coverage percentage based on requirements with at least one mapped task
- Includes both functional and non-functional requirements in denominator
- Distinguishes between "unmapped requirements" (coverage gap) and "unmapped tasks" (scope creep)

**Operational Guidance**:

1. Build bidirectional mapping: requirements → tasks and tasks → requirements
2. Calculate coverage as: (count of requirements with ≥1 task) / (total requirements) × 100
3. Report unmapped requirements separately from unmapped tasks
4. Include non-functional requirements in coverage calculation (security, performance, etc.)
5. Flag coverage below 80% as requiring attention before implementation

### Pattern 5: Bounded Finding Reports

**Objective**: Prevent analysis output from overwhelming users with excessive detail.

**Context of Application**: When artifact quality issues exceed reasonable remediation capacity in a single iteration.

**Key Characteristics**:

- Hard limit of 50 findings in primary report table
- Overflow findings aggregated into summary counts by category/severity
- Focus on highest-severity, highest-impact findings first

**Operational Guidance**:

1. Collect all findings during detection passes without filtering
2. Sort by severity (CRITICAL → HIGH → MEDIUM → LOW), then by category
3. Take top 50 findings for detailed reporting in main table
4. Count remaining findings by category and severity; append overflow summary
5. Include statement: "N additional findings suppressed; prioritize top issues first"

## Anti-Patterns: Common Mistakes to Avoid

### Anti-Pattern 1: Exhaustive Artifact Dumping

**Description**: Loading entire specification artifacts into analysis context and iterating over every line, paragraph, or section regardless of relevance.

**Reasons to Avoid**:

- Consumes excessive tokens on boilerplate, examples, and non-normative content
- Degrades analysis focus by diluting signal with noise
- May exceed context window limits on large specifications

**Negative Consequences**:

- Slower analysis execution
- Higher computational cost
- Risk of incomplete analysis if context limits are hit mid-process
- Difficulty identifying high-priority issues amid verbose output

**Correct Alternative**: Use Pattern 1 (Progressive Artifact Loading) to extract only semantically relevant sections and build compact semantic models.

### Anti-Pattern 2: Non-Deterministic Finding IDs

**Description**: Generating finding identifiers using timestamps, random UUIDs, or hash-based schemes that change between analysis runs even when artifacts remain unchanged.

**Reasons to Avoid**:

- Makes it impossible to track whether specific findings have been resolved across iterations
- Prevents automated regression detection (e.g., "Finding X reappeared after remediation")
- Complicates audit trails and change tracking

**Negative Consequences**:

- Users cannot reliably reference specific findings in discussions or issue tracking
- Automated tooling cannot correlate findings across runs
- Increases cognitive load when comparing analysis reports

**Correct Alternative**: Use Pattern 2 (Stable Identifier Generation) with category prefixes and sequential numbering based on detection order.

### Anti-Pattern 3: Constitution Relativism

**Description**: Treating constitutional principles as negotiable guidelines, suggesting spec/plan modifications "could be aligned" with constitution, or recommending "reconsidering" a principle when conflicts arise.

**Reasons to Avoid**:

- Undermines governance framework and project consistency
- Introduces ambiguity into what should be clear constraints
- Encourages gradual erosion of established standards

**Negative Consequences**:

- Constitution becomes meaningless as "guidelines" rather than binding rules
- Different team members interpret principles differently
- Quality and consistency degrade over time as violations are rationalized
- Loss of trust in specification process

**Correct Alternative**: Use Pattern 3 (Constitution-First Validation) to treat violations as CRITICAL errors requiring artifact correction, never principle reinterpretation.

### Anti-Pattern 4: Task-Centric Coverage Analysis

**Description**: Calculating coverage as (count of tasks with requirements) / (total tasks) instead of (count of requirements with tasks) / (total requirements).

**Reasons to Avoid**:

- Inverts the purpose of coverage metrics (validating spec completeness, not task justification)
- Allows 100% coverage even when critical requirements have no implementation tasks
- Obscures gaps in requirement fulfillment

**Negative Consequences**:

- False confidence that specification is implementation-ready
- Critical functionality may be omitted from task breakdown
- Wastes implementation effort on out-of-scope tasks while missing required features
- Difficult to trace deliverables back to user needs

**Correct Alternative**: Use Pattern 4 (Coverage-Centric Metrics) to measure requirements-to-tasks mapping, flagging unmapped requirements as coverage gaps.

### Anti-Pattern 5: Unbounded Finding Enumeration

**Description**: Reporting every detected issue regardless of quantity, producing reports with hundreds or thousands of findings.

**Reasons to Avoid**:

- Overwhelms users with information, making it unclear where to start
- Obscures critical issues among minor style suggestions
- Creates "analysis paralysis" where remediation seems insurmountable

**Negative Consequences**:

- Users ignore the report entirely due to overwhelming volume
- Critical issues buried among low-priority findings
- Remediation effort spread thinly across many minor issues instead of focusing on blockers
- Decreased trust in analysis tool's judgment and prioritization

**Correct Alternative**: Use Pattern 5 (Bounded Finding Reports) to limit detailed findings to top 50 issues by severity, summarizing overflow.

### Anti-Pattern 6: Automated File Modification

**Description**: Automatically applying remediation edits to spec.md, plan.md, or tasks.md based on detected findings without explicit user approval.

**Reasons to Avoid**:

- Violates read-only contract of analysis command
- May introduce errors or unintended changes
- Removes user from decision-making loop on critical specification changes

**Negative Consequences**:

- User loses visibility into what changed and why
- Incorrect automated fixes may require manual rollback
- Destroys audit trail of specification evolution
- May overwrite user intent with algorithmic interpretation

**Correct Alternative**: Follow operating constraint to **NEVER modify files** during analysis; offer remediation suggestions only, requiring explicit user approval before any edits.

### Anti-Pattern 7: Ambiguity Tolerance

**Description**: Accepting vague qualitative terms (e.g., "fast", "secure", "user-friendly") in requirements without flagging them for measurable criteria.

**Reasons to Avoid**:

- Prevents objective validation of requirement fulfillment
- Different stakeholders interpret subjective terms differently
- Impossible to write meaningful tests or acceptance criteria

**Negative Consequences**:

- Implementation disputes about whether requirements are met
- Scope creep as "secure enough" keeps shifting
- Quality gate failures when stakeholders realize expectations were misaligned
- Rework to establish proper success criteria post-implementation

**Correct Alternative**: Flag all vague adjectives per Detection Pass B (Ambiguity Detection) and require numeric thresholds, SLOs, or testable acceptance criteria.

### Anti-Pattern 8: Missing Artifact Hallucination

**Description**: When a required artifact (spec.md, plan.md, tasks.md) is absent, generating placeholder analysis or assuming default content instead of reporting the missing file accurately.

**Reasons to Avoid**:

- Provides false confidence that analysis is complete
- Hides prerequisite step failures from user
- May base subsequent analysis on incorrect assumptions

**Negative Consequences**:

- User proceeds with incomplete specification believing analysis was successful
- Downstream commands fail due to missing prerequisites
- Wasted effort debugging problems that should have been caught early
- Loss of trust in tool's error reporting

**Correct Alternative**: Follow analysis guideline to **NEVER hallucinate missing sections**; abort with clear error message instructing user to run prerequisite commands.

## Context

{{args}}
