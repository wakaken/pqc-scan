# CI measurement and evidence retention

Task053 integrates the existing quality reporting path. It does not approve a
release, complete unfinished upstream work or introduce numeric quality gates.

| Event/job | Work and cost | Evidence |
|---|---|---|
| Pull request: quality | Existing Rust, Node and evaluator tests; bounded offline catalog/hash/source checks. No new scanner benchmark, network probe, dependency download or upstream experiment is launched by aggregation. | Integrated JSON/Markdown and this plan; always-upload step, 30 days. |
| develop push: baseline | Existing release build and repeated fixed-corpus measurements; offline all-axis aggregation runs before expensive measurements. | Existing raw baseline reports plus integrated outputs; always-upload step, 30 days. |
| Manual workflow | Existing enabled quality/baseline/scan jobs retain their event conditions. | Same artifacts and retention. No new schedule or dispatch. |
| Protocols | Existing job remains disabled, including manual runs. | No new hosted protocol evidence is claimed. |
| Focused high-cost studies | Coverage, mutation, properties, fuzzing, large resource studies, protocol matrices, platform/rebuild/signature studies and process/advisory inspection run only under their existing task authorization and isolation/tool/cost limits. | Dated durable task records and raw evidence are enrolled by exact hashes; aggregation never executes their stored commands. |

Run the lightweight aggregation locally in a fresh directory:

```sh
python3 tests/quality/report.py --evidence-catalog docs/quality/EVIDENCE-CATALOG.json --out-dir target/quality-integrated
cp docs/quality/CI-MEASUREMENT-PLAN.md target/quality-integrated/ci-measurement-plan.md
```

The catalog accounts for every authoritative Q1-Q6 requirement, including missing
053 self-evaluation and054 release acceptance. It binds original artifacts,
measurement contexts, observations, commands/exits, source populations,
limitations, priorities, comparisons and review references.052 remains unfinished;
its missing ownership and operational facts remain Unknown. Product findings,
tool errors, source drift and unavailable measurements are different states.
A valid aggregate can contain Failed, Partial, Unknown or Not Tested outcomes.
A successful command is not evidence that a test or requirement is adequate.

Original studies remain immutable. Add new dated catalog entries for new studies;
never replace old measurements or relabel post-observation work as blind truth.
An artifact hash mismatch fails aggregation before quality reports are written.
A new safe output directory receives quality-infrastructure-error.json for
always-upload retention; an existing output directory is never overwritten. A measured
source mismatch retains the historical outcome and marks current applicability
Partial. Declared matching bytes do not expand the original population. Review
artifacts without normalized independent source-bound acceptance stay explicit
and unverified; they are not silently accepted because their task is done.
For normalized reviews, original controller/worker bindings are retained and
compared separately from the measured product population; either source drift
prevents current review adequacy from becoming Verified.

The generated quality-evidence-index.json and test-evidence-index.json are the
machine views; quality-assessment.md renders the same integrated facts. A new
output directory is required and releaseDecision remains Unknown. Existing
baseline-only report.py invocations and their outputs are unchanged.

Actions artifacts expire after30days (subject to repository retention limits).
Durable task records remain in tasks/<ID>/evidence with original manifests and
hashes; they are not replaced by expiring upload URLs. Re-run the scoped study
when required source, tools, inputs, feature scope, advisories or live settings
change. An aggregation run is not a new performance measurement or hosted CI
success. This local implementation has not dispatched a remote workflow.
