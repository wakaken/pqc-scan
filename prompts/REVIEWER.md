# Independent Review Request Template

The main agent fills the request below for each required review stage and sends
it to that stage's reviewer. Use the same template for both stages; do not combine
reviewer identities. Follow AGENTS.md for criteria, budget and revision handling.

## Request

- Stage: implementation review or test review
- Requirements: original acceptance criteria, scope and compatibility constraints
- Source: baseline/current revision and relevant uncommitted changes
- Change: relevant diff or diff artifact, affected files and pre-existing changes
- Evidence: commands, features/configurations, results and artifact paths
- Gaps: failed, unavailable or unexecuted required checks
- Follow-up, if applicable: prior findings, material fixes and affected conclusions

## Reviewer Instructions

- Apply the assigned stage's criteria in AGENTS.md. Read additional source or
  documentation only as needed to evaluate the supplied scope.
- Do not modify files or delegate. Rely on supplied test results unless a
  specific concern requires independent reproduction; do not automatically
  repeat the full suite.
- For follow-up, check material fixes and whether evidence covers the final
  source. Do not assume an earlier review covers later changes.

## Response

- Reviewed stage, source version and scope
- Actionable findings: severity, file/line, requirement impact and reasoning
- Material coverage gaps or unknowns, including missing required evidence
- Acceptance decision for the assigned scope, with reasons

Explicitly state when no actionable findings were identified. Review acceptance
is not task completion; the main agent applies all completion gates in AGENTS.md.
