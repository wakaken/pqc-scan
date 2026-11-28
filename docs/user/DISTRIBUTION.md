# Installation and distribution

[日本語](DISTRIBUTION.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/develop/README.md)

This guide explains installation on macOS Apple Silicon (`aarch64-apple-darwin`).
See the [v0.5.0 release notes](../releases/v0.5.0.md) for changes.

<a id="release-status"></a>

## Choose a download

Choose a version and variant on [GitHub Releases](https://github.com/wakaken/pqc-scan/releases).
Check the Release page for availability. Actions artifacts are separate from published Releases.

The compressed distribution file (`.tar.gz`) bundles the executable, rules and user guides.

| Variant | Filename | Features |
|---|---|---|
| `static` | `pqc-scan-0.5.0-aarch64-apple-darwin-static.tar.gz` | Local scanning, local web interface and migration review |
| `network` | `pqc-scan-0.5.0-aarch64-apple-darwin-network.tar.gz` | All static features plus explicit TLS/SSH endpoint diagnostics |

Download both the compressed file and its matching `.sha256` file separately from
**Assets** on the same release page. The `.sha256` file checks the downloaded
compressed file for corruption; it is distributed alongside it, not inside it.
Its name is the compressed filename with `.sha256` appended.

For example, download these two files for `static`:

- `pqc-scan-0.5.0-aarch64-apple-darwin-static.tar.gz`
- `pqc-scan-0.5.0-aarch64-apple-darwin-static.tar.gz.sha256`

Endpoint diagnostics require the administrator's authorization; follow the [TLS/SSH guide](PROBING.md).
Binaries for other operating systems or CPUs and installation through crates.io are outside this scope.

<a id="install-an-archive"></a>

## Install from a compressed file

1. Save the compressed file and its `.sha256` file in the same folder.
2. In that folder, run `shasum -a 256 -c FILE.sha256`, replacing `FILE.sha256` with the downloaded filename. Confirm that verification succeeds.
3. Create a new empty directory and extract with `tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY`, replacing the filename and destination.
4. Keep `bin/pqc-scan` and `rules/default` together. Extraction does not change your PATH.

The bundle includes rules, license materials, user guides, illustrated steps and
release notes. Replace `/absolute/path/to/bundle` below with the bundle's top-level directory.

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

## Use the browser interface

```sh
/absolute/path/to/bundle/bin/pqc-scan serve --root /absolute/path/to/projects --rules-dir /absolute/path/to/bundle/rules/default
```

1. Open the complete session URL printed in the terminal.
2. Select folders and choose **Scan selected folders**. Ordinary mode opens the scan report automatically for one folder; multiple folders show named results.
3. Stop the server with Ctrl+C in the terminal.

Add `--migration` at startup to use migration review. See the
[eight illustrated steps](SCANNING.md#web-migration-walkthrough) for the full workflow.

## Install from source

Obtain the repository and use the Rust toolchain pinned in `rust-toolchain.toml`.
Run from the repository root and retain the checkout so its rules remain available.

```sh
cargo install --locked --path crates/pqc_scan_cli
pqc-scan rules list --rules-dir /absolute/path/to/pqc-scan/rules/default
pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/pqc-scan/rules/default --format all --out-dir /absolute/path/outside/repository/reports
```

Building may require dependency downloads. Keep report output outside the scanned repository.

<a id="use-migration-review-in-v04"></a>

## Use the migration review feature introduced in v0.4

| Goal | Feature and guide |
|---|---|
| Enter purpose/usage and review actions and conditions for proceeding | `serve --migration`; see the [migration report guide](REPORTS.md#saved-assessment) |
| Reassess saved findings | Generate input with `context-template`, then evaluate with `assess`; see the [saved-input guide](SAVED-EVIDENCE.md) |

## Upgrade and rollback

1. Extract the new bundle into a separate empty directory. Retain the previous binary and its rules together; do not overlay rule packs.
2. Check `--help`, `rules list` and a scan of a nonsecret folder with the new binary.
3. Switch paths after checking, and retain settings and reports for comparison.
4. To roll back, use the previous bundle's path.

## Licenses and reporting issues

Bundled dependency license materials are in `DEPENDENCY-LICENSES.json` and
`licenses/`. Review reports before sharing: they can include source snippets and
paths. See [SECURITY.md](../../SECURITY.md) for vulnerability reporting.
