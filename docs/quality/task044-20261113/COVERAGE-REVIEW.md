# Q4-13 到達率と重要な未記録箇所

基点は`f8aff1888525f0f3e350a734166c12c6de3e71ed`、測定日は2026-11-13です。Linux arm64の既存workspaceテストバイナリを、通常設定と全feature設定でそれぞれ実行しました。Rust 1.93.1ではline・regionを測り、固定nightly 2026-02-12（Rust 1.95.0-nightly／LLVM 22.1.0）でbranchを補いました。各設定は1回の観測であり、測定のばらつきや一般的な品質の分布を推定していません。

## 集計値

次の表はresolverの集約が曖昧な1ファイルを分離した集合です。通常設定ではそのファイルは元からfeature対象外です。全featureでは当該ファイルの121 line・199 region・10 function、nightlyでは20 branchカウンターを分離します。所見や正解の除外ではなく、測定可能範囲の分離です。全ファイルを含む原集計も下に残します。

| 設定 | line | region | branch | 成功 / ignored |
|---|---:|---:|---:|---:|
| Rust 1.93.1 通常 | 19,549 / 21,265（91.93%） | 31,626 / 34,661（91.24%） | 未測定 | 514 / 0 |
| Rust 1.93.1 全feature・分離後 | 20,671 / 22,464（92.02%） | 33,409 / 36,683（91.07%） | 未測定 | 565 / 1 |
| nightly 通常 | 19,549 / 21,265（91.93%） | 31,626 / 34,661（91.24%） | 3,938 / 5,334（73.83%） | 514 / 0 |
| nightly 全feature・分離後 | 20,671 / 22,464（92.02%） | 33,409 / 36,683（91.07%） | 4,063 / 5,514（73.69%） | 565 / 1 |

resolverを含むLLVM原集計は、Rust 1.93.1全featureでline 20,782/22,585・region 33,578/36,882、nightly全featureでline 20,749/22,585・region 33,534/36,882・branch 4,077/5,534です。原JSON、分離後の分母、未収録ファイルは[MEASUREMENTS.json](MEASUREMENTS.json)にあります。branchの分母はLLVMのbranchカウンターであり、条件数やMC/DC達成数へ読み替えません。

分母は`crates/*/src/`にマップされたソース位置です。依存crateと独立した`tests/`は含みませんが、`src`内のインラインテストは含みます。コンパイルされない処理やマップのない型定義もあるため、全製品コードの網羅率ではありません。今回の実行は`cargo test --workspace --no-run`が列挙したバイナリで、doctestは含みません。

## resolverの測定制約

`crates/pqc_scan_cli/src/probe_resolver.rs`はCLIと`#[path]`のintegration testモジュールとしてコンパイルされます。全featureの成功ログには`self_executable_resolves_once_and_authorizes_all_local_answers`がありますが、nightlyのfile合算では同テストが呼ぶ`for_executable`等が0カウンターになります。Rust 1.93.1との33 line・44 region・2 functionの差はこのファイルだけです。これを製品退行や、そのテストが未実行だった証拠にはしません。複数の定義を同一source位置に集約した結果から個々の実行を断定できないため、上の集合から分離しました。

さらに、resolver子プロセスは製品の安全境界として`env_clear()`を使います。`LLVM_PROFILE_FILE`も渡らないので、子内部の実行カウンターは今回の収集先へ届きません。kill・異常終了でも終了時profileが残らない場合があります。収集のためにこの境界を緩めていません。差の唯一の原因を特定したという主張はせず、原出力と成功ログを保存しています。

## 安全上重要な未記録箇所

以下は全featureのRust 1.93.1でも0だったregion-entryの開始位置です。行全体の未実行やbranchの未到達とは同一視しません。[全開始位置](UNOBSERVED-COUNTERS.json)と原source-coverageから対象を追跡できます。

| ソース位置（行:列） | 確認できた不足・対象外理由 | 既存試験との関係・適切な次の検証 |
|---|---|---|
| `crates/pqc_scan_core/src/risk.rs` 33:27、34:26 | Info→Low、Low→Mediumの昇格結果が未記録 | 同ファイルのrisk試験と対照し、045の重要判断・変異試験の候補とする。暗号判断を今回変更しない |
| `crates/pqc_scan_core/src/privacy.rs` 337:20、345:24 | line=0、column=0時の保守的な秘匿判定のregionが未記録 | `evidence_privacy`の成功だけで全分岐を保証せず、057の位置不明・混合位置の故障表で確認する候補 |
| `crates/pqc_scan_core/src/safe_read.rs` 7:56、13:54、53:20、55:74/55:84 | root外のstrip_prefix失敗、一部の事前metadata拒否、root directoryのopen失敗、NUL component変換のregionが未記録 | `safe_read`の読取上限とlink/FIFO試験は実行済み。これら追加の負例とOS差は057の候補 |
| `crates/pqc_scan_detectors/src/certificate_detector/pkcs12.rs` 109:17、110:17、120:28 | ContentInfo件数制限・一部の破損safe contentsの回復経路が未記録 | `pkcs12`試験の有効/暗号化/破損例を保持し、047の上限付き入力探索・057の故障例の候補とする |
| `crates/pqc_scan_detectors/src/dependency_detector/decode.rs` 127:21〜130:21 | TOMLのdotted key上限のregionが未記録 | 既存dependency試験の成功とは分け、047の深さ・幅・引用内の点の境界候補とする |
| `crates/pqc_scan_core/src/target_policy.rs` 329:9、333:73、335:20 | 登録済みsocket、lock失敗、登録時cancel等のregionが未記録 | 既存transport試験と結び、057の競合・故障注入で確認する候補。外部接続は行わない |
| `crates/pqc_scan_probe_ssh/src/parser.rs` 48:58、51:52 | u32→usize変換と`+4`の失敗region | この64-bit targetではu32と4の加算がusizeへ収まる。32-bitでは意味が異なるので削除せず、今回の数値から他archへ一般化しない |

これは既存タスクへの検討材料であり、新しいタスクを作成したり、045・047・057を開始した記録ではありません。未記録箇所の修正は044の受入条件ではありません。

## 分母に入らない範囲と警告

- 通常設定ではCLI probe command/resolver、core probe transport、TLS runtime、SSH runtime/negotiatedはfeature対象外です。全featureではマップを取得しました。
- 全featureでも`core/src/error.rs`、TLS/SSHの`lib.rs`はマップに現れません。型・derive・re-export等に対して実行可能位置の分母を捏造しません。
- `safe_read.rs`のWindows実装とUnix/Windows以外のfallbackは今回のLinuxではコンパイルされません。Windows、macOS、32-bitの実測を行ったとはしません。
- tree-sitterとaws-lc等のC/assembly、依存crate、生成されないgeneric実体は今回のRust source集計の外です。FFIの内部安全性はこの数値から判定できません。
- OpenSSH daemon/keygenを必要とする明示実行試験1件はignoredです。049の新しい隔離protocol matrixを今回実施したとは扱いません。
- LLVMが出したmismatch警告は、stableで6/8件、nightlyで3/4件です。`-dump`の記録から全て依存crate `memchr::cow::CowBytes`の関数であることを照合しました。警告を消して合格とせず、対象外理由と原診断を残しました。

ソース・工具・実行ファイルhash・コマンド・終了値・kernel制限・原出力は[RAW-EVIDENCE.tar.gz](RAW-EVIDENCE.tar.gz)、最終sourceとの照合は[SOURCE-MATCH.json](SOURCE-MATCH.json)にあります。各設定のbuildとテスト、merge/exportは終了0です。Rust 1.93.1のbranchは`Not Measured`で、0%ではありません。到達率の高さを安全性やテストassertionの十分性へ読み替えません。
