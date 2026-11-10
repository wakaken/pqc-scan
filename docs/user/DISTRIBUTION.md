# Local installation and distribution preparation

The current channel is a source checkout. Version 0.1.0 is a development
snapshot. Maintenance is best-effort for the latest verified development snapshot;
no SLA, older-version backports or 1.0 support guarantee is declared. Local archive
preparation covers macOS Apple Silicon (`aarch64-apple-darwin`) only. Other targets
and crates.io publication are not established by this work. See
[security reporting](../../SECURITY.md).

## Source installation

Use the toolchain pinned in `rust-toolchain.toml`, retain this checkout's reviewed
rules, and run from its root:

```sh
cargo install --locked --path crates/pqc_scan_cli
pqc-scan rules list --rules-dir /absolute/path/to/pqc-scan/rules/default
pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/pqc-scan/rules/default --format all --out-dir /absolute/path/outside/repository/reports
```

Static scanning uses local inputs and rules offline after installation. Building
may require dependency downloads. Keep report output outside the scanned root.
Only scan repositories you are authorized to inspect. Reports can contain source
and endpoint metadata; review them before sharing.

## Local archive preparation

This produces unsigned, unpublished preparation artifacts, not an official release.
The `static` variant has no optional runtime probes. `combined-probes` enables
`probe-tls,probe-ssh`; probes require separate explicit commands and authorization.
Read the bundled [probe authorization and usage guide](PROBING.md) before using
optional network commands. Rules are always selected explicitly with `--rules-dir`; unpacking does not change
rule discovery or configure your PATH.

```sh
cargo build --locked --release --package pqc_scan_cli
python3 scripts/package-release.py --binary target/release/pqc-scan --variant static --out-dir target/task-038/static
cargo build --locked --release --package pqc_scan_cli --features probe-tls,probe-ssh --target-dir target/probe-release-build
python3 scripts/package-release.py --binary target/probe-release-build/release/pqc-scan --variant combined-probes --out-dir target/task-038/combined
python3 scripts/package-release.py --verify target/task-038/static/pqc-scan-0.1.0-aarch64-apple-darwin-static.tar.gz
```

Preparation uses offline Cargo metadata: fetch dependencies during an authorized
build first. Binary provenance records the supplied binary's hash, not proof that
it was built from the recorded source. Bind it to separate build logs before any
release. Dirty state, source inputs, features and tool versions appear in
`PROVENANCE.json`. `DEPENDENCY-LICENSES.json` lists resolved normal/build packages
for the selected target/features, their declared licenses and collected notices.
It is audit evidence, not legal certification or a proof of exact binary linkage.
An empty notice list requires review before publication.

Verify the archive's adjacent `.sha256` file using `shasum -a 256 -c FILE.sha256`
from its directory. The Python verifier checks all internal member hashes and
rejects unsafe paths and nonregular members. These checks detect modification
against the supplied manifests; without a trusted signature/source they do not
establish authenticity. Inspect the archive before extracting it into an empty
local directory with `tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY`. Its top directory
contains `bin/pqc-scan`, `rules/default`, license notices and these guides. Invoke:

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

The archive normalizes paths, permissions, owners and times. Repackaging the same
binary and unchanged source/tool inputs must yield identical bytes. This does not
claim independently reproducible Rust binaries or cross-host reproducibility.

## Upgrade and rollback

Keep the previous bundle and its rules together. Verify the new archive, unpack
into a new empty directory, inspect its provenance, run `--help` and `rules list`,
and scan a nonsecret fixture before switching your invocation path. Use explicit
absolute paths so the binary and rules belong to the same snapshot. Roll back by
using the previous bundle path; do not overlay rule packs or silently replace
custom rules. Keep your scan configuration and reports for comparison. Remote
publication, signing, automatic updates and credential setup require separate work.
