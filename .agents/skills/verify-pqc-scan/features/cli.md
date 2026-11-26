# CLI and rule discovery

## Sub-features

Help, version, `scan`, `rules list`, `rules show`, argument validation, severity failure policy, and strict inspection exits.

## How to get to it (user POV)

Run `pqc-scan --help`, `pqc-scan scan --help`, `pqc-scan rules list --rules-dir rules`, and `pqc-scan rules show JWT_RS256 --rules-dir rules` from the checkout. Use absolute rule paths outside it.

## Driving it with the CLI

Run the Smoke helper. Inspect the captured help/version/rules output and invalid-input logs. Invalid `--lang fr` and `--max-file-size-mib 0` currently exit 1 before creating the requested output directory.

For Regression, run `cargo test -p pqc_scan_cli --test cli_information --test rule_validation --test output_safety --test file_size_limit --test test_selection --locked` with a run-owned `CARGO_TARGET_DIR`. Exercise a positive high-severity fixture with `--fail-on high` and require exit 2 with reports retained. Exercise incomplete inspection with `--require-complete` and require exit 3. Existing tests in `crates/pqc_scan_cli/src/main.rs` cover these exit distinctions.

## Gotchas

Argument errors use the product's top-level exit mapping, not clap's usual exit code. Successful scan execution does not mean no findings. Missing rules and unwritable or unsafe output paths are failures. Use existing `context_template`, `inventory_diff`, `migration_assess`, and `serve` integration tests when those separate commands change. Do not use a static-scan smoke pass as probe/network verification.
