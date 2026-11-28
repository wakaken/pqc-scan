# Task084 OSS validation results

Status: **completed bounded OSS evaluation**. All30 named projects have before/after OSS measurements:28 materialized repository roots plus2 explicitly licensed subsets. These are not30 complete mixed-license roots. The commercial/unselected portions and missing parent context remain outside the measurements. The original request does not require non-OSS commercial code; the earlier extra full-root approval condition was withdrawn after checking the exact request and task criteria. No user permission for commercial inclusion is inferred. No release, commit, push or PR occurred.

## Source and execution

HEAD `7e8cd91426fa21609a91c0511de44252eaa890d4` plus the original dirty/untracked v0.5 source was preserved before improvements. BASELINE-SOURCE.json identifies the external archive and complete byte bindings; rechecking HEAD alone cannot restore that baseline. Final owned delta: IMPLEMENTATION-SOURCE-03.json and combined-implementation-03.patch,15 files. Rules/dependencies/public schemas and package version0.4.0 are unchanged.

Baseline release/default CLI SHA-256 `100052dd2549b18c71f0adc9edad09884b1ac5af96f1a01ae35a5ee207dffa0a`; final `dd2ce0b3fa9babc4f09e57c2c2532c7e86726fa79c62849b779280479cdf75c1`. Final runtime/test-source digest `cf59f96589ecbf0ea87c0f3079b8e12cc774f905c9e57d7061efa00b319adee5`. Rust1.93.1, CARGO_INCREMENTAL0, macOS/arm64, one worker,2MiB/default selection, all output formats and diagnostics,600-second process timeout. Each default-output series has one first plus three later samples; filesystem byte checks precede every scan, so none is an OS-cold-cache measurement. Build time is excluded. Native opt-in and instrumented metadata are separate one-scan technical profiles, never pooled with default CLI performance.

All30 original commits/trees and licenses are in UPSTREAMS.json and LICENSE-REVIEW.json. The complete materialized input population is112,180 regular files plus4 symlinks;17 absent gitlinks and zero LFS pointers are recorded. No target project was built, installed, executed, resolved through its package manager, or probed at runtime. Source bodies remain in the external cache.

## Scoped reference-detection metrics

The immutable initial47 source-first units comprise42 positive and5 negative API-reference questions. They are main-authored exploratory/tuning data with prior implementation knowledge, not independent annotations, a random holdout or a full-file/repository oracle. Only nominated rule/token intervals are scored by the unchanged existing one-to-one scorer. Unattributed and other-rule predictions are separate, not established product FP. All47 initial units were evaluated; the2 additional project subsets introduce no invented TN.

| Initial47 | TP | FP | FN | TN | Precision | Recall | F1 |
|---|---:|---:|---:|---:|---:|---:|---:|
| Before | 37 | 0 | 5 | 5 | 100.0% | 88.1% | 93.7% |
| After | 40 | 0 | 2 | 5 | 100.0% | 95.2% | 97.6% |

These ratios measure finding the source reference, not correct algorithm identity, predicted operation purpose, runtime use or migration judgment. Five negative controls do not establish population false-positive performance. The historical summary group key `classical_api` is a generic Crypto_API nomination group and can include PHP PQC caller-context questions; it is not an algorithm classification.

| Language | Positive / negative units | Before TP/FP/FN/TN | After TP/FP/FN/TN | After recall |
|---|---:|---|---|---:|
| PHP | 15 / 4 | 14/0/1/4 | 15/0/0/4 | 100.0% |
| C | 11 / 0 | 10/0/1/0 | 11/0/0/0 | 100.0% |
| C++ | 2 / 1 | 1/0/1/1 | 1/0/1/1 | 50.0% |
| C# | 14 / 0 | 12/0/2/0 | 13/0/1/0 | 92.9% |

C and C++ are never pooled. The C++ denominator is only2 positive questions. Headers and INC remain separate in file/stage accounting.

Supplemental exposed data are separate: TRUTH-SUPPLEMENT-02.json has6 positive API/dependency checks, plus2 unscored implementation questions. Its2 dependency intervals were corrected from package-token columns to declaration column1 without changing labels; v1 and old results remain preserved. The baseline using the corrected labels is NATIVE-BASELINE-AUDIT-02.json; final is NATIVE-FINAL-AUDIT.json. The6 checks change from4 found/2 missed to6 found/0 missed, with no negative background sample; this is not repository precision. The declaration wording does not mean independent annotators (SUPPLEMENT-ANNOTATION-CLARIFICATION.json).

The original12 domain questions,2 implementation questions and2 later certificate-verification questions are unscored positive coverage facts. DOMAIN-TRUTH-SUPPLEMENT-01.json and CERTIFICATE-DOMAIN-AUDIT.json retain the latter: OpenSSL X509_verify_cert and .NET X509Chain.Build are source verification requests, not evidence of successful trust, hostname/revocation policy, selected key algorithm or actual execution. No equivalent observation occurs on those source lines in either phase.

## Per-project default-output comparison

TP/FP/FN/TN columns apply only to initial nominated intervals. Unscored projects retain their source-domain questions; they are not TN. Time/RSS are medians of3 later successful invocations, not mixed-language performance attribution. Scope and original commits remain explicit in the linked input manifests.

| Project | Scope | Findings before→after | Seconds before→after | Peak RSS MiB before→after | Initial TP/FP/FN/TN before→after |
|---|---|---:|---:|---:|---|
| phpseclib/phpseclib | materialized root | 195→195 | 2.52→2.85 | 63.12→58.94 | 1/0/0/2→1/0/0/2 |
| firebase/php-jwt | materialized root | 162→162 | 0.17→0.17 | 27.31→27.31 | 3/0/0/0→3/0/0/0 |
| lcobucci/jwt | materialized root | 78→78 | 0.32→0.32 | 23.98→23.67 | 3/0/0/0→3/0/0/0 |
| web-token/jwt-framework | materialized root | 940→940 | 2.57→2.61 | 59.70→60.19 | 2/0/0/1→2/0/0/1 |
| robrichards/xmlseclibs | materialized root | 22→26 | 0.19→0.23 | 24.09→24.17 | unscored→unscored |
| onelogin/php-saml | materialized root | 45→45 | 0.87→0.88 | 28.80→28.80 | 3/0/0/0→3/0/0/0 |
| simplesamlphp/simplesamlphp | materialized root | 6→6 | 1.86→1.87 | 27.36→26.97 | unscored→unscored |
| nextcloud/server | materialized root | 301→301 | 34.56→34.77 | 79.67→76.69 | 2/0/0/1→2/0/0/1 |
| PHPMailer/PHPMailer | materialized root | 3→4 | 0.40→0.53 | 30.22→30.22 | 0/0/1/0→1/0/0/0 |
| roundcube/roundcubemail | materialized root | 0→0 | 6.97→8.45 | 76.50→76.45 | unscored→unscored |
| openssl/openssl | materialized root | 5951→5980 | 11.42→11.49 | 280.86→279.50 | 2/0/0/0→2/0/0/0 |
| google/boringssl | materialized root | 14309→14313 | 13.64→13.64 | 544.00→540.11 | 1/0/1/1→1/0/1/1 |
| wolfSSL/wolfssl | materialized root | 3040→3048 | 29.62→29.62 | 405.44→370.33 | unscored→unscored |
| randombit/botan | materialized root | 4329→4329 | 9.50→9.50 | 219.86→219.47 | unscored→unscored |
| openssh/openssh-portable | materialized root | 456→460 | 2.21→2.23 | 70.72→70.94 | 3/0/0/0→3/0/0/0 |
| strongswan/strongswan | materialized root | 566→572 | 9.42→9.40 | 87.38→85.97 | 2/0/0/0→2/0/0/0 |
| bitcoin/bitcoin | materialized root | 0→0 | 35.18→35.41 | 147.55→144.25 | unscored→unscored |
| curl/curl | materialized root | 168→169 | 3.36→3.40 | 42.56→39.83 | 1/0/1/0→2/0/0/0 |
| libssh2/libssh2 | materialized root | 237→243 | 0.60→0.61 | 35.05→35.05 | 1/0/0/0→1/0/0/0 |
| lsh123/xmlsec | materialized root | 606→606 | 2.45→2.44 | 50.75→50.02 | 1/0/0/0→1/0/0/0 |
| bcgit/bc-csharp | materialized root | 9160→9182 | 37.65→38.52 | 393.44→399.12 | 1/0/0/0→1/0/0/0 |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | materialized root | 437→437 | 13.96→13.99 | 55.64→55.73 | 2/0/1/0→2/0/1/0 |
| openiddict/openiddict-core | materialized root | 105→105 | 15.86→15.94 | 69.73→69.95 | 2/0/0/0→2/0/0/0 |
| ITfoxtec/ITfoxtec.Identity.Saml2 | materialized root | 69→69 | 2.82→2.85 | 63.03→59.28 | unscored→unscored |
| Sustainsys/Saml2 | licensed OSS subset | 4→4 | 0.49→0.49 | 24.19→24.23 | unscored domain question |
| jstedfast/MimeKit | materialized root | 158→162 | 33.40→33.58 | 87.97→87.91 | 1/0/1/0→2/0/0/0 |
| win-acme/win-acme | materialized root | 65→65 | 2.44→2.39 | 30.95→29.72 | 2/0/0/0→2/0/0/0 |
| OrchardCMS/OrchardCore | materialized root | 38→38 | 51.45→51.62 | 372.12→370.41 | 1/0/0/0→1/0/0/0 |
| bitwarden/server | licensed OSS subset | 194→194 | 202.73→202.95 | 91.94→93.42 | unscored domain question |
| dotnet/aspnetcore | materialized root | 299→299 | 105.87→106.24 | 538.11→539.19 | 3/0/0/0→3/0/0/0 |

Bitcoin has0 findings despite a pinned source ECDSA/secp256k1 API request. Zero findings must not be read as absence of cryptography or a safe migration result. Full per-project samples, resource ranges, CPU, all retained output bytes/logs, order/semantic stability and source/config bindings are in the reference summaries and receipts. Descriptive medians are not statistical significance or a new numeric pass gate.

Descriptive later-sample medians increased for PHPMailer (0.40→0.53s), Roundcube (6.97→8.45s) and phpseclib (2.52→2.85s). No repeated-session significance or causal profile was measured; the increases remain visible rather than being declared cost-free. The largest final elapsed median is the Bitwarden OSS subset (202.95s); the largest final median peak RSS is BoringSSL (540.11MiB).

All360 comparable measurement processes exited0 without timeout:120 default-output samples,30 native audits and30 metadata-only observer audits per phase. The excluded47-sample pilot,4 compatibility invocations and synthetic observer tests are not pooled into these totals. Each project has stable default finding identity/order across its4 samples; cross-phase changes remain visible. Exit0 coexists with the recorded Partial/Unknown/Truncated inspection outcomes. OSS-SUBSET-PIPELINE.json and OSS-SUBSET-COVERAGE-COMMANDS.json bind the last completed operations.

## Algorithm, purpose and artifact strata

These are source-question strata of reference detection, not algorithm/purpose classification accuracy. Imported key types and macro/nonlocal factories can remain unresolved; EC is coarser than ECDSA. Full native expectation/identity/assurance/coordinate tables are in NATIVE-BASELINE-AUDIT-02.json and NATIVE-FINAL-AUDIT.json. All matched initial API observations have syntactic/referenced assurance and passing original byte/line/Unicode-column checks. No active-use claim follows.

### Source algorithm questions

| Source stratum | Positive / negative | Before TP/FP/FN/TN | After TP/FP/FN/TN |
|---|---:|---|---|
| DH | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| DSA | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| DSA (macro selector) | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| EC (caller context) | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| ECDH | 2 / 0 | 2/0/0/0 | 2/0/0/0 |
| ECDSA | 4 / 1 | 4/0/0/1 | 4/0/0/1 |
| Ed25519 | 8 / 0 | 7/0/1/0 | 8/0/0/0 |
| ML-DSA component; composite parent | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| ML-DSA family (caller context) | 2 / 0 | 2/0/0/0 | 2/0/0/0 |
| ML-DSA-44 | 2 / 0 | 2/0/0/0 | 2/0/0/0 |
| ML-DSA-65 | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| ML-KEM-768 | 1 / 0 | 1/0/0/0 | 1/0/0/0 |
| None | 0 / 4 | 0/0/0/4 | 0/0/0/4 |
| RSA | 13 / 0 | 9/0/4/0 | 11/0/2/0 |
| key-dependent | 4 / 0 | 4/0/0/0 | 4/0/0/0 |

### Source purpose questions

| Source stratum | Positive / negative | Before TP/FP/FN/TN | After TP/FP/FN/TN |
|---|---:|---|---|
| DKIM_signature | 2 / 0 | 0/0/2/0 | 2/0/0/0 |
| comment | 0 / 3 | 0/0/0/3 | 0/0/0/3 |
| decryption | 2 / 0 | 2/0/0/0 | 2/0/0/0 |
| encryption | 2 / 0 | 2/0/0/0 | 2/0/0/0 |
| function_definition_not_call | 0 / 1 | 0/0/0/1 | 0/0/0/1 |
| key_agreement | 2 / 0 | 2/0/0/0 | 2/0/0/0 |
| key_context_request | 7 / 0 | 6/0/1/0 | 6/0/1/0 |
| key_generation | 7 / 0 | 7/0/0/0 | 7/0/0/0 |
| key_import | 8 / 0 | 6/0/2/0 | 7/0/1/0 |
| log_string | 0 / 1 | 0/0/0/1 | 0/0/0/1 |
| signature | 9 / 0 | 9/0/0/0 | 9/0/0/0 |
| signature_verification | 3 / 0 | 3/0/0/0 | 3/0/0/0 |

### Source artifact questions

| Source stratum | Positive / negative | Before TP/FP/FN/TN | After TP/FP/FN/TN |
|---|---:|---|---|
| source | 42 / 5 | 37/0/5/5 | 40/0/2/5 |

Source purposes include signing/verification, encryption/decryption, key agreement/generation/import and KEM requests. The native algorithm purpose is a reference; a complete predicted-purpose truth audit is not established. The audited initial API rows carry quantum-uncertain labels; legacy safe/vulnerable labels likewise classify their rule evidence, not an operator-scoped migration decision. All28 full-root native inventories have assessment_status not_evaluated.

## Inspection, evidence and practical limits

BASELINE-LANGUAGE-COVERAGE.json and FINAL-LANGUAGE-COVERAGE.json contain per-project/per-language selected populations and every observed detector status; the2 subset coverage files keep their separate scopes. Inventory path groups include symlinks, while selected/read detector grids include regular files only. PHP/.phtml suffix counts do not silently absorb tagged INC; suffix Not Applicable is neither a detector status nor a safety result. Header ambiguity, .C precedence and C++ fragments ending.cc.inc stay explicit. Complete/Analyzed means bounded stage execution, not complete cryptographic recognition.

For28 roots, both phases select/read92,335 files:63,067 terminal Analyzed,29,084 Partial,178 NotApplicable,6 Unknown. Four enumeration failures are non-followed symlinks reported before selection as read_failed diagnostics, even though selected read_failed_files is0. Detector failures/Unsupported/Truncated remain inside Partial files and are not erased by exit0. The source inventory versus selection and default excluded paths/media/2MiB/ignore policy remain separately inspectable; missing gitlinks are not silently treated as scanned dependencies.

The2 licensed subsets add8,187 selected/read files in each phase:4,921 Analyzed and3,266 Partial, with no selected-read or terminal-analysis failures. Their C# stage grid has5,965 files:2,739 Complete,3,121 Partial and105 Truncated. Across the explicitly defined30 OSS inputs,100,522 files are selected/read per phase (67,988 Analyzed,32,350 Partial,178 NotApplicable,6 Unknown). These30 input frames are28 roots plus2 subsets, not30 full mixed-license checkouts. All30 native inventories retain not_evaluated assessments.

BASELINE-OUTPUT-AUDIT.json and FINAL-OUTPUT-AUDIT.json distinguish exact finding-ID duplication, coincident locations, multiple rules and differing risk labels. Baseline has0 duplicate finding IDs,4,813 multi-rule locations and2,332 locations with differing risk labels. These are not automatically adjudicated false positives, independent crypto uses or separate vulnerabilities. In the initial5 negative token intervals, the all-rule coincidence audit finds no prediction; this remains a tiny interval-specific control. Default/native labels can differ from masked text to code-owned finite labels, with equal finding identity/count/coverage; keep profiles separate.

The following final Tree-sitter stage counts use the disclosed30 input scopes. Baseline counts are identical. Other detector stages and per-project partitions remain in the bound coverage files; this table does not equate parser completion with semantic support.

| Suffix group | Inventory paths | Selected/read | Complete | Partial | Truncated | NotApplicable |
|---|---:|---:|---:|---:|---:|---:|
| PHP | 8,663 | 8,660 | 4,976 | 3,645 | 39 | 0 |
| C | 5,239 | 5,086 | 946 | 4,113 | 27 | 0 |
| C++ | 2,718 | 2,716 | 390 | 2,229 | 97 | 0 |
| C# | 29,224 | 29,217 | 14,284 | 14,299 | 634 | 0 |
| Ambiguous C/C++ header | 4,590 | 4,584 | 187 | 4,334 | 63 | 0 |
| Unresolved INC | 1,636 | 1,631 | 1,129 | 0 | 371 | 131 |

Practical usefulness is a main-author engineering source audit, not a practitioner study. Accurate API coordinates support triage and exposed four generic defects. Unfamiliar wrappers, internal implementations, macros, function pointers, templates/interfaces, nonlocal receivers, construction relationships and incomplete files still need manual source work. API calls, implementation definitions, available configuration and actual runtime use are different facts. No Windows/Linux execution, target backend/provider capability, compiled branch or real protocol negotiation was measured.

## Manual source, duplicate and risk judgments

MANUAL-SOURCE-RISK-DUPLICATE-AUDIT.json records6 main-authored, output-exposed judgments against pinned source, rule descriptions and primary API/TLS/NIST references. It is not a population accuracy sample. BoringSSL RSA_generate_key_ex has3 rule representations of one source request. BC registry documentation/declarations and a MimeKit Received-header fixture contain suite names without selecting or negotiating them. ECDHE_RSA uses ephemeral ECDH plus RSA authentication; the broad RSA-based rule hint correctly distinguishes that from static RSA transport. But Critical/usage wording for the inert fixture overstates actionable project use: this is a validated context counterexample, not a nominated-API FP and not proof of deployed vulnerability. The2 NuGet rows describe one package declaration, with no installed-capability promotion. Exact ML-KEM and an ML-DSA composite component retain reference-only scope without declaring a working provider or whole-construction safety. Remaining locations are unadjudicated, explicitly outside these6 judgments.

A generic safe improvement for the context counterexample needs comment/declaration/inert-data/configuration controls. Ignoring all strings or tests would suppress real configuration and crypto tests; legacy labels were not silently rewritten. The counterexample remains in the pinned data and audit with impact/future direction, rather than disappearing into0FP on the separate47-unit API denominator.

## Improvements and residuals

| Generic correction | Regression/evidence | Effect |
|---|---|---|
| PHP local binding saturation | Two RED→GREEN controls; cap64/cutoff/global limits unchanged | PHPMailer openssl_sign miss becomes a reference; independent requests retained with Truncated |
| C# local binding saturation | Two RED→GREEN controls plus22 C# tests | Bouncy Castle ML-DSA/SLH-DSA constructor references survive unrelated saturation; later values remain family-only/unknown |
| Four OpenSSL raw public/private imports | C/C++ selector-dialect, role, shadow, mutation and privacy controls | curl import reference found; canonical Ed25519/X25519 and finite explicit ML-KEM/ML-DSA requests, no key-byte validity inferred |
| Canonical BC RsaDigestSigner | Alias, nonzero RSA reference, namesake and privacy controls | MimeKit constructor miss becomes a source reference |

The2 initial misses retained after fixes are BoringSSL nested C++ direct initialization (grammar declarator/parameter view, no call node) and AzureAD same-name RSA property/type lookup. RESIDUAL-SYNTAX-DIAGNOSIS.json binds minimal parser observations and the C# specification. Safe follow-ups need bounded declarator reconciliation and canonical declared-type equivalence with alias/scope/namesake controls. Reinterpreting every declarator or ignoring every value shadow is unsafe.

Positive domain gaps retain wolfSSL/Botan/libsecp256k1/XML-wrapper APIs, certificate verification, BoringSSL implementation entries, OpenSSH conditional hybrid construction and a BC composite component. No equivalent observation on a source line is a measured coverage question, not a whole-repository absence proof. Future work should specify generic library ownership and static operation/construction evidence with negative controls; whole implementations and deployed hybrid safety cannot be established by broad name matching. No follow-up task was automatically created.

## Verification and reviews

Default workspace: 930 passed, 3 ignored. All features: 989 passed, 5 ignored. REQUIRED-CHECKS-02.json records format/diff, both Clippy configurations and default/release builds, all exit0 for final source03. Seven new meaningful Rust regressions have RED101/GREEN0 evidence. External adapter8 controls, observer2 parity/failure controls and compatibility4 actual CLI invocations pass; held saved JSON/HTML/Markdown are exact and old-seven8-file/nonempty12-finding signatures and native IDs are verified.

The broader Python suite previously ran180 tests with1 inherited applicability/hash-guard error and3 skips; baseline proof is PREEXISTING-APPLICABILITY-FAILURE.json. It is not counted as passing. The old Node-feedback failure is unchanged and was not rerun for these detector-only changes. Report templates/presentation/browser code were unchanged, so no new browser presentation acceptance was required. No remote CI, packaging release or cross-platform runtime claim is made.

Implementation reviewer: /root/oss_implementation_review. Test reviewer: /root/oss_test_review. Both accepted the implemented source03/test scope. Aggregation and dataset-frame implementation acceptance and final-data follow-up decisions are recorded in TASK-084-REVIEWS.md; both final aggregation/data and original-criterion/report reviews are accepted. These are the only2 reviewer identities and neither created source truth.

## OSS eligibility and completion boundary

Sustainsys is limited to src/foss plus license notices (168 files);57 other entries include commercial material and parent/root context. Bitwarden excludes bitwarden_license (8,128 included,637 excluded). Original full-root inventories/commits, subset filters, byte/mode bindings and omissions are retained in OSS-SUBSET-SCOPE-DECISION.json and UPSTREAMS-OSS-SUBSETS.json. Subset scans do not reconstruct missing parent MSBuild/commercial dependency context or establish full mixed-license-root coverage. The earlier commercial-inclusion question remains unanswered; no permission was inferred.

The original user request is an OSS study across30 named projects, not an obligation to include unlicensed commercial parts or every checkout file. Both retained reviewers corrected the earlier full-root shorthand; no additional scope approval is needed for these disclosed, outcome-independent OSS filters. Final data/test-review acceptance and criterion/source reconciliation are recorded below and in TASK-084-REVIEWS.md. Optional commercial inclusion would be separate work and is not assumed. Do not claim whole-product cryptographic coverage, independent truth or runtime safety from this bounded study.

## Original criterion reconciliation

| Original obligation | Evidence and outcome |
|---|---|
| Pin and characterize30 inputs without target execution | UPSTREAMS.json, LICENSE-REVIEW.json, SOURCE-CHARACTERISTICS.json, SOURCE-CANDIDATES.json and licensed subset decision retain hashes, revisions, omissions and source-only boundary. |
| Baseline and final scans of all30 | 360 comparable exit0 processes; default/native/observer receipts and separate2-subset summaries/coverage. Limits remain visible; success is process execution, not exhaustive analysis. |
| Ground truth and bounded metrics | Immutable initial47, corrected versioned supplemental6,16 unscored domain/implementation questions;69 total source questions. Precision/Recall/F1 are interval-only. |
| Actual source/spec evaluation | NATIVE-BASELINE-AUDIT-02.json, NATIVE-FINAL-AUDIT.json,2 subset native audits, CERTIFICATE-DOMAIN-AUDIT.json and6-case manual source/risk/overlap audit. Manual binding correction preserves the original adjudication and verifies frozen hashes. |
| C/C++ separate and inspection/resource limits | Per-language/stage coverage, per-project repeated wall/CPU/RSS/output-byte distributions; failure/unsupported/truncated/unknown and excluded populations retained. |
| Generic improvements and compatibility | Four detector corrections,7 new RED/GREEN regressions,64 affected-language checks,930/989 suite checks, exact old-seven/saved-output compatibility and final source bindings. |
| Final reporting, residuals and reviews | This report links bounded outcomes and root causes/proposals. The2 retained independent reviewers accepted final aggregation/data and this report, including the final language/performance tables; TASK-084-REVIEWS.md retains each decision. |
