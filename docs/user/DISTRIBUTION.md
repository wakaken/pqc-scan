# Installation and distribution

[日本語](DISTRIBUTION.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/develop/README.md)

This guide covers source installation from the **v0.3 development checkout** and the published **v0.2.0** archives. Build this checkout with its matching rules to use the development changes; downloading the v0.2.0 archive does not include them. The public
distribution channel is GitHub Releases with a static macOS Apple Silicon archive
(`aarch64-apple-darwin`), its matching rules/notices, SHA-256 checksums and a GitHub
build-provenance attestation. A merged candidate or Actions artifact is not yet a
published Release. See the [v0.2 release notes](../releases/v0.2.0.md).

Maintenance is best-effort for the latest verified 0.2.x snapshot. No SLA or
older-version backport guarantee is declared. Probes remain experimental;
combined/all-probes packages are technical preparation and retain their notice
review flags. Other binary platforms and crates.io publication are outside this
release scope. See [security reporting](../../SECURITY.md).

<a id="release-status"></a>

## Distribution variants and probe publication checks

The v0.2 candidate has passed hosted CI and archive repeatability/clean-install
checks for all three macOS arm64 variants. Only `static` is included in public binary distribution. Its dependency inventory has no missing notices or
`notice_review_required` flags. See the [v0.2.0 GitHub Release](https://github.com/wakaken/pqc-scan/releases/tag/v0.2.0)
for published assets. The probe bundles below are not part of that release.

| Variant | Optional commands | Public distribution status |
|---|---|---|
| `static` | None | v0.2 distribution target; verify hosted build provenance |
| `combined-probes` | `probe tls`, `probe ssh` | Technical validation completed; four dependency notice reviews remain |
| `all-probes` | All above plus `probe ssh-kex` | Technical validation completed; those four reviews plus one source-provenance review remain |

The unresolved dependency checks are recorded in the
[exact-version notice manifest](../../third_party/notices/manifest.json):

| Dependency | Affected bundles | What must be checked before publication |
|---|---|---|
| `block2 0.6.2`, `dispatch2 0.3.1`, `objc2-encode 4.1.0`, `objc2 0.6.5` | Both probe variants | The collected upstream statements link permission texts and raise Apple SDK-derived-work questions. Collect the applicable complete texts/attributions, review the redistribution conditions for these exact versions, and record a supported publication decision. |
| `russh-util 0.52.0` | `all-probes` | Its registry archive records `dirty=true` in Cargo VCS metadata. Compare that exact archive with the recorded upstream revision, account for the source differences and their notices, and record the result. The bundled Apache-2.0 text alone does not prove source identity. |

These are open notice/source-origin reviews, not evidence that a probe is broken
or that a dependency is unlawful. Preserve the flags until the checks are
resolved with evidence. If a dependency is replaced or upgraded, repeat its
feature, protocol, packaging and installation checks.

The static variant does not include these dependencies and is distributed
separately from the pending probe reviews. Release assets include the archive
validated by CI/distribution checks for the release commit, its SHA-256,
`build-provenance.jsonl` and bilingual notes. Verify provenance against the source
commit recorded in the Release.

Publishing a probe bundle is additional scope: resolve its reviews above,
provide and verify hosted provenance for that variant (the current workflow
attests only `static`), then validate the final release inputs before upload.
See the [candidate validation record](https://github.com/wakaken/pqc-scan/blob/develop/docs/quality/V02-RELEASE-VALIDATION.md) for the
completed technical checks. `review_required` is not an end-user setting to disable.

## Source installation

The workspace currently declares package version `0.2.0` during v0.3 development. Record the source commit and keep its binary/rules together; `--version` alone does not identify the development snapshot. Local package filenames below follow the Cargo version and do not establish a new official release.

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
python3 scripts/package-release.py --verify target/task-038/static/pqc-scan-0.2.0-aarch64-apple-darwin-static.tar.gz
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

The archive normalizes paths, permissions, owners and times. Repackaging the same
binary and unchanged source/tool inputs must yield identical bytes. This does not
claim independently reproducible Rust binaries or cross-host reproducibility.

## Upgrade and rollback

Keep the previous bundle and its rules together. Verify the new archive, unpack
into a new empty directory, inspect its provenance, run `--help` and `rules list`,
and scan a nonsecret fixture before switching your invocation path. Use explicit
absolute paths so the binary and rules belong to the same snapshot. Roll back by
using the previous bundle path; do not overlay rule packs or silently replace
custom rules. Keep your scan configuration and reports for comparison. Automatic updates are not provided. Hosted static build provenance is verified
separately as described below; a locally packaged archive has no such attestation
by default.

## Variant including SSH KEX

`all-probes` enables `probe-tls,probe-ssh,probe-ssh-kex`. The existing two variants
retain their command surfaces.

```sh
cargo build --locked --release --package pqc_scan_cli --features probe-tls,probe-ssh,probe-ssh-kex --target-dir target/all-probes-release-build
python3 scripts/package-release.py --binary target/all-probes-release-build/release/pqc-scan --variant all-probes --out-dir target/task-038/all
python3 tests/distribution/check-install.py target/task-038/all/*.tar.gz --out-dir target/task-038/all-install
```

Dependency notices and provenance use the same feature set. Installation validation
checks exact TLS/SSH/SSH KEX command availability and the public host-key pin option.
The four existing objc2-family review flags remain. This variant also flags
russh-util: its Cargo archive VCS metadata records dirty=true, so the difference
from its recorded upstream revision needs publication review. Full license text
from that fixed revision is bundled. This remains unsigned, unpublished technical
preparation.

## Verify hosted build identity

For an archive generated by the hosted preparation workflow, verify provenance
against the repository, workflow and the exact source commit intended for release:

```sh
gh attestation verify pqc-scan-0.2.0-aarch64-apple-darwin-static.tar.gz --repo wakaken/pqc-scan --signer-workflow wakaken/pqc-scan/.github/workflows/release.yml --source-digest FULL_RELEASE_COMMIT_SHA
```

Use the commit recorded for the selected release, rather than trusting a version
label or a checksum alone. Verification proves the recorded build identity; it
does not certify scanner accuracy or dependency licensing. Local archives remain
unsigned unless accompanied by a successfully verified hosted attestation. Public
tagging/upload is a separate maintainer release action after candidate acceptance.
