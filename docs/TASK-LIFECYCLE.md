# Task Lifecycle

Tasks are YAML files in `tasks/`. AGENTS.md defines the working rules,
verification and completion gates; this document explains task states.

The main agent owns planning, implementation and evaluation. Planner, Executor,
Evaluator, Reflection and Monitor are responsibility labels retained in task
records, not separate processes. Independent reviewers assess the two review
stages required by AGENTS.md; they do not close tasks themselves.

# Status Progression

pending → planned → in-progress → review → done

Optional states are `blocked` and `cancelled`. These have different meanings:
blocked work is unfinished; cancellation records an explicit maintainer decision.

# Status Definitions

| State | Meaning | Responsibility |
|---|---|---|
| pending | Authorized task exists but has not been selected for execution | Main agent; historically Planner / Reflection / Monitor |
| planned | Scope, original acceptance criteria and implementation steps are established | Main agent as Planner |
| in-progress | Implementation or corrections are underway | Main agent as Executor |
| review | Change is ready for assessment; required review and validation may still be unfinished | Main agent as Executor records readiness; reviewers record findings |
| done | Every original criterion and deliverable is verified against the final source, including required reviews | Main agent as Evaluator |
| blocked | Required progress depends on human input or unavailable validation | Main agent or a role identifying the blocker |
| cancelled | Maintainer explicitly ended further work | Main agent as Planner records the decision |

A concise plan in the task record is sufficient for small work. A separate plan
document is not required merely to enter planned. Do not create tasks because
the queue is empty or automatically generate tasks from reflection or monitoring.
Task creation requires authorization under AGENTS.md.

# Corrections and Replanning

- A resolvable failure returns the task to in-progress for corrections, then
  review when ready. It does not automatically reset work to pending.
- Replan when assumptions or dependencies change. Record the revised approach
  before proceeding and obtain agreement before changing scope or criteria.
- Preserve prior evidence and distinguish historical results from validation
  of the final source. Passing tests alone do not establish done.
- Apply `docs/RESTART-LOOP.md` only to the restart baseline and task 014, unless
  explicitly extended by the user. Its applicable retry limits, pending
  transition and completion gates remain exceptions to the normal flow.

# Blocked Work

Record the blocker, attempts made, unmet criteria and concrete action needed.
Do not claim completion while required verification is unavailable.
Resume only after the dependency is resolved and any required human decision
is obtained. Record the reason and return to the appropriate state. A temporary
validation outage does not itself require a new human approval when no such
decision or stop condition applies.

# Cancellation and Historical Evidence

Only an explicit maintainer decision permits cancellation. Record its date and
reason and exclude the task from the active queue. Cancellation asserts neither
completed implementation nor newly passed validation; preserve existing
functionality and historical evidence.

Only an explicit new maintainer request may reopen cancelled work. The main
agent records the resumption decision under Planner responsibility before work
starts. When reopening incorrectly completed work, preserve previous evidence
and record why the former completion decision is superseded.

Old role names and removed prompt paths in completed tasks and past review
artifacts are historical evidence. Preserve them rather than rewriting history
to match the current guidance layout.
