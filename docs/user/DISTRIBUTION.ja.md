# インストールと配布物

[English](DISTRIBUTION.md) · [README](https://github.com/wakaken/pqc-scan/blob/codex/v04-roadmap-20261118/README.ja.md)

このガイドは、macOS Apple Silicon（`aarch64-apple-darwin`）向けの
**v0.4.0**の導入方法を説明します。変更点は[v0.4.0のリリースノート](../releases/v0.4.0.md)を参照してください。

<a id="release-status"></a>

## 配布物を選ぶ

[GitHub Releases](https://github.com/wakaken/pqc-scan/releases)で、利用するバージョンと構成を選びます。
公開物の有無はReleaseページで確認してください。Actions成果物は公開済みReleaseとは別です。

| 構成 | ファイル名 | 使える機能 |
|---|---|---|
| `static` | `pqc-scan-0.4.0-aarch64-apple-darwin-static.tar.gz` | ローカルスキャン、ローカルWeb画面、移行確認 |
| `network`（実験的） | `pqc-scan-0.4.0-aarch64-apple-darwin-network.tar.gz` | 静的版の全機能に加え、明示したTLS・SSH接続先の診断 |

選んだアーカイブに対応する `.sha256` ファイルも取得してください。
接続先の診断には管理者の許可が必要です。[TLS・SSH診断ガイド](PROBING.ja.md)に沿って実行します。
他のOS・CPU向けバイナリとcrates.ioからの導入は対象外です。

## アーカイブから導入する

1. アーカイブと `.sha256` を同じディレクトリに保存します。
2. そのディレクトリで `shasum -a 256 -c FILE.sha256` を実行します。`FILE.sha256` はダウンロードしたファイル名に置き換え、照合が成功したことを確認します。
3. 新しい空のディレクトリを用意し、`tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY` で展開します。ファイル名と展開先を置き換えてください。
4. 展開先の `bin/pqc-scan` と `rules/default` を一組で使います。展開だけではPATHは変更されません。

配布物には、ルール、ライセンス資料、利用ガイド、画面付き手順とリリースノートが含まれます。
以下の `/absolute/path/to/bundle` は展開した配布物の最上位ディレクトリに置き換えてください。

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

## ブラウザで使う

```sh
/absolute/path/to/bundle/bin/pqc-scan serve --root /absolute/path/to/projects --rules-dir /absolute/path/to/bundle/rules/default
```

ターミナルに表示された完全なセッションURLを開き、フォルダを選択して
**選択したフォルダをスキャン**を押します。通常モードで1フォルダだけの場合は
スキャンレポートが自動で開き、複数の場合は対象名付きの結果一覧が表示されます。
移行確認を使うには起動時に `--migration` を加えます。
[画面付きの8ステップ](SCANNING.ja.md#web-migration-walkthrough)に沿って操作してください。
終了時はCtrl+Cを押します。

## ソースから導入する

リポジトリを取得し、`rust-toolchain.toml` で指定されたRustを使って、リポジトリ直下で実行します。
導入後もルールを参照できるよう、このチェックアウトを保管してください。

```sh
cargo install --locked --path crates/pqc_scan_cli
pqc-scan rules list --rules-dir /absolute/path/to/pqc-scan/rules/default
pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/pqc-scan/rules/default --format all --out-dir /absolute/path/outside/repository/reports
```

ビルド時には依存パッケージの取得が必要になる場合があります。
スキャン結果は対象リポジトリの外へ保存してください。

## v0.4で移行確認を使う

`serve --migration` で用途・利用状況を入力し、移行に向けた「やること」と「次に進める条件」を確認できます。
保存済みの検出結果は `assess` で再評価でき、入力のひな形は `context-template` で用意できます。
再評価には、元の `scan.inventory.v1.json` と対応する入力JSONを一緒に保管します。
[移行確認の読み方](REPORTS.ja.md#saved-assessment)と[保存済み入力の使い方](SAVED-EVIDENCE.ja.md)を参照してください。

## 更新と差し戻し

新しい配布物は別の空ディレクトリに展開します。以前のバイナリとルールは一組で残してください。
新しいコマンドで `--help`、`rules list`、秘密情報を含まないフォルダのスキャンを確認してから、
利用するパスを切り替えます。戻す場合は以前の配布物を指定します。ルールを上書き混在させないでください。
自動更新機能はありません。比較用に設定とレポートを残します。

## ライセンスと問い合わせ

同梱の `DEPENDENCY-LICENSES.json` と `licenses/` に依存パッケージのライセンス資料を収録しています。
共有するレポートにはソース断片やパスが含まれるため、内容を確認してください。
脆弱性の報告は[SECURITY.ja.md](https://github.com/wakaken/pqc-scan/blob/codex/v04-roadmap-20261118/SECURITY.ja.md)を参照してください。
