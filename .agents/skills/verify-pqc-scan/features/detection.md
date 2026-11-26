# Static detection and inspection scope

## Sub-features

Rule detection and locations, benign negatives, repeated occurrences, test selection, file-size boundaries, ignored paths, symlinks, malformed/unsupported inputs, inspection completeness, and redaction.

## How to get to it (user POV)

Run `pqc-scan scan INPUT --rules-dir rules --format all --diagnostics --out-dir OUTPUT`. Use `--exclude-tests` and `--require-complete` when the affected requirement calls for them. Outputs must stay outside INPUT.

## Driving it with the CLI

Smoke creates a synthetic `app.rs` with one `RS256` reference and a benign `plain.rs`. Require `JWT_RS256` at `app.rs:1` in the positive report, zero negative findings, one scanned file in each, and unchanged input hashes. Read `diagnostics.json` for actual terminal dispositions; zero findings alone never proves complete inspection.

For Regression, use `cargo test -p pqc_scan_cli --test ast_occurrences --test example_inspection --test file_size_limit --test test_selection --test evidence_privacy --test dependency_privacy --locked`. Add the affected detector/core/rule tests. Existing `crates/pqc_scan_detectors/tests/individual_rule_quality.rs` and the frozen corpus runner cover broader rule cases. Rules must retain schema/unique-ID validation and relevant positives and negatives.

## Gotchas

The synthetic smoke does not cover certificates, keys, dependency resolution facts, all languages, or every rule. Keep failed, partial, unsupported, unknown, and complete inspection distinct. A malformed fixture's unavailable precondition is a failed journey, not a reason to delete the assertion. Never run scanned code or install its dependencies. Use synthetic privacy canaries and existing public certificate fixtures only.
