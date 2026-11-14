# TASK-042 試験妥当性レビュー

担当: task042_test_review（この担当のみ。再委任なし）。最終判定: **承認**（TASK-042の評価基盤・有限データの検証範囲）。未解消の指摘なし。

## 独立した正解の確認

最初に AGENTS.md / prompts/REVIEWER.md、追加 PROTOCOL.md / corpus.json / provenance.json と全入力、指定された公開証明書manifestだけを読み、全141入力のSHAを確認した。規則・scanner実装・結果・他者ラベルを見る前に156単位を独立注釈した。

- labels-b SHA256: a90ad40e58089d5dcacfbdfa769f3d6e58414dee4d417bc316a0fe80939c19c7
- 保存時刻: 2026-11-11T10:00:59.772543Z
- 元の分類: positive63 / negative58 / unsupported34 / ambiguous1
- 元記録は以後不変。規則や結果から正解を作成していない。

joseの事実前提について、元ラベル保存後・初回測定前に公式の固定版READMEとkey-generationソース、CIRCL公式READMEだけを独立確認し、dependency-scope-b.jsonを追記した。PQC-capableという事実的な問いにはjoseの能力未確立をambiguousとする提案である。原protocolのrecognized集合presenceをpositiveとした元記録は変更しない。補足SHA256: ef5ad01fb7f5b0175f571cd4c34833a95a6013a7c673a390f560acee204f6412。裁定は両補足をhashで結び、対象3単位をambiguousとした。

## 試験の内容と証拠

単に成功ログを見るだけでなく、test_evaluate.py / test_taxonomy.py / test_report.py / test_run.pyと対応する実装を確認した。

- 独立した手計算でTP/FP/FN/TN、重複予測、同一行の別呼出し、half-open境界、誤ったrule、依存名の完全一致、FPRの負例単位母数とFP所見数の相違を試験している。
- 方式の組合せセルと帰属不能FPを分離し、ambiguous/unsupportedを適格母数から除き、分母0をnullとして保持する。
- 注釈欠落・重複・未知語彙、不一致裁定のresolution欠落/不整合、原ラベル・protocolの改変、sourceOnly等のattestation、UTC時系列とfreeze前測定の拒否を確認した。
- source/runtime/harness/inputのhash、raw report/diagnostics/measurementと集約値の一致、失敗・timeout・coverage矛盾・反復安定性の偽装を拒否する。
- 任意corpus-dirと既定57単位の分離、symlink/逸脱拒否、全catalog規則と空セルの表示、overwrite拒否を確認した。
- 実コーパスは固定truth読込み・全入力整合・旧57単位保持を試験する。42件成功のpython-frozen.logを確認した。フルスイートは重複実行していない。
- checks-initial/commands.jsonのrelease build、fmt check、workspace test、Clippyはいずれもexit0。初期Python失敗は保存され、修正後ログと区別されている。node-bound.logは31成功・失敗0・skip0。
- legacy-regression-02の3回は意味・診断・採点が安定し、TP25 FP3 FN5 TN17。legacy-report-02の同一入力比較は全split/familyのTP/FP/FN差0。

## 初回追加データと測定の意味

FREEZE.jsonは2026-11-11T10:05:03.121198+00:00、expanded-firstのenvironment clockは10:05:27.307048+00:00。元ラベル・補足・裁定が先、scanner測定が後という順序を確認した。固定truthは60P/58N/4A/34Uで156単位、適格118。141ファイル読込成功、94analyzed / 26partial / 21unknown。未列挙母数unknownを保持する。

- tuning: TP36 FP10 FN13 TN50、適格102。
- holdout: TP0 FP0 FN11 TN5、適格16。
- out-of-scope33。3回安定。
- 119規則は17Verified / 5Partial / 97Not Tested。Verifiedは正負単位がある意味であり精度合格ではない。
- 方式・用途・artifact・input format・caseの各集約の合計がすべてTP36 FP10 FN24 TN55と一致した。
- 7追加JSONとquality.json/mdの9ファイルが生成されている。

FP10はprotocolの閉世界計数で、3件は注釈済みnegativeへの誤報、7件はunattributed。後者には真のRSA署名が未注釈のためFP扱いとなった所見が含まれる。従って製品誤報10件や母集団精度とは判断できず、注釈完全性と精度解釈はPartialである。v1と初回結果を変更せず保持し、将来の043で所見と注釈不足を分ける方針は適切。上流holdoutは選択者が規則を知る一つの相関系統であり、独立母集団標本とは扱わない。追加性能・実運用・暗号安全性・release合否はこの試験で検証していない。

## 指摘

**[P2] 補足reviewerと原ラベルSHAの対応を試験する。** taxonomy.validate_freezeはoriginal_shaが原ラベルSHAのどちらかに含まれるかだけを検証するため、B補足を有効なA原ラベルSHAへ取り違えても受理する。現在の実データは正しく、原記録/truthの変更は不要。reviewer identityから本人の原ラベルSHAを求め、本人との一致を要求する。既存wrong_original試験は0*64だけなので、他方の有効SHAを入れた拒否試験を追加する。ファイルを変更しないin-memory substitutionでこの受理経路を確認した。

最終判定は上記修正、最終ソース対応ログ、自己scan・immutable確認・実行記録・限界文書の確認後に同じ担当で追記する。検証器のhash変更と初回測定時のharness hash差も明示する必要がある。

## 最終確認と承認

同じ試験レビュー担当として指摘修正と最終証拠を確認した。今回の有限データに対する評価基盤と試験は受入条件を確認するのに十分であり、**承認**する。これは検出性能の合格、注釈完全性、暗号安全性、全119規則の検証完了、リリース受入の承認ではない。

- P2は解消した。reviewer_originalsで本人の原記録SHAを要求し、other_valid_original（他人の有効SHA）を拒否する。cross-reviewer-red.logの当該失敗、修正後の成功を確認した。本人の原ラベルと補足は不変。
- 実装レビュー側で追加された、帰属不能予測0を根拠に注釈完全性Verifiedとしない修正も確認した。ゼロ予測の試験はsource-independentな合成raw/測定値を整合させ、完全性Partialとundefined precisionを検証する。保存された修正前コードでの単独Red再現と、最終Python44件成功を確認した。predictionAttributionStatusと完全性statusを区別する現実装は妥当。
- 最終のexpanded-accepted / expanded-report-acceptedは同じ凍結truthを使い、accuracy全体がexpanded-firstと完全一致する。全split/family差分0、3回安定、3 negative prediction / 0 duplicate positive / 7 unattributed、注釈完全性Partialを維持している。新規完全性試験は製品スキャンを実行しない。
- docs/quality/task042/MEASUREMENT.jsonを読み、128件のartifactBindingsと全command log SHAを実ファイルで照合し、不一致0を確認した。埋め込まれたexpandedAssessmentは最終quality.jsonと完全一致する。記録のRust356件は実workspace logのpassed合計356と一致し、build/fmt/test/Clippyのexit0、Python44件・Node31件と整合する。
- 最終測定のPython harness bindingは現ソースと一致する。accepted-source-snapshotのharness/runtime全件も記録hashに一致する。初回snapshotはharness201/runtime125件の全hashを独立照合済みで、first-report-reproduction.jsonに初回レポート全体の再生成一致が残っている。初回/中間/最終は異なる出力先に保存され、上書きしていない。
- IMMUTABILITY.jsonの保護対象198件をENTRYのSHAと実バイトで照合し、製品・規則・Schema・旧57単位/旧証拠の不変を確認した。score関数のASTも初回snapshotと同一。FREEZEのtruth・両原ラベル・両補足は全hash一致。
- 自己scanはraw1223 = expected1195 + retained28。登録56ファイル・113新規所見分は、今回の不活性入力/metadataと既存benchmark文字列の厳密な登録である。ソースhashは現ファイルと一致する。アップロード用projectionを精度測定へ利用していない。現expectationsのfile SHAとcanonical JSON SHAを別に照合し、auditが後者を使用することも確認した。従来の製品・依存所見28件は保持する。
- TASK-042-RESULTS.mdの正解数・適格母数・実測・由来・時系列・未帰属/相関/未評価の限定は最終成果物と一致する。hosted CI、追加性能、実プロジェクト/他OS/暗号安全性はNot Testedまたは別評価のまま。ここで未試験を成功に読み替えていない。

レビュー担当は新しいscanner実行やフルスイートの重複実行を行っていない。具体的な取り違え懸念のみをin-memoryで確認し、主担当の修正後試験と実際のバイト/hashを根拠に承認した。最終のレビューhashは測定記録へ後付けせず、別のREVIEW-ACCEPTANCE記録に束縛する方針を確認した。
