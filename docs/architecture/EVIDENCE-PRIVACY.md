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

## Limits

Opaque/obfuscated content, arbitrary secret names, encoded field names, unsupported assignment syntax and secret-bearing structural fields are outside this bounded contract. Free-form caller results cannot recover private block context lost before construction. Public certificate fingerprints/subjects/endpoints are existing typed facts, not automatically secret; consumers must assess their own distribution policy. URLs may remain public and identifiable after credentials are removed. Conservative fallback can remove benign text in an uninspected source region. Supported PHP/C#/C/C++ key-input roles protect literal key expressions and explicit same-function initializers (bounded alias following, with conservative literal-initializer protection when alias/expression/node limits prevent resolving a key input); public messages, variable references and selectors are presentation evidence, not new typed crypto facts. Native algorithm suppression is independent from presentation privacy, preserving the existing source-handler-only inventory policy. Named literals include camel-case spellings and multiline quoted values. Canary success proves the exercised paths, not universal absence of secrets.

See [task evidence](../../tasks/029/TASK-029-EVIDENCE-PRIVACY.md) for validation and independent review.
