# タスク029 独立Evaluator受け入れ

2026-11-10。判定: **承認**。既存writerの広い `PRIVATE KEY` 断片保護を維持する
最終修正後、下記のソース束縛・出力比較・限定テストをすべて再検証しました。
task029の限定された認識構文・合成fixtureに対する受け入れ条件を満たしています。
Evaluator権限で `review → done`、ownerを `Evaluator Agent` に変更しました。

対象HEADは `4324d2c31e96ce62de211882192d6c4a8acded6d`、branchは
`codex/scanner-quality-improvements`。実装はこのHEAD上の未コミット差分です。
最終ソースは [COMPARISON.json](task029/COMPARISON.json) のsourceBindingsで識別します。
最終privacy.rsのSHA-256は
`5505c9ab854b1038d13d4c492f32b2234efa1edde07cfcf179af1348d27a49e8` です。
製品編集・外部操作・commitはEvaluatorから行っていません。

## 独立実行・再照合

- sourceBindings全76件を現在のファイルSHA-256と再照合し一致。保存旧binaryと
  新release binaryのSHA、3profileのfullArtifact SHAも一致しました。
- `cargo test --offline --locked -p pqc_scan_core --lib privacy::tests`:
  exit0、5件成功。
- `cargo test --offline --locked -p pqc_scan_core -p pqc_scan_report -p pqc_scan_cli --test evidence_privacy`:
  exit0、CLI1件・core3件・report1件の計5件成功。
  認識済み6marker、credential/userinfo/query、英日9出力、schema、machine payload、
  ID/native association、source snapshot、固定診断、caller non-mutationを確認する
  既存の新規回帰を独立実行しました。
- 保存された自己スキャン結果を独立比較し、930件のFinding ID配列、summary、
  CBOM行数、SARIF全体が旧新版で一致しました。inventory/policyの追加出力の有無でも
  generated_atを除くlegacy JSONが一致しました。
  `target/task029/evaluator-comparison.json` に再照合結果を保存しています。
- `node shell/prepare-code-scanning.cjs --input target/task029/self-after/report.sarif --expectations docs/quality/SELF-SCAN-EXPECTATIONS.json --root . --output target/task029/evaluator-projected.sarif --audit target/task029/evaluator-ci-audit.json`:
  exit0、raw930 = expected926 + retained4。生成auditは親のauditと完全一致。
  sourceChecks117件も現在のsource SHAと独立照合しました。expectations digestは
  生成実装どおりcanonical JSONに対する値であり、ファイルraw bytesのhashではありません。
- 固定corpusのbefore/after/policyについてinput bindings、分類別accuracy groups、
  各profile内のsemantic/diagnostic/raw-order/score repeatabilityを照合して一致。
  これらの測定そのものは親の実行であり、Evaluatorは反復していません。

## 親の実行証跡を確認した項目

`target/task029/verification.json` と各log、最終COMPARISONを照合しました。
debug/release workspace build、fmt check、workspace test、clippy、Python、Node、
3profile corpus測定はいずれも記録exit0でlog内容も整合しています。
Rustの各test-resultを集計して224件、Node28件、Python20件の成功を確認しました。
フルworkspaceテストや性能測定をEvaluator自身が再実行したという意味ではありません。

## 設計・ソース・文書の評価

独立Reviewerの最終source/docs承認とCI expectation差分承認を確認しました。
最終追加のevidence_fieldはmatch/preview/snippetに既存の広いfragment保護を残す
限定差分であり、追加unit assertionとReviewerの追補承認を確認しました。
当初のpreview短縮とdot/percent queryのP1指摘は修正と回帰で解決しています。
元snapshotの位置だけを収集し、ID/native binding確定後にlegacyをマスクする順序、
CBOMの行順・件数とmasked優先、writer cloneへの共通policyを確認しました。
ルールpattern・公開model/schema・CLIの変更はなく、技術値・ID/pathの保持と
free textの意図した変更を区別しています。README英日・privacy契約・証拠文書の
限界説明は実装の限定範囲と整合します。

## 残る限界・未実施

remote CI、他OS、外部製品/実データの探索、ブラウザー実機操作、push/PR/mergeは
未実施です。HTML templateの変更はありません。数値性能予算は未承認であり、
task023の予算gateや1.0 release gateの達成を意味しません。
未知/難読化/任意の秘密名、複数行credentialの位置追跡、structural fieldsの秘密、
元contextのないcaller body-only値は万能に検出しません。本承認は認識済み構文と
実際に通した合成経路に限り、全秘密情報の不存在を保証しません。

Evaluator出力は `target/task029/EVALUATION.md` にも保存しています。
