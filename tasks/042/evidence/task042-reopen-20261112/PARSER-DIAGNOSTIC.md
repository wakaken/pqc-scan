# C07の回復構文木を確認した結果

前回は補助Cプログラムのリンクに失敗し、構文エラー時の見逃しの詳細原因を確認できませんでした。今回はプロジェクトと同じRust 1.93.1、tree-sitter 0.26.6、Java grammar 0.23.5、Python grammar 0.25.0で、3入力の全ノードを出力しました。コマンドは終了0です。対象のPython/Javaコード自体は実行していません。

| 評価単位 | 実際の回復木 | 現行検出器で一致しない理由 |
|---|---|---|
| u027 | ERROR内に識別子とattributeが残る | `call`がなく、候補の識別子は`MLKEM768PrivateKey`等の断片になる。規則が要求する`.generate(`を含む全体は候補にならない |
| x011 | u027と同じくERRORとattribute、識別子が残る | `call`がなく、同じ理由で生成API全体を規則へ渡せない |
| x005 | ERROR内にidentifierとstring_literal/string_fragmentが残る | `method_invocation`がなく、`Signature.getInstance("ML-DSA-65"`全体を規則へ渡せない。文字列単独では当該規則に一致しない |

検出器はERROR、Pythonのattribute、Javaのprogram全体を候補にしていません。`tree_sitter_detector.rs`の`is_semantic_node_kind`と全ノードの記録を照合すると、今回3例の見逃しを説明できます。構文エラーをPartialと報告する動作自体も、今回の実測で保持されています。

これは破損した入力からどこまで所見を残せるかという検出範囲の制約です。正常入力のパーサー全般が故障しているという結果ではありません。ERROR全体の検索を加える修正は、文字列の誤検出や上限処理に影響するため、この評価作業では行っていません。

前回のSDKエラー記録はそのまま残しました。同じバージョンをCargoでビルドする方法で今回の診断は成功しており、以前の環境制約を今回の未完了理由として引き継ぎません。今回の実行だけから、前回のCリンカーやSDK自体が修復されたとは判断しません。

実行コマンド・終了値は`PARSER-COMMAND.json`、全ノードは`parser-diagnostic.log`、診断用ソース・固定依存は`parser-diagnostic/`に保存しています。原記録は`docs/quality/task043/TRIAGE.json`のC07です。今回の追補で詳細原因を確認しましたが、過去のPartial記録は上書きしません。
