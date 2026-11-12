# リポジトリのスキャン方法

[English](SCANNING.md) · [README](../../README.ja.md)

v0.2.0の静的スキャンで使う出力オプション、CIの終了判定、検査できなかった箇所の確認方法を説明します。
棚卸しや評価の見方は[結果の読み方](REPORTS.ja.md)、形式ごとの制約は[対応する入力と検査範囲](COVERAGE.ja.md)を参照してください。

## 目次

- [コマンドと指定方法](#command)
- [GitHub Actions](#github-actions)
- [ソースから実行する](#from-source)
- [診断情報と検査の完全性](#diagnostics)

<a id="command"></a>

## コマンドと指定方法

```bash
pqc-scan scan <path> [--format json|html|md|sarif|all] [--lang en|ja] [--out-dir ./pqc-report] [--rules-dir ./rules] [--fail-on high|critical] [--threads N] [--diagnostics] [--require-complete] [--include-non-scannable] [--cyclonedx-17] [--inventory-json] [--repository-scope ID] [--policy-profile migration-review-v1] [--assessment-context PATH]
pqc-scan rules list [--rules-dir ./rules]
pqc-scan rules show <rule-id> [--rules-dir ./rules]
pqc-scan diff <before-inventory> <after-inventory> --repository-scope ID --out-dir DIR
```

既定値は `--format all`、`--lang en`、`--out-dir ./pqc-report`、`--rules-dir ./rules`、`--threads 0`（自動選択）です。
所見の重大度や検査の完全性による終了判定は、明示しない限り有効になりません。相対パスは実行時の作業ディレクトリを基準にします。

`--repository-scope` と `--policy-profile` には `--inventory-json` が必要です。
`--assessment-context` は評価方針を指定したときだけ使えます。全オプションは `pqc-scan scan --help` で確認できます。

別ファイルのCycloneDX 1.7棚卸しには `--cyclonedx-17` を付けます。
用途と制約は[形式の説明](REPORTS.ja.md#cyclonedx)と[棚卸しの比較](REPORTS.ja.md#diff)を参照してください。

### プロダクトリポジトリでの推奨実行方法

`pqc-scan` を自分のプロダクトリポジトリ内（`pqc-scan` リポジトリ外）で実行する場合は、`--rules-dir` に **絶対パス** を指定してください。

```bash
pqc-scan scan . --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

### よく使うコマンド

```bash
# すべてのレポート形式を出力

pqc-scan scan . --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules

# SARIF形式で出力（GitHub Code Scanning向け）

pqc-scan scan . --format sarif --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules

# CI失敗ゲート

pqc-scan scan . --fail-on high --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules

# 1つのルール詳細を確認

pqc-scan rules show JWT_RS256 --rules-dir /path/to/pqc-scan/rules
```

<a id="github-actions"></a>

## GitHub Actions

プロダクトのCIでは `--format sarif` で出力し、所見でジョブを失敗させる場合は `--fail-on high|critical` を指定します。
検査の不完全さも失敗とする場合に限り、`--require-complete` を加えます。終了コード3は所見のしきい値より優先されるため、対象範囲や未対応形式も確認してください。

このリポジトリの[ワークフロー](../../.github/workflows/pqc-scan.yml)に、GitHub Code Scanningへの送信と成果物保存の例があります。
元のレポートも保存し、所見を確認できるようにします。
本リポジトリの自己スキャンでは、確認済みのテスト・分類データをソースと結果の一致に基づいて扱います。テストディレクトリ全体の除外ではありません。
その送信準備を別のプロジェクトへ転用する前に、[自己スキャンの方針](../quality/CI-SELF-SCAN.md)を確認してください。

<a id="from-source"></a>

## ソースから実行する

インストールせず試す場合は、ソースから直接実行できます。

```bash
cargo run --manifest-path /path/to/pqc-scan/crates/pqc_scan_cli/Cargo.toml --bin pqc-scan -- scan /path/to/your-product --format all --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

<a id="diagnostics"></a>

## 診断情報と検査の完全性

検出がゼロでも、すべての対象を検査できたとは限りません。読み込みや解析に失敗した場合も、ほかの検出結果は保持します。
検査が部分成功・失敗・不明の場合は、標準エラーに警告を表示します。これとは別に、ignoreなどで列挙していないファイルがあり得るという注記も表示します。注記だけで検査失敗にはなりません。
CLIが生成するHTML・Markdownにも検査状態と件数を表示します。
`--diagnostics` を付けると安全な理由情報も確認できます。言語指定で変わるのは人間向けの文面だけです。

### 診断情報を保存し、必要に応じてCIを止める

検査できなかった箇所を調べるには `--diagnostics`、不完全な検査をCIで失敗とするには `--require-complete` を使います。
2つのオプションは単独でも指定できます。

| オプション | 動作 |
|---|---|
| `--diagnostics` | `--out-dir` に `diagnostics.json` を追加し、検査範囲・集計・理由コードを保存する |
| `--require-complete` | 検査の完了を確認できなければ、要求されたレポートを出力した後に終了コード3を返す |

両方を指定する例です。

```bash
pqc-scan scan . --diagnostics --require-complete --out-dir /path/to/reports/product-scan --rules-dir /path/to/pqc-scan/rules
```

`diagnostics.json` には `schema_version: "1"` と生成時刻も含みます。ファイル内容や検出器・I/Oのエラー本文は含めません。
`--format all` だけでは追加されず、既存6形式のファイル名と機械出力スキーマは変わりません。`--lang` の対象はHTMLとMarkdownだけです。

終了コードは次のように読み分けます。

| コード | 意味 |
|---|---|
| 0 | 実行成功。検査の完了や安全性を保証するものではない |
| 1 | 読み込みの準備や出力などの実行エラー |
| 2 | `--fail-on` で指定した重大度のしきい値に到達 |
| 3 | `--require-complete` を指定し、検査の完了を確認できなかった |

複数の条件に当てはまる場合は、1、3、2の順に優先します。診断情報を出す指定だけでは、完全性による終了判定は有効になりません。

### 「完了」が意味する範囲

完了判定は、Gitのignore設定・既定の除外・サイズ制限を適用した検査範囲内に限ります。
有効な範囲に対象ファイルがなければ完了扱いになりますが、範囲外に暗号がないという意味ではありません。
また、`Complete` は対応するパターンや形式を検査したという意味で、名前解決、実際の稼働、全鍵形式の網羅を保証しません。

集計を見るときは、ファイルの選択と解析の完了を区別してください。`scanned_files` はwalkerが選択したファイル数です。
診断情報では、選択・読み込み成功／失敗・解析完了・部分成功・解析失敗・未対応・対象外・不明を分けて記録します。
列挙できたポリシー除外と列挙失敗は別に数え、ignoreで見えないファイル数は不明として扱います。

### 検出件数と処理量の上限

1ルール・1ファイル当たりの採用上限は、正規表現とASTが4,096件、秘密鍵マーカーが8件です。正規表現は言語ごとのコメント除外後に数えます。上限ちょうどでは打ち切りとせず、追加の有効な一致があるか、探索を終えられなかった場合に `detector_truncated` を記録します。それまでの所見は残し、省略件数は推測しません。

正規表現とASTの一致候補は、1ルール・1ファイル当たり65,536件までです。ASTには、構文候補262,144件、処理予定ノード2,000,000件、全ルールで調べる候補テキストの累計256 MiBというファイル単位の上限もあります。候補テキスト全体を検査し、表示用プレビューの短縮は検査範囲に影響しません。性能の保証値ではなく、処理量を抑えるための制限です。[対応範囲](COVERAGE.ja.md#source-limits)も参照してください。

### 解析エラー・未対応形式の扱い

ASTに構文エラーがあれば、取得できた検出を残して `detector_partial` とします。パーサーの初期化や解析木の取得に失敗した場合は検出器エラーです。
バイナリ入力、未対応のソース言語、適用ルールがない場合は、その検出器の対象外として扱います。

証明書では、不正な形式、未知の鍵属性、未対応ブロックの混在を `detector_partial`、未対応コンテナーを `detector_unsupported` として報告します。
コンテナーを検出しても、内部を検査できたとは限りません。解析できてもルールに一致しない証明書は所見を出しません。

依存関係では、有効な空データ、不正な対応形式、部分成功、未対応の版、上限到達を区別します。
対応する構造化データの範囲では完了を報告し、関係のないファイルは対象外です。Go・Maven POM・requirements・Rubyの定義とロックは、対応する構文を解析できた範囲で完了を報告します。Gemfile・gemspecのリテラルによる依存宣言は、選択済みのローカルgemspec参照を含め、Rubyの構文木で検査します。動的・未対応の記述は未完了とし、Gradleの簡易解析は `detector_not_reported` とします。
完了はパッケージ管理ツールと同じ検証やインストール状態の保証ではありません。

`--require-complete` の終了コードの規約は変わりませんが、解析結果や対応範囲によって、実際に返るコードは変わり得ます。
依存関係ごとの詳細は[指定と確認根拠](../architecture/DEPENDENCY-PROVENANCE.md)を参照してください。

## 通常は除外するバイナリ形式を含める

`scan` は既知の画像・アーカイブ・コンパイル済みデータの拡張子と、テキストとして扱えないバイナリを、ファイルの選択前に除外します。除外件数は検査未完了と分けて記録します。`--include-non-scannable` でこれらも対象に戻せますが、形式の解析機能が増えるわけではなく「適用する検査なし」の件数が増える場合があります。既存のディレクトリ・Markdown・Git ignore・サイズの除外は引き続き適用します。鍵・証明書・信頼ストア等の拡張子は、検査できない部分も把握できるよう既定で残します。全パスは診断JSONに残し、人向け診断は件数と理由ごとに最大3件の例に集約します。
