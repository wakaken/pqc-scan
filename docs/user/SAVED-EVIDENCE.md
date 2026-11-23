# Read saved cryptographic evidence

[日本語](SAVED-EVIDENCE.ja.md)

For the browser workflow, start with the [illustrated scan and migration steps](SCANNING.md#web-migration-walkthrough). This guide explains saved-input/context binding, declaration semantics and CLI/library contracts.

The core library's `migration::SavedEvidence` reads one explicitly selected saved
Inventory v1, TLS v1, SSH offers v1 or SSH KEX v1 file. `read` accepts a regular
local file; `from_bytes` accepts immutable bytes. These readers are available in
the default static build. They do not scan source, contact endpoints, load TLS/SSH
providers or repeat trust/signature verification.

The library and the explicit `assess` CLI support offline migration decisions
with scoped organization context. Existing scan commands and reports retain
their v1 behavior.

## What the targets mean

Each target identifies an observation and a fixed technical role in the exact
saved bytes. An algorithm definition shared by two observations does not merge
their targets. Changing a byte, including JSON whitespace, changes the input
digest and target IDs. Native run/observation/asset IDs remain unchanged.
Repeated source references can receive explicit declarations when their saved
positions and records are uniquely distinguishable inside this exact input.
Incomplete or colliding source locations remain unbindable; native cross-scan
ambiguity flags are preserved. The digest establishes content identity, not collector authenticity.

Static references retain lexical/syntactic assurance; parsed certificate facts
remain parsed attributes. Dependency declarations and resolution remain distinct
from provider activation or deployed cryptography. Unknown identifiers and
parameter encodings remain unknown.

TLS targets distinguish negotiated key establishment from certificate signatures
and subject keys. A verified-session label means the saved collector reported a
full verified session with matching peer binding. Valid partial records can keep
known negotiated facts while certificate collection or the group remains
incomplete. Contradictory handshake/verification/success claims are rejected.
Missing CertificateVerify schemes and ASN.1 parameter encodings are not invented.

SSH offers remain ordered unauthenticated advertisements; extension markers are
excluded from cryptographic targets. Saved candidate classifications are not
trusted as technical authority. SSH KEX requires the recorded supported host pin,
completed handshake and no user authentication. Host signatures and key
establishment remain separate. Hybrid components retain their exact final/draft
identities and a stated combiner scope; a PQC component is not whole-system safety.

## Input limits and errors

Inventory input is at most32 MiB; each runtime input is at most1 MiB. Readers
reject duplicate keys, malformed UTF-8/JSON, BOM, trailing JSON values, depth over16,
objects over64 fields and strings over1024 UTF-8 bytes. Inventory limits are16384
assets,32768 observations,65536 projected targets and64 relations/references per
record. Existing smaller protocol limits also apply. Native reader extensions
remain tolerated within those bounds; TLS/SSH unknown fields reject.

Unsupported formats/versions, invalid references, contradictory facts and resource
overflow produce fixed errors without echoing rejected input. Safe typed output
omits raw evidence, arbitrary metadata and unknown names. Existing source paths,
public fingerprints and identifiers can still be sensitive operational metadata.
Experimental TLS/SSH collection and publication status are unchanged.

## Bind organization and system claims

`migration::MigrationContext` accepts the separate
[`pqc-scan.migration-context` v1 schema](../../schemas/migration-context-v1.schema.json).
Read a technical manifest first, then explicitly bind its input format, exact
SHA-256 and target IDs to declared system aliases. `bind` requires that same
`SavedEvidence` and an explicit evaluation time. Another file, reformatted content,
unknown/duplicate target, source identity that cannot be distinguished within the
input, or unknown system is rejected.
The mapping remains an operator declaration about the real business system.

Claims may declare applicability, intended purpose/use, criticality,
confidentiality, protection/signature-verification lifetimes, historical
ciphertext, migration lead time, a sourced deadline and integration support.
Each claim is either known with a typed value or explicitly unknown, and carries
its own actor, source kind, assertion time and optional expiry/reference.
No credentials, key material or free-text justification are required.

Defaults apply only to explicitly bound targets, in organization → system → target
order. Omission inherits; explicit unknown replaces the inherited claim; false
remains false. Each support prerequisite inherits independently. A known purpose
conflict retains the technical fact and conflicting claim instead of rewriting
the primitive. Claims do not transfer through shared algorithm assets.

Claims are active from their assertion time, inclusive, until expiry, exclusive.
Future, expired or expiry-unknown claims remain visible and cannot establish an
active favorable claim. An external-review label remains a claim, not scanner
verification. All new claim/evaluation times require an offset and whole seconds;
the library normalizes them to UTC and rejects evaluation before collection.

Context is bounded to1 MiB,256 systems and4096 bindings. Organization/system/actor
aliases use lower-case ASCII identifiers up to64 bytes; reference identifiers are
visible ASCII up to128 bytes. Unknown/duplicate fields, null, wrong types and
invalid intervals/references reject with fixed errors. Local references are
declarations; optional HTTPS links reject credentials/query/control characters
and are never fetched. Context v1 for `migration-review-v1` remains separate.

## Reassess saved evidence offline

```console
pqc-scan assess saved.inventory.v1.json --input-format inventory-v1 \
  --as-of 2026-11-18T00:00:00Z --context migration-context.json \
  --out-dir ./migration-report
```

Choose `inventory-v1`, `tls-v1`, `ssh-offers-v1` or `ssh-kex-v1` explicitly.
`--context` is optional. Without it, technical concerns remain visible while
business applicability, urgency and deployment readiness can remain unknown.
To author bindings, inspect `migration::SavedEvidence::targets()` in the library,
or the context-free report's `targets` array, then bind those exact target IDs
and the `input.sha256`. Reformatting the input requires new bindings.

The command writes `migration.assessment.html`, `migration.assessment.md` and
`migration.assessment.v1.json`, separately from the existing scan reports.
Open `migration.assessment.html` in your browser. It is a real, self-contained
product output; no Markdown preview server or conversion is required. The page
uses the scan report's styles and local language/theme controls. `--lang en`
(default) or `--lang ja` chooses the HTML initial language and Markdown language;
JSON, IDs, facts and decisions are language-independent.

Migration cards group compatible decisions and show actions and conditions for proceeding.
Use **View detection locations** for the searchable, paged location list and
**Details** for matched rules, methods and reasons. Declarations used by the
decision appear within details; there is no separate paged declaration/source
ledger at the bottom. See the [report guide](REPORTS.md#saved-assessment) for
workflow conditions, schedules, urgency and saving.

Distinguish **test-only usage** from an explicit **outside-scope** declaration.
Test-only usage alone does not lower urgent known classical concerns. A valid
explicit scope exclusion takes precedence while retaining facts and reassessment
conditions.

Complete facts and decisions remain in a shared local HTML data archive, reducing
per-target/per-language duplication. No external assets are required. JavaScript
is needed to browse all records; without it, the first20 work items remain visible.
Printing covers the visible items. Use Markdown or JSON for the complete report;
the original assessment data can also be saved from the page. **Unidentified**
means the saved record does not identify the algorithm. Grouping preserves each
target's uncertainty, limitations and provenance.
The library entry is `migration::evaluate(&saved, context.as_ref(), as_of)`;
`evaluate_with_versions` selects explicit supported catalog, policy and model
versions and rejects unknown versions without falling back to legacy v1.
The output uses catalog `migration-facts-v1/1.0.0`, local advisory policy
`migration-decision-v1/1.0.0` and model `decision_table/1.0.0`.

Each target has independent urgency (`urgent`, `plan`, `monitor`, `undetermined`,
`not_applicable`), information (`sufficient`, `incomplete`, `conflicting`),
readiness (`ready`, `blocked`, `unknown`, `not_applicable`) and next action.
Ready means the relevant support claims are active and true; it does not verify
an implementation. An urgent result can remain blocked or require investigation.
Monitor applies only to the bounded role and facts. Not applicable is a scoped
operator declaration and preserves known technical concerns.

Actions link to proposed candidates, required support, applied claims, missing
inputs and validation steps. Candidates need exact parameter/protocol choices
and separate interoperability/deployment validation; they are not deployed
replacements. The report preserves collection/evaluation times, content/context
digests, provenance, fixed reasons, sources and coverage limits. References
provided by an operator use a `context:` namespace and remain unverified.
Only claims actually used by a decision are copied. There is no aggregate safety
verdict, numerical migration score or invented quantum-arrival date.

Successful advisory assessment exits0, including urgent/incomplete results.
Invalid input, context, time or output exits1 with fixed errors. Input and context
must not alias an output name/inode. Each report is bounded to64 MiB and written
using the existing safe output boundary; oversized reports reject without
truncation. Existing files are atomically replaced individually. The command
never executes input, fetches references, rescans source or contacts endpoints,
including builds with experimental probe features enabled.

## Generate context without copying target IDs

The explicit noninteractive `context-template` command lists readable target numbers:

```console
pqc-scan context-template saved.inventory.v1.json --input-format inventory-v1 --list-targets
pqc-scan context-template saved.inventory.v1.json --input-format inventory-v1 \
  --target 1 --out migration-context.json --as-of 2026-11-18T00:00:00Z \
  --set 'use_case="signature"' --set 'usage_state="active"' \
  --set 'business_critical=false' --unknown confidential \
  --actor owner --asserted-at 2026-11-17T00:00:00Z
```

Select only the displayed numbers you intend to bind; repeat `--target` for an
explicit group. The command calculates the exact input hash/IDs. Without selected
targets it writes a valid unbound template and refuses claims. It never selects all
targets implicitly. Ambiguous targets, duplicates and invalid numbers reject.
Default aliases `organization` and `system-1` need no company or department;
`--organization`/`--system` customize them. Numbers apply to this exact saved file;
review the list again if it changes. Existing context bound to other bytes rejects.

`--set FIELD=JSON_VALUE` uses adopted field names, with `support.peer` and other
support names for prerequisites. String values require JSON quotes; booleans and
seconds use JSON types. `--unknown FIELD` explicitly records uncertainty; omission
leaves no answer. Answers require an actor and assertion instant; optional
`--valid-until` is never generated. Without expiry the existing freshness rules
remain unknown. The local form can author reference-backed deadlines; advanced inheritance can be authored
in the adopted context document. References are explicit declarations, never invented by the simple answer builder.
No personal-use shortcut changes criticality. Use the generated file with `assess`.

Output uses the existing safe writer and1MiB context limit. Input aliases, hardlinks,
linked destinations, bad values/provenance and oversized output reject before
replacement. The shared library `generate_context`, `ContextSelection`, `ContextClaims`
and `ClaimAnswer` use the same reader/binding rules; `MigrationContext::to_bytes`
exports all supported fields with normalized times, preserving claims and references.
This is separate from legacy Context v1 and does not change existing scan defaults.


### Selected migration configuration

The optional `migration_target` claim saves purpose, algorithm, key-size/configuration
and provenance. Supply it through the local UI or `context-template --set`. The reader
validates the family/purpose combination, not the cryptographic validity of custom
settings or real deployment compatibility. Support is bound to the configuration in
the same claim scope; inherited known answers from another scope cannot be rebound.
Inactive configuration or use cannot establish configuration-specific known support.
Existing files without this field retain generic support semantics. See the
[local workflow](SCANNING.md#optional-migration-context-and-reassessment).

The optional provenanced `migration_goal_month` claim retains a planning goal at
month precision (`YYYY-MM`, years 0001–9999), or explicit unknown. It does not
convert to an instant, replace `deadline`, or affect policy decisions. Contexts
without it retain their existing meaning. The local UI converts calendar durations
to the existing exact `migration_lead_seconds` value; importing does not reinterpret
that value.

Reference-backed deadlines can also be authored through the local form. The reference-aware generation API accepts explicitly supplied references and validates them with the existing schema; old no-reference builder entrypoints retain their behavior. `assess --report-label` names only the HTML presentation and does not change saved machine decisions.

### Distinguishable repeated source records

Explicit target selection may bind separate source references with the same weak
matching anchor when their complete saved byte spans and line/column locations are
unique and their usage records are consistent. A malformed or unlocated neighbor
in that group prevents the exception. This only identifies a record in the held
file; it does not identify the same place in another scan or verify active use.
Fixed reference-only targets can receive a declared purpose while the algorithm
remains unknown. Unsupported algorithm components and unresolved component roles
stay ineligible. Regenerating or reformatting the input still requires new bindings.
