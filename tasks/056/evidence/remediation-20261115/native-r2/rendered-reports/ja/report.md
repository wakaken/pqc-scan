# 耐量子暗号への移行調査レポート

作成日時: 2026-11-15 00:00:00 UTC

対応を確認する候補: 4 / 参考情報: 0 / 調査したファイル: 4

コードや設定の手掛かりをまとめた調査結果です。

|重大度|件数（全検出）|
|---|---:|
|中|4|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[ENVOY056](#rule-0) 日本語の説明が未登録のルールです。原文：Synthetic migration reference|中|量子計算によるリスクあり|1|1|
|[JAVA_CURRENT056](#rule-1) 日本語の説明が未登録のルールです。原文：Synthetic migration reference|中|量子計算によるリスクあり|1|1|
|[JAVA_UNKNOWN056](#rule-2) 日本語の説明が未登録のルールです。原文：Synthetic migration reference|中|量子計算によるリスクあり|1|1|
|[PKI056](#rule-3) 日本語の説明が未登録のルールです。原文：Synthetic migration reference|中|量子計算によるリスクあり|1|1|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## ENVOY056

日本語の説明が未登録のルールです。原文：Synthetic migration reference

確認すること: ルール作成者の確認事項（原文）：Review actual use before migration

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[envoy.yaml:1:1](#finding-0)|synthetic reference|TLS 1.3を優先し、鍵共有方式を確認する / [EnvoyのPQC証明書署名の互換性を確認する](#action-0) / [TLS 1.3を優先し、鍵共有方式を確認する](#action-2) / [EnvoyのTLS設定を見直す](#action-3) / 周辺コードなし|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-1)

<a id="rule-1"></a>

## JAVA_CURRENT056

日本語の説明が未登録のルールです。原文：Synthetic migration reference

確認すること: ルール作成者の確認事項（原文）：Review actual use before migration

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-1"></a>[current/Example.java:1:1](#finding-1)|synthetic reference|Java標準のPQC APIへの移行を検討する / [Java標準のPQC APIへの移行を検討する](#action-4) / 周辺コードなし|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-1)
- [実行環境と通信相手のPQC対応を確認する](#action-5)

<a id="rule-2"></a>

## JAVA_UNKNOWN056

日本語の説明が未登録のルールです。原文：Synthetic migration reference

確認すること: ルール作成者の確認事項（原文）：Review actual use before migration

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-2"></a>[unknown/Example.java:1:1](#finding-2)|synthetic reference|BouncyCastleによるPQC対応を検討する / [Javaの版を確認し、必要ならPQC API対応版へ更新する](#action-6) / [BouncyCastleによるPQC対応を検討する](#action-7) / 周辺コードなし|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-1)
- [実行環境と通信相手のPQC対応を確認する](#action-5)

<a id="rule-3"></a>

## PKI056

日本語の説明が未登録のルールです。原文：Synthetic migration reference

確認すること: ルール作成者の確認事項（原文）：Review actual use before migration

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-3"></a>[certificate.pem:1:1](#finding-3)|synthetic reference|ルール作成者の確認事項（原文）：Review actual use before migration / 周辺コードなし|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-1)
- [証明書の発行と信頼設定の更新を計画する](#action-8)

## 対応案の詳細

各検出箇所から、その箇所に該当する対応案だけを参照します。同じ内容は一度だけ掲載します。

<a id="action-0"></a>


#### EnvoyのPQC証明書署名の互換性を確認する (advisory)

鍵共有と証明書の署名は別です。プロバイダーと通信相手の対応を確認してください。

1. TLSの最低・最高バージョンを見直し、設定に静的RSA鍵交換があれば移行を検討してください。ECDHE_RSAのRSA認証とは区別してください。
2. cipher_suitesはTLS 1.0〜1.2の設定で、TLS 1.3には適用されません。
3. 下流・上流の両方でPQC証明書署名の互換性を試し、段階的な展開と切戻しを準備してください。

参考資料:

- https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/security/ssl.html
- https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/tls.proto
<a id="action-1"></a>


#### 既存方式とPQCを併用し、段階的に移行する (早めに対応)

検出した方式には、量子計算による将来のリスクがあります。接続相手との互換性を確かめながら移行を進めてください。

1. 利用箇所と担当チームを洗い出してください。
2. 既存のPQC・複合構成を確認し、移行が必要なら新旧方式の併用を試してください。
3. 旧方式を廃止する条件と時期を決めてください。

参考資料:

- https://www.nist.gov/itl/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
<a id="action-2"></a>


#### TLS 1.3を優先し、鍵共有方式を確認する (早めに対応)

RSA鍵交換や古いTLS設定は移行時の確認対象です。この一般的な確認項目だけで、RSAの利用が確定するわけではありません。

1. TLS 1.3を優先し、全接続先が対応していれば1.3だけを許可してください。TLS 1.0・1.1は無効にしてください。
2. TLS 1.2は互換性が必要な場合に限り、安全な設定で残してください。理由を記録し、鍵共有方式のPQC対応も別に確認してください。
3. クライアントと負荷分散装置を含め、互換性を継続して確認してください。

参考資料:

- https://www.rfc-editor.org/rfc/rfc8446
<a id="action-3"></a>


#### EnvoyのTLS設定を見直す (早めに対応)

最低・最高バージョンと暗号スイートを設定で管理できます。

1. tls_minimum_protocol_versionはTLSv1_3を優先し、TLSv1_2は互換性が必要な場合に限って検討してください。
2. cipher_suitesはTLS 1.0〜1.2の設定で、TLS 1.3には適用されません。静的RSA鍵交換と、ECDHE_RSAのRSA認証を区別してください。
3. RSA認証を外す前に、証明書の署名方式と通信相手の互換性を別に確認してください。

参考資料:

- https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/tls.proto
<a id="action-4"></a>


#### Java標準のPQC APIへの移行を検討する (早めに対応)

対応するJDKでは、JCAの標準方式名でML-DSA・ML-KEMを指定できます。

1. パラメーターセットを選び、新しいML-DSA鍵ペアを生成してください。RSAの鍵を流用しないでください。
2. 署名側を切り替える前に、対応する公開鍵とJOSE・PKIの符号化形式をすべての検証側へ配備してください。
3. ML-KEMは鍵共有に使う方式です。TLSなどの対応は別に確認してください。
4. 旧方式の検証を残す条件と廃止時期は、移行方針として明示してください。

参考資料:

- https://openjdk.org/jeps/496
- https://openjdk.org/jeps/497

変更前 (java)

    Signature sig = Signature.getInstance("SHA256withRSA");
    sig.initSign(privateKey);
    sig.update(payload);
    byte[] signature = sig.sign();

変更後

    // JDK 24+: provision a new ML-DSA key pair for the selected parameter set.
    // mlDsaPrivateKey is its private key, never the previous RSA key.
    // Provision the matching public key and encoding to verifiers before rollout.
    Signature sig = Signature.getInstance("ML-DSA");
    sig.initSign(mlDsaPrivateKey);
    sig.update(payload);
    byte[] signature = sig.sign();
    // Keep legacy verification only under the explicit migration policy.

<a id="action-5"></a>


#### 実行環境と通信相手のPQC対応を確認する (計画して対応)

コードに方式名があるだけでは、その環境で動くか、接続相手が対応しているかは分かりません。

1. JDK 24以降のML-KEM・ML-DSA対応を確認してください。プロバイダーや通信規約の対応は別に確認が必要です。
2. 実際に配備している版、プロバイダー、機能設定を確認してください。
3. 本番設定を変える前に、接続相手との互換性を試してください。

参考資料:

- https://docs.oracle.com/en/java/javase/27/security/java-secure-socket-extension-jsse-reference-guide.html
<a id="action-6"></a>


#### Javaの版を確認し、必要ならPQC API対応版へ更新する (計画して対応)

版が不明な場合は、配備先のJavaを確認してください。宣言された版だけでは実環境のAPI対応は確定しません。

1. 標準のML-KEM・ML-DSA APIが必要な場合は、JDK 24以降の対応を確認してください。
2. 本番への適用前にCIで互換性を試してください。

参考資料:

- https://openjdk.org/jeps/496
- https://openjdk.org/jeps/497
<a id="action-7"></a>


#### BouncyCastleによるPQC対応を検討する (早めに対応)

標準APIを使えない環境では、ライブラリによる対応が移行候補になります。

1. 解決済みのBouncyCastleの版と配備先JDKの対応を確認してください。例はBC Java 1.85のBCプロバイダーを使います。BCPQCとは区別してください。
2. 用途に合うパラメーターセットを選び、新しいML-DSA鍵ペアなど、方式に対応した鍵を生成してください。RSA鍵を流用せず、公開鍵と符号化形式を検証側へ配備してください。
3. プロバイダーの優先順位とFIPSなどの運用要件への影響を記録してください。

参考資料:

- https://www.bouncycastle.org/java.html
- https://downloads.bouncycastle.org/java/docs/bcprov-jdk18on-javadoc/

変更前 (java)

    Security.addProvider(new BouncyCastleProvider());
    Signature sig = Signature.getInstance("SHA256withRSA", "BC");

変更後

    // BC Java 1.85 example: verify the resolved release/provider supports this name.
    Security.addProvider(new BouncyCastleProvider());
    // Provision a new ML-DSA-65 key pair and its public key to verifiers.
    // mlDsaPrivateKey must not be the previous RSA key.
    Signature sig = Signature.getInstance("ML-DSA-65", "BC");
    sig.initSign(mlDsaPrivateKey);

<a id="action-8"></a>


#### 証明書の発行と信頼設定の更新を計画する (早めに対応)

FIPS 204/205は署名方式の規格であり、共通のハイブリッド証明書形式を定めていません。

1. CBOMで証明書の署名方式と鍵情報を確認してください。
2. 具体的な構成と符号化形式を選び、プロバイダー、認証局、クライアント、信頼ストアの対応を確認してから発行を計画してください。
3. 対応が確認できた構成だけを本番以外で試してください。形式や利用環境が非対応なら未検証のままとし、別の対応済みの移行方法を検討してください。
4. ルート・中間・サーバー証明書の再発行手順を決めてください。

参考資料:

- https://csrc.nist.gov/pubs/fips/204/final
- https://csrc.nist.gov/pubs/fips/205/final

## 棚卸しデータ

- [CBOM](cbom.json): 0
- [Dependency SBOM](dependency-sbom.json): 0

