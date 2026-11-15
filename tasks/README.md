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

## Task Definition Fields

- id
- title
- type
- priority
- status
- owner
- description

Use `implementation_steps` for ordered work within the same task and PR; steps
have no separate lifecycle status. Add acceptance criteria and evidence required
by the selected task without changing historical definitions merely to add fields.

## Lifecycle and Working Rules

Follow [AGENTS.md](../AGENTS.md) for authorization and required reading,
[verification and completion](../docs/agent/VERIFICATION.md) for full completion
gates, [review policy](../docs/agent/REVIEWS.md) for independent reviews, and
[task lifecycle](../docs/TASK-LIFECYCLE.md) for the detailed procedures:

- [Status definitions and responsibility labels](../docs/TASK-LIFECYCLE.md#status-definitions)
- [Corrections and replanning, including the scoped restart exception](../docs/TASK-LIFECYCLE.md#corrections-and-replanning)
- [Blocked work and resumption](../docs/TASK-LIFECYCLE.md#blocked-work)
- [Cancellation and historical evidence](../docs/TASK-LIFECYCLE.md#cancellation-and-historical-evidence)
- [Task and PR boundaries](../docs/TASK-LIFECYCLE.md#task-boundaries)

The lifecycle document owns these definitions; this guide owns task layout and
field conventions. Preserve existing owner labels and historical evidence when
updating current guidance. For work without a task record, use the
[review-record procedure](../docs/agent/REVIEWS.md#review-records) rather than
creating a task solely for review metadata.
