# TASK-049 固定TLS profileとRFC9846差分

元要求Q4-08の実group確認とprofile/providerの対応を説明します。正式サポートの拡張、RFC全体への適合認定は行いません。

## 固定profile

製品は`default_secure_fresh`、aws_lc_rs 1.18.0、rustls 0.23.46です。TLS1.3/1.2、X25519/P256/P384/X25519MLKEM768を提示します。今回のlocal peerでそれぞれのgroupを交渉し、提示profileと実交渉値を別に保存します。SECP256R1MLKEM768(4587)はこのprofileに含まれず、その方式しかないpeerの失敗を「リモートにPQCなし」と読み替えません。providerを入れ替えた測定はしていません。

## RFC差分の適用整理

参照は[RFC9846 §1.2](https://www.rfc-editor.org/info/rfc9846/#section-1.2)と[§1.4](https://www.rfc-editor.org/info/rfc9846/#section-1.4)です。2026-11-14に公式TXTを準備工程で取得し、保存証拠のstandards/SOURCE.jsonにURL/hashを記録します。以下は短い初回診断への影響の整理です。処理をlibraryへ委譲する項目を、独立に全組合せ試験済みとは扱いません。

| 差分の論点 | このprobeへの適用と証拠 |
|---|---|
| 接続間のKeyShare再利用禁止 | fresh接続2回のpublic KeyShare hashが異なる。秘密値やshare本体は保存しない |
| TLS1.0/1.1廃止 | 実ClientHelloのsupported_versionsは772/771のみ |
| PSKとHRRのhash | PSKは提示せず再開無効。PSKなしのhybrid HRRは実交渉で確認 |
| 再開非対応時のticket扱い | resumption disabled。ticketから次回接続を生成しない固定設定。全ticket形式の適合試験ではない |
| KeyUpdate前の鍵使用上限 | application通信と長時間接続を行わない。送受信512KiB/64KiBで停止し、鍵使用上限の認定をしない |
| KeyUpdate回数制限 | post-handshake通信を継続しない。library側処理の全網羅は主張しない |
| close_notifyのalert level | 製品はbounded transportを閉じる。TLS alertを独自に構築せず、相互運用の全close動作は認定しない |
| user_canceledの扱い | local cancellationを型付き中止として保持。peer alertの詳細はlibraryへ委譲 |
| general_error alert | libraryのerror分類に従う。unknown errorを成功としない。個別alert全種の適合認定はしない |
| CertificateRequest extensions下限 | mTLSは非対応として停止。client証明書・認証用recordを送らないことをpeerで確認 |
| ClientHello extensions下限 | 実ClientHelloを独立に読んでversions/PSK/early-data/shareを確認。全拡張の文法適合はlibraryへ委譲 |
| NewSessionTicket extensions上限 | 再開無効で診断結果には使わない。全境界のticket parser試験ではない |
| KEMを含む鍵交換の表現 | 方式名と実groupを分離。4588をhybrid、既知の古典groupをclassical、不明をunknownとして保存 |
| 将来仕様とtranscript hash | 固定libraryと実HRRで確認する範囲。将来の拡張に適合すると推定しない |
| RSA-PSS要件の整理 | このtaskで署名の許可方式や検証方針を変更しない。通常の証明書検証をlibraryへ委譲 |
| TLS1.2に及ぶ更新 | TLS1.2の実交渉は確認するが、旧版を含む全TLS適合試験とは扱わない |

この表に未認定の適合範囲があることと、049の6つの観測・失敗・認証境界条件を満たしたかは別に判定します。TLSを全面的に再実装したり、暗号仕様を変更して不足を隠したりしません。
