# TASK-043 保守性改善：実装レビュー

判定：実装は承認します。修正必須の指摘はありません。これは実装段階の判定であり、タスク全体の完了判定ではありません。主担当による最終必須試験・固定コーパス比較と、別の試験妥当性レビューは引き続き必要です。

## 対象と基準

- 基点HEAD：`0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。既存dirty変更を含む `ENTRY.json` / `before/` と現在との差分 `CHANGE.patch` を基準としました。既存041/042/043実装全体を今回の変更とみなしていません。
- 計画：`docs/quality/TASK-043-MAINTAINABILITY-PLAN.md`。
- 七つのコード／試験ファイルの現在のSHA-256は、レビュー時の `CHANGES.json` のafter値とすべて一致しました。
- 過去の入力・正解・品質測定を変更する修正、規則YAML／公開型／Schema／依存関係の変更は今回のコード差分にありません。

## 確認した事項

1. **正本の集約**：六規則のpattern/scope/kindの複製を除き、既定YAMLの `include_str!` と一度だけ初期化するカタログへ集約しています。Rustに残る規則IDとhandler・言語の対応は、既存の意味解析を選ぶための実装識別です。`DEFAULTS` のRuleは定義比較にだけ用い、未compileのpatternを検出へ渡していません。
2. **カスタム規則の互換性**：ID/pattern/scope/kindが一致する場合だけhandlerへbindします。変更されたカスタムpattern/scope/kindは `rule_matches` / `replaces` の両方で除外され、既存の汎用matcherへ戻ります。説明文などの編集は引き続きbindを維持します。
3. **方式・APIの選択**：Nodeの有限文字列値とPythonの正規化した呼出し・import・構文回復は、実際にロードしてcompileしたpatternを参照します。Nodeの生成APIの名前、trusted module、スコープ／alias／鍵や署名の意味は必要な言語/APIアダプターとして維持されています。完全な正規化prefixの一致を要求するため、埋込引用符や方式名の部分一致で陽性を作りません。
4. **証拠範囲と秘密情報**：Rust側の `Facts::call` を `Facts::callee` へ置き換えた二箇所は、変更前と同じfunction末尾を使います。他のcallは引き続き第一引数末尾までです。共通層の規則ID分岐が消え、長さ・件数・深さ・候補数の既存上限およびマスキング経路は変更されていません。
5. **実行時移植性とビルド**：埋込YAMLはビルド入力であり、実行時にリポジトリ内の絶対／相対パスを読みません。現行のフルcheckoutを使うworkspace/path installの前提に合致します。crates.io単独公開を新たな対応保証とはしていません。
6. **文書**：両READMEと規則リファレンスの説明は、規則ファイルが引き続き実行時に必要なこと、内部bindingの正本だけをビルドへ取り込むこと、変更カスタム規則が汎用経路へ戻ることと整合します。

## 参照した検証証拠

- `detector-final.log`：主担当の検出crate試験。新規selector/binding試験、既存TASK-043の28試験、位置・上限・scopeなどの既存関連試験が成功しています。
- `cli-portability.log`：主担当のCLI統合試験2件が成功しています。コピーした規則と入力を別cwdから使う試験、および全出力・native位置・秘密canaryを検査する既存試験を読み合わせました。
- `selectors_tests.rs` は変更テストYAMLを直接アダプターへ渡す制御試験で、改変カスタムYAMLを本番bindingへ自動許可するという主張ではありません。bindingの除外テストと役割が分かれています。
- 全試験の再実行はしていません。具体的な再現を必要とする実装懸念がなかったため、主担当のログとソースを参照しました。最終的な試験十分性は第2段階のレビュー対象です。

## ソース版

| 対象 | SHA-256 |
|---|---|
| `crates/pqc_scan_cli/tests/task043_source_facts.rs` | `48e595a6defcb7fcbe77de023ceb077b67e8e320b3f3f54f37f820124aa97a53` |
| `crates/pqc_scan_detectors/src/source_facts/bindings.rs` | `2ca7f017d3f08b12e578f2f4d2377e261497f7bc637b633eb575749238823df0` |
| `crates/pqc_scan_detectors/src/source_facts/javascript.rs` | `41b1f92591c8a3ab97445eabe7724b5d7d728230ff6cc5d9c37d5a7e476e9fa0` |
| `crates/pqc_scan_detectors/src/source_facts/python.rs` | `e486e0f4bc40e74d9c8351e06b78a0e14b36c83073d9b1734a75228664edf5ed` |
| `crates/pqc_scan_detectors/src/source_facts/rust.rs` | `891495b773fca0b6a67334bb4688c16640f5261bd14e8e05be04ecb288018d96` |
| `crates/pqc_scan_detectors/src/source_facts/selectors_tests.rs` | `a664d6cdc4cdc9a56c3a9f157b3795b861178ea745279c2ba5aa962a0984727a` |
| `crates/pqc_scan_detectors/src/source_facts.rs` | `0d5f76691a6f697a0cddab928344732a24a332a19fae68ebc651ad809aa3b2f0` |

## 最終packetの再確認（同一実装レビュー担当）

判定：実装承認を維持します。最終combined結果に修正必須の矛盾はありません。冒頭の「最終必須試験・固定コーパス比較・試験妥当性レビューが必要」という初回時点の留保について、以下の最終証拠が揃ったことを確認しました。タスク状態と最終受入は主担当が反映します。

- `docs/quality/TASK-043-MAINTAINABILITY-RESULTS.md` と保存packetを読み合わせ、今回の保守性改善・同じ固定入力での前後不変性と、一般的な精度・性能保証や未知APIの自動解析を区別していることを確認しました。
- 七変更ファイルのhashは初回承認時の値のままです。`FROZEN.json` の356ファイル、`SOURCE-MATCH.json` の現runtime/harness binding、実releaseバイナリのhashも独立に読み取り照合し、不一致はありませんでした。
- 最終8コマンドは終了0です。`RAW-EVIDENCE.tar.gz` 内のworkspace原ログから成功件数を再集計して390件を確認し、quality54件・protocol11件中6skip・distribution10件・Node31件の原ログと報告が整合しています。protocol未実行6件を実行成功に数えていません。
- `COMPARISON.json` の五固定コーパスは各版三反復、採点・所見・位置・ID・診断・順序が一致しています。Stage B担当の独立raw digest/binding照合は `TEST-REVIEW-BINDINGS.json` でerrors=[]、`TEST-REVIEW.md` は承認です。実装レビュー側で同じ全raw検証や全suiteを重複実行していません。
- `PERFORMANCE.json` のwarm最大wall/RSS比とRESULTSの丸めた値は整合し、既存2倍調査信号はありません。単一ホスト・合成負荷の制約も明記されています。
- 追記直前の `EVIDENCE-INDEX.json` にある16成果物は、実ファイルのhashおよびbytesとすべて一致しました。本レビュー追記後は主担当がindexを再生成する必要があります。

現releaseバイナリ：`15bd9bbc85f0d20c5799aa2a953e5aa7ed3169ba8b983607547f8ad43352a243`。
現runtime source digest：`6706ba8d708b47f55a8805b5698b8110c2ce556413fd259482da28f8ea9f9d2f`。
