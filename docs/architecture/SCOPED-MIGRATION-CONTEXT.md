# Scoped migration context (task068)

The core `migration::MigrationContext` implements the typed claim/binding part of
the [adopted contract](MIGRATION-ASSESSMENT-V1.md), separately from Context v1.
The [new schema](../../schemas/migration-context-v1.schema.json) describes closed
records; duplicate/byte/depth/count/freshness/content/reference checks also run in
the reader. It reuses067 bounded JSON and file reading, content hashes, target
identity and existing domain-separated ID helpers. No provider/network code runs.

Parsing requires the format/version, selected immutable input, organization alias,
systems and explicit target bindings. Null is rejected before serde Option can
treat it as absence. Generic known/unknown claim wrappers retain complete typed
values and provenance. Every declared claim is validated even if not selected by
a later branch. Exact SHA-256 binds raw bytes before time normalization.

Binding checks format/digest, actual target membership, unambiguous target identity
and one owner per target; missing IDs are errors, not skipped rows. Unbound targets
receive no default claims. Organization → system → target precedence replaces a
whole record; support entries inherit individually. Explicit unknown can suppress
an inherited known claim without becoming false. Claim IDs bind context digest,
fixed JSON pointer and field. A shared default claim can be referenced by several
targets without sharing system-specific overrides or readiness.

Freshness uses asserted_at <= as_of < valid_until. Missing expiry is unknown,
future assertion is not_yet_effective, expiry is stale. Actor/source/interval and
optional reference stay attached to every resolved claim. External review is still
an unverified claim. Whole-second clocks normalize to UTC, reject invalid/leap/
overflow dates and cannot precede the exact collection instant (including067's
fractional captured times). Known incompatible use_case marks a semantic purpose
conflict; it does not rewrite a KEM as a signature or resolve generic RSA to hashing.

Reference IDs resolve within this one context. Optional HTTPS links are bounded,
reject credentials/query/control characters and are never opened. Ordinary input
strings remain subject to the shared1024-byte bound; aliases64 and references128
use their adopted ASCII vocabularies. The closed context schema has a shallower
maximum valid shape than the generic JSON depth/field ceiling: shared067 tests
establish below/at/above parser limits on valid extension-tolerant Native input,
and068 verifies that the same preflight rejects context overflow before typed
parsing. No impossible deep closed-schema context is fabricated as valid.

`BoundContext` exposes only resolved typed claim records, references, explicit
ownership and conflicts. This is not a business-priority evaluator or proof that
an operator mapped the real system correctly. Task069 consumes it for pure policy
and saved-evidence reassessment; task070 renders those results. Existing policy,
rules, native IDs/schemas and scan-wide context semantics are untouched.


Task073 adds a shared typed generator and canonical complete export, using the same
ClaimSet/Provenance validator and final context parser/bind. Explicit target indices
resolve only within the held SavedEvidence; duplicate/ambiguous/unselected targets
never receive implicit claims. Aggregate cloning is bounded before serialization;
canonical output omits absent Option fields rather than emitting forbidden null.
No storage schema, inheritance/evaluator, existing reader or default scan behavior
changes. The concrete opt-in CLI D4 supplement is in task073's plan.


The074 integration follow-up adds ClaimValue at raw CLI/HTTP answer boundaries,
reusing the original duplicate-aware bounded serde seed before Value conversion.
Complete export uses pretty serialization when it fits, otherwise bounded compact
serialization; absent optional empty references need not be added. Supported data is
not dropped to fit. Readiness question hints reuse the evaluator's existing support
requirements; they do not create claims, freshness or decisions. Core semantics and
stored schemas remain unchanged; original conformance cases remain required.
