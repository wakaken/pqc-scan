# Supported inputs and inspection limits

[日本語](COVERAGE.ja.md) · [README](../../README.md)

This guide describes the v0.3 development checkout. Detected references and findings can differ from the published v0.2.0 release.

Start with the [input overview](../../README.md#targets). This guide gives the supported formats and boundaries behind that table. Support for a file type means the documented subset can be inspected; it does not mean every cryptographic operation or runtime path can be found.

Audit rule coverage: the RSA-size rule requires an explicit RSA token or recognized RSA-size identifier, not any `Key` substring. It remains nominal matching, not binding or measured key-size proof. `PQC_STANDARD_REFERENCE` recognizes ML-KEM/ML-DSA/SLH-DSA parameter names and X25519MLKEM768, including compact MLKEM/MLDSA symbols. Evidence metadata records `reference_kind` as `name_literal`, `call_reference` or `symbol_reference` based on the eligible AST candidate; none establishes execution. Hash-commented `.conf`/`.cfg` lines are excluded by the lexical comment filter. Generic C/text protocol references remain references, not effective settings.

Gradle's lexical single-line subset accepts the existing six dependency configurations with quoted coordinates or literal `group`/`name` map fields and optional `version`. Variables/interpolation are unresolved, declarations never supply a resolved version, and lexical inspection remains NotReported rather than full Gradle analysis. Dynamic expressions, multiline declarations and script evaluation remain outside this subset.

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

These are bounded component/package subsets: CycloneDX 1.4 to 1.7 and SPDX 2.2 to 2.3. They are input formats, not full-schema validation, installed-state proof or standards export. Structured dependency parsing, lexical fallbacks, supported versions and limits are listed in the [dependency provenance contract](../architecture/DEPENDENCY-PROVENANCE.md).

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

`certificate` and other general scopes use composite-text matching. The reserved
scopes `rsa_modulus_bits`, `certificate_validity` and `legacy_not_after_2026` use
typed modulus, current-validity and fixed-cutoff inputs respectively.
Existing `signature_oid`/`public_key_oid` scopes remain exact identifier inputs.
Corrected facts, typed evidence and risk tags intentionally change affected
certificate findings, IDs and legacy CBOM algorithm strings; CLI options, report
structures/enums and core ID/scoring algorithms remain unchanged. No signature,
chain or runtime-use guarantee follows. PSS digest parameters and unlisted
signature OIDs are not fully assessed; absence of a hash finding does not prove a
safe digest. See [classification specification](../../tasks/021/TASK-021-IMPLEMENTATION.md)
and [focused baseline](../../tasks/021/CERTIFICATE-BASELINE-021.md).
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

Settings remain static references, not effective runtime configuration. Supported Java/Ruby rules include selected SLH-DSA variants and classical-only TLS group settings; pre-standard algorithm names are checked case-insensitively. Hash-commented SSH settings in `.conf`/`.cfg` files are excluded by the applicable comment filter. See the rule reference for exact patterns; these checks do not negotiate a connection.

### Built-in Rule Set

- Default rules live under `rules/default/`
- Current built-in rules: 123
- Full reference (all rule IDs + severity definitions): [docs/rules-reference.md](../rules-reference.md)

Rule packs reject blank IDs, non-finite confidence or values outside `0..1`, duplicate IDs, invalid patterns and traversal failures. Invalid packs return operational error 1 instead of a partial pack. Valid custom scopes retain their per-detector semantics; unknown scope labels are not rejected. Check [scope compatibility](../architecture/RULE-VALIDATION.md) when upgrading a custom pack.

<a id="source-limits"></a>

## Source-code inspection limits

AST findings distinguish source spans per rule: repeated uses on the same line
remain separate, while parent/child representations of the same lexical match
produce one observation. Per rule/file, at most 4,096 accepted hits and 65,536 examined matches are retained; bounded-work truncation remains explicit. A display-only clipping ellipsis is never treated
as source. Finding IDs use the rule, path, position and original matched text.
AST Unicode column limitations apply (see [SARIF locations](REPORTS.md#sarif-locations)).

TypeScript and TSX use their dedicated pinned grammars. Syntax errors retain available findings and report partial inspection.

<a id="source-facts"></a>

### Built-in local source analysis

Selected built-in rules supplement pattern matching with imports, aliases and bindings within one file.

| Target | Relationships inspected |
|---|---|
| Node PQC and RSA APIs | Supported crypto imports/aliases, local bindings, literal finite `for...of` lists and synchronously generated RSA keys passed to signing/verification |
| Python PQC generation APIs | Supported explicit imports/aliases and local bindings that shadow the referenced name |
| Rust PQC APIs | Supported crate/type imports and local ML-KEM key roles and receivers |
| Java PQC APIs | Supported explicit algorithm requests through `getInstance`; available references are retained in incomplete syntax |

Where bindings can be resolved, these checks exclude unrelated namesakes and help strings. They do not resolve computed names, external modules, arbitrary expressions or runtime paths. Other rules can still miss aliases or match display strings/comments. A dependency or an HS256/AES reference alone does not establish RSA use.

Local analysis uses the `DetectorBudget` limits below. Additional analysis limits include alias depth 8, finite arrays of 32 elements and traversal depth 64. Exhausted analysis reports `detector_truncated`; syntax errors remain partial. Resolving a reference does not prove successful execution, enabled functionality or completed PQC migration.

This analysis applies when the loaded rule ID, kind, pattern and scope match the default definition corresponding to that binary. Modified custom patterns/scopes use generic matching and do not automatically gain the same alias tracking. The runtime rule directory is still required.

## Repository inspection budgets and encoded text

AST candidates borrow full source slices: the former 2,000-character matching cutoff is removed; only displayed previews are clipped. Defaults allow 262,144 semantic candidates, 2,000,000 scheduled nodes, 4,096 accepted hits per rule/file, 65,536 examined matches per rule/file, and 256 MiB of cumulative candidate text examined across AST rules per file. Exactly reaching a hit cap is distinguished from observing another eligible hit. Real exhaustion remains `detector_truncated`; these are finite resource controls, not proof that a specific number of findings was lost. Library callers can use `DetectorBudget` with `TreeSitterDetector::with_budget` / `RegexDetector::with_budget` to set explicit budgets.

The pinned module grammars are supplemented by a parse-only compatibility view for TypeScript `export type *` (including namespace re-exports) and JavaScript/TypeScript `import defer * as ...`. ASCII modifier bytes are replaced with spaces only in actual top-level module statements; evidence and matching use the original decoded text with unchanged offsets. Other syntax errors retain partial status. This does not evaluate modules or validate runtime behavior.

UTF-16LE/BE and UTF-32LE/BE BOMs are recognized and decoded strictly; BOM-less wide text needs a strong ASCII-lane signature and valid printable Unicode. Malformed BOM-marked wide text is reported as unsupported rather than completed or silently excluded. Detection, source context and redaction use the same decoded text. Binary certificate/private container bytes remain opaque to source detectors, and no decryption is performed.

Default pre-inspection policy excludes known media, archives, compiled data (including WebAssembly, MO and protobuf-style extensions), and non-text binary input. Protected certificate/key/store extensions remain selected. The explicit opt-out is `--include-non-scannable`; `.gitignore`, directory exclusions and the 2 MiB limit still apply. Do not infer full repository enumeration from selected counts.

Dependency structured-value decoding is bounded at 500,000 values and depth 64; aliases and explicit tags remain unsupported. YAML plain/quoted/block scalar punctuation is distinguished from node properties, and empty metadata values are valid nulls.

pnpm single-document locks and the boot/project two-document layout are both supported. Both graphs are inventoried separately with globally unique record ordinals; matching schema versions and at most two documents are required. The 500,000-value and 10,000-record budgets apply to the entire file, not separately to each document. See [pnpm’s lockfile reader guidance](https://github.com/pnpm/pnpm/blob/main/pnpm/docs/lockfile.md).

Dockerfile routing uses the final filename (`Dockerfile` or `Dockerfile.*`) plus a first meaningful `FROM`/`ARG` instruction, skipping comments and a BOM. Ordinary language source with the same filename keeps its language parser; lexical rules continue on Dockerfiles. Bundler platform suffixes require a matching `PLATFORMS` declaration (including Linux GNU normalization), and only registry records gain a resolved base version; Git/path records remain unresolved. Compatibility for empty type-alias parameter lists, type-context `import(...)` and quoted JSX attributes is parse-only. Runtime imports and expression operators are preserved, malformed inputs retain partial status, and compatibility work remains bounded.

### Static context and conditional declarations

Maven parent properties are inherited only from selected, bounded regular files inside the scan root, after matching the declared group/artifact/version. Local references honor empty relativePath, reject links and excluded/out-of-root files, and stop at 16 levels or 8 MiB of parent input. Conflicting properties remain unresolved; identical duplicates are idempotent for declaration inspection. Profile property candidates are labelled conditional (`dep_context`, native observation capture status and dependency uncertainty), never selected as an effective configuration or a resolved artifact. No Maven execution, BOM fetch, transitive graph or installed-package claim is implied.

Ruby Gemfile/gemspec inspection reads literal dependency calls and supported groups/platforms/branches using the Ruby syntax tree. A gemspec receiver must be bound to the specification block; rebound/ambiguous receivers, dynamic versions/names and evaluation/loading remain incomplete. Delegated gemspec dependencies keep the gemspec's own path and positions instead of being copied to the Gemfile. Git/path sources stay unresolved. Unknown DSL calls remain incomplete; arbitrary Ruby is not evaluated.

Recognized unrendered template directives have an offset-preserving static parse view. Requirements branch declarations are retained separately as conditional candidates. Template rendering/branch selection is not executed, and generated values remain partial even when the static source is parsed. Literal strings, comments and regular expressions do not become source directives solely by resembling a template.

Selected V8 grammar compatibility covers native-call expression positions, sloppy-script `yield` identifiers, tagged-template escape content and legacy runtime-error assignment targets. It does not execute those calls or assert runtime success. Explicit module/strict boundaries and malformed controls remain incomplete. Legacy DSA OID 1.3.14.3.2.12 uses the known public-parameter/public-integer layout; absent parameters retain partial status and no key size or domain parameters are invented. Sources: [OpenSSL object identifiers](https://github.com/openssl/openssl/blob/master/crypto/objects/objects.txt), [OpenSSL DSA decoder](https://github.com/openssl/openssl/blob/master/crypto/dsa/dsa_ameth.c), [Maven POM reference](https://maven.apache.org/pom.html), [Django template language](https://docs.djangoproject.com/en/5.2/ref/templates/language/).

<a id="privacy"></a>

## Privacy and excluded files

Recognized private-key blocks and credential literals are masked in exported evidence. The policy covers RSA, generic PKCS#8, OpenSSH, EC, DSA and encrypted private-key markers; single-line named password/token/key literals; Bearer/Basic tokens; and HTTP(S) userinfo and named credential query values. Marker masking does not add private-key parsing or decryption support.

Previews are conservatively masked when recognized private/credential context is present. Metadata values, action text/code, CBOM and dependency text also pass through bounded redaction. Typed inventory and diagnostics omit raw payloads and arbitrary error text.

This is not a universal secret scanner. Unknown/obfuscated syntax, arbitrary credential names, multiline credential position tracking and secret-bearing structural fields are outside the contract. Keep paths, IDs, repository scopes and custom metadata keys free of secrets. Public certificate fingerprints/subjects and other retained facts can still require distribution review. Some benign preview text is omitted conservatively. See the [supported syntax, field policy and limits](../architecture/EVIDENCE-PRIVACY.md) and [synthetic-canary verification](../../tasks/029/TASK-029-EVIDENCE-PRIVACY.md).

Repository walking respects Git ignore settings. Default exclusions include `.git`, `node_modules`, `target`, `dist`, `docs` and Markdown files; files larger than 2 MiB are skipped. Test directories are not generally excluded. These scope limits mean a successful scan does not establish absence of cryptography in ignored or unsupported files.
