# Task 026 assessment and local policy design

Base: 065646ed67a5211011b5e05b38b2ae5d0d4a6968, clean same branch.
User explicitly authorizes task026 implementation/tests/docs/independent review/
local commit; no push/PR. Existing CLI, schemas, IDs, rules, Risk, severity and
RecommendedAction remain unchanged. No crypto implementation changes.

## Chosen applicability and compatibility

Use an explicitly selected, versioned **migration-review-v1** local advisory
profile. It reviews candidate suitability and known migration/classical concerns,
not compliance or deployment readiness. No deadlines or regulatory application.
An operator can explicitly mark this profile not applicable; unknown applicability
stays Indeterminate. Context absent means local advisory applicability with unknown
purpose/runtime/provider/peer support, never assumed support.

Keep inventory v1 byte-semantics and its not_evaluated/empty assessment placeholders:
its published schema forbids populated assessments. Instead `--policy-profile
migration-review-v1` requires `--inventory-json` and writes a separate
`scan.assessment.v1.json`, linked to the same scan and asset/observation IDs.
`--assessment-context PATH` requires profile selection and reads bounded strict
v1 enum/boolean JSON offline. No new exit codes or policy fail-on switch. Unknown
flags/profile/context fail safely before output. --native-json remains removed.

Rejected: altering inventory v1 in place violates strict consumers; automatically
applying a federal timeline violates scope. New report/profile names are routine
implementation choices, not a claim that the user approved regulatory applicability.

## Model and behavior

Pure assessment.rs/catalog separates quantum exposure, classical issues,
construction, standardization, usage, certainty, purpose, context and priority.
Known parameterized ML-KEM/ML-DSA/SLH-DSA definitions retain their purpose; generic
family names/unknown/draft mappings stay uncertain. Classical RSA/EC/DSA/EdDSA
quantum exposure is distinct from key-size/digest/validity concerns. Certificate
signature and subject-key components are retained independently. No trust claim.
Symmetric/hash catalog functions describe parameter/attack-specific idealized
query models; numeric exponents are not operational security levels. New catalog
entries do not expand detector rules or silently change inventory normalization.
Hybrid catalog evaluations retain both components and an unknown combiner result;
standardization of a component alone never establishes hybrid resistance.

Known weak parameters/MD5/SHA1 collision concerns and certificate validity coexist
with quantum conclusions. Observations retain referenced/configured/unknown usage
and method assurance; heuristic confidence stays uncalibrated. No numeric calibrated
probabilities. Unknown future asset kinds/facts produce bounded unknown results.

Pure policy.rs emits Pass/Fail/Indeterminate/NotApplicable with fixed reason codes,
profile version/scope/source editions/effective-date policy and operator provenance.
Known applicable violations can Fail even with incomplete inspection. Aggregate
Pass is prohibited when native completeness is not exhaustive, legacy coverage is
incomplete or relevant facts/prerequisites are unresolved. Per-asset Pass concerns
only a bounded primitive/candidate suitability check, not a deployed product.
Non-algorithm inventory kinds are NotApplicable to primitive policy. Unknown
profile applicability is Indeterminate; explicit not-applicable is recorded.

Migration candidates retain purpose/status/parameter-selection requirements,
runtime/provider/peer/protocol prerequisites, sources and validation steps. KEMs
are not signing replacements. JWT additionally needs JOSE identifiers/encodings
and verifier rollout; PKI needs issuer/certificate/trust-store support. Dependency
presence does not satisfy any support prerequisite. Legacy recommendation wording
and payload remain unchanged; new candidate metadata is in the assessment report.

## Privacy and validation

Context permits only v1 fixed enums/optional booleans (<=16KiB, deny unknown fields).
Do not echo invalid input/errors or retain arbitrary labels/content. New report
uses fixed vocabulary and validated inventory IDs, never raw match/snippet/key
material. Catalog functions never echo unknown algorithm input. Reports have a
separate repository-owned schema; validate reference integrity to inventory.

Tests: policy decision tables; incomplete/unknown scope, no positive evidence;
parameters, classical+quantum coexistence, generic/draft/hybrid/AES/hash attacks;
certificate key/signature/validity; purpose-specific KEM/signature/JWT/PKI support;
strict/sensitive context, source provenance, deterministic output; actual opt-in
CLI schema/golden/reference/exit checks and old/new default/inventory shadows.
Frozen task023 corpus/labels remain unchanged; no approved budget is invented.
Independent design/source review plus pinned workspace build/fmt/test/clippy,
CI source-expectation rebind review, Node/Python and local evidence before commit.

## Design-review resolutions

Decision order: explicit profile NotApplicable, unknown applicability,
known applicable violation, unresolved facts/context, bounded per-primitive Pass.
Aggregate uses the same first three steps, then unresolved/incomplete/native
not-exhaustive before any Pass. Empty scans cannot Pass. Fail plus Unknown remains
Fail (with coverage/uncertainty reasons also recorded). All certificate and key
components participate; they cannot disappear through non-algorithm NA handling.

Context is scan-wide operator declaration, never verified telemetry. Support
claims apply only to a primitive whose purpose matches the declared use-case;
otherwise they are not transferable. JWT/PKI declarations only apply to signature
candidates; a KEM mismatch is explicit, not a signing recommendation. Default
unknown use-case/support cannot Pass. Explicit claims/provenance stay visible.
Classical hash collision issues are considered with collision-sensitive usage;
unknown purpose does not invent a compliance failure. Certificate signature
MD5/SHA1 is intrinsically signature-scoped. Query-exponent fractions are ideal
oracle models with separate key-search/preimage/collision assumptions and memory/
parallelism caveats, never calibrated operational security levels.
