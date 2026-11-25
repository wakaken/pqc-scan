# v0.5 requirements and acceptance map

> Current execution selection (2026-11-24): the maintainer selected tasks 076-083 in dependency order. Earlier registration-only statements below remain dated history. Task YAML owns current status; no requirement, human-review gate, review budget, quality claim or release authority is changed by selection.

Status: requirements and task definitions registered on 2026-11-24; implementation is not selected by this registration. All new tasks start pending. No requirement below is implemented or passed merely because it is documented.

## Purpose and authority

The maintainer selected language expansion as v0.5 and explicitly redirected the interrupted implementation request to requirements, roadmap and task definitions first. The earlier [implementation plan](../../product/PQC-LANGUAGE-IMPLEMENTATION-PLAN-20261119.md) and [ecosystem research](../../product/PQC-LANGUAGE-EXPANSION-20261118.md) supply the bounded scope. This document retains all 30 L01-L30 clauses from that plan; the [roadmap](ROADMAP.md) defines the complete initial source/dependency matrix and common constraints, and [TASKS.md](TASKS.md) defines ownership and sequencing. The documents are developer-facing English under AGENTS.md; future user documentation remains paired English/Japanese.

Source baseline: `7e8cd91426fa21609a91c0511de44252eaa890d4`, imported from `codex/v04-roadmap-20261118` into `codex/pqc-language-research`. Workspace package version is 0.4.0. Task records 071-075 are done for their recorded amended scope; this is not proof of unrestricted product quality, release/publication or new language support. Original historical studies and limits stay unchanged.

## Mandatory outcomes

| Group | Required outcome | Relevant clauses |
|---|---|---|
| V05-R1 | Inspect PHP, C#, C and C++ using the existing offline bounded source-analysis architecture | L01-L09 |
| V05-R2 | Inspect Composer, NuGet and the bounded C/C++ dependency subsets without executing package managers | L10-L12 |
| V05-R3 | Keep rules, typed evidence, identity, custom-pack behavior and uncertainty correct | L13-L18 |
| V05-R4 | Carry new evidence through existing v0.4 context, assessment, CLI/local-web and bilingual reports | L19-L24 |
| V05-R5 | Preserve privacy/compatibility and prove final-source conformance with honest measurements | L25-L30 |

Each group and every detailed clause is mandatory for the specified bounded matrix. Research coverage of a library is not a promise to implement every API. Conversely, an accepted initial API/input row cannot be omitted or moved to later work to close v0.5.

## Clause-to-owner and evidence map

The required behavior column preserves the original clause wording. Primary owner means the task that closes the complete clause; supporting tasks provide their scoped implementation and tests. In particular, 078/079/080 must each deliver the applicable L03-L07 language cases before 082 may close the language-wide clause. A primary owner cannot replace missing component evidence with an integration smoke test.

| ID | Required behavior | Primary owner | Supporting owners | Future evidence |
|---|---|---|---|---|
| L01 | All four grammars build with the fixed toolchain/runtime and parse required syntax; unsupported syntax retains findings with Partial | [077](../../../tasks/077/task.yaml) | 076, 083 | S1/S2; grammar pins, build and syntax corpus |
| L02 | `.c`/`.C` and mixed PHP route correctly; ambiguous `.h` cannot combine incompatible bindings or duplicate one occurrence | [077](../../../tasks/077/task.yaml) | 080, 083 | S2/S5; routing fixtures |
| L03 | Each required PHP, C# and C/C++ API family has actual detector-level positive cases and negative namesake/comment/help-string controls | [082](../../../tasks/082/task.yaml) | 076, 078, 079, 080, 083 | S3-S5; per-family matrix with zero unmet expectations |
| L04 | Aliases, qualified names, local scope and reassignment resolve only within the stated subset; unresolved/dynamic/cross-file cases never fabricate a binding | [082](../../../tasks/082/task.yaml) | 077, 078, 079, 080, 083 | S3-S5; positive/shadowing/rebound fixtures |
| L05 | KEM/signature/encryption/key-agreement roles remain distinct; generic sign/EVP/import operations with unknown key type do not become RSA | [082](../../../tasks/082/task.yaml) | 078, 079, 080, 083 | S3-S5; role and unknown tests |
| L06 | Standard ML names/parameter sets remain separate from Kyber/Dilithium drafts and from incomplete parameter names | [082](../../../tasks/082/task.yaml) | 076, 078, 079, 080, 083 | S2-S5; exact/missing/contradictory parameter cases |
| L07 | X-Wing, TLS hybrid and composite/HPKE references are not equated; unsupported technical construction remains unknown with source traceability | [082](../../../tasks/082/task.yaml) | 076, 078, 079, 080, 083 | S2-S7; inventory/assessment negative controls |
| L08 | Original spans, Unicode locations, two calls on one line and parent/child AST candidates produce the expected count and stable identities | [077](../../../tasks/077/task.yaml) | 078, 079, 080, 082, 083 | S2-S5; span/dedup assertions |
| L09 | Budget exhaustion, malformed syntax and unsupported inputs retain applicable safe observations with correct incomplete status; strict-mode behavior agrees | [083](../../../tasks/083/task.yaml) | 077, 078, 079, 080, 081, 082 | S2-S6; below/at/above limits and CLI failures |
| L10 | Every specified dependency input handles valid empty data, supported schema records and explicit incomplete/unsupported controls without becoming a false empty success | [081](../../../tasks/081/task.yaml) | 076, 083 | S6; parser matrix |
| L11 | Declared, locked, emulated platform, conditional, Git/path and SDK facts remain distinct; only eligible registry locks resolve; no invented purl/installed graph | [081](../../../tasks/081/task.yaml) | 082, 083 | S6; six-field and export assertions |
| L12 | Nearest selected in-root literal central-package context works; excluded/symlink/out-of-root/dynamic imports are not followed or evaluated | [081](../../../tasks/081/task.yaml) | 083 | S6; context-boundary tests |
| L13 | Every new rule ID is unique, schema-valid, language/library-scoped, documented and represented once in current source-bound quality metadata | [083](../../../tasks/083/task.yaml) | 078, 079, 080, 081 | S3-S6; loader/companion/ID population checks |
| L14 | Modified custom ID/kind/pattern/scope retains generic matching; new built-in binding logic attaches only to exact built-in semantics | [083](../../../tasks/083/task.yaml) | 077, 078, 079, 080 | S3-S6; registry selector tests |
| L15 | Existing OpenSSL lexical findings/IDs are preserved unless an explicit approved correction says otherwise; overlapping new AST findings have a documented disposition | [083](../../../tasks/083/task.yaml) | 080 | S5/S7; old-rule regression and coexistence cases |
| L16 | New precise API facts become safe typed algorithm assets in inventory; method stays ast_pattern, assurance syntactic, usage referenced | [082](../../../tasks/082/task.yaml) | 077, 078, 079, 080, 083 | S2-S7; no arbitrary metadata promotion |
| L17 | New dependencies, headers, imports, provider load calls and incomplete API facts stay references where algorithm identity/activation is unresolved | [082](../../../tasks/082/task.yaml) | 078, 079, 080, 081, 083 | S3-S7; negative downstream cases |
| L18 | Exact ML-KEM/ML-DSA/SLH-DSA identities/parameters reach SavedEvidence and existing technical judgments; legacy saved inputs/catalog/policy judgments remain unchanged | [082](../../../tasks/082/task.yaml) | 083 | S7; new end-to-end and old goldens |
| L19 | Rescanned/new input bytes invalidate old context bindings; target references/digests/ownership are exact and no automatic cross-input remapping occurs | [082](../../../tasks/082/task.yaml) | 083 | S7; stale-context rejection |
| L20 | Sparse claims and unknown native support never become ready; false prerequisites remain blocked even with unknowns; source references cannot establish negotiated/active use | [082](../../../tasks/082/task.yaml) | 081, 083 | S7; decision semantics fixtures |
| L21 | Scoped claims for one PHP/.NET/native target do not leak to another target/system/role; expiry and purpose conflicts retain existing semantics | [082](../../../tasks/082/task.yaml) | 083 | S7; mixed-system/purpose fixtures |
| L22 | Bilingual migration JSON/Markdown/HTML preserve the same decision/evidence and useful location/guidance; writers do not evaluate policy | [082](../../../tasks/082/task.yaml) | 083 | S7; structured/rendered comparisons and browser observations |
| L23 | CLI/local-web scans use the same new language coverage; saved assess does not rescan or start network/probe activity | [082](../../../tasks/082/task.yaml) | 083 | S7; CLI/serve integration and offline controls |
| L24 | If committed 073/074 are imported, their template/target/claim round trip, relevant input support fields and replacement invalidation work for the new targets | [082](../../../tasks/082/task.yaml) | 083 | S7; mandatory on that final source; otherwise record baseline exclusion explicitly |
| L25 | Secret canaries in keys/credentials/manifests/rejected metadata never reach legacy reports, inventory, assessment, diagnostics, IDs or UI | [083](../../../tasks/083/task.yaml) | 077, 078, 079, 080, 081, 082 | S2-S7; existing privacy policy plus canaries |
| L26 | Old seven-language frozen results, public API use, custom packs, CLI/report defaults and schema contracts pass the source-bound regression suite | [083](../../../tasks/083/task.yaml) | 076, 077, 082 | S1/S7; compatibility evidence |
| L27 | New finding/threshold impacts and inspected subsets are consistent in English/Japanese user docs, reports and library-specific guidance | [083](../../../tasks/083/task.yaml) | 078, 079, 080, 081, 082 | S7; paired documentation and representative CLI/browser checks |
| L28 | All required Rust/rule/package/feature checks and both independent reviews pass on final source; remaining gaps stay unfinished | [083](../../../tasks/083/task.yaml) | 082 | S7; commands/exits/logs/reviewer continuity |
| L29 | Measure new corpus runtime, peak resource observations and output size under fixed conditions; enforce finite parser budgets and disclose changes without claiming calibrated population accuracy | [083](../../../tasks/083/task.yaml) | 076 | S1/S7; before/after source-bound measurement |
| L30 | Corpus expectations are frozen before implementation observations; regex-target fixtures alone do not count as source accuracy, and no synthetic result is called human usefulness | [076](../../../tasks/076/task.yaml) | 082, 083 | S1/S7; corpus provenance and declared denominators |

## Applicability and evidence rules

- L24 is mandatory on this baseline: tasks 073/074 and their shared input-support/template/local-web components are now committed. The original conditional wording is preserved for traceability, not used as an exemption.
- L03-L07 expectations are frozen per required API family and language, including negative namesakes, help text, aliases, unknown keys, draft/final/hybrid/composite distinctions and unknown receiver chains. Pattern-target fixtures alone do not establish source-analysis accuracy.
- L01 requires actual candidate ABI/toolchain/build/syntax results. Available grammar packages do not prove compatibility. Task 076 may find an infeasible pin; it must document an alternative or the affected blocker without dropping a language.
- L09 applies to source, dependency and integrated paths, not only routing. L13-L15 apply to each added rule and existing overlapping OpenSSL observations. L25-L28 apply to every component owner and again to final source.
- L29 requires fixed-condition before/after runtime, resource and output observations; L30 freezes expectations, corpus provenance, denominators, protocol and interpretation before new implementation observations. These do not create an unsupported numeric performance/accuracy gate or human study. No participant recruitment, external contact, fee or score validation is requested.
- Expected unknown is a passing result only when the frozen case requires unknown. Missing, failed or unexecuted required verification remains unfinished. New unsupported libraries do not become implied safe results.
- Every implementation task ships its meaningful positive/negative tests and affected paired user documentation with the behavior; 083 reconciles integrated evidence and does not defer component testing until the end.
- Preserve known source/identity/parameter facts, input/context hashes, catalog/policy versions, explicit clock, feature/toolchain/environment, command/exit results and reviewer continuity. Reuse prior evidence only after checking applicability to final source and inputs.

## Existing assets and limits

| Asset | Reuse | Limit |
|---|---|---|
| [Source fact dispatcher](../../../crates/pqc_scan_detectors/src/source_facts.rs), [AST detector](../../../crates/pqc_scan_detectors/src/tree_sitter_detector.rs) | Tree-sitter routing, bounded SourceMatch/Facts and algorithm references | Current handlers do not establish PHP/C#/C/C++ bindings; new grammar scopes need tests |
| [Typed observations](../../../crates/pqc_scan_core/src/observation.rs), [inventory contract](../../architecture/INVENTORY-REPORT-V1.md) | Existing source references and safe canonical facts | Do not duplicate the proposed older constructor if current with_algorithm_references suffices; no arbitrary metadata promotion |
| [Dependency provenance](../../architecture/DEPENDENCY-PROVENANCE.md) | Six-field source-local declarations/lock facts and parser/status conventions | No installed graph, manifest-lock join or package-manager execution |
| [Migration readers](../../architecture/SAVED-EVIDENCE-READERS.md), [scoped context](../../architecture/SCOPED-MIGRATION-CONTEXT.md), [decisions](../../architecture/OFFLINE-MIGRATION-DECISIONS.md) | Exact saved input, role/identity, scope, freshness and independent policy axes | Syntactic reference/dependency is not execution, backend activation, negotiation or readiness |
| [Rule quality companion](../../quality/rules/RULE-QUALITY.md) | Current 125-rule population at this baseline, sources and control conventions | Historical audits do not cover new rules; no blind regeneration of judgments |
| [v0.4 requirements](../v0.4/REQUIREMENTS.md), [v0.4 task map](../v0.4/TASKS.md) | Completed implementation and source-bound evidence in its amended scope | Do not rewrite historical 071 studies or infer human utility/scoring from synthetic conformance |

## Completion and change boundaries

Task 083 must reconcile every original L clause and required API/input row to final-source implementation, applicable component/integration/browser measurements, tests and required reviews. One unmet mandatory item prevents completion. Creating artifacts, a task done label, passing limited logs or a complete source inventory is not sufficient by itself. Feasibility, implementation, acceptance, measured product quality and release/publication are separate conclusions.

Storage-format changes, high-risk architecture, unclear cryptographic logic and potentially incompatible rule changes retain the explicit human-review gates in AGENTS.md. This registration does not grant those decisions or relax the two-reviewer limit per authorized implementation request. Scope changes require explicit agreement; blocked work records the dependency, attempts, unmet criteria and concrete needed decision without rewriting this baseline.
