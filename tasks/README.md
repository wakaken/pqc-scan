# Task Queue

Task definitions are stored in `<ID>/task.yaml`, where `<ID>` is the three-digit
task number. The YAML `id` must remain a string matching its directory name.

Current lifecycle:

pending → planned → in-progress → review → done

Optional status:

- blocked
- cancelled

Example:

tasks/001/task.yaml

Plans, results and reviews belong beside `task.yaml`. Keep measured data, logs,
images and archives under the task's `evidence/` directory, retaining original run
subdirectories and historical contents. An unstarted task needs only its definition.
Version roadmaps remain in `docs/roadmap/`; records without a single owning task
remain in `docs/records/` outside task directories.

The compatibility link `docs/records/task043-maintainability-20261112` resolves to
`tasks/043/evidence/task043-maintainability-20261112` so the unchanged integration
evidence can retain its original relative link. It is not a second copy of the evidence.

Read task definitions only from `tasks/<ID>/task.yaml`; do not recursively treat
YAML files inside `evidence/` as current definitions. Discovering or validating
pending definitions does not authorize executing them or changing their status.

Paths in task YAML fields and commands are relative to the repository root;
run those commands from that root unless explicitly stated otherwise. Markdown
links are relative to the containing document. Historical paths and commands
inside evidence describe their original run and must not be rewritten.

The confined source snapshot includes direct task definitions and this guide,
while task plans, results, reviews and `evidence/` remain outside build inputs.
This snapshot boundary does not exclude those files from repository self-scans;
self-scan expectations require individual review and exact source/result bindings.

Fields:

- id
- title
- type
- priority
- status
- owner
- description

## Working Rules

Follow [AGENTS.md](../AGENTS.md) for authorization and required reading,
[verification and completion](../docs/agent/VERIFICATION.md) for full completion
gates, [review policy](../docs/agent/REVIEWS.md) for independent reviews, and
[task lifecycle](../docs/TASK-LIFECYCLE.md) for states and task boundaries.
The main agent performs planning, implementation and evaluation. Role names in
task records are responsibility labels, not separate agents. `prompts/REVIEWER.md` is the only review template.
Do not create unrelated tasks or automatically generate tasks from an empty queue,
reflection or monitoring without authorization.

## Status Meanings

- `pending`: task exists but has not been selected yet
- `planned`: the main agent established scope, acceptance criteria and implementation steps
- `in-progress`: the main agent started implementation or corrections
- `review`: the change is ready for assessment; required reviews and validation may still be unfinished
- `done`: the main agent verified all original criteria and deliverables against the final source, including required reviews
- `blocked`: progress cannot continue without human input or unavailable validation
- `cancelled`: the maintainer explicitly ended further work; record the reason and preserve implementation history. This does not assert implementation or validation completion.

## Ownership

The main agent applies the following responsibility labels; these do not
require separate processes. Existing owner fields remain valid historical data.

- `pending`: Planner / Reflection / Monitor
- `planned`: Planner
- `in-progress`: Executor
- `review`: Executor
- `done`: Evaluator
- `blocked`: any agent
- `cancelled`: Planner, only after an explicit maintainer decision

## Corrections and Resumption

- Correct resolvable failures in `in-progress`, then return to `review`; do not
  automatically reset tasks to `pending`.
- Replan when assumptions or dependencies change, preserving evidence and
  obtaining agreement before changing scope or acceptance criteria.
- For blocked work, record the dependency, attempts, unmet criteria and next
  action. Resume at the appropriate state once the dependency is resolved and
  any required human decision is obtained.
- The restart baseline and task 014 retain the applicable special transitions,
  retry limits and gates in `docs/RESTART-LOOP.md`, as scoped by AGENTS.md.

## Notes

- Keep lifecycle terminology aligned with `AGENTS.md` and `docs/TASK-LIFECYCLE.md`.
- Use `blocked` when work cannot safely continue, for example due to missing CI, unclear specification, or required human review.

## Task and PR boundaries

- Default to one task per PR. Include implementation, tests and documentation for the task together.
- Use `implementation_steps` for the ordered work inside that PR; steps have no separate PR or lifecycle status. Validate the full acceptance criteria before closing the task.
- Keep a prerequisite feasibility study separate when its result determines whether or how to implement the dependent feature. The study and implementation remain distinct tasks, each normally delivered in one PR.
- Do not preassign A/B/C PRs or split tests and documentation away from the behavior they validate. If actual scope requires a different task boundary, Planner records the reason and dependencies before proceeding.
- Conditional steps do not automatically require a follow-up PR. Record why a step applies or is omitted without weakening the task's acceptance criteria.

Cancelled tasks are excluded from the active queue. Existing functionality is not
removed by cancellation. Reopening requires an explicit new maintainer request
and a recorded Planner decision; agents must not resume them automatically.

Preserve completed task records and past review artifacts when reorganizing
current guidance. Removed prompt paths and former role names in those records
remain historical evidence, not current operational dependencies.
