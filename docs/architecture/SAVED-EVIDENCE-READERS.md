# Saved-evidence technical readers (task067)

Implementation of the technical/ingestion portion of the
[adopted migration contract](MIGRATION-ASSESSMENT-V1.md). The new core `migration`
module has no dependency on protocol crates. `SavedEvidence` holds a validated
immutable input reference and a private target vector. Public read-only accessors
expose the context-free manifest; serialization of target records is not a second
saved inventory format. Scoped context is implemented by
[task068](SCOPED-MIGRATION-CONTEXT.md); policy, CLI and human rendering are later tasks.

`bounded` rejects oversized input before parsing, limits JSON containers/strings,
detects duplicate keys recursively and consumes exactly one JSON value. Local
reads retain an opened regular file for subsequent output identity guards.
Existing collection timestamps can contain fractions; their original text stays
unchanged. Whole-second evaluation clocks compare against the collection instant
rounded upward, avoiding acceptance of a clock before fractional collection.

`native` reuses NativeScan reference validation and the existing embedded schema's
known shape. Its private validator removes closed-extra-field constraints to
preserve the published extension-tolerant reader contract; the schema file is
unchanged. Unknown kinds/vocabulary/attributes never supply invented facts. Role
resolution follows only the explicit algorithm/public-key/certificate paths,
never arbitrary graph traversal. Targets retain observation/root/role references;
ambiguous anchors or role relations cannot receive inferred business bindings.

`runtime` validates the original embedded TLS/SSH schemas and semantic constraints:
peer/answer/pin binding, traffic caps, outcome/diagnostics, exact registry pairs,
selected/offered consistency and protocol-specific assurance gates. Valid TLS
partial results preserve independently established reported negotiation. Failed
verification cannot carry negotiated success. Certificate attributes remain
parse-only with unknown encoding assurance. SSH offer classification is recomputed
from finite names; no first-common or stored-label selection is inferred.

JSON Schema uses the same pinned0.29.1 dependency already in workspace tests, with
default features disabled. Only repository-constant schemas/local references are
compiled and cached. Untrusted input cannot select a schema or enable URI retrieval.
This is shape validation, not trust or implementation validation.

`technical` calls the original pure primitive evaluator, supplements only adopted
new-dispatch DSA/DH parameters and exact TLS/SSH protocol/component facts, and
exposes source edition/clause/limit records. `migration-facts-v1`/1.0.0 is explicit;
unknown catalog versions reject. `local-assessment-1`, `migration-review-v1`,
native schemas/IDs, rules, Risk/confidence and output contracts remain unchanged.

Task067 evidence and row reconciliation live in
[its result record](../../tasks/067/TASK-067-RESULTS.md). Frozen technical examples
are source-exposed conformance cases, not independent practitioner data.
