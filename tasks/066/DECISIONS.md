# Task 066 maintainer decision package

Status: **recommended D1-D5 package adopted by the maintainer on 2026-11-18**. Proposed against integrated
HEAD48276e48f5c6d4eadafb2a3aa9d4c1a54808f1ab. A proposal, task-selection request
or independent AI review is not an adoption. The actual response, scope, date and
final artifact hashes must be recorded here before task066 can complete.

## Concrete recommended package

| Decision | Recommended choice | Reason and compatibility/security effect | Status |
|---|---|---|---|---|
| D1 | Separate opt-in `assess` path, new `migration-context-v1` and `migration.assessment.v1.json`; preserve all stored input formats/v1 policy. Exact byte-SHA256 input/context bindings; context-free target manifest; explicit per-target/system bindings. Adopt the finite proposed limits in the contract | Existing schemas already provide sufficient facts; no integrated-inventory migration or new crate. Content-bound targets prevent cross-input/system leakage, but do not independently verify the business mapping or saved collector claims | adopted 2026-11-18 |
| D2 | Adopt the P1-P8 local advisory table and fixed urgency/information/readiness/action axes, explicit provenance/claim validity intervals and clock. Known concerns survive unknowns; readiness never reduces urgency; no quantum-arrival input or invented universal deadline | Defines reproducible policy intent without claiming empirical business truth. Critical declared use/protection/deadline conditions can require urgent investigation; stale due claims require refresh. Missing unrelated fields do not suppress a supported branch | adopted 2026-11-18 |
| D3 | Read saved Inventory, TLS, SSH offers and SSH KEX v1 offline, with separate assurance gates. Unknown formats/versions reject | Both SSH formats already exist and share the envelope. Adds two detail validators/case groups, not transports or publication support. Offers remain unauthenticated; KEX/pin/user-auth distinctions remain exact. Alternative minimum is Inventory+TLS with explicit errors for both SSH inputs | adopted 2026-11-18 |
| D4 | New `assess` command writes the separate structured report and selected-language Markdown (en/ja); no new HTML/serve integration | Reuses safe output and writers. Preserves newly merged v0.3 HTML themes/languages/navigation/rule companion and local web UI. Report/CLI names become explicit only on approval | adopted 2026-11-18 |
| D5 | Preserve every executable rule value/tag,0.45, legacy IDs and v1 outputs. Add only the new-dispatch finite fact mappings in FACT-SOURCE-MAPPING.md, with exact final protocol groups/components/assurance and unknown encoding limits. No repair of the four retained exclusion false positives is selected | Reuse existing pure assessment; a new mapping cannot silently mutate shared v1 behavior. Task065 audit annotation/path gaps are already covered in the supplement. New storage/crypto semantics require this approval before downstream coding | adopted 2026-11-18 |

The [full contract](../../docs/architecture/MIGRATION-ASSESSMENT-V1.md) is the
reviewable field/limit/identity/error/report proposal. The
[feasibility comparison](FEASIBILITY.md), [source mapping](FACT-SOURCE-MAPPING.md),
[acceptance cases](ACCEPTANCE-CASES.json) and
[evaluation protocol](EVALUATION-PROTOCOL.md) are part of the package, not optional
omitted acceptance. Approving the recommended package includes its branch truth
as local policy and preregistration design; it does not approve product
implementation results, participant availability, external contact, a numeric
product score or deployment.

## D6/D7 boundaries

D6 protocol proposal: M1 decision table versus M2 ordinal matrix; screen numerical
and MCDA alternatives without arbitrary weights. Planned bounded study24 vignettes
(12 pairs), two practitioners rating both methods, independent technical
adjudication and policy authority, declared lineage/exposure,8 tuning and16 held-
out cases, at most90 minutes per practitioner plus30 minutes discussion. These
are planning/resource requests, not booked people or a hard universal quality gate.
No resources have been secured. Task071 must obtain actual authority/availability,
case provenance and frozen independent data before observations; missing resources
keep its affected evaluation unfinished. No external outreach is authorized.

D7 remains later: withhold any product numerical score until an observed valid
comparison supports a separately approved formula/scale/model/version and
compatibility plan. Task066 does not adopt or validate a numerical product score.

## Actual maintainer response

The maintainer answered the three explicit questions:

1. D1/D2/D4/D5 package: “推奨案を採用する”.
2. D3: “Inventory・TLS・SSH提示・SSH鍵交換の4形式”.
3. Evaluation planning: “この評価計画を採用する”.

The exact scope and exclusions are retained in
[evidence/MAINTAINER-DECISIONS.json](evidence/MAINTAINER-DECISIONS.json).
This records actual policy/storage/scope approval, not inferred approval from task
selection. The approved resource plan still has no secured practitioners and does
not authorize outreach/spending. D7 adoption remains later. Human review is
separate from both independent executable review stages. Corrections that only
clarify the accepted semantics will be recorded; a material change to these
choices requires another actual decision.

The binding sources are task066 acceptance (actual D1-D5 decisions, unresolved
blocking decisions prevent completion), roadmap D1-D7 and AGENTS human gates for
storage/high-risk architecture/rule/unspecified cryptographic changes. No proposal
text or test success grants implicit approval.

## Review clarifications within the adopted choices

The implementation reviewer identified three P2 specification defects after the
package was adopted: valid collector TLS partial results were conflated with
verification failure; A15 had an invalid zero-length claim interval; and mixed
false/missing readiness plus unknown-readiness action lacked unique precedence.
The corrections preserve the adopted semantics/scope: retain independently
verified reported TLS facts despite partial collection, use a valid stale interval,
and preserve known blockers with deterministic actions. No new storage format,
rule behavior, policy objective or input scope is adopted by those corrections.
The same reviewer must confirm the corrected final bytes.
