# README再構成・利用ガイド分割の独立レビュー

2026-11-10。Reviewer Agentとして独立した文脈で確認しました。
判定: **承認**。未解決の重大な情報欠落・正確性・英日対応の問題はありません。
比較基準は `f7237f147ca51d413ae27bcf08d027dc152581db` の既存READMEです。
本Reviewerの編集はこのローカル記録のみです。過去のレビュー記録、製品、task状態、commit/pushは変更していません。

## 依頼との対応

- 両READMEから実装状況・開発者用品質baseline・開発互換仕様のsectionを除き、初めて使う人向けの説明へ整理しています。
- 主な機能より前に6種類の対象成果物を表で定義し、対応パターン・形式に限ること、live endpointsは対象外であることを併記しています。
- 成果物→機能→初回実行→結果→詳細ガイド→制約の順で読めます。
- PQC参照の説明は、結果を読んでriskの意味を理解した後に観測例と追加確認事項で説明します。
- CBOM/SBOMは出力を選ぶ文脈で定義し、依存ライブラリの記録とソース上の方式参照の例で役割を示しています。
- READMEとSCANNING/REPORTS/COVERAGEの英日すべてに目次と固定anchorがあり、言語間・READMEへの導線があります。

## 情報保持と実装の一致

利用者に必要なCLI既定値・必須option関係、rules list/show、絶対rules path、6既定出力と3任意出力、
SARIF指定でも2flat inventoryが出ること、英日表示の範囲、inspection診断、exit優先順位、
accepted-hit/work上限、証明書・鍵・dependency/SBOM対応範囲、custom rule互換注意、
旧native CLI削除、policyの前提とaggregate Pass不可、bounded privacyと除外範囲を保持しています。
開発者API/architecture説明、task状態、測定基盤の管理情報、旧画面写真の削除は依頼された整理の範囲です。

追加のriskラベル表はRule Risk語彙と一致します。PQCの名前と実稼働、依存関係と解決/配備、
certificate OIDと署名/信頼検証、static inspection Completeと移行Passを区別しています。
SBOM inputと未実装CycloneDX outputも混同していません。
新たな機能・runtime保証・精度保証は追加されていません。

英日で対象6種類、機能、初回手順、全9出力の条件、risk4種、PQC観測例、
証明書と依存の制約、診断/退出コード、privacyの意味が対応しています。
023の状態を変更せず、OSS自己scanだけで品質baselineのdoneやprotocol/budget承認を代替する説明もありません。

## 指摘と解決

移設前文脈の「上記SARIF／SARIF above」は、REPORTS内の固定anchorへ明示リンクする形で解消しました。
SARIF例の「SARIFのみ／SARIF only」は2一覧が追加される実装と矛盾しない表現へ修正されました。
未解決の必須修正はありません。
参考改善の日本語COVERAGEのrules-reference導線も反映され、英語版と揃いました。最終追補でリンク117件の参照先を再確認し、不在0でした。

## Reviewer自身による軽量検証

- 対象8文書のlocal Markdownリンク117件とanchorを独立確認し、不在0。
- 英日bash blockはREADME2、SCANNING5、REPORTS3の計10組で、コメントと空行を除くコマンドがすべて一致。コードfenceも整合。
- task029受け入れsourceBindings全76件をSHA-256で再計算し、すべて一致。
- `git diff --name-only -- tasks` は空でtask変更なし。
- `git diff --check` 成功。

外部scan、製品コード変更、Rust workspace test/build/clippy、browser、hosted CIは実行していません。
本承認は文書の構成と正確性に対するもので、製品全体の新たな品質認定ではありません。

## レビュー対象SHA-256

```json
{
  "README.md": "d94a7f96a5c05a8e2ac9c84a9d033ac80a58d7f8c66e73160bb92c92158febd1",
  "README.ja.md": "f47f71fba5fce7694a7f8f0cd2d7bb30f90a3c68bf41ffe6ad90d852b7ea68d8",
  "docs/user/COVERAGE.ja.md": "e6811de064ce9289292a5ffe5df37a8179d76d602489631b66a37328b76a7453",
  "docs/user/COVERAGE.md": "45fc3043a1b77cc2f5fc1f1231e6cbbc07f00be674b6ef363ad6f53e1f6eff5e",
  "docs/user/REPORTS.ja.md": "ecdb96a5d3cfaf503c0c078e9f37909366b50ee5541678f404a03d811a78f568",
  "docs/user/REPORTS.md": "10a868fa2317ae6aa39ff28fdc645e42944841eb0b869568b5d6e512aabd9138",
  "docs/user/SCANNING.ja.md": "357039907e530c9fc4fe7c31c985926d44253dab05a56cd5f38377937ba75edd",
  "docs/user/SCANNING.md": "a03f32127c87d7ca3d33b45363b72b69e96a6a21e8cc82b5a61ce0143fa789b8"
}
```

## Executor validation

The parent additionally exercised seven documented CLI selections against local `examples` using the accepted task029 release binary: default six reports, Japanese six reports, all opt-ins nine files, SARIF plus two inventories, strict completeness exit3 before finding threshold, finding threshold exit2, and removed native switch exit1 with no report directory. All expected outcomes matched; scans containing report.json retained 289 findings. Exact commands/outputs are saved under ignored `target/readme-restructure/{smoke.json,*.stdout,*.stderr}`. The documentation checker verified 117 local links, 59 anchors, matching bilingual Bash blocks, balanced fences, Japanese wording checks and unchanged 76 runtime source bindings. No product code, rule, schema or task state changed; no external OSS product was scanned. Product build/tests/clippy and hosted CI were not rerun for this documentation-only change.
