# Machine-readable exports

## Sub-features

Legacy JSON/CBOM/dependency SBOM, SARIF 2.1.0, opt-in native inventory/assessment, CycloneDX 1.7, stable finding IDs, locations, and reference integrity.

## How to get to it (user POV)

Run `scan INPUT --format all --inventory-json --cyclonedx-17 --repository-scope verification/synthetic --diagnostics --out-dir OUTPUT --rules-dir rules`. An advisory assessment additionally needs `--policy-profile migration-review-v1`; it is not implicit in inventory.

## Driving it with the CLI

Smoke checks SARIF result count and rule identity against JSON findings, native `legacy_associations[].legacy_id` against finding IDs, CBOM `RS256` at `app.rs:1`, CycloneDX version headers, and unchanged English/Japanese machine findings. Require empty negative findings and CBOM. These assertions use actual saved files, not internal setters.

For schema, negative schema controls, opt-in compatibility, identity, Unicode/unknown locations, and broader cross-output integrity, reuse `cargo test -p pqc_scan_cli --test output_contracts048 --test native_json --test sarif_locations --test cyclonedx_export --test inventory_diff --locked`. Set `PQC048_EVIDENCE` to a new run-owned path before `output_contracts048` to retain its exports. Schemas are vendored locally; prohibit network schema fetching.

## Gotchas

JSON parse success and CycloneDX headers are not schema validation. Schema validity is not accurate cryptographic interpretation. Preserve default filenames, IDs, ordering, legacy arrays, and opt-in behavior. Native inventory without policy has `not_evaluated` assessment status. `--format all` also writes a companion `report.rules.html` that must stay beside the main HTML report.
