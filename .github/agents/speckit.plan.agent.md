---
description: Execute the implementation planning workflow using the plan template to generate design artifacts.
handoffs: 
  - label: Create Tasks
    agent: speckit.tasks
    prompt: Break the plan into tasks
    send: true
  - label: Create Checklist
    agent: speckit.checklist
    prompt: Create a checklist for the following domain...
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

1. **Setup**: Run `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/setup-plan.ps1 -Json` from repo root and parse JSON for FEATURE_SPEC, IMPL_PLAN, SPECS_DIR, BRANCH. For single quotes in args like "I'm Groot", use PowerShell escape syntax: double the quote inside single-quoted strings (e.g., 'I''m Groot') or use double quotes (e.g., "I'm Groot").

2. **Load context**: Read FEATURE_SPEC and `.specify/memory/constitution.md`. Load IMPL_PLAN template (already copied by Setup step).

3. **Execute plan workflow**: Follow the structure in IMPL_PLAN template to:
   - Fill Technical Context (mark unknowns as "NEEDS CLARIFICATION")
   - Fill Constitution Check section from constitution
   - Evaluate gates (ERROR if violations unjustified)
   - Phase 0: Generate research.md (resolve all NEEDS CLARIFICATION)
   - Phase 1: Generate data-model.md, contracts/, quickstart.md
   - Phase 1: Update agent context by running the agent script
   - Re-evaluate Constitution Check post-design

4. **Stop and report**: Command ends after Phase 2 planning. Report branch, IMPL_PLAN path, and generated artifacts.

## Phases

### Phase 0: Outline & Research

1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:

   ```text
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

### Phase 1: Design & Contracts

**Prerequisites:** `research.md` complete

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Agent context update**:
   - Run `.specify/scripts/powershell/update-agent-context.ps1 -AgentType qwen`
   - These scripts detect which AI agent is in use
   - Update the appropriate agent-specific context file
   - Add only new technology from current plan
   - Preserve manual additions between markers

**Output**: data-model.md, /contracts/*, quickstart.md, agent-specific file

## Patterns: Best Practices for Implementation Planning

### Pattern 1: Explicit Clarification Tracking

**Objective**: Prevent proceeding with incomplete or ambiguous information that could lead to implementation failures.

**Context of application**: Apply during Technical Context gathering (Step 3) and throughout all phases when encountering undefined requirements, unclear dependencies, or ambiguous specifications.

**Key characteristics**:

- Unknowns are explicitly marked with "NEEDS CLARIFICATION" rather than making assumptions
- Each clarification item is tracked through to resolution in Phase 0
- Gate evaluation blocks progress until all clarifications are resolved

**Operational guidance**:

1. During Technical Context analysis, flag every uncertainty with "NEEDS CLARIFICATION" tag
2. Document the specific question or unknown (not just "unclear")
3. Convert each flagged item into a discrete research task in Phase 0
4. Verify in research.md that each clarification has a documented decision and rationale
5. Re-check Technical Context to ensure all "NEEDS CLARIFICATION" markers are removed before Phase 1

### Pattern 2: Constitution-Driven Design Gates

**Objective**: Ensure architectural and design decisions align with project principles and constraints before committing to implementation.

**Context of application**: Use at constitution check evaluation (Step 3), after initial design, and post-design re-evaluation.

**Key characteristics**:

- Constitution is loaded as authoritative source for project constraints
- Violations trigger ERROR state rather than warnings
- Gates require justification for any deviations
- Post-design re-evaluation catches drift introduced during artifact generation

**Operational guidance**:

1. Load `.specify/memory/constitution.md` at workflow start
2. Document each constitution principle that applies to current feature
3. For each design decision in Technical Context, explicitly check against applicable principles
4. If violation is necessary, document justification before proceeding
5. After Phase 1 artifact generation, re-run constitution check to catch emergent violations
6. Treat unjustified violations as blocking errors that halt the workflow

### Pattern 3: Research-First Design Approach

**Objective**: Ground all design decisions in researched alternatives and documented rationale rather than assumptions or defaults.

**Context of application**: Apply in Phase 0 before any artifact generation, especially when selecting technologies, patterns, or architectural approaches.

**Key characteristics**:

- Every "NEEDS CLARIFICATION" generates a research task
- Research tasks explicitly evaluate alternatives
- Decisions are documented with rationale and rejected alternatives
- Technology choices include best practices research

**Operational guidance**:

1. Extract all uncertainties from Technical Context into discrete research tasks
2. For each task, formulate as a research question (not just "learn about X")
3. Research must produce: chosen decision, rationale for choice, alternatives considered
4. Document findings in structured research.md before proceeding to Phase 1
5. Reference research.md decisions when generating artifacts in Phase 1
6. Ensure research tasks cover both functional unknowns and technology best practices

### Pattern 4: Incremental Agent Context Preservation

**Objective**: Maintain continuity of agent knowledge across planning sessions while avoiding context pollution from outdated information.

**Context of application**: Apply during agent context updates (Phase 1, Step 3) when new technologies or patterns are introduced.

**Key characteristics**:

- Only new technology from current plan is added
- Manual additions are preserved between designated markers
- Agent-specific context files are updated programmatically
- Updates are incremental rather than complete replacements

**Operational guidance**:

1. Run update script only after new artifacts are generated
2. Use `-AgentType` parameter to target correct agent configuration
3. Verify script detects current AI agent automatically
4. Check that manual additions between markers remain intact
5. Review diff to confirm only current plan's technologies were added
6. Commit context updates with artifact commits for traceability

### Pattern 5: Absolute Path Discipline

**Objective**: Eliminate path resolution errors and ensure scripts execute correctly regardless of working directory context.

**Context of application**: Apply when referencing files, executing scripts, or generating artifacts throughout all workflow phases.

**Key characteristics**:

- All file references use absolute paths
- Script execution specifies full path from repository root
- Generated artifacts are written to absolute paths
- Path construction accounts for cross-platform differences

**Operational guidance**:

1. Always execute setup script from repository root: `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/setup-plan.ps1`
2. Parse JSON output to obtain absolute paths (FEATURE_SPEC, IMPL_PLAN, SPECS_DIR)
3. Use parsed absolute paths for all subsequent file operations
4. When generating artifacts, construct paths as: `{SPECS_DIR}/{artifact_name}`
5. Avoid relative paths like `../` or `./` in automation scripts
6. Verify path existence before write operations to fail fast on configuration issues

## Anti-Patterns: Common Mistakes to Avoid

### Anti-Pattern 1: Assumption-Driven Planning

**Description**: Proceeding with design and artifact generation while unknowns remain unresolved, filling gaps with assumptions rather than research.

**Reasons to avoid**: Assumptions made early in planning become embedded in contracts, data models, and quickstart guides, requiring costly rework when revealed incorrect. Downstream implementers inherit these assumptions without visibility into their uncertain foundation.

**Negative consequences**:

- Generated contracts may not match actual integration requirements
- Data models miss critical fields or relationships discovered later
- Implementation work begins on incorrect foundation, wasting development time
- Rework cascades through all Phase 1 artifacts when assumptions prove false
- Team loses confidence in planning process credibility

**Correct alternative**: Apply Pattern 1 (Explicit Clarification Tracking) to mark all unknowns and resolve them through research in Phase 0 before generating any artifacts.

### Anti-Pattern 2: Gate Bypassing

**Description**: Treating constitution violations or gate failures as warnings rather than blocking errors, allowing work to proceed despite unjustified deviations from project principles.

**Reasons to avoid**: Gates exist to prevent accumulation of technical debt and architectural inconsistency. Bypassing them creates precedent for ignoring constraints, leading to fragmented architecture where different features follow incompatible patterns.

**Negative consequences**:

- Constitution becomes symbolic rather than enforced, undermining governance
- Architectural drift accumulates silently across features
- Integration complexity increases as features use incompatible patterns
- Post-implementation remediation is expensive and disruptive
- Team debates repeat for each feature rather than referencing established principles

**Correct alternative**: Apply Pattern 2 (Constitution-Driven Design Gates) to treat violations as ERROR states that halt workflow until justified or resolved.

### Anti-Pattern 3: Design-Then-Research Workflow

**Description**: Generating data models, contracts, and other artifacts first, then conducting research to validate decisions afterward (if at all).

**Reasons to avoid**: Artifacts created before research encode initial intuitions rather than informed decisions. When research contradicts the design, either costly rework occurs or research findings are ignored to avoid rework, defeating the purpose of research.

**Negative consequences**:

- Artifacts become outdated immediately upon research completion
- Confirmation bias encourages ignoring research that contradicts existing design
- Best practices for chosen technologies are applied inconsistently or not at all
- Alternative technologies that research reveals as superior cannot be adopted without scrapping work
- Research becomes compliance theater rather than decision input

**Correct alternative**: Apply Pattern 3 (Research-First Design Approach) to complete all research and resolve unknowns in Phase 0 before beginning artifact generation in Phase 1.

### Anti-Pattern 4: Context File Overwriting

**Description**: Replacing agent context files entirely with each update rather than incrementally adding new technologies while preserving manual additions and historical context.

**Reasons to avoid**: Complete replacement loses manual refinements, project-specific conventions, and accumulated agent learning. Team members who carefully curated context files for optimal agent performance see their work erased, creating frustration and reducing context quality over time.

**Negative consequences**:

- Manual improvements to agent prompts and context are lost
- Agent performance regresses as curated context disappears
- Team stops investing effort in context refinement since it won't persist
- No historical record of what technologies were added when
- Context files lack project-specific conventions that aren't in templates

**Correct alternative**: Apply Pattern 4 (Incremental Agent Context Preservation) to add only new technologies while preserving content between designated markers.

### Anti-Pattern 5: Relative Path Dependencies

**Description**: Using relative paths (`../`, `./`) in scripts and file references, assuming a particular working directory context that may not hold across different execution environments or automation systems.

**Reasons to avoid**: Relative paths break when scripts are executed from different directories, when working directories change during execution, or when automation systems have different directory conventions. Debugging path resolution failures is time-consuming and frustrating.

**Negative consequences**:

- Scripts fail with cryptic "file not found" errors in CI/CD systems
- Manual execution from different directories produces different results
- Path resolution errors only surface in specific environments, not during testing
- Team members waste time debugging path issues rather than feature work
- Workarounds accumulate (cd commands, environment variables) that obscure actual paths

**Correct alternative**: Apply Pattern 5 (Absolute Path Discipline) to use absolute paths obtained from setup script JSON output for all file operations.

### Anti-Pattern 6: Single-Pass Constitution Checking

**Description**: Evaluating constitution compliance only at the beginning of the workflow without re-checking after artifact generation, assuming design decisions won't introduce new violations.

**Reasons to avoid**: Constitution violations often emerge during artifact generation when abstract designs become concrete implementations. Contract details, data model constraints, and technology choices made during Phase 1 may violate principles that seemed satisfied in initial Technical Context.

**Negative consequences**:

- Violations discovered post-implementation require expensive refactoring
- Generated artifacts may be shared or used before violations are caught
- No systematic mechanism catches emergent violations before code is written
- Constitution becomes pre-flight checklist rather than continuous constraint
- Teams develop habits of checking compliance only at project start

**Correct alternative**: Re-evaluate Constitution Check after Phase 1 artifact generation (as specified in Step 3) to catch violations introduced during design elaboration.

## Key rules

- Use absolute paths
- ERROR on gate failures or unresolved clarifications
