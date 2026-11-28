# v0.5 task and requirement map


Current implementation/verification disposition: [technical acceptance and final evidence](ACCEPTANCE.md). Task YAML and final stage reviews remain authoritative.
> Current execution selection (2026-11-24): the maintainer selected tasks 076-083 in dependency order. Earlier registration-only statements below remain dated history. Task YAML owns current status; no requirement, human-review gate, review budget, quality claim or release authority is changed by selection.

Registration date: 2026-11-24. Tasks 076-083 exist as **pending** definitions only. The user redirected implementation to requirements/task definition first; discovery or dependency order does not select tasks for execution. Baseline is `7e8cd91426fa21609a91c0511de44252eaa890d4` after importing committed v0.4. See [ROADMAP.md](ROADMAP.md) and [REQUIREMENTS.md](REQUIREMENTS.md).

The task YAML under `tasks/<ID>/task.yaml` owns each task's current state. Future plans/results/reviews belong beside it, raw evidence under its evidence directory. This map owns cross-task requirement ownership and sequencing; it is not another task queue. Completed 001-075 records stay unchanged.

## Tasks and dependencies

| ID | Deliverable | Dependencies | Primary full clauses |
|---|---|---|---|
| [076](../../../tasks/076/task.yaml) | Freeze language-expansion feasibility, contracts and validation corpus | 075 | L30 |
| [077](../../../tasks/077/task.yaml) | Add PHP, C#, C and C++ routing and bounded typed-source foundation | 076 | L01, L02, L08 |
| [078](../../../tasks/078/task.yaml) | Add bounded PHP cryptographic source and API detection | 077 | Language-scoped L03-L07; full closure in 082 |
| [079](../../../tasks/079/task.yaml) | Add bounded C# cryptographic type, alias and API detection | 077 | Language-scoped L03-L07; full closure in 082 |
| [080](../../../tasks/080/task.yaml) | Add bounded C and C++ OpenSSL and OQS source detection | 077 | Language-scoped L03-L07; full closure in 082 |
| [081](../../../tasks/081/task.yaml) | Add bounded Composer, NuGet and C/C++ dependency inputs and migration guidance | 077 | L10, L11, L12 |
| [082](../../../tasks/082/task.yaml) | Integrate all new language evidence with v0.4 context and migration workflows | 078, 079, 080, 081 | L03, L04, L05, L06, L07, L16, L17, L18, L19, L20, L21, L22, L23, L24 |
| [083](../../../tasks/083/task.yaml) | Reconcile v0.5 final-source acceptance, rule quality and compatibility evidence | 076, 077, 078, 079, 080, 081, 082 | L09, L13, L14, L15, L25, L26, L27, L28, L29 |

## Clause-to-task mapping

Exactly one primary task closes each complete L clause. Supporting owners must deliver their stated component requirements/tests; they are not optional or allowed to close a global clause with partial language coverage.

| Clause | Primary | Supporting tasks |
|---|---|---|
| L01 | 077 | 076, 083 |
| L02 | 077 | 080, 083 |
| L03 | 082 | 076, 078, 079, 080, 083 |
| L04 | 082 | 077, 078, 079, 080, 083 |
| L05 | 082 | 078, 079, 080, 083 |
| L06 | 082 | 076, 078, 079, 080, 083 |
| L07 | 082 | 076, 078, 079, 080, 083 |
| L08 | 077 | 078, 079, 080, 082, 083 |
| L09 | 083 | 077, 078, 079, 080, 081, 082 |
| L10 | 081 | 076, 083 |
| L11 | 081 | 082, 083 |
| L12 | 081 | 083 |
| L13 | 083 | 078, 079, 080, 081 |
| L14 | 083 | 077, 078, 079, 080 |
| L15 | 083 | 080 |
| L16 | 082 | 077, 078, 079, 080, 083 |
| L17 | 082 | 078, 079, 080, 081, 083 |
| L18 | 082 | 083 |
| L19 | 082 | 083 |
| L20 | 082 | 081, 083 |
| L21 | 082 | 083 |
| L22 | 082 | 083 |
| L23 | 082 | 083 |
| L24 | 082 | 083 |
| L25 | 083 | 077, 078, 079, 080, 081, 082 |
| L26 | 083 | 076, 077, 082 |
| L27 | 083 | 078, 079, 080, 081, 082 |
| L28 | 083 | 082 |
| L29 | 083 | 076 |
| L30 | 076 | 082, 083 |

## Task boundaries and execution limits

- 076 is a prerequisite feasibility/specification deliverable because safe grammar pins, bounded interfaces and frozen corpus expectations determine dependent implementation. Its scratch compatibility checks are future work, not selected by this registration.
- 077 implements the shared syntax/capture foundation. 078/079/080 implement independently testable PHP, C# and C/C++ source slices, each with rules, tests, quality metadata and paired user scope documentation. Their L03-L07 contributions are scoped; 082 verifies all languages before closing those clauses.
- 081 implements all dependency subsets and environment-aware recommendations, with parser/provenance/guidance tests and docs. It can be sequenced after 077 without relying on a completed source slice; no dependency manager runs.
- 082 integrates all four source languages and dependency facts with committed v0.4 readers, context-template, scoped claims and CLI/local web/report workflows. L24 is mandatory because 073/074 are now imported.
- 083 is the final all-row/API-matrix evidence and regression disposition, not a separate place to postpone missing component implementation/tests/docs. Any required correction is owned and verified without inventing new tasks or rewriting historical results.
- Default is one coherent task per future PR, including the behavior's tests/docs. Logical steps do not create extra tasks, PRs or review identities. The main agent performs investigation/planning/implementation/testing/documentation; no parallel agents are authorized by this map.
- At most two reviewer identities apply per future authorized implementation request, one per required stage, reused for material fixes across its combined scope. Task splitting or multiple PRs within that request does not reset the budget. Prose-only registration requires no review subagents.

## Completion and release boundaries

None of the eight tasks is selected, planned, started, evaluated or done by creation. Use the normal lifecycle and original criteria on later selection. Invalid/failed/unexecuted required work prevents completion; source/library availability, artifact lists or previous done states do not cover missing new-language requirements. Final v0.5 technical acceptance requires all L01-L30 plus every initial API/input row and final-source applicable tests, browser evidence, measurements and reviews. Product quality, score/human-usefulness conclusions and release/publication remain separate; no such action is authorized here.
