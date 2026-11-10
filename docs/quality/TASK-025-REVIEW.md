# Task 025 independent review record

Final source review: **APPROVED**. Reviewed against base
`8ba863ff335023b3e3a33affa7cb250b3f7bec25` on branch
`codex/task-022-sarif-location`. This records local source/design review,
not remote CI or publication. Initial findings below are historical and were
resolved by the final follow-ups. Coordinator execution evidence is recorded in
[TASK-025-NATIVE-SCAN.md](TASK-025-NATIVE-SCAN.md).

# Task 025 independent design review

Date: 2026-11-10. Role: independent Reviewer/builder. This is a source/design
review before product implementation, not build, schema or runtime acceptance.

## Decision

The additive architecture is appropriate. Retaining legacy normalization and
collecting drafts inside the existing detector execution is preferable to an
adapter over final findings. No new crate, replacement storage contract, new
cryptographic interpretation or architecture escalation is needed for this scope.
Proceed after recording the concrete resolutions below in the implementation
contract. These are material implementation requirements, not requests for a
larger feature or a new user approval round.

Reviewed: AGENTS.md, prompts/REVIEWER.md, task 025, both READMEs, architecture and
knowledge documents, DOMAIN-MODEL.md, NEXT-GEN-ARCHITECTURE.md, native-v1 design
and plan, QUALITY-GATES.md; current core pipeline/Detector/DetectorOutcome/
ScanDocument and source/dependency/certificate capture boundaries. RESTART-LOOP
explicitly applies only to the restart baseline/task 014, not this task.

## Required resolutions

1. **Preserve legacy status when collecting inventory without matching rules.**
   `CertificateDetector::detect_with_time` currently returns NotApplicable before
   parsing when no certificate rules exist. Native collection must still capture
   supported parsed objects without a rule match, including an empty ruleset.
   Bypass that early return only for observation collection, and keep the legacy
   status/detections unchanged. Otherwise enabling native changes coverage,
   warnings and `--require-complete` exit behavior. Make native capture limitations
   separately explicit instead of silently promoting legacy completeness. Test
   empty certificate rules, parsed-but-unmatched, malformed and mixed bundles.

2. **Define the exact safe conversion boundary, including custom detectors.**
   `detect_observed` should default to one `detect_detailed` call, retain its
   status, and make Unknown/Unknown drafts without reading arbitrary metadata as
   typed facts. Explicit opt-in typed constructors may state assurance; document
   that these are detector assertions, not verified third-party trust. Built-ins
   classify allowlisted tokens immediately at their accepted-hit site, before
   `mask_preview`; AST candidate text/regex match must never become retained raw
   draft fields. A helper over final Detection cannot recover masked identifiers.
   Reuse the same parse result in each built-in, including fixed-clock certificate
   detectors. Test one invocation per detector/file and both old API literals and
   external detector default adaptation.

3. **Specify omission versus unknown typed facts.**
   Exact algorithm identities require explicit unknown purpose/parameters rather
   than invented values. Unknown syntactically parsed numeric OIDs may remain safe
   certificate facts, with no guessed algorithm asset; arbitrary unknown source
   tokens must be omitted. RSA-PSS digest parameters are currently not fully
   interpreted and must not turn into a precise digest/parameter assertion.
   Preserve parsed certificate/SPKI fingerprints even with incomplete attributes;
   mark incompleteness and omit unsupported derived claims. Never copy the
   existing arbitrary parameter-content digest as a new identity component merely
   because it already exists in public parser attributes.

4. **Make compatibility association transformation exact and collision-safe.**
   Existing findings relativize paths after hashing/dedup, while transient hits
   carry absolute paths. Match bindings using the same path transformation and
   evidence match semantics as normalization, after final finding sorting. Do not
   include preview/metadata fields that snippet enrichment later mutates. Retain
   every matching final index plus ID, never a map keyed only by ID. Preserve
   multi-method observations even when the old dedup collapses hits. Unknown-rule
   hits and dependency inventory pseudo-hits get explicit, different loss reasons;
   neither needs a fake finding. Test equal old IDs on distinct records and
   old delimiter-based dedup collisions separately from ordinary duplicates.

5. **Bound and validate every new identifier input, not only evidence.**
   Native source targets must be repository-relative from the actual scanned file;
   do not trust a custom Detection's outside-root/absolute path. Reject or omit
   unsafe paths with an explicit loss instead of leaking an absolute root. Validate
   fingerprint length/hex, numeric bounds, package grammar/version token length
   and scope before both identity construction and serialization. Never export
   package URLs, credentials, raw source or arbitrary metadata. A grammar-valid
   package/path/rule name can itself be operator-sensitive: document the permitted
   identifier surface honestly rather than claiming general secret detection.
   Canary tests must cover invalid package/version/metadata/endpoint payloads and
   ensure omitted payloads do not survive through hashes or loss-note text.

6. **Freeze duplicate ordering and ID semantics before the first golden.**
   Sort drafts by relative target, method, rule and a total typed-fact order, with
   physical offset/position as a local tie-breaker for identical weak anchors.
   Exclude line/offset from the correspondence hash where a structural anchor
   exists. Flag every member of an ambiguous duplicate group, not only the second
   record. Occurrence keys remain absent without scope; usage asset IDs must then
   include run identity so they cannot imply cross-run matching. Specify that
   deleting/inserting an identical duplicate can reassign ordinals. Explicitly
   sort/deduplicate parameter tuples before hashing. Observation IDs include a
   run/attempt discriminator but record ordering must not depend on that value.

## Minimum acceptance evidence

- Old detailed/native shadow equality for all six legacy artifacts after the
  documented volatile-field normalization; default output filename set unchanged.
- Native schema/golden/unknown-reader and unique/resolving references; offline
  validation uses the already pinned dependency, without schema network access.
- One key shared by two different public certificates, independent signature and
  subject algorithms, zero-rule parse facts, malformed partial retention.
- Root relocation, Unicode paths, line insertions, file moves, repeated anchors,
  explicit/no scope, tuple ambiguity and same old ID with multiple final indices.
- Accepted source matches classified before masking, unknown/draft identifiers,
  canaries through all new fields and IDs, third-party metadata forgery tests.
- Native CLI opt-in/language/exit precedence, fixed-clock tests, bounded outcomes,
  legacy dependency NotReported behavior and no repeated detector/parse execution.

No product files, task statuses, legacy outputs or rules were modified by this
review. No build/tests/benchmarks or remote CI were run; those remain final
implementation acceptance gates. The sole review artifact is this file.

# Task 025 independent correctness review

Date: 2026-11-10. Review target: uncommitted additive native-v1 implementation
relative to base 8ba863ff335023b3e3a33affa7cb250b3f7bec25. Initial review;
implementation and acceptance tests are still being completed concurrently.

## Decision

Changes requested before acceptance. The additive pipeline, private legacy
bindings, default Unknown adapter and index-based legacy links are sound choices.
The following are concrete gaps against the accepted design, rather than requests
for an architecture change. No product files or task statuses were edited.

## Findings

### P2 — Do not use uninterpreted parameter bytes as exact algorithm identity

`crates/pqc_scan_core/src/inventory.rs:207-210` copies
`parameters_content_sha256` into AlgorithmSpec.parameters and therefore the new
exact algorithm asset ID. The parser's digest is a fingerprint of unparsed ASN.1
parameter content, not an interpreted algorithm parameter. This directly
contradicts design-review resolution 3 and the accepted concrete specification.
For unknown or sequence/PSS parameters retain the public SPKI/certificate asset
fingerprints, omit this digest from algorithm identity, and explicitly mark
parameter interpretation unavailable. Add a regression that varying an unparsed
parameter-content digest cannot manufacture precise algorithm distinctions.

### P2 — Preserve container parse limitations when good certificates survive

`crates/pqc_scan_detectors/src/certificate_detector.rs:286-294` uses only each
certificate's `info.status` for capture status, discarding the aggregate status
from `parse_der_object` / `parse_pem_objects`. A valid DER certificate with trailing
bytes, or a PEM bundle with one good and one malformed block, consequently has
only `complete` native observations. With no certificate rules, legacy status is
correctly NotApplicable, so no other field records this known capture failure.
Keep the good parsed certificate and emit a separate fixed container capture issue
(or equivalent explicit native limitation) without changing legacy status.

Runtime reproduction with current `target/debug/pqc-scan`: copy checked-in
`rsa2048.der` to `/tmp/pqc-native-review025/input/trailing.der`, append NUL, SOH
and `garbage`, and scan with only `rules/default/jwt.yml`, `--native-json --format
json`. `/tmp/pqc-native-review025/out/scan.native.v1.json` has one
`certificate_parse` observation with `capture_status=complete`, empty uncertainty,
and no certificate diagnostic. Legacy diagnostics contain only dependency
NotReported. This is narrower than the fixed global `not_exhaustive` disclaimer:
a known malformed-container signal has been lost.

### P2 — Bind native source targets to the file actually being inspected

`crates/pqc_scan_core/src/pipeline.rs:262` accepts draft paths without consulting
`file.path`; `inventory.rs` later only checks whether the self-reported Detection
path is lexically relative/in-root. A custom detector processing `app.rs` can
report `other.rs` or a different absolute path under the root and obtain an
apparently valid scoped occurrence for that other file. The design explicitly
requires targets derived from the actual scanned file. Validate draft path
against the inspected file at collection and omit/reject mismatches with a fixed
uncertainty reason. Keep legacy Detection locations unchanged for compatibility.
Test forged relative as well as outside-root absolute paths.

### P2 — Explain why legacy conversion did not produce an association

`crates/pqc_scan_core/src/inventory.rs:314-318` reduces every unassociated draft
to its observation ID. No per-observation reason or count distinguishes expected
certificate parse-only/dependency inventory-only observations from unknown-rule
hits and delimiter-dedup losses. The fixed export-loss notes do not describe
these cases. Preserve a bounded reason from the compatibility projection and
include counts/reasons in the native contract; no raw rule/match/metadata text
should be needed. Regression fixtures should independently cover unknown rules,
inventory pseudo-hits, parse-only facts and legacy delimiter collisions.

## Additional validation note

The dependency version allowlist in observation.rs rejects ordinary prereleases
such as `1.2.3-beta.1` and PEP440 `3.0.0rc1`, although these are safe bounded
version tokens. This loses native reported version and merges package identities
without a specific omission reason. Review the intended version grammar and add
ordinary prerelease and malicious URL/credential cases together.

## Review coverage and limits

Read the required repository documentation, Reviewer prompt, task, accepted spec
and design review; inspected new core modules, pipeline, detector capture changes,
report/CLI surface and schema. Executed only the targeted malformed DER CLI
reproduction above. The parent's full workspace tests, clippy, schema/golden,
legacy-shadow and performance evidence remain independent acceptance gates; this
initial review does not claim those checks passed. A final follow-up should assess
fixes and completed evidence.

## Follow-up review after first corrections

The first four findings are resolved in the reviewed source: opaque parameter
hashes no longer define algorithms; aggregate incomplete certificate capture gets
a separate container observation; drafts are bound to the inspected file before
native normalization; and the compatibility ledger includes fixed reasons,
counts and units. Safe prereleases are now retained, reference validation is
available, and the writer remains accessible through captured ScanDocument only.
Reviewed the added regression tests and core-fixed/fixes-tests logs.

Two additional P2 issues were reproduced against the rebuilt CLI:

### P2 — Give asset ordering a stable total tie-breaker

`inventory.rs` sorts assets only by `(kind, attributes.to_string())`. Different
rules/producers can produce usage assets with identical attributes (same path,
method, local ordinal and ambiguity), whose IDs differ. Without repository scope
those IDs incorporate run identity. The stable sort therefore retains the
BTreeMap's volatile ID order, violating the promised run-independent record
ordering. Add a stable typed anchor/descriptor tie-breaker that does not derive
from run-local IDs, and test two distinct rules matching the same file/match
across different run values.

Runtime reproduction: `/tmp/pqc-native-order025` contains one RSA match and two
regex rules RULE_ONE/RULE_TWO. Eight consecutive CLI scans without scope produced
usage owner orders TWO/ONE, TWO/ONE, ONE/TWO, ONE/TWO, ONE/TWO, ONE/TWO,
ONE/TWO, TWO/ONE. Both assets had identical sort attributes.

### P2 — Enforce package grammar before serialization and hashing

`observation.rs:206-225` permits `/` in any dependency name and only validates
slash structure when `@` is present. Npm dependencies named
`/SECRET_CANARY/absolute` and `foo//bar` consequently become native dependency
assets and new identity inputs. These are not valid npm package identifiers;
the former is an absolute-path-shaped value explicitly excluded by the safe
identifier design. Apply ecosystem-appropriate identifier grammar and reject
absolute, empty and dot path segments before both identity and serialization;
retain valid scoped npm, Maven and Go names. Invalid names need a bounded loss
indication without copying rejected data.

Runtime reproduction: `/tmp/pqc-native-package025/input/package.json` declares
those two names with version 1.0.0; the native output under its `out` directory
contains both verbatim as dependency attributes. Only temporary fixtures and this
review document were written; no product source was changed. Review decision
remains changes requested pending these corrections and final acceptance evidence.

## Final source follow-up

Both additional P2 findings are resolved: usage assets include the safe typed
anchor, making their ordering independent of volatile run IDs; dependency names
are validated using ecosystem-specific forms before asset construction. Added
regressions cover eight no-scope run values, ordinary prereleases, invalid path/
credential shapes and valid scoped npm/Maven/Go names. Reviewed the updated schema,
semantic golden, tolerant-reader tests, both README additions and
`docs/architecture/NATIVE-SCAN-V1.md` against the implementation.

Independent commands executed after these corrections:

- `cargo test -p pqc_scan_core -p pqc_scan_detectors -p pqc_scan_cli --test native_capture --test native_inventory --test native_json`: 7 passed, 0 failed; includes offline schema/golden and CLI legacy-shadow comparison.
- `cargo test -p pqc_scan_core --lib`: 48 passed, 0 failed; includes stable ordering, framed identities, reader/reference checks and safe package/version tests.

No remaining blocking correctness/security findings in the reviewed changes.
Source review is approved. Final completion still requires the coordinator's
workspace format/test/clippy and recorded CI/self-scan/performance evidence; these
are not asserted by this independent source approval. A minor npm single-dot
identifier edge was communicated separately for consistency (the general name
filter already rejects double-dot).

## CI expectation-registry review

Independently compared `docs/quality/SELF-SCAN-EXPECTATIONS.json` with the base
commit. Exactly seven existing source-file registrations changed; 118 existing
result entries were rebound. Every paired entry preserves path, reason, rule,
column and count. For all 118 entries, the old and new referenced source lines
are byte-for-byte identical and remain within the existing `#[cfg(test)]` module.
No file/result registrations were added, and every current registered source
SHA-256 matches the actual file. No new native module or native integration-test
path appears in the omission audit.

Independently reran `prepareCodeScanning()` against the supplied raw self-scan,
current policy and repository root. Its complete audit exactly matches the saved
`target/task025/code-scanning-audit.json`: 814 raw, 660 expected, 154 retained;
all 99 registered sources verified. The 652 distinct omission entries sum to 660
results. Both the raw SARIF byte digest and canonical policy digest verify. The
policy digest intentionally hashes canonical JSON, not the policy file's bytes.

The reviewed registry changes do not hide production observations or register new
test observations. Approved as a mechanical rebinding of existing reviewed test
inputs. These counts refer to the supplied self-after/audit snapshot; the final
regeneration after additional tests must record its own final counts. The npm
single-dot boundary is now rejected and covered by a regression.

Final dependency omission-status adjustment reviewed: when validated dependency
facts are unavailable, the draft now emits the fixed
`unsafe_or_unavailable_dependency_identity_omitted` capture status. It retains no
rejected payload and makes the omission explicit. The package-grammar regression
asserts this status for rejected names. Independently executed
`cargo test -p pqc_scan_core package_grammar_rejects_paths_and_credential_shapes_before_identity`:
1 passed, 0 failed. Approval remains unchanged.
