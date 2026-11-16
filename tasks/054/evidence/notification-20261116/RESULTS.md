# Code Scanning notification repair — local verification

The authorized local implementation and current-source checks pass. Both independent review stages are accepted for the local change. Hosted
publication/verification remains a separate gate. Task054 remains
in-progress and its original73-requirement assessment remains unfinished.

The existing exact-result projection now separates reviewed reference material
from results requiring review. The scan scope, crypto rules, severity, thresholds
and version1policy contract are unchanged. File SHA, complete result SHA, exact
locations and duplicate counts are required; new/changed/ambiguous results remain
visible. A reviewed reference is not automatically a false positive or safe use.

| Actual full-scan replay | Before | After |
| --- | ---: | ---: |
| All detected references | 5255 | 5255 |
| Results sent to Code Scanning | 3763 | 15 |
| Reviewed references preserved outside notifications | 1492 | 5240 |

The15results comprise11actual workspace dependency observations and4fixture
observations without a source region (7warning/8note; no error). They remain
review-required; dependency presence does not establish exploitability. The new
GitHub Actions summary groups the review queue and separately summarizes reviewed
reference contexts. Full SARIF, upload, audit, exact policy and summary are archived.
Historical5255results are not deleted or relabeled as passed measurements.

The final offline non-root current-tree scan produced5255results byte-identical
to the hosted baseline, across3531selected files.555files were skipped and
inspection remains partial; unknown ignore/traversal populations are not made zero.
The current policy verifies399source files and exact result/count groups. The
actual scanner negative control still sends identical unregistered product input
to review while separating only its explicitly reviewed synthetic counterpart.

Final confined Node tests:50passed/0failed/0skipped, including actual scanner
execution. Related Python tests:2CI +29evidence passed. Syntax, YAML, aggregation
and whitespace checks exit0. The container enforced no network, readonly
source/root, nonroot UID, memory/CPU/PID/tmpfs limits and300-second phase deadlines;
it was removed. See CONFINEMENT.json and CHECKS.json for exact versions and evidence.

Old policy bytes are preserved. Four synthetic historical test modules now use
that frozen policy, while the new regression module validates the current399file
population and new-result/duplicate/source-drift rejection. Two pre-change artifact
snapshots repair current catalog references without replacing original hashes,
measurements, source bindings, populationDigest or result labels. The existing
exact-reconstruction and corrupt-snapshot negative controls pass.

Implementation reviewer /root/notification_implementation_review accepted the
18bound files and the scoped follow-up. No extra agents or reviewer identities
were used. Test reviewer /root/notification_test_review accepted the same18bound files,
test adequacy and actual output population. No actionable findings remain within
the locally verified scope.

This is a local repair, not a claim that live GitHub alerts are already reorganized.
Push/publication and a real hosted run require the maintainer's separate approval.
No remote mutation, dismissal, merge, release, tag or signing occurred. The old
Code Scanning failure and2135cohort question remain historical054evidence; neither
this cleanup nor a later successful upload retroactively completes that assessment.

Evidence: [source](SOURCE.json), [measurement](MEASUREMENT.json),
[checks](CHECKS.json), [confinement](CONFINEMENT.json),
[implementation review](IMPLEMENTATION-REVIEW.json), [test review](TEST-REVIEW.json),
[artifacts](ARTIFACTS.json),
[rendered-summary source](self-scan-summary.md), [audit](code-scanning-audit.json).
