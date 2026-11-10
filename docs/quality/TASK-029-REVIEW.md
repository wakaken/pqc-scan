# Task029 独立コードレビュー（暫定）

- 基準HEAD: `4324d2c31e96ce62de211882192d6c4a8acded6d`
- 対象: 同HEAD上の未コミット作業差分。`privacy.rs`、pipelineの収集・ID確定・native関連付け・legacy返却、report境界、追加回帰テスト。
- 証拠区分: source/data-flow reviewのみ。Reviewer自身によるRust実行・exploit実行・新しい外部対象への走査は行っていません。
- 判定: 以下のP1を解決後、再レビューが必要です。

## P1: 構文を失ったpreviewへ認識済みcredential断片が残る

`privacy.rs:142-147` はsource contextがsensitiveでも `evidence.match` のみを無条件置換し、`snippet_preview` は構文認識だけに依存します。一方、既存RegexDetectorは `snippet_around(...,64)` を利用し、その内部の `mask_preview` は20文字超を先頭8文字・`***`・末尾6文字へ短縮します。長いpassword/token文字列を含む周辺previewは元の引用・代入構文を失うため、後段redact_textは断片を検知できません。

また、finding自身がcredentialの外にあるとsource sensitive_idsにも含まれません。RS256の直後の同一行に長い引用passwordを置けば、RS256のpreview末尾がpasswordの途中6文字になり得ます。private PEM bodyに一致したfindingでも、markerを含まない短縮previewは同様です。source snippetを安全にする処理は別フィールドであり、previewを保護しません。

修正案: source snapshotに基づき、安全なpreviewを短縮前に生成するか、credential/private contextと周辺範囲を使ってpreviewを保守的に置換してください。認識済みcredentialの後方から始まる部分一致、credential外のtechnical hit、PEM bodyの短縮previewを回帰に含めてください。canaryの先頭/末尾トークンだけでなく、短縮で残る中間断片も検証してください。

## P1: URL queryの通常のtoken値が非引用value文字集合から漏れる

`privacy.rs:40` の非引用valueは `[a-z0-9_+/=-]+` に限定され、次に指定された終端を要求します。従って `https://example.invalid/?token=LEAK029A.jwt` の`.`付きtokenは代入全体に一致せず、redact_textでもSourcePrivacyでも検知されません。HTTP queryにJWT様の値を載せるケースは不透明文字列の一般判定とは異なり、既に認識対象としている名前付きcredentialです。

修正案: HTTP queryの名前付きcredentialを独立して処理し、URL境界までの値を置換してください。通常のコード式 `const token = jwt.sign(...)` を保持する既存制約と両立できます。一般的なpercent-encodingを含むquery値も、対応範囲を明示して回帰に含めてください。

## 良好な点・確認できた範囲

- normalizationとnative association確定の後にlegacyをマスクしており、既存finding ID生成入力を変更していません。
- SourcePrivacyはpayload/hashを保持せず、行・文字/byte位置だけを保持します。
- CBOMの元の行数・順序を保ち、sensitive occurrenceだけで置換mapを作るため、同一行・別columnのbenign occurrenceが上書きして復元する経路は抑制されています。
- secret metadata valueとfreeform evidence/action/dependency文字列は共通のwriter防御に到達します。operator ID/pathの対象外扱いは設計と整合します。
- 固定のtyped diagnostic reasonとnative allowlistは新しいraw error出力経路を追加していません。

## 未実施・残件

最終README/architecture/契約・計測文書は更新中です。最終全体差分・上記修正・実行証跡・最終HEADの再レビューは未実施です。ここでのsource reviewはworkspace test、offline schema、saved binary shadow、実測性能、remote CIの成功を意味しません。

## 最終source/docs再レビュー

判定: **Approve（ソース・データフロー・契約文書のレビューとして承認）**。

同じ基準HEAD上の作業差分を再確認しました。先のP1 2件は解消しています。

- 認識済みprivate/credential contextを持つファイルでは全findingのsnippet_previewを固定マスクし、検出器による短縮で構文が失われても断片を公開しません。sensitiveなcaller-provided private_key evidenceのpreviewも固定マスクされます。
- `query_value`専用認識が、dot/percentを含む名前付きquery値を区切りまでマスクします。同じvalue位置をSourcePrivacyへ収集しています。
- `sensitive_match`はcredential valueから始まるmatchに加え、手前から交差する非短縮matchと、同一credential行の短縮matchを保守的にマスクします。
- native association用のlegacy bindingはpreviewをキーにしていません。ID確定後・native構築前のpreview置換による関連付け破壊はソース上認めません。
- CLI回帰にはRS256の直後に置く長いMID029繰返しcredential、dot/percent queryが加わり、両言語・9出力に対して先頭/末尾だけでなく中間MID029も不在を検査しています。

README英日、architectureリンク、`docs/architecture/EVIDENCE-PRIVACY.md`を確認しました。単一行quoted/bounded bare literal、prefix、query、private marker context、unknown locationの保守処理、caller結果がsource contextを持たない点、構造ID/path・opaque/encoded field namesが対象外である点が記載されています。万能なsecret scanning、秘密鍵の形式検証、検出ルール追加、PQC安全性を主張していません。

最終レビュー時点で重大なsource/docs残件は認めません。上記修正後の実行結果・性能測定・最終コミットの検証はEvaluator/親エージェントの独立した受け入れ証跡が必要です。本承認はRust実行、offline schema実行、saved-binary比較、remote CI実行をReviewerが実施したという意味ではありません。productionコード・task status・commitはReviewerから変更していません。

## CI self-scan expectation registry追加レビュー

判定: **Approve（限定されたexpectation差分）**。

`docs/quality/SELF-SCAN-EXPECTATIONS.json`を基準HEADと比較し、以下を独立確認しました。product/scanner実行は行わず、JSON・source・git旧sourceの読み取り比較のみです。

- version/top-level構造は同じで、削除されたファイル登録はありません。
- 新規登録はprivacy回帰の4ファイルだけです。CLI:20件、core privacy cfg(test):31件、report integration test:21件、core integration test:1件、計73結果（69登録行。重複countを含む）です。`new-ci-registrations.json`と登録内容は一致します。
- 新規登録が指す全source行を読み取りました。いずれも合成fixture、期待値assert、metadataの数値・アルゴリズム名、またはコード例の文字列であり、productionの暗号操作を登録していません。privacy.rsの対象行は369行のcfg(test)以降です。
- 既存登録の変更はpipeline.rsの32行、report/lib.rsの3行、計35登録行のline/digest再束縛だけです。ruleId/column/countとreasonは同一で、old/new source行の本文は全件同一でした。`registry-rebind.json`を旧登録へ適用すると現在の既存登録と完全一致します。
- 変更/追加された6ファイルすべてのファイルSHA-256は現在のsource bytesと一致します。他の既存ファイル登録は変更されていません。

このレビューは、登録範囲を広げてproduction findingを隠す変更がないことのsource-bound確認です。親エージェントが提示したraw930=expected926+retained4および修正前binaryとのSARIF一致は、このReviewer自身によるscanner再実行として扱いません。CI分類結果の実行受け入れ証跡は親/Evaluatorの記録を参照してください。

## 既存evidence fragment保護の最終差分

判定: **Approve（source-only）**。

追加の`evidence_field`と3呼出箇所を確認しました。match・snippet_preview・source snippet lineに限り、旧writerと同じcase-insensitive `PRIVATE KEY` substring保護を先行させ、そのほかは共通redactionへ委譲しています。既存マスク文字列は同substringを持たず、反復適用でも安全です。新たに保護するmetadata/action自由文にはこの広いsubstring判定を適用しておらず、意図しない自然文の全面消去を増やしません。既存unitへのfragment assertも確認しました。重大残件はありません。

この追加によってprivacy.rsの行番号・ファイルdigestは前回registry確認時から変わっています。最終CI再束縛と実行検証は親/Evaluatorの最終証跡を用いてください。
