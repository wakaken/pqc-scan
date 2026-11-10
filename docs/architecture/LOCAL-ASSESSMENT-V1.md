# Local assessment and policy v1 (task 026)

`--inventory-json --policy-profile migration-review-v1` writes the separate
`scan.assessment.v1.json`. `--assessment-context PATH` optionally supplies an offline
JSON context and requires the profile. No flags means no evaluation or extra
artifact. The existing six reports, diagnostic/inventory schemas, finding IDs,
`Risk`, severity, RecommendedAction and exit precedence remain unchanged. Inventory
v1 still has `assessment_status=not_evaluated` and empty `assessments`: its strict
published schema cannot be populated in place. The separate
[assessment schema](../../schemas/assessment-v1.schema.json) is original MIT project
material. Report records reference the same inventory scan/assets/observations.

## Applicability and decisions

The versioned local profile `migration-review-v1` / `1.0.0` is an **advisory review
of primitive/candidate suitability**, not regulation, deployment readiness, a
security audit or product-wide PQC certification. It has no universal deadlines or
effective-date gate. Source editions are embedded. It does not download rules or
call an approval service. Policy Fail is recorded in JSON and does not add an exit
code or alter `--fail-on`/`--require-complete` behavior.

| Condition, in order | Decision |
|---|---|
| Operator explicitly excludes the local profile | NotApplicable; exclusion retained |
| Profile applicability is unknown | Indeterminate |
| An applicable known quantum/classical concern is present | Fail; incomplete-scope reasons remain |
| Facts, parameters, purpose or support are unresolved | Indeterminate |
| Bounded primitive and declared support checks succeed | Per-asset Pass only |

Aggregate Pass additionally requires positive relevant results and complete
inspection/inventory. **Current inventory v1 is always not exhaustive, so this
implementation cannot report aggregate Pass**, including on empty scans. Fail
plus unknown remains Fail with uncertainty, rather than losing the known issue.
Usage/dependency records are NotApplicable to primitive policy, but related
algorithm/public-key/certificate records are evaluated; certificate issues cannot
disappear through those NotApplicable entries.

## Independent assessment axes

`assessment.rs` produces pure catalog/asset assessments. `policy.rs` consumes
those results plus validated local context. Writers only serialize/validate.
Construction, standardization, quantum exposure, classical concerns, observation
assurance, usage, priority, prerequisites and final policy decisions are distinct.
Source heuristic confidence is never converted to a calibrated probability.
References/configuration do not prove active use. Trust/signatures/implementations
are not verified.

The finite catalog supports classical public-key families; exact ML-KEM/ML-DSA/
SLH-DSA parameter names and their pure NIST CSOR OIDs; draft names; one recognized
draft hybrid; AES-128/192/256 and SHA-1/SHA-256/384/512/MD5 mathematical models.
Catalog support does not expand rules or inventory normalization: e.g. source AES
and hash matches without normalized algorithm assets remain unassessed. Unknown
names/OIDs/future kinds remain unknown, and arbitrary inputs are not echoed.
HashML-DSA and other unlisted encodings are not silently equated with pure ML-DSA.
Namespace mismatch and unknown/uninterpreted parameters remain uncertain. Parameterless
PQC OIDs with a reported non-absent parameter encoding cannot be known encodings.

RSA/EC quantum exposure can coexist with small-key classical concerns. Local
reference thresholds are RSA 2048 / EC 224 bits, pinned to SP800-131A Rev2 for this
advisory profile, not a claim of current legal compliance or sufficient strength
for every use. Certificate signature and subject-key components are assessed
separately, including validity at the captured scan time; no new hidden clock.
MD5/SHA1 collision concerns are explicit; hash-only concerns need collision-sensitive
context, while a weak digest in a certificate signature is intrinsically signing.

Hybrids retain component assessments and an **unverified combiner** conclusion;
PQC component standardization never implies hybrid resistance. AES key search,
hash preimage and hash collision have distinct ideal-oracle query fractions.
Those fractions exclude memory/parallelism/circuit cost and implementation attacks;
they are neither operational security bits nor NIST security categories. Broken
classical hashes retain their separate collision issue. AES-128 is not declared
broken or mandated to be replaced by a public-key PQC primitive.

## Context, provenance and candidates

Context input is at most 16KiB, UTF-8, version 1, fixed enums and optional booleans;
unknown/duplicate fields and invalid encodings are rejected with fixed errors before
scanning/report creation. No arbitrary labels, endpoints or secret data are needed.
For example, an intentionally incomplete context is:

```json
{"version":1,"use_case":"jwt"}
```

`applicability` is `applicable` (default local advisory scope), `unknown`, or
`not_applicable`. `use_case` is `unknown`, `key_establishment`, `jwt`, `pki`,
`signature`, `encryption`, or `hashing`. Optional `long_lived_confidentiality` affects
review priority independently of severity; `hash_collision_required` scopes hash
collision concerns. `support` permits optional `runtime`, `provider`, `peer`,
`protocol`, `parameters`, `jose`, `verifiers`, `pki` booleans. Missing is unknown;
false does not satisfy a requirement. These are **scan-wide operator claims for
all purpose-matching targets**, not verified capabilities. A heterogeneous rollout
that cannot make that claim must leave fields unknown. Claims for signing cannot
be transferred to a KEM. Dependency presence satisfies no prerequisite.

Candidates carry purpose, unverified-integration status, explicit parameter
selection, fixed validation steps, sources and prerequisites. KEMs serve
establishment, signatures serve signing, hashes remain hashes. Unknown primitives
receive no guessed replacement. RSA's dual possible purpose is conditioned on the
declared intended use; it does not infer actual use. RSA-PSS and its restricted
public-key OID are signature-only (RFC4055), so an establishment context cannot
turn them into KEM candidates. Generic RSA/EC intended-use resolution is limited
to signing or establishment, never hashing or symmetric encryption. A known KEM
in a certificate signature slot retains its establishment purpose and produces
a purpose-mismatch concern; the role does not rewrite the primitive. JWT additionally requires
JOSE identifier/encoding and verifier rollout claims; PKI requires certificate
binding and issuer/trust-store support. Primitive standardization alone is not
protocol/provider/peer availability, and no unverified hybrid PKI chain is mandated.

## Reproducibility and privacy

The evaluator uses captured inventory time, fixed catalog/profile versions and
strictly typed context. IDs use the existing length-framed SHA-256 helper with a
separate assessment domain; scan-local IDs and references are normalized together
in the checked-in full golden. Stable scoped asset IDs remain exact. Results
validate target/observation references against the inventory. Native v1 identities
and projection links are never regenerated.

Only fixed vocabulary, safe numeric facts and validated IDs enter the new report;
raw matches/snippets/private material/arbitrary metadata and invalid context values
are not retained. Existing output-directory files are not deleted. Legacy masking
and recommendations remain on their original path. Offline execution does not
mean operator claims, implementation safety or complete inspection were verified.
