# OSS Product Scan Targets

This list focuses on OSS products/applications (not crypto libraries), so findings are closer to real migration work.

## Verified targets (scanned with this repository's pqc-scan)

| Alias | Repository | Primary language(s) | Why useful for PQC migration test | Example findings (current run) |
|---|---|---|---|---|
| `java-realworld` | https://github.com/gothinkster/spring-boot-realworld-example-app | Java | Product-style backend with auth paths and Java crypto APIs | 18 |
| `python-product` | https://github.com/apache/airflow | Python | Large production codebase with many auth/config/dependency surfaces | 472 |
| `go-realworld` | https://github.com/gothinkster/golang-gin-realworld-example-app | Go | Product-style API service using common auth stack patterns | 16 |
| `js-realworld` | https://github.com/gothinkster/node-express-realworld-example-app | JavaScript | Node product app with JWT-oriented auth flow | 9 |
| `ts-realworld` | https://github.com/lujakob/nestjs-realworld-example-app | TypeScript | TypeScript API product with realistic auth/application modules | 15 |
| `ruby-realworld` | https://github.com/gothinkster/rails-realworld-example-app | Ruby | Rails product app with auth/dependency patterns | 2 |
| `rust-realworld` | https://github.com/launchbadge/realworld-axum-sqlx | Rust | Rust web product structure with modern dependency stack | 6 |

Counts above are from local scans executed on 2026-03-01 and can vary by upstream changes.

## Reproduce the scan

```bash
cargo build --bin pqc-scan
./examples/scripts/scan_oss_products.sh
```

Reports are generated in `/tmp/pqc-oss-reports/<alias>/report.json`.

## Notes

- Use these repositories as regression fixtures to evaluate false positives/false negatives.
- For CI stability, pin a specific commit hash in your own fixture list.
- Some targets are intentionally "realworld example apps" rather than minimal demos because they include product-like auth/config/dependency surfaces.
