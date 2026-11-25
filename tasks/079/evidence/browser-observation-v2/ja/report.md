# 耐量子暗号への移行調査レポート

作成日時: 2026-11-24 14:45:59 +09:00 (UTC+09:00)

確認が必要な指摘（件）: 2 / 参考情報（件）: 6 / 検査対象ファイル数: 1

|重大度|件数（全検出）|
|---|---:|
|参考情報|6|
|中|2|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[CSHARP_CRYPTO_API](#rule-0) C#の暗号APIまたはライブラリの参照が見つかりました。|中|要確認（実装や設定の確認が必要）|2|1|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[CSHARP_CRYPTO_REFERENCE](#rule-1) C#のTLSライブラリまたはOQSラッパーの参照が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|3|1|
|[CSHARP_PQC_API](#rule-2) C#のPQC方式または複合方式・HPKEの参照が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|3|1|

<a id="rule-0"></a>

## CSHARP_CRYPTO_API

C#の暗号APIまたはライブラリの参照が見つかりました。

確認すること: 型と用途、鍵を生成する側と利用する側の対応、OSとプロバイダーを確認してください。ソースの参照だけでは実行時の有効性を判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[api.cs:1:395](#finding-0)|\[masked-sensitive-content\]|型と用途、鍵を生成する側と利用する側の対応、OSとプロバイダーを確認してください。ソースの参照だけでは実行時の有効性を判断できません。 / [周辺コード](#source-0)|
|<a id="finding-1"></a>[api.cs:1:437](#finding-1)|\[masked-sensitive-content\]|型と用途、鍵を生成する側と利用する側の対応、OSとプロバイダーを確認してください。ソースの参照だけでは実行時の有効性を判断できません。 / [周辺コード](#source-1)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [暗号方式を切り替えられる共通処理を設ける](#action-0)

### api.cs:1:395 — 一致したルール

- `CSHARP_CRYPTO_API` (中): C#の暗号APIまたはライブラリの参照が見つかりました。 — 型と用途、鍵を生成する側と利用する側の対応、OSとプロバイダーを確認してください。ソースの参照だけでは実行時の有効性を判断できません。

<a id="source-0"></a>

### api.cs:1:395

    > 1 [masked-sensitive-file-content]

### api.cs:1:437 — 一致したルール

- `CSHARP_CRYPTO_API` (中): C#の暗号APIまたはライブラリの参照が見つかりました。 — 型と用途、鍵を生成する側と利用する側の対応、OSとプロバイダーを確認してください。ソースの参照だけでは実行時の有効性を判断できません。

<a id="source-1"></a>

### api.cs:1:437

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## CSHARP_CRYPTO_REFERENCE

C#のTLSライブラリまたはOQSラッパーの参照が見つかりました。

確認すること: Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-2"></a>[api.cs:1:7](#finding-2)|\[masked-sensitive-content\]|Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。 / [周辺コード](#source-2)|
|<a id="finding-3"></a>[api.cs:1:24](#finding-3)|\[masked-sensitive-content\]|Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。 / [周辺コード](#source-3)|
|<a id="finding-4"></a>[api.cs:1:65](#finding-4)|\[masked-sensitive-content\]|Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。 / [周辺コード](#source-4)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [暗号方式を切り替えられる共通処理を設ける](#action-0)

### api.cs:1:7 — 一致したルール

- `CSHARP_CRYPTO_REFERENCE` (参考情報): C#のTLSライブラリまたはOQSラッパーの参照が見つかりました。 — Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。

<a id="source-2"></a>

### api.cs:1:7

    > 1 [masked-sensitive-file-content]

### api.cs:1:24 — 一致したルール

- `CSHARP_CRYPTO_REFERENCE` (参考情報): C#のTLSライブラリまたはOQSラッパーの参照が見つかりました。 — Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。

<a id="source-3"></a>

### api.cs:1:24

    > 1 [masked-sensitive-file-content]

### api.cs:1:65 — 一致したルール

- `CSHARP_CRYPTO_REFERENCE` (参考情報): C#のTLSライブラリまたはOQSラッパーの参照が見つかりました。 — Bouncy CastleのTLS、廃止済みの公式dotnetOQS、独立したOpenForgeを区別してください。通信設定やnativeライブラリ、方式の有効性は別途確認してください。

<a id="source-4"></a>

### api.cs:1:65

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## CSHARP_PQC_API

C#のPQC方式または複合方式・HPKEの参照が見つかりました。

確認すること: 正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-5"></a>[api.cs:1:110](#finding-5)|\[masked-sensitive-content\]|正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。 / [周辺コード](#source-5)|
|<a id="finding-6"></a>[api.cs:1:314](#finding-6)|\[masked-sensitive-content\]|正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。 / [周辺コード](#source-6)|
|<a id="finding-7"></a>[api.cs:1:334](#finding-7)|\[masked-sensitive-content\]|正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。 / [周辺コード](#source-7)|

### api.cs:1:110 — 一致したルール

- `CSHARP_PQC_API` (参考情報): C#のPQC方式または複合方式・HPKEの参照が見つかりました。 — 正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。

<a id="source-5"></a>

### api.cs:1:110

    > 1 [masked-sensitive-file-content]

### api.cs:1:314 — 一致したルール

- `CSHARP_PQC_API` (参考情報): C#のPQC方式または複合方式・HPKEの参照が見つかりました。 — 正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。

<a id="source-6"></a>

### api.cs:1:314

    > 1 [masked-sensitive-file-content]

### api.cs:1:334 — 一致したルール

- `CSHARP_PQC_API` (参考情報): C#のPQC方式または複合方式・HPKEの参照が見つかりました。 — 正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。

<a id="source-7"></a>

### api.cs:1:334

    > 1 [masked-sensitive-file-content]

## 対応案の詳細

各検出箇所から、その箇所に該当する対応案だけを参照します。同じ内容は一度だけ掲載します。

<a id="action-0"></a>


#### 暗号方式を切り替えられる共通処理を設ける (早めに対応)

方式に依存する呼出しを分離すると、古典暗号からPQCへ段階的に移行しやすくなります。

1. 署名・検証・鍵共有を共通の処理にまとめてください。
2. 設定で古典・ハイブリッド・PQCの各実装を選べる構成を検討してください。
3. 実装の変更を結合テストで確認してください。

参考資料:

- https://www.nist.gov/itl/post-quantum-cryptography

## 棚卸しデータ

- [CBOM](cbom.json): 6
- [Dependency SBOM](dependency-sbom.json): 0


## 検査範囲と完了状況

完了は、ignore設定・除外・サイズ制限を適用した範囲内で、対応する検査を終えたという意味です。実際の稼働、証明書の信頼性、安全性を保証しません。列挙されなかったファイルは不明です。

|項目|内容|
|---|---|
|検査状態|宣言した範囲内で検査完了 (complete_within_scope)|
|選択したファイル|1|
|読み込み成功|1|
|読み込み失敗|0|
|解析完了ファイル数|1|
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
|observation-v1-49739092dcff5b5f4672654eeb1765291d71a39f75a626d5e5461f0497e11df4|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-8000dd67acb6231b4481d2a6b878626088a71845550810f9f7d96fe58a3c7056|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-746d298599630d84f8725f0ac50a2c0a2e762678b99ac3be586ddfd7ce8a1778|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-295606abd27f8c212be8138beb35e8c9bafa8b4a506d6bb2406abbe0f2a91e43|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-52517791a3158e1072ff781789454df7a74c835f1f5341d3e405ed3732bf4cbf|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-a80e9c829afc40d0e87f0a51c7cf9b7773792978b05dfdf82d860ff3fd2aaea2|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-d1f9ad2f685f6925a531ab842e7db59bb51133d5b15b52d13e6fda99a836d40c|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-cc6b0060151f8d99f719af68eec6f79347705706d15af891ce52b868c701c1b6|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
