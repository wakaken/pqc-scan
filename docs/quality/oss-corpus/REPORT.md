# Pinned OSS cryptography corpus: 2026-11-21

The corpus now includes all **18 nominated additions** across six language groups,
alongside the **eight retained targets**. All 26 full-repository scans exited 0
without timeout. They emitted **32,978 findings** (21,848 in the additions).
These are reference counts, not verified vulnerabilities, operations or accuracy.

The highest-priority gaps are WebCrypto and OpenPGP composite APIs, receiver-bound
Python/Ruby operations, Go ECDSA/X.509/SSH calls, and nonliteral or suffixed PQC
names. The [64 source-checked observations](OBSERVATIONS.md) distinguish implementation,
API use, configuration, dependencies and tests. No rules or product source changed.

## Source and scope

- Scanner: `22aa9f9fb3e368c24d005060015252a2800b422a` (`origin/develop` at fetch).
- Branch: `codex/oss-crypto-corpus`; isolated worktree, original checkout edits preserved.
- CLI: v0.3.0, Rust 1.93.1, release, locked/default features, one scan thread.
- Original mechanisms reused: `shell/run-benchmark.sh`,
  `examples/scripts/scan_oss_products.sh`, `tests/quality/run.py` resource/environment
  helpers and `tests/quality/evaluate.py` semantic digest. No new committed runner.
- [Protocol](PROTOCOL.md), [pinned manifest](manifest.json),
  [machine summary](summary.json), [CSV comparison](comparison.csv),
  [environment and source/binary bindings](../../records/oss-corpus-20261121/evidence/environment.json).
- Foreign repositories were only cloned/archived and read. No foreign scripts,
  builds, tests, submodule initialization or dependency installation ran.

## Added repositories and measurements

Times include scanner startup and JSON/diagnostic writing, excluding acquisition
and compilation. Each row is one first-observed sample, not a cold-cache claim
or a stable ranking. RSS is `wait4` child high-water memory in MiB. Input sizes,
ignored paths and languages differ; counts/times cannot rank detection accuracy.

| Language | Added repository | Pinned commit | Findings | Read files | Wall s | Peak RSS MiB | Inspection |
|---|---|---|---:|---:|---:|---:|---|
| Java | [bcgit/bc-java](https://github.com/bcgit/bc-java) | [bb81bfa75c9f](https://github.com/bcgit/bc-java/commit/bb81bfa75c9fa81c7fde467a6314374d22715472) | 10,324 | 9,931 | 16.978 | 217.8 | partial |
| Java | [Keyfactor/ejbca-ce](https://github.com/Keyfactor/ejbca-ce) | [a46c89e0c6b6](https://github.com/Keyfactor/ejbca-ce/commit/a46c89e0c6b670c3c4a1cf267380abf1ae181424) | 613 | 3,473 | 7.193 | 55.3 | partial |
| Java | [apache/mina-sshd](https://github.com/apache/mina-sshd) | [f885e0fc9f68](https://github.com/apache/mina-sshd/commit/f885e0fc9f68701507c666856906d2d83bdfd9eb) | 447 | 1,794 | 2.902 | 29.6 | partial |
| Go | [cloudflare/circl](https://github.com/cloudflare/circl) | [31be93db4772](https://github.com/cloudflare/circl/commit/31be93db4772d9482a06c44eb2ed134fdacd2a2b) | 269 | 670 | 1.248 | 30.5 | partial |
| Go | [smallstep/certificates](https://github.com/smallstep/certificates) | [0d8a80647845](https://github.com/smallstep/certificates/commit/0d8a806478454d360e46b1bc0d19dcb5673fb246) | 1,472 | 541 | 1.575 | 54.8 | complete_within_scope |
| Go | [gravitational/teleport](https://github.com/gravitational/teleport) | [1283425b60ec](https://github.com/gravitational/teleport/commit/1283425b60ec5f60d509ba4c791183d452923ff7) | 798 | 12,667 | 33.530 | 200.3 | partial |
| Ruby | [ruby/openssl](https://github.com/ruby/openssl) | [b5774711c1f0](https://github.com/ruby/openssl/commit/b5774711c1f0737cb96ca3698facb2d8b6d0ee91) | 138 | 141 | 0.219 | 24.6 | complete_within_scope |
| Ruby | [net-ssh/net-ssh](https://github.com/net-ssh/net-ssh) | [348b78049bfe](https://github.com/net-ssh/net-ssh/commit/348b78049bfe815d4c4e46d3d0185f3b9fd88978) | 679 | 268 | 0.758 | 32.6 | partial |
| Ruby | [jwt/ruby-jwt](https://github.com/jwt/ruby-jwt) | [c22701ef8e0e](https://github.com/jwt/ruby-jwt/commit/c22701ef8e0e0a33e4fbfaebee36fc48acba1a08) | 537 | 122 | 0.270 | 29.5 | partial |
| Python | [pyca/cryptography](https://github.com/pyca/cryptography) | [0b3a52a7cb58](https://github.com/pyca/cryptography/commit/0b3a52a7cb58ecb2d0b07e12a2635586034896d3) | 2,979 | 2,454 | 2.929 | 87.5 | partial |
| Python | [paramiko/paramiko](https://github.com/paramiko/paramiko) | [0e9ccb91e0f9](https://github.com/paramiko/paramiko/commit/0e9ccb91e0f9a68f2587233d523ad7fe46e730fe) | 263 | 172 | 0.544 | 24.7 | complete_within_scope |
| Python | [certbot/certbot](https://github.com/certbot/certbot) | [9d5c9d44e57d](https://github.com/certbot/certbot/commit/9d5c9d44e57df4c070287e99338e49029cc45cba) | 292 | 1,044 | 2.064 | 31.0 | partial |
| JS/TS | [panva/jose](https://github.com/panva/jose) | [d1e200f22d2b](https://github.com/panva/jose/commit/d1e200f22d2bc66cd2ed6cc48fc622b6f1765cc5) | 973 | 186 | 0.624 | 37.1 | partial |
| JS/TS | [openpgpjs/openpgpjs](https://github.com/openpgpjs/openpgpjs) | [ea041b3c70eb](https://github.com/openpgpjs/openpgpjs/commit/ea041b3c70ebc680f955127cfbc1f922a9f3fd0a) | 29 | 181 | 1.215 | 34.7 | partial |
| JS/TS | [digitalbazaar/forge](https://github.com/digitalbazaar/forge) | [01119d67bf65](https://github.com/digitalbazaar/forge/commit/01119d67bf65e7898717501ff6417bed9cc37233) | 163 | 160 | 0.851 | 27.9 | partial |
| Rust | [rustls/rustls](https://github.com/rustls/rustls) | [796b8d3b0848](https://github.com/rustls/rustls/commit/796b8d3b08481e7f5119c0b3d0b640586c45f0d8) | 786 | 497 | 1.271 | 40.0 | partial |
| Rust | [Eugeny/russh](https://github.com/Eugeny/russh) | [56a999871aa1](https://github.com/Eugeny/russh/commit/56a999871aa134072e4f2a7395fe2de313870e21) | 93 | 125 | 0.522 | 27.5 | complete_within_scope |
| Rust | [aws/aws-lc-rs](https://github.com/aws/aws-lc-rs) | [104142736550](https://github.com/aws/aws-lc-rs/commit/104142736550c9860d0dfdb389e0d6c4c51e0005) | 993 | 499 | 6.485 | 414.4 | partial |

All 18 candidates are suitable for targeted static reference evaluation. Their
different roles are intentional: primitive/provider libraries reveal implementation
and API gaps; PKI/SSH/JOSE applications exercise calls and configuration. Root
license paths, retained upstream notices, purpose and verified representative
technologies are recorded per repository in the manifest. Underlying native
implementations and all unannotated findings are not automatically adjudicated.

## Existing population retained

The benchmark scripts and their additional historical targets are unchanged.
These eight explicitly requested existing repositories were also remeasured from
their existing local committed snapshots; their working-tree edits were excluded.
They were not silently advanced to new upstream heads.

| Repository | Pinned commit | Findings | Read files | Wall s | Peak RSS MiB |
|---|---|---:|---:|---:|---:|
| keycloak | [18559075f50c](https://github.com/keycloak/keycloak/commit/18559075f50c8934d315035aea0c9a96314d37a9) | 4,072 | 10,452 | 19.859 | 138.1 |
| airflow | [c4e2d0054bd0](https://github.com/apache/airflow/commit/c4e2d0054bd0649d9db9af41837f46cb0dcbc31d) | 56 | 9,372 | 32.911 | 73.3 |
| kubernetes | [a0594a28a435](https://github.com/kubernetes/kubernetes/commit/a0594a28a435f392360f5e6b360cfc9e156d5ef3) | 1,989 | 27,111 | 54.385 | 201.8 |
| node | [330e3ee67e84](https://github.com/nodejs/node/commit/330e3ee67e84e0d4845ff29898f22908a6a6b727) | 4,481 | 43,871 | 116.282 | 354.8 |
| n8n | [9a84403d2fae](https://github.com/n8n-io/n8n/commit/9a84403d2fae449bf5dbff8fc4fc9a5158ccf37c) | 282 | 15,022 | 59.991 | 88.9 |
| discourse | [6dfa59ea612b](https://github.com/discourse/discourse/commit/6dfa59ea612b2ddf1bc57e1204f0410b6618a092) | 72 | 16,404 | 45.031 | 69.0 |
| rails | [ee9b18fdb0fc](https://github.com/rails/rails/commit/ee9b18fdb0fca18fee96bf992ed476c9daf946f9) | 1 | 4,676 | 11.873 | 47.8 |
| vector | [7e97ead1775c](https://github.com/vectordotdev/vector/commit/7e97ead1775c1cadd882ca176aa2497d8b7ee1b5) | 177 | 7,415 | 12.604 | 49.2 |

Historical reports/counts were preserved. Different scanner versions, source
commits, output formats and scan policies prevent direct historical regressions
from being inferred from these numbers.

## Language comparison: additions only

Language labels identify the target population, not every file language in a
multilingual repository. Totals of different-size populations are descriptive.

| Target language | Repositories | Findings | Read files | Total wall s | Maximum child peak RSS MiB |
|---|---:|---:|---:|---:|---:|
| Java | 3 | 11,384 | 15,198 | 27.073 | 217.8 |
| Go | 3 | 2,539 | 13,878 | 36.352 | 200.3 |
| Ruby | 3 | 1,354 | 531 | 1.247 | 32.6 |
| Python | 3 | 3,534 | 3,670 | 5.537 | 87.5 |
| JS/TS | 3 | 1,165 | 527 | 2.691 | 37.1 |
| Rust | 3 | 1,872 | 1,121 | 8.278 | 414.4 |

## Technology reference signals

These are **grouped emitted findings**, not truth units or cryptographic uses.
One finding can appear in multiple columns; several rules may cover the same
source fact. Certificate properties/containers can refer to one object. JWT/JOSE
is a rule-family grouping and includes protocol-ambiguous algorithm tokens.
Masked match text cannot always be assigned a family. Exact grouping definitions
and complete rule counts are in `summary.json`; zero means no grouped emitted
signal, never absence or safety. For example, the source-checked OpenPGP composite
implementation below is present despite zero grouped ML-KEM/ML-DSA signals.

| Addition | RSA | ECDSA | ECDH | X.509 rules | SSH rules | JWT/JOSE rules | ML-KEM | ML-DSA | Hybrid PQC |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| bc-java | 5309 | 39 | 822 | 4342 | 32 | 133 | 431 | 757 | 30 |
| circl | 87 | 0 | 7 | 1 | 0 | 0 | 45 | 42 | 7 |
| openssl | 61 | 0 | 2 | 0 | 0 | 0 | 1 | 6 | 1 |
| cryptography | 1471 | 25 | 7 | 1916 | 70 | 0 | 174 | 175 | 0 |
| jose | 552 | 209 | 0 | 0 | 0 | 750 | 0 | 65 | 0 |
| rustls | 566 | 25 | 315 | 127 | 0 | 6 | 46 | 0 | 23 |
| ejbca-ce | 262 | 5 | 6 | 21 | 12 | 70 | 3 | 65 | 0 |
| mina-sshd | 308 | 0 | 0 | 0 | 304 | 9 | 18 | 0 | 0 |
| certificates | 669 | 715 | 188 | 146 | 0 | 822 | 0 | 1 | 0 |
| teleport | 620 | 95 | 30 | 23 | 133 | 357 | 0 | 0 | 0 |
| net-ssh | 551 | 0 | 0 | 2 | 599 | 0 | 0 | 0 | 0 |
| ruby-jwt | 448 | 73 | 0 | 0 | 0 | 350 | 0 | 0 | 0 |
| paramiko | 245 | 0 | 0 | 0 | 242 | 0 | 3 | 0 | 0 |
| certbot | 162 | 11 | 0 | 144 | 1 | 51 | 0 | 0 | 0 |
| openpgpjs | 9 | 0 | 0 | 0 | 0 | 18 | 0 | 0 | 0 |
| forge | 153 | 0 | 0 | 3 | 12 | 0 | 0 | 0 | 0 |
| russh | 62 | 0 | 0 | 0 | 66 | 0 | 11 | 0 | 0 |
| aws-lc-rs | 638 | 61 | 26 | 4 | 0 | 0 | 91 | 120 | 20 |

## Source-verified detection and coverage questions

| Technology | Confirmed source fact | Observed behavior |
|---|---|---|
| RSA | Teleport `rsa.GenerateKey` (OSS-031), Certbot generation (OSS-044) | API hits exist. Python backend-qualified generation (OSS-011) and Ruby key-receiver signing (OSS-037) have no hit at the operation. |
| ECDSA | Certbot EC generation under its ECDSA branch (OSS-045); Ruby JWT ECDSA signer (OSS-038) | Generation is detected; Ruby `dsa_sign_asn1` and Paramiko signing (OSS-041) are unmatched. EC generation alone is not an ECDSA-purpose assertion. |
| ECDH | Net::SSH shared-secret calculation (OSS-035); Paramiko `exchange(ec.ECDH(), ...)` (OSS-042) | Nearby key constructors are found, but agreement calls are unmatched. |
| X.509 | Smallstep CSR construction (OSS-029), Teleport certificate creation (OSS-032) | Source API calls have no direct hit. Existing certificate-object rules find public fixtures separately. |
| SSH | MINA hybrid preferences (OSS-026), Smallstep SSH certificate creation (OSS-064) | SSH RSA strings are found across the corpus; these factory/helper calls are unmatched. |
| JWT/JOSE | jose WebCrypto JWS signing (OSS-014), Ruby RSA/EC JWA signing (OSS-037/038) | Algorithm tables generate many hits; generic signing operations are unmatched. ACME JWS (OSS-030) is not necessarily JWT. |
| ML-KEM | CIRCL encapsulation (OSS-004), Rust SSH generation (OSS-053) | Standard-name references are found; Rust generation has a reference-rule hit. BC implementation (OSS-002), Python suffixed key generation (OSS-043) and OpenPGP noble encapsulation (OSS-047) are unmatched at inspected operations. |
| ML-DSA | jose JWS algorithm entry (OSS-016); cryptography backend generation (OSS-062) | Literal parameter-set name is found; backend generation, BC implementation (OSS-003), OpenPGP `ml_dsa65.sign` (OSS-048) and AWS-LC underscored enum (OSS-058) are unmatched. |
| Hybrid PQC | CIRCL descriptor (OSS-006), rustls defaults (OSS-019/020), OpenPGP combiner (OSS-049) | Some compact descriptor names are found. MINA composite factory names and cryptography suffixed HPKE names (OSS-013) are unmatched. No negotiated protection or combiner security was tested. |

## Suspected false positives and interpretation risks

- **Draft versus standardized implementation:** CIRCL imports a Kyber-named CPA
  primitive inside its explicitly FIPS203 ML-KEM wrapper (OSS-007).
  `PQC_DRAFT_ALGORITHM` marks the import. This is a candidate semantic
  overclassification if interpreted as an exposed draft KEM, not a confirmed
  matcher false positive: the Kyber reference really exists. AWS-LC also retains
  a local `kyber_key` variable in its NID-selected KEM generation (OSS-057).
- **Tests and implementation are not deployed use:** the Ruby/OpenSSL EC unit
  test (OSS-008), EJBCA CMP test utility (OSS-024), library definitions and
  certificate fixtures produce legitimate reference findings. Treating those as
  production vulnerabilities or live credential incidents would exceed the evidence.
- **Overlapping rules:** source locations can have multiple findings, e.g. Go
  RSA imports and JOSE RS/PS table entries. `locationsWithMultipleFindings` and
  full per-rule counts remain visible. Counts are not distinct operations.
- **Names do not determine purpose:** EC generation does not choose ECDSA versus
  ECDH; `dsa_sign_asn1` is used by an EC signer; JWS does not imply JWT; a generic
  SSH signer does not identify its key algorithm.
- **Unmatched does not automatically mean a contract violation:** method
  definitions, computed algorithms, native C, dynamic backends and unsupported
  dependency syntax have different contracts. The 64 observations are output-aware,
  targeted, single-analyst source facts. Unreviewed predictions remain unadjudicated.
  Precision/Recall/F1 and a population accuracy estimate are intentionally absent.

## Inspection errors and limits

Four repositories report `complete_within_scope` (Ruby/OpenSSL, smallstep/certificates,
Paramiko, russh); 22 report `partial`. This is the scanner inspection contract,
not comprehensive algorithm coverage. Every repository retains a not-enumerated
population under ignore policies. The early pilot progress description grouping
all cases as partial was too broad; raw diagnostics and these final counts govern.

- Five `detector_failed` diagnostics: one in EJBCA
  `modules/ejbca-ejb-cli/build.gradle.kts`; four in Node.js intentionally invalid
  package-manifest fixtures. The EJBCA file uses Gradle catalog/project expressions
  and a literal `runtimeOnly(":jboss:client")`; the safe native diagnostic does not
  establish the internal cause. It is a parser/unsupported-input investigation
  candidate, not evidence that Gradle or the project build fails.
- 113 `read_failed` diagnostics point to tracked **symlinks**, not 113 failed
  selected regular-file reads. Coverage counters and diagnostic entry counts are
  separate; no links were followed or rewritten. See each target's
  `diagnostic-errors.json` and raw diagnostics.
- 162 truncation, 468 unsupported and 480 partial-detector diagnostic entries
  remain visible; entries may refer to multiple detectors on one file. 44 files
  exceed the size limit. These are not counts of missed cryptographic operations.
- Teleport's `e` submodule and both AWS-LC native backend submodules were not
  initialized. This study evaluates only the materialized public root trees.
- Acquisition initially stopped while hashing Discourse's media fixture
  `spec/fixtures/images/fake.jpg` with OS errno 1. It was subsequently absent;
  the external cause was not established and no protection was disabled. Its Git
  blob identity is retained as unavailable input and the image is outside scanner
  media scope. One Node.js tracked JavaScript fixture,
  `deps/v8/test/mjsunit/regress/regress-crbug-808192.js`, was also absent; an
  ordinary attempt to materialize its Git bytes received OS errno 1. Its bytes
  remain unavailable and its cryptographic content was not assessed. The exact
  external cause was not established. Both omissions are explicit in the input
  ledgers and acquisition/correction records, separately from scanner exits.
- Final Git-tree validation also found 1,302 Airflow files omitted by upstream
  `export-ignore`, plus 26 existing non-ASCII Node.js/Rails files missing from
  the first path ledger. The Airflow regular files and omitted `CLAUDE.md` symlink
  were restored from exact Git objects before the final Airflow scan
  (56 findings); its initial 54-finding run, intermediate run and old
  manifests remain under `evidence/input-binding-correction/`. Non-ASCII files
  were already scanned and only their bindings changed. Final source hashes
  and Git blob identities cover every available tracked regular file. Normal
  Git checkout CRLF conversion is recorded separately from measured raw-byte
  hashes; no measured bytes were silently normalized.
- First-observed timings are from a shared macOS Apple Silicon host, without
  cache flushing or system-load isolation. The largest added-target child RSS
  was AWS-LC Rust at 414.4 MiB; performance distributions and cross-host claims
  are not established by this study.

## Proposed rule priorities (no implementation in this change)

| Priority | Proposed improvement | Evidence and required care |
|---|---|---|
| 1 | Import/receiver-aware WebCrypto and OpenPGP PQC API coverage; recognize underscored/suffixed/composite standard names | OSS-013/014/017/018/047–050/058. Bind algorithms to imported APIs and descriptors; negative cases must reject unrelated names, display strings and shadowing. Do not label every generic sign call RSA. |
| 2 | Python backend-qualified RSA/PQC key APIs and receiver-bound ECDSA/ECDH operations | OSS-011/012/041–043/062. Track explicit imports, backend gates and key types; retain unknown dynamic values. |
| 3 | Go ECDSA, X.509 CSR/certificate and SSH signer/helper references | OSS-028/029/032/033/064. Separate certificate construction, public object parsing and polymorphic signers. Do not infer key type from helper name alone. |
| 4 | Ruby typed signing/verification/derive calls | OSS-009/035/037–039/060/061. Resolve enclosing OpenSSL key class and receiver provenance; distinguish ECDSA from DSA and DH from ECDH. |
| 5 | Java JCA computed/OID selection, Bouncy Castle operations, and hybrid factory constants | OSS-002/003/022/023/025/026. Record unresolved algorithm rather than guessing RSA; implementation facts and effective configuration remain distinct. |
| 6 | Rust AWS-LC descriptor/helper APIs and standardized/draft name context | OSS-021/056–059 and CIRCL OSS-007. Preserve package/algorithm identity and generic-dispatch uncertainty. Inspect the EJBCA dependency error independently of crypto rule tuning. |

Each proposal requires separately authorized product work and applicable reviews;
no follow-up task was automatically created. Future scored accuracy work needs
complete rule-specific truth populations and independent source-only labels,
using the existing quality corpus/evaluator rather than converting these 64
selected observations into an assumed denominator.

## Reproduction and evidence checks

Each `evidence/scans/<id>/measurement.json` retains the exact argument array, exit,
timeout, wall/CPU/RSS and compressed/raw artifact hashes. Fetch the manifest commit
into a fresh directory without initializing submodules; run the unchanged CLI with
the recorded rules and options and a fresh output directory. Existing targets may
be materialized from the same commit with Git archive. Materializing source files
does not authorize running their scripts. Root-path/time-dependent outputs may
differ; semantic repeatability was checked for CIRCL and jose at the same root.

The six-target pilot completed before expanded acquisition. Both repeat runs
had the same native semantic digest as their first run. The existing resource
helper's six tests passed. Final artifact checks cover all 26 receipts, raw/gzip
digests, source trees, available tracked byte hashes, 64 source spans, native
coverage partitions, table totals, unchanged product inputs and links. See
[validation receipt](../../records/oss-corpus-20261121/evidence/validation.json).

This research/docs-only change does not alter executable behavior or the scored
test corpus. Implementation/test review subagents and full Rust test/clippy runs
are not applicable to the change category; the scanner release build and the
actual scans above were executed. Raw prior reports and the old benchmark entry
points remain unchanged.
