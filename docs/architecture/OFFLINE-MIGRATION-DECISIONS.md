# Offline migration decisions

The adopted contract is [migration assessment v1](MIGRATION-ASSESSMENT-V1.md),
with frozen semantic expectations in tasks/066/ACCEPTANCE-CASES.json. Core
`migration::evaluate` consumes immutable SavedEvidence and optional scoped
MigrationContext plus an explicit clock. It performs no collection or network IO.
This document describes the implementation; it does not redefine the contract.

`migration/decision.rs` implements ordered P1-P8, independent urgency, information,
readiness and action. Branch-minimum claims are retained with per-claim provenance
and freshness. Relevant false support blocks even when other prerequisites are
missing. Conflict, stale claims needing refresh and unresolved use force
investigation without lowering known urgency. Only active claims establish
favorable readiness, applicability or ordinary planning. Explicit unknown,
omission, false and not applicable remain distinct.

Primitive facts come from the separate technical catalog; candidates reuse the
existing pure `policy::candidates` with effective purpose and new scoped IDs.
The legacy algorithm assessment, context, policy and six report formats are
unchanged. Support inheritance remains per field under the effective purpose;
JOSE/verifier and PKI prerequisites are added only where relevant. No candidate
or declaration establishes provider activation, verified rollout or whole-system
safety. Certificate validity, signature-verification lifetime and confidentiality
obligations are separate values. There is no inferred quantum date or score.

Root MigrationAssessment fields are private and serialize-only. `validate_for`
checks exact opened input/targets, bound claims, context identity, clock, selected
versions, cardinalities, scoped candidate prerequisites and source references.
The writer calls this guard without evaluating the policy. Embedded schema
`migration-assessment-v1.schema.json` independently specifies the public shape.
Sources include exact catalog edition/clause and operator references prefixed
`context:` to avoid catalog collisions. Local references are never fetched.

The thin CLI `assess` dispatch reads one explicit format and optional context,
evaluates once, and requests separate JSON, readable Markdown and self-contained bilingual HTML reports. Success is advisory exit0;
operational failure is1. Fixed errors exclude rejected content/path details.
New opened input/context handles and canonical names are checked against output
names and inode identities before writes. JSON serialization is bounded before
creating output and uses the existing safe_output writer. Each file is atomic;
a report set is not a filesystem transaction. Human writers share localized terminology and typed claim-value formatting.
HTML reuses existing scan-report CSS and the local language/theme controller,
with a target index, response cards, candidate support tables and expandable
technical evidence. Markdown uses a response overview and readable tables.
Exact machine records remain in the unchanged JSON companion, not raw dumps in
human views. All three representations render within64MiB/file and all destination
aliases validate before any write. JSON-only library output remains available.
The maintainer explicitly selected this HTML extension in the task070 follow-up.

Evidence and requirement reconciliation are in tasks/069. Specification agreement
is distinct from independent technical truth or practitioner usefulness (071).
