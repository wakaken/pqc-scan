# Cursor監査2件の照合と対応 — 2026-11-11

基準HEAD: `5135eb71f94d51e6918697614cfa771c1f81af1b`。
ブランチ: `codex/scanner-quality-improvements`。今回の修正は未コミット。
監査1: `target/cursor-audit/2026-11-11T1108+0900/`。
監査2: `target/cursor-audit/2026-11-11T112341+0900/`。
両監査のREPORT/FINDINGS/COVERAGE/TASK-MATRIX/commandsと監査2のAUDITを読み、
現コード・開始時の未コミット変更を照合した。監査成果物と開始時の変更は
`/tmp/pqc-scan-audit-followup-backup/` にも保全した。

後続のID統一は[FINDING-IDENTITY.md](../../architecture/FINDING-IDENTITY.md)、現在の厳格な隔離・検証は
[PROTOCOL-CONFINEMENT.md](../protocol-confinement/PROTOCOL-CONFINEMENT.md)を参照。
以下の354/405件と旧nativeランナーの制限は、監査対応時点のsource-bound履歴である。
現在のLinux container実行では、メモリ・合計書込み量・プロセス群の制限とpreview/OpenSSHの
同一境界での実行を実装・実測している。過去の測定・証拠を現在のソースへ付け替えてはいない。

## 照合結果

| 監査の指摘 | 妥当性・開始時の対応状況 | 今回の対応・残件 |
|---|---|---|
| 1 F1: walkerがリンク・FIFOを読み、リンク自身の長さで上限判定 | ソースと回帰テストで確認 | 通常ファイルのみ選択。特殊エントリはReadFailedとして不完全性を記録。Unixは親fdから各成分をNOFOLLOW/NONBLOCKで開き、開いたファイルの種別・長さ・実読取limit+1を検査。検出・snippet・runtime hintにも適用。単一通常ファイルrootも維持。 |
| 1 F2: legacy finding_idの境界なし連結 | 確認。line/columnの1+23と12+3は同じ入力。native IDは長さ区切り済み | 後続の維持者指示でfinding-v1の長さ付きtuple/full SHA-256へ統一。重複除去も同じ方式にした。現仕様はarchitectureのFinding identityを参照。 |
| 1 F3 / 2 F2: YAML004と先頭ゼロID | 004の構文は開始時差分で修正済み。未引用IDの型・値変換は残存 | 全40IDを引用文字列へ統一。PyYAML6.0.2とserde_yaml0.9の両方で文字列・ファイル番号一致・一意性・依存解決を確認。PyYAMLでは008/009は文字列なので010/011の整数8/9とは衝突しない。STATUSの誤説明を訂正。 |
| 1 F4 / 2 F1: 023承認と品質文書の不整合 | HEADにはあるが、開始時差分のQUALITY-GATES/BASELINE/ROADMAP/023と別受入記録で対応済み | 承認範囲とdoneを保持。凍結測定値・truth・コーパスは変更しない。023完了を現在ソースの性能測定や1.0合格とは扱わない。 |
| 1 F5 / 2 F8: 当該HEADのhosted CIなし | 過去422照会・ローカル履歴とblocked記録は整合。今回は新規API照会をしていない | 001/037をblockedのまま保持。qualityのtest/clippyとscanのbuild/runに--locked、workflow既定権限にcontents:readを追加。Actionsの可変タグのSHA固定は未実施。対象の公開コミットとrun/artifactの確認が必要。 |
| 1 F6: 既定scanとprobe分離 | 欠陥ではなく確認事項 | 8 feature組合せcheck成功。既定の通常依存にrussh/Tokioなし。既定scanにネットワーク処理を追加していない。 |
| 1 F7 / 2 F4: SSH認証なしの証拠 | 呼出しなしとfalseフィールドはクライアント方針であり、各接続のパケット監視値ではない。過去記録も存在 | 既存キャッシュimage内で現在ソースをoffline buildし、実OpenSSHの5ケースを再実行。成功JA/ENは20,30,21,1、誤pinは20,30,21、署名改変は20,30、方式不一致は20。型5と50以上なし。将来の依存変更時も再確認が必要。 |
| 1 F8 / 2 F7: HTML参考評価とREADME | HTML削除は既に実装・テスト済み。英語READMEが曖昧 | 日英とも参考判定の出力先をMarkdown/明示JSONに限定と明記。HTMLには戻さない。既存方針有無のHTML一致テストと実ブラウザを確認。 |
| 1 F9 / 2 F8: objc系4件・配布範囲 | 記録どおり。実装欠陥ではなく公開前条件 | review_required4件を維持。既存archiveに新SSH KEXが入ったとは主張しない。公開・署名・新archive生成は未実施。 |
| 2 F3: 未列挙母数の留保がCLIにない | ignore fixtureで再現。coverage内の範囲内完了自体は意図した仕様 | stderrに範囲と未列挙母数のnote。所見ゼロは全体安全の証明でない旨を表示。require-complete・fail-on終了コードとreport schemaは維持。母数のフラグは現行では保守的に常時trueであり、実際の隠れた件数の測定ではない。 |
| 2 F5: protocol runnerの隔離が広い | allow default・環境継承・ビルド非隔離を確認 | deny default、空からのenv、専用HOME/TMP/CARGO、source読取のみ、test時scratchのみ書込、loopbackのみ通信、offline buildも別profileで隔離。CPU/ファイルサイズ/fd/NPROC/実行時間を制限。当時のnative実行ではメモリ・総ディスク量・プロセス木全体の資源上限は未達。後続の共通containerランナーでこの残件を解消した（PROTOCOL-CONFINEMENT.md）。 |
| 2 F6: TLS失敗に成功詳細が残り得る | 制御フロー上の懸念は妥当。正常完了後rustlsがNoneを返す実例は未再現 | version/suite/kindの検証が済むまで成功詳細を書かない。欠落値を直接与えるhelperテストと実TLS suiteを確認。暗号処理・真正性検証・partial許可診断は変更していない。 |

014/016のcancelledと日本語機能を保持し、rules list追加フィルターを実装していない。
001の古い014再開指示のみ現行判断への参照へ訂正した。

## 最終検証

証拠は `target/audit-followup/`。ローカルRustは宣言済み1.93.1、macOS arm64。
過去の347/397件とは別の、今回ソースへの実行である。

| 検証 | 結果・証拠 |
|---|---|
| cargo build --workspace --locked --offline | 成功、build-final.log |
| cargo fmt --all --check | 成功、fmt-final.log |
| cargo test --workspace --locked --offline | 354成功、workspace-final.log |
| 強化runnerの全feature exact binaries | 405成功・OpenSSH旧fixture1件ignore。55 executables全exit0、ソース/binary前後hash一致。protocol-final-sdk/RESULTS.jsonと各log |
| cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings | 成功、clippy-final.log。既定featureもclippy-default-final.log |
| feature8組 | 全check成功、features/RESULTS.json。通常依存のdefault-tree.logにもrussh/Tokioなし |
| Python品質検証 | 21成功、python-quality-final.log。YAML1.1 cross-checkはPyYAMLがなければ明示skip、今回は6.0.2で実行済み。YAML1.2はRustのtask_ids.rsでも実行 |
| Python配布検証 | 8成功、distribution.log |
| Node | 28成功、node-final.log |
| OS境界テスト | 1成功、isolation-final.log。秘密用ダミーHOMEの読取・source書込・外部connectの拒否とloopback成功を実測 |
| 現ソース/実OpenSSH製品試験 | 5成功、openssh/RESULTS.jsonとreports/RESULTS.json。既存imageを利用しbuild/runともnetwork none、host HOME/agent/socketのmountなし。コンテナ内sourceは書込可能なので厳格な監査用readonly-source実行とは区別 |
| 実ブラウザ | 下記観察とbrowser/*.png。HTMLを変更したという主張ではない |

強化runner開発中の失敗ログは削除していない。protocol/はdep-infoに必要なmetadata許可不足、
protocol-metadata/はxcrunの利用者cache書込・license lookupにより失敗。
protocol-final/は暗黙に選ばれたSDKとclangの不一致で失敗。
最終runnerは空環境の `xcrun --sdk macosx` で対応するclang/SDKを特定し、明示PATH/CC/SDKROOTを設定する。
OS設定変更やライセンス再同意は行っていない。macOSでのメモリrlimit設定も失敗したため、
未達を記録し、合格扱いにはしていない。

### 自己スキャンの既存期待値との対応

初回Nodeは4件失敗した。今回のmain/pipeline/recommendation等の行移動でsource bindingが古くなったため。
HEADのsource SHAと旧raw SARIFを確認し、未変更行の一対一写像だけで93記録・4ファイルを再対応した。
新raw SARIFに同じ内容の所見が存在することを確認し、新規の期待値・除外は0件。
`self-scan/REBIND-LEDGER.json` と `rebind-self-scan.py` が根拠。
raw1101件のうち既存の確認済み期待値1004件、保持97件。新しいテスト所見も無条件で除外していない。
rawとupload用copyを別々に保存している。これは精度測定や実製品受入ではない。

### ブラウザ観察

合成JSとignore fixtureだけから日英HTMLを生成し、専用directoryを127.0.0.1で配信した。
通常1280×720と狭幅390×844の日本語画面で、見出し・検査範囲・カードの折返しを確認。
Enterでルール、Tab→Enterで箇所を展開し、ソース抜粋・対応手順が表示された。
英語も範囲内完了・未列挙母数の留保を表示。HTMLに「ローカルの参考評価」はない。
配信serverを停止した後、読み込み済み英語画面で検索すると4→1ルールになり、Enter展開も動作した。
これは読み込み後の動作確認であり、offlineでの再読込は主張しない。
狭幅の未完了/空結果/大規模paginationの全組合せを今回再受入したわけではない。
serverを同じportで再開し、日本語tabを保持した。
確認用URL: http://127.0.0.1:63983/ja/report.html 。専用serverは継続中。

## レビューと残る条件

実装レビュー1名が単一ファイルrootの回帰を指摘。主担当が修正し、直接readとCLIの
strict-success/所見保持を追加確認した。同じ担当の再確認は承認・残る必須指摘0。
最終テスト後に別の担当が、アサーション・正常/異常/境界・feature・最終source/証拠SHA・
自己スキャンの未変更行93記録を照合し、承認・必須改善0と判断した。子agentや追加reviewerは起動していない。
結果とsource/evidence SHAはCURSOR-AUDIT-FOLLOWUP-2026-11-11.jsonにも保存する。

テストの限界として、metadata後の増大は実ファイル競合ではなく決定的なreader試験で
limit+1打切りを確認した。snippet/runtime hintの異常系は共通safe_read試験と呼出し経路の
確認で支えており、専用の再読取統合試験は追加していない。

Windows読取経路の実行、
Actionsタグ固定、最終ソースに対するhosted CI/artifact、実製品受入と公開前の法的確認は残る。
今回のローカル検証をこれらの代用にはしない。新しい性能基準測定、公開、署名、push、PRは実施していない。
