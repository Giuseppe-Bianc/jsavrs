---
description: Generate an actionable, dependency-ordered tasks.md for the feature based on available design artifacts.
handoffs: 
  - label: Analyze For Consistency
    agent: speckit.analyze
    prompt: Run a project analysis for consistency
    send: true
  - label: Implement Project
    agent: speckit.implement
    prompt: Start the implementation in phases
    send: true
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

1. **Setup**: Run `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/check-prerequisites.ps1 -Json` from repo root and parse FEATURE_DIR and AVAILABLE_DOCS list. All paths must be absolute. For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").

2. **Load design documents**: Read from FEATURE_DIR:
   - **Required**: plan.md (tech stack, libraries, structure), spec.md (user stories with priorities)
   - **Optional**: data-model.md (entities), contracts/ (API endpoints), research.md (decisions), quickstart.md (test scenarios)
   - Note: Not all projects have all documents. Generate tasks based on what's available.

3. **Execute task generation workflow**:
   - Load plan.md and extract tech stack, libraries, project structure
   - Load spec.md and extract user stories with their priorities (P1, P2, P3, etc.)
   - If data-model.md exists: Extract entities and map to user stories
   - If contracts/ exists: Map endpoints to user stories
   - If research.md exists: Extract decisions for setup tasks
   - Generate tasks organized by user story (see Task Generation Rules below)
   - Generate dependency graph showing user story completion order
   - Create parallel execution examples per user story
   - Validate task completeness (each user story has all needed tasks, independently testable)

4. **Generate tasks.md**: Use `.specify/templates/tasks-template.md` as structure, fill with:
   - Correct feature name from plan.md
   - Phase 1: Setup tasks (project initialization)
   - Phase 2: Foundational tasks (blocking prerequisites for all user stories)
   - Phase 3+: One phase per user story (in priority order from spec.md)
   - Each phase includes: story goal, independent test criteria, tests (if requested), implementation tasks
   - Final Phase: Polish & cross-cutting concerns
   - All tasks must follow the strict checklist format (see Task Generation Rules below)
   - Clear file paths for each task
   - Dependencies section showing story completion order
   - Parallel execution examples per story
   - Implementation strategy section (MVP first, incremental delivery)

5. **Report**: Output path to generated tasks.md and summary:
   - Total task count
   - Task count per user story
   - Parallel opportunities identified
   - Independent test criteria for each story
   - Suggested MVP scope (typically just User Story 1)
   - Format validation: Confirm ALL tasks follow the checklist format (checkbox, ID, labels, file paths)

Context for task generation: $ARGUMENTS

The tasks.md should be immediately executable - each task must be specific enough that an LLM can complete it without additional context.

## Task Generation Rules

**CRITICAL**: Tasks MUST be organized by user story to enable independent implementation and testing.

**Tests are OPTIONAL**: Only generate test tasks if explicitly requested in the feature specification or if user requests TDD approach.

### Checklist Format (REQUIRED)

Every task MUST strictly follow this format:

```text
- [ ] [TaskID] [P?] [Story?] Description with file path
```

**Format Components**:

1. **Checkbox**: ALWAYS start with `- [ ]` (markdown checkbox)
2. **Task ID**: Sequential number (T001, T002, T003...) in execution order
3. **[P] marker**: Include ONLY if task is parallelizable (different files, no dependencies on incomplete tasks)
4. **[Story] label**: REQUIRED for user story phase tasks only
   - Format: [US1], [US2], [US3], etc. (maps to user stories from spec.md)
   - Setup phase: NO story label
   - Foundational phase: NO story label  
   - User Story phases: MUST have story label
   - Polish phase: NO story label
5. **Description**: Clear action with exact file path

**Examples**:

- ✅ CORRECT: `- [ ] T001 Create project structure per implementation plan`
- ✅ CORRECT: `- [ ] T005 [P] Implement authentication middleware in src/middleware/auth.py`
- ✅ CORRECT: `- [ ] T012 [P] [US1] Create User model in src/models/user.py`
- ✅ CORRECT: `- [ ] T014 [US1] Implement UserService in src/services/user_service.py`
- ❌ WRONG: `- [ ] Create User model` (missing ID and Story label)
- ❌ WRONG: `T001 [US1] Create model` (missing checkbox)
- ❌ WRONG: `- [ ] [US1] Create User model` (missing Task ID)
- ❌ WRONG: `- [ ] T001 [US1] Create model` (missing file path)

### Task Organization

1. **From User Stories (spec.md)** - PRIMARY ORGANIZATION:
   - Each user story (P1, P2, P3...) gets its own phase
   - Map all related components to their story:
     - Models needed for that story
     - Services needed for that story
     - Endpoints/UI needed for that story
     - If tests requested: Tests specific to that story
   - Mark story dependencies (most stories should be independent)

2. **From Contracts**:
   - Map each contract/endpoint → to the user story it serves
   - If tests requested: Each contract → contract test task [P] before implementation in that story's phase

3. **From Data Model**:
   - Map each entity to the user story(ies) that need it
   - If entity serves multiple stories: Put in earliest story or Setup phase
   - Relationships → service layer tasks in appropriate story phase

4. **From Setup/Infrastructure**:
   - Shared infrastructure → Setup phase (Phase 1)
   - Foundational/blocking tasks → Foundational phase (Phase 2)
   - Story-specific setup → within that story's phase

### Phase Structure

- **Phase 1**: Setup (project initialization)
- **Phase 2**: Foundational (blocking prerequisites - MUST complete before user stories)
- **Phase 3+**: User Stories in priority order (P1, P2, P3...)
  - Within each story: Tests (if requested) → Models → Services → Endpoints → Integration
  - Each phase should be a complete, independently testable increment
- **Final Phase**: Polish & Cross-Cutting Concerns


## Patterns: Best Practices for Task Generation

### Pattern 1: User Story-Centric Organization

**Objective**: Enable independent, parallel development and testing of features by organizing tasks around user stories rather than technical layers.

**Context of Application**: When generating tasks.md from design artifacts where multiple user stories exist with different priorities. Apply this pattern whenever spec.md contains P1, P2, P3+ user stories.

**Key Characteristics**:

- Each user story maps to a dedicated phase in the task list
- All components needed for a story (models, services, endpoints) are grouped within that story's phase
- Stories can be implemented independently without waiting for unrelated stories to complete
- Each story phase represents a testable, deliverable increment

**Operational Guidance**:

1. Parse spec.md to extract all user stories with their priority levels
2. Create one phase per user story, ordered by priority (P1 first, then P2, P3, etc.)
3. For each story, trace backward through all design artifacts to identify required components
4. Assign each component (model, service, endpoint) to the story that needs it
5. If a component serves multiple stories, place it in the earliest story or foundational phase
6. Ensure each story phase includes an independent test criteria section
7. Mark dependencies explicitly only when one story genuinely depends on another's completion

### Pattern 2: Strict Format Compliance

**Objective**: Ensure all tasks are machine-parseable and can be tracked, filtered, and analyzed programmatically.

**Context of Application**: Every single task in the generated tasks.md must follow the checklist format without exception. Apply to all phases: Setup, Foundational, User Story, and Polish.

**Key Characteristics**:

- Consistent checkbox prefix for tracking completion state
- Sequential task IDs enabling dependency references and progress tracking
- Explicit parallelizability markers for workflow optimization
- Story labels providing traceability to requirements
- File paths making tasks immediately actionable

**Operational Guidance**:

1. Always begin each task with `- [ ]` (markdown checkbox with space)
2. Assign sequential IDs starting from T001, incrementing by 1
3. Add `[P]` marker only when task can run in parallel with others (different files, no blocking dependencies)
4. Add story label `[US#]` for all tasks in user story phases (Phase 3+)
5. Omit story labels for Setup, Foundational, and Polish phases
6. Include specific file path in task description (e.g., `in src/models/user.py`)
7. Run format validation before finalizing: check every task has checkbox, ID, appropriate labels, and file path

### Pattern 3: Dependency Layering

**Objective**: Prevent blocking issues by establishing clear layers of dependencies and ensuring foundational tasks complete before dependent work begins.

**Context of Application**: When design artifacts reveal shared infrastructure, common utilities, or cross-cutting concerns that multiple user stories depend on.

**Key Characteristics**:

- Setup phase contains project initialization with no external dependencies
- Foundational phase contains blocking prerequisites needed by all or most user stories
- User story phases depend only on Setup/Foundational completion, not on each other (unless explicitly required)
- Shared resources are identified early and placed in appropriate foundation layers

**Operational Guidance**:

1. Identify truly shared infrastructure from plan.md and research.md (e.g., database setup, authentication framework, logging)
2. Place project initialization tasks in Phase 1 (Setup)
3. Place blocking prerequisites in Phase 2 (Foundational) - these MUST complete before any user story work
4. Keep user story phases (Phase 3+) independent unless there's a genuine business dependency
5. Generate dependency graph showing which phases must complete before others can start
6. Validate that foundational tasks are truly foundational - avoid putting story-specific work in this phase
7. Document parallel execution opportunities within the dependency structure

### Pattern 4: Granular Task Definition

**Objective**: Create tasks specific enough that an LLM or developer can execute them without requiring additional context or clarification.

**Context of Application**: Every task in tasks.md, regardless of complexity. Particularly critical when generating tasks that will be executed by automated agents or distributed teams.

**Key Characteristics**:

- Each task describes a single, concrete action
- File paths are explicit and complete
- Expected inputs and outputs are clear from the description
- No ambiguous verbs like "handle," "manage," or "deal with"
- Tasks reference specific design artifacts when applicable

**Operational Guidance**:

1. Use specific action verbs: "Create," "Implement," "Define," "Configure," "Connect," "Validate"
2. Include exact file path in every implementation task
3. Reference specific sections of design documents when relevant (e.g., "per data-model.md User entity")
4. Break complex tasks into multiple atomic tasks if a single task would require multiple files or steps
5. Specify the artifact type (e.g., "Create User model class" not just "Create User")
6. For configuration tasks, specify what is being configured and where (e.g., "Configure database connection in config/database.py")
7. Ensure task description alone provides sufficient context - avoid relying on phase headers for critical information

### Pattern 5: Incremental Deliverability

**Objective**: Structure tasks so each phase produces a working, testable increment that delivers value independently.

**Context of Application**: When organizing user story phases and defining what constitutes "done" for each phase. Essential for MVP planning and iterative delivery.

**Key Characteristics**:

- Each user story phase includes all layers needed for that story (model, service, endpoint/UI)
- Independent test criteria defined per story
- Earlier phases (lower priority user stories) deliver subset of functionality
- MVP scope clearly identified (typically User Story 1 only)
- Each phase can be demonstrated and validated without waiting for subsequent phases

**Operational Guidance**:

1. For each user story phase, include tasks spanning full stack: data layer → business logic → presentation/API
2. Define independent test criteria showing how to verify the story works in isolation
3. Order user stories by priority so Phase 3 = P1 story, Phase 4 = P2 story, etc.
4. Mark MVP scope in the report section (usually just the first user story)
5. Ensure each story phase is a complete vertical slice, not a horizontal layer
6. Generate test tasks (if requested) that validate story completion independently
7. Document what value each phase delivers to enable stakeholder validation

## Anti-Patterns: Common Mistakes to Avoid

### Anti-Pattern 1: Layer-Based Organization

**Description**: Organizing tasks by technical layers (all models together, all services together, all endpoints together) instead of by user stories.

**Reasons to Avoid**: Layer-based organization creates artificial dependencies, prevents parallel development, obscures the relationship between code and business value, and makes it impossible to deliver working increments until all layers are complete.

**Negative Consequences**:

- Cannot deliver any user-facing functionality until all layers are built
- Teams must wait for each other unnecessarily (frontend blocked on backend, etc.)
- No incremental value delivery - it's all or nothing
- Difficult to prioritize work based on business value
- Testing can only happen at the very end when all layers integrate
- Higher risk of integration failures discovered late in development

**Correct Alternative**: Use Pattern 1 (User Story-Centric Organization) to group all components needed for each story together in dedicated phases.

### Anti-Pattern 2: Inconsistent Task Format

**Description**: Tasks that mix different formats, omit required components (checkboxes, IDs, file paths), or use custom formatting that breaks parseability.

**Reasons to Avoid**: Inconsistent formatting prevents automated tracking, makes it impossible to programmatically filter or analyze tasks, creates confusion about task status, and reduces the document's utility as a project management tool.

**Negative Consequences**:

- Cannot programmatically track progress or generate reports
- Tools cannot parse tasks to show completion percentages or identify bottlenecks
- Difficult to filter tasks by story, parallelizability, or other criteria
- Team members may interpret task structure differently
- Integration with project management tools fails
- Manual effort required to track what's actually complete

**Correct Alternative**: Use Pattern 2 (Strict Format Compliance) and apply format validation to ensure every task follows the required structure.

### Anti-Pattern 3: Missing or Circular Dependencies

**Description**: Failing to identify foundational dependencies, creating circular dependencies between user stories, or marking all tasks as parallelizable without considering actual constraints.

**Reasons to Avoid**: Missing dependencies cause build failures and blocked work. Circular dependencies make the project impossible to execute. Incorrectly marking tasks as parallel creates race conditions and integration issues.

**Negative Consequences**:

- Developers attempt tasks before prerequisites are ready, causing rework
- Build and runtime failures due to missing infrastructure
- Wasted effort when parallel tasks conflict or overwrite each other's work
- Cannot determine a valid execution order
- Integration issues from assumptions about what exists
- Team frustration from constantly blocked work

**Correct Alternative**: Use Pattern 3 (Dependency Layering) to establish clear foundational phases and validate that dependencies flow in one direction only.

### Anti-Pattern 4: Vague or Ambiguous Tasks

**Description**: Tasks that lack specific file paths, use unclear verbs, combine multiple unrelated actions, or rely on implicit context that isn't documented.

**Reasons to Avoid**: Vague tasks require constant clarification, lead to inconsistent implementation across the team, cannot be executed by automated agents, and increase the cognitive load on developers who must guess at the intent.

**Negative Consequences**:

- Developers spend time asking for clarification instead of implementing
- Different interpretations lead to inconsistent architecture
- LLM-based agents cannot execute tasks autonomously
- Rework required when assumptions prove incorrect
- Difficulty reviewing code when expected outcome was unclear
- Project delays from constant back-and-forth on task intent

**Correct Alternative**: Use Pattern 4 (Granular Task Definition) to create specific, actionable tasks with explicit file paths and clear expected outcomes.

### Anti-Pattern 5: Big-Bang Integration

**Description**: Structuring tasks so nothing can be tested or validated until all work is complete, with no intermediate deliverables or integration points.

**Reasons to Avoid**: Big-bang integration defers risk to the end, prevents early validation with stakeholders, provides no opportunity for course correction, and maximizes the cost of discovering problems.

**Negative Consequences**:

- Integration issues discovered only at project end when fixing is most expensive
- No stakeholder feedback until everything is built (may build wrong thing)
- Cannot demonstrate progress or value during development
- High risk of catastrophic failure at integration time
- Team morale suffers from lack of visible progress
- No opportunity to validate assumptions early
- Impossible to ship incrementally or pivot based on learning

**Correct Alternative**: Use Pattern 5 (Incremental Deliverability) to structure each phase as a complete, testable vertical slice that delivers independent value.

### Anti-Pattern 6: Inappropriate Foundational Tasks

**Description**: Placing story-specific implementation details in the Foundational phase, or conversely, leaving genuinely shared infrastructure in user story phases.

**Reasons to Avoid**: Misclassifying task dependencies creates false blocking relationships, delays user story work unnecessarily, or causes duplication when shared code is implemented multiple times in different stories.

**Negative Consequences**:

- User stories blocked waiting for "foundational" work that's actually story-specific
- Duplicated effort when shared infrastructure is rebuilt in each story
- Foundational phase becomes a dumping ground that never completes
- Parallel execution opportunities missed due to artificial serialization
- Difficulty maintaining shared code scattered across story phases
- Confusion about what truly must complete before story work begins

**Correct Alternative**: Apply Pattern 3 (Dependency Layering) criteria: Foundational phase contains only work that blocks multiple user stories. Story-specific setup goes in that story's phase, even if it feels "low-level."

### Anti-Pattern 7: Test-Last Thinking

**Description**: When tests are requested, placing all test tasks at the end of phases or in a separate testing phase, rather than integrating test tasks before or alongside implementation tasks.

**Reasons to Avoid**: Test-last approaches delay defect discovery, reduce test effectiveness as a design tool, create a separate "testing phase" that becomes a bottleneck, and miss the benefits of test-driven development.

**Negative Consequences**:

- Defects discovered late when fixing is expensive
- Tests written to match implementation rather than validate requirements
- Testing becomes a gate that blocks delivery rather than enabling it
- Lost opportunity to use tests as executable specifications
- Temptation to skip tests when "running out of time"
- Poor test coverage as tests are rushed at the end

**Correct Alternative**: When tests are requested, generate test tasks within each user story phase, before the implementation tasks they validate. Follow the within-phase order: Tests → Models → Services → Endpoints → Integration.
