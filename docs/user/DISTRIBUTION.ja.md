# インストールと配布

[English](DISTRIBUTION.md) · [README](https://github.com/wakaken/pqc-scan/blob/7bcbca20bc61380873005be71440bd53889ff5dd/README.ja.md)

このガイドは、macOS Apple Silicon（`aarch64-apple-darwin`）向けの**v0.4.0静的版候補**を扱います。
公開済みの配布物は[GitHub Releases](https://github.com/wakaken/pqc-scan/releases)で確認し、
バイナリ・対応するルールとライセンス文書・SHA-256・ホスト型のビルド来歴証明を一組で管理してください。
ローカルの準備物やActions成果物は、公開済みReleaseとは別です。
[v0.3.0のノート](../releases/v0.3.0.ja.md)と、過去の[v0.2.0のノート](../releases/v0.2.0.ja.md)も参照してください。
以前の配布物にv0.4.0の変更は含まれません。[v0.4.0のノート](../releases/v0.4.0.md)も参照してください。

0.2.x・0.3.xの各系列で最新の検証済みバージョンを可能な範囲で保守し、SLAや旧バージョンへの修正提供は保証しません。
TLS／SSHは実験的機能です。probe同梱アーカイブのライセンス確認フラグと公開保留は維持します。
他のバイナリ環境やcrates.io公開は今回の範囲に含めません。
[脆弱性の連絡方法](https://github.com/wakaken/pqc-scan/blob/7bcbca20bc61380873005be71440bd53889ff5dd/SECURITY.ja.md)も確認してください。

## 静的版0.4.0の準備

候補版では、オフラインの `assess`、`context-template`、`serve --migration` を使えます。
保存した `scan.inventory.v1.json` の元のバイト列と、対応するcontext JSONを一緒に管理します。
日程・やること・次に進める条件は[レポートガイド](REPORTS.ja.md#saved-assessment)、
対応付けは[保存済み入力](SAVED-EVIDENCE.ja.md)を参照してください。これらのガイドの日英版、
対応するルールと通知文を同梱します。`uuid-simd 0.8.0` と `vsimd 0.8.0` は、既存の
オフライン検証を保ったまま、正確な上流リビジョンの通知文を登録しました。

候補の準備と公開は別です。最終のcleanなSHAで2回梱包し、バイト一致、チェックサム、
クリーンな環境へのオフライン導入を確認します。ホスト型CI、静的版のビルド来歴と
対象ソースの照合後、維持者の範囲を限定した製品受入・公開承認を記録してからタグ・公開へ
進みます。既存の手動workflowは続けて `network` も梱包するため、静的版の結果と
workflow全体の結果を区別してください。静的版の成功でネットワーク版の公開は承認されません。
下記の既存の保守方針は維持し、v0.4の受入は別途記録します。

## 静的版0.3.0（以前の公開版）

維持者はmacOS Apple Silicon向け静的版を評価範囲と制約を踏まえて受け入れ、
最終ホスト型ビルドの来歴検証後の公開を承認しています。
[技術評価](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/releases/static-candidate-20261117/AUDIT-FIXES.md)には、
完了した確認と残る制約を記録しています。製品受入、ビルドの出所、Release公開は別の事実です。
バイナリ・ルール・アーカイブのマニフェスト・ソースコミットを一組で管理してください。バージョン番号だけでは対象を特定できません。

Unixでは出力先のシンボリックリンクを拒否し、通常ファイルの置き換えには非公開の一時ファイルを使います。新しい出力はモード0600となり、以前のinodeの権限やACLは引き継ぎません。Unix以外への出力は未対応です。バージョンをまたぐ位置・根拠の補正、JavaのRSA参照の追加、証明書属性の訂正では、件数・値・該当IDが変わる場合があります。リスク・信頼度の方針や形式のバージョンは別に扱います。[互換性情報](../releases/v0.3.0.ja.md#compatibility)を参照してください。レポートは対応範囲内の調査材料であり、リポジトリの安全性やPQC移行完了を証明するものではありません。networkバージョンの公開保留も維持します。

<a id="release-status"></a>

## 配布構成とprobe同梱版の確認事項

v0.2候補では、macOS arm64向けの3構成すべてでホスト型CI、同一アーカイブの再作成、クリーンな環境への導入を確認しています。配布バイナリは `static` です。この構成には通知文の不足や `notice_review_required` はありません。過去の公開物は[v0.2.0のGitHub Release](https://github.com/wakaken/pqc-scan/releases/tag/v0.2.0)、現在の静的版の公開物は[GitHub Releases](https://github.com/wakaken/pqc-scan/releases)で確認してください。probe同梱版は公開対象に含みません。

| 構成 | 追加コマンド | 公開までの状況 |
|---|---|---|
| `static` | なし | v0.4.0候補を準備中。v0.3.0の公開物とは別に、選んだバージョンの来歴を検証する |
| `network`（接続先診断付き） | 静的スキャンに加え、`probe tls`、`probe ssh`、`probe ssh-kex` | 未公開。依存パッケージ4件の通知文・再配布条件の確認が残る |

確認対象は、[バージョンを固定した通知文の記録](../../third_party/notices/manifest.json)に残しています。

| 依存パッケージ | 該当する構成 | 公開前に行うこと |
|---|---|---|
| `block2 0.6.2`、`dispatch2 0.3.1`、`objc2-encode 4.1.0`、`objc2 0.6.5` | `network` | 収集済みの上流文書には、許諾文へのリンクとApple SDK由来部分に関する疑問が記載されている。このバージョンに適用する許諾文の全文と著作権表示を揃え、再配布条件を確認し、根拠と公開判断を記録する |

`russh-util 0.52.0` のソースを比較し、Rustファイル3件は記録された上流リビジョンと一致し、元のマニフェストの差はパッケージのバージョン番号だけであることを確認しました。配布物の `dirty=true` の記録は保持し、Apache-2.0本文も固定リビジョンに対応付けています。この技術的な確認フラグは解消しましたが、公開を許可するものではありません。`network`は、上記4件の通知文確認が残るため公開保留を維持します。[追加調査の記録](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/tasks/063/TASK-063-RESULTS.md)を参照してください。

これらは通知文とソースの由来についての未確認事項です。probeの故障や依存パッケージの違法性が判明したという意味ではありません。根拠を揃えるまでは確認フラグを残します。依存パッケージを差し替えたり更新したりする場合は、機能構成・プロトコル・梱包・導入を再検証してください。

静的版はこれらの依存パッケージを含まないため、probeの確認事項と分けて配布します。公開物には、リリース対象コミットのCI・配布検証を通過したアーカイブ、SHA-256、`build-provenance.jsonl`、日英リリースノートを添付します。Releaseに記録されたソースコミットと来歴を照合してください。

probe同梱版も公開する場合は、上の確認に加え、その構成のホスト型ビルド来歴を生成・検証する必要があります。現在のワークフローが来歴を証明するのは `static` だけです。最後に公開対象の入力で再検証してからアップロードします。済んでいる技術検証は[候補の検証記録](https://github.com/wakaken/pqc-scan/blob/d20a1f1872d8527d90dc3832a0d8b1b2284405b7/docs/records/v0.2-release/V02-RELEASE-VALIDATION.md)で確認できます。`review_required` は利用者が無効にする設定ではありません。

## ソースから導入する

このチェックアウトの候補の製品バージョンは、静的版`0.4.0`です。ソースのコミットを記録し、対応するバイナリとルールを一緒に管理してください。`--version`だけでは候補のどのスナップショットかを区別できません。以下のローカル配布物の名前もCargoのバージョンに従い、新しい正式リリースを示すものではありません。

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
`static`は実接続probeを含まず、`network`は`probe-tls,probe-ssh,probe-ssh-kex`を有効にします。
probeは明示コマンドと接続許可が必要です。実行前に同梱の
[probeの許可条件と使い方](PROBING.ja.md)を確認してください。ルールは常に`--rules-dir`で指定します。
展開だけではPATHやルール探索の設定は変わりません。

```sh
cargo build --locked --release --package pqc_scan_cli
python3 scripts/package-release.py --binary target/release/pqc-scan --variant static --out-dir target/task-038/static
cargo build --locked --release --package pqc_scan_cli --features probe-tls,probe-ssh,probe-ssh-kex --target-dir target/network-release-build
python3 scripts/package-release.py --binary target/network-release-build/release/pqc-scan --variant network --out-dir target/task-038/network
python3 scripts/package-release.py --verify target/task-038/static/pqc-scan-0.4.0-aarch64-apple-darwin-static.tar.gz
```

準備処理はオフラインのCargoメタデータを使うため、依存物は先にビルドで取得します。
`PROVENANCE.json`には入力バイナリのハッシュ、ソースの変更状態、入力ファイル、
featureとツールのバージョンを記録します。記録したソースからそのバイナリが生成された証明では
ありません。公開前に別途ビルドの証跡と照合してください。
`DEPENDENCY-LICENSES.json`は指定target/featureの通常・ビルド依存、申告ライセンス、
収集した通知文を記録します。法的な認証やバイナリの正確なリンク構成の証明ではありません。
必要な通知文が見つからない依存物があると、準備処理は失敗します。

アーカイブと同じディレクトリで`shasum -a 256 -c FILE.sha256`を実行し、隣接するハッシュと
照合します。Pythonの検証コマンドは内部の全ファイルのハッシュも照合し、危険なパスや
通常ファイル以外を拒否します。これらは入力したハッシュに対する改変検出です。
信頼できる署名や取得元がなければ真正性までは確認できません。
中身を確認してから、空のローカルディレクトリへ
`tar -xzf FILE.tar.gz -C EMPTY_DIRECTORY`で展開します。最上位ディレクトリには
`bin/pqc-scan`、`rules/default`、ライセンス通知、リリースノート、導入・probeの案内が入っています。

```sh
/absolute/path/to/bundle/bin/pqc-scan rules list --rules-dir /absolute/path/to/bundle/rules/default
/absolute/path/to/bundle/bin/pqc-scan scan /absolute/path/to/authorized/repository --rules-dir /absolute/path/to/bundle/rules/default --format json --out-dir /absolute/path/outside/repository/reports
```

既定の静的版にも `serve` が含まれます。ブラウザで対象を選ぶには、次を実行します。

```sh
/absolute/path/to/bundle/bin/pqc-scan serve --root /absolute/path/to/projects --rules-dir /absolute/path/to/bundle/rules/default
```

フォルダを1つ以上選び、「選択したフォルダをスキャン」を押します。通常の `serve` で1フォルダなら
スキャンレポートが自動で開き、複数なら対象名の付いた結果カードを表示します。`--migration` を
加えると、スキャンレポートと移行確認を選べます。[画面付きの手順](SCANNING.ja.md#web-migration-walkthrough)も参照してください。終了はCtrl+Cです。
サーバーは `127.0.0.1` だけで待ち受けます。ターミナルに表示された完全なセッションURLを開いてください。
配布物には日英の[スキャン手順](SCANNING.ja.md#ローカルwebスキャン)と[レポートの読み方](REPORTS.ja.md)も含みます。

アーカイブはパス、権限、所有者、時刻を正規化します。同じバイナリ、変更のないソースと
ツール入力なら再生成したバイト列も一致します。Rustバイナリそのものの独立した再現ビルドや、
異なるホスト間の再現性は保証していません。

## 更新と差し戻し

以前のバイナリとルールは一組で残します。新しいアーカイブを検証して別の空ディレクトリに
展開し、来歴、`--help`、`rules list`、秘密を含まないfixtureのスキャンを確認してから
実行パスを切り替えます。バイナリとルールには同じスナップショットの絶対パスを指定します。
差し戻すときは以前のディレクトリを指定してください。ルールの上書き混在やカスタムルールの
無断置換は避け、比較用に設定とレポートを残します。自動更新機能はありません。ホスト型の静的版は後述の来歴検証を行います。ローカルで梱包するだけでは、その証明は付きません。

## 同梱するライセンス資料

レジストリーの配布物に通知文がない場合は、Cargo.lockのchecksumと配布物のVCSバージョンに一致する、事前取得済みの上流資料を使います。梱包時の取得通信は行わず、バージョン・SHA・出所が一致しなければ停止します。
静的版で補完する通知文には、元の著作権表示を含む上流のライセンス全文を同梱します。
objc2系4組の上流資料は、ライセンス本文へのリンクとApple SDK由来部分の注意書きです。本文や公開許諾を確認したことにはならないため、`review_required` を残します。[バージョンを固定した資料一覧](../../third_party/notices/README.md)を参照してください。

## 接続先診断付きの構成と個別ビルド

新しく準備する配布物は、`static`と`network`の2構成です。従来の`all-probes`は
`network`へ改名し、`combined-probes`の配布物は新規作成しません。準備コマンドでは
旧名2つを受け付けませんが、過去のアーカイブの検証・導入は引き続き可能です。
過去の記録は旧名のまま保持します。前述のv0.2の3構成の検証は、当時の候補についての記録です。

内部のCargo featureは個別に残します。ソースからは、例えば
`--features probe-tls,probe-ssh`だけを選んでビルドできます。これは3つ目の配布構成ではありません。
`network`にはSSHの提示方式収集と、公開鍵を指定して行うハイブリッド鍵交換の両方を含みます。

```sh
python3 tests/distribution/check-install.py target/task-038/network/*.tar.gz --out-dir target/task-038/network-install
```

依存noticeとprovenanceも同じfeature構成から生成します。インストール検証は`tls`・`ssh`・`ssh-kex`の過不足と公開鍵pinオプションを確認します。既存のobjc2系4件の確認フラグは維持します。russh-utilのソース差分調査は前述のとおり解消しましたが、配布物のdirty=trueの記録は保持します。固定リビジョンのライセンス原文は同梱します。これは未署名・未公開の技術的な配布準備です。

## ホスト型ビルドの来歴を検証する

次の例は公開済みv0.2.0のアーカイブ用です。ローカルで準備した0.4.0候補にホスト型の来歴証明は付きません。配布準備workflowが生成したアーカイブでは、リポジトリ・workflow・公開対象のソースコミットを指定して検証します。

```sh
gh attestation verify pqc-scan-0.2.0-aarch64-apple-darwin-static.tar.gz --repo wakaken/pqc-scan --signer-workflow wakaken/pqc-scan/.github/workflows/release.yml --source-digest FULL_RELEASE_COMMIT_SHA
```

`FULL_RELEASE_COMMIT_SHA` は選択したリリースの記録から確認します。バージョン名やハッシュだけで来歴を判断しません。この検証はビルドの出所を確認するもので、検出精度やライセンスの適法性を認定するものではありません。ローカルアーカイブは、ホスト型の来歴証明を検証できない限り未署名の準備物です。タグ作成・Releaseへのアップロードは候補の受入後に別途実施します。
