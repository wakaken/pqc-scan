# タスク029：検出根拠のプライバシー検証

2026-11-10、`codex/scanner-quality-improvements`。比較元HEADは `4324d2c31e96ce62de211882192d6c4a8acded6d`。実装仕様は [契約](../architecture/EVIDENCE-PRIVACY.md)、実行コマンド・終了値・ソースSHA・バイナリSHA・コーパス測定値は [COMPARISON.json](task029/COMPARISON.json) に記録しています。HEADは比較元を指し、実装ソースは同ファイルのsourceBindingsで識別します。

## 対応

従来の短い文字列を残すmatch処理や先頭・末尾を残すpreviewだけでは、秘密鍵ブロック本文や認証情報の断片が出力に残りました。呼び出し側が構築するmetadata・対処案・CBOM・依存関係の自由文にも同様の経路がありました。実際の秘密情報を使わず、合成データで失敗を再現し、共通の既知構文マスクと収集時の位置情報による保護を追加しました。

原文から秘密鍵範囲と認証情報の位置だけを収集し、従来のID確定と棚卸しの対応付けが終わってから出力値をマスクします。検出ルール・CLI・ファイル名・出力スキーマは変更していません。CBOMも再生成せず、行数・順序を保ちます。ライブラリのレポート関数は入力を複製してマスクするため、呼び出し側の値を変更しません。従来のevidenceに対するBEGIN/ENDなしのPRIVATE KEY断片マスクも維持し、回帰テストで確認しています。

## 検証

- `cargo build --offline --locked --workspace`、releaseビルド、`cargo fmt --all --check`、workspaceテスト224件、clippy `-D warnings`：すべてexit0。
- Nodeテスト28件、品質測定Pythonテスト20件：成功。
- 新規10テストで、6種類の秘密鍵マーカー、未終了ブロック・孤立END、Unicode、quoted/escaped credentials、Bearer、HTTP userinfo・query、metadata、対処案、CBOM、依存関係、診断、stderrを確認。CLI9出力・ライブラリ6形式を英日両方で確認し、schema検証と英日machine payload比較を実施しました。
- 独立レビューでpreview短縮による中間断片漏れとdot/percent入りqueryの漏れを指摘され、修正。長い値の中間断片も不存在を検証する回帰を追加しました。[レビュー](TASK-029-REVIEW.md)を参照してください。
- 同じ現行ソースを旧・新releaseで自己スキャンし、930件のFinding ID・summary・CBOM行数とSARIF全体が一致しました。自由文のマスク540フィールドが意図した差分です。追加レポートの有無でも従来JSONは一致しました。source snapshotが後から変わっても保護され、native associationの参照先も維持されることを別途テストしています。
- 固定コーパス41ファイル・5746 bytes・37検出を各3回測定。修正前後・assessment有効時の分類別TP/FP/FN、全disposition、coverageは一致。32 analyzed / 8 partial / 1 unknownという既存制約も維持しています。マスクされた自由文を含むsemantic digestの変更は検出意味の変更として数えていません。
- small/medium/large/pathological各fixtureの初回＋warm3回でwall/CPU/RSSを記録。数値予算は未承認で、性能向上や許容閾値達成は主張しません。タスク023の状態は変更していません。

ログは `target/task029/` のtests.log、clippy.log、node-tests.log、python-tests.log、final-before/after/policy、legacy-shadow.jsonに保存しています。初回失敗ログ・レビュー指摘も残し、最終成功と区別しています。検証補助スクリプトの最初のline-map生成は既存binary fixtureのUTF-8 decodeで失敗し、置換decodeによる行マッピングへ修正しました。製品スキャンは成功しており、同じ比較を再実行済みです。accuracyの最初の比較は結果配列順序の違いで失敗し、全detailをソートして照合しました。再測定では既存証跡への上書きが拒否されたため、旧結果をinitial-*へ保存して新規実行しました。新規ファイルは基準HEADの行マップ対象から外し、個別登録として照合しています。

## CI自己スキャンへの影響

新しいテスト内のRS256・2048等の文字列73件を行ごとに確認し、4ソースの完全SHAと結果SHA・rule・line・column・件数で登録しました。既存テスト35entryは変更されていない行への対応を確認して再束縛しました。productionの新規検出を包括的に除外していません。

raw930件 = reviewed expected926件 + retained4件、117ソースすべてSHA verified。従来のraw857 = expected853 + retained4から、合成テスト73件だけ増加しています。raw SARIFを保持し、既存の4件は判定対象として維持します。GitHub上のアラート・CI・設定は操作していません。

## 制約と未実施

認識済みの単一行credential構文と秘密鍵マーカーに対する保護です。任意の秘密名・難読化・未知形式・複数行credentialの位置追跡や、構造フィールドに埋め込まれた秘密まで保証するものではありません。認証情報を含むファイルのpreviewは保守的に全置換され、公開テキストも省略される場合があります。秘密鍵の検出形式そのものは拡張していません。

remote CI、他OS、実データでの秘密探索、外部製品スキャン、ブラウザー操作、push・PR・mergeは未実施です。HTMLテンプレートや画面操作は変更していません。既存のルール誤検出・未対応範囲の解消や、あらゆる秘密情報の不存在は主張しません。
