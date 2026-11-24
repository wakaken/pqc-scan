# Saved-evidence feasibility and architecture comparison

Status: recommended D1/D3/D5 choices adopted by the maintainer on 2026-11-18,
based on integrated HEAD
48276e48f5c6d4eadafb2a3aa9d4c1a54808f1ab and the dirty-source manifest.
This is a source/contract study, not an implemented importer or interoperability
measurement. [Task065](../065/TASK-065-RESULTS.md) supplies the current all-rule audit.

## Reusable boundaries

- `NativeScan` and `validate()` in `crates/pqc_scan_core/src/inventory.rs` already
  represent static assets/observations and validate v1 identity/reference structure.
  `assessment::evaluate_inventory` already follows bounded kind-specific links;
  `policy::evaluate` consumes that result for `migration-review-v1` only.
- `probe::ProbeReport<T>::validate` checks envelope version, attempt, endpoint
  binding, limits/traffic and outcome/diagnostics. It is not a TLS field-consistency
  validator or a generic saved-file reader.
- TLS `TlsObservation` and SSH output models derive Serialize, not Deserialize;
  protocol implementations are optional and protocol crates depend on core.
  Core must not depend back on those crates or initialize their runtimes to read
  JSON. Reuse their field/schema contracts, not a network-capable constructor.
- Existing schemas identify `native-scan-v1`, `probe-tls-v1`,
  `probe-ssh-offers-v1` and `probe-ssh-kex-v1`. A saved probe is not Inventory v1.
  Endpoints cannot be encoded as source file paths. Strict published schemas
  cannot receive in-place new business-system/context/decision fields.
- Existing length-framed SHA-256 helpers can generate additional new-namespace
  assessment-target IDs while preserving every legacy/native ID. Safe output
  already provides per-file atomic, non-symlink writes; it is not a transaction
  across all files or a guarantee against someone replacing input/output files.
- The integrated v0.3 HTML now includes language/theme controls, file navigation,
  `report.rules.html` and the separate `serve` scan UI. These are the compatibility
  baseline. New assessment is not inserted into scan/serve defaults or those
  files. Minimum new human output remains bilingual Markdown support.

## Alternatives

| Option | Source evidence and benefit | Cost/compatibility | Disposition proposed |
|---|---|---|---|
| Existing-format adapters | Already versioned saved facts and local validators; no need to recollect files or connect | Add bounded strict readers and protocol-specific semantic checks, then project selected facts | Select |
| Small internal fact projection | Static observations and runtime attempts have different targets but share purpose/identity/assurance/technical-assessment needs | A narrow internal tuple of validated target, facts, support and limitations is needed; it must not become a second persistent inventory or generic framework | Use only inside the adapters/evaluator; no new crate, stored format or collection path |
| Integrated Inventory major migration | Could unify file and endpoint identities | Requires new schema, builders/readers/exports/identity migration and compatibility work with no demonstrated unmet requirement | Reject for v0.4 |
| Replace v1 evaluator | Appears to reduce dispatch branches | Reinterprets published v1 policy and duplicates prior work | Reject |

The minimum sufficient implementation is new opt-in dispatch plus small adapters
feeding shared pure assessment. Keep core provider-independent. A private reader
DTO may mirror the accepted saved envelope to validate it, but no second writer
or inventory construction is required. Share pure primitive helpers only when
old v1 behavior remains exact under its existing dispatch. Protocol combination
facts need the separately versioned mapping in FACT-SOURCE-MAPPING.md.

## Field-to-fact and capture gaps

| Input | Actual reusable fields | Facts supported | Explicit gaps and future adapter checks |
|---|---|---|---|
| Inventory v1 | run/profile/schema/identity scheme, coverage/diagnostics, assets/relations, observations/target/method/assurance/usage | Exact recognized primitive/public-object facts with observation-specific links; configured or referenced use stays distinct | Tolerant unknown vocabulary becomes unknown. Some lexical findings have no algorithm asset; retain a reference-only target and investigate rather than invent a primitive. Duplicate/ambiguous occurrence anchors cannot receive guessed system bindings. Current inventory is not exhaustive |
| TLS v1 | envelope outcome/peer binding; details.handshake_complete/kind; verification chain/name result; numeric negotiated version/suite/group; peer certificate attributes/fingerprints and collection status | Collector-reported negotiated key establishment only when all success gates agree; parsed signature/subject-key facts kept in separate roles | No CertificateVerify scheme, deployment-wide coverage, reverified chain, revocation verification or DER bodies. ASN.1 parameter-encoding details are absent; an OID alone must not acquire encoding-validation assurance. Validate redundant flags and known ID/name pairs; incomplete collection retains safe facts without a global safety result |
| SSH offers v1 | envelope; details.assurance; ten ordered offer lists, extension markers and collection limits | Unauthenticated advertised algorithms, with exact names and role; no selected method | Lists are not a session or host trust. Do not infer first common algorithm as negotiated. Recompute finite classification from names; never trust stored classification strings as authority. Reject duplicate/over-limit structural fields and preserve unknown algorithm names as unknown facts |
| SSH KEX v1 | completed/identity-verified flags, negotiated_kex, host_signature, offered profile, trust_source, user_authentication_attempted | Collector-reported pinned-host KEX result only under the original no-user-auth/verification gates | No user authentication, full offer-list transcript, arbitrary host-key algorithms or verified PQC authentication. Current supported pin is Ed25519. False/contradictory verification or user-auth flags cannot produce a verified-session claim |

All inputs are untrusted local files. A digest binds their content; it does not
prove the collector's identity or truth. Preserve the saved record's reported
verification scope while explicitly stating that offline reassessment does not
repeat cryptographic verification. Operator claims cannot raise collection
assurance. A copied `verified` string is never newly verified by this importer.

## D3: runtime scope options

| Choice | Adapter work and cases | Benefit/limit |
|---|---|---|
| Inventory + TLS only | Two formats, one runtime envelope; explicit errors for both SSH formats | Meets proposed minimum; A16 remains a specified offer/negotiation distinction with rejected ingestion |
| Also SSH offers | Reuse envelope and validate ten finite lists/markers; positive, unknown, partial and no-negotiation cases | Adds low-assurance capability review; leaves already saved negotiated KEX unsupported |
| Also SSH offers and KEX (recommended) | One envelope implementation plus two narrow detail validators; KEX verification/pin/no-user-auth cases separately | Reuses both current products and avoids throwing away existing saved session evidence. More cases/field checks than the minimum, with no transport/backend work |

Cost labels above are structural comparisons, not time or performance estimates.
Both SSH readers can remain available in the default static binary because they
only parse saved JSON. They do not enable protocol collection features, `serve`
network targets, endpoint discovery or network package publication. D3 must
actually select one option before the draft cases/contract are finalized.

## D5 repair boundary

Use task065's correction impacts. Preserve all executable rule values, categories,
Risk, confidence, 0.45, existing IDs and v1 results. Do not repair the four retained
configuration false positives in this task set without a separately approved
specific repair. New facts may describe classical and quantum issues separately
without changing those legacy fields. Proposed new mappings cover exact final
TLS/SSH hybrid identifiers and known component roles; draft/prehash/unknown
encodings retain uncertainty. Task065 annotation/reference-path augmentations
remain development evidence, not new algorithm support.

The source-bound technical mapping is a prerequisite for decision inputs. System
claims refer to adapter targets, not an algorithm name or normalized asset ID
alone. The target manifest can be produced without context, so authors can create
bindings without a circular requirement to supply context first.
