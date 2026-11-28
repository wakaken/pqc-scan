# Evidence privacy contract (task 029)

This contract covers bounded, recognized syntax in local static scans and report exports. It is not a general secret detector. All regression payloads are synthetic, invalid key material; no real keys, decryption or protocol connections are needed.

## Collection to export

1. Core records only private-block line ranges and named credential value positions from the original immutable source snapshot before detector callbacks. Payloads and secret digests are not retained in this context index.
2. Detection, framed finding ID computation, summary/CBOM construction and native observation association run before evidence redaction. Finding IDs use the original rule, relative path, position and evidence match; public schemas remain unchanged.
3. Core sanitizes exportable free text after native associations are built. Sensitive matches and surrounding previews are replaced. Any file with recognized private/credential context has its detector preview replaced because a detector may already have shortened it and destroyed redaction syntax. Source windows and supported call excerpts come from the inspected snapshot, with private/credential spans redacted before clipping. Public neighbours remain visible instead of masking private-marker files wholesale; explicit detector fallback and protected dependency/binary containers remain conservative. Display restoration runs after identity/native/CBOM construction and never supplies algorithm identities.
4. CBOM keeps original row counts/order. Sensitive replacements use location/category/original match, and a sensitive occurrence wins a column-less collision. Rows are not rebuilt or deduplicated.
5. Evidence match/preview/snippet fields retain the legacy writer's broader protection for fragments containing `PRIVATE KEY`, even when BEGIN/END was lost. Report writers sanitize a cloned legacy result again, including caller-constructed results; the caller is unchanged. Such results have no original source snapshot and therefore receive syntax-based and explicit private-key evidence masking only.

| Data | Export policy |
| --- | --- |
| Evidence match/preview, source line text | Recognized syntax redaction; source context additionally protects core matches/previews |
| Evidence metadata values | Known credential field names mask the complete value; other values get syntax redaction |
| Description, migration hint, action title/rationale/steps/references and code text | Syntax redaction before rendering |
| CBOM algorithm/hint; dependency name/version/ecosystem/source type/purl | Syntax redaction and sensitive CBOM occurrence mapping |
| Native inventory, assessment, diagnostic errors | Existing typed allowlists/fixed error codes; raw detector errors are not exported |
| IDs, rule/category/risk/severity, confidence, paths, structural metadata keys, code language | Stable structural fields; operators and extension authors must not place secrets here |

## Recognized syntax

Private BEGIN/END markers: RSA, generic PKCS#8 PRIVATE KEY, OpenSSH, EC, DSA and ENCRYPTED PRIVATE KEY. Unterminated blocks protect through EOF; orphan END markers conservatively protect preceding lines. This is masking coverage, not valid-key parsing or an expansion of key detection formats.

Named literal assignments support password/passwd/secret/token/api_key/apikey/api_token/access_token/client_secret/private_key/key_material/authorization/secret_key/secret_access_key, case-insensitively, with underscore/hyphen prefixes. Single-line quoted literals (including escaped characters), quoted PHP/Ruby-style credential keys with `=>`, bounded ASCII bare literals, Bearer/Basic tokens, HTTP(S) userinfo and named query values are recognized. Quoted-key handling does not treat ordinary JavaScript arrow parameters as credential assignments. Supported source ASTs also protect literal credential/key array initializers under the same finite field-name policy; computed expressions keep their public arguments. Named credential initializers inspect up to256 wrapper levels and conservatively protect the right-hand side beyond that limit. Caller-provided metadata retains the existing arbitrary-prefix credential suffix policy, including dotted and Unicode prefixes. Query values extend to an ampersand, fragment or textual delimiter and include dots/percent encoding. Code expressions and common keywords are preserved. Existing long base64-like whole fragments also receive conservative masking.

Source positions account for Unicode character columns; legacy AST adapters may mix byte/character columns, so their protection envelope is conservative. Unknown locations are conservative. A truncated match on a credential line is protected; an untruncated match is checked for overlap. Technical identifiers outside sensitive spans, key sizes, OIDs, versions, risk values and references remain available. Masked raw evidence/preview text intentionally differs from previous releases.

Structured presentation masking also recognizes finite private/shared/seed/scalar field names, including private-key JWK/PEM/PKCS#8/byte containers and private exponents. Quoted, numeric, array and object literals are protected, including multiline literals and supported literal byte wrappers. Number-like literal tokens cover decimal, hexadecimal, binary and octal digits with separators and decimal exponents. Identifier-like suffixes remain opaque to presentation masking, including Rust typed suffixes, C# decimal suffixes, Java long suffixes and JavaScript BigInt suffixes. A suffix must follow the numeric part without a member-selection dot; dotted selectors, operators and calls remain expressions. Parentheses accept literal-only tuples and grouping; literal-prefix computed expressions remain visible. Comment-aware tokens track nested block-comment depth iteratively so comment delimiters cannot terminate a source container or alter object-local JWK association. Comment documents anchor recognized private/JWK field names and literal values independently of prose quotes and preceding braces. Unknown prose labels, including quoted description text, do not establish value scopes that hide later recognized private fields. Quoted contents inside an anchored literal container retain their own lexical scope. Comment fields do not associate with executable source objects, and public parameters remain visible outside protected private containers. Short JWK fields `d`, `p`, `q`, `dp`, `dq`, `qi` and `k` are protected only in an object carrying `kty`; ordinary mathematical variables remain unchanged. The RSA private `oth` container is protected as a whole under the same object-local JWK context ([RFC 7518 section 6.3.2.7](https://www.rfc-editor.org/rfc/rfc7518#section-6.3.2.7)); `r` and `t` are not global private names. Quoted source data is inspected independently of quote kind, language prefixes or whether it is used at runtime. Direct quoted-body ranges retain public parameters outside private values. JSON-escaped structured text is inspected through at most two nested string decodings; decoded content with private ranges conservatively withholds the complete original literal because decoded offsets do not map directly to source bytes. Raw quoted-body inspection has a maximum of eight nested levels and a shared visited-byte budget of eight times the original text length, including comment rescans and decoded text. A region whose byte budget is exhausted, or a syntax-bearing quote at the depth limit, is withheld conservatively. This fallback can also remove public text inside that region; it never restores an uninspected private body. These source ranges are presentation-only and do not change native algorithm suppression, detection, identity or classification. Source snapshots are masked before excerpts are clipped. The pure `redact_source_text` helper preserves line terminators but replacements can change byte and column offsets; consumers must use original line mappings rather than slicing the masked string at original byte offsets.

OpenSSL-style textual dumps beginning with a `Private-Key:` header, optionally preceded by an algorithm name such as `ED25519`, are presentation-sensitive. Token traversal withholds the header and remaining enclosing inspected quote/comment region, or through EOF when no lexical end is known. This includes colon-separated hexadecimal bodies and escaped newlines inside strings. Source, literal-data and comment-data regions retain their ownership during traversal. A printed dump header followed later in the same owned data by a physical private-component line is checked before parsing child quote tokens, so textual inner quotes cannot expose the remaining private body. Public `modulus` and `publicExponent` lines may precede the private component. The component registry is `priv`, `privateExponent`, `prime1`, `prime2`, `exponent1`, `exponent2` and `coefficient`, case-insensitively, with optional comment-line decoration. This check uses bounded linear header and component searches; it does not require adjacency. Python display checks operate at the current decoded layer, preserving the boundary of a still-encoded serialized child. The containing owned region can be withheld conservatively, including public parameters inside that same region. Text outside its proved closing boundary remains visible. Text outside the bounded region remains available. A `Public-Key:` dump or standalone hexadecimal text does not trigger this policy. Dump ranges are export-only and do not enter legacy credential ranges or native algorithm suppression.

## AST key-input ranges

Ruby EC private-key setters also protect supported literal inputs and their
explicit scoped initializers. The adapter follows recognized
`OpenSSL::PKey::EC.new` / `.generate` receiver aliases and the first non-comment
argument of `OpenSSL::BN.new`. It indexes lexical bindings before resolving
inputs. Loops share their surrounding local bindings; blocks can capture outer
locals, while parameters, explicit block locals and method/class/module barriers
remain separate. These scope distinctions follow the
[Ruby assignment specification](https://docs.ruby-lang.org/en/4.0/syntax/assignment_rdoc.html#label-Local+Variable+Scope).

Identifier tokens come from the already bounded source snapshot. They do not
use the shorter general-purpose syntax-preview limit. AST traversal and alias
following remain finite. Unresolved expressions can conservatively protect
literal initializers in the affected lexical regions. This is static source
protection, not runtime key-use analysis or a general secret detector.

AST key-input ranges participate in the existing detector sensitive-range
policy, including suppression of overlapping algorithms on observations without
the `source_api_typed` classification. They are distinct from export-only
presentation ranges described above. Compatibility checks therefore verify
findings and native associations as well as redacted text; snippet changes alone
do not prove native equivalence.

## Limits

Opaque/obfuscated content, arbitrary secret names, encoded field names, unsupported assignment syntax and secret-bearing structural fields are outside this bounded contract. Free-form caller results cannot recover private block context lost before construction. Public certificate fingerprints/subjects/endpoints are existing typed facts, not automatically secret; consumers must assess their own distribution policy. URLs may remain public and identifiable after credentials are removed. Conservative fallback can remove benign text in an uninspected source region. Supported PHP/C#/C/C++ key-input roles protect literal key expressions and explicit same-function initializers (bounded alias following, with conservative literal-initializer protection when alias/expression/node limits prevent resolving a key input); public messages, variable references and selectors do not establish new typed crypto facts. Export-only presentation privacy does not change the existing native suppression policy. Named literals include camel-case spellings and multiline quoted values. Canary success proves the exercised paths, not universal absence of secrets.

See [task evidence](../../tasks/029/TASK-029-EVIDENCE-PRIVACY.md) for validation and independent review.
