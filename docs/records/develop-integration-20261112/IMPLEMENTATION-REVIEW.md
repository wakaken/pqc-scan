# develop統合の実装レビュー

対象: HEAD `784ceb1b63c0ef8a071aab9c7a9a054c10340136` と origin/develop `4ee5ceb2ca6bab3cf3336fae17fb9f42f3446d65` の統合作業ツリー。両親との差分を照合し、TASK-042/043とv0.2契約の共存を確認した。レビュー担当は製品コードを変更していない。

## 指摘と対応

- [P2・修正済みソース確認] Factsの固定4,096件・8,192byte制限、およびJava/Python回復経路の2,000文字切断、JS/Python/Rustの固定4,096候補制限がv0.2の公開DetectorBudgetと矛盾する。組込みfactsがgeneric経路を置換するため、長いJava引数等でv0.2が扱う入力を取り落とす。主担当は蓄積候補・byte数とexamined数を公開budgetへ接続し、Java/Pythonの切断もbudget対応へ変更した。最終テスト結果の確認は下記。

## その他の確認

- source_factsとregex/ASTが同じsource_treeでparse-only互換viewと元位置を共有する。custom pattern/scope/kindは既定YAML一致時だけのbindingを外れて従来generic経路を維持する。
- Ruby Gemfile/gemspecの重複lexical parserを除去し、heuristic経路も公開v0.2のruby::inspectへ統一する。動的versionでもliteral dependency名を保持しunknown/partialとする仕様を優先する。
- 既定ルールmetadataはTASK-043のpattern変更とv0.2の境界説明を併存させ、historical SELF-SCAN ledgerをcurrent-source bindingへ再解釈しない。synthetic unit controlsと実ソース不一致時に抑制しない検査は役割が分離されている。
- expanded-v2の元corpus/truth・歴史測定を上書きしない。適用範囲レビューはreview-a.jsonへ独立記録し、六属性unitの公開scope外判断と残り240 unitのscope維持を元source/契約から確認した。全203 source hash一致。旧P12/PFX内側encryptedData OIDと非暗号化controlのdata OIDを独立構造解析で確認。既存ファイル名シグナルは維持する。

## 検証状況

修正後の `integration-tests-5.log` は develop_integration の5テスト成功を記録し、元の40件既定/2件明示上限、ゼロwork予算、TSX/TS/mjs両経路、拡大5,000件予算の4,100件と9,000空白span、Javaの3,000空白およびPython/Rust最終4,101行の検出を確認できる。`integration-tests-4.log` ではTASK-043の28テストが成功した。`native-commands.json` のcargo fmt --all --checkはexit 0。主担当のworkspace/clippy等の最終検証は別途継続中であり、このレビューで成功を代言しない。

判定: 実装レビュー承認。指摘した公開budget不整合は最終ソースと対応する焦点テストで解消を確認した。追加の実装修正要求なし。古い失敗ログやbinaryに基づく測定を最終ソースの合格根拠としない。全テスト再実行はレビュー担当として重複実施しない。

## 最終ソース結合

- `crates/pqc_scan_detectors/src/source_facts.rs`: `2d3395efce86784d31d94e497b9bc7b92e2537d7a232088238bd2b4f0c1f6c0c`
- `crates/pqc_scan_detectors/src/source_facts/context.rs`: `71a930443c96c0ed0941e5b081f365c38ca374ad2fdb1dab263b57da0f8212f3`
- `crates/pqc_scan_detectors/src/source_facts/javascript.rs`: `d4eea104d0d330d8dac4f6a1673f2aa38cef83102ac59301eb667d1b32e02608`
- `crates/pqc_scan_detectors/src/source_facts/python.rs`: `ccf88cc6d18cbaffd99d9f4f754f62370ca5298e390327ad0f7776aeb33c37ea`
- `crates/pqc_scan_detectors/src/source_facts/rust.rs`: `0e8d57d38fe519bcbf72fc946c8e721433c513fbf3d24c7b08d7ab76584380f4`
- `crates/pqc_scan_detectors/tests/develop_integration.rs`: `0c98f9744b68e3dca11d49f397959cf5b2276d31c295ed3dd2d4514d08ba6b80`
