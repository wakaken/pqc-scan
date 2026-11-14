# TASK-042 実装レビュー（初回）

担当: task042_implementation_review。追加の委任なし。判定: **修正後の再確認が必要**。

## ブラインド注釈

AGENTS.md / prompts/REVIEWER.md と追加コーパスの PROTOCOL.md / provenance.json、入力および指定された公開証明書manifestのみを参照し、実装・規則・測定・他者ラベルを見る前に156単位の labels-a.json を保存した。

- corpus SHA256: defcedb55fd4965d78e56bbcee8ee8c7ee5dccd83cf7d824d62f91060901b92b
- labels-a SHA256: 097bc595eb4c092d24a77f1040b9d03c330e79ed7f9e9adfdcafaac578c2e143
- positive 63 / negative 58 / unsupported 34 / ambiguous 1
- sourceOnly=true / otherLabelsSeen=false / scannerResultsSeen=false

保存後に実装レビューへ移行した。labels-b は開いていない。追加コーパスのscanner実行も行っていない。本人の注釈は以後変更しない。

## 指摘

1. **[P2] 裁定の resolution と最終注釈を常に一致させる** — tests/quality/taxonomy.py:64–68。
   現在は final がどちらか一人の注釈と一致すると resolution の整合確認を省略する。A=RSA、B=DSA、final=A の合成例で、裁定 resolution に negative/unknown という矛盾した値を置いても validate_annotations が受理することを確認した。理由だけの裁定も受理するため、最終ラベルと明示的裁定の結び付けが不完全である。不一致の全単位に正規化した完全な resolution を要求し、final と一致することを検証する。A/Bを採用する場合、第三案を採用する場合、resolution の欠落・矛盾を試験する。

2. **[P2] freeze とラベル記録の時刻を検証する** — tests/quality/taxonomy.py:89–95。
   timestampUtc は非空だけを確認し、レビュー記録側の時刻は存在すら要求しない。既存の合成 EvidenceTests.enrich() を用い、freeze.timestampUtc='not-a-timestamp' とラベル時刻なしでも load_truth が受理されることを確認した。freeze は初測定前に二者の記録を固定したという証拠であり、誤った時刻・freeze後の日付のラベルを受理すると監査可能な時間順序を満たさない。UTC日時を解析・検証し、各 reviewer.timestampUtc <= freeze.timestampUtc を要求する。現在の実データの独立性を否定する指摘ではなく、検証器の不足である。

## 確認した点

- evaluate.py の score() の照合・重複FP・half-open span・依存名照合は今回の差分で変わっていない。
- aggregate は既存の disposition を再集計する。複数方式を組合せセルにし、注釈のないFPを unattributed に保持する。空の語彙セルと null 分母は保持する。
- metadata catalog は119件・119個の一意ID。rule_coverage は全項目を残し、両側/片側/適格なしを区別する。現時点で未測定の規則を検証済みと扱っていない。
- corpus-dir は tests/quality 配下の実ディレクトリに制限され、既定は従来コーパス。CLI/製品の公開Schemaは変更しない。
- README両言語と開発ガイド・CIの追加経路を確認した。CIは同じrunner/reporterを再利用し、追加性能はskip-benchmarksによりNot Tested。上流1系統・選択者の規則既知・相関・初回測定の限界は文書化されている。
- ENTRY.jsonに対するhash確認: rules 8、schemas 14、crates 136（許可されたtask_ids.rsテストのみ除外）、旧corpus入力29件が不変。旧corpus.json / truth.json / labels-final-a.json / labels-final-c.json / PROTOCOL.mdも不変。report.py / test_report.py は許可された実装変更。
- 主担当の python-enhanced-fixed.log は39件成功。レビューでは上記2つの具体的懸念だけを合成例で再現し、フルスイートを重複実行していない。

## 未完了の範囲

二者目の注釈、裁定、truth freeze、追加コーパス初測定、最終9成果物、Rust/旧57単位回帰結果は主担当の進行待ち。このレビューは、それらが既に完成・合格したとの判定ではない。指摘修正と必要な試験の後、同じ担当で範囲を絞って再確認する。

## 修正確認（同じ実装レビュー担当）

上記2指摘を **解消** と判定し、現在の実装を **承認** する。新たな実装指摘なし。判定対象はコード・既定コーパス互換性と追加評価経路であり、これから固定・測定する実データや最終成果物の完成を先取りした承認ではない。

- 全不一致で decision.resolution == 正規化したfinalを要求するよう修正された。A/B採用時の裁定欠落と第三案の不一致を拒否する試験を確認した。algorithmsのlist型必須も追加された。
- utc_time() はUTC aware日時を要求する。各label <= freezeをvalidate_freezeが検証し、runnerはscanner起動前にfreeze <= measurementを確認する。reporterも同じ時系列を確認する。
- test_freeze_chronology_and_source_only_attestations_fail_closed の不正/naive時刻、freeze後のラベル、結果/他ラベル参照、protocol不一致、freeze前測定の拒否をソースで確認した。
- 正式な修正後ログ python-chronology.log の40件成功を確認した。初期のRed失敗は保存されたままで、成功に読み替えていない。レビュー担当はフルスイートを再実行せず主担当の結果を利用した。
- 主担当報告: Rust build/fmt/test/Clippy exit0、旧57単位 TP25 FP3 FN5 TN17 と041最新比較の全split/family差分0、Node source binding修正後31件成功。これらの全詳細の試験妥当性は別の試験レビュー担当が確認する。
- labels-a.jsonのSHA256は初回凍結値と一致し、書換えなし。labels-b未参照、追加コーパスscanner未実行の状態を維持した。

修正確認時のコードSHA256:
- tests/quality/evaluate.py: 4b341b0d4c8374615198d18a8ccf449de326f1df197ebc98a9994ddc8333351c
- tests/quality/taxonomy.py: e3c002d33b7879ea3403a33c95d67beadf41f7ff7670fe0af4527c78b7c85171
- tests/quality/run.py: 7d2733b06c8e329b3c762b992d3308d34e1715d9e541b1faf693888652bf5ce9
- tests/quality/report.py: 4f2896efd5d92ef147ae961b8bfeb639a39ebd1118ed8a4c091c6568a4443397
- tests/quality/test_taxonomy.py: 85985809142dfacbf3687c1a8b0c616acce8dc619b296c203825264a2bd40c3c
- tests/quality/test_report.py: 98d2d3faff0b46ffe92283ae855221ffdd645a7d04ead78552f75ee51eed2843

## 初測定後の限定確認

主担当の依頼により、最終truthの裁定・freeze metadata、taxonomy/reportの追加差分とpython-frozen.logを確認した。labels-b / dependency-scope-bの本文は開いていない。元labels-a / dependency-scope-aの書換えは行っていない。

- joseの3件についてAはnegativeを提案したが、PQC能力未確立の事実を保ったambiguous裁定は、主担当の不確実性保持として受容する。JOSE存在をPQC能力のpositiveに読み替える問題は解消している。固定前のsource-only補足2件がhashで束縛され、元記録とPROTOCOLも保持される。
- 元PROTOCOL/corpus/ラベルを結果に合わせて変更しない方針と、25件の裁定の完全resolution、freeze時刻を確認した。lineageAnalysisは同一bytes群と系統別ファイル数を残す。ruleCoverageのVerifiedは「正負truthが存在する」という限定的意味がreasonに明記されており、検出成功と誤認しない運用が必要である。
- python-frozen.logは42件成功。全スイートは再実行していない。

追加差分に1件の実装指摘:

**[P2] 補足の原ラベルhashを本人のreviewer identityに対応させる** — taxonomy.pyのsourceFactReviewBindings検証。
現在はoriginal_shaが2つのlabelSources値のいずれかであり、reviewerが既存2名のいずれかなら受理する。その結果、B補足がAの原ラベルhashを参照しても合成例でload_truthを通過した（原ラベルファイルを読替えず、合成EvidenceTests.enrichだけで再現）。reviewer→本人の原ラベルhashを対応させ、補足のoriginal_shaとの一致を要求する。既存のwrong_originalは無関係hashの拒否しか試験していないため、他方reviewerの有効hashを参照した負例を追加する。今回の実ファイルの不正を指摘しているのではなく、検証器の結び付け不足である。

測定解釈の受入条件:

固定注釈への参照一致評価として042-v1を保存することは可能である。tuningの10FPには、3件のnegativeへの誤報と7件のunattributed所見が含まれる。後者には正しい未注釈所見が含まれ得るため、precision/F1は製品全体の真のprecision/F1とは解釈できない。既存scorerの仕様を維持し、truthを測定後に書き換えず、043でtriageする判断は適切である。生成JSON/Markdownにもこの内訳と解釈を明示し、unattributed全件を製品不具合と扱わないことを受入条件とする。現在の初生成物はまだこの具体的説明を含まない。初測定とそのraw evidenceを不変に保持し、説明や検証器修正後の報告・実行は別パスに保存する。

## 最終差分の確認（継続）

- 補足のreviewer identity→本人の元ラベルSHA対応を実装で確認した。other_valid_original負例が修正前に失敗したcross-reviewer-red.logと、修正後python-final.logの43件成功を確認し、補足bindingのP2は解消と判定する。
- final quality.json / quality.md / metrics.json / comparison.jsonにPartial・negative対応3/重複0/未帰属7と、製品の誤検出とは断定しない説明が入った。TASK-042-RESULTS.mdの限定評価・相関・jose補足・042-v1を変更せず043でtriageする説明も確認した。
- 初回の保存snapshotについて、実ファイルhashを201harness / 125runtime全件で記録と照合し一致した。first-report-reproduction.jsonは初回quality全体の再生成一致を記録している。
- IMMUTABILITY.jsonのprotectedFiles198件をENTRY.jsonと実hashで再照合し差分なし。score() ASTは初回snapshotと現行で一致した。本人labels-a / dependency-scope-aのhashも元の凍結値と一致した。
- 自己scanの追加期待値56ファイルはexpanded配下と既存report.py/run.pyの合成benchmark文字列に限定されている。raw1223 / expected1195 / retained28の監査と正確なsource/result/count bindingを確認した。監査のexpectationsSha256はJSON正規化後のhashであり、byte hashとの差は不整合ではない。既存resultDigest(policy)によるc993ad0...とauditの一致を独立確認した。

生成ロジックに残る小さな意味上の指摘:

**[P2] 未帰属所見ゼロから注釈完全性をVerifiedにしない** — report.pyのannotationInterpretation.status。
現行の条件式はunattributedがゼロならVerifiedを返し、Markdownでは「注釈の完全性」と表示する。しかし予測がゼロでも未注釈事実はあり得る。今回のtruthは全nominated所見を網羅的に列挙したものではないため、注釈完全性は出力数とは独立にPartialとするか、状態名をFP帰属の状態だけに限定する必要がある。現在の実測7件未帰属の報告値は正しい。修正により元truthや初測定を更新する必要はない。

## 最終承認

**TASK-042の実装を承認する。未解消の実装指摘はない。**

最後のannotationInterpretation.statusは、出力件数と無関係にPartialを維持するよう修正された。ゼロ予測で未帰属0でも注釈完全性をVerifiedにしないtest_zero_predictions_cannot_verify_annotation_completenessを確認した。zero-prediction-red-reproduction.logに旧実装での失敗が保存され、python-accepted.logは修正後44件成功である。これにより、最後の意味上のP2も解消した。

最終結果の受入範囲は「凍結した有限注釈への名目的参照検出一致の測定と、その基盤・証拠・限界」である。未注釈所見7件を製品誤検出と断定せず、joseの不確実性、上流一系統の相関、未評価範囲、性能/hosted CI未実施を明示する。Q1〜Q6全体やリリース品質の合格とは扱わない。試験妥当性の最終判定は別の試験レビュー担当に委ねる。

レビュー担当は対象差分・生成物・必要なhash/具体的な拒否再現のみを確認し、scannerやフル試験を再実行していない。原labels-aと本人の補足を維持し、labels-b / dependency-scope-b本文は参照していない。

承認時ソースSHA256:
- tests/quality/evaluate.py: 4b341b0d4c8374615198d18a8ccf449de326f1df197ebc98a9994ddc8333351c
- tests/quality/taxonomy.py: 8c0535d26a815a8a908735edad5a0a928d22193f6408d3479d46794231401b2b
- tests/quality/run.py: 7d2733b06c8e329b3c762b992d3308d34e1715d9e541b1faf693888652bf5ce9
- tests/quality/report.py: 0d20b3c5ccc8d373861798c3cee77d43e0eb62464ea17d084eab137a80159051
- tests/quality/test_taxonomy.py: 91ded7dbc370d0d644e73b56a87980dad26934706af7126c84fa75989aad3382
- tests/quality/test_report.py: f243074f09e660ed95d6694e87d2d551725fe17eb3219c7f51f5df19ecde68c0
- docs/quality/TASK-042-RESULTS.md: e27ef82bf6be2084ab3c3287a23277c0a4f677ce4b064b78177c7821e73c4204

記録時刻UTC: 2026-11-11T10:23:53.636837+00:00
