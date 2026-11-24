# v0.4 release-readiness assessment — 2026-11-19

Decision: **HOLD / NOT READY for publication**. The current amended v0.4 task
acceptance is complete, but a distributable v0.4 release has not been prepared or
verified. This is an assessment requested by the maintainer, not release notes,
product acceptance, publication authorization or an actual release.

Evaluated code: `ad548d2c81702576475d86c9cf6557fbae1e7759`, branch
`codex/v04-roadmap-20261118`. The code checkpoint was committed and pushed before
these prose/evidence records were added. Later recording commits do not change
that evaluated code or manufacture hosted receipts for it. The declared Cargo
and optimized-binary product version is still `0.3.0`.

The selected distribution scope remains static/default macOS Apple Silicon
(`aarch64-apple-darwin`). No other platform or network package becomes supported.
The [machine-readable decision](RELEASE-DECISION.json) retains gate statuses,
actions, source identity and evidence hashes.

## Task completion and what it establishes

All eleven task records, 065–075, are `done` under their current acceptance
criteria. The explicit 2026-11-19 maintainer amendment changes only the identified
071 human/comparative/numerical evaluation scope and dependent 072/075 conditions.
The [amendment](../../../tasks/071/evidence/MAINTAINER-AMENDMENT.json), archived
original plan, old blocked results and prior approvals remain preserved.

| Task | Purpose and actual work | Established result and boundary |
|---|---|---|
| [071](../../../tasks/071/TASK-071-RESULTS.md) | Check technical handling, adopted decision specifications and fixed-condition reproducibility using a separate AI's case proposals. Main adjudicated primary-source facts and literal specification expectations, froze cases/truth/splits/conditions before observing output, then used existing PQS-QAM comparison. | 24 synthetic cases/12 pairs, split 8 tuning/16 final; 48 actual CLI runs. Agreement on 222/222 eligible technical properties and 507/507 specification properties; 24/24 complete semantic repeat comparisons. 56 unconfirmed statements excluded. Procedural case-author separation is not full independence or population accuracy. |
| [072](../../../tasks/072/TASK-072-RESULTS.md) | Verify that the assembled implementation meets the amended roadmap and retains existing contracts. Reconcile every current clause with actual source, tests, evidence and both independent review stages. | All 30 current clauses reconciled; 65 original frozen specification cases across A01–A18 unchanged and mapped to executed regression assertions; 10 compatibility areas covered. This is bounded integrated acceptance, not an unrestricted quality or release decision. |
| [075](../../../tasks/075/TASK-075-CURRENT-RESULTS.md) | Check the manual JSON, assisted CLI-template and local web entry paths for personal and organizational contexts, then reconcile all current requirements after 071/072 completion. | Six preserved M/C/W operations succeeded; canonical complete JSON agrees across the three paths for each context; next-action checks pass in all six. One selector failure/recovery remains recorded. All 39 current clauses reconciled. Recorded tool/orchestration times do not establish human workload or usability. |

Human practical usefulness, actual human workload, human inter-rater agreement,
independent human expert verification, and numerical-score validity/superiority/
adoption remain unmeasured. The original 96 planned human judgments remain
unexecuted, with zero observed; AI counts do not replace them. These exclusions
do not reopen the explicitly amended task scope, and no positive claim is made.

Existing implementation and test reviewers accepted the applicable final-source
work; their [071](../../../tasks/071/TASK-071-REVIEWS.md),
[072](../../../tasks/072/TASK-072-REVIEWS.md) and
[075](../../../tasks/075/TASK-075-REVIEWS.md) records retain scope and limitations.
No additional reviewer or case-author work was commissioned for this prose-only
release assessment.

## Release gates

| Gate | Assessment |
|---|---|
| Current feature acceptance and local regressions | Passed within the recorded scope. Prior applicable evidence remains source-bound: 719 workspace passes / 3 existing ignored supervisor entries, 226 isolated all-feature core passes, 69 Python quality passes, 23 featured CLI checks, build/fmt/strict clippy, and retained actual browser coverage. |
| Local optimized static build and distribution tests | Passed now: locked release build exit 0; 25 distribution tests pass. These tests do not establish that the real dependency graph can be packaged. |
| Actual static archive preparation | **Failed, exit 1.** The existing packager stops at exact-version notice registration. Same-graph read-only diagnosis identifies `uuid-simd 0.8.0` and `vsimd 0.8.0`: no collected license files in the local crate cache and no matching fallback registration. No guard was bypassed. This is a collection/registration failure, not a conclusion about upstream license validity. |
| Repeat archive, verification and clean offline install | **Not Tested**, blocked by archive preparation. The second archive and installation commands were not reached; no current archive digest or successful clean install is claimed. |
| Exact-source hosted checks and provenance | **Not Tested.** GitHub returned zero workflow runs and zero commit statuses for the evaluated code commit. This branch's push is not a configured CI trigger; checks use develop/PR/manual events. No hosted dispatch, final archive attestation or verification was performed. Local checks do not replace these receipts. |
| v0.4 version and release/support documentation | **Not Ready.** Cargo and binary still identify `0.3.0`; final v0.4 release notes, version/support disposition and artifact binding remain to be prepared. |
| Product acceptance and signing/tag/publication | Separate maintainer decisions. Historical [authorization](../static-candidate-20261117/PUBLICATION-APPROVAL.md) is bounded to static macOS arm64 v0.3.0; it does not authorize v0.4 publication. This request authorizes assessment and git commit/push. |

The human-evaluation exclusions are not being introduced as new blockers for the
approved limited evaluation. Publication is held because the concrete packaging,
release-identity and final-source hosted/artifact gates above are not satisfied.
The historical PQS-QAM catalog's `releaseDecision: Unknown` remains historical;
neither its aggregate nor the new study's property counts certify a release.
The four existing network notice holds remain outside this selected static scope.

## Evidence and remaining sequence

[Source reuse](evidence/SOURCE-REUSE.json) confirms all 6,670 files bound by the
last task075 snapshot still match. The code/authored-prose whitespace check is
exit 0. The complete commit check is exit 2 because original downloaded sources
and raw browser evidence preserve whitespace; its 143 evidence paths are listed
separately. Hash-bound evidence was not rewritten to silence those warnings.

[Commands](evidence/COMMANDS.json), [build identity](evidence/BUILD-IDENTITY.json),
[distribution tests](evidence/distribution-tests.log),
[optimized build](evidence/release-build.log),
[packaging failure](evidence/package-first.log),
[notice diagnosis](evidence/NOTICE-DIAGNOSTIC.json),
[hosted runs](evidence/HOSTED-RUNS.json) and
[commit statuses](evidence/HOSTED-STATUSES.json) retain actual outcomes.
All newly attempted build/test/version commands exit 0 except archive preparation
(exit 1). No archive, second preparation or install success is inferred.

The remaining order is:

1. Collect and verify applicable complete notices for the two exact package
   versions, retain the existing bindings/guards, and review/test any executable
   change. No notice policy or dependency version was changed by this assessment.
2. Prepare the v0.4 product version and paired release/support documentation
   within the approved static support scope.
3. On the final agreed source, prepare the static archive twice, compare bytes,
   verify it, and perform clean offline installation, including the new migration
   entrypoints and bundled documentation/rules.
4. Obtain actual final-source hosted CI and release-preparation/provenance
   receipts through the existing workflows, reassessing changed inputs.
5. Reassess technical readiness and obtain explicit bounded v0.4 product and
   publication authorization before actual signing, tagging or publication.

No new task was created, no existing acceptance criterion was relaxed, and no
version bump, external contact, recruitment, fee, product score, hosted workflow,
signing, tag or publication was performed. Existing task completion and release
readiness remain distinct, as required by the
[roadmap](../../roadmap/v0.4/ROADMAP.md) and
[verification rules](../../agent/VERIFICATION.md).
