# Task 027 最終評価

2026-11-10、Evaluator Agent。**承認。task027をreviewからdoneへ更新しました。**
基準/現在HEAD: `2baac32cc60adfae5cb2aca957803aae0ae414c5`。
評価対象はこのHEADに対する未コミットworktreeです。コミット・push・PR・remote CIは実施していません。
依存task025はdoneであることを確認しました。

## 評価基準と範囲

prompts/EVALUATOR.md、AGENTS.md、QUALITY-GATES、task027 acceptance、設計仕様・計画を適用。
RESTART-LOOPは本文どおりtask014のpilot限定です。
最終実装・文書、独立コードレビュー、比較・資源測定、source-bound CI監査を検証しました。
この評価で編集したのはtask027 YAMLのstatus/ownerと本記録のみです。

## 最終証拠の独立検証

- COMPARISON.jsonのruntimeSources 43件、binary 2件、schema 2件のSHA-256を現在の実ファイルから再計算し全一致。
- environment.runtimeSourceBindingsのruntime/test 71件とartifactDigests 4件も全一致。
- 保存before binaryのSHAはtask026のafterBinarySha256と一致。
- verification.jsonの最終10 commandは全exit0であり、対応する最終logを確認。
  debug/release workspace build、fmt、workspace test、clippy warnings denied、
  Node/Python、before/after/policyの測定を含みます。
- Rust最終logのpassed件数を独立集計して201件。Nodeは22件、Pythonは20件の成功をlogで確認。
  初期red/fix途中logを最終結果に算入していません。
- CI監査はprepareCodeScanningをメモリ内で独立再実行し、保存auditのcounts/sourceChecks/
  omissions/expectations digestと完全一致。856 raw = 659 reviewed omissions + 197 retained。
  sourceChecks 99件は全て現在の実ファイルSHAと一致。
- 現default/inventory/policyのreport.jsonを生成時刻のみ除外して独立比較し一致。
- frozen corpusのbefore/after/policy 3 profileのsemantic/diagnostic/score repeatabilityが全true。
  corpus/rule/schemaの変更がないことをdiffで確認。比較記録の808 unchanged / 47 dependency
  changed / 1 added / 1 removedは自己scan結果856件と整合し、意図的差分の説明があります。
- 164 source-only rebind、6 Cargo位置補正、消失Maven expectation 1件削除は比較記録と
  CI監査で追跡可能。新たなblanket suppressionの追加はありません。

## Evaluator自身の実行

| command / 監査 | 結果 |
|---|---|
| `cargo test --offline --locked -p pqc_scan_detectors --test dependency_provenance` | exit0、11 passed |
| `cargo test --offline --locked -p pqc_scan_cli --test dependency_privacy` | exit0、1 passed |
| `cargo fmt --all --check` | exit0 |
| `git diff --check` | exit0 |
| Pythonによるsource/binary/schema/artifact hash・最終log・件数・default/opt-in比較 | exit0 |
| Node prepareCodeScanningによる保存SARIF/現在policy/rootの独立監査再計算 | exit0 |

全workspace build/test/clippy/測定の再実行は行わず、保存された最終logと現在source/binary hash
の一致を検証しました。これとEvaluatorの上記独立実行を区別しています。
探索的なprepare-code-scanningの`--help`呼出しは未対応引数としてexit1でしたが、
実際の監査APIを読み、正しいin-memory再計算で成功を確認しています。

## Acceptanceと指摘の解消

1. manifest ^1.2.3、exact manifest、go.mod/requirements、SBOM claimはresolvedと分離。
   支持されるconcrete registry lock recordのみversioned purlを生成します。
2. source path/accepted ordinal/full six factsのtyped assertion IDによって宣言/lock/同file origin
   を分離。strict v1の6 attributesとlegacy flat shapeは維持し、legacy projection損失を説明。
3. valid empty、malformed、partial、unsupported、NotReported、limitsを区別。
   heuristic/unsupportedが空でも誤ってCompleteとなる設計ではありません。
4. 初期設計レビューのsnippet locator漏出、parse前/中資源制限、同file assertion分離は解消。
   escaped locatorを含む実CLI privacy回帰もEvaluator自身が成功を確認。
5. 独立コードレビューは承認済み。最終文書のshrinkwrap誤記、ID不変の過大説明、
   非依存findingのsnippet/preview変更の説明も修正済み。

未解決の重要指摘、human decisionを要する追加schema/rule/暗号設計変更はありません。
現在の限定されたacceptanceを満たすため、Evaluatorとしてdoneにします。

## 限界

remote CI、外部製品scan、他OS/architecture、browser/UI、package managerとの完全同値、
registry trust/実インストール状態、数値budget承認は未実施です。
local build/test/CI前処理監査の成功をremote CI成功とは扱いません。
nativeはnot_exhaustive、lexicalはNotReported、best-effort line0/ordinal変化、
filewide snippet maskingによる文脈損失、保守的なprivate registry/SBOM namespace処理を残します。
仕様文書で明示され、本taskの完了を阻害しない範囲です。
