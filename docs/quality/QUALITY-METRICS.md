# PQS-QAMの指標と分母

対象ロードマップ: **pqc-scan v0.3**（品質評価・検証基盤の包括的強化。[位置付け](QUALITY-ROADMAP.md#v03-scope)）。

定義版: 1。正解単位、Unicode位置の半開区間、不明な依存関係位置の照合は[既存の測定手順](../../tests/quality/PROTOCOL.md)と `evaluate.py` が正本です。
この文書は既存の採点を説明します。追加する測定は[要求対応表](QUALITY-REQUIREMENTS.md)にまとめています。

## Q1: 検出精度の計算

PはPositiveの正解単位数、NはNegativeの正解単位数です。採点対象はP+Nです。
TPは一対一で検出できたPositive単位、FNは見逃したPositive単位で、P=TP+FNです。TNは対象予測が一つも出なかったNegative単位です。

FPは、Negative上の誤報、検出済みPositive上の重複、未注釈の位置に出た対象規則の所見数です。
一方、FPUは一つ以上の誤報があるNegative単位数で、N−TNです。同じNegativeに誤報が何件出ても、FPUでは一単位と数えます。
Ambiguous/Unsupported上の予測と、対象外の規則による予測は別に集計します。

| 指標 | 分子 / 分母 | 意味 |
|---|---|---|
| Precision | TP / (TP+FP) | 対象予測の適合率。重複・余分な所見も誤報として数える |
| Recall | TP / P | 定義したPositive参照を検出できた割合 |
| F1 | 2TP / (2TP+FP+FN) | 既存の所見採点におけるPrecisionとRecallの調和平均 |
| False Positive Rate | FPU / N | Negative単位のうち、一つ以上の誤報があった割合 |
| False Negative Rate | FN / P | Positive単位を見逃した割合 |

分母0はJSONの `null`、Markdownの `undefined` とし、0%や100%に置き換えません。
FPには重複や未注釈の所見も含まれるため、`FP/(FP+TN)` を単位別のFPRとして計算しません。`TP+FP+FN+TN` も採点対象数とは限りません。
単位別の混同行列にはTP・FN・FPU・TNを使い、従来の所見数FPを併記します。

例えばTP1/FN1/TN1で、同じNegativeへの誤報2件と未注釈予測1件があれば、FP3です。
Precisionは1/4、Recallは1/2、F1は1/3、FPRは1/2になります。

## 層別の集計

`split/family` はデータ区分と既存の評価対象分類の組合せで、023の承認済み回帰条件を適用する単位です。
`report.py` は追加の部分集合も同じscorerに渡します。別の照合エンジンで採点し直しません。

| 区分 | 現在の分類方法 | 読み取れる範囲と不足 |
|---|---|---|
| 言語 | 固定入力pathの拡張子。非ソースはNot Applicable、SwiftはUnsupported Swift | 成功した予測から言語を逆算しない。7言語全てに独立holdoutがあるわけではない |
| 規則 | corpus.jsonで指定したrule ID | 023-v2は19、042-v1は22規則を指定。042-v1の全119規則表は選択した追加版内での正解の有無を示し、旧版の測定は別に保持 |
| 検出器 | 119ルールの台帳にあるkind | 実行時の適用可能性や検査網羅性とは別 |
| 利用形態 | 023-v2はsource_api等のfamily、042-v1はレビュー済みのusage | 追加版は署名・KEM等の参照を分類する。実行可能性・機能有効化は証明しない |
| 入力種別 | source/configuration/certificate/key_marker/dependency_manifest/sbom | 現行データの既知file/family分類。複数種別が混在するfileは明示分類を要するとして拒否 |
| アルゴリズム | 023-v2: Not Tested / 042-v1: Partial | 追加版は独立にレビューした方式別の参照検出を集約する。暗号分類の正確性とは別。元の023-v2に方式の正解は追加しない |

データ区分と評価分類の空セルを残し、追加区分で出現しないカテゴリは未測定と扱います。
`errorDetails` はFP/FN/採点対象外の予測を規則・位置・正解単位ごとに保持します。
全体の集計は補助値です。弱いセルや少数の分母を隠す用途には使いません。
母集団からの無作為標本ではないため、一般の95%精度保証や根拠のない独立標本数を示しません。

## Q2: 列挙・適用・完了を分ける

既存diagnosticsの件数は次の等式に従います。

```
selected = read_ok + read_failed
read_ok = analyzed + partial + analysis_failed + unsupported + not_applicable + unknown
```

read_ok/selectedは読み取り成功率です。analyzed/read_okは選択範囲内の処理状態を表します。
ignore等で対象の総数が不明なら、入力列挙率は計算できません。`not_enumerated_population: true` は未知の母数があり得ることを表し、具体的な件数ではありません。
資源制限で停止した場合は `detector_truncated` 等の理由とファイルを記録し、処理状態はPartialに含めます。
検出器のNotApplicable、Unsupported、読み取り失敗、ファイル全体のUnsupportedは別々に数えます。
検出器ごとの適用可能数・完了数・失敗数の集約は、今後の055で扱います。

## Q3〜Q6の測定単位

Q3は規格・用途・暗号属性・評価方針の判断ケースを単位とします。検出のTPとは別に、正しい判断数とレビュー済みケース数、未評価の組合せを示します。現状は判断全体の正答率を確立していません。

Q4は入力や故障注入ケースごとのpanic、timeout、制限超過、秘匿用文字列の残存、保持結果・診断を記録します。
変異試験のscoreはcaught/(caught+missed)とし、timeout/unviable/未選択を別掲します。timeoutを自動的にcaughtへ数えません。
到達率はline・region・branchを分けます。

Q5は入力レシピ、hash、build/features、ホストと隔離条件が同じ試料を比較します。
処理量はfiles/wall、bytes/wallで定義しますが、現在は起動・出力を含む小規模入力の値です。
本当のOS cold cache、失敗時の資源、複数ホストの差は別測定です。

Q6は設定、手順、環境、検証項目ごとに状態と根拠を示します。全体点には合算しません。
