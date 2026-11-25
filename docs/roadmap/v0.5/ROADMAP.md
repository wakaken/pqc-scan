# v0.5 roadmap: PHP, C# and C/C++ language expansion


Current implementation/verification disposition: [technical acceptance and final evidence](ACCEPTANCE.md). Task YAML and final stage reviews remain authoritative.
> Current execution selection (2026-11-24): the maintainer selected tasks 076-083 in dependency order. Earlier registration-only statements below remain dated history. Task YAML owns current status; no requirement, human-review gate, review budget, quality claim or release authority is changed by selection.

Status: roadmap and requirements defined; tasks 076-083 registered as pending on 2026-11-24. This registration does not select their execution or claim language support, tests, acceptance or release completion.

## Purpose and authority

Expand pqc-scan's existing static scanner to the requested PHP, C# and C/C++ families. The user should be able to locate the specified classical/PQC API references and dependency assertions and carry them into the existing v0.4 migration workflow with their uncertainty intact. Reuse the scanner, inventory, technical evaluator, context helpers, reports and local UI; do not build a parallel analysis or assessment system.

The maintainer's latest instruction supersedes the interrupted implementation request: first define v0.5 requirements, roadmap and tasks, using task IDs from 076. The [2026-11-19 implementation plan](../../product/PQC-LANGUAGE-IMPLEMENTATION-PLAN-20261119.md) supplies the original bounded matrix and L01-L30 acceptance. Its earlier no-task-creation restriction described the previous request; the present instruction explicitly authorizes these definitions. [REQUIREMENTS.md](REQUIREMENTS.md) retains all 30 clauses; [TASKS.md](TASKS.md) owns the task/requirement map. Do not execute pending tasks or infer a later selection from registration.

## Current source and preserved work

Baseline: `7e8cd91426fa21609a91c0511de44252eaa890d4` (`codex/v04-roadmap-20261118`, package 0.4.0), imported by fast-forward into `codex/pqc-language-research` on 2026-11-24. Local and remote source tips matched at import. The repository still uses Rust 1.93.1 and Tree-sitter 0.26.6. Later upstream changes require a fresh source/contract comparison before implementation; do not silently change requirements to match them.

Unlike the original plan baseline, v0.4's scoped input generation, context-template and local-web reassessment (073/074) are now committed. Records 071-075 are done for the explicitly amended limited scope. Their historical human/scoring limitations remain; this roadmap does not reopen or rewrite them or claim unrestricted product quality/release publication.

The interrupted attempt authored only a language-routing test draft, not product code, new rules or dependency changes. The draft and red/baseline logs are retained under ignored `target/language-expansion/`; the draft was removed from Cargo test discovery for this definition-only request. It is preliminary, source-exposed material, not a frozen independent corpus or passed v0.5 criterion. All new task statuses remain pending; no execution history is fabricated for them.

## Design boundaries and approaches

| Approach | Benefit / limitation | Disposition |
|---|---|---|
| Regex-only names | Simple lexical observations; weak namespace, alias, receiver and comment handling | Supplement existing behavior; insufficient for the agreed bounded source support |
| Tree-sitter + bounded local facts + structured dependency subsets | Matches current Rust/offline architecture; explainable source facts and explicit unknowns | Selected direction; adapt current interfaces rather than duplicate old plan proposals |
| Roslyn, Clang, PHP/runtime or package-manager execution | Potentially deeper project typing at significant execution, toolchain, portability and packaging cost | Outside v0.5 scope |

C and C++ have separate grammars. Seven existing source languages plus PHP, C#, C and C++ give eleven language entries, not ten. No Kotlin or Swift work is registered.

## Mandatory initial coverage


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

Reuse [the existing six-field provenance contract](../../architecture/DEPENDENCY-PROVENANCE.md), including source-local assertions and no manifest-lock graph joining.

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


## Shared architecture and v0.4 integration

The implementation must reuse existing Detector interfaces, file-level pipeline, budgets, scoped default-rule registry, SourceMatch/Facts, typed observations, six-field dependency provenance, inventory v1 and saved-evidence assessment. Current v0.4 already carries algorithm_references and with_algorithm_references: first check that route's sufficiency in 076, then extend it minimally in 077 only where necessary. Do not create the original plan's additional constructor merely to match a stale interface proposal.

| Surface | v0.5 obligation | Owners |
|---|---|---|
| Source grammars and scopes | Four pinned adapters; actual parse/build compatibility; namesakes, aliases, reassignment, dynamic/conditional and header/template limits | 076-080 |
| Typed capture | Canonical allowed primitive/parameter facts with original spans; ast_pattern/syntactic/referenced stays unchanged | 077-080, 082 |
| Rules and overlapping observations | Unique IDs, source-bound rationale/limits/controls; preserve legacy OpenSSL IDs unless an incompatible correction is explicitly approved | 078-081, 083 |
| Dependency/context reads | Composer/NuGet/vcpkg/Conan/build subsets; in-root selected regular files, supported conditional context and unresolved versions | 081 |
| v0.4 evaluation | Existing catalog/policy meanings and scoped input hashes; no algorithm/readiness invented from import, provider or package presence | 082 |
| Context-template and local web | Mandatory round trip/target selection/replacement invalidation and CLI/UI parity on this baseline | 082 |
| Privacy and presentation | Safe snippets/metadata, private-data canaries, useful bilingual scan/migration outputs and actual browser observations | Every component, 082/083 |

Exact recognized ML-KEM/ML-DSA/SLH-DSA parameters should reach existing finite catalog judgments. Unknown RSA/EC operation purpose must remain unknown when the adopted contract cannot express it; do not guess a role to improve a verdict. X-Wing, composite and unlisted constructions retain exact inventory identity but unknown technical judgment where unsupported. X-Wing is not synonymous with the TLS group X25519MLKEM768. Additional catalog meaning or source-operation storage semantics needs its own clear, versioned specification and applicable human decision, not an unannounced change under existing versions.

## Work packages and dependency order

| Task | Independently reviewable deliverable | Dependencies | Prior plan |
|---|---|---|---|
| [076](../../../tasks/076/task.yaml) | Pin grammar/API/input feasibility and freeze source-backed expectations and measurement protocol before feature observations | 075 (already done baseline) | S1 and interface preflight |
| [077](../../../tasks/077/task.yaml) | Four-language routing, bounded syntax/diagnostics and safe typed capture foundation, with its tests/docs | 076 | S2 |
| [078](../../../tasks/078/task.yaml) | PHP source/API and local-name/receiver support, language rules, tests and paired scope docs | 077 | S3 |
| [079](../../../tasks/079/task.yaml) | C# source/API and local type/alias/receiver support, rules, tests and paired scope docs | 077 | S4 |
| [080](../../../tasks/080/task.yaml) | C/C++ OpenSSL/OQS source support, conditional/header handling, rules, tests and paired scope docs | 077 | S5 |
| [081](../../../tasks/081/task.yaml) | All specified dependency subsets/provenance and environment-aware guidance, with its parser/negative tests/docs | 077 | S6 |
| [082](../../../tasks/082/task.yaml) | New-language scan-to-inventory-to-context-to-assessment workflows in CLI/local web and reports | 078, 079, 080, 081 | S7 integration |
| [083](../../../tasks/083/task.yaml) | Final-source all-row/API-matrix reconciliation, compatibility, rule-audit, privacy and measurement disposition | 076-082 | S7 final acceptance |

Task 076 remains separate because its results determine safe grammar pins, interfaces and corpus/format boundaries. Each implementation task owns the behavior's tests and affected documentation in the same task/PR; 083 does not collect missing component work to make earlier tasks appear complete. The three language slices own separate adapters; shared-file corrections are reconciled together. Dependencies do not authorize parallel agents: the main agent performs all investigation/design/implementation/test/documentation. Splitting tasks, PRs or resumption never resets the two-reviewer budget for one authorized implementation request.

## Compatibility, safety and non-goals

- Product scanning and reassessment remain bounded and offline. Never execute scanned projects, PHP/FFI, dotnet/MSBuild, compilers/preprocessors, dependency managers or build scripts; never fetch schemas, registries, native providers or remote trust data during scanning.
- Preserve CLI defaults, output names, serialized schema/field shapes, public struct-literal compatibility, identity algorithm, Risk vocabulary and held-input catalog/policy semantics. New detections on newly covered files may change counts/fail thresholds; document these additive effects.
- Do not weaken cryptographic checks, resource limits, path confinement, diagnostic/strict-mode behavior or privacy masking. No finding or dependency proves absence of cryptography, successful execution, active PQC, trust, negotiated TLS or whole-system readiness.
- No new probes, policy engine, web application, cloud service, numerical risk score, universal migration deadline, Kotlin/Swift coverage or full project-wide dataflow. Surveyed libraries outside the mandatory initial matrix remain research/reference targets with explicit limits.
- No task registration selects an implementation, marks a task planned/done, authorizes external outreach/cost or publishes/releases v0.5. Task numbers do not reserve new reviewer identities.

## Verification and evidence strategy

Use the original L01-L30 matrix and required API/input rows, not only aggregate artifact counts. Freeze expected behavior before new feature observations, with primary-source versions, corpus authorship/exposure, exact inputs, API/algorithm/parameter/operation, desired rule/span/count and downstream known/unknown outcome. Reuse existing seven-language/saved-evidence goldens only after source/input applicability checks. The 125-rule companion and old v0.4 logs are historical baselines, not new-language accuracy measurements.

Future implementation uses the fixed toolchain, meaningful targeted red/green cases, all applicable workspace/format/Clippy/build/optional-feature/packaging checks in [VERIFICATION.md](../../agent/VERIFICATION.md), and real browser evidence required by [REPORTS.md](../../agent/REPORTS.md). Language/rule behavior needs positive and negative tests; defaults/custom packs, Unicode, duplicate spans, budget boundaries, malformed and unknown input, private-data canaries, invalid context hashes and CLI/UI parity are not optional.

Record source and dirty-file hashes, corpus/library/schema/catalog/policy/context/time/environment, commands, exits and failures. Report fixed-condition runtime/resource/output measurements without claiming universal accuracy/performance or human usefulness. Do not invent numeric pass thresholds after observing results. Required implementation and test reviews follow [REVIEWS.md](../../agent/REVIEWS.md); main-agent self-check is sufficient for these prose/task definitions unless an explicit additional review is requested. No review subagents are started by this registration.

## Decisions and completion gates

The direction and bounded feature families come from the user-approved earlier plan. Routine finite implementation choices are settled by the main agent and recorded when executed. Task 076 records exact feasible pins, mapped API symbols, corpus expectations and shared interfaces; an alternative grammar pin with equivalent verified coverage is not permission to drop a required language. Scope/criterion changes require explicit agreement.

Stop affected work for actual storage-format changes, high-risk architecture, cryptographic logic without clear specification or potentially incompatible rule changes, as required by AGENTS.md. Continue independent authorized preparation without crossing that gate. Missing external validation/resources are reported, not replaced with passing synthetic claims. No human/practitioner study is introduced into v0.5 by analogy with historical v0.4 plans.

Task 083 can close v0.5 technical acceptance only when every initial matrix row and all 30 clauses have final-source implementation, applicable passing evidence and required reviews. Failure, unavailable or deferred required work prevents completion; expected unknowns pass only their explicitly specified unknown cases. Release/publication, measured product quality and task/roadmap acceptance remain separate decisions. Existing v0.4 records and their limited conclusions stay intact.
