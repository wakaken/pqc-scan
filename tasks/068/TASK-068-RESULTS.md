# Task 068 results

Status: done. All original R4-01..04/R4-06 criteria and deliverables are verified
for this component, including both independent review stages. HEAD c7e76b59da44ffdc8a60da86efd5393e70a039e3
plus preserved task067 and current task068 changes on the specified worktree/branch.
The maintainer explicitly chose to proceed through067-070 and decide071 evaluation
resources later; no recruitment/contact/spending is authorized or performed.

## Behavior and purpose

`migration::MigrationContext` accepts the separately namespaced, closed typed
context and binds its exact saved input format/digest and target IDs to explicit
systems. It reuses067 preflight, bounded regular-file reads, hashes and target
identity. Whole-claim organization/system/target precedence and per-support-entry
inheritance preserve omitted, unknown and false. Unbound targets get no defaults;
ambiguous/missing/duplicate/conflicting ownership or substituted input rejects.

Per-claim actor/source/assertion/expiry/reference survives resolution. Freshness
uses an explicit clock with inclusive assertion/exclusive expiry, unknown expiry
and future/stale dispositions. A self-labelled external review remains a claim.
Incompatible intended purpose is retained as a conflict without changing the
primitive. Fixed errors omit rejected data. The schema is a new approved namespace;
old Context v1, policy, native schemas/IDs and rule behavior are unchanged.

See [plan](TASK-068-PLAN.md),
[architecture](../../docs/architecture/SCOPED-MIGRATION-CONTEXT.md),
[schema](../../schemas/migration-context-v1.schema.json) and paired
[user guide](../../docs/user/SAVED-EVIDENCE.md).

## Acceptance reconciliation

| Original clause | Implementation and targeted evidence | Disposition |
|---|---|---|
| R4-01 | Closed typed claim fields, known/unknown wrappers, no null, no unrelated mandatory business inputs; all fields and false/unknown/absence exercised | Verified; both independent stages accepted |
| R4-02 | Exact format/content/real target bindings, one declared system, old IDs retained; changed bytes, unknown IDs and ambiguous anchors reject;4096 real-target bindings exercised | Verified; both independent stages accepted |
| R4-03 | Whole-record precedence, per-entry support merge, unbound exclusion and separate systems sharing one RSA asset; explicit ownership conflicts reject | Verified; both independent stages accepted |
| R4-04 | Actor/source/pointer/date/reference/freshness, stale/future/missing expiry and purpose conflicts; no scanner-verification label accepted | Verified; both independent stages accepted |
| R4-06 |1MiB/256-system/4096-binding/64-alias/128-reference limits; reused generic preflight, strict fields/types/intervals/references, no fetch/file mutation and canaries | Verified; both independent stages accepted |

## Executed checks and preserved failure

Initial core check and strict core/all-target clippy exited0. Initial context
tests passed12. Expanded negative controls found that serde's internally tagged
unit `Deadline::None` accepted surplus fields despite deny_unknown_fields:
targeted-expanded.log exited101 (14 passed,1 failed). Replacing the unit variant
with a closed empty struct variant rejects that invalid record. The unchanged
negative assertion then passed in targeted-fixed.log:15 context tests and9 retained
assessment tests, exit0. No failed measurement was relabelled or deleted.

The structural context has a shallower valid shape than the shared JSON ceiling.
067 valid Native extension fixtures prove below/at/above generic depth/fields/
string/array limits;068 exercises the identical preflight on malformed context
overflow and its own otherwise-valid context-specific boundaries. It does not
fabricate an impossible depth16 closed context as valid. The maximal-binding
control resolves actual manifest targets, not only syntactically valid IDs.

## Limits and next work

Context binding is an operator statement about the business system, not independent
identity certification. No business decision, CLI/report, practitioner usefulness
or score is claimed. Task069 is now eligible after completed067/068;071 resources
remain explicitly deferred to later maintainer judgment, not secured or measured.
No release/publication or support expansion is performed.

## Final-source verification

Sources are bound by [IMPLEMENTATION-FINAL-SOURCE.json](evidence/IMPLEMENTATION-FINAL-SOURCE.json);
commands/exits/log/binary hashes and environment by
[COMMANDS.json](evidence/COMMANDS.json). Final affected tests passed48 (24 reader,
15 context,9 retained assessment); full workspace659 passed/3 existing ignored
supervisor entrypoints; isolated all-features core197 passed. Workspace build,
fmt check and strict default/all-feature/all-target clippy exited0. The final
workspace rerun includes the literal byte-cap/hash/parse ordering adjustment;
earlier pre-ordering logs are preserved separately. No product source changed
after these final runs. Link/parity, task066 freeze and legacy-source preservation
checks are in [documentation-and-source-checks.log](evidence/documentation-and-source-checks.log).

The same implementation reviewer accepted the combined scope, including the
ordering adjustment and effective-purpose inheritance clarification. See
[TASK-068-REVIEWS.md](TASK-068-REVIEWS.md). The same independent test reviewer accepted the final scope and verified all seven log/103 binary hashes; no actionable finding remains.

## Completion decision

The main agent reconciled each original clause to implemented behavior, executed
final-source checks and both independent acceptances. No required068 work is
moved to a later task. The shared technical/input behavior remains covered by067
regressions; old policy/context/native/rule sources remain unchanged. Final
artifacts and applicability are bound in [FINAL-SOURCE.json](evidence/FINAL-SOURCE.json).
Later lifecycle/result/review prose does not change executable inputs. Completion
is component acceptance, not complete v0.4, practitioner validation or release.
Changes remain uncommitted on the requested branch. Next eligible task:069.
