# 耐量子暗号への移行調査レポート

作成日時: 2026-11-24 16:30:09 +09:00 (UTC+09:00)

確認が必要な指摘（件）: 6 / 参考情報（件）: 12 / 検査対象ファイル数: 3

|重大度|件数（全検出）|
|---|---:|
|参考情報|12|
|中|6|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[API_OPENSSL_RSA_NEW](#rule-0) OpenSSLの低水準RSA APIの参照が見つかりました。|中|量子計算によるリスクあり|1|1|
|[C_FAMILY_CRYPTO_API](#rule-1) C/C++の暗号APIまたは設定の参照が見つかりました。|中|要確認（実装や設定の確認が必要）|5|1|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[C_FAMILY_PQC_API](#rule-2) C/C++のPQC方式の参照が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|9|3|
|[C_FAMILY_TLS_REFERENCE](#rule-3) C/C++の暗号ライブラリをincludeする記述が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|3|2|

<a id="rule-0"></a>

## API_OPENSSL_RSA_NEW

OpenSSLの低水準RSA APIの参照が見つかりました。

確認すること: 暗号方式の変更に備え、EVPなどの高水準APIを使う構成を検討してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[api.c:3:411](#finding-0)|\[masked-sensitive-content\]|暗号方式の変更に備え、EVPなどの高水準APIを使う構成を検討してください。 / [周辺コード](#source-0)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [既存方式とPQCを併用し、段階的に移行する](#action-0)
- [暗号方式を切り替えられる共通処理を設ける](#action-1)

### api.c:3:411 — 一致したルール

- `API_OPENSSL_RSA_NEW` (中): OpenSSLの低水準RSA APIの参照が見つかりました。 — 暗号方式の変更に備え、EVPなどの高水準APIを使う構成を検討してください。

<a id="source-0"></a>

### api.c:3:411

    > 3 [masked-sensitive-file-content]

<a id="rule-1"></a>

## C_FAMILY_CRYPTO_API

C/C++の暗号APIまたは設定の参照が見つかりました。

確認すること: 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-1"></a>[api.c:3:220](#finding-1)|C/C++ provider configuration reference: oqsprovider|方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。 / [周辺コード](#source-1)|
|<a id="finding-2"></a>[api.c:3:259](#finding-2)|C/C++ property configuration reference: provider=default|方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。 / [周辺コード](#source-2)|
|<a id="finding-3"></a>[api.c:3:311](#finding-3)|C/C++ TLS group configuration reference: X25519MLKEM768:P-256|方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。 / [周辺コード](#source-3)|
|<a id="finding-4"></a>[api.c:3:364](#finding-4)|\[masked-sensitive-content\]|方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。 / [周辺コード](#source-4)|
|<a id="finding-5"></a>[api.c:3:411](#finding-5)|\[masked-sensitive-content\]|方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。 / [周辺コード](#source-5)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [暗号方式を切り替えられる共通処理を設ける](#action-1)

### api.c:3:220 — 一致したルール

- `C_FAMILY_CRYPTO_API` (中): C/C++の暗号APIまたは設定の参照が見つかりました。 — 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

<a id="source-1"></a>

### api.c:3:220

    > 3 [masked-sensitive-file-content]

### api.c:3:259 — 一致したルール

- `C_FAMILY_CRYPTO_API` (中): C/C++の暗号APIまたは設定の参照が見つかりました。 — 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

<a id="source-2"></a>

### api.c:3:259

    > 3 [masked-sensitive-file-content]

### api.c:3:311 — 一致したルール

- `C_FAMILY_CRYPTO_API` (中): C/C++の暗号APIまたは設定の参照が見つかりました。 — 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

<a id="source-3"></a>

### api.c:3:311

    > 3 [masked-sensitive-file-content]

### api.c:3:364 — 一致したルール

- `C_FAMILY_CRYPTO_API` (中): C/C++の暗号APIまたは設定の参照が見つかりました。 — 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

<a id="source-4"></a>

### api.c:3:364

    > 3 [masked-sensitive-file-content]

### api.c:3:411 — 一致したルール

- `C_FAMILY_CRYPTO_API` (中): C/C++の暗号APIまたは設定の参照が見つかりました。 — 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

<a id="source-5"></a>

### api.c:3:411

    > 3 [masked-sensitive-file-content]

<a id="rule-2"></a>

## C_FAMILY_PQC_API

C/C++のPQC方式の参照が見つかりました。

確認すること: 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-6"></a>[api.c:3:28](#finding-6)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-6)|
|<a id="finding-7"></a>[api.c:3:98](#finding-7)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-7)|
|<a id="finding-8"></a>[api.c:3:124](#finding-8)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-8)|
|<a id="finding-9"></a>[api.cpp:2:10](#finding-9)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-9)|
|<a id="finding-10"></a>[api.cpp:2:40](#finding-10)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-10)|
|<a id="finding-11"></a>[api.cpp:2:113](#finding-11)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-11)|
|<a id="finding-12"></a>[api.cpp:2:151](#finding-12)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-12)|
|<a id="finding-13"></a>[common.h:1:21](#finding-13)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-13)|
|<a id="finding-14"></a>[common.h:1:49](#finding-14)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-14)|

### api.c:3:28 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-6"></a>

### api.c:3:28

    > 3 [masked-sensitive-file-content]

### api.c:3:98 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-7"></a>

### api.c:3:98

    > 3 [masked-sensitive-file-content]

### api.c:3:124 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-8"></a>

### api.c:3:124

    > 3 [masked-sensitive-file-content]

### api.cpp:2:10 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-9"></a>

### api.cpp:2:10

    > 2 [masked-sensitive-file-content]

### api.cpp:2:40 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-10"></a>

### api.cpp:2:40

    > 2 [masked-sensitive-file-content]

### api.cpp:2:113 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-11"></a>

### api.cpp:2:113

    > 2 [masked-sensitive-file-content]

### api.cpp:2:151 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-12"></a>

### api.cpp:2:151

    > 2 [masked-sensitive-file-content]

### common.h:1:21 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-13"></a>

### common.h:1:21

    > 1 [masked-sensitive-file-content]

### common.h:1:49 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-14"></a>

### common.h:1:49

    > 1 [masked-sensitive-file-content]

<a id="rule-3"></a>

## C_FAMILY_TLS_REFERENCE

C/C++の暗号ライブラリをincludeする記述が見つかりました。

確認すること: OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-15"></a>[api.c:1:1](#finding-15)|C/C++ OpenSSL include reference|OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。 / [周辺コード](#source-15)|
|<a id="finding-16"></a>[api.c:2:1](#finding-16)|C/C++ liboqs include reference|OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。 / [周辺コード](#source-16)|
|<a id="finding-17"></a>[api.cpp:1:1](#finding-17)|C/C++ liboqs-cpp include reference|OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。 / [周辺コード](#source-17)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [暗号方式を切り替えられる共通処理を設ける](#action-1)

### api.c:1:1 — 一致したルール

- `C_FAMILY_TLS_REFERENCE` (参考情報): C/C++の暗号ライブラリをincludeする記述が見つかりました。 — OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。

<a id="source-15"></a>

### api.c:1:1

    > 1 [masked-sensitive-file-content]

### api.c:2:1 — 一致したルール

- `C_FAMILY_TLS_REFERENCE` (参考情報): C/C++の暗号ライブラリをincludeする記述が見つかりました。 — OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。

<a id="source-16"></a>

### api.c:2:1

    > 2 [masked-sensitive-file-content]

### api.cpp:1:1 — 一致したルール

- `C_FAMILY_TLS_REFERENCE` (参考情報): C/C++の暗号ライブラリをincludeする記述が見つかりました。 — OpenSSL・liboqs・liboqs-cppを区別し、リンク先やTLS設定を確認してください。includeだけでは、導入済みの実装や方式の有効性、通信での合意を判断できません。

<a id="source-17"></a>

### api.cpp:1:1

    > 1 [masked-sensitive-file-content]

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


#### 暗号方式を切り替えられる共通処理を設ける (早めに対応)

方式に依存する呼出しを分離すると、古典暗号からPQCへ段階的に移行しやすくなります。

1. 署名・検証・鍵共有を共通の処理にまとめてください。
2. 設定で古典・ハイブリッド・PQCの各実装を選べる構成を検討してください。
3. 実装の変更を結合テストで確認してください。

参考資料:

- https://www.nist.gov/itl/post-quantum-cryptography

## 棚卸しデータ

- [CBOM](cbom.json): 13
- [Dependency SBOM](dependency-sbom.json): 0


## 検査範囲と完了状況

完了は、ignore設定・除外・サイズ制限を適用した範囲内で、対応する検査を終えたという意味です。実際の稼働、証明書の信頼性、安全性を保証しません。列挙されなかったファイルは不明です。

|項目|内容|
|---|---|
|検査状態|宣言した範囲内で検査完了 (complete_within_scope)|
|選択したファイル|3|
|読み込み成功|3|
|読み込み失敗|0|
|解析完了ファイル数|3|
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
|observation-v1-a3a43ecfbffe2ca5ee55d07c513713e7b926f838b4325d69ef25ae9999df5458|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-c245de194c29be2119d06acbbedd5fa63dedc9f12131f984375a3484b23b68e7|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-600d0316a8babda12ddbaaea63525c0152c363538f275b28b917d8ae9b4b92c6|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-7fffa006bb4c5e32635b7a70403163d35ce35a4dcd4128a6ba90749cbbe20fc9|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-c0124115644bf1332dda4f2fd65f4828facb30ceeb0059541478742afcace073|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-0f02d130bfff91ed482548f2bd9c51890bb886cda4c2fbfcc1d572d3c62bc9c2|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-5693c55b42bb1a74cd715415493c52bfe4bcb897f1255b707b27dc98bc07a520|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-7731c1c2f6e9383aaa8fb068f6cf2c6fde3f7ef6053de773caa79610598c8074|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-47dbf90bac191d6194dd49c9713f1a5996e1e3deb62e9daf34258c38632bb4e9|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-1d2ce541cd4bf418d27e9d842b4df41f19d312abbaef99f15af2718c0cd3bd4c|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-397c85e5b8bef6337751c76e755a3ceb7d970095d4cd9720a0c3bbb83efedc77|確認根拠: 文字列の一致 (lexical); 利用状態: 参照あり (referenced)|
|observation-v1-7bd4b3ceae2d7955b25355eda8eb4e72531785aab92bcee475e517002c5f3445|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-229832fdedf8788b27c72dce874e5447b9cad20098020d1f17cba6610e829822|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-b174c9d0d91f473acef7c62e37a3afbd856d56bfdb652997010a8a272e9cc73a|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-bc1e75100ff47dd97632cf22d65245c363700ee82df47e81088e3454f14c56cb|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-7fa634834b1ad9fe8baeeb9b3ad74cae4a3ec7e6e088c6173b533ffcdfee1e3b|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-4969b01c1eda12f8a6256d8ac4a1e4cc93685ff571b167a9aba1dd9785ef6baa|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-b597509b600dacd2a8d6037fefe50d9abbc1a8f2c2f406b76774c1e6d1e413ae|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
