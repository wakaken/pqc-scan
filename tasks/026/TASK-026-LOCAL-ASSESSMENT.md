# Task 026 local assessment validation

Implemented on `codex/task-022-sarif-location` from clean base
`065646ed67a5211011b5e05b38b2ae5d0d4a6968`. User authorized implementation,
tests/docs/independent review/local commit on the same branch; no push/PR.

## Delivered scope and decisions

Pure `assessment.rs` and `policy.rs` separate construction/standardization,
quantum exposure, classical concerns, certainty, usage, context, priority and local
policy decisions. `--inventory-json --policy-profile migration-review-v1` adds
`scan.assessment.v1.json`. Optional `--assessment-context PATH` accepts bounded,
strict offline v1 enum/boolean input. No automatic crypto edits/network requests.

The local profile is advisory candidate review with explicit applicability and no
universal deadline. Regulatory applicability was not assumed. Existing rule
classification, Risk, severity and RecommendedAction were not changed, so the
conditional legacy-classification decision gate was not invoked. Existing strict
inventory v1 cannot accept populated assessments; the chosen separate artifact
preserves its existing schema, fields, IDs and not_evaluated/empty placeholders.
No existing storage format is replaced. This implements the assessment intent
without breaking strict task025 consumers. `--native-json` stays removed.

The [contract](../../docs/architecture/LOCAL-ASSESSMENT-V1.md) describes all profile/context
fields, verdict precedence and limitations. Both READMEs, current architecture and
[standards register](../../docs/product/STANDARDS-REGISTER.md) document the feature. The
[assessment schema](../../schemas/assessment-v1.schema.json) is original MIT project
material. No dependencies, rule files or existing model structs were changed.

## Correctness and independent review

Executed pinned Rust1.93.1, workspace default features, offline Cargo resolution:

- Workspace debug/release build, fmt check and clippy with warnings denied passed.
- **189 Rust tests, 20 Python tests and 22 Node tests passed.**
- Nine new core tables test coexisting quantum/classical axes, draft/hybrid
  components, exact PQC names/OIDs, namespace/parameter uncertainty, AES/hash attack
  fractions, strict/duplicate/oversize/private-canary context, applicability and
  aggregate decisions, purpose/support constraints, certificate signature/key/
  validity, missing assurance, empty scan and deterministic evaluation.
- Three new actual CLI tests cover explicit opt-in, separate artifacts, unchanged
  inventory/legacy reports, schema/references, invalid profile/dependencies,
  malformed/oversize/invalid UTF-8 context without content leakage or output,
  JOSE prerequisites, KEM/signature mismatch, and a full repeatable semantic golden.
  Existing tests retain private-marker masking, old consumers, CLI default/exit
  contracts and all prior schema behavior.
- Policy outcome does not affect exit codes. Existing threshold/completeness
  precedence remains checked by the unchanged CLI regression suite.

[Independent design and source review](TASK-026-REVIEW.md) approved the final change.
The material P2 finding was a missing formal PQC OID catalog: it made an ML-KEM
public key unknown and could suggest a signing candidate. The reviewer reproduced
this with an explicitly synthetic parser-only public DER OID mutation, not a
validated PQC certificate. Official CSOR mappings, namespace guards, no unknown
replacement guessing and regression tables corrected the issue. The reviewer
reran the actual CLI reproduction and all 12 new tests successfully. Opaque
parameters remain unavailable; no cryptographic verification claim was added.
Further purpose-table regressions keep RSA-PSS signature-only, restrict generic
RSA/EC candidates to signing/establishment, and retain a known KEM in a certificate
signature slot as a purpose mismatch rather than rewriting it into a signature.
These fixes were independently re-reviewed.
Observation indexing also avoids repeated per-asset full scans while preserving
order/deduplication. Final docs/schema/golden were independently reviewed.

## Compatibility and CI projection

Saved old release, old inventory mode, new default, new inventory mode and new
policy mode scanned the same current source/root/rules/one-thread input. All five
produce **856 findings**. The six legacy reports plus diagnostics agree after
removing only generation time. Old/new inventory payloads agree after normalizing
run/time/observation IDs and their references; asset IDs/facts remain exact.

CI expectations rebind only **37 existing entries in two modified files**. Source
line content/rule/column/count/path/reason are unchanged. No new omission is
registered. Independent recomputation confirms **856 raw = 660 expected fixture
results omitted + 196 retained**, all **99 source hashes verified**. Task025's
historical 814/660/154 became 856/660/196 because the current source includes new
catalog/tests/golden text: the saved old binary also sees 856. New production
catalogs and test evidence are retained rather than broadly suppressed.

## Frozen measurements

Task023 corpus/truth/labels are unchanged: 41 files / 5746 bytes, 36 raw predictions,
7 partial/34 unknown file dispositions. Per-family confusion counts and sorted
prediction dispositions agree across old/default/policy runs. Each uses three
accuracy repetitions and first-measured plus three warm samples on four fixed
input sizes. Semantic/diagnostic/score repeatability passes. Cross-root raw order
can vary because legacy IDs include checkout roots; new assessment golden normalizes
only explicit scan-local IDs and references, retaining stable asset identities.

Warm wall-time medians in ms (three samples):

| Input | Old default | New default | Inventory + policy |
|---|---:|---:|---:|
| Small | 26.59 | 26.75 | 32.93 |
| Medium | 63.64 | 58.00 | 89.37 |
| Large | 145.12 | 132.94 | 252.42 |
| Pathological | 20.79 | 20.66 | 33.40 |

Policy mode includes inventory capture/reporting; this is not policy-only cost.
Distributions include launch/report writes, use 5ms child polling and do not
measure OS cold cache. These descriptive local samples do not approve task023's
still-open numeric budgets or establish a general speed improvement/regression.

[COMPARISON.json](evidence/task026/COMPARISON.json) binds all 66 current runtime source files,
base/binaries/schema/golden/corpus inputs, environment, commands, exit statuses,
metrics and audit digests. Detailed artifacts remain local under `target/task026/`:
`verification.json`, `legacy-shadow.json`, `code-scanning-audit.json`,
`final-{before,after,policy}/`, red/green logs and both independent reviews.
The saved old binary hash matches the previous task025 follow-up evidence.

## Remaining constraints and unexecuted checks

Current inventory is not exhaustive, so aggregate Pass is impossible. Per-asset
Pass is bounded primitive review using unverified operator claims, never readiness
or implementation assurance. A scan-wide context applies only to matching purposes;
heterogeneous deployments need unknown declarations or future per-target context.
KEM/signature/JOSE/PKI prerequisites are distinct. Dependency presence proves none.

The catalog is finite: unknown OIDs, HashML-DSA encodings and unsupported hybrid
combiners are not guessed. Symmetric/hash catalog functions do not add detector
normalization or claim complete source coverage. Ideal oracle fractions are not
operational security bits. Native evidence does not validate trust, signatures,
secret-key possession, runtime use or general cryptographic implementation safety.
No deadline/compliance engine, remote profile/rule service or policy fail-on switch.

No external-product acceptance, network peers, real private keys, platform matrix,
browser/UI acceptance, remote CI, push, PR, merge or publication was performed.
Existing human-report rendering did not change. Initial missing-module/flag tests,
OID/parameter regression failures and one missing-import build failure were
corrected before the final passing checks; historical logs remain separate.
