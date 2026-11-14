# TASK-042 再開後の試験妥当性レビューB

判定: **承認**。当初の9受入条件に必要な検証について、未達の必須条件・解消を要する試験不足は見つかりませんでした。これは有限な評価の実施と妥当性への承認です。検出器のFP/FN解消、全119規則の精度保証、暗号学的正しさ、母集団性能、製品全体やリリースの合格を意味しません。

担当: `q42_reopen_test_review`（試験レビューB）。委譲なし。対象HEADは`0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`、未コミット差分を含みます。2026-11-12にレビューしました。実装レビューAの最終追確認、正解レビュー、最終試験結果、実ソース、生成物を別々に確認しています。

## 対象版と証拠

| 対象 | SHA-256 |
|---|---|
| `tests/quality/report.py` | `00eaf7993453a1d29a2a3c4e47b23af942ad975d5befe5d0cecb6542ba4fbdbf` |
| `tests/quality/test_report.py` | `bac8f3d3e452a747b708eeee1b157581a413d0566bad74d02202b0cb5a1c9c31` |
| `tests/quality/taxonomy.py` | `be7a90ad270ba5ede8986091322764cf2f8ea78ee36e9a97706efd75f6bb8858` |
| v2 corpus | `f16a9bdc2b2dbd5852059ff8ea4d9727093c537c620eed8491ec94de86740b5b` |
| v2 truth | `e976696f5f71c965cb48b50e8bad065cbefb6119ce0b46fa73823a6d3c4f42a0` |
| lock補遺 corpus | `55d668218071458baadd4857f8bac3043b1f5a4b48e583a0a08722b580cd5f73` |
| lock補遺 truth | `a3c909060509b4d7f57544f3f854d3c3fe13bbbb977fc0ff76171ed77e214f2c` |

`docs/quality/task042-reopen-20261112/EVIDENCE-INDEX.json`は初回80ファイル、その後82ファイルを実バイトと照合し、その時点で欠落・hash相違は0件でした。文書追記後も対象Pythonソースのhashは同一でした。`CHECK-COMMANDS.json`、`NATIVE-COMMANDS.json`、`MEASURE-COMMANDS.json`と原ログを参照しました。Python50件・Node31件・workflow YAML解析・diff check・4種の最終レポート生成は終了0です。Rustのlocked/offline build・fmt check・workspace test・Clippyの終了0を確認し、native試験ログの61区画を合計すると356成功・失敗0でした。`IMMUTABILITY.json`の318保護ファイル、元57の定義、既存score関数ASTの不変記録を確認しました。

読取り検証用`validate-evidence.py`の初回限定実行は、台帳追記とindex更新の並行中に`FORMAT-MATRIX.json`のhash不一致で終了1となりました。主担当から最終payload凍結の連絡を受けて文書差分を確認し、同じ`python3 docs/quality/task042-reopen-20261112/validate-evidence.py`を再実行しました。最終index SHA-256は`5e090a1b2d247ccbcedee46da22fd4faba04cda9b68a78e715ba882f75ac7377`。再実行は終了0で、82成果物・318保護ファイル・archive内の各反復raw hash・全corpus束縛・原scorerの不変性のassertが全て成功しました。最終証拠の未束縛は残っていません。

全suiteは自動的に再実行していません。主担当の実行結果が最終ソースに対応することを確認した上で、次の限定確認を別途実施しました（いずれもPython3の読取・assert処理、最終終了0）。

- 証拠indexの全hash、承認対象ソースのhash、archive内のSARIF実体を照合。
- v2/補遺の`load_truth`を絶対パスで呼び、二者ラベル・裁定・監査・freeze束縛を検証。
- 元57/v1/v2/補遺の計数を読取り、v2/補遺と対応範囲別表の全層を独立加算。TP/FP/FN/TN、正負、適格、曖昧・対応外の合計が一致。
- 全率の分母を独立照合し、生成`quality.json`内の対応範囲別表と`supported-scope.json`の内容一致を確認。
- 形式49行、全119規則台帳、全248単位、全要求ケースの実体と、再利用したRust既存試験の入力・assertを確認。

補助確認の初回は`load_truth`に相対ディレクトリを直接渡したためパス検証で拒否されました。同APIの絶対パス前提に合わせた再確認は終了0であり、製品や試験の失敗とは扱っていません。

## 元の9条件との照合

| 条件 | 試験が実際に確認している内容と判定 |
|---|---|
| Q1-01 | `MetricsTests`と既存scorer試験は、重複FPと誤報されたnegative単位数を区別し、半開区間・一対一一致・同一行の欠落・除外・分母0を手計算でassertします。混合方式の集計を二重計上しない試験もあります。実生成物の全層合計と分母も独立照合しました。適合。 |
| Q1-02 | `rule_coverage`の両側・片側・未評価の試験と119件の実台帳を照合。評価状態は精度合格の意味へ置き換わっていません。空の方式・用途・ケース層とnull率が残ります。適合。 |
| Q1-03 | 出力前に固定した二者の方式・パラメータ注釈、25相違の裁定、方式表・単位台帳を照合。NodeのML-KEM/ML-DSA/SLH-DSA全列挙変種にはコメント負例が対応します。Java/Rubyの対照と旧称の負例は実ソース/既存default規則assertを確認。一般EdDSA/ECDH/Hybrid専用検出の適用外理由、数値自体の分類性能とは違う範囲も明示されています。適合。 |
| Q1-04 | 用途はnominalなimport/API/literal/設定/依存/属性に分かれ、直接RSA暗号化・AES対照・破損入力があります。`aggregate`の用途集計と実byUsageの合計が全適格・除外母数に一致。実行成功やPQC有効化の正解に流用していません。適合。 |
| Q1-05 | 形式表の47対応行と2適用外行を確認。TSX、CDX1.4–1.7、SPDX2.2–2.3、7設定形式、非暗号化証明書のみP12/PFXの正負・破損があり、Ruby/package.jsonの再利用試験には当該性質を確認するassertがあります。lock2形式の正例は別固定補遺で実測済み。旧暗号化容器の6属性は二者の事後適用性判定により別の対応内表でunsupportedへ分離し、原truth/計数を保持。適合。 |
| Q1-06 | CASE-MATRIX/UNIT-LEDGERにcomment/string/namesake/alias/shadowing/dynamic/dead_code/disabled/mixed/same_line/typed/malformedが実在。動的例の曖昧性、FP/FN、未実行のholdoutセルが残っています。043の20群・50観測への対応とC07の固定版回復木・実行記録を確認。件数のみで受け入れていません。適合。 |
| Q1-07 | v2の6090組、補遺の60組について二者ソース監査・母数・protocol・hash・異なるID・freezeと初測定時刻を確認。ラベル差異は理由付き裁定。由来・ライセンス・既知曝露を保持し、旧Nodeをtuningへ移し、新RustCrypto4単位だけを新規由来としています。適合。 |
| Q1-09 | `compare_accuracy`の異truth拒否と同一reference時の再評価表示、referenceなしの中立表示を実assertします。元57/v1は退行なし、v2は別データと表示し、補遺は事後tuningです。別表の変化は検出器改善と表示していません。適合。 |
| Q1-10 | 正解レビューと評価器/試験レビューを分離。全監査の必須flagをtruthから落としても`load_truth`が拒否する統合試験が追加済みで、先のB指摘は解消しました。zero predictionsのみでは完全性をVerifiedにしない試験、監査のhash/母数/範囲/未解決・理由/時系列の拒否、同系統・変換・有限ループの相関説明を確認。適合。 |

## 新しい適用範囲処理の試験の意味

`test_applicability_preserves_original_and_rescores_excluded_predictions`は元TPを保持したまま別表では予測をexcludedへ移すことをassertし、元結果の上書きを検出します。`test_applicability_excludes_unreported_positive_and_negative_units`は無予測のFNとTNも除外してeligibleを0にし、recallをnullにする手計算例です。改ざん試験は異truth、変更review hash、同一人物、未解決、未知/重複ID、negativeへの改変、空理由、freeze後のreview、契約変更を拒否します。単に成功ログを数える試験ではありません。

実データでは元v2 **58TP/4FP/44FN/89TN・eligible195・unsupported47**が保持され、公開対応範囲は別に **58TP/4FP/40FN/87TN・eligible189・unsupported53**となっています。ambiguous4は両方で保持。方式/パラメータ/用途/artifact/case/言語/形式/規則/検出器の各表は、それぞれこの全体値と一致しました。

原archiveの自己scan SARIFは1357所見、フィルター後は28所見です。raw SHAは`ddb74bcba51fa51a36da64ea2b4190ab3345096bb65d4f42ab1060feac7d5014`、filtered SHAは`1903267ef0acb3b4ea30ff2ebe9d4dde494018ab1222a411936ff3715a2acb1a`でした。残る28をすべて実利用や本番依存と呼ばない説明も確認しています。

## 独立性と承認の限界

Bは同一担当として、出力取得前のv2/補遺ラベルを作成しました。その後の適用性レビューは測定既知の事後確認であり、blind/freshとは記録していません。今回の試験レビューでは実装Aの最終レビュー、裁定、測定値を参照しています。これらの異なる段階を混同していません。

追加性能、別OS、fuzz/mutation、任意feature全protocol、変更後hosted CIは未実行と明記されています。元TASK-042の9条件をそれらの完了へ拡張してはいません。実測されたFP/FNとRustCryptoの4FNも保持され、品質結果の不合格を評価作業未完了と混同しません。

今回の試験妥当性に関する修正要求はありません。主担当/Evaluatorは、このレビューを耐久証拠へ束縛し、最終受入記録・task状態・roadmapを実際の判定と一致させてください。承認後に対象ソースや受入証拠が変更される場合は、その変更に関係する範囲を再確認する必要があります。
