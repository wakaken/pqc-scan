# OSS scan results

[日本語](README.ja.md) · [Scan your project](../../user/SCANNING.md)

## Repositories

Commits identify the input for each result. The primary selection has three
repositories in each of nine language groups. C and C++ share one group, as do
JavaScript and TypeScript.

### Primary repositories

| Repository | Group | Frozen commit | Inspection | Findings |
| --- | --- | --- | --- | --- |
| [bcgit/bc-java](https://github.com/bcgit/bc-java) | Java | bb81bfa75c9f | Partial | 10,494 |
| [Keyfactor/ejbca-ce](https://github.com/Keyfactor/ejbca-ce) | Java | a46c89e0c6b6 | Partial | 617 |
| [apache/mina-sshd](https://github.com/apache/mina-sshd) | Java | f885e0fc9f68 | Partial | 447 |
| [cloudflare/circl](https://github.com/cloudflare/circl) | Go | 31be93db4772 | Partial | 269 |
| [smallstep/certificates](https://github.com/smallstep/certificates) | Go | 0d8a80647845 | Complete | 1,472 |
| [FiloSottile/age](https://github.com/FiloSottile/age) | Go | b74dce4cdbe3 | Complete | 73 |
| [ruby/openssl](https://github.com/ruby/openssl) | Ruby | b5774711c1f0 | Partial | 238 |
| [net-ssh/net-ssh](https://github.com/net-ssh/net-ssh) | Ruby | 348b78049bfe | Partial | 688 |
| [jwt/ruby-jwt](https://github.com/jwt/ruby-jwt) | Ruby | c22701ef8e0e | Partial | 537 |
| [pyca/cryptography](https://github.com/pyca/cryptography) | Python | 0b3a52a7cb58 | Partial | 2,979 |
| [paramiko/paramiko](https://github.com/paramiko/paramiko) | Python | 0e9ccb91e0f9 | Complete | 263 |
| [certbot/certbot](https://github.com/certbot/certbot) | Python | 9d5c9d44e57d | Partial | 292 |
| [panva/jose](https://github.com/panva/jose) | JavaScript / TypeScript | d1e200f22d2b | Partial | 973 |
| [openpgpjs/openpgpjs](https://github.com/openpgpjs/openpgpjs) | JavaScript / TypeScript | ea041b3c70eb | Partial | 37 |
| [paulmillr/noble-post-quantum](https://github.com/paulmillr/noble-post-quantum) | JavaScript / TypeScript | 48e493397e69 | Partial | 288 |
| [rustls/rustls](https://github.com/rustls/rustls) | Rust | 796b8d3b0848 | Partial | 792 |
| [Eugeny/russh](https://github.com/Eugeny/russh) | Rust | 56a999871aa1 | Complete | 93 |
| [RustCrypto/KEMs](https://github.com/RustCrypto/KEMs) | Rust | b26db85a9b44 | Partial | 167 |
| [phpseclib/phpseclib](https://github.com/phpseclib/phpseclib) | PHP | 808f39f816c4 | Partial | 195 |
| [firebase/php-jwt](https://github.com/firebase/php-jwt) | PHP | 99efeb17d6de | Partial | 289 |
| [lcobucci/jwt](https://github.com/lcobucci/jwt) | PHP | 8bd5e392228c | Partial | 78 |
| [openssl/openssl](https://github.com/openssl/openssl) | C / C++ | 18d0baa70de7 | Partial | 6,031 |
| [google/boringssl](https://github.com/google/boringssl) | C / C++ | 5ec099107553 | Partial | 14,380 |
| [wolfSSL/wolfssl](https://github.com/wolfSSL/wolfssl) | C / C++ | f4f52d975ff1 | Partial | 3,358 |
| [bcgit/bc-csharp](https://github.com/bcgit/bc-csharp) | C# | 4814a10bc95d | Partial | 6,200 |
| [AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet](https://github.com/AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet) | C# | 0d96aca5a57a | Partial | 451 |
| [openiddict/openiddict-core](https://github.com/openiddict/openiddict-core) | C# | 68269d28b439 | Partial | 115 |

### Comparison repositories

| Repository | Group | Frozen commit | Inspection | Findings |
| --- | --- | --- | --- | --- |
| [keycloak/keycloak](https://github.com/keycloak/keycloak) | Java | 18559075f50c | Partial | 4,072 |
| [apache/airflow](https://github.com/apache/airflow) | Python | c4e2d0054bd0 | Partial | 56 |
| [kubernetes/kubernetes](https://github.com/kubernetes/kubernetes) | Go | a0594a28a435 | Partial | 1,989 |
| [nodejs/node](https://github.com/nodejs/node) | JavaScript / TypeScript | 330e3ee67e84 | Partial | 5,978 |
| [n8n-io/n8n](https://github.com/n8n-io/n8n) | JavaScript / TypeScript | 9a84403d2fae | Complete | 282 |
| [discourse/discourse](https://github.com/discourse/discourse) | Ruby | 6dfa59ea612b | Partial | 72 |
| [rails/rails](https://github.com/rails/rails) | Ruby | ee9b18fdb0fc | Partial | 8 |
| [vectordotdev/vector](https://github.com/vectordotdev/vector) | Rust | 7e97ead1775c | Partial | 185 |

## Results at a glance

| Population | Succeeded | Complete | Partial | Findings |
| --- | --- | --- | --- | --- |
| Primary 27 | 27 | 4 | 23 | 51,816 |
| Comparison 8 | 8 | 1 | 7 | 12,642 |

All 35 scans completed. Complete means inspection finished within the selected
input and settings. Partial means some input could not be fully inspected.
Neither state guarantees safety. Findings are static references, not confirmed
vulnerabilities or proof of runtime use. Zero findings do not establish safety.

## Findings by language

These counts describe finding locations, not repository language groups.
A repository can contain several languages. Headers without a resolved language,
configuration files, and unattributed locations remain separate.

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

## Findings by category

Categories describe recorded references and do not establish active use of a
technology, confirmed vulnerabilities, or completed PQC migration.

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

## Algorithm assets

Native inventory groups cryptographic observations into algorithm assets.
These counts differ from finding counts. Unknown means that the family could
not be identified; an identified family does not establish safety or conformance.

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
