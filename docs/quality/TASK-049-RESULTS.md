# TASK-049 TLS/SSHの観測・失敗・認証境界

対象はHEAD `b0e386e9d8503cf422c310e85fbb2a429bfb57d2`と049の差分です。既存ブランチ`codex/quality-assessment-model`と[049の元の6条件](../../tasks/049-quality-protocol-matrix.yaml)を維持します。probeは実験的のままです。

## 実施方法

既存のTLS local peerとOpenSSH、非root・外部通信なしのstrict runnerを再利用します。製品の暗号処理、公開Schema、正式サポート、CI protocols停止設定は変更しません。実装は評価用試験・集約・証拠保存の追加です。

TLSではTLS1.2/X25519、TLS1.3/X25519・P256・P384・X25519MLKEM768を交渉します。hybridではHelloRetryRequestも確認します。同じ暗号スイートを使うX25519とP256を対照にし、スイート名からgroupを推測していないことを確認します。未知/欠落groupは内部アクセサーの単体対照でunknownを保ち、実際に未知groupとの接続が成功したとは扱いません。

証明書は信頼・名前・期限切れ・有効開始前を分け、mTLS要求、証明書個数/ライブラリmessage上限、接続拒否・時間切れ・中止・巨大/途中recordも区別します。検証に失敗した場合に交渉成功の事実を残さないことと、検証後の証明書収集制限では確認済みの交渉を消さずPartialにすることを区別します。

SSHではoffersとpin付きKEXの2モードに各13枠のraw peerを使い、peerが受けた全クライアントbytesからメッセージ種別を取り出します。古典/hybrid/unknown、分割frame、巨大長・途中切断・不正disconnect・想定外auth・時間切れ・中止・送受信上限・接続拒否を含みます。raw peerの不正disconnectは1バイトの不完全なpayloadで、正式なdisconnectと想定外メッセージは既存runtime試験も再実行します。

OpenSSHの独立した対向では、古典/hybrid offer収集2枠と、実製品CLIのML-KEM成功(日英)・pin不一致・署名改変・古典のみの対向という5枠を再利用します。暗号化後のメッセージもserver側の復号後の受信記録で確認します。クライアントの`user_authentication_attempted: false`だけで認証禁止を判定しません。

## 隔離と保存

工具の準備は対象実行と分け、固定イメージとlocked Cargo cacheを使います。対象実行はnetwork none、非root、読取専用root/source、6GiBメモリ・追加swapなし・2CPU・128PID・4GiB tmpfs・1MiB共有メモリ・1200秒です。外部接続を遮断したnamespace内のloopbackだけを使用し、終了後にcontainerを削除します。実秘密や外部の診断対象は使いません。TLS fixtureの鍵はメモリ内、OpenSSHの一時鍵はtmpfs内で生成し、保存証拠には含めません。

6成果物は`target/quality-task-049/`の`protocol-assurance.json`、`tls-matrix.json`、`tls-failures.json`、`ssh-matrix.json`、`ssh-no-auth.json`、`ssh-failures.json`です。入力の条件、実report、peerの受信種別、終了結果、工具とソースhashを結び付けます。集約器自体にも、offerを交渉済みに見せる・auth/service/channelを混ぜる・後始末や通信上限を壊す・TLSのgroup/検証状態を変える負例を用意します。

## RFC9846との関係

[RFC9846 §1.2](https://www.rfc-editor.org/info/rfc9846/#section-1.2)の差分を[対応表](TASK-049-TLS-PROFILE.md)で、固定providerとこの診断の動作へ対応付けます。全条項への適合認定にはしません。再開・application通信・client認証・長時間接続を行わないprobeの範囲と、TLS libraryが担う処理を区別します。

## 最終判定

元の6条件と指定6成果物、二段階レビューを最終ソースに照合し、049をdoneとしました。製品の全環境・全入力の安全性や、RFC全面適合の認定ではありません。

TLSは21記録です。うち18はlocal peerとの成功/失敗report、2は未知/欠落groupのアクセサー単体対照、1はfresh接続2回の公開KeyShare hashとClientHelloの記録です。SSHはraw peer26枠とOpenSSH offers2枠、実製品CLI5枠を確認しました。TLS対向もrustls系で、独立した別TLS実装との総合相互運用試験ではありません。

| 検証 | 実測結果 |
|---|---|
| `cargo test --workspace --locked --offline` | 544成功、3ignored、終了0 |
| `cargo test --workspace --locked --offline --all-features` | 603成功、5ignored、終了0 |
| 通常/全機能のclippy `--workspace --all-targets … -- -D warnings`、build `--workspace …` | いずれも終了0 |
| `cargo fmt --all --check` | 終了0 |
| ignored OpenSSH offers試験の明示起動 | 古典/hybridの2枠成功、終了0 |
| 既存のOpenSSH実製品CLI script | 5枠成功、終了0 |
| Python品質試験 | 80成功、終了0 |
| container runnerのホスト境界試験 | 4成功、終了0 |
| macOSの対象Rust6試験、SSH runtimeのみ構成 | 成功。全機能clippyも終了0 |

ignoredのうちOpenSSHは別途実行しています。残る047workerは049の受入根拠には使っていません。省略なしのコマンドは[検証台帳](task049-20261114/VALIDATION.json)、実際の制限・source前後一致・container削除は[隔離台帳](task049-20261114/HOST-RESULTS.json)を参照してください。最終runはconfined-r2で、全11コマンドと外側runnerが終了0です。

初回confined-r1はtargetコマンドすべて成功後、集約器が不完全なSSH結果にoffer収集完了のassuranceを求めて終了1でした。接続前のnot_observed、banner後のpeer_claim_only、offer収集済みのunauthenticated_advertisementを区別する回帰試験を追加し、修正しました。その後同じ隔離条件で全体をconfined-r2として再実行し、成功を確認しています。初回の失敗・生データと修正前後の記録は保存archiveへ残しました。

[実装レビューと試験レビュー](task049-20261114/REVIEWS.md)はいずれも受け入れられ、対応必須の指摘はありません。実装担当はR1とR2の集約修正・RFC/profile文書を確認し、試験担当はR3の14ファイルと最終runのsource/binary/input/output hash・assertion・peer記録・全コマンドを照合しました。レビュー後は完了記録と保存証拠だけを更新し、製品・試験・集約コードは変えていません。

保存する成果物は以下のとおりです。

- [観測状態と判断](task049-20261114/protocol-assurance.json)
- [TLSのgroup/profile](task049-20261114/tls-matrix.json)と[失敗](task049-20261114/tls-failures.json)
- [SSHのoffer/KEX](task049-20261114/ssh-matrix.json)、[認証禁止](task049-20261114/ssh-no-auth.json)、[失敗と上限](task049-20261114/ssh-failures.json)
- [最終ソース](task049-20261114/FINAL-SOURCE.json)と[保存索引](task049-20261114/EVIDENCE-INDEX.json)。生ログ・原本・公式RFC参照・工具準備・差分ソースは`VERIFICATION.tar.gz`。

実行確認はLinux arm64の隔離測定とmacOS arm64の対象検証です。Windows、他provider、全protocol状態、全RFC条項、一般の外部接続先の安全性には広げません。今回commit・push・PR・merge・release、GitHub Actions実行は行っていません。CI protocols停止と実験的probeの扱いは維持しています。
