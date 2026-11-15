# GPT-6 Agent Guidance Optimization

## Timeline and reading this record

All stages below occurred on 2026-11-16, in the order shown. Earlier status
statements and source hashes describe their own stage, not the current state.

1. Initial guidance optimization: both reviews and closure checks accepted.
2. Review-template relocation: implementation, both scoped confirmations and
   closure checks accepted; the stage-by-stage entries are retained below.
3. Duplication cleanup: the latest follow-up is recorded at the end. Its evidence
   identifies the current source; earlier records remain historical.

## Request, baseline and boundaries

The maintainer approved retaining the current document structure, implementing
safe wording changes, and using two independent reviewers even for this prose-only
request. The main agent owns investigation, design, editing and verification.
This record is evidence, not an additional mandatory reading source or task.

- Date: 2026-11-16 (Asia/Tokyo).
- Fetched baseline: `929caed4caf36461ee6d7fc9a81491807b2fdcb4` (`origin/develop`).
- Branch: `codex/gpt6-agent-guidance`.
- Workspace: `/Users/waka/.codex/worktrees/gpt6-agent-guidance/pqc-scan`.
- Source identity: baseline plus file SHA-256 bindings in
  [final structural evidence](evidence/final.json) and the
  [final source index](evidence/final-source.json). HEAD alone does not identify
  these local edits. The source index excludes its own self-hash.
- The original checkout contains unrelated work; it was not copied or edited.
- No product, rules, tests, executable examples, dependencies, CI, scripts,
  packaging, storage formats, user-facing documents or global settings change.
- No new task, Skill, delegated implementation, benchmark or monitoring job.

## Official evidence and repository decisions

Sources were accessible on the date above. The requested Codex landing page
redirects to ChatGPT Learn; the specific configuration guide supplies the relevant
discovery details. No unavailable material is treated as official specification.

| Official source | Confirmed recommendation or behavior | Repository application (our decision) |
|---|---|---|
| [GPT-6 guide](https://developers.openai.com/api/docs/guides/latest-model) | The prompting guidance discusses Astra's tendency to clarify or stop early, explicit authorization and follow-through, skill conflicts, and calibrated testing. It calls for evaluation with the selected model and workload. | Explicitly authorize existing in-scope stages; keep every repository approval and completion gate. Do not hard-code a model or promise the same behavior across the family. |
| [Rethinking skills and prompts for GPT-6 Astra](https://developers.openai.com/blog/rethinking-skills-and-prompts-for-gpt-6-astra) | Use narrow Skill descriptions, progressive disclosure and relevant reading; audit overly prescriptive steps and define completion and safe continuation boundaries. | Retain the existing activity router and evidence-reuse table; clarify continuation rather than remove security checks or reviews. |
| [AGENTS.md format](https://agents.md/) | AGENTS.md is a predictable place for project instructions; it supports ordinary Markdown without a required schema. | Keep the single repository entry point and its existing headings. The four instruction purposes are our classification, not a format requirement. |
| [Codex documentation](https://developers.openai.com/codex/) and [instruction discovery](https://learn.chatgpt.com/docs/agent-configuration/agents-md) | Codex discovers layered global/project instructions. Discovery is distinct from opening linked procedure documents. | Retain explicit activity-based reading triggers and resumption rules. No discovery or model setting change is necessary. |

## Current instruction system and reference relationships

The baseline already includes PR #30's conditional headings, safe evidence reuse,
review records without creating a task, and reviewer continuity. These are useful
existing mechanisms, not newly implemented features or defects to undo.

| Current source | Role and relevant findings | Decision |
|---|---|---|
| [AGENTS.md](../../../AGENTS.md) | Core constraints, outcome, activity router, review limits, approval boundaries and completion summary. Always active. | Add explicit continuation and stopping explanations; preserve router and boundaries. |
| [Verification](../../agent/VERIFICATION.md) | Change categories accumulate; required final-source evidence; reuse invalidation; completion and partial-result rules. Required before changes/completion. | Distinguish development checks from final gates and discourage unjustified repetition. |
| [Reviews](../../agent/REVIEWS.md) | Two stages for executable changes, prose exception, maximum two identities, isolated context, continuity and material-fix confirmation. | Unchanged; maintainer explicitly requests both stages for this work. |
| [Reports](../../agent/REPORTS.md) | Generated-report content and browser rules, conditional on report work. | Unchanged; this developer record is not a generated product report. |
| [Task lifecycle](../../TASK-LIFECYCLE.md) and [queue guide](../../../tasks/README.md) | Authorized task boundaries, state transitions, blockers, cancellation and immutable historical evidence. | Unchanged; use this authorized investigation record, without creating a task. |
| [Best Practices](../../BEST-PRACTICES.md) | Normative development and documentation synchronization. | Unchanged; documentation section applies. |
| [Patterns](../../PATTERNS.md#documentation-synchronization-pattern) | Reusable patterns and the division of core rules, detailed guides and reviewer template. | Unchanged; no term/path relocation or new owner. |
| [Known mistakes](../../MISTAKES.md#documentation-drift) | Historical lessons, including reference drift and feature-specific evidence attribution. | Unchanged; preserve historical language and measurements. |
| [Reviewer template](../../../prompts/REVIEWER.md) | Sole current prompt; stage, original criteria, source/diff, evidence/gaps, follow-up and scoped decision. | Clarify the two stages for prose-only guidance and the limits of structural evidence. |
| [Decision log](../../DECISIONS.md) | Recording template, not evidence of absent architecture decisions. | Unchanged; no product architecture decision. |
| [Restart loop](../../RESTART-LOOP.md) | Pilot-only restart/retry/state exceptions for baseline/task 014. | Unchanged; not extended to this request. |
| [English README](../../../README.md#features), [Japanese README](../../../README.ja.md#features), [architecture](../../architecture.md#workspace-modules), [rules reference](../../rules-reference.md#validation-and-quality-review) | Product context and conditional targets of AGENTS.md. README anchors are explicit HTML IDs. | Valid references; no behavior or rules change. |
| `.github/workflows/pqc-scan.yml`, `.github/workflows/release.yml`, `rust-toolchain.toml` | Actual quality/release gates and pinned Rust 1.93.1; prose paths skip the normal workflow; protocol job is explicitly paused. | Inspect, preserve; a skipped/paused job is not passing evidence. |
| `scripts/package-release.py`, `tests/quality/README.md`, `tests/quality/run.py`, `shell/prepare-code-scanning.cjs`, protocol/distribution checks | Executable packaging and source-bound product measurement tools. | Inspect relevant entry points; do not repurpose product-quality scores as agent-performance evidence. |

The operational flow remains: establish original criteria and applicable reading;
implement and run initial checks; implementation review; fix and complete required
checks; test review; resolve material findings and establish final-source coverage;
main agent decides completion. Resumption restores scope, decisions, evidence,
reviewer identities and next actions before reinvestigation. References are
conditional, cumulative where applicable, and not an instruction to read every
linked document for every edit. Historical role names and removed prompts in old
tasks/records are not current workflow instructions.

### Skills and Codex configuration

Repository inventory found no checked-in `SKILL.md`, `.agents`, `.codex` or
`.claude` configuration and no current prompt besides `prompts/REVIEWER.md`.
The local Codex configuration selects `gpt-6.1-sol` with high effort; its permission
settings are host controls, not permission to bypass repository human decisions.
Global AGENTS.md requests Japanese responses. No settings were changed.

Targeted inspection of the applicable installed `using-superpowers`,
`brainstorming`, `writing-plans`, `using-git-worktrees`, `executing-plans`,
`verification-before-completion` and `requesting-code-review` Skills, including
`using-superpowers/references/codex-tools.md`, found generic requirements for extra
approval, design/ledger files, broader baseline checks or additional delegation.
For example, brainstorming requires design approval before building; its design
storage and builder delegation defaults do not override the maintainer's approved
plan and the repository's no-extra-document/no-delegated-implementation limits.
Execution/review instructions are adapted to the existing two-identity policy;
prose-only verification follows the approved plan. No per-step reconfirmation,
extra agents or redundant product builds are introduced. Irrelevant personal
Skills were not loaded merely because they were installed. External plugin caches
and user Skills remain untouched; the repository cannot establish their behavior
for every future host or model.

## Improvement decisions

Baseline line numbers refer to the fetched revision above. Each implemented item
is a clarification/addition; existing normative paragraphs, commands and reading
triggers are retained. No policy-relaxation candidate was implemented.

| Baseline location | Problem or opportunity | Change and expected effect | Quality/security impact; disposition |
|---|---|---|---|
| `AGENTS.md:10` | Purposes of instructions are implicit and mixed by topic. | Briefly define Constraint, Outcome, Guidance and Verification without changing required wording or headings. | Existing requirements cannot be downgraded; implemented. |
| `AGENTS.md:20` | Safe continuation exists, but does not explicitly cover each authorized stage or minor reversible choices. | Continue implementation, checks, required reviews and fixes without renewed stage approval. | All stop/scope boundaries remain; implemented. |
| `AGENTS.md:30` | Conflict identification lacks an actionable explanation of the stopping source. | Link and quote the applicable instruction, separate interpretation, state the required decision and finish independent authorized preparation. | Does not authorize a gated action or extra delegation; implemented. |
| `AGENTS.md:161` | Resolvable work must continue, but repeating an unchanged failure is not addressed generally. | Require a correction, evidence or changed approach supporting retries. | Preserve task-specific limits and unfinished blockers; implemented. |
| `docs/agent/VERIFICATION.md:29,75` | Development/final distinction and when to stop adding checks are dispersed. | Use targeted checks during development; require all applicable final gates; reuse applicable success without unjustified repetition. | No required check removed or replaced by targeted success; implemented. |
| `prompts/REVIEWER.md:20` | Generic criteria do not explicitly distinguish document meaning from verification or runtime performance. | State prose-only implementation/test review responsibilities and evidence limits. | Same stages, budget and material-fix obligations; implemented. |
| `AGENTS.md:32`, review policy, evidence reuse table and detailed guides | Recent improvements already address conditional reading, attribution and reviewer continuity. | Retain them; no broad reorganization or duplicated procedures. | Preserves established safeguards; no change needed. |

## Acceptance scenarios and policy preservation

These are static semantic assessments of instructions, not agent execution trials.
Automated structural results supplement them; neither establishes runtime gains.

| Scenario | Required interpretation and supporting location |
|---|---|
| Authorized implementation with reversible choices | Proceed through the requested stages within scope; no renewed stage approval. AGENTS.md Working Approach; Workflow and Review Limits. |
| High-risk architecture, unspecified crypto, possibly breaking rules or storage format | Stop affected work for human review; finish independent authorized preparation only. AGENTS.md Security and Compatibility. |
| Correctable failure or repeated failure | Fix in scope and rerun invalidated checks; do not retry unchanged failure blindly; preserve applicable limits. AGENTS.md Completion; lifecycle Corrections and Replanning. |
| Required verification unavailable | Report dependency, attempts, unmet criteria and needed action; remain unfinished. Verification Completion and Partial Results. |
| Prose-only change and mixed executable change | Prose follows reference/consistency verification; mixed work accumulates executable gates and both reviews. This request explicitly requires both reviews. Verification change types; Reviews Required Review Stages. |
| Compaction or resumed work | Restore original criteria, decisions, evidence, reviewer IDs and next action without new tasks or reset budgets. AGENTS.md reading policy; Reviews Revisions and Resumption. |
| Changes after review | Reassess evidence, rerun affected checks and get existing reviewer confirmation when conclusions materially change. Reviews Revisions and Resumption. |
| Completion or performance claim | All original criteria need final-source evidence; structural acceptance is not proof of model performance or product quality. Verification completion; reviewer template. |

The original twelve absolute conditions are retained as follows:

| Condition | Evidence/implementation boundary |
|---|---|
| 1 Security | Security and Compatibility block unchanged; no executable changes. |
| 2 Cryptographic correctness | Same cryptographic checks and human-review requirement. |
| 3 API/output compatibility | Same compatibility guarantees; no interface/output changes. |
| 4 Two-stage independent reviews | Existing review policy unchanged; two explicitly authorized identities for this request. |
| 5 Required tests/verification | All existing requirements/commands retained; final applicability still mandatory. |
| 6 No unverified completion | Existing completion prohibitions retained; pending reviews prevent completion. |
| 7 No unauthorized tasks | No task created or selected; one approved request. |
| 8 Original acceptance criteria | This record covers the original audit, implementation, verification and reporting requirements. |
| 9 Historical evidence | No past task/review/measurement edits. |
| 10 User edits | Fresh worktree from fetched develop; unrelated checkout untouched. |
| 11 No unnecessary product changes | Only three guidance documents and this new record/evidence change. |
| 12 Preserve reasonable mechanisms | Router, detailed policies, current single prompt, reuse table and lifecycle retained. |

## Verification, source binding and reviews

Status: implementation and both independent review stages accepted with no
actionable findings; final structural verification passed. Initial command: `python3 target/agent-guidance-validation/check.py initial`
(exit 0), including `git diff --check` (exit 0), 178 local link checks and six
validator controls. The disposable validator is an ignored local artifact; its
exact source and digest are retained in the JSON evidence, not added as a
repository executable.

Required structural checks cover local targets/anchors (including explicit HTML
IDs), unchanged original normative text, headings, required-reading router,
security/review/approval blocks, existing command fences, unchanged detailed
guides and the allowed file boundary. The validator also exercises missing-target
and missing-anchor rejection, explicit HTML IDs and duplicate heading IDs.
`git diff --check` must pass. Each run records its exact command, source hashes,
exit results and counts in evidence; later changes require applicability review.

Rust builds/tests, product executions and CI are not run: the change is prose-only
and changes no executable examples or acceptance conditions requiring them. They
are not reported as passed. Live agent A/B execution is outside this approved
implementation and remains unmeasured.

Implementation reviewer: `/root/implementation_review`; accepted with no
findings. See the [review record](evidence/implementation-review.json) and
[reviewed file bindings](evidence/implementation-scope.json). The reviewer
confirmed all five file digests. The three guidance documents remain unchanged
since review; subsequent report edits record evidence and status only.

Final structural command: `python3 target/agent-guidance-validation/check.py final`
(exit 0); [evidence](evidence/final.json) records 181 local links, six validator
controls, preserved policies/commands, unchanged related files and allowed scope.
`git diff --check` exited 0. Guidance hashes match implementation review. Report
changes since that check only record its results; references and assessed
instructions are unchanged, so the check remains applicable.

Test reviewer: `/root/test_review`; accepted with no findings. See the
[review record](evidence/test-review.json) and [reviewed bindings](evidence/test-scope.json).
The reviewer verified all eight file digests plus source/diff/validator hashes,
and accepted the static scenarios, original-condition mapping and evidence limits.

No guidance source changed after either review. Subsequent changes only record
review decisions, source identification and closure status in this report and new
evidence. They do not materially affect the reviewed instruction conclusions;
scoped re-review is therefore unnecessary. Closing command:
`python3 target/agent-guidance-validation/check.py closure`; its actual exit status,
links and final-source bindings are in [closure evidence](evidence/closure.json).
The extra check is justified by the newly added review/source-index references.

Main-agent assessment: the original investigation, safe implementation,
structural/semantic verification, two-stage review and reporting deliverables are
satisfied when the closure command passes. There are no unresolved findings or
required deferred checks. No performance improvement is claimed. The prepared
branch remains uncommitted and unpushed; no PR or merge was requested. No task
status or queue was changed. Future execution trials and global-Skill changes
remain proposals, not unfinished deliverables or automatically created work.

## Measurements and future evaluation

Measured guidance sizes (before -> after):

| File | Lines | UTF-8 bytes |
|---|---|---|
| AGENTS.md | 170 -> 187 | 12,273 -> 13,523 |
| Verification | 129 -> 137 | 5,927 -> 6,465 |
| Reviewer prompt | 39 -> 44 | 1,980 -> 2,352 |

These additions make authorization explicit; reducing length is not the goal.
Structural line/byte/link counts are recorded from the real before/after files. They describe document size and reference structure, not context actually
loaded by a model. No token estimate or speed/quality improvement is inferred.

| Runtime metric | This request |
|---|---|
| Task completion rate; unnecessary questions; premature stops | Unmeasured |
| Actual context loaded; tokens consumed; completion time | Unmeasured |
| Duplicate test runs; substantive review defect-detection rate | Unmeasured |
| Quality-gate violations; out-of-scope changes during agent trials | Unmeasured |

For a future comparison, freeze the task prompts, initial source/input fixtures,
model/version, reasoning effort, host settings, tool permissions, Skill inventory,
review policy and environment; change only the guidance revision. Use fresh,
isolated sessions for each arm, balanced execution order and multiple repetitions.
Include representative small edits, executable changes, review-driven fixes and
approval/blocker scenarios. Record original-criterion completion, avoidable
questions/stops, loaded documents, actual usage/time, duplicate checks, valid
review findings, gate violations and out-of-scope edits. Separate necessary
approvals from unnecessary questions; compare task-level results and variability.
Neither a shorter prompt nor faster completion can compensate for a violated
gate. Do not execute such trials or create standing monitoring without authorization.

## Open matters and human decisions

No relaxation of security, approval, compatibility, review or completion policy is
needed for the implemented clarifications. Changes to those policies, enabling
currently paused CI, and editing/disabling global Skills would require separate
scope and appropriate maintainer decisions; none is included or silently queued.
The global Skill conflicts are an environment-level consideration, not an
unresolved permission to weaken this repository. Runtime benefit remains
unmeasured; this does not block the explicitly structural implementation scope.


## Follow-up: review template relocation

### Stage 1: relocation implemented; reviews pending (historical)

The maintainer requested moving the sole review template into `docs/agent/`.
The current template is [docs/agent/REVIEWER.md](../../agent/REVIEWER.md).
Only its two relative links and the current references in AGENTS.md, the review
policy and Patterns change. The empty `prompts/` directory is removed. Review
criteria, budget and all template content remain otherwise unchanged. CI's
legacy `prompts/**/*.md` ignore pattern remains harmless and accurate as a
configured path pattern; `docs/**/*.md` already covers the new location. Therefore
CI and its paired README descriptions need no change.

Earlier sections, hashes, review decisions and old template paths describe the
completed pre-relocation snapshot and are preserved as history, including the
old-path Markdown link in the inventory. That link is historical, not the current
entry point. Earlier evidence is not claimed to identify the relocated source.
New relocation evidence separately checks current references, source bindings,
unchanged template semantics and the allowed change boundary. The same two
reviewers will confirm this material follow-up; no additional identities or tasks
are created. Status: relocation implemented; scoped confirmations pending.


### Stage 2: implementation review accepted

Relocation implementation review: `/root/implementation_review` accepted with no
findings, verifying all five source digests. See
[scoped review](evidence/relocation-implementation-review.json) and
[initial checks](evidence/relocation-initial.json). Final relocation verification
uses `python3 target/agent-guidance-validation/relocation.py relocation-final`;
[its evidence](evidence/relocation-final.json) also checks the unchanged historical
artifacts and original report prefix. The new record is separate from earlier
measurements; no old evidence is overwritten.


### Stage 3: verification and test review accepted

Relocation final checks passed (exit 0, 66 current-document links). The first
attempt failed because the disposable validator read the moved path before its
exclusion; [the failed attempt](evidence/relocation-final-attempt1.json) is retained
separately. Correcting that ordering produced the recorded passing result without
changing guidance. Test reviewer `/root/test_review` accepted with no findings;
see [scoped test review](evidence/relocation-test-review.json). Both reviewers
confirmed the same five guidance hashes; no guidance changed afterward.

### Stage 4: closure verification

Relocation closure uses
`python3 target/agent-guidance-validation/relocation.py relocation-closure`;
[closure evidence](evidence/relocation-closure.json) covers the appended record's
current links as well. On its passing result the relocation request is satisfied:
no review findings or required work remain. Historical source indexes continue to
identify their original snapshots; relocation evidence identifies the new paths.


## Follow-up: duplication cleanup

The maintainer authorized consolidating three minor overlaps while preserving
important rules and all quality gates. AGENTS.md retains the authorized-stage
continuation rule and the independent-preparation rule at the pause boundary;
the duplicate continuation clause is removed from the clarification bullet.
Verification merges the adjacent reuse/no-unnecessary-rerun bullets, retaining
source/input/configuration/environment applicability, reruns after invalidation
and every required check. The reviewer template retains the no-edit/no-delegation
constraint and references the detailed evidence/reproduction policy rather than
copying it. REVIEWS.md and Patterns need no further changes.

The timeline overview and relocation stage headings are additions only. Every
previous report line remains in its original order, including past pending states,
and all earlier evidence files are retained byte for byte. No current rule is
weakened by historical status statements. No product, executable example, CI,
script, task or global Skill is changed. Initial status: editing complete;
scoped implementation review and final verification/test review pending.


### Cleanup stage 2: implementation review accepted

`/root/implementation_review` accepted with no findings, confirming eleven source
hashes, preserved mandatory meaning and chronological clarity. See
[review record](evidence/dedup-implementation-review.json) and
[initial checks](evidence/dedup-initial.json). Final verification command:
`python3 target/agent-guidance-validation/dedup.py dedup-final`;
[final evidence](evidence/dedup-final.json) records the actual results. The next
required stage is scoped test review; no new reviewer identity is used.


### Cleanup stage 3: final verification and test review accepted

Final checks passed (exit 0, 100 links); `/root/test_review` accepted with no
findings, confirming eleven hashes, preserved rule meanings, all fourteen prior
evidence files and every prior report line in order. See
[test review](evidence/dedup-test-review.json). No guidance changed after either
review; this final addition records decisions only and does not require renewed
semantic review. Rust/product/CI checks remain inapplicable to this prose-only
change; no such execution is claimed.

### Cleanup stage 4: closure

`python3 target/agent-guidance-validation/dedup.py dedup-closure` checks the final
report references and unchanged reviewed guidance; the actual exit code, hashes
and counts are in [closure evidence](evidence/dedup-closure.json). With its passing
result, the three requested consolidations and chronological presentation are
complete. There are no unresolved findings or required deferred checks. Earlier
pending statements are historical stage records. No task, policy, product change,
additional reviewer identity, commit or publication was introduced.
