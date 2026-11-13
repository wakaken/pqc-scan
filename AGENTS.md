# AGENTS.md

Respond to the user in clear, concise and polite Japanese.

**pqc-scan** is a Rust CLI static analyzer of Post-Quantum Cryptography (PQC)
migration risks. It scans repositories for cryptographic usage using rules and
produces migration reports. Security, correctness, backward compatibility and
stable output are critical.

## Working Approach

- The main agent investigates, plans, implements, debugs, tests, evaluates and
  documents. Planner, Executor and Evaluator are responsibilities, not processes
  or separate prompts. The only review prompt is `prompts/REVIEWER.md`.
- Prefer small, safe changes; avoid unrelated refactoring and preserve user edits.
- Clear, small changes need no separate design or implementation plan.
  Explanations, research and small changes need no separate task record.
- For complex or ambiguous work, first summarize purpose, acceptance criteria,
  constraints, compatibility, viable approaches, recommendation, steps and checks.
- Reuse applicable plans, task records and evidence. Ask about unresolved decisions
  materially affecting correctness, compatibility, security or scope; continue
  authorized work that does not depend on them.
- Do not change agreed scope or acceptance criteria without explicit agreement.
- Do not select unrelated work or create tasks because the queue is empty.
  Task creation and scope expansion require authorization. Report out-of-scope
  findings as proposals, not new tasks.
- Monitor only explicitly requested signals within the authorized scope and
  duration. Do not automatically generate tasks from reflection or monitoring.
- Saved plans, handoffs and skills must obey these limits and completion gates.
- Identify unresolved instruction conflicts before performing dependent work.

## Required References by Activity

This file and the applicable documents below define mandatory working rules.
The table is for the main agent; reviewers follow the scoped reading policy in
[review details](docs/agent/REVIEWS.md). Before the corresponding activity, read
its references; combine rows when several apply. References are not automatically
loaded: open them explicitly. If scope expands, read newly applicable references
before the affected work.

| Activity | Required references |
|---|---|
| Any repository work | This file; selected task, original acceptance criteria and applicable plans, if any |
| Any change; verification; completion decision | [Verification and completion](docs/agent/VERIFICATION.md) |
| Product implementation, Rust tests, Cargo, rules, CI, scripts or packaging changes | [Best practices](docs/BEST-PRACTICES.md); relevant sections of [patterns](docs/PATTERNS.md) and [known mistakes](docs/MISTAKES.md) |
| Understanding or changing product behavior, components or interfaces | Relevant sections of [English README](README.md), [Japanese README](README.ja.md), [architecture](docs/architecture.md), and affected user guides/designs |
| Design or architecture decisions | Relevant sections of [architecture](docs/architecture.md), [decisions](docs/DECISIONS.md), [patterns](docs/PATTERNS.md) and [known mistakes](docs/MISTAKES.md) |
| User implementation request; independent review or material follow-up | [Review policy](docs/agent/REVIEWS.md); reviewers also read this file, [review template](prompts/REVIEWER.md) and their review package |
| Creating, updating, resuming or closing task records | [Task lifecycle](docs/TASK-LIFECYCLE.md) and [task queue guide](tasks/README.md) |
| Rule changes | [Rules reference](docs/rules-reference.md) |
| Report content, presentation, interaction or browser acceptance | [Report and browser rules](docs/agent/REPORTS.md), plus affected report/user/design documentation |
| Agent guidance or workflow documentation changes | [Best practices](docs/BEST-PRACTICES.md), documentation synchronization in [patterns](docs/PATTERNS.md), and every affected current guidance document |
| Restart baseline or task 014 only, unless explicitly extended by the user | [Restart loop](docs/RESTART-LOOP.md) |

- Read required material once per session and reuse it; revisit changed or
  uncertain sections. Use targeted searches, not repeated whole-directory reads.
- After compaction or handoff, restore scope, decisions, evidence, reviewer IDs
  and next actions from the existing task record or handoff before reinvestigating.
- Conditional reading never reduces required verification or relevant evidence.
- Apply restart-loop entry, retry, status-transition and completion gates only
  within its stated scope; monitoring and automatic task creation remain disabled
  during that pilot.

## Workflow and Review Limits

1. Establish outcome, original criteria, applicable task and existing evidence.
2. Implement minimal changes and meaningful tests, run initial checks and inspect
   the combined diff before implementation review.
3. Obtain required implementation review, fix findings, complete required tests,
   then obtain required test review. Resolve findings and verify final-source evidence.
4. Apply all completion gates; record durable lessons in existing knowledge docs
   only when useful. Explanations, research and small changes need not use every step.

- Main-agent investigation, design, implementation, testing and documentation
  must not be delegated. Explanations, research, design proposals and
  documentation-only changes do not automatically require review subagents.
- Each user implementation request requires two independent stages: implementation
  review after implementation, then test review after required tests.
- Use at most two reviewer identities per request, one per stage. Review the
  combined change; reuse each reviewer for material fixes. Reviewers cannot delegate.
- Extra agents require explicit user authorization of purpose and maximum count.
  Tools, task count and generic skill/delegation wording are not authorization.
- Splitting plans, fixing findings, resuming or handing off never resets the budget.
- Follow [review details](docs/agent/REVIEWS.md), including isolated reviewer context,
  evidence, ID recording and scoped confirmation after material fixes.
- Tasks live in `tasks/*.yaml`. Normal states are pending → planned → in-progress
  → review → done; blocked and cancelled are distinct optional states. Follow
  [task lifecycle](docs/TASK-LIFECYCLE.md) for transitions, failures and resumption.
- Default to one task per PR, including implementation, tests and docs. Plan steps
  do not create separate tasks, PRs or reviewers. Keep prerequisite feasibility
  studies separate when their results determine whether/how implementation proceeds.

## Security and Compatibility

- Validate inputs; never expose secrets, private key material or sensitive
  cryptographic data in output or logs. Never weaken cryptographic checks.
- Preserve cryptographic integrity, API compatibility and external output contracts.
- An absent detection is not proof of safety or full support unless specified.
- Rules are in `rules/default/*.yml`: maintain unique IDs and valid schemas, test
  matching/non-matching inputs, assess compatibility and update the reference as needed.

Stop affected work and obtain human review for:

- high-risk architectural changes;
- cryptographic logic changes without a clear specification;
- rule changes that could break compatibility;
- storage format changes.

Preserve work and evidence; explain the required decision before proceeding.
Human review is separate from independent reviews and grants no extra agent slots.

## Documentation

- `README.md` is English; `README.ja.md` is Japanese. Update both for user-facing
  behavior changes; report format changes also require applicable report docs,
  subject to the human review conditions above.
- Update affected agent references together when terms or paths change.
- Preserve past reviews, completed tasks and measurements. Historical role names
  and paths describe their time, not instructions to restore removed prompts.
- Use existing knowledge documents for durable lessons; avoid duplicate knowledge
  and unnecessary design/reflection docs. Documentation never replaces validation.

## Completion and Communication

- Use `rust-toolchain.toml` and the applicable checks in
  [verification rules](docs/agent/VERIFICATION.md); optional features need their
  own applicable checks. Documentation-only work normally needs no Rust checks.
- Mark done only when every original criterion/deliverable is implemented and
  verified against final source, including required reviews. Partial, Unknown,
  Not Tested, failed or deferred required work prevents completion.
- Artifact lists, passing logs or limited reviews do not cover requirements
  outside their scope. Never narrow criteria or move work elsewhere to claim done.
- Evaluation completion and measured product quality are different; incomplete
  truth, missing measurements and unverified assessment validity stay unfinished.
- Continue resolvable work. For blockers, keep work unfinished and record the
  dependency, attempts, unmet criteria and concrete action/decision needed.
- Reopening incorrectly closed work preserves history and reconciles queue/roadmap;
  cancellation requires a maintainer decision and reopening an explicit new request.
- Report in concise, polite Japanese: resulting behavior, why it changed, actual
  verification and remaining work. Distinguish implementation, measurements,
  proposals and unexecuted checks. Provide source version, commands, exit results
  and artifact links for changes/measurements; summarize routine successful checks.
- Claim completion only against original criteria. Lead partial results with the
  unfinished status and reason; never describe unfinished work as complete.
