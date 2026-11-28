# v0.5 final readiness assessment

Assessment date:2026-11-25 Japan time. Source: `b182a2f045ff8256441b0d6797e73542e6108427`, branch `codex/pqc-language-research`; remote branch matches. The source worktree was clean at assessment entry. This report is a readiness judgment, not release authorization, a new task or a product fix.

Verdict: **hold the public v0.5 release**. The bounded static language implementation is a release candidate, with accepted implementation/test reviews and no unresolved findings in their scope. A real CI-quality test failure, missing linked archive documentation and version/release preparation prevent an unqualified release-ready judgment.

## Verified implementation and evidence reuse

All336runtime/input hashes match [task087 final source09](../../../tasks/087/evidence/IMPLEMENTATION-SOURCE-09.json). Its nine command/log hashes were revalidated: Rust default983passed/3ignored; all-features1042passed/5ignored; Node134passed; fmt, both Clippy configurations and default/release builds passed. These are reused final-runtime results, not newly rerun full Rust suites. Later commits only changed prose and retained evidence. Both [retained review stages](../../../tasks/087/evidence/REVIEW-DECISIONS-09.json) accepted that same source.

The original v0.5 [technical acceptance](../../roadmap/v0.5/ACCEPTANCE.md) closes L01-L30 and the bounded API/input matrix. [Task085](../../../tasks/085/TASK-085-RESULTS.md) records the all30pinned-OSS follow-up, generic known-gap fixes,42/42nominated positive units and5negative controls. This is exposed source-derived evidence, not language-wide precision/recall. [Task087](../../../tasks/087/TASK-087-RESULTS.md) additionally confirms four displayed scopes at current runtime source09, selective secret masking, native/default parity and actual bilingual browser acceptance. No new all30scan or migration-assessment run is claimed here.

## Fresh checks

- CI Node command `node --test shell/*.test.cjs tests/report-ui/*.test.cjs`:81passed,0failed. This overlaps the retained134test set; counts must not be added as independent tests.
- CI Python command `python3 -m unittest discover -s tests/quality -p 'test_*.py'`:181tests,1error,3skipped. The failure is described below. Diagnostic dependency-audit messages in this unit-test log are synthetic test cases, not evidence of an actual dependency vulnerability audit.
- Focused `python3 -m unittest discover -s tests/quality -p test_ci_applicability.py`:9tests,1error, exit1, reproducing the same binding failure.
- `python3 -m unittest discover -s tests/distribution -v`:25tests, all pass.
- Actual local static archive preparation and `tests/distribution/check-install.py`: pass. Clean-prefix help/version/rule listing and an external synthetic scan work with network access denied. The candidate still identifies as0.4.0 and is not a v0.5 release.
- The archive contains13rule files/139rules, all four new grammar notice records and177dependency records; missing notices0, review-required notices0. This check does not replace hosted build provenance.
- Quality evidence aggregation with the configured catalog exits0 and reports `releaseDecision: Unknown`; aggregation is not a release approval. An initial attempt used an unsupported output location outside `target/` and was rejected before aggregation; the corrected command used `target/v05-final-readiness-20261125/quality-integrated`.
- GitHub read-only queries found no Actions runs for the three language-branch commits and no status checks at the current tip. Push checks target `develop`; this feature-branch push does not trigger the quality job. No remote workflow was dispatched. No v0.5 tag exists; the latest published release observed isv0.4.0.

Raw logs, package/install receipts, source bindings and observations are retained in [evidence/READINESS.json](evidence/READINESS.json).

## Public-release blockers

| Priority | Finding | Required next result |
|---|---|---|
| P1 | `test_workflow_uses_current_complete_review_without_changing_scope` errors with `Applicability source/review binding changed`. CI still selects `tests/quality/applicability-v040-readme-20261123/assessment.json`; both COVERAGE guides differ from its recorded hashes. These mismatches already exist at31a4638 and512f2c0, before the latest documentation commit. | Create a current source-bound applicability assessment with its required independent review evidence; preserve the old assessment, truth and scope. Update the CI selection and corresponding test only after that assessment is valid. Do not merely replace hashes, disable the guard or rewrite old reviews. Re-run the Python tests and hosted CI. |
| P1 | The actual static archive omits `README.md`, `README.ja.md`, `docs/user/COVERAGE.md` and `docs/user/COVERAGE.ja.md`, while its bundled scanning guides reference them through relative links. Binary installation succeeds but documentation navigation is broken. | Include required linked documents, or provide intentionally valid equivalent links. Verify archive members and local documentation navigation with the candidate package. |
| Release preparation | Workspace/CLI/archive still say0.4.0; v0.5 release notes are absent. There is no final-source hosted CI or distribution/provenance run for this candidate. | Prepare0.5.0 metadata/lock updates and paired release notes, run applicable checks and hosted distribution preparation on the final source, then verify exact archive/rule/doc hashes, clean installation and provenance before a separate publication decision. |

The current-summary page `docs/roadmap/v0.5/ACCEPTANCE.md` also still refers to137rules and the previously failing Node feedback control. Its current disposition needs a source-bound follow-up for139rules and the now-passing134Node tests. Preserve the historical task083 measurements and reviews. This is documentation/evidence synchronization, not an unresolved Node product bug.

## Retained limits, separate from blockers

- PHP/C#/C/C++ support is bounded syntactic/local analysis. Arbitrary include/preprocessor execution, template instantiation, whole-project compiler semantics and deployed provider/peer verification remain outside the agreed v0.5 scope. Absence of findings does not prove safety. The evidence does not rank C++ accuracy against Java/Go.
- The task085 full-root migration reports remain27generated/3explicit refusals: OpenSSL and Botan contain contradictory negative certificate fixtures; Bitwarden exceeds the16384asset input limit. These are preserved input-validation limits, not silently successful reports. No validation or safety limit should be weakened to make the count30/30.
- No universal secret detection, independent population-accuracy study, human-usefulness score, cross-host reproducible build or expanded platform/network-publication claim is made. These are not newly invented acceptance gates for the agreed static scope.

Recommended order: repair the CI evidence binding and package documentation; synchronize current acceptance/release notes and0.5.0metadata; run final-source hosted checks and package/provenance acceptance; then decide publication within the intended platform/feature scope. No source fixes, version change, CI dispatch, tag, release, commit or push was performed by this assessment.
