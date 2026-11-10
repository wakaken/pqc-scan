# Task 025 additive native v1 design

Base: 8ba863ff335023b3e3a33affa7cb250b3f7bec25, same user-selected branch.
Task025 explicitly authorizes an additive native export; no legacy storage/ID,
rule semantics, cryptographic checks or default format replacement is authorized.

## Chosen approach and alternatives

Retain the legacy normalization pipeline unchanged. Add a separate observed scan
entrypoint, optional detector observation capture, pure inventory/identity/compat
modules and a separately selected writer. Deriving observations from final
findings would discard duplicate methods and inventory-only/public-cert facts;
replacing legacy normalization would risk old consumer IDs/bytes. A new crate,
generic graph engine, resolver or reparse is unnecessary.

## CLI/API boundary

`scan --native-json [--repository-scope org/project]` adds only
`scan.native.v1.json` beside the selected legacy formats; --format all has its
existing six files. Scope requires --native-json and a bounded, non-secret opaque
project name (ASCII alphanumeric and ._- segments separated by /; no URL,
credentials, empty/dot traversal or checkout path). Without scope, occurrence keys
are absent and usage identities are run-local; never promise cross-project diff.
Native does not affect exit precedence 1/3/2 or human-language behavior.

Existing ScanConfig/Detection/Finding/ScanResult public literals and old scan/report
entrypoints stay unchanged. Add `NativeOptions`, `scan_repository_native`,
`ScanDocument::native()`. Add optional object-safe `Detector::detect_observed`
with a default adapter calling detect_detailed once. Existing external detectors
need no edits and receive Unknown assurance, never inferred Parsed assurance from
untrusted metadata. Built-ins share internal implementation with an observe flag;
default scans do not capture native data, native scans do not re-run detectors.
DetectorOutcome adds private drafts/accessors/constructors only.

## Safe observations

Each pre-normalization detector hit can contribute a method-specific observation.
Capture exact allowlisted algorithm identifiers before legacy preview masking;
never export/hash raw match, snippets, file contents, arbitrary metadata, private
key bytes, public DER/SPKI bytes, endpoints or arbitrary object labels. Unknown
raw identifiers are omitted with a loss note rather than guessed. Recognized
draft identifiers remain in a draft namespace, never upgraded to final standards.
A structural anchor uses method/rule + normalized typed facts, not literal input.
Repeated identical weak anchors get deterministic local ordinals and explicit
ambiguous_duplicate_anchor confidence; a key is correspondence material, not
proof of stable identity. Native legacy positions retain their declared unit
(legacy_mixed for AST); known byte spans are separate from Unicode columns.

Lexical matches are Lexical/Referenced; AST patterns Syntactic/Referenced, not
resolved APIs. Configuration matches are Lexical/Configured references, not active
runtime configuration. Dependencies are declared/resolved/unknown only when the
existing parser source provides that distinction; uncertain provenance stays
unknown (task027 owns parser expansion). Valid parsed cert attributes are Parsed/
Unknown usage, not trusted/verified/deployed. Private markers get no key asset.
The default legacy adapter is Unknown/Unknown, even if it forges evidence fields.

Safe typed evidence contains only normalized algorithms, validated numeric/public
certificate facts/fingerprints, bounded package identifiers/version tokens and
fixed marker/method classifications. Import schemas/unknown enum strings are
tolerant, but the writer only accepts a captured ScanDocument, not arbitrary
imported JSON. Collection and final build both validate recognized facts.

## Inventory and identity

Assets: exact algorithm definitions, public SPKI keys, public DER certificates,
source/config usage instances and dependencies. No algorithm means no guessed
algorithm asset. Cert signature algorithm and subject key algorithm are separate;
two DER certificates sharing SPKI are two certificates/one key. Strong cert/key
identity is fingerprint only; role/metadata conflicts are recorded as uncertainty,
not hidden. Exact namespace+identifier+purpose+parameters identifies algorithms.

Use full SHA-256 of domain/version plus u64 big-endian length-prefixed UTF-8 tuple
components, including tuple length. IDs have distinct asset/observation/occurrence
v1 prefixes. No concatenation ambiguity, truncation, credential-bearing remotes or
absolute checkout roots. Observation IDs are scan/attempt-local; generated scan ID
is provenance only. Algorithm/cert/key IDs are strong/global; usage keys require
explicit repository scope and relative path. Line insertion preserves a weak
anchor ordinal where otherwise unchanged; moves change path keys; duplicates are
explicitly ambiguous. No private material or unsafe raw evidence enters new IDs.

Observations retain all methods before legacy dedup. Assets merge only exact
strong identities or explicitly scoped usage keys; never merge solely by a display
name or guessed overlap. Order assets by kind/typed descriptor, observations by
source target/method/rule/local ordinal, independently of volatile run IDs.

## Compatibility and schema

Build native alongside the unchanged final legacy snapshot, not from a rewritten
legacy projection. Private transient LegacyBinding tuples match exact existing
rule/path/line/column/masked-evidence fields; they are never serialized or used in
new identity hashes. Match to final sorted finding indices, retaining the legacy
ID multimap even when IDs collide. Multiple observations can link one finding;
unmatched/dropped/colliding legacy conversions get explicit loss notes/counts.
Dependency inventory-only and certificate parse-only observations need no fake
legacy finding. Do not collapse by legacy ID alone.

Native envelope: schema_version 1, identity_scheme, scanner/rule/catalog versions
or digests, run metadata, optional repository scope, existing inspection scope /
coverage/diagnostics, assets, observations, legacy associations, assessments empty
with assessment_status not_evaluated, and explicit export-loss notes. Publish
schemas/native-scan-v1.schema.json and checked-in semantic golden fixtures.
Unknown string enum values/additive reader fields are tolerated. Writer references
must be unique/resolving and schema validated offline with already pinned
jsonschema dev dependency. Legacy report.json/CBOM/SBOM/SARIF are unchanged, and
native does not claim CycloneDX compliance or policy/quantum assessment (task026).

## Acceptance/review focus

Tuple ambiguity/root/Unicode/path/line/move/duplicate-scope cases; old public API
struct literals/third-party detector and zero double invocation; multi-method
observations surviving legacy dedup/ID collision; cert-vs-key and signature-vs-
subject-key sharing; unknown/malformed parsed values; canaries in every field and
native IDs; explicit opt-in/files/language/exit behavior; official local native
schema, golden/tolerant reader/reference checks; same-root all-format old/new
shadow and frozen corpus/performance; strict CI raw projection reviewed updates.
No new crypto/rule semantics or runtime dependencies. If an actual breaking
storage/model/architecture change is required, stop and report the concrete gate.

## Builder review resolutions

Independent design025_review accepted additive architecture without an architecture
stop condition. Certificate capture without certificate rules may inspect once,
but its legacy DetectorStatus remains NotApplicable and legacy diagnostics/exit
stay unchanged; native capture_status and fixed issues separately reveal malformed/
unsupported/unexhaustive capture. Native completeness is never inferred from the
legacy applicability counters. Numeric OIDs from explicit public parsing are
retained; unparsed/PSS parameters stay unavailable rather than exact guessed data.

Relative paths, rule/provider tokens, numeric OIDs/digests, package identifiers and
version tokens are validated before new identity as well as serialization. Invalid
outside-root/URL/credential paths become no target with uncertainty, never hashes
of the rejected value. Paths/package/rule labels are permitted operator-sensitive
metadata, not a claim of universal secret detection. Default custom adapter ignores
all apparent parsed metadata. Every weak duplicate member is flagged ambiguous.

Legacy dependency source_type collapses manifests/locks, so native preserves only
validated reported version with declared/resolved provenance explicitly unavailable
until task027; it does not manufacture a resolved dependency graph. Unknown raw
algorithm strings are intentionally omitted; numeric public OIDs can be represented
as exact OID identifiers with unknown parameter interpretation. No security levels,
trust validation, active-key claim or assessed quantum verdict is inferred.
