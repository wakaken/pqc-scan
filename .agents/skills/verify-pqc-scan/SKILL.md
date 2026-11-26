---
name: verify-pqc-scan
description: Verify the real pqc-scan CLI, static detections, JSON/SARIF/CBOM exports, rendered HTML reports, and source-bound corpus evidence. Use after affected changes or when proving pqc-scan behavior through poteto-mode.
---

# Verify pqc-scan

Use this as poteto-mode's project driver. Keep pstack's development and independent review orchestration. Read [AGENTS.override.md](../../../AGENTS.override.md), [verification requirements](../../../docs/agent/VERIFICATION.md), and [report requirements](../../../docs/agent/REPORTS.md). Those requirements and the original acceptance criteria accumulate. No mode waives them.

## Launch

Run from the repository root on a POSIX host with Python 3 and rustup. Cargo selects Rust 1.93.1 through `rust-toolchain.toml`. The helper uses only the Python standard library. It builds the real default-feature executable with `cargo build --locked --bin pqc-scan`, then drives that exact binary. It needs no database, credentials, production inputs, server, or dependency installation in scanned projects.

```sh
.agents/skills/verify-pqc-scan/scripts/verify.py doctor
.agents/skills/verify-pqc-scan/scripts/verify.py smoke
```

Success prints `Evidence retained at` with a fresh absolute run directory and exits 0. Read `manifest.json` and command logs before claiming success. A retained directory alone can belong to a failed attempt. The helper writes only a new `target/verification/smoke-*` directory and its own temporary fixture directory. Each attempt builds in its own `cargo-default` directory. Check free disk before launching, and run feature builds serially when capacity is limited. Never reuse a binary that another build can overwrite.

## Doctor

The read-only doctor checks the pinned Rust/Cargo tools and the default JWT rule file. Inspect a built executable with its version and SHA-256:

```sh
.agents/skills/verify-pqc-scan/scripts/verify.py doctor --binary "$VERIFY_BINARY"
```

Set `VERIFY_BINARY` to the absolute binary path in the selected manifest first. Compare the printed hash with that manifest. Doctor establishes readiness, not detection correctness. Missing dependencies, a changed binary, failed startup, or a rules mismatch must be resolved before driving.

## Drive

Choose the narrowest applicable mode, then add every check required by the change. Use the [feature map](features/README.md) to select user journeys.

| Mode | Required scope | Completion meaning |
| --- | --- | --- |
| Smoke | Run the helper. It drives help, version, rules list, positive and negative scans, English/Japanese reports, native inventory, SARIF, CBOM, CycloneDX headers, invalid language and invalid size. | A small real CLI/export proof. Browser, detector families beyond the fixture, official schemas, corpus accuracy, and optional features remain untested. |
| Regression | Smoke plus affected feature-map commands, positive/negative/error/compatibility cases, and rendered browser observations when required. | Evidence covers the changed behavior and its callers. Add the workspace checks required by the change category. |
| Full | Regression plus all applicable workspace/tooling checks, optional-feature configurations, in-tree quality corpora, and required HTML journeys. Add actual OSS evaluation only when its frozen external inputs and coordinator are available. | Report every required result separately. A missing OSS population, browser check, CI job, or feature configuration remains unexecuted. |

For Rust changes, run the following in a new run-owned directory. Record the exact command and exit result, preserve stdout/stderr, and bind the results to the final source. `cargo fmt --all --check` is a read-only check; implementation work must also apply `cargo fmt --all` and inspect its changes as required by the verification guide.

```sh
VERIFY_CHECKS="$(mktemp -d "$PWD/target/verification/checks-XXXXXXXX")"
export CARGO_TARGET_DIR="$VERIFY_CHECKS/cargo-default"
cargo fmt --all --check
cargo build --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Optional probes need separate directories for each selected combination. Derive the current features from `crates/pqc_scan_cli/Cargo.toml`. At minimum, affected `probe-tls`, `probe-ssh`, `probe-ssh-kex`, combined, and all-feature behavior must be checked when relevant. Bind both test executables and the CLI to hashes before running protocol evidence. Use existing local-peer tests; never contact Internet endpoints to prove static scanning. Default tests do not cover disabled features.

```sh
CARGO_TARGET_DIR="$VERIFY_CHECKS/cargo-all" cargo test --workspace --all-features --locked
CARGO_TARGET_DIR="$VERIFY_CHECKS/cargo-all" cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
node --test shell/compact-report.test.cjs
```

Run only applicable additional Node/Python/script suites and package checks from current CI. Check [the quality map](features/oss-quality.md) for exact existing corpus commands. No default mode starts a performance benchmark. Measurement requests require pstack's benchmark checklist and the existing measurement protocols.

## Evidence

`manifest.json` records the revision, Git dirty status, SHA-256 of every tracked and nonignored untracked file, a canonical file-map hash, tool versions, feature/profile/thread settings, exact argv/cwd/exit codes, binary hash, assertions, artifact hashes, and cleanup status. It captures source before and after the run and fails if it changes. Finalize edits before running. Preserve both failed attempts and reruns.

The retained `inputs/` contain only synthetic fixtures. `reports/en`, `reports/ja`, and `reports/negative` contain the real CLI outputs. Command stdout and stderr are separate files. The fixture proves one Rust RS256 reference with `JWT_RS256` at `app.rs:1` and one benign negative. It does not prove universal crypto detection, readiness, or absence of risk. Inventory legacy associations and SARIF rule/count correspondence are asserted. Official schema validity and broader reference integrity use existing contract tests in the feature map.

For additional commands, save source/input/configuration bindings, argv, cwd, environment settings that affect the result, start/end times, exit result, logs, binary hashes, and artifact hashes in a separate receipt under the run. Record selected tests and unsupported/unexecuted cases. Never dump the whole environment, source diffs, credentials, or private key data into evidence. Use synthetic fixtures and public test assets. Do not execute scanned source or resolve its dependencies.

Browser evidence must include actual browser/version, viewport, loopback URL, action and resulting state, screenshots, keyboard observations, and offline observations. Follow [the HTML journey](features/html-report.md). DOM assertions and the helper's file existence checks do not satisfy rendered acceptance. Keep browser and corpus evidence distinct from Cargo tests and CI results.

Reassess evidence after changes using the verification guide. A source hash change requires an applicability explanation or affected reruns. Never relabel a historical pass as proof of changed source.

## Cleanup

The helper kills and reaps only each command's own process group and removes its own temporary scan fixtures on success or failure. It retains evidence, reports, synthetic input copies, logs, and its isolated Cargo directory. Interrupted or failed attempts remain available. Do not remove historical evidence or another task's build output to gain space.

For HTML, stop only the dedicated server PID you started when viewing is complete unless the requested viewing workflow keeps it alive. Preserve the user's report tab and give its loopback URL. No scanner daemon exists to stop.

## Helpers

[`scripts/verify.py`](scripts/verify.py) is executable and implements only `doctor` and `smoke`. Regression and Full compose existing repository checks. Run the generated driver end to end before treating changes to this skill as verified. Validate frontmatter, links, executable mode, all five feature files, and their four H2 sections. An unrun mode must be reported as unrun.

Use `pstack:maintain-verification-skill` when behavior changes so the map continues to describe actual entry points.
