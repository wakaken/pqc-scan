# Task 065 correction impact and D5 inputs

These are bounded audit dispositions against the selected source in
[evidence/SOURCE.json](evidence/SOURCE.json). Except for task-local audit
annotations and the maintained guide, no correction is implemented. No new task,
rule rewrite, policy, schema, support expansion or numerical gate is authorized.
The rule lists and current values resolve in [RULE-AUDIT.json](RULE-AUDIT.json).

## C065-01: configuration exclusion false positives

Affected: `HTTPD_SSLPROTOCOL_LEGACY`, `HTTPD_SSLCIPHERSUITE_RSA`,
`NGINX_SSL_CIPHERS_RSA`, `K8S_INGRESS_RSA_CIPHERS`.

The four companion negative targets currently match: disabled protocol operands
and excluded RSA/weak-cipher operands are treated as positive references. These
are retained compiled-target findings, not new end-to-end incidence estimates.
See the exact desired/current pairs in the per-rule `fp_fn` records and the
passing current Rust companion test, which deliberately preserves disagreement.

Recommended future direction: specify a narrow per-directive interpretation of
positive and negative operands, preserving unsupported/unknown configuration as
unknown. Do not apply a universal token-exclusion regex or assume effective runtime
configuration from a static line. Parsing whole vendor grammars is not approved.

Impact: findings, evidence ranges, finding IDs, counts and downstream report/CBOM
associations may change when false-positive rows disappear. With that row as the
only threshold-reaching finding, `--fail-on high` can change exit 2 to 0;
operational error 1 and incomplete-inspection 3 retain precedence. Base values and
Risk need not change. The three cipher rules are Middleware and do not receive
the sensitive-category bump. Custom packs may depend on today's generic patterns;
compare exact shipped selectors with changed-pattern/scope/ID controls before
choosing a repair contract. No automatic custom-pack rewrite/rejection.

Positive cases required before adoption: explicitly enabled TLS1.0/1.1; positive
RSA or weak-cipher operands; mixed positive and negative operands whose selected
meaning is specified. Negative cases: the four exact companion negatives;
TLS1.2/1.3; excluded-only RSA/weak operands. Include repeated directives,
unsupported modifiers, malformed/partial config and a changed custom selector.

Decision/review: maintainer D5 approval of affected rule meaning and compatibility
before a repair; implementation review followed by test review with actual
CLI/report cases. Recommendation for task 065: retain current behavior and the
finding. Task 066 may prepare a bounded proposal; task 067 may apply only an
approved correction within its own scope.

## C065-02: incomplete audit annotations and relocated references

Affected family annotations: `PQC_JAVA_API`, `PQC_RUBY_API`. Their current predicates
include SLH-DSA, but inherited family lists and reference IDs omit SLH-DSA/FIPS205.
The task-local supplement adds that family and the existing FIPS205 reference,
whose official publication abstract identifies the signature primitive. This is
an audit annotation, not new algorithm support or proof of working APIs.

Affected local references: every row using `LOCAL021` or `LOCAL028` (the exact sets
are recorded in [REUSE.json](evidence/REUSE.json)). The inherited URLs no longer
exist after document relocation. Effective audit URLs resolve to
`tasks/021/CERTIFICATE-BASELINE-021.md` and
`docs/quality/rules/RULE-QUALITY.md`. Original URLs and their failed existence
checks remain visible in `reference_path_augmentations` and reuse evidence.
The current Rust validator checks reference IDs/edition/scope, but does not test
whether a local URL exists; task 065 adds the actual path-existence audit.

Disposition: completed in this audit supplement; the inherited companion is
unchanged. Recommendation: reuse the composed audit now. A later documentation
maintenance change can synchronize the companion if selected; no task is created.

Impact: zero rule matching, IDs, runtime reports, schema, severity/confidence,
CLI exit or custom-pack acceptance effects. Development-only annotation consumers
must resolve the supplement's effective reference map rather than treating the
old path as valid. Positive checks: Java/Ruby SLH branch and FIPS205 ID;
existing local targets. Negative checks: a non-SLH OTHER name, unresolved ID,
and the two obsolete paths. The generic reference-ID negative was actually run;
new family-specific detector coverage is not claimed from annotation changes.
No human rule/storage gate applies to this annotation-only correction; no
independent executable review is required.

## C065-03: legacy Risk does not express the full technical concern

Affected classical-focus tags: `CERT_RSA_1024`, `SSH_DH_GROUP1`,
`SSH_DH_GROUP14_SHA1`, `SSH_DSS_SIGALG`. A classical concern does not remove the
family's quantum exposure. Affected uncertain tags: `JWT_ES256`, `TS_JWT_ES_ALG`,
`TLS_ECDHE_RSA`. Uncertainty of use is different from uncertainty about a known
public-key family's exposure. Affected dependency vulnerable tags:
`DEP_RUBY_JWT`, `DEP_RUST_RSA_CRATE`; package presence does not prove selected RSA.
Mixed-operand tags: `ENVOY_CIPHER_SUITES_RSA`, `HTTPD_SSLCIPHERSUITE_RSA`,
`ISTIO_DESTINATIONRULE_RSA_CIPHERS`, `K8S_INGRESS_RSA_CIPHERS`,
`NGINX_SSL_CIPHERS_RSA`; alternatives mix public-key and weak symmetric concerns.

Recommended direction: retain legacy enums and expose separately grounded
technical facts through the approved future assessment contract. Reuse the
existing assessment/catalog; do not reinterpret uncertainty as resistance,
classical weakness as resistance, or a package as deployed cryptography.

Impact if legacy tags were changed: JSON/SARIF/CBOM and human text may change;
Risk-dependent action routing/policy/export may change. JWT/ECDHE tags changed to
vulnerable would enter sensitive-category promotion at existing confidence,
possibly changing severity and fail-on exits. CryptoAPI/Middleware/SSH remain
outside that gate unless categories also change. Custom rules retain their own
values; a global reinterpretation would affect them and requires an explicit
contract. A separately selected new path must preserve v1 defaults and goldens.

Positive cases: known RSA1024, DH/SHA1, DSA, ES256 and ECDHE_RSA component concerns;
explicit confirmed RSA versus HMAC use with the same package; positive weak-cipher
branch without RSA. Negative/unknown cases: missing parameters, package without
use, ECDSA authentication versus RSA transport, unsupported/computed configuration.
The reused companion and task056 cases cover current bounded characterizations;
these proposed new-path cases are obligations for a later approved implementation.

Decision/review: D5 classification fact contract and D1/D2 version/dispatch before
affected behavior; human review for any changed crypto/rule meaning and both
independent executable review stages. No change enacted here.

## C065-04: generic references inherit a sensitive category

Affected: `GO_TLS_RSA_IMPORT`, `JAVA_KEYFACTORY_RSA`, `OPENSSL_RSA_API`,
`TS_CERT_SIGALG_LITERAL`, `TS_RSA_KEY_SIZE_LITERAL`, `TLS_CERT_PIN_RSA`.
Their static names/imports/nearby literals do not establish an active TLS/PKI
system or a parsed pinned certificate. The first five satisfy current promotion
gates; `TLS_CERT_PIN_RSA` is suppressed by confidence 0.42. Existing companion
purpose/limits remain the factual description.

Recommended direction: preserve legacy categories and derive assurance/usage for
the future decision path from captured facts. Do not silently recategorize them.
Any category correction can change grouping, recommendations, exports, severity
and threshold exits. For example a sole Medium-to-High generic import hit can
stop producing exit 2 under fail-on high if moved outside the sensitive gate.
Custom tags must not acquire inferred TLS meaning or a stricter grammar.

Positive cases: actual supported TLS/PKI source/config reference, explicit RSA
size context and independently parsed certificate attributes. Negative cases:
unused import, non-TLS key factory/encryption, a mere algorithm-name catalog,
non-RSA size literals, a pin variable name without certificate data. Missing
business context remains unknown. No current source-reference capture is promoted
to execution evidence.

Decision/review: D5 before changing categories and legacy output/exit meaning;
both independent stages for a later implementation. No change enacted here.

## C065-05: exact numeric rationale remains unknown

Affected: all 123 base severity/confidence choices; all 25 category/risk-eligible
rules (24 promoted at 0.45, `TLS_CERT_PIN_RSA` below it); compatible custom rules.
Every row explicitly identifies this unknown rather than deriving empirical
justification from a source snapshot. The policy comment explains a false-positive
escalation intention; it does not establish why 0.45 or a one-step bump is optimal.

Recommendation: retain 0.45 and all base values for compatibility. The task056
maintainer disposition already authorizes preservation, not calibration. Do not
replace it with 0.5 merely because today's built-ins have no confidence in
[0.45,0.5): valid custom rules in that interval change, including their
private_key path. A replacement needs a defined policy objective and separately
adjudicated impact/action truth. Standards do not supply detector confidence.

Impact: even with unchanged matching and evidence, a threshold or base-value
change affects severity exports, human ordering and fail-on high/critical exits;
changing confidence also changes rule-show/JSON values. Confidence is f32 after
validated f64 input: decimal spelling near the boundary must not bypass transport
semantics. Changes affect custom category/risk/evidence combinations too.

Positive/negative/boundary obligations: every base level, saturation at Critical,
all four risks, sensitive/nonsensitive and case-variant categories, confidence
below/at/above 0.45, ordinary/private_key evidence, and custom [0.45,0.5) values.
The current 960 frozen severity cases and five risk tests verify preservation;
no new intended severity policy or calibrated probabilities were measured.

Decision/review: a later replacement requires maintainer D5 policy approval,
explicit compatibility disposition and both independent review stages. No
replacement is needed for this task's original audit acceptance. This is not a
request to initiate numerical calibration or automatically create follow-up work.
