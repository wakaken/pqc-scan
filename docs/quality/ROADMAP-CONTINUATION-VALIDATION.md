# Roadmap continuation: local implementation acceptance

Date: 2026-11-10. Baseline:
`3c357cef879f69ae4058fefe7a03b4f074283b3f`.
Accepted integration code HEAD:
`802f2c6aa7436ce02332c2c3013c835d8c85d904`.
Branch: `codex/inspection-format-support`. The initial worktree was clean.

The source snapshot and executed commands are bound in
[ROADMAP-CONTINUATION-BINDINGS.json](ROADMAP-CONTINUATION-BINDINGS.json): 194
committed code/config/schema/fixture/user-guide files match this code HEAD and
their SHA-256 values. Source-map digest:
`3651f16408a0309703b6d4116526e929e67ebc88d51b3cf2aeaae839592205a1`.
The independent task031 feasibility workspace is subsequent work and is not
included in these acceptance claims. Documentation-only acceptance records may
follow this code revision without changing the bound runtime sources.

## Scope and decisions

The maintainer explicitly selected task030's public-unicast default, mandatory
host/port commands and finite ceilings, and task034/035's separate output and
comparison contracts. See [the decision record](../roadmap/CONTINUATION-DECISIONS.md).
Existing scan remains local; no TLS/SSH protocol command exists at this revision.

| Task | Local commit | Evaluation |
|---|---|---|
| 036 | `0963ebb` | Implementation/source review accepted; blocked on real-browser acceptance |
| 034 | `0a315ec` | Done: separate truthful opt-in CycloneDX exports |
| 035 | `2d476a9` | Done: scope/identity/completeness-aware comparison |
| 030 | `802f2c6` | Done: explicit policy, cancellable resolution and bounded optional transport |

Commits are separated by task. Shared source/doc changes were staged from
task-specific snapshots while preserving the final working tree. Each staged
source snapshot independently passed `cargo check --offline --locked --workspace`
after extraction into ignored `target/roadmap-continuation/staged-tree`. These
checks prove buildable commit boundaries; the test matrix below validates the
combined bound integration source. No push, PR, merge or publication occurred.

## Executed checks

Rust toolchain: 1.93.1, host `aarch64-apple-darwin`. All commands below exited 0.
Logs are under ignored `target/roadmap-continuation/`, outside fixture roots.

| Check | Actual result |
|---|---|
| `cargo build --workspace --locked` | Passed |
| `cargo build --workspace --release --locked` | Passed; binary SHA bound in JSON |
| `cargo fmt --all --check` | Passed after final formatting |
| `cargo test --workspace --locked` | 321 passed |
| Workspace tests with `pqc_scan_cli/probe-transport`, separate target dir | 338 passed |
| Default workspace clippy, all targets, `-D warnings` | Passed |
| CLI/core transport-feature clippy, all targets, `-D warnings` | Passed |
| `node --test shell/*.test.cjs` | 28 passed |
| Python quality-harness unit tests | 20 passed |
| Fixed release baseline, 3 repeats/3 warm samples | Semantic stability true; numeric budgets remain unapproved |

The final opt-in CLI regression also passed after its formatting-only change.
CycloneDX's eleven exporter tests validate the pinned official 1.7 schemas and
semantic/reference/privacy properties offline. Its CLI regression proves six
default versus eight opt-in outputs, no implicit inventory file, and unchanged
legacy machine content after generation-time normalization. Diff tests include
eighteen correspondence cases and six CLI/schema/adapter cases. Existing native,
assessment, SARIF, privacy, seven-language and format-support tests remain enabled.
Test counts are execution evidence, not accuracy or security scores.

## Isolated transport evidence

Final tests use `sandbox-exec` with:

```text
(version 1) (allow default) (deny network-outbound)
(allow network-outbound (remote ip "localhost:*"))
```

Policy13, transport11 and resolver4 passed with no Internet target. Test binary
and launched CLI hashes were checked unchanged before/after execution; paths,
hashes and exits are in the JSON and
`target/roadmap-continuation/030-final-isolation-results.json`.
This is local macOS socket isolation evidence, not another-OS or hosted CI claim.

The first isolated resolver run failed while concurrent default builds used the
same CLI path. Executable replacement was suspected; the failed binary's hash
was not captured, so that cause is not established. The failure remains in
`030-isolated-resolver.log`.
The successful matrix uses a dedicated `target/probe-transport-build`. The final
record uses `030-final-isolated-*.log`, not the failed attempt.

## Independent review and corrections

- `decision_review` accepted 030 after fixed-deadline idle shutdown/notify/join and
  direct `waitpid`/`ECHILD` child-reaping assertions. Addresses, original-budget
  binding, limits and optional/default dependency isolation were reviewed.
- `decision_review` accepted 034 after full-DER certificate hashes were removed
  from the standard fingerprint field and retained in an explicit property with
  conversion loss; source-column units are preserved.
- `report_recon` accepted 035 after ambiguity was counted before candidate
  filtering, actual referenced asset facts were compared, and uncertain new
  assets remained unresolved. Independent reproductions now emit zero false moves.
- `report_recon` accepted 036's source correction: same-run-ID/different-facts
  assessments are rejected; per-ID bilingual assurance/usage and policy rows,
  custom fallback and escaping are tested. Real-browser acceptance remains open.

All meaningful reproduced defects received failing regressions before correction.
Author self-review is not presented as these independent reviews.

## Source-bound self-scan projection

At the accepted integration input snapshot:
**1033 raw = 1004 existing reviewed results + 29 retained**, all 119 registered
source hashes verified. The earlier baseline was 1008 = 1004 + 4. Newly encountered
results remain visible; none was added to the exception list.

The existing classification refresh touched 62 entries across four modified source
files. Exactly 37 positions/result digests changed; 25 results were unchanged.
Old/new result bodies are identical outside their line positions, and matched
unchanged source lines and the new raw full-result digests. Rule, column, count
and reviewed reason are preserved. The 63+130 historical regression records remain
byte-equivalent; no test directory or rule was broadly excluded.

An independent implementation checked the final candidate and in-memory projection
before application. Approved canonical policy digest:
`e02594a7513d0f457dad96e4052a4193a953d542ffe8fb7e2d0860ee993b5f10`.
Raw SARIF is retained under `target/roadmap-continuation/self-scan`; no GitHub alert
or prior triage was changed. This record does not assert future source snapshots
will match the same digests.

## Remaining gates and unexecuted work

036 remains blocked: the in-app browser is unavailable, and Chrome's automatic
URL review rejected `file:` report display. No bypass was used. Local report links
were supplied to the user, but desktop/mobile/keyboard/offline confirmation has
not been received. Simulated DOM tests do not count as real-browser evidence.

No current-HEAD hosted CI, Linux/Windows protocol/platform validation, deployed
consumer test, public TLS/SSH probe, publication or numeric release-budget approval
is claimed. Task031's pinned backend study is reviewed separately; 032 depends on
its backend decision. 038's packaging/support decisions precede 037's final release
evaluation. 039 remains conditional on a selected negotiated-SSH use case.
