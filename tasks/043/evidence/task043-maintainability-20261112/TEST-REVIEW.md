# TASK-043 保守性改善：試験妥当性レビュー

判定：今回の保守性改善と既存挙動維持を確認する試験として承認します。修正必須の試験不足はありません。製品全体の品質ゲート達成や一般的な型推論の完成を意味しません。

## 対象と確認方法

- 基点HEADは `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。既存dirty変更を含む `ENTRY.json` / `before/` に対する七ファイルの `CHANGE.patch` と `CHANGES.json`、計画 `docs/quality/TASK-043-MAINTAINABILITY-PLAN.md` を対象としました。
- 試験ソースの入力とassertion、変更された本番呼出し経路を読み合わせ、主担当の最終ログを確認しました。全試験の再実行はしていません。
- 独立の読み取りスクリプト `review-bindings.py` を実行し、終了0を確認しました。結果は `TEST-REVIEW-BINDINGS.json`。コーパスや検体の実行、ネットワーク通信、鍵生成、probeは行っていません。

## 変更の検証力

| 変更・要件 | 実際の検証と評価 |
|---|---|
| YAMLのpattern/scope/kindを正本にする | `bindings::tests::installed_rule_copies_bind_but_modified_semantics_do_not` は実際の規則をロードして六handlerすべてがbindすること、pattern/scope/kind/idを個別に変えるとbindしないこと、説明文変更はbindを維持することをassertしています。単なる成功ログではなく、汎用matcherへ戻す判断点を検証しています。 |
| Node/Pythonの方式再列挙を除く | 新selector試験は、製品の既定一覧にない `future-kem` / `FuturePrivateKey` を一時YAMLへ入れてRuleSetでcompileし、実際の言語アダプターへ渡します。Nodeはimport aliasと有限ループの二呼出し、Pythonはimport aliasと不完全構文回復のimport＋呼出しをassertし、従来方式をselectorから外す陰性対照もあります。 |
| 誤った部分一致を避ける | Nodeの引用符＋未知式を含む文字列二種、方式名末尾への余分な文字について0件をassertしています。完全prefix一致の防護を検証しています。 |
| カスタム規則の意味を維持する | 既存28件の中に、同じ規則IDの変更patternで文字列を汎用検出する統合試験、同じpatternで変更scopeを指定するとJava文字列へ汎用一致する統合試験があります。kind変更は上記binding単体試験で判断点を網羅しています。 |
| 必要な言語/API解析を維持する | 既存28件はalias、スコープ・再束縛・shadowing、有限ループの未知値や書換え、Rust鍵role、RSA戻り値由来、help文字列、構文回復とPartial、深さ・件数・候補上限とTruncatedを陽性・陰性で検証しています。 |
| 共通call/calleeの範囲を維持する | CLI統合試験はUnicode前置きを含むRSA生成・署名の位置を文字単位columnとbyte_start/endでassertし、第一引数を越える鍵オプションを含めないこと、全八出力とstdout/stderrから秘密canaryの先頭・中間・末尾が消えることをassertしています。Rustのcalleeへの二箇所変更は旧function末尾と同じであり、五コーパスの全report同値でも確認しています。 |
| checkout内の実行時パスに依存しない | CLI移植性試験は、三規則ファイルをtempへコピーし、checkout外のcwd・入力・規則ディレクトリで実バイナリを起動し、Node alias1件とPython alias2件をassertしています。ソースでもYAMLは `include_str!` のビルド入力であり実行時読み出しではありません。 |

selector試験は本番bindingを意図的に迂回する制御試験です。カスタムYAMLが本番で意味解析へ自動bindすることは主張していません。binding単体試験、カスタム規則の汎用matcher統合試験、実CLI既定規則試験との組合せにより役割を分けています。

`red-selectors.log` では、旧選択実装に対してNode/Pythonが実際に0件対期待2件で失敗しています。これはcompile失敗ではありません。最終 `check-workspace.log` にはbinding1件とselector3件が成功しており、中間 `green-selectors.log` にだけ存在した削除済みテストを最終証拠へ数えていません。

## 最終実行の証拠

`CHECKS.json` の八コマンドはすべて終了0です。主担当のログから確認した内容は以下です。

- `cargo fmt --all -- --check`、`cargo test --workspace --offline --locked`（合計390件成功）、`cargo clippy --workspace --all-targets --offline --locked -- -D warnings`、`cargo build --workspace --release --offline --locked`。
- Python quality54件、distribution10件、protocol11件中5件実行成功・6件skip、Node31件成功。protocolのskipをoptional featureの検証済みとは扱いません。今回の変更は静的検出器内でありprotocol機能は変更していません。
- `cli-portability.log` の2件は最終workspaceでも再度成功しています。既存TASK-043の28件も最終workspaceで成功しています。

`FINAL-MEASUREMENT-COMMANDS.json` の五コーパス測定・各レポートと、既存ベンチマークの前後測定・各レポートはすべて終了0です。測定器は、ファイル入力hashを確認してmaterializeし、読み取り数・診断coverageの整合を確認し、各scanの非0終了やtimeout、秘密canaryの残存をエラーにする実装です。

## 前後比較とソースへの結合

独立スクリプトで、五組の `before/target/{name}/summary.json` と `final-{name}/summary.json` を照合しました。

| コーパス | 入力ファイル数 | 採点detail件数 | 前後比較 |
|---|---:|---:|---|
| 043-v2 | 206 | 197 | 同値 |
| 042-v2 | 203 | 197 | 同値 |
| legacy expanded | 141 | 134 | 同値 |
| lock controls | 2 | 2 | 同値 |
| original | 41 | 53 | 同値 |

各組について、corpus/truth hash、materializeした全入力binding、accuracy全体（groups・個別detail・範囲外予測）、harness bindingが同じことを確認しました。各三反復のraw `report.json` / `diagnostics.json` からsemanticDigest/orderDigest/diagnosticDigestを再計算してsummaryと一致し、さらに全前後反復で同じことを確認しました。正規化は生成時刻と既知の証明書評価時刻だけを除き、所見・ID・位置・分類・証拠・順序・診断を保持しています。

- 七変更ファイルの現在hashは `CHANGES.json` のafter値と一致します。
- 前後各測定のruntime source bindingとharness bindingは、それぞれ実ファイルの現在hash／保存before hashとすべて一致します。
- 最終測定バイナリSHA-256は `15bd9bbc85f0d20c5799aa2a953e5aa7ed3169ba8b983607547f8ad43352a243`、保存beforeは `cfb12f4b8751915b9c2a5d23be5e83e872358704e20e0b19d3273236aa4c288f`。各summary値と実ファイルが一致します。

## 判定の限界

既存originalコーパスの過去基準に対するFailedや、Partial / Not Testedの品質軸は解消したと扱いません。今回確認したのはその同じ入力・正解・評価器に対する前後不変性です。既存ベンチマークは単一ホストの記述測定であり、性能保証・cold cache・多ホスト・実製品評価ではありません。YAML埋込方式のcrates.io単独公開は今回の保証範囲外であり、既存のworkspace/path install前提と整合しています。これらは本変更に必要な試験不足ではありません。

## 最終保存packetの受入（同一試験レビュー担当による追記）

`docs/quality/task043-maintainability-20261112/` の最終packetと `TASK-043-MAINTAINABILITY-RESULTS.md` を確認し、本変更の受入証拠として承認します。数値・対象範囲・既存の制約・実装レビューの承認と、試験妥当性判断の間に矛盾はありません。

- CHECKS／VALIDATIONの件数・終了値、COMPARISONの五組同値、SOURCE-MATCHの現ソース・評価器・実バイナリhashが、初回レビュー済み証拠と一致しました。FROZENの356ファイルも現在hashと一致しました。
- PERFORMANCEの四負荷×wall/RSSについて、前後summaryのwarmDistributionから原値・最大比・2倍調査信号を独立照合しました。最大wall比は約1.01/1.00/1.06/1.23で、wall/RSSとも既存調査信号なしという記載に合致します。これは新しい性能保証や品質ゲートではありません。
- 保存した両レビュー・CHECKS・CHANGES・独立照合JSONはtarget原本と同一でした。SOURCE-OVERLAYS archive内の全通常ファイルは対応する保存before／現ソースと一致し、RAW-EVIDENCE archive内のbefore/after測定・原出力の全通常ファイルも、既にレビューした原本と一致しました。archiveを展開・検体実行することなくstreamで内容hashを照合しました。
- 結果文書の043-v2 TP103/FP0/FN0/TN94・適格197と255単位の区別、protocol5実行・6skip、既存品質のFailed／未測定を維持する境界は整合しています。

今回の追記ではsuiteを再実行していません。最終EVIDENCE-INDEXは、この追記を保存した後に主担当が再生成し全件照合する手順とします。
