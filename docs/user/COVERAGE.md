# Supported inputs and inspection limits

[日本語](COVERAGE.ja.md) · [README](../../README.md)

This guide describes static v0.3.0. Detected references, evidence and IDs can differ between versions. See the [release notes](../releases/v0.3.0.md) for compatibility and [GitHub Releases](https://github.com/wakaken/pqc-scan/releases) for published assets. Local preparation and Actions artifacts are not published Releases.

Start with the [input overview](../../README.md#targets). This guide gives the supported formats and boundaries behind that table. Support for a file type means the documented subset can be inspected; it does not mean every cryptographic operation or runtime path can be found.

Rule coverage: the RSA-size rule requires an explicit RSA token or recognized RSA-size identifier, not any `Key` substring. It remains nominal matching, not binding or measured key-size proof. `PQC_STANDARD_REFERENCE` recognizes ML-KEM/ML-DSA/SLH-DSA parameter names and X25519MLKEM768, including compact MLKEM/MLDSA symbols. Evidence metadata records `reference_kind` as `name_literal`, `call_reference` or `symbol_reference` based on the eligible AST candidate; none establishes execution. Hash-commented `.conf`/`.cfg` lines are excluded by the lexical comment filter. Generic C/text protocol references remain references, not effective settings.

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

The Rust RSA/JWT dependency rule matches exact Cargo package names `rsa` and `jsonwebtoken` (case-insensitive), including supported manifest, lock and SBOM records. Names containing those strings, such as `universal-hash` or `rsa-helper`, are excluded. An inventory entry is preserved even without this crypto finding; presence does not prove RSA use or enabled features.

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

Certificate evidence metadata includes public certificate/SPKI SHA-256
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
Certificate facts, typed evidence and risk tags determine the findings and
legacy CBOM algorithm strings. Version changes can affect their values and IDs;
see [versions and identity](REPORTS.md#versions-and-identity). No signature,
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

Configuration-reference whitespace handling applies only to regex rules with
`scope: config` and a non-whitespace remainder, not every configuration file.
The exact built-in TLS 1.0 rule instead uses a token capture and reuses delimiters
for adjacent references. Custom patterns/scopes keep their generic ranges. See
[reference positions](SCANNING.md#reference-positions) for the applicable conditions.

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

Selected built-in rules supplement pattern matching with imports, aliases and bindings within one file. The bounded relative ESM case below also follows selected JS/TS files.

| Target | Relationships inspected |
|---|---|
| Node PQC and RSA APIs | Supported crypto imports/aliases, local bindings, literal finite `for...of` lists and synchronously generated RSA keys passed to signing/verification |
| Node jsonwebtoken signing | Explicit namespace/default/named imports or local require; the same sign call's literal algorithm option and consistent header; unresolved/spread/duplicate/computed options stay unknown |
| Python PQC generation APIs | Supported explicit imports/aliases and local bindings that shadow the referenced name |
| Rust PQC APIs | Supported crate/type imports and local ML-KEM key roles and receivers |
| Ruby OpenSSL cipher API | Exact `OpenSSL::Cipher.new` calls; bounded same-class single literal constants, local/instance variables, zero-argument methods, optional parameter defaults and branch alternatives for recognized AES-GCM/CBC names |
| Rust OpenSSL TLS API | Qualified constructor/method-selector references and explicit nested imports/type aliases; no negotiated algorithm inference |
| Java PQC APIs | Supported explicit algorithm requests through `getInstance`; available references are retained in incomplete syntax |

JS/TS relative ESM tracking supports named imports of exported string constants and exported, two-argument functions that return one `jsonwebtoken.sign(payload, key, {algorithm: ...})` call without changing the arguments. The import target must already be selected inside the scan root. Exact `.js`/`.ts`/`.mjs`/`.mts` paths and unambiguous extensionless siblings are supported. Package/path-map resolution, re-exports, directory fallback, dynamic configuration, spreads, conflicting headers, changed/shadowed bindings, cycles, links and excluded files remain unresolved. Module surfaces are restricted to imports, constant string declarations and function declarations; nothing is executed. Discovery has separate limits of 256 relative-path probes (including extension probes), 64 selected files and 4 MiB of retained source bytes per caller. For a wrapper call, a target without the literal package name `jsonwebtoken` cannot satisfy the supported wrapper shape and is rejected before cryptographic-analysis budgets are spent; presence of the name alone proves nothing. Imported constants and initialization validation still use the full checks. Missing extension candidates and unrelated helpers do not consume the cryptographic module budget. Cryptographic validation keeps limits of 4 files on the active import chain (including the caller), 16 distinct modules, 64 KiB per imported module, 256 KiB in total and 100,000 syntax nodes per imported module and 32 module parses and 128 cached export results per caller. Validated export results are memoized; all relative imports must have a supported pure initialization surface, including unused named imports. These are static references, not proof of runtime use.

Short human error phrases in recognised error-producing syntax or an explicitly named error-message array are retained as informational display references (`reference_kind: display_text`), without operational migration actions. Plain algorithm configuration values and arbitrary calls remain candidates. Invalid syntax, unknown/masked spans, interpolation, shadowed error names and unsupported display patterns are not demoted. Names visibly used outside the recognised error callee are treated conservatively as potentially shadowed or escaped, including loop/pattern aliases and imports. This can retain harmless messages as candidates. Interpolated or non-plain strings also make error-callee bindings uncertain, so error-call references in that file are retained conservatively. A single inline Vue script block is supported for this classification; external or multiple script blocks are not. The classifier uses at most 200,000 named syntax nodes and 512 bytes per string and fails conservatively if that bound is exceeded.

Only ordinary instance-method instance-variable flow is modeled; class/eigenclass body variables and class-receiver instance variables stay unresolved. Ruby candidates remain references even when a dynamic option or subclass can override them. Generated accessors, external methods, inheritance and caller argument propagation are not evaluated; an optional default is only a candidate and does not establish the supplied value; repeated assignments stay unresolved. Inside blocks/lambdas/loops, only direct literal arguments are resolved; outer variables and receiver dispatch are not followed. Candidate evaluation is bounded to depth8,16 recognized names and the existing candidate budget. Rust explicit module imports and module aliases are followed for the OpenSSL TLS APIs; wildcard imports and external wrappers are not followed.


Where bindings can be resolved, these checks exclude unrelated namesakes and help strings. Outside the relative ESM case above, they do not resolve computed names, external modules, arbitrary expressions or runtime paths. Other rules can still miss aliases or match display strings/comments. A dependency or an HS256/AES reference alone does not establish RSA use. Exact RSA module/name tokens, Ed25519 type/module references and JOSE ES384/ES512/EdDSA names retain typed algorithm references. ES384/ES512 normalize to ECDSA with P-384/P-521 parameters; their original names remain at the source locations. EdDSA retains only the generic EC public-key family, with unknown curve and primitive purpose. Namespace aliases that are written to or escape into containers/calls/returns remain unresolved. Imports and candidate defaults still do not prove active use; effective RSA key sizes, cipher overrides and negotiated TLS algorithms need separate evidence.

Local analysis uses the `DetectorBudget` limits below. TypeScript declaration signatures and function/constructor/call types have separate parameter scopes; unrelated signatures do not consume one shared name budget. Genuine repeated bindings in one scope retain the existing cap of 64. Additional analysis limits include alias depth 8, finite arrays of 32 elements and lexical-scope lookup depth 128. AST traversal is iterative and bounded by scheduled nodes, so long fluent chains and union types alone do not exhaust a separate depth limit. Exhausted analysis reports `detector_truncated`; syntax errors remain partial. Resolving a reference does not prove successful execution, enabled functionality or completed PQC migration.

This analysis applies when the loaded rule ID, kind, pattern and scope match the default definition corresponding to that binary. Modified custom patterns/scopes use generic matching and do not automatically gain the same alias tracking. The runtime rule directory is still required.

## Repository inspection budgets and encoded text

AST candidates borrow full source slices; only displayed previews are clipped. Defaults allow 262,144 semantic candidates, 2,000,000 scheduled nodes, 4,096 accepted hits per rule/file, 65,536 examined matches per rule/file, and 256 MiB of cumulative candidate text examined across AST rules per file. Exactly reaching a hit cap is distinguished from observing another eligible hit. Real exhaustion remains `detector_truncated`; these are finite resource controls, not proof that a specific number of findings was lost. Library callers can use `DetectorBudget` with `TreeSitterDetector::with_budget` / `RegexDetector::with_budget` to set explicit budgets.

The pinned module grammars are supplemented by a parse-only compatibility view for TypeScript `export type *` (including namespace re-exports) and JavaScript/TypeScript `import defer * as ...`. ASCII modifier bytes are replaced with spaces only in actual top-level module statements; evidence and matching use the original decoded text with unchanged offsets. Other syntax errors retain partial status. This does not evaluate modules or validate runtime behavior.

UTF-16LE/BE and UTF-32LE/BE BOMs are recognized and decoded strictly; BOM-less wide text needs a strong ASCII-lane signature and valid printable Unicode. Malformed BOM-marked wide text is reported as unsupported rather than completed or silently excluded. Detection, source context and redaction use the same decoded text. Binary certificate/private container bytes remain opaque to source detectors, and no decryption is performed.

Default pre-inspection policy excludes known media, archives, compiled data (including WebAssembly, MO and protobuf-style extensions), and non-text binary input. Protected certificate/key/store extensions remain selected. The explicit opt-out is `--include-non-scannable`; `.gitignore`, directory exclusions and the configured file limit (default 2 MiB) still apply. Do not infer full repository enumeration from selected counts.

Dependency structured-value decoding is bounded at 500,000 values and depth 64; aliases and explicit tags remain unsupported. YAML plain/quoted/block scalar punctuation is distinguished from node properties, and empty metadata values are valid nulls.

pnpm single-document locks and the boot/project two-document layout are both supported. Both graphs are inventoried separately with globally unique record ordinals; matching schema versions and at most two documents are required. The 500,000-value and 10,000-record budgets apply to the entire file, not separately to each document. See [pnpm’s lockfile reader guidance](https://github.com/pnpm/pnpm/blob/main/pnpm/docs/lockfile.md).

Dockerfile routing uses the final filename (`Dockerfile` or `Dockerfile.*`) plus a first meaningful `FROM`/`ARG` instruction, skipping comments and a BOM. Ordinary language source with the same filename keeps its language parser; lexical rules continue on Dockerfiles. Bundler platform suffixes require a matching `PLATFORMS` declaration (including Linux GNU normalization), and only registry records gain a resolved base version; Git/path records remain unresolved. Compatibility for empty type-alias parameter lists, type-context `import(...)` and quoted JSX attributes is parse-only. Runtime imports and expression operators are preserved, malformed inputs retain partial status, and compatibility work remains bounded.

### Static context and conditional declarations

Ruby dependency declarations can use a single top-level, unmodified literal version variable or the exact `File.read(File.expand_path('relative', __dir__)).strip` form assigned to that variable. Version files must already be selected inside the scan root; links, ignored/out-of-root files and absolute paths are not read. At most eight distinct input paths are read per manifest, and accepted version files contain at most 128 ASCII bytes. Reassignments, mutation, escaping variables, external Ruby loading/evaluation, calls outside the supported declaration/metadata/read-only subset and unsupported expressions remain unresolved. A top-level bare `gemspec` directive checks one selected sibling with the same read boundary, without recursively delegating again; repeated directives reuse one child inspection, and dependencies retain the sibling file location. This extracts a declared constraint only, never an installed/resolved version or executable Ruby result. Static quoted declarations remain available when other values are unresolved.


An initial sequence of literal `require_relative` directives can be checked without executing Ruby when each selected `.rb` target contains only comments, nested module definitions and literal `VERSION` assignments. Extensionless references use the `.rb` source suffix; native extensions, general `require`, dynamic load paths, other code and directives after other statements remain unresolved. Targets are memoized, with at most eight paths, 16 KiB per target, 4,096 scheduled nodes and eight module levels. Imported constants are not propagated as dependency versions. See the [Ruby loading reference](https://docs.ruby-lang.org/en/3.3/Kernel.html#method-i-require_relative).

Maven parent properties are inherited only from selected, bounded regular files inside the scan root, after matching the declared group/artifact/version. Local references honor empty relativePath, reject links and excluded/out-of-root files, and stop at 16 levels or 8 MiB of parent input. Conflicting properties remain unresolved; identical duplicates are idempotent for declaration inspection. Profile property candidates are labelled conditional (`dep_context`, native observation capture status and dependency uncertainty), never selected as an effective configuration or a resolved artifact. No Maven execution, BOM fetch, transitive graph or installed-package claim is implied.

Ruby Gemfile/gemspec inspection reads literal dependency calls and supported groups/platforms/branches using the Ruby syntax tree. A gemspec receiver must be bound to the specification block; rebound/ambiguous receivers, dynamic versions/names and evaluation/loading remain incomplete. Delegated gemspec dependencies keep the gemspec's own path and positions instead of being copied to the Gemfile. Git/path sources stay unresolved. Unknown DSL calls remain incomplete; arbitrary Ruby is not evaluated.

Recognized unrendered template directives have an offset-preserving static parse view. Requirements branch declarations are retained separately as conditional candidates. Template rendering/branch selection is not executed, and generated values remain partial even when the static source is parsed. Literal strings, comments and regular expressions do not become source directives solely by resembling a template.

Selected V8 grammar compatibility covers native-call expression positions, sloppy-script `yield` identifiers, tagged-template escape content and legacy runtime-error assignment targets. It does not execute those calls or assert runtime success. Explicit module/strict boundaries and malformed controls remain incomplete. Legacy DSA OID 1.3.14.3.2.12 uses the known public-parameter/public-integer layout; absent parameters retain partial status and no key size or domain parameters are invented. Sources: [OpenSSL object identifiers](https://github.com/openssl/openssl/blob/master/crypto/objects/objects.txt), [OpenSSL DSA decoder](https://github.com/openssl/openssl/blob/master/crypto/dsa/dsa_ameth.c), [Maven POM reference](https://maven.apache.org/pom.html), [Django template language](https://docs.djangoproject.com/en/5.2/ref/templates/language/).

<a id="privacy"></a>

## Privacy and excluded files

Recognized private-key blocks and credential literals are masked in exported evidence. The policy covers RSA, generic PKCS#8, OpenSSH, EC, DSA and encrypted private-key markers; single-line named password/token/key literals; Bearer/Basic tokens; and HTTP(S) userinfo and named credential query values. Marker masking does not add private-key parsing or decryption support.

Previews are conservatively masked when recognized private/credential context is present. Metadata values, action text/code, CBOM and dependency text also pass through bounded redaction. Typed inventory and diagnostics omit raw payloads and arbitrary error text.

This is not a universal secret scanner. Unknown/obfuscated syntax, arbitrary credential names, multiline credential position tracking and secret-bearing structural fields are outside the contract. Keep paths, IDs, repository scopes and custom metadata keys free of secrets. Public certificate fingerprints/subjects and other retained facts can still require distribution review. Some benign preview text is omitted conservatively. See the [supported syntax, field policy and limits](../architecture/EVIDENCE-PRIVACY.md) and [synthetic-canary verification](../../tasks/029/TASK-029-EVIDENCE-PRIVACY.md).

Repository walking respects Git ignore settings. Default exclusions include `.git`, `node_modules`, `target`, `dist`, `docs` and Markdown files; files larger than 2 MiB are skipped by default. CLI scan/serve and the folder picker accept `--max-file-size-mib` / a size field from 1 to 64 MiB. This changes selection and bounded primary reads only; secondary reference readers and detector-specific budgets retain their own limits. Test directories are not generally excluded. These scope limits mean a successful scan does not establish absence of cryptography in ignored or unsupported files.

`--exclude-tests` optionally omits conventional test paths and filenames; defaults still include tests. Syntax-proven Rust test-only findings are also omitted; other inline tests remain. See [selection conventions](SCANNING.md#exclude-test-code).

Versionless legacy Cargo.lock package inventories are supported when `[[package]]` is present and the historical `root` table is absent. Unknown format versions and unsupported shapes are not declared complete.

Ruby cipher constants are candidates only from a single literal definition in the same class/module body. Redefinitions, compound/multiple assignments, qualified writes, methods called on the constant value or direct string mutation, detected `const_set` or dynamic evaluation remain unresolved. A constant escaping to another variable, container or arbitrary call is unresolved; only its direct argument use in the supported OpenSSL cipher constructor is modeled. Inheritance, external files and reopened class bodies are not connected.

### Concrete unfinished-inspection diagnostics

The existing diagnostic `message` identifies known causes and next checks: unsupported encrypted PKCS#12 contents, absent DSA parameters, dynamic dependency declarations, unrendered templates, no-follow symbolic links, source syntax and bounded work. Known limits include their configured value and completed work, not an invented number of missed findings. Some older inspection paths still report a bounded category without exact counters. Partial findings are retained. Diagnostic record counts and distinct file counts are separate; a file may have several detector records. Symlink policy records retain their legacy diagnostic code/counter for compatibility, while their explanation distinguishes intentional no-follow handling from an I/O error.
