# AGENTS.md

This repository is maintained by a main agent with two independent review
stages for user implementation requests.

Respond to the user in clear, concise and polite Japanese.

# Project Context

**pqc-scan** is a Rust-based CLI static analysis tool that detects
Post-Quantum Cryptography (PQC) migration risks in software.

It scans repositories for cryptographic usage and produces migration reports
using a rule-based detection system.

Security, correctness, backward compatibility and stable output are critical.

# Working Approach

- The main agent investigates, plans, implements, tests, evaluates and documents.
- Role names describe responsibilities, not separate agent processes.
- Independent reviews follow the Subagent and Token Policy below.
- For clear, small changes, proceed without a separate design document or
  implementation plan.
- For complex or ambiguous changes, first summarize:
  - purpose and acceptance criteria
  - constraints and compatibility requirements
  - viable approaches and the recommended approach
  - implementation steps and verification
- Reuse existing plans and task records when they remain applicable.
- Ask about unresolved decisions that materially affect correctness,
  compatibility, security or scope.
- Continue authorized work that does not depend on an unresolved decision.
- Do not expand scope, start continuous monitoring or create unrelated tasks
  unless explicitly requested.
- Prefer small, safe changes. Avoid unrelated refactoring.
- Preserve unrelated user changes.

# Main-Agent Workflow and Review Template

This file defines the working rules. The only separate prompt is
`prompts/REVIEWER.md`, a shared request template for the two review stages.
Planner, Executor and Evaluator are responsibility labels used in existing
task records, not separate agents or prompt files.

For authorized work, the main agent:

1. Establishes the requested outcome, acceptance criteria, applicable task and
   existing evidence. Plans complex work as described in Working Approach.
2. Implements minimal changes and meaningful tests, runs relevant initial
   checks and inspects the combined diff before implementation review.
3. Requests implementation review when required, fixes findings, completes
   required verification and then requests test review when required.
4. Resolves findings, confirms evidence applies to the final source and
   determines completion under Completion and Partial Results.
5. Records durable lessons in existing knowledge documents only when useful.

- An explanation, research request or small change does not require every step
  or a separate task record. Required verification still applies.
- Do not generate tasks merely because the queue is empty. Create tasks only
  when authorized and do not select unrelated work.
- Monitor only explicitly requested signals within the authorized scope and
  duration. Report out-of-scope findings as proposals rather than creating tasks.
- Saved plans, handoffs and skills must follow the limits and completion
  requirements in this file. Generic delegation wording does not authorize spawns.
- Apply `docs/RESTART-LOOP.md` only to the restart baseline and task 014,
  unless the user explicitly extends its scope. Preserve its applicable entry,
  retry, status-transition and completion gates; monitoring and automatic task
  creation remain disabled during that pilot.
- If instructions conflict and this file does not resolve the conflict,
  identify the specific conflict before performing dependent work.

# Subagent and Token Policy

## Main-Agent Responsibilities

The main agent performs investigation, design, planning, implementation,
debugging, testing and documentation without delegating them to subagents.

Explanations, research, design proposals and documentation-only changes
do not automatically require independent review subagents.

## Required Review Stages

Use two review stages for each user implementation request:

1. Implementation review:
   - After implementation, start one implementation-review subagent.
   - Review the combined change for correctness, requirements, security,
     architecture consistency and compatibility.
   - Supply available test results and identify checks that remain unexecuted.

2. Test review:
   - After the main agent completes required tests, start one test-review
     subagent.
   - Check whether the tests actually verify the final change.
   - Inspect coverage, relevant positive and negative cases, meaningful
     assertions, commands, features and evidence matching the final source.
   - Passing logs alone do not establish adequate testing.

The main agent fixes findings and runs necessary verification.

## Review Budget and Context

- Use at most two reviewer identities per user implementation request:
  one for each stage.
- Review the combined change rather than starting reviewers for individual
  plan steps, files, edits, test commands or log files.
- Design, task, branch, final and pre-merge reviews do not create additional
  review slots.
- Reuse each stage's reviewer for scoped verification of material fixes.
- Batch related findings and follow-up requests.
- Reviewers must not delegate further.
- Any additional subagent requires explicit user authorization covering
  its purpose and maximum agent count.
- Tool availability, task count and generic skill wording are not authorization.

When starting a reviewer:

- Use `fork_turns: "none"`.
- Provide concise requirements, the relevant diff or diff artifact,
  source version and test evidence.
- Identify relevant pre-existing changes and exclude unrelated work.
- Do not copy the whole conversation.
- Request actionable findings with file references and requirement impact.

Reviewers rely on the main agent's test evidence unless a specific concern
requires independent reproduction. Do not automatically repeat the full suite.

## Revisions and Resumption

- Record reviewer IDs, review scope and reviewed source versions in the
  existing task record or handoff.
- Splitting a plan, fixing findings, resuming a chat or handing off work
  does not reset the review budget.
- After review-driven changes, rerun affected checks and determine whether
  prior review evidence still applies.
- Obtain scoped confirmation from the existing reviewer when changes
  materially affect the reviewed conclusions.
- A test-review-driven product change must also receive scoped
  implementation review when it affects correctness or compatibility.
- Do not claim an earlier review covers later changes without checking them.

# Required References and Efficient Reading

At the start of repository work, the main agent reviews:

- `README.md`
- `README.ja.md`
- `docs/architecture.md`
- `docs/BEST-PRACTICES.md`
- `docs/DECISIONS.md`
- `docs/MISTAKES.md`
- `docs/PATTERNS.md`
- `docs/TASK-LIFECYCLE.md`

The main agent also reviews:

- `tasks/README.md` when creating or updating task records
- the selected task, its acceptance criteria and applicable plans
- `docs/rules-reference.md` when modifying rules
- relevant user guides and design documents for the affected behavior

Reviewers read this file, `prompts/REVIEWER.md` and the supplied review package.
They consult additional source and documents only as needed to assess their
assigned scope; the main agent's full reading list is not a reviewer checklist.

Reading policy:

- Read required references once per session and reuse the acquired context.
- Revisit changed sections or sections needed to resolve uncertainty.
- Use targeted searches to locate additional relevant material.
- Do not repeatedly read entire directories or unchanged large documents.
- After compaction or handoff, restore scope, decisions, evidence and next
  actions from the existing task record before repeating investigation.
- Do not reduce required verification or omit relevant evidence to save tokens.

# Task Queue and Lifecycle

Tasks are stored as YAML files in `tasks/`.

Normal lifecycle:

pending → planned → in-progress → review → done

Optional states:

- blocked
- cancelled

Responsibilities:

- Planner selects tasks and moves them to `planned`.
- Executor moves tasks to `in-progress` and then `review`.
- Reviewer records findings and review acceptance.
- Evaluator marks tasks `done` only after all completion gates pass.
- Any role may record `blocked` when required progress depends on human
  input or unavailable validation.
- Planner records `cancelled` only after an explicit maintainer decision.

These responsibilities do not require separate agent processes.

## Failures and Replanning

- A resolvable implementation or test failure does not automatically return
  a task to `pending`.
- For implementation corrections, return to Executor responsibility and
  record the task as `in-progress`, then return it to `review` when ready.
- Keep the original acceptance criteria and preserve previous evidence.
- Replan when findings change the task's assumptions, dependencies or
  implementation approach.
- Do not change agreed scope or acceptance criteria without explicit
  user agreement.
- Use `blocked` when required progress cannot continue without an external
  change or human decision. Explain the concrete dependency.
- Apply any explicitly applicable task-specific retry limits.
- Resume blocked work only after its dependency is resolved and any required
  human decision is obtained. Record the reason and resume at the appropriate
  state; this does not authorize scope changes or reopening cancelled tasks.

## Task Boundaries

- Default to one task per PR, including its implementation, tests and docs.
- Plan steps do not require separate tasks, PRs or review agents.
- Keep a prerequisite feasibility study separate when its result determines
  whether or how implementation should proceed.
- Do not select unrelated queue items or generate new work automatically.
- Record out-of-scope findings as proposals unless task creation is authorized.

# Development and Verification

Use the Rust toolchain specified in `rust-toolchain.toml`.

Typical commands:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

Example CLI run:

```bash
cargo run --bin pqc-scan -- scan ./examples \
  --format all \
  --out-dir ./target/pqc-report \
  --rules-dir ./rules
```

## Verification by Change Type

For Rust implementation, Rust tests, Cargo configuration or dependency changes:

- Run relevant targeted tests during development.
- Run `cargo fmt --all` and inspect resulting changes.
- Run `cargo test --workspace`.
- Run `cargo clippy --workspace --all-targets -- -D warnings`.
- Run `cargo build --workspace` when validating executable behavior,
  packaging or a required build deliverable.
- Exercise affected optional features and supported configurations.
  Default workspace tests do not establish coverage of disabled features.

For rule changes:

- Validate rule schema and unique IDs.
- Run relevant positive and negative detection tests.
- Run workspace tests and required task-specific acceptance checks.
- Run Rust checks above when Rust source or test code also changes.

For documentation-only changes:

- Check references, links, commands and consistency with actual behavior.
- Check agreement between English and Japanese documentation where applicable.
- Rust builds and tests are not required unless the documentation change
  affects executable examples or its acceptance criteria require them.

For CI, scripts or packaging changes:

- Run the relevant syntax checks, script tests and packaging checks.
- Validate affected workflows or produced artifacts as required.
- Local checks do not substitute for required CI execution.

For user-facing behavior:

- Exercise the affected CLI or report workflow.
- Verify compatibility and relevant error paths.
- Follow the browser verification requirements for report presentation
  or interaction changes.

## Verification Evidence

- Record the source revision, relevant uncommitted changes, command,
  feature configuration and exit result.
- Preserve logs or artifacts needed to assess required acceptance criteria.
- A HEAD SHA alone is insufficient when the worktree contains tested changes.
- Reuse passing results while their source, inputs and environment remain
  applicable. Repeat checks when changes or unresolved concerns invalidate them.
- Do not write tests that merely mirror implementation details.
- Use assertions that establish observable requirements and relevant failures.
- Do not claim an unexecuted, unavailable or failed check passed.

# Local Report Browser Verification

When report presentation, browser interaction or report acceptance requires
browser checks:

- Use a temporary HTTP server bound to `127.0.0.1`.
- Do not repeatedly attempt unsupported `file://` navigation.
- Serve a dedicated directory containing only the intended generated reports.
- Never serve the repository root, home directory or private production inputs.
- Record actual browser and viewport observations and captures.
- Verify relevant layout, keyboard behavior and loaded-offline behavior.
- Source or DOM unit tests do not substitute for required browser observations.
- Preserve the user's visible report tab and report the loopback URL.
- Keep or stop the dedicated server according to the requested viewing workflow.
- If required browser checks are unavailable, record them as unexecuted and
  keep the affected acceptance criteria unfinished.

# Security and Compatibility

- Validate inputs and avoid exposing secrets.
- Never expose private key material or log sensitive cryptographic data.
- Never weaken cryptographic checks.
- Maintain cryptographic integrity and API compatibility.
- Preserve output contracts relied upon by external systems.
- Do not treat an absent detection as proof that an input is safe or fully
  supported unless the specified behavior establishes that conclusion.

## Rule System

Rules are stored in `rules/default/*.yml`.

When modifying rules:

- rule IDs must be unique
- rule schema must remain valid
- test relevant matching and non-matching inputs
- update `docs/rules-reference.md` when necessary
- assess compatibility before changing existing rule behavior

# Documentation

- `README.md` must be written in English.
- `README.ja.md` must be written in Japanese.
- Update both README files when user-facing behavior changes.
- Report format changes require updates to both README files and applicable
  report documentation, subject to the human review requirements below.
- Update affected agent-facing references together when workflow terms or
  document locations change.
- Keep past review artifacts, completed task records and measurements unchanged
  when reorganizing current guidance. Historical references describe the layout
  at the time; they are not instructions to restore removed prompts.
- Record durable lessons in existing knowledge documents when useful.
- Avoid duplicate knowledge and unnecessary design or reflection documents.
- Documentation updates do not replace implementation or validation.

# Completion and Partial Results

Mark a task `done` only when every original acceptance criterion and required
deliverable has been implemented and verified with evidence matching the
final source, including the required review stages.

- An artifact list, passing tests or a limited review does not establish
  completion of requirements outside its scope.
- Do not mark a task done while required work is Partial, Unknown, Not Tested,
  failed or deferred.
- Recording a limitation or transferring work to another task does not satisfy
  the original criterion.
- Do not narrow acceptance criteria or reinterpret scope without explicit
  user agreement.
- Distinguish completion of an evaluation from measured product quality.
  Finding a product failure can be a valid completed evaluation only when
  the evaluation itself is complete and valid for every required criterion.
- Incomplete truth, missing measurements, unresolved required reviews and
  unverified assessment validity remain unfinished work.
- Continue work on resolvable issues.
- If a blocker prevents completion, keep the task unfinished and explain:
  - what is blocked and why
  - what was attempted
  - which acceptance criteria remain unmet
  - the concrete action or decision needed to proceed
- When reopening an incorrectly closed task, preserve historical evidence,
  record why the previous completion decision is superseded, and reconcile
  the task queue and roadmap without rewriting old measurements.
- Cancellation is an explicit maintainer decision, not implementation completion.
  Preserve its reason and history. Reopening requires an explicit new request.

# Reports to the User

- Explain in clear, concise and polite Japanese what changed and why.
- Describe resulting behavior, actual verification outcomes and remaining work.
- Distinguish implemented changes, measured facts, proposals and unexecuted checks.
- For changes and measurements, provide the relevant source version,
  verification commands and exit results, and links to reviewable artifacts.
- Summarize routine successful checks without dumping raw logs.
- State completion only against the original acceptance criteria.
- If the result is Partial, lead with the fact that the task is unfinished
  and explain the reason.
- Do not describe unfinished work as complete or say there is no remaining work.

# Stop Conditions

Stop the affected work and obtain human review when:

- high-risk architectural changes are required
- cryptographic logic must change without a clear specification
- rule system changes could break compatibility
- storage formats change

Preserve current work and evidence. Explain the required decision before
performing the affected change.

Human review at these boundaries is separate from the two subagent review
stages and does not authorize additional subagents.
