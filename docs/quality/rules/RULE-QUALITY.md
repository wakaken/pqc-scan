# Built-in rule quality companion (task028)

[Machine-readable review](RULE-QUALITY-METADATA.json) covers all **139 current
built-in rules**, not the historical counts in task descriptions. Existing YAML is
the executable source of truth. This original MIT development-only companion is
not loaded by the CLI, does not add report fields, and does not certify accuracy.

Each row binds the complete rule snapshot and its source-file SHA-256, then records
purpose, algorithm family, quantum exposure, separate classical concerns, reference
editions, method limits, known gaps and synthetic positive/negative pattern-target
controls. Confidence snapshots are compared in the existing public f32 domain;
source-byte hashes still bind the exact original confidence spelling and all YAML.
Reference IDs resolve to explicit editions and scope limits. A standard can justify
a primitive/purpose distinction without validating a source heuristic or vendor
configuration grammar. No standard text or real private keys are copied.

## Classification review decisions

- RSA/ECDSA/DSA/DH family quantum exposure and classical issues are separate axes.
  Legacy `non-quantum-risk` on RSA1024, SSH DSA or DH/SHA1 focuses one classical
  issue; it is not proof of quantum resistance. ES256/ECDHE-RSA legacy uncertainty
  is not a complete family classification. Existing Risk/severity tags stay intact.
- TLS protocol version, cipher ordering, metadata (`kid`, OIDC option names) and
  package presence do not alone identify selected cryptography or runtime safety.
  In particular, JWT packages can select several algorithm families; a dependency
  rule's legacy vulnerable tag does not prove RSA signing is enabled.
- Middleware cipher alternatives can mix RSA with 3DES/RC4/DES. A single legacy
  quantum-vulnerable tag does not describe every alternative. Authentication
  signatures and static RSA key transport remain distinct migration concerns.
- ML-KEM is a KEM; ML-DSA/SLH-DSA are signatures. Exact primitive/API/OID reference
  does not verify execution, providers, parameters, negotiation, trust or complete
  migration. Kyber/Dilithium draft names and hybrid combiners are not silently
  equated with final standards. Certificate container names prove none of these.
- Current certificate expiry/validity predicates and the historical fixed cutoff
  before2027 are different. The latter is local policy, not an external deadline.
  Certificate signature and subject public-key OIDs are separate target scopes.

## Fixtures and known gaps

The 305 current examples exercise **compiled regex targets only**, with desired labels
separate from expected current matches. They are not end-to-end source truth,
symbol resolution, a key parser test, provider validation or performance evidence.
The existing detector/CLI/certificate/schema suites and frozen task023 corpus still
provide their own bounded integration evidence. Do not count these controls as139
new independently labeled accuracy units or as approved quality budgets.

The original task028 recorded eight negative controls with false positives. Four
remain in the current companion; the others have passing negative expectations
after later repairs. The historical targets and their current dispositions are:

| Rule | Negative intent | Historical issue and current disposition |
|---|---|---|
| TLS_VERSION_1_0 | `TLSv1_3` | Fixed: current predicate rejects this modern version; later span/adjacency repair is also retained |
| NODE_TLS_MIN_VERSION | `minVersion: "TLSv1.3"` | Fixed: current predicate rejects this modern version |
| TS_TLS_LEGACY_VERSION_LITERAL | `TLSv1_3` | Fixed: current predicate rejects this modern version |
| TS_RUBY_RSA_USAGE | JWT HMAC example | Fixed: generic encode/decode branch removed; computed/aliased references remain bounded |
| HTTPD_SSLPROTOCOL_LEGACY | Explicitly disabled TLS1.0/1.1 | Does not parse enable/disable modifiers |
| HTTPD_SSLCIPHERSUITE_RSA | ECDSA cipher plus excluded RSA/weak operands | Does not interpret cipher exclusions |
| NGINX_SSL_CIPHERS_RSA | ECDSA cipher plus excluded RSA/weak operands | Does not interpret cipher exclusions |
| K8S_INGRESS_RSA_CIPHERS | ECDSA cipher plus excluded RSA/weak operands | Does not interpret cipher exclusions |

These cipher examples are negative for positive selection of the named RSA/weak
operands; they do not call ECDSA quantum resistant. Generic Java RSA encryption
and Go RSA imports are not classified as TLS use. Ruby generic JWT APIs retain
unknown algorithm branches separately from explicit RSA references.

No pattern, scope, ID, Risk or severity was changed to make these examples pass.
Changing such semantics remains a separate reviewed decision. Broad API patterns,
comments/strings, aliases, shadowing, dynamic settings and finite parser/occurrence
budgets remain explicit limitations. Matrix classifications are review annotations,
not changes to rule decisions or a comprehensive proof of every unsupported case.

## Maintenance

Development test `quality_metadata` loads authoritative built-ins, checks all12
source hashes, exact rule population and snapshots, reference resolution/editions,
required review fields and positive/negative pattern outcomes. A matcher/source
change makes this binding fail until the companion is re-reviewed; never regenerate
expected outcomes blindly. Metadata is under docs to avoid self-scan recursion.
All fixtures are synthetic and owned by this repository. Runtime needs no new
metadata lookup, network access or dependency.

Pinned primary references in the companion include RFC7518 (2015), RFC8996 (2021),
RFC9325 (2022), RFC8332 (2018), RFC9142 (2022), RFC5280 (2008), SP800-131A Rev2
(2019), FIPS203/204/205 final (2024), and reviewed NIST PQC/CSOR registers. These
are classification context, not a current-law compliance engine. Local task021
cutoff and task028 method policy are explicitly marked repository references.

The 2026-11-12 source repair refresh includes three additional classical API/EdDSA rules, narrowed Ruby/Go/TLS predicates and source-bound controls. Pattern fixtures remain distinct from language-aware detector tests (`semantic_repairs.rs`); current changes do not retroactively alter historical task028 evidence.

## Maintainer-approved F054-01 correction — 2026-11-17

The current DEP_RUST_RSA_CRATE predicate now matches exactly the Cargo package
components rsa and jsonwebtoken, with the existing case-insensitive policy.
Its other fields and the dependency parser are unchanged. Three additional
compiled-target controls cover universal-hash, rsa-helper and a versioned
jsonwebtoken purl; the current metadata has123rules and268controls. These are
synthetic controls, not independent accuracy units. Historical task062 metadata
and the earlier measurements remain unchanged; the maintained metadata source
hash/snapshot update records this explicitly approved correction. Detector and
CLI regressions additionally test real manifests,locks,SBOMs and provenance.

## Current-source audit supplement (task 065)

The [task 065 audit](../../../tasks/065/RULE-AUDIT.json) composes exactly one
supplemental row per maintained companion row, bound to the selected source.
See [results and correction impacts](../../../tasks/065/TASK-065-RESULTS.md) and
[population evidence](../../../tasks/065/evidence/POPULATION.json). The supplement
adds explicit parameter limits, per-rule base Severity/Confidence rationale gaps,
promotion gates and current FP/FN dispositions without changing executable rules.
It also supplies the SLH-DSA family and existing FIPS205 reference missing from
the Java/Ruby companion annotations. The maintained companion's snapshots and
historical evidence remain unchanged. The supplement also resolves the two local
reference paths left behind by documentation relocation, preserving their original
values as audit findings.

All 123 exact numeric confidence choices and independent justifications for exact
base Severity levels remain explicitly unknown in the bounded reviewed evidence;
source descriptions and cryptographic-family facts do not calibrate those values.
The existing 0.45 promotion policy remains compatibility behavior. Current
268 compiled-target controls include four retained false-positive controls,
not an estimate of end-to-end false-positive/false-negative rates.

Task078 adds three PHP rows and six pattern controls, while preserving the previous
125 rows. Its 180 frozen source cases and supplementary scope/privacy tests are
separate from these pattern controls; neither cohort estimates deployment accuracy.

Task079 adds three C# rows and six pattern controls without changing the previous
128 rows. Its218 frozen focal cases and supplemental source-flow/privacy tests are
separate evidence. The mixed PQC/HPKE/composite source rule uses quantum-uncertain
because a construction name does not identify a post-quantum component.

## C/C++ source additions (task080)

Three scoped definitions append to the inherited131 rows unchanged. They cover
finite low-level/EVP/OpenSSL and OQS requests, qualified C++ receivers and opaque
include references. Provider/property/TLS-group settings use existing finite
public evidence markers without algorithm assets or negotiation assertions.
Legacy OpenSSL lexical IDs/occurrences overlap intentionally and are retained;
argument/context privacy can mask their evidence. The frozen484 source cases and
supplemental hostile/privacy/scope controls are separate detector conformance,
not independent accuracy or calibration. Source-bound task080 review/checks must
complete before its implementation is accepted.

## Task081 dependency input supplement

Three additive exact-name/ecosystem predicates cover Composer/platform, NuGet and native build declarations. The inherited134 rows remain byte-for-byte unchanged. Fifteen new compiled-target controls are distinct from parser/provenance/privacy/CLI tests. Dependency presence never selects an algorithm or proves deployment.

## Implementation/construction reference additions (task085)

Two informational, quantum-uncertain AST rows add separate implementation-definition and conditional/component-reference evidence. The137 earlier YAML rows and their target controls are unchanged. New source-handler regressions distinguish definitions from calls/prototypes, canonical ownership from namesakes, conditional guards from string data, and composite components from whole-construction safety. No runtime, conformance or population-accuracy conclusion follows. The metadata records both positive/negative pattern targets and the stronger AST-layer interpretation.
