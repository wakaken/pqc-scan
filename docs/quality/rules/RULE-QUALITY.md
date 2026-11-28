# Built-in rule quality

[Machine-readable review](RULE-QUALITY-METADATA.json) covers all **139 current
built-in rules**, not the historical counts in task descriptions. Existing YAML is
the executable source of truth. This development-only companion is
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

## Pattern controls and known gaps

The companion's synthetic examples exercise compiled regex targets. Desired labels
and expected current matches are separate. These are not end-to-end source truth,
symbol-resolution tests, key parsing, provider validation, or performance evidence.
Detector, CLI, certificate, and schema tests provide their own bounded evidence.
Do not count pattern controls as independent accuracy units or quality budgets.

The following negative controls retain known limitations:

| Rule | Negative intent | Limit |
| --- | --- | --- |
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

Development test `quality_metadata` loads authoritative built-ins, checks source-file
hashes, exact rule population and snapshots, reference resolution/editions,
required review fields and positive/negative pattern outcomes. A matcher/source
change makes this binding fail until the companion is re-reviewed; never regenerate
expected outcomes blindly. Metadata is under docs to avoid self-scan recursion.
All fixtures are synthetic and owned by this repository. Runtime needs no new
metadata lookup, network access or dependency.

Pinned primary references in the companion include RFC7518 (2015), RFC8996 (2021),
RFC9325 (2022), RFC8332 (2018), RFC9142 (2022), RFC5280 (2008), SP800-131A Rev2
(2019), FIPS203/204/205 final (2024), and reviewed NIST PQC/CSOR registers. These
are classification context, not a current-law compliance engine. The local certificate
cutoff and review method policy are explicitly marked repository references.

## Interpreting rule evidence

KEM, signature, key exchange, implementation definitions, and construction/component
references have different meanings. A dependency or container candidate does not
select an algorithm or establish its contents. Lexical member names, macro prefixes,
documentation, and type checks remain references, not proof of executed operations.
Parsed certificate facts and incomplete inspection states remain separate.

Exact confidence values and base severity levels are not independently calibrated
by cryptographic-family rationale. Preserve known gaps and current outcomes rather
than changing expectations to obtain a pass. Apply the
[rule validation requirements](../../rules-reference.md#validation-and-quality-review)
when rules or their source bindings change.
