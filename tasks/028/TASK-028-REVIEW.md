# Task028 独立コード・119分類レビュー

実施日: 2026-11-10。対象ブランチ codex/scanner-quality-improvements、比較元 b21330034d33ef30ec002c8e0d8222effd4b9d01。Reviewerとして読み取り専用レビューを実施し、この記録以外の製品・タスク状態・コミットは変更していません。

## 判定

ローダー実装および修正後の119分類companionを承認します。初回のP2指摘3件は以下の再レビューで解消を確認しました。README/reference/contractは親担当で作業中のため最終版の承認はこの記録の対象外です。全workspace検証・shadow/corpus結果は別Evaluatorの担当です。

## 初回指摘（修正済み）

1. **[P2] 汎用RSA APIをTLS用途に限定しない。** `API_CIPHER_RSA` は一般のJava `Cipher.getInstance("RSA")` なのに purpose が `tls_authentication_or_key_establishment_reference`、参考資料がRFC9325、限界説明がnegotiated useです。`GO_TLS_RSA_IMPORT`も `crypto/rsa` インポートだけでTLS用途を判定できません。一般的な暗号操作/インポートの分類とし、TLS固有の根拠を外してください。JavaのCipherは汎用暗号化/復号APIです。[Oracle Java25 Cipher](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/javax/crypto/Cipher.html)。

2. **[P2] Ruby JWT汎用分岐はアルゴリズム不明として記録する。** `TS_RUBY_RSA_USAGE` の `JWT.encode|decode` 分岐はRSAの証拠ではありません。`JWT.encode({}, key, 'HS256')`も現在のpatternに一致します。全分岐をRSA量子脆弱性と断定するcompanion分類を条件付きにし、known_gapsおよびdesired=false / expected_pattern_match=trueのcontrolを記録してください。製品pattern変更は要求しません。[ruby-jwt公式README](https://github.com/jwt/ruby-jwt)、[RFC7518](https://www.rfc-editor.org/rfc/rfc7518.html)。

3. **[P2] 設定の除外トークンを解釈しない限界を記録する。** `HTTPD_SSLPROTOCOL_LEGACY` は `SSLProtocol -all +TLSv1.2 +TLSv1.3 -TLSv1 -TLSv1.1` に、`HTTPD_SSLCIPHERSUITE_RSA`は `SSLCipherSuite HIGH:!aNULL:!MD5:!RSA:!3DES:!RC4:!DES` に一致します。該当するOpenSSL-style cipher行も同様です。現在の一般的な「effective configuration不明」だけより具体的なknown_gapsとnegative controlsを追記し、既知FPを正直に固定してください。検出意味の変更は要求しません。[Apache mod_ssl SSLProtocol](https://httpd.apache.org/docs/2.4/mod/mod_ssl.html#sslprotocol)、[SSLCipherSuite](https://httpd.apache.org/docs/2.4/mod/mod_ssl.html#sslciphersuite)。

## 確認範囲

- AGENTS.md、prompts/REVIEWER.md、docs/quality/QUALITY-GATES.md、RESTART-LOOP、指定design/plan、既存ルールとローダー、追加tests、119行すべてのpurpose/algorithm/quantum/classical/refs/limits/controlsを確認。
- container名のみと内部アルゴリズムの区別、dependency presenceのunknown、DSA/DH/ECDSAの量子脆弱性と既存Riskとの差、ES256、PQC OIDとtrust/implementationの区別、暗号suiteのRSA/weak-symmetric混在、draftとfinal名称の区別は適切に記録されています。
- 8sourceファイルhashと119全snapshotは自動testで照合。confidenceのみ既存公開f32領域で比較し、他フィールドは完全一致します。
- source-f64有限値/範囲検証をf32狭小化前に実施。nonblank IDのみを判定しvalid IDは加工していません。WalkDirの全errorを伝播し途中packを返さない実装です。
- private iterator seamのみ追加しpublic Rule/RuleSet signatureおよびserialized fieldは維持。runtime dependency追加なし。default rule YAML変更なし。
- scopeは既存detectorごとの意味を実detector testsで固定。未知scope拒否や新stderr warningはありません。

## 独立実行した検証

- `cargo test -p pqc_scan_rules --test validation --test quality_metadata`: exit0、7test成功。
- `cargo test -p pqc_scan_rules --lib`: exit0、5test成功（late traversal error含む）。
- `cargo test -p pqc_scan_detectors --test rule_scope_compatibility`: exit0、4test成功。
- `git diff --check`: exit0。
- 上記3件の具体的誤一致はPython pattern再現で確認。compiled Rust pattern controlsへの正式追加と再実行はExecutor担当。

精度/再現率/性能の数値認定、外部CI、ネットワーク上の実環境調査、push/PR/mergeは実施していません。

## 修正後の再レビュー

2026-11-10、以下のcompanion変更を独立に再確認しました。

- API_CIPHER_RSAのpurposeは一般暗号化、refs/known_gapsもTLS限定を除去。GO_TLS_RSA_IMPORTとAPI_GO_CRYPTO_RSAは一般importと条件付きRSA使用として整理。
- TS_RUBY_RSA_USAGEはRSA分岐とalgorithm不明の汎用JWT分岐を区別し、HS256の既知FP controlを追加。
- HTTPD_SSLPROTOCOL_LEGACYの無効化指定、HTTPD/NGINX/K8S cipher-listの!除外指定をknown_gapsとdesired-negative/actual-positive controlsに記録。ECDHE-ECDSA例を量子耐性とは扱っていない点も確認。
- TLS_CERT_PIN_RSAもpin名のhintとして明示し、証明書identity/trustと区別。
- 119行・244compiled-target controls・8件の既知FPを確認。`cargo test -p pqc_scan_rules --test quality_metadata`を再実行しexit0。
- `git diff --exit-code b21330034d33ef30ec002c8e0d8222effd4b9d01 -- rules/default`はexit0で全built-in YAMLの不変を再確認。

この承認はソース実装と分類レビューの範囲です。全workspace、CLI/output shadow、frozen corpus、最終docsの検証を代替しません。既存検出の8件のFPは正直に記録した既知制約であり、修正したという意味ではありません。
