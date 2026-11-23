# Installation and distribution

[日本語](DISTRIBUTION.ja.md) · [README](https://github.com/wakaken/pqc-scan/blob/codex/v04-roadmap-20261118/README.md)

This guide describes installation of **v0.4.0 static** for macOS Apple Silicon
(`aarch64-apple-darwin`). See the [v0.4.0 release notes](../releases/v0.4.0.md) for changes.

<a id="release-status"></a>

## Choose a download

On [GitHub Releases](https://github.com/wakaken/pqc-scan/releases), look for
`pqc-scan-0.4.0-aarch64-apple-darwin-static.tar.gz` and its matching `.sha256` file
under the version you want to use. Check the Release page for availability.
Actions artifacts are separate from published Releases.

This guide covers `static`, which scans local repositories and includes the local
web interface and migration review. The experimental `network` variant with
TLS/SSH endpoint diagnostics is excluded from public distribution. Binaries for
other operating systems or CPUs and installation through crates.io are outside this scope.

## Install an archive

1. Save the archive and its `.sha256` file in the same directory.
2. In that directory, run `shasum -a 256 -c FILE.sha256`, replacing `FILE.sha256` with the downloaded filename. Confirm that verification succeeds.
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

Open the complete session URL printed in the terminal, select folders and choose
**Scan selected folders**. Ordinary mode opens the scan report automatically for
one folder; multiple folders show named result cards. Add `--migration` when
starting the server to use migration review. Follow the
[eight illustrated steps](SCANNING.md#web-migration-walkthrough). Stop with Ctrl+C.

## Install from source

Obtain the repository and use the Rust toolchain pinned in `rust-toolchain.toml`.
Run from the repository root and retain the checkout so its rules remain available.

```sh
cargo install --locked --path crates/pqc_scan_cli
pqc-scan rules list --rules-dir /absolute/path/to/pqc-scan/rules/default
pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/pqc-scan/rules/default --format all --out-dir /absolute/path/outside/repository/reports
```

Building may require dependency downloads. Keep report output outside the scanned repository.

## Use migration review in v0.4

With `serve --migration`, enter purpose and usage information to see actions and
conditions for proceeding. Use `assess` to reassess saved findings and
`context-template` to prepare an input template. Retain the original
`scan.inventory.v1.json` together with its matching input JSON. See the
[migration report guide](REPORTS.md#saved-assessment) and [saved-input guide](SAVED-EVIDENCE.md).

## Upgrade and rollback

Extract the new bundle into a separate empty directory. Retain the previous binary
and its rules together. Check `--help`, `rules list` and a scan of a nonsecret
folder before switching paths. To roll back, use the previous bundle's path.
Do not overlay rule packs. Automatic updates are not provided; retain settings and reports for comparison.

## Licenses and reporting issues

Bundled dependency license materials are in `DEPENDENCY-LICENSES.json` and
`licenses/`. Review reports before sharing: they can include source snippets and
paths. See [SECURITY.md](../../SECURITY.md) for vulnerability reporting.
