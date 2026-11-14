# TASK-041 テスト妥当性レビュー

レビュー日: 2026-11-11。担当: `quality_test_review`（第2段階）。
対象: `codex/quality-assessment-model` の未コミット変更、base HEAD
`0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。

**承認します。現在のPhase 0／承認不要のPhase 1の受入に必須となる試験不足はありません。**
製品全体の安全性、公開可否、新方式別精度、未push変更のhosted CI成功を承認するものではありません。
追加委譲、ソース変更、タスク状態変更、フルsuiteの再実行は行っていません。

## 確認方法

AGENTS.md、README両言語、architecture／knowledge文書、Reviewer／Evaluator prompts、
適用範囲が014のみであるRESTART-LOOPを読み、最終report.py・新規10試験・既存scorer／runner試験、
CI・queueテスト・self-scan期待値の差分と提示されたraw／ログを照合しました。
passing logsだけでは判断せず、assertionの入力・期待値・拒否経路を読みました。
追加の実行は、既存成果物のhash照合、truth／dispositionからの独立集計、
過去ファイル／SARIFの比較に限定しました。scannerや全試験は再実行していません。

## 要件と試験の対応

| 観点 | 確認した試験・証拠と判断 |
|---|---|
| 所見採点とnegative単位FPR | `test_report.py:14` はTP1／FP3／FN1／TN1／P2／N2からprecision1/4、recall1/2、F1=1/3、FPR1/2、FNR1/2、分母とFPU1を手計算で固定。FPをFPR分子に転用する実装を検出できます。既存`test_evaluate.py:28`、`:46`がpositive重複、negative／未注釈位置、対象外ルールを実際の所見から検証しています。 |
| 母数0・除外・未実施 | `test_report.py:29`、`:51`は空母数をnull、positiveの全見逃しをrecall0／FNR1として区別。既存`test_evaluate.py:39`はambiguous／unsupportedと予測の除外を検証。`:123`の新試験はunknown母数、algorithm／Q3／Q4／Q6 Not Tested、release Unknown、性能未実施scopeの完全一致を確認します。実`--skip-benchmarks`出力もNot Testedです。 |
| 既存matcher再利用と層別 | 最終実装は既存scoreを呼び、matchingを新設していません。新試験`:123`でsplit/family、rule、language、detector、artifactを個別にassert。さらにレビューで最終報告84行（12＋38＋13＋10＋11）を、truthラベルと既存scorerのdispositionから独立に集計し、全COUNTSと5率が一致しました。既存truth57件=P30/N20/ambiguous2/unsupported5、合計TP25/FP3/FN5/TN17を確認しています。 |
| 証拠不整合・失敗の拒否 | `test_report.py:140`はruntime空／必須項目欠落／digest不一致、evaluate・run欠落を拒否。欠落ケースではdigestも再計算するため、hash不一致だけを検出する偽の負例ではありません。`:160`はscore／raw／exit／timeout／coverage／source／corpus／input／repeatを変更して拒否、`:201`はtruth改ざんとrule provenance欠落を拒否します。 |
| 性能入力・raw・集計 | `test_report.py:180`はfirst＋warm3の正例とmedian3を確認し、分布／timeout／input／recipe／raw欠落を拒否。分布の正解をrunnerから作る箇所は、既存`test_run.py:60`の独立したmin1／median2／max3とnull資源のassertionで補完されています。既存の実child測定・RSS非継承・timeout試験は、wall／CPU／RSSとtimeout失敗を確認します。最終4workloadとaccuracy-onlyの実コマンド成功もあります。 |
| 回帰比較・書出し | `test_report.py:37`は同一truthのFP増加をFailed、truth違いをNot Applicableかつdeltasなしにします。最終実reportは同一truth回帰Passed、release Unknown。`:213`はJSON一致と既存出力への上書き拒否・内容保持をassert。実Markdownの全12行を読み、空セルundefined、holdout/configのprecision undefined／recall0、tuning/source_apiのFPR0.5を確認しました。 |
| Queue互換性 | Rust／Python両方で既存001〜040の存在、ID文字列・filename整合・一意性・依存解決を保持し、追加だけを許容。既存014／016 cancelled、023／001／037の受入条件をPythonが保持。040重複で実際にRust試験が失敗し、041訂正後の限定試験と全workspace成功が保存されており、一意性を緩めて通したものではありません。 |
| Self-scanの限定登録 | HEADの既存132 entriesとversionは現行先頭部分と完全一致。追加はreport.py108行36列の合成レシピ3所見のみ。raw1116=確認済み1088＋保持28。旧upload31件から登録full-result hash3件だけを除いたSARIF全体が最終uploadとdeepEqualです。source不一致の負例では3件とも残り31件、重複超過の負例ではcount超過の2件を残して30件です。既存Node31試験の最終成功も確認しました。 |

## 最終ソースと証拠の一致

- `EVIDENCE.json` のsourceBindings17件、evidenceBindings62件は現物SHA256と全一致しました。
- `QUALITY.json` は `report-accepted/quality.json` とdeepEqualです。
- reporter SHA256、baseline-final summary SHA256、25 rawBindingsはすべて現物と一致しました。
- 最終report.py: `63b798a59a7b51417b8282487a09f5dbe0f3f8a6bd75776e77300fd66f08e0f4`。
- 最終test_report.py: `174b8a72e072e9666681500dac8ec1d069cd04d664246c8cb3af58117e6e9173`。
- corpus／truth／evaluate.py／run.py、および`docs/quality/task023/`の既存2ファイルはbase HEADとbyte一致しました。
  製品Rust、rules、lock、schemasの変更はありません。
- 初回未実装6件失敗、review-fixのbinding欠落等7 failure＋未実施表示1 error、
  scope修正前の1 failure、それぞれのgreenログを読みました。最終の欠落membership試験は
  後からdigestを再計算する強化も含み、post-reviewで実行済みです。

## コマンドと主張の妥当性

提示ログを読み、`verified-commands.json`、`post-review-commands.json`、
`accepted-commands.json`と照合しました。

| 検証 | 結果と範囲 |
|---|---|
| pinned Rust build／fmt／workspace／Clippy | 全exit0。Rust1.93.1、workspace default、locked／offline。全workspace356成功（doctestを含む）。Clippy all-targets／-D warnings。Rust差分はtask-ID試験だけで、最終Python修正後もfmt check成功です。 |
| Quality Python | verified時30成功（旧report9）、最終post-review31成功（新report10）。最終ソースの10 method名がログにあります。 |
| Node／distribution | 最終Node31成功・skip0、distribution10成功。self-scan期待値追補後にもNode／qualityが成功しています。 |
| 実baseline／report | 反復3、各workload first＋warm3、raw測定を保存。最終reportとaccuracy-only生成はexit0。通常scan用製品schemaと数値CIゲートの変更はありません。 |
| Protocol | ローカルharness11件のうち5成功／6skip。厳格container・全feature試験を今回再実行したとは記載していません。変更前exact base HEADのhosted4 jobs successと、未push変更のhosted CI未実行を区別しています。変更はprotocol製品コードに触れていないため、今回の報告器受入に追加の全feature再試験は必須ではありません。 |

CI追加は既存baseline jobのrun.py生成物をreport.pyで読み、既存artifactへ保存する経路です。
不整合rawを評価基盤失敗にする一方、accuracyComparison Failedを新たな数値exit gateにはしていません。
過去baseline・過去2x調査履歴・既存受入を置換していません。
coverage／mutation／fuzz／Miri／Scorecard、新独立方式truth、real-product／multi-host／cold-cache性能、
変更worktreeのhosted CIと今回のbrowser確認は未実施と明記され、成功への転用はありません。
HTML変更がなく出力がJSON／Markdownのため、新しいブラウザー受入は今回の必須事項ではありません。

## 任意の試験強化（承認を妨げません）

- **P3** `tests/quality/test_report.py:123`: 現fixtureは1 split／1 family／1 ruleです。
  今後の変更時に、2 split・2 rule／familyと異なるlanguage／artifactを持つ小fixtureで、
  隣の層へ流入しないことと空セルのnullを永続的なregression testにすると堅くなります。
  今回は最終実データ全84行を独立照合済みで、現行受入の不足ではありません。
- **P3** `tests/quality/test_report.py:213`: Markdownのassertionは語・group名中心です。
  具体的な数値行とundefined行を固定assertすると表示順・式の取り違えも自動検出できます。
  今回は実Markdownの全12行を照合済みで、誤表示はありません。

必須修正要求: **なし**。本レビューはテストの妥当性承認であり、Evaluatorによる
最終台帳・タスク状態の更新は主エージェントに委ねます。
