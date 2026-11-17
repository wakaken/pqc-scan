# F054-01 bounded correction proposal

Status: proposal only; no rule, output contract, threshold or finding is changed.
Original requirement affected: Q3-02. Confirmed current counterexample: Cargo.lock
line2963, universal-hash0.6.1, DEP_RUST_RSA_CRATE. The unbounded Cargo package
substring matcher consumes the rsa letters within universal-hash and incorrectly
emits RSA/JWT and quantum-vulnerable facts. This does not establish an actual
cryptographic vulnerability in universal-hash.

## Concrete correction and compatibility review

Review an exact Cargo package-name matcher for the intended rsa and jsonwebtoken
packages. Parse or bound the package component of the Cargo purl instead of
matching rsa anywhere in the remaining text. Do not broaden the allowlist, suppress
this source/result, change severity or infer deployed features. Confirm the
intended policy for differently named wrappers before changing their detection.
Preserve the rule ID and published output schema; corrected findings and derived
IDs/counts can change and require an explicit compatibility impact assessment.

Required cases: rsa/jsonwebtoken exact names with supported declared/resolved
versions; universal-hash and misleading prefixed/suffixed names; mixed case under
the existing policy; version, qualifier, subpath, punctuation and malformed purl
boundaries; lock/manifest/SBOM representations consumed by this rule. Verify
retained declaration/resolution provenance, no enabled-state inference and the
actual end-to-end classification. Preserve this pre-fix result.

Run schema/unique-ID and positive/negative rule validation, applicable workspace
and original acceptance checks. Rust changes, if actually necessary, additionally
need their required Rust checks. Reuse the existing implementation and test
reviewer identities for both stages; a new reviewer is not authorized. Reconcile
final source, actual rule outputs, Q3-02 and affected quality evidence. Do not
update a policy hash or normalizer merely to hide this failure.

## Human-review boundary

AGENTS.md requires stopping affected work and obtaining human review for

> rule changes that could break compatibility;

Task054's approval_boundary also requires impact analysis and approval for
detection-rule changes. This assessment request authorizes recording the
proposal, not applying the rule change. A maintainer must review the exact
package-identity behavior and compatibility impact before implementation.
No scope waiver, new task or release authorization is requested or assumed here.

## Acceptance and sequencing

First finish the current negative assessment and its scoped reviews. Separately,
obtain the required correction approval and execute the checks above. Only the
verified corrected product can close Q3-02 and permit reconsidering complete
roadmap fulfilment. Task054 assessment completion, roadmap fulfilment, product
acceptance and publication remain separate decisions.
