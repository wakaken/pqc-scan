# Restart pilot

This protocol applies only to the restart baseline and task 014. Run roles
sequentially in one Codex session. A role transition is not an independent review.
Do not start parallel agents, add tasks automatically, or change frameworks.

## Entry gate

Before task 014 implementation, record a baseline HEAD, worktree changes, build,
format check, workspace tests, clippy, synthetic acceptance, and all-format report
inspection. Record CI evidence with its exact HEAD; historical success is not CI
validation of a changed worktree. HTML interaction requires browser or user
confirmation. Obtain the authorized real-product path before its acceptance scan.
Missing evidence stays unexecuted; it never counts as success.

## Role gates

- Planner: select only 014 after baseline acceptance; fix its scope and acceptance
  criteria before setting planned. CLI filters, their tests, and both READMEs are
  the allowed feature scope. Preserve scan rules and report formats.
- Executor: set in-progress, implement with targeted tests, and set review only
  after targeted verification. Do not overwrite unrelated edits.
- Reviewer: inspect correctness, compatibility, scope, and meaningful coverage.
  Record findings and their resolution, even when the same session changes roles.
- Evaluator: require review acceptance, build, formatting, workspace tests, clippy,
  CLI usage, report comparison, real-product acceptance, and CI on the final code
  revision before done. Return failures to pending for Planner reassessment.
- Reflection: record durable lessons in existing knowledge documents without
  automatically creating tasks. Monitor is disabled during this pilot.

## Retry and stop rules

Allow at most two corrective cycles after the initial implementation. Record the
counter before each cycle; failed validation reruns consume the same budget.
After exhaustion, set blocked and return the failure evidence to the user.
Unavailable verification, conflicting user edits, scope expansion, rule or report
compatibility changes, and cryptographic specification changes require a stop and
human decision. Environment repairs need explicit authorization. Do not silently
relax gates or repeatedly recycle a failed task.

## Resume record

Keep a restart_record mapping in the selected task YAML: baseline/current HEAD,
worktree changes, scope, acceptance criteria, command and exit status, artifact/log
paths, review findings, corrective cycle count, unexecuted checks, blocker, and
next action. Never put secrets or real key material in logs. Generated reports
belong under target, outside the scanned repository. If artifacts are unavailable,
record that limitation instead of claiming their contents were inspected.

## Task 014 acceptance

Use single-valued --kind (regex, tree_sitter, dependency, certificate, key) and
--severity (info, low, medium, high, critical). Severity is exact, combined filters
use AND, zero matches succeed, and invalid values fail. Preserve unfiltered output
and row order/format. Filtered summaries show total loaded and displayed counts;
kind counts describe displayed rules. Filters never affect scan execution.

Test individual/combined filters, zero results, invalid values, and unfiltered
compatibility. Exercise scan -> rules list -> rules show -> HTML evidence. Account
for report severity adjustment when comparing it to rule severity. Compare stable
report content before/after, excluding timestamps and ordering of equivalent hits.
Use synthetic acceptance plus one user-authorized real product. New detection is
outside this pilot; any later detection task must define positive, negative, and
existing-output regression cases before implementation.
