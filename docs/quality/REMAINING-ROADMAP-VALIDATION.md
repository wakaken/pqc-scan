# Remaining roadmap local validation — 2026-11-11

This is a historical source-bound checkpoint. Its pending/unapproved statements
describe that checkpoint. Current task 023 acceptance and local criteria are in
[TASK-023-ACCEPTANCE.md](TASK-023-ACCEPTANCE.md) and
[QUALITY-GATES.md](QUALITY-GATES.md); current task decisions are in
[STATUS-2026-11-11.md](STATUS-2026-11-11.md). Saved measurements and reviewer
judgments below are preserved.

Implementation source: `75b9afd`, reviewed against `2826c75`. Source hashes,
commands, raw evidence hashes and package bindings are recorded in
`REMAINING-ROADMAP-VALIDATION.json`; local artifacts remain in
`target/continuation-20261111/`. Main agent implemented and executed checks;
one implementation reviewer accepted the source changes; one separate test reviewer
independently accepted test adequacy, raw outcomes, source/binary/package/page
bindings and the unchanged-line self-scan projection. No blocking findings remain.
Evaluator accepted032/033/036 and038 technical preparation within the scope below.

## Executed local checks

- Pinned Rust 1.93.1: default workspace tests 346 passed; isolated all-feature
  matrix 386 passed, one OpenSSH integration test ignored, 52 executables.
  The matrix records before/after hashes for every test executable and source.
- Format and both default/all-feature clippy completed without warnings.
- Node report/self-scan tests: 28 passed. Distribution Python: 8 passed;
  quality-runner Python: 20 passed.
- Exact unchanged-line self-scan projection: 34 results in main.rs shifted only
  by added CLI help handling. Full SARIF result bodies match after line mapping;
  no new exception or classification was accepted. Existing reviewed fixtures
  remain unchanged. Raw findings increased1075→1082: seven new test-fixture/
  assertion findings are retained without new exceptions; one other pre-existing
  unregistered result shifted lines. SELF-SCAN-NEW-RESULTS.json contains the eight
  unmatched raw bodies, not a suppression list. See rebind-candidate/LEDGER.json
  and the raw self-scan.
- CLI help/version now exit0 on stdout; invalid options still fail. Real-process
  tests cover global and subcommand information flags and default command surface.
- Actual TLS and SSH local successful/denied fixture reports in both languages
  were generated under loopback-only sandbox isolation. TLS used ordinary CA
  validation; SSH sent only KEXINIT [20], with no later packet bytes. Temporary
  fixture keys were removed and are absent from report output.
- Actual HTTP browser acceptance, captures, measured widths, keyboard, filters,
  paging and loaded-document offline checks: `LOCAL-BROWSER-ACCEPTANCE.md`.
  This closes the previously unestablished browser evidence, within that stated
  browser/viewport scope. These are synthetic local fixtures, not remote audits.

## Reproducible preparation scope

Clean source `75b9afd` produced two byte-identical archives for each macOS arm64
variant (static and combined-probes); both passed internal checksum verification
and clean-prefix installed scans with all network operations denied. A positive
fixture yielded exactly the two expected rules and a negative fixture no finding.
Installed help/version, explicit rule assets and optional command surfaces passed.
See FINAL-PACKAGES.json and final-*/INSTALL-RESULTS.json.

Static inventory has 139 packages and no missing notices or review flags.
Combined inventory has 168 packages and no missing notices; four exact-version
objc-family upstream license statements remain review-required before publication.
These flags are deliberately preserved; technical preparation is not legal clearance.
No publication, signing, push or hosted CI run occurred.

## Baseline and remaining 1.0 decision

Fixed 023-v2 corpus/input/truth hashes match the earlier probe-integration baseline.
All per-family/split metrics and the complete accuracy-detail multiset are unchanged;
only array order differs. Three repeated scans are semantically, diagnostically and
score stable. Benchmarks use three warm samples after first measurement, on the
same M4 Pro / macOS / release toolchain and configuration; the short self-scan
completed during initial runner startup, not a competing benchmark/build job.

The existing TASK-023 proposal remains unapproved: preserve TP and do not increase
FP/FN on unchanged eligible truth; flag wall time or peak RSS above twice the old
observed warm maximum. Current warm maxima / proposed wall limits in seconds:
small 0.031895 / 0.048949; medium 0.068401 / 0.123680;
large 0.158429 / 0.361025; pathological 0.024651 / 0.049299.
All current RSS maxima also fall below that proposed signal. This is descriptive
local headroom, not statistical confidence or a universal accuracy claim.

037 remains blocked on maintainer approval of budgets/1.0 support scope and actual
current-HEAD hosted CI. Local best-effort macOS arm64 preparation does not promise
other platforms or authorize a 1.0 release. 039 remains conditional/pending: no
consumer requiring completed SSH KEX was selected, so the current advertisement
mode remains sufficient and no negotiated mode was invented.

## Maintainer acceptance decision — 2026-11-11

The maintainer explicitly approved the proposed regression signals and 1.0 scope.
For unchanged eligible truth, per split/family TP must not decrease and FP/FN must
not increase. On the same host/profile/config/input, wall time or peak RSS above
twice the previous observed warm maximum requires investigation. Existing local
measurements are within these signals; this does not establish a universal accuracy
or performance guarantee or silently enable threshold enforcement in CI.

Approved scope: static scanning is the formal 1.0 target; TLS/SSH probes remain
experimental; local distribution validation covers macOS arm64. This approval
supersedes the earlier pending-budget/scope statements, without declaring 1.0
released or promising other platforms. 037 remains blocked on successful hosted
CI for the actual target commit. Four combined-probes notice reviews remain
publication prerequisites. Push, publication and signing were not authorized.
