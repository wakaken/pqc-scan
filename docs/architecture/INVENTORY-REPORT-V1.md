# Cryptographic Inventory and Evidence Report v1

The user-facing name is **暗号資産・検出根拠レポート** in Japanese.
`--inventory-json` writes `scan.inventory.v1.json`. This is an original pqc-scan
JSON format, not the name of a shared standard. `--format all` alone still writes
the six legacy reports. `--repository-scope team/project` requires the report
option and must be a non-secret relative project identifier.

`--inventory-json` is the only CLI switch for this report. The former
`--native-json` is rejected as an unknown argument, including when combined with
the current switch, before report output. CLI consumers must use
`--inventory-json` and read `scan.inventory.v1.json`. The CLI does not produce
`scan.native.v1.json`. Existing files in the output directory are not deleted.
The library's `write_inventory_json` is the current writer; the existing
`write_native_json` function still retains its original filename for library
compatibility, without exposing another CLI switch.
Core API names (`NativeScan`, `NativeOptions`, `scan_repository_native`,
`ScanDocument::native`) remain available. JSON field names, identity scheme,
ID generation and schema version do not change with the descriptive rename.

The [MIT schema](../../schemas/native-scan-v1.schema.json) retains its original
path and `$id` as compatibility identifiers. Only its human-readable title changes.
It uses JSON Schema Draft 7, requires v1 fields and supports open string vocabulary
for future readers. Rust readers ignore additional fields and preserve unknown
strings; `validate()` rejects unsupported major versions, duplicate IDs and
unresolved references. The report is not CycloneDX or an assessed PQC verdict.

## Collection and meaning

`scan_repository_native` adds capture alongside the existing pipeline. Existing
entrypoints, public legacy structs, default filenames, IDs, normalization and
report writers remain usable. `Detector::detect_observed` is optional: the default
adapter runs an existing detector once and emits Unknown assurance/usage. It does
not interpret arbitrary evidence metadata as parsed facts. Built-in detectors
capture before legacy masking/normalization without repeating their parser.

Observations retain method, producer, assurance and usage separately. Lexical
matches are references; AST-pattern matches are syntactic references, not semantic
resolution or active use. Configuration matches are configured references.
Dependency reports do not prove deployment or a resolved dependency graph.
Certificate parsing does not verify trust, signatures, private-key availability or
active use. Assessments are empty with status `not_evaluated`.

Algorithm definitions, public keys, certificates, reported dependencies and source
usage instances are different asset kinds. Exact recognized names are preserved;
Kyber/Dilithium draft names are not equated with final ML-KEM/ML-DSA names. Unknown
raw identifiers are omitted. Parsed public numeric OIDs remain OIDs. Uninterpreted
ASN.1 parameter bytes are not exact algorithm parameters: sequence/unknown values
are marked unavailable, never differentiated by opaque parameter digests.
Certificate DER fingerprints identify certificates, SPKI fingerprints identify
public keys; certificate signature and subject-key algorithms have distinct roles.
Two certificates may reference one key. Conflicting descriptors carry uncertainty.

Certificate capture also runs without certificate rules, preserving the legacy
NotApplicable coverage/status. Parse-only objects and malformed/trailing container
issues remain inventory observations. Legacy coverage is copied with its original
meaning; `native_completeness=not_exhaustive` does not promise a complete inventory.

## Identity and ordering

Inventory SHA-256 IDs use the full digest, a versioned domain and length-prefixed
UTF-8 tuple components plus component count. Legacy IDs are unchanged. Algorithm
and public-object IDs describe content/normalized facts, not source placement.
Observation IDs are scoped to a scan run and static attempt. Without an explicit
repository scope, source usage IDs are run-local. With scope, occurrence keys use
relative path, a typed weak anchor and local duplicate ordinal. Checkout root,
line/column and raw evidence never enter these occurrence identities.

Line insertions preserve a unique weak occurrence; moves and scope changes do
not. All repeated anchors are ambiguous, and inserting a duplicate can renumber
ordinals. Keys are correlation hints, not proof of cross-scan identity. Positions
break ordering ties only. Usage attributes retain the safe typed anchor to make
asset order independent of random run IDs, including same-file/method cases.
The semantic golden normalizes only scan/time, observation IDs and legacy IDs;
references are rewritten together. Scoped occurrence and asset hashes stay exact.

## Privacy and compatibility losses

Inventory evidence includes only fixed capture status and marker kind. It omits raw
matches, snippets, arbitrary metadata and private payloads, and never fingerprints
private key bytes. Targets must match the file actually inspected, then pass
relative-path validation. Unsafe targets/facts are omitted with fixed uncertainty.
Only bounded package/version identifiers and public-parser facts enter assets.
Rejected dependency identities have fixed capture status
`unsafe_or_unavailable_dependency_identity_omitted`, without retaining the rejected value.
Paths, rule/producer labels and package names remain operator-sensitive metadata;
this validation is not a general secret detector. Legacy outputs retain their
existing masking policy and are not derived from inventory.

Every final legacy finding has a `legacy_associations` entry with its final index,
legacy ID and all contributing observation IDs. Index is authoritative even if
legacy hashes collide. Exact transient legacy tuples link observations; raw tuple
material is never serialized or used in native IDs. `unassociated_observation_ids`
provides a convenient list. `legacy_loss_records` adds per-observation fixed
reasons, positive counts and units (`legacy_binding` or `capture_only_observation`):

- `certificate_parse_without_rule_finding`: parsed public object without a legacy hit.
- `container_capture_without_rule_finding`: container limitation without a legacy hit.
- `dependency_inventory_not_a_legacy_finding`: the existing inventory pseudo-hit.
- `unknown_rule_dropped_by_legacy_normalization`: an unknown rule dropped by legacy.
- `legacy_normalization_collapsed_distinct_binding`: a distinct tuple lost by legacy deduplication.
- `legacy_target_unavailable`: a legacy tuple cannot be represented for association.

Counts describe unprojected bindings/capture-only observations, not missed crypto
usage. A many-method observation can contribute to the same final finding; an
observation with some links may still have losses for other bindings. Version
provenance is `legacy_unclassified`, or `unavailable_or_unsafe_version_omitted`;
declared constraints and resolved versions remain unavailable until task 027.

## Explicit task 026 assessment

The strict v1 `not_evaluated`/empty assessment fields remain unchanged.
`--policy-profile migration-review-v1` with inventory selection writes a separate
`scan.assessment.v1.json`. See [local advisory assessment](LOCAL-ASSESSMENT-V1.md)
for axes, applicability, unknown context, prerequisites and the no-aggregate-Pass
rule. Existing report/schema/identity behavior is preserved.
