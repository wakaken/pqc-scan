# Repository self-scan

**Review required: 15 results**

These results are sent to GitHub Code Scanning. This is not a vulnerability count.

No source position: 4 results. Results without a precise source position remain in the review queue.

| File | Rule | Level | Count |
| --- | --- | --- | ---: |
| Cargo.lock | DEP_PQC_CAPABLE_LIBRARY | note | 3 |
| Cargo.lock | DEP_RUST_OPENSSL | warning | 3 |
| Cargo.lock | DEP_RUST_RSA_CRATE | warning | 1 |
| crates/pqc_scan_probe_ssh/Cargo.toml | DEP_PQC_CAPABLE_LIBRARY | note | 1 |
| crates/pqc_scan_probe_tls/Cargo.toml | DEP_PQC_CAPABLE_LIBRARY | note | 2 |
| crates/pqc_scan_probe_tls/Cargo.toml | DEP_RUST_OPENSSL | warning | 1 |
| tests/quality/corpus/holdout/sbom.json | DEP_OPENSSL | warning | 1 |
| tests/quality/corpus/holdout/sbom.json | DEP_PQC_CAPABLE_LIBRARY | note | 1 |
| tests/quality/corpus/holdout/sbom.json | DEP_RUST_OPENSSL | warning | 1 |
| tests/quality/corpus/independent-holdout/Cargo.toml | DEP_PQC_CAPABLE_LIBRARY | note | 1 |

## Reviewed references: 5240 results

Exact, previously reviewed references are archived without creating Code Scanning alerts.

| Context | Count |
| --- | ---: |
| Evaluation records | 3014 |
| Examples | 322 |
| Scanner reference data and test strings | 490 |
| Tests and fixtures | 1414 |

Total: 5255 = 15 requiring review + 5240 reviewed references.

Full scan: report.sarif. Upload: code-scanning.sarif. Exact routing reasons and source hashes: code-scanning-audit.json.
New or changed results, extra duplicates and changed source files require review again.
