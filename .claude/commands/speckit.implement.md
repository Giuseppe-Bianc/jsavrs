---
description: Execute the implementation plan by processing and executing all tasks defined in tasks.md
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

1. Run `pwsh -ExecutionPolicy Bypass -File .specify/scripts/powershell/check-prerequisites.ps1 -Json -RequireTasks -IncludeTasks` from repo root and parse FEATURE_DIR and AVAILABLE_DOCS list. All paths must be absolute. For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").

2. **Check checklists status** (if FEATURE_DIR/checklists/ exists):
   - Scan all checklist files in the checklists/ directory
   - For each checklist, count:
     - Total items: All lines matching `- [ ]` or `- [X]` or `- [x]`
     - Completed items: Lines matching `- [X]` or `- [x]`
     - Incomplete items: Lines matching `- [ ]`
   - Create a status table:

     ```text
     | Checklist | Total | Completed | Incomplete | Status |
     |-----------|-------|-----------|------------|--------|
     | ux.md     | 12    | 12        | 0          | ✓ PASS |
     | test.md   | 8     | 5         | 3          | ✗ FAIL |
     | security.md | 6   | 6         | 0          | ✓ PASS |
     ```

   - Calculate overall status:
     - **PASS**: All checklists have 0 incomplete items
     - **FAIL**: One or more checklists have incomplete items

   - **If any checklist is incomplete**:
     - Display the table with incomplete item counts
     - **STOP** and ask: "Some checklists are incomplete. Do you want to proceed with implementation anyway? (yes/no)"
     - Wait for user response before continuing
     - If user says "no" or "wait" or "stop", halt execution
     - If user says "yes" or "proceed" or "continue", proceed to step 3

   - **If all checklists are complete**:
     - Display the table showing all checklists passed
     - Automatically proceed to step 3

3. Load and analyze the implementation context:
   - **REQUIRED**: Read tasks.md for the complete task list and execution plan
   - **REQUIRED**: Read plan.md for tech stack, architecture, and file structure
   - **IF EXISTS**: Read data-model.md for entities and relationships
   - **IF EXISTS**: Read contracts/ for API specifications and test requirements
   - **IF EXISTS**: Read research.md for technical decisions and constraints
   - **IF EXISTS**: Read quickstart.md for integration scenarios

4. **Project Setup Verification**:
   - **REQUIRED**: Create/verify ignore files based on actual project setup:

   **Detection & Creation Logic**:
   - Check if the following command succeeds to determine if the repository is a git repo (create/verify .gitignore if so):

     ```sh
     git rev-parse --git-dir 2>/dev/null
     ```

   - Check if Dockerfile* exists or Docker in plan.md → create/verify .dockerignore
   - Check if .eslintrc* exists → create/verify .eslintignore
   - Check if eslint.config.* exists → ensure the config's `ignores` entries cover required patterns
   - Check if .prettierrc* exists → create/verify .prettierignore
   - Check if .npmrc or package.json exists → create/verify .npmignore (if publishing)
   - Check if terraform files (*.tf) exist → create/verify .terraformignore
   - Check if .helmignore needed (helm charts present) → create/verify .helmignore

   **If ignore file already exists**: Verify it contains essential patterns, append missing critical patterns only
   **If ignore file missing**: Create with full pattern set for detected technology

   **Common Patterns by Technology** (from plan.md tech stack):
   - **Node.js/JavaScript/TypeScript**: `node_modules/`, `dist/`, `build/`, `*.log`, `.env*`
   - **Python**: `__pycache__/`, `*.pyc`, `.venv/`, `venv/`, `dist/`, `*.egg-info/`
   - **Java**: `target/`, `*.class`, `*.jar`, `.gradle/`, `build/`
   - **C#/.NET**: `bin/`, `obj/`, `*.user`, `*.suo`, `packages/`
   - **Go**: `*.exe`, `*.test`, `vendor/`, `*.out`
   - **Ruby**: `.bundle/`, `log/`, `tmp/`, `*.gem`, `vendor/bundle/`
   - **PHP**: `vendor/`, `*.log`, `*.cache`, `*.env`
   - **Rust**: `target/`, `debug/`, `release/`, `*.rs.bk`, `*.rlib`, `*.prof*`, `.idea/`, `*.log`, `.env*`
   - **Kotlin**: `build/`, `out/`, `.gradle/`, `.idea/`, `*.class`, `*.jar`, `*.iml`, `*.log`, `.env*`
   - **C++**: `build/`, `bin/`, `obj/`, `out/`, `*.o`, `*.so`, `*.a`, `*.exe`, `*.dll`, `.idea/`, `*.log`, `.env*`
   - **C**: `build/`, `bin/`, `obj/`, `out/`, `*.o`, `*.a`, `*.so`, `*.exe`, `Makefile`, `config.log`, `.idea/`, `*.log`, `.env*`
   - **Swift**: `.build/`, `DerivedData/`, `*.swiftpm/`, `Packages/`
   - **R**: `.Rproj.user/`, `.Rhistory`, `.RData`, `.Ruserdata`, `*.Rproj`, `packrat/`, `renv/`
   - **Universal**: `.DS_Store`, `Thumbs.db`, `*.tmp`, `*.swp`, `.vscode/`, `.idea/`

   **Tool-Specific Patterns**:
   - **Docker**: `node_modules/`, `.git/`, `Dockerfile*`, `.dockerignore`, `*.log*`, `.env*`, `coverage/`
   - **ESLint**: `node_modules/`, `dist/`, `build/`, `coverage/`, `*.min.js`
   - **Prettier**: `node_modules/`, `dist/`, `build/`, `coverage/`, `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`
   - **Terraform**: `.terraform/`, `*.tfstate*`, `*.tfvars`, `.terraform.lock.hcl`
   - **Kubernetes/k8s**: `*.secret.yaml`, `secrets/`, `.kube/`, `kubeconfig*`, `*.key`, `*.crt`

5. Parse tasks.md structure and extract:
   - **Task phases**: Setup, Tests, Core, Integration, Polish
   - **Task dependencies**: Sequential vs parallel execution rules
   - **Task details**: ID, description, file paths, parallel markers [P]
   - **Execution flow**: Order and dependency requirements

6. Execute implementation following the task plan:
   - **Phase-by-phase execution**: Complete each phase before moving to the next
   - **Respect dependencies**: Run sequential tasks in order, parallel tasks [P] can run together  
   - **Follow TDD approach**: Execute test tasks before their corresponding implementation tasks
   - **File-based coordination**: Tasks affecting the same files must run sequentially
   - **Validation checkpoints**: Verify each phase completion before proceeding

7. Implementation execution rules:
   - **Setup first**: Initialize project structure, dependencies, configuration
   - **Tests before code**: If you need to write tests for contracts, entities, and integration scenarios
   - **Core development**: Implement models, services, CLI commands, endpoints
   - **Integration work**: Database connections, middleware, logging, external services
   - **Polish and validation**: Unit tests, performance optimization, documentation

8. Progress tracking and error handling:
   - Report progress after each completed task
   - Halt execution if any non-parallel task fails
   - For parallel tasks [P], continue with successful tasks, report failed ones
   - Provide clear error messages with context for debugging
   - Suggest next steps if implementation cannot proceed
   - **IMPORTANT** For completed tasks, make sure to mark the task off as [X] in the tasks file.

9. Completion validation:
   - Verify all required tasks are completed
   - Check that implemented features match the original specification
   - Validate that tests pass and coverage meets requirements
   - Confirm the implementation follows the technical plan
   - Report final status with summary of completed work

## Patterns: Best Practices for Implementation Execution

### Pattern: Gated Readiness Validation

**Objective:** Prevent implementation of incomplete or under-validated features by requiring explicit checklist completion or informed override before code execution.

**Context of Application:** Before loading implementation context or executing any tasks, when checklists exist in the feature directory.

**Key Characteristics:**

- All checklists scanned and status aggregated into pass/fail table
- Incomplete checklists trigger blocking prompt requiring user acknowledgment
- User retains override authority for exploratory or spike work
- Zero incomplete items required for automatic progression

**Operational Guidance:**

1. After prerequisites check (step 1), immediately scan `FEATURE_DIR/checklists/` directory for all `.md` files
2. For each checklist file, parse line-by-line using regex patterns:
   - Total items: count lines matching `- \[ \]` or `- \[X\]` or `- \[x\]`
   - Completed: count lines matching `- \[X\]` or `- \[x\]`
   - Incomplete: count lines matching `- \[ \]`
3. Build status table with columns: Checklist (filename), Total, Completed, Incomplete, Status (✓ PASS / ✗ FAIL)
4. Calculate overall readiness: PASS if all checklists have 0 incomplete items, otherwise FAIL
5. If FAIL status:
   - Display table prominently with incomplete counts highlighted
   - Output blocking prompt: "Some checklists are incomplete. Do you want to proceed with implementation anyway? (yes/no)"
   - Await user response; parse for affirmative ("yes", "proceed", "continue") vs. negative ("no", "wait", "stop")
   - On negative response, halt execution with message: "Implementation halted. Complete checklists or re-run with override when ready."
   - On affirmative response, log override decision and proceed to step 3
6. If PASS status:
   - Display table showing all ✓ PASS statuses
   - Output confirmation: "All checklists complete. Proceeding to implementation."
   - Automatically proceed to step 3 without blocking
7. If no checklists directory exists, skip validation and proceed directly to step 3

### Pattern: Context-Complete Document Loading

**Objective:** Ensure implementation agent has complete architectural and specification context by loading all available planning artifacts before code generation.

**Context of Application:** Step 3 (implementation context analysis), after readiness validation passes.

**Key Characteristics:**

- Required documents (tasks.md, plan.md) loaded unconditionally with failure halting execution
- Optional documents (data-model.md, contracts/, research.md, quickstart.md) loaded opportunistically when present
- Document availability checked before read attempts to avoid unnecessary errors
- Loaded content indexed by concern: tasks (what), plan (how), data-model (entities), contracts (interfaces), research (decisions), quickstart (usage)

**Operational Guidance:**

1. Define required document set: `tasks.md` (task breakdown), `plan.md` (architecture/tech stack)
2. Attempt to read each required document from `FEATURE_DIR/`:
   - If read fails, halt execution with error: "Required document {filename} missing or unreadable. Run /speckit.tasks or /speckit.plan to generate."
   - If read succeeds, parse content and store in context memory indexed by document type
3. Define optional document set with conditional logic:
   - `data-model.md`: Check existence; if present, load entity definitions, relationships, constraints
   - `contracts/`: Check directory existence; if present, enumerate and load all contract files (API specs, schemas, test requirements)
   - `research.md`: Check existence; if present, load technical decisions, rejected alternatives, constraint rationale
   - `quickstart.md`: Check existence; if present, load integration patterns, example usage scenarios
4. For each optional document, attempt read but continue on failure (document absence is valid state)
5. Build unified implementation context structure containing:
   - Tasks: phase structure, task list with IDs/descriptions/files/dependencies
   - Architecture: tech stack, file structure, component relationships
   - Data: entities, fields, relationships, validation rules (if data-model.md exists)
   - Contracts: API endpoints, request/response schemas, test coverage requirements (if contracts/ exists)
   - Decisions: chosen approaches, rejected alternatives, constraint justifications (if research.md exists)
   - Patterns: integration scenarios, usage examples (if quickstart.md exists)
6. Validate context completeness: Can tasks be executed with available information? If critical info missing (e.g., tasks reference entities but no data-model.md), warn user before proceeding
7. Use loaded context throughout implementation to ensure code consistency with specification

### Pattern: Technology-Aware Ignore File Generation

**Objective:** Prevent accidental commits of build artifacts, credentials, and IDE metadata by automatically creating comprehensive ignore files matched to detected project technologies.

**Context of Application:** Step 4 (project setup verification), after context loading, before task execution.

**Key Characteristics:**

- Ignore file requirements detected from actual project state (files present, plan.md tech stack)
- Technology-specific pattern sets applied (Node.js vs Python vs Java have different artifact patterns)
- Existing ignore files augmented rather than replaced, preserving user customizations
- Universal patterns (`.DS_Store`, `.env*`) included in all ignore files

**Operational Guidance:**

1. Detect project technologies and tools by examining:
   - File system: Check for `package.json` (Node.js), `requirements.txt` (Python), `pom.xml` (Java), `Cargo.toml` (Rust), etc.
   - Plan document: Parse `plan.md` for tech stack declarations in "Technology Stack" or "Dependencies" sections
   - Configuration files: Detect `.eslintrc*`, `.prettierrc*`, `Dockerfile*`, `*.tf` (Terraform)
2. For each detected technology/tool, determine which ignore files are needed:
   - Git repository (check `git rev-parse --git-dir` success) → `.gitignore` required
   - Docker present (`Dockerfile*` or plan mentions Docker) → `.dockerignore` required
   - ESLint config present (`.eslintrc*` or `eslint.config.*`) → `.eslintignore` or config `ignores` field required
   - Prettier config present (`.prettierrc*`) → `.prettierignore` required
   - NPM publishing (plan mentions npm publish or `private: false` in package.json) → `.npmignore` required
   - Terraform files present (`*.tf`) → `.terraformignore` required
   - Helm charts present (`Chart.yaml`) → `.helmignore` required
3. For each required ignore file:
   - **If file exists**: Read current content; extract existing patterns; identify missing critical patterns from technology pattern set; append only missing patterns with comment header `# Added by speckit.implement`
   - **If file missing**: Create new file with complete pattern set for all detected technologies plus universal patterns
4. Pattern set construction:
   - Start with universal patterns: `.DS_Store`, `Thumbs.db`, `*.tmp`, `*.swp`, `.vscode/`, `.idea/`
   - Add technology-specific patterns from lookup table (see step 4 pattern lists)
   - Add tool-specific patterns (Docker, ESLint, Prettier, Terraform, Kubernetes)
   - Include security-critical patterns: `.env*`, `*.key`, `*.pem`, `secrets/`, `*.secret.*`
5. Write ignore files atomically (temp file, then rename) to prevent corruption
6. For `eslint.config.*` (flat config), update the `ignores` array rather than separate `.eslintignore` file
7. Log created/updated ignore files in implementation progress report

### Pattern: Phase-Ordered Task Execution with Dependency Respect

**Objective:** Maximize implementation correctness and debuggability by executing tasks in dependency-respecting order: setup before tests, tests before implementation, core before integration, integration before polish.

**Context of Application:** Steps 5-7 (task parsing and execution), throughout the implementation workflow.

**Key Characteristics:**

- Tasks organized into phases: Setup → Tests → Core → Integration → Polish
- Within each phase, sequential tasks executed in declared order
- Parallel tasks (marked `[P]`) can execute concurrently if no file conflicts
- File-based coordination: tasks touching same files forced into sequential execution
- Phase completion validated before advancing to next phase

**Operational Guidance:**

1. Parse `tasks.md` to extract task structure (step 5):
   - Identify phase boundaries by section headings: `## Setup Phase`, `## Tests Phase`, `## Core Phase`, `## Integration Phase`, `## Polish Phase`
   - For each task, extract: ID (e.g., `S1`, `T2`), description, file paths affected, parallel marker (`[P]`)
   - Build dependency graph: tasks in same phase with overlapping file paths must be sequential; tasks with `[P]` marker and non-overlapping files can be parallel
2. Initialize phase execution queue: `[Setup, Tests, Core, Integration, Polish]`
3. For each phase in queue:
   - Load all tasks for current phase
   - Partition tasks into sequential groups based on file conflicts:
     - Group 1: All tasks affecting `file_a.js` (must run in declared order)
     - Group 2: All tasks affecting `file_b.py` with no overlap to Group 1 (can run parallel to Group 1 if all marked `[P]`)
   - Execute groups:
     - Sequential tasks: run one at a time, halt on failure, report progress after each
     - Parallel tasks: spawn concurrent executions, collect results, continue with successful tasks, report failed tasks
   - After all tasks in phase complete, run phase validation:
     - Setup phase: verify project structure created, dependencies installed, configuration files present
     - Tests phase: verify test files created, test runner configured
     - Core phase: verify core logic files exist, basic compilation/import succeeds
     - Integration phase: verify external connections configured, middleware in place
     - Polish phase: verify documentation updated, tests pass
   - If phase validation fails, halt and report specific validation failure
4. Follow TDD discipline within phases:
   - Tests phase tasks execute before corresponding Core phase implementation tasks
   - Test tasks write failing tests based on contracts
   - Core tasks implement code to make tests pass
5. Mark completed tasks in `tasks.md` by replacing `- [ ]` with `- [X]` after successful execution (step 8)
6. Maintain execution log with timestamps: `S1 started 14:32:15`, `S1 completed 14:32:47`, `S2 started 14:32:48`

### Pattern: Incremental Progress Persistence

**Objective:** Prevent work loss and enable resume-after-failure by persisting task completion status immediately after each successful task execution.

**Context of Application:** Step 8 (progress tracking), after each individual task completes successfully.

**Key Characteristics:**

- Task completion marked in `tasks.md` immediately (not batched at end)
- Checkbox syntax updated: `- [ ] Task description` → `- [X] Task description`
- File written atomically after each update to prevent corruption
- Progress log maintained separately from task file for debugging

**Operational Guidance:**

1. After any task executes successfully (code written, tests pass, validation succeeds):
   - Locate task in loaded `tasks.md` content by matching task ID or exact description
   - Replace checkbox marker: `- [ ]` → `- [X]` (preserve indentation and whitespace)
   - Write updated `tasks.md` content to disk using atomic file replacement (write temp, rename)
2. Verify write succeeded by reading back and confirming checkbox updated
3. If write fails, retry once; if second attempt fails, log error but continue execution (don't halt implementation for task file update failure)
4. Maintain separate progress log file `FEATURE_DIR/.implementation-log` with entries: `[2024-02-06 14:32:47] S1 COMPLETE: Created project structure`
5. Progress log appended (not overwritten) for durability and debugging
6. On any execution halt (error, user interrupt), final state reflects all successfully completed tasks marked in `tasks.md`
7. Resume capability: if implementation re-run after failure, skip tasks already marked `[X]`, resume from first `[ ]` task in current phase

### Pattern: Fail-Fast Sequential with Graceful Parallel Degradation

**Objective:** Maximize debuggability for critical sequential tasks while maintaining implementation momentum for independent parallel tasks.

**Context of Application:** Step 8 (error handling), throughout task execution for both sequential and parallel task sets.

**Key Characteristics:**

- Sequential task failure immediately halts execution with detailed error context
- Parallel task failure isolated; successful parallel tasks continue; failed tasks reported in aggregate
- Error messages include task ID, description, affected files, error output, and suggested remediation
- Execution can proceed if some parallel tasks fail but critical path tasks succeed

**Operational Guidance:**

1. For sequential tasks (default execution mode):
   - Execute task and capture both success/failure status and any error output
   - On failure: immediately halt execution, do NOT proceed to next task
   - Report error with structure:

     ```text
     ❌ Task {ID} FAILED: {description}
     Files affected: {file_list}
     Error: {error_message}
     Suggested action: {remediation_hint}
     ```

   - Remediation hints based on error type:
     - Compilation error → "Check syntax in {file}, verify imports"
     - Test failure → "Review test expectations in {test_file}, validate implementation logic"
     - File not found → "Verify file structure matches plan.md, check task dependencies"
     - Permission error → "Check file permissions, ensure write access to {directory}"
   - Provide next steps: "Fix the error and re-run /speckit.implement to resume from this task."
2. For parallel tasks (marked `[P]`):
   - Execute all parallel tasks concurrently (spawn separate execution contexts)
   - Collect results for each: {task_id: success/failure, output/error}
   - After all parallel tasks complete, analyze results:
     - If all succeeded: report success count, proceed to next task/phase
     - If some failed: report success count + failure count, list failed task IDs with errors
   - Continue execution unless ALL parallel tasks failed or a failed task is critical path dependency for subsequent tasks
   - Update `tasks.md` with `[X]` for successful parallel tasks only
3. Error context enrichment:
   - Include last 20 lines of error output for debugging
   - Reference related documents: "This task implements feature described in spec.md section 3.2"
   - Suggest related tasks: "This failure may affect dependent tasks: T5, T7"
4. State preservation on failure: ensure all completed tasks marked, progress log written, partial code changes committed to feature branch if possible

## Anti-Patterns: Common Mistakes in Implementation Execution

### Anti-Pattern: Unchecked Implementation Rush

**Description:** Proceeding directly to code generation and task execution without validating that prerequisite checklists (security, UX, testing, etc.) are complete, potentially implementing features that violate unresolved concerns.

**Reasons to Avoid:**

- Security vulnerabilities introduced because security checklist items not addressed before code written
- UX inconsistencies baked into implementation before UX validation complete
- Test gaps discovered post-implementation, requiring costly refactoring to accommodate proper test coverage
- Rework required when checklist completion reveals design flaws that invalidate early implementation

**Negative Consequences:**

- Implementation of authentication system proceeds before security checklist confirms token expiration strategy, requiring rewrite when insecure default discovered
- UI components built before accessibility checklist complete, necessitating significant DOM restructuring for ARIA compliance
- Database schema implemented before data validation checklist reviewed, causing migration hell when validation rules require schema changes
- Code review identifies critical gaps from incomplete checklists, forcing large PRs to be blocked or reverted
- Time waste: implementing features correctly from start (after checklist completion) faster than implement-discover-refactor cycle

**Correct Alternative:** Use the **Gated Readiness Validation** pattern to scan all checklists, block on incomplete items with explicit user override prompt, and ensure informed decision before proceeding with implementation.

### Anti-Pattern: Context-Starved Implementation

**Description:** Executing tasks from `tasks.md` without loading supporting specification documents (plan.md, data-model.md, contracts/, research.md, quickstart.md), causing implementation to diverge from architectural decisions and data model constraints.

**Reasons to Avoid:**

- Agent lacks architectural context, generating code inconsistent with tech stack or design patterns
- Data model mismatches: entities implemented with wrong fields, relationships, or validation rules
- API contract violations: endpoints return different schemas than specified in contracts/
- Technical decisions ignored: implementing rejected alternatives or violating constraint rationale from research.md
- Integration pattern gaps: missing reference examples from quickstart.md leads to incorrect usage patterns

**Negative Consequences:**

- REST API implemented using Express when plan.md specifies FastAPI, requiring complete rewrite
- User entity created without required email validation constraint specified in data-model.md, causing test failures
- Authentication endpoint returns `{token: string}` when contract specifies `{access_token: string, refresh_token: string}`, breaking client integration
- Database connection pooling omitted despite research.md documenting pool size constraint rationale, causing production scalability issues
- Error handling implemented with generic try/catch when quickstart.md demonstrates specific error recovery pattern, creating inconsistent error UX

**Correct Alternative:** Apply the **Context-Complete Document Loading** pattern to load all required and available optional documents before task execution, ensuring implementation aligns with specification.

### Anti-Pattern: Universal Ignore File Templates

**Description:** Copying generic ignore file templates (e.g., GitHub's standard `.gitignore` for Node.js) without customizing to actual project technologies, tools, and specific artifact patterns, leaving project-specific build outputs, credentials, or IDE files unprotected.

**Reasons to Avoid:**

- Generic templates miss project-specific patterns: custom build directories, framework-specific cache files, generated documentation
- Over-inclusive templates ignore files that should be committed: fixture data, test snapshots, vendored dependencies
- Under-inclusive templates expose credentials: `.env.local`, `secrets.dev.yaml`, API key files in non-standard locations
- Multi-technology projects need composite patterns: Node.js + Python + Docker requires merging three pattern sets

**Negative Consequences:**

- `.gitignore` copied from Node.js template but project uses TypeScript; `dist/` ignored but `build/` committed to repo, bloating repository
- Standard Python template ignores `venv/` but project uses `pipenv`; `Pipfile.lock` committed causing dependency conflicts
- Docker template missing, resulting in `node_modules/` and `.git/` copied into Docker image, creating 500MB images instead of 50MB
- `.env.production` not in ignore pattern because template only covers `.env`; production credentials accidentally committed
- ESLint configuration uses flat config (`eslint.config.js`) but implementation creates `.eslintignore` instead of updating config `ignores` field, causing patterns to be ignored

**Correct Alternative:** Use the **Technology-Aware Ignore File Generation** pattern to detect actual project technologies and tools, apply appropriate pattern sets, augment existing files rather than replacing, and include security-critical universal patterns.

### Anti-Pattern: Dependency-Blind Task Execution

**Description:** Executing tasks in the order they appear in `tasks.md` without analyzing dependencies, parallel opportunities, or phase boundaries, causing test execution before setup, implementation before tests, or sequential execution of parallelizable tasks.

**Reasons to Avoid:**

- Tests fail because dependencies not installed (setup tasks skipped or executed out of order)
- TDD violated: implementation code written before tests, losing design-through-testing benefits
- Implementation inefficiency: 10 parallel-safe tasks executed sequentially, multiplying implementation time by 10x
- Integration failures: database migrations run before database client library installed
- File conflicts: two tasks modifying same file concurrently, causing race conditions and corruption

**Negative Consequences:**

- Test task `T1: Write user authentication tests` executed before setup task `S3: Install pytest framework`, causing test runner errors
- Core task `C2: Implement user service` executed before test task `T2: Write user service tests`, missing TDD design feedback that would have improved API design
- Five parallel-marked tasks creating independent React components executed sequentially, taking 50 minutes instead of 10 minutes
- Integration task `I1: Run database migrations` executed before setup task `S4: Install database client`, causing connection errors
- Tasks `C3: Update user.js` and `C5: Refactor user.js` run concurrently, causing file write conflict and lost changes

**Correct Alternative:** Apply the **Phase-Ordered Task Execution with Dependency Respect** pattern to organize tasks into phases, respect sequential dependencies, exploit parallel opportunities, and enforce file-based coordination.

### Anti-Pattern: End-of-Batch Task Status Update

**Description:** Accumulating all task completion status updates in memory throughout implementation and writing them to `tasks.md` only at the very end of execution, risking complete loss of progress tracking on failure or interruption.

**Reasons to Avoid:**

- Process interruption (crash, timeout, user cancel) loses record of all completed tasks
- Failure mid-implementation leaves `tasks.md` unchanged despite 20 completed tasks
- Resume impossible: no way to determine which tasks succeeded before failure
- Debugging hindered: cannot correlate task completion timing with logs or file changes

**Negative Consequences:**

- Implementation runs for 2 hours, completes 45 of 60 tasks, then fails on task 46; `tasks.md` shows 0 tasks complete because batch update never written
- User cancels execution after 30 minutes; no record of which 15 tasks completed successfully; must manually inspect codebase to determine resume point
- Execution crash leaves incomplete state: files created but no task marked complete; subsequent re-run attempts to recreate files, causing conflicts
- Progress reporting to user shows "25 tasks complete" but `tasks.md` still shows all unchecked; user opens file and sees contradiction, loses trust in system
- Post-mortem debugging after failure cannot determine when task T12 completed because timestamps not recorded incrementally

**Correct Alternative:** Use the **Incremental Progress Persistence** pattern to update `tasks.md` immediately after each successful task completion, ensuring durable progress tracking and resume capability.

### Anti-Pattern: Global Halt on Parallel Task Failure

**Description:** Treating all task failures identically regardless of sequential vs. parallel status, halting entire implementation when a single parallel task fails even though other independent parallel tasks could safely continue.

**Reasons to Avoid:**

- Implementation momentum lost unnecessarily: 9 of 10 parallel tasks could succeed but all halted because 1 failed
- Reduced implementation efficiency: parallel tasks designed for concurrent execution forced into sequential retry-one-at-a-time pattern
- Misleading error attribution: user sees "implementation failed" when actually 90% of parallel work succeeded
- Wasted execution time: successful parallel tasks must re-run on retry despite already completing correctly

**Negative Consequences:**

- Ten parallel tasks creating independent API endpoints; endpoint 7 fails due to typo; endpoints 8-10 never execute despite being completely independent
- User corrects typo in endpoint 7, re-runs implementation; endpoints 1-6 re-executed redundantly, wasting 15 minutes
- Parallel test writing tasks for different modules; one test has syntax error; all other test files never created; user must manually identify which tests missing
- Implementation report shows "❌ FAILED after 5 minutes" when reality is "✅ 9/10 parallel tasks succeeded, 1 fixable error in independent task"
- Sequential dependency tasks blocked waiting for parallel tasks that already succeeded but weren't marked complete because batch execution halted

**Correct Alternative:** Apply the **Fail-Fast Sequential with Graceful Parallel Degradation** pattern to halt immediately on sequential task failure while allowing parallel tasks to complete, reporting aggregate parallel results.

### Anti-Pattern: Opaque Error Reporting

**Description:** Reporting task failures with minimal context ("Task T5 failed"), omitting critical debugging information like affected files, error messages, stack traces, suggested remediation, or relationship to specification documents.

**Reasons to Avoid:**

- User cannot diagnose failure without manually inspecting all files, logs, and specification documents
- No actionable remediation guidance forces user into trial-and-error debugging
- Error reproduction difficult without understanding which files were being modified
- Learning opportunity lost: user doesn't understand why failure occurred or how to prevent similar issues

**Negative Consequences:**

- Error report "❌ Task C7 failed" requires user to: (1) find task C7 in tasks.md, (2) read description, (3) identify affected files, (4) examine file contents, (5) check logs, (6) search specification for context, (7) guess at fix
- Compilation error in `services/payment.ts` reported as generic failure; user unaware which file to examine or what syntax error occurred
- Test failure without showing expected vs. actual values leaves user debugging blindly
- Suggested remediation missing: user doesn't know if failure requires dependency installation, configuration change, code fix, or specification clarification
- Relationship to spec unclear: user doesn't know task C7 implements feature from spec.md section 4.3, making it hard to validate if implementation matches intent

**Correct Alternative:** Use the **Fail-Fast Sequential with Graceful Parallel Degradation** pattern's error reporting structure to include task ID, description, affected files, error output, and remediation hints in every failure message.

Note: This command assumes a complete task breakdown exists in tasks.md. If tasks are incomplete or missing, suggest running `/speckit.tasks` first to regenerate the task list.
