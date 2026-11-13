# TASK-044 Miriビルド停止の追加調査

2026-11-13、Miriの停止原因を解消し、元のcrate構成で実行できる範囲を広げる追加依頼を受けました。以前の044の評価・受入・原ログは`task044-20261113/`に保持します。今回の実行原本は`target/quality-task-044/miri-followup-20261113/`です。永続保存先は[追加調査の証拠](task044-miri-followup-20261113/)です。基点HEADは`f8aff1888525f0f3e350a734166c12c6de3e71ed`、既存ブランチは`codex/quality-assessment-model`です。開始時に前回044の未コミット差分があり、保持しています。

## 確認できた根本原因

失敗した固定工具はRust/Miri `7057231bd7`、Cargo `0c9e687d2`、nightly 2026-02-12です。`proc-macro2`のbuild scriptがELF実行ファイルとして生成された後、Miri runnerへ渡され、JSONとして読まれて停止していました。

同じ工具・新しいtarget directory・offline条件で、依存のない最小crateを比較しました。`build.rs`なしは終了0、`build.rs`を加えるだけで終了101となり、同じinvalid JSONエラーを再現しました。生成物のELF magic `7f454c46`も確認しました。古いキャッシュや、proc-macro2固有の非対応と推測して済ませていません。

この組合せでは次の処理が衝突します。

1. [固定版Miri](https://github.com/rust-lang/rust/blob/7057231bd78d6c7893f905ea1832365d4c5efe17/src/tools/miri/cargo-miri/src/phases.rs)は、targetに`cfg(all()).runner = cargo-miri runner`を設定し、解釈対象の実行情報をJSONに保存します。一方、hostのbuild scriptは通常のELFにコンパイルします。
2. [固定版Cargo](https://github.com/rust-lang/cargo/blob/0c9e687d237ff04b53ccb67b4ce63e9483789e88/src/cargo/core/compiler/compilation.rs)の`host_process`も`target_runner(Host)`を使い、汎用cfg runnerへ到達します。
3. その結果、通常のbuild scriptまでMiri用JSONのrunnerへ渡ります。Miriの`phase_runner`はJSONを想定するため読み込めません。

対処はhost runnerを明示し、targetのrunnerと分離することです。今回のhostはLinuxコンテナー内のビルド側であり、Mac上でbuild scriptを直接実行する意味ではありません。

```sh
cargo +nightly-2026-02-12 miri test \
  -Zhost-config -Ztarget-applies-to-host \
  --config 'host.runner=["/usr/bin/env"]' \
  -p pqc_scan_core --lib identity::tests --locked --offline
```

Cargoの2つの`-Z`フラグを指定するだけでは終了101のままです。host runnerを明示すると最小crateは終了0となり、元の`pqc_scan_core`のidentityテスト2件も約3秒で成功しました。製品ソース、依存の版、Cargo.lock、Rust 1.93.1という製品toolchainを変える必要はありませんでした。設定は固定工具向けの検証コマンドに閉じています。

## 実行証拠の取り方

元のworkspaceを`--all-features --no-run`でMiri用にビルドし、63テストtarget・566件の登録を確認しました。関数の抽出やテストの書き直しではありません。libtestのJSONで開始・成功・失敗・ignoredを追い、途中停止したbatchの未実行テストを次のbatchへ渡します。batchが終了1でも、その前に明示的なokイベントが出たテストだけをPassedとし、残りへ成功を一般化しません。

直接runnerを使う際はCargoの実行環境とcrateごとの作業ディレクトリを保持します。Miriの内部`MIRI_BE_RUSTC`の復元規則も同版ソースで照合し、正常対照と合成null pointerのUB負対照を直接runnerでも確認しています。native実行へ置き換えてテストを通してはいません。

全実行は既存の非root・read-only source/root・外部通信禁止・空の環境・メモリ6 GiB/追加swapなし・CPU2・PID128・scratch4 GiBのコンテナー内です。timeout時にはprocess groupを停止し、subreaperで子孫を回収します。子を残す対照で実際の回収も確認しました。

## 標準設定とOSアクセス設定

標準Miri設定では566件の登録をすべて列挙でき、120件がPassed、441件がUnsupported、4件が時間上限、1件が既存のignoredでした。終了1のbatchの中にも明示的なokがあるため、batchの終了値とテストごとの結果は区別しています。時間上限4件はテスト開始後に完了を観測できなかったPartialです。原JSONの旧ラベル`Not Tested: timeout`は変更せず、[正規化した結果](task044-miri-followup-20261113/DEFAULT-RESULTS.json)に対応を記録しています。

Unsupported441件のうち396件は、Miri内部のOS隔離によりstatx・mkdir・clock_gettime等を利用できないという診断でした。この396件だけを候補として、同じ元crate・feature構成・登録名を照合し、`MIRIFLAGS=-Zmiri-disable-isolation`で追加試行しました。別条件の結果として保持し、標準条件の成功へ付け替えません。

このフラグはMiriからコンテナー内のファイル・時刻・乱数等へのアクセスを許します。[固定版の公式説明](https://github.com/rust-lang/rust/blob/7057231bd78d6c7893f905ea1832365d4c5efe17/src/tools/miri/README.md)と照合しました。外側のコンテナー境界は維持し、Macのホームや秘密情報は渡しません。UB・借用・provenance・leak検査の無効化、FFIのnative実行への切替は行っていません。build scriptとproc macroはビルド側のnative実行であり、Miriで検査された扱いにはしません。

標準設定で残った45件の非対応はsocket28件、aws-lcのnative関数10件、tree-sitterのnative関数4件、aarch64 intrinsic3件です。OSアクセスを許可するだけで対応するものとは扱いません。時間上限は、比較出力golden、178k候補生成、証明書署名shape、SARIF公式schemaの4件です。Miriで重い処理を有限時間内に完走できなかった事実を保持します。

OS試行の初回はCLIの2件がそれぞれ60秒の時間上限に達したため、中断して原ログを保持しました。次の試行ではcrate targetの順序をcore・report等からに変更し、CLIを後へ回しました。target内の実行順はlibtestに従います。試験内容や登録集合は変更していません。追加条件の実行予算は、列挙後1600秒、batch上限600秒、イベント進行なし60秒、外側2400秒です。

## 対照・証拠と未検証の境界

原因切分けの最終試行`diagnose-03`では次の結果でした。失敗は仮説を確認するための想定結果として保持し、製品試験の成功には数えません。

| 条件 | 終了状態 | 確認できたこと |
| --- | --- | --- |
| 元core・未修正コマンド | 101 | build scriptのJSON誤読を再現 |
| 依存なし・build.rsなし | 0 | Miri自体は起動可能 |
| 同じ最小crateにbuild.rsを追加 | 101 | 外部依存なしでも再現 |
| host関連の`-Z`フラグだけを追加 | 101 | flagだけでは分離できない |
| host runnerも明示 | 0 | native build scriptを正しく実行 |
| 元core・host runner明示 | 0、identity 2件成功 | 抽出なしの元crateで解消 |

range測定でも、同じbuild.rsの再現101／修正0を確認しています。加えて直接runnerの正対照0・null pointer UB負対照非0と診断文、Cargo経由のUB負対照、timeoutした親と子の回収を確認しました。登録0件や開始のみのテストを成功扱いする集計は拒否します。

[製品ソース照合](task044-miri-followup-20261113/PRODUCT-SOURCE-MATCH.json)の195ファイルはHEADと一致します。各実行の`HOST-RESULTS.json`は実際に渡したsnapshot全体と境界を記録します。default測定後のdriver変更はOS条件の追加、選択・実行順、libtestの非終端timeout通知への対応とtimeoutの表示分類です。defaultの原ログと測定値は保持しています。

今回Miriを実行したのはLinux/aarch64、固定nightly、all-features、既定seed、テストスレッド1です。featureを無効にしたdefault構成、別OS・別arch・複数seed/スケジュール、doctest、全入力、未到達分岐は今回の元crate Miri測定では検証していません。build script・proc macro・未対応FFIの内部もMiri検証済みには含めません。標準設定での成功とOSアクセス設定での成功は、設定ごとの有限な実行の観測です。

代替となる通常のRustテストは以前の044で実行したdefault514件／all-features565件成功・既存ignored1件の証拠を再利用します。今回新たに通常のworkspace全試験・clippyが成功したという報告ではありません。対応する製品・Cargo・Rustテストは変更しておらず、旧測定と今回のsourceを照合しています。未対応FFIの内部のUBや時間切れ部分を、通常テストの成功でMiri合格へ読み替えません。

Python品質試験61件、protocol unit試験5件成功・6件skip、構文チェックと既存タスクの要求対応の照合を行いました。hosted CIの実行証拠とは区別します。

途中の設定失敗・中断試行も[試行一覧](task044-miri-followup-20261113/TRIALS.json)と原ログに残しています。特に`range-01`はhost終了0でも登録0件・列挙63件失敗であり、製品の成功証拠には使いません。最終範囲の根拠は`range-03`と`os-range-02`です。

## 適用判断

今回のhost runner分離により、元のcrateでMiriを使うためのビルド障害は解消しました。まずidentity等の純Rustの既存テストを固定条件・明示フィルターで使うことは可能です。全面的なMiri採用やCI追加は今回行いません。

外部ライブラリのnative関数やOS機能の非対応は、Rustコードのバグを修正すれば必ず解消する種類の停止ではありません。元のテストが使う機能や安全上のファイルオープンフラグを取り除いて成功させることは避けました。時間切れの試験は、Miriの更新・対応targetの比較・時間予算の増加等を別途判断できるよう、名前・条件・実測時間を残します。未試行の組合せの成功や費用までは推定しません。

## 最終測定と完了確認

| 条件 | 登録・候補 | 成功 | Miri未対応 | 時間上限 | 未着手・除外 |
| --- | ---: | ---: | ---: | ---: | ---: |
| 標準Miri | 566 | 120 | 441 | 4 | 既存ignored 1 |
| OSアクセス許可の追加条件 | 396候補 | 23 | 46 | 23 | 追加予算終了で304未着手 |

OS追加条件では候補396件のうち92件を試行し、testコマンドの実測合計は約1600秒でした。未着手304件は、標準条件ではOS隔離で止まったことを確認していますが、OSアクセス許可条件では実行していません。未対応46件の理由は`pidfd_spawnp`24件、ファイルオープンのflags12件、`fchmod`5件、tree-sitter3件、aarch64 intrinsic1件、socket1件です。

成功した登録は条件をまたいで重複なく143件ですが、2条件は等価ではありません。両条件を混ぜて「標準Miriで143件成功」とは報告しません。件数はtarget別のテスト登録であり、同じ関数が別targetに登録されている場合も別件です。確認できた成功は、それぞれの条件・入力・seedの範囲に限ります。UB負対照は意図的な工具確認で、製品の件数に含めません。

[測定の集計](task044-miri-followup-20261113/MEASUREMENTS.json)、[標準の個別結果](task044-miri-followup-20261113/DEFAULT-RESULTS.json)、[OS追加の個別結果](task044-miri-followup-20261113/OS-RESULTS.json)でテスト名・状態・ログを確認できます。原ログ、コマンドと環境、工具版、実行時source bindings、終了状態は証拠フォルダーのarchiveと各JSONに保持しました。両最終試行でsourceの前後一致、63targetの列挙成功、実行情報の不変、コンテナー削除を確認しています。元crateのbuild停止の解消、所定条件での実行、残る未検証理由の整理に対して完了を判定します。全製品のMiri合格は主張しません。

最終チェックと同じ二者による実装・試験レビューを受け、停止原因の解消・元crateでの有限範囲拡大・未検証理由の明示を完了しました。[受入記録](task044-miri-followup-20261113/ACCEPTANCE.json)と[実装レビュー](task044-miri-followup-20261113/IMPLEMENTATION-REVIEW-R2.md)・[試験レビュー](task044-miri-followup-20261113/TEST-REVIEW.md)を参照してください。記載した未検証範囲は残っています。

## 現行実装との関係

この追加調査の結果と証拠は保持しています。Miri専用実装は後続の[計測基盤の整理](TASK-044-RETIREMENT.md)で退役対象としました。ここに示すコマンドとsource hashは過去の固定工具・測定条件の記録です。
