# Read saved cryptographic evidence

[日本語](SAVED-EVIDENCE.ja.md)

Reassess saved findings with reusable purpose and usage input. `assess` runs
offline without rescanning source or contacting endpoints.

| Workflow | Start here |
|---|---|
| Browser | Follow the [illustrated scan and migration steps](SCANNING.md#web-migration-walkthrough), then [save and reload input](#browser-save) |
| CLI | [List targets and generate context](#context-template), then [reassess and open the result](#reassess) |

<a id="browser-save"></a>

## Saving and reassessing in the browser

1. Choose **Save input (JSON)**, check retrieval status and confirm `migration-context.json` in the browser's download location. It is separate from assessment-data JSON.
2. Reload this context with the same original findings for reassessment.
3. Normally use the current time when preparing items. Choose a specific time for a past situation; the page warns that later assertions will not apply then.

Omitting expiry leaves validity unknown. See [time and validity](#claim-validity).

<a id="context-template"></a>

## Generate context without copying target IDs

### 1. List targets and create context

Use noninteractive `context-template` to inspect target numbers, algorithms, roles
and locations, then select the numbers to bind. Replace paths and timestamps below.
`YYYY-MM-DDTHH:MM:SS+09:00` is a placeholder, not an executable value. Evaluation
must be at or after collection; assertion time is when information was confirmed.
Include seconds and an explicit timezone offset.

```console
pqc-scan context-template saved.inventory.v1.json --input-format inventory-v1 --list-targets
pqc-scan context-template saved.inventory.v1.json --input-format inventory-v1 \
  --target 1 --out migration-context.json --as-of 'YYYY-MM-DDTHH:MM:SS+09:00' \
  --set 'use_case="signature"' --set 'usage_state="active"' \
  --set 'business_critical=false' --unknown confidential \
  --actor owner --asserted-at 'YYYY-MM-DDTHH:MM:SS+09:00'
```

| Setting | Control or condition |
|---|---|
| Targets | Select displayed numbers with `--target`; repeat for a group. Numbers apply only to this exact saved input |
| Hash and target IDs | Calculated by the command. Ambiguous targets, duplicates, invalid numbers and existing context bound to other bytes are rejected |
| No selected target | Writes a valid unbound template and refuses claims; never selects every target implicitly |
| Organization/system aliases | Defaults `organization` and `system-1` need no company/department. Customize with `--organization` and `--system` |

### 2. Supply the required answers

| Option | Input |
|---|---|
| `--set FIELD=JSON_VALUE` | Use defined fields, such as `support.peer` for prerequisites. Strings require JSON quotes; booleans and seconds use JSON types |
| `--unknown FIELD` | Explicit uncertainty; omission means no answer |
| `--actor` / `--asserted-at` | Answers require an actor and assertion time |
| `--valid-until` | Optional expiry, never generated automatically; see [validity conditions](#claim-validity) |

Enter reference-backed deadlines through the local form. Personal use does not
change criticality. See [claim binding](#claims) and its JSON schema for inheritance
and advanced settings, then use the generated context with `assess` below.

<a id="reassess"></a>

## Reassess saved evidence offline

### 3. Reassess and open the result

Supply saved `saved.inventory.v1.json` and matching `migration-context.json`.
Replace evaluation time with the actual time to review, at or after collection.

```console
pqc-scan assess saved.inventory.v1.json --input-format inventory-v1 \
  --as-of 'YYYY-MM-DDTHH:MM:SS+09:00' --context migration-context.json \
  --out-dir ./migration-report
```

1. Open `migration.assessment.html` in a browser. It is direct product output; no Markdown conversion or preview server is required.
2. Read actions and conditions for proceeding. Use **View detection locations** to search and page through the evidence when needed.
3. Retain the original input and context. If input changes, including regeneration or reformatting, review the target list and create matching context again. Bindings cannot be carried over to another scan.

| Input/output | Meaning |
|---|---|
| `--input-format` | Explicitly choose `inventory-v1`, `tls-v1`, `ssh-offers-v1` or `ssh-kex-v1` |
| `--context` | Optional. Without it, technical concerns remain, but business applicability, urgency and readiness may be unknown |
| HTML | `migration.assessment.html`, using scan-report styles and language/Light/Dark controls |
| Markdown/JSON | `migration.assessment.md` and `migration.assessment.v1.json`, separate from existing scan reports |
| `--lang` | `en` (default) or `ja` sets initial HTML and Markdown language. JSON, IDs, facts and decisions are unchanged |
| `--report-label` | Sets the HTML display name only; does not change saved assessment JSON |

The [report guide](REPORTS.md#saved-assessment) covers workflow conditions,
schedules, urgency and saving. Cards group compatible decisions while preserving
each target's uncertainty, limits and provenance. **Details** shows rules, methods,
reasons and declarations used by the decision; no separate declaration/source
ledger is appended.

### Viewing and saving limits

| Condition | Behavior |
|---|---|
| Offline | Standalone HTML; no external assets required |
| Without JavaScript | First 20 work items only. Use JavaScript, Markdown or JSON for all records |
| Print | Visible items only. Original assessment data for all records can also be saved from the page |
| **Unidentified** | The saved record does not identify the algorithm |

### Interpreting the assessment

| Dimension | Values and meaning |
|---|---|
| Urgency | `urgent`, `plan`, `monitor`, `undetermined`, `not_applicable` |
| Information | `sufficient`, `incomplete`, `conflicting` |
| Readiness | `ready`, `blocked`, `unknown`, `not_applicable`. Ready means required support claims are active and true, not that implementation was verified |
| Next action | Urgency can coexist with blocked prerequisites or investigation. Actions link candidates, required support, applied claims, missing input and validation steps |

Test-only usage alone does not lower urgent known classical concerns. A valid
explicit outside-scope declaration takes precedence while preserving facts and
reassessment conditions. `monitor` applies only to the bounded role and facts;
`not_applicable` is limited to the operator's declared scope.

Candidates need exact parameter/protocol choices and separate interoperability
and deployment validation. Reports retain collection/evaluation times, input/context
digests, claim provenance, reasons, sources and coverage limits. Only applied
claims are copied; operator references use the unverified `context:` namespace.
There is no aggregate safety verdict, numerical migration score or quantum-arrival prediction.

### Selected migration configuration

| Optional claim | Storage and assessment |
|---|---|
| `migration_target` | Saves purpose, algorithm, key-size/configuration and provenance through the local form or `context-template --set` |
| Support binding | Tied to configuration in the same claim scope. Inherited known answers from another scope cannot be rebound. Inactive configuration or use cannot establish configuration-specific support |
| Configuration checks | Family/purpose combinations are validated; custom cryptographic validity and deployment compatibility are not. Existing files without this field retain generic support semantics |
| `migration_goal_month` | Provenanced goal month (`YYYY-MM`, years 0001–9999) or explicit unknown. Does not become an instant, replace `deadline`, change decisions or alter the meaning of inputs lacking the field |
| Calendar durations in the form | Converted to exact `migration_lead_seconds` and not reinterpreted on import |

See [optional migration context and reassessment](SCANNING.md#optional-migration-context-and-reassessment) for form operations.

### When a record cannot be selected

Records that cannot receive use declarations are disabled with an explanation.
Ordinary answers cannot resolve an unidentified record; inspect its original
location and explanation. Reference-only records may accept a purpose while the
algorithm stays unknown. Entering a purpose does not identify the algorithm.

## What the targets mean

A target identifies an observation and technical role in the exact saved bytes.
Observations sharing an algorithm definition are not merged. Changing any byte,
including JSON whitespace, changes the input digest and target IDs; native
run/observation/asset IDs stay unchanged. A digest establishes content identity,
not collector authenticity.

Repeated source references can receive declarations when saved positions and
records are uniquely distinguishable. Missing/colliding locations remain
unbindable; cross-scan ambiguity flags are preserved.

| Input | Meaning of saved evidence |
|---|---|
| Source, certificates and dependencies | Static references remain lexical/syntactic; certificates retain parsed attributes. Dependency declaration/resolution is distinct from provider activation or deployment. Unknown identifiers and parameter encodings are not invented |
| TLS | Negotiated key establishment, certificate signatures and subject keys are separate targets. Verified session means the saved collector reported a full verified handshake with matching peer binding. Valid partial records retain known negotiation despite incomplete certificate collection/group data; contradictory success/verification/handshake claims reject. Missing CertificateVerify schemes and ASN.1 encodings are not invented |
| SSH offers | Ordered unauthenticated advertisements. Extension markers are excluded from cryptographic targets; saved candidate classifications are not technical authority |
| SSH KEX | Requires recorded supported host-key pinning, completed handshake and no attempted user authentication. Host signatures and key establishment stay separate |

Hybrid components retain exact final/draft identities and the stated combiner
scope. A PQC component does not establish whole-system safety.

<a id="claims"></a>

## Bind organization and system claims

Context JSON follows the [`pqc-scan.migration-context` v1 schema](../../schemas/migration-context-v1.schema.json).
Normally generate it with `context-template`; check the following for manual authoring.

| Item | Condition |
|---|---|
| Binding | Read the target manifest and explicitly map input format, exact SHA-256 and target IDs to declared system aliases, using the same saved input and evaluation time |
| Rejected bindings | Another file, reformatted bytes, unknown/duplicate targets, indistinguishable source locations or unknown systems |
| Claim fields | Applicability, purpose/use, criticality, confidentiality, protection/signature-verification lifetimes, historical ciphertext, migration lead time, sourced deadline and integration support |
| Claim representation | Typed known value or explicit unknown, with actor, source kind, assertion time and optional expiry/reference per claim. No credentials, key material or free-text justification required |
| Defaults and overrides | Explicitly bound targets only, in organization → system → target order. Omission inherits, explicit unknown replaces, `false` stays false. Support prerequisites inherit independently |
| Conflicts and sharing | Conflicting purpose preserves technical facts and the claim instead of rewriting the primitive. Claims do not transfer through shared algorithm assets |

The mapping remains an operator declaration about a business system, not verified
use. Reference links are stored as claims and never fetched.

<a id="claim-validity"></a>

### Time and validity

Claims are active from assertion time inclusive until expiry exclusive. Evaluation
must be at or after assertion and before expiry. Future, expired and expiry-unknown
claims remain visible but cannot establish an active favorable claim. An external-review
label is still a claim, not scanner verification.

New assertion/evaluation times require seconds and an explicit offset and are
normalized to UTC. Evaluation before collection is rejected.

## Input limits and errors

| Item | Limit or behavior |
|---|---|
| Inventory input | 32 MiB |
| Each TLS/SSH input format | 1 MiB |
| Context JSON | 1 MiB for reading and generation; use defined fields and valid times |
| Each assessment report | 64 MiB; overflow rejects without truncation |
| Destination | Cannot be the same file as input/context. `context-template` rejects input aliases, hardlinks, linked destinations, bad values/provenance and overflow before replacement. Existing output files are replaced individually |
| Exit 0 | Successful assessment, including urgent or incomplete results |
| Exit 1 | Input/context/time/output error, using fixed errors without echoing rejected values |

Invalid/unsupported formats or versions, invalid references, contradictory facts
and oversized input are rejected. Typed output omits raw evidence, arbitrary
metadata and unknown names, but source paths, public fingerprints and identifiers
can remain sensitive operational metadata. Even probe-enabled builds never
execute input, fetch references, rescan source or contact endpoints. See the
[probe guide](PROBING.md) for TLS/SSH collection and distributions.
