# Task Queue

Task definitions are stored in `<ID>/task.yaml`, where `<ID>` is the three-digit
task number. The YAML `id` must remain a string matching its directory name.

Existing record status vocabulary (not a required pstack execution sequence):

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

Existing `implementation_steps` describe ordered work within a task; steps
have no separate stored lifecycle status. Add acceptance criteria and evidence required
by the selected task without changing historical definitions merely to add fields.

## Trial workflow and record compatibility

[AGENTS.override.md](../AGENTS.override.md) selects Open Pstack as the development
harness. The custom lifecycle and fixed review process are inactive; the status
vocabulary and file layout above remain compatible with existing records.
Pstack plans and work units do not automatically need a new numbered YAML task
or the old pending-to-done execution sequence.

When explicitly working on an existing task, retain its original acceptance
criteria, dependencies and evidence. A `done` record still requires final-source
validation; blocked and cancelled remain distinct from completion. Do not rewrite
past status transitions, owner labels, reviews or measurements. Starting this
trial does not select queue items, reopen cancelled tasks or change task status.
The retired [lifecycle guide](../docs/TASK-LIFECYCLE.md) is available only to
interpret old records or roll back; links to it do not reactivate its process.

Record evidence in the owning task when one exists. Otherwise use the current
chat or PR and pstack's appropriate artifacts; do not create a task solely to
satisfy the retired review-metadata procedure.
