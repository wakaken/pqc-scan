# Supported inputs and inspection limits

[日本語](COVERAGE.ja.md) · [README](../../README.md)

Start with the [input overview](../../README.md#targets). This guide gives the supported formats and boundaries behind that table. Support for a file type means the documented subset can be inspected; it does not mean every cryptographic operation or runtime path can be found.

## Contents

- [Dependency files](#dependencies)
- [SBOM input](#sbom)
- [Certificates and key markers](#certificates)
- [Configuration files and rules](#configuration)
- [Source-code inspection limits](#source-limits)
- [Privacy and excluded files](#privacy)

<a id="dependencies"></a>

## Dependency files

- `Cargo.toml`, `Cargo.lock`
- `Gemfile`, `Gemfile.lock`, `gems.rb`, `gems.locked`, `*.gemspec`
- `pom.xml`, `build.gradle`, `build.gradle.kts`, `gradle.lockfile`
- `go.mod`
- `package.json`, `package-lock.json`, `pnpm-lock.yaml`
- `requirements.txt`, `Pipfile.lock`, `poetry.lock`

<a id="sbom"></a>

## SBOM input

- CycloneDX JSON (`*.cdx.json`, `*bom*.json`)
- SPDX JSON (`*.spdx.json`, `*sbom*.json`)

These are bounded component/package subsets: CycloneDX 1.4–1.7 and SPDX 2.2–2.3. They are input formats, not full-schema validation, installed-state proof or standards export. Structured dependency parsing, lexical fallbacks, supported versions and limits are listed in the [dependency provenance contract](../architecture/DEPENDENCY-PROVENANCE.md).

<a id="certificates"></a>

## Certificates and key markers

- Certificate candidates: `.pem`, `.crt`, `.cer`, `.der` (PEM or DER selected by
  content). `.p12`/`.pfx` additionally inspect unencrypted public certificates inside supported PKCS#12 containers.
- Private key markers:
  - `BEGIN RSA PRIVATE KEY`
  - `BEGIN PRIVATE KEY`
  - `BEGIN OPENSSH PRIVATE KEY`

Certificate parsing separates the signature algorithm from the subject public key.
RSA key size is the modulus bit length; EC size comes from supported named curves
(P-256/P-384/P-521), not encoded point length. Ed25519/Ed448/X25519/X448 encoded
lengths are recorded separately, without inventing RSA key size or security strength.
Unknown/PQC subject-key encodings retain their OID and public fingerprints with
unknown semantic size; a PQC OID is not proof of a valid PQC key.

Existing certificate evidence metadata now includes public certificate/SPKI SHA-256
fingerprints, explicitly named encoded byte lengths, key type/parameters, known
semantic size/curve, validity timestamps and object positions. PEM bundles preserve
each certificate's real line/column and block index, including distinct certificates
sharing one public key. DER retains the legacy 1:1 location; `byte_offset: "0"`
identifies the object, not a source-code position. Malformed/unsupported blocks retain
other parsed certificates and report incomplete inspection through `--diagnostics`.
No certificate signatures, chains or mathematical key/point validity are verified;
PKCS#12 decryption and standalone public-key file detection are not implemented.

Built-in certificate signature rules use exact parsed OIDs. RSA-size rules require
an RSA/PSS subject modulus, so filenames or EC point sizes cannot satisfy them.
ECDSA and DSA are classified `quantum-vulnerable`; under the unchanged PKI promotion
policy their final severities are high and critical respectively. SHA-1/MD5 and
weak-RSA findings express separate classical concerns. DSA migration guidance
separates replacement of signing from legacy signature verification.

`CERT_EXPIRED` and `CERT_NOT_YET_VALID` use parsed UTC timestamps and an explicit
Unix-second assessment clock; both validity endpoints are inclusive. The old
`CERT_EXPIRY_2026_OR_BEFORE` ID retains its fixed parsed cutoff before 2027-01-01
UTC and is labeled legacy; it is not an alias for current expiry. Reversed ranges
or unavailable assessment clocks prevent a complete validity verdict while
retaining static facts. Time findings record `assessment_time_unix` and
`validity_status` in the existing evidence metadata map.

The CLI uses a fresh certificate assessment clock for each scan.

Existing custom `certificate` and other legacy scopes keep composite-text matching,
except the newly reserved opt-in scopes `rsa_modulus_bits`, `certificate_validity`
and `legacy_not_after_2026`. External packs previously using those exact strings as
opaque legacy labels should rename them (or use `certificate`) before upgrading.
Existing `signature_oid`/`public_key_oid` scopes remain exact identifier inputs.
Corrected facts, typed evidence and risk tags intentionally change affected
certificate findings, IDs and legacy CBOM algorithm strings; CLI options, report
structures/enums and core ID/scoring algorithms remain unchanged. No signature,
chain or runtime-use guarantee follows. PSS digest parameters and unlisted
signature OIDs are not fully assessed; absence of a hash finding does not prove a
safe digest. See [classification specification](../roadmap/TASK-021-IMPLEMENTATION.md)
and [focused baseline](../quality/CERTIFICATE-BASELINE-021.md).
Files containing EC/DSA/encrypted private-key PEM markers also receive file-wide source
snippet masking; this does not add new private-key detection rules or decrypt keys.

### PKCS#12 support and limits

The parser reads the DER definite-length subset of PFX v3 Data/AuthenticatedSafe/SafeContents and X.509 certificate bags. Public certificates pass through the same attribute extraction as PEM/DER. Nested safe contents are bounded; BER indefinite encodings and constructed octet strings are outside this subset.

Encrypted content, private-key bags and unknown bag/content types remain opaque and unsupported. If readable public certificates coexist with them, those certificates are retained and inspection is partial. Encrypted-only input is unsupported, malformed input is partial, and a resource limit reports truncation. The container finding records a fixed reason and explicitly records MAC and trust verification as not performed. A completed structural inspection is not an integrity/trust verdict.

Limits are 2 MiB, 256 bags, 256 content entries, nesting depth 8 and 4096 attribute values. Source previews/snippets are never read from P12/PFX, even when malformed bytes look like text. Legacy binary locations retain 1:1 and actual byte offsets; inventory locations are unknown with object indices.

<a id="configuration"></a>

## Configuration files

- Kubernetes / Ingress
- nginx
- Apache httpd
- Envoy
- Istio
- HAProxy
- Traefik

### Built-in Rule Set

- Default rules live under `rules/default/`
- Current built-in rules: 119
- Full reference (all rule IDs + severity definitions): [docs/rules-reference.md](../rules-reference.md)

Rule packs reject blank IDs, non-finite confidence or values outside `0..1`, duplicate IDs, invalid patterns and traversal failures. Invalid packs return operational error 1 instead of a partial pack. Valid custom scopes retain their per-detector semantics; unknown scope labels are not rejected. Check [scope compatibility](../architecture/RULE-VALIDATION.md) when upgrading a custom pack.

<a id="source-limits"></a>

## Source-code inspection limits

AST findings distinguish source spans per rule: repeated uses on the same line
remain separate, while parent/child representations of the same lexical match
produce one observation. The existing 32-hit rule/file cap and bounded-work
truncation diagnostics remain. A display-only clipping ellipsis is never treated
as source. CLI flags, report schemas/filenames and IDs for unchanged occurrences
are preserved; finding counts and new occurrence IDs may increase intentionally.
Legacy AST Unicode column limitations still apply (see [SARIF locations](REPORTS.md#sarif-locations)).

TypeScript and TSX use their dedicated pinned grammars. Valid type syntax no longer becomes partial merely because it is not JavaScript. Syntax errors still retain available findings and report partial inspection. Matching remains nominal: imported aliases can be missed, namesakes/display strings can be false positives, and comments inside ancestor text are not full semantic analysis. No symbol resolver or dataflow engine is implied.

<a id="privacy"></a>

## Privacy and excluded files

Recognized private-key blocks and credential literals are masked in exported evidence. The policy covers RSA, generic PKCS#8, OpenSSH, EC, DSA and encrypted private-key markers; single-line named password/token/key literals; Bearer/Basic tokens; and HTTP(S) userinfo and named credential query values. Marker masking does not add private-key parsing or decryption support.

Previews are conservatively masked when recognized private/credential context is present. Metadata values, action text/code, CBOM and dependency text also pass through bounded redaction. Typed inventory and diagnostics omit raw payloads and arbitrary error text.

This is not a universal secret scanner. Unknown/obfuscated syntax, arbitrary credential names, multiline credential position tracking and secret-bearing structural fields are outside the contract. Keep paths, IDs, repository scopes and custom metadata keys free of secrets. Public certificate fingerprints/subjects and other retained facts can still require distribution review. Some benign preview text is omitted conservatively. See the [supported syntax, field policy and limits](../architecture/EVIDENCE-PRIVACY.md) and [synthetic-canary verification](../quality/TASK-029-EVIDENCE-PRIVACY.md).

Repository walking respects Git ignore settings. Default exclusions include `.git`, `node_modules`, `target`, `dist`, `docs` and Markdown files; files larger than 2 MiB are skipped. Test directories are not generally excluded. These scope limits mean a successful scan does not establish absence of cryptography in ignored or unsupported files.
