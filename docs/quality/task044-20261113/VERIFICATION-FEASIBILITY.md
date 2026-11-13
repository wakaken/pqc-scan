# Q4-16 Miri・形式検証の適用判断

Linux arm64の同じ隔離環境で、固定nightly 2026-02-12のMiriを試しました。工具とMiri sysrootはネットワークを使える準備段階で取得し、対象ソースの実行は外部通信なし・非root・容量/CPU/メモリ/時間制限下で行っています。Rust 1.93.1という製品toolchainは変更しません。

## 実際に試した範囲

| 試行 | 実行結果 | 読み取れること |
|---|---|---|
| `cargo +nightly-2026-02-12 miri test -p pqc_scan_core --lib identity::tests --locked --offline` | 終了101。proc-macro2のbuild scriptでMiri用JSONを読めず停止 | identityのMiri試験が成功したとはしない。製品UBを検出した結果でもない |
| 元の`safe_token`/`valid_scope`をそのまま使うstd-only crate | 終了0。accepted/rejected/length_boundaryの3試験成功 | 正負のscope/token、区切り・URL・日本語・128/129 byte境界という有限の入力で実行できた |
| 正常な配列アクセスの工具対照 | 終了0 | 単なる工具起動不能ではないことを確認 |
| 合成null pointerの`read_volatile`による負対照 | 終了1。`Undefined Behavior`を検出 | 危険なアクセスを成功扱いにしないことを確認。製品の欠陥ではない |

小範囲試験は`crates/pqc_scan_core/src/identity.rs`の`safe_token`開始から`relative_path`開始直前までの原バイトを取り出しています。関数を書き直した模倣実装ではありません。元ファイルhash・byte範囲・断片hash・追加したテスト全文を`miri-final/miri-source.json`に記録し、最終ソースと一致することを確認しました。Miriのログには3件の成功と終了0があり、事前のcore経由の失敗は別に保存しています。

branch-02時点の`feasibility.json`のscope説明には以前のcore試行名が残っていました。原記録は変更せず、最終driverの説明を直して`miri-final`を新しく実行しました。最終集計はこの最終試行を参照しています。過去のJSONやhashを付け替えていません。

## 適用範囲と限界

| 対象 | 判断 | 理由と代替検証 |
|---|---|---|
| 依存の少ないscope等の純Rust helper | 小範囲では適用可能 | 今回の3試験で実行を確認。原ソースへの束縛と正負対照を保ち、必要な入力だけを選ぶ |
| core全体を入口にするMiri | 今回の固定環境では実行経路に制約 | build scriptで停止した原ログを保持。製品から依存やチェックを取り除いて通すことはしない。局所試行と既存native試験を使い分ける |
| tree-sitter・aws-lc等のFFI内部 | 今回のMiri試行の対象外 | Rust wrapperの到達とC/assemblyの安全性は別。既存native境界試験、047のfuzz、057の故障表等が候補 |
| filesystemの`openat`/Win32、network、process cancellation | 純Rust試行では未検証 | kernelやOSの挙動を局所Miriで代替しない。今回のnative試験と、今後のOS/隔離protocol試験を使う |
| 並行実行・全入力・全unsafe・暗号の健全性 | 保証しない | 有限の実行と1つのMiri設定だけでは一般のsoundnessや暗号学的安全性を証明できない |

Miriのforeign functionやsoundnessの限界は[公式資料](https://github.com/rust-lang/miri)とも照合しました。isolation、UB検査、provenance検査を無効にする旗やnative FFI bypassを使って成功させてはいません。今回のhelperには直接のunsafe処理がないため、追加の欠陥検出力は限定的です。全workspaceや必須CIへの一律導入を勧める結果ではありません。

## 形式検証

Kaniを候補として検討しました。[公式の配布対象](https://model-checking.github.io/kani/install-guide.html)はx86_64 Linux、x86_64 macOS、arm64 macOSで、今回の隔離先arm64 Linuxはそのまま使える配布対象に含まれません。工具image内の`cargo kani --version`も終了101で未導入を確認しました。これはKaniがあらゆる環境で利用不能だという意味ではありません。ソースからの工具build、別archの隔離環境、macOS上での実行は今回試していません。実測していない導入費用を「費用過大」とも判定しません。

純Rustのscope/tokenの長さ・許容文字、SSH frame長のchecked arithmeticは形式検証の局所候補です。ただし、文字列長・loopのbound、整数幅、std/FFIやOSのモデルを固定しなければ、製品が扱う全入力の証明にはなりません。今回は形式的な証明結果を`Not Tested`として残し、実行できた局所Miriとnativeの正負/境界試験を代替として示します。工具の導入数を成果とは数えません。

## 費用と証拠

工具の新しいDocker layerには対象コードを入れず、Rust/LLVM/nightly/Miriとlocked cache、std sysrootを準備しました。利用したimage ID、Dockerfile、準備ログ、全実行コマンド・wall time・timeoutは[原記録](RAW-EVIDENCE.tar.gz)にあります。target実行の制限はメモリ6 GiB、追加swapなし、CPU 2、PID 128、scratch 4 GiBで、最後にcontainer削除も確認しました。

Miriの対象3試験のtest実行部分は約0.54秒でした。これは工具取得・sysroot準備・compileを含む総費用ではありません。取得とbuildも含む費用の評価は原コマンドの時間を別々に読んでください。長時間の全機能Miriや形式検証を完了したという主張はしません。

このQ4-16は、適用できる小範囲の試行と、適用できなかった経路・代替策の評価です。製品全体のUBやPQC判断の合格判定ではありません。機械集計は[MEASUREMENTS.json](MEASUREMENTS.json)、ソースとの照合は[SOURCE-MATCH.json](SOURCE-MATCH.json)から追跡できます。
