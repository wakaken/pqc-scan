# v0.4 task and requirement map

Registered on 2026-11-18 against baseline
`3f2bc6e6209e1f2813caac8077c8d7a7198832cc` and the uncommitted v0.4 planning
documents. See the [roadmap](ROADMAP.md), [requirements](REQUIREMENTS.md) and
[task layout guide](../../../tasks/README.md).

The maintainer authorized task definitions and this map, not execution. Tasks
065-072 are newly registered as `pending`; they have not been selected, started,
implemented or evaluated. Their definitions do not settle D1-D7 or authorize
product/schema/policy/support changes. The definitions' lifecycle state remains
authoritative if this dated registration summary later becomes historical.

## Location and source of truth

This map belongs in `docs/roadmap/v0.4/` because it describes a release's
cross-task requirement ownership and sequencing. `tasks/<ID>/task.yaml` remains
the definition of each task; its future plans, results, reviews and raw evidence
stay in that task's directory. Putting another aggregate definition under
`tasks/` would split the release plan between two locations. No extra root task
index or second machine-readable mapping is introduced.

The YAML uses the existing task fields, including `quality_requirements` and
`planning_record`, without changing any historical definition or adding a task
schema. For these v0.4 definitions:

- `planning_record.primary_requirements` names the task's product-acceptance ownership.
- `planning_record.supporting_requirements` names prerequisite specification work that does not itself implement those requirements.
- `quality_requirements` is the union of those lists, retaining the existing field convention for requirement participation.
- `depends_on` and `status` in the task YAML are authoritative for execution order and state.

Each of the 30 detailed requirement rows has exactly one primary owner below.
Task 066 is a necessary prerequisite specification with no product-acceptance
ownership: completing it cannot mark the features it specifies implemented.
Task 072 checks fulfillment of every row through X05, without taking implementation
ownership away from earlier tasks. Keep this human-readable view consistent with
the YAML metadata whenever an approved mapping changes.

## Tasks in dependency order

Numbers follow prerequisite order. Every dependency has a smaller ID, including
the completed pre-v0.4 dependencies 028 and 056. The listed order is a valid
topological execution order; IDs do not authorize skipping dependency completion
or the applicable maintainer decisions.

| Task | Scope | Work packages | Direct dependencies | Primary requirements |
|---|---|---|---|---|
| [065](../../../tasks/065/task.yaml) | Reuse the all-rule audit and fill current-source/rationale gaps without bulk behavioral changes | W1 | [028](../../../tasks/028/task.yaml), [056](../../../tasks/056/task.yaml) | R1-01 through R1-04 |
| [066](../../../tasks/066/task.yaml) | Resolve saved-evidence feasibility; specify technical/context/decision/report contracts and freeze the evaluation protocol | W2 contract, W3 | 065 | None; prerequisite support for R2, R3, R4, R5-01/02 and X02-X04 |
| [067](../../../tasks/067/task.yaml) | Implement approved shared technical assessment and saved-input adapters, preserving v1 | W2 implementation, W4 input | 066 | R2-01 through R2-05 |
| [068](../../../tasks/068/task.yaml) | Implement bounded scoped context, immutable-content/target bindings and provenance | W4 context | 066, 067 | R4-01 through R4-04, R4-06 |
| [069](../../../tasks/069/task.yaml) | Implement decisions, saved-evidence reassessment and the structured report as one usable CLI/library workflow | W4 decisions, W5 structured output | 067, 068 | R3-01 through R3-05, R4-05, X02, X04 |
| [070](../../../tasks/070/task.yaml) | Add the selected paired human report and its rendered/browser evidence | W5 human output | 069 | X03 |
| [071](../../../tasks/071/task.yaml) | Execute the frozen comparison/practitioner study and extend existing QAM aggregation | W6 | 066, 069, 070 | R5-01 through R5-05 |
| [072](../../../tasks/072/task.yaml) | Reconcile integrated compatibility and every original row against final-source evidence | W7 | 065, 066, 067, 068, 069, 070, 071 | X01, X05 |

Task 066 combines the W2 fact/input contract and W3 joint specification in ordered
steps, while remaining separate from implementation because feasibility and
human decisions determine the implementation approach. Technical mapping comes
before dependent decision inputs; the evaluation protocol is frozen before new
implementation observations. D6 recruitment/resource planning is not a claim that
participants have been secured.

Tasks 067 and 068 each deliver an independently testable shared component with
its own tests and documentation. Task 069 integrates those components with the
decision model, explicit entry point and structured report; the human view in
070 consumes that result rather than duplicating policy logic. Task 071 evaluates
the delivered workflow rather than manufacturing another product engine. Task
072 is the roadmap's integrated acceptance, not a place to postpone the feature
tests that each implementation task already requires. Default to one task/PR,
including its implementation, meaningful tests and affected documentation.

## Requirement-to-task mapping

The requirements' original acceptance clauses remain unchanged. A primary owner
must supply the behavior and evidence required by its rows; prerequisite or final
assessment work cannot substitute for that implementation. The numeric order does
not lower priority or make any of the five mandatory outcomes optional.

| Requirement | Primary acceptance task |
|---|---|
| R1-01 | [065](../../../tasks/065/task.yaml) |
| R1-02 | [065](../../../tasks/065/task.yaml) |
| R1-03 | [065](../../../tasks/065/task.yaml) |
| R1-04 | [065](../../../tasks/065/task.yaml) |
| R2-01 | [067](../../../tasks/067/task.yaml) |
| R2-02 | [067](../../../tasks/067/task.yaml) |
| R2-03 | [067](../../../tasks/067/task.yaml) |
| R2-04 | [067](../../../tasks/067/task.yaml) |
| R2-05 | [067](../../../tasks/067/task.yaml) |
| R3-01 | [069](../../../tasks/069/task.yaml) |
| R3-02 | [069](../../../tasks/069/task.yaml) |
| R3-03 | [069](../../../tasks/069/task.yaml) |
| R3-04 | [069](../../../tasks/069/task.yaml) |
| R3-05 | [069](../../../tasks/069/task.yaml) |
| R4-01 | [068](../../../tasks/068/task.yaml) |
| R4-02 | [068](../../../tasks/068/task.yaml) |
| R4-03 | [068](../../../tasks/068/task.yaml) |
| R4-04 | [068](../../../tasks/068/task.yaml) |
| R4-05 | [069](../../../tasks/069/task.yaml) |
| R4-06 | [068](../../../tasks/068/task.yaml) |
| R5-01 | [071](../../../tasks/071/task.yaml) |
| R5-02 | [071](../../../tasks/071/task.yaml) |
| R5-03 | [071](../../../tasks/071/task.yaml) |
| R5-04 | [071](../../../tasks/071/task.yaml) |
| R5-05 | [071](../../../tasks/071/task.yaml) |
| X01 | [072](../../../tasks/072/task.yaml) |
| X02 | [069](../../../tasks/069/task.yaml) |
| X03 | [070](../../../tasks/070/task.yaml) |
| X04 | [069](../../../tasks/069/task.yaml) |
| X05 | [072](../../../tasks/072/task.yaml) |

The task-066 supporting set is listed explicitly in its YAML; it does not create
a second primary owner. A01-A18 remain the acceptance examples for the selected
scope. Relevant cases belong in each feature's tests; task 072 reconciles their
applicability and evidence across the final integrated source.

## Decision and completion boundaries

| Decision | Where it is prepared or verified | Execution boundary |
|---|---|---|
| D1: context/assessment contract | 066 prepares and records the actual human decision; 068/069 apply it | No affected stored schema/input/report implementation before approval |
| D2: decision vocabulary and precedence | 066 specifies the joint table and minimum inputs; 068/069 apply it | No guessed urgency/unknown/readiness semantics or silent v1 reinterpretation |
| D3: runtime scope | 066 compares formats and records selection; 067 implements the selected adapters | Inventory v1/TLS v1 remain the minimum proposal; SSH selection is explicit, with no probe support/publication promotion |
| D4: CLI and human output | 066 records the contract; 069 implements entry/structured output; 070 implements the selected human view | No new default UI or output contract is assumed from registration |
| D5: audit/classification repairs | 065 identifies impact; 066 records necessary decisions; 067 applies approved gaps only | AGENTS rule/crypto/architecture/storage gates remain in force |
| D6: study authority and resources | 066 freezes the protocol and recruitment plan; 071 secures and verifies actual resources before observations | Missing required truth/participants keeps the evaluation unfinished; external contact still requires authorization |
| D7: numerical product adoption | 071 provides a supported disposition; a maintainer decides on any adoption separately | Evaluation is mandatory, but this task set does not authorize numerical product-score integration |

All tasks start `pending`. Select a task for execution only on a subsequent
instruction, complete its prerequisites and record the lifecycle transitions
under [TASK-LIFECYCLE.md](../../TASK-LIFECYCLE.md). Registration does not grant
extra reviewer identities; [REVIEWS.md](../../agent/REVIEWS.md) limits reviews
per authorized request, not per task/PR/package. Reuse reviewers for material
follow-up and apply both stages when executable behavior changes.

Required verification, failed attempts, source/input/model/context/time bindings,
evidence applicability and remaining decisions must accompany each task's result.
Do not mark tasks done from artifact lists, source inspection, Unknown displays or
task counts alone. No automatic scope reduction, task creation or unrelated
remediation is authorized. Product acceptance, support, score adoption and
release/publication remain separate from task completion.
