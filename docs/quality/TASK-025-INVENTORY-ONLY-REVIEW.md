# task025 CLI単一化 独立レビュー

基準HEAD: `9a3934c2f49756592a8ee9361955bf411719197d`。レビュー日: 2026-11-10。
ユーザー追加指示による `--native-json` 廃止を前提とする。旧CLI互換性の維持はこの点で明示的に上書きされている。

## 判定

実装・テスト・現行ドキュメントは承認。P0/P1/P2の指摘なし。最終CI台帳再bindと検証結果文書は親agentからの後続依頼で追認する。

## 確認内容

- main.rsから旧CLIフィールド・引数グループ・旧writer import/出力分岐を削除した。captureは新flagのみで1回行い、scopeは `requires = inventory_json` で制限する。
- 引数解析はscan_cmd呼び出し前なので旧flag単独・新旧併用を副作用前に拒否する。テストはunknown argumentを確認し、新規out-dir自体が生成されないことを確認する。
- CLI testは新flagのみでsemantic golden/schema、秘密payload非出力、言語非依存、公開証明書・依存関係・危険package識別子除外を検証する。終了コード0/2/3と完全性優先順、scope省略・必須capture、新filenameと旧filename非生成、helpの新flagのみ表示も確認する。
- reporting libraryの旧writerは変更していない。core、JSON/schema/ID、rules/detectors/既定legacy出力は変更しない。現在README・契約は廃止と移行先を明記し、既存出力ファイルを削除しない点も説明する。
- historical proposal内の旧flag表記は、冒頭注記で歴史的/internal名称であることを示しており、過去の検証記録は書き換えない。
- 先行レビューからAGENTS.md・REVIEWER promptおよび必須knowledge文書の確認を引き継いだ。

## 独立実行

- `cargo test --offline -p pqc_scan_cli --test native_json`: exit 0、4 passed。
- `git diff --check`: exit 0。

## 未実行・境界

全workspace build/fmt/test/clippy、Python/Node、self-scan/CI台帳検証の最終実行は親agent担当であり、この独立実行には含めない。remote CI/commit/push/PRは実行しない。

## レビュー対象SHA-256

- `README.ja.md`: `31f537504c89c720f1eb0003569938cdcf5969209321c1c243efdfce2f3ee27b`
- `README.md`: `92b077b56d048724b9188158c940bcc5f4d11dcb0e9ea3434da0c41f148f07d0`
- `crates/pqc_scan_cli/src/main.rs`: `e902a863f9f577dca04f0e1e76514cd20051c4ea02b04e8aa95658c9a034a852`
- `crates/pqc_scan_cli/tests/native_json.rs`: `0037b4cf655e0eca3c864da027a40d7ac5a2d699ea365447688758f1116234dd`
- `docs/architecture.md`: `b269bc7bb02f015b8fc9a35ea4c8cb44d0ca2beb1a7c2b4196b2621de138d84b`
- `docs/architecture/DOMAIN-MODEL.md`: `be96d546b1e7742b5ee833d11eed9a1982fbacc0c4d26543116c626982c9f05c`
- `docs/architecture/INVENTORY-REPORT-V1.md`: `99d1e8a73174afb14076704f7c7e3d95367ab4763d368f22c1e8b09818a5bc9f`
- `docs/architecture/NEXT-GEN-ARCHITECTURE.md`: `ea3ef0f4af298dfa2a6b3b913c47f3f487fe40750c4e85774846fdcbaf0a4370`

## 最終CI・検証証跡の追認

承認を維持する。P0/P1/P2指摘なし。前回記録した本体・テスト・現行docsのSHA-256は不変である。

Nodeの独立assertで旧/current期待値を比較し、main.rs 1fileの既存34groupsのみ変更、files/results数・path/reason/ruleId/column/count不変、対象行の本文が旧HEADと同一かつcfg(test)内、現source SHA一致を確認した。新規除外許可は0である。registry-rebind.jsonとregistry-line-map.jsonの34件の位置対応も確認した。

実SARIFに `prepareCodeScanning` を独立再実行し、保存されたfiltered SARIFとの完全一致、およびauditのpartition/sourceChecks一致をassertした。結果は **814 raw = 660 omitted + 154 retained、99/99 sources verified**。

親agentの `legacy-shadow.json` を確認し、旧/newのdefault/inventory計4profilesが814 findings、既存6reports+diagnosticsが生成時刻のみ除去して一致、inventory payloadがrun/time/observation ID参照正規化後一致する記録を確認した。この4scan自体は独立再実行していない。

`verification.json` の全7項目exit 0と対応ログを確認した。Rustテスト成功数を集計して177、Python20、Node22成功を確認した。これら全workspace検証は親実行の証跡確認であり、本reviewの独立再実行とは区別する。`git diff --check` も再度exit 0である。

最終の短い検証結果文書とlocal commit/clean-tree確認は親agentが実施する。remote CI/push/PRは未実行のままである。

最終registry SHA-256: `523bf70c2237fe8ae042f037fa0d63688ebe13ddbb9ccc2bfe07aa2cfd721645`。
