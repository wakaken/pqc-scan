# Task029 bounded evidence privacy regression design

Base4324d2c31e96ce62de211882192d6c4a8acded6d, clean same branch
codex/scanner-quality-improvements. User authorizes implementation/tests/docs/
independent review/local commit; no push/PR/merge. This is bounded synthetic
regression engineering, not an adversarial repository audit or production probe.
No real keys, valid generated secret material, decryption or external targets.

## Data-flow and scope

Trace source -> Detector evidence/metadata -> legacy normalization/IDs -> actions/
source snippets/CBOM -> native associations -> returned ScanDocument -> writers.
Existing snippet masking and safe typed native capture are not proof that every
freeform report field is covered. First reproduce canary failures; fix only these
bounded collection/export paths. Shared core redaction may cover known private PEM
markers, named credential assignments/metadata keys, HTTP(S) userinfo and long
base64-like fragments under the existing heuristic. Arbitrary opaque strings,
operator paths/rule IDs/configuration and unrecognized formats are not universally
secret-scanned. Preserve safe numeric/OID/algorithm/version/position fields.

Private marker context covers RSA/PKCS8/OpenSSH/EC/DSA/encrypted PEM including
unterminated blocks; no format detection/rule expansion or payload parsing claim.
Source snippets remain filewide masked for private/dependency/SBOM containers.
Context line/range information (no private payload retained) can suppress a
matched body or credential value even when its marker is outside the preview.
Legacy AST columns can be mixed; conservative range envelopes can mask extra raw
text, while normalized facts/IDs/locations remain unchanged. No new raw payload
hash or private fingerprint is introduced.

Normalize finding IDs and native legacy associations before redacting exports.
Then sanitize returned legacy data, preserving IDs, indices, count/severity/risk,
technical metadata and all native reference integrity. Synchronize CBOM copies of
sensitive evidence. The report boundary also sanitizes caller-provided ScanResult
freeform evidence/metadata/actions/code examples/CBOM/dependency strings using the
same policy. Keep all public struct fields, CLI flags, filenames and schemas.
Never rewrite structural IDs/paths merely because they contain a keyword.

## Approach

Choose a small shared core privacy module plus a final pipeline redaction pass and
writer defense in depth. Reject detector-time mutation of match text: it would
change existing IDs and break captured legacy associations. Reject removal of all
metadata/actions/snippets: it would erase benign technical facts. Keep existing
key-rule detection coverage unchanged; privacy marker coverage is not key parsing.
No new rule DSL, universal classifier, network lookup, or compliance claim.

## Validation

Red synthetic canaries across returned core result, all legacy formats, inventory,
assessment, diagnostics, stdout/stderr and both languages. Cover private formats,
multiline/inline/unterminated markers, credential fields/URLs, metadata/actions,
markup escaping, safe public facts and benign long tokens. Control IDs and native
associations across privacy-context changes and saved-binary CLI comparisons.
Fixed safe diagnostics must not echo detector/parser canaries. Library writer
fixtures distinguish untrusted freeform content from fixed operator metadata.

Pinned build/fmt/test/clippy, Node/Python, offline schemas/current golden tests,
source-bound old/new report comparisons, frozen task023 corpus/resource measurements
and CI expectation audit. Preserve corpus labels/rule patterns/task023 budget gate.
Independent source/data-flow review and Evaluator acceptance before local commit.
