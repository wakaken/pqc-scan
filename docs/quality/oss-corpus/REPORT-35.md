# Selected 35-project cryptographic evaluation, 2026-11-27

This evaluation narrows the existing corpus to 27 primary projects and eight comparison projects, preserves evidence on the existing disk, and reassesses confirmed repairs. All 35 selected attempts reached a terminal state. All 27 primary projects succeeded. Seven comparison projects succeeded and Node.js timed out at 600 seconds. The selected Node.js EVP wording case remains unassessed, so overall evaluation completion remains false.

The previous 28 successes are not added to this run. Reuse of the current pilot and representative observations refers only to identical final-source attempts.

The selection is a subset of the existing 98 entries, with no new acquisition or duplicate project counting. Each of nine primary language groups contains three projects. JavaScript/TypeScript and C/C++ each form one group. The replacements are age, noble-post-quantum and RustCrypto/KEMs. The other 63 primary projects remain future work; their registry entries, sources and evidence are preserved and excluded from this run completion condition.

## Conditions and state meanings

Every project retains its fixed commit, actual file hashes and existing input scope. Conditions are one thread, one attempt, a 600-second timeout, a 2 MiB file limit, 4 GiB free-disk admission and 11 required formats. collectPerformance=false disables OSS CPU, RSS and elapsed-performance observations; load and swap monitoring remain deferred. No scanned OSS code, builds or dependency resolution ran. The two user-approved unavailable files in Node.js and Discourse remain excluded; host protection was unchanged.

Succeeded means the scan and required outputs were admitted. Complete and Partial separately describe inspection coverage and input limitations; neither certifies cryptographic safety. Counts are raw finding observations, including tests, comments and overlapping rules. Timed-out output, detection counts and accuracy remain null.

## Primary 27

| Project | SHA (12) | Execution | Inspection | Findings | Output audit |
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
| wolfSSL/wolfssl | f4f52d975ff1 | Succeeded | Partial | 3,304 | PASS |
| bcgit/bc-csharp | 4814a10bc95d | Succeeded | Partial | 5,788 | PASS |
| AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet | 0d96aca5a57a | Succeeded | Partial | 449 | PASS |
| openiddict/openiddict-core | 68269d28b439 | Succeeded | Partial | 109 | PASS |

The 27 available reports contain 51,342 raw findings. Primary and comparison aggregates remain separate.

## Comparison eight

| Project | SHA (12) | Execution | Inspection | Findings | Output audit |
| --- | --- | --- | --- | --- | --- |
| keycloak/keycloak | 18559075f50c | Succeeded | Partial | 4,072 | PASS |
| apache/airflow | c4e2d0054bd0 | Succeeded | Partial | 56 | PASS |
| kubernetes/kubernetes | a0594a28a435 | Succeeded | Partial | 1,989 | PASS |
| nodejs/node | 330e3ee67e84 | TimedOut | Unavailable | null | Unavailable |
| n8n-io/n8n | 9a84403d2fae | Succeeded | Complete | 282 | PASS |
| discourse/discourse | 6dfa59ea612b | Succeeded | Partial | 72 | PASS |
| rails/rails | ee9b18fdb0fc | Succeeded | Partial | 8 | PASS |
| vectordotdev/vector | 7e97ead1775c | Succeeded | Partial | 185 | PASS |

The 7 available reports contain 6,664 raw findings. Primary and comparison aggregates remain separate.

## Language groups and finding locations

| Primary group | Reports | Findings |
| --- | --- | --- |
| C / C++ | 3 | 23,715 |
| C# | 3 | 6,346 |
| Go | 3 | 1,814 |
| Java | 3 | 11,558 |
| JavaScript / TypeScript | 3 | 1,298 |
| PHP | 3 | 562 |
| Python | 3 | 3,534 |
| Ruby | 3 | 1,463 |
| Rust | 3 | 1,052 |

| Location attribution | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| C | 4,416 | 0 |
| C++ | 981 | 0 |
| C# | 6,051 | 0 |
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

Location attribution uses the existing extension classifier, not the declared repository language. The .h bucket remains C/C++ unresolved; configuration and other files remain separate. A comparison zero is an observed zero in seven reports and says nothing about Node.js.

## Cryptographic technology observations

| Output category | Primary (27) | Comparison (7 available) |
| --- | --- | --- |
| CryptoAPI | 12,473 | 1,784 |
| JWT | 10,936 | 3,900 |
| KeyMaterial | 1,501 | 167 |
| Middleware | 34 | 0 |
| PKI | 13,459 | 323 |
| PQC | 3,727 | 27 |
| SSH | 1,351 | 57 |
| TLS | 7,861 | 406 |

The output observes classical cryptographic API references and TLS, SSH, PKI, JWT and PQC categories. Category names retain the original output meaning; they do not prove active use, vulnerabilities or completed migration. Rule counts remain in the descriptive JSON.

Native capture filters the original 129 cases to the exact 41 selected cases and seals actual observations before opening the oracle. It reuses the original source evidence, approved oracle and unchanged judgment.compare.

| Population | Cases | Selectors | Verified | No exact location | Not assessed |
| --- | --- | --- | --- | --- | --- |
| Primary | 38 | 11 | 2 | 9 | 27 |
| Comparison | 3 | 0 | 0 | 0 | 3 |

Two of the 11 comparable primary field expectations agree; nine have no_exact_location. Comparison has no assessed selector and a null agreement rate. This is finite field representation agreement, not classification accuracy. The nine unjoined cases are not converted to false negatives. Primary completeness retains ten partial and one not_exhaustive case. The exact historical original-after projection to the same 41 cases has identical finite fields and join states; temporal and whole-output causal equivalence remain Unknown.

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

These are Native algorithm-asset counts, with units distinct from findings and active use. Only the existing finite family allowlist is mapped; other identifiers remain Unknown. Node.js asset counts are null.

The first pre-oracle provenance check failed because whole-dictionary equality required an omitted optional bytes field. The original coordinator and failed logs are preserved. Version 2 changes only three assertions to verify declared reference fields. The scorer, oracle and sealed observations are unchanged. The original assessment-helper reference is historical metadata, not a currently readable same-bytes input at that path. The current helper is bound to the previous final Ruby epoch. Independent review records are linked in the index.

## Admitted accuracy scope

The 27 projects are a purposive cryptographic selection, not a random sample or unused holdout. Prior output exposure, tuning and adjudication records remain intact. Separate same-family AI raters do not establish independent truth. The historical 392-line and 324-claim denominators are not reused.

The selection contains 140 existing sampled owned lines. The unchanged scorer processed 136 lines belonging to 34 successful projects. Only two primary positive intervals have canonical mappings; comparison has none. No negative interval is admitted. Source and consulted evidence/context hashes, labels and scorer bindings were authenticated.

| Project | Historical split | TP | FP | FN | Precision | Recall |
| --- | --- | --- | --- | --- | --- | --- |
| RustCrypto/KEMs | holdout (now exposed) | 1 | 0 | 0 | 1.0 | 1.0 |
| jwt/ruby-jwt | tuning | 0 | 0 | 1 | null | 0.0 |

Both interval results match the authenticated original-after output. FP0 applies only inside the admitted intervals; other predictions remain unattributed. Historical tuning and holdout units are not pooled into a generalization score or F1. Whole-population and comparison precision, recall and F1 remain null. The sampled Ruby interval remains a miss.

Among 38 known primary propositions, canonical scoring proves one detection and one mapped miss; 36 positive mappings remain unresolved. These comprise nine canonically unrepresentable facts, 25 without a mapped rule and two unsupported inputs. They are not 36 false negatives. Three unknown facts and three incomplete sampled lines remain. The two known comparison propositions have no mapped rule and are not converted to false negatives.

## Confirmed repair reassessment

| Repair scope | Current result |
| --- | --- |
| C# predefined types, bc-csharp | 15 historical hits removed |
| JWT kid wording, BoringSSL | 15 conservative wording checks passed |
| EVP wording, OpenSSL | 2 conservative wording checks passed |
| EVP wording, Node.js | 1 not reassessed; TimedOut; null |
| JWK private output fragments | 4 values / 12 fragments; 0 survivors |

An independent reviewer gave scoped PASS to 32 source-bound cases. The C# cases comprise 12 int, two string and one bool occurrence. Other findings and diagnostics show those files were processed, but six files remain detector_partial. This does not certify elimination of every false positive or complete inspection. The 17 wording cases retain their legacy finding IDs.

The privacy observation is limited to zero output survivors for 12 fragments derived from four BoringSSL private values. Values, fragments and private-value hashes are not logged. This does not establish universal oth ownership, printed-RSA ownership, raw-literal fallback, newline preservation or public-value retention. Earlier related FAIL and later scoped PASS records remain preserved.

Historical output used release and the current binary uses dev; features=[] agree. Fixed inputs, source hashes and rule predicates were checked, but clocks, protocol and performance settings differ. Only finite non-temporal positions, types, conservative wording and the two canonical intervals are compared. Whole-output causal improvement and equivalence of time-dependent risk/expiry values remain Unknown. The 340 product files match the previous final Ruby epoch; no duplicate product repair was implemented.

## Ten remaining improvement themes

| Theme | Selected repositories | Future repositories |
| --- | --- | --- |
| API-RUBY-SODIUM-BINDINGS | 0 | 1 |
| API-PYTHON-OQS-ROLES | 0 | 1 |
| NATIVE-PQC-UNDERSCORE-ALIASES | 4 | 0 |
| INSPECT-SOURCE-GRAMMAR | 17 | 37 |
| INSPECT-DEPENDENCY-FAILURE | 3 | 4 |
| INSPECT-DEPENDENCY-SUBSET | 9 | 18 |
| INSPECT-DEPENDENCY-COMPLETENESS | 2 | 6 |
| INSPECT-CERTIFICATE-FORMAT | 14 | 22 |
| REPORT-DIAGNOSTIC-TRIAGE | 4 | 7 |
| JWT-RUBY-MEMBER-SYNTAX-SCOPE | 1 | 0 |

Counts are repository intersections with the existing candidate ledger, not defect counts or new adjudications. Candidate IDs, affected projects and original sourceConfirmed/investigate states remain in the reassessment JSON. The RbNaCl and liboqs-python API candidates are outside this selection and remain unresolved. Native underscore aliases, grammar coverage, dependency/certificate scope, diagnostic triage and Ruby kid: syntax remain open.

## Storage cleanup and preservation

Initial free space was 501,297,152 bytes, about 0.47 GiB. At report creation, free space was 5,212,106,752 bytes, about 4.85 GiB, observed at 2026-11-26T22:31:44.330192+00:00.

| Operation | Files | Logical bytes | Observed free-space increase (bytes) |
| --- | --- | --- | --- |
| Delete regenerable .rlib/.rmeta/.o/.a | 5740 | 4,832,751,840 | 82,464,768 |
| Transparent historical-output compression | 933 | 6,613,521,895 | 5,869,359,104 |
| Transparent finished-primary-output compression | 249 | 831,632,638 | 743,354,368 |

No whole folder was deleted. Canonical paths, purpose, current references and preservation boundaries were checked before deleting 5,740 individually hash-verified files. The final compression replaced 249 of 286 candidates, retaining 37 without benefit and 65 excluded files. Original paths, logical bytes, size, mode and mtime were verified before and after compression. Inode, ctime and compression flags can change.

APFS shared blocks require separating logical sizes from observed free-space gains. Each before/after observation may also include other disk activity; nested du sizes are not summed. Sources, truth, adjudication, samples, splits, raw output bytes, logs, manifests, reviews, historical measurements, required binaries and future 63-project evidence are preserved. Existing outputs remain readable at the same paths.

STORAGE-CLOSURE.json links every deleted/compressed canonical path and hash. Deletion was limited to manifest-listed intermediate files under these five canonical roots.

| Canonical build root |
| --- |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/product-export-privacy-fix/target-default |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/isolated-temporal-ci-f2054133015d4a69acb084711db37acb/target-ci |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/ruby-scalar-range-implementation-73c7e151b22c425b9b5a12e458b5857d/target-default |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/shipping-envelope-1ef73ce550084ea9bff61b199835d483/target-overlay-warm |
| /Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/ruby-epoch-final-validation-9976999cac6e47dfb0717fdb3e630bec/target-default |

To obtain a separate uncompressed file, the recorded method is ditto --noclone --nopreserveHFSCompression --nohfsCompression SRC NEWPATH. Restoration was checked before compression; verify the original SHA-256 before replacement. No large full backup copy was made. Build-copy cp -cR can fall back on unsupported or different filesystems. An append-only correction supersedes the earlier overstatement and distinguishes command semantics from actual same-APFS observations.

## Required validation, independent review and browser acceptance

The worktree is /Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan on codex/oss-crypto-corpus at cf3bfe2071228921f5a33d2a44e0f3b94de4d1de. Existing dirty and untracked changes remain intact. All 340 product files match sourceEpoch oss98-final-ruby-dc58c84b6f8f4bc9873dadbaab5c3e4f; all 111 current harness files match their fixed hashes. The pinned Rust toolchain is 1.93.1. The evaluation binary SHA-256 is 8b1d0bbc11d7e08483c31d6d50913126957234ee3b796aac323cc7ca06b04a69.

| Validation | Result |
| --- | --- |
| Default Rust tests | 1045 passed / 3 ignored; source-exact reuse |
| Default fmt / clippy / build | exit 0; source-exact reuse |
| All-features tests | 1107 passed / 5 ignored |
| All-features clippy / build | exit 0; isolated target |
| Rebuilt CLI subset | 132 passed; overlap with1107; 29 rebuilt executables |
| Python CI | 197 passed; installed outer wrapper passed |
| No-performance regressions | 60 passed |
| Selected scorer controls | 6 passed |
| Native existing regressions | 22 passed plus orchestration controls |
| Successful output audit | 34 passed (27 primary / 7 comparison) |

The original incomplete 1,696-byte test file is preserved, without chmod or execution. All-features validation used isolated build output. Copied CLI tests embedded an origin executable path, so 29 executables were rebuilt in another isolated target and the overlapping 132 tests reran against the correct CLI. They are not added to 1,107. Python CI changed only the outer timeout constant to 900 seconds; all 197 checks and exit assertions remain. The OSS scan timeout stays 600 seconds.

The old 98-specific 105-test preparation and historical clock replay are not used by this explicit-35 current-clock route. Their pending records and false permissions remain; no 105-test PASS is claimed. Affected-route checks and actual final-source input validation apply without mechanically repeating unrelated checks. The independent technical review assessed this applicability.

Implementation correctness, test adequacy, scoring, aggregation and report review use codex:gpt-6.1-sol@high. Work-record auditing separately uses codex:gpt-6.1-terra@high. Scope-specific verdicts and consolidated review/audit records, including report hashes, are linked in the index. Authors do not independently approve their own changes, and historical reviewer identities remain unchanged.

Actual Ruby JWT and RustCrypto/KEMs HTML were inspected in a browser. English/Japanese, search, severity/file/test filters, dialog Tab/Shift-Tab/Escape and focus return, narrow layout and loaded-offline controls worked. The observed narrow viewport was 354 by 767 with no horizontal overflow. Offline controls were checked by pausing only the dedicated local server; OS network and protection settings were unchanged.

Ruby JWT raw JSON has 537 findings and the UI has 355 intentionally coalesced rows; all 537 finding IDs remain in each language dataset. KEMs scanner Complete covers the selected 154 files and is distinct from repository-input Partial caused by an unpopulated gitlink. Screenshots and observations are bound in BROWSER-OBSERVATIONS.json.

Prove It Works drove actual-output, source and browser checks. Sequence Work into Verifiable Units drove expansion from a small input to four representatives and all 35. Explain the Number kept population counts separate from qualified accuracy denominators. Model the Domain drove explicit collectPerformance booleans and retained null unmeasured values.

## Evidence and restart conditions

The [selected-run index](../../../benchmark/evidence/selected35-index.json) links fixed entries, input hashes, all attempt states, command/exit records, scores, audits and independent reviews. See the [protocol](PROTOCOL-35.md) and [Japanese report](REPORT-35.ja.md). Historical 98-project records remain intact.

Node.js reached the fixed 600-second deadline with empty stdout/stderr and no report. Input integrity is retained, but the stalled stage is unlocalized. Identical conditions were not repeated. Restart requires bounded non-performance progress evidence to diagnose the cause, or a separately agreed execution bound/configuration. After success, audit all 11 formats, score its four owned lines and reassess the single EVP case. Preserve the failed attempt and distinguish any changed-condition attempt.

Future 63-project work, implementation of every candidate, whole-corpus accuracy research, performance measurement, develop integration and v0.5 publication are separate. No push, PR, merge, tag, publication, external contact or billing action occurred. Scope reduction does not grant release approval or waive required validation.
