# TASK-043の修正・検証結果

対象ロードマップ: **pqc-scan v0.3**（品質評価・検証基盤の包括的強化。[位置付け](QUALITY-ROADMAP.md#v03-scope)）。

043を再開し、ユーザーが指定したFP4件・FN40件に関するP1/P2の修正を実施しました。他タスクへの延期はしていません。全44件について、同じ入力・同じ正解で修正前の誤りと修正後の解消を確認しています。[全件台帳](task043-fixes-20261112/REPAIRS.md)と[位置・入力hash・試験の詳細](task043-fixes-20261112/REPAIRS.json)を保存しました。

## 何を直したか、その理由

| 修正 | 理由 | 元の対象観測 |
|---|---|---:|
| Nodeの実際の呼出し、別名import、変数・引数の参照先を確認し、SSHのコメントを除外。Javaの設定書式を追加 | 説明文字列や同名関数を暗号APIと誤認せず、明示した設定を取りこぼさないため | 6 |
| Java/RubyのSLH-DSAのパラメータ違い、旧称の大小文字、Java/Pythonの破損構文を処理 | 正しい方式名・明示参照を残し、不正な値は方式へ変換しないため | 5 |
| Gradle/gemspec、CIRCL、各SBOM形式の依存を処理 | 静的な依存宣言とpackage/ecosystemの組を認識するため。PQC機能の有効化とは区別する | 10 |
| RSA鍵、Python/Rustのimport・型・receiverを局所的に追跡。未完了Rust useも保持 | 鍵・型の来歴が分かる参照を見つけ、変数名だけで暗号を推測しないため | 14 |
| Nodeの有限配列・タプルによるfor...ofを処理 | 明示した有限の方式候補を認識し、任意コードの実行や無制限な伝播を避けるため | 9 |

レビューで見つかったdefault/rest/型付き引数、Pythonのloop・with・except・別module import、gemspecのblock外参照と再代入、Rustの後置itemの遮蔽も同じ043で修正しました。解析上限時のTruncated伝達も試験しています。[検出仕様と制限](TASK-043-FIXES-CONTRACT.md)に具体的な範囲を記載しました。

CLI、公開型・Schema、規則ID/kind、Risk/Severity/Confidenceを維持しています。新規依存はありません。組込判定は既定pattern/scope/kindに限定し、カスタム規則は従来の汎用matcherを使います。所見の数・位置・証拠・IDは修正に応じて変わります。

## 比較結果

測定対象は`codex/quality-assessment-model`、基点HEADは`0a281e90d4e7e0402e2dd5d0aa19a7249c5373bf`です。未コミットの041/042等の変更を含む作業開始状態を保存し、その状態からの差分を測定しました。HEADだけで測定版を特定せず、[製品差分](task043-fixes-20261112/PRODUCT-DELTA.json)、[測定hashと集計](task043-fixes-20261112/METRICS.json)、[開始状態](task043-fixes-20261112/ENTRY.json)を併用してください。

| 評価対象 | TP | FP | FN | TN | 適格分母 |
|---|---:|---:|---:|---:|---:|
| 元の042-v2対応範囲・修正前 | 58 | 4 | 40 | 87 | 189 |
| 元の042-v2対応範囲・修正後の旧注釈との一致 | 98 | 2 | 0 | 91 | 189 |
| 注釈を補った043-v2・同じデータで修正前を再測定 | 61 | 4 | 42 | 90 | 197 |
| 注釈を補った043-v2・修正後 | 103 | 0 | 0 | 94 | 197 |

旧042-v2の修正後に残るFP2は、RSAの検証オブジェクトの生成と検証呼出しの未注釈参照です。二者がソースだけから正当な参照と確認し、別版へ2単位を追加しました。元のFP4件はすべてTN、FN40件はすべてTPです。検証の検出を抑えて件数を合わせてはいません。

043-v2は206ファイル・255単位です。197単位を判定し、53単位はUnsupported、5単位はAmbiguousとして保持しました。対応外の暗号化PKCS#12属性6単位は042で承認済みの公開対応範囲に従います。元の対象44件を除外・ラベル変更してはいません。

調整用の継承データはTP100/FP0/FN0/TN91です。新しい第三者由来の7質問はTP3/FP0/FN0/TN3、曖昧1です。auth0/node-jsonwebtokenとBouncyCastleの、スキャナー開発前に公開されたソースを使いました。質問の選択には偏りがあり、同一系統・同じ行の複数規則は相関します。この小さな成績から製品全体の精度や正答確率を推定しません。

旧57単位も変更せず測定しました。固定注釈との生スコアはTP28/FP4/FN2/TN19、旧数値比較は**Failed**のまま保存しています。独立したソース確認では、Python importとXML設定の開始列による2組のFP/FN、未注釈の別名import、Ruby literalの定義差に帰属しました。新しい製品欠陥は確認されていません。旧比較をPassedへ書き換えず、品質ゲートも緩和していません。この調査はリリース承認を代替しません。

## 正解と出典の管理

新9問は二者がソースから判断し、最初の測定前に固定しました。継承246問は元ソースを確認した後、既存の独立正解を参照しており、全255問の新しい、出力を見ない独立注釈とはしていません。二者は全206ファイル×指定30規則の6180組を確認しました。元の042のデータ・適用範囲・過去の記録を含む343ファイルの不変性も照合しました。

最初の043-v1では、コネクタの応答を保存した際に末尾の改行（LF）が1つ増え、保存コピーのhashを元のupstream hashと表記してしまいました。旧版と測定を保持し、043-v2で両hashと変換を区別しました。[公開されたcommitの原バイトを再取得した照合](task043-fixes-20261112/UPSTREAM-HASH-VERIFICATION.json)も成功しています。検体・質問・正解は不変です。

再評価時のレビュー担当は既に実装や歴史的結果を見ています。その履歴を明示したまま、元の独立注釈記録との完全一致を既存評価器で検証します。入力・質問・定義・正解・個々のレビューの変更、原本hashの不一致、再評価の連鎖は拒否します。JSON/Markdownにも「新しい独立試行ではない」と表示します。最初のmetadata再評価テストの失敗は一時ディレクトリのsymlink解決漏れによるもので、TDD成功とは数えません。その後の正しいfixture、改変対照、旧guardの再構成による回帰対照を区別して保存しています。

## 実行と証拠

変更前は保存したソース・規則・バイナリを使う専用snapshot、変更後は現在の作業ツリーで実行しました。評価器・入力・正解のhashは一致し、バイナリと製品ソースのhashは異なります。各3反復の意味・診断・順序・成績は安定していました。検体の実行、依存DSLの評価、鍵生成、外部サービスのprobeは行っていません。

主要な再実行コマンドは次のとおりです。出力先は毎回新しいディレクトリを選びます。

```sh
cargo fmt --all -- --check
cargo test --workspace --offline --locked
cargo clippy --workspace --all-targets --offline --locked -- -D warnings
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests/quality -p 'test_*.py'
node --test shell/*.test.cjs
python3 tests/quality/run.py --corpus-dir tests/quality/task043-corpus-v2 --out-dir target/new-043-measurement --build-profile release --repeats 3 --skip-benchmarks
python3 tests/quality/report.py --corpus-dir tests/quality/task043-corpus-v2 --baseline target/new-043-measurement --out-dir target/new-043-assessment
```

[変更前コマンドと終了値](task043-fixes-20261112/COMPLETE-BEFORE-COMMANDS.json)、[変更後コマンドと終了値](task043-fixes-20261112/COMPLETE-AFTER-COMMANDS.json)、[生成した品質報告](task043-fixes-20261112/FINAL-quality.md)から原出力を追跡できます。検出器28テストとCLI1テストは、新経路の正負例、束縛・上限、Unicode列、4参照のbyte span、8成果物への出力と合成credentialの全体・中間断片の秘匿を確認します。既存のSchema・ID・出力互換性・公開証明書の試験もworkspaceに含まれます。

今回の測定は精度・再現性用です。性能ベンチマーク、hosted CI、全OS、任意の言語構文、全暗号形式、実通信、全Q1〜Q6の品質保証はこの043の受入対象ではありません。自動報告の軸全体のPartial/Not Testedと、043で実施を約束した作業の完了は別々に照合します。リリース判定はUnknownのままです。

最終のローカル検証は、workspace Rust385件、品質Python54件、配布試験10件、Node31件が成功しました。protocol実行基盤は5件成功・6件スキップです。Nodeはreport.pyの有限な自己スキャン許可のsource hashを更新した後に31件を再確認しました。前の失敗ログも保存しています。[検証台帳](task043-fixes-20261112/VALIDATION.json)にコマンドと終了値を記録しました。

自己スキャンは1595所見中1241件が既存の有限な確認済み所見、354件が可視のまま残っています。抑制の追加や自己スキャンを「問題ゼロ」とは報告しません。今回変更したのは既存許可のsource hash3件だけです。原SARIFと変換後を別々に保存しています。
