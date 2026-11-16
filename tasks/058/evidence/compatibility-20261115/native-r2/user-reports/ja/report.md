# 耐量子暗号への移行調査レポート

作成日時: 2026-11-15 07:21:21.477155 UTC

対応を確認する候補: 3 / 参考情報: 0 / 調査したファイル: 2

コードや設定の手掛かりをまとめた調査結果です。

|重大度|件数（全検出）|
|---|---:|
|緊急|3|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[TLS_RSA_CIPHER_SUITE](#rule-0) RSAに関係するTLS暗号スイートの参照が見つかりました。|緊急|量子計算によるリスクあり|1|1|
|[TLS_RSA_KEY_EXCHANGE](#rule-1) RSAに関係するTLS暗号スイートの参照が見つかりました。|緊急|量子計算によるリスクあり|1|1|
|[TS_TLS_RSA_CIPHER_LITERAL](#rule-2) RSAに関係するTLS暗号スイートの参照が見つかりました。|緊急|量子計算によるリスクあり|1|1|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## TLS_RSA_CIPHER_SUITE

RSAに関係するTLS暗号スイートの参照が見つかりました。

確認すること: RSA_WITHのRSA鍵交換と、ECDHE_RSA・DHE_RSAのRSA認証を区別してください。実際の利用を確認し、ハイブリッド鍵共有とPQC認証を別々に検討してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[app.py:1:17](#finding-0)|TLS_RSA_***BC_SHA|TLSの参照と実際の設定を確認する / [TLSの参照と実際の設定を確認する](#action-1) / [周辺コード](#source-0)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [実行環境と通信相手のPQC対応を確認する](#action-2)

<a id="source-0"></a>

### app.py:1:17

    > 1 cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"

<a id="rule-1"></a>

## TLS_RSA_KEY_EXCHANGE

RSAに関係するTLS暗号スイートの参照が見つかりました。

確認すること: RSA_WITHのRSA鍵交換と、ECDHE_RSA・DHE_RSAのRSA認証を区別してください。実際の利用を確認し、ハイブリッド鍵共有とPQC認証を別々に検討してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-1"></a>[app.py:1:17](#finding-1)|TLS_RSA_***BC_SHA|TLSの参照と実際の設定を確認する / [TLSの参照と実際の設定を確認する](#action-1) / [周辺コード](#source-1)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [実行環境と通信相手のPQC対応を確認する](#action-2)

<a id="source-1"></a>

### app.py:1:17

    > 1 cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"

<a id="rule-2"></a>

## TS_TLS_RSA_CIPHER_LITERAL

RSAに関係するTLS暗号スイートの参照が見つかりました。

確認すること: RSA_WITHのRSA鍵交換と、ECDHE_RSA・DHE_RSAのRSA認証を区別してください。実際の利用を確認し、ハイブリッド鍵共有とPQC認証を別々に検討してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-2"></a>[app.py:1:17](#finding-2)|TLS_RSA_***BC_SHA|TLSの参照と実際の設定を確認する / [TLSの参照と実際の設定を確認する](#action-1) / [周辺コード](#source-2)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [実行環境と通信相手のPQC対応を確認する](#action-2)

<a id="source-2"></a>

### app.py:1:17

    > 1 cipher_suite = "TLS_RSA_WITH_AES_128_CBC_SHA"

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


#### TLSの参照と実際の設定を確認する (計画して対応)

方式名や表の参照だけでは、有効な設定や接続結果を判断できません。ECDHE_RSA・DHE_RSAはRSA認証であり、RSA鍵交換ではありません。

1. 定数・コメント・テスト・有効な設定を区別してください。
2. 鍵共有・認証・TLSバージョンを別々に調べ、既存のハイブリッド対応と通信相手の互換性を確認してください。

参考資料:

- https://www.rfc-editor.org/rfc/rfc8422.html#section-2.2
<a id="action-2"></a>


#### 実行環境と通信相手のPQC対応を確認する (計画して対応)

コードに方式名があるだけでは、その環境で動くか、接続相手が対応しているかは分かりません。

1. cryptography 47で追加されたAPIと、48で追加されたOpenSSL 3.5以降への対応を確認してください。実際の暗号処理を担うライブラリにも依存します。
2. 実際に配備している版、プロバイダー、機能設定を確認してください。
3. 本番設定を変える前に、接続相手との互換性を試してください。

参考資料:

- https://cryptography.io/en/50.0.2/changelog/

## 棚卸しデータ

- [CBOM](cbom.json): 3
- [Dependency SBOM](dependency-sbom.json): 0


## 検査範囲と完了状況

完了は、ignore設定・除外・サイズ制限を適用した範囲内で、対応する検査を終えたという意味です。実際の稼働、証明書の信頼性、安全性を保証しません。列挙されなかったファイルは不明です。

|項目|内容|
|---|---|
|検査状態|一部を検査できませんでした (partial)|
|選択したファイル|2|
|読み込み成功|2|
|読み込み失敗|0|
|解析完了|1|
|部分成功|1|
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
|検査の一部だけが完了しました (detector_partial)|1件; Gemfile|

## 観測の確認根拠と利用状態

確認根拠は情報を集めた方法、利用状態は観測した内容を示します。静的な参照や証明書の解析だけでは、稼働や信頼性を確認したことにはなりません。

|項目|内容|
|---|---|
|observation-v1-304742f360b18e5413dfb0af21d92b9b3b62e9a9267b61f14687d42ce2629306|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-a11f636d71b72d3900115cf5a926bd4f78571b2abe400bce8b1a6ed7e7d7c68a|確認根拠: 文字列の一致 (lexical); 利用状態: 参照あり (referenced)|
|observation-v1-c2cdbf24b73308165a017baf09e29d180886ce3ce6ca2219f270abb4c56fe76e|確認根拠: 文字列の一致 (lexical); 利用状態: 参照あり (referenced)|

## ローカルの参考評価

収集した事実と、利用者が申告した未検証の条件に基づくローカル方針の評価です。稼働確認や移行完了を保証しません。条件を満たす判定も、その評価方針の範囲に限ります。

|項目|内容|
|---|---|
|評価方針|migration-review-v1|
|全体の判定|判定保留 (indeterminate)|
|理由コード|inspection_incomplete_or_unknown, inventory_not_exhaustive, no_complete_positive_policy_evidence|
|asset-v1-f467ff14d5b6e191402e81ef19b8356787c8158d1df3c1397124aee7fe310efa|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-48efc8829f6bc2f221265523ba14c1e52958837d810b3d33f541ed66acb4d17a|評価対象外 (not_applicable); not_a_primitive_policy_target|
|asset-v1-c66d9decaab3e3d21a46e1697dcf30a1da822246a92899697b426c080fb25a13|評価対象外 (not_applicable); not_a_primitive_policy_target|
