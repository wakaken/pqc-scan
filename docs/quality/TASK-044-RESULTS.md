# TASK-044 到達率と追加検証手法

対象はv0.3のQ4-13・Q4-16です。[044の当初条件](../../tasks/044-quality-coverage-feasibility.yaml)と[既存ロードマップ](QUALITY-ROADMAP.md)を使い、到達率と追加検証の適用範囲を調べます。検出精度の再評価、未到達箇所の製品修正、mutation・fuzz、リリース判定は行いません。

測定の基点は`f8aff1888525f0f3e350a734166c12c6de3e71ed`、ブランチは`codex/quality-assessment-model`です。測定用Pythonとタスク記録の未コミット差分を含みます。製品Rust・Cargo・規則・公開Schema・CIの挙動は変更しません。測定対象はHEADだけでなく、sourceBindingsと実行ファイルのSHA-256で特定します。

## 測定の方法と分母

Rust 1.93.1の`-C instrument-coverage`と同じtoolchainの`llvm-profdata`・`llvm-cov`を使います。通常設定と全feature設定でそれぞれ`cargo test --workspace --locked --offline --no-run --message-format=json`を実行し、Cargoが列挙した全テストバイナリを実行します。CLIの子プロセスにも別のprofileを書かせ、実際の実行ファイルもhashで照合します。doctestを含む通常の`cargo test`全体と同じ実行範囲とは扱いません。

profile、対象バイナリ、集計を設定ごとに分けます。通常設定の測定を閉じた後、同じbuild cacheで全featureを追加し、新しいraw profileだけで再集計します。分子と分母はLLVMが出力したline・region・functionの値です。regionをbranchの代わりには使いません。branchは固定nightlyで通常設定・全featureのworkspaceを測り、Rust 1.93.1とは別条件として報告します。先行したcore内identityだけの小試行も原記録に残します。

集計対象は`crates/*/src/`のRustソースです。依存crate、独立した`tests/`、工具、C/FFI実装は除外します。`src`内のインラインテストは分母に残ります。型定義だけのファイル、無効feature、対象OSでコンパイルされない処理、生成されないgeneric実体は、単純な「未到達」とは区別します。未収録ファイル一覧も保持します。

[公式rustc資料](https://doc.rust-lang.org/rustc/instrument-coverage.html)の方式を使います。数値は今回のソース・テスト・Linux arm64・build設定に対する観測で、製品全体の安全率でも、全プラットフォームの網羅率でもありません。新しい数値ハードゲートは設けません。

## 隔離と工具の取得

既存の`tests/protocol/container_runner.py`と`container_entry.py`のsource snapshot、kernel境界確認、有限のartifact抽出、終了時のcontainer削除を再利用します。044専用caseだけを追加し、既存suiteやCIのprotocols停止条件は変えません。

対象コードを実行するcontainerは非root、読み取り専用source/root、空の環境、ホストHOME・認証情報なし、外部通信なしです。メモリ6 GiB・追加swapなし、CPU 2、PID 128、scratch 4 GiB、共有メモリ1 MiBとhost deadlineを使います。実際のkernel値とsource不変性、container削除をHOST-RESULTS.jsonとchecks.jsonで確認します。

既存2テストが`CARGO_TARGET_DIR`ではなくrepository直下の`target/`へ一時規則を書くため、044のsnapshotだけに`target -> /scratch/test-target`を設けます。tracked sourceの読み取り専用は維持し、一時出力も同じ容量上限に収めます。元のテストと製品コードは変更しません。

工具準備のDocker layerには現行manifestと空stubだけを渡し、locked cacheを取得します。Rust/LLVMと固定nightly/Miri、Miri用sysrootの準備も対象コードの実行から分けます。実行段階は`--network none`・`--locked --offline`です。

## 失敗した試行の扱い

最初の試行は既存tool imageのCargo cache不足で`ahash`取得が必要となり、offline制約によってbuild前に停止しました。次の試行は既存テスト2件の一時書込先が読み取り専用で失敗しました。いずれも製品判定の退行やテスト成功とは扱いません。

一時出力先を調整した後の通常設定では514件が成功しましたが、LLVMの診断がJSONの先頭へ混入して集計が停止しました。機械データとstderrを分離し、診断を隠さず保存するよう測定器を修正しました。これらの原ログは`target/quality-task-044/run-20261113/measurement*`に別々に残します。


## 測定結果と限界

当初のQ4-13の成果物は[到達率の評価](task044-20261113/COVERAGE-REVIEW.md)、Q4-16は[追加検証の適用判断](task044-20261113/VERIFICATION-FEASIBILITY.md)です。指定された`target/quality-task-044/coverage-review.md`と`verification-feasibility.md`も作成しています。

Rust 1.93.1の通常設定はline 19,549/21,265（91.93%）、region 31,626/34,661（91.24%）でした。全featureは、集約が曖昧なresolverファイルを分離した集合でline 20,671/22,464（92.02%）、region 33,409/36,683（91.07%）です。分離前の原集計も保持しました。固定nightlyのbranchは通常3,938/5,334（73.83%）、全featureの分離後4,063/5,514（73.69%）でした。stableのbranchを0%には置き換えていません。

各compilerで通常514件、全feature565件のテストが成功しました。全featureのOpenSSH明示試験1件はignoredです。Unix以外の処理、32-bit、C/assembly内部、doctest、明示protocol matrixは今回の到達率の分母に含みません。インラインテストが分母に入ること、resolverの多重コンパイルと子processのprofile欠落を明記しています。

Miriのcore経由試行はproc-macro2のbuild scriptで停止しました。元のscope検証helperを原バイトのまま使うstd-only crateへ切り分け、最終試行では正例・負例・128/129 byte境界の3試験が成功しました。正常対照は終了0、合成null pointerの負対照は終了1でUBを検出しました。全identityや全unsafeの保証とはしません。Kaniの候補・選定環境での配布制約・未試行部分・代替を記録し、形式的証明が成功したとは扱いません。

## 保存した証拠

[機械集計](task044-20261113/MEASUREMENTS.json)、[未記録カウンター位置](task044-20261113/UNOBSERVED-COUNTERS.json)、[195保護ファイルの最終照合](task044-20261113/SOURCE-MATCH.json)、[工具準備と失敗を含む原記録](task044-20261113/RAW-EVIDENCE.tar.gz)を保存しました。保護対象はRust・Cargo・規則・Schema等で、基点HEADおよびstable/branch/Miriの3試行に一致しています。異なるdriver版の原hashはそれぞれのHOST-RESULTSに残し、後の修正を古い版へ付け替えていません。

一時領域にprofileを溜め込む方式はartifact上限に達したため、テストバイナリごとにmergeし、raw profileのhashと圧縮した最終profdataを保存する方式へ変えました。失敗したmeasurement-04/05ではbulk exportが拒否され、内部例外の詳細を取り出せていません。同じJSON汚染はmeasurement-06で詳細を保存して確認しました。推定と直接確認を区別し、失敗を成功へ書き換えていません。branch-01のdriver構文失敗も保存しています。

stableの最終実行では、全featureのlink後にscratchの残量を確認し、完了済みのbuild cacheと、先行設定で測定・hash照合済みの古い実行ファイルだけを削除しました。操作はdisk-maintenance.jsonとretired-default-binaries.jsonに記録しました。最終driverは同じretirementを自動化しており、その経路はbranchの全featureで実行済みです。製品ソース、現設定の実行ファイル、過去の保存済み測定は削除していません。

この調査の完了は、製品全体の品質やリリース可否の合格とは区別します。高いline到達率でもseverity昇格・位置不明時の秘匿・parserの上限等に未記録領域があり、既存の後続タスクの検討材料として残しています。後続タスクは開始していません。GitHub Actions、mutation、fuzz、別OS、Miriの全workspace、形式的証明は新たに成功した検証へ含めません。


## 受入

[実装レビュー](task044-20261113/IMPLEMENTATION-REVIEW.md)はP2の位置説明1件を訂正したR2で受入、[試験妥当性レビュー](task044-20261113/TEST-REVIEW.md)は別担当がR3で受け入れました。対応が必要な指摘は残っていません。最終Python57件、protocol単体5件成功/6件skip、fmt、構文、task YAML、145ローカルリンク、原証拠hashを確認しました。Node35件・配布11件の成功は関連ソースが変わっていないため再利用しています。[検証台帳](task044-20261113/VALIDATION.json)と[最終チェック原本](task044-20261113/FINAL-CHECKS.tar.gz)を参照してください。

Evaluatorは当初の2条件と指定成果物を照合し、044をdoneとしました。[受入対応](task044-20261113/ACCEPTANCE.json)に確認範囲を保存しています。主担当が最後に加えたのは受入・review ID・状態の記録であり、受入済みのコード・測定値・評価本文は変えていません。045以降への着手、commit、push、PR、merge、tag、releaseは行っていません。

## 後続のMiri追加調査

ユーザーの追加指示により、固定工具でのbuild script停止を再現し、host/target runnerの分離で解消しました。元のcrate構成で実行範囲を広げた結果・未検証理由・追加分のレビューは[TASK-044 Miri追加調査](TASK-044-MIRI-FOLLOWUP.md)を参照してください。上記の当初測定・受入と`task044-20261113/`の証拠は保持しています。

## 現行の計測経路

Miri専用実装の退役とcoverageの独立化は[計測基盤の整理](TASK-044-RETIREMENT.md)を参照してください。この文書の測定・受入は当時の条件の記録として保持します。
