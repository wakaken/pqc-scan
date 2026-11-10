# リポジトリのスキャン方法

[English](SCANNING.md) · [README](../../README.ja.md)

出力の選択、CIでの終了判定、検査できなかった箇所の確認を説明します。
棚卸しや評価の見方は[結果の読み方](REPORTS.ja.md)、形式ごとの制約は[対応する入力と検査範囲](COVERAGE.ja.md)を参照してください。

## 目次

- [コマンドと指定方法](#command)
- [診断情報と検査の完全性](#diagnostics)
- [GitHub Actions](#github-actions)
- [ソースから実行する](#from-source)

<a id="command"></a>

## コマンドと指定方法

CLIツールは以下の通り実行します。

```bash
pqc-scan scan <path> [--format json|html|md|sarif|all] [--lang en|ja] [--out-dir ./pqc-report] [--rules-dir ./rules] [--fail-on high|critical] [--threads N] [--diagnostics] [--require-complete] [--inventory-json] [--repository-scope ID] [--policy-profile migration-review-v1] [--assessment-context PATH]
pqc-scan rules list [--rules-dir ./rules]
pqc-scan rules show <rule-id> [--rules-dir ./rules]
```

既定値は `--format all`、`--lang en`、`--out-dir ./pqc-report`、`--rules-dir ./rules`、`--threads 0`（自動選択）です。
所見の重大度や検査の完全性による終了判定は、明示しない限り有効になりません。相対パスは実行時の作業ディレクトリを基準にします。

`--repository-scope` と `--policy-profile` には `--inventory-json` が必要です。
`--assessment-context` は評価方針を指定したときだけ使えます。全オプションは `pqc-scan scan --help` で確認できます。

### プロダクトリポジトリでの推奨実行方法

`pqc-scan` を自分のプロダクトリポジトリ内（`pqc-scan` リポジトリ外）で実行する場合は、`--rules-dir` に **絶対パス** を指定してください。

```bash
pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules
```

### よく使うコマンド

```bash
# すべてのレポート形式を出力

pqc-scan scan . --format all --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# SARIF形式で出力（GitHub Code Scanning向け）

pqc-scan scan . --format sarif --out-dir ./pqc-report --rules-dir /path/to/pqc-scan/rules

# CI失敗ゲート

pqc-scan scan . --fail-on high --rules-dir /path/to/pqc-scan/rules

# 1つのルール詳細を確認

pqc-scan rules show JWT_RS256 --rules-dir /path/to/pqc-scan/rules
```

<a id="diagnostics"></a>

## 診断情報と検査の完全性

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

証明書検出器も、適用可否と解析・属性取得の完全性を報告します。
不正な形式、未知の鍵属性、未対応ブロックの混在は `detector_partial`、
未対応コンテナーは `detector_unsupported` として通知します。
コンテナーの検出は、内部の検査成功を意味しません。解析できてもルールに一致しない証明書は所見を出しません。

依存関係の検査では、有効な空データ・対応形式の不正データ・部分成功・未対応の版・上限到達を区別します。
対応する構造化データの範囲では完了を報告し、関係のないファイルは対象外とします。
Ruby・Gradle・XML・requirements・Goの簡易解析では、引き続き `detector_not_reported` を返します。
完了は対応する解析範囲内の意味で、パッケージ管理ツールと同じ検証やインストール状態の保証ではありません。
`--require-complete` の終了コードの規約は維持しますが、検査状況の修正により実際のコードは変わり得ます。
詳しくは[依存関係の指定と確認根拠](../architecture/DEPENDENCY-PROVENANCE.md)を参照してください。

従来の `scanned_files` は、引き続きwalkerが選択したファイル数です。
新しい集計では、選択・読み込み成功／失敗・解析完了・部分成功・解析失敗・未対応・対象外・不明を区別します。
列挙できたポリシー除外と列挙失敗は別に集計し、ignoreで見えない件数は不明として明示します。
完了の判定は、既存のignore設定・既定の除外・サイズ制限を含む検査範囲内に限ります。

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
