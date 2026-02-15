---
description: Identify underspecified areas in the current feature spec by asking up to 5 highly targeted clarification questions and encoding answers back into the spec.
handoffs: 
  - label: Build Technical Plan
    agent: speckit.plan
    prompt: Create a plan for the spec. I am building with...
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

Goal: Detect and reduce ambiguity or missing decision points in the active feature specification and record the clarifications directly in the spec file.

Note: This clarification workflow is expected to run (and be completed) BEFORE invoking `/speckit.plan`. If the user explicitly states they are skipping clarification (e.g., exploratory spike), you may proceed, but must warn that downstream rework risk increases.

Execution steps:

1. Run `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/check-prerequisites.ps1 -Json -PathsOnly` from repo root **once** (combined `--json --paths-only` mode / `-Json -PathsOnly`). Parse minimal JSON payload fields:
   - `FEATURE_DIR`
   - `FEATURE_SPEC`
   - (Optionally capture `IMPL_PLAN`, `TASKS` for future chained flows.)
   - If JSON parsing fails, abort and instruct user to re-run `/speckit.specify` or verify feature branch environment.
   - For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").

2. Load the current spec file. Perform a structured ambiguity & coverage scan using this taxonomy. For each category, mark status: Clear / Partial / Missing. Produce an internal coverage map used for prioritization (do not output raw map unless no questions will be asked).

   Functional Scope & Behavior:
   - Core user goals & success criteria
   - Explicit out-of-scope declarations
   - User roles / personas differentiation

   Domain & Data Model:
   - Entities, attributes, relationships
   - Identity & uniqueness rules
   - Lifecycle/state transitions
   - Data volume / scale assumptions

   Interaction & UX Flow:
   - Critical user journeys / sequences
   - Error/empty/loading states
   - Accessibility or localization notes

   Non-Functional Quality Attributes:
   - Performance (latency, throughput targets)
   - Scalability (horizontal/vertical, limits)
   - Reliability & availability (uptime, recovery expectations)
   - Observability (logging, metrics, tracing signals)
   - Security & privacy (authN/Z, data protection, threat assumptions)
   - Compliance / regulatory constraints (if any)

   Integration & External Dependencies:
   - External services/APIs and failure modes
   - Data import/export formats
   - Protocol/versioning assumptions

   Edge Cases & Failure Handling:
   - Negative scenarios
   - Rate limiting / throttling
   - Conflict resolution (e.g., concurrent edits)

   Constraints & Tradeoffs:
   - Technical constraints (language, storage, hosting)
   - Explicit tradeoffs or rejected alternatives

   Terminology & Consistency:
   - Canonical glossary terms
   - Avoided synonyms / deprecated terms

   Completion Signals:
   - Acceptance criteria testability
   - Measurable Definition of Done style indicators

   Misc / Placeholders:
   - TODO markers / unresolved decisions
   - Ambiguous adjectives ("robust", "intuitive") lacking quantification

   For each category with Partial or Missing status, add a candidate question opportunity unless:
   - Clarification would not materially change implementation or validation strategy
   - Information is better deferred to planning phase (note internally)

3. Generate (internally) a prioritized queue of candidate clarification questions (maximum 5). Do NOT output them all at once. Apply these constraints:
    - Maximum of 10 total questions across the whole session.
    - Each question must be answerable with EITHER:
       - A short multiple‑choice selection (2–5 distinct, mutually exclusive options), OR
       - A one-word / short‑phrase answer (explicitly constrain: "Answer in <=5 words").
    - Only include questions whose answers materially impact architecture, data modeling, task decomposition, test design, UX behavior, operational readiness, or compliance validation.
    - Ensure category coverage balance: attempt to cover the highest impact unresolved categories first; avoid asking two low-impact questions when a single high-impact area (e.g., security posture) is unresolved.
    - Exclude questions already answered, trivial stylistic preferences, or plan-level execution details (unless blocking correctness).
    - Favor clarifications that reduce downstream rework risk or prevent misaligned acceptance tests.
    - If more than 5 categories remain unresolved, select the top 5 by (Impact * Uncertainty) heuristic.

4. Sequential questioning loop (interactive):
    - Present EXACTLY ONE question at a time.
    - For multiple‑choice questions:
       - **Analyze all options** and determine the **most suitable option** based on:
          - Best practices for the project type
          - Common patterns in similar implementations
          - Risk reduction (security, performance, maintainability)
          - Alignment with any explicit project goals or constraints visible in the spec
       - Present your **recommended option prominently** at the top with clear reasoning (1-2 sentences explaining why this is the best choice).
       - Format as: `**Recommended:** Option [X] - <reasoning>`
       - Then render all options as a Markdown table:

       | Option | Description                                                                                         |
       | ------ | --------------------------------------------------------------------------------------------------- |
       | A      | <Option A description>                                                                              |
       | B      | <Option B description>                                                                              |
       | C      | <Option C description> (add D/E as needed up to 5)                                                  |
       | Short  | Provide a different short answer (<=5 words) (Include only if free-form alternative is appropriate) |

       - After the table, add: `You can reply with the option letter (e.g., "A"), accept the recommendation by saying "yes" or "recommended", or provide your own short answer.`
    - For short‑answer style (no meaningful discrete options):
       - Provide your **suggested answer** based on best practices and context.
       - Format as: `**Suggested:** <your proposed answer> - <brief reasoning>`
       - Then output: `Format: Short answer (<=5 words). You can accept the suggestion by saying "yes" or "suggested", or provide your own answer.`
    - After the user answers:
       - If the user replies with "yes", "recommended", or "suggested", use your previously stated recommendation/suggestion as the answer.
       - Otherwise, validate the answer maps to one option or fits the <=5 word constraint.
       - If ambiguous, ask for a quick disambiguation (count still belongs to same question; do not advance).
       - Once satisfactory, record it in working memory (do not yet write to disk) and move to the next queued question.
    - Stop asking further questions when:
       - All critical ambiguities resolved early (remaining queued items become unnecessary), OR
       - User signals completion ("done", "good", "no more"), OR
       - You reach 5 asked questions.
    - Never reveal future queued questions in advance.
    - If no valid questions exist at start, immediately report no critical ambiguities.

5. Integration after EACH accepted answer (incremental update approach):
    - Maintain in-memory representation of the spec (loaded once at start) plus the raw file contents.
    - For the first integrated answer in this session:
       - Ensure a `## Clarifications` section exists (create it just after the highest-level contextual/overview section per the spec template if missing).
       - Under it, create (if not present) a `### Session YYYY-MM-DD` subheading for today.
    - Append a bullet line immediately after acceptance: `- Q: <question> → A: <final answer>`.
    - Then immediately apply the clarification to the most appropriate section(s):
       - Functional ambiguity → Update or add a bullet in Functional Requirements.
       - User interaction / actor distinction → Update User Stories or Actors subsection (if present) with clarified role, constraint, or scenario.
       - Data shape / entities → Update Data Model (add fields, types, relationships) preserving ordering; note added constraints succinctly.
       - Non-functional constraint → Add/modify measurable criteria in Non-Functional / Quality Attributes section (convert vague adjective to metric or explicit target).
       - Edge case / negative flow → Add a new bullet under Edge Cases / Error Handling (or create such subsection if template provides placeholder for it).
       - Terminology conflict → Normalize term across spec; retain original only if necessary by adding `(formerly referred to as "X")` once.
    - If the clarification invalidates an earlier ambiguous statement, replace that statement instead of duplicating; leave no obsolete contradictory text.
    - Save the spec file AFTER each integration to minimize risk of context loss (atomic overwrite).
    - Preserve formatting: do not reorder unrelated sections; keep heading hierarchy intact.
    - Keep each inserted clarification minimal and testable (avoid narrative drift).

6. Validation (performed after EACH write plus final pass):
   - Clarifications session contains exactly one bullet per accepted answer (no duplicates).
   - Total asked (accepted) questions ≤ 5.
   - Updated sections contain no lingering vague placeholders the new answer was meant to resolve.
   - No contradictory earlier statement remains (scan for now-invalid alternative choices removed).
   - Markdown structure valid; only allowed new headings: `## Clarifications`, `### Session YYYY-MM-DD`.
   - Terminology consistency: same canonical term used across all updated sections.

7. Write the updated spec back to `FEATURE_SPEC`.

8. Report completion (after questioning loop ends or early termination):
   - Number of questions asked & answered.
   - Path to updated spec.
   - Sections touched (list names).
   - Coverage summary table listing each taxonomy category with Status: Resolved (was Partial/Missing and addressed), Deferred (exceeds question quota or better suited for planning), Clear (already sufficient), Outstanding (still Partial/Missing but low impact).
   - If any Outstanding or Deferred remain, recommend whether to proceed to `/speckit.plan` or run `/speckit.clarify` again later post-plan.
   - Suggested next command.

## Patterns: Best Practices for Specification Clarification

### Pattern: Incremental Atomic Integration

**Objective:** Minimize data loss and maintain spec consistency by persisting each clarification immediately after acceptance.

**Context of Application:** Any interactive specification refinement workflow where multiple clarifications are gathered sequentially, especially in environments with potential session interruptions or context limitations.

**Key Characteristics:**

- Each accepted answer triggers an immediate write operation to the spec file
- In-memory representation synchronized with disk after every change
- Clarifications section grows incrementally rather than batch-updated
- File system state remains consistent at each interaction boundary

**Operational Guidance:**

1. Load the specification file once at workflow initialization and maintain an in-memory working copy
2. After each user answer is validated and accepted, immediately append the Q&A pair to the Clarifications section
3. Apply the semantic integration to relevant spec sections (Functional Requirements, Data Model, etc.) in the same atomic operation
4. Write the complete updated spec to disk using atomic file replacement (write to temp, then rename)
5. Verify the write succeeded before proceeding to the next question
6. In case of write failure, retry once, then abort the session with clear error messaging
7. Never accumulate multiple unwritten changes in memory across question boundaries

### Pattern: Impact-Weighted Question Prioritization

**Objective:** Maximize specification quality improvement within strict question quota constraints by addressing highest-impact ambiguities first.

**Context of Application:** When scanning a specification reveals more potential ambiguities than the question budget allows, requiring systematic prioritization.

**Key Characteristics:**

- Each candidate question scored on two dimensions: Impact (architectural/testing/validation consequences) and Uncertainty (degree of ambiguity)
- Priority queue sorted by Impact × Uncertainty heuristic
- Category balance enforced to prevent over-focusing on single taxonomy area
- Questions deferred to planning phase explicitly documented with rationale

**Operational Guidance:**

1. During coverage scan (step 2), tag each identified gap with its taxonomy category
2. Score Impact on scale 1-5 based on: Does this affect architecture (5), data modeling (4), UX flows (3), edge case handling (2), or documentation only (1)?
3. Score Uncertainty on scale 1-5: Complete absence (5), conflicting hints (4), vague language (3), partial info (2), mostly clear (1)
4. Calculate composite score: Impact × Uncertainty for each candidate question
5. Sort candidate questions by composite score descending
6. Apply category diversity filter: if top 5 questions span fewer than 3 categories, demote lowest-impact duplicates and promote highest-impact questions from underrepresented categories
7. Queue exactly 5 questions (or fewer if insufficient candidates meet materiality threshold)
8. Document deferred questions in internal state for completion report

### Pattern: Guided Answer Recommendation

**Objective:** Accelerate decision-making and reduce cognitive load by providing expert-informed default options while preserving user autonomy.

**Context of Application:** All clarification questions, whether multiple-choice or short-answer format, where best practices or common patterns can inform a sensible default.

**Key Characteristics:**

- Agent analyzes all available options against domain best practices, risk factors, and visible project constraints
- Recommendation presented prominently with concise justification (1-2 sentences)
- User retains full control: can accept, reject, or override with custom answer
- Acceptance shortcuts ("yes", "recommended", "suggested") streamline interaction

**Operational Guidance:**

1. For multiple-choice questions, evaluate each option against: security implications, performance characteristics, maintainability burden, industry standards, alignment with any explicit project goals visible in spec
2. Select the option that optimally balances these factors; if genuinely ambiguous, select the safest/most conservative option
3. Format recommendation: `**Recommended:** Option [X] - <reasoning>` where reasoning crisply explains the primary advantage
4. Present complete option table below the recommendation for transparency
5. For short-answer questions, formulate a suggested answer following the same evaluation approach
6. Format suggestion: `**Suggested:** <answer> - <reasoning>`
7. Always include acceptance language: "You can accept by saying 'yes'..." to establish interaction pattern
8. When user responds with acceptance keyword, use the stated recommendation/suggestion verbatim as the final answer

### Pattern: Dual-Track Clarification Recording

**Objective:** Maintain both audit trail and semantic integration by recording clarifications in dedicated session log AND updating relevant specification sections.

**Context of Application:** All clarification integrations where traceability and discoverability are both valued.

**Key Characteristics:**

- Clarifications section serves as chronological session log with verbatim Q&A pairs
- Semantic content from answers propagated to appropriate domain sections (Functional Requirements, Data Model, etc.)
- No duplication of full answer text; session log references only, domain sections contain actionable specifications
- Outdated or contradictory statements removed during semantic integration

**Operational Guidance:**

1. On first accepted answer in a session, ensure `## Clarifications` section exists (create after overview/context section if absent)
2. Create `### Session YYYY-MM-DD` subsection using current date
3. Append bullet: `- Q: <question text> → A: <accepted answer>` to session subsection
4. Analyze the answer's semantic category (functional, data, non-functional, edge case, terminology)
5. Navigate to the appropriate specification section(s)
6. Insert or update content: for functional scope add requirements bullet; for data model add entity/field/constraint; for edge cases add scenario; for terminology normalize all occurrences
7. If the new clarification contradicts or obsoletes existing text, delete or replace the old statement rather than leaving both
8. Preserve all other formatting, ordering, and hierarchy in the spec
9. Validate that the session log entry remains concise (single line) while domain sections contain full actionable detail

### Pattern: Bounded Interactive Clarification

**Objective:** Prevent specification clarification from becoming an unbounded requirements elicitation exercise by enforcing strict question quotas and materiality thresholds.

**Context of Application:** Any specification refinement workflow where scope creep and analysis paralysis are risks, particularly when clarification precedes technical planning phases.

**Key Characteristics:**

- Hard limit of 5 asked questions per clarification session
- Materiality filter: only include questions whose answers substantively affect implementation, testing, or validation strategies
- Early termination signals respected ("done", "stop", "proceed")
- Coverage tracking differentiates Critical vs. Low-Impact gaps

**Operational Guidance:**

1. During question generation (step 3), apply materiality test to each candidate: Would not having this answer cause architectural rework, test case failure, incorrect data modeling, or missed edge cases? If no, exclude the question.
2. Initialize question counter at 0; increment only when user provides accepted answer (disambiguation retries do not count)
3. After each accepted answer, re-evaluate remaining queued questions: have dependencies been resolved that make subsequent questions unnecessary? If yes, remove them from queue.
4. Check question counter against limit (5) before presenting next question; if limit reached, proceed directly to completion report
5. Monitor user responses for termination signals: "done", "good", "no more", "stop", "proceed" → immediately exit questioning loop
6. In completion report, categorize unasked questions as Deferred (exceeded quota) or Outstanding (low impact), with explicit rationale
7. Never exceed the 5-question limit, even if high-impact ambiguities remain; instead flag them clearly and suggest re-running clarification after initial planning if they prove blocking

## Anti-Patterns: Common Mistakes in Specification Clarification

### Anti-Pattern: Batch Integration with Deferred Persistence

**Description:** Accumulating all clarification answers in memory throughout the questioning session and writing the updated specification only once at the end.

**Reasons to Avoid:**

- Session interruption (network failure, timeout, user disconnect) results in complete loss of all gathered clarifications
- Large context windows increase risk of data corruption or inconsistency between in-memory state and intended file state
- Debugging integration errors becomes harder when multiple changes applied simultaneously
- User cannot verify incremental changes for correctness, reducing trust and increasing rework likelihood

**Negative Consequences:**

- Lost work requires complete re-run of clarification session, frustrating users and wasting time
- Batch errors affect multiple sections simultaneously, making rollback and diagnosis complex
- Specification file remains in stale state during entire session, creating synchronization issues if user examines file externally
- Memory pressure in long sessions may cause performance degradation or crashes

**Correct Alternative:** Use the **Incremental Atomic Integration** pattern to persist each clarification immediately after acceptance, maintaining specification consistency at all interaction boundaries.

### Anti-Pattern: Unprioritized Question Flooding

**Description:** Presenting all identified ambiguities as clarification questions without impact assessment or quota management, often overwhelming the user with 10-20+ questions covering trivial and critical gaps indiscriminately.

**Reasons to Avoid:**

- User fatigue leads to degraded answer quality for later questions, particularly for genuinely important decisions
- Low-impact questions consume limited question budget, preventing coverage of high-impact ambiguities
- No systematic approach to determine which gaps can be safely deferred to planning phase
- Creates perception that specification process is bureaucratic and low-value

**Negative Consequences:**

- Critical architectural ambiguities remain unresolved while trivial styling preferences are clarified
- User abandons clarification session before completion, leaving specification in inconsistent partially-clarified state
- Downstream planning and implementation phases still require rework due to missed high-impact decisions
- Reduced user engagement with future clarification workflows due to negative experience

**Correct Alternative:** Apply the **Impact-Weighted Question Prioritization** pattern to systematically rank candidate questions by (Impact × Uncertainty) and enforce strict budget limits, ensuring critical gaps are addressed first.

### Anti-Pattern: Silent Default Assumption

**Description:** When encountering ambiguities, the agent makes implicit assumptions about intended behavior, data models, or constraints without surfacing them to the user for validation, then silently encodes these assumptions into the specification.

**Reasons to Avoid:**

- Assumptions may directly contradict user's actual intent, creating incorrect specification baseline
- User remains unaware of critical decisions being made on their behalf, preventing informed course correction
- No audit trail of why certain design choices were made, obscuring decision rationale for future readers
- Violates principle of user autonomy and informed consent in specification development

**Negative Consequences:**

- Implementation proceeds based on incorrect assumptions, requiring extensive rework when discovered during testing or deployment
- Specification becomes internally inconsistent as some sections reflect user intent while others reflect agent assumptions
- Trust erosion: user loses confidence in specification accuracy and must manually audit all content
- Knowledge gap: team members cannot understand specification evolution or challenge questionable decisions

**Correct Alternative:** Use the **Guided Answer Recommendation** pattern to surface expert-informed defaults explicitly, with reasoning, while preserving user's ability to accept, reject, or override. Always ask rather than assume.

### Anti-Pattern: Append-Only Clarification Logging

**Description:** Recording each clarification answer exclusively in the Clarifications session log without propagating the semantic content to the relevant domain sections of the specification (Functional Requirements, Data Model, etc.).

**Reasons to Avoid:**

- Clarifications remain buried in chronological log format, requiring readers to manually correlate Q&A pairs with relevant spec sections
- Specification sections retain original ambiguous or contradictory language, making them unreliable as standalone reference
- Duplicate maintenance burden: same information conceptually exists in two places (session log and reader's mental model) but only one is written
- Testing, implementation, and validation teams unlikely to discover critical constraints hidden in clarification logs

**Negative Consequences:**

- Developers implement features based on incomplete or ambiguous spec sections, missing critical clarifications
- Test cases fail to cover edge cases or constraints documented only in session logs
- Code reviews cannot validate correctness against specification because constraints are not in expected sections
- Specification becomes progressively less useful over time as clarification log grows while core sections remain static

**Correct Alternative:** Use the **Dual-Track Clarification Recording** pattern to maintain session audit trail while simultaneously updating relevant specification sections with actionable content, removing contradictory outdated statements.

### Anti-Pattern: Unbounded Requirements Elicitation

**Description:** Treating specification clarification as an open-ended requirements gathering exercise, continuously generating new questions without quota limits, materiality filters, or clear completion criteria.

**Reasons to Avoid:**

- Clarification phase bleeds into requirements analysis and design, violating phase boundaries and delaying planning/implementation
- Diminishing returns: later questions often address marginal concerns with minimal implementation impact
- User cannot distinguish between "must answer now" versus "can decide during planning" questions
- No forcing function to accept reasonable uncertainty and proceed with incomplete information

**Negative Consequences:**

- Specification process becomes perceived as bloated and slow, reducing team adoption and engagement
- Analysis paralysis: team becomes stuck in clarification loops while competitive pressure or deadlines mount
- Over-specified solutions lose flexibility for implementation-time discoveries and emergent insights
- Resource waste: hours spent clarifying details that planning/implementation would naturally resolve through technical constraints

**Correct Alternative:** Apply the **Bounded Interactive Clarification** pattern with strict 5-question limit, materiality threshold requiring architectural/testing impact, and explicit categorization of deferred vs. outstanding gaps in completion report.

### Anti-Pattern: Opaque Option Presentation

**Description:** Presenting multiple-choice clarification questions as bare option lists without analysis, recommendation, or reasoning, forcing users to independently evaluate unfamiliar technical tradeoffs.

**Reasons to Avoid:**

- User lacks domain expertise to evaluate security, performance, or maintainability implications of each option
- Decision paralysis when all options seem equally valid or all seem problematic
- Suboptimal choices made due to incomplete understanding of consequences
- Missed opportunity to transfer knowledge and build user's technical judgment

**Negative Consequences:**

- User selects option with hidden drawbacks (security vulnerability, scalability bottleneck, vendor lock-in) due to lack of expert guidance
- Frustration and reduced trust: user feels abandoned to make critical technical decisions without adequate support
- Inconsistent decisions across similar questions due to lack of coherent decision framework
- Increased rework when problematic choice discovered during implementation or security review

**Correct Alternative:** Use the **Guided Answer Recommendation** pattern to analyze options against best practices, present explicit recommendation with reasoning, and provide acceptance shortcuts while preserving user autonomy to override when their context differs.

Behavior rules:

- If no meaningful ambiguities found (or all potential questions would be low-impact), respond: "No critical ambiguities detected worth formal clarification." and suggest proceeding.
- If spec file missing, instruct user to run `/speckit.specify` first (do not create a new spec here).
- Never exceed 5 total asked questions (clarification retries for a single question do not count as new questions).
- Avoid speculative tech stack questions unless the absence blocks functional clarity.
- Respect user early termination signals ("stop", "done", "proceed").
- If no questions asked due to full coverage, output a compact coverage summary (all categories Clear) then suggest advancing.
- If quota reached with unresolved high-impact categories remaining, explicitly flag them under Deferred with rationale.

Context for prioritization: $ARGUMENTS
