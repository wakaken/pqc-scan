# Task074 plan and concrete D4 supplement

Selected by the maintainer2026-11-18, dependent on completed070/073. Planning may
precede073 closure; executable changes start after073 completion. Original U03-U07
remain intact.071/072 stay byte-for-byte unchanged;075 final acceptance depends on them.
Main agent owns design/code/tests/docs; reuse the same two reviewer identities.

## Purpose and choice

Extend the existing local serve page, rather than creating an application or
browser evaluator. Use073 generation/complete import-export,069 pure evaluation,
070 styled bilingual report writers and existing safe_output. Optional personal/
organization presentation changes hints only, never criticality/policy. Unknown,
omitted, false and scoped N/A remain distinct. No company, department, deadline,
expiry, known criticality, deployment support or verification is invented.

D4 entry: serve --migration explicitly enables the optional workflow; default false
retains normal scan options/output and automatic scan-report navigation. Selected
migration scans additionally request scan.inventory.v1.json. Ready jobs provide
readable target selection and optional forms. Context-free evaluation is selectable.
The final result opens the existing real bilingual product HTML, with all structured
axes, evidence, influential/missing claims, blockers, reasons and next checks; no
JavaScript decision logic or duplicate report engine. Ordinary report links remain.

The form exposes purpose/use/applicability, optional business_critical with its actual
business semantics, relevant confidentiality/signature lifetimes and integration
support; no personal-mode low-priority preset. Actor/assertion are required for
answers, optional expiry is never inferred. More complex inherited/reference-backed
context is imported and previewed losslessly, with unsupported form-edit fields
explicitly read-only. Imported context cannot silently project into simpler fields;
starting a new form explicitly discards that draft and requires new scope review.

## Local API, bounds and safety

All routes retain exact Host, random session prefix, same-origin POST and existing
cross-site/navigation checks. Migration routes are unavailable without --migration.
No user-controlled path or command, outbound operation, project execution, alternate
schema or storage model. One synchronous bounded preview/assessment at a time follows
the existing HTTP coordinator; no extra worker or background evaluator.

- GET api/config exposes enabled flag; migration.js is loaded only when selected.
- GET api/migration/targets?job=ID&page=N lists100 rows per page from a ready session
  job's fixed scan.inventory.v1.json, returning exact digest and one-based indices.
  At most65536 technical targets; selection cap4096, ambiguous rows disabled.
- POST api/migration/import accepts explicitly selected raw Inventory v1 JSON, at most
  32MiB, stores exact validated bytes in a new private session job (same20-job limit).
  Files are selected in-browser; no arbitrary filesystem path/read API is added.
- POST api/migration/preview?job=ID accepts a closed JSON envelope (at most3MiB) with
  input_sha256, whole-second as_of, explicit selected indices/aliases and known or
  unknown answers/provenance, or an imported context string, or explicit no-context.
  Imported context is limited to1MiB, fully validated/bound and canonically exported
  with all supported fields. Mixed form/import/no-context modes reject.
- A successful preview retains only one bounded draft per job and returns a fresh
  opaque token, exact scope and downloadable context. An altered input, job, answer,
  scope or time requires another preview. Confirm never silently accepts stale IDs.
- POST api/migration/assess?job=ID confirms the current token, rereads and checks exact
  input digest, evaluates without rescanning, and writes the existing3 bounded reports
  plus optional migration-context.json in that job's migration subdirectory. No new
  public report schema or policy. GET allowlist serves only those completed artifacts.
- POST api/migration/cancel?job=ID revokes the matching draft token before confirmation.
  Confirmed synchronous evaluation is not cancellable mid-write; UI disables cancel
  and reports progress. Failures are explicit, each file replacement remains atomic,
  and no multi-file transaction or cancellation guarantee is invented.

Session limit20 jobs includes imports; at most20 drafts of1MiB each. Generation uses
existing256-system/4096-binding limits. Migration JSON responses are capped16MiB;
reports remain64MiB per file. No external assets. Reject wrong content type, missing
or oversized declared bodies and malformed/duplicate/unknown envelope fields before
use; bounded readers enforce actual lengths. Static source/input files are never
served outside the fixed ready-job artifact allowlist. A replaced input invalidates
preview and selection. No ambient job/target binding is allowed.

## Steps, verification and completion

Implement the opt-in controller, shared helper calls and fixed artifact routing;
add bilingual accessible selector/forms/provenance preview, safe downloads/import,
reset/error/progress/cancel states and final report navigation. Meaningful actual
HTTP/CLI tests cover guards, bounds, ready/failed/jobs, no-rescan equality, defaults,
imports/lossless roundtrip, stale/wrong input/target, false/unknown/omitted, aliases
and injection/canaries. Preserve and extend existing serve integration controls.
Run required Rust/full/feature checks and both reviewer stages. Real loopback browser
observations cover EN/JA desktop/narrow, focus/errors, optional personal and grouped
organization paths, context import/export, immutable substitution, report next checks,
language and loaded-offline native report operations. Preserve visible final report.
Source inspection alone cannot close U06. Main agent records final-source evidence.

No high-risk architecture or stored-contract change is selected: this is an opt-in
bounded adapter around existing session/validation/assessment/writer components.
If implementation requires either, stop affected work with a concrete proposal.


Implementation-review supplement: target metadata includes support-field hints derived
by the existing Rust prerequisite function for each supported intended-purpose label.
The browser only selects/merges those hints and never mirrors policy. Large supported
imports use a lossless compact fallback within the same1MiB contract when pretty output
would overflow. Raw authoring values use the existing bounded duplicate-aware JSON seed
before serde Value conversion; this corrects073's CLI boundary as an integration follow-up.
No schema, decision-table meaning or original acceptance condition changes.


Browser QA corrections: unique preview-content DOM id; explicit provenance/scope/time
errors; dynamic messages and selected-job labels follow language changes; canonical
role labels come from070 shared terminology. Input replacement clears all declarative
state and imported context, not just target numbers. Network failures identify the
local server rather than blaming declarations. Final browser verification must bind
these changes, not preliminary screenshots from the superseded server binary.
