# pqc-scan - 耐量子暗号（PQC）スキャンツール

`pqc-scan` は、耐量子暗号（PQC）への移行準備を支援するRust製CLIです。  
リポジトリ内の暗号利用パターンをスキャンし、量子時代に関連するリスクを検出し、チーム向けに実行可能な移行レポートを生成します。

## このツールの目的

多くの組織ではソフトウェアサプライチェーン可視化のためにSBOMを導入していますが、PQC移行では **コードや設定における暗号利用の可視化** も必要です。  
`pqc-scan` は、トリアージ、計画策定、CIに使える実践的な出力でその可視化を提供します。

## クイックスタート

ご自身で利用されているプロダクトのリポジトリを以下のコマンドでスキャンします。

```bash
pqc-scan scan /path/to/your-product --format all --out-dir /path/to/your-product/pqc-report --rules-dir /path/to/pqc-scan/rules
```

生成されるファイル:

- `/path/to/your-product/pqc-report/report.json`
- `/path/to/your-product/pqc-report/report.html`
- `/path/to/your-product/pqc-report/report.md`
- `/path/to/your-product/pqc-report/report.sarif`
- `/path/to/your-product/pqc-report/cbom.json`
- `/path/to/your-product/pqc-report/dependency-sbom.json`

## インストール

### Option A: `pqc-scan` コマンドとしてインストール（推奨）

```bash
cd /path/to/pqc-scan
cargo install --path crates/pqc_scan_cli
```

その後、プロダクトリポジトリ側で実行します。

```bash
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

インストール後に `pqc-scan: command not found` が表示される場合は、`~/.cargo/bin` が `PATH` に含まれているか確認してください。

### Option B: インストールせずに実行（ソースからの直接実行）

インストールせず試す場合は、ソースから直接実行できます。

```bash
cargo run --manifest-path /path/to/pqc-scan/crates/pqc_scan_cli/Cargo.toml --bin pqc-scan -- scan /path/to/your-product --format all --out-dir /path/to/your-product/pqc-report --rules-dir /path/to/pqc-scan/rules
```

## ツールの主な機能

- `tree-sitter` と `regex` を使ったハイブリッドな検出
- 重大度、信頼度、量子リスクタグのレポート出力
- 検出項目ごとの移行ガイダンス（ランタイム/言語に応じた推奨アクション）
- マッチ行をハイライトしたソーススニペット抽出（HTML + JSON）
- CBOMの出力（`cbom.json`）
- 依存関係インベントリの出力（`dependency-sbom.json`）
- GitHub Code Scanning向けSARIFを含むCI/CD向けの出力

## CBOM と SBOM の違い

### PQC対応状況の棚卸し（2026年11月9日）

7言語のML-KEM・ML-DSA参照、NodeのSLH-DSA鍵生成、PQC対応候補の依存関係、
旧Kyber・Dilithium名、Java・Rubyの古典方式限定TLS設定、Goの
`GODEBUG=tlsmlkem=0` を検出します。静的な手掛かりであり、実際の通信方式の確認ではありません。
証明書のML-DSA・SLH-DSA署名OIDと公開鍵OIDは別々に照合し、ファイル名では判定しません。
署名・信頼チェーンの検証は行いません。既存の出力フィールド名とルールIDを維持し、
追加の情報レベルの検出もCBOM・SARIFに含めます。

`quantum-safe` は参照された暗号方式の分類であり、製品全体の安全性や移行完了の保証ではありません。
依存関係は `quantum-uncertain` とし、解決済みバージョン・provider・features・接続相手を確認します。
Node cryptoとWebCryptoでは必要なマイナーバージョンが異なります。
ブラウザやTypeScriptの実行環境は別途確認し、JWT署名の変更前にJOSE・ライブラリ・検証側の互換性を確認してください。
[静的検証例と公式資料](examples/pqc-readiness/README.md)も参照してください。

| 成果物 | 目的 | 問い |
|---|---|---|
| `dependency-sbom.json` | ソフトウェア依存関係の棚卸し | 「どのパッケージ/ライブラリが含まれているか？」 |
| `cbom.json` | 暗号利用の棚卸し | 「どのアルゴリズム/暗号利用があり、量子リスクは何か？」 |

実務での使い分け:

- SBOMデータでコンポーネント/パッケージの露出を追跡する。
- CBOMデータで暗号移行作業（JWT/TLS/PKI/SSHなど）の優先度を決める。
- SBOMとCBOMを併用し、PQC移行計画を進める。

## 検出カバレッジ

### Detectorモジュール

- `tree_sitter_detector`
- `regex_detector`
- `dependency_detector`
- `certificate_detector`
- `key_detector`

### Tree-sitterの対応言語

- Java
- Go
- JavaScript
- TypeScript
- Python
- Rust
- Ruby

### 依存関係/マニフェストのファイル
以下の依存関係/マニフェストファイルをスキャンします。

- `Cargo.toml`, `Cargo.lock`
- `Gemfile`, `Gemfile.lock`, `gems.rb`, `gems.locked`, `*.gemspec`
- `pom.xml`, `build.gradle`, `build.gradle.kts`, `gradle.lockfile`
- `go.mod`
- `package.json`, `package-lock.json`, `pnpm-lock.yaml`
- `requirements.txt`, `Pipfile.lock`, `poetry.lock`

### SBOMファイル
下記フォーマットのSBOMをスキャンします。

- CycloneDX JSON（`*.cdx.json`, `*bom*.json`）
- SPDX JSON（`*.spdx.json`, `*sbom*.json`）

### 証明書/鍵ファイル
下記フォーマットの証明書や鍵ファイルをスキャンします。

- 証明書ファイル: `.pem`, `.crt`, `.cer`, `.der`, `.p12`, `.pfx`
- 秘密鍵のマーカー:
  - `BEGIN RSA PRIVATE KEY`
  - `BEGIN PRIVATE KEY`
  - `BEGIN OPENSSH PRIVATE KEY`

### 各種ミドルウェア設定
下記ミドルウェアの設定ファイルをスキャンします。

- Kubernetes / Ingress
- nginx
- Apache httpd
- Envoy
- Istio
- HAProxy
- Traefik

### 組み込みルールセット

- デフォルトルールは `rules/default/` 配下にあります。
- 現在の組み込みルール数: **100以上**

## CLI

CLIツールは以下の通り実行します。

```bash
pqc-scan scan <path> [--format json|html|md|sarif|all] [--lang en|ja] [--out-dir ./pqc-report] [--rules-dir ./rules] [--fail-on high|critical] [--threads N] [--diagnostics] [--require-complete]
pqc-scan rules list [--rules-dir ./rules]
pqc-scan rules show <rule-id> [--rules-dir ./rules]
```

### プロダクトリポジトリでの推奨実行方法

`pqc-scan` を自分のプロダクトリポジトリ内（`pqc-scan` リポジトリ外）で実行する場合は、`--rules-dir` に **絶対パス** を指定してください。

```bash
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

### よく使うコマンド

```bash
# すべてのレポート形式を出力
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# SARIFのみ出力（GitHub Code Scanning向け）
pqc-scan scan . --format sarif --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# CI失敗ゲート
pqc-scan scan . --fail-on high --rules-dir /path/to/pqc-scan/rules

# 1つのルール詳細を確認
pqc-scan rules show JWT_RS256 --rules-dir /path/to/pqc-scan/rules
```

### 検査の完全性と診断情報

ファイルの読み込みや検出器の実行に失敗しても、ほかの検出結果は保持します。
検査が部分成功・失敗・不明の場合、CLIは標準エラーへ警告を1回表示します。
既定の終了コードは従来どおり、成功が0、実行エラーが1、`--fail-on` の到達が2です。
検出がゼロでも、検査できたとは限りません。

`--diagnostics` を付けると、`--out-dir` に `diagnostics.json` を追加出力します。
別ファイルとして `schema_version: "1"`、生成時刻、検査範囲、集計値、理由コードを記録します。
検出器・I/Oのエラー本文やファイル内容は含めません。
既存6形式のファイル名と機械出力スキーマは維持し、`--format all` だけでは追加しません。
`--lang` の対象も従来どおりHTMLとMarkdownです。

```bash
pqc-scan scan . --diagnostics --require-complete --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

`--require-complete` は、検査の完了を確認できない場合に、要求されたレポートを出力してから
終了コード**3**を返します。優先順位は、実行エラーの1、完全性を確認できない場合の3、
検出しきい値到達の2です。2つのオプションは単独でも使えます。
有効な検査範囲に対象ファイルがなければ完了扱いになりますが、範囲外に暗号がないという意味ではありません。

正規表現・AST・秘密鍵マーカーの検出器は、適用可否と検査上限を報告します。
採用する検出件数の上限は、1ルール・1ファイル当たり**32件・32件・8件**のままです。
正規表現は既存のコメント除外後に件数を数えるため、コメントだけで検出枠を使い切りません。
言語ごとのコメント判定は変更していません。この修正により、以前は見逃していたコードの検出が増える場合があります。

上限ちょうどの件数だけでは打ち切りと判定せず、有限の処理量で残りの候補を確認します。
追加の有効な検出が見つかった場合や、処理量の上限で探索が終わらなかった場合は
`detector_truncated` とし、それまでの検出を保持して部分成功として扱います。
省略された検出件数は推測しません。正規表現・ASTは1ルール・1ファイル当たり最大4096候補と
残りを確認する1候補を扱います。ASTには、保持する意味ノード4096件、探索に積むノード100000件の上限も設けています。
既存の2000文字での候補切り詰めも通知します。これらは処理量を抑えるための上限であり、性能測定の目標値ではありません。
ASTに構文エラーがあれば取得できた検出を残し、`detector_partial` として報告します。
パーサーの初期化失敗・解析木を取得できない場合は検出器エラーです。
バイナリ入力、未対応のソース言語、適用ルールがない場合は、その検出器の対象外です。
完了判定は既存のパターン・マーカー検査の範囲に限り、名前解決、実際の実行、全鍵形式の網羅を保証しません。

**現時点の制約：** 依存関係・証明書検出器は引き続き互換アダプターを使い、
`detector_not_reported` を返します。対象ファイルがある通常のスキャンは `unknown` のまま、
または上限・構文エラーにより `partial` となるため、厳密モードではいずれも3を返します。
既存レポートのスキーマと診断スキーマの版は維持し、診断値に新しい検査状況を反映します。

従来の `scanned_files` は、引き続きwalkerが選択したファイル数です。
新しい集計では、選択・読み込み成功／失敗・解析完了・部分成功・解析失敗・未対応・対象外・不明を区別します。
列挙できたポリシー除外と列挙失敗は別に集計し、ignoreで見えない件数は不明として明示します。
完了の判定は、既存のignore設定・既定の除外・サイズ制限を含む検査範囲内に限ります。

ライブラリ向けには `scan_repository_detailed` と、アクセサーで参照する `ScanDocument` を追加しました。
既存の `scan_repository`、`ScanResult`、レポートAPI、`Detector::detect` 実装は維持します。
検出器は `detect_detailed` と `DetectorOutcome` で検査結果を報告できます。
`Complete` は検査の完了を表し、安全性の保証ではありません。

## レポート出力

### 日本語レポート

HTMLでは、重大度順の短いルール一覧から確認を始めます。ルール名・説明・ファイル名の検索と、
重大度・リスクの絞り込みに対応し、「対応候補」とPQC参照などの「参考情報」を切り替えられます。
ルールと検出箇所はそれぞれ20件ずつ表示します。
ルール、検出箇所の順に開くと、その箇所のコードと対応を確認できます。
同じ共通手順は繰り返さず参照リンクにまとめ、IDが同じでも内容の異なる対応案は残します。
JavaScriptが無効な場合は全件を表示し、標準の折りたたみで確認できます。
外部スクリプトや通信は不要です。Markdownにもルール・検出箇所への索引を設けています。

TLSの推奨は1.3を優先し、1.2は互換性が必要な場合に限って安全な設定で残す方針です。
PQCの上書き設定にはその設定の確認手順を示し、RSA利用を断定しません。
検出結果と機械出力の構造は維持しますが、推奨文とその適用対象は改善しています。

`--lang ja` を付けると、HTMLとMarkdownを日本語で出力します。
既定は英語で、`--lang en` でも指定できます。

```bash
pqc-scan scan . --lang ja --format all --out-dir ./pqc-report-ja --rules-dir /path/to/pqc-scan/rules
```

見出し、組み込みルールの説明、確認事項、推奨アクションを日本語で記載します。
「参考情報」は問題の指摘ではなく、暗号利用の棚卸しです。
PQCの記述が見つかっても、実際の通信や製品全体の対応完了を保証するものではありません。

JSON・SARIF・CBOM・依存関係SBOMは、言語指定によらず従来の内容を維持します。
検出ID、ファイル名、ソースコード、方式名、参考URLも書き換えません。
独自ルールや対応案に日本語文面がない場合は、その旨を明示して原文を残します。
訳文はプログラムに含まれており、翻訳サービスへコードを送信することはありません。

文章の推敲には[humanizer-jp](https://github.com/tqkqt0/humanizer-jp)を使用しています。
スキルは開発時の編集方針として使い、CLIの実行には必要ありません。

### `report.json`

機械出力したJSON形式のレポートです。検出結果、メタデータ、推奨アクション、ソーススニペットを含みます。

### `report.html`

人間にとってみやすいHTML形式のレポートです。以下を含みます。

- severityバッジ
- findingメタデータ
- ハイライト付きソーススニペット
- 推奨移行アクション

#### HTMLレポートのサンプル

以下のスクリーンショットは、実際の `report.html` 出力例です。  
ルール単位の集約表示、Occurrencesテーブル、ハイライト付きスニペット、Recommended Actions を確認できます。

![HTMLレポート例](docs/html-report-exmaple.jpg)

### `report.md`

監査メモ、Pull Request、成果物保管に適したポータブルなテキスト形式のレポートです。

### `report.sarif`

コードスキャンプラットフォーム（GitHub Code Scanning 含む）向けのSARIF形式のレポートです。

### `cbom.json`

アルゴリズム/位置/リスク/移行方針を含むCBOM(Cryptographic Bill of Materials)です。

### `dependency-sbom.json`

マニフェストとSBOMファイルから抽出した依存関係インベントリです。

## セキュリティ/プライバシー制御

- 完全な秘密鍵情報は出力しません。
- 機微なスニペット出力はマスクします。
- リポジトリ走査時に `.gitignore` をチェックし記載のある成果物はスキャン対象外とします。
- デフォルト除外: `.git`, `node_modules`, `target`, `dist`はデフォルトでスキャン対象外とします。
- 2MB超のファイルはスキャン対象外とします。

## 精度と免責事項

`pqc-scan` は、ルールとヒューリスティクスに基づく **ベストエフォートな静的解析** を行います。  
トリアージと移行計画の高速化を目的としており、**形式的証明ツールではありません**。そのため、**100%の検出精度は保証できません**。

以下を前提に利用してください:

- 誤検知（False Positive）は発生し得ます。
- 見逃し（False Negative）は発生し得ます。
- 本番運用やコンプライアンス判断の前に、各環境での検証が必要です。

本ツールの結果は助言的な情報です。専門家レビュー、アーキテクチャ文脈、実行時/セキュリティ試験と組み合わせて評価してください。

## 既知の制約

- 静的スキャンのみでは、実行時の暗号ネゴシエーション経路を完全には推定できません。
- ライブラリ固有の挙動はバージョンや設定で変わる場合があります。
- 一部のパーサ非依存フォーマットはヒューリスティック検出であり、カスタムルールが必要になる場合があります。

## アーキテクチャ

ワークスペースクレート:

- `pqc_scan_cli`: CLI解析とコマンドディスパッチ
- `pqc_scan_core`: スキャンパイプライン、リスクエンジン、重複排除、推奨アクション付与
- `pqc_scan_rules`: YAMLルール読込とregexコンパイル
- `pqc_scan_detectors`: detector実装
- `pqc_scan_report`: JSON/HTML/Markdown/SARIF/CBOMライター

詳細は [docs/architecture.md](docs/architecture.md) を参照してください。

## GitHub Actions

サンプルワークフロー:

- `.github/workflows/pqc-scan.yml`

このワークフローは、スキャナーのビルド、SARIFスキャン出力、GitHub Code ScanningへのSARIFアップロード、および `dependency-sbom.json` のartifactアップロードを実行します。

## ライセンス

MIT
