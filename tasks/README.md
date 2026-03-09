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
