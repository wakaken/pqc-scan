# Existing self-scan alert backlog

Base `dfda62ae18af4e276a8d92b4593bb89eb6c1ce7b`, same PR14 feature branch.
User requested resolution of the remaining TLS and other Code Scanning alerts.
This is a repository-specific expectation/triage correction. No detector/rule
behavior, Rust source, cryptographic logic, schema or normal CLI scope changes.

## Investigation

The live default-branch page contained **128 open alerts (112 errors / 16 warnings)**,
all from develop commit `1335033fb3ffd96632f9f39a51bf8f2a115f2ead`. They are distinct
from PR14's 63 new alerts addressed in the previous follow-up.

All reviewed locations are fixed data or prose, not deployed TLS configuration:

| Source | Review conclusion |
|---|---|
| Six `rules/default/*.yml` files | IDs, regex patterns, descriptions and migration advice defining what to detect |
| `pqc_scan_report/src/japanese.rs` | Translation keys and guidance, including advice to disable old TLS |
| `pqc_scan_report/src/compact.rs` | Protocol rule-ID routing keys |
| `pqc_scan_core/src/recommendation.rs` | Guidance and before/after code examples returned as strings; never evaluated here |
| `pqc_scan_detectors/src/certificate_detector.rs` | Public OID-to-signature-name labels, not signing operations |
| `tasks/017-actionable-compact-reports.yaml` | Task acceptance/security prose |

No TLS listener/client setting or private key material was found in these reviewed
locations. In particular, a sentence recommending TLS1.3 and a regex matching TLS1.0
must not be treated as this scanner enabling a legacy TLS connection.

[ALERT-REVIEW.json](self-scan-backlog/ALERT-REVIEW.json) binds all 128 alert numbers,
source refs, paths and mapped current lines/source hashes. 124 match current
positions uniquely. Four old AST alerts map to two identical line-only SARIF groups
with two occurrences each; current sub-line identity is unavailable, and only the
source/group correspondence is asserted. Current AST occurrence preservation also
retains two additional guidance matches, making 130 current reviewed observations.

## Correction

Add 130 observations as **128 exact result groups in 11 explicit files**, using the
existing source/result-hash/position/count checks. Nine files are new registrations;
two existing file reasons are expanded to distinguish tests and metadata. Every
prior result allowance and source hash is preserved. No directory/rule-wide bypass
or automatic acceptance of new findings is added.

Four artifact-only test observations remain in the upload because a proven positive
line is still required for an exception. Retaining these limitations is intentional.
Full raw SARIF, upload SARIF, audit and registry remain workflow artifacts. Normal
user scans continue to report the original results.

Default-branch historical alert state and PR source fixes are separate. Individually
verified false positives can be dismissed without changing develop source or merging
the PR. The [official GitHub API](https://docs.github.com/en/rest/code-scanning/code-scanning#update-an-alert)
supports `state=dismissed` with `dismissed_reason=false positive`. Each selected
alert must still match its reviewed source/rule/message/location before mutation;
changed/unreviewed alerts are never included automatically. This does not establish
that all future results in these files are harmless. GitHub dismissal is separate
from the exact fail-closed policy for future uploads.

## Validation

- Fresh current raw SARIF is byte-identical to the previous hosted/local 856-result
  scan; input and run metadata are preserved.
- Projection changes **856 = 722 omitted + 134 retained** to **856 = 852 omitted +
  4 retained**, with all **112** source hashes verified. Remaining payload/order is
  exactly the previous upload minus the finite reviewed records.
- Three new regressions failed before registration and passed afterwards. They
  cover exact reviewed-data omission/raw immutability, a new result in a registered
  rule-definition source, and retention of artifact-only test findings.
- **28 Node tests passed**, including previous stale-source/new-production-error/
  duplicate/symlink/count/indirection cases. All 43 task027 Rust/Cargo runtime source
  bindings remain identical. No Rust build/test/clippy rerun is claimed for this
  metadata/test-only correction; the earlier hosted quality success is separate.
- [VERIFICATION.json](self-scan-backlog/VERIFICATION.json) records source/data hashes,
  partitions and checks. Detailed original API snapshots, red/green logs and full
  scan/audits are under ignored `target/self-scan-backlog/`.

[Independent review](SELF-SCAN-BACKLOG-REVIEW.md) approved the finite source
classification and alert mappings before historical alert triage/publication.

## Publication and constraints

No merge is authorized/performed. The source fix needs normal PR review/merge before
it applies to develop's subsequent scans. PR CI is checked on the pushed follow-up
commit; at documentation preparation time that remote execution is pending.
Historical-alert triage is in progress at source-commit preparation time. Each
mutation uses a fresh read-check; single-alert GET includes an extra message.markdown
field absent from list responses, so the check compares exact text and all other
instance/rule fields rather than mistaking this representation difference for
changed content. No alert was modified by the initial stopped preflight. Final
read-back and remote-CI outcomes are recorded in PR14 and local ignored audit logs.
The four unknown-location observations remain visible; no broad completeness,
production security or universal false-positive classification is claimed.
