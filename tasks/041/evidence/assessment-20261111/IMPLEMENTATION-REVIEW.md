# TASK-041 実装レビュー

レビュー日: 2026-11-11。対象branch: `codex/quality-assessment-model`、base HEAD:
`0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。未コミット差分・新規ファイルを確認しました。
主エージェントによる既存040との重複発見後、新規タスクは041として扱っています。
追加委譲および製品ソース変更は行っていません。

## 初回判定

**修正依頼（現時点では承認不可）**。以下3件の修正と、対応する負例の検証を要します。

### [P1] ソースbindingの欠落を受理してしまう

対象: `tests/quality/report.py:89-94`。

`runtimeSourceBindings` は記載された項目だけ、`harnessBindings` は存在する
evaluate.py/run.py項目だけ照合しています。必須項目集合や記録済みdigestの整合を
確認しないため、runtimeSourceBindings全削除、またはevaluate.py/run.pyのbinding削除で
ソース確認を迂回できます。いずれも生成処理が成功することを再現しました。
「raw/current source hashesが追跡可能」「根拠欠落を成功にしない」という受入条件を満たしません。

修正: runnerと同じ現物ファイル集合との完全性照合、記録済みruntime digest整合、
evaluate.py/run.pyの必須bindingとharness digest整合を確認してください。
単なる値不一致に加え、項目削除・空mapping・digest不一致を負例に含めてください。

### [P2] benchmarkの入力母数とhashを検証せず品質報告へ転載する

対象: `tests/quality/report.py:127-143`。

性能sampleとmeasurement.jsonの一致は確認していますが、benchmark.inputは独立照合を
行っていません。large.inputを`files=0, bytes=0, sha256=全0, bindings={}`へ変更しても
受理し、1312ファイルの実測値に0ファイルの入力情報を結び付けた品質JSONを生成できます。
summaryのhashを新しく付けるだけでは、その入力情報の妥当性は確立しません。

修正: 既存4レシピについて、corpusと既存pathological入力から期待する入力binding・件数・
bytes・digest・recipeを照合してください。独立照合できない場合は未検証と明示し、
検証済み性能の根拠として扱わないでください。

### [P2] 性能未実施でもQ5の説明が性能sampleの存在を断定する

対象: `tests/quality/report.py:184-190`。

既存runnerは`--skip-benchmarks`をサポートしています。`benchmarks={}`は受理されますが、
Q5のscopeは常に`Repeated semantics and synthetic resource samples`です。
性能を一度も測定していない正規入力でも、資源sampleを確認したように表示します。

修正: 繰返しの証拠と性能測定の状態を分け、性能未実施を`Not Tested`として表示してください。
Q5全体をPartialとする場合も、理由に未実施の性能sampleを含めないでください。

## 確認できた点

- 既存run.py/evaluate.py/truthは変更せず、同じscorerを使う構成です。
- FPRをnegative truth単位の`negative-tn`で計算し、重複・extra所見FPとの分母差を維持しています。
- 分母0はnull/undefinedです。Q3/Q4/Q6・algorithm truth・releaseDecisionの限界を明示しています。
- 層別は入力分類から既存scorerを呼び直し、scanner出力からtruthを作っていません。
- 通常CLI/rules/risk/schemaの差分はなく、CIは既存baseline jobへの派生報告追加です。
  数値比較Failedを新しい数値CIハードゲートにはしていません。
- README両言語の案内、モデル/指標/実測/評価/ロードマップの役割は整合しています。
  exact base HEADのhosted成功と未push変更の未実行を区別しています。
- 既存測定の上書きを避ける出力制御と、対象をtarget配下に限定する処理があります。

## 検証方法と限界

提示済みのentry試験、report9件成功、候補生成結果を参照しました。フルsuiteは再実行せず、
具体的疑義だけPythonで再現しました。既存候補baselineの各rawディレクトリを一時ディレクトリへ
リンクし、summaryの対象項目だけ変えてbuild_assessmentを呼びました。
並行中の既知task-IDテスト変更による無関係な拒否を避けるため、この再現用summaryに限り
runtime bindingを現物hashへ合わせ、対照ケースの成功も確認しました。元の測定・ソースは変更していません。

再現結果は対照、runtime binding削除、evaluator/runner binding削除、benchmark input改変、
benchmark未実施の全5ケースで生成成功でした。後4ケースが上記指摘の根拠です。

本レビューは実装レビューです。最終試験の妥当性レビューと最終成果物hash照合は別段階です。
修正後は同じレビュー担当が上記3件と必要な関連差分に絞って再確認します。

## 修正後の限定再確認

**実装レビュー承認**。上記3件は解消し、実装上の必須修正は残っていません。
同じレビュー担当で修正箇所・関連試験・実測成果物に限定して再確認しました。

- P1解消: `runtime_bindings()`はrunnerと同じ対象集合を現物から列挙し、mapping全体と
  digestの両方を照合します。evaluate.py/run.pyは必須bindingとなり、harness digestも
  確認します。削除後にdigestを再計算した負例があり、digest不一致だけに依存しない検証です。
- benchmark入力のP2解消: `benchmark_input()`は既存small/medium/largeのコピー数と
  pathologicalの既存文字列を使い、input全体とrecipeを照合します。偽の入力母数・hash・
  recipeの拒否を確認するassertionがあります。raw実測4workloadからの生成も成功しています。
- 未実施表示のP2解消: `performanceMeasurement.status/scope`とQ5 scopeが条件分岐し、
  benchmarkなしでは`Not Tested` / `Performance workload samples not measured`と
  表示します。scope文字列の完全一致assertionを含み、未実施のsample確認を断定しません。
- 新タスクは041、既存040は保持されています。元の001〜040の存在・ID一意性・依存解決を
  保ったまま追加を許容しており、重複IDを許可する変更はありません。

`review-fix-red.log`の失敗と`review-fix-green.log`の10件成功、
`performance-copy-red.log`のscope assertion失敗と同greenの10件成功を読みました。
最終的なreport.pyから`baseline-final`を読み直す限定確認も成功し、4workload、
同truth回帰Passed、releaseDecision Unknownを確認しました。フルsuiteは再実行していません。

確認時のSHA256:

| 対象 | SHA256 |
|---|---|
| tests/quality/report.py | `63b798a59a7b51417b8282487a09f5dbe0f3f8a6bd75776e77300fd66f08e0f4` |
| tests/quality/test_report.py | `174b8a72e072e9666681500dac8ec1d069cd04d664246c8cb3af58117e6e9173` |

`report-revised`のsourceSummarySha256は`baseline-final/summary.json`と一致し、
記録された25 rawBindingsはすべて現物hashと一致しました。
その後の未実施scope文言修正でreporter hashが更新されたため、最終の生成報告・
docs snapshot/EVIDENCEは主エージェントがこの最終source版へ更新してください。
これは予定済みの最終証拠整理であり、新しい製品実装修正要求ではありません。
最終試験と試験妥当性レビューの完了判断、公開可否は本実装承認の対象外です。

## CI self-scan期待値の追補レビュー

**追補を承認**。対象は`docs/quality/SELF-SCAN-EXPECTATIONS.json`への
`tests/quality/report.py`一件追加です。既存TASK-040の運用文書とhelperの条件を確認しました。

- HEAD版の既存132 file entriesとversionは、現行JSONの先頭132 entries・versionと
  意味的に完全一致しました。追加は一つの明示file entryのみです。
- 追加元のfull SHA256は現物`report.py`と一致します。登録された3件は
  `TS_JWT_RSA_ALGS`、`JWT_RS256`、`API_JOSE_RSA_ALG`で、いずれも108行36列、count1です。
  raw SARIFのfull-result digest、path、rule、line、column、件数との一致を確認しました。
- 当該ソースは既存pathological benchmark文字列の入力hash再構成です。
  署名・鍵生成・稼働中暗号設定ではありません。新規登録理由と実コードの責務が一致します。
- 1116 raw = 1088 reviewed omissions + 28 retainedの分割を確認しました。
  追加前のupload copy31件から当該full-result digest3件だけを除いたSARIF全体が、
  `code-scanning-reviewed.sarif`とdeepEqualになりました。結果以外のmetadataも保持しています。
- helper、scanner、rules、threshold、広域除外に追補差分はなく、full source/result/location/count
  bindingを使う既存方式の範囲です。実装修正の必須事項は追加されません。

現時点のJSONでは全体の再整形による大きなtext diffもありますが、意味的差分は上記の一件だけです。
従来の書式を保つことは任意の可読性改善として主エージェントへ伝えました。
最終のNode/Python・実コマンド検証は主エージェントが実施し、その妥当性は試験レビュー段階で判断します。
