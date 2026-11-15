# Confidence promotion boundary: analysis and recommendation

Status: proposal, not an approved policy change. The maintainer reported on
2026-11-15 JST that the intended boundary was unknown and requested analysis.
This is not approval of 0.45 or of another threshold. Product logic, rules,
reported severity and CLI exit behavior remain unchanged.

## Recommendation

Preserve the existing 0.45 gate for compatibility during validation, explicitly
identify it as a legacy uncalibrated promotion heuristic, and do not treat it as
independent judgment truth. Do not replace it with an arbitrary round/high value.
For a separately approved policy revision, prefer explicit rule-level severity
and evidence/context-based investigation priority rather than a global numeric
match-confidence gate that silently increases severity.

The recommendation has two horizons. Current compatibility is an observable
contract; a defensible intended severity policy still needs an explicit decision.
Keeping old behavior does not establish it is empirically optimal or endorse it
as the new independent oracle. An evaluation cannot manufacture maintainer intent
from an implementation constant.

## What the evidence establishes

`git blame` traces the 0.45 comparison and sensitive-category logic to the initial
public commit `0c948ad`. The tracked history and inspected current design/task
records did not disclose a study, calibration set, cost tradeoff or rationale
selecting that particular value. This is a bounded absence-of-evidence finding,
not a claim the original author selected it randomly.

The gate uses rule-level `confidence`, copied into findings by the pipeline.
Confidence is documented as heuristic match certainty, not a calibrated
probability of vulnerability. It does not measure deployment, exploitability,
crypto impact, data lifetime or a need for incident response. A numeric value
alone cannot establish those facts. The current legacy `Severity` is already
described to users as investigation priority rather than a standardized CVSS score.

The existing algorithm is conditional: the category must be TLS/PKI/JWT/auth/
authentication (case insensitive), risk must be quantum-vulnerable, and the f32
confidence comparison must meet the gate. It then bumps one level, saturating at
critical; a `private_key` evidence type takes the critical path after those gates.
A marker is not itself proof of a live exposed credential. Confidence below the
gate can therefore suppress the special critical path for compatible custom
rules as well. No change to this behavior is authorized by this analysis.

## Measured source characterization and hypothetical sensitivity

Source: `f07d782cd68cfac95d93585d5ace049f590160d4` plus the preserved local
baseline. Current rule-file and risk-source SHA-256 bindings, exact git commands
and results are in [the study](evidence/assessment-20261115/THRESHOLD-STUDY.json)
and [history](evidence/assessment-20261115/THRESHOLD-HISTORY.json).

There are 123 current built-in rules; 25 satisfy the category/risk gates. Of
those, one has confidence 0.42 and the other 24 span 0.50 through 0.78. None of
these 25 has confidence exactly 0.45. All 960 previously captured synthetic
severity observations agree with the independently implemented *description of
current behavior*. That check confirms characterization, not intended-policy
correctness or calibration.

The table below is a **hypothetical one ordinary hit per built-in rule** study.
It is not a rerun of real findings, not accuracy, and not a prediction of changed
findings in an arbitrary repository. The implementation's f32 narrowing is
represented in the simulation. Private-key-type custom-rule scenarios are
separate and are not included in these counts.

| Gate | Built-in rules promoted | Rules changing from 0.45 | Rule-hit scenarios changing `--fail-on high` result | Changing `--fail-on critical` result |
|---|---:|---:|---:|---:|
| 0.45 (current) | 24 | 0 | 0 | 0 |
| 0.50 | 24 | 0 | 0 | 0 |
| 0.60 | 16 | 8 | 5 | 3 |
| 0.70 | 5 | 19 | 8 | 11 |
| 0.80 | 0 | 24 | 8 | 16 |

Raising the gate to 0.8 disables every current built-in promotion, while leaving
their existing base severity intact. That is not a small confidence-label change.
0.5 only happens to match these built-in outcomes: compatible custom rules in
the 0.45-to-below-0.5 interval change, and future built-ins could change too.
Changing the gate can change JSON/SARIF/Markdown/HTML severity and `--fail-on`
exit 2 even when matching, raw evidence and quantum classification are identical.

## Alternatives

| Approach | Advantages | Problems / required evidence |
|---|---|---|
| Keep 0.45 as legacy compatibility behavior | Preserves existing reports and custom packs; needs no invented new cutoff | Empirical selection remains unverified; document its limited meaning; do not claim independent gold |
| Raise to 0.5, 0.7 or 0.8 | Simple implementation | No calibration/risk-cost basis; built-in and custom output/exit compatibility effects; round numbers are not stronger evidence |
| Remove implicit confidence bump; review base severities explicitly | Separates match evidence from investigation impact; enables per-rule reasoning | Changes existing behavior and requires rule-by-rule adjudication, impact analysis, regression cases and maintainer approval |
| Introduce evidence/context-dependent priority in a versioned policy | Can represent lexical references, parsed facts, relevant configuration, uncertainty and operator context distinctly | Must not call AST/configuration evidence executed/effective deployment; needs a specification and independent cases; no automatic new architecture/task is authorized |

Prefer the first option now and the third/fourth direction for a separately
approved revision. Reuse existing assessment/assurance concepts where possible;
do not add an unnecessary parallel scoring system. Actual usage/exposure claims
must remain Unknown unless observed. The existing support-claim provenance model
must not become verified deployment evidence.

## Evidence needed before selecting a replacement

1. Define what an escalation means: investigation priority, impact, or a different
   notion. Preserve the existing user meaning unless a change is approved.
2. Separately adjudicate false matches and correct references whose usage/impact
   is unknown. Matcher TP/FP labels alone are not severity truth.
3. Use relevant positive/negative/boundary cases across categories, evidence kinds
   and custom rules; independently assign intended priorities and action costs.
4. Compare candidate policies on those frozen decisions, including false
   escalation, missed escalation and CLI exit changes. Include uncertainty and
   correlated/limited data; avoid a single aggregate score as acceptance.
5. If a probability interpretation is desired, specify and independently evaluate
   calibration separately. It is not necessary merely to define a deterministic
   investigation-priority policy.

These are criteria for an agreed revision, not automatic task creation or
permission to modify rules. New numeric hard gates or changed severity/crypto
meaning require the existing human decision process. Task056 acceptance Q3-08
explicitly requires intentional changes to be separated after impact analysis
and approval.

## Relation to standards and task056

The [FIRST CVSS4 specification](https://www.first.org/cvss/v4.0/specification-document)
separates intrinsic, threat and environmental characteristics for vulnerability
severity. It does not supply a pqc-scan detector-confidence cutoff. This is
context supporting separation of concepts, not a claim that CVSS governs this
tool or that its numeric thresholds can be copied.

The 320 exact-0.45 oracle cases remain characterization. The maintainer's request
to investigate does not convert them into approved independent truth. The 885
other frozen judgment comparisons remain their bounded measurements. Pending
legacy recommendation/provider/version/assurance/provenance work and the two
required implementation/test review stages also remain unfinished; this proposal
does not close task056, task052 or task053.

## Maintainer disposition (2026-11-15)

The maintainer selected the recommendation: retain 0.45 and continue the
remaining task056 validation. Task052 remains unfinished and task053 interrupted.
This authorizes compatibility regression expectations at the existing boundary;
it does not certify that the threshold is optimal, calibrated or a probability.
