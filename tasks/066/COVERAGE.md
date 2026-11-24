# Prerequisite requirement and manual-case reconciliation

This table covers task066's supporting specification obligations, not product
acceptance owned by tasks067-072. Contract =
[adopted saved-evidence proposal](../../docs/architecture/MIGRATION-ASSESSMENT-V1.md).
All A-groups/variants are in [ACCEPTANCE-CASES.json](ACCEPTANCE-CASES.json).

| Requirement | Specification/evidence | Manual positive/negative obligation checked |
|---|---|---|
| R2-01 | FACT-SOURCE-MAPPING F01-F05/F11; contract technical dispatch | A01-A04/A07, draft/prehash/purpose conflict; no KEM/signature substitution |
| R2-02 | F06-F10/F12; separate component target roles | A05/A10/A16, numeric ID mismatch, valid partial/missing group, advertisement versus selection |
| R2-03 | FEASIBILITY field/gap table; F12; observation/claim shapes | A01/A04/A06/A11/A15/A16, declared/resolved and safe partial facts; no execution/trust promotion |
| R2-04 | Four actually selected formats; current schema/envelope fields and strict additional semantic validators | A05/A11/A16/A18; unknown format, contradictory success, valid partial, no provider/network dependency |
| R2-05 | Frozen old catalog/profile versus new dispatch; F01/F05/F14 | A17 plus unsupported policy/catalog versions; old values/goldens are downstream implementation obligations |
| R3-01 | Contract P1-P8 and claim/field table; actual D2 authority | A02/A03/A08/A12/A13; same RSA with separate scope yields urgent versus plan |
| R3-02 | Separate axes, false-before-unknown readiness, action precedence | A01/A07/A09/A11/A18-N/A; known urgency/blocker survives missing inputs |
| R3-03 | Explicit clock and D<=as_of+L; F13, distinct lifetimes | A03/A08/A15/A18-before-collection and overflow; no quantum-arrival date |
| R3-04 | Fixed reasons, target-scoped claims/candidates/actions and exact source refs | A01/A02/A05/A09/A10; purpose mismatch and unknown readiness have deterministic actions |
| R3-05 | Immutable bytes/target/context/time/version bindings and deterministic ordering | A08 boundary variants/A13 repeat/version change/A09 false+missing; preserve explanatory differences |
| R4-01 | Exact closed record/claim shapes; relevant prerequisite subsets | A02/A03/A07/A09, false/missing/null and no unnecessary confidentiality inputs for signing |
| R4-02 | Context-free manifest and exact content+target/system links | A12/A14, shared algorithm versus distinct observations, unknown/ambiguous target and reformatted input |
| R4-03 | Whole-claim precedence organization→system→target, one owner per target; purpose relevance | A12 override/duplicate owner/A07 purpose conflict; no cross-system or cross-purpose copying |
| R4-04 | Per-claim actor/source/asserted/expiry/reference and active interval | A15 before/at start/expiry, missing valid_until and self-labelled external review; no scanner verification |
| R4-05 | Reassess one immutable saved input under contexts A/B; safe input/output alias rejection | A13/A14-reformat/A18-no-network/input-output-alias; no collection or input mutation |
| R4-06 | Finite bytes/count/depth/string/output limits and fixed errors | A18 canary, duplicate key, null, over-byte/depth/time/binding failures; boundaries below |
| R5-01 | EVALUATION-PROTOCOL method screening and common facts/policy objectives | Decision tree baseline versus ordinal matrix; numeric/MCDA authority and missingness explicit, no arbitrary weights |
| R5-02 | Separate fact truth/conformance/usefulness, role/resource and lineage holdout plan | Actual maintainer local-policy decision recorded; no secured independent experts/holdout or fabricated blind authorship |
| X02 | Exact output shapes/content/target/context/version/clock refs; safe applied-claim records | A12-A14 plus output target/claim/candidate/source resolution; no schema implementation claimed |
| X03 | Adopted separate JSON and en/ja Markdown; new v0.3 HTML/serve baseline preserved | Matching meaning/escaping obligations for task070; browser evidence here tests imported v0.3 only |
| X04 | No-network/offline/default-static contract, safe output/input guards | A18 and strict saved-only dispatch; no probe/package support promotion |

## Hand-derived checks of the critical decision boundaries

- Same algorithm/different system: A12 shares an RSA algorithm asset but targets
  distinct observations. System A declares active critical signing (P4 urgent);
  system B declares noncritical signing and deadline none (P6 plan). Claim records
  are looked up by exact target→system mapping, not by the shared algorithm ID.
- Unknown versus false: missing peer support makes readiness unknown; active
  peer=false makes blocked. With runtime=false and peer missing, the known false
  wins for readiness, missing peer remains information=incomplete, urgency stays
  urgent, and the sole selected action is unblock. False never substitutes for
  applicability not-applicable or an unknown wrapper.
- Urgent but unready: A09's known quantum concern plus active critical signing
  matches P4. Peer=false changes readiness/action, not the matched urgent row.
  With unknown readiness, the action is investigate, including when urgency=plan.
- Time boundary: as_of2026-11-18T00:00:00Z plus86400 seconds equals
  2026-11-19T00:00:00Z. That deadline and one second earlier are urgent; one
  second later is outside deadline pressure and permits plan only with the other
  explicit nonurgent facts. Missing lead is not zero. Claims are active at
  asserted_at and stale at valid_until. A15's corrected asserted_at2026-11-16 is
  strictly earlier than its expiry2026-11-17; it is a valid stale claim.
- Failed/partial TLS: failed verification plus negotiated success is contradictory
  and rejects. Current runtime.rs sets independently verified negotiation before
  missing-group/attribute collection diagnostics can mark partial; those valid
  partial records retain known version/suite/group. Missing group stays unknown;
  partial certificate attributes do not erase a completed bounded KX role or
  establish overall completeness. No offline crypto re-verification is claimed.
- Context-free workflow: target IDs depend only on immutable input/role, so a
  first evaluation can emit them without context. A second context references
  that manifest and exact input digest. No decision/context result is an input to
  discovery of its own targets; the binding contract is not circular.

## Complete limit/error boundary obligations

For every inclusive maximum in the contract (Inventory/probe/context/output
bytes, depth/fields, assets/observations/targets/relations, systems/bindings,
ordinary strings/aliases/references, candidates/reasons/missing/actions), freeze
three generated test obligations: limit-1 accepted when otherwise valid, limit
accepted, limit+1 rejected with resource_limit. The generator must vary only that
boundary and preserve required structural facts. These are downstream executable
obligations, not claims that a new reader was run here. For migration lead,
0 and315576000 are valid; -1 and315576001 reject. Invalid UTC/overflow/intervals
reject; unknown/null/false/absence have the distinct semantics in the contract.

Reference failures cover missing/duplicate/extra target/claim/source/candidate IDs,
wrong-kind/ambiguous role paths and context/input substitution. Privacy controls
put synthetic canaries only in rejected/uncopied fields; legitimate operator
aliases remain disclosed metadata, not a claimed general secret detector.

## Completion meaning

Specification coverage, source verification and actual D1-D5 decisions establish
this prerequisite. They do not establish implemented readers/schemas/decisions,
interop, usefulness, calibration, future detection quality or product requirement
completion. D6 participants/independent data remain task071 dependencies; D7 score
adoption is later. No task067+ work is selected by this reconciliation.
