# 耐量子暗号への移行調査レポート

作成日時: 2026-11-15 07:14:56.769014 UTC

対応を確認する候補: 12 / 参考情報: 2 / 調査したファイル: 5

コードや設定の手掛かりをまとめた調査結果です。

|重大度|件数（全検出）|
|---|---:|
|緊急|3|
|高|2|
|参考情報|2|
|中|7|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[CERT_RSA_2048](#rule-0) 証明書の公開鍵に2048ビットRSAが使われています。|緊急|量子計算によるリスクあり|1|1|
|[CERT_RSA_SIGNATURE](#rule-1) 証明書のRSA系署名方式が見つかりました。|緊急|量子計算によるリスクあり|1|1|
|[PRIVATE_KEY_RSA_HEADER](#rule-2) RSA秘密鍵のヘッダーが見つかりました。|緊急|量子計算とは別のセキュリティ上の注意点|1|1|
|[TS_RSA_KEY_SIZE_LITERAL](#rule-3) RSAの鍵長に関する数値が見つかりました。|高|量子計算によるリスクあり|2|1|
|[API_PY_RSA_MODULE](#rule-4) PythonのRSA関連APIの参照が見つかりました。|中|量子計算によるリスクあり|1|1|
|[DEP_OPENSSL](#rule-5) 暗号処理に関係するライブラリへの依存が見つかりました。|中|要確認（実装や設定の確認が必要）|2|2|
|[DEP_RUST_OPENSSL](#rule-6) 暗号処理に関係するライブラリへの依存が見つかりました。|中|要確認（実装や設定の確認が必要）|2|2|
|[TS_PY_RSA_USAGE](#rule-7) PythonのRSA関連APIの参照が見つかりました。|中|量子計算によるリスクあり|2|1|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[DEP_PQC_CAPABLE_LIBRARY](#rule-8) 版や設定によってPQCに対応するライブラリが見つかりました。|参考情報|要確認（実装や設定の確認が必要）|2|2|

<a id="rule-0"></a>

## CERT_RSA_2048

証明書の公開鍵に2048ビットRSAが使われています。

確認すること: 認証局と検証側が対応するPQC署名への移行を計画してください。従来型の鍵を大きくするだけでは量子脆弱性は解消しません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[public.der:1:1](#finding-0)|RSA 2048-bit|認証局と検証側が対応するPQC署名への移行を計画してください。従来型の鍵を大きくするだけでは量子脆弱性は解消しません。 / 周辺コードなし|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [証明書の発行と信頼設定の更新を計画する](#action-1)

<a id="rule-1"></a>

## CERT_RSA_SIGNATURE

証明書のRSA系署名方式が見つかりました。

確認すること: 認証局・検証側・鍵形式の対応を確認し、標準化されたPQC署名への移行候補を評価してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-1"></a>[public.der:1:1](#finding-1)|sha256WithRSAEncryption|認証局・検証側・鍵形式の対応を確認し、標準化されたPQC署名への移行候補を評価してください。 / 周辺コードなし|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [証明書の発行と信頼設定の更新を計画する](#action-1)

<a id="rule-2"></a>

## PRIVATE_KEY_RSA_HEADER

RSA秘密鍵のヘッダーが見つかりました。

確認すること: ヘッダーの参照、テスト用の鍵、運用中の鍵を区別し、運用中の鍵が露出していれば失効・再発行してください。鍵本体はレポートに出力していません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-2"></a>[marker.txt:1:1](#finding-2)|\[masked-private-key\]|秘密鍵の参照を確認し、露出した運用中の鍵を更新する / [秘密鍵の参照を確認し、露出した運用中の鍵を更新する](#action-2) / [周辺コード](#source-2)|

<a id="source-2"></a>

### marker.txt:1:1

    > 1 [masked-private-key-material]

<a id="rule-3"></a>

## TS_RSA_KEY_SIZE_LITERAL

RSAの鍵長に関する数値が見つかりました。

確認すること: この数値が実際のRSA鍵生成に使われているか、周辺のコードを確認してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-3"></a>[app.py:2:3](#finding-3)|RSA.generate(2048|この数値が実際のRSA鍵生成に使われているか、周辺のコードを確認してください。 / [周辺コード](#source-3)|
|<a id="finding-4"></a>[app.py:3:3](#finding-4)|RSA.generate(2048|この数値が実際のRSA鍵生成に使われているか、周辺のコードを確認してください。 / [周辺コード](#source-4)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [証明書の発行と信頼設定の更新を計画する](#action-1)
- [実行環境と通信相手のPQC対応を確認する](#action-3)

<a id="source-3"></a>

### app.py:2:3

      1 from Crypto.PublicKey import RSA
    > 2 a=RSA.generate(2048)
      3 b=RSA.generate(2048)

<a id="source-4"></a>

### app.py:3:3

      1 from Crypto.PublicKey import RSA
      2 a=RSA.generate(2048)
    > 3 b=RSA.generate(2048)

<a id="rule-4"></a>

## API_PY_RSA_MODULE

PythonのRSA関連APIの参照が見つかりました。

確認すること: 暗号ライブラリの版と実装を確認し、署名・鍵共有それぞれのPQC移行候補を調べてください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-5"></a>[app.py:1:1](#finding-5)|from Cry***rt RSA|暗号ライブラリの版と実装を確認し、署名・鍵共有それぞれのPQC移行候補を調べてください。 / [周辺コード](#source-5)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [暗号方式を切り替えられる共通処理を設ける](#action-4)
- [実行環境と通信相手のPQC対応を確認する](#action-3)

<a id="source-5"></a>

### app.py:1:1

    > 1 from Crypto.PublicKey import RSA
      2 a=RSA.generate(2048)
      3 b=RSA.generate(2048)

<a id="rule-5"></a>

## DEP_OPENSSL

暗号処理に関係するライブラリへの依存が見つかりました。

確認すること: 表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-6"></a>[Cargo.lock:3:1](#finding-6)|openssl@0.10.75|表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。 / [周辺コード](#source-6)|
|<a id="finding-7"></a>[Cargo.toml:2:1](#finding-7)|openssl@0.10|表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。 / [周辺コード](#source-7)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-5)

<a id="source-6"></a>

### Cargo.lock:3:1

    > 3 [masked-sensitive-file-content]

<a id="source-7"></a>

### Cargo.toml:2:1

    > 2 [masked-sensitive-file-content]

<a id="rule-6"></a>

## DEP_RUST_OPENSSL

暗号処理に関係するライブラリへの依存が見つかりました。

確認すること: 表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-8"></a>[Cargo.lock:3:1](#finding-8)|openssl@0.10.75|表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。 / [周辺コード](#source-8)|
|<a id="finding-9"></a>[Cargo.toml:2:1](#finding-9)|openssl@0.10|表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。 / [周辺コード](#source-9)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-5)

<a id="source-8"></a>

### Cargo.lock:3:1

    > 3 [masked-sensitive-file-content]

<a id="source-9"></a>

### Cargo.toml:2:1

    > 2 [masked-sensitive-file-content]

<a id="rule-7"></a>

## TS_PY_RSA_USAGE

PythonのRSA関連APIの参照が見つかりました。

確認すること: 暗号ライブラリの版と実装を確認し、署名・鍵共有それぞれのPQC移行候補を調べてください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-10"></a>[app.py:2:3](#finding-10)|RSA.generate|暗号ライブラリの版と実装を確認し、署名・鍵共有それぞれのPQC移行候補を調べてください。 / [周辺コード](#source-10)|
|<a id="finding-11"></a>[app.py:3:3](#finding-11)|RSA.generate|暗号ライブラリの版と実装を確認し、署名・鍵共有それぞれのPQC移行候補を調べてください。 / [周辺コード](#source-11)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [暗号方式を切り替えられる共通処理を設ける](#action-4)
- [実行環境と通信相手のPQC対応を確認する](#action-3)

<a id="source-10"></a>

### app.py:2:3

      1 from Crypto.PublicKey import RSA
    > 2 a=RSA.generate(2048)
      3 b=RSA.generate(2048)

<a id="source-11"></a>

### app.py:3:3

      1 from Crypto.PublicKey import RSA
      2 a=RSA.generate(2048)
    > 3 b=RSA.generate(2048)

<a id="rule-8"></a>

## DEP_PQC_CAPABLE_LIBRARY

版や設定によってPQCに対応するライブラリが見つかりました。

確認すること: 解決済みの版、features、実装を確認してください。rustls 0.23.27のaws-lc-rs構成やcryptography 48以降のOpenSSL 3.5対応などが確認候補です。依存関係だけでは有効化を判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-12"></a>[Cargo.lock:3:1](#finding-12)|openssl@0.10.75|解決済みの版、features、実装を確認してください。rustls 0.23.27のaws-lc-rs構成やcryptography 48以降のOpenSSL 3.5対応などが確認候補です。依存関係だけでは有効化を判断できません。 / [周辺コード](#source-12)|
|<a id="finding-13"></a>[Cargo.toml:2:1](#finding-13)|openssl@0.10|解決済みの版、features、実装を確認してください。rustls 0.23.27のaws-lc-rs構成やcryptography 48以降のOpenSSL 3.5対応などが確認候補です。依存関係だけでは有効化を判断できません。 / [周辺コード](#source-13)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-5)

<a id="source-12"></a>

### Cargo.lock:3:1

    > 3 [masked-sensitive-file-content]

<a id="source-13"></a>

### Cargo.toml:2:1

    > 2 [masked-sensitive-file-content]

## 対応案の詳細

各検出箇所から、その箇所に該当する対応案だけを参照します。同じ内容は一度だけ掲載します。

<a id="action-0"></a>


#### 既存方式とPQCを併用し、段階的に移行する (早めに対応)

検出した方式には、量子計算による将来のリスクがあります。接続相手との互換性を確かめながら移行を進めてください。

1. 利用箇所と担当チームを洗い出してください。
2. 既存のPQC・複合構成を確認し、移行が必要なら新旧方式の併用を試してください。
3. 旧方式を廃止する条件と時期を決めてください。

参考資料:

- https://www.nist.gov/itl/post-quantum-cryptography
- https://csrc.nist.gov/pubs/fips/203/final
- https://csrc.nist.gov/pubs/fips/204/final
<a id="action-1"></a>


#### 証明書の発行と信頼設定の更新を計画する (早めに対応)

FIPS 204/205は署名方式の規格であり、共通のハイブリッド証明書形式を定めていません。

1. CBOMで証明書の署名方式と鍵情報を確認してください。
2. 具体的な構成と符号化形式を選び、プロバイダー、認証局、クライアント、信頼ストアの対応を確認してから発行を計画してください。
3. 対応が確認できた構成だけを本番以外で試してください。形式や利用環境が非対応なら未検証のままとし、別の対応済みの移行方法を検討してください。
4. ルート・中間・サーバー証明書の再発行手順を決めてください。

参考資料:

- https://csrc.nist.gov/pubs/fips/204/final
- https://csrc.nist.gov/pubs/fips/205/final
<a id="action-2"></a>


#### 秘密鍵の参照を確認し、露出した運用中の鍵を更新する (至急)

ヘッダーだけでは漏えいの発生を断定できません。鍵本体の存在と利用状況を確認してください。

1. ヘッダーの参照、テスト用の鍵、運用中の鍵を区別してください。
2. 運用中の鍵が露出していれば失効・再発行し、影響する認証情報やセッションを更新してください。
3. 鍵を専用の保管先へ移し、リポジトリへの再混入を防いでください。

参考資料:

- https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html
<a id="action-3"></a>


#### 実行環境と通信相手のPQC対応を確認する (計画して対応)

コードに方式名があるだけでは、その環境で動くか、接続相手が対応しているかは分かりません。

1. cryptography 47で追加されたAPIと、48で追加されたOpenSSL 3.5以降への対応を確認してください。実際の暗号処理を担うライブラリにも依存します。
2. 実際に配備している版、プロバイダー、機能設定を確認してください。
3. 本番設定を変える前に、接続相手との互換性を試してください。

参考資料:

- https://cryptography.io/en/50.0.2/changelog/
<a id="action-4"></a>


#### 暗号方式を切り替えられる共通処理を設ける (早めに対応)

方式に依存する呼出しを分離すると、古典暗号からPQCへ段階的に移行しやすくなります。

1. 署名・検証・鍵共有を共通の処理にまとめてください。
2. 設定で古典・ハイブリッド・PQCの各実装を選べる構成を検討してください。
3. 実装の変更を結合テストで確認してください。

参考資料:

- https://www.nist.gov/itl/post-quantum-cryptography
<a id="action-5"></a>


#### 依存ライブラリのPQC対応を確認する (計画して対応)

PQCへの移行方法は、利用しているライブラリの版や設定にも左右されます。

1. dependency-sbom.jsonで現在の版を確認し、更新候補を決めてください。
2. リリース情報でPQC機能とAPIの変更を確認してください。
3. 本番導入の前にPQC・ハイブリッド方式の結合テストを行ってください。

参考資料:

- https://cyclonedx.org/
- https://spdx.dev/

## 棚卸しデータ

- [CBOM](cbom.json): 14
- [Dependency SBOM](dependency-sbom.json): 2


## 検査範囲と完了状況

完了は、ignore設定・除外・サイズ制限を適用した範囲内で、対応する検査を終えたという意味です。実際の稼働、証明書の信頼性、安全性を保証しません。列挙されなかったファイルは不明です。

|項目|内容|
|---|---|
|検査状態|宣言した範囲内で検査完了 (complete_within_scope)|
|選択したファイル|5|
|読み込み成功|5|
|読み込み失敗|0|
|解析完了|5|
|部分成功|0|
|解析失敗|0|
|未対応|0|
|検出器の対象外|0|
|不明|0|
|列挙できた除外|0|
|列挙失敗|0|
|列挙されなかった範囲|不明|

## 検査の診断情報

理由ごとの件数と最大3件のファイル例を示します。全件の記録は診断JSONに残ります。

|項目|内容|
|---|---|

## 観測の確認根拠と利用状態

確認根拠は情報を集めた方法、利用状態は観測した内容を示します。静的な参照や証明書の解析だけでは、稼働や信頼性を確認したことにはなりません。

|項目|内容|
|---|---|
|observation-v1-d7907a7119ec67e0b0425d5aeef28fb36e283ef0e03f15e7dff5b82622eb6127|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-a24311802377cc2b50e8020c7fd36a97ec01905e0665fc72dfc00c2cd2b12f27|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-ce7e2797333b871d642054c324e6669181533da23826c527fa207bcd05b4752a|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-c995635b7ba47578637512e865f120a3bc2f68b7bee220bbd1b0cd78dfa84f27|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-8f39aad7a172c6b156cd730b718b55df2178500aa131b6a2b87ec1a6607b1de0|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-9a231d7ca3260cf55c9ef5935e03ede1d29f61a819fe4e8d810a3358024e480b|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-17f6b49f11039889447ef6cf642c774fd44235984047348d8ea380034a071472|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-121cfea96f37359bb34b5c5be3415f33dd597ba9b32edfff19be934f12d0cf18|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-0f2603fce2109d96357574273cd7a88ee032757cadedd689ea7f111c42db6b87|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-41f458c76897838267f02129afdbee85cbeb8f38a5177f9f0b30ea8fa75f4f55|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-7078db0d2aa22bcdb473f9b2fbacef7b2a44ba979bc88535c1e0dc94c1b2b4d8|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-290ab33a0eb57b39502dde0347d7e23b93bfb72db1d7962a3fda4a6dd39cf15d|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-1484e959d4bfe38904069ae68d3264ca9605d20386b2efa835d0bc09606e3c88|確認根拠: 文字列の一致 (lexical); 利用状態: 参照あり (referenced)|
|observation-v1-1d7acd2589b5a3a350d4229dc4f7e710024becf91cabd1052bee9063717bd3f8|確認根拠: 文字列の一致 (lexical); 利用状態: 不明 (unknown)|
|observation-v1-9330568e77aee8340f0a59fc7d1f375a8afd30b24b84e49033254a5a3d79f1b7|確認根拠: 公開情報を解析済み (parsed); 利用状態: 不明 (unknown)|

## ローカルの参考評価

収集した事実と、利用者が申告した未検証の条件に基づくローカル方針の評価です。稼働確認や移行完了を保証しません。条件を満たす判定も、その評価方針の範囲に限ります。

|項目|内容|
|---|---|
|評価方針|migration-review-v1|
|全体の判定|ローカル方針で要確認 (fail)|
|理由コード|inventory_not_exhaustive, applicable_asset_violation|
|asset-v1-dae48197d26ca7335bd9cfb1cb9f7777aaeedd585645f6cf68af6b5450eff8d8|ローカル方針で要確認 (fail); observed_primitive_quantum_vulnerable|
|asset-v1-21bb03bffb268e7b5726fdb5ff7bfc95036a64a99256d51d55fc7af879da6cb5|ローカル方針で要確認 (fail); observed_primitive_quantum_vulnerable|
|asset-v1-7bbf1f29e19625b0e55aab993b0f879f9b6e4412b02839f7727cae2aadf29cd3|ローカル方針で要確認 (fail); observed_primitive_quantum_vulnerable|
|asset-v1-8f840a694ce3a2aabc09a6c926ba150227da1967ad9ea770b8d1da6e8d9e7fa1|ローカル方針で要確認 (fail); observed_primitive_quantum_vulnerable|
|asset-v1-c107b6bb88260f51b5fec82879d5da9be71caec5d9d2e9a68cd7519c6566478b|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-dc3ec79c66d3d45929703676e81e944c1bcd6c569260c6d1c2a285c4fccb670e|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-70040dd83548ebe9db3bc9797c050f9e331241f6289c246b0d2a0e3c0c3e80a0|ローカル方針で要確認 (fail); observed_primitive_quantum_vulnerable|
|asset-v1-222f95df2eda5cd7e961041e14962a3da065aec445e5c7fa345f1414cb5cf991|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-2fa9b19ac26dbbe2be3d9f32a3b565671c0b3e76e36915e243de7e353aeb9139|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-36078846a597a7122ccc96c1a60441ae58d885416447099aa6b764f5a5e5a57e|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-1c535d35d99a0dece1474f4262ecc5db18f3a107995760b4d1af82b496807333|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-b7724157d8d35b191705db319379e7cd94448f242341541771013fe1dcf1f8e5|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-c7b120b4e0196a11bc5211143dc260b587d39d0efc02a86d1f1588ea4e05b0fd|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-3aec3730c162169b5c20bb6abf01abc7b1cee005f4580e7671cdd9612aa492e8|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-516b4df7ddf7bfccaf2493f00e08f9761e141a44c51ad6a72062f10344ad0ee1|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-3cebd944749f559480f6688ffc1e60e0c4019c1eaf04cf5bfc2ec58adfe16c17|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-29bef60a61c288d6a4944ec341d20a20f50ed1781926d91f8d05a238695872fc|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-d3eb48213bf72af36df0d89ddcf3cf5d243d8ed2c1e9bfec830a3c87a7754c05|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-07ae183c86ce79c0ef179c061d46c42ecc624806d8efbea7463e194a191b68dd|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-64bdb0f44a5ff674fb62b3aea9372ae7451a93ba6b3c1a167900db31b85bf6b5|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-f2152403a630c9234c9261e4e962e51d73195d02807cd4e58c725c8699952147|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-3cd221be6cf0a113cc951ff750b100ec7c3ad83d153e0e3c253b14ae033de438|評価対象外 (not_applicable); not_a_primitive_policy_target|
