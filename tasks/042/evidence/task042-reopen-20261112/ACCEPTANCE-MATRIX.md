# TASK-042 当初9受入条件と今回の証拠

受入条件は`tasks/042-quality-corpus-expansion.yaml`の元の9行を維持しました。以下は実施済みの内容とその証拠です。完了判定はこの対応表に加え、実装レビューと最終試験妥当性レビューの結果を必要とします。

| 要求 | 今回完了した検証 | 必須成果物・証拠 |
|---|---|---|
| Q1-01 | TP/FP/FN/TN、重複予測、誤報negative単位数、母数ゼロを手計算と照合。分母を全率に保持。旧57/v1/v2/補遺の実測と公開対応範囲の別表を保存 | 各データの`metrics.json`、`quality.json`、`supported-scope.json`。`MetricsTests`と`test_taxonomy.py`、`python-final.log` |
| Q1-02 | 119規則を全件列挙し、正負あり・片側のみ・適格例なしを理由付きで表示。言語・規則・検出器別の分母を保存し、空のholdout層を隠さない | `RULE-INVENTORY.json`、各`case-coverage.json`、`quality.json`のbyLanguage/byRule/byDetector |
| Q1-03 | 指定方式と見えるパラメータの正負・対応外理由を出力前に二者で確認。Node標準PQC変種の負例を追加。一般ECDH/EdDSA/Hybrid専用検出器がない範囲は安全と扱わない | `ALGORITHM-MATRIX.md/json`、各`algorithm-truth.json`、固定truthと二者ラベル |
| Q1-04 | 署名、鍵交換、KEM、暗号化、JWT、設定、依存、属性、literal/importを用途別に集計。RSA暗号化の直接例とAES対照・破損例を追加。名目的参照を実行成功と区別 | 各`usage-truth.json`と`quality.json`のbyUsage、`UNIT-LEDGER.json` |
| Q1-05 | 公開対応形式・版の正負・破損例を49行の表へ整理。TSX、CDX1.4/1.6/1.7、SPDX2.2、設定7形式、平文公開証明書P12/PFXを補った。Rubyとpackage.jsonは意味が一致する既存default規則試験を再利用。lock2形式は別の固定補遺で測定。旧暗号化容器の6属性は対応外として別表から除外し、原属性一致表も保持 | `FORMAT-MATRIX.md/json`、各`artifact-metrics.json`、`v2-report/supported-scope.json`、`tests/quality/applicability-042-v2/`の二者契約レビュー。v2の全6090組と補遺の全60組の二者ソース監査 |
| Q1-06 | コメント、文字列、同名、alias、shadowing、動的指定、未使用、無効設定、混在、同一行、型/引数、破損を分類し実測。FP/FNを残し、動的unknownを負例にしない。043の50観測・20群の扱いを対応付け、C07の回復木も確認 | `CASE-MATRIX.json`、各`adversarial-cases.json`、`043-FOLLOWUP.json`、`PARSER-DIAGNOSTIC.md` |
| Q1-07 | 二者のソース由来ラベル、相違の裁定、全指定規則×入力の監査、freeze時刻、入力/由来/ライセンスをhashで固定してから測定。既知の質問文・旧集計への曝露は開示。旧Node系統は調整用へ移し、新しいRustCrypto系統だけを初評価と区別 | corpus別の`PROTOCOL.md`、`provenance.json`、二者labels、truth、`FREEZE.json`/`LOCK-FREEZE.json`、`LABEL-REVIEW.md` |
| Q1-09 | 旧57とv1は同一データ比較で退行なし。v2は別truthなので差分合格としない。RustCryptoの初評価4単位と事後調整部分を分ける。再測定を初評価と呼ぶ表示不具合を修正 | 各`comparison.json`、旧57/v1のaccuracyComparison、v2のbySplitFamily、同一reference試験 |
| Q1-10 | 正解レビューと評価器レビューを分ける。二者監査のhash・範囲・母数・プロトコル・独立IDと必須flagの改ざん拒否を試験。所見ゼロから注釈完全性を推測しない。系統重複と少数標本を報告 | `IMPLEMENTATION-REVIEW.md`、`TEST-REVIEW.md`、`LABEL-REVIEW.md`、`ACCEPTANCE.json`、Python試験、系統分析 |

119規則の全精度を実測する条件へ置き換えてはいません。元のQ1-02は全規則の評価状態を明示する条件です。未測定規則を対応外にしたり、全規則の品質が合格したりしたとは表現しません。

未対応やambiguousを残すことと、必須検証を実施しないことを分けました。今回はそれらも正解ラベルと判定理由を確認して測定し、分母からの除外を明示しています。042の評価器・正解・指定形式に必要な未確認を放置することは認めません。

検出器のFP/FNを修正すること、Q3の暗号判断を検証すること、Q2/Q4/Q5/Q6の全タスクを完了することは、042の元条件ではありません。測定で残った製品上の失敗は品質結果として保持し、今回の評価基盤の完了と区別します。製品の安全性や1.0リリースを合格にはしません。
