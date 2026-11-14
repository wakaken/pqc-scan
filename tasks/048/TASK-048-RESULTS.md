# TASK-048 出力契約の検証

対象はHEAD `c1bc58e59baffdee73beae73f79b1f9a68804bd2`と048の未コミット差分です。ブランチは`codex/quality-assessment-model`を継続しています。元の要求は[048 YAML](task.yaml)のQ3-11・Q3-12・Q4-05です。

## 変更

既存のjsonschema 0.29.1と固定済みSchemaを再利用し、実CLIの全静的出力とdiff、全3種類のprobe出力の正負例をまとめました。SARIF 2.1.0 Errata 01、CycloneDX 1.7、プロジェクト独自v1の由来・license・hashを区別します。通常のreport.json、legacy CBOM、dependency-sbom、diagnosticsには公式標準Schemaがなく、独自に標準適合を名乗らず互換・意味の照合を行います。legacy bare-array CBOMはCycloneDX validatorで拒否されます。

実入力には公開RSA2048証明書、RSA参照2箇所、合成の秘密鍵マーカー、Cargo宣言`^0.10`とregistry lock版`0.10.75`を使用しました。公開証明書のSHA-256、実体の区別、位置・ID・重複・参照先・unknown・来歴・宣言/解決版・export lossを照合します。日英と異なる配置先でfinding ID・assetは一致します。runごとのobservation IDを恒久IDとは扱いません。重複した弱いアンカーはdiffでもunresolvedを保ちます。

Markdownの複数行snippetと単独CRによるコード表示の逸脱を修正しました。LF/CRLF/CRを正規化し、全行を字下げします。HTML・JSON・SARIFの公開構造、ルール、暗号判断は変えていません。日英READMEと利用ガイドにも表示動作を反映しました。

## 検証と成果物

正本の測定成果物は`target/quality-task-048/{schema-results.json,output-semantics.json,render-security.md}`です。保存先、最終ソースとコマンド・終了結果は本タスクの保存証拠に記録します。

| 要求 | 証拠と確認内容 |
|---|---|
| Q3-11 | 実CLIのSchema正負23例、全probeの失敗→completed偽装拒否、既存の空出力・境界・成功probe・公式Schema digest試験。ネットワーク取得機能を無効にしたvalidatorでoffline検証 |
| Q3-12 | 実CLIの日英・配置変更、JSON/SARIF/HTML/Markdown/legacy CBOM/CycloneDXの照合。固定入力の省略理由の件数、全投影observationの位置・単位・method/assurance/usage、未知状態・版来歴を照合。来歴消去・改変を拒否する対照も追加 |
| Q4-05 | 日英8群のHTML/Markdown変換計16ページをChrome 1466×1381で観察。攻撃文字列保持、注入実行要素0、認識対象の合成秘密0、陽性対照、キーボード・読込後offlineを確認 |

実装レビューは`/root/quality048_implementation_review`を使用しました。R1でexport loss/観測来歴の集約不足を指摘され、R2で内容照合と改変拒否を追加して承認されています。試験レビューは`/root/quality048_test_review`がR3の15ファイルと最終ログ・実ブラウザー観察を確認し、元の3条件を満たすとして受け入れました。レビュー後は完了記録だけを更新し、製品・試験・計測コードは変えていません。

## 評価の範囲

有限の固定入力に対する契約検証です。製品全体の精度、全入力への安全性、全ブラウザー/Markdown処理系、Windows実行の保証ではありません。probeの攻撃writer入力は合成オブジェクトで、protocol Schema試験の実CLI失敗出力とは別です。通信成功・交渉・認証の網羅評価は049の範囲です。

Markdownでコード表示に入れる値に改行があれば、先頭だけでなく継続する全行を同じ文脈へ入れる必要があります。JSONで構造が正しくても、コード表示・属性・URIへの投影では別の検証が必要です。

## 最終判定

当初のQ3-11・Q3-12・Q4-05と必須3成果物、二段階レビューを照合し、048をdoneとしました。測定の完了であり、製品全体の安全性の認定ではありません。

| 実行 | 結果 |
|---|---|
| `cargo test --workspace --locked --offline` | 544成功、3ignored、終了0 |
| `cargo test --workspace --locked --offline --all-features` | 597成功、5ignored、終了0 |
| 通常・全機能の`cargo clippy --workspace --all-targets --locked --offline … -- -D warnings` | 両方終了0 |
| 通常・全機能の`cargo build --workspace --locked --offline …` | 両方終了0 |
| `cargo fmt --all --check` | 終了0 |
| `python3 -m unittest discover -s tests/quality -p 'test_*.py'` | 77成功、終了0 |
| `node --test shell/compact-report.test.cjs` | 3成功、終了0 |
| `python3 tests/quality/task048/contracts.py --raw target/quality-task-048/raw --out target/quality-task-048` | 終了0 |

コマンドの省略なしの記録は[VALIDATION.json](evidence/task048-20261114/VALIDATION.json)、入力・工具・規則と版は保存archiveにあります。[保存索引](evidence/task048-20261114/EVIDENCE-INDEX.json)、[最終ソース](evidence/task048-20261114/FINAL-SOURCE.json)、[レビュー](evidence/task048-20261114/REVIEWS.md)、[Schema](evidence/task048-20261114/schema-results.json)、[意味照合](evidence/task048-20261114/output-semantics.json)、[表示観察](evidence/task048-20261114/render-security.md)を参照してください。生ログ・原本・画像・差分ソースは`VERIFICATION.tar.gz`にまとめました。

ignoredは既存047の明示起動workerと隔離OpenSSH相互運用で、048の受入根拠には使っていません。CI、Windows、全通信状態、他ブラウザーは今回実行していません。commit・push・PR・releaseも行っていません。

runner開始後のRust実装/試験/Cargo/Schemaは不変です。途中で実装レビューを受けてPython集約とその試験を修正し、その最終版でPython77件と集約を実行しました。`APPLICABILITY.json`が初期台帳と最終ソースの違いを説明します。ブラウザー原本は観察時点のものを別保存し、その後の再生成によるprobe日時の差と混同していません。

作業中に現れたAGENTS.md・DECISIONS.md・PATTERNS.md・REVIEWS.mdの別作業変更は保持し、048の差分とレビュー対象から除外しました。
