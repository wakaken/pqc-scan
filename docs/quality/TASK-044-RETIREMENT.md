# TASK-044 計測基盤の整理

044の当初目的は、重要な未到達箇所と追加検証手法の適用範囲を把握することです。製品全体のMiri合格は目標にしません。ユーザーの追加指示に従い、到達率計測と共通隔離基盤を残し、Miri専用実装を通常のテスト領域から退役させます。

基点は`f8aff1888525f0f3e350a734166c12c6de3e71ed`、既存ブランチは`codex/quality-assessment-model`です。041〜043等の既存変更と044の未コミット成果を引き継いでいます。製品のRust・Cargo・規則・公開Schema、CIのprotocols停止条件は今回の整理で変更しません。

## 残すものと退役するもの

- `tests/quality/task044/coverage.py`と集計試験を維持します。未測定branchと0%、分母のない位置、対象外ファイルを区別します。
- `entry.py`のLLVM計測、設定別profile、バイナリ・sourceの識別、raw profileの容量管理を維持します。
- 共通コンテナーの非root、読み取り専用source/root、外部通信禁止、空の環境、資源上限、有限export、終了後削除を維持します。
- Miri専用の`miri_followup.py`、`miri_range.py`、`miri_support.py`、`miri_os_selection.json`、`test_miri044.py`を通常のテスト領域から除きます。
- `entry.py`内のMiri試行と断片crateの生成、共通入口のMiri専用4ケースを除きます。Miriの逐次再試行・結果parserを常設基盤として保守しません。

## 現在の実行経路

通常の`coverage044`はRust 1.93.1でworkspaceの通常設定・全featureのline/regionを計測します。Miriをimport・実行せず、nightlyも呼びません。初期調査用のnightly identity小試行も通常経路から除きました。

branchは明示的な`coverage044-branch`で、固定nightly 2026-02-12を使って通常設定・全featureを計測します。nightlyの指定はcoverage側に置き、Miri用moduleに依存しません。

対象は従来どおりLinux arm64です。通常計測にはRust 1.93.1と対応するLLVM工具、branch計測には固定nightlyと対応するLLVM工具が必要です。汎用protocol imageだけでこれらが揃うとは扱いません。工具準備と計測対象の実行は分離します。既存のimmutable imageとlocked cacheを使い、次の例は既存コンテナー境界で動作します。branch計測は`case`だけを`coverage044-branch`へ変えます。Miri sysrootは必要ありません。

```python
import sys
sys.path.insert(0, 'tests/protocol')
from container_runner import execute
execute(immutable_image_id, new_output_directory, case='coverage044', timeout=1800)
```

この例は工具imageの取得・導入を自動実行するものではありません。再実行時には工具版とsourceが過去の測定と同じか確認し、結果を新しいディレクトリへ保存します。

## 過去証拠と再現情報

[当初の測定](TASK-044-RESULTS.md)と[Miri追加調査](TASK-044-MIRI-FOLLOWUP.md)の証拠フォルダーは書き換えません。両フォルダーの全77ファイルを[保護台帳](task044-retirement-20261113/HISTORICAL-HASHES.json)で固定しています。

退役前の現行コード10ファイルを[ソースarchive](task044-retirement-20261113/PRE-RETIREMENT-SOURCE.tar.gz)と[hash台帳](task044-retirement-20261113/PRE-RETIREMENT-SOURCE.json)へ保存しました。古いsourceBindingsやレビューhashを現在の版へ付け替えません。保存コードは過去の調査を理解・再現するための資料であり、現行の通常テストやCIからは呼びません。

Miriのhost/target runner衝突と固定工具での対処コマンドは、追加調査の文書に残します。将来Miriを使う場合は、対象リスクと欠陥仮説を定め、元crateの既存テストを明示的に選ぶところから判断します。

## 検証とレビュー

Miriを利用できない状態でcoverageの通常・branchの起動を試験し、通常経路からnightlyやMiriを呼ばないことを確認します。退役ケースはコンテナー操作前に拒否します。集計試験と実際の通常coverage計測、共通隔離境界試験を併用します。

実装・試験レビューは既存の二者を再利用します。最終結果と判定はこの文書および[044のタスク記録](../../tasks/044-quality-coverage-feasibility.yaml)に追記します。過去の有限な測定結果を、現行の全面的なMiri合格や製品安全性の証明へ読み替えません。

## 実行結果

通常coverageと明示branchの両経路が終了0で完走しました。各経路で通常514件成功、全feature565件成功・既存ignored1件です。通常経路の265コマンドにnightly/Miri呼出しがなく、branch経路にもMiri呼出しがないことを照合しました。source前後一致・コンテナー削除も確認しています。

Python品質59件、実コンテナー境界10件、macOS隔離1件、preview単体1件、Node35件、配布11件、fmt・Python構文チェックは終了0です。coverageのdoctestや新規Miri実行、新しい非instrumented workspace/clippyの成功は主張しません。詳細は[検証台帳](task044-retirement-20261113/VALIDATION.json)と[原記録](task044-retirement-20261113/VERIFICATION.tar.gz)に保存しました。

## 公開時のdevelop更新

作業中に別PR #24がdevelopへマージされ、targetは`26fa19e7bd2434836d6c4dbed0185f34e15b9e45`へ進みました。作業ツリーを変更しない`git merge-tree`で5ファイルの競合を確認しています。Gradleの静的呼出し解析とmap形式解析など、片側を選ぶだけでは既存修正を失う箇所があります。

初期の「新しいdevelopを自動で取り込まない」という指示を維持し、developの取り込み・競合解消・mergeはまだ行っていません。今回の測定は既存ブランチのsourceに対する証拠であり、統合後の製品の成功を自動的に保証しません。競合を解消する場合は両側の検出仕様・既存試験を維持し、統合したsourceで必要なRust・rule・文書契約の検証と同じ二者の確認が必要です。

## 判定

既存の[実装・試験レビュー](task044-retirement-20261113/REVIEWS.md)はともに受入で、修正必須の指摘はありません。退役変更の検証は完了しました。

公開作業はdevelopとの競合で未完了です。初期の取込禁止と今回のmerge指示の関係を確認するため、044は`blocked`としてPRを提示します。統合してよいことが確認できた後、競合解消したsourceの検証と必要な同じ二者の限定確認を行い、doneとmergeを判断します。

## develop統合の再開と検証

ユーザーがdevelopの取込・競合解消・再検証・mergeを明示承認したため、上記の公開依存を解除しました。親はfeature `1bb45f8` とdevelop `26fa19e`です。タスク001〜062、configコメント処理、syntheticな歴史的self-scan試験を維持しました。Gradleは静的callとmapを同じcomment/string境界で扱い、動的versionはliteralな依存名を残して宣言値・解決値をUnknownにします。これはPQC有効化の証明ではありません。

レビューで指摘されたinline map終端のP2は、quote外の`;`・`}`・コメントで区切る処理へ修正し、versionあり/なし・同一行複数宣言・quote内delimiterの回帰を追加しました。

旧RULEメタデータは[exactな旧版](task044-integration-20261113/PRE-INTEGRATION-RULE-METADATA.json)へ保存し、現行companionはtarget由来118行とfeature由来5行のsnapshot/fixtureをそのまま継承して新sourceに束縛しました。旧AUDIT companionや旧applicability reviewは変更していません。COVERAGE契約の更新には、新しい二者の独立レビューを[別のapplicability記録](../../tests/quality/applicability-044-develop-integration/assessment.json)として保存しました。

修正後のRust通常527件・全feature578件成功/既存ignored1件、両clippy/build、Python59・Node35・配布11・fmtは終了0です。5corpusはdebug通常構成で各3反復し、nominal報告と別のscoped報告を生成しました。043固定コーパスのFP/FNは0を維持します。baselineのFP4/FN2、expandedのFP17/FN0、expanded-v2のFP2/FN4等のnominal集計を保持します。対応単位のない予測は未注釈として区別し、製品の確定的な誤検知とはしません。全製品の精度へ一般化しません。性能比較は行っていません。

[統合検証台帳](task044-integration-20261113/VALIDATION.json)・[原記録](task044-integration-20261113/VERIFICATION.tar.gz)・[限定レビュー](task044-integration-20261113/REVIEWS.md)を参照してください。renderer13ファイルはdevelopと同じbytesで、PR24の実browser証拠を変更のない表示範囲に再利用しました。coverage/共通隔離のコードも1bbから不変です。過去の測定値・レビューhashは現在の版へ付け替えていません。

統合修正・ローカル検証・既存二者の限定レビューを終え、044をdoneとしました。公開前のこの記録はhosted CIやmergeの成功を先取りするものではありません。PR25で実際のCIを確認してからmergeします。

## 公開CIで判明したRSS試験の修正

統合head b1b1488の[初回CI](https://github.com/wakaken/pqc-scan/actions/runs/43382596747)はRust・fmt・clippy・Nodeを通過しましたが、PythonのRSS比較1件が失敗しました。固定32MiBの子より親imageのRSS floorが大きく、Linuxで大小の子が同じpeakを報告する条件を再現しました。計測器や過去のRSS値は変更せず、試験の大きい子を親SELF peakより大きな実接触入力に変更しました。

Linux高RSS親でのred/green、累積値を返す誤実装を検出する負対照、Linux計測試験6件・Mac品質59件を確認し、既存二者が受入済みです。[修正証拠](task044-ci-fix-20261113/VALIDATION.json)・[レビュー](task044-ci-fix-20261113/REVIEWS.md)を参照してください。Sourceとraw log、正確な再現recipeを保持しています。044をdoneへ戻し、最終headの実CIで再確認してからmergeします。
