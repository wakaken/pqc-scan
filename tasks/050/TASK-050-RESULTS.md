# TASK-050 性能と再現性

対象は`9e820669df04d0eb20efadc5786dc5a847e08174`と050の差分、既存ブランチ`codex/quality-assessment-model`です。
元の[9条件と9成果物](task.yaml)を維持し、測定実装・実測・二段階レビューを照合します。
製品のRust・規則・暗号判断・出力Schema・CI・承認済み数値条件は変更していません。

## 実施したことと理由

既存`tests/quality/run.py`の`wait4`測定に、明示opt-inの`--performance-study`を追加しました。
旧baselineの既定実行は維持します。wallは起動・走査・全出力・終了の回収まで、buildと測定後のhash/圧縮は含めません。
CPUはOSの子process会計のuser+system、RSSはOS定義のhigh-waterです。process tree全体の同時メモリ合計ではありません。
LinuxのRSSはKiBからbytes、macOSは元からbytes、取得不能はnullです。
Linuxではexec前のlauncher像の下限が含まれ得る点と、5ms pollingの分解能を開示します。

再配布可能なMITの本リポジトリ製品RustソースとCargo.toml/lockを1/4/8コピーし、入力ファイルごとのbytes/hashを前後照合します。
コピーは規模を変えるためのもので、独立した製品群や実世界の精度母集団ではありません。
同じrootで1/2threads、各1初回+3warmを逐次実行します。通常/全機能のrelease buildとLinux隔離/macOSを区別します。
初回は各設定の最初の標本で、2threadsの初回には同じ入力を1threadで既に走査しています。
本当のcold cacheは測っていません。初回も入力作成でcacheに載り得ます。特権操作でcacheを消してはいません。
過去の別日測定は日付・出典・差を残しますが、条件差があるため制御された日間変動や性能退行とは呼びません。

全11出力について、生bytesのhashと、限定した生成時刻の正規化後の意味/順序hashを別に測ります。
所見・資産・対応付け・観測ID、位置、根拠、unknown、重複は消しません。
JSONは明示した集合だけ並べ替え、重複を保持します。HTML/Markdownは生成時刻だけを除いた保守的な文字列比較です。
実行固有のrun/observation IDが変わる出力を「意味が完全に安定」とは呼びません。

閾値終了、不完全検査、不正rules、期限超過を実CLIで測り、成功throughput/分布から除きます。
不完全検査と閾値終了で所見・診断を保持し、不正rulesや期限超過で生成されなかった出力を捏造しません。
大きすぎるファイルの除外は検査scopeの選択であり、それだけでrequire-completeが失敗するとは扱いません。
不完全検査の対照には、既存baselineと同じ反復RS256による検出器の打切りを使いました。

## 測定器の検証と履歴

手計算の処理速度、失敗除外、null/単位、所見IDと重複の破壊、順序だけの変化、入力改変、実CLIのexit2/3、異なる環境の比較拒否を試験しました。
初回のID名取り違え、失敗fixtureが閾値未到達、サイズ除外を不完全検査と誤認した失敗は保存し、回帰を追加しました。
実装レビューのP2ではbinary/harness hash差が比較条件から漏れていました。差だけを変える回帰が赤となることを確認し、比較不可理由に両者を追加しました。

全標本の測定・入力・出力hashと意味/順序digestを保存します。出力原本は各studyのsmall/t1/初回を圧縮保存し、他標本は検査後に削除しています。
失敗の原本・ログ・保持結果は全4ケースを圧縮保存します。全成功標本の原本保存を意味しません。
Linuxは既存の非root、network none、source/root読取専用、6GiB・追加swapなし・2CPU・128PID・4GiB tmpfs・1200秒・64MiB export境界を維持します。
ホストでは外部通信や入力コードの実行を行いません。測定はホスト負荷を完全制御した実験ではありません。

## 測定結果と最終受入

最終測定は6study（macOS通常2、Linux通常2、Linux全機能2）、成功144標本と意図的失敗24ケースです。
入力はsmall 88files/1,186,600bytes、medium 352files/4,746,400bytes、large 704files/9,492,800bytesです。
同条件で比較できる18セルに2x超過はありません。warm最大値の前回比はwall 0.969〜1.088、RSS 0.963〜1.019です。
OS/build/features/測定器などが異なる72セルは比較不可理由を示し、製品退行と判定しません。

| 検証 | 結果 |
|---|---|
| `PQC050_BINARY=… python3 -m unittest discover -s tests/quality -p 'test_*.py'` | 91件成功、skipなし、終了0 |
| `python3 -m unittest discover -s tests/protocol -p test_container_runner.py` | ホスト4件成功、container明示起動6件skip、終了0。実際の境界は本隔離測定で確認 |
| 通常/全機能release build | macOSはworkspace、LinuxはCLIと依存workspace。すべて終了0 |
| `cargo fmt --all --check`、Python構文検査 | 終了0 |
| 通常/全機能の固定023-v2精度測定 | 各3反復で意味・順序・score一致、実行終了0。旧数値条件は下記のFailedを維持 |
| Linux隔離runner | 6コマンドと外側runner終了0、source前後一致、container削除済み |
| 原本の再照合 | 6studyのsource/hash、代表全11出力と全失敗原本のhash/bytesを照合 |

製品Rust/Cargo/rulesは049の検証時から変更していません。今回の主検証はPython測定器・実CLI・release buildです。
049のRust suite成功を今回の新規実行として数えてはいません。レポート表示を変更していないため新たなブラウザー受入は対象外です。
実装レビューのR2受入後、テストレビューがPython版差の比較漏れをP2として指摘しました。
Python版・CPU数・wall timer/cache条件の比較を追加し、差だけで比較不可・調査信号なしとなる回帰の赤→緑を確認しました。
最終Python91件も再実行し成功しています。走査・資源測定・出力検査の全関数は不変で、実行コードの差は比較条件定数だけです。
保存した144/24標本は書き換えず、最終ソースで比較を再集約しました。比較可否・比率・調査信号は不変で、比較不可の理由が増えています。
全harnessのbytes一致とは主張せず、[適用性記録](evidence/task050-20261114/MEASUREMENT-APPLICABILITY.json)で再利用範囲を説明します。
実装レビュー・テストレビューはともにR4で修正と測定再利用を受け入れました。追加必須指摘はありません。

旧固定注釈の生スコアはTP28/FP4/FN2/TN19で通常/全機能が一致しています。
旧baseline比でholdout/source_apiとholdout/configのFP増加があり、条件判定はFailedです。
043の既存調査と一致し、050で新しい検出器欠陥を修正したとは扱いません。
検出器を調整してスコアを合わせたり、注釈や基準を変えたりはしていません。

旧023-v2の精度条件も変更せず再計測しています。旧注釈と現在の検出の不一致は、043の[既存調査](../043/TASK-043-FIXES-RESULTS.md)で既に記録されています。
旧数値条件のFailedを再現した場合もそのまま残し、050の評価完了と製品の品質合格を分けます。

真のcold cache、制御した同条件での複数日測定、Windows、別CPU architecture、TLS provider性能は今回の実測対象外です。
正式対応環境の追加、閾値改定、CI実行、commit/push/PR/merge/releaseはこの依頼では行いません。

## 要求ごとの証拠

| 元の条件 | 確認した内容 | 成果物 |
|---|---|---|
| Q5-01 | wall/CPU/RSSの対象・単位、OS変換とnull | [resources](evidence/task050-20261114/resources.json) |
| Q5-02 | 実input manifest、前後照合、全files/bytesと起動/報告込みの分母 | [throughput](evidence/task050-20261114/throughput.json) |
| Q5-03 | MIT製品sourceの3規模・2threads・全標本と分布 | [scaling](evidence/task050-20261114/scaling.json) |
| Q5-04 | 初回/warm、sample数と分布、cold未測定・過去別日の条件差を明示 | [noise study](evidence/task050-20261114/noise-study.md) |
| Q5-05 | 変動除去を限定し、ID・事実・重複の改変検出、全出力比較 | [semantic reproducibility](evidence/task050-20261114/semantic-reproducibility.json) |
| Q5-06 | 意味と順序の別digest、文字列比較の範囲と安定性の条件 | [ordering](evidence/task050-20261114/ordering.json) |
| Q5-07 | OS/arch/tool/build/features/binary/harness/input/rulesの比較条件 | [environment comparison](evidence/task050-20261114/environment-comparison.json) |
| Q5-08 | exit2/3/1/deadlineと資源・保持結果、成功分布から除外 | [failure resources](evidence/task050-20261114/failure-resources.json) |
| Q5-09 | 2x調査条件を維持、比較不可を退行にしない、旧精度Failedも保持 | [regression review](evidence/task050-20261114/regression-review.md) |

保存証拠の入口は[索引](evidence/task050-20261114/EVIDENCE-INDEX.json)、[レビュー](evidence/task050-20261114/REVIEWS.md)、
[ソース](evidence/task050-20261114/FINAL-SOURCE.json)、[実コマンド](evidence/task050-20261114/HOST-VALIDATION.json)です。

## 完了判定

元のQ5-01〜09と9成果物、二段階レビューを照合して050をdoneとしました。
これは評価・測定の完了です。旧注釈に対する精度条件Failed、実行固有IDを含む出力の不一致を、製品品質の合格へ置き換えていません。
測定版と最終比較版の差、出力原本の限定保存、未実測範囲を含めた判断です。
