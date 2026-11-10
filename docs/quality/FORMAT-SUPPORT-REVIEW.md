# 独立レビュー

対象: `codex/inspection-format-support`、比較基点 `e7f32bb`、2026-11-10の作業ツリー。
担当: 読み取り専用の独立Reviewer。製品コード・ドキュメント・タスク状態は変更していない。

## 判定

承認。確認した製品差分に未解消の具体的な指摘はない。

最終チェックはTS/TSX文法、bounded依存パーサー、SBOM identity/provenance、PKCS12抽出、coreのopaque source/snippet処理。workspace全体の検証、既存corpus比較、CI omission registry、最終commitは親が担当する。本承認は最終CIの成功を主張しない。

## 指摘と解消

1. P2: Gemfile/requirementsの制約を許可文字だけで受理し、不正な宣言をCompleteとした。`gem 'openssl', '>='`、`'garbage'`、`'1.2.3', '<'`、`openssl==1.*.*`で独立再現。制約構文の全消費検証が追加され、名前の検出を残しつつ非Completeになることを確認。
2. P2: SBOMのPURLだけに版があると、その唯一の版情報を捨てた。validated PURL版をreportedとして保持し、resolvedに昇格させない修正を確認。矛盾する明示版は非Complete。
3. P2: Goの`v1.2.3` PURL版を先頭数字チェックで拒否し、既存の名前を失い得た。Go用のv接頭辞+semver検証に修正。無版npm scopeのraw `@`も版区切りと区別する修正を確認。
4. 補足: POMの無視されたmixed text/未知namespaceがCompleteになった。非Complete化と回帰テストを確認。

## 確認した境界

- TS/TSX専用文法への切替のみでルール・ヒット制限・occurrence処理は変更しない。
- PKCS12はDataから公開X.509 cert bagのみを抽出し、暗号化・秘密鍵bagの値を再帰解析/ハッシュ/出力しない。
- ASN.1タグのclass/constructed/完全消費、入力/深さ/bag/attribute上限を確認。
- PKCS12の本文をtext detectorとsnippet再読込から除外。公開DER由来のoffsetとMAC/trust非検証を明示。
- malformed/unsupported混在時は安全な取得結果を保持し、Partial/Unsupported/Truncatedを適切に維持。
- 動的Go/Ruby/Python/Maven構文で非Completeを維持。SBOMはインストール/解決済みの証拠に昇格しない。
- 既存reportフィールド形状、ルール、identityアルゴリズムの変更なし。回収できた新しい事実による値/件数変更は機能の意図した範囲。

## 独立実行証跡

最終コマンド:

```text
cargo test --offline -p pqc_scan_detectors --test format_dependency_support --test typescript_grammar --test pkcs12_certificates --test dependency_provenance --test ast_occurrences --test certificate_attributes
```

exit 0、71件成功（AST 7、証明書属性20、依存provenance 11、形式依存15、PKCS12 12、TS 6）。

`git diff --check`もexit 0。

修正前には追加された不正version回帰テストが赤であることを独立確認した。ignored `target/format-support/review-harness`のDetector APIハーネスでも境界入力を再現。ネットワーク・package manager実行・スキャン対象ソースの実行はしていない。

## Freeze直前の追加差分レビュー

不正Go PURLの版があっても、独立して安全なnamespaced labelをtyped name validationで保持するfallbackを追加確認。PURLは必ず空にし、provided claim不正によりPartialを維持する。reported source assertionをresolvedに昇格させず、未知/危険なlabelは従来のvalidationで拒否する。追加の具体的指摘なし、承認を維持。

再実行: `cargo test --offline -p pqc_scan_detectors --test format_dependency_support --test dependency_provenance` exit 0、27件成功（16+11）。これにより形式依存テストは15件から16件になった。

## 最終source freeze境界

requirementsコメントの空白境界を追加レビュー。`openssl==1.2#junk`を不正のままPartialにし、安全なopensslの記録を保持する。先頭/空白直後のコメントはCompleteを維持し、Rubyの空白不要コメントは従来どおり扱う。具体的な新規指摘なし。

英日README/COVERAGE/SCANNINGとarchitecture/dependency provenanceの差分も確認。PKCS12非暗号化限定、秘密鍵opaque、MAC/trust非検証、バイナリ位置、TS/TSX名義照合の限界、bounded subsetの未対応構文の非Completeという説明に実装との重大な不一致なし。CI registryはこのレビュー時点では親による更新待ち。

再実行: `cargo test --offline -p pqc_scan_detectors --test format_dependency_support --test dependency_provenance` exit 0、28件成功（17+11）。承認を維持。

## 最終CI registry / fixtureレビュー

`SELF-SCAN-EXPECTATIONS.json`差分を独立確認。41件の再束縛は旧行と新行のソース本文が全件一致（core model 2、pipeline 32、certificate detector 7）。2件の退役は不正PEM→公開証明書置換とRuby JWTの実在行修正に対応。追加78entryは専用TS grammarテスト70entry/72結果、holdout typed fixture1、公開PEM/PFX4、Ruby実行3行目1、SPDX canonical identity2。新たなproduction observationの除外はない。ファイル単位やrule単位の一括除外は追加していない。

登録全ファイルのSHA一致を独立確認。既存のprepareCodeScanning APIをraw final SARIFに適用し、1008 raw =1004 expected+4 retained、全sourceChecks=verifiedを再現。残る4件は位置不明のholdout SBOM3件とindependent-holdout Cargo1件で、登録なしの観測は引き続き保持。

既存Nodeテスト25/28成功の回帰を独立再現し親に報告。その後`self-scan-backlog/REVIEWED-RESULTS.json`の6件再束縛を確認。130件/11filesは維持し、変更キーはlocationsのみでrule/message/properties/個数は全一致。証明書signature labelの旧521–526→新559–564に限定。helper/テスト論理は変更せず、remote alert履歴も変更していない。

独立再実行: `node --test shell/*.test.cjs` exit 0、28件成功。ログ: `target/format-support/independent-node-final.log`。

英日primary docsの限界説明と公開sampleの説明を再確認し、実装との重大な不一致なし。CI registryを含め承認を維持。

## 受入記録の最終評価

`FORMAT-SUPPORT-VALIDATION.md`と`format-support/COMPARISON.json`を独立確認。sourceBindings82件、fixtureBindings18件、sample3件のSHAは実ファイルと全一致。実行バイナリafterのSHAも`target/release/pqc-scan`と一致。Rustログの成功数合計262、Node28、Python20、fmt/release/clippy各成功ログと受入記録は整合。

before/after/policy各3repeats、同一corpus/truth SHA、source_api TP10→11/FN4→3/FP3不変、coverage 32/8/1→33/7/1、examples55完了/294所見、CI1008=1004+4/119verifiedの数値が記録と一致。性能は未承認の数値予算を合格基準に転用せず、hosted CI/他OS/外部OSS/画面検証等の未実施を明示しており、保証の過大表現を認めない。

最終CLIテストのschema追加を読み取り確認し、`cargo test --offline --locked -p pqc_scan_cli --test example_inspection`を独立実行してexit0を確認。9出力、55ファイルComplete、inventory/assessment/SARIF固定schemaの成功を確認。

軽微な記録改善として、CLIテスト参照の`crates/pqc_scan_report/tests/schema/sarif-schema-2.1.0.json`もfixtureBindingsへ追加することを親に提案。現状の製品挙動/検証結果に影響する問題ではない。製品差分と受入記録の承認を維持する。

## 最終注釈・schema binding追記

SARIF固定schemaのSHA追加を確認し、fixtureBindingsは19件すべて実ファイルと一致。sourceBindings82件も引き続き全一致。

`RULE-QUALITY-METADATA.json`のJSON差分を再帰比較した結果、変更は22件の`method_limits[1]`文言のみ（旧JavaScript grammar説明→専用TS/TSX grammarとsyntax error時のpartial説明）。ruleのID/pattern/risk/snapshot等の変更はなく、default rulesも基点から不変。追加されたqualityCompanionSha256も現ファイルと一致。`quality-metadata-final.log`で既存quality_metadata検証1件成功を確認した。

製品挙動を変える差分はなく、承認を維持する。
