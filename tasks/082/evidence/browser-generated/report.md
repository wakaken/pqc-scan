# 耐量子暗号への移行調査レポート

作成日時: 2026-11-24 18:23:43 +09:00 (UTC+09:00)

確認が必要な指摘（件）: 4 / 参考情報（件）: 8 / 検査対象ファイル数: 9

|重大度|件数（全検出）|
|---|---:|
|参考情報|8|
|中|4|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[C_FAMILY_CRYPTO_API](#rule-0) C/C++の暗号APIまたは設定の参照が見つかりました。|中|要確認（実装や設定の確認が必要）|1|1|
|[DEP_BOUNCYCASTLE](#rule-1) 暗号処理に関係するライブラリへの依存が見つかりました。|中|要確認（実装や設定の確認が必要）|1|1|
|[DEP_OPENSSL](#rule-2) 暗号処理に関係するライブラリへの依存が見つかりました。|中|要確認（実装や設定の確認が必要）|1|1|
|[PHP_CRYPTO_API](#rule-3) PHPの暗号APIまたはライブラリの参照が見つかりました。|中|要確認（実装や設定の確認が必要）|1|1|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[CSHARP_PQC_API](#rule-4) C#のPQC方式または複合方式・HPKEの参照が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|1|1|
|[C_FAMILY_PQC_API](#rule-5) C/C++のPQC方式の参照が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|2|2|
|[DEP_NATIVE_CRYPTO](#rule-6) C/C++の暗号ライブラリへの依存やビルド設定の参照が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|1|1|
|[DEP_NUGET_CRYPTO](#rule-7) NuGetの暗号パッケージに関する記録が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|1|1|
|[DEP_PHP_CRYPTO](#rule-8) PHPの暗号ライブラリや拡張機能への依存宣言が見つかりました。|参考情報|要確認（実装や設定の確認が必要）|2|1|
|[PHP_PQC_API](#rule-9) PHPのPQC方式またはX-Wingの参照が見つかりました。|参考情報|PQC方式の参照あり（対応完了の保証ではありません）|1|1|

<a id="rule-0"></a>

## C_FAMILY_CRYPTO_API

C/C++の暗号APIまたは設定の参照が見つかりました。

確認すること: 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[unknown.c:1:21](#finding-0)|\[masked-sensitive-content\]|C/C++のライブラリ・プロバイダー・ビルド設定を確認する / [C/C++のライブラリ・プロバイダー・ビルド設定を確認する](#action-0) / [周辺コード](#source-0)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [暗号方式を切り替えられる共通処理を設ける](#action-1)

### unknown.c:1:21 — 一致したルール

- `C_FAMILY_CRYPTO_API` (中): C/C++の暗号APIまたは設定の参照が見つかりました。 — 方式、鍵の役割、リンクするプロバイダーとビルド設定を確認してください。プロバイダーやTLSグループの指定だけでは、実行時の有効性や通信での合意を判断できません。

<a id="source-0"></a>

### unknown.c:1:21

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## DEP_BOUNCYCASTLE

暗号処理に関係するライブラリへの依存が見つかりました。

確認すること: 表にあるライブラリ名とバージョンを確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-1"></a>[packages.lock.json:1:1](#finding-1)|bouncyca***@2.6.2|.NETの対象・パッケージ・OSの対応を確認する / [.NETの対象・パッケージ・OSの対応を確認する](#action-2) / [周辺コード](#source-1)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-3)

### packages.lock.json:1:1 — 一致したルール

- `DEP_BOUNCYCASTLE` (中): 暗号処理に関係するライブラリへの依存が見つかりました。 — 表にあるライブラリ名とバージョンを確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。

<a id="source-1"></a>

### packages.lock.json:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-2"></a>

## DEP_OPENSSL

暗号処理に関係するライブラリへの依存が見つかりました。

確認すること: 表にあるライブラリ名とバージョンを確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-2"></a>[vcpkg.json:1:1](#finding-2)|openssl@unknown|C/C++のライブラリ・プロバイダー・ビルド設定を確認する / [C/C++のライブラリ・プロバイダー・ビルド設定を確認する](#action-0) / [周辺コード](#source-2)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-3)

### vcpkg.json:1:1 — 一致したルール

- `DEP_OPENSSL` (中): 暗号処理に関係するライブラリへの依存が見つかりました。 — 表にあるライブラリ名とバージョンを確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。

<a id="source-2"></a>

### vcpkg.json:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-3"></a>

## PHP_CRYPTO_API

PHPの暗号APIまたはライブラリの参照が見つかりました。

確認すること: 検出された方式と用途を確認してください。汎用の署名処理や外部から読み込む鍵、OQSラッパーの参照だけでは、方式や実行時の有効性を判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-3"></a>[rsa.php:2:1](#finding-3)|\[masked-sensitive-content\]|PHPの拡張機能とネイティブライブラリを確認する / [PHPの拡張機能とネイティブライブラリを確認する](#action-4) / [周辺コード](#source-3)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [暗号方式を切り替えられる共通処理を設ける](#action-1)

### rsa.php:2:1 — 一致したルール

- `PHP_CRYPTO_API` (中): PHPの暗号APIまたはライブラリの参照が見つかりました。 — 検出された方式と用途を確認してください。汎用の署名処理や外部から読み込む鍵、OQSラッパーの参照だけでは、方式や実行時の有効性を判断できません。

<a id="source-3"></a>

### rsa.php:2:1

    > 2 [masked-sensitive-file-content]

<a id="rule-4"></a>

## CSHARP_PQC_API

C#のPQC方式または複合方式・HPKEの参照が見つかりました。

確認すること: 正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-4"></a>[signature.cs:1:37](#finding-4)|\[masked-sensitive-content\]|.NETの対象・パッケージ・OSの対応を確認する / [.NETの対象・パッケージ・OSの対応を確認する](#action-2) / [周辺コード](#source-4)|

### signature.cs:1:37 — 一致したルール

- `CSHARP_PQC_API` (参考情報): C#のPQC方式または複合方式・HPKEの参照が見つかりました。 — 正確な方式とパラメーター、.NETやBouncy Castleの実装とバージョンを確認してください。複合方式や汎用のHPKEを単独のPQC方式と同一視せず、構成と実行環境を確認してください。

<a id="source-4"></a>

### signature.cs:1:37

    > 1 [masked-sensitive-file-content]

<a id="rule-5"></a>

## C_FAMILY_PQC_API

C/C++のPQC方式の参照が見つかりました。

確認すること: 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-5"></a>[signature.c:1:39](#finding-5)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-5)|
|<a id="finding-6"></a>[signature.cpp:1:17](#finding-6)|\[masked-sensitive-content\]|標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。 / [周辺コード](#source-6)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [C/C++のライブラリ・プロバイダー・ビルド設定を確認する](#action-0)

### signature.c:1:39 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-5"></a>

### signature.c:1:39

    > 1 [masked-sensitive-file-content]

### signature.cpp:1:17 — 一致したルール

- `C_FAMILY_PQC_API` (参考情報): C/C++のPQC方式の参照が見つかりました。 — 標準とドラフトの方式名・パラメーター、OQSやOpenSSLのバージョンとビルド設定を確認してください。方式の指定だけで、有効化済み・移行完了とは判断できません。

<a id="source-6"></a>

### signature.cpp:1:17

    > 1 [masked-sensitive-file-content]

<a id="rule-6"></a>

## DEP_NATIVE_CRYPTO

C/C++の暗号ライブラリへの依存やビルド設定の参照が見つかりました。

確認すること: ネイティブライブラリ、ビルド設定、プロバイダーを確認してください。vcpkg・Conan・CMakeの宣言だけでは、導入済み・方式の選択済み・TLSの合意済みとは判断できません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-7"></a>[vcpkg.json:1:1](#finding-7)|openssl@unknown|C/C++のライブラリ・プロバイダー・ビルド設定を確認する / [C/C++のライブラリ・プロバイダー・ビルド設定を確認する](#action-0) / [周辺コード](#source-7)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-3)

### vcpkg.json:1:1 — 一致したルール

- `DEP_NATIVE_CRYPTO` (参考情報): C/C++の暗号ライブラリへの依存やビルド設定の参照が見つかりました。 — ネイティブライブラリ、ビルド設定、プロバイダーを確認してください。vcpkg・Conan・CMakeの宣言だけでは、導入済み・方式の選択済み・TLSの合意済みとは判断できません。

<a id="source-7"></a>

### vcpkg.json:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-7"></a>

## DEP_NUGET_CRYPTO

NuGetの暗号パッケージに関する記録が見つかりました。

確認すること: 正確なパッケージ、対象フレームワーク、実行環境、OSとネイティブバックエンドを確認してください。ロックの記録は、導入済み環境やTLSの有効化を示しません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-8"></a>[packages.lock.json:1:1](#finding-8)|bouncyca***@2.6.2|.NETの対象・パッケージ・OSの対応を確認する / [.NETの対象・パッケージ・OSの対応を確認する](#action-2) / [周辺コード](#source-8)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [依存ライブラリのPQC対応を確認する](#action-3)

### packages.lock.json:1:1 — 一致したルール

- `DEP_NUGET_CRYPTO` (参考情報): NuGetの暗号パッケージに関する記録が見つかりました。 — 正確なパッケージ、対象フレームワーク、実行環境、OSとネイティブバックエンドを確認してください。ロックの記録は、導入済み環境やTLSの有効化を示しません。

<a id="source-8"></a>

### packages.lock.json:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-8"></a>

## DEP_PHP_CRYPTO

PHPの暗号ライブラリや拡張機能への依存宣言が見つかりました。

確認すること: Composerの宣言、実際にロードする拡張機能、ネイティブライブラリを別々に確認してください。platformのエミュレート設定は、配備先の環境を示しません。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-9"></a>[composer.json:1:1](#finding-9)|ext-sodium@unknown|Composerの宣言、実際にロードする拡張機能、ネイティブライブラリを別々に確認してください。platformのエミュレート設定は、配備先の環境を示しません。 / [周辺コード](#source-9)|
|<a id="finding-10"></a>[composer.json:1:1](#finding-10)|phpsecli***ib@3.0|Composerの宣言、実際にロードする拡張機能、ネイティブライブラリを別々に確認してください。platformのエミュレート設定は、配備先の環境を示しません。 / [周辺コード](#source-10)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [PHPの拡張機能とネイティブライブラリを確認する](#action-4)
- [依存ライブラリのPQC対応を確認する](#action-3)

### composer.json:1:1 — 一致したルール

- `DEP_PHP_CRYPTO` (参考情報): PHPの暗号ライブラリや拡張機能への依存宣言が見つかりました。 — Composerの宣言、実際にロードする拡張機能、ネイティブライブラリを別々に確認してください。platformのエミュレート設定は、配備先の環境を示しません。

<a id="source-9"></a>

### composer.json:1:1

    > 1 [masked-sensitive-file-content]

### composer.json:1:1 — 一致したルール

- `DEP_PHP_CRYPTO` (参考情報): PHPの暗号ライブラリや拡張機能への依存宣言が見つかりました。 — Composerの宣言、実際にロードする拡張機能、ネイティブライブラリを別々に確認してください。platformのエミュレート設定は、配備先の環境を示しません。

<a id="source-10"></a>

### composer.json:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-9"></a>

## PHP_PQC_API

PHPのPQC方式またはX-Wingの参照が見つかりました。

確認すること: 方式名とパラメーター、Sodium・ParagonIEの実装とバージョンを確認してください。X-Wingは単独のML-KEMやTLSの複合方式と区別し、実行環境で利用できるかも確認してください。

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-11"></a>[kem.php:2:1](#finding-11)|\[masked-sensitive-content\]|PHPの拡張機能とネイティブライブラリを確認する / [PHPの拡張機能とネイティブライブラリを確認する](#action-4) / [周辺コード](#source-11)|

### kem.php:2:1 — 一致したルール

- `PHP_PQC_API` (参考情報): PHPのPQC方式またはX-Wingの参照が見つかりました。 — 方式名とパラメーター、Sodium・ParagonIEの実装とバージョンを確認してください。X-Wingは単独のML-KEMやTLSの複合方式と区別し、実行環境で利用できるかも確認してください。

<a id="source-11"></a>

### kem.php:2:1

    > 2 [masked-sensitive-file-content]

## 対応案の詳細

各検出箇所から、その箇所に該当する対応案だけを参照します。同じ内容は一度だけ掲載します。

<a id="action-0"></a>


#### C/C++のライブラリ・プロバイダー・ビルド設定を確認する (計画して対応)

参照や依存宣言から配備先の対応状況は分かりません。不明なバージョンは未確認のまま扱ってください。

1. 配備先のネイティブライブラリ・ビルド設定・プロバイダー・通信相手は未確認です。vcpkg・Conanの宣言やbaseline、CMakeの参照は、導入済みライブラリや復元したビルド依存関係ではありません。
2. OpenSSL 3.5は既定プロバイダーにML-KEM・ML-DSA・SLH-DSAを備えます。すべてのPQC利用にoqsproviderが必要なわけではありません。正確なビルド・有効な方式・プロバイダーとプロパティを確認してください。
3. liboqs・liboqs-cpp・oqsproviderは、それぞれの互換バージョン・有効／無効な方式・ビルドフラグ・保守状況を確認してください。OQSの統合はプロトタイプであり、ラッパーやincludeだけでは本番への適性を判断できません。
4. TLSグループやプロバイダーのロードは設定の要求です。通信相手、合意したグループ、認証署名とプロトコルを別々に確認してください。適切なAESを一律に公開鍵KEMへ置き換える案にはなりません。

参考資料:

- https://docs.openssl.org/3.5/man7/EVP_KEM-ML-KEM/
- https://openquantumsafe.org/
- https://github.com/open-quantum-safe/oqs-provider
- https://github.com/open-quantum-safe/liboqs-cpp
<a id="action-1"></a>


#### 暗号方式を切り替えられる共通処理を設ける (早めに対応)

方式に依存する呼出しを分離すると、古典暗号からPQCへ段階的に移行しやすくなります。

1. 署名・検証・鍵共有を共通の処理にまとめてください。
2. 設定で古典・ハイブリッド・PQCの各実装を選べる構成を検討してください。
3. 実装の変更を結合テストで確認してください。

参考資料:

- https://www.nist.gov/itl/post-quantum-cryptography
<a id="action-2"></a>


#### .NETの対象・パッケージ・OSの対応を確認する (計画して対応)

参照や依存宣言から配備先の対応状況は分かりません。不明なバージョンは未確認のまま扱ってください。

1. 実際のランタイム・OS・プロバイダー・バックエンドは未確認です。対象フレームワーク、global.jsonのSDK指定、NuGet宣言は別々の情報であり、導入済み環境を示しません。
2. .NET 10のMLKem・MLDsa・SlhDsaは、パラメーター・OS・バックエンドと配備先のIsSupported／IsAlgorithmSupportedを確認してください。LinuxではOpenSSL 3.5以降が必要な機能があります。Windows・Linux・macOSの対応を同一視しないでください。
3. .NET 11のCompositeMLKemとHpkeは正確なAPIと実験的な診断を確認してください。HPKEだけではPQCのKEMとは分かりません。Bouncy Castleのパッケージとバージョンも別途確認してください。
4. ASP.NET／KestrelのTLSはOS・TLSバックエンドと通信相手に依存します。暗号APIの存在だけではPQC TLSは有効になりません。Data ProtectionのAESを一律にKEMへ置き換える案にはなりません。

参考資料:

- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10/libraries
- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-11/libraries
- https://learn.microsoft.com/en-us/dotnet/standard/security/cross-platform-cryptography
- https://learn.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel/endpoints?view=aspnetcore-10.0
<a id="action-3"></a>


#### 依存ライブラリのPQC対応を確認する (計画して対応)

PQCへの移行方法は、利用しているライブラリのバージョンや設定にも左右されます。

1. dependency-sbom.jsonで現在のバージョンを確認し、更新候補を決めてください。
2. リリース情報でPQC機能とAPIの変更を確認してください。
3. 本番導入の前にPQC・ハイブリッド方式の結合テストを行ってください。

参考資料:

- https://cyclonedx.org/
- https://spdx.dev/
<a id="action-4"></a>


#### PHPの拡張機能とネイティブライブラリを確認する (計画して対応)

参照や依存宣言から配備先の対応状況は分かりません。不明なバージョンは未確認のまま扱ってください。

1. 配備先のPHP・拡張機能・ネイティブライブラリのバージョンは未確認です。Composerのconfig.platformは依存解決用のエミュレート設定です。
2. PHP 8.6のSodium KEM APIはlibsodium 1.0.22以降が必要です。X-Wing、単独のML-KEM-768、TLSのハイブリッドグループを区別し、ロード済み拡張機能と操作を確認してください。
3. ext-pqcryptoとparagonie/pqcrypto_compatは別々に調べ、対応方式・バージョン・保守と監査の状況を確認してください。Composerパッケージはネイティブ拡張機能とは別です。
4. PHP OpenSSLはリンク先・プロバイダーと公開APIを確認してください。OpenSSLのPQC機能だけではPHPのKEMやTLS対応を判断できません。フレームワークのAES暗号化を一律にKEMへ置き換える案にはなりません。

参考資料:

- https://www.php.net/manual/en/migration86.new-functions.php
- https://github.com/paragonie/ext-pqcrypto
- https://github.com/paragonie/pqcrypto_compat
- https://www.php.net/manual/en/ref.openssl.php

## 棚卸しデータ

- [CBOM](cbom.json): 12
- [Dependency SBOM](dependency-sbom.json): 6


## 検査範囲と完了状況

完了は、ignore設定・除外・サイズ制限を適用した範囲内で、対応する検査を終えたという意味です。実際の稼働、証明書の信頼性、安全性を保証しません。列挙されなかったファイルは不明です。

|項目|内容|
|---|---|
|検査状態|宣言した範囲内で検査完了 (complete_within_scope)|
|選択したファイル|9|
|読み込み成功|9|
|読み込み失敗|0|
|解析完了ファイル数|9|
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
|observation-v1-d0d229790eb2d39452e7e7ec3b99cd3e4658280a72a0a3c03aa7ef4f737ba885|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-0a7a033d5e18ce36c4634eaf18b3b8d67e78e1977b64557faf9bb6badfa36efc|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-f989a069fde72ee8f90718bd2079c767a1886c4595d652d622d34aac61a0875e|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-9c9c03b89a70ee1343acfc69a7874b673648d1d3f933c216ca15e0f4ca145086|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-4a8b9487d8001f0e293e7e212434a85435f093aa14a9c079386e82ebed8410b1|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-2b59bbb82a7c448369947adc8f4a420056f9b87e6b0e621e07e709e069bfe883|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-9d0247251eb2248103e86a2429c3dc679efd45e12c46bfbb04197cb818119ce2|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-0649fe80c36ffe7f0c436a6c52b2b88fc61e01a7307f914dc735680fbf08f30b|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-dbff24f17dcd9f87e43b1c95ca3f4fb421fc062e5b915e1692a4a58a1a8a8618|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-1a827946a90e3ec3b5ab612b75a9a6e10861d50e554af17dff224a8e6bbfaf56|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-e03816637deb8a0496abaf6d3059ca30e72a3cadaa3ac1a371919dcdc019257b|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-92a17139ff861978bd91039f1615618bdb83f3a6049f39012a3825a8f5d1b772|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-7238582bd9957f9815b53fd98f88add076a7e9bbfd438ba2e452247bbbb68e81|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-e6f7c75b66f67918cb9a17387d7392688076b2ac340a64005467599b1364c9d1|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-0b70bd7f324d74525117b118620405930f7703afd71e605a778973670e193bfa|確認根拠: 構文パターンの一致 (syntactic); 利用状態: 参照あり (referenced)|
|observation-v1-e81016e1f949a70a93d71e2d4d391295b9ad4af17c3b74037c90620cb3c576a7|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-6f31bfd4f3b496ec4e58229aa78648c8a1a6a243113cb7d5eb0c4fe9a5cc3fea|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
|observation-v1-95977c232d876181e6c057995bb2d190abd3992c1643a19ee8fdb3bfadb208fb|確認根拠: 公開情報を解析済み (parsed); 利用状態: 参照あり (referenced)|
