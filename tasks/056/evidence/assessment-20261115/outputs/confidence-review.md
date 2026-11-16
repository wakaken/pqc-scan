# Confidence interpretation and downstream review

Status: Partial. No calibrated probability was introduced or measured.

The English/Japanese user report guides state that confidence is heuristic
match confidence, not the probability an application is vulnerable. Pipeline
construction copies `rule.confidence` into findings; severity adjustment uses it
as a local promotion condition. The separate asset evaluator uses fixed
`unknown` / `uncalibrated_static_evidence` certainty vocabulary and does not turn
static references into execution or trust evidence. Rule validation rejects
nonfinite/out-of-range values.

The 640 independently expected low/high-confidence severity cases preserve the
public f32 value (1e-7 numeric transport tolerance only) and match the frozen
table. Their inputs are correlated cross products, not calibration samples.
320 exact 0.45 cases were observed without an independent expected answer:
the numeric boundary is present in implementation, while high-confidence wording
and selected examples alone do not independently specify that exact threshold.
These rows stay Not Tested for judgment correctness, not passing evidence.

No probability training, calibration population, Brier score or posterior
correctness claim is appropriate here. Before claiming complete Q3-09 coverage,
finish the source-bound downstream/UI/export terminology audit and independently
verify the exact threshold policy. Existing report guide wording and selected
source paths are evidence, not an exhaustive all-output audit.

Sources: `docs/user/REPORTS.md`, `docs/user/REPORTS.ja.md`,
`docs/architecture/LOCAL-ASSESSMENT-V1.md`, `crates/pqc_scan_core/src/pipeline.rs`,
`assessment.rs`, `risk.rs`, and `crates/pqc_scan_rules/src/lib.rs`.
Actual observations and comparisons are bound in this output set's
`assessment-oracle.json` and `severity-cases.json`; no source assertion is
substituted for an unexecuted measurement.
