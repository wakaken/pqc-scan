# Quality evaluation model

PQS-QAM organizes evidence about detection, inspection coverage, cryptographic
interpretation, safety, performance, and product operation. It is a project model,
not NIST approval, ISO certification, or proof of cryptographic safety.

Use the [metric definitions](QUALITY-METRICS.md) for scoring and denominators,
[reference register](QUALITY-REFERENCES.md) for source scope, and
[quality checks](QUALITY-GATES.md) for change-specific validation.

## Evidence states

Attach the evaluated source, inputs, conditions, and evidence to each result.
Command success, source applicability, and independent verification are separate.

| State | Meaning |
| --- | --- |
| Verified | Agreement with independent supporting evidence has been checked. This is separate from acceptance. |
| Passed | An executed check meets its stated condition within the recorded scope. |
| Failed | An executed check does not meet its condition, even if the cause is unknown. |
| Partial | Only part of the required scope has been checked or completed. |
| Unsupported | The feature, input format, or environment lies outside supported scope. This does not establish safety. |
| Unknown | Evidence, population, authority, or other facts are insufficient for a conclusion. |
| Not Tested | The defined check or measurement has not been executed. |
| Not Applicable | The condition does not apply to this target, with a recorded reason. |

A successful command can be Passed while the overall evaluation remains Partial.
Do not average the axes into a release decision or infer completeness from test counts.

## Quality axes

| Axis | Evidence | Interpretation limits |
| --- | --- | --- |
| Q1 Detection accuracy | Source-backed labels, one-to-one matching, TP/FP/FN, denominators, and split/family strata | Static references do not establish runtime use. Unlabeled and exposed inputs do not supply independent deployment accuracy. |
| Q2 Inspection coverage | File enumeration, reads, applicable detectors, terminal states, diagnostics, and reconciled counts | Enumeration, applicability, and completion are distinct. An unknown ignored population is not zero. |
| Q3 Cryptographic interpretation | Rule rationale, standards, certificate facts, and assessment-policy cases | Detection accuracy differs from interpretation. Confidence is not a calibrated probability; references and offers do not prove execution or negotiation. |
| Q4 Robustness and safety | Fault injection, resource boundaries, synthetic secret canaries, escaping, filesystem and protocol safety controls | Distinguish crashes, timeouts, limits, read failures, and secret survivors. Finite success does not prove safety for every input. |
| Q5 Performance and reproducibility | Repeated runs with fixed input, binary, rules, source, environment, features, and cold/warm conditions | Report wall/CPU/RSS and noise with their measurement scope. Input copies are not independent accuracy samples, and changed environments do not establish a regression. |
| Q6 Product operation | CI, platform/feature builds, install/distribution tests, notices, compatibility, security and maintenance procedures, provenance, and publication evidence | Repository operation scores are separate from detection accuracy. Identical archives from one supplied binary differ from independent reproducible builds. |

## Source binding and aggregation

Record revision, branch, dirty changes, time, toolchain, features, build conditions,
environment, and input/truth/rule/binary/source hashes. Bind logs and terminal
results to that context. A HEAD SHA alone does not identify dirty source, and
binaries from different feature builds must remain distinguishable.

`tests/quality/run.py` collects measurements, `evaluate.py` applies the scoring
contract, and `report.py` checks and aggregates recorded evidence. Q1/Q2/Q5
baseline measurements do not supply missing Q3/Q4/Q6 evidence. The all-axis catalog
and [CI measurement contract](CI-MEASUREMENT-PLAN.md) define integrated reporting.
Unattributed predictions counted as FP by a scorer can include correct unlabeled
facts; retain that distinction from confirmed product false positives.

## Labels and change control

Define truth from input facts before inspecting results. Preserve provenance,
author, adjudication, ambiguity, license, fixed input identity, and exposure history.
Do not overwrite existing labels or reuse a tuned holdout as an independent test.
Keep regression on exposed data separate from evaluation on new independent data.

Positive and Negative follow the declared static reference or configuration unit,
including unused code. Keep Ambiguous and Unsupported outside eligible denominators
while retaining their counts, provenance, and predictions. Unsupported truth labels
describe scoring eligibility, not the product's complete format support.

Changes to rule semantics, severity, public schemas, or numeric hard gates require
the applicable impact review and approval. Complete only the original criteria
supported by implementation and required validation; displaying Unknown does not
implement a missing capability.
