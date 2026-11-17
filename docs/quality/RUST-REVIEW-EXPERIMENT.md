# Rust review supplement comparison proposal

Status: 24 app trial reports collected; independent human evaluation is pending.
No CLI or direct API experiment is authorized.
Date: 2026-11-17. Following the user's selection of existing historical source,
the [six-case dataset](../../tests/quality/rust-review-pilot-v1/README.md) now
provides frozen request packets, provenance, draft truth, schedule, execution
procedure and cost estimate. The [short supplement](../agent/RUST-REVIEW-SUPPLEMENT.md)
remains unchanged. The user selected this app and authorized the explained
24-chat comparison on 2026-11-17 after the method update. Installation and
Clippy/CI/product changes remain excluded.

## Question and scope

Does the short supplement improve Codex's actionable Rust review findings beyond
the existing project instructions, without increasing unsafe/incompatible advice
or disproportionate review cost? Measure review behavior on fixed patches, not
scanner detection accuracy or generated-code quality. Those require separate
experiments; this pilot cannot establish improvement in Q1-Q6 product outcomes.

This design was prepared against HEAD
`f07d782cd68cfac95d93585d5ace049f590160d4` plus the existing dirty worktree.
That revision alone does not identify the future experiment source. Freeze exact
input bytes, uncommitted changes and tool/configuration bindings before execution.

## Conditions and common review package

| Arm | Material supplied |
|---|---|
| A: existing review | Existing project policy, template, scoped source/diff and evidence only. |
| B: supplemented review | Byte-identical common package, plus the frozen supplement explicitly named as an optional reference. |

The prepared packets embed the historical `prompts/REVIEWER.md` as their only
review prompt, with stage set to implementation review. The live checkout now
uses [docs/agent/REVIEWER.md](../agent/REVIEWER.md). Do not silently regenerate
packets or claim that frozen results evaluate the newer guidance. Both arms
receive baseline copies of
[AGENTS.md](../../AGENTS.md), [review policy](../agent/REVIEWS.md),
[verification rules](../agent/VERIFICATION.md), [best practices](../BEST-PRACTICES.md),
and the same scoped architecture/pattern/contract excerpts where relevant.
Both receive the same original requirements, baseline/current source, diff,
pre-existing-change exclusions, supplied command/exit/feature evidence and gaps.
No arm receives another arm's report, truth labels, grading rubric or this design.

The active method is the [Codex app runbook](../../tests/quality/rust-review-pilot-v1/RUNBOOK.md).
Send one unchanged packet per fresh projectless app chat, without this parent
conversation, repository, labels or another arm. No direct API or CLI execution,
API-key setup, skill installation or trial subagent is part of the app run.
Inspect visible tool/turn histories for compliance with the packet's prohibition
on tools, lookup, delegation and superpowers. This is context separation, not
verified OS isolation. Hidden app configuration and unavailable model/settings
remain Unknown and qualify causal conclusions. Do not claim the earlier CLI
isolation conditions were met. Preserve all original A/B packet bytes.

## Six-case pilot

Use three historical before/after source pairs, as selected by the user after
this proposal's initial synthetic-case version. The original proposed synthetic
trait/privacy/API cases have not been implemented or measured. The prepared
cases use exact Git line ranges as inert `.rs.txt` excerpts, without modifying
current Rust or inventing defects. Review scoped source snapshots, not a new
implementation diff. Neutral case IDs and source maps are in the dataset.

| Pair | Historical fix | Nominated defect and repaired control |
|---|---|---|
| c17 / c42 | f993691 | Unframed finding-ID tuple boundaries / framed tuple encoding. |
| c63 / c08 | b0e386e | Unindented embedded Markdown source lines / normalized indented continuation lines. |
| c29 / c51 | 35d988e | Native unknown zero emitted as a SARIF coordinate / unknown fields omitted. |

The complete [truth and provenance](../../tests/quality/rust-review-pilot-v1/README.md)
are operator-only. Each control asserts only absence of the nominated defect,
not that the historical file is generally clean. No current product defect or
browser exploit is inferred from old source. The main agent has prepared the
labels from source and historical repair evidence. The user's start instruction
authorizes using this fixed reference for the app run; record the digest and
authorization separately from historical label metadata. This does not establish
independent human relabeling. Human outcome adjudication remains required before
a definitive quality evaluation decision.
These public, targeted, correlated examples are not representative or independent
holdout, and may already be known to the tested model.

## Run order, isolation and bounds

Proposed pilot: 6 cases x 2 arms x 2 repeats = 24 review trials, run sequentially.
Use a fresh independent context for each trial; no earlier outputs or conversation
history. Pin model identifier, reasoning setting, client version, tool allowlist,
context/output limits and relevant sampling settings; record unavailable settings.
Randomize case order with a frozen seed (proposed: 20261117). For each case,
counterbalance A/B order between the two repeats. Retain the generated schedule.
Two repeats are descriptive stability checks, not six independent populations.

Use the configured app default model/settings without an unrequested override;
record exposed metadata and retain unavailable fields as Unknown. The user
approved the explained 24 new trial chats. They do not reset the ordinary
[reviewer budget](../agent/REVIEWS.md) or authorize trial subagents.

Five minutes and 4,000 visible output tokens are observation budgets, not hard
app limits. Use existing account allowance with no purchases or resets. Read
usage before launch and after each six completions. Stop further launches when
ordinary usage is unavailable, payment/reset is required, or reported remaining
allowance reaches 10 percent. Preserve errors, unfinished/overrun reports and
tool violations. No retries or extra attempts. See the
[app usage budget](../../tests/quality/rust-review-pilot-v1/COST.md).

## Blind scoring

Assign each report an opaque ID, remove arm/session metadata, and shuffle reports
before a human scores them against frozen truth. Do not rewrite findings or strip
their substantive reasoning. A report may reveal the treatment through its wording;
record suspected unblinding. Do not use the tested model as the sole judge.

Match findings one-to-one to root causes using the declared trigger, consequence
and compatible source location. Equivalent wording is acceptable; a rule citation
alone is not a true positive. Merge repeated findings on one cause and retain
the duplicate count. Classify every other finding as unsupported/false, a newly
verified defect, optional advice, or unresolved. Verify novel claims without arm
labels; preserve original truth and add an adjudication record. An unresolved
claim blocks a definitive precision comparison; it is never silently discarded.

| Measure | Calculation or recorded observation |
|---|---|
| Review precision | TP / (TP + FP) over unique actionable claims; report counts and unresolved claims. Zero denominator is null. |
| Review recall | Matched frozen defects / frozen defects on defective cases; separately list each missed identity/Markdown/unknown-coordinate defect. |
| Clean-control false alarms | Clean cases with at least one unjustified actionable claim / 3, separately for each repeat. |
| Harmful advice | Required fixes that expose sensitive data, weaken bounds/uncertainty, or break a public contract; optional advice is separate. |
| Finding usefulness | For each TP, record location, causal/impact explanation, requirement/contract basis and compatible fix guidance as separate yes/no observations. |
| Review overhead | Duplicate claims, optional style claims, human grading/triage minutes and tool calls. |
| Model cost and latency | Input/cached/output tokens, actual charge or declared-rate estimate, wall time and timeout/tool-error counts. |

Precision uses the final blind adjudication of actionable claims, including newly
verified defects; recall retains the original frozen denominator. If novel defects
invalidate a clean case, mark it invalid, retain all reports, and withhold the
pilot decision until a separately authorized amended experiment is run.
Report per-case/per-repeat counts before aggregate rates. Repeated observations
of the same defect do not become independent truth units. Do not use the scanner's
existing TP/FP matcher to score prose, change its protocol, or mix these counts
with product Q1 metrics.

## Decision and evidence

This is a proposed experiment decision rule, not a numeric CI/release gate.
Recommend a limited continuation only when the experiment is valid and complete,
B introduces no harmful advice or new missed nominated defect, and B improves
recall without lowering precision in both repeats. Require a human judgment that
the recorded token/time/triage increase is worth the gain. Otherwise record no
demonstrated benefit, a regression, or an inconclusive result; do not install a
skill or add lints just because the pilot ran. A tie supports keeping the current
workflow. Do not tune the supplement and retest on the same six cases as holdout.

The [operator runbook](../../tests/quality/rust-review-pilot-v1/RUNBOOK.md) now
uses native app thread creation/status tools. Preserve the former CLI proposal
and original manifest in the
[historical preparation](../records/rust-review-app-20261117/historical-preparation/).
Store current receipts and actual outcomes under
`docs/records/rust-review-app-20261117/`, since no task record exists. No new task
is created. Bind packet/truth hashes, thread/host IDs, raw creation/status/final
receipts, visible tool history, usage snapshots, unknowns and later blind scoring.
Missing telemetry is not zero cost. Model reports do not establish human
adjudication or eliminate hidden-instruction uncertainty.

Do not modify the current evidence catalog, historical results or quality gates.
Later catalog enrollment would be a separate authorized change with source-bound
evidence, following the [measurement plan](CI-MEASUREMENT-PLAN.md).
Coverage is limited to these three defect families; privacy completeness, API
migration, parser budgets, unsafe, concurrency, cryptographic judgment and product
performance remain Not Tested by
this pilot. A separate fresh-case study is needed before broad claims of review
accuracy, and a later implementation study before claims of improved Rust code.

## Branch relocation (2026-11-17)

The user requested a separate branch from `origin/develop`. The prepared 54 files
were preserved byte-for-byte in commit `8ab51a8` on
`codex/beautifulrust-review-pilot`, based on develop commit
`af472b23c03d096ae6709edd94096fe4a3af813c`. The dataset, truth, packets and original
preparation/review receipts remain unchanged; their historical source baseline
is still `f07d782cd68cfac95d93585d5ace049f590160d4` plus the recorded worktree.
Only these live documentation references were subsequently aligned with the
relocated review template. Current guidance differs from the frozen baseline;
a comparison against updated guidance would require separately prepared inputs.

Source excerpts retain original trailing blank lines, including the two SARIF
type excerpts. They are immutable source data, not whitespace cleanup targets.
The transfer check ignores only Git's blank-at-EOF warning for those bytes;
ordinary authored-document whitespace checks still apply. No experiment,
installation, Clippy/CI/product change or additional review identity was added.

## App execution authorization (2026-11-17)

The user selected this app and permitted experiment start after the method
update. This supersedes the preparation-only hold and CLI/API budget proposal.
The six cases, unchanged A/B packets, 24-attempt maximum, no-install/no-Clippy/
no-CI restrictions, historical evidence and human outcome-adjudication condition
remain. This is an app pilot with explicit control limitations, not a claim of
verified hard spending limits or OS isolation.

The [app collection results](../records/rust-review-app-20261117/RESULTS.md) retain
all 24 reports and observed limitations. Collection completion is not final
quality acceptance, and no supplement adoption decision has been made.
