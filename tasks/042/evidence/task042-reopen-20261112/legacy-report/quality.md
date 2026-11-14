# PQS-QAM 品質測定レポート

測定対象: `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。
コーパス: `023-v2`。リリース判定: **Unknown**。
検出参照の有限データ評価です。安全性・実利用・PQC移行完了の認定ではありません。

| 軸 | 状態 | 範囲 |
|---|---|---|
| Q1 | Partial | Frozen nominated reference detection only; per-algorithm truth and population generalization absent |
| Q2 | Partial | File partitions only; unenumerated population unknown, per-detector applicability not aggregated |
| Q3 | Not Tested | Detection labels do not validate risk/severity/recommendations or cryptography |
| Q4 | Not Tested | Separate safety tests required; successful scans do not prove robustness/security |
| Q5 | Partial | Repeated semantics; performance Not Tested; cold cache/multi-host/real product unmeasured |
| Q6 | Not Tested | Separate CI/process/platform/Scorecard evidence required |

FPは所見数、FPRは誤報があったnegative単位数/negative単位数です。母数ゼロはundefined。

| split/family | TP | FP | FN | TN | eligible | precision | recall | F1 | FPR | FNR |
|---|---:|---:|---:|---:|---:|---|---|---|---|---|
| holdout/source_api | 2 | 0 | 1 | 2 | 5 | 1.000000 | 0.666667 | 0.800000 | 0.000000 | 0.333333 |
| holdout/draft | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| holdout/config | 0 | 0 | 1 | 1 | 2 | undefined | 0.000000 | 0.000000 | 0.000000 | 1.000000 |
| holdout/dependency | 2 | 0 | 0 | 2 | 4 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |
| holdout/certificate | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| holdout/key | 0 | 0 | 0 | 0 | 0 | undefined | undefined | undefined | undefined | undefined |
| tuning/source_api | 11 | 3 | 3 | 3 | 20 | 0.785714 | 0.785714 | 0.785714 | 0.500000 | 0.214286 |
| tuning/draft | 1 | 0 | 0 | 2 | 3 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |
| tuning/config | 3 | 0 | 0 | 3 | 6 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |
| tuning/dependency | 1 | 0 | 0 | 1 | 2 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |
| tuning/certificate | 4 | 0 | 0 | 2 | 6 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |
| tuning/key | 1 | 0 | 0 | 1 | 2 | 1.000000 | 1.000000 | 1.000000 | 0.000000 | 0.000000 |

未評価の他ルール所見: 10。
言語・ルール・検出器・入力種別の分母、FP/FN、coverage、環境、反復、性能の全値はquality.jsonを参照。
アルゴリズム別: Not Tested。未列挙母数はunknownであり、0ではありません。
品質モデル・式・限界はdocs/quality/QUALITY-MODEL.mdとQUALITY-METRICS.mdを参照。

同一truthの回帰比較: **Passed**。数値CIゲートは変更していません。
性能の同条件比較・調査信号は別評価であり、この判定に含みません。
