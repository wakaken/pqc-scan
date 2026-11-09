# Task Queue

Tasks are stored as YAML files in this directory.

Current lifecycle:

pending → planned → in-progress → review → done

Optional status:

- blocked

Example:

tasks/001-add-oauth.yaml

Fields:

- id
- title
- type
- priority
- status
- owner
- description

## Status Meanings

- `pending`: task exists but has not been selected yet
- `planned`: Planner selected the task and prepared an execution plan
- `in-progress`: Executor started implementation
- `review`: Executor finished implementation and the task is ready for Reviewer assessment
- `done`: Evaluator validated the completed work
- `blocked`: progress cannot continue without human input or unavailable validation

## Ownership

- `pending`: Planner / Reflection / Monitor
- `planned`: Planner
- `in-progress`: Executor
- `review`: Executor
- `done`: Evaluator
- `blocked`: any agent

## Notes

- Keep lifecycle terminology aligned with `AGENTS.md` and `docs/TASK-LIFECYCLE.md`.
- Use `blocked` when work cannot safely continue, for example due to missing CI, unclear specification, or required human review.

## Task and PR boundaries

- Default to one task per PR. Include implementation, tests and documentation for the task together.
- Use `implementation_steps` for the ordered work inside that PR; steps have no separate PR or lifecycle status. Validate the full acceptance criteria before closing the task.
- Keep a prerequisite feasibility study separate when its result determines whether or how to implement the dependent feature. The study and implementation remain distinct tasks, each normally delivered in one PR.
- Do not preassign A/B/C PRs or split tests and documentation away from the behavior they validate. If actual scope requires a different task boundary, Planner records the reason and dependencies before proceeding.
- Conditional steps do not automatically require a follow-up PR. Record why a step applies or is omitted without weakening the task's acceptance criteria.
