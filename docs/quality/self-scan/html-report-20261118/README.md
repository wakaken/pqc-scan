# HTML report CI source-binding repair

The first PR #41 CI run passed Rust tests and Clippy, then failed the old Node report harness and source-bound evidence checks. The report harness now models the current DOM APIs and filter controls; it keeps its original pagination, inventory, count and deep-link assertions.

The two task058 artifact paths now point to exact archived base-commit bytes. Their original artifact hashes, measured source bindings and result claims are unchanged. Changed current sources therefore remain visibly inapplicable to those historical measurements rather than inheriting an old quality pass.

`POLICY-REBIND.json` records 203 previously approved exact reference groups in 13 changed files. Both sides were scanned with the current unchanged detector binary. Each registration is rebound only when its old complete SARIF digest/count is reproduced, its source line is byte-identical under an equal diff block, and its new rule/line/column/count is unique. No new reference is registered. 243 raw references are present on each side; 40 remain unregistered on both sides. Eight stale TLS allowances that already failed to match the before-source scan were removed conservatively; their findings are retained, not newly suppressed.

`REBIND-SOURCES.json.gz` preserves before-source bytes and both file hashes. `BEFORE-CHANGED-FILES.sarif.gz` and `AFTER-CHANGED-FILES.sarif.gz` preserve the actual scans. The policy still requires complete source and full-result hashes, exact location and count. Changed/new results continue to fail closed. A regression test checks the unchanged source lines and both actual result digests.

The older 487 = 477 reviewed + 10 retained product measurement is tested against `HISTORICAL-PRODUCT-EXPECTATIONS.json` and the exact 27 corresponding historical sources in `HISTORICAL-PRODUCT-SOURCES.json.gz`. It no longer asserts that old SARIF locations describe the current implementation. The current policy is separately checked against every current source hash, and the current product scan is verified without changing omission rules or thresholds.

The existing coverage-contract documents remain byte-identical to their accepted base. Their unrelated wording edits were reverted: detector support and PKCS12 scope did not change, and the human-only cutoff wording is documented in REPORTS.md/REPORTS.ja.md. The existing source-bound applicability assessment therefore remains valid without rewriting it. CI now runs tests/report-ui alongside the legacy shell tests.
