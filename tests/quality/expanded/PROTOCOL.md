# TASK-042 source-only annotation protocol, version 1

This extends ../PROTOCOL.md without changing its matching or nominal-reference
semantics. Read input bytes and the questions in corpus.json. Never execute,
import, compile, install, or generate keys from any specimen. No secret keys are
present: DER/PEM/CRT/CER files are public certificates, PKCS12/PFX files contain
only a public certificate (exported with -nokeys and an empty password), and marker bodies are
deliberately invalid canaries. Review public certificate attributes against
crates/pqc_scan_detectors/tests/fixtures/certificates/manifest.json.

## Independent records before measurement

Each of two reviewers writes only their own labels file. Before writing it, do
not open scanner code, rule patterns, scanner outputs, metrics, other labels or
the coordinator's proposed classifications. The manifest contains questions and
nominated rule IDs, not desired labels. The descriptions below define the
evaluated property independently of the implementation's match patterns.
Record reviewer identity, corpusSha256, protocolSha256, sourceOnly=true,
otherLabelsSeen=false, scannerResultsSeen=false, and one labels entry per unit.
Each entry has id, label, algorithms, usage, artifact, caseKind, parameters and
a concrete source-based reason. The coordinator adjudicates differences from
source alone and records reason/resolution before the first new-data scan.
Unresolved facts remain ambiguous. Neither label record is rewritten by the
coordinator. Final truth binds both records, protocol, provenance and freeze time.

## Evaluated properties

- PQC_*_API: nominal standard PQC API/import/symbol references for the nominated
  language/API. Aliases with explicit imports retain provenance. Explicit local
  shadowing or unrelated local definitions are negative. A string/comment is not
  an API call. Dead code retains a static reference; execution is never implied.
  Java KEM.getInstance and Signature.getInstance have distinct uses. A Node
  key-type argument is a string, not a WebCrypto name object. Unknown names are
  negative for an identified standardized PQC API, never evidence of safety.
  A locally bound finite loop of explicit algorithm names may identify a family;
  an externally unconstrained dynamic parameter remains ambiguous. Runtime
  capability guards do not erase a static reference. Missing syntax does not
  erase a visible nominal reference; undecidable truncated arguments are unsupported.
- PQC_DRAFT_ALGORITHM: a named pre-standard PQC algorithm reference. Record the
  exact old name in parameters; never equate it with a final standardized API.
- PQC_JAVA_CLASSICAL_GROUPS: effective explicit list of only recognized classical
  TLS groups; a commented assignment is negative, a hybrid list is negative.
- SSH_DSS_SIGALG / SSH_DH_GROUP14_SHA1: the exact configured DSA signature /
  classical DH group14 SHA1 KEX respectively. Other algorithms are negative.
  An incomplete directive with no algorithm is unsupported.
- JWT_RS256: nominal JWT signature algorithm selection, not arbitrary prose.
  SHA256 alone does not identify RSA. TS_NODE_RSA_USAGE: nominal RSA API use.
  API_NODE_CRYPTO_SIGN_RSA: RSA signing reference; consider a locally explicit
  RSA-generated key passed into signing, but do not infer RSA from SHA256 alone.
- DEP_PQC_CAPABLE_LIBRARY: recognized library declaration/presence, irrespective
  of version-dependent backend/feature support. This corpus uses openssl, jose,
  cryptography, CIRCL and BouncyCastle declarations. No execution or enabledness
  is implied. Versions are synthetic declaration examples, not release claims.
  Malformed undecidable documents are unsupported. Crypto-only CBOM assets are
  outside dependency ingestion support, not negative dependency evidence.
- CERT_RSA_3072, CERT_RSA_SIGNATURE, CERT_ECDSA_SIGNATURE: the exact public
  object's SPKI size or signature attribute, not filename or trust validation.
  Broken containers have unsupported attributes, not negative security evidence.
- CERT_P12_CONTAINER: PKCS12/PFX container presence only, without key presence or
  decryption success. The conversion record in provenance.json binds the original
  RSA public certificate; the related negative signature question is about that
  known object, not a claim that the scanner can decrypt arbitrary PKCS12 files.
- PRIVATE_KEY_*_HEADER: complete recognized BEGIN marker only. A deliberately
  invalid body is still positive marker presence. Incomplete marker is negative
  for the complete marker property, not a claim that no secret exists.

## Annotation vocabulary

label: positive, negative, ambiguous, unsupported. Unsupported language/format is
unsupported even if a familiar name appears. A negative is relative to the exact
nominated property, never an assertion that the file is safe.

algorithms (nonempty unique list): RSA, DSA, ECDSA, ECDH, DH, EdDSA, ML-KEM,
ML-DSA, SLH-DSA, Hybrid, pre_standard, unknown, not_applicable, AES, HMAC.
Record the algorithm actually visible in the source, including a classical
alternative in a negative PQC unit. Hybrid represents an explicit combined group;
if multiple independent operations form one indivisible unit, list all known
algorithms. Use unknown when an algorithm fact is absent/undecidable.
Library presence alone has algorithms=[unknown]; it never proves any particular
algorithm is enabled. parameters is a list of exact visible type, variant,
key-size, group or declared-version strings; [] when no parameter is established.
Do not multiply loop parameters into independent evaluation units.

usage: signature, key_exchange, kem, encryption, key_generation,
dependency_presence, certificate_signature, public_key, key_marker,
configuration, algorithm_reference, unknown, not_applicable. Classify the
observed nominal operation: generating keys is key_generation; requesting KEM
is kem. A string-only hybrid advertisement is algorithm_reference; it is not a
negotiated exchange. A certificate SPKI-size unit is public_key; a certificate
signature unit is certificate_signature. No runtime execution is claimed.

artifact: source, configuration, certificate, key_marker, dependency_manifest,
sbom. Use the manifest artifact; the coordinator validates file consistency.

caseKind (one primary structural case): ordinary, comment, string, namesake,
alias, shadowing, dynamic, dead_code, disabled, mixed, same_line, typed,
malformed, unsupported. Unsupported format/language takes precedence; malformed
syntax takes precedence over ordinary; dynamic means unconstrained argument,
not a finite locally resolved loop. mixed means distinct algorithms on one line;
same_line means multiple calls in the same family. Primary categories are not
independent dimensions. Library declarations, parameter variants and upstream
loops are correlated. Select by structure, not whether the scanner succeeds.

## Limits and scoring

Only nominated rules are scored. Every nominated finding outside a unit is an
FP. Preserve ambiguous/unsupported predictions separately. A positive matches
once; duplicates are FP. Use the existing scorer and its half-open Unicode
intervals. Unknown-position dependencies may bind by exact dependencyName.
Group by independently reviewed annotations; unannotated FP are unattributed.
Multi-algorithm units use a combined cell to avoid double counting. All rates
have denominators and zero denominators are null. Empty vocabulary cells and
all untested catalog rules remain visible.

Tuning is coordinator-authored with knowledge of rules. Holdout is public Node.js
source written before pqc-scan existed. It is one upstream lineage, selected by
a rule-aware coordinator; this is authorship independence, not an unbiased
population sample or a secret benchmark. No cross-split lineage is allowed.
After this first frozen evaluation, using its errors to tune requires a fresh
independent held-out lineage. Keep this version and results immutable.
