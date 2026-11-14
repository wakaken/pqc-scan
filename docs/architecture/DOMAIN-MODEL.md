# Domain model and compatible evolution

Naming update: the user-facing report is now **Cryptographic Inventory and Evidence
Report** (暗号資産・検出根拠レポート), selected by `--inventory-json` and written to
`scan.inventory.v1.json`. Earlier native naming below is historical/internal;
`--native-json` has been removed from the CLI. The
[current contract](INVENTORY-REPORT-V1.md) specifies compatibility.


Design date: 2026-11-09. Status: proposal; Rust below illustrates intended types and API contracts, not implemented or compile-tested code. The current model remains in `crates/pqc_scan_core/src/model.rs`.

## Retain existing contracts

`Detection` is a rule-bound detector hit. `Finding` is the existing rule/risk/severity/confidence presentation and integration record. `ScanResult` is the existing report payload. `CbomEntry` is the existing finding-derived inventory view. `RecommendedAction` is reusable guidance presentation. Keep these public types, constructors used by consumers, `scan_repository`, and report writer entrypoints intact.

Do not expand every existing public struct with mandatory fields. Add a private-field `ScanDocument` with constructors/accessors and a new detailed scan API. Phase 0 needs only diagnostics, coverage and an unchanged legacy result; Phase 1 populates inventory/observations/assessments. This avoids an early inventory framework and avoids breaking downstream Rust struct literals.

## Concepts and identities

| Concept | Identity and meaning | Not equivalent to |
|---|---|---|
| Algorithm definition | Canonical namespace + exact family/variant/parameters/purpose | Every use of that primitive |
| Key asset | Fingerprint of public SPKI when available | Private key contents or encoded BIT STRING size |
| Certificate asset | SHA-256 of public certificate DER | Its signature algorithm or its subject key |
| Protocol definition | Protocol/version and precise standardized identifiers | A session or an endpoint |
| Usage instance | Source/configuration anchor or particular endpoint role | A global algorithm definition |
| Observation | One method-specific observation of a target in a scan/attempt | Verified production use or an assessment |
| Assessment | A contextual conclusion referencing assets/observations and a policy/catalog version | Raw observation evidence |
| Finding | Legacy user/integration view associated with a rule hit | Unique asset or durable cross-scan identity |

An RSA-signed certificate with an EC subject key creates a certificate, an EC public-key asset, RSA/EC algorithm definitions and explicit relationships. It must not be flattened into an RSA key. Two certificates sharing one SPKI are two certificates and one public key. Repeated source use of RSA shares an algorithm definition but has separate usage instances. Seeing one certificate both on disk and over TLS links two observations to the certificate; it does not prove the file configures that endpoint.

## Proposed Rust shape

Start as `model` plus `observation`, `inventory`, `assessment`, `diagnostic`, `identity`, `compat` modules in core. Names below may be refined without changing these semantics.

```rust
// New types; do not replace existing Finding / ScanResult / Risk.
struct AssetId(String);
struct ObservationId(String);
struct OccurrenceKey(String);
struct AssessmentId(String);

struct CryptoAsset {
    id: AssetId,
    kind: AssetKind,
    relations: Vec<AssetRelation>,
    identity_basis: IdentityBasis,
}
enum AssetKind {
    Algorithm(AlgorithmSpec),       // namespace, exact identifier, purpose, parameters
    PublicKey(PublicKeyDescriptor), // SPKI hash, algorithm ref, semantic size/curve
    Certificate(CertificateDescriptor), // DER hash, validity, signature/key refs
    Protocol(ProtocolDescriptor),
    Usage(UsageDescriptor),         // source/config/endpoint role and algorithm refs
    Unknown { original_kind: String },
}
struct Observation {
    id: ObservationId,              // identity within a particular run/attempt
    occurrence_key: Option<OccurrenceKey>, // eligible cross-scan correspondence
    asset_ids: Vec<AssetId>,
    target: ObservationTarget,
    method: ObservationMethod,
    assurance: EvidenceAssurance,
    usage: UsageState,
    evidence: SafeEvidence,         // typed allowlisted fields, no raw key/payload
    provenance: Provenance,         // detector/version, rule ref, run/attempt, clock
    heuristic_score: Option<HeuristicScore>,
}
enum ObservationMethod {
    LexicalMatch,
    AstPattern,                     // current tree-sitter capability
    AstResolvedApi,                 // only after symbol-resolution evidence exists
    Configuration,
    CertificateParse,
    TlsAdvertised,
    TlsNegotiated,
    SshAdvertised,
    SshNegotiated,
    Unknown(String),
}
enum ObservationTarget {
    Source { relative_path: String, span: Option<SourceSpan> },
    Artifact { relative_path: String, object_index: Option<u32> },
    Endpoint(EndpointObservation), // requested host/port, pinned IP, SNI, attempt
}
enum UsageState { Referenced, Configured, SessionObserved, Unknown }
enum EvidenceAssurance {
    Lexical, Syntactic, Parsed,
    HandshakeCompleted { peer_identity_verified: bool },
    UnauthenticatedOffer,
    Unknown,
}
struct HeuristicScore {
    value: f32,                     // finite, 0..=1 when present
    source_rule: String,
    calibration: Calibration,      // Uncalibrated unless backed by a dataset
}
```

Assurance and usage are distinct: a fully parsed certificate may be unused. An authenticated TLS session describes that endpoint/IP/client profile/time only. An AST pattern is not a resolved API call. An SSH offer is structurally parsed but unauthenticated. A configured algorithm is not necessarily negotiated. TLS/SSH advertised and negotiated methods must never be substituted for each other.

```rust
struct Assessment {
    id: AssessmentId,
    subjects: Vec<AssetId>,
    observations: Vec<ObservationId>,
    quantum: Vec<QuantumAssessment>, // per primitive/role, separate from construction
    classical_issues: Vec<ClassicalIssue>, // weak key, expiry, obsolete hash, etc.
    uncertainty: Vec<Uncertainty>,  // unresolved API, incomplete inspection, trust
    usage: UsageState,
    impact: ImpactContext,         // optional purpose, data lifetime, exposure
    migration_priority: PriorityDecision, // includes Unknown and rationale
    policy: Vec<PolicyDecision>,   // Pass / Fail / Indeterminate / NotApplicable
    migration_options: Vec<MigrationOption>,
    basis: AssessmentBasis,        // catalog/profile versions, source refs, time
}
struct QuantumAssessment {
    subject: AssetId,
    construction: Construction,    // composition, not a security verdict
    exposure: QuantumExposure,
    attack_model: String,          // key recovery, preimage, collision, etc.
    assumptions: Vec<String>,      // parameters, combiner, implementation conditions
    rationale: String,
    source_refs: Vec<String>,
}
enum Construction {
    SinglePrimitive,
    Hybrid { components: Vec<AssetId> },
    Unknown,
}
enum QuantumExposure {
    Vulnerable,
    ResistantUnderStatedAssumptions,
    ParameterDependent,
    Unknown,
}
struct ScanDiagnostic {
    code: DiagnosticCode,
    stage: DiagnosticStage,
    target: Option<ObservationTarget>,
    detector: Option<String>,
    rule_id: Option<String>,
    disposition: DiagnosticDisposition,
    safe_message: String,
    limit: Option<LimitDetail>,     // configured cap, accepted count, omission unknown
}
enum DiagnosticDisposition {
    ExcludedByPolicy, Unsupported, Failed, Partial, Truncated, NotReported,
}
struct CoverageSummary {
    selected_files: u64,
    read_ok_files: u64,
    read_failed_files: u64,
    analyzed_files: u64,
    partial_files: u64,
    analysis_failed_files: u64,
    unsupported_files: u64,
    not_applicable_files: u64,
    unknown_files: u64,
    excluded_enumerated_files: u64,
    not_enumerated_population: bool,
    status: InspectionStatus,       // CompleteWithinScope / Partial / Failed / Unknown
}
struct ScanDocument {
    // Private fields from its first release; use accessors/builders.
    // schema, run metadata, legacy ScanResult, diagnostics, coverage;
    // Phase 1 adds inventory, observations, assessments, compatibility associations.
}
```

Standardized/PQC is a catalog status, hybrid is a construction, and quantum resistance is an assessment with assumptions. They are not interchangeable enum values. A hybrid retains per-component assessments plus a combiner-specific conclusion; it is not automatically resistant. Symmetric/hash assessments can be known and parameter/attack-specific rather than forced into Unknown because they are not PQC public-key primitives. Certificate signature and subject-key roles may receive different assessments. No numeric security level is inferred from construction alone.

All counts have explicit units and disjoint per-file terminal dispositions. The required equalities are:

```text
selected_files = read_failed_files + read_ok_files
read_ok_files = analyzed_files + partial_files + analysis_failed_files
              + unsupported_files + not_applicable_files + unknown_files
```

`analysis_failed` means all applicable analysis failed without a usable result; `partial` means a usable result plus a failure or limit. `unsupported` means the selected input requires an unavailable parser/inspection; `not_applicable` means all detectors explicitly report no applicable check within the declared supported scope. `unknown` includes legacy adapters that cannot account for their internal execution. File disposition precedence is read failure, partial (if some usable result), analysis failure, unsupported, unknown, not-applicable, analyzed; overlapping detector outcomes must be reduced once using this rule. `analyzed` requires at least one applicable check and every applicable supported stage completed within limits. A cap with uncertain omissions is partial. Detector-level applicability counts form a separate table and must not be added to file counts.

Enumeration exclusions are reported separately. Ignore-hidden populations are unknown, not zero. A denominator is only the declared scope; this is not whole-program completeness. An aggregate failed-file count, if presented, is computed as read_failed plus analysis_failed, not an additional disjoint category.

`CompleteWithinScope` requires every selected target/stage accounted for without unknown, unsupported required checks, error or truncation. Intentional configured exclusions can coexist with that status only when scope and exclusions are explicit. Default limits are not implicit proof that all eligible matches were inspected.

## Detector API evolution

Keep the existing required `detect` method. Add an object-safe default method so existing external implementations still compile:

```rust
fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet)
    -> anyhow::Result<DetectorOutcome>;
// Default calls detect; includes a NotReported capability/coverage marker.
// First-party overrides distinguish not-applicable, parsed, unsupported and truncated.

pub fn scan_repository_detailed(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[std::sync::Arc<dyn Detector>],
) -> anyhow::Result<ScanDocument>;
```

`DetectorOutcome` initially contains existing detections, diagnostics and an explicit applicability/outcome status. Phase 1 adds sanitized observation drafts through constructors. The default adapter cannot infer completeness from `Ok([])`; third-party legacy detectors remain not-reported until they implement the new contract. The pipeline catches per-target failures, retains other results and fails outright only when no meaningful run can start or outputs cannot be written. Unsupported format and malformed supported format get different codes.

Keep `scan_repository -> Result<ScanResult>` as a compatibility wrapper around the same pipeline; do not run detectors twice. CLI uses the detailed path and warns about partial inspection. Preserve old exit codes; proposed `--require-complete` adds exit 3 only when explicitly selected (operational fatal error 1, existing finding threshold 2; precedence 1 then 3 then 2). Both READMEs must explain this new behavior before release.

## Normalization, inventory and assessment flow

1. Walker accounts for selected/excluded scope; reader records failures.
2. Existing detectors emit legacy hits and, incrementally, observation drafts. Parse certificate/public-key attributes once; record object indexes/fingerprints.
3. Normalize precise identifiers using a small versioned registry. Preserve unknown identifiers, original namespace and draft names; never guess parameters.
4. Inventory merges only strong identities; source instances retain path/span anchors. Dependency presence and configuration are observations, not proof of active crypto.
5. Existing risk/recommendation stages produce the same legacy result. New pure assessment functions consume observations plus optional explicit local context and policy. No report writer performs assessment.
6. Store associations between native observations and legacy finding indices plus IDs. Generated finding IDs use framed tuples; retain the multimap/index for caller-constructed records that reuse IDs.
7. Redact before serialization; export through current writers or explicit new exporters.

Proposed module APIs: `inventory::build(&[ObservationDraft]) -> InventoryBuild`, `assessment::evaluate(&Inventory, &[Observation], &AssessmentContext) -> Vec<Assessment>`, `policy::evaluate(&[Assessment], &PolicyProfile) -> PolicyEvaluation`. These are concrete local functions, not generic plugin registries. Core imports no TLS/SSH library. Probe modules return observation batches using core types; the CLI composes them. Local context values include provenance (`operator_supplied` vs inferred); absence remains unknown.

## IDs and diff semantics

Use full SHA-256 with a domain tag, identity version and canonical length-prefixed tuple encoding. Never concatenate unseparated fields. Test tuple ambiguity, Unicode, path separators and duplicate anchors. Repository scope is explicitly supplied or derived from a documented local project identifier; do not silently read or publish a credential-bearing remote URL. Paths are repository-relative, without the checkout root. A missing repository scope prevents reliable cross-project matching.

- `AssetId`: exact algorithm definition (including purpose/parameters), certificate DER or public SPKI identity. Private-key bytes are never hashed/exported to establish identity. A marker-only private-key usage has only a source occurrence identity.
- `ObservationId`: run + attempt + target/object + method + local occurrence discriminator; unique records, not a promise of stability between scans. Timestamps are provenance, not asset identity.
- `OccurrenceKey`: versioned repository scope + relative path + detector/method + safe structural anchor + local disambiguator. Exclude line numbers where a structural anchor exists, but duplicate identical statements can still be ambiguous. Do not fingerprint literal secrets; use a redacted structural representation.
- `finding_id`: `finding-v1-<SHA-256>` over the framed rule ID, relative path, line, column and evidence match. Exact duplicates share an ID. Asset/occurrence keys provide the separate inventory diff semantics.

Diffing first matches exact asset/occurrence keys with the same identity scheme and scope. Then optional file-move mappings or bounded structural similarity produce explicit candidate correspondences with a reason; they never silently establish identity. Line insertions may preserve anchors, file moves change path-based keys, duplicate identical calls may remain unmatched. Show added/removed/changed/unresolved, never mark a previous finding resolved when the new scan skipped/failed that target. Compare scope, rules/catalog/profile and completeness before reporting progress.

## Serialization and rollout

The additive Native JSON artifact is proposed as `scan.native.v1.json`, enabled by a new `--native-json` option; the six existing output filenames and default machine shapes stay intact. Phase 0 may emit opt-in `diagnostics.json` with its own version before the full native schema is available. Do not replace `report.json` or the two inventory arrays.

Native envelope records `schema_version`, identity scheme, scanner/rule/catalog/profile versions or digests, configuration/scope/exclusions, clock, run and attempt IDs, assets, observations, assessments, diagnostics, coverage, legacy associations and export-loss notes. Empty values, unknown values and uninspected values remain distinct. Deterministic record ordering is specified independently of volatile run metadata.

Publish a checked-in schema and golden fixtures when implementing the artifact. Future additive optional fields require tolerant-reader tests; incompatible semantics or required fields use a new major version and separately selected artifact. Unknown enum strings must be preservable at import boundaries. The numeric schema version is independent of the CLI package version and CycloneDX version.

Compatibility conversion is explicit and lossy. Initially keep the original legacy normalization path and snapshot beside native records; shadow comparisons gate any later derivation from observations. Do not regenerate old IDs from new asset IDs. Record exact losses: diagnostics, asset identity, observation assurance and multi-axis assessments cannot fit old structs. Correctness fixes may intentionally alter hits; every such delta needs reviewed fixtures and release notes, not blanket byte-equivalence claims.

Phase 0/1 changes are **existing implementation corrections/extensions**; native export, policy and diff commands are **new feature additions**. No existing subsystem replacement is proposed. See [architecture](NEXT-GEN-ARCHITECTURE.md), [roadmap](../roadmap/v0.2/ROADMAP.md) and [quality gates](../quality/QUALITY-GATES.md).
