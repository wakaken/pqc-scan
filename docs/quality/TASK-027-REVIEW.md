# Task 027 独立コードレビュー

2026-11-10、基準HEAD `2baac32cc60adfae5cb2aca957803aae0ae414c5`。

## 判定

**レビュー対象の実装は承認。未解決の重要なコード指摘はありません。**
両README・契約文書の最終更新とworkspace全体のEvaluator検証は親担当の作業として残ります。
本判定はremote CI・task完了・コミットの承認を意味しません。
実装・index・HEAD・task statusは変更していません。

## 対象と方法

AGENTS、Reviewer prompt、task027、spec/plan、DESIGN-REVIEW、両README、architecture、
知識文書とQUALITY-GATESを参照しました。追跡済diffと新規ファイルを確認し、特に
 dependency_detector.rs / decode.rs / parse.rs / lexical.rs、core observation.rs /
 inventory.rs / pipeline.rs、dependency_provenance.rs、CLI dependency_privacy.rsを確認しました。

初回は静的レビュー、修正後は静的な再レビューと以下の独立実行を行いました。

| 検証 | 結果 |
|---|---|
| `cargo test -p pqc_scan_detectors --test dependency_provenance` | exit 0、11 passed |
| `cargo test -p pqc_scan_cli --test dependency_privacy` | exit 0、1 passed |
| `git diff --check` | exit 0 |

CLI privacyはall形式、inventory、assessment、diagnostics、stdout/stderrを検査しています。
修正前の独立CLI実行では、JSON escaped registry locatorのsynthetic canaryが
report.json/report.md/report.htmlに残存することを再現しました。修正後は同じ経路を含む
CLI回帰テストの成功を独立実行で確認しました。fixtureは一時ディレクトリへ作成し、
終了時に削除しています。実鍵・実認証情報は使っていません。

## 初回指摘と解消

1. P1 動的versionのresolved昇格: 共通のresolved判定とnpm/Cargoのstrict semver判定を追加。
   1.x / 1.2.x / 1.+を除外し、recordを残しながら不完全性を報告します。回帰テストで確認。
2. P1 lock locatorのregistry誤認: npm v1/v2/v3とpnpmでgit/path/archiveを先に分類し、
   canonical npm registry以外のURLを解決済みregistry purlへ昇格させません。
   raw locatorをtyped facts/purlへ含めない分岐を確認しました。
3. P2 pnpm peer suffix: suffixをidentity分離前に処理し、5.4/6/9形式のrecordを保持。
   各形式についてopenssl@1.2.3と実際のYAML行を回帰検証しています。
4. P2 pnpm root declarations: rootとimporterの両方を処理し、specifiersを宣言として保持。
   宣言versionをresolvedに変換しません。
5. P2 誤ったsource行: 同名key/nameは最初の一致を流用せずunknownの0とします。
   ordinalとsource pathでorigin identityを分離し、異なる同file宣言のasset IDを確認。
   span parserを追加しない保守的な対応は本taskの範囲として妥当です。

## 再レビューで追加した指摘と解消

6. **P1 escaped locatorのsnippet漏洩**: raw textのmarker探索は
   `https:\u002f\u002fTOKEN_CANARY@private.invalid`を見逃し、publishConfig.registry等が
   近傍snippetへ出力されました。parentへ再現を送信し、依存manifest/SBOM全体を固定maskする
   小さな変更で解消。marker列挙とHashSet特例を削除し、JSON escapeやrecord外locatorも
   source snippetへ流れない構造になりました。escaped入力を追加したCLI test成功を確認。

## SBOM報告名保持の追補レビュー

親担当が全体比較で見つけたSPDX既存fixtureのrecord喪失への修正も独立確認しました。
Maven namespaceのない安全な報告名をpurlのecosystem validationで落とす代わりに、
name/versionのreported claimを保持し、ecosystemをunknown、purlを空、結果をPartialとします。
SPDXとCycloneDXの両経路に同じ保守的な扱いがあり、外部purlのidentityを報告名へ推測して
結合しません。unsafe nameの許容範囲は広げていません。
追加回帰を含むdependency_provenance 11テストを独立再実行し、全件成功を確認しました。
この追補修正について新たな重要指摘はなく、コード承認を維持します。

## 正しさ・互換性・限界

- detect/detailed/observedは同じinspectを使用します。manifestのexact指定やgo.mod/
  requirementsは宣言、SBOM versionはreported claimであり、lock resolutionと分離しています。
- default legacy adapterはtyped resolutionを信頼しません。strict v1の既存6 attributesを
  維持し、typed assetはscope/path/accepted ordinal/full factsによって分離します。
- 完全なsupported空input、malformed syntax、mixed invalid record、unsupported schema、
  heuristic NotReported、nontext input、byte/node/depth limitを異なる経路へ分類しています。
  JSON duplicate拒否、YAML alias拒否、fixed safe errorも確認しました。
- full external schema validation、universal secret detection、cross-file resolver、graph、
  runtime使用・installed packageの保証は行いません。heuristicはNotReportedのままです。
- 行位置は保守的です。曖昧なJSON/TOML/YAML keyやspanを取得できない構文は0であり、
  行移動・同名recordのsource ordinal変動は保証しません。docsに制約を明記する必要があります。
- legacy field shapeは保持していますが、purl/metadata/coverage/snippet内容は意図的に変わります。
  custom versioned-purl matchingとlegacy flat projectionの損失を両README/契約文書で説明してください。

## ソース束縛

再レビュー時の主要ファイルSHA-256:

```text
025f1746c50bab424875c089d7d0d93b5d627f10231b468b21b2b8c964cf17ce dependency_detector.rs
65ae5ff08d9fcfe9b4155906aa116ba1e32832cb3db32388dd08149c21e51559 dependency_detector/decode.rs
87f4e224991a951536c3c54b1c342575f46ea4106fd7d6ca962173e471c9a97e dependency_detector/parse.rs
a8ccd7b1b1908002e3a1d76e570425f58b0a5f43d9776a4dd7bc017b8aa1d98f dependency_detector/lexical.rs
d59910d905785dc9ecab19500b4001d0c535f3afc7b3e681ba23715aacb6591e core/observation.rs
e00846ebf8064c2e8ec68a9e0210b355167b130ce73a79ba47ebdf8db030c8cd core/inventory.rs
3e6c9cde00b03ac103dffe87e72f8810062bddc617273d8ad1c4e13b1a5a7a87 core/pipeline.rs
```

このレビュー自身ではworkspace build/fmt/test/clippy、frozen corpus、golden/schema比較、
performance、remote CIは実行していません。親担当/Evaluatorの記録と区別してください。
