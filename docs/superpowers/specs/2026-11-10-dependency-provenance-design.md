# Task 027 dependency provenance design

Base 2baac32cc60adfae5cb2aca957803aae0ae414c5, clean same branch. User requests
implementation/tests/docs/independent review/local commit, no push/PR.
Use superpowers:brainstorming/writing-plans/TDD, execute inline.

## Decisions

Preserve CLI switches and legacy array/struct/schema shapes. Keep the legacy
version display normalization for compatibility, but correct generated purls:
manifest constraints (even exact declarations) do not establish resolution.
Only supported lock records with exact safe registry versions get @version.
Path/git/workspace/alias/tag/unknowns stay unresolved; never export credentials or
locator bodies. SBOM versions are reported component claims, not lock resolution:
retain reported_version with provenance, resolved_version absent, purl unversioned.
Intentional purl/metadata/diagnostic/coverage corrections are reviewed separately
from format stability. Do not silently claim byte-identical old dependency values.
Existing rule patterns/Risk/severity and ID algorithm stay unchanged.

Populate the six existing native dependency attributes (reported_version,
declared_constraint, resolved_version, version_provenance plus name/ecosystem),
without adding fields to strict inventory v1. Native source path/line/object ordinal
and existing usage/observation links retain origins. New typed dependency assertion
IDs include full normalized facts, source-relative path and scope (or run-local
scope), avoiding manifest/lock and conflicting-origin merges. No inferred graph or
cross-file resolution. Existing legacy flat projection remains lossy and documented.
Default old-detector observation adapters remain Unknown; do not trust arbitrary
legacy metadata as typed parsed resolution evidence.

## Parsing matrix and boundedness

A single inspect path feeds detect/detect_detailed/detect_observed. Replace JSON
parse-to-empty fallbacks with typed outcomes. Add pinned TOML parsing for Cargo
manifest/lock and Poetry lock, reuse existing serde_yaml for supported pnpm shapes;
reuse current lexical parsers for other ecosystem files without pretending full
language/DSL coverage. No package manager, download, manifest execution or resolver.

JSON/TOML/YAML parse with bounded nodes/depth and duplicate-key rejection. Supported
empty documents are Complete; malformed syntax/required shapes without usable
records are Failed, mixed usable/invalid records Partial, resource limits Truncated.
Unsupported format/schema versions retain safe usable reported records but are
Unsupported; headerless/heuristic/ambiguous structures are NotReported/Partial and
never Complete. Non-candidate files are NotApplicable. Diagnostics remain existing
fixed safe code/message types, no raw parser errors.

Supported structured schemas: Cargo manifest tables; Cargo lock versions1–4;
package.json dependency sections; package-lock versions1–3 (v1 recursive dependencies,
v2/3 packages authoritative); Pipfile.lock spec6; Poetry lock1.1/2.0/2.1; pnpm legacy
5.x package keys and modern6/9 package keys; CycloneDX1.4–1.7 and SPDX2.2/2.3 JSON
flat/nested component capture. Unknown/missing schema versions do not imply complete
inspection or registry resolution. Full external schema validation is not claimed.
Ruby/Gradle/XML/requirements/go.mod lexical capture stays NotReported, with typed
constraint/lock provenance where safely justified. go.mod and pinned requirements
are declarations, not installed/build-selected versions. Gem/Gradle locks need
safe concrete record evidence; git/path sources never become registry purls.

Retain each raw accepted observation before legacy dedup. Local JSON package paths
map to source key positions/object indices; no invented parent/child relationships.
Manifest/lock records with matching versions can coexist with incompatible
constraints, section/source kinds or file origins. Never select a winning version.

## Validation

TDD fixtures: ^1.2.3 declaration versus concrete lock, conflicts, workspace/path/git,
renames, each existing ecosystem, partial/empty/malformed/duplicate/unsupported/
nontext/limits; purl percent encoding and no raw credentials; declaration versus
reported SBOM; native schema/identity/reference/root/line/determinism; default CLI
and explicit inventory/policy paths. Existing diagnostics may correctly become
complete on fully supported input; --require-complete still means code3 only for
incomplete inspection (no exit code contract change). Native remains not exhaustive.

Keep frozen corpus/labels, compare intentional semantic deltas and field shapes,
measure old/new costs, review strict CI source bindings with no blanket suppression.
Pinned workspace build/fmt/test/clippy, Python/Node, source-bound evidence and
independent review/Evaluator before local commit. No remote CI or publication claim.
