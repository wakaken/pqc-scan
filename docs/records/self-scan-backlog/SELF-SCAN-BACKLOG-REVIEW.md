# 独立レビュー: self-scan backlog

判定: **承認**（以下の有限な登録差分・履歴アラート分類に限る）。重大・修正必須の問題は見つかりませんでした。

対象基準: `dfda62ae18af4e276a8d92b4593bb89eb6c1ce7b`。レビュー担当は実装担当とは別コンテキストで読み取りと検証を行い、この記録以外のファイルを変更していません。外部状態操作、コミット、push、mergeは行っていません。

## 独立確認した証拠

- `prompts/REVIEWER.md`、両README、architectureと指定のknowledge文書を確認しました。`RESTART-LOOP.md`の適用範囲はrestart baseline/task014であり、本変更をtask014の再実行として扱っていません。
- 台帳追加分は128個の完全一致result group、計130件、11ファイルです。新規9ファイルと既存2ファイルへの追加であり、既存103ファイルのsource hashおよび全result allowanceは維持されています。ポリシーschema、projection helper、Rust、ルール本体に変更はありません。
- 130件の参照行を実ソースで確認しました。rule YAMLのID/pattern/description/migration_hint、翻訳キーと説明文、reportのrule-ID routing、CodeExampleのbefore/after文字列と助言、公開OIDの表示名対応表、task017の安全要件の文章です。これらの行にTLS設定の適用、鍵生成、署名等の暗号実行はありません。秘密鍵のヘッダ検出パターンは鍵本体ではありません。
- 追加台帳とREVIEWED-RESULTS.jsonをfull-result digestで突合し、128group/130occurrenceが過不足なく対応することを確認しました。基準台帳では130件全件が保持され、変更後だけ全件が除外されます。
- `node --test shell/*.test.cjs`: 28成功、0失敗。既存の変更ソース、新規製品所見、過剰重複、複数run、symlink、間接URI等の保持テストと、新規3テストを含みます。新規テストはraw不変、同一登録ファイルの未知所見保持、位置不明所見保持を検証しています。
- 現raw SARIFを基準台帳・変更台帳の両方で独立replayしました。**856 = 722除外 + 134保持 → 856 = 852除外 + 4保持**。保持payloadと順序は基準projectionから有限130件を引いたものに完全一致し、入力JSONは不変でした。rawは前回rawとbyte一致します。
- 全112 source hashがverifiedです。保持4件は元のartifact-only test observationsであり、行位置がないため除外されません。新規製品所見を包括的に除く例外はありません。
- VERIFICATION.jsonの列挙ファイルhash、task027の43 Rust/Cargo runtime source hashを独立計算して一致を確認しました。
- 両README、CI-SELF-SCAN.md、SELF-SCAN-BACKLOG-FOLLOWUP.mdの記述は上記の有限範囲と制約に合致しています。

## 既存GitHubアラート128件の分類

保存されたGitHub API snapshotとALERT-REVIEW.jsonを独立突合しました。128件全件についてalert number/URL、rule、path、旧line、commit、severity、messageとcurrent source SHA-256が一致します。`git show 1335033fb3ffd96632f9f39a51bf8f2a115f2ead:<path>`を取得し、diffのunchanged blockから旧line→current lineを再計算して全件一致を確認しました。

124件はcurrent columnまで一意に対応します。#584/#583はjapanese.rs:85、#580/#579は同:157の各同一groupに対応します。これら4件は旧instanceにcolumnがある一方、current SARIFはline-onlyのため、各2件のソース上の同値対応までを確認しています。個々のsub-line identityが確立したとは判断していません。message/severityと実ソースも含めた有限group単位のfalse positive判定には十分です。

以上の128件を、暗号処理の実利用という観点で**false positiveとして個別にtriageする根拠は妥当**です。各mutation直前のread-checkで現在のinstanceがレビュー済みのsource/rule/message/location/commitに一致することを確認し、不一致や新規アラートを対象外にする、という実装担当の方針を前提とします。これは許可された範囲の妥当性に関するレビューであり、外部操作完了の証明ではありません。

## 限界と残作業

- live GitHubの変更およびread-back結果は本レビューでは未実施です。128件の現在stateやtriage成功は別途検証が必要です。
- PR CI、push後commit、developへのmergeは本レビューの証拠に含めません。developへのfix適用と履歴アラートdismissalは別の状態です。
- 本変更はmetadata/NodeテストのためRust build/test/clippyを再実行していません。以前のCI成功を現差分のCI成功とは扱っていません。
- 4件の位置不明所見が保持されるため「全所見ゼロ」や製品全体の暗号安全性は主張できません。将来の同一ファイル上の新規所見やsource変更も再レビューが必要です。
