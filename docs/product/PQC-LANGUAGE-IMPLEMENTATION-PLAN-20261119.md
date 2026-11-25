# PHP, C#, C and C++ Implementation Plan

> Execution ownership: the main agent implements, investigates and verifies the combined change under AGENTS.md. If implementation is later authorized, use the inline `superpowers:executing-plans` workflow; only the two required review stages may use reviewers. This document does not authorize implementation, create task records, select a release, or allocate additional agents.

**Goal:** Extend the existing offline Tree-sitter, bounded local-name/binding and dependency-input analysis to PHP, C#, C and C++, preserving existing behavior and connecting new observations to v0.4 inventory and migration assessment.

**Architecture:** Reuse the current detector pipeline, budgets, typed observation capture, inventory, saved-evidence reader and migration evaluator. Add language/library adapters and bounded dependency parsers; do not execute projects or introduce a second assessment engine.

**Tech stack:** Rust 1.93.1 from `rust-toolchain.toml`, existing Tree-sitter 0.26.6, YAML rules, bounded JSON/XML/text parsing, current inventory/context/assessment v1 contracts.

**Research:** [2026-11-18 ecosystem inventory](PQC-LANGUAGE-EXPANSION-20261118.md). That document remains a dated research snapshot. This plan's matrices define the proposed initial implementation scope, not every surveyed library's full API surface. Acceptance below is proposed for maintainer agreement; no product capability or passing test is claimed by writing it.

## 1. Source integration and v0.4 boundary

On 2026-11-19, `git fetch origin develop codex/v04-roadmap-20261118` succeeded. The local and remote v0.4 branch tips both identified `c1f8a4bc5478eefb2b1babb6b2aa86295f9c8d34`. `git merge --ff-only codex/v04-roadmap-20261118` succeeded on `codex/pqc-language-research`, advancing it from `3f2bc6e6209e1f2813caac8077c8d7a7198832cc` to that exact tip. There was no conflict, merge resolution, cherry-pick or newly authored product-code edit. The earlier untracked research document was retained.

The imported range includes the intervening v0.3 report/local-web changes and v0.4 commits `c7e76b5` and `c1f8a4b`. It is a development implementation snapshot, not proof of v0.4 release completion: committed task records mark 065-070 done, 071 blocked and 072-075 pending; the workspace package version remains 0.3.0. Historical results retain their original scope and do not certify the proposed language extension.

The v0.4 worktree also has **uncommitted** context generation, CLI template, local-web reassessment, evaluation and documentation work. These files were inspected read-only and **were not imported**. Their observed task states and amended evaluation scope differ from committed records. They are integration watchpoints, not adopted requirements or evidence that the committed baseline includes those features. A path/hash manifest is preserved in [the WIP snapshot](PQC-LANGUAGE-V04-WIP-20261119.json); it identifies inspected files without copying their source or changing their worktree. Reconcile the later committed tip before language implementation, without silently adopting future requirement changes.

## 2. Global constraints

- Scope is PHP, C#, C and C++; Kotlin/Swift, new probes, new web applications and new cryptographic implementations are excluded. Seven existing source languages plus these four produce eleven language entries.
- Scan/reassessment remains offline. No PHP, Composer, dotnet/MSBuild/NuGet, compiler, preprocessor, CMake/Conan/vcpkg, FFI payload or scanned build script is executed. No registry/schema/provider fetch occurs during product scanning.
- Preserve existing CLI options, output filenames, serialized field shapes/versions, Risk vocabulary, identity algorithms and public struct-literal compatibility. Additive internal facts/API methods must not require new public struct fields.
- Existing seven-language results and existing saved-input judgments remain unchanged for unchanged inputs, except an explicitly documented and approved semantic correction. Additive findings on newly covered code are expected and can affect fail thresholds; document this rather than claiming every old repository exit code stays identical.
- AST/binding results remain syntactic references: no active use, successful execution, installed native backend, negotiated TLS, verified trust or migration-ready claim follows. Dependencies satisfy no readiness prerequisite.
- Preserve `DetectorBudget`, safe snippets/redaction, strict diagnostic behavior and bounded repository reads. Exhaustion remains Truncated; malformed syntax remains Partial; unsupported/dynamic constructs never become falsely Complete.
- No storage/schema change is required by this plan. If a required fact cannot be represented without changing a published contract, stop affected implementation for the human review required by AGENTS.md; do not add a field or reinterpret a frozen policy under the same version.
- No new numerical risk score, arbitrary Confidence probability, default policy branch, deadline or capability claim is introduced. Use the existing policy and unknown states.
- Keep dated research, audits, tasks and review evidence historical. Register no new task IDs or v0.4 roadmap items through this plan.

## 3. v0.4 impact assessment

| v0.4 surface | Observed implementation / source | Required language-extension response |
|---|---|---|
| All-rule audit (065) | Source-bound rule population, classifications and quality companion; [rule-quality guide](../quality/rules/RULE-QUALITY.md) | Every added rule needs rationale, limits, positive/negative controls and current companion/source hashes. Do not edit the historical 065 audit to imply it reviewed new rules. |
| Technical facts (067) | [technical.rs](../../crates/pqc_scan_core/src/migration/technical.rs) reuses the primitive evaluator and adds separately versioned facts | Feed exact recognized identifiers/parameters. Reuse existing meanings; unlisted constructions remain unknown. New catalog meaning requires a separately justified/versioned decision, not a name-matching shortcut. |
| Typed capture and normalization | [observation.rs](../../crates/pqc_scan_core/src/observation.rs) tokenizes text into a finite `AlgorithmSpec::recognized` vocabulary | A detected `MLKem768`, PHP function suffix or `EVP_PKEY` call does not necessarily create an algorithm asset today. Add a safe typed source-fact path for new handlers; do not infer semantics from arbitrary legacy metadata. |
| Native saved-evidence adapter | [migration/native.rs](../../crates/pqc_scan_core/src/migration/native.rs) follows explicit algorithm/public-key/certificate roles, otherwise emits reference-only targets | End-to-end tests must prove new exact primitive facts reach the existing adapter. Preserve dependencies/reference-only targets and ambiguous identity instead of manufacturing algorithm relations. |
| Context binding (068) | [scoped-context contract](../architecture/SCOPED-MIGRATION-CONTEXT.md): exact bytes/digest, target IDs, ownership and claim provenance | Rescanning with new language support creates a new input binding. Old context must fail against different input; no positional, path-only or automatic cross-input rebinding. |
| Decisions (069) | [decision contract](../architecture/OFFLINE-MIGRATION-DECISIONS.md): independent urgency/information/readiness/actions | New languages do not add policy branches. Unknown parameters, backend or use remain unknown; a source PQC call cannot establish readiness. |
| Human reports (070) | JSON, bilingual Markdown and HTML from structured migration results | New source/library labels and locations must render coherently in both languages without renderer-side reassessment. Reuse safe output and browser acceptance. |
| Local scan UI / CLI | Imported `serve` scans through the shared pipeline; saved `assess` is opt-in | Verify direct CLI and local-web scan parity for the new languages; saved assessment must not rescan or fetch providers. No new UI is needed for basic language support. |
| Uncommitted 073/074 | `generate_context`, `Target::is_bindable`, `input_support_fields`, `context-template`, `serve_migration` | After these land, test generic target selection, claim round trip, purpose-relevant prerequisites, replacement invalidation and CLI/UI parity. Do not copy or reimplement those helpers on this branch. |
| Uncommitted 071/075 amendment | Evaluation scope changes to limited synthetic technical correctness/conformance/reproducibility | Do not infer human usefulness or scoring validity. Treat frozen measurements as population/source-bound; add language-specific validation without rewriting historical populations or importing an uncommitted acceptance amendment. |

**Conclusion:** v0.4 does not require a different scanning engine. It increases the integration obligation: detection must produce appropriate typed inventory facts and preserve uncertainty through assessment, context and reports. The main shared-file risks are `observation.rs`, `recommendation.rs`, dependency identity validation, rule-quality metadata and integration tests; pending context/UI helpers are consumed only once committed.

## 4. Initial support contract

All rows below are required for the proposed initial release. A row labelled reference-only is deliberately not a promise of arbitrary dataflow or runtime capability. Broader library coverage in the research remains a proposal for later authorization, not work silently transferred from an already agreed implementation criterion.

| Family | Required initial source/API coverage | Dependency inputs |
|---|---|---|
| PHP | OpenSSL explicit RSA/EC key creation and public-key operations; DH/ECDH derive; Sodium Ed25519/X25519/box/kx and ML-KEM-768/X-Wing KEM calls; ParagonIE ext-pqcrypto and Compat ML-KEM/ML-DSA/X-Wing generation and bounded receiver operations; ext-pqcrypto SLH-DSA qualified references; phpseclib RSA/EC/DH qualified references | Composer manifests/locks and explicit PHP/extension requirements |
| C# | Standard RSA/DSA/ECDsa/ECDiffieHellman plus concrete provider types; .NET MLKem/MLDsa/SlhDsa generation/import and bounded receiver operations; BC classical and ML-KEM/ML-DSA/SLH-DSA types/parameters and operations; explicit CompositeMLKem/CompositeMLDsa/HPKE references retained as construction-unresolved | NuGet declarations/lock records, project/SDK context and bounded literal central-package context |
| C | OpenSSL classical key APIs, explicit EVP algorithm selection, KEM/signature operations and provider/TLS-group configuration; liboqs constructor/parameter/operation families | vcpkg/Conan structured subsets and literal build declarations |
| C++ | The same C APIs in C++ plus official liboqs-cpp qualified KEM/signature construction and receiver operations | The same C/C++ dependency subsets |

Third-party OQS PHP/C# wrappers and deprecated official .NET wrapper are required dependency/reference observations, with distinct project identities and maintenance guidance. No generic “liboqs-dotnet” substring may conflate official and OpenForge packages. Botan, wolfCrypt, AWS-LC, BoringSSL, LibreSSL, GnuTLS, PSA, PQClean/native packages and commercial SDKs are not promised full initial API analysis; when present through supported dependency/SBOM inputs, preserve safe package inventory without claiming activation or universal support.

### 4.1 Syntax, names and incomplete code

- PHP: `.php`/`.phtml` use the PHP grammar with mixed HTML; `.inc` is eligible only with an actual PHP opening tag. `.blade.php` is parsed as static PHP with partial/template limits, never rendered; standalone Twig is outside source support.
- C#: `.cs` only. Razor, assemblies, source generators and project-wide semantic resolution are excluded. Support explicit qualified and file-local `using`/alias/global-using declarations; a global alias in another file is unresolved. Handle ordinary and file-scoped namespaces.
- C: lowercase `.c`; C++: `.cc`, `.cpp`, `.cxx`, `.hpp`, `.hh`, `.hxx`, and uppercase `.C` checked **before** extension lowercasing. Ambiguous `.h` runs bounded C/C++ candidate parsing under one combined budget; agree on identical facts once, and retain ambiguity/partial inspection where resolution differs. No configured build language is inferred from filename alone.
- Each language resolves only literal API names, literal parameters and local imports/aliases/bindings with existing depth/budget bounds. Preserve source order, scope and reassignment; stop uncertain receiver/alias chains rather than choosing the first matching declaration.
- C/C++ includes and C# namespace imports are references, not proof of a linked implementation. Do not execute/evaluate conditional compilation. Conditional references cannot establish enabled functionality; missing branch semantics is explicit uncertainty.

### 4.2 Dependency subset and provenance

Reuse [the existing six-field provenance contract](../architecture/DEPENDENCY-PROVENANCE.md), including source-local assertions and no manifest-lock graph joining.

| Input | Initial static subset and required result |
|---|---|
| `composer.json` | Literal `require`/`require-dev`, PHP/extension platform requirements and `config.platform`; declarations/emulation only, not installed versions. Preserve vendor/package identity with a narrowly validated Composer namespace. |
| `composer.lock` | Literal `packages`/`packages-dev` name/version/source records and platform requirements; concrete registry records may resolve, Git/path/custom source remains unresolved. Never copy source URLs/credentials into output. |
| `.csproj`, `Directory.Packages.props`, `Directory.Build.props`, `packages.config`, `global.json` | Literal PackageReference/PackageVersion and legacy package entries, target frameworks and SDK declarations; nearest selected in-root central context only. Properties/conditions/imports outside the explicit literal subset remain conditional or unresolved; no arbitrary MSBuild evaluation. |
| `packages.lock.json` | Supported version 1/2 records with target-framework buckets and explicit requested/resolved values; declared vs locked facts remain separate. NuGet ID comparison is case-insensitive without merging distinct source assertions. Unsupported record/schema versions are incomplete/unsupported. |
| `vcpkg.json`, `vcpkg-configuration.json` | Literal dependencies/features, version constraints, overrides/baseline metadata; declarations only. A baseline or version constraint alone does not resolve installed ports. |
| `conanfile.txt`, `conan.lock` | Static requires/tool-requires and lock schema 0.5 literal reference records; bounded concrete package versions, preserving revision and source uncertainty without inventing deployment/graph facts. Unknown schemas stay unsupported. `conanfile.py` is not executed. |
| `CMakeLists.txt`, `*.cmake` | Literal `find_package`/link/FetchContent declarations are optional-context references with lexical/incomplete status. No variable/script evaluation, download or package-resolution claim. |

Confirm the named format editions against pinned official examples before implementing each parser; a differing edition is an explicit unsupported case until specified, not a permissive fallback. Keep the current shared byte/depth/value/record caps; new parsers must not enlarge them. Add Composer/NuGet identity and purl handling only for registry semantics that can be represented correctly; otherwise retain dependency facts with no fabricated purl. CMake/Conan/vcpkg names are not automatically registry-resolved or mapped to a made-up purl type.

Format references checked for this plan: [Composer schema](https://getcomposer.org/doc/04-schema.md), [NuGet PackageReference/locking](https://learn.microsoft.com/en-us/nuget/consume-packages/package-references-in-project-files), [vcpkg manifest reference](https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json), [Conan lockfile 0.5 example](https://docs.conan.io/2/tutorial/versioning/lockfiles.html). These specifications inform bounded subsets; supporting the subset is not full package-manager equivalence.

## 5. File and interface map

Paths marked new are proposed implementation files, not files created by this planning request. Retain the current layout and public entrypoints; do not restructure unrelated modules.

| Responsibility | Existing integration files | New focused files/tests |
|---|---|---|
| Grammars/routing | `Cargo.toml`, `Cargo.lock`, `crates/pqc_scan_detectors/Cargo.toml`, `src/tree_sitter_detector.rs`, `src/template_source.rs` where needed | detector `tests/language_routing.rs` |
| Source facts | detector `src/source_facts.rs`, `src/source_facts/bindings.rs`, `src/source_facts/context.rs`; existing `tree_sitter_detector/source_api` adapter | detector `src/source_facts/php.rs`, `csharp.rs`, `c_family.rs`; `tests/php_source.rs`, `csharp_source.rs`, `c_family_source.rs` |
| Safe normalized capture | core `src/observation.rs`; existing `inventory.rs` and `migration/native.rs` consumers | core `tests/language_source_inventory.rs`; detector capture tests alongside language tests |
| Dependencies | detector `src/dependency_detector.rs`, `src/dependency_detector/parse.rs`; core `observation.rs` identity validation | detector `src/dependency_detector/composer.rs`, `nuget.rs`, `c_family.rs`; `tests/language_dependencies.rs` |
| Rules and audit | `rules/default/`, `docs/rules-reference.md`, `docs/quality/rules/RULE-QUALITY-METADATA.json`, applicable `tests/quality/judgment/` fixtures | proposed `rules/default/php.yml`, `csharp.yml`, `c_family.yml`; targeted tests and current companion rows |
| Guidance/assessment/report | core `recommendation.rs`; existing migration and report modules | CLI `tests/language_expansion.rs`, core `tests/language_migration.rs`, report `tests/language_migration.rs` |
| Bilingual scope/contracts | root README pair; `docs/user/COVERAGE` and relevant SCANNING/REPORTS pairs; affected architecture/dependency/rule docs | synthetic fixture corpus under `tests/fixtures/language-expansion/` |

Proposed internal language handler contract: keep the existing `inspect(root: Node<'_>, text: &str, language: &str, rules: &RuleSet, budget: DetectorBudget) -> Facts` dispatcher and `Facts::rule_matches`/`status`. New module entrypoints accept `&mut Context<'_, '_>`, `&mut Facts` and `&RuleSet`; extend the shared Context only where the new grammar needs explicit scope/write/import semantics, with old-language regression tests. Do not treat the existing JS/Python/Rust scope list as automatically correct for PHP/C# or C++.

Proposed additive typed capture API in core: `AlgorithmSpec::source_api(identifier: &str, parameters: &BTreeMap<String, String>) -> Option<AlgorithmSpec>` and `ObservationDraft::source_with_algorithms(d: &Detection, method: ObservationMethod, span: Option<(usize, usize)>, algorithms: &[AlgorithmSpec]) -> ObservationDraft`. These accept only finite identifiers/parameter keys and bounded values; unknown identifiers are omitted with fixed uncertainty. Existing `ObservationDraft::source` and `AlgorithmSpec::recognized` retain old behavior. New handlers pass typed facts captured from parsed calls/receivers; they do not pass raw secret-bearing arguments or arbitrary metadata. Source spans continue to describe original code, not synthesized canonical text.

Where private SourceMatch/Facts needs to carry these facts, add a private bounded vector of safe `AlgorithmSpec` values; do not expose new mandatory fields on `Detection`, `Finding`, `ScanConfig` or `RuntimeProfile`. Supplement runtime guidance from existing dependency/context mechanisms or a private side context. Do not infer runtime readiness from that side context.

Exact ML-KEM/ML-DSA/SLH-DSA parameter names enter the existing catalog. RSA/EC purposes remain unknown when the current saved-fact contract cannot express their operation reliably; do not convert RSA encryption into signing to improve the report. X-Wing, composite and unlisted experimental constructions retain exact inventory identity but unknown technical judgment where the finite catalog lacks them. Do not identify X-Wing as `X25519MLKEM768`: sharing components does not make protocols/combiners identical. A broader catalog/role extension requires a separately agreed, versioned specification.

## 6. Implementation sequence

These are steps within one combined feature, not new registered tasks or reviewer allocations. The order is a dependency order: shared capture/routing → per-language adapters → dependency context → integration. Each implementation step is followed by relevant targeted tests; commits may follow coherent passing increments, with one combined implementation review and one test review.

### S1. Freeze fixtures, grammar compatibility and baselines

- [ ] Freeze a source/library/API manifest for every required row in section 4, with official source commit/tag, exact symbol/parameter mapping, limits and expected observation strength. API examples are never copied as real secret/key fixtures.
- [ ] Start with exact grammar candidates verified from crates.io on 2026-11-19: `tree-sitter-php =0.25.1`, `tree-sitter-c-sharp =0.24.0`, `tree-sitter-c =0.24.2`, `tree-sitter-cpp =0.23.4`. These are candidates, **not build-tested here**. Verify Rust 1.93.1/Tree-sitter 0.26.6 ABI, syntax and supported platforms before adopting the pins. A failed candidate requires a recorded alternative pin and the same checks, not a toolchain upgrade by default.
- [ ] Capture old-seven-language inventory/report/assessment goldens, rule population, feature configurations and scan corpus runtime/output sizes before changes. Expected outputs independently encode intended behavior and uncertainty, not output copied from the implementation under test.

### S2. Add routing and safe typed source capture

- [ ] Write failing `language_routing` tests for four grammars, `.C` vs `.c`, ambiguous `.h`, PHP mixed text, modern C#, malformed syntax, encoded Unicode, exact spans and budget exhaustion.
- [ ] Add minimal pinned dependencies/routing/semantic nodes and safe typed capture interfaces above. Add positive/negative allowlist, unknown identifier, contradictory parameter and secret-canary tests before implementing them.
- [ ] Run detector routing and core inventory tests. Require one lexical occurrence per rule/span, preserved old vocabulary/identity, explicit incomplete status and no secret/raw-argument retention.

### S3. Implement PHP coverage

- [ ] Write `php_source` fixtures covering all required PHP API families, aliases, namespace fallback, function/class shadowing, reassignment, optional dynamic calls and imported keys whose type is unknown.
- [ ] Add PHP local scopes/imports/literal receiver handling and scoped rules. An OpenSSL sign/verify operation alone must not assert RSA. Detect the two Sodium KEM families separately; preserve qualified ParagonIE identities and version prerequisites.
- [ ] Run PHP detector/inventory tests, positive and negative per API family; require correct original span, expected canonical identifier or explicit unknown, and no standalone AES/password-hash quantum-public-key finding.

### S4. Implement C# coverage

- [ ] Write `csharp_source` fixtures for standard/BC types, qualified aliases, local namesakes, namespace/global-using limits, rebound receivers, literal parameter selections, composite/HPKE ambiguity and conditional references.
- [ ] Add C# scope/import/type/receiver subsets and rules. Do not require PQC classes to derive from AsymmetricAlgorithm. Keep .NET standard, BC TLS and OQS wrapper observations separate.
- [ ] Run C# detector/inventory tests; ensure type-only imports and bare dependency names never become successful operations or readiness claims.

### S5. Implement C/C++ coverage

- [ ] Write `c_family_source` fixtures for OpenSSL low-level/EVP calls, literal algorithm selection, provider properties, TLS group lists, liboqs C and official C++ wrapper operations.
- [ ] Add C/C++ lexical scope/literal/receiver subsets and rules. Bind generic EVP operations only when the local key/context selection is unambiguous; preserve dynamic selections and conditionals as unresolved. Cross-file pointers, macros, include resolution and templates do not receive invented bindings.
- [ ] Run C/C++ detector/inventory tests; keep include-only/reference and actual API request distinct, preserve draft variants and duplicate same-line operations, and exercise hostile/deep headers under combined budgets.

### S6. Add dependency parsing and language guidance

- [ ] Write `language_dependencies` tests for every table row, including valid empty input, duplicate keys, malformed sibling records, unsupported versions, platform emulation, registry/Git/path identity, conditional/multitarget declarations, malicious XML and out-of-root context reads.
- [ ] Implement the exact static subsets with existing parser limits, statuses and six-field provenance. Add narrow safe Composer/NuGet identity/purl support; do not join manifests/locks or execute managers.
- [ ] Add language-specific migration guidance using explicit .NET/native backend, PHP extension and OpenSSL/provider prerequisites. Unknown versions stay unknown. Recommend separately assessed replacement roles, not a blind RSA→ML-KEM substitution or universal OQS deployment.
- [ ] For every new rule, record the proposed base severity, heuristic confidence and promotion behavior separately with comparable existing-rule rationale and empirical calibration marked unknown where unmeasured. Review these values against positive/negative detector evidence; do not copy a package's PQC marketing claim into a favorable risk label.
- [ ] Run dependency and recommendation tests; verify both default and custom rule packs and the updated source-bound rule-quality companion. Retain historical audit records untouched.

### S7. Verify v0.4 integration and publishable scope

- [ ] Add CLI/core/report tests below for static scan → inventory → saved evidence → context → migration JSON/Markdown/HTML, preserving references and independent policy axes. Do not change schemas to make new fixtures pass.
- [ ] Rebase/merge only later authorized committed v0.4 changes, retaining this plan's source record; inspect changed shared files and add the conditional 073/074 integration checks if those APIs land. Do not close a relevant acceptance row using this earlier WIP inspection.
- [ ] Update paired user documentation and current rule reference/quality companion. Show precise initial library support, known gaps, parameter ambiguity, new finding/exit impacts and static-only evidence.
- [ ] Run required checks, inspect the combined source/rule/docs diff, obtain implementation review, fix findings, complete final tests and obtain test review under the same two reviewer identities. Follow browser rules for new labels/report acceptance.

Grammar sources: [PHP](https://crates.io/crates/tree-sitter-php), [C#](https://crates.io/crates/tree-sitter-c-sharp), [C](https://crates.io/crates/tree-sitter-c), [C++](https://crates.io/crates/tree-sitter-cpp). Candidate availability was checked through the registry API; no crates were installed by this request.

## 7. Acceptance matrix

All IDs below are required for the proposed bounded scope. One failed, unknown or unexecuted required row prevents implementation completion. Expected uncertainty is a passing result only when the row explicitly requires it; missing evidence is not equivalent to an expected unknown. Each corpus row records language/library, source version, API/parameter/operation, expected rule IDs/span/count, typed inventory identity or explicit unknown, method/assurance/status, and downstream expectation.

| ID | Observable acceptance condition | Owning step / evidence |
|---|---|---|
| L01 | All four grammars build with the fixed toolchain/runtime and parse required syntax; unsupported syntax retains findings with Partial | S1/S2; grammar pins, build and syntax corpus |
| L02 | `.c`/`.C` and mixed PHP route correctly; ambiguous `.h` cannot combine incompatible bindings or duplicate one occurrence | S2/S5; routing fixtures |
| L03 | Each required PHP, C# and C/C++ API family has actual detector-level positive cases and negative namesake/comment/help-string controls | S3-S5; per-family matrix with zero unmet expectations |
| L04 | Aliases, qualified names, local scope and reassignment resolve only within the stated subset; unresolved/dynamic/cross-file cases never fabricate a binding | S3-S5; positive/shadowing/rebound fixtures |
| L05 | KEM/signature/encryption/key-agreement roles remain distinct; generic sign/EVP/import operations with unknown key type do not become RSA | S3-S5; role and unknown tests |
| L06 | Standard ML names/parameter sets remain separate from Kyber/Dilithium drafts and from incomplete parameter names | S2-S5; exact/missing/contradictory parameter cases |
| L07 | X-Wing, TLS hybrid and composite/HPKE references are not equated; unsupported technical construction remains unknown with source traceability | S2-S7; inventory/assessment negative controls |
| L08 | Original spans, Unicode locations, two calls on one line and parent/child AST candidates produce the expected count and stable identities | S2-S5; span/dedup assertions |
| L09 | Budget exhaustion, malformed syntax and unsupported inputs retain applicable safe observations with correct incomplete status; strict-mode behavior agrees | S2-S6; below/at/above limits and CLI failures |
| L10 | Every specified dependency input handles valid empty data, supported schema records and explicit incomplete/unsupported controls without becoming a false empty success | S6; parser matrix |
| L11 | Declared, locked, emulated platform, conditional, Git/path and SDK facts remain distinct; only eligible registry locks resolve; no invented purl/installed graph | S6; six-field and export assertions |
| L12 | Nearest selected in-root literal central-package context works; excluded/symlink/out-of-root/dynamic imports are not followed or evaluated | S6; context-boundary tests |
| L13 | Every new rule ID is unique, schema-valid, language/library-scoped, documented and represented once in current source-bound quality metadata | S3-S6; loader/companion/ID population checks |
| L14 | Modified custom ID/kind/pattern/scope retains generic matching; new built-in binding logic attaches only to exact built-in semantics | S3-S6; registry selector tests |
| L15 | Existing OpenSSL lexical findings/IDs are preserved unless an explicit approved correction says otherwise; overlapping new AST findings have a documented disposition | S5/S7; old-rule regression and coexistence cases |
| L16 | New precise API facts become safe typed algorithm assets in inventory; method stays ast_pattern, assurance syntactic, usage referenced | S2-S7; no arbitrary metadata promotion |
| L17 | New dependencies, headers, imports, provider load calls and incomplete API facts stay references where algorithm identity/activation is unresolved | S3-S7; negative downstream cases |
| L18 | Exact ML-KEM/ML-DSA/SLH-DSA identities/parameters reach SavedEvidence and existing technical judgments; legacy saved inputs/catalog/policy judgments remain unchanged | S7; new end-to-end and old goldens |
| L19 | Rescanned/new input bytes invalidate old context bindings; target references/digests/ownership are exact and no automatic cross-input remapping occurs | S7; stale-context rejection |
| L20 | Sparse claims and unknown native support never become ready; false prerequisites remain blocked even with unknowns; source references cannot establish negotiated/active use | S7; decision semantics fixtures |
| L21 | Scoped claims for one PHP/.NET/native target do not leak to another target/system/role; expiry and purpose conflicts retain existing semantics | S7; mixed-system/purpose fixtures |
| L22 | Bilingual migration JSON/Markdown/HTML preserve the same decision/evidence and useful location/guidance; writers do not evaluate policy | S7; structured/rendered comparisons and browser observations |
| L23 | CLI/local-web scans use the same new language coverage; saved assess does not rescan or start network/probe activity | S7; CLI/serve integration and offline controls |
| L24 | If committed 073/074 are imported, their template/target/claim round trip, relevant input support fields and replacement invalidation work for the new targets | S7; mandatory on that final source; otherwise record baseline exclusion explicitly |
| L25 | Secret canaries in keys/credentials/manifests/rejected metadata never reach legacy reports, inventory, assessment, diagnostics, IDs or UI | S2-S7; existing privacy policy plus canaries |
| L26 | Old seven-language frozen results, public API use, custom packs, CLI/report defaults and schema contracts pass the source-bound regression suite | S1/S7; compatibility evidence |
| L27 | New finding/threshold impacts and inspected subsets are consistent in English/Japanese user docs, reports and library-specific guidance | S7; paired documentation and representative CLI/browser checks |
| L28 | All required Rust/rule/package/feature checks and both independent reviews pass on final source; remaining gaps stay unfinished | S7; commands/exits/logs/reviewer continuity |
| L29 | Measure new corpus runtime, peak resource observations and output size under fixed conditions; enforce finite parser budgets and disclose changes without claiming calibrated population accuracy | S1/S7; before/after source-bound measurement |
| L30 | Corpus expectations are frozen before implementation observations; regex-target fixtures alone do not count as source accuracy, and no synthetic result is called human usefulness | S1/S7; corpus provenance and declared denominators |

Review focus for the five most likely integration failures: **F1** API names matching unrelated symbols (L03/L04); **F2** typed algorithm identity lost between detection and assessment (L16/L18); **F3** native backend/dependency mistaken for readiness (L11/L20); **F4** new target IDs incorrectly reusing old context (L19/L21); **F5** macros, `.h`, templates and malformed inputs falsely classified complete (L02/L09). Each is assigned to concrete tests above.

## 8. Verification commands and evidence

At implementation time, use `rust-toolchain.toml`. Intended targeted commands include `cargo test -p pqc_scan_detectors --test language_routing`, `--test php_source`, `--test csharp_source`, `--test c_family_source` and `--test language_dependencies`; core `cargo test -p pqc_scan_core --test language_source_inventory` / `--test language_migration`; CLI `cargo test -p pqc_scan_cli --test language_expansion`; report `cargo test -p pqc_scan_report --test language_migration`; and `cargo test -p pqc_scan_rules`. These are proposed new test targets, not commands claimed to run today.

Required final checks: `cargo fmt --all` followed by diff inspection; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo build --workspace`; relevant package/license/dependency provenance checks for the four grammar additions. Exercise `probe-tls`, `probe-ssh` and `probe-ssh-kex` with the applicable existing protocol acceptance instructions where changed shared core/dependencies affect them, using separate target directories. Preserve platform/distribution checks from current packaging contracts; default workspace success is not evidence for disabled features. New rules must pass unique-ID/schema, source-bound metadata and positive/negative detector checks.

For representative end-to-end fixtures, explicitly request inventory, inspect the new algorithms/unknown references and safely saved dependency facts, then run the existing `assess` workflow against the saved file with an explicit fixed clock/context. Preserve input hashes, target IDs and catalog/policy versions. Verify rejection of stale context against a changed input. HTML/browser checks use a dedicated generated-report directory served on 127.0.0.1, with English/Japanese views, keyboard/offline operation and actual captures. Reuse the existing local serve workflow; do not expose the repository root.

Evidence must bind final HEAD plus dirty changes, corpus/source hashes, toolchain/features/OS, commands and exit results. Record precision/recall only for a corpus with actual independently specified labels and denominators; synthetic conformance rates are named as such. No absolute scan-time or accuracy promise is invented here. The implementation cannot be marked done by narrowing any accepted row or by moving failed required work elsewhere.

## 9. Planning-only verification and next boundary

This request imports committed v0.4 source and produces this plan plus the preserved research/WIP identification. No new language code, rule, dependency, schema, test fixture or executable example is authored or executed. Verify source identity against the imported commit, the WIP manifest, document links, acceptance-ID coverage and whitespace. Rust compilation/tests, grammar ABI checks, new-language detection and browser acceptance are future implementation evidence, not results of this planning request.

Before coding, confirm this proposed bounded support/acceptance matrix and reconcile the committed v0.4 tip. In particular, full analysis of additional surveyed libraries, new catalog meanings or new serialized source-operation roles requires an explicit scope/contract decision. This document does not change the existing v0.4 task states, evaluation criteria or release gates.
