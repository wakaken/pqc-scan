# Selected 35-project reevaluation after five inspection repairs, 2026-11-27

This evaluation checks five C/C++ and C# inspection repairs against the same selected 27 primary projects and eight comparison projects, while preserving source-bound evidence and the limits of the accuracy assessment.

All 27 primary projects succeeded, with 51,816 raw findings. Seven comparison projects succeeded, with 6,664 raw findings. Node.js timed out at 600 seconds and has no full report. Execution coverage, inspection completeness and accuracy remain separate; the selected evaluation remains incomplete.

The selection is the existing 35-project subset of 98 entries, with no new acquisition or duplicate project counting. Each of nine primary language groups contains three projects. JavaScript/TypeScript and C/C++ each form one group. The other 63 projects remain future work. Their sources and records are preserved and excluded from this run's completion condition. Previous successes are not added to this run.

## Conditions and state meanings

The fixed commits, registered scope and successful acquisition bindings are retained. Conditions are one thread, one attempt per admitted entry, a 600-second timeout, a 2 MiB file limit, 4 GiB free-disk admission and 11 required artifacts. collectPerformance=false leaves CPU, RSS and performance distributions null. No scanned OSS code, builds or dependency resolution ran. The two approved unavailable files in Node.js and Discourse remain excluded, and host protection is unchanged.

Succeeded describes execution and admission of the required outputs. Complete and Partial describe inspection coverage separately. Complete means inspection of the selected inputs under the stated configuration; it does not certify cryptographic safety or cover unavailable or excluded code. Raw findings include overlapping rules, comments and tests. An absent detection does not establish safety.

## Primary 27

| Project | SHA (12) | Execution | Inspection | Findings | 11-format audit |
| --- | --- | --- | --- | --- | --- |
| bcgit/bc-java | bb81bfa75c9f | Succeeded | Partial | 10,494 | PASS |
| Keyfactor/ejbca-ce | a46c89e0c6b6 | Succeeded | Partial | 617 | PASS |
| apache/mina-sshd | f885e0fc9f68 | Succeeded | Partial | 447 | PASS |
| cloudflare/circl | 31be93db4772 | Succeeded | Partial | 269 | PASS |
| smallstep/certificates | 0d8a80647845 | Succeeded | Complete | 1,472 | PASS |
| FiloSottile/age | b74dce4cdbe3 | Succeeded | Complete | 73 | PASS |
| ruby/openssl | b5774711c1f0 | Succeeded | Partial | 238 | PASS |
| net-ssh/net-ssh | 348b78049bfe | Succeeded | Partial | 688 | PASS |
| jwt/ruby-jwt | c22701ef8e0e | Succeeded | Partial | 537 | PASS |
| pyca/cryptography | 0b3a52a7cb58 | Succeeded | Partial | 2,979 | PASS |
| paramiko/paramiko | 0e9ccb91e0f9 | Succeeded | Complete | 263 | PASS |
| certbot/certbot | 9d5c9d44e57d | Succeeded | Partial | 292 | PASS |
| panva/jose | d1e200f22d2b | Succeeded | Partial | 973 | PASS |
| openpgpjs/openpgpjs | ea041b3c70eb | Succeeded | Partial | 37 | PASS |
| paulmillr/noble-post-quantum | 48e493397e69 | Succeeded | Partial | 288 | PASS |
| rustls/rustls | 796b8d3b0848 | Succeeded | Partial | 792 | PASS |
| Eugeny/russh | 56a999871aa1 | Succeeded | Complete | 93 | PASS |
| RustCrypto/KEMs | b26db85a9b44 | Succeeded | Partial | 167 | PASS |
| phpseclib/phpseclib | 808f39f816c4 | Succeeded | Partial | 195 | PASS |
| firebase/php-jwt | 99efeb17d6de | Succeeded | Partial | 289 | PASS |
| lcobucci/jwt | 8bd5e392228c | Succeeded | Partial | 78 | PASS |
| openssl/openssl | 18d0baa70de7 | Succeeded | Partial | 6,031 | PASS |
| google/boringssl | 5ec099107553 | Succeeded | Partial | 14,380 | PASS |
| wolfSSL/wolfssl | f4f52d975ff1 | Succeeded | Partial | 3,358 | PASS |
| bcgit/bc-csharp | 4814a10bc95d | Succeeded | Partial | 6,200 | PASS |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | 0d96aca5a57a | Succeeded | Partial | 451 | PASS |
| openiddict/openiddict-core | 68269d28b439 | Succeeded | Partial | 115 | PASS |

All 27 primary projects succeeded and passed all 11 output audits. Four reports remain Complete and 23 remain Partial. The available reports contain 51,816 raw findings, compared with 51,342 before the five repairs. The 474 added rule/file/line/column identities comprise 420 C# observations and 54 wolfSSL observations, with zero removals. These are descriptive reference deltas, not new TP labels or an accuracy improvement rate. All six selected C/C++ and C# repositories remain Partial.

## Comparison eight

| Project | SHA (12) | Execution | Inspection | Findings | 11-format audit |
| --- | --- | --- | --- | --- | --- |
| keycloak/keycloak | 18559075f50c | Succeeded | Partial | 4,072 | PASS |
| apache/airflow | c4e2d0054bd0 | Succeeded | Partial | 56 | PASS |
| kubernetes/kubernetes | a0594a28a435 | Succeeded | Partial | 1,989 | PASS |
| nodejs/node | 330e3ee67e84 | TimedOut | Unavailable | null | Unavailable |
| n8n-io/n8n | 9a84403d2fae | Succeeded | Complete | 282 | PASS |
| discourse/discourse | 6dfa59ea612b | Succeeded | Partial | 72 | PASS |
| rails/rails | ee9b18fdb0fc | Succeeded | Partial | 8 | PASS |
| vectordotdev/vector | 7e97ead1775c | Succeeded | Partial | 185 | PASS |

Seven available comparison reports contain 6,664 raw findings. One report is Complete and six are Partial. Node.js inspection and counts remain Unavailable/null. Comparison results are not added to primary results.

Three canonical command records bind the primary run, first comparison run and comparison admission retry. Keycloak and Airflow were NotRun before scanner launch. Code and retained evidence support the inference that disk admission stopped those entries; free space at the exact stop and its origin were not observed and remain null. Both later succeeded with Partial inspection under the same 600-second and 4 GiB settings; the retry closed at 03:50:11 UTC. Their two NotRun history records are retained, not counted as new projects. Seven primary representative observations reuse identical current-source attempt records. The effective set has 35 projects and 37 execution-history rows.

Node.js full-run output remains unavailable after the fixed 600-second timeout. Its finding counts, full-report Native assets, full-report scoring, 11-format audit and full-report EVP reassessment remain null or Unavailable. A separate bounded file-only regression, if recorded, cannot establish success or coverage for the repository-wide attempt. The separate paired file-only regression actually passed within its recorded scope. Both frozen binaries scanned the same 1,187,495-byte Node.js file, joined the single API_EVP_PKEY_RSA reference at 13,778:24, retained its paired legacy ID and conservative wording/hint, and passed their receipt and offline export checks. This PASS_FILE_SCOPE_ONLY outcome does not reassess the unavailable full-scope 33rd case or backfill the four owned lines.

Resuming the full Node.js assessment requires static input isolation or analysis-progress evidence that identifies the input and cause of the 600-second timeout. Do not repeat the same full invocation without that evidence. After a fix or another evidenced basis for retry, retain at least 4 GiB free space and the same fixed SHA, actual file hashes, complete input scope, 600-second limit, and configuration. Recheck the 11 required outputs, offline audit, four unscored owned lines, and full-scope EVP wording. CPU, RSS, load, and swap monitoring remain disabled.

## Language groups and finding locations

| Primary group | Reports | Raw findings |
| --- | --- | --- |
| C / C++ | 3 | 23,769 |
| C# | 3 | 6,766 |
| Go | 3 | 1,814 |
| Java | 3 | 11,558 |
| JavaScript / TypeScript | 3 | 1,298 |
| PHP | 3 | 562 |
| Python | 3 | 3,534 |
| Ruby | 3 | 1,463 |
| Rust | 3 | 1,052 |

| Location attribution | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| C | 4,470 | 0 |
| C++ | 981 | 0 |
| C# | 6,471 | 0 |
| Go | 2,260 | 1,854 |
| Java | 6,847 | 3,292 |
| JavaScript | 68 | 0 |
| PHP | 491 | 0 |
| Python | 1,081 | 56 |
| Ruby | 1,329 | 73 |
| Rust | 1,110 | 8 |
| TypeScript | 1,228 | 283 |
| C/C++ header unresolved | 534 | 0 |
| Manifest/configuration | 9,254 | 739 |
| Other/unattributed | 15,692 | 359 |

Location attribution uses the existing file-extension classifier, independently of the declared repository language. The .h bucket remains C/C++ unresolved. Configuration and other files stay separate. An observed zero in available comparison reports says nothing about unavailable Node.js output.

## Cryptographic technology observations

| Output category | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| CryptoAPI | 12,946 | 1,784 |
| JWT | 10,936 | 3,900 |
| KeyMaterial | 1,501 | 167 |
| Middleware | 34 | 0 |
| PKI | 13,459 | 323 |
| PQC | 3,728 | 27 |
| SSH | 1,351 | 57 |
| TLS | 7,861 | 406 |

These are raw output categories, not active-use or vulnerability counts. The categories retain their original meaning. Rule counts remain descriptive and do not reimplement the 139 rules or 11-language evaluation.

| Population | Cases | Selectors | Verified | No exact location | Not assessed |
| --- | --- | --- | --- | --- | --- |
| primary | 38 | 11 | 2 | 9 | 27 |
| comparison | 3 | 0 | 0 | 0 | 3 |

Fresh observations were sealed before this grading stage opened the exact approved oracle. The independent sealed-capture review recomputed all 41 case projections and all 35 technology rows without opening that oracle, then released the unchanged existing judgment.compare route. The assessment command actually exited 0. Among the 38 primary cases, 11 selectors are comparable: two finite field expectations agree and nine have no_exact_location. The 2/11 value is finite attribute representation agreement, not classification precision, recall or F1. The nine unjoined cases are not nine false negatives. Another 27 primary cases and all three comparison cases remain not assessed; the comparison agreement rate is null. Primary completeness retains ten partial cases and one not_exhaustive case. Historical original-after and current outputs have unchanged finite fields and join transitions for these exact cases. Temporal equivalence remains unknown and causalComparisonEligible is false. The current assessment helper differs from the historical recipe, with the drift and exact unchanged judgment.compare binding recorded. This Native comparison is distinct from the separate paired old/new CLI regressions and raw finding comparison. Prior exposure prevents a blind or independent classification claim. The separate independent review passed the actual finite Native grading scope after recomputing 82 current/historical case-phase rows and reconciling all technology counts. This scoped approval is not classification validation.

| Native algorithm family | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| ECDH | 5 | 0 |
| ECDSA | 11 | 6 |
| Ed25519 | 6 | 2 |
| ML-DSA | 41 | 3 |
| ML-KEM | 37 | 1 |
| RSA | 23 | 5 |
| Unknown | 300 | 41 |
| X25519 | 3 | 0 |

Native algorithm assets have a different unit from findings. Only the existing finite family allowlist is mapped; other identifiers remain Unknown. Node.js full-report asset counts are null.

## Qualified accuracy scope

The primary 27 are a purposive cryptographic selection, not a random sample or unused holdout. Prior output exposure, tuning, source evidence and adjudication remain recorded. Separate AI agents from the same model family do not establish independent truth. Historical denominators of 392 locations and 324 claims are not reused.

The unchanged existing scorer processed 136 of the selected 140 owned lines from 34 successful projects, and both primary canonical positive intervals. Node.js four owned lines remain unscored. The scorer exited with code 3 for the expected unavailable Node.js output, so selected scoring completion remains false. No new gold, denominator or holdout was introduced. The primary finite source propositions contain 38 known facts, one canonical TP-proven detection, one mapped miss, and 36 unresolved positive mappings. The unresolved mappings are nine canonically unrepresentable facts, 25 without a mapped rule and two unsupported inputs. Three unknown facts and three incomplete owned lines remain. The comparison has two known propositions, zero proven detections and two unresolved mappings. These lower counts are not whole-source recall. The two interval scores are not pooled across their different historical strata and canonical families. FP0 is confined to the admitted intervals, with predictions outside them unattributed. The different historical strata and canonical families remain separate.

| Project | Historical stratum | Canonical family | TP | FP | FN | Precision | Recall |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RustCrypto/KEMs | holdout, now exposed | source_algorithm_reference | 1 | 0 | 0 | 1.0 | 1.0 |
| jwt/ruby-jwt | tuning | key_routing | 0 | 0 | 1 | null | 0.0 |

No negative interval is admitted. Predictions outside admitted intervals remain unattributed. Unresolved claims and unjoined locations are not converted to false negatives. Whole-population and comparison precision, recall and F1 remain null. Any interval-specific precision applies only to that interval and cannot support a global accuracy claim.

## Confirmed repair reassessment

| Scope | Result |
| --- | --- |
| C# predefined types, bc-csharp | 15 old hits removed |
| JWT kid wording, BoringSSL | 15 conservative wording checks passed |
| EVP wording, OpenSSL | 2 conservative wording checks passed |
| EVP wording, Node.js full report | Not reassessed; TimedOut; null |
| JWK private output fragments | 4 values, 12 fragments, 0 survivors |

The independent primary-only reassessment gave scoped PASS to 32 source-bound cases. The 15 removed C# hits comprise 12 int, two string and one bool occurrence. Other findings and diagnostics show the files were processed, while incomplete inspection remains explicit. The 17 wording cases preserve legacy finding IDs; those IDs do not prove Native or canonical identity. Zero surviving fragments is limited to 12 selected fragments from four BoringSSL private values. No private values, fragments or private-value hashes are printed. These checks do not prove universal oth ownership, printed-RSA ownership, raw-literal fallback, newline preservation or public retention. Historical related FAIL and later scoped PASS records remain preserved.

All 35 registered-condition comparisons agree. Successful before/after acquisition bindings agree for 34 available projects; Node.js lacks a successful after binding. The protocol and rules hashes match. Product sources and actual clocks differ. Both raw comparison binaries are default debug builds. Only the explicitly checked finite references, positions, types, wording and canonical intervals support comparison. Whole-report causal equivalence and time-dependent risk/expiry equivalence remain Unknown. Finite after-only diagnostic-cause phrase counts may overlap within a message and are not distinct-file totals; old generic messages cannot resolve those causes, so their before counts remain null.

| Repository-wide tree-sitter diagnostics | Partial before | Partial after | Truncated before | Truncated after |
| --- | --- | --- | --- | --- |
| openssl/openssl | 1,724 | 1,702 | 13 | 11 |
| google/boringssl | 923 | 825 | 15 | 7 |
| wolfSSL/wolfssl | 1,431 | 1,407 | 33 | 32 |
| bcgit/bc-csharp | 1,576 | 823 | 50 | 43 |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | 352 | 111 | 8 | 3 |
| openiddict/openiddict-core | 283 | 62 | 45 | 4 |

| After-only finite cause phrases | Parser recovery | Conditional source | Unresolved crypto binding | Opaque macro | Other uncertainty |
| --- | --- | --- | --- | --- | --- |
| openssl/openssl | 672 | 899 | 108 | 698 | 302 |
| google/boringssl | 386 | 243 | 25 | 138 | 76 |
| wolfSSL/wolfssl | 404 | 753 | 13 | 631 | 350 |
| bcgit/bc-csharp | 68 | 645 | 226 | 0 | 0 |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | 8 | 107 | 20 | 0 | 0 |
| openiddict/openiddict-core | 17 | 59 | 0 | 0 | 2 |

These are repository-wide tree-sitter diagnostic entries, not unique files, defects or findings. Repositories can contain several languages. All six repositories remain Partial. The five finite cause phrases can overlap within one message and are not exhaustive; incomplete-header and truncation classes remain separate. Before cause counts are null, because old generic messages do not identify those causes. Lower diagnostic counts do not prove precision gains or removal of real bounds. Browser observations with filters or a narrower suffix scope do not share these full-repository denominators.

## Five implemented repairs

| Priority | Problem | Change and real CLI evidence |
| --- | --- | --- |
| P1 | C# charged long expressions against name limits | Check eligible name syntax before charging. The comment-padded reproduction recovers two RSA references from zero; its short control retains two. Real long names and incomplete name indexes remain bounded. |
| P2 | C/C++ charged long literals and expression shapes as names | Check identifier and static-callee shapes first. Remove irrelevant truncation while preserving the actual 256-byte identifier and 512-byte callee boundaries. |
| P2 | Partial explanations did not distinguish causes | Distinguish parser recovery, conditional source, unresolved supported crypto binding, opaque macros and other source uncertainty. Preserve combined causes and Truncated precedence. Distinguish incomplete header candidates from actual source identity disagreement. |
| P2 | Ordinary non-crypto receiver calls manufactured Partial | Do not add crypto uncertainty for an ordinary call alone. Preserve binding invalidation, then retain Partial for unresolved supported crypto operations without guessing an algorithm. |
| P3 | Finite wolfSSL API and BoringSSL macro coverage | Recognize exact unshadowed wc_MakeRsaKey calls and actual definitions as existing RSA source references. Exclude prototypes, members, pointers and shadows. Reclassify only the proven trailing macro artifact in the actual BoringSSL fragment, preserving positions and Partial. |

An independent reviewer ran 44 fixed-input before/after pairs, 88 CLI scans, and seven EN/JA pairs, 14 scans. All 103 supplemental output assertions passed. This is bounded regression evidence, not corpus accuracy. The wolfSSL reference does not establish effective key size, exponent, provider or execution. A synthetic secret canary was absent from every emitted file and process stream.

Actual wolfSSL raw findings change from3,304 to3,358. The54 added identities join wc_MakeRsaKey call spans across19 files, with syntactic referenced RSA and empty Native parameters. The public callee remains visible in53 legacy evidence strings; one remains privacy-masked. Protection is preserved. These54 references are not new TP labels or an accuracy improvement rate.

## Input and safety constraints versus residual product work

| Class | Remaining reason | Interpretation |
| --- | --- | --- |
| Input and safety | Unexpanded gitlinks, approved quarantine exclusions and unsupported certificate/dependency formats | Preserve input scope and Unknown/Partial. Do not fetch missing contents or change protection settings. |
| Input and safety | Unknown branches, macro expansion, build definitions and configured C/C++ header language | Retain statically supported references without executing OSS or builds. The actual BoringSSL macro remains Partial. |
| Input and safety | Genuine name, input-size and analysis-budget limits | Keep limits and Truncated/Partial. These differ from the irrelevant budget charges repaired here. |
| Residual pqc-scan work | Unsupported syntax, parser recovery and unresolved supported crypto operations/bindings | Unspecified cases remain unresolved. Actual OpenSSL parser and conditional uncertainty remain Partial. |
| Residual pqc-scan work | The existing ten API, Native normalization, dependency/certificate, diagnostic and Ruby syntax themes | These five repairs do not close entire themes. Excluded RbNaCl and liboqs-python remain future work. |

Complete describes inspection of selected inputs under the stated configuration. It does not establish cryptographic safety or inspection of unavailable/excluded code. Succeeded describes execution and required output, independently of Complete/Partial.

## Ten residual improvement themes

| Theme | Selected35 repository intersection | Future63 intersection | Current relationship |
| --- | --- | --- | --- |
| API-RUBY-SODIUM-BINDINGS | 0 | 1 | Excluded API candidate remains unresolved. |
| API-PYTHON-OQS-ROLES | 0 | 1 | Excluded API candidate remains unresolved. |
| NATIVE-PQC-UNDERSCORE-ALIASES | 4 | 0 | Underscore PQC Native normalization remains open. |
| INSPECT-SOURCE-GRAMMAR | 17 | 37 | The five finite repair units improve a subset. Other syntax, bindings and build conditions remain unresolved. |
| INSPECT-DEPENDENCY-FAILURE | 3 | 4 | Dependency failures and unsupported inputs remain. |
| INSPECT-DEPENDENCY-SUBSET | 9 | 18 | Partial dependency scopes remain. |
| INSPECT-DEPENDENCY-COMPLETENESS | 2 | 6 | Some dependency enumeration remains non-exhaustive. |
| INSPECT-CERTIFICATE-FORMAT | 14 | 22 | Unsupported certificate/container formats remain. |
| REPORT-DIAGNOSTIC-TRIAGE | 4 | 7 | Five source causes and header wording improve; the entire theme remains open. |
| JWT-RUBY-MEMBER-SYNTAX-SCOPE | 1 | 0 | Ruby kid: syntax and the canonical interval miss remain. |

Counts are intersections of the existing candidate ledger with the fixed selected set. They are not defect counts, new adjudications or numbers of repairs. Preserve source evidence, historical outputs and sourceConfirmed/investigate dispositions in the existing candidate and prior reassessment records. Local improvements do not close an entire theme or excluded future63 work.

## Storage preservation

| Operation | Files compressed | Logical bytes retained | Free before | Free after | Observed increase |
| --- | --- | --- | --- | --- | --- |
| Current transparent compression | 220 | 935,774,803 | 4,218,871,808 | 4,968,120,320 | 749,248,512 |

The final v3 storage record verifies pre/post bytes, SHA-256, metadata and compression attributes for 357 retained paths, including bindings to the canonical execution receipts for 27 primary and the first eight comparison entries, plus six browser copies. The new operation transparently compresses 220 files and deletes no data. At the closure observation of 2026-11-27T03:44:40.097047+00:00, current free space was 4,975,411,200 bytes. This observation is distinct from the operation before/after values in the table. A later separate filesystem observation at 2026-11-27T03:59:29.403996+00:00 measured 4,827,881,472 available bytes, about 4.496315 GiB. This dated observation is not an additional compression gain or a performance result.

Logical bytes and observed free-space changes are separate measures. APFS shared blocks and other disk activity prevent adding gains across stages. Nested du sizes are not summed. Sources, truth, adjudication, samples, splits, raw output bytes, logs, manifests, reviews, historical measurements, required binaries and future-project evidence are preserved.

## Validation, independent review and rendered browser observations

The source epoch is partial-five-c61a2ad4383542819d1481269efdc8d9 at HEAD cf3bfe2071228921f5a33d2a44e0f3b94de4d1de on codex/oss-crypto-corpus. The freeze contains 343 product files and 111 harness files. The default evaluation binary SHA-256 is 8c6b054481f06064506fda089bc74ab938605d8f45a80a9898f611145a9f160c. Existing dirty and untracked changes remain recorded.

| Validation | Result |
| --- | --- |
| Default Rust tests | 1,064 passed; 3 ignored |
| All-features Rust tests | 1,126 passed; 5 ignored; overlaps default tests |
| Workspace fmt check; build/clippy in both feature configurations | Passed |
| Final CLI smoke | 33 assertions across 12 commands |
| Temporal v2 | 10 passed |
| Required artifact receipt checks | 34 succeeded projects, 11 artifacts each |
| Offline export audit | 34 directories passed; 4 schema-validated exports and 7 JSON projection/reference checks each |

The two Cargo feature matrices use isolated build outputs. Passing and ignored counts are not added across configurations. Temporal v2 records the environment correction for the /tmp no-follow condition; earlier failed records remain preserved. These validation results establish the finite checked behavior and do not supply OSS accuracy labels. The offline audit distinguishes four schema-validated exports, Native inventory, SARIF, cryptographic CycloneDX 1.7 and dependency CycloneDX 1.7, from seven JSON projection/reference checks. It does not claim 11 official schemas. The historical task076 contract manifest retains SHA-256 c2d5114feaa4285121cc94d76f648ea5c2df1c76a02143528dba604f9e3cd939. A versioned 484-case c_family_partial_contract_v2 changes only six unknown-EVP inspection expectations from Complete to Partial; finding counts, rules, spans and algorithms stay unchanged. This exposed first-party regression expectation correction is not new classification gold. The historical 882-case manifest and parent pending-adjudication record remain unchanged. A separate independent codex:gpt-6.1-sol@high review actually approved this scoped status migration after verifying all 484 selected source hashes, unchanged focal rules/spans/algorithms/other expectations, and the six status-only changes. The new fixture matches the 343-file source freeze and existing final-source test evidence; this source-based approval does not claim a fresh test execution.

The unchanged 197 Python tests and the 900-second outer-wrapper passing evidence were reused after checking the 111-file harness, schemas, actual Python executable, and recorded dependency versions. Historical package contents were not hashed in full, so this reuse relies on the recorded environment identity. These checks do not execute the current Rust scanner. The separate ci.py witness builds and runs current Rust; its v2 run passed 10 tests. These are local checks, not a claim that the complete GitHub CI or all 105 preparation checks for the historical 98-repository scope ran.

Actual final-source KEM, OpenSSL and C# reports and finite controls for five Partial causes were inspected in a browser. The 13 saved captures bind the observations. English/Japanese explanations, incomplete-header wording, keyboard disclosure and actual OpenSSL loaded-offline search and language switching were observed. The narrow viewport measured 354 by 767 with scrollWidth 354. Browser acceptance is representative, not an observation of every report.

Implementation correctness and test adequacy use codex:gpt-6.1-sol@high. The independent runtime review approved the finite final-source scope. The final multi-receipt data check gave PASS_SCOPED_DATA_WITH_NODE_UNAVAILABLE and joined 37 historical receipt rows to the effective 35 projects, with 34 successful offline audits, zero schema errors and 238 artifact SHA joins. The report constructor has a separate PASS_SOURCE_ONLY review. The browser evidence review examined bound records and three captures, without replaying every interaction. These scope-specific results do not independently approve the final report or establish independent scanner truth. Final report review and the separate work-record audit by codex:gpt-6.1-terra@high are recorded with report hashes in the final closure record. No future PASS is asserted here. The actual sealed-capture and Native grading reviews also passed their finite scopes under codex:gpt-6.1-sol@high, with the separate receipts retained.

## Evidence and remaining work

The [selected-run index](../../../benchmark/evidence/selected35-partial-fixes-index.json) binds the fixed inputs, all execution histories, descriptive counts, score, Native observations, audits, source continuity and independent review receipts. The [final closure record](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/partial-five-fixes-c61a2ad4383542819d1481269efdc8d9/FINAL-CLOSURE.json) holds the final report hashes and final technical-review and work-record-audit outcomes. It is a separate closure record, not a prefilled approval. Historical reports and failed attempts remain preserved.

No global accuracy or performance result, future 63-project completion, develop integration, release approval or publication is claimed. Old reports, failed attempts and historical reviewer identities remain intact.
