# Selected 35 repositories, final v0.5 evaluation

The goal is to finish the selected 35-repository evaluation on the final v0.5 source, retain the fixed evaluation boundaries, and identify the work still needed before release. All 35 repositories have fresh successful executions from the same production release binary. Primary and comparison populations remain separate. Inspection is Complete for 4 primary and 1 comparison repositories. The other 30 remain Partial. Finding counts describe output volume and do not establish detection accuracy.

This report records completed local evaluation stages and preserves their limits. Independent technical and work-record verdicts are bound separately in the [final closure](/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan/benchmark/evidence/selected35-v05-release-final-index.json). No publication, hosted workflow dispatch, or release approval is claimed. The [fact bindings](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-report-ready/FACT-BINDINGS.json) retain full source, input and output identities.

## Fresh final-source execution covers the fixed selection

| Population | Succeeded | Complete | Partial | Findings | Product accuracy |
| --- | --- | --- | --- | --- | --- |
| Primary 27 | 27 | 4 | 23 | 51,816 | null |
| Comparison 8 | 8 | 1 | 7 | 12,642 | null |

The selection is the original purposeful 27-primary plus 8-comparison subset of the 98-repository study. The 63 remaining primary candidates retain future scope. No result from those 63 is required to finish this selected evaluation. The 35 are exposed research inputs, not a random population sample or a fresh blind holdout. Authorized Avast quarantine exclusions for Node and Discourse remain in force. Input manifests, per-entry frozen commits, and unavailable submodule records remain unchanged. [Selection](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/SELECTION.json).

The executable reports `pqc-scan 0.5.0`. Its build profile is `release-debug0`, its SHA-256 is `92f85ef71b8580c527e1ebe5b3e09df9dafb30f151d152ae43dfbb4e4bd08fbd`, and its source revision is `cf3bfe2071228921f5a33d2a44e0f3b94de4d1de` with recorded dirty changes. All 343 product source bindings are in the [build receipt](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/BUILD-RECEIPT.json). Relative to the five-repair source, the integrated product file changes are Cargo version metadata, its lockfile, and the native golden version. That source difference does not add another detector repair. [Integrated source](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/INTEGRATED-PRODUCT-SOURCE.json).

Every repository has one fresh final execution. The run uses one thread, one sample, a 600-second timeout, a 2 MiB maximum file size, default features, all report formats, diagnostics, native inventory, and CycloneDX 1.7. The former 28-run subset and debug outputs are historical evidence, not substitutes for these executions. `collectPerformance=false` leaves wall, CPU, RSS, load, and swap measurements unavailable. No throughput, speedup, or performance regression conclusion follows. OSS files were read statically. They were not executed, built, or used for dependency resolution. [Protocol](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/PROTOCOL.json), [Descriptive results](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/SELECTED-DESCRIPTIVE-COUNTS.json).

### The primary population has 27 repositories

| Repository | Group | Frozen commit | Execution | Inspection | Findings | Owned lines / units | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [bcgit/bc-java](https://github.com/bcgit/bc-java) | Java | bb81bfa75c9f | Succeeded | Partial | 10,494 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/bcgit__bc-java/v05-release-final/61f585a6fc184776b5908a4bdf8009d0/output/report.html) |
| [Keyfactor/ejbca-ce](https://github.com/Keyfactor/ejbca-ce) | Java | a46c89e0c6b6 | Succeeded | Partial | 617 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/Keyfactor__ejbca-ce/v05-release-final/89755f1271944cbc8255c31d942d911d/output/report.html) |
| [apache/mina-sshd](https://github.com/apache/mina-sshd) | Java | f885e0fc9f68 | Succeeded | Partial | 447 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/apache__mina-sshd/v05-release-final/7b52d8d4e46d4960ba42b6de114a0a00/output/report.html) |
| [cloudflare/circl](https://github.com/cloudflare/circl) | Go | 31be93db4772 | Succeeded | Partial | 269 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/cloudflare__circl/v05-release-final/00d46dfea1494704852598af3b469925/output/report.html) |
| [smallstep/certificates](https://github.com/smallstep/certificates) | Go | 0d8a80647845 | Succeeded | Complete | 1,472 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/smallstep__certificates/v05-release-final/5f41ce9fa0ac4e629fd12e11cc107ee6/output/report.html) |
| [FiloSottile/age](https://github.com/FiloSottile/age) | Go | b74dce4cdbe3 | Succeeded | Complete | 73 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/FiloSottile__age/v05-release-final/42476803c3ba4b2987f8b7b908ca6ae6/output/report.html) |
| [ruby/openssl](https://github.com/ruby/openssl) | Ruby | b5774711c1f0 | Succeeded | Partial | 238 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/ruby__openssl/v05-release-final/05b71dd9aa864ca187db4c8ff3471234/output/report.html) |
| [net-ssh/net-ssh](https://github.com/net-ssh/net-ssh) | Ruby | 348b78049bfe | Succeeded | Partial | 688 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/net-ssh__net-ssh/v05-release-final/240c938380c644fd8db1eef951bf4ad5/output/report.html) |
| [jwt/ruby-jwt](https://github.com/jwt/ruby-jwt) | Ruby | c22701ef8e0e | Succeeded | Partial | 537 | 4 / 1 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/jwt__ruby-jwt/v05-release-final/139f0200d85a4a1493d7a908f5ed3ca8/output/report.html) |
| [pyca/cryptography](https://github.com/pyca/cryptography) | Python | 0b3a52a7cb58 | Succeeded | Partial | 2,979 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/pyca__cryptography/v05-release-final/d3e1874a38b84e7aa5d3a823af40fd70/output/report.html) |
| [paramiko/paramiko](https://github.com/paramiko/paramiko) | Python | 0e9ccb91e0f9 | Succeeded | Complete | 263 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/paramiko__paramiko/v05-release-final/227331a1178d405b9ea5ef632652b36f/output/report.html) |
| [certbot/certbot](https://github.com/certbot/certbot) | Python | 9d5c9d44e57d | Succeeded | Partial | 292 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/certbot__certbot/v05-release-final/ec053039fac34ba8afa89a3961c0b5b0/output/report.html) |
| [panva/jose](https://github.com/panva/jose) | JavaScript / TypeScript | d1e200f22d2b | Succeeded | Partial | 973 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/panva__jose/v05-release-final/3982317245244a65b9864e5615e4697c/output/report.html) |
| [openpgpjs/openpgpjs](https://github.com/openpgpjs/openpgpjs) | JavaScript / TypeScript | ea041b3c70eb | Succeeded | Partial | 37 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/openpgpjs__openpgpjs/v05-release-final/63bf30eddef64fad9a4d96496235fee0/output/report.html) |
| [paulmillr/noble-post-quantum](https://github.com/paulmillr/noble-post-quantum) | JavaScript / TypeScript | 48e493397e69 | Succeeded | Partial | 288 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/paulmillr__noble-post-quantum/v05-release-final/088c9b16edfe4d6890ac9e4e11162258/output/report.html) |
| [rustls/rustls](https://github.com/rustls/rustls) | Rust | 796b8d3b0848 | Succeeded | Partial | 792 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/rustls__rustls/v05-release-final/2cb602c00fe641c8a6604442f000cc49/output/report.html) |
| [Eugeny/russh](https://github.com/Eugeny/russh) | Rust | 56a999871aa1 | Succeeded | Complete | 93 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/Eugeny__russh/v05-release-final/9cdd3ca0d3724ee1bf4b185d8755ae9e/output/report.html) |
| [RustCrypto/KEMs](https://github.com/RustCrypto/KEMs) | Rust | b26db85a9b44 | Succeeded | Partial | 167 | 4 / 1 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/RustCrypto__KEMs/v05-release-final/fe352a2dd62b41d0acab13ce9ed47c33/output/report.html) |
| [phpseclib/phpseclib](https://github.com/phpseclib/phpseclib) | PHP | 808f39f816c4 | Succeeded | Partial | 195 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/phpseclib__phpseclib/v05-release-final/11aeeba1d4864b87b8f59a5f572faf10/output/report.html) |
| [firebase/php-jwt](https://github.com/firebase/php-jwt) | PHP | 99efeb17d6de | Succeeded | Partial | 289 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/firebase__php-jwt/v05-release-final/5ccc63115f0c4fd5a081e4824d04a30e/output/report.html) |
| [lcobucci/jwt](https://github.com/lcobucci/jwt) | PHP | 8bd5e392228c | Succeeded | Partial | 78 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/lcobucci__jwt/v05-release-final/b6be5f37ef2541ab97a7a1bbfbed16a0/output/report.html) |
| [openssl/openssl](https://github.com/openssl/openssl) | C / C++ | 18d0baa70de7 | Succeeded | Partial | 6,031 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/openssl__openssl/v05-release-final/9e26be7ce7e94cada9ce6dd2338c048d/output/report.html) |
| [google/boringssl](https://github.com/google/boringssl) | C / C++ | 5ec099107553 | Succeeded | Partial | 14,380 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/google__boringssl/v05-release-final/8a0b1a7897064ebdad051ec36e5395a6/output/report.html) |
| [wolfSSL/wolfssl](https://github.com/wolfSSL/wolfssl) | C / C++ | f4f52d975ff1 | Succeeded | Partial | 3,358 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/wolfSSL__wolfssl/v05-release-final/0b0d39d0af7c4d53a31310701648cd39/output/report.html) |
| [bcgit/bc-csharp](https://github.com/bcgit/bc-csharp) | C# | 4814a10bc95d | Succeeded | Partial | 6,200 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/bcgit__bc-csharp/v05-release-final/94e9a10764da49f9bc0dcac2e854e504/output/report.html) |
| [AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet](https://github.com/AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet) | C# | 0d96aca5a57a | Succeeded | Partial | 451 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/AzureAD__azure-activedirectory-identitymodel-extensions-for-dotnet/v05-release-final/94e480ec427a495485f3a3301616cd6e/output/report.html) |
| [openiddict/openiddict-core](https://github.com/openiddict/openiddict-core) | C# | 68269d28b439 | Succeeded | Partial | 115 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/openiddict__openiddict-core/v05-release-final/e7456f55d1eb493882381644467793e0/output/report.html) |

### The comparison population has 8 repositories

| Repository | Group | Frozen commit | Execution | Inspection | Findings | Owned lines / units | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [keycloak/keycloak](https://github.com/keycloak/keycloak) | Java | 18559075f50c | Succeeded | Partial | 4,072 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/keycloak__keycloak/v05-release-final/c00a474f88d04e4182ab0e47d9c1637f/output/report.html) |
| [apache/airflow](https://github.com/apache/airflow) | Python | c4e2d0054bd0 | Succeeded | Partial | 56 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/apache__airflow/v05-release-final/ab55d61b9085496ba53329368c1960d1/output/report.html) |
| [kubernetes/kubernetes](https://github.com/kubernetes/kubernetes) | Go | a0594a28a435 | Succeeded | Partial | 1,989 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/kubernetes__kubernetes/v05-release-final/46f8264ac8a04f1aba47eaa979d076d3/output/report.html) |
| [nodejs/node](https://github.com/nodejs/node) | JavaScript / TypeScript | 330e3ee67e84 | Succeeded | Partial | 5,978 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/nodejs__node/v05-release-final/9ff29643ef924bd78f9c029f06be1326/output/report.html) |
| [n8n-io/n8n](https://github.com/n8n-io/n8n) | JavaScript / TypeScript | 9a84403d2fae | Succeeded | Complete | 282 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/n8n-io__n8n/v05-release-final/9fbf79e16f264f79b5b7234f3ea266af/output/report.html) |
| [discourse/discourse](https://github.com/discourse/discourse) | Ruby | 6dfa59ea612b | Succeeded | Partial | 72 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/discourse__discourse/v05-release-final/6cee5fe8973d4462af219da74c67b9f3/output/report.html) |
| [rails/rails](https://github.com/rails/rails) | Ruby | ee9b18fdb0fc | Succeeded | Partial | 8 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/rails__rails/v05-release-final/7c7ad5e72c464062beda34bf59c26234/output/report.html) |
| [vectordotdev/vector](https://github.com/vectordotdev/vector) | Rust | 7e97ead1775c | Succeeded | Partial | 185 | 4 / 0 | [Output](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/vectordotdev__vector/v05-release-final/bd393f4ec3b9402380925e8ca7a08951/output/report.html) |

All rows are one successful execution. The owned-line and canonical-unit columns describe the fixed annotation scope, not complete project labels. Full commit hashes and input-manifest digests appear in the fact-binding manifest. Each Output link opens the current generated HTML report.

## Nine selection groups and 11 location languages answer different questions

The primary population has nine repository groups with three repositories each. C and C++ share one group. JavaScript and TypeScript share another group. This is not an 11-by-3 design.

| Primary selection group | Repositories | Findings |
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

The next table classifies finding locations, not repository selection. A repository can contain several languages. Unresolved C/C++ headers, configuration files, and unattributed locations remain explicit.

| Finding-location language | Primary 27 | Comparison 8 |
| --- | --- | --- |
| C | 4,470 | 1,439 |
| C++ | 981 | 97 |
| C# | 6,471 | 0 |
| Go | 2,260 | 1,854 |
| Java | 6,847 | 3,292 |
| JavaScript | 68 | 887 |
| PHP | 491 | 0 |
| Python | 1,081 | 59 |
| Ruby | 1,329 | 73 |
| Rust | 1,110 | 8 |
| TypeScript | 1,228 | 283 |
| C/C++ header unresolved | 534 | 958 |
| Manifest/configuration | 9,254 | 754 |
| Other/unattributed | 15,692 | 2,938 |

The following category counts are finding observations. CryptoAPI, TLS, SSH, PKI, JWT, and PQC are separate recorded categories. KeyMaterial and Middleware remain visible. None is a count of confirmed vulnerabilities, executed operations, or independent truth labels.

| Finding category | Primary 27 | Comparison 8 |
| --- | --- | --- |
| CryptoAPI | 12,946 | 4,784 |
| JWT | 10,936 | 3,938 |
| KeyMaterial | 1,501 | 267 |
| Middleware | 34 | 0 |
| PKI | 13,459 | 498 |
| PQC | 3,728 | 493 |
| SSH | 1,351 | 57 |
| TLS | 7,861 | 2,605 |

The next table counts only allowlisted Native algorithm assets summed across repositories. These are neither finding counts nor operation counts. Unknown remains Unknown. These assets do not prove an algorithm is active, safe, correctly implemented, or standards conformant. [Technology populations](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-native-assessment-63db5aeaadda43a4809e1f1008982399/TECHNOLOGY-POPULATIONS.json).

| Native algorithm-asset family | Primary 27 | Comparison 8 |
| --- | --- | --- |
| ECDH | 5 | 1 |
| ECDSA | 11 | 9 |
| Ed25519 | 6 | 3 |
| ML-DSA | 41 | 6 |
| ML-KEM | 37 | 4 |
| RSA | 23 | 6 |
| Unknown | 300 | 57 |
| X25519 | 3 | 0 |

## Inspection Partial retains input limits and product uncertainty

Succeeded means the execution and required output completed. Complete means inspection completed within the selected scope and configuration. Neither state proves cryptographic safety. The top-level 4/23 and 1/7 inspection partitions include corpus-input limitations. Native coverage counters describe their own selected-file scope and are not interchangeable with that partition. For example, RustCrypto KEMs has complete scanner coverage within available files but remains overall Partial because `thirdparty/wycheproof` is an unpopulated submodule. The generated HTML cannot infer that external corpus-input limit by itself.

Unexpanded gitlinks, approved quarantine exclusions, unsupported certificate or dependency formats, unknown conditional branches, opaque macros, configured header language, and genuine resource limits preserve Unknown, Partial, or Truncated as applicable. Product work remains for unsupported syntax, parser recovery, and unresolved supported cryptographic operations or bindings. OpenSSL source uncertainty and the bounded BoringSSL macro case remain Partial. Input recovery does not authorize fetching dependencies or weakening safety boundaries. [Per-repository coverage and limitations](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-report-ready/FACT-BINDINGS.json), [Prior five-repair classification](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/partial-five-fixes-c61a2ad4383542819d1481269efdc8d9/REPORT-STATIC.md).

## Two fixed annotations support a narrow reference comparison

The unchanged scorer processed all 140 selected owned lines and both admitted positive canonical units. Historical 392-location and 324-claim totals are not reused as this denominator. There are no admitted negative intervals. Unattributed predictions, unmapped propositions, and unknown source facts are not converted to false positives or false negatives. [Final source score](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-source-score/selected-score.json).

| Fixed reference | TP | FP | FN | Precision | Recall | Derived F1 | Product applicability |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Ruby `kid:` member | 0 | 0 | 1 | null | 0/1 | null | Unknown |
| Rust ML-KEM-1024 name reference | 1 | 0 | 0 | 1/1 | 1/1 | 1/1 | Supported reference |
| Two-reference arithmetic | 1 | 0 | 1 | 1/1 | 1/2 | 2/3 | Qualified product rates null |

Ruby line 308 sets JWS routing metadata with `kid:`, then passes the object to JWT.encode. Its unchanged positive reference and historical FN1 remain visible. The current rule contract does not settle whether Ruby symbol-key syntax is promised. Product applicability is Unknown, rather than a confirmed product defect or a post-result unsupported relabel. Reference precision is undefined because no positive prediction joins the interval. This report derives F1 only when both precision and recall are defined. The unchanged scorer itself does not emit F1.

The KEM interval at `ml-kem/src/lib.rs:388` is a supported ML-KEM-1024 test type-reference and matches its existing source-supported positive annotation. Its conditional reference precision, recall, and F1 are each 1/1. It does not establish encapsulation, decapsulation, correct cryptography, full-file precision, or full-project recall.

The combined arithmetic includes one tuning item and one exposed former holdout item. Original raters use the same model family, and projected rule labels do not establish independent gold. Separate reviewers and source hashes improve traceability but do not certify truth independence. Qualified product precision, recall, and F1 remain null for Ruby, KEM, the combined sample, and both repository populations. Empty or unassessed strata retain null. [Independent scope interpretation](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/canonical-scope-interpretation-review/INTERPRETATION.md).

The scorer also reports finite source propositions. Primary has S=38 and canonical-TP-proven D=1. Comparison has S=3 and D=0. The complement of D is not a count of misses, and S is not a broad recall denominator. The raw scorer retains `evaluationComplete=false`, `nativeFieldRepresentation=NotRun`, and generic remaining requirements because it does not consume the separate Native, browser, or review stages. Its `selectedScoringComplete=true` is the completed source-score stage only. The final coordinator must join separately bound completion evidence without rewriting that raw result.

## Native assessment preserves finite disagreement and null accuracy

The 41 selected Native cases contain 38 primary cases and 3 comparison cases. Only 11 primary cases have selectors. Against the unchanged approved oracle, two agree, nine fail attribute agreement, and 27 primary cases are not assessed. All three comparison cases remain not assessed.

| Population | Selected cases | With selector | Agreed | Failed agreement | Not assessed | Field agreement | Product P/R/F1 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| primary | 38 | 11 | 2 | 9 | 27 | 2/11 | null |
| comparison | 3 | 0 | 0 | 0 | 3 | null | null |

Primary field-representation agreement is 2/11. This is not classifier precision, recall, F1, or whole-source coverage. The nine cases without exact joined locations do not become false negatives. The historical finite fields and their current results are unchanged. No blind or independent-gold claim is made. Formal product rates remain null. [Final Native assessment](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-native-assessment-63db5aeaadda43a4809e1f1008982399/NATIVE-ASSESSMENT.json).

## Five repairs have bounded behavior claims

| Repair | Behavior and boundary |
| --- | --- |
| C# name-budget accounting | Check eligible name syntax before charging. The padded reproduction recovers two RSA references. Real long names remain bounded. |
| C/C++ name-budget accounting | Check identifier and static-callee shapes before charging. Keep the genuine 256-byte identifier and 512-byte callee limits. |
| Partial cause explanations | Distinguish parser recovery, conditional source, unresolved supported binding, opaque macros, and other uncertainty. Keep combined causes and Truncated precedence. |
| Ordinary receiver calls | An ordinary non-crypto call alone does not manufacture crypto Partial. Unresolved supported cryptographic operations still retain Partial. |
| Finite wolfSSL and BoringSSL coverage | Recognize exact unshadowed `wc_MakeRsaKey` references and definitions. Exclude prototypes, members, pointers, and shadows. The proven BoringSSL trailing macro artifact retains its position and Partial status. |

The prior matched debug comparison recorded 474 additional identities, 420 C# and 54 wolfSSL, with no removals. Those additions are static references rather than new true-positive labels. The prior independent bounded regression used 44 pairs and seven EN/JA pairs with 103 supplemental assertions. Historical evidence remains historical. [Five-repair evidence](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/partial-five-fixes-c61a2ad4383542819d1481269efdc8d9/REPORT-STATIC.md).

The final release-versus-debug comparison is cross-profile consistency evidence, not a same-condition causal comparison or a speedup test. All 34 comparable repositories have identical rule/location tuple multisets and legacy ID sets. Full finding objects are identical for 20. Fourteen retain differences in 1,690 certificate evaluation-time metadata fields, including 881 previews containing the same changed timestamp. Other finding fields remain unchanged. Those clocks are reported, not normalized away.

The original matched debug comparison is independently rechecked as 474 added identities and zero removed. Additions are wolfSSL 54, bc-csharp 412, AzureAD 2 and OpenIddict 6. They are references, not new TP labels. All original 33 finite regression cases are reassessed against current full outputs, including Node.js. Fifteen retain removal of the old unwanted hit, and eighteen preserve supported wording and ID. The bounded privacy check finds zero survivors among twelve selected fragments from four private values. This does not establish universal secret detection.

The old and current Node focal projection has 40 identical findings, including full objects. Current whole Node completes within 600 seconds with 5,978 findings. Historical debug timeouts remain preserved. Failed adapter attempts remain recorded. Their missing-old-Node metadata, object-versus-array manifest and source-context assumptions were corrected under separate source reviews before the final successful two-command run. Runtime343 and tracked context are unchanged across that run. [Actual comparison and reassessment](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-five-fix-regression-4d31bbc6f5c140c2b6550c4360e4d416/SUMMARY.json).

## Ten earlier improvement themes retain their own scope

| Existing theme | In primary 27 | In comparison 8 | In future 63 |
| --- | --- | --- | --- |
| API-RUBY-SODIUM-BINDINGS | 0 | 0 | 1 |
| API-PYTHON-OQS-ROLES | 0 | 0 | 1 |
| NATIVE-PQC-UNDERSCORE-ALIASES | 4 | 0 | 0 |
| INSPECT-SOURCE-GRAMMAR | 13 | 4 | 37 |
| INSPECT-DEPENDENCY-FAILURE | 2 | 1 | 4 |
| INSPECT-DEPENDENCY-SUBSET | 8 | 1 | 18 |
| INSPECT-DEPENDENCY-COMPLETENESS | 2 | 0 | 6 |
| INSPECT-CERTIFICATE-FORMAT | 13 | 1 | 22 |
| REPORT-DIAGNOSTIC-TRIAGE | 3 | 1 | 7 |
| RUBY-JWT-MEMBER-SYNTAX | 1 | 0 | 0 |

These are intersections with the existing ten-theme ledger, not ten new release gates or defect counts. Five bounded repairs do not close every theme. Confirmed RbNaCl and liboqs-python API gaps lie outside the selected 35 and remain future work. Ruby applicability, Native underscore aliases, certificate and dependency limits, source grammar, and diagnostic triage remain unresolved where recorded. [Selected-scope mapping](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/IMPROVEMENT-SELECTED35-MAPPING.json).

## Final-source validation and output audits have distinct limits

| Check | Actual recorded result | Evidence |
| --- | --- | --- |
| Default workspace | Build, fmt check, clippy pass. Tests 1,064 passed, 3 ignored. | [Default receipt](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/default-VALIDATION.json) |
| All features | Build and clippy pass. Tests 1,126 passed, 5 ignored. Shared fmt check belongs to the same source. | [All-feature receipt](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/all-features-VALIDATION.json) |
| Temporal | 10 tests pass. | [Temporal receipt](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/TEMPORAL-FINAL-VALIDATION.json) |
| Real CLI smoke | 33 assertions pass across 11 commands. Positive, negative, EN/JA, identities, rejection, and unchanged inputs are checked. | [Smoke manifest](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/verification/smoke-yn70sxx5/manifest.json) |
| Distribution checks | 26 pass; both actual final packages pass repeat archive and denied-network installation checks. | [Package receipt](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/local-final-package-execution-ad1987632c2b48268a9be4bd6cea09dc/FINAL-LOCAL-PACKAGE-RECEIPT.json) |
| Current CI applicability | Existing 50 methods and 13 tamper variants pass. This admission fixture does not establish actual detector accuracy. | [CI validation](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/ci-final-validation/VALIDATION.json) |
| All 35 output audits | Exit 0. All 35 rows pass. Seven JSON/schema/reference-bearing outputs are checked. | [Output audit](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-output-audit-and-source-score/AUDIT.json) |

Each repository retains 11 requested reports and two process streams, for 13 files. The seven audited machine outputs include legacy JSON, Native inventory, SARIF, legacy CBOM, dependency SBOM, and both CycloneDX 1.7 outputs. Official schemas and local references are checked where standards apply. Legacy formats have no official schema. Omitted SARIF columns, CBOM uncertain groups, and unresolved grouping differences retain their recorded limits. Schema validity does not establish detection accuracy or rendered acceptance.

The frozen quality corpus has two semantically stable repeats, stable diagnostics, order, and score. The original report call failed because it received a summary file rather than the baseline directory. The corrected report-only call exits 0 and does not rescan. Both attempts remain recorded. [Correction](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-QUALITY-REPORT-CORRECTION.json), [Quality result](/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan/target/v05-release-quality-0b3e1453492a47fdba50736aedd7bed7-assessed-v2/quality.json).

| Layer | Existing split | TP | FP | FN | TN | eligible | unsupported | ambiguous |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Original | holdout | 4 | 0 | 0 | 0 | 4 | 0 | 0 |
| Original | tuning | 94 | 2 | 4 | 93 | 191 | 47 | 4 |
| Supported-input | holdout | 4 | 0 | 0 | 0 | 4 | 0 | 0 |
| Supported-input | tuning | 94 | 2 | 0 | 91 | 185 | 53 | 4 |

These are existing frozen-corpus annotation-agreement counts, separate from selected35. Two unassigned tuning outputs contribute the original FP count and are not established product false positives. Current supported-input review covers 203 original inputs and 246 nominal questions. Six encrypted inner-property questions are excluded while original truth stays frozen. The remaining 240 questions contain 189 eligible questions, 98 positive and 91 negative, plus 47 unsupported and four ambiguous. The scorer represents the six exclusions as unsupported, for 53 unsupported. This post-observation applicability correction is not detector improvement or a fresh independent evaluation. Quality `releaseDecision` remains Unknown and performance remains Not Tested. The existing 197 synthetic temporal methods and outer wrapper retain applicable unchanged inputs. Of 111 recorded harness bindings, only the CI applicability test differs and its current consumers passed. The compiled current witness is separately rerun. The current workflow Node suite passes all85 tests with no skips. This does not claim a complete local or hosted quality-workflow run. [Validation applicability](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-VALIDATION-APPLICABILITY.json).

### Actual packages and browser observations are retained

Both static and network macOS arm64 variants pass all sixteen final package commands. Each pair of archives is byte-identical for the same supplied binary and source assets. Existing verification, checksums and network-denied clean-prefix installation pass. Both contain the exact139 rule IDs and YAML bytes and produce the three expected synthetic fixture findings. Current selected dependency graphs contain177 and304 packages. Neither has a notice marked review_required. The network graph selects signal-hook and excludes ctrlc and Objective-C entries. This is declared dependency-graph and notice coverage, not a legal-compliance certification or independent reproducible builds. [Package closure](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/local-final-package-execution-ad1987632c2b48268a9be4bd6cea09dc/FINAL-LOCAL-PACKAGE-RECEIPT.json).

Actual C#, BoringSSL and KEM reports were observed in the Codex in-app browser and Chrome158.0.8127.88. Observations cover Japanese and English, desktop and390x844 mobile, search/empty/reset, source detail, rule dialog, Escape/focus return, pagination and Partial reasons. Report reading and interactions worked after reload with external-origin subresources/connections blocked by CSP and loopback allowed. This is not an OS-offline or top-level navigation restriction.

Derived previews from the actual archive cover the English/Japanese README to scanning/coverage anchors, distribution to release notes, saved-evidence/probing/privacy/security guides and nine loaded local images. Original Markdown bytes remain unchanged. The preview does not certify GitHub-flavored Markdown parity. Existing guide README backlinks lead online, so entire-guide offline navigation is not claimed. Tool transport and .md-route limitations, corrected HTML aliases, screenshots and observed paths are retained in the [browser receipt](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-BROWSER-ACCEPTANCE.json). The visible report is retained at <http://127.0.0.1:56650/bcgit__bc-csharp/report.html?lang=ja&theme=light>.

### The unused original98 preparation tool remains failed

The original pending105-test synthetic replay has now run, including both sparse64MiB oversize-denial cases, which pass. The full run exits1. Its unchanged old110-harness and source-hash authority rejects the current111-harness source. Summary13, preparation12 and root-guard19 modules pass. The remaining modules do not collectively pass. No actual98 index or target is admitted and no scanner is launched. [Actual failed105 replay](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-PREPARATION-105.json).

The independent technical dependency analysis binds18 helper files and explicit dynamic routes. Current35 execution, aggregation, source scoring and native capture/judgment do not call this external98 reader, paired overlay or configure tool. The failed replay remains unfinished future98 preparation work under the user's narrowed scope. It is not a current35 validation success and is not relabelled complete. A future98 task must correct and legitimately refreeze that tool, rerun all105 and boundary controls and obtain a new independent review. [Current35 dependency analysis](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-integrated-technical-review/CURRENT35-LEGACY98-APPLICABILITY.json).

## Storage preservation and external release conditions remain explicit

The initial preservation closure verifies7,963 compressed paths, two unchanged read-only files and709 reference hashes. Only230 ended regenerable test executables were removed in two reviewed115-file operations. Their exact canonical paths, purpose, hashes and regeneration steps remain in the plans and RESTORE.md records. Recorded logical removal is3,863,748,208 bytes. The two operations observed free bytes changing2,892,214,272 to4,864,774,144 and2,732,519,424 to4,505,305,088. These are separate concurrent APFS observations, not additive physical recovery.

Release-library166-file and historical Node8-file compression preserve logical bytes. All455 raw files from current35 remain, with123 transparently compressed. Source, gold, adjudication, sample/split records, logs, manifests, reviews, fixed required binaries and future63 evidence remain. The final storage snapshot observes3,768,180,736 bytes (3.509GiB) free. Shared blocks, nested sizes and concurrent writes prevent exclusive physical attribution. [Exact storage ledger](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-STORAGE-SUMMARY.json).

No CPU, RSS, load, swap or performance collection was added. Scanned OSS code was not executed, its dependencies were not resolved, no unrelated process was stopped and no protected evidence was deleted.

Independent technical review is assigned to codex:gpt-6.1-sol@high and work-record audit to codex:gpt-6.1-terra@high. Their actual verdicts are recorded separately in the final closure and are never supplied by the author. Final-source hosted quality CI and distribution attestation remain unavailable under the no-push/no-publication authorization. Manual workflow dispatch also starts a prohibited performance baseline and must not be used blindly. Disabled protocol jobs are skipped, not passed. Historical L29 remains accepted for its original frozen137-rule scope. The maintainer must explicitly decide its applicability to current139-rule source while current performance is held. Develop integration and publication are separate decisions. Future63 completion and all ten candidate themes are not additional release gates. [Scope and concrete resumption conditions](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-SCOPE-AND-VALIDATION-DISPOSITION.json).
