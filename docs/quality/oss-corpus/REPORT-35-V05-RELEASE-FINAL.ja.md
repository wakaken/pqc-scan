# 選定35件の最終v0.5評価

この評価の目的は、最終v0.5ソースで選定35件の評価を完了し、固定した評価範囲を保ったまま、公開前に残る作業を明らかにすることです。35件すべてについて、同一の製品用releaseバイナリによる新規実行が成功しました。主対象27件と比較対象8件は分けて集計します。検査Completeは主対象4件・比較対象1件で、残り30件はPartialです。検出記録数は出力量であり、検出精度ではありません。

本書はローカルで完了した評価工程と、その限界を記録しています。独立技術レビューと作業記録監査の実際の結果は、別の[完了記録](/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan/benchmark/evidence/selected35-v05-release-final-index.json)に結び付けます。公開、ホスト上のワークフロー実行、リリース承認は行っていません。ソース・入力・出力の完全な対応は[根拠マニフェスト](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-report-ready/FACT-BINDINGS.json)に保存しています。

## 固定した35件を最終ソースで新規実行した

| 対象 | Succeeded | Complete | Partial | 検出記録 | 製品精度 |
| --- | --- | --- | --- | --- | --- |
| 主対象27 | 27 | 4 | 23 | 51,816 | null |
| 比較対象8 | 8 | 1 | 7 | 12,642 | null |

対象は、元の98件の調査から目的に沿って選んだ主対象27件と比較対象8件です。残りの主対象候補63件は、引き続き将来の対象です。選定35件の評価完了条件には加えません。35件は過去の出力を既知の調査入力であり、無作為標本や新しい盲検評価用データではありません。NodeとDiscourseの承認済みAvast隔離除外を維持しました。入力マニフェスト、各固定commit、取得していないsubmoduleの記録も維持しています。[選定記録](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/SELECTION.json)。

実行ファイルの表示は`pqc-scan 0.5.0`、ビルド条件は`release-debug0`です。SHA-256は`92f85ef71b8580c527e1ebe5b3e09df9dafb30f151d152ae43dfbb4e4bd08fbd`、ソースrevisionは`cf3bfe2071228921f5a33d2a44e0f3b94de4d1de`で、未コミットの変更を記録しています。343個の製品ソースの対応は[ビルド証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/BUILD-RECEIPT.json)に保存しました。5修正のソースから変わった製品ファイルは、Cargoの版情報、そのlockfile、Native goldenの版情報です。検出器の追加修正ではありません。[統合ソース記録](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/INTEGRATED-PRODUCT-SOURCE.json)。

全35件で新規実行を1回ずつ行いました。条件は1スレッド、1標本、600秒の上限、最大ファイルサイズ2 MiB、default features、全レポート形式、diagnostics、Native inventory、CycloneDX 1.7です。以前の28件の実行やdebug出力で代用していません。`collectPerformance=false`のため、経過時間、CPU、RSS、load、swapの性能値は未取得です。速度向上や性能回帰の結論は出しません。OSSファイルは静的に読み取り、実行、ビルド、依存解決は行っていません。[実行条件](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/PROTOCOL.json)、[記述集計](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/SELECTED-DESCRIPTIVE-COUNTS.json)。

### 主対象27件の結果

| リポジトリ | 群 | 固定commit | 実行 | 検査 | 検出記録 | 所有行 / 注釈単位 | 根拠 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [bcgit/bc-java](https://github.com/bcgit/bc-java) | Java | bb81bfa75c9f | Succeeded | Partial | 10,494 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/bcgit__bc-java/v05-release-final/61f585a6fc184776b5908a4bdf8009d0/output/report.html) |
| [Keyfactor/ejbca-ce](https://github.com/Keyfactor/ejbca-ce) | Java | a46c89e0c6b6 | Succeeded | Partial | 617 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/Keyfactor__ejbca-ce/v05-release-final/89755f1271944cbc8255c31d942d911d/output/report.html) |
| [apache/mina-sshd](https://github.com/apache/mina-sshd) | Java | f885e0fc9f68 | Succeeded | Partial | 447 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/apache__mina-sshd/v05-release-final/7b52d8d4e46d4960ba42b6de114a0a00/output/report.html) |
| [cloudflare/circl](https://github.com/cloudflare/circl) | Go | 31be93db4772 | Succeeded | Partial | 269 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/cloudflare__circl/v05-release-final/00d46dfea1494704852598af3b469925/output/report.html) |
| [smallstep/certificates](https://github.com/smallstep/certificates) | Go | 0d8a80647845 | Succeeded | Complete | 1,472 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/smallstep__certificates/v05-release-final/5f41ce9fa0ac4e629fd12e11cc107ee6/output/report.html) |
| [FiloSottile/age](https://github.com/FiloSottile/age) | Go | b74dce4cdbe3 | Succeeded | Complete | 73 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/FiloSottile__age/v05-release-final/42476803c3ba4b2987f8b7b908ca6ae6/output/report.html) |
| [ruby/openssl](https://github.com/ruby/openssl) | Ruby | b5774711c1f0 | Succeeded | Partial | 238 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/ruby__openssl/v05-release-final/05b71dd9aa864ca187db4c8ff3471234/output/report.html) |
| [net-ssh/net-ssh](https://github.com/net-ssh/net-ssh) | Ruby | 348b78049bfe | Succeeded | Partial | 688 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/net-ssh__net-ssh/v05-release-final/240c938380c644fd8db1eef951bf4ad5/output/report.html) |
| [jwt/ruby-jwt](https://github.com/jwt/ruby-jwt) | Ruby | c22701ef8e0e | Succeeded | Partial | 537 | 4 / 1 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/jwt__ruby-jwt/v05-release-final/139f0200d85a4a1493d7a908f5ed3ca8/output/report.html) |
| [pyca/cryptography](https://github.com/pyca/cryptography) | Python | 0b3a52a7cb58 | Succeeded | Partial | 2,979 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/pyca__cryptography/v05-release-final/d3e1874a38b84e7aa5d3a823af40fd70/output/report.html) |
| [paramiko/paramiko](https://github.com/paramiko/paramiko) | Python | 0e9ccb91e0f9 | Succeeded | Complete | 263 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/paramiko__paramiko/v05-release-final/227331a1178d405b9ea5ef632652b36f/output/report.html) |
| [certbot/certbot](https://github.com/certbot/certbot) | Python | 9d5c9d44e57d | Succeeded | Partial | 292 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/certbot__certbot/v05-release-final/ec053039fac34ba8afa89a3961c0b5b0/output/report.html) |
| [panva/jose](https://github.com/panva/jose) | JavaScript / TypeScript | d1e200f22d2b | Succeeded | Partial | 973 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/panva__jose/v05-release-final/3982317245244a65b9864e5615e4697c/output/report.html) |
| [openpgpjs/openpgpjs](https://github.com/openpgpjs/openpgpjs) | JavaScript / TypeScript | ea041b3c70eb | Succeeded | Partial | 37 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/openpgpjs__openpgpjs/v05-release-final/63bf30eddef64fad9a4d96496235fee0/output/report.html) |
| [paulmillr/noble-post-quantum](https://github.com/paulmillr/noble-post-quantum) | JavaScript / TypeScript | 48e493397e69 | Succeeded | Partial | 288 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/paulmillr__noble-post-quantum/v05-release-final/088c9b16edfe4d6890ac9e4e11162258/output/report.html) |
| [rustls/rustls](https://github.com/rustls/rustls) | Rust | 796b8d3b0848 | Succeeded | Partial | 792 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/rustls__rustls/v05-release-final/2cb602c00fe641c8a6604442f000cc49/output/report.html) |
| [Eugeny/russh](https://github.com/Eugeny/russh) | Rust | 56a999871aa1 | Succeeded | Complete | 93 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/Eugeny__russh/v05-release-final/9cdd3ca0d3724ee1bf4b185d8755ae9e/output/report.html) |
| [RustCrypto/KEMs](https://github.com/RustCrypto/KEMs) | Rust | b26db85a9b44 | Succeeded | Partial | 167 | 4 / 1 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/RustCrypto__KEMs/v05-release-final/fe352a2dd62b41d0acab13ce9ed47c33/output/report.html) |
| [phpseclib/phpseclib](https://github.com/phpseclib/phpseclib) | PHP | 808f39f816c4 | Succeeded | Partial | 195 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/phpseclib__phpseclib/v05-release-final/11aeeba1d4864b87b8f59a5f572faf10/output/report.html) |
| [firebase/php-jwt](https://github.com/firebase/php-jwt) | PHP | 99efeb17d6de | Succeeded | Partial | 289 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/firebase__php-jwt/v05-release-final/5ccc63115f0c4fd5a081e4824d04a30e/output/report.html) |
| [lcobucci/jwt](https://github.com/lcobucci/jwt) | PHP | 8bd5e392228c | Succeeded | Partial | 78 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/lcobucci__jwt/v05-release-final/b6be5f37ef2541ab97a7a1bbfbed16a0/output/report.html) |
| [openssl/openssl](https://github.com/openssl/openssl) | C / C++ | 18d0baa70de7 | Succeeded | Partial | 6,031 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/openssl__openssl/v05-release-final/9e26be7ce7e94cada9ce6dd2338c048d/output/report.html) |
| [google/boringssl](https://github.com/google/boringssl) | C / C++ | 5ec099107553 | Succeeded | Partial | 14,380 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/google__boringssl/v05-release-final/8a0b1a7897064ebdad051ec36e5395a6/output/report.html) |
| [wolfSSL/wolfssl](https://github.com/wolfSSL/wolfssl) | C / C++ | f4f52d975ff1 | Succeeded | Partial | 3,358 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/wolfSSL__wolfssl/v05-release-final/0b0d39d0af7c4d53a31310701648cd39/output/report.html) |
| [bcgit/bc-csharp](https://github.com/bcgit/bc-csharp) | C# | 4814a10bc95d | Succeeded | Partial | 6,200 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/bcgit__bc-csharp/v05-release-final/94e9a10764da49f9bc0dcac2e854e504/output/report.html) |
| [AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet](https://github.com/AzureAD/azure-activedirectory-identitymodel-extensions-for-dotnet) | C# | 0d96aca5a57a | Succeeded | Partial | 451 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/AzureAD__azure-activedirectory-identitymodel-extensions-for-dotnet/v05-release-final/94e480ec427a495485f3a3301616cd6e/output/report.html) |
| [openiddict/openiddict-core](https://github.com/openiddict/openiddict-core) | C# | 68269d28b439 | Succeeded | Partial | 115 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/openiddict__openiddict-core/v05-release-final/e7456f55d1eb493882381644467793e0/output/report.html) |

### 比較対象8件の結果

| リポジトリ | 群 | 固定commit | 実行 | 検査 | 検出記録 | 所有行 / 注釈単位 | 根拠 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [keycloak/keycloak](https://github.com/keycloak/keycloak) | Java | 18559075f50c | Succeeded | Partial | 4,072 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/keycloak__keycloak/v05-release-final/c00a474f88d04e4182ab0e47d9c1637f/output/report.html) |
| [apache/airflow](https://github.com/apache/airflow) | Python | c4e2d0054bd0 | Succeeded | Partial | 56 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/apache__airflow/v05-release-final/ab55d61b9085496ba53329368c1960d1/output/report.html) |
| [kubernetes/kubernetes](https://github.com/kubernetes/kubernetes) | Go | a0594a28a435 | Succeeded | Partial | 1,989 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/kubernetes__kubernetes/v05-release-final/46f8264ac8a04f1aba47eaa979d076d3/output/report.html) |
| [nodejs/node](https://github.com/nodejs/node) | JavaScript / TypeScript | 330e3ee67e84 | Succeeded | Partial | 5,978 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/nodejs__node/v05-release-final/9ff29643ef924bd78f9c029f06be1326/output/report.html) |
| [n8n-io/n8n](https://github.com/n8n-io/n8n) | JavaScript / TypeScript | 9a84403d2fae | Succeeded | Complete | 282 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/n8n-io__n8n/v05-release-final/9fbf79e16f264f79b5b7234f3ea266af/output/report.html) |
| [discourse/discourse](https://github.com/discourse/discourse) | Ruby | 6dfa59ea612b | Succeeded | Partial | 72 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/discourse__discourse/v05-release-final/6cee5fe8973d4462af219da74c67b9f3/output/report.html) |
| [rails/rails](https://github.com/rails/rails) | Ruby | ee9b18fdb0fc | Succeeded | Partial | 8 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/rails__rails/v05-release-final/7c7ad5e72c464062beda34bf59c26234/output/report.html) |
| [vectordotdev/vector](https://github.com/vectordotdev/vector) | Rust | 7e97ead1775c | Succeeded | Partial | 185 | 4 / 0 | [出力](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/runs/runs/vectordotdev__vector/v05-release-final/bd393f4ec3b9402380925e8ca7a08951/output/report.html) |

各行は新規実行1回の結果です。所有行と注釈単位は固定した注釈範囲を表し、プロジェクト全体の正解データではありません。完全なcommitと入力マニフェストのハッシュは根拠マニフェストにあります。「出力」は今回の生成HTMLを開きます。

## 選定の9群と検出箇所の11言語は区別する

主対象は9群に3件ずつあります。CとC++は1群、JavaScriptとTypeScriptも1群です。11言語に3件ずつある設計ではありません。

| 主対象の選定群 | 件数 | 検出記録 |
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

次の表はリポジトリの選定区分ではなく、検出箇所の言語を集計したものです。1件に複数言語が含まれます。C/C++の言語未確定ヘッダー、設定ファイル、言語を帰属できない箇所も残しています。

| 検出箇所の言語区分 | 主対象27 | 比較対象8 |
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

次のcategory表は検出記録数です。CryptoAPI、TLS、SSH、PKI、JWT、PQCは出力に記録された別々の区分です。KeyMaterialとMiddlewareも省略しません。脆弱性の確定数、実行された操作数、独立した正解ラベル数ではありません。

| 記録のcategory | 主対象27 | 比較対象8 |
| --- | --- | --- |
| CryptoAPI | 12,946 | 4,784 |
| JWT | 10,936 | 3,938 |
| KeyMaterial | 1,501 | 267 |
| Middleware | 34 | 0 |
| PKI | 13,459 | 498 |
| PQC | 3,728 | 493 |
| SSH | 1,351 | 57 |
| TLS | 7,861 | 2,605 |

Nativeの表は、許可した方式assetのfamilyをリポジトリ間で合算したものです。検出記録数や操作数とは異なります。UnknownはUnknownのまま残します。方式の実利用、安全性、実装の正しさ、標準適合を示す結果ではありません。[方式別集計](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-native-assessment-63db5aeaadda43a4809e1f1008982399/TECHNOLOGY-POPULATIONS.json)。

| Native方式assetのfamily | 主対象27 | 比較対象8 |
| --- | --- | --- |
| ECDH | 5 | 1 |
| ECDSA | 11 | 9 |
| Ed25519 | 6 | 3 |
| ML-DSA | 41 | 6 |
| ML-KEM | 37 | 4 |
| RSA | 23 | 6 |
| Unknown | 300 | 57 |
| X25519 | 3 | 0 |

## Partialでは入力制約と製品側の未解決点を残す

Succeededは実行と必要な出力の完了を表し、Completeは選定した入力と設定の範囲内で検査が完了したことを表します。暗号の安全性を保証する状態ではありません。主対象4/23件・比較対象1/7件の検査状態には、コーパス入力の制約も含みます。Nativeのcoverageカウンターは、選定ファイル内の検査範囲を表す別の値です。たとえばRustCrypto KEMsは、存在するファイル内のスキャナー検査は完了していても、`thirdparty/wycheproof`のsubmoduleが未取得のため、全体ではPartialです。生成HTMLだけでは、この外部の入力制約を判断できません。

未展開のgitlink、承認済み隔離除外、未対応の証明書・依存形式、条件分岐、展開できないマクロ、ヘッダーの言語設定、実際の資源上限は、該当するUnknown・Partial・Truncatedを維持します。未対応構文、構文回復、対応する暗号操作やbindingの解決不足は、製品側の未解決点として残ります。OpenSSLのソース不確実性と、限定的に修正したBoringSSLのマクロもPartialです。入力整理を理由に依存を取得したり、安全境界を緩めたりしていません。[各件のcoverageと入力制約](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-report-ready/FACT-BINDINGS.json)、[5修正時の分類](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/partial-five-fixes-c61a2ad4383542819d1481269efdc8d9/REPORT-STATIC.ja.md)。

## 固定した2注釈との限定的な比較結果

変更していないscorerで、選定した所有行140行と、採点対象として受理した正例のcanonical注釈2単位をすべて処理しました。過去の392箇所・324主張を今回の分母として再利用していません。受理した負例区間はありません。対応付けできない出力、ルールに対応しない主張、未知のソース事実をFPやFNへ変換していません。[最終ソース採点](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-source-score/selected-score.json)。

| 固定した参照注釈 | TP | FP | FN | 適合率 | 再現率 | 算出F1 | 製品としての適用範囲 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Rubyの`kid:` member | 0 | 0 | 1 | null | 0/1 | null | Unknown |
| RustのML-KEM-1024方式名参照 | 1 | 0 | 0 | 1/1 | 1/1 | 1/1 | 対応する参照 |
| 2参照の算術合算 | 1 | 0 | 1 | 1/1 | 1/2 | 2/3 | 製品精度はnull |

Rubyの308行目は`kid:`でJWSのキー振り分け情報を設定し、次行でJWT.encodeへ渡します。既存の正例注釈と過去のFN1をそのまま残します。現行ルールの仕様からは、Rubyのシンボルキー構文まで対応を約束しているか確定できません。製品への適用条件はUnknownであり、確定した製品不具合や、結果を見た後の対象外への変更とは扱いません。正例出力が注釈区間に対応付かないため、参照比較の適合率は未定義のnullです。本書では適合率と再現率の両方が定義される場合だけF1を算出します。元のscorerはF1自体を出力しません。

KEMの`ml-kem/src/lib.rs:388`は、現行仕様で対応するML-KEM-1024のテスト型参照で、既存のソース根拠付き正例注釈と一致しました。この1区間に限る参照比較の適合率・再現率・F1は各1/1です。カプセル化、デカプセル化、暗号の正しい動作、ファイル全体の適合率、プロジェクト全体の再現率を示す結果ではありません。

合算には調整用1単位と、現在は結果を既知の過去の保留評価用1単位を含みます。元の評価者は同じモデル系列で、ルールラベルも独立した正解とは認定していません。別のレビュー担当者やソースのハッシュは追跡可能性を高めますが、正解の独立性は保証しません。Ruby、KEM、合算、主対象・比較対象の製品精度としての適合率・再現率・F1はnullのままです。空の区分や未評価もnullを維持します。[独立した適用範囲の解釈](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/canonical-scope-interpretation-review/INTERPRETATION.md)。

有限のソース主張について、主対象はS=38・canonical TPで確認できたD=1、比較対象はS=3・D=0です。D以外を見逃し数とは扱わず、Sを広い再現率の分母にも使いません。元のscorerは別途実行したNative、ブラウザ、レビューを参照しないため、`evaluationComplete=false`、`nativeFieldRepresentation=NotRun`、一般的な残要件を保持しています。`selectedScoringComplete=true`はソース採点工程の完了だけを表します。最終調整担当は元データを書き換えず、別の完了証跡を結び付けます。

## Nativeでは有限の不一致と精度のnullを維持する

選定したNative 41caseは、主対象38caseと比較対象3caseです。selectorがあるのは主対象の11caseだけです。変更していない承認済みoracleとの属性比較では2caseが一致、9caseが不一致、主対象27caseは未評価でした。比較対象3caseもすべて未評価です。

| 対象 | 選定case | selectorあり | 一致 | 不一致 | 未評価 | 属性一致率 | 製品P/R/F1 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 主対象27 | 38 | 11 | 2 | 9 | 27 | 2/11 | null |
| 比較対象8 | 3 | 0 | 0 | 0 | 3 | null | null |

主対象の属性表現一致率は2/11です。分類器の適合率・再現率・F1や、ソース全体の網羅率ではありません。正確な箇所に対応付かない9caseをFNへ変換していません。過去の有限属性と今回の結果は変わっていません。盲検や独立した正解による評価とは扱わず、正式な製品精度はnullとします。[最終Native評価](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-native-assessment-63db5aeaadda43a4809e1f1008982399/NATIVE-ASSESSMENT.json)。

## 5修正は確認した挙動の範囲で説明する

| 修正 | 挙動と境界 |
| --- | --- |
| C#の名前上限の計上 | 名前として対象になる構文か先に確認します。長いコメントを含む再現例でRSA参照2件を回復し、実際の長い名前の上限は保ちます。 |
| C/C++の名前上限の計上 | identifierと静的calleeの構文を先に確認します。実際の256-byte identifierと512-byte calleeの上限は保ちます。 |
| Partialの原因説明 | 構文回復、条件付きソース、対応bindingの未解決、展開できないマクロ、その他の不確実性を分けます。複合原因とTruncatedの優先は保ちます。 |
| 通常のreceiver呼び出し | 暗号と無関係な通常呼び出しだけで暗号のPartialを作りません。対応する暗号操作が未解決ならPartialを残します。 |
| wolfSSLとBoringSSLの限定対応 | shadowされていない正確な`wc_MakeRsaKey`参照・定義を扱います。prototype、member、pointer、shadowは除きます。実証したBoringSSL末尾マクロの位置とPartialは保ちます。 |

以前の同条件debug比較では、追加identityは474件で、C#が420件、wolfSSLが54件、削除はありませんでした。追加は静的参照であり、新たなTPラベルではありません。当時の独立した限定回帰検証は44組と英日7組で、追加assertion 103件が成功しました。過去の証跡は、今回の最終実行とは区別して残します。[5修正の証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/partial-five-fixes-c61a2ad4383542819d1481269efdc8d9/REPORT-STATIC.ja.md)。

最終releaseと以前のdebugの比較は、ビルド条件をまたぐ整合性の確認です。同条件の因果比較や高速化の測定ではありません。比較できる34件すべてで、ルール・位置の組とlegacy IDが一致しました。検出の全属性は20件で一致しました。残る14件の1,690記録では証明書の評価時刻が変わり、そのうち881記録では同じ時刻を含むプレビューも変わっています。他の属性は一致し、時刻差を隠す正規化はしていません。

以前の同条件debug比較も再照合し、追加474件・削除0件を確認しました。内訳はwolfSSL54件、bc-csharp412件、AzureAD2件、OpenIddict6件です。新たなTPではありません。元の33回帰ケースを今回の全体出力に再適用し、不要だった旧検出の削除維持15ケース、対応する文言・IDの維持18ケースを確認しました。Node.jsも全体出力で確認済みです。秘密値4件に由来する限定した12断片は残存0件でした。あらゆる秘密情報を検出できることの証明ではありません。

Node.jsの旧限定出力と今回の全体出力内の同じ範囲では、40件の位置・ルール・ID・全属性が一致しました。今回のNode全体は同じ600秒以内で完了し、検出記録は5,978件でした。以前のdebugタイムアウトは履歴に残します。比較用ツールで発生した旧Nodeの欠落メタデータ、配列形式の入力マニフェスト、ソース照合対象の扱いの誤りは、各失敗を保存して独立レビュー後に修正しました。最終の2コマンドは成功し、343ソースと追跡対象の作業状態は前後一致しています。[実比較・回帰結果](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-five-fix-regression-4d31bbc6f5c140c2b6550c4360e4d416/SUMMARY.json)。

## 既存10テーマは元の範囲を維持する

| 既存テーマ | 主対象27内 | 比較対象8内 | 将来63内 |
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

表は既存10テーマ台帳と対象集合の共通部分を数えたものです。新しい10件のリリース条件や、不具合件数ではありません。限定した5修正でテーマ全体を完了扱いにしません。確認済みのRbNaClとliboqs-pythonのAPI不足は選定35件の外にあり、将来の未解決作業です。Rubyの適用範囲、Nativeの下線alias、証明書・依存形式、ソース構文、diagnosticsの整理も、記録した範囲で未解決です。[選定範囲への対応表](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/IMPROVEMENT-SELECTED35-MAPPING.json)。

## 最終ソースの検証と出力監査を区別する

| 確認 | 実際の記録 | 根拠 |
| --- | --- | --- |
| Default workspace | build、fmt check、clippy成功。テスト1,064成功・3 ignored。 | [Default証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/default-VALIDATION.json) |
| All features | build、clippy成功。テスト1,126成功・5 ignored。同一ソースのfmt checkを共有。 | [全feature証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/all-features-VALIDATION.json) |
| Temporal | 10テスト成功。 | [Temporal証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/TEMPORAL-FINAL-VALIDATION.json) |
| 実CLI smoke | 11コマンドで33 assertion成功。正負例、英日、identity、入力拒否、入力不変を確認。 | [Smokeマニフェスト](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/verification/smoke-yn70sxx5/manifest.json) |
| 配布検証 | 26成功。最終2配布物の反復アーカイブ・通信拒否下の導入も成功。 | [配布物証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/local-final-package-execution-ad1987632c2b48268a9be4bd6cea09dc/FINAL-LOCAL-PACKAGE-RECEIPT.json) |
| 現行CIの適用範囲 | 既存50 methodと13改ざんcaseが成功。この採用条件用fixtureでは実検出器の精度を評価しない。 | [CI検証](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/ci-final-validation/VALIDATION.json) |
| 全35件の出力監査 | exit 0、35行すべて成功。JSON・schema・参照を持つ7出力を確認。 | [出力監査](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-output-audit-and-source-score/AUDIT.json) |

各件で必要な11レポートと標準出力・標準エラーの2ファイルを残し、計13ファイルを保存しています。機械出力の監査対象はlegacy JSON、Native inventory、SARIF、legacy CBOM、dependency SBOM、CycloneDX 1.7の2出力です。標準形式は公式schemaとローカル参照を確認しました。legacy形式には公式schemaがありません。SARIFの列省略、CBOMの不確定group、group化の未解決差分は記録どおり残します。schema適合は検出精度や描画の合格を意味しません。

固定した品質コーパスは2回の意味結果が一致し、diagnostics、順序、スコアも安定しました。最初のレポート呼び出しは、baselineのdirectoryへ渡すべき箇所にsummaryファイルを指定して失敗しました。修正後はレポート生成だけを再実行してexit 0で、スキャンし直していません。両方の記録を残しています。[呼び出し修正](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-QUALITY-REPORT-CORRECTION.json)、[品質結果](/Users/waka/.codex/worktrees/oss-crypto-corpus/pqc-scan/target/v05-release-quality-0b3e1453492a47fdba50736aedd7bed7-assessed-v2/quality.json)。

| 層 | 既存split | TP | FP | FN | TN | eligible | unsupported | ambiguous |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Original | holdout | 4 | 0 | 0 | 0 | 4 | 0 | 0 |
| Original | tuning | 94 | 2 | 4 | 93 | 191 | 47 | 4 |
| Supported-input | holdout | 4 | 0 | 0 | 0 | 4 | 0 | 0 |
| Supported-input | tuning | 94 | 2 | 0 | 91 | 185 | 53 | 4 |

この表は固定品質コーパスの既存注釈との一致数であり、選定35件とは別です。調整用の未帰属出力2件は元のFP数へ算入されていますが、製品の誤検出とは確定していません。現行の対応入力レビューでは元の203入力・246の名目質問を確認しました。暗号化された内部属性の6質問は除外し、元の正解データを保存しています。残る240質問は採点対象189、正例98、負例91、unsupported 47、ambiguous 4です。scorerでは除外6件をunsupportedへ含めるため53となります。結果を見た後の適用範囲修正であり、検出器の精度向上や新しい独立評価とは扱いません。品質結果の`releaseDecision`はUnknown、性能はNot Testedです。197件の合成Temporal検証と外側の起動処理は、同じ入力への適用可能性を照合して再利用しました。記録した111基盤ファイルのうち変更はCI適用範囲テスト1件で、現在の関連検証は成功しています。実ライブラリを使うTemporal検証は別に再実行しました。現行ワークフローのNode検証は85件すべて成功し、skipはありません。これを品質ワークフロー全体やホスト上のCIの成功とは扱いません。[検証の適用範囲](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-VALIDATION-APPLICABILITY.json)。

### 実配布物と実ブラウザの確認結果

macOS arm64向けstatic・network版の最終16コマンドはすべて成功しました。同じバイナリと同梱入力で作成した各2回のアーカイブが一致し、検証、チェックサム、通信を禁止した新規導入も成功しました。139ルールのIDとYAML内容はソースと一致し、合成入力で期待した3検出を確認しました。選択された依存グラフは177件・304件で、追加レビューが必要なnoticeはありません。network版ではsignal-hookが選択され、ctrlcとObjective-Cの項目は選択されていません。これは宣言された依存関係とnoticeの照合であり、法的な適合認定や別々のビルドによる再現性の認定ではありません。[配布物の完了証跡](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/local-final-package-execution-ad1987632c2b48268a9be4bd6cea09dc/FINAL-LOCAL-PACKAGE-RECEIPT.json)。

C#、BoringSSL、KEMの実HTMLをCodex内ブラウザとChrome158.0.8127.88で確認しました。英日切替、デスクトップ・390×844のモバイル表示、検索・該当なし・解除、ソース位置と詳細、ルール説明、Escapeによるフォーカス復帰、ページ送り、Partialの理由を確認しています。CSPで外部オリジンの素材読込と接続を制限し、ループバックを許可した再読込後も操作できました。OS全体のオフライン設定や外部ページへの移動禁止とは区別します。

実アーカイブ由来の文書プレビューでは、英日READMEからスキャン手順・検査範囲のアンカー、導入手順からリリースノート、保存済み入力・接続診断・秘密情報・セキュリティの案内と、ローカル画像9点を確認しました。元のMarkdownは変更していません。プレビューはGitHubのMarkdown表示全体の再現ではなく、既存ガイドのREADMEへのリンクにはオンラインのものが残るため、ガイドの全経路をオフラインで辿れるとは認定していません。操作ツールの制約と補正、実画面は[ブラウザ確認記録](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-BROWSER-ACCEPTANCE.json)に保存しています。表示用の[最終C#レポート](http://127.0.0.1:56650/bcgit__bc-csharp/report.html?lang=ja&theme=light)と、その専用サーバーを残しています。

### 今回使わない旧98件用の準備ツールは未解決として保持する

以前保留だった105件の合成検証をすべて実行しました。64MiBを超える入力を拒否する2ケースは成功しましたが、全体はexit1です。旧110ファイルと旧ソースのハッシュを固定する検査が、現在の111ファイルと異なるソースを拒否しました。集計13件、準備12件、root保護19件の各モジュールは成功し、残るモジュールの全体成功は成立していません。実際の98件の台帳・入力は許可せず、スキャナーも起動していません。[105件の実行記録](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-PREPARATION-105.json)。

独立した技術レビューでは18ファイルと動的な呼出先を照合し、今回の35件の実行・集計・採点・Native取得と評価が、この旧98件専用のreader・paired overlay・configureを呼ばないことを確認しました。利用者が指定した対象縮小に基づき、旧ツールの失敗は将来の98件準備作業として残します。今回35件の検証成功へ加算せず、旧ツールを完了扱いにもしていません。将来再開する場合は、その時点のソースで正当に固定し直し、必要な修正、全105件と境界検証の成功、新たな独立レビューが必要です。[35件の依存経路確認](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/final-integrated-technical-review/CURRENT35-LEGACY98-APPLICABILITY.json)。

## 容量整理の保存条件と外部の公開条件を残す

最初の保全確認では、7,963ファイルを圧縮し、読み取り専用2ファイルを変更せず残し、参照ハッシュ709件の一致を確認しました。削除したのは終了済みで再生成できるテスト実行ファイル230件だけです。独立レビューを受けた115件ずつ2回の整理で、正規パス・用途・ハッシュ・再生成方法を各計画とRESTORE.mdに保存しています。削除対象の論理容量は3,863,748,208 bytesでした。各処理前後の空きは2,892,214,272→4,864,774,144 bytes、2,732,519,424→4,505,305,088 bytesです。APFSと並行書込み下の個別観測であり、物理的な解放量として合算しません。

releaseライブラリ166ファイルと過去のNode出力8ファイルの圧縮でも、論理内容を保持しています。今回35件の生出力455ファイルはすべて残し、うち123ファイルを透過圧縮しました。ソース、正解・裁定、標本・分割、ログ、マニフェスト、レビュー、必要な固定バイナリ、将来63件の証跡も保持しています。容量整理の最終記録時点の空きは3,768,180,736 bytes（3.509GiB）です。共有ブロック・入れ子の容量・並行処理があるため、この作業だけで物理的に解放した容量は確定していません。[正確なパスと容量の台帳](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-STORAGE-SUMMARY.json)。

CPU・RSS・負荷・swap・性能の収集は追加していません。スキャン対象OSSの実行や依存解決、無関係なプロセスの停止、保護対象の証跡の削除も行っていません。

独立技術レビューはcodex:gpt-6.1-sol@high、作業記録監査はcodex:gpt-6.1-terra@highが担当します。実際の判定は別の完了記録に結び付け、著者自身の確認を承認へ算入しません。最終ソースでのホスト上の品質CIと配布物の来歴証明は、push・公開が許可されていないため未取得です。手動ワークフロー実行も性能baselineを開始するため、そのまま実行できません。無効なprotocol jobはskipでありpassではありません。L29の過去の測定は元の固定137ルールの範囲に保持し、現在の139ルールへの適用は性能保留を守った上で維持者が明示的に判断する必要があります。developへの統合と公開は別の判断です。将来63件や既存10テーマ全体の完了を、新しい公開条件として追加しません。[対象範囲と具体的な再開条件](/Users/waka/.cache/pqc-scan-oss98-20261125/accuracy-expansion-20261126-68b7cfbb41114d8c910d4de72b468fd9/v05-release-preparation-bc20507df81c418cabc07b02e6dc74a9/FINAL-SCOPE-AND-VALIDATION-DISPOSITION.json)。
