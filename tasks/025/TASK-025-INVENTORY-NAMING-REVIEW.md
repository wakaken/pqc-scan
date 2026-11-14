# task025 名称変更 独立レビュー

レビュー日: 2026-11-10。基準HEAD: `7e4d0ce226fed69fe6573462aa63dde02a71425f`。
対象: 基準から作業ツリーへの差分と新規契約・計画文書。

## 判定

承認。P0/P1/P2の指摘なし。名称・CLI・ファイル名の変更と既存互換性は、以下のレビュー対象スナップショットで要件を満たす。CI期待値の最終再bindも確認済み。

## 確認内容

- `--inventory-json` のみは新ファイル、`--native-json` のみは旧ファイルを選択する。両指定は1回の `scan_repository_native` の結果を共通serializerに渡し、同一内容を両ファイルへ出力する。
- 旧 `write_native_json` のシグネチャ・旧filenameを保持し、新 `write_inventory_json` を追加している。captureが無い場合の既存エラーも維持する。
- `--repository-scope` はmultiple=trueの引数グループに依存し、新旧どちらの指定でも利用できる。capture無しは拒否する。
- opt-in以外のscan/report/exit判定を変更していない。core/detectors/rules、JSON serializer、payload fields、識別子生成、schema URI/pathは変更しない。schema差分はtitleのみ。
- 新flagで既存semantic goldenとschema検証を実施し、従来レポートとのshadow比較を維持している。新旧単独/両指定とexit 0/2/3の組合せをテストしている。
- 英日READMEは新名称・flag・filenameと互換性を説明する。新contractへ従来の制約・ID・プライバシー・損失説明を保持し、旧contract pathはstubで残す。歴史的task025 evidenceを変更していない。
- AGENTS.md、prompts/REVIEWER.md、README両言語、architectureおよびknowledge documentsを確認した。RESTART-LOOPの適用範囲はrestart baseline/task014に限定される。

## 独立実行

- `cargo test -p pqc_scan_cli --test native_json`: exit 0、4 tests passed。
- `git diff --check`: exit 0。

## 未検証・境界

- workspace全体のbuild/fmt/test/clippy、Node/Python、実self-scanとCI期待値再bindの最終証跡は親agentが実行中であり、本レビューの独立実行成功には含めていない。
- remote CI/push/PRは依頼範囲外で未実行。
- 既存helpのtry_parse経路は変更しない。テストはstdout/stderr双方から新flag・名称を確認する。既存status1の挙動を新規契約として固定しないため、statusの厳密assertは追加しない方針を親agentと確認した。

## CI期待値・実artifactの独立追認

Nodeのassertで以下を独立確認した（exit 0）。台帳更新は既存2files/37groupsに限定され、登録path/reason/rule/column/count・files数・各results数が旧HEADと一致する。対象結果行の本文は旧HEADと同一で、いずれも実際のcfg(test)モジュール内にある。対象2filesのSHA-256は現在のソースと一致する。したがって行移動およびその完全result digest再bindだけで、新規の除外許可は無い。

`prepareCodeScanning` を現在台帳と親が取得した実SARIFに対して独立再実行し、保存済み `code-scanning.sarif` との完全一致をassertした。結果は **814 raw = 660 omitted + 154 retained、99/99 source verified**。両flag出力の `scan.inventory.v1.json` と `scan.native.v1.json` は独立byte比較でも一致した。

親が提供した `legacy-shadow.json` は読了し、旧default/旧native/新default/新native/新inventory/新両指定の814件一致とlegacy各artifact比較の証跡を確認した。これらの6scanそのものを独立再実行したという主張ではない。

## 承認するソース条件

以下は本独立レビュー時点の変更対象SHA-256である。親が追加する検証結果文書・本レビューのコピー以外にこれらの内容変更がある場合は再レビューする。最終workspace checksの実行・成功確認とlocal commit、remote未実行の管理は親agentが担当する。

- `README.ja.md`: `4041cc17223ffec248458ebccfca6b448420f22bc16375d16bc1756ea7240c24`
- `README.md`: `127a32b7059bea657e6d24918c48bcc164befebb6b1552dc6a4ef9357740f369`
- `crates/pqc_scan_cli/src/main.rs`: `92a207fb374752d4f3fba6e4b450f3f6b89bb5650c32bc18406e7d0fa888a614`
- `crates/pqc_scan_cli/tests/native_json.rs`: `5ee6e2f8deeb392bda17a5fd978d1072890cbb103c76d9eb000d497736e9a1c8`
- `crates/pqc_scan_report/src/lib.rs`: `05de91af4e4157ed05ad112ba961db9f2c87a3bd42bdb446d780a429bfda61b3`
- `docs/architecture.md`: `369f0887aa912628a8e7a8295879f2592b3badaaadb2a3a8274aabd71e3e133c`
- `docs/architecture/DOMAIN-MODEL.md`: `cd28ff580dcffc06f09df4bd87aa76e2013e8301056b8850fdd9ec4406547931`
- `docs/architecture/NATIVE-SCAN-V1.md`: `018786f0d6c2b484c0ba6f85077053d5f54b7ce52e8f8784e9af2b89c30a254b`
- `docs/architecture/NEXT-GEN-ARCHITECTURE.md`: `b2ddf15e1240ecd8b935da90c32c9d15cbaeaed45539060da5a1f2f1ce99ee42`
- `docs/quality/SELF-SCAN-EXPECTATIONS.json`: `52d2bcdb8f55067d24d855c3a1072d60321296afc4b6866d6f4c44f8321147e7`
- `schemas/native-scan-v1.schema.json`: `9de66059f1704141c5e24b7d590770053cbce4bf16598a0da0bc7cc4130ea8f0`
- `docs/architecture/INVENTORY-REPORT-V1.md`: `324ba98953421fc5eef1c5c22dcf211d88185e1b8863244338278c9a359851f2`
- `docs/superpowers/plans/2026-11-10-inventory-report-name.md`: `18147d3ca37b673d87051e5fc2c417406fcf0497871b45ef399d06f4ab389966`

## 最終証跡の追認

`tasks/025/TASK-025-INVENTORY-NAMING.md`、同配下 `evidence/task025/INVENTORY-NAMING.json`、完了チェックと実行結果リンクを追記した計画を確認し、承認を維持する。レビュー対象の実装・schema・README・契約・registry・testのSHA-256は前回承認時から不変である。機械記録のruntimeSourceBindings全件も現在ファイルのSHA-256と一致する。

親agentのログを確認し、Rustテスト合計177、Python20、Node22の全成功、およびverification.jsonのbuild/fmt/test/clippy/Python/Nodeのexit 0を確認した。これらは親実行の証跡確認であり、独立再実行とは区別する。最終証跡は互換性、raw/filtered partition、未実行のremote CI・性能再測定の境界を正しく記載している。local commitとclean-tree確認は親がこの追認後に行う。

今回の追記前のレビューコピーと、機械記録 `independentReviewSourceSha256` の一致も確認済み。本追記をコピーへ反映する場合は、そのhashも追従更新すること。
