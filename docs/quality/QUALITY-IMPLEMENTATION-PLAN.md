# 品質評価モデル・証拠集約の実装計画（TASK-041）

対象: 2026-11-11、開始HEAD `0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`。
これはTASK-041で実施したPhase 0とPhase 1の計画・完了記録です。今回の計画整理では追加実装を行いません。後続タスクは[ロードマップ](QUALITY-ROADMAP.md)で管理します。

## 設計判断

既存TASK-023の `run.py` / `evaluate.py` / 023-v2の正解データを再利用し、
測定済み成果物を読む開発用 `tests/quality/report.py` を追加します。
新しい評価エンジンや通常のCLI経路は作りません。JSON/Markdownの品質報告は
製品レポートと別の成果物です。依存追加、検出器・ルール・製品Schema変更はありません。

集計値の手作業での転記では継続測定に向かず、採点器を別に作ると正解との照合方法が分かれます。そこで既存採点器を呼び、測定原本、正解、反復結果との一致を確かめてから指標を計算します。
FPの所見数と、誤報があったNegative単位数を区別します。
アルゴリズム別の独立ラベルがない場合はNot Testedとします。

TASK-041の実装判断は、依頼された範囲に限ります。新しい数値ゲートを承認した記録ではありません。
調査・実装・試験は主エージェントが実施し、AGENTS.md指定のレビュー2名のみ利用します。

## ファイル責務

| 対象 | 責務 |
|---|---|
| `tests/quality/report.py`, `test_report.py` | 既存測定から指標・分母・限界・比較とMarkdown/JSON生成、整合性の負例試験 |
| `.github/workflows/pqc-scan.yml` | 既存baseline jobに報告生成・保存だけ追加 |
| `docs/quality/SELF-SCAN-EXPECTATIONS.json` | 新規レシピ文字列の3所見だけを既存方式で個別レビューし、rawを保持したCI projectionへ登録 |
| `docs/quality/QUALITY-MODEL.md` | Q1〜Q6と証拠状態の定義 |
| `docs/quality/QUALITY-METRICS.md` | 評価単位・式・未測定母数の定義 |
| `docs/quality/QUALITY-BASELINE.md` | 最新実測、過去との比較、再実行方法 |
| `docs/quality/QUALITY-ASSESSMENT.md` | 監査結果・残存リスク・受入判断・根拠参照 |
| `docs/quality/QUALITY-ROADMAP.md` | Phase 2〜6の独立した作業単位・依存・受入条件 |
| `docs/quality/QUALITY-REFERENCES.md` | 公式資料の版・状態・採用範囲 |
| `docs/quality/QUALITY-GATES.md` | 既存承認を保った状態判定・報告への参照 |
| `tasks/041-quality-assessment-model.yaml` | 開発用の単一タスク、ライフサイクル・証拠 |

## 実行手順

- [x] HEAD、clean状態、指示、品質資産・タスク・CIを読み、公式資料を調査する。
- [x] 既存のオフラインbuild/fmt/test/clippy/Python/Node/配布試験と023測定を新規出力先で実行する。
- [x] 手計算に基づく指標・不整合・重複・母数ゼロ・上書き防止のテストを先に失敗させる。
- [x] 既存採点器を再利用して最小実装し、既存baseline CIに統合する。
- [x] 定義・調査・実測・ロードマップを分離して文書化する。
- [x] 変更全体の実装レビュー1名、修正、最終試験、試験妥当性レビュー1名を行う。
- [x] 測定・コマンド・終了状態・ソースhash・制約を保存し、開発用受入を記録する。

## 検証焦点

同じNegativeへの重複FP、Positiveへの余分なFP、未注釈位置のFP、空の集計区分、
Ambiguous/Unsupported、異なる正解での比較、集計や測定原本の改ざん、
部分検査、失敗/timeout、時間変動、既存ファイル上書きの各条件を確認します。
既存の正解、ラベル、測定スクリプト、照合処理は変更しません。
製品のRust全テスト、fmt、Clippyを実行し、必要に応じ既存隔離protocol runnerを使います。
HTML変更はなく、今回の品質報告はMarkdownとJSONです。新しいブラウザー受入は主張しません。

成果物は新規 `target/quality-assessment-20261111/` に保存し、過去記録を上書きしません。
公開・push・PR作成・merge・tag・GitHub設定変更はこの作業では行いません。
