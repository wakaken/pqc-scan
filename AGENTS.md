# AGENTS.md

Respond to the user in clear, concise and polite Japanese.

**pqc-scan** is a Rust CLI static analyzer of Post-Quantum Cryptography (PQC)
migration risks. It scans repositories for cryptographic usage using rules and
produces migration reports. Security, correctness, backward compatibility and
stable output are critical.

## Working Approach

Apply instructions by their purpose: **Constraints** are mandatory boundaries;
**Outcomes** are required results; **Guidance** recommends methods within those
boundaries; **Verification** defines required evidence and completion gates.
This distinction does not downgrade existing requirements or create exceptions.

- The main agent investigates, plans, implements, debugs, tests, evaluates and
  documents. Planner, Executor and Evaluator are responsibilities, not processes
  or separate prompts. The only review prompt is `docs/agent/REVIEWER.md`.
- Prefer small, safe changes; avoid unrelated refactoring and preserve user edits.
- Clear, small changes need no separate design or implementation plan.
  Explanations, research and small changes need no separate task record.
- For complex or ambiguous work, first summarize purpose, acceptance criteria,
  constraints, compatibility, viable approaches, recommendation, steps and checks.
- Reuse applicable plans, task records and evidence. Ask about unresolved decisions
  materially affecting correctness, compatibility, security or scope.
- For authorized work, continue through implementation, applicable tests,
  required reviews, fixes and final verification without asking for approval at
  each stage. Resolve minor, reversible choices within scope; a plan or first
  implementation alone does not fulfill an implementation request.
- Do not change agreed scope or acceptance criteria without explicit agreement.
- Do not select unrelated work or create tasks because the queue is empty.
  Task creation and scope expansion require authorization. Report out-of-scope
  findings as proposals, not new tasks.
- Monitor only explicitly requested signals within the authorized scope and
  duration. Do not automatically generate tasks from reflection or monitoring.
- Saved plans, handoffs and skills must obey these limits and completion gates.
- Identify unresolved instruction conflicts before performing dependent work.
- When a rule or skill requires a pause or human decision, identify and link the
  exact source, quote the relevant instruction and explain the decision needed.
  Distinguish an explicit requirement from interpretation; complete independent,
  authorized preparation without crossing the approval boundary. Skills do not
  authorize scope expansion, delegation or bypassing repository constraints.

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
| Any change; verification; completion decision | [Verification by change type](docs/agent/VERIFICATION.md#verification-by-change-type), evidence and completion rules in that guide |
| Product implementation, Rust tests, Cargo, rules, CI, scripts or packaging changes | [Best practices](docs/BEST-PRACTICES.md); relevant sections of [patterns by activity](docs/PATTERNS.md#activity-index) and [known mistakes](docs/MISTAKES.md) |
| Understanding or changing product behavior, components or interfaces | Relevant sections of [English README](README.md#features), [Japanese README](README.ja.md#features), [architecture](docs/architecture.md#workspace-modules), and affected user guides/designs |
| Design or architecture decisions | Relevant sections of [architecture](docs/architecture.md#workspace-modules), recorded decisions in the [decision log](docs/DECISIONS.md), [patterns by activity](docs/PATTERNS.md#activity-index) and [known mistakes](docs/MISTAKES.md); use the decision template only when recording an agreed decision |
| User implementation request; independent review or material follow-up | [Review policy](docs/agent/REVIEWS.md#required-review-stages); reviewers also read this file, [review template](docs/agent/REVIEWER.md) and their review package |
| Creating, updating, resuming or closing task records | [Task lifecycle definitions](docs/TASK-LIFECYCLE.md#status-definitions) and [task layout and fields](tasks/README.md) |
| Rule changes | [Rule validation and quality review](docs/rules-reference.md#validation-and-quality-review) and affected rule entries |
| Report content, presentation, interaction or browser acceptance | [Report content](docs/agent/REPORTS.md#user-facing-report-content) and [browser verification](docs/agent/REPORTS.md#local-report-browser-verification), plus affected report/user/design documentation |
| Agent guidance or workflow documentation changes | [Documentation best practices](docs/BEST-PRACTICES.md#documentation), [documentation synchronization](docs/PATTERNS.md#documentation-synchronization-pattern), and every affected current guidance document |
| Restart baseline or task 014 only, unless explicitly extended by the user | [Restart loop](docs/RESTART-LOOP.md) |

- Read required material once per session and reuse it; revisit changed or
  uncertain sections. Use targeted searches, not repeated whole-directory reads.
  Linked headings are entry points; read all other applicable sections, not only
  the landing section.
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
- Requests changing product code, tests, rules, Cargo/dependencies, CI, scripts,
  packaging or other executable behavior require two independent stages:
  implementation review after implementation, then test review after required tests.
  Prose-only documentation changes are normally exempt; mixed requests and
  executable examples require both stages. Explicit review requirements still apply.
- Use at most two reviewer identities per request, one per stage. Review the
  combined change; reuse each reviewer for material fixes. Reviewers cannot delegate.
- Extra agents require explicit user authorization of purpose and maximum count.
  Tools, task count and generic skill/delegation wording are not authorization.
- Splitting plans, fixing findings, resuming or handing off never resets the budget.
- Follow [review details](docs/agent/REVIEWS.md), including isolated reviewer context,
  evidence, [recording without a task](docs/agent/REVIEWS.md#review-records),
  reviewer continuity and scoped confirmation after material fixes.
- Tasks live in `tasks/<ID>/task.yaml`; plans, results and reviews belong in the
  same task directory, with raw evidence under `evidence/`. Normal states are
  pending → planned → in-progress
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

- Maintain English and Japanese versions of end-user documentation: the root
  `README.md` (English), root `README.ja.md` (Japanese), and documents under
  `docs/user/`. Keep paired versions factually consistent and update both for
  user-facing behavior changes. Report format changes also require applicable
  report docs, subject to the human review conditions above.
- Write developer-only documentation in English: `tasks/`, `docs/agent/`,
  `docs/quality/`, `docs/records/`, `docs/roadmap/`, `docs/architecture/`,
  `docs/product/`, `docs/releases/`, and all other files under `docs/` outside
  `docs/user/`. This includes explanatory prose in task definitions and results.
- Apply these language rules to newly authored or revised explanatory prose.
  Do not translate code, commands, identifiers, paths, structured data or quoted
  material to satisfy this policy. These rules do not change the requirement to
  communicate with the user in Japanese.
- Update affected agent references together when terms or paths change.
- This file owns core constraints and summarizes detailed rules. Linked guides
  own verification/completion (`docs/agent/VERIFICATION.md`), reviews
  (`docs/agent/REVIEWS.md`), reports (`docs/agent/REPORTS.md`) and task lifecycle
  (`docs/TASK-LIFECYCLE.md`) details. Core constraints always apply; update affected
  summaries, details and references together in the same change. Resolve conflicts
  before dependent work; neither location permits weakening the other's constraints.
- Preserve past reviews, completed tasks and measurements, including their
  original languages and wording; the language policy does not authorize
  translating or rewriting historical records or evidence. Historical role names
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
- Base retries on a correction, new evidence or a changed approach; do not repeat
  an unchanged failed attempt without a reason to expect progress. Preserve any
  applicable task-specific retry limits and keep blocked work unfinished.
- Reopening incorrectly closed work preserves history and reconciles queue/roadmap;
  cancellation requires a maintainer decision and reopening an explicit new request.
- Report in concise, polite Japanese: resulting behavior, why it changed, actual
  verification and remaining work. Distinguish implementation, measurements,
  proposals and unexecuted checks. Provide source version, commands, exit results
  and artifact links for changes/measurements; summarize routine successful checks.
- Claim completion only against original criteria. Lead partial results with the
  unfinished status and reason; never describe unfinished work as complete.
