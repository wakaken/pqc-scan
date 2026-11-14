# Runtime probes: source-bound local validation

Date: 2026-11-10. Selected local code HEAD:
`9201dc5022ea918e1416db7abf710db58a4bd855`, on
`codex/inspection-format-support`. The original local `3c357ce` is preserved.
No push, PR, merge, tag, publication, signing credential or repository setting
change occurred. The user delegated the technical selection and continuation;
[the selected contract](../../architecture/RUNTIME-PROBE-IMPLEMENTATION.md) records
that choice. Existing task030 and task034/035 approvals remain in force.

[Machine bindings](RUNTIME-INTEGRATION-BINDINGS.json) bind 147 code/config/schema/
fixture files and executed binaries. Most protocol tests ran on that exact source
snapshot while HEAD was `cae2730` and the worktree contained these implementations;
all 147 files now match the selected committed code HEAD byte-for-byte. Final
default and Node suites ran after the code commits. Source-bound execution is not
hosted current-HEAD CI. Later evidence-only commits may follow without changing
these bound sources.

## Local task boundaries and independent evaluation

| Task | Commit | Evaluated scope and remaining acceptance |
|---|---|---|
| 031 | `48fad23`, adoption `cae2730` | Done; tested pinned backend selected under user delegation |
| 032 | `dac4516` | Verified fresh TLS implementation and common runtime reports accepted in source/protocol review; blocked on actual human-report browser acceptance |
| 033 | `727a88b` | Bounded SSH advertisements accepted, including real local OpenSSH interop; blocked on actual human-report browser acceptance |
| 038 | `9201dc5` | Local package preparation accepted in source review; final clean-source install evidence below, license/dependency gates remain |

Task032 introduces the thin common envelope/report writer; task033 adds SSH and
refreshes the final combined self-scan bindings. The two staged task snapshots
were extracted under ignored `target/probe-integration/staged-{tls,ssh}` and each
passed `cargo check --workspace --all-features --offline --locked` in a separate
target directory. The TLS snapshot also passed formatting. This checks buildable
commit boundaries; the combined test matrix validates the final bound integration.

The independent TLS reviewer reviewed ordinary trust/name/time verification,
fresh/full handshake completion and drained writes, TLS1.2/1.3 mTLS abort before
another client write, strict public-CA bounds, certificate collection, feature
isolation and secret handling. The reviewer found sub-millisecond limits were
truncated in the common envelope: a failing 1ns regression led to exact integer
nanosecond fields in both schemas/model and translated report labels. The final
common integration review accepted CLI errors/exits, endpoint/outcome/counter
invariants, schemas, masking and bilingual guides with no blocking finding.

The independent SSH reviewer checked framing/caps, ordered lists/unknown names,
stop-before-KEX/authentication behavior and real interop evidence. A UTF-8 banner
comment rejected by an overly strict ASCII check was fixed with a failing
regression; software identifiers remain ASCII and controls/overlong input denied.
Both reviewers rechecked their corrected scope. These are focused source and
protocol reviews, not a comprehensive security certification.

## Executed matrix

Toolchain: Rust1.93.1, macOS arm64 (host OS reported macOS27.2).
Commands/log hashes and exact binaries are in the machine record. All final checks
below exited0; generated files are under ignored `target/`, outside inputs.

| Check | Result |
|---|---|
| Workspace default build, release build, format check | Passed |
| Final default workspace tests | 337 passed |
| All-feature exact test executables, isolated loopback only | 377 passed, 1 deliberately ignored real-OpenSSH test, 51 executables |
| Default/all-target and all-feature/all-target clippy, `-D warnings` | Passed |
| CLI TLS-only and SSH-only feature tests under the same isolation | 2 and 1 passed respectively |
| Combined-probe release build | Passed |
| Node report/self-scan tests | 28 passed |
| Python quality evaluator tests | 20 passed |
| Packaging tests | 5 passed |
| Fixed static baseline, 3 repeats / 3 warm samples | Semantic/diagnostic/order/score stable |

Default CLI normal dependencies contain no rustls/AWS-LC/socket2/ctrlc/rcgen; the
explicit TLS normal graph contains the selected provider, with rcgen confined to
`test-fixtures`. The all-feature test union is not the default product graph.
CLI tests exercise denied internal endpoints, fixed failed reports, language
parity and private-block CA rejection without echoing synthetic content. Existing
static output, privacy, seven-language, inventory/assessment, diff and pinned
CycloneDX/SARIF offline-schema regressions remain enabled. No detector/rule logic,
legacy filename/exit contract or static inventory schema was changed.

## Protocol isolation and failures retained

macOS execution compiled outside isolation, then hashed each emitted executable
before and after exact-binary execution with:

```text
(version 1) (allow default) (deny network-outbound)
(allow network-outbound (remote ip "localhost:*"))
```

`target/protocol-final/RESULTS.json` binds the 147 sources and binaries with exit0.
The TLS matrix includes TLS1.2/1.3, classical/actual X25519MLKEM768, HRR, normal
trust/name/expiry failures, mTLS stop, SNI/IP/root provenance, strict CA input,
unsupported client profile, malformed/oversized records, certificate/resource
limits, deadlines/cancellation and no application/auth payload. See
[TLS worker evidence](../../../tasks/032/TASK-032-TLS-RUNTIME.md) for narrower commands, initial RED
runs and upstream-library limits. Provisional failed-handshake accessor values do
not become verified negotiated facts; a failure never proves server-wide PQC
absence. CertificateVerify algorithm and revocation are not observed/checked.

A macOS system OpenSSH fixture failed because its nested sandbox could not
initialize (`sandbox_init: Operation not permitted`). That failure was retained;
protections were not disabled. A local Docker Linux arm64 OpenSSH9.2p1 fixture
then ran the exact otherwise-ignored test with `--network none`, no privileged
mode/security override. Classical curve25519 and hybrid sntrup761 variants both
passed. The server recorded client packet types `[20]` only: no KEX execution,
service, userauth or channel requests. The result binds image/base digests,
compiled binary, allowed source context and server versions. See
[SSH evidence](../../../tasks/033/TASK-033-SSH-DISCOVERY.md). Build dependencies were downloaded while
preparing the fixture image; the actual protocol test had no external network.
This is one Linux SSH fixture, not Linux TLS/product/platform qualification.
Ephemeral fixture keys were cleaned up; no key/session-secret logs were retained.
The agent-started Docker Desktop was stopped after no containers remained.

## Static compatibility and self-scan

The fixed corpus/truth digests equal the prior accepted continuation baseline.
Per-family TP/FP/FN/TN and the complete detail multiset, including duplicates,
match it. Detail array order across those runs differs; no observation was removed.
Within each new three-repeat run semantic, diagnostic, order and score stability
are true. Existing misses/false positives remain visible, including holdout source
and configuration misses. This finite synthetic corpus is not an accuracy or
latency guarantee. Timing samples are observations and may include concurrent
local checks; no release performance threshold is approved here.

Raw self-scan: **1071 = 1004 existing reviewed + 67 retained**; all119 registered
sources verified. Exactly37 existing result positions in main.rs and report/lib.rs
were projected from accepted `802f2c6` source/raw SARIF through unchanged source
lines. Full result bodies match apart from corresponding line positions; rule,
column, count and review reasons are unchanged. The193 historical regression
records remain byte-equivalent. New exceptions:0. An independent reviewer checked
the candidate and in-memory projection before application. The canonical policy
digest is `7747a92afaae9bb8e965195910171c8ea4c23b782c25847a8705c6e3ca85b125`.
Compact source formatting was retained after structural-equality verification.

The initial Node run failed2 source-binding checks, as intended for stale
classification hashes. After this reviewed exact projection, all28 passed.
`target/probe-integration/self-scan/report.sarif` is unchanged raw evidence;
`code-scanning.sarif` is only the existing upload projection. No remote alerts or
historical triage were changed and no new test-directory/rule exclusion was added.

## Distribution and remaining gates

[Task038 clean-source evidence](../../../tasks/038/evidence/TASK-038-FINAL-CLEAN.json) records two variants,
each packaged twice from clean `9201dc5`, byte equality, integrity checks and
network-denied empty-prefix installation with installed rules and an external
synthetic input. All12 commands exited0. Package repeatability means identical
supplied binary/source/tool metadata, not independent cross-host binary rebuilds.
Source/build inputs and binary hashes match the parent bindings. License notices
are missing for5 static /9 combined-probe dependencies; declared expressions and
missing entries are visible. Publication review is not satisfied.

Task036 remains blocked: the browser tool's automatic URL review rejected local
`file:` report display and the in-app browser was unavailable. No alternative
route was used to bypass that denial. Actual offline desktop/mobile/keyboard
acceptance has not been observed. Automated source/rendering tests do not establish
layout/accessibility. The original static examples remain under
`target/task-036/report-{ja,en}/report.html`. Additional no-network denied-target
runtime previews are under `target/task-036/runtime-previews/{tls,ssh}-{ja,en}`;
these are rendering samples, not completed protocol or browser evidence.

Task037 therefore remains blocked: dependency/browser acceptance, approved
baseline-derived 1.0 budgets/release scope and hosted CI for the current local
HEAD are missing. No Linux/Windows TLS or full install/platform matrix, signed
public release, independent binary rebuild or universal/compliance claim is made.
Task039 is deliberately deferred: initial SSH advertisements satisfy the selected
use case; no concrete need for a completed negotiated session was selected.
No russh/ssh2 feasibility or negotiated product mode was implemented.

## Documentation/task checks

All ten030–039 task records parse as YAML; their reported statuses match the
checkpoint above. Changed Markdown fences/local links and diff whitespace pass.
The40-task dependency graph resolves without cycles. The preexisting unquoted
backtick in task004 still prevents strict YAML parsing of that unrelated file;
it is unchanged from3c357ce and was treated as its existing no-dependency node
for the graph check. This does not claim that every legacy task YAML is valid.

## Final independent evidence review

An independent evaluator checked the final147 source/committed-code bindings,
executed log/binary/artifact hashes, the complete baseline detail multiset,
reviewed self-scan projection and125-input package scope. The evaluator confirmed
the337/377 Rust outcomes, isolated protocol/install claims and deliberately open
browser/CI/platform/notice/release gates; no blocking finding remained. The
reviewed machine-record byte SHA-256 is
`8e9d18803a2829c28111d76d2a07c8d2efed309742d7001aed420f701a263a3c`.
The old task038 dirty-snapshot summary was then clarified as superseded by its
final clean binding; no compiled source or acceptance threshold changed.
