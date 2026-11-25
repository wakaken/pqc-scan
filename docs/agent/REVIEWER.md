> **Inactive during the Open Pstack trial.** [AGENTS.override.md](../../AGENTS.override.md)
> selects pstack as the development harness. This file preserves the legacy two-stage review request template
> for historical interpretation and rollback; its execution instructions are not
> active, including when an old task or skill links here. Original task acceptance
> criteria and evidence remain binding; retirement of a process does not complete
> or cancel any task.

# Independent Review Request Template

The main agent fills the request below for each required review stage and sends
it to that stage's reviewer. Use the same template for both stages; do not combine
reviewer identities. Read [AGENTS.md](../../AGENTS.md) and the
[review policy](REVIEWS.md) for criteria, budget and revision handling.

## Request

- Stage: implementation review or test review
- Requirements: original acceptance criteria, scope and compatibility constraints
- Source: baseline/current revision and relevant uncommitted changes
- Change: relevant diff or diff artifact, affected files and pre-existing changes
- Evidence: commands, features/configurations, results and artifact paths
- Gaps: failed, unavailable or unexecuted required checks
- Follow-up, if applicable: previously reviewed source, changes since that review,
  resolved/unresolved findings, affected conclusions, rerun checks and reasons
  that reused evidence still applies

## Reviewer Instructions

- Apply the assigned stage's criteria in `docs/agent/REVIEWS.md` and the applicable
  completion/verification gates in `docs/agent/VERIFICATION.md`. Read additional
  source or documentation only as needed to evaluate the supplied scope.
- Do not modify files or delegate. Follow the test-evidence and reproduction
  rules in the [review policy](REVIEWS.md#review-budget-and-context).
- For follow-up, check material fixes and whether evidence covers the final
  source. Do not assume an earlier review covers later changes.
- For prose-only guidance changes requiring review, implementation review checks
  instruction meaning, reference relationships and preservation of mandatory
  gates. Test review checks whether structural and consistency verification
  establishes those requirements for the final source. Structural checks alone
  do not demonstrate improved agent execution performance.

## Response

- Reviewed stage, source version and scope
- Actionable findings: severity, file/line, requirement impact and reasoning
- Material coverage gaps or unknowns, including missing required evidence
- Acceptance decision for the assigned scope, with reasons

Explicitly state when no actionable findings were identified. Review acceptance
is not task completion; the main agent applies all completion gates in AGENTS.md.
