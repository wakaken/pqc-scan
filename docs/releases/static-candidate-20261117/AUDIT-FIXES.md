# Static candidate audit fixes and final acceptance reconciliation

Status: in progress. Baseline a983be17297d3d87606d0b1531784a9a78c510c0.
The maintainer requests fixing independent-audit F-01/F-02, required tests,
compatibility, both existing reviews, hosted CI/Code Scanning, all 73 original
requirements and a final static macOS arm64 distribution check. No merge,
publication, tag or actual signing. Original criteria and historical evidence
remain unchanged. This is the continuation of the existing candidate work.

## Plan and compatibility

F-01: include the generic suite in the existing source-target routing to finite
scratch. Check the real output destination, readonly source/root, kernel limits,
source hashes and cleanup. Run the original run-isolated command with previews
and SSH KEX through all stages. Do not substitute dedicated protocol049 success.

F-02: the exact default TLS_VERSION_1_0 pattern has a named token capture;
iteration reports that token and reuses its trailing delimiter. Custom IDs,
modified patterns/scopes and empty generic matches retain find_iter behavior.
No severity, risk, confidence, schemas, truth or numeric gate changes. The
maintainer explicitly approved 2 -> 3 references, token locations 2:1 / 2:9 / 3:1
and affected ID changes on 2026-11-17 in this chat. This satisfies the rule
compatibility human-review boundary before implementing that rule change.

Initial checks: five real-detector boundary/negative/comment/custom/budget tests
pass. The actual CLI preserves token positions and distinct finding IDs across
English/Japanese JSON/SARIF/HTML and native observation byte spans. Markdown uses
local anchors, not full finding IDs; legacy CBOM intentionally collapses same-line
references and has no occurrence/column contract. These losses are checked,
not hidden. No report renderer or presentation code changes.

A first negative test incorrectly assumed // comments in a .conf file; corrected
using the pre-existing language-dependent comment contract, with explicit .js/.conf
controls. Initial CLI checks incorrectly queried id instead of finding_id and
assumed Markdown emits full IDs. Those assertion failures are retained separately;
no product behavior was changed to fit them. The initial isolated snapshot includes
the earlier CLI assertion and cannot establish success for the final source.

The historical task061 catalog artifact directly referenced mutable container
entry source. Its exact baseline bytes/hash are archived, not rebound as new
measurements. The catalog restoration and corruption controls remain unchanged.
The current rule companion is updated for the new source pattern; classifications,
references, prior truth and known limits are preserved.

## Completion gates

Implementation review: reuse /root/output_implementation_review, then required
final checks and /root/output_test_review. No additional reviewer identities.
Fresh default/all-feature Rust checks, relevant Python/Node/script checks, the
original confined suite, canonical compatibility, fresh hosted commit and alerts,
source-bound independent static builds/package/clean install and all 73 clause
reconciliation are required. Unknown/Failed work cannot be promoted to Passed.
Release readiness, task/roadmap evaluation completion, maintainer product
acceptance and authorization to publish remain separate decisions.

## Local results and evidence applicability

Final Rust 1.93.1 default 611 passed/0 failed/3 ignored; all features
670/0/5. Both builds and strict Clippy succeed; the placement-only Clippy
failure and earlier assertion failures remain preserved. The original generic
suite succeeds across 83 test executables, strict all-feature Clippy, previews
and SSH KEX, under unchanged kernel boundaries. Host source before/after matches
and the container is removed. This resolves the reproduced F-01 condition.

The current static binary repeats the same frozen 023/042/043 corpora three
times with stable 52/205/216 findings. Full finding fields differ only for two
TLS_VERSION_1_0 rows in each of 042/043; 023 is unchanged. F-02's added audit
case reports three references at 2:1, 2:9 and 3:1 with exact native byte spans,
EN/JA ID parity and relevant export/legacy-loss controls. This resolves that
counterexample, not all lexical false negatives.

Native independent confined A/B static builds produce identical raw binaries;
separate source/home/target directories share only readonly dependency inputs
and fixed tools on one physical macOS host. The preparatory archive contains
139 static dependencies with missing/review notice flags zero, and passes
checksum and clean-prefix offline install checks. It honestly records dirty
source; the final clean-commit candidate is still a separate gate.

Current self-scan verifies 106 files/1,284,712 bytes: 487 raw = 477 individually
reviewed references + 10 unchanged full review payloads. The two new references
are exact matcher-fingerprint literals, not cryptographic calls. The changed
positions and unchanged inline-test translations have retained source-line
reasons. The appended CLI test preserves every old byte and allowance; only its
source hash is rebound. Historical 98-result static-review tests now use exact
frozen source bytes, retaining their original evidence instead of interpreting
old locations against changed current source. A new actual-current SARIF test
verifies the 487/477/10 partition and retains a mutated new TLS payload. Node
54/0 and Python quality160 with3skips / distribution25 pass.

Current ordinary Linux default LLVM build/test/merge/export/summary succeeds
with immutable-source/cleanup checks: raw src-path lines21,086/22,575,
regions34,469/37,098, branches4,311/5,544. Inline tests are included, so these
are not production-only or macOS percentages. Seven mismatch diagnostics name
the same locked external memchr definition. The earlier auxiliary -dump
command aborted with -6 and is retained as Failed; the corrected collection
uses ordinary summary-only export and does not claim to fix LLVM. Previous
all-feature dump/fuzz limits remain. The 136 individually adjudicated static
spans' files and preconditions are unchanged; nine invariant directions remain
unobserved, not covered or universally safe.

The 73-row matrix retains every original ID, quote, owner, historical label and
source-linked artifact. Each row states current applicability and method limits.
Historical studies are reused only for their bounded evaluation methods and
recorded source/environment; no present whole-product performance, independent
holdout accuracy, badge maturity, operational readiness or continuous dependency
monitoring claim is manufactured. No final product acceptance is asserted yet.

Evidence is under audit-fixes/evidence/. Implementation R1/R2 is accepted by
/root/output_implementation_review. R3 scoped confirmation accepted the later historical/current Node evidence
separation. /root/output_test_review accepted the local tests and all73
local assessment rows after the Q6-12 browser evidence link was repaired; no
actionable finding remains. Hosted/clean-source package receipts remain pending. No original task states
or historical roadmap decisions are rewritten by this follow-up.

## Review decisions and remaining remote gates

Implementation reviewer /root/output_implementation_review accepted R1, R2 and
R3; final combined executable inputs are recorded in evidence/FINAL-SOURCE.json.
Test reviewer /root/output_test_review accepted F-01/F-02 tests and, after the
Q6-12 browser-reference correction, the local 73-row evaluation. The initial
missing browser reference is preserved as a corrected P2; no product retest was
needed for that evidence-only repair. Neither review grants product acceptance
or publication rights. Next: commit/push these verified inputs, verify hosted
CI and exact Code Scanning payloads, bind a clean-commit static package to the
verified independent build, then reconcile final acceptance without weakening
original criteria.
