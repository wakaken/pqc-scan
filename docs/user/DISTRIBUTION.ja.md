# ローカル導入と配布準備

リリース目標は **v0.2.0** です。ソースからの導入に加え、GitHub Releasesで静的スキャン版のmacOS Apple Silicon（`aarch64-apple-darwin`）アーカイブ、対応するルールとライセンス文書、SHA-256、GitHubのビルド来歴証明を配布する方針です。developへのマージやActionsの成果物保存は、Release公開とは区別します。[リリース内容](../releases/v0.2.0.ja.md)も参照してください。

保守は最新の検証済み0.2.xに対するベストエフォートとし、SLAや旧版への修正提供は保証しません。TLS／SSHは実験的機能です。probe同梱アーカイブは技術的な配布準備として維持し、既存のライセンス確認フラグも残します。他のバイナリ環境やcrates.io公開は今回の範囲に含めません。[脆弱性の連絡方法](../../SECURITY.md)も確認してください。

## ソースから導入する

`rust-toolchain.toml`で指定したツールチェーンを使い、チェックアウト直下で実行します。
ルールはこのチェックアウトとともに保管してください。

```sh
cargo install --locked --path crates/pqc_scan_cli
pqc-scan rules list --rules-dir /absolute/path/to/pqc-scan/rules/default
pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/pqc-scan/rules/default --format all --out-dir /absolute/path/outside/repository/reports
```

導入後の静的スキャンは、ローカルの入力とルールだけで動作します。ビルド時には依存物の
取得が必要になる場合があります。レポートの出力先はスキャン対象の外に置きます。
許可されたリポジトリだけを調査してください。レポートにはソースや接続先の情報が
含まれ得るため、共有する前に内容を確認してください。

## ローカルアーカイブを準備する

生成物は未署名・未公開の配布準備用アーカイブです。正式リリースではありません。
`static`は実接続probeを含まず、`combined-probes`は`probe-tls,probe-ssh`を有効にします。
probeは明示コマンドと接続許可が必要です。実行前に同梱の
[probeの許可条件と使い方](PROBING.ja.md)を確認してください。ルールは常に`--rules-dir`で指定します。
展開だけではPATHやルール探索の設定は変わりません。

```sh
cargo build --locked --release --package pqc_scan_cli
python3 scripts/package-release.py --binary target/release/pqc-scan --variant static --out-dir target/task-038/static
cargo build --locked --release --package pqc_scan_cli --features probe-tls,probe-ssh --target-dir target/probe-release-build
python3 scripts/package-release.py --binary target/probe-release-build/release/pqc-scan --variant combined-probes --out-dir target/task-038/combined
python3 scripts/package-release.py --verify target/task-038/static/pqc-scan-0.2.0-aarch64-apple-darwin-static.tar.gz
```

準備処理はオフラインのCargoメタデータを使うため、依存物は先にビルドで取得します。
`PROVENANCE.json`には入力バイナリのハッシュ、ソースの変更状態、入力ファイル、
featureとツールの版を記録します。記録したソースからそのバイナリが生成された証明では
ありません。公開前に別途ビルドの証跡と照合してください。
`DEPENDENCY-LICENSES.json`は指定target/featureの通常・ビルド依存、申告ライセンス、
収集した通知文を記録します。法的な認証やバイナリの正確なリンク構成の証明ではありません。
通知文が空の依存物は公開前に確認が必要です。

アーカイブと同じディレクトリで`shasum -a 256 -c FILE.sha256`を実行し、隣接するハッシュと
照合します。Pythonの検証コマンドは内部の全ファイルのハッシュも照合し、危険なパスや
通常ファイル以外を拒否します。これらは入力したハッシュに対する改変検出です。
信頼できる署名や取得元がなければ真正性までは確認できません。
中身を確認してから、空のローカルディレクトリへ
`tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY`で展開します。最上位ディレクトリには
`bin/pqc-scan`、`rules/default`、ライセンス通知と利用案内が入っています。

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

アーカイブはパス、権限、所有者、時刻を正規化します。同じバイナリ、変更のないソースと
ツール入力なら再生成したバイト列も一致します。Rustバイナリそのものの独立した再現ビルドや、
異なるホスト間の再現性は保証していません。

## 更新と差し戻し

以前のバイナリとルールは一組で残します。新しいアーカイブを検証して別の空ディレクトリに
展開し、来歴、`--help`、`rules list`、秘密を含まないfixtureのスキャンを確認してから
実行パスを切り替えます。バイナリとルールには同じスナップショットの絶対パスを指定します。
差し戻すときは以前のディレクトリを指定してください。ルールの上書き混在やカスタムルールの
無断置換は避け、比較用に設定とレポートを残します。外部公開、署名、自動更新、認証情報の
設定は別の作業です。

## 同梱するライセンス資料

レジストリーの配布物に通知文がない場合は、Cargo.lockのchecksumと配布物のVCS版に一致する、事前取得済みの上流資料を使います。梱包時の取得通信は行わず、版・SHA・出所が一致しなければ停止します。
静的版で不足していた5組は、元の著作権表示を含む上流のライセンス全文を同梱します。
objc2系4組の上流資料は、ライセンス本文へのリンクとApple SDK由来部分の注意書きです。本文や公開許諾を確認したことにはならないため、`review_required` を残します。[版を固定した資料一覧](../../third_party/notices/README.md)を参照してください。

## SSH KEXを含む構成

`all-probes`は`probe-tls,probe-ssh,probe-ssh-kex`を有効にします。既存の2構成は維持します。

```sh
cargo build --locked --release --package pqc_scan_cli --features probe-tls,probe-ssh,probe-ssh-kex --target-dir target/all-probes-release-build
python3 scripts/package-release.py --binary target/all-probes-release-build/release/pqc-scan --variant all-probes --out-dir target/task-038/all
python3 tests/distribution/check-install.py target/task-038/all/*.tar.gz --out-dir target/task-038/all-install
```

依存noticeとprovenanceも同じfeature構成から生成します。インストール検証は`tls`・`ssh`・`ssh-kex`の過不足と公開鍵pinオプションを確認します。既存のobjc2系4件に加え、この構成ではrussh-utilのCargo VCS記録がdirty=trueのため、上流リビジョンと配布ソースの差の公開前確認を残します。固定リビジョンのライセンス原文は同梱します。これは未署名・未公開の技術的な配布準備です。

## ホスト型ビルドの来歴を検証する

配布準備workflowが生成したアーカイブでは、リポジトリ・workflow・公開対象のソースコミットを指定して検証します。

```sh
gh attestation verify pqc-scan-0.2.0-aarch64-apple-darwin-static.tar.gz --repo wakaken/pqc-scan --signer-workflow wakaken/pqc-scan/.github/workflows/release.yml --source-digest FULL_RELEASE_COMMIT_SHA
```

`FULL_RELEASE_COMMIT_SHA` は選択したリリースの記録から確認します。版名やハッシュだけで来歴を判断しません。この検証はビルドの出所を確認するもので、検出精度やライセンスの適法性を認定するものではありません。ローカルアーカイブは、ホスト型の来歴証明を検証できない限り未署名の準備物です。タグ作成・Releaseへのアップロードは候補の受入後に別途実施します。
