# PR14 self-scan alert investigation and correction

Base HEAD: `aac154fd36bf47fd4c3a361013de113afd38cca2`, same feature branch.
User requested investigation of the failed check, then explicitly selected removal
from CI failure/alert registration with original scan evidence preserved. This is
a CI expectation follow-up to the combined task022–027 PR, not a Rust detector,
rule, output-schema or cryptographic behavior change.

## Observed cause

[The actual failed check](https://github.com/wakaken/pqc-scan/pull/14/checks?check_run_id=129626912848)
reported **63 new alerts: 29 errors and 34 warnings**. Both hosted push/PR workflows'
Rust quality, baseline and scan jobs passed. The failure is GitHub Code Scanning's
new-alert threshold, not a Rust test/build failure or a scanner execution error.

| Source class | Files | Errors | Warnings | Total |
|---|---|---:|---:|---:|
| Synthetic integration tests | CLI `policy_report.rs`, core `assessment_policy.rs` | 10 | 5 | 15 |
| Fixed classification data | core `observation.rs`, `assessment.rs` | 19 | 29 | 48 |

The former tests known vulnerable/unknown algorithms intentionally. The latter
matches fixed identifier vocabularies, reports algorithm classes and compares
public key sizes against policy thresholds; it does not sign, generate keys or
perform encryption. Merely excluding tests would leave the 19 error-level
classification-data matches and the same check failure.

The prior exact expectation registry did not include these newly added inputs and
classification references. Source names alone cannot distinguish classification
from actual use, so the finite reviewed results are registered individually.

## Change and safety boundaries

Add exactly **63 groups in four explicit files** to the existing upload-only
expectation registry. Source-file SHA, complete SARIF-result SHA, rule, line,
column and observed count must all match. No previous allowance is altered.
No file/directory-wide suppression, rule/severity change or threshold downgrade.
New errors at the same production-source position remain visible; stale source
bindings retain the original findings rather than accepting them silently.

The original full self-scan remains intact and archived with the filtered upload,
audit and exact registry in `pqc-self-scan-evidence`. Normal scanner CLI/output
behavior is unchanged. The preparation helper's console description now says
“reviewed expected results” because allowances include classification data.
Both READMEs and the CI policy distinguish these references from cryptographic use.

## Validation

- Downloaded actual hosted workflow artifact `42754549021/pqc-self-scan-evidence`.
  Its raw SARIF and old upload are byte-identical to task027's saved local outputs.
- Fresh local self-scan is also byte-identical to that hosted raw SARIF: 856 results.
- Replay of that hosted raw output changes **856 = 659 omitted + 197 retained** to
  **856 = 722 omitted + 134 retained**, all **103** source hashes verified.
- Remaining results are exactly the previous upload minus the 63 reviewed records,
  in the same order and with their entire result payloads preserved. All 63 bind
  one-to-one to remote annotations by path, line, column and full message.
- Three focused Node regressions failed before registry registration, then passed:
  exact omission with raw immutability; a new same-position production error stays;
  a stale classification-source hash keeps every result from that source.
- **All 25 Node tests passed**, including the existing traversal/symlink/count/
  duplicate/indirection/line-only fail-closed cases. Diff whitespace check passed.

[VERIFICATION.json](evidence/pr14/VERIFICATION.json) records exact input/source digests and
counts. [REVIEWED-RESULTS.json](evidence/pr14/REVIEWED-RESULTS.json) is the finite checked-in
SARIF-result regression fixture, stored under docs to avoid recursive scanning.
Detailed original annotations, raw/upload/audit artifacts and red/green/full test
logs remain under ignored `target/pr14-self-scan/`.

Independent review is recorded in [PR-014-SELF-SCAN-REVIEW.md](PR-014-SELF-SCAN-REVIEW.md).

## Limits and hosted outcome at commit time

This is repository-specific CI alert handling, not a general scanner claim that
all test code or algorithm strings are harmless. New/changed expectations require
review. No real secrets were used; no GitHub alert was manually dismissed.

Rust/Cargo/rules were not modified, so Rust build/test/clippy were not rerun for
this data/JavaScript-only follow-up. Task027's 201 Rust tests and the original
hosted quality jobs remain separate evidence. The selected tests and scripts were
rerun as above. Remote CI for this follow-up is pending publication at commit time;
its result must be checked on the pushed commit. No merge was performed.
