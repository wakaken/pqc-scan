# 品質ベースライン（2026-11-11測定）

対象ロードマップ: **pqc-scan v0.3**（品質評価・検証基盤の包括的強化。[位置付け](QUALITY-ROADMAP.md#v03-scope)）。

測定対象HEAD: `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`、version `0.1.0`。
測定開始時はdevelopで未コミット変更なし。作業ブランチは `codex/quality-assessment-model`。
TASK-041では製品のRust実装・ルール・Cargo.lockを変更していません。今回の文書推敲でも再測定はしていません。
既存履歴[TASK-023](TASK-023-BASELINE.md)は保持し、今回分を追記しました。
TASK-041で検出器を改善したわけではありません。以下の旧測定との差は、それ以前の変更が測定対象HEADへ反映された結果です。

## 測定条件と再実行

Rust1.93.1のreleaseビルド、ワークスペースの既定features、Apple M4 Pro / arm64、
macOS27.2、論理CPU 12個、メモリ48GiB、Python3.12.7です。
既存の測定スクリプトでスレッド数1、既定の2MiB上限とignore・除外方針を指定し、全レポートと診断を出力しました。
ビルド/スキャンとも `sandbox-exec` で外部通信を禁止し、評価用データに含まれるコードは実行していません。
合成・公開入力だけを使用しました。

```
cargo build --workspace --release --locked --offline
python3 tests/quality/run.py --binary target/release/pqc-scan --build-profile release \
  --out-dir target/quality-assessment-20261111/baseline --repeats 3 --warm-samples 3
```

実行時は各コマンドの前に `sandbox-exec -p '(version 1)(allow default)(deny network*)'` を付けています。
上記は終了コード0。再実行時は必ず新規out-dirを選びます。
初期測定の詳細は `target/quality-assessment-20261111/baseline/summary.json`、
実行台帳は同ディレクトリの親の `entry-commands.json` とログです。
同じ入力・正解データによる最終報告と証拠は[評価書](QUALITY-ASSESSMENT.md)から辿れます。

評価データは `023-v2` の41ファイル / 5746 bytes、正解単位57件です。
内訳はPositive30・Negative20・Ambiguous2・Unsupported5です。
corpus SHA256 `d6fa706b8f2c2c1267e9e1f0383d6318f4c11b2761f8785b6d7865797ee2afb6`。
正解・規則・バイナリ・ソース・環境のhashは機械可読の成果物に保存しています。

## 検出結果と既知の誤検出・見逃し

適格50単位: **TP25 / FP3 / FN5 / TN17**。precision25/28=89.29%、
recall25/30=83.33%、F1=50/58=86.21%、FPU3/negative20=15%、FNR5/30=16.67%。
これらは指定した規則の静的参照について、固定データで測った値です。製品全体の精度や安全率には一般化できません。
対象外の他ルール所見10件は正解として採点していません。
データ区分・評価分類などの全表と分母は、生成したJSON/Markdownを参照してください。

| 主な集計区分 | TP/FP/FN/TN | 解釈 |
|---|---|---|
| tuning/source_api | 11/3/3/3 | precision/recallとも11/14。別名のimport、同名シンボルによる隠蔽、表示用文字列、不正構文の課題が残る |
| holdout/source_api | 2/0/1/2 | recall2/3。少数の独立評価データによるprecision2/2を一般化しない |
| holdout/config | 0/0/1/1 | Java XML設定を見逃す。precision母数0はundefined |
| holdout/dependency | 2/0/0/2 | 依存が宣言されている証拠。機能が有効という証拠ではない |
| holdout/certificate・key・draft | eligible0 | 良好なholdout精度とは判定不能 |

FP: u015（JSの同名関数）、u021（Rustのローカルモジュール）、u022（Ruby表示文字列）。
FN: u014（JSの別名import）、u020（Pythonの別名import）、u027（不正構文内の名目的参照）、
u051（ML-DSAの複数行import）、u054（XML namedGroups）。正解ラベルは変更していません。

元023はTP22/FP3/FN8/TN17。差はholdout/dependency TP+1/FN−1、
tuning/source_api TP+2/FN−2です。他のデータ区分・評価分類は不変で、承認済み回帰条件を満たします。
直近 `target/final-gaps/baseline/summary.json` とも採点対象の全グループが同じです。
これは同一公開データでの回帰比較であり、新規独立データでの改善検証ではありません。

## 網羅性・再現性・秘匿

41 selected = 41 read_ok + 0 read_failed、41 read_ok = 33 analyzed + 7 partial + 1 unknown。
残りのファイル処理状態は0件です。全体はPartialで、列挙されていない対象の母数は不明です。
正解ラベルのUnsupported5件と、ファイル処理状態のUnsupported0件は別の数え方です。
3回の実行で意味・診断・採点が一致し、元の出力順序も一致しました。
比較時に除く時刻情報は `PROTOCOL.md` で限定しています。期限の判定結果そのものは除きません。
秘匿確認用の合成文字列は、全出力・標準出力・標準エラーに残りませんでした。この試験で、あらゆる秘密情報の検出を保証することはできません。

## 性能

初期測定の結果です。入力生成後の初回に続いて3回測定しました。起動とレポート生成を含み、build時間は含みません。

| workload | files / bytes | wall min / median / max秒 | CPU median秒 | RSS median MiB |
|---|---:|---|---:|---:|
| small | 41 / 5746 | 0.031613 / 0.032972 / 0.033412 | 0.021823 | 18.16 |
| medium | 328 / 45968 | 0.095117 / 0.095557 / 0.100386 | 0.078986 | 20.92 |
| large | 1312 / 183872 | 0.324824 / 0.326653 / 0.336911 | 0.269965 | 33.86 |
| pathological | 1 / 60017 | 0.026088 / 0.027066 / 0.027121 | 0.018478 | 16.20 |

CPU/RSSはwait4で得た子プロセスの値です。wall timeは約5ms間隔で終了を確認するため、smallの小さな差を速度差と解釈しないでください。
大量一致候補を含む入力では、打ち切り診断が残りました。失敗・timeout時の性能試料はありません。
大規模な実製品、本当のcold cache、Windows、複数ホスト、長期変動、失敗時の資源使用はNot Testedです。

同じホスト・toolchain/profile/features・入力・外部通信禁止条件で行った直近のfinal-gaps測定
（large warm max0.354668秒）に比べ、新しい2x調査トリガーはありません。
ただし前々回の0.158429秒に対する既存の2x超過は[TASK-037の調査履歴](FINAL-GAPS-VALIDATION.md)に残します。
過去の調査では、安全にリンクを扱う読み取り処理の費用が原因とされました。その調査履歴は残しており、基準の変更で超過を消してはいません。
OS版等の環境項目は機械的に照合します。少数の測定値の変動だけでは速度向上と判定しません。
