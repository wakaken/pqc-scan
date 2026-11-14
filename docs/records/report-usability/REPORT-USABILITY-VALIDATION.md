# Human report usability follow-up

Date: 2026-11-11. Code commit:
`6945a03176623b5fdf51f0bec7f32fbe3d44f7ed`, based on local `e27de9e`.
[Source/binary/artifact bindings](REPORT-USABILITY-BINDINGS.json) record147 compiled
source/schema/fixture files, all matching the committed code. This is a requested
presentation correction to036/032. Network behavior, rules, machine schemas,
identity calculation and full Markdown ledgers were not changed. No publication,
push, PR, merge or repository setting change occurred.

## Why the previous HTML was confusing

The saved Japanese sample had83 rule groups/294 findings, plus712 always-rendered
rows outside the finding filters/pagination: coverage13, diagnostics2,
observations325 and policy372. The latter included369 assets,358 outside the local
policy. Long observation/asset IDs consumed52,612 bytes without a direct usable
file-location link. Coverage was appended after all findings. Source/file parsing,
not browser observation, established these counts.

## What changed

- Coverage precedes finding filters: selected/analyzed/follow-up file counts,
  readable completion state, actual exclusions, file-size ceiling and unknown
  unenumerated population. Legacy duplicate scanned/skipped cards are omitted
  only when this richer overview is present. Legacy report entrypoints retain
  their five metrics.
- Failed, partial, unsupported and unknown checks remain distinct; file counts
  are never presented as a percentage of all cryptographic use. Enumeration
  failures are listed separately from selected-file counts.
- Observation and per-asset ID ledgers are absent from HTML, not merely hidden.
  Exact per-record evidence states and decisions remain in JSON/Markdown. The
  optional policy summary shows aggregate counts and its advisory limitation.
- Diagnostic reasons are grouped, with at most three escaped example paths per
  reason and an explicit remaining count. Full records stay in Markdown and
  optional diagnostics JSON. Reason details are closed initially.
- Runtime HTML starts with endpoint/outcome, observed facts, unconfirmed scope,
  reasons and next checks. The full original field table remains in a closed
  native-details section. TLS summaries require ordinarily verified fresh
  handshake facts, a nonfailed envelope and compatible diagnostics; SSH offers
  stay unauthenticated advertisements. Runtime JSON/Markdown are unchanged.

The same55-file sample retains294 findings and the existing canonical legacy
semantic digest. Its coverage remains55 selected/55 analyzed, zero failed,
partial, unsupported or unknown files,2 recorded exclusions and an unknown
not-enumerated population. Separate CLI runs have new run/observation/assessment
IDs; these were not stripped to claim byte equality. Same-document language
parity and machine-schema/golden tests remain enabled. A sample inventory diff
reports no additions/removals/changes,292 unchanged assets/248 unchanged occurrences
and77 unresolved records; ambiguity is not forced into a clean comparison.

| Language | Previous HTML bytes | Updated HTML bytes | Reduction |
|---|---:|---:|---:|
| Japanese | 1,049,042 | 917,609 | 12.5% |
| English | 921,494 | 796,703 | 13.5% |

Old samples are retained under `target/report-ux/before-{ja,en}` and as
`before-{ja,en}.html`. Regenerated samples are under `target/report-ux/after-{ja,en}`
and copied back to the originally viewed `target/task-036/report-{ja,en}`.
Original finding/action associations remain intact; globally deduplicating the
remaining action payloads was not part of this correction.

Runtime previews are actual denied-loopback CLI runs under `(deny network*)`,
not completed connection examples. They were refreshed at
`target/task-036/runtime-previews/{tls,ssh}-{ja,en}`. Same-input runtime HTML grows
885–989 bytes (6.44–7.15%) to add the readable summary; initially visible raw-field
rows are zero. This is reduced visible complexity, not a runtime file-size claim.

## Executed verification and independent reviews

Rust1.93.1 on macOS arm64. All final commands in the machine record exit0:
workspace release build/fmt, default tests344, all-feature tests384 with1 ignored
OpenSSH fixture across51 isolated executables, default/all-feature clippy with
`-D warnings`, Node28 and quality Python20. All-feature execution binds hashes
before/after with the existing loopback-only sandbox profile. No protocol source
was changed; the prior isolated real-OpenSSH evidence was not re-run or claimed
as proof of this HTML. Packaging remains bound to the earlier9201dc5 source.

Meaningful RED/GREEN regressions cover overview ordering/actual limits, bounded
reason examples, omitted HTML IDs with full Markdown retention, TLS/SSH summaries,
unknown offers and actionable failures. A preliminary frozen-source matrix also
passed, then was superseded after the following review correction.

The independent static reviewer accepted counts/status/scope, escaping, unchanged
finding/action/JS bindings and corrected the unsupported count label to explicitly
say files. The independent runtime reviewer reproduced contradictory generic
`Failed`/fatal-diagnostic input promoting detail fields to a verified TLS claim.
A failing regression was added, then a typed guard denied failed/fatal envelopes
while preserving verified handshakes with only post-handshake collection limits.
The reviewer independently ran all12 runtime-report tests and accepted the fix.

The self-scan source refresh was independently checked before application:
**1075 raw =1004 existing reviewed +71 retained**, all119 source bindings verified.
Only compact.rs's source SHA changed in the policy; its four existing result
positions/bodies/digests and all193 historical regression records are unchanged.
New exceptions:0. The stale-source Node run failed1 check as designed; all28 passed
with the reviewed source binding. Canonical policy digest:
`72287c356bf8e31940bdf12860b5f1f3ef88cd56b204d576df0a8981159b5afd`.
Raw SARIF remains under `target/report-ux/self-scan`; no remote alert/triage changed.

## Requested captures and task explanation

Actual screenshots could not be taken. Binding the user's already-open in-app
`file:///.../target/task-036/report-ja/report.html` tab was explicitly blocked by
Browser Use URL policy. The computer-use task was closed with that result. No
alternate browser, localhost server or other route was used to bypass it, and
no generated mockup is presented as a capture. Source tests/file-size measurements
are not real desktop/mobile/keyboard/browser acceptance. Tasks032/033/036 retain
that blocked gate; current-HEAD hosted CI also remains unexecuted.

| Task | User-facing capability | HTML change |
|---|---|---|
| 030 | Explicit target authorization, pinned address, finite attempts/time/traffic | No standalone HTML; infrastructure for endpoint probes |
| 031 | Isolated study selecting the tested TLS/provider implementation | No HTML; feasibility evidence is the deliverable |
| 032 | One ordinarily verified fresh TLS observation with version/suite/group/public-certificate facts | A separate endpoint HTML/JSON/Markdown, not the repository report |
| 034 | Opt-in standard CycloneDX1.7 crypto and dependency JSON exports | No dedicated HTML feature; native capture previously caused036 to append observation rows, now removed |
| 035 | Explicit-scope inventory comparison without declaring absence to be remediation | JSON/Markdown comparison only; no dedicated HTML |

The confusing static overview/observation/assessment additions came from036.
The fix preserves the useful records without requiring ordinary HTML readers to
interpret their internal identifiers or state enums.

## Final independent evidence check

The independent evaluator verified all147 source hashes against the current tree
and6945a031, log/artifact/executable hashes, sample bytes/coverage/294 findings,
removed HTML IDs and the unchanged canonical legacy facts. The evaluator also
confirmed the self-scan binding and deliberately blocked032/036 statuses. No
blocking correction remained. Reviewed machine-record byte SHA-256:
`8a829831ee3116138c1895733988fc14736f140529ff161da3dfef80629a7ee0`.

Additional user-readable examples were generated locally: CycloneDX's eight
opt-in outputs under `target/report-ux/cyclonedx/` (the two1.7 files are JSON),
and the035 comparison at `target/report-ux/diff/scan.diff.v1.md`. These are files,
not screenshots or dedicated HTML sections for030/031/034/035.
