# 形式・版別の受入証拠

対象は公開対応範囲の有限な代表例です。Verifiedは正負・破損例を評価した状態を表し、検出器が全件合格した意味ではありません。各IDの規則・原文・正解・測定結果はUNIT-LEDGER.jsonへ結び付きます。

| 形式・版 | 正例ID | 負例ID | 破損ID | 既存試験の再利用 / 対応外理由 |
|---|---|---|---|---|
| CBOM-1.7 |  |  |  | 対応外: 暗号資産だけのCBOMは、文書化されたソフトウェア依存入力の範囲外。安全という負例にはしない。 |
| CER | v2/x151, v2/v2-a4 | v2/x152 | v2/x130 | 固定正解と実測を照合 |
| CRT | v2/x149, v2/v2-a5 | v2/x150 | v2/x129 | 固定正解と実測を照合 |
| Cargo.lock | v2/x054 | v2/x055 | v2/x056 | 固定正解と実測を照合 |
| Cargo.toml | v2/x051 | v2/x052 | v2/x053 | 固定正解と実測を照合 |
| CycloneDX-1.4 | v2/w182 | v2/w183 | v2/w184 | 固定正解と実測を照合 |
| CycloneDX-1.5 | v2/x105 | v2/x106 | v2/x107 | 固定正解と実測を照合 |
| CycloneDX-1.6 | v2/w185 | v2/w186 | v2/w187 | 固定正解と実測を照合 |
| CycloneDX-1.7 | v2/w188 | v2/w189 | v2/w190 | 固定正解と実測を照合 |
| DER | v2/x122, v2/x124, v2/v2-a3 | v2/x123, v2/x157 | v2/x125 | 固定正解と実測を照合 |
| Gemfile | v2/x090 | v2/x091 | v2/x092 | 固定正解と実測を照合 |
| Gemfile.lock | v2/x096 | v2/x097 | v2/x098 | 固定正解と実測を照合 |
| Go | v2/x006 | v2/x007 | v2/x008 | 固定正解と実測を照合 |
| Java | v2/x001, v2/x002, v2/x003, v2/x005, v2/v2-Encrypt.java | v2/x004, v2/v2-Symmetric.java, v2/w237 | v2/x005, v2/v2-BrokenCipher.java | 固定正解と実測を照合 |
| JavaScript | v2/x021, v2/x025, v2/x028, v2/x030, v2/x031, v2/x032 ほか20（全IDはJSON） | v2/x022, v2/x023, v2/x024, v2/x026, v2/x029, v2/x033 ほか28（全IDはJSON） | v2/x041 | 固定正解と実測を照合 |
| P12 | v2/x153, v2/v2-a1, v2/public-p12-CERT_P12_CONTAINER, v2/public-p12-CERT_RSA_3072, v2/public-p12-CERT_RSA_SIGNATURE | v2/public-p12-CERT_ECDSA_SIGNATURE | v2/x127, v2/v2-a1 | 固定正解と実測を照合 |
| PEM | v2/x147, v2/v2-a6 | v2/x148 | v2/x126 | 固定正解と実測を照合 |
| PEM marker | v2/x113, v2/x116, v2/x119 | v2/x114, v2/x115, v2/x117, v2/x118, v2/x120, v2/x121 | v2/x115, v2/x118, v2/x121 | 固定正解と実測を照合 |
| PFX | v2/x155, v2/v2-a2, v2/public-pfx-CERT_P12_CONTAINER, v2/public-pfx-CERT_RSA_3072, v2/public-pfx-CERT_RSA_SIGNATURE | v2/public-pfx-CERT_ECDSA_SIGNATURE | v2/x128, v2/v2-a2 | 固定正解と実測を照合 |
| Pipfile.lock | v2/x084 | v2/x085 | v2/x086 | 固定正解と実測を照合 |
| Python | v2/x009, v2/x011, v2/v2-audit0 | v2/x010 | v2/x011 | 固定正解と実測を照合 |
| Ruby | v2/x015, v2/x016 | 既存試験のコメント・同名識別子負例 | v2/x017 | inventories_pqc_in_all_seven_languages_but_not_comments_or_similar_names |
| Rust | v2/x012, v2/x014, v2/v2-independent-import, v2/v2-audit8, v2/v2-audit9, v2/v2-audit10 | v2/x013 | v2/x014 | 固定正解と実測を照合 |
| SPDX-2.2 | v2/w191 | v2/w192 | v2/w193 | 固定正解と実測を照合 |
| SPDX-2.3 | v2/x109 | v2/x110 | v2/x111 | 固定正解と実測を照合 |
| Swift (unsupported) |  |  |  | 対応外: Swiftは対応grammarの対象外。適格分母から除外し、安全という負例にはしない。 |
| TLS/SSH text | v2/x042, v2/x046, v2/x048 | v2/x043, v2/x044, v2/x047, v2/x049, v2/w236 | v2/x045, v2/x050 | 固定正解と実測を照合 |
| TSX | v2/w194 | v2/w195 | v2/w196 | 固定正解と実測を照合 |
| TypeScript | v2/x018 | v2/x019 | v2/x020 | 固定正解と実測を照合 |
| apache directive fragment | v2/w200 | v2/w201 | v2/w202 | 固定正解と実測を照合 |
| build.gradle | v2/x072 | v2/x073 | v2/x074 | 固定正解と実測を照合 |
| build.gradle.kts | v2/x075 | v2/x076 | v2/x077 | 固定正解と実測を照合 |
| envoy directive fragment | v2/w203 | v2/w204 | v2/w205 | 固定正解と実測を照合 |
| fixture.gemspec | v2/x102 | v2/x103 | v2/x104 | 固定正解と実測を照合 |
| gems.locked | v2/x099 | v2/x100 | v2/x101 | 固定正解と実測を照合 |
| gems.rb | v2/x093 | v2/x094 | v2/x095 | 固定正解と実測を照合 |
| go.mod | v2/x066 | v2/x067 | v2/x068 | 固定正解と実測を照合 |
| gradle.lockfile | v2/x078 | v2/x079 | v2/x080 | 固定正解と実測を照合 |
| haproxy directive fragment | v2/w209 | v2/w210 | v2/w211 | 固定正解と実測を照合 |
| istio directive fragment | v2/w206 | v2/w207 | v2/w208 | 固定正解と実測を照合 |
| kubernetes directive fragment | v2/w215 | v2/w216 | v2/w217 | 固定正解と実測を照合 |
| nginx directive fragment | v2/w197 | v2/w198 | v2/w199 | 固定正解と実測を照合 |
| package-lock.json | lock-controls/lock-0 | v2/x061 | v2/x062 | 固定正解と実測を照合 |
| package.json | 既存試験の@noble/post-quantum正例 | v2/x058 | v2/x059 | inventories_dependencies_without_claiming_pqc_is_enabled |
| pnpm-lock.yaml | lock-controls/lock-1 | v2/x064 | v2/x065 | 固定正解と実測を照合 |
| poetry.lock | v2/x087 | v2/x088 | v2/x089 | 固定正解と実測を照合 |
| pom.xml | v2/x069 | v2/x070 | v2/x071 | 固定正解と実測を照合 |
| requirements.txt | v2/x081 | v2/x082 | v2/x083 | 固定正解と実測を照合 |
| traefik directive fragment | v2/w212 | v2/w213 | v2/w214 | 固定正解と実測を照合 |

暗号化された旧P12/PFXの6属性は、二者の適用範囲レビューに基づき上表の正負から除外しました。JSONの元positive/negativeとsupportedScopeを別に保持しています。原真値と測定は上書きしていません。
TLS/SSH textはJava property断片とSSH設定断片を含みます。7サーバー設定もdirective断片で、構成全体の有効性や実交渉は検証しません。PEM/CRT/CERの負例は同じRSA証明書にECDSA署名がないという独立属性です。PKCS#12の破損filename signalと証明書属性のunsupportedを分け、暗号化された旧容器と新しい公開証明書のみの平文容器を両方保持します。
依存の版は原文の宣言または入力ロックの記録です。インストール結果や機能有効化を証明しません。jose3例の能力はambiguousのまま保持し、認識済みライブラリの正例を別に補いました。
Ruby負例の旧u022はliteral定義と一致しないため、新しい負例の根拠に流用しません。上記の既存default規則試験のcomment/identifier負例を使います。
