# PQS-QAM 品質測定レポート

測定対象: `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。
コーパス: `042-v2-lock-controls`。リリース判定: **Unknown**。
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
| tuning/source_api | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| tuning/draft | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| tuning/config | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| tuning/dependency | 2 | 0 | 0 | 0 | 2 | 1.000000 | 1.000000 | 1.000000 | undefined | 0.000000 |
| tuning/key | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| tuning/certificate | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |

未評価の他ルール所見: 0。
言語・ルール・検出器・入力種別の分母、FP/FN、coverage、環境、反復、性能の全値はquality.jsonを参照。
方式別・用途別はquality.jsonを参照。暗号分類や実行の正しさは未検証です。
品質モデル・式・限界はdocs/quality/QUALITY-MODEL.mdとQUALITY-METRICS.mdを参照。

指定した規則と入力の範囲での注釈確認: **Verified**。負例に対応したFP所見0件、positiveへの重複0件、帰属不能0件。
帰属不能には正しい未注釈の事実が含まれ得ます。製品の誤検出とは断定できません。Precision/F1は固定注釈への一致成績です。
二者の全指定規則・全入力のソース監査と予測の帰属を確認しました。この有限範囲を超える検出精度の保証ではありません。検出器のFP/FNは保持します。
