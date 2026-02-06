---
description: Create or update the feature specification from a natural language feature description.
handoffs: 
  - label: Build Technical Plan
    agent: speckit.plan
    prompt: Create a plan for the spec. I am building with...
  - label: Clarify Spec Requirements
    agent: speckit.clarify
    prompt: Clarify specification requirements
    send: true
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

The text the user typed after `/speckit.specify` in the triggering message **is** the feature description. Assume you always have it available in this conversation even if `{{args}}` appears literally below. Do not ask the user to repeat it unless they provided an empty command.

Given that feature description, do this:

1. **Generate a concise short name** (2-4 words) for the branch:
   - Analyze the feature description and extract the most meaningful keywords
   - Create a 2-4 word short name that captures the essence of the feature
   - Use action-noun format when possible (e.g., "add-user-auth", "fix-payment-bug")
   - Preserve technical terms and acronyms (OAuth2, API, JWT, etc.)
   - Keep it concise but descriptive enough to understand the feature at a glance
   - Examples:
     - "I want to add user authentication" → "user-auth"
     - "Implement OAuth2 integration for the API" → "oauth2-api-integration"
     - "Create a dashboard for analytics" → "analytics-dashboard"
     - "Fix payment processing timeout bug" → "fix-payment-timeout"

2. **Check for existing branches before creating new one**:

   a. First, fetch all remote branches to ensure we have the latest information:

      ```bash
      git fetch --all --prune
      ```

   b. Find the highest feature number across all sources for the short-name:
      - Remote branches: `git ls-remote --heads origin | grep -E 'refs/heads/[0-9]+-<short-name>$'`
      - Local branches: `git branch | grep -E '^[* ]*[0-9]+-<short-name>$'`
      - Specs directories: Check for directories matching `specs/[0-9]+-<short-name>`

   c. Determine the next available number:
      - Extract all numbers from all three sources
      - Find the highest number N
      - Use N+1 for the new branch number

   d. Run the script `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/create-new-feature.ps1 -Json "{{args}}"` with the calculated number and short-name:
      - Pass `--number N+1` and `--short-name "your-short-name"` along with the feature description
      - Bash example: `.specify/scripts/powershell/create-new-feature.ps1 -Json "{{args}}" --json --number 5 --short-name "user-auth" "Add user authentication"`
      - PowerShell example: `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/create-new-feature.ps1 -Json "{{args}}" -Json -Number 5 -ShortName "user-auth" "Add user authentication"`

   **IMPORTANT**:
   - Check all three sources (remote branches, local branches, specs directories) to find the highest number
   - Only match branches/directories with the exact short-name pattern
   - If no existing branches/directories found with this short-name, start with number 1
   - You must only ever run this script once per feature
   - The JSON is provided in the terminal as output - always refer to it to get the actual content you're looking for
   - The JSON output will contain BRANCH_NAME and SPEC_FILE paths
   - For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot")

3. Load `.specify/templates/spec-template.md` to understand required sections.

4. Follow this execution flow:

    1. Parse user description from Input
       If empty: ERROR "No feature description provided"
    2. Extract key concepts from description
       Identify: actors, actions, data, constraints
    3. For unclear aspects:
       - Make informed guesses based on context and industry standards
       - Only mark with [NEEDS CLARIFICATION: specific question] if:
         - The choice significantly impacts feature scope or user experience
         - Multiple reasonable interpretations exist with different implications
         - No reasonable default exists
       - **LIMIT: Maximum 3 [NEEDS CLARIFICATION] markers total**
       - Prioritize clarifications by impact: scope > security/privacy > user experience > technical details
    4. Fill User Scenarios & Testing section
       If no clear user flow: ERROR "Cannot determine user scenarios"
    5. Generate Functional Requirements
       Each requirement must be testable
       Use reasonable defaults for unspecified details (document assumptions in Assumptions section)
    6. Define Success Criteria
       Create measurable, technology-agnostic outcomes
       Include both quantitative metrics (time, performance, volume) and qualitative measures (user satisfaction, task completion)
       Each criterion must be verifiable without implementation details
    7. Identify Key Entities (if data involved)
    8. Return: SUCCESS (spec ready for planning)

5. Write the specification to SPEC_FILE using the template structure, replacing placeholders with concrete details derived from the feature description (arguments) while preserving section order and headings.

6. **Specification Quality Validation**: After writing the initial spec, validate it against quality criteria:

   a. **Create Spec Quality Checklist**: Generate a checklist file at `FEATURE_DIR/checklists/requirements.md` using the checklist template structure with these validation items:

      ```markdown
      # Specification Quality Checklist: [FEATURE NAME]
      
      **Purpose**: Validate specification completeness and quality before proceeding to planning
      **Created**: [DATE]
      **Feature**: [Link to spec.md]
      
      ## Content Quality
      
      - [ ] No implementation details (languages, frameworks, APIs)
      - [ ] Focused on user value and business needs
      - [ ] Written for non-technical stakeholders
      - [ ] All mandatory sections completed
      
      ## Requirement Completeness
      
      - [ ] No [NEEDS CLARIFICATION] markers remain
      - [ ] Requirements are testable and unambiguous
      - [ ] Success criteria are measurable
      - [ ] Success criteria are technology-agnostic (no implementation details)
      - [ ] All acceptance scenarios are defined
      - [ ] Edge cases are identified
      - [ ] Scope is clearly bounded
      - [ ] Dependencies and assumptions identified
      
      ## Feature Readiness
      
      - [ ] All functional requirements have clear acceptance criteria
      - [ ] User scenarios cover primary flows
      - [ ] Feature meets measurable outcomes defined in Success Criteria
      - [ ] No implementation details leak into specification
      
      ## Notes
      
      - Items marked incomplete require spec updates before `/speckit.clarify` or `/speckit.plan`
      ```

   b. **Run Validation Check**: Review the spec against each checklist item:
      - For each item, determine if it passes or fails
      - Document specific issues found (quote relevant spec sections)

   c. **Handle Validation Results**:

      - **If all items pass**: Mark checklist complete and proceed to step 7

      - **If items fail (excluding [NEEDS CLARIFICATION])**:
        1. List the failing items and specific issues
        2. Update the spec to address each issue
        3. Re-run validation until all items pass (max 3 iterations)
        4. If still failing after 3 iterations, document remaining issues in checklist notes and warn user

      - **If [NEEDS CLARIFICATION] markers remain**:
        1. Extract all [NEEDS CLARIFICATION: ...] markers from the spec
        2. **LIMIT CHECK**: If more than 3 markers exist, keep only the 3 most critical (by scope/security/UX impact) and make informed guesses for the rest
        3. For each clarification needed (max 3), present options to user in this format:

           ```markdown
           ## Question [N]: [Topic]
           
           **Context**: [Quote relevant spec section]
           
           **What we need to know**: [Specific question from NEEDS CLARIFICATION marker]
           
           **Suggested Answers**:
           
           | Option | Answer | Implications |
           |--------|--------|--------------|
           | A      | [First suggested answer] | [What this means for the feature] |
           | B      | [Second suggested answer] | [What this means for the feature] |
           | C      | [Third suggested answer] | [What this means for the feature] |
           | Custom | Provide your own answer | [Explain how to provide custom input] |
           
           **Your choice**: _[Wait for user response]_
           ```

        4. **CRITICAL - Table Formatting**: Ensure markdown tables are properly formatted:
           - Use consistent spacing with pipes aligned
           - Each cell should have spaces around content: `| Content |` not `|Content|`
           - Header separator must have at least 3 dashes: `|--------|`
           - Test that the table renders correctly in markdown preview
        5. Number questions sequentially (Q1, Q2, Q3 - max 3 total)
        6. Present all questions together before waiting for responses
        7. Wait for user to respond with their choices for all questions (e.g., "Q1: A, Q2: Custom - [details], Q3: B")
        8. Update the spec by replacing each [NEEDS CLARIFICATION] marker with the user's selected or provided answer
        9. Re-run validation after all clarifications are resolved

   d. **Update Checklist**: After each validation iteration, update the checklist file with current pass/fail status

7. Report completion with branch name, spec file path, checklist results, and readiness for the next phase (`/speckit.clarify` or `/speckit.plan`).

**NOTE:** The script creates and checks out the new branch and initializes the spec file before writing.

## Patterns: Best Practices for Specification Creation

### Pattern 1: Informed Inference Over Premature Clarification

**Objective**: Produce complete, actionable specifications efficiently by leveraging domain knowledge and industry standards rather than deferring to users for routine decisions.

**Context of application**: Apply during specification generation (Step 4) when encountering unspecified details that have reasonable defaults or can be inferred from context.

**Key characteristics**:

- Maximum 3 [NEEDS CLARIFICATION] markers per specification
- Clarifications reserved for scope-critical, security-sensitive, or user-experience-defining decisions
- Assumptions documented explicitly in Assumptions section
- Defaults based on industry standards for the domain

**Operational guidance**:

1. When encountering unspecified detail, first check: Does this have an industry-standard default? (e.g., session-based auth, RESTful APIs, standard performance expectations)
2. If yes, apply the default and document assumption in spec's Assumptions section
3. If no clear default exists, evaluate impact: Does this significantly affect scope, security, or core UX?
4. Only if high-impact AND multiple conflicting interpretations exist, add [NEEDS CLARIFICATION] marker
5. Track clarification count; if approaching 3, convert lower-priority clarifications to documented assumptions
6. Prioritize by: scope boundaries > security/compliance > user experience > technical preferences

### Pattern 2: Multi-Source Branch Number Reconciliation

**Objective**: Prevent branch number collisions and specification conflicts by checking all sources of truth before assigning new feature numbers.

**Context of application**: Apply during branch creation (Step 2) before running the create-new-feature script.

**Key characteristics**:

- Checks remote branches, local branches, and specs directories
- Fetches latest remote state before analysis
- Uses exact pattern matching for short-name
- Increments from highest found number across all sources

**Operational guidance**:

1. Execute `git fetch --all --prune` to ensure remote branch list is current
2. Search remote branches: `git ls-remote --heads origin | grep -E 'refs/heads/[0-9]+-<short-name>$'`
3. Search local branches: `git branch | grep -E '^[* ]*[0-9]+-<short-name>$'`
4. Check filesystem: Look for `specs/[0-9]+-<short-name>` directories
5. Extract all numbers from all three sources (handle empty results gracefully)
6. If any matches found, use max(all_numbers) + 1; otherwise use 1
7. Pass calculated number to script with `-Number` parameter
8. Verify script output confirms expected branch name before proceeding

### Pattern 3: Technology-Agnostic Success Criteria

**Objective**: Define measurable outcomes that remain valid regardless of implementation approach, enabling architectural flexibility and meaningful progress tracking.

**Context of application**: Apply when defining Success Criteria (Step 4.6) for any feature specification.

**Key characteristics**:

- Criteria describe user-observable outcomes, not system internals
- Metrics are quantitative (time, percentage, count, rate) or qualitative (satisfaction, completion)
- No mention of technologies, frameworks, databases, languages, or tools
- All criteria verifiable through user testing or business metrics

**Operational guidance**:

1. For each success criterion, ask: "Can this be verified without knowing the implementation?"
2. Replace implementation-focused metrics (API response time, cache hit rate, database TPS) with user-facing equivalents (perceived speed, task completion time, concurrent user capacity)
3. Use concrete numbers: "Users complete checkout in under 3 minutes" not "Checkout is fast"
4. Include both performance (quantitative) and quality (qualitative) measures
5. Test criterion against: Would this remain valid if we completely changed the tech stack?
6. If criterion mentions specific technology, reformulate from user/business perspective

### Pattern 4: Iterative Validation with Bounded Remediation

**Objective**: Ensure specification quality through systematic validation while preventing infinite refinement loops.

**Context of application**: Apply during specification quality validation (Step 6) after initial spec generation.

**Key characteristics**:

- Separate checklist file in `checklists/requirements.md`, not embedded in spec
- Automated review against defined quality criteria
- Maximum 3 validation-remediation iterations
- Explicit documentation of persistent issues if iterations exhausted

**Operational guidance**:

1. Generate checklist immediately after writing spec (before user review)
2. Review spec against each checklist item systematically
3. Document specific failures with quoted spec sections as evidence
4. If failures found, update spec to address each specific issue
5. Re-run validation after updates (increment iteration counter)
6. If still failing after iteration 3, document remaining issues in checklist Notes section
7. Warn user about persistent quality issues and recommend manual review
8. Always update checklist file with current pass/fail status after each iteration

### Pattern 5: Structured Clarification with Bounded Options

**Objective**: Resolve specification ambiguities efficiently through guided user choice rather than open-ended questions, while maintaining conversation flow.

**Context of application**: Apply when [NEEDS CLARIFICATION] markers exist in spec (Step 6.c) and user input is required to proceed.

**Key characteristics**:

- Maximum 3 clarification questions per specification
- Each question presents 3 concrete options plus "Custom"
- All questions presented together before waiting for response
- Properly formatted markdown tables for readability
- Sequential numbering (Q1, Q2, Q3)

**Operational guidance**:

1. Extract all [NEEDS CLARIFICATION] markers from spec
2. If more than 3 exist, rank by impact (scope > security > UX) and keep only top 3
3. For dropped clarifications, make informed guess and document in Assumptions
4. For each remaining clarification, formulate as structured question with context quote
5. Generate 3 realistic options (A, B, C) with implications for each
6. Format as markdown table with consistent spacing: `| Option | Answer | Implications |`
7. Ensure header separator has minimum 3 dashes: `|--------|--------|--------------|`
8. Present all questions together (Q1, Q2, Q3) in single response
9. Wait for user response in format "Q1: A, Q2: Custom - [details], Q3: B"
10. Update spec by replacing markers with selected/provided answers
11. Re-run validation to ensure clarifications resolved quality issues

### Pattern 6: Separate Checklist File Structure

**Objective**: Maintain clean separation between specification content and validation artifacts, improving readability and preventing checklist clutter in user-facing documents.

**Context of application**: Apply during spec quality validation (Step 6.a) when creating validation checklists.

**Key characteristics**:

- Checklists live in `FEATURE_DIR/checklists/` directory, not in spec.md
- Each checklist is a standalone markdown file with clear purpose statement
- Checklist links back to spec.md for traceability
- Updates to checklist don't require spec.md changes

**Operational guidance**:

1. Never embed checklist items directly in spec.md file
2. Create `FEATURE_DIR/checklists/requirements.md` as separate file
3. Include checklist metadata: purpose, creation date, link to spec
4. Structure checklist with clear section headers matching validation categories
5. Use standard markdown checkbox syntax: `- [ ]` for incomplete, `- [x]` for complete
6. Update checklist file independently when validation status changes
7. Reference checklist path in completion report to user
8. Keep spec.md focused purely on feature requirements and business value

## Anti-Patterns: Common Mistakes to Avoid

### Anti-Pattern 1: Clarification Overload

**Description**: Marking every unspecified detail with [NEEDS CLARIFICATION] rather than applying domain knowledge and industry standards to fill gaps.

**Reasons to avoid**: Excessive clarification requests burden users with routine decisions they expect the system to handle intelligently. This creates friction in the specification process and signals lack of domain expertise. Users lose confidence when asked to decide obvious defaults like "Should we use user-friendly error messages?" or "Should performance be acceptable?"

**Negative consequences**:

- Specification process becomes tediously interactive rather than efficiently generative
- Users abandon workflow due to decision fatigue from trivial questions
- Obvious industry standards are treated as open questions
- Spec completion time increases dramatically
- User perceives AI as unable to apply common sense or domain knowledge
- Critical clarifications get buried among trivial ones

**Correct alternative**: Apply Pattern 1 (Informed Inference Over Premature Clarification) to use industry standards and reasonable defaults for routine decisions, reserving clarifications for genuinely ambiguous, high-impact choices.

### Anti-Pattern 2: Single-Source Branch Numbering

**Description**: Checking only one source (local branches OR remote branches OR specs directories) when determining the next feature number, leading to number collisions.

**Reasons to avoid**: Different sources of truth can diverge. A feature branch may exist remotely but not locally, or a specs directory may exist from incomplete work that never created a branch. Using partial information guarantees eventual conflicts when multiple sources contain the same short-name with different numbers.

**Negative consequences**:

- Feature number collisions create ambiguous branch names (two features with same number)
- Specs directories get orphaned or overwritten when numbers conflict
- Merge conflicts arise when parallel features use same numbers
- Manual reconciliation required to resolve numbering conflicts
- Team loses trust in automated numbering system
- Git history becomes confusing with duplicate feature numbers

**Correct alternative**: Apply Pattern 2 (Multi-Source Branch Number Reconciliation) to check remote branches, local branches, AND specs directories before assigning any new feature number.

### Anti-Pattern 3: Implementation-Focused Success Criteria

**Description**: Defining success criteria using technical metrics (API response times, database throughput, cache hit rates, framework-specific performance) rather than user-observable outcomes.

**Reasons to avoid**: Implementation-focused criteria lock specifications to particular technical approaches before design exploration occurs. When architecture changes or different technologies are selected during planning, success criteria become invalid or meaningless. Business stakeholders cannot understand or validate technical metrics.

**Negative consequences**:

- Success criteria become obsolete when implementation approach changes
- Specifications leak technical constraints into business requirements
- Non-technical stakeholders cannot evaluate feature success meaningfully
- Architectural flexibility is artificially constrained by premature technical decisions
- Testing and validation require knowledge of internal implementation details
- Metrics focus on system internals rather than user value delivery

**Correct alternative**: Apply Pattern 3 (Technology-Agnostic Success Criteria) to define outcomes from user/business perspective using metrics that remain valid regardless of implementation approach.

### Anti-Pattern 4: Embedded Checklist Pollution

**Description**: Placing validation checklists directly within spec.md file rather than creating separate checklist files in the checklists/ directory.

**Reasons to avoid**: Embedded checklists clutter the specification with meta-content that isn't part of the feature description. Business stakeholders reviewing specs encounter validation artifacts that aren't relevant to understanding feature requirements. Checklist updates require modifying the spec file, creating spurious diffs and version history noise.

**Negative consequences**:

- Spec readability degrades as validation meta-content interrupts feature description
- Business stakeholders confused by checklist items appearing alongside requirements
- Version control diffs show checklist updates mixed with actual spec changes
- Harder to track which spec sections changed vs. which validation items changed
- Templates become inconsistent when some specs have embedded checklists and others don't
- Cannot update validation status without touching spec.md file

**Correct alternative**: Apply Pattern 6 (Separate Checklist File Structure) to create standalone checklist files in `FEATURE_DIR/checklists/` that link back to spec.md for traceability.

### Anti-Pattern 5: Unbounded Validation Iteration

**Description**: Continuously re-running validation and updating specifications in an infinite loop when quality issues persist, without limit on remediation attempts.

**Reasons to avoid**: Some specification issues require human judgment or additional context that automated validation cannot provide. Infinitely iterating wastes resources and may introduce new problems while attempting to fix existing ones. Without bounds, the process can become stuck on edge cases or subjective quality judgments.

**Negative consequences**:

- Process hangs indefinitely on specifications with inherent ambiguity
- Resource exhaustion from repeated validation-update cycles
- Spec quality may degrade as automated fixes introduce new issues
- User waits indefinitely without visibility into process state
- System appears broken or frozen when iteration doesn't converge
- No escape mechanism when validation criteria are too strict or inappropriate

**Correct alternative**: Apply Pattern 4 (Iterative Validation with Bounded Remediation) to limit validation-remediation cycles to 3 iterations, documenting persistent issues for human review if convergence fails.

### Anti-Pattern 6: Malformed Clarification Tables

**Description**: Presenting clarification questions with improperly formatted markdown tables that fail to render correctly due to missing spaces, insufficient dashes, or misaligned pipes.

**Reasons to avoid**: Malformed tables break rendering in markdown viewers, making clarification questions illegible or confusing. Users cannot parse options clearly when table structure collapses. Professional credibility suffers when basic markdown formatting is incorrect.

**Negative consequences**:

- Clarification questions become unreadable when tables fail to render
- Users cannot distinguish between options A, B, and C clearly
- Implications columns merge with answer columns in broken layouts
- Users respond with wrong option due to parsing confusion
- Need to regenerate questions with correct formatting, wasting time
- System appears low-quality when it cannot render basic markdown correctly

**Correct alternative**: Apply Pattern 5 (Structured Clarification with Bounded Options) step 4 guidance to ensure proper table formatting with consistent spacing, minimum 3-dash separators, and aligned pipes.

### Anti-Pattern 7: Script Multi-Execution

**Description**: Running the create-new-feature script multiple times for the same feature, typically due to error handling or retry logic.

**Reasons to avoid**: The script creates branches, initializes directories, and sets up file structure. Multiple executions create duplicate branches with incremented numbers, orphaned spec directories, and confused git state. The script is designed for single execution with idempotent setup operations.

**Negative consequences**:

- Multiple feature branches created for same feature (5-user-auth, 6-user-auth, 7-user-auth)
- Specs directories proliferate with duplicate content
- Git checkout state becomes unpredictable (which branch are we on?)
- Cleanup required to remove duplicate branches and directories
- Feature numbering sequence becomes polluted with gaps
- User confusion about which branch/directory contains current work

**Correct alternative**: Execute the create-new-feature script exactly once per feature. If script fails, diagnose and fix the underlying issue rather than retrying. Verify branch and directory creation success from script JSON output before proceeding.

## General Guidelines

### Quick Guidelines

- Focus on **WHAT** users need and **WHY**.
- Avoid HOW to implement (no tech stack, APIs, code structure).
- Written for business stakeholders, not developers.
- DO NOT embed checklists directly within the spec.md file. Instead, create checklists as separate files in the `checklists/` directory (as specified in step 6.a).

### Section Requirements

- **Mandatory sections**: Must be completed for every feature
- **Optional sections**: Include only when relevant to the feature
- When a section doesn't apply, remove it entirely (don't leave as "N/A")

### For AI Generation

When creating this spec from a user prompt:

1. **Make informed guesses**: Use context, industry standards, and common patterns to fill gaps
2. **Document assumptions**: Record reasonable defaults in the Assumptions section
3. **Limit clarifications**: Maximum 3 [NEEDS CLARIFICATION] markers - use only for critical decisions that:
   - Significantly impact feature scope or user experience
   - Have multiple reasonable interpretations with different implications
   - Lack any reasonable default
4. **Prioritize clarifications**: scope > security/privacy > user experience > technical details
5. **Think like a tester**: Every vague requirement should fail the "testable and unambiguous" checklist item
6. **Common areas needing clarification** (only if no reasonable default exists):
   - Feature scope and boundaries (include/exclude specific use cases)
   - User types and permissions (if multiple conflicting interpretations possible)
   - Security/compliance requirements (when legally/financially significant)

**Examples of reasonable defaults** (don't ask about these):

- Data retention: Industry-standard practices for the domain
- Performance targets: Standard web/mobile app expectations unless specified
- Error handling: User-friendly messages with appropriate fallbacks
- Authentication method: Standard session-based or OAuth2 for web apps
- Integration patterns: RESTful APIs unless specified otherwise

### Success Criteria Guidelines

Success criteria must be:

1. **Measurable**: Include specific metrics (time, percentage, count, rate)
2. **Technology-agnostic**: No mention of frameworks, languages, databases, or tools
3. **User-focused**: Describe outcomes from user/business perspective, not system internals
4. **Verifiable**: Can be tested/validated without knowing implementation details

**Good examples**:

- "Users can complete checkout in under 3 minutes"
- "System supports 10,000 concurrent users"
- "95% of searches return results in under 1 second"
- "Task completion rate improves by 40%"

**Bad examples** (implementation-focused):

- "API response time is under 200ms" (too technical, use "Users see results instantly")
- "Database can handle 1000 TPS" (implementation detail, use user-facing metric)
- "React components render efficiently" (framework-specific)
- "Redis cache hit rate above 80%" (technology-specific)
