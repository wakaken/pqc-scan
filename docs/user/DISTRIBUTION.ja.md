# インストールと配布物

[English](DISTRIBUTION.md) · [README](https://github.com/wakaken/pqc-scan/blob/develop/README.ja.md)

macOS Apple Silicon（`aarch64-apple-darwin`）向けの導入方法を説明します。\
バージョンごとの変更点は[v0.5.0のリリースノート](../releases/v0.5.0.ja.md)を参照してください。

<a id="release-status"></a>

## 配布物を選ぶ

[GitHub Releases](https://github.com/wakaken/pqc-scan/releases)で、利用するバージョンと構成を選びます。\
公開物の有無はReleaseページで確認してください。\
Actions成果物は公開済みReleaseとは別です。

配布用の圧縮ファイル（`.tar.gz`）には、実行ファイル・ルール・利用ガイドなどをまとめています。

| 構成 | ファイル名 | 使える機能 |
|---|---|---|
| `static` | `pqc-scan-0.5.0-aarch64-apple-darwin-static.tar.gz` | ローカルスキャン、ローカルWeb画面、移行確認 |
| `network` | `pqc-scan-0.5.0-aarch64-apple-darwin-network.tar.gz` | 静的版の全機能に加え、明示したTLS・SSH接続先の診断 |

圧縮ファイルと、対応する `.sha256` ファイルを、同じリリースページの **Assets** からそれぞれダウンロードしてください。\
`.sha256` はダウンロードした圧縮ファイルが壊れていないか照合するためのファイルで、圧縮ファイルの中ではなく別ファイルとして配布しています。\
名前は圧縮ファイル名の末尾に `.sha256` を付けたものです。

例：`static`版では、次の2ファイルを取得します。

- `pqc-scan-0.5.0-aarch64-apple-darwin-static.tar.gz`
- `pqc-scan-0.5.0-aarch64-apple-darwin-static.tar.gz.sha256`

接続先の診断には管理者の許可が必要です。\
[TLS・SSH診断ガイド](PROBING.ja.md)に沿って実行します。\
他のOS・CPU向けバイナリとcrates.ioからの導入は対象外です。

<a id="アーカイブから導入する"></a>

## 圧縮ファイルから導入する

1. 圧縮ファイルと `.sha256` を同じフォルダに保存します。
2. そのフォルダで `shasum -a 256 -c FILE.sha256` を実行します。<br>
   `FILE.sha256` はダウンロードしたファイル名に置き換え、照合が成功したことを確認します。
3. 新しい空のディレクトリを用意し、`tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY` で展開します。<br>
   ファイル名と展開先を置き換えてください。
4. 展開先の `bin/pqc-scan` と `rules/default` を一組で使います。<br>
   展開だけではPATHは変更されません。

配布物には、ルール、ライセンス資料、利用ガイド、画面付き手順とリリースノートが含まれます。\
以下の `/absolute/path/to/bundle` は展開した配布物の最上位ディレクトリに置き換えてください。

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

## ブラウザで使う

```sh
/absolute/path/to/bundle/bin/pqc-scan serve --root /absolute/path/to/projects --rules-dir /absolute/path/to/bundle/rules/default
```

1. ターミナルに表示された完全なセッションURLを開きます。
2. 対象フォルダを選び、**選択したフォルダをスキャン**を押します。<br>
   通常モードは1フォルダならスキャンレポートを自動で開き、複数なら対象名付きの結果一覧を表示します。
3. 終了時はターミナルでCtrl+Cを押します。

移行確認も使う場合は、起動時に `--migration` を加えます。\
詳しい操作は[画面付きの8ステップ](SCANNING.ja.md#web-migration-walkthrough)を参照してください。

## ソースから導入する

リポジトリを取得し、`rust-toolchain.toml` で指定されたRustを使って、リポジトリ直下で実行します。\
導入後もルールを参照できるよう、このチェックアウトを保管してください。

```sh
cargo install --locked --path crates/pqc_scan_cli
pqc-scan rules list --rules-dir /absolute/path/to/pqc-scan/rules/default
pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/pqc-scan/rules/default --format all --out-dir /absolute/path/outside/repository/reports
```

ビルド時には依存パッケージの取得が必要になる場合があります。\
スキャン結果は対象リポジトリの外へ保存してください。

## v0.4で導入された移行確認機能を使う

| やりたいこと | 使う機能・参照先 |
|---|---|
| 用途・利用状況を入力し、やることと次に進める条件を確認する | `serve --migration`。[移行確認の読み方](REPORTS.ja.md#saved-assessment) |
| 保存済みの検出結果を再評価する | `context-template` で入力のひな形を作り、`assess` で評価。[保存済み入力の使い方](SAVED-EVIDENCE.ja.md) |


## 更新と差し戻し

1. 新しい配布物を別の空ディレクトリに展開します。<br>
   以前のバイナリとルールは一組で残し、新旧のルールを上書き混在させないでください。
2. 新しいコマンドで `--help`、`rules list`、秘密情報を含まないフォルダのスキャンを確認します。
3. 確認後に利用するパスを切り替え、比較用に設定とレポートを残します。
4. 差し戻す場合は、以前の配布物のパスを指定します。


## ライセンスと問い合わせ

同梱の `DEPENDENCY-LICENSES.json` と `licenses/` に依存パッケージのライセンス資料を収録しています。\
共有するレポートにはソース断片やパスが含まれるため、内容を確認してください。\
脆弱性の報告は[SECURITY.ja.md](https://github.com/wakaken/pqc-scan/blob/develop/SECURITY.ja.md)を参照してください。
