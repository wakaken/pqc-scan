# Real-OSS validation study plan

Goal: evaluate and improve the actual PHP, C, C++ and C# scanner against every repository in the original handoff request. Completion requires all four phases, not acquisition or successful scans alone. The source of the original request is copied verbatim into the external handoff; task.yaml preserves the acceptance obligations.

## Architecture and decisions

Keep the dirty v0.5 product baseline intact and snapshot actual bytes before improvements. Store upstream source in `/Users/waka/.cache/pqc-scan-oss-validation-084`, outside the checkout; keep manifests, labels and results outside scan roots. Use shallow Git checkouts at recorded full commits, with hooks and external filters disabled. Do not recurse submodules or hydrate LFS. Record missing nested objects explicitly. Never execute upstream code, build files, tools or cryptographic operations.

Reuse `tests/quality/run.py::measure` for sequential per-child wall/CPU/RSS measurements and `evaluate.py::score` for one-to-one rule/file/line/Unicode-column matching. Do not relax `validate_corpus` or `load_truth`: their MIT/local-materialization/two-label contracts belong to earlier corpora. A separate study adapter may supply validated in-memory inputs to the unchanged scorer; this introduces no product storage/schema change. Existing report language mapping needs a minimal tested extension if used for these inputs; preserve `.C` precedence and explicit ambiguous `.h` accounting.

Alternatives considered: copying full trees into the existing synthetic corpus would misrepresent licenses and materialization limits; a new parallel scoring implementation would duplicate matching semantics. External pinned sources with existing scorer/resource primitives preserve source provenance and require the smallest adaptation.

Annotation is main-authored and source-first, with prior implementation/rule knowledge disclosed. Historical independent annotators are scoped to task023/042/043 corpora, not new agents authorized for this request. This study is exploratory/tuning evidence, never an independently labeled holdout or a practitioner study. Preserve all first labels and baseline outputs. Any later label correction needs old/new rationale and comparable denominator accounting. Reviewers evaluate implementation/tests; they do not become truth annotators.

## Phase 1: freeze and prepare

- [x] Verify all 80 accepted source hashes; snapshot actual tracked/untracked source and baseline binary before changes. Record HEAD, manifest/source/rules/binary digests, toolchain, OS, profile and features. Freeze release/default build separately from existing dev measurements.
- [x] Acquire all 30 requested default-branch tips once; retain full SHA, tree ID, fetch receipts, `.gitmodules`, LFS pointers, regular file byte/hash/mode inventory and license paths/digests. Check license statements and mixed/vendored qualifications without relabeling sources MIT.
- [x] Inspect source-derived API/direct implementation/dependency characteristics for each project. Establish PHP/C/C++/C# and header populations without deriving language from successful findings.
- [x] Freeze eligible truth units and annotation coverage before first scanner-output exposure. Include positive/negative/ambiguous/unsupported and direct implementations or unfamiliar wrappers; do not remove gaps because the scanner does not recognize them.

## Phase 2: baseline scan and evaluate

- [x] Use one fixed release/default scanner, explicit frozen default rules, `--threads 1 --diagnostics --format all`, default 2 MiB cap/exclusions and timeout 600 seconds per process. Run one first sample plus three warm samples sequentially per project; include startup/output time, exclude build. No concurrent benchmark jobs, no OS-cold-cache claim.
- [x] Preserve every exit/timeout/panic/read error/Partial/Unsupported, reports and diagnostics with exact argv/hash bindings. Do not raise parser/file limits to conceal failures. Record file enumeration, selection, applicability and terminal states separately.
- [x] Reuse scorer on explicitly annotated rule/spans. Predictions outside partially annotated ranges are unattributed, not established FP; independent negative hits, duplicate predictions and legitimate method overlap remain separate. Record native coordinates, algorithm, purpose, artifact/evidence class, risk validity and privacy without echoing source arguments or key material.
- [x] Report per-project and language counts with denominators/nulls, plus algorithm/purpose/artifact strata and source-inspection completeness. Partial annotation supports bounded-unit accuracy only.

## Phase 3: improvements

- [x] For each observed defect, establish root cause and general language/API specification before changing behavior. Write failing meaningful positive/negative/position/privacy regression controls, then minimal generic fixes. Retain unknown/reference-only semantics for undecidable facts.
- [x] Keep relevant tests and paired end-user docs with the behavior. Preserve old languages, API/schema/output, rule IDs and sensitive evidence protection. Stop affected work for actual AGENTS human-review conditions; continue independent preparation.
- [x] Inspect combined diff; request implementation review once changes and initial checks are ready. Reuse its identity for material fixes.
- [x] Run applicable fixed-toolchain targeted/workspace/default/optional-feature, fmt, Clippy, build, rules, harness and packaging checks. Reuse existing evidence only with recorded final-source applicability. Do not silently fix the unrelated known Node feedback failure.

## Phase 4: final comparison

- [x] Repeat all 30 scans at identical upstream commits/input paths/profile/features/limits, with one first and three warm samples per project, after final fixes. Reconcile any justified label amendment explicitly; same-input rescans are regression evidence, not a fresh holdout.
- [x] Compare per-project/language/algorithm/purpose/artifact counts, quality, diagnostics, evidence correctness, practical triage usefulness and resource distributions. Preserve failure samples separately from successful timing distributions.
- [x] Request test review after required tests and final-source evidence are available; reuse the same two stage identities for material fixes.
- [x] Reconcile all original criteria and report actual findings, improvements, remaining product limits and proposals. Keep work unfinished for missing truth/measurements/reviews. No release/push/version bump implied.

## Review focus

Mixed licenses and external source roots must not pass through a fictional MIT manifest. Partial annotation must not turn uninspected predictions into FP or support global accuracy claims. Ambiguous headers and `.C` must not pool C/C++. Direct implementations and unfamiliar abstractions remain study gaps. Actual dirty source and exact binary/input/config bindings must survive every comparison.

## Execution rulings

- Repository rules override generic skill instructions for plan location, unrequested commits, agent dispatch and approval pauses. This user already authorized the four-phase execution; routine preparation needs no new approval. Main implements inline; only the two required review stages may delegate.
- New evidence records describe this study and do not change a product or existing corpus storage format. An actual format/semantic compatibility change would require the separate AGENTS gate.


## Confirmed Phase2 defect and next implementation

The corrected PHPMailer baseline misses initial truth unit oss-019 (`src/PHPMailer.php:5273`, `openssl_sign`). Diagnostics retain `php_same_name_bindings`, limit64, Truncated. `php::inspect` returns immediately after `Values::new` if any local binding cap marks Context truncated, discarding independent API references in other functions. The per-variable overflow map already invalidates values after its exact cutoff. Proposed minimal correction: keep local binding-cap state in Values, retain the existing64-record cap and unknown overflowed values, continue using the unchanged global name/candidate budgets, and mark Context Truncated after retaining independently resolvable references. Global structural/name-index limits remain fail-closed. Tests must prove nonzero retained RSA requests in an unaffected function, unknown key types after the overflow, namesake suppression, unchanged Truncated status and canary masking. No limits, rule semantics, storage schema or cryptographic algorithm implementation change is proposed. New regression tests are written; they have not yet run while fixed-binary OSS measurements are active. The baseline CLI/source/rules snapshot remains unchanged.


## Supplemental native technical audit

The reused default-output performance runner writes legacy JSON/CBOM, which deliberately masks or omits canonical typed source assets. To audit algorithm identity and syntactic/reference-only assurance, also capture the existing opt-in `--inventory-json` once per permitted root with a stable repository-name-derived scope. Keep this as a distinct technical-audit stratum in both before/after phases; do not pool it with the one-first/three-warm default-output performance samples or claim repeated native-profile performance. Use the same frozen binary/rules/roots/limits and full before/after input/selection validation. No native schema, operator/business context or policy is changed.


The corrected curl baseline also misses initial unit oss-030 (`lib/curl_ed25519.c:60`): `EVP_PKEY_new_raw_private_key` is absent from the finite C-family constructor list. OpenSSL3.5 primary API documentation defines four raw public/private import constructors, numeric-selector versus `_ex` name-selector argument positions and supported types. Prepare generic C/C++ positive import/Key-to-Context propagation and unknown/wrong-dialect/shadow/privacy tests. Only existing canonical Ed25519/X25519 and exact recognized PQC identities are candidates; no new algorithm implementation or catalog semantics are needed. Do not infer a successful runtime operation from an import request. Tests are written and not yet run.


The corrected MimeKit baseline misses initial unit oss-041 (`DkimSignerBase.cs:337`, `new RsaDigestSigner`). The official pinned Bouncy Castle class in `crypto/src/crypto/signers/RsaDigestSigner.cs` explicitly implements the RSA signature signer interface; the finite C# class map lacks this type. Prepare ordinary/using-alias/resolved receiver, local namesake suppression, nonzero reference and privacy tests, then add only the canonical `Org.BouncyCastle.Crypto.Signers.RsaDigestSigner` to the existing RSA signer kind if those tests confirm the cause. No arbitrary namesake matching, blanket Bouncy Castle support, runtime claim or new rule is intended. Tests are written but not run during the fixed baseline measurement job.

## Supplemental local saturation finding

Supplemental source unit supp-002 (Bouncy Castle SignerUtilities.cs:962, SlhDsaSigner) is omitted while diagnostics name csharp_same_name_bindings64. C# has the same local overflow map and whole-file early return pattern as PHP. Retain the64-record bound and exact overflow cutoff, carry a local binding_limited flag, and report Truncated after inspecting independent references. Do not weaken global name/AST budgets or restore stale parameter variants. Add two RED/GREEN controls covering unaffected RSA/SLH-DSA, exact-variant loss after overflow, local namesakes and privacy. Earlier three-fix release is preserved as candidate source/environment; it has not been used for final OSS scans. Required checks and scoped implementation review must be renewed for affected final source.
