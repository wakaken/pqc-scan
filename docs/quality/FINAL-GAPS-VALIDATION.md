# Final gap validation — 2026-11-11

Base: merged6e098de; source changes are tracked in codex/final-quality-gaps.
Scope: state reconciliation, individual self-scan triage and all-probes technical
packaging including SSH KEX. Publication/signing and existing maintainer decisions
are preserved. Implementation reviewer final_gaps_implementation_review approved the combined
change with no required fixes, checking actual result/source digests and new
notice bindings. Final-test review and command/source/archive bindings are recorded
after execution. Earlier evidence remains historical.

Code Scanning:109 retained results reviewed,81 exact expected results registered,
28 dependency observations retained (14warning/14note). Full raw1113 results retained.
No scanner/rule/severity/threshold change or live alert dismissal.

Packaging:static and combined-probes preserved;all-probes adds probe-ssh-kex to
build,metadata,provenance,notices and exact clean-install command-surface checks.
All-probes includes271 packages with no missing notices. Five publication-review
flags remain: the original four objc-family flags plus russh-util dirty-VCS provenance.

## Final execution

Implementation source31e7952 (same Rust/lock as6e098de) passed hosted CI:
https://github.com/wakaken/pqc-scan/actions/runs/42971971441 — quality,scan,baseline,
protocols all successful. Source hashes distinguish later state/evidence-only edits.

Fresh native build/fmt/workspace356/clippy succeeded; Node31,quality Python21,
distribution Python10 and kernel boundary11 passed. Strict all-feature execution:
407 Rust passed,one existing ignored,55 exact executables,all-feature clippy,
preview8 and actual OpenSSH5 cases passed. Executable/source hashes were stable;
container cleanup succeeded. Three macOS arm64 archives (static139 packages,
combined168,all-probes271) each repeat byte-identically and clean-install offline
from clean31e7952 source. Installed all-probes exposes TLS,SSH and SSH KEX with
public host-key pin help; existing variants retain their exact command surfaces.
The archives are under target/final-gaps/{variant}-first/.

## Approved performance investigation signal

Frozen corpus/truth/input hashes and all per-split/family accuracy groups are
unchanged from the accepted prior same-host study. Three repeats preserve semantic,
diagnostic,order and score stability. Rust1.93.1,release,hardware,OS,features and
input match. RSS and three workload wall maxima are within the prior2x signal.
Large warm wall maximum0.354668s exceeds twice the pre-audit0.158429s maximum
(0.316857s). This trigger is retained,not reported as a <2x pass.

Investigation rebuilt base6e098de separately: with the same current harness and
sandbox its large maximum0.358308s matches current0.354668s; native current is
0.332501s. Current and base Rust/manifests/lock are identical. Sampling on a larger
synthetic-only workload observed1012 openat and127 root-open samples among1246
worker samples,dominated by the existing link-safe held-directory read boundary.
The result identifies the existing audit safety cost rather than a new packaging
or triage regression. The approved condition requires investigation above2x;
it is not a silently revised numeric limit. No safety check was weakened.
Raw initial measurements,controlled comparison and sample are retained in
BASELINE-COMPARISON.json,baseline-base-sandbox,baseline-current-native and
profile-sample.txt. Sampling is supporting evidence,not a universal throughput claim.

## Evidence and scope

FINAL-GAPS-VALIDATION.json binds commands,source files,binaries,archives,install
records,protocol logs and metrics. The additional Node and lifecycle-test changes
have their own native execution bindings; they do not change the Rust binaries.
No HTML renderer changed in this request; no fresh browser execution is claimed.
Existing actual browser acceptance remains source-bound historical evidence.

## Final acceptance

A different reviewer,final_gaps_test_review,approved test adequacy after checking
source/evidence hashes,finite positive/negative triage cases,actual archives and
positive/negative offline installs,and the preserved performance investigation.
Both stages found no required fixes. Actual mixed-binary archive negatives also
reject missing SSH KEX,extra SSH KEX and a probe command in the static variant.

Evaluator closes001 and037 for technical acceptance on the evidence above.
The product remains beta0.1.0; the formal scope is static scanning with experimental
probes. Publication/signing and notice clearance remain separate. Action refs are now pinned to their verified upstream commit SHAs.
014/016 remain cancelled and023 done. There are38 done tasks and2 cancelled.
Final source hashes and metadata-only follow-up checks are in the JSON record.

## Fresh hosted distribution preparation

The first manual hosted attempt42973704296 exposed a fresh-cache prerequisite:
static compilation succeeded,but offline workspace metadata required an uncached
crate(ahash0.8.12). The workflow now explicitly runs `cargo fetch --locked` before
all offline packaging. This trusted dependency preparation executes no target code;
packaging/install tests retain their offline boundaries. The failed attempt remains
recorded at target/final-gaps/distribution-hosted-failed.log; it is not a pass.
The corrected workflow is checked by the same two reviewers and rerun on GitHub.

## Action immutability

All workflow uses now pin the same upstream commit previously selected by the
existing version/tag(or stable branch),with a human-readable ref comment.
Official GitHub Git-ref APIs were queried on2026-11-11;the annotated CodeQLv3 tag
was dereferenced to its commit. No action version or Rust toolchain was changed.
Pins and reference identities are in FINAL-GAPS-VALIDATION.json. Both reviewers
reuse their existing identities for this final scoped hardening check.
