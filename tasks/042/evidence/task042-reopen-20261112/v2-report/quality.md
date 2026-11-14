# PQS-QAM 品質測定レポート

測定対象: `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。
コーパス: `042-v2`。リリース判定: **Unknown**。
検出参照の有限データ評価です。安全性・実利用・PQC移行完了の認定ではありません。

| 軸 | 状態 | 範囲 |
|---|---|---|
| Q1 | Partial | Reviewed algorithm/use annotations; empty strata retained; correlated finite data, not population accuracy |
| Q2 | Partial | File partitions only; unenumerated population unknown, per-detector applicability not aggregated |
| Q3 | Not Tested | Detection labels do not validate risk/severity/recommendations or cryptography |
| Q4 | Not Tested | Separate safety tests required; successful scans do not prove robustness/security |
| Q5 | Partial | Repeated semantics; performance Not Tested; cold cache/multi-host/real product unmeasured |
| Q6 | Not Tested | Separate CI/process/platform/Scorecard evidence required |

FPは所見数、FPRは誤報があったnegative単位数/negative単位数です。母数ゼロはundefined。

| split/family | TP | FP | FN | TN | eligible | precision | recall | F1 | FPR | FNR |
|---|---:|---:|---:|---:|---:|---|---|---|---|---|
| holdout/source_api | 0 | 0 | 4 | 0 | 4 | undefined | 0.000000 | 0.000000 | undefined | 1.000000 |
| holdout/draft | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| holdout/config | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| holdout/dependency | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| holdout/key | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| holdout/certificate | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| tuning/source_api | 16 | 3 | 24 | 39 | 82 | 0.842105 | 0.400000 | 0.542373 | 0.071429 | 0.600000 |
| tuning/draft | 0 | 0 | 1 | 0 | 1 | undefined | 0.000000 | 0.000000 | undefined | 1.000000 |
| tuning/config | 9 | 1 | 1 | 11 | 22 | 0.900000 | 0.900000 | 0.900000 | 0.083333 | 0.100000 |
| tuning/dependency | 11 | 0 | 10 | 24 | 45 | 1.000000 | 0.523810 | 0.687500 | 0.000000 | 0.476190 |
| tuning/key | 3 | 0 | 0 | 6 | 9 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |
| tuning/certificate | 19 | 0 | 4 | 9 | 32 | 1.000000 | 0.826087 | 0.904762 | 0.000000 | 0.173913 |

未評価の他ルール所見: 42。
言語・ルール・検出器・入力種別の分母、FP/FN、coverage、環境、反復、性能の全値はquality.jsonを参照。
方式別・用途別はquality.jsonを参照。暗号分類や実行の正しさは未検証です。
品質モデル・式・限界はdocs/quality/QUALITY-MODEL.mdとQUALITY-METRICS.mdを参照。

同一truthの回帰比較: **Not Applicable**。数値CIゲートは変更していません。
性能の同条件比較・調査信号は別評価であり、この判定に含みません。

指定した規則と入力の範囲での注釈確認: **Verified**。負例に対応したFP所見4件、positiveへの重複0件、帰属不能0件。
帰属不能には正しい未注釈の事実が含まれ得ます。製品の誤検出とは断定できません。Precision/F1は固定注釈への一致成績です。
二者の全指定規則・全入力のソース監査と予測の帰属を確認しました。この有限範囲を超える検出精度の保証ではありません。検出器のFP/FNは保持します。

公開対応範囲の別評価はsupported-scope.jsonを参照してください。
元の属性正解・成績を保持し、二者が対応外と確認した6単位を別表の適格分母から除きました。
これは測定後の適用範囲訂正です。検出器の改善や新しい独立評価とは扱いません。
