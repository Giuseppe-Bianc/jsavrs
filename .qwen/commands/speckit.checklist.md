---
description: Generate a custom checklist for the current feature based on user requirements.
---

## Checklist Purpose: "Unit Tests for English"

**CRITICAL CONCEPT**: Checklists are **UNIT TESTS FOR REQUIREMENTS WRITING** - they validate the quality, clarity, and completeness of requirements in a given domain.

**NOT for verification/testing**:

- ❌ NOT "Verify the button clicks correctly"
- ❌ NOT "Test error handling works"
- ❌ NOT "Confirm the API returns 200"
- ❌ NOT checking if code/implementation matches the spec

**FOR requirements quality validation**:

- ✅ "Are visual hierarchy requirements defined for all card types?" (completeness)
- ✅ "Is 'prominent display' quantified with specific sizing/positioning?" (clarity)
- ✅ "Are hover state requirements consistent across all interactive elements?" (consistency)
- ✅ "Are accessibility requirements defined for keyboard navigation?" (coverage)
- ✅ "Does the spec define what happens when logo image fails to load?" (edge cases)

**Metaphor**: If your spec is code written in English, the checklist is its unit test suite. You're testing whether the requirements are well-written, complete, unambiguous, and ready for implementation - NOT whether the implementation works.

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Execution Steps

1. **Setup**: Run `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/check-prerequisites.ps1 -Json` from repo root and parse JSON for FEATURE_DIR and AVAILABLE_DOCS list.
   - All file paths must be absolute.
   - For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").

2. **Clarify intent (dynamic)**: Derive up to THREE initial contextual clarifying questions (no pre-baked catalog). They MUST:
   - Be generated from the user's phrasing + extracted signals from spec/plan/tasks
   - Only ask about information that materially changes checklist content
   - Be skipped individually if already unambiguous in `$ARGUMENTS`
   - Prefer precision over breadth

   Generation algorithm:
   1. Extract signals: feature domain keywords (e.g., auth, latency, UX, API), risk indicators ("critical", "must", "compliance"), stakeholder hints ("QA", "review", "security team"), and explicit deliverables ("a11y", "rollback", "contracts").
   2. Cluster signals into candidate focus areas (max 4) ranked by relevance.
   3. Identify probable audience & timing (author, reviewer, QA, release) if not explicit.
   4. Detect missing dimensions: scope breadth, depth/rigor, risk emphasis, exclusion boundaries, measurable acceptance criteria.
   5. Formulate questions chosen from these archetypes:
      - Scope refinement (e.g., "Should this include integration touchpoints with X and Y or stay limited to local module correctness?")
      - Risk prioritization (e.g., "Which of these potential risk areas should receive mandatory gating checks?")
      - Depth calibration (e.g., "Is this a lightweight pre-commit sanity list or a formal release gate?")
      - Audience framing (e.g., "Will this be used by the author only or peers during PR review?")
      - Boundary exclusion (e.g., "Should we explicitly exclude performance tuning items this round?")
      - Scenario class gap (e.g., "No recovery flows detected—are rollback / partial failure paths in scope?")

   Question formatting rules:
   - If presenting options, generate a compact table with columns: Option | Candidate | Why It Matters
   - Limit to A–E options maximum; omit table if a free-form answer is clearer
   - Never ask the user to restate what they already said
   - Avoid speculative categories (no hallucination). If uncertain, ask explicitly: "Confirm whether X belongs in scope."

   Defaults when interaction impossible:
   - Depth: Standard
   - Audience: Reviewer (PR) if code-related; Author otherwise
   - Focus: Top 2 relevance clusters

   Output the questions (label Q1/Q2/Q3). After answers: if ≥2 scenario classes (Alternate / Exception / Recovery / Non-Functional domain) remain unclear, you MAY ask up to TWO more targeted follow‑ups (Q4/Q5) with a one-line justification each (e.g., "Unresolved recovery path risk"). Do not exceed five total questions. Skip escalation if user explicitly declines more.

3. **Understand user request**: Combine `$ARGUMENTS` + clarifying answers:
   - Derive checklist theme (e.g., security, review, deploy, ux)
   - Consolidate explicit must-have items mentioned by user
   - Map focus selections to category scaffolding
   - Infer any missing context from spec/plan/tasks (do NOT hallucinate)

4. **Load feature context**: Read from FEATURE_DIR:
   - spec.md: Feature requirements and scope
   - plan.md (if exists): Technical details, dependencies
   - tasks.md (if exists): Implementation tasks

   **Context Loading Strategy**:
   - Load only necessary portions relevant to active focus areas (avoid full-file dumping)
   - Prefer summarizing long sections into concise scenario/requirement bullets
   - Use progressive disclosure: add follow-on retrieval only if gaps detected
   - If source docs are large, generate interim summary items instead of embedding raw text

5. **Generate checklist** - Create "Unit Tests for Requirements":
   - Create `FEATURE_DIR/checklists/` directory if it doesn't exist
   - Generate unique checklist filename:
     - Use short, descriptive name based on domain (e.g., `ux.md`, `api.md`, `security.md`)
     - Format: `[domain].md`
     - If file exists, append to existing file
   - Number items sequentially starting from CHK001
   - Each `/speckit.checklist` run creates a NEW file (never overwrites existing checklists)

   **CORE PRINCIPLE - Test the Requirements, Not the Implementation**:
   Every checklist item MUST evaluate the REQUIREMENTS THEMSELVES for:
   - **Completeness**: Are all necessary requirements present?
   - **Clarity**: Are requirements unambiguous and specific?
   - **Consistency**: Do requirements align with each other?
   - **Measurability**: Can requirements be objectively verified?
   - **Coverage**: Are all scenarios/edge cases addressed?

   **Category Structure** - Group items by requirement quality dimensions:
   - **Requirement Completeness** (Are all necessary requirements documented?)
   - **Requirement Clarity** (Are requirements specific and unambiguous?)
   - **Requirement Consistency** (Do requirements align without conflicts?)
   - **Acceptance Criteria Quality** (Are success criteria measurable?)
   - **Scenario Coverage** (Are all flows/cases addressed?)
   - **Edge Case Coverage** (Are boundary conditions defined?)
   - **Non-Functional Requirements** (Performance, Security, Accessibility, etc. - are they specified?)
   - **Dependencies & Assumptions** (Are they documented and validated?)
   - **Ambiguities & Conflicts** (What needs clarification?)

   **HOW TO WRITE CHECKLIST ITEMS - "Unit Tests for English"**:

   ❌ **WRONG** (Testing implementation):
   - "Verify landing page displays 3 episode cards"
   - "Test hover states work on desktop"
   - "Confirm logo click navigates home"

   ✅ **CORRECT** (Testing requirements quality):
   - "Are the exact number and layout of featured episodes specified?" [Completeness]
   - "Is 'prominent display' quantified with specific sizing/positioning?" [Clarity]
   - "Are hover state requirements consistent across all interactive elements?" [Consistency]
   - "Are keyboard navigation requirements defined for all interactive UI?" [Coverage]
   - "Is the fallback behavior specified when logo image fails to load?" [Edge Cases]
   - "Are loading states defined for asynchronous episode data?" [Completeness]
   - "Does the spec define visual hierarchy for competing UI elements?" [Clarity]

   **ITEM STRUCTURE**:
   Each item should follow this pattern:
   - Question format asking about requirement quality
   - Focus on what's WRITTEN (or not written) in the spec/plan
   - Include quality dimension in brackets [Completeness/Clarity/Consistency/etc.]
   - Reference spec section `[Spec §X.Y]` when checking existing requirements
   - Use `[Gap]` marker when checking for missing requirements

   **EXAMPLES BY QUALITY DIMENSION**:

   Completeness:
   - "Are error handling requirements defined for all API failure modes? [Gap]"
   - "Are accessibility requirements specified for all interactive elements? [Completeness]"
   - "Are mobile breakpoint requirements defined for responsive layouts? [Gap]"

   Clarity:
   - "Is 'fast loading' quantified with specific timing thresholds? [Clarity, Spec §NFR-2]"
   - "Are 'related episodes' selection criteria explicitly defined? [Clarity, Spec §FR-5]"
   - "Is 'prominent' defined with measurable visual properties? [Ambiguity, Spec §FR-4]"

   Consistency:
   - "Do navigation requirements align across all pages? [Consistency, Spec §FR-10]"
   - "Are card component requirements consistent between landing and detail pages? [Consistency]"

   Coverage:
   - "Are requirements defined for zero-state scenarios (no episodes)? [Coverage, Edge Case]"
   - "Are concurrent user interaction scenarios addressed? [Coverage, Gap]"
   - "Are requirements specified for partial data loading failures? [Coverage, Exception Flow]"

   Measurability:
   - "Are visual hierarchy requirements measurable/testable? [Acceptance Criteria, Spec §FR-1]"
   - "Can 'balanced visual weight' be objectively verified? [Measurability, Spec §FR-2]"

   **Scenario Classification & Coverage** (Requirements Quality Focus):
   - Check if requirements exist for: Primary, Alternate, Exception/Error, Recovery, Non-Functional scenarios
   - For each scenario class, ask: "Are [scenario type] requirements complete, clear, and consistent?"
   - If scenario class missing: "Are [scenario type] requirements intentionally excluded or missing? [Gap]"
   - Include resilience/rollback when state mutation occurs: "Are rollback requirements defined for migration failures? [Gap]"

   **Traceability Requirements**:
   - MINIMUM: ≥80% of items MUST include at least one traceability reference
   - Each item should reference: spec section `[Spec §X.Y]`, or use markers: `[Gap]`, `[Ambiguity]`, `[Conflict]`, `[Assumption]`
   - If no ID system exists: "Is a requirement & acceptance criteria ID scheme established? [Traceability]"

   **Surface & Resolve Issues** (Requirements Quality Problems):
   Ask questions about the requirements themselves:
   - Ambiguities: "Is the term 'fast' quantified with specific metrics? [Ambiguity, Spec §NFR-1]"
   - Conflicts: "Do navigation requirements conflict between §FR-10 and §FR-10a? [Conflict]"
   - Assumptions: "Is the assumption of 'always available podcast API' validated? [Assumption]"
   - Dependencies: "Are external podcast API requirements documented? [Dependency, Gap]"
   - Missing definitions: "Is 'visual hierarchy' defined with measurable criteria? [Gap]"

   **Content Consolidation**:
   - Soft cap: If raw candidate items > 40, prioritize by risk/impact
   - Merge near-duplicates checking the same requirement aspect
   - If >5 low-impact edge cases, create one item: "Are edge cases X, Y, Z addressed in requirements? [Coverage]"

   **🚫 ABSOLUTELY PROHIBITED** - These make it an implementation test, not a requirements test:
   - ❌ Any item starting with "Verify", "Test", "Confirm", "Check" + implementation behavior
   - ❌ References to code execution, user actions, system behavior
   - ❌ "Displays correctly", "works properly", "functions as expected"
   - ❌ "Click", "navigate", "render", "load", "execute"
   - ❌ Test cases, test plans, QA procedures
   - ❌ Implementation details (frameworks, APIs, algorithms)

   **✅ REQUIRED PATTERNS** - These test requirements quality:
   - ✅ "Are [requirement type] defined/specified/documented for [scenario]?"
   - ✅ "Is [vague term] quantified/clarified with specific criteria?"
   - ✅ "Are requirements consistent between [section A] and [section B]?"
   - ✅ "Can [requirement] be objectively measured/verified?"
   - ✅ "Are [edge cases/scenarios] addressed in requirements?"
   - ✅ "Does the spec define [missing aspect]?"

6. **Structure Reference**: Generate the checklist following the canonical template in `.specify/templates/checklist-template.md` for title, meta section, category headings, and ID formatting. If template is unavailable, use: H1 title, purpose/created meta lines, `##` category sections containing `- [ ] CHK### <requirement item>` lines with globally incrementing IDs starting at CHK001.

7. **Report**: Output full path to created checklist, item count, and remind user that each run creates a new file. Summarize:
   - Focus areas selected
   - Depth level
   - Actor/timing
   - Any explicit user-specified must-have items incorporated

**Important**: Each `/speckit.checklist` command invocation creates a checklist file using short, descriptive names unless file already exists. This allows:

- Multiple checklists of different types (e.g., `ux.md`, `test.md`, `security.md`)
- Simple, memorable filenames that indicate checklist purpose
- Easy identification and navigation in the `checklists/` folder

To avoid clutter, use descriptive types and clean up obsolete checklists when done.

## Patterns: Best Practices for Requirements Validation Checklists

### Pattern 1: Question-Based Requirement Validation

**Objective**: Evaluate the presence, quality, and completeness of requirements documentation rather than implementation correctness.

**Context of Application**: Use when creating any checklist item intended to assess whether requirements are properly documented, specified, or defined.

**Key Characteristics**:

- Phrased as questions about the requirements themselves
- Focuses on documentation quality dimensions (completeness, clarity, consistency, measurability, coverage)
- References source documentation (spec sections, plan documents)
- Includes quality dimension markers in brackets
- Uses traceability markers ([Spec §X.Y], [Gap], [Ambiguity], etc.)

**Operational Guidance**:

1. Start with interrogative forms: "Are...", "Is...", "Does...", "Can..."
2. Target the requirement artifact, not the system: "Are requirements defined for..." not "Does the system..."
3. Specify what quality dimension you're testing: [Completeness], [Clarity], [Consistency], [Measurability], [Coverage]
4. Add traceability: reference spec sections when validating existing content, use [Gap] when checking for missing content
5. Make it answerable by reading the spec/plan alone, without needing to see implementation

**Example Applications**:

- "Are error handling requirements defined for all API failure modes? [Gap]"
- "Is 'prominent display' quantified with specific sizing/positioning? [Clarity, Spec §FR-4]"
- "Are hover state requirements consistent across all interactive elements? [Consistency]"

---

### Pattern 2: Domain-Specific Checklist Organization

**Objective**: Group related requirement validation items by technical domain to enable focused review and appropriate expertise application.

**Context of Application**: Use when organizing checklists for complex features spanning multiple technical domains or when different stakeholders need to review different aspects.

**Key Characteristics**:

- Separate checklist files per domain (ux.md, api.md, security.md, performance.md)
- Domain-specific quality dimensions and concerns
- Consistent structure across domains but domain-appropriate content
- Short, memorable filenames indicating purpose

**Operational Guidance**:

1. Identify primary technical domains from feature context (UX, API, security, performance, accessibility, etc.)
2. Create separate checklist files using pattern `[domain].md`
3. Within each domain, use standard quality dimension categories but with domain-specific items
4. Ensure cross-domain consistency requirements are captured (e.g., "Are API error codes consistent with UX error messaging?")
5. Allow multiple checklists per feature to enable parallel review by different experts

**Example Applications**:

- UX checklist focuses on visual hierarchy, interaction states, accessibility requirements
- API checklist focuses on error formats, versioning, authentication consistency
- Performance checklist focuses on quantified metrics, load scenarios, degradation requirements
- Security checklist focuses on authentication coverage, data protection, threat model alignment

---

### Pattern 3: Progressive Context Loading

**Objective**: Minimize cognitive load and processing time by loading only the portions of source documentation relevant to the active focus areas.

**Context of Application**: Use when generating checklists from large specification documents, plans, or task lists.

**Key Characteristics**:

- Loads targeted sections rather than entire documents
- Summarizes long sections into concise bullets
- Uses follow-on retrieval only when gaps are detected
- Generates interim summary items for oversized sources

**Operational Guidance**:

1. Parse user input and clarifying answers to identify 2-4 focus areas
2. Map focus areas to relevant sections in spec.md, plan.md, tasks.md
3. Load only those sections initially
4. Summarize multi-paragraph sections into requirement bullets
5. If checklist generation reveals knowledge gaps, perform targeted follow-on retrieval
6. For documents >500 lines, create summary points instead of embedding raw text

**Example Applications**:

- User requests security checklist → load only security-related spec sections, threat model, authentication requirements
- User requests UX checklist → load UI/UX sections, accessibility requirements, interaction specifications
- Detect gap in error handling during generation → perform targeted retrieval of error handling sections

---

### Pattern 4: Signal-Based Dynamic Questioning

**Objective**: Generate contextually relevant clarifying questions by extracting and analyzing signals from user input and source documents rather than using pre-baked question templates.

**Context of Application**: Use during the clarification phase when user input is ambiguous or when additional context would materially change checklist content.

**Key Characteristics**:

- Questions derived from actual content, not generic templates
- Focuses on information that changes checklist scope, depth, or focus
- Skips questions where answers are already clear in user input
- Limits to 3-5 total questions maximum
- Includes justification for follow-up questions beyond initial three

**Operational Guidance**:

1. Extract signals from user input: domain keywords, risk indicators, stakeholder hints, deliverables
2. Extract signals from spec/plan: missing scenario classes, ambiguous terms, undefined non-functionals
3. Cluster signals into 2-4 candidate focus areas ranked by relevance
4. Identify what's already unambiguous in $ARGUMENTS
5. Generate questions only for: scope refinement, risk prioritization, depth calibration, audience framing, boundary exclusion, scenario gaps
6. Format with option tables when appropriate (2-5 options, columns: Option | Candidate | Why It Matters)
7. Stop at 3 questions unless ≥2 scenario classes remain unclear, then ask max 2 more with justifications

**Example Applications**:

- Detected keywords "auth", "compliance" → Q1: "Which compliance frameworks should security requirements align with?"
- No recovery flows in spec → Q2: "Are rollback/partial failure paths in scope for this checklist?"
- Ambiguous depth → Q3: "Is this a lightweight pre-commit sanity list or formal release gate?"

---

### Pattern 5: Traceability-First Item Construction

**Objective**: Ensure every checklist item can be traced back to specific sections of source documentation or explicitly marked as identifying gaps.

**Context of Application**: Use when writing all checklist items to enable efficient validation and gap analysis.

**Key Characteristics**:

- Minimum 80% of items include traceability references
- Uses spec section references [Spec §X.Y] for existing requirements
- Uses markers [Gap], [Ambiguity], [Conflict], [Assumption] for issues
- Enables direct navigation to source for validation
- Facilitates gap/issue reporting

**Operational Guidance**:

1. When validating existing requirements: include [Spec §X.Y] reference pointing to the section being evaluated
2. When checking for missing requirements: use [Gap] marker
3. When identifying unclear requirements: use [Ambiguity, Spec §X.Y]
4. When identifying conflicting requirements: use [Conflict] and reference both sections
5. When checking assumptions: use [Assumption] or [Assumption, Spec §X.Y]
6. If spec has no section IDs: create checklist item "Is a requirement & acceptance criteria ID scheme established? [Traceability]"
7. Ensure ≥80% compliance across the entire checklist

**Example Applications**:

- "Is 'fast loading' quantified with specific timing thresholds? [Clarity, Spec §NFR-2]"
- "Are rollback requirements defined for migration failures? [Gap]"
- "Do navigation requirements conflict between §FR-10 and §FR-10a? [Conflict]"
- "Is the assumption of 'always available podcast API' validated? [Assumption, Spec §INT-3]"

---

### Pattern 6: Scenario Class Coverage Validation

**Objective**: Systematically ensure requirements exist for all relevant scenario types (primary, alternate, exception, recovery, non-functional) rather than just happy-path flows.

**Context of Application**: Use when generating checklists for any feature involving user interactions, state changes, external dependencies, or non-trivial complexity.

**Key Characteristics**:

- Validates requirement coverage across scenario taxonomy: Primary, Alternate, Exception/Error, Recovery, Non-Functional
- Distinguishes between intentionally excluded and missing scenario classes
- Emphasizes resilience and rollback for state-mutating operations
- Tests whether requirements for each scenario class are complete, clear, and consistent

**Operational Guidance**:

1. Identify which scenario classes apply to the feature domain
2. For each applicable class, create checklist item: "Are [scenario type] requirements complete, clear, and consistent?"
3. For potentially missing classes, ask: "Are [scenario type] requirements intentionally excluded or missing? [Gap]"
4. For state-mutating operations (data writes, migrations, deployments): always include "Are rollback/recovery requirements defined? [Gap]"
5. For external dependencies: include "Are timeout/retry requirements defined for [dependency]? [Coverage, Gap]"
6. For user-facing features: include "Are zero-state/empty-state requirements defined? [Coverage, Edge Case]"

**Example Applications**:

- "Are exception/error flow requirements defined for payment processing? [Coverage, Gap]"
- "Are recovery requirements documented for database migration failures? [Gap, Exception Flow]"
- "Are alternate path requirements specified for offline scenarios? [Coverage, Alternate Flow]"
- "Are performance degradation requirements defined for high-load conditions? [Coverage, Non-Functional]"

## Anti-Patterns: Common Mistakes to Avoid

### Anti-Pattern 1: Implementation Verification Items

**Description**: Writing checklist items that test whether the implemented system behaves correctly rather than whether the requirements are properly documented.

**Reasons to Avoid**:

- Fundamentally misunderstands the purpose of requirements validation checklists
- Cannot be completed by reading specification documents alone
- Requires access to running system or codebase
- Conflates requirements phase with testing/QA phase
- Makes checklist unusable during requirements review or before implementation exists

**Negative Consequences**:

- Checklist becomes unusable for its intended purpose (pre-implementation requirements validation)
- Cannot be completed by product managers, analysts, or reviewers without technical implementation access
- Delays discovery of requirement gaps until implementation phase when changes are costly
- Creates confusion about checklist purpose among stakeholders
- Wastes time creating items that duplicate QA test plans

**Correct Alternative**: Use Question-Based Requirement Validation pattern (Pattern 1). Transform verification items into questions about requirement documentation quality.

**Examples of This Anti-Pattern**:

- ❌ "Verify landing page displays 3 episode cards"
- ❌ "Test hover states work correctly on desktop"
- ❌ "Confirm API returns 401 for unauthorized requests"
- ❌ "Check that logo click navigates to home page"

**Corrected Versions**:

- ✅ "Are the number and layout of featured episodes explicitly specified? [Completeness, Spec §FR-001]"
- ✅ "Are hover state requirements consistently defined for all interactive elements? [Consistency, Spec §FR-003]"
- ✅ "Are authentication error responses specified for all protected endpoints? [Completeness, Gap]"
- ✅ "Are navigation requirements clear for all clickable brand elements? [Clarity, Spec §FR-010]"

---

### Anti-Pattern 2: Imperative Verification Verbs

**Description**: Starting checklist items with action verbs that imply executing tests or performing system verification (Verify, Test, Confirm, Check, Execute, Run, Validate [system behavior]).

**Reasons to Avoid**:

- Grammatically signals implementation testing rather than documentation review
- Primes reader to think about system behavior instead of specification quality
- Encourages confusion between requirements validation and QA activities
- Makes items sound like test cases rather than documentation quality checks

**Negative Consequences**:

- Readers misinterpret checklist purpose and attempt to execute items as tests
- Reinforces incorrect mental model of checklist as test plan
- Makes items incompletable during requirements phase
- Stakeholders defer checklist completion until implementation exists
- Team develops duplicate testing artifacts unnecessarily

**Correct Alternative**: Use interrogative question format (Pattern 1): "Are...", "Is...", "Does [the spec] define...", "Can [requirement] be measured..."

**Examples of This Anti-Pattern**:

- ❌ "Verify error messages are user-friendly"
- ❌ "Test that the API handles concurrent requests"
- ❌ "Confirm loading spinner appears during data fetch"
- ❌ "Check navigation works on mobile devices"
- ❌ "Validate input sanitization prevents XSS"

**Corrected Versions**:

- ✅ "Are user-friendly error message requirements specified with examples? [Clarity, Gap]"
- ✅ "Are concurrent request handling requirements defined? [Completeness, Gap]"
- ✅ "Are loading state requirements specified for asynchronous operations? [Coverage, Gap]"
- ✅ "Are mobile navigation requirements explicitly documented? [Completeness, Spec §FR-NAV]"
- ✅ "Are input sanitization requirements defined to prevent XSS? [Security, Gap]"

---

### Anti-Pattern 3: Traceability-Free Items

**Description**: Writing checklist items without references to source documentation sections or gap/issue markers, making validation and follow-up impossible.

**Reasons to Avoid**:

- Reviewer cannot efficiently locate referenced requirements to validate the item
- No way to distinguish between "checking existing requirement quality" vs "identifying missing requirement"
- Impossible to generate actionable gap reports or issue lists
- Breaks ability to trace checklist completion back to spec improvements
- Violates the 80% minimum traceability requirement

**Negative Consequences**:

- Checklist completion requires exhaustive spec searching, wasting reviewer time
- Cannot generate spec improvement action items from checklist results
- No data on where spec gaps exist
- Difficult to measure checklist effectiveness or spec coverage
- Undermines checklist value as requirements quality diagnostic tool

**Correct Alternative**: Use Traceability-First Item Construction pattern (Pattern 5). Include [Spec §X.Y] for existing requirements or [Gap], [Ambiguity], [Conflict], [Assumption] markers.

**Examples of This Anti-Pattern**:

- ❌ "Are performance requirements quantified?"
- ❌ "Is error handling documented?"
- ❌ "Are accessibility requirements included?"
- ❌ "Is the authentication flow defined?"

**Corrected Versions**:

- ✅ "Are performance requirements quantified with specific metrics? [Clarity, Spec §NFR-2]"
- ✅ "Are error handling requirements defined for all failure modes? [Gap]"
- ✅ "Are accessibility requirements specified for all interactive elements? [Completeness, Spec §ACC-1]"
- ✅ "Is the authentication flow defined with state transitions? [Clarity, Spec §SEC-3]"

---

### Anti-Pattern 4: Vague or Unmeasurable Quality Questions

**Description**: Asking whether requirements are "good", "adequate", "sufficient", or "appropriate" without specifying measurable quality criteria or dimension.

**Reasons to Avoid**:

- Subjective terms lead to inconsistent checklist completion across reviewers
- No clear pass/fail criteria for the item
- Doesn't guide reviewer on what specifically to check
- Fails to identify the specific quality dimension being tested
- Provides no actionable feedback on how to improve requirements

**Negative Consequences**:

- Different reviewers interpret items differently, producing inconsistent results
- Items get checked off despite requirement quality issues
- No guidance provided to spec authors on what needs improvement
- Checklist loses diagnostic value
- Doesn't surface specific quality gaps (missing metrics, ambiguous terms, etc.)

**Correct Alternative**: Specify the exact quality dimension being tested and make it measurable. Use patterns from Pattern 1 examples with explicit dimensions: [Completeness], [Clarity], [Consistency], [Measurability], [Coverage].

**Examples of This Anti-Pattern**:

- ❌ "Are the requirements good enough?"
- ❌ "Is the security approach adequate?"
- ❌ "Are performance considerations sufficient?"
- ❌ "Is error handling appropriate?"
- ❌ "Are the UX requirements satisfactory?"

**Corrected Versions**:

- ✅ "Are all functional requirements mapped to acceptance criteria? [Completeness, Traceability]"
- ✅ "Are authentication requirements specified for all protected resources? [Coverage, Spec §SEC-2]"
- ✅ "Are performance targets quantified with specific latency thresholds? [Measurability, Spec §NFR-1]"
- ✅ "Are error response formats specified for all API failure scenarios? [Completeness, Gap]"
- ✅ "Is 'intuitive navigation' defined with measurable usability criteria? [Clarity, Spec §UX-4]"

---

### Anti-Pattern 5: Monolithic Single-Domain Checklists

**Description**: Creating one massive checklist that combines all domains (UX, API, security, performance, etc.) into a single file regardless of feature complexity.

**Reasons to Avoid**:

- Single file becomes unwieldy for complex features (40+ items)
- Cannot distribute review tasks to domain experts efficiently
- Difficult to navigate and find relevant items
- Mixes concerns that may have different reviewers or timelines
- Loses ability to track domain-specific coverage

**Negative Consequences**:

- Checklist review becomes time-consuming and overwhelming
- Domain experts cannot focus on their area without scanning irrelevant items
- Parallel review by multiple experts becomes impractical
- Higher likelihood of items being skipped or overlooked
- Cannot retire/archive domain-specific checklists independently

**Correct Alternative**: Use Domain-Specific Checklist Organization pattern (Pattern 2). Create separate checklist files per domain (ux.md, api.md, security.md) for complex features.

**When This Anti-Pattern Applies**:

- Feature spans 3+ distinct technical domains
- Feature has 30+ total checklist items
- Multiple specialists need to review different aspects
- Domains have different review timelines (e.g., security review after UX review)

**Corrected Approach**:

- Create `ux.md` with 15 items focused on visual hierarchy, interaction states, accessibility
- Create `api.md` with 12 items focused on error formats, authentication, versioning
- Create `security.md` with 8 items focused on threat model, data protection, authorization
- Allows UX designer, backend engineer, and security specialist to work in parallel

---

### Anti-Pattern 6: Pre-Implementation Assumption Overload

**Description**: Writing checklist items that assume implementation details, technology choices, or design decisions that haven't been finalized in requirements phase.

**Reasons to Avoid**:

- Requirements should be implementation-agnostic where possible
- Checklist becomes obsolete if technology choices change
- Prematurely constrains implementation options
- Confuses requirements validation with design review
- May reference frameworks, libraries, or patterns not yet selected

**Negative Consequences**:

- Checklist must be rewritten if implementation approach changes
- Distracts from validating actual business/functional requirements
- May invalidate checklist before requirements phase completes
- Creates false dependencies on technical decisions
- Reduces reusability of checklist items across similar features

**Correct Alternative**: Focus on requirement quality independent of implementation. Ask about required capabilities, constraints, and quality attributes rather than specific technologies or patterns.

**Examples of This Anti-Pattern**:

- ❌ "Are React component prop types documented? [Spec §TECH-5]"
- ❌ "Are Redux action creators specified for state mutations? [Completeness]"
- ❌ "Is the REST API versioning strategy using URL paths? [Clarity]"
- ❌ "Are PostgreSQL transaction isolation levels defined? [Gap]"

**Corrected Versions**:

- ✅ "Are component interface contracts (inputs/outputs) documented? [Completeness, Spec §ARCH-5]"
- ✅ "Are state mutation requirements and invariants specified? [Clarity, Gap]"
- ✅ "Is the API versioning strategy documented? [Completeness, Spec §API-2]"
- ✅ "Are transaction isolation requirements defined for concurrent operations? [Consistency, Gap]"

---

### Anti-Pattern 7: Scenario Class Blind Spots

**Description**: Focusing exclusively on primary/happy-path requirements while neglecting to validate whether alternate, exception, recovery, and non-functional scenario requirements exist.

**Reasons to Avoid**:

- Most requirement gaps occur in non-primary scenarios
- Exception and recovery paths are often completely undocumented
- Non-functional requirements (performance, security) frequently omitted
- Real-world failures occur in alternate and exception paths
- Production issues traced to missing edge case requirements

**Negative Consequences**:

- Checklist gives false sense of completeness while major gaps remain
- Teams discover missing requirements during implementation (costly)
- Production incidents occur due to unspecified error handling
- Recovery procedures undefined, leading to operational failures
- Non-functional requirement gaps discovered during load testing or security audit

**Correct Alternative**: Use Scenario Class Coverage Validation pattern (Pattern 6). Systematically validate requirement coverage across Primary, Alternate, Exception, Recovery, and Non-Functional scenario classes.

**Examples of This Anti-Pattern** (missing scenario coverage):

```markdown
## Requirement Completeness
- [ ] CHK001 - Are user registration fields specified? [Spec §FR-1]
- [ ] CHK002 - Are login flow steps documented? [Spec §FR-2]
- [ ] CHK003 - Are dashboard layout requirements defined? [Spec §FR-3]
```

**Corrected Version** (comprehensive scenario coverage):

```markdown
## Requirement Completeness
- [ ] CHK001 - Are user registration fields specified? [Spec §FR-1]
- [ ] CHK002 - Are login flow steps documented? [Spec §FR-2]
- [ ] CHK003 - Are dashboard layout requirements defined? [Spec §FR-3]

## Exception & Error Coverage
- [ ] CHK004 - Are authentication failure scenarios and error messages defined? [Gap, Exception Flow]
- [ ] CHK005 - Are requirements specified for invalid registration input? [Gap]
- [ ] CHK006 - Are network timeout/failure requirements documented? [Gap, Exception Flow]

## Recovery & Resilience
- [ ] CHK007 - Are session timeout and re-authentication requirements defined? [Gap, Recovery]
- [ ] CHK008 - Are password reset flow requirements specified? [Coverage, Alternate Flow]

## Non-Functional Requirements
- [ ] CHK009 - Are authentication performance targets quantified? [Gap, Performance]
- [ ] CHK010 - Are password strength and storage requirements defined? [Gap, Security]
- [ ] CHK011 - Are accessibility requirements specified for login forms? [Gap, Accessibility]
```

## Example Checklist Types & Sample Items

**UX Requirements Quality:** `ux.md`

Sample items (testing the requirements, NOT the implementation):

- "Are visual hierarchy requirements defined with measurable criteria? [Clarity, Spec §FR-1]"
- "Is the number and positioning of UI elements explicitly specified? [Completeness, Spec §FR-1]"
- "Are interaction state requirements (hover, focus, active) consistently defined? [Consistency]"
- "Are accessibility requirements specified for all interactive elements? [Coverage, Gap]"
- "Is fallback behavior defined when images fail to load? [Edge Case, Gap]"
- "Can 'prominent display' be objectively measured? [Measurability, Spec §FR-4]"

**API Requirements Quality:** `api.md`

Sample items:

- "Are error response formats specified for all failure scenarios? [Completeness]"
- "Are rate limiting requirements quantified with specific thresholds? [Clarity]"
- "Are authentication requirements consistent across all endpoints? [Consistency]"
- "Are retry/timeout requirements defined for external dependencies? [Coverage, Gap]"
- "Is versioning strategy documented in requirements? [Gap]"

**Performance Requirements Quality:** `performance.md`

Sample items:

- "Are performance requirements quantified with specific metrics? [Clarity]"
- "Are performance targets defined for all critical user journeys? [Coverage]"
- "Are performance requirements under different load conditions specified? [Completeness]"
- "Can performance requirements be objectively measured? [Measurability]"
- "Are degradation requirements defined for high-load scenarios? [Edge Case, Gap]"

**Security Requirements Quality:** `security.md`

Sample items:

- "Are authentication requirements specified for all protected resources? [Coverage]"
- "Are data protection requirements defined for sensitive information? [Completeness]"
- "Is the threat model documented and requirements aligned to it? [Traceability]"
- "Are security requirements consistent with compliance obligations? [Consistency]"
- "Are security failure/breach response requirements defined? [Gap, Exception Flow]"

## Anti-Examples: What NOT To Do

**❌ WRONG - These test implementation, not requirements:**

```markdown
- [ ] CHK001 - Verify landing page displays 3 episode cards [Spec §FR-001]
- [ ] CHK002 - Test hover states work correctly on desktop [Spec §FR-003]
- [ ] CHK003 - Confirm logo click navigates to home page [Spec §FR-010]
- [ ] CHK004 - Check that related episodes section shows 3-5 items [Spec §FR-005]
```

**✅ CORRECT - These test requirements quality:**

```markdown
- [ ] CHK001 - Are the number and layout of featured episodes explicitly specified? [Completeness, Spec §FR-001]
- [ ] CHK002 - Are hover state requirements consistently defined for all interactive elements? [Consistency, Spec §FR-003]
- [ ] CHK003 - Are navigation requirements clear for all clickable brand elements? [Clarity, Spec §FR-010]
- [ ] CHK004 - Is the selection criteria for related episodes documented? [Gap, Spec §FR-005]
- [ ] CHK005 - Are loading state requirements defined for asynchronous episode data? [Gap]
- [ ] CHK006 - Can "visual hierarchy" requirements be objectively measured? [Measurability, Spec §FR-001]
```

**Key Differences:**

- Wrong: Tests if the system works correctly
- Correct: Tests if the requirements are written correctly
- Wrong: Verification of behavior
- Correct: Validation of requirement quality
- Wrong: "Does it do X?"
- Correct: "Is X clearly specified?"
