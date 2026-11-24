# Distribution engineering

Maintainer reference for archive preparation, notices and custom builds.
End-user installation is documented in [the installation guide](../user/DISTRIBUTION.md).
Preparation does not create a GitHub Release or authorize network publication.

## Local archive preparation


This produces unsigned, unpublished preparation artifacts, not an official release.
The `static` variant has no optional runtime probes. `network` enables
`probe-tls,probe-ssh,probe-ssh-kex`; probes require separate explicit commands and authorization.
Read the bundled [probe authorization and usage guide](../user/PROBING.md) before using
optional network commands. Rules are always selected explicitly with `--rules-dir`; unpacking does not change
rule discovery or configure your PATH.

```sh
cargo build --locked --release --package pqc_scan_cli
python3 scripts/package-release.py --binary target/release/pqc-scan --variant static --out-dir target/task-038/static
cargo build --locked --release --package pqc_scan_cli --features probe-tls,probe-ssh,probe-ssh-kex --target-dir target/network-release-build
python3 scripts/package-release.py --binary target/network-release-build/release/pqc-scan --variant network --out-dir target/task-038/network
python3 scripts/package-release.py --verify target/task-038/static/pqc-scan-0.4.0-aarch64-apple-darwin-static.tar.gz
```

Preparation uses offline Cargo metadata: fetch dependencies during an authorized
build first. Binary provenance records the supplied binary's hash, not proof that
it was built from the recorded source. Bind it to separate build logs before any
release. Dirty state, source inputs, features and tool versions appear in
`PROVENANCE.json`. `DEPENDENCY-LICENSES.json` lists resolved normal/build packages
for the selected target/features, their declared licenses and collected notices. Exact-version upstream fallbacks
are checked against Cargo.lock checksum, archive VCS revision and bundled file
hashes, with no network fetching during packaging.
It is audit evidence, not legal certification or a proof of exact binary linkage.
Missing notices fail preparation. Static notice fallbacks include complete
upstream permission texts and original attributions. Four objc2-family
statements remain explicitly review-required: they link license texts and flag
Apple SDK-derived-work questions. They are not full permission texts or publication
clearance; see [the pinned notice inventory](../../third_party/notices/README.md).

Verify the archive's adjacent `.sha256` file using `shasum -a 256 -c FILE.sha256`
from its directory. The Python verifier checks all internal member hashes and
rejects unsafe paths and nonregular members. These checks detect modification
against the supplied manifests; without a trusted signature/source they do not
establish authenticity. Inspect the archive before extracting it into an empty
local directory with `tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY`. Its top directory
contains `bin/pqc-scan`, `rules/default`, license notices, release notes, and the installation/probe guides. Invoke:

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

The default static bundle also includes `serve`. To choose a project in the browser:

```sh
/absolute/path/to/bundle/bin/pqc-scan serve --root /absolute/path/to/projects --rules-dir /absolute/path/to/bundle/rules/default
```

Select one or more folders and choose **Scan selected folders**. Ordinary `serve`
opens the scan report automatically for one folder; multiple folders show named
result cards. Add `--migration` to choose the scan report or migration review.
Follow the [illustrated workflow](../user/SCANNING.md#web-migration-walkthrough). Stop with Ctrl+C.
The server binds only to `127.0.0.1`. Open the complete session URL printed in the
terminal. The archive includes the [scanning guide](../user/SCANNING.md#local-web-scanning)
and [report guide](../user/REPORTS.md) in English and Japanese.

The archive normalizes paths, permissions, owners and times. Repackaging the same
binary and unchanged source/tool inputs must yield identical bytes. This does not
claim independently reproducible Rust binaries or cross-host reproducibility.

## Upgrade and rollback

Keep the previous bundle and its rules together. Verify the new archive, unpack
into a new empty directory, inspect its provenance, run `--help` and `rules list`,
and scan a nonsecret fixture before switching your invocation path. Use explicit
absolute paths so the binary and rules belong to the same snapshot. Roll back by
using the previous bundle path; do not overlay rule packs or silently replace
custom rules. Keep your scan configuration and reports for comparison. Automatic updates are not provided. A locally packaged archive has no hosted attestation by default.

## Network diagnostics and custom source builds

New archives use exactly two names: `static` and `network`. The former
`all-probes` distribution is now `network`; `combined-probes` is no longer
prepared. The preparer rejects both old selectors. Archive verification and
installation checks still accept historical archives; past records retain their
original names. Historical v0.2 three-variant checks are recorded in the v0.2 release evidence.

Internal Cargo feature switches remain independent. Custom source builds can
still select, for example, `--features probe-tls,probe-ssh`; this is not a third
distribution configuration. `network` includes SSH advertisement collection and
the separate pinned hybrid SSH key exchange command.

```sh
python3 tests/distribution/check-install.py target/task-038/network/*.tar.gz --out-dir target/task-038/network-install
```

Dependency notices and provenance use the same feature set. Installation validation
checks exact TLS/SSH/SSH KEX command availability and the public host-key pin option.
The four historical objc2-family review flags remain in the notice manifest, but
the v0.4 Unix network graph does not select them. Validate the actual selected
archive inventory rather than clearing those historical flags. The russh-util source
comparison is resolved in the task 063 evidence; its archived dirty=true fact is
preserved. Full license text from the fixed revision is bundled. This remains
unsigned, unpublished technical preparation.
