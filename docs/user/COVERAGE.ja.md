# 対応する入力と検査範囲

[English](COVERAGE.md) · [README](../../README.ja.md)

v0.4.0で検査できる入力と、スキャン結果のレポートを読むときの注意点を説明します。\
[入力の概要](../../README.ja.md#targets)と[スキャン手順](SCANNING.ja.md)も参照してください。

## 目次

- [依存関係ファイル](#dependencies)
- [SBOM](#sbom)
- [証明書と秘密鍵マーカー](#certificates)
- [設定ファイルとルール](#configuration)
- [ソースコード](#source-limits)
- [秘密情報と除外範囲](#privacy)

<a id="dependencies"></a>

## 依存関係ファイル

次のファイルから、関連ライブラリとバージョン情報を調べます。

- Rust: `Cargo.toml`、`Cargo.lock`
- Ruby: `Gemfile`、`Gemfile.lock`、`gems.rb`、`gems.locked`、`*.gemspec`
- Java: `pom.xml`、`build.gradle`、`build.gradle.kts`、`gradle.lockfile`
- Go: `go.mod`
- JavaScript／TypeScript: `package.json`、`package-lock.json`、`pnpm-lock.yaml`
- Python: `requirements.txt`、`Pipfile.lock`、`poetry.lock`

宣言されたバージョンと、ロックファイルに記録されたバージョンは区別します。\
動的に決まる値を確定した値として扱いません。\
ライブラリが見つかっても、その暗号方式が実際に使われているとは限りません。

<a id="sbom"></a>

## SBOMの読み込み

CycloneDX JSON（1.4〜1.7、`*.cdx.json`／`*bom*.json`）と
SPDX JSON（2.2〜2.3、`*.spdx.json`／`*sbom*.json`）に対応します。\
記録されたコンポーネント・パッケージを調べるもので、インストール状態の検査ではありません。

<a id="certificates"></a>

## 証明書と秘密鍵マーカー

`.pem`、`.crt`、`.cer`、`.der` の公開証明書から、署名方式、公開鍵の方式、
対応する鍵長・曲線、有効期間を確認します。\
証明書の署名と公開鍵は別々に表示します。\
期限の判定はスキャン時点の日時に基づきます。\
証明書の信頼性や接続先での利用を保証するものではありません。

`.p12`／`.pfx` は、読み取れる公開証明書を検査します。\
パスワード入力や復号には対応しません。\
内容を検査できなかった場合は、未完了の表示を確認してください。

RSA・PKCS#8・OpenSSHの秘密鍵ヘッダーの存在も検出します。\
秘密鍵を復号したり、正しい鍵か検証したりする機能ではありません。

<a id="configuration"></a>

## 設定ファイルとルール

Kubernetes／Ingress、nginx、Apache httpd、Envoy、Istio、HAProxy、Traefikの
TLS・暗号スイート・鍵交換グループなどの設定を調べます。\
ファイル上の設定であり、稼働中の接続で有効な方式を確認した結果ではありません。

組み込みルールは125件です。\
同梱の `rules/default` を指定して使ってください。

<a id="source-limits"></a>
<a id="source-facts"></a>

## ソースコードの検査範囲

Java、Go、JavaScript、TypeScript／TSX、Python、Rust、Rubyの暗号APIや方式名の参照を調べます。コードは実行しません。\
対応する利用パターンに範囲があるため、すべての暗号処理を発見するものではありません。\
暗号アルゴリズムなどの名前がコード中に見つかっても、その処理が実際に使われていることや、PQCへの移行が完了していることまでは確認できません。

構文エラー、未対応の記述、処理上限などで検査が終わらなかった場合も、取得できた検出結果は残ります。\
[検査状況と未完了の確認](SCANNING.ja.md#diagnostics)を参照し、表示された理由と次の確認方法に沿って調べてください。

<a id="privacy"></a>

## 秘密情報の扱いと検査の除外範囲

### レポートを共有する前に確認すること

このツールは認識できる秘密鍵や認証情報はレポートでマスクしますが、すべての秘密情報を検出するものではありません。\
レポートにはソースの該当箇所を含む断片、ファイルのパス、証明書の公開情報などが残るため、組織や第三者などにレポートを共有する前に実際に内容を確認してください。

### 検査対象から外れる条件

| 条件 | 既定の扱い・変更方法 |
|---|---|
| Gitの除外設定 | 設定に従って対象から除外します |
| ディレクトリ・形式 | `.git`、`node_modules`、`target`、`dist`、`docs`、Markdown、既知の画像・アーカイブ・バイナリなどを除外します |
| ファイルサイズ | 既定は2 MiBまでです。CLIとフォルダ選択画面で1〜64 MiBに変更できます |
| テストコード | 既定で含みます。`--exclude-tests` または画面の「テストコードを除外」で除外できます |
| 独自のパス | `.pqcignore` に指定できます |
| シンボリックリンク | リンク先のファイルは読み取りません |

設定方法は[サイズ上限](SCANNING.ja.md#ファイルサイズの上限)と[対象の選び方](SCANNING.ja.md)を参照してください。\
除外された範囲や未対応の入力もあるため、検出0件となった場合でも安全性を証明するものではないことに注意してください。
