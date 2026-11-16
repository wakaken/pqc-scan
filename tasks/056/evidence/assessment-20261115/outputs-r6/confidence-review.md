# Confidence interpretation and downstream review

Evaluation complete against the current source; both independent reviews accepted.
No probability calibration was requested, introduced or claimed.

The English and Japanese report guides describe heuristic match confidence,
explicitly excluding vulnerability probability. Rule validation rejects nonfinite
and out-of-range values. Pipeline construction copies rule confidence directly
into findings; recommendation annotation retains it. The 960 canonical severity
cases cover sensitive/non-sensitive categories, vulnerable/uncertain/safe risk,
all five base severities, code/private-key evidence and low/exact/high boundary
values. Numeric tolerance applies only to f32 serialization (1e-7), never a
judgment enum or a calibration inference.

The maintainer explicitly selected preserving0.45 on2026-11-15. Its 320 exact
boundary reassessments are compatibility expectations. They do not prove that
0.45 is calibrated or optimal. The original characterization table and earlier
Partial output remain unchanged. The descriptive threshold study is a rule-
metadata scenario, not a finding population or empirical accuracy estimate.

`CONFIDENCE-AUDIT.json` binds thirteen current source/guide consumers by hash:
rule input, finding construction, severity, asset certainty, JSON, SARIF,
HTML/Markdown, inspection, CBOM, Japanese localization and CLI explain output.
JSON preserves the existing heuristic number. SARIF does not export it as
probability; HTML/Markdown do not create a confidence percentage. The separate
asset certainty vocabulary remains unknown/uncalibrated_static_evidence; static
observations do not become execution or trust claims. CLI explain formats the
rule value under confidence, without asserting probability.

This is a source and terminology audit plus measured preservation checks, not a
new browser acceptance or a user-interface change. No Brier score, confidence
interval, probability posterior or universal quality claim follows from these
correlated synthetic cases. New consumers or future probability claims require
new specification and independent calibration data.
