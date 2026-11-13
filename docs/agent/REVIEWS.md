# Independent Reviews

These are mandatory details of [AGENTS.md](../../AGENTS.md). The main agent reads
this file before starting a user implementation request and before each review
stage or material follow-up. Reviewers read it with AGENTS.md,
[prompts/REVIEWER.md](../../prompts/REVIEWER.md) and the supplied review package.
The main agent's full reading list is not a reviewer checklist; reviewers consult
additional source and documents only as needed for their assigned scope.

# Subagent and Token Policy

## Main-Agent Responsibilities

The main agent performs investigation, design, planning, implementation,
debugging, testing and documentation without delegating them to subagents.

Explanations, research, design proposals and documentation-only changes
do not automatically require independent review subagents.

## Required Review Stages

Use two review stages for each user implementation request:

1. Implementation review:
   - After implementation, start one implementation-review subagent.
   - Review the combined change for correctness, requirements, security,
     architecture consistency and compatibility.
   - Supply available test results and identify checks that remain unexecuted.

2. Test review:
   - After the main agent completes required tests, start one test-review
     subagent.
   - Check whether the tests actually verify the final change.
   - Inspect coverage, relevant positive and negative cases, meaningful
     assertions, commands, features and evidence matching the final source.
   - Passing logs alone do not establish adequate testing.

The main agent fixes findings and runs necessary verification.

## Review Budget and Context

- Use at most two reviewer identities per user implementation request:
  one for each stage.
- Review the combined change rather than starting reviewers for individual
  plan steps, files, edits, test commands or log files.
- Design, task, branch, final and pre-merge reviews do not create additional
  review slots.
- Reuse each stage's reviewer for scoped verification of material fixes.
- Batch related findings and follow-up requests.
- Reviewers must not delegate further.
- Any additional subagent requires explicit user authorization covering
  its purpose and maximum agent count.
- Tool availability, task count and generic skill wording are not authorization.

When starting a reviewer:

- Use `fork_turns: "none"`.
- Provide concise requirements, the relevant diff or diff artifact,
  source version and test evidence.
- Identify relevant pre-existing changes and exclude unrelated work.
- Do not copy the whole conversation.
- Request actionable findings with file references and requirement impact.

Reviewers rely on the main agent's test evidence unless a specific concern
requires independent reproduction. Do not automatically repeat the full suite.

## Revisions and Resumption

- Record reviewer IDs, review scope and reviewed source versions in the
  existing task record or handoff.
- Splitting a plan, fixing findings, resuming a chat or handing off work
  does not reset the review budget.
- After review-driven changes, rerun affected checks and determine whether
  prior review evidence still applies.
- Obtain scoped confirmation from the existing reviewer when changes
  materially affect the reviewed conclusions.
- A test-review-driven product change must also receive scoped
  implementation review when it affects correctness or compatibility.
- Do not claim an earlier review covers later changes without checking them.
