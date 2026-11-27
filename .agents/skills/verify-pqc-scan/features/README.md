# pqc-scan verification map

Read this index before selecting Regression or Full journeys. Each feature file records sub-features, the user entry point, a real driver, and limitations. Smoke coverage is deliberately small.

| Feature | Driver and observable proof |
| --- | --- |
| [CLI and rules](cli.md) | Real executable help/version/rule discovery and rejected inputs. |
| [Static detection and scope](detection.md) | Positive/negative files, actual locations, inspection dispositions, and privacy controls. |
| [Machine exports](exports.md) | JSON, inventory, SARIF, CBOM, CycloneDX, schemas, and identity/reference integrity. |
| [Rendered HTML reports](html-report.md) | Loopback browser, search/filter/reset, details, rule explanations, keyboard, languages, and offline layout. |
| [Corpus and OSS quality](oss-quality.md) | Existing frozen in-tree corpora and separately available full OSS coordinator with source-bound evidence. |

Smoke is automated by `../scripts/verify.py`. The other entries compose repository harnesses and browser observations; they do not claim all paths were exercised. Optional probes, `serve`, inventory `diff`, context, and migration-assessment workflows remain additional affected-feature checks through their existing CLI integration tests and user guides. Expand this map when work changes those journeys.
