# Phase 0/1 implementation alignment

Checked on 2026-11-10 against runtime HEAD `1f61dfa6b3fa6050c4ec4af5f2efe9883cfbe242` on `codex/scanner-quality-improvements`. This is a documentation and implementation-record audit, not a new product-wide quality certification. Task YAML is authoritative. The documentation refresh does not change code, rules, schemas, task states or numeric budgets.

## Task coverage

| Task | Phase | YAML state | Implemented scope and primary acceptance record | Implementation commit |
|---|---|---|---|---|
| [018](../../tasks/018-inspection-diagnostics.yaml) | 0 | done | [Detailed outcomes, safe diagnostics, coverage and opt-in exit 3](../roadmap/TASK-018-IMPLEMENTATION.md) | `6e8c0e2` |
| [019](../../tasks/019-bounded-detector-outcomes.yaml) | 0 | done | [Bounded regex/AST/key inspection; accepted-hit versus work limits](../roadmap/TASK-019-IMPLEMENTATION.md) | `c94ca95` |
| [020](../../tasks/020-certificate-attribute-parsing.yaml) | 0 | done | [Parsed certificate/SPKI attributes, sizes, public identities and bundle positions](../roadmap/TASK-020-IMPLEMENTATION.md) | `87b09cf` |
| [021](../../tasks/021-certificate-classification-policy.yaml) | 0 | done | [Exact OID/subject/time predicates and reviewed classification corrections](CERTIFICATE-BASELINE-021.md) | `3fe0d78` |
| [022](../../tasks/022-sarif-location-contract.yaml) | 0 | done | [SARIF URI/unknown-region/Unicode projection and offline schema checks](SARIF-LOCATION-CONTRACT-022.md) | `35d988e` |
| [023](../../tasks/023-quality-baseline.yaml) | 0 | review | [Frozen labeled corpus, evaluator, repeatability and performance measurements](TASK-023-BASELINE.md); maintainer protocol/budget approval remains open | `757d067` |
| [024](../../tasks/024-ast-occurrence-fidelity.yaml) | 0 | done | [Distinct same-line occurrences and measured grammar limits](TASK-024-AST-OCCURRENCES.md) | `8ba863f` |
| [025](../../tasks/025-observation-inventory-native.yaml) | 1 | done | [Observations, normalized inventory and legacy associations](TASK-025-NATIVE-SCAN.md); [rename](TASK-025-INVENTORY-NAMING.md) and [old switch removal](TASK-025-INVENTORY-ONLY.md) | `7e4d0ce`, `9a3934c`, `065646e` |
| [026](../../tasks/026-assessment-local-policy.yaml) | 1 | done | [Separate assessment axes and explicit offline advisory policy](TASK-026-LOCAL-ASSESSMENT.md) | `2baac32` |
| [027](../../tasks/027-dependency-version-provenance.yaml) | 1 | done | [Declared/resolved provenance, safe purls and typed parser outcomes](TASK-027-DEPENDENCY-PROVENANCE.md) | `aac154f` |
| [028](../../tasks/028-rule-validation-provenance.yaml) | 0 | done | [Rule loading validation and all-119-rule quality provenance](TASK-028-RULE-QUALITY.md) | `4324d2c` |
| [029](../../tasks/029-evidence-privacy-contract.yaml) | 0 | done | [Known-syntax evidence redaction, synthetic canaries and stable identity linkage](TASK-029-EVIDENCE-PRIVACY.md) | `1f61dfa` |

Phase 0 comprises nine tasks: eight done and one review. Phase 1 comprises three tasks, all done. Phase numbering describes responsibility, not a requirement to implement tasks in numeric order: 028/029 remain Phase 0 even though 025–027 were implemented first. Tasks 030–039 remain pending. No TLS/SSH probe, standards export, diff engine or 1.0 release gate is completed by these records.

## Dependencies and remaining decisions

Task 023 has a usable reviewed technical baseline: 41 files, 57 truth units, independent labels, tuning/holdout lineage, repeatability and measured resource samples. Its queue record explicitly retains `review` until maintainers approve the measurement protocol and measured budget proposal. Later explicit user requests consumed this baseline and recorded their own before/after evidence; they did not approve the numeric budgets or silently close 023. Dependencies on that baseline are satisfied for the documented technical comparisons, while the remaining policy decision stays visible. This audit does not authorize future work to bypass its own decision gates.

Task 021 used its reviewed certificate-only baseline before task 023 existed, as already specified in the roadmap. Phase 1 starts through task prerequisites rather than a blanket declaration that Phase 0 is complete. Local acceptance is distinct from hosted CI and cross-platform/release acceptance; historical records retain their executed/unexecuted distinctions.

The default one-task/one-PR policy was overridden by subsequent explicit same-branch and accumulated-publication instructions: tasks 022–027 and CI follow-ups were merged in PR #14 (`b213300`). Task records and independent validations remain separate. Tasks 028/029 were based on that develop checkpoint and are on the current feature branch. Pushing that branch does not merge it into develop.

## Documentation corrections

- Updated roadmap 028's scope from the original 117-rule design baseline to the actual 119 reviewed rules; the two additions are current-validity certificate rules. The historical gap analysis retains its baseline numbers and now links to this current checkpoint.
- Updated roadmap/current architecture output naming to `--inventory-json` / `scan.inventory.v1.json`. Internal native types, schema paths, historical task titles and past filenames retain their original meaning; the removed CLI switch is not an alias.
- Both READMEs distinguish the six legacy reports from three explicit opt-ins, inspection completeness from security assessment, and supported reading of SBOMs from future CycloneDX export.
- Replaced unconditional private-material export claims with task 029's bounded contract. Certificate/dependency corrections and occurrence retention preserve structures and identity algorithms, while intentionally corrected facts can change findings/IDs; this is not byte-for-byte compatibility with erroneous values.
- Japanese prose was edited using the requested humanizer-jp skill. Commands, identifiers, numbers and link targets were checked against the English text and local implementation; no runtime translation dependency was added.

## Verification for this refresh

See [the refresh validation and independent review](README-REFRESH-VALIDATION.md) for executed commands and results. Product source hashes are checked against task 029's accepted source bindings, with CLI/help/examples and documentation links checked on the current checkout. Existing task acceptance records remain historical evidence of their particular source states; this refresh does not relabel them as new hosted CI or broader accuracy evidence.
