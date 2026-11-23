# Installation and distribution

[日本語](DISTRIBUTION.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/7bcbca20bc61380873005be71440bd53889ff5dd/README.md)

This guide covers **v0.4.0 static candidate** distribution on macOS Apple Silicon
(`aarch64-apple-darwin`). Download published assets from
[GitHub Releases](https://github.com/wakaken/pqc-scan/releases), keeping the binary,
matching rules/notices, SHA-256 checksums and hosted build provenance together.
Local preparation and Actions artifacts are not published Releases. See the
[v0.3.0 notes](../releases/v0.3.0.md) and historical [v0.2.0 notes](../releases/v0.2.0.md).
Earlier downloads do not contain v0.4.0 changes. See the [v0.4.0 notes](../releases/v0.4.0.md).

The latest verified snapshots in 0.2.x and 0.3.x receive best-effort maintenance.
No SLA or older-version backport guarantee is declared. Probes remain experimental;
the `network` package retains its notice review flags and publication hold.
Other binary platforms and crates.io publication are outside this release scope.
See [security reporting](../../SECURITY.md).

## Static 0.4.0 preparation

The candidate adds offline `assess`, `context-template`, and `serve --migration`.
Keep saved `scan.inventory.v1.json` bytes with their context JSON. The bundled
[report guide](REPORTS.md#saved-assessment) explains schedules, actions and
conditions for proceeding; [saved evidence](SAVED-EVIDENCE.md) explains binding.
The archive contains both language versions of these guides and the matching
rules and notices. `uuid-simd 0.8.0` and `vsimd 0.8.0` now have exact-revision
upstream notices registered under the existing offline verification guards.

A candidate is not a published release. Prepare it twice on the final clean SHA,
compare bytes, verify checksums and perform clean offline installation. Obtain
hosted CI, static build provenance and exact-source verification. Then record
bounded maintainer product/publication approval before tagging and uploading.
The existing manual preparation workflow also packages `network`; its overall
result and the static step's result must be distinguished. Static success does
not approve a network release. Existing maintenance promises below remain
unchanged; v0.4 acceptance is a separate recorded decision.

## Static 0.3.0 (previous release)

The maintainer accepted the bounded static macOS Apple Silicon product and
authorized publication after final hosted build-provenance verification.
The [technical evaluation](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/releases/static-candidate-20261117/AUDIT-FIXES.md)
records its checks and remaining limits. Acceptance, build identity and actual
Release publication are separate facts. Keep the binary, rules, archive manifest
and source commit together; the version string alone is insufficient.

Unix writers reject output symlinks and use private staged files for ordinary
replacement. New files use mode 0600; old inode permissions and ACLs are not
preserved. Non-Unix output is unsupported. Version-to-version position/evidence
corrections, additional Java RSA references and corrected certificate attributes
can change counts, values and affected IDs. Risk/confidence policy and format
versions remain separate from these changes; see [compatibility](../releases/v0.3.0.md#compatibility). Reports remain bounded investigation evidence, not proof of repository safety or completed PQC migration. Network publication holds remain in place.

<a id="release-status"></a>

## Distribution variants and probe publication checks

The v0.2 candidate has passed hosted CI and archive repeatability/clean-install
checks for all three macOS arm64 variants. Only `static` is included in public binary distribution. Its dependency inventory has no missing notices or
`notice_review_required` flags. See the [v0.2.0 GitHub Release](https://github.com/wakaken/pqc-scan/releases/tag/v0.2.0)
for those historical assets; current published static assets are listed in
[GitHub Releases](https://github.com/wakaken/pqc-scan/releases). Probe bundles remain excluded.

| Variant | Optional commands | Public distribution status |
|---|---|---|
| `static` | None | v0.4.0 candidate in preparation; v0.3.0 published separately; verify the chosen release's provenance |
| `network` (network diagnostics included) | `probe tls`, `probe ssh`, `probe ssh-kex`, plus static scanning | Unpublished; four dependency notice reviews remain |

The unresolved dependency checks are recorded in the
[exact-version notice manifest](../../third_party/notices/manifest.json):

| Dependency | Affected bundles | What must be checked before publication |
|---|---|---|
| `block2 0.6.2`, `dispatch2 0.3.1`, `objc2-encode 4.1.0`, `objc2 0.6.5` | The network variant | The collected upstream statements link permission texts and raise Apple SDK-derived-work questions. Collect the applicable complete texts/attributions, review the redistribution conditions for these exact versions, and record a supported publication decision. |

The `russh-util 0.52.0` source comparison found that all three Rust files match
the recorded revision and the original manifest differs only in the package
version. Its `dirty=true` archive record is preserved, and the Apache-2.0 text
remains bound to that revision. This resolves that technical review flag; it does
not approve publication. The network variant remains on hold for the four notice
reviews above. See the [follow-up evidence](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/tasks/063/TASK-063-RESULTS.md).

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
See the [candidate validation record](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/records/v0.2-release/V02-RELEASE-VALIDATION.md) for the
completed technical checks. `review_required` is not an end-user setting to disable.

## Source installation

This checkout declares candidate product version `0.4.0` for static scanning. Record the source commit and keep its binary/rules together; `--version` alone does not identify the candidate snapshot. Local package filenames below follow the Cargo version and do not establish a new official release.

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
The `static` variant has no optional runtime probes. `network` enables
`probe-tls,probe-ssh,probe-ssh-kex`; probes require separate explicit commands and authorization.
Read the bundled [probe authorization and usage guide](PROBING.md) before using
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

Choose **Scan this folder**; the report opens after generation. Stop with Ctrl+C.
The server binds only to `127.0.0.1`. Open the complete session URL printed in the
terminal. The archive includes the [scanning guide](SCANNING.md#local-web-scanning)
and [report guide](REPORTS.md) in English and Japanese.

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

## Network diagnostics and custom source builds

New archives use exactly two names: `static` and `network`. The former
`all-probes` distribution is now `network`; `combined-probes` is no longer
prepared. The preparer rejects both old selectors. Archive verification and
installation checks still accept historical archives; past records retain their
original names. The v0.2 three-variant checks above describe that historical candidate.

Internal Cargo feature switches remain independent. Custom source builds can
still select, for example, `--features probe-tls,probe-ssh`; this is not a third
distribution configuration. `network` includes SSH advertisement collection and
the separate pinned hybrid SSH key exchange command.

```sh
python3 tests/distribution/check-install.py target/task-038/network/*.tar.gz --out-dir target/task-038/network-install
```

Dependency notices and provenance use the same feature set. Installation validation
checks exact TLS/SSH/SSH KEX command availability and the public host-key pin option.
The four existing objc2-family review flags remain. The russh-util source
comparison is resolved as described above; its archived dirty=true fact is
preserved. Full license text from the fixed revision is bundled. This remains
unsigned, unpublished technical preparation.

## Verify hosted build identity

The following example is for the published v0.2.0 archive. A locally prepared
0.4.0 candidate has no hosted attestation by default. For an archive generated
by the hosted preparation workflow, verify provenance
against the repository, workflow and the exact source commit intended for release:

```sh
gh attestation verify pqc-scan-0.2.0-aarch64-apple-darwin-static.tar.gz --repo wakaken/pqc-scan --signer-workflow wakaken/pqc-scan/.github/workflows/release.yml --source-digest FULL_RELEASE_COMMIT_SHA
```

Use the commit recorded for the selected release, rather than trusting a version
label or a checksum alone. Verification proves the recorded build identity; it
does not certify scanner accuracy or dependency licensing. Local archives remain
unsigned unless accompanied by a successfully verified hosted attestation. Public
tagging/upload is a separate maintainer release action after candidate acceptance.
