# Dependency declarations and resolution evidence (task 027)

The scanner reads local files only. It does not install packages, query registries,
join manifests to locks, execute build scripts or infer dependency edges. A lock
record is evidence of a recorded resolution, not proof of deployment or provenance
verification. SBOM versions are unverified reports, never installed-version proof.

## Existing inventory fields

Dependency assets retain exactly the six v1 attributes: `name`, `ecosystem`,
`reported_version`, `declared_constraint`, `resolved_version`, `version_provenance`.
Missing facts are null. Manifest `^1.2.3` remains that declaration, with no resolved
version. Supported registry lock `1.2.3` can supply a resolved version. Conflicting
sources remain separate assertions; neither wins silently. `reported_version`
preserves a safe source claim, not the legacy display normalization.

Fixed origins distinguish manifest/default/dev/peer/optional declarations,
workspace declarations/inheritance, unresolved path/git/archive dependencies,
lock resolution/unverified records, SBOM claims, lexical declarations/locks and
unknown source. Dynamic versions, unsafe names/versions and locators cannot become
registry resolution facts. Versions/names are bounded allowlisted ASCII identifiers;
this deliberately omits valid but unsupported syntax rather than guessing.

Only an accepted concrete lock record creates a versioned purl. Declarations and
SBOM claims have at most an unversioned purl; path/git/archive/workspace inheritance
has no registry purl. Encoded scoped npm and Maven namespace components are emitted
from validated fields; raw input purls/URLs are never copied. npm locks with custom
HTTP registry/archive locations are conservatively unresolved. Cargo locks without
`source` describe local/workspace records, not a registry resolution.

## Supported subsets and outcomes

| Input | Parsing and resolution boundary |
|---|---|
| package.json | JSON dependency/dev/peer/optional sections, safe aliases; declarations only |
| package-lock.json | Versions 1–3; v1 nested dependencies, v2/3 packages; concrete registry records, links/locators remain unresolved |
| Cargo.toml | TOML dependency/build/dev/target/workspace tables, aliases, version/path/git/workspace sources; no workspace inheritance resolution |
| Cargo.lock | Versions 1–4; concrete registry source records; git/local records unresolved |
| Pipfile.lock | pipfile-spec 6, default/develop; exact registry pins, separate source kinds |
| poetry.lock | Lock versions 1.1/2.0/2.1; registry vs git/directory/file/URL sources |
| pnpm-lock.yaml | Versions 5.3/5.4/6/6.0/9/9.0; package records, root/importer declarations and peer suffixes; no graph inference |
| CycloneDX / SPDX JSON | CycloneDX 1.4–1.7 / SPDX 2.2–2.3 component/package subset; reported claims only, no trust/full-schema validation |
| requirements.txt | Fully consumed bounded name/version-specifier subset; directives, markers/extras and unsupported syntax retain safe lexical records and incomplete status; no include/URL fetching |
| go.mod | Bounded module/go/require declarations; unsupported directives retain safe facts and incomplete status; no module resolution |
| Gemfile / gems.rb / gemspec | Static quoted declarations and guarded top-level version variables, including dependency calls on a recognized `Gem::Specification` block receiver; selected plain ASCII version inputs (eight paths, 128 bytes each) may supply declared constraints through the exact relative File.read/expand_path/strip form; leading relative imports of selected constant-only `.rb` modules may be checked (eight paths/16 KiB/4096 nodes/eight module levels), without propagating imported constants or executing Ruby; dynamic Ruby stays incomplete |
| Gemfile.lock / gems.locked | Bounded sections and records; only accepted GEM registry source records resolve, git/path/custom sources remain unverified |
| pom.xml | Bounded XML dependency subset, DTD/external entity disabled; unknown/dynamic/property/profile constructs remain incomplete |
| Gradle | Existing bounded lexical fallback, NotReported |

`Complete` means this supported inventory subset was inspected, including valid
empty containers. It is not complete package-manager schema validation. Duplicate
keys and malformed supported documents fail the detector with fixed messages;
usable records survive malformed sibling records as `Partial`. Unsupported schema
versions/containers remain Unsupported or NotReported, never Complete. A malformed
file is not silently converted into a successful empty inventory. Other files are
NotApplicable. These statuses flow into existing diagnostics/strict-mode rules.

Input is capped at 2 MiB, nesting at 64, accepted records at 10,000, and finite structured-value construction
budgets (500,000 for the existing decoder default; 50,000 for v0.5 JSON formats). Truncation remains explicit. YAML aliases/tags are rejected
before expansion; multiple YAML documents and duplicate mappings are rejected.
A byte/nesting preflight precedes bounded serde value construction. TOML's parser
also constructs its own bounded-input syntax representation. No universal syntax,
resource/performance guarantee or full package-manager equivalence is claimed.

## Identity, traceability and compatibility

Typed dependency asset IDs use the existing v1 hash scheme over a fixed dependency
assertion domain, repository scope (or run-local fallback), relative source path,
accepted-record ordinal and normalized six facts. Different source files/sections
therefore do not merge. Ordinals can shift when earlier records change; these are
source assertions, not installed package or cross-file graph identities. Unique
key/name matches provide best-effort source lines; repeated/ambiguous matches use
line 0 (unknown), never the first unrelated record. The ordinal remains available.

Existing CLI flags, filenames, public legacy struct shapes and flat dependency
SBOM fields are unchanged. Legacy `version` still uses its existing display
normalization, and locks still project to the historical `source_type: manifest`.
Finding deduplication merges identical rule/path/position/match tuples.
The richer inventory preserves parsed origins;
third-party Detector default adapters still cannot assert parsed provenance.

Intentional semantic corrections: purls no longer claim manifest/SBOM versions;
invalid/source-specific records no longer fabricate registry identity; coverage
and strict-mode results reflect actual outcomes; source lines may change or become
unknown. These can affect custom dependency regexes targeting purl text and legacy
finding IDs, whose framed inputs include relative path and position/evidence.
They are not a promise of byte-identical findings under corrected semantics.
Nondependency rule matching and finding IDs remain unchanged for unchanged inputs;
snippets/previews inside dependency/SBOM containers are intentionally masked.
An SBOM display label that lacks the namespace required by its reported purl type
is retained with unknown ecosystem and Partial inspection rather than guessed from
an unrelated purl identity or dropped.

Dependency/SBOM source snippets and previews are masked filewide in every legacy
report. This includes escaped URLs and locator fields outside dependency records.
Safe parsed dependency facts remain available. Inventory already omits raw snippets,
locators and arbitrary metadata. This is not a general secret-scanning guarantee:
paths/package names and findings from unrelated detectors retain their own policy.

## Parser dependencies and references

The TOML runtime parser is pinned `toml=0.8.23` (parse only, MIT/Apache-2.0); locked support
crates are toml_datetime 0.6.11, serde_spanned 0.6.9, toml_edit 0.22.27 and winnow
0.7.15. Existing serde/serde_json are reused. Task 063 replaces the archived YAML
implementation with yaml_serde 0.10.7 under the existing serde_yaml Cargo alias;
its maintained low-level parser is libyaml-rs 0.3.0. This addresses maintenance
status, not a claim that translated unsafe parser code has become memory-safe.
Compatibility comparisons and regression evidence are in
[task 063](../../tasks/063/TASK-063-RESULTS.md). Build-time package fetching
is distinct from scanner runtime; no runtime network resolution was added.

Source semantics were checked against the official [Cargo dependency reference](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html),
[npm package-lock reference](https://docs.npmjs.com/cli/v11/configuring-npm/package-lock-json/),
[Python dependency specifiers](https://packaging.python.org/en/latest/specifications/dependency-specifiers/)
and [Go module require directive](https://go.dev/ref/mod#go-mod-file-require).
The tests/above matrix define this scanner's deliberately narrower implementation.

## Added format and SBOM identity support

The local XML parser is pinned roxmltree0.20.0; DTD/external entities are disabled, with input/nesting/node bounds. Other new supported declaration subsets reuse local parsing without execution or downloads. Legacy flat version display remains compatible, while declared constraints and unverified reported versions stay separate in inventory.

SBOM display labels do not need to contain ecosystem namespace syntax. A validated purl can supply canonical type/namespace/name and unverified reported version; its source claim never becomes a lock resolution. Invalid or conflicting identities remain incomplete and cannot fabricate registry purls. The source SBOM display name is not rewritten on disk.

## v0.5 bounded language dependency inputs (task081)

The user-facing [input table](../user/COVERAGE.md#php-net-and-native-dependency-inputs) lists every new format. Separate `languages/{composer,nuget,c_family}.rs` modules reuse Build, duplicate-rejecting JSON, pinned roxmltree and selected RepositoryContext reads. New JSON enforces a50,000 visited-value construction cap to the shared2MiB/depth64 bound; XML has50,000nodes/depth64, CMake50,000tokens/depth64/delimiter64, Conan text50,000lines, and every parser accepts at most10,000records. Unique JSON string locations and XML line offsets are indexed in finite passes; ambiguous JSON occurrences use line0. No regex rescans the whole input for every record. Central lookup examines at most32parent levels and only the first selected readable Directory.Packages.props; truncated/partial/malformed/conditional/dynamic/duplicate/imported central values never supply an effective version. Explicit malformed project versions never fall back to central values. Directory.Build.props is independently inventoried without simulating import order. No manager, project, compiler, preprocessor, build, network or crypto executes.

The existing public DependencyOrigin variants and six serialized dependency fields remain intact. Additive non-exhaustive DependencyContext is finite typed parser context, accepted only on typed parsed dependency drafts; it reuses open provenance/capture strings and never sets a resolved version. Platform required/dev/emulated/disabled, SDK/TFM declaration, lock-framework report, baseline/feature/override, conditional, development-source kinds, build requirement/lock, lexical build reference, legacy declaration and inherited central constraints remain explicit. No arbitrary metadata promotes assurance. PHP platform/SDK/TFM/feature/baseline records are assertions about source configuration, not installed components; feature names use port/feature, baselines use fixed registry-kind/ordinal names. Separate ordinals preserve identity without fabricating graph edges, bucket membership links or cross-file resolutions.

Composer lowercases and validates exactly vendor/package. Its lock records lack authenticated registry origin in this subset, so Git/path/archive/custom/plain records are reported/unverified without registry purls or resolved versions. NuGet lowercases identifiers for case-insensitive comparison but retains separate source ordinals, accepts locks1/2 only and independently preserves requested/resolved claims. Legacy packages.config remains reported/declaration only. NuGet bracket ranges, safe Conan revisions and vcpkg port revisions use ecosystem-specific validation; old ecosystems retain their original validator semantics. Native build sources never fabricate a purl. [Composer purl definition](https://github.com/package-url/purl-spec/blob/main/types/composer-definition.json) requires a vendor namespace and lowercase identity; [NuGet purl definition](https://github.com/package-url/purl-spec/blob/main/types/nuget-definition.json) has no namespace. These definitions inform representation only, not registry authentication. Full source provenance and frozen editions remain in task076 FORMAT-SOURCES.json.

Three additive dependency predicates use exact names and ecosystem markers. Legacy broad dependency patterns may still match new inputs; their risk is not algorithm selection. New language guidance distinguishes API availability from backend/OS/build/provider/TLS/peer support and exact OQS wrapper maintenance identities; new NuGet BC records do not introduce a Java-BC runtime signal. No evaluator/catalog/policy meaning, CLI defaults, output field shapes or identity algorithm is changed.
