# develop統合のテスト妥当性レビュー

判定: 承認。今回の統合要求を検証するテストと最終証拠に、修正を要する不足はありません。

対象: HEAD `784ceb1b63c0ef8a071aab9c7a9a054c10340136` と develop `4ee5ceb2ca6bab3cf3336fae17fb9f42f3446d65` の統合。両親差分、統合テスト、既存回帰テスト、検証コマンドのログを読みました。製品コード編集・追加委任・フルテスト再実行はしていません。読んだ主要テストは `test-review-source-bindings.json` にSHA256を記録しています。

## 要求と検証内容

- 公開予算: `develop_integration.rs` は既定予算でJS局所解析の40所見、明示2所見でTruncated、node/candidate/match/byteの小さい予算で所見なしのTruncatedを確認します。拡大予算では4,100所見と9,000空白の長いspanがCompleteになり、Python/Rustの4,100回目とJavaの長い引数も到達します。旧固定capの再導入を捕捉できる入力です。`red-budgets.log` は当初40所見のComplete期待に対してTruncatedで失敗しています。
- 境界と負例: TASK043の32/33所見は明示budgetで引き続きComplete/Truncatedを区別し、alias深さ8/9、AST深さ/候補、有限配列の境界も残します。Java/Python/Rustの欠けた構文は明示参照を保持してPartial、文字列中の類似表現・namesake・shadowing・unknown key・EC keyはRSA/PQCに推定しません。count・statusの両方を検証しています。
- 構文互換: 両検出経路に同じTSX引用属性、空type引数、deferred importを入れてCompleteと元行2の所見を確認しています。v0.2のinspection/semantic/context testsはmalformed対照、comments/stringsを変更しないこと、元座標、テンプレート限界も検証します。
- Ruby: 動的versionの旧期待値を単に緩和せず、固定名の1所見、Partial、`dep_version=unknown` と `dynamic_version_not_evaluated` を確認しています。v0.2専用Ruby経路のscope/rebinding/動的宣言の対照もworkspaceに含まれます。
- custom rules: builtinと同IDでもpattern/scopeを変更した場合の通常照合を実際のRuleSet経由で確認し、selectorテストは仮想方式のYAML変更がalias・回復経路に反映されることを検証しています。
- exports/privacy: CLIテストはstdout/stderrおよびHTML/Markdown/JSON/SARIF/CBOM/dependency-SBOM/diagnostics/native inventory全8ファイルのcanary不出力を確認します。Unicodeを含む行columnとnative byte_start/endを実ソースから計算して比較し、複数の署名参照を区別します。コピーしたruleをcheckout外cwdで使うテストも維持されています。これは形式内容の検証であり、今回の実ブラウザlayout観測ではありません。
- 品質評価器: 手計算TP/FP/FN/TN、ゼロ/除外母集団のnull、異なるtruth/inputの比較拒否、stale契約/レビューhash/重複/未解決/同一reviewerの拒否を確認します。通過ログだけから評価器の正しさを判断していません。
- 自己スキャン: 合成ソースfixtureは旧SARIF有限allowanceの単体検証に限定され、現在のsource hash不一致時には全観測を保持する対照テストがあります。自己スキャンの409 retained所見すべての安全性審査を今回完了したとは判定しません。

## 確認した実行証拠

`native-commands.json` のfmt check、default workspace、Clippy warnings-as-errors、all-features workspace、release workspace buildはすべてexit 0。ログ内集計はdefault 514 passed/0 failed/0 ignored、all-features 565 passed/0 failed/1 ignoredです。ignoredは明示隔離のOpenSSH interopで、すべてを実行したとは扱いません。今回の5統合テスト、28 TASK043 detectorテスト、2 CLIテストが実際に走っています。

Python品質54、Node35、distribution11の成功ログを確認しました。`protocol-kernel.log` はhost境界とDocker kernel境界を合わせ11件すべて成功・skipなしで、プロセス子孫/メモリ/PID/timeout/network/source/環境/scratch/archiveの対照を含みます。このログは任意環境での実行安全性の一般証明ではありません。

## 042適用範囲の独立確認

指定プロトコル、現在のCOVERAGEとpkcs12.rs、246単位の原ファイル/指定行/ラベルを確認しました。203入力の記録hashはすべて一致しました。旧P12/PFXはPFX v3外側data内の唯一のContentInfoがencryptedData OIDで平文の既知DERは0回、非暗号化対照はdata OIDで同DER1回です。変換来歴はnominal属性を説明しますが、暗号化contentを非復号の公開抽出対象にはしません。6属性単位をunsupported、残240を元scopeのまま維持する独立結果を `tests/quality/applicability-042-develop-integration/review-b.json` に記録しました。容器filename signalは属性抽出と区別し保持します。旧/他applicabilityレビューは読んでいません。過去測定を見たpost-observationレビューで、新blind truthでもbinary改善でもありません。

## 最終証拠

`final-measurement-commands.json` は5測定+5報告の10コマンドすべてexit 0です。各 `final/{name}/summary.json` と取り込み前 `before/target/integration-before/{name}/summary.json` のcorpusSha256・truthSha256・corpusInputおよび全split/family別のaccuracy groupsが一致しました。各3反復のsemantic/diagnostic/raw-order/scoreがすべてstable、各scanはexit 0・timeoutなしです。

| corpus | TP | FP | FN | TN | 取り込み前比較 |
|---|---:|---:|---:|---:|---|
| task023 | 28 | 4 | 2 | 19 | 全group一致 |
| expanded | 60 | 17 | 0 | 57 | 全group一致 |
| expanded-v2 nominal | 98 | 2 | 4 | 93 | 全group一致 |
| lock-controls | 2 | 0 | 0 | 0 | 全group一致 |
| task043 | 103 | 0 | 0 | 94 | 全group一致 |

expanded-v2の別supported scopeは98/2/0/91です。nominalの4 FNと2 negativeを範囲外にするsource-based correctionであり、取り込みによる検出改善とは解釈しません。元truthは維持されています。

5環境記録のruntimeSourceBindings各158件・harnessBindings各333件を現在の全記載ファイルと照合し、不一致0件、release binary SHA256も一致しました。5報告の全rawBindings、sourceSummarySha256、reporterSha256、ruleMetadataSha256を再計算し一致を確認しました（`test-review-measurement-checks.json`）。CI差分は旧042 manifestを新しい独立レビューに束縛したmanifestへ切り替える1引数です。最終Nodeログも35成功・0失敗・0skipです。

この承認は統合の回帰検証に対するものです。残るnominal FP/FN、部分検査、自己スキャン未審査所見、保留中v0.3品質基準が解消された、または新しいrelease判定が完了したとは示しません。


## 追加の性能切り分けレビュー

判定: 切り分けの測定方法と記録は妥当です。取り込み前比の増加は **Investigation Warning** として保持します。製品ソース変更や追加hard gateはなく、全suiteを再実行する理由はありません。

三者の記録は `before/target/integration-before-performance/summary.json`、`final/task023/summary.json`、`develop/target/integration-performance/summary.json` です。各small/medium/large/pathologicalのrecipe、入力ファイル数・byte数・SHA256が一致することを確認しました。各workloadはfirst-measured 1回とwarm 3回、全48サンプルがexit 0・timeoutなしです。同一macOS/arm64、release、threads=1、all形式+diagnosticsで、RSSは`wait4 per child`のbytesです。first回をwarm中央値に混ぜていません。4workload×3者のwall/cpu/RSS warm中央値を生サンプルから再計算し、`PERFORMANCE.json` と一致しました。これは管理されたcold-cache測定や統計的同等性試験ではありません。

退避binary・統合binary・develop binaryのSHA256を実ファイルと照合しました。各環境記録のruntime hashは対応するbefore134件・統合158件・develop148件のファイルと一致します。さらにdevelopのruntime全148ファイルと既存run.pyを `git show 4ee5ceb2ca6bab3cf3336fae17fb9f42f3446d65:<path>` の原バイトに照合し一致しました。archive内のgit rev-parseは外側repositoryのHEADを拾うため、上流ソース版の根拠はそのHEAD欄ではなく、この直接照合と別build/binary hashです。

大量一致の同一60,017-byte入力では、取り込み前の128所見・約1.00MB出力に対し、統合版は16,384所見・約122.25MB出力です。warm中央値はwall 0.02697→0.85015秒（31.52倍）、RSS 16,990,208→330,170,368 bytes（19.43倍、約16.20→314.88 MiB）です。この実測増加を性能非退行とは呼びません。

develop単体も同じ16,384所見、約122.25MB、wall中央値0.85008秒、RSS329,826,304 bytesとなり、統合/developのwall比1.00008・RSS比1.00104でした。取り込み前の規則別32所見制限と、v0.2の公開既定4096所見制限のソース差、および両v0.2系で同規模出力となることは、主な増加を公開予算/出力量の変更に切り分ける根拠です。出力は`detector_truncated`を保持し、完全文書検査とは扱いません。レンダラー単体や解析単体の時間寄与を測ったものではありません。

通常small/medium/largeの統合/develop wall比は0.995/1.100/1.001で、同時に所見数と出力サイズも異なります。取り込み前比は1.188/1.432/1.270であり、どちらの比較もこの固定workload・少数逐次サンプルに限定します。性能同等の一般保証や品質ゲート変更を導く証拠ではありません。
