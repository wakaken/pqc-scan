use std::path::Path;

use anyhow::Result;
use pqc_scan_core::{Finding, RecommendedAction, ScanResult};

type RuleCopy = (&'static str, &'static str);

fn rule_text(id: &str) -> Option<RuleCopy> {
    Some(match id {
        "JWT_RS256" | "JWT_RS384" | "JWT_RS512" | "JWT_PS256" | "JWT_PS384" | "JWT_PS512" | "JWT_SIGNING_METHOD_RS" | "TS_JWT_RSA_ALGS" | "API_JOSE_RSA_ALG" => ("RSA系のJWT・JOSE署名方式の参照が見つかりました。", "署名を発行する側と検証する側のライブラリ、鍵形式、対応方式を確認し、PQC署名への移行手順を決めてください。"),
        "JWT_ES256" | "TS_JWT_ES_ALG" => ("ECDSA系のJWT署名方式の参照が見つかりました。", "ECDSAからPQC署名への移行候補を調べ、検証側との互換性を確認してください。"),
        "JWT_LIB_JSONWEBTOKEN" => ("jsonwebtokenへの依存が見つかりました。", "利用している版と署名方式を確認し、PQC署名への対応予定を調べてください。"),
        "JWT_LIB_JOSE" => ("JOSEライブラリへの依存が見つかりました。", "利用しているJOSE実装で、PQC署名の方式名と鍵形式に対応しているか確認してください。"),
        "JWT_KID_HEADER" => ("JWTの鍵識別子kidの参照が見つかりました。", "鍵の更新手順にPQC用の鍵を含める場合、検証側が新旧の鍵を識別できるか確認してください。"),
        "JWK_RSA_KEY" => ("RSAのJWK鍵を示す記述が見つかりました。", "PQC用の鍵形式と検証側の対応を確認し、鍵の更新計画を立ててください。"),
        "OIDC_ID_TOKEN_SIGALG" => ("OIDCのIDトークン署名方式の設定が見つかりました。", "認証基盤とクライアントが対応する署名方式を調べ、PQCへの移行時に認証を継続できるか確認してください。"),
        "TS_JAVA_RSA_API" | "API_RSA_KEYPAIR_GENERATOR" | "JAVA_KEYFACTORY_RSA" => ("JavaのRSA鍵生成・鍵処理APIの参照が見つかりました。", "鍵の用途を確認し、ML-KEMやML-DSAなど、目的に合ったPQC対応APIへの移行を検討してください。"),
        "API_SIGNATURE_RSA" => ("JavaのRSA署名APIの参照が見つかりました。", "ML-DSAなどのPQC署名を候補に、鍵形式と検証側の対応を確認してください。"),
        "API_CIPHER_RSA" => ("JavaのRSA暗号化APIの参照が見つかりました。", "鍵の共有にはKEM、データの暗号化には認証付き暗号を使う構成を検討し、互換性を確認してください。"),
        "TS_GO_RSA_USAGE" | "API_GO_CRYPTO_RSA" | "GO_TLS_RSA_IMPORT" => ("GoのRSA関連API・パッケージの参照が見つかりました。", "署名と鍵共有のどちらに使っているか確認し、用途に応じてPQC対応方式への移行を検討してください。"),
        "TS_NODE_RSA_USAGE" | "API_NODE_CRYPTO_SIGN_RSA" => ("Node.jsのRSA署名・鍵処理APIの参照が見つかりました。", "実行環境の版と検証側の対応を確認し、PQC署名への移行計画を立ててください。"),
        "TS_NODE_JOSE_USAGE" => ("Node.jsのJOSE関連処理が見つかりました。", "利用している署名方式とJOSE実装の対応範囲を確認してください。"),
        "TS_PY_RSA_USAGE" | "API_PY_RSA_MODULE" => ("PythonのRSA関連APIの参照が見つかりました。", "暗号ライブラリの版と実装を確認し、署名・鍵共有それぞれのPQC移行候補を調べてください。"),
        "TS_RUST_RSA_USAGE" => ("RustのRSA関連APIの参照が見つかりました。", "署名と鍵共有の処理を整理し、PQC対応実装を切り替えられる構成を検討してください。"),
        "TS_RUBY_RSA_USAGE" => ("RubyのRSA関連APIの参照が見つかりました。", "利用中のOpenSSLと署名方式を確認し、PQC移行時の鍵更新・検証手順を決めてください。"),
        "API_OPENSSL_RSA_NEW" | "OPENSSL_RSA_API" => ("OpenSSLの低水準RSA APIの参照が見つかりました。", "暗号方式の変更に備え、EVPなどの高水準APIを使う構成を検討してください。"),
        "API_EVP_PKEY_RSA" => ("OpenSSLのEVP_PKEYをRSA用に使う記述が見つかりました。", "鍵の種類を切り替えられる構成か確認し、PQC対応への移行に備えてください。"),
        "API_BC_RSA_ENGINE" => ("BouncyCastleのRSA処理が見つかりました。", "利用中の版とプロバイダーを確認し、PQC対応APIへの移行を検討してください。"),
        "TLS_RSA_KEY_EXCHANGE" | "TLS_RSA_CIPHER_SUITE" | "TS_TLS_RSA_CIPHER_LITERAL" => ("RSAに依存するTLS暗号スイートの記述が見つかりました。", "接続相手との互換性を確認し、X25519MLKEM768などのハイブリッド鍵共有を検討してください。旧Kyber方式とは通信形式が異なります。"),
        "TLS_ECDHE_RSA" => ("ECDHE-RSAを使うTLS暗号スイートの記述が見つかりました。", "鍵共有とRSA署名を分けて確認し、ハイブリッド鍵共有とPQC認証の移行計画を立ててください。"),
        "TLS_VERSION_1_0" => ("TLS 1.0を示す記述が見つかりました。", "TLS 1.0・1.1を廃止し、TLS 1.3を優先してください。TLS 1.2は接続相手との互換性が必要な場合に限り、安全な設定で残してください。"),
        "TLS_VERSION_1_1" => ("TLS 1.1を示す記述が見つかりました。", "TLS 1.0・1.1を廃止し、TLS 1.3を優先してください。TLS 1.2は接続相手との互換性が必要な場合に限り、安全な設定で残してください。"),
        "TS_TLS_LEGACY_VERSION_LITERAL" | "NODE_TLS_MIN_VERSION" => ("古いTLSバージョンを許可する記述が見つかりました。", "TLS 1.3を優先し、接続相手がすべて対応していれば1.3に限定してください。TLS 1.2を残す場合は互換性の理由と安全な設定を確認してください。"),
        "TLS_CERT_PIN_RSA" => ("RSA証明書の固定設定を示す記述が見つかりました。", "証明書を更新するときに通信が止まらないよう、固定設定とPQC移行をあわせて計画してください。"),
        "TS_RSA_KEY_SIZE_LITERAL" => ("RSAの鍵長に関する数値が見つかりました。", "この数値が実際のRSA鍵生成に使われているか、周辺のコードを確認してください。"),
        "TS_CERT_SIGALG_LITERAL" => ("証明書の署名方式に関する記述が見つかりました。", "実際の証明書と発行手順を確認し、PQC署名への移行候補を調べてください。"),
        "SSH_RSA_ALGO" | "TS_SSH_RSA_STRING" => ("SSHのRSA関連設定・方式名が見つかりました。", "ホスト認証のPQC対応とハイブリッド鍵共有を分けて計画してください。ssh-ed25519も古典暗号の署名方式です。"),
        "SSH_RSA_SHA2_SIGALG" => ("SSHのRSA-SHA2署名方式が見つかりました。", "検証側の対応を確認し、PQC署名への移行計画を立ててください。"),
        "SSH_DSS_SIGALG" => ("廃止されたSSHのssh-dss方式が見つかりました。", "ssh-dssを無効にし、接続先と合意できる方式へ更新してください。"),
        "SSH_DH_GROUP1" => ("SSHの弱いgroup1鍵共有方式が見つかりました。", "group14以降などの方式へ更新し、ハイブリッド鍵共有への対応も確認してください。"),
        "SSH_DH_GROUP14_SHA1" => ("SHA-1に依存するSSH鍵共有方式が見つかりました。", "SHA-1に依存しない方式への更新を計画してください。"),
        "SSH_CONFIG_HOSTKEY_RSA" => ("SSHのRSAホスト鍵を指定する設定が見つかりました。", "ホスト鍵の更新計画にPQC対応と接続クライアントの互換性確認を含めてください。"),
        "SSH_AUTHORIZED_KEYS_RSA" => ("authorized_keysのRSA公開鍵が見つかりました。", "利用者と接続元を確認し、RSA鍵の段階的な更新を計画してください。"),
        "PRIVATE_KEY_RSA_HEADER" => ("RSA秘密鍵のヘッダーが見つかりました。", "鍵が漏えいした可能性を調査し、対象の鍵を失効・再発行してください。鍵本体はレポートに出力していません。"),
        "PRIVATE_KEY_PKCS8_HEADER" => ("PKCS#8秘密鍵のヘッダーが見つかりました。", "秘密鍵の保管場所と漏えい範囲を確認し、必要な鍵・認証情報を更新してください。"),
        "PRIVATE_KEY_OPENSSH_HEADER" => ("OpenSSH秘密鍵のヘッダーが見つかりました。", "秘密鍵の露出として速やかに調査し、鍵の失効・更新と保管方法の見直しを行ってください。"),
        "CERT_RSA_SIGNATURE" | "CERT_OID_RSA_PSS" => ("証明書のRSA系署名方式が見つかりました。", "認証局・検証側・鍵形式の対応を確認し、標準化されたPQC署名への移行候補を評価してください。"),
        "CERT_SHA1_SIGNATURE" => ("証明書のSHA-1署名が見つかりました。", "SHA-256以降を使う署名方式での証明書更新を検討してください。"),
        "CERT_RSA_1024" => ("証明書の公開鍵に1024ビットRSAが使われています。", "利用中か確認し、弱い鍵の更新と用途に応じたPQC移行を計画してください。認証局と検証側の対応も確認してください。"),
        "CERT_RSA_2048" => ("証明書の公開鍵に2048ビットRSAが使われています。", "認証局と検証側が対応するPQC署名への移行を計画してください。従来型の鍵を大きくするだけでは量子脆弱性は解消しません。"),
        "CERT_RSA_3072" => ("証明書の公開鍵に3072ビットRSAが使われています。", "認証局と検証側の対応を確認し、標準化されたPQC署名の互換性を検証してください。"),
        "CERT_ECDSA_SIGNATURE" => ("証明書のECDSA署名が見つかりました。", "ML-DSAやSLH-DSAなどの代替候補と、認証局・検証側の対応を調べてください。"),
        "CERT_DSA_SIGNATURE" => ("証明書の署名方式にDSAが使われています。", "DSA署名の置き換えを計画し、過去の署名を検証する互換性も別に確認してください。"),
        "CERT_MD5_SIGNATURE" => ("証明書のMD5署名が見つかりました。", "対象の証明書を速やかに再発行してください。"),
        "CERT_P12_CONTAINER" => ("PKCS#12・PFX形式のファイルが見つかりました。", "ファイル内部の証明書と鍵の暗号方式を確認してください。ファイルの存在だけでは方式を判断できません。"),
        "CERT_PEM_CONTAINER" => ("証明書を格納する形式のファイルが見つかりました。", "内容を確認し、署名方式と公開鍵方式を調べてください。ファイル名だけの検出も含まれます。"),
        "CERT_EXPIRY_2026_OR_BEFORE" => ("解析した証明書の有効期限が2026年以前です（従来の固定基準）。", "現在時刻での期限切れ所見を確認し、固定基準の所見は互換性維持のための情報として扱ってください。"),
        "CERT_EXPIRED" => ("判定時刻で証明書の有効期限が切れています。", "利用中か確認し、必要なら認証局と検証側の互換性を確認して更新してください。"),
        "CERT_NOT_YET_VALID" => ("判定時刻では、証明書の有効期間が始まっていません。", "利用開始の予定とシステム時刻を確認し、必要に応じて現在有効な証明書を使ってください。"),
        "PQC_JAVA_API" => ("JavaでPQCの暗号方式を指定するAPI呼出しが見つかりました。", "JDK 24以降のML-KEM・ML-DSA対応を確認してください。プロバイダーや通信規約の対応は別に確認が必要です。"),
        "PQC_GO_API" => ("Go標準のPQCパッケージの参照が見つかりました。", "crypto/mlkemはGo 1.24以降、crypto/mldsaはGo 1.27以降が対象です。参照だけでなく、実際の利用箇所も確認してください。"),
        "PQC_NODE_API" => ("Node.js形式のPQC鍵生成の呼出しが見つかりました。", "ML-DSAは24.6以降、ML-KEMは24.7以降、SLH-DSAは24.8以降の対応を確認してください。検出だけでは実行成功を判断できません。"),
        "PQC_WEBCRYPTO_API" => ("WebCrypto形式のPQC鍵生成の呼出しが見つかりました。", "Node.jsでは24.7以降のML-KEM・ML-DSA対応を確認してください。ブラウザの対応は別に調べてください。"),
        "PQC_PYTHON_API" => ("PythonのPQC鍵生成APIの参照が見つかりました。", "cryptography 47で追加されたAPIと、48で追加されたOpenSSL 3.5以降への対応を確認してください。実際の暗号処理を担うライブラリにも依存します。"),
        "PQC_RUST_API" => ("RustのPQCライブラリの識別子が見つかりました。", "クレートの版、features、暗号プロバイダーを確認してください。PQCライブラリの参照だけではTLSでの利用を判断できません。"),
        "PQC_RUBY_API" => ("RubyでPQCの暗号方式名が見つかりました。", "Ruby本体の版だけでなく、openssl gemとリンク先ライブラリ・プロバイダーが対象方式に対応しているか確認してください。"),
        "PQC_GO_TLS_DISABLED" => ("Goのハイブリッド鍵共有を無効にする設定が見つかりました。", "tlsmlkem=0を設定した理由と実際の環境変数を確認してください。Go 1.24以降の既定動作に戻す前に、接続相手との通信を試してください。"),
        "PQC_JAVA_CLASSICAL_GROUPS" => ("JavaのTLS鍵共有方式を古典暗号だけに限定する設定が見つかりました。", "jdk.tls.namedGroupsの指定と利用中のJSSEの対応方式を確認し、接続相手に合うハイブリッド方式を検討してください。"),
        "PQC_RUBY_CLASSICAL_GROUPS" => ("Ruby形式のTLS設定で、鍵共有方式を古典暗号だけに限定する記述が見つかりました。", "groups=・ecdh_curves=の指定とリンク先ライブラリを確認し、ハイブリッド方式で接続できるか試してください。"),
        "DEP_PQC_CAPABLE_LIBRARY" => ("版や設定によってPQCに対応するライブラリが見つかりました。", "解決済みの版、features、実装を確認してください。rustls 0.23.27のaws-lc-rs構成やcryptography 48以降のOpenSSL 3.5対応などが確認候補です。依存関係だけでは有効化を判断できません。"),
        "PQC_DRAFT_ALGORITHM" => ("標準化前のKyber・Dilithium系の方式名が見つかりました。", "正確な実装と通信形式を確認してください。旧方式名だけで、最終標準のML-KEM・ML-DSAに準拠しているとは判断できません。"),
        "CERT_ML_DSA_SIGNATURE" => ("証明書の署名方式にML-DSAのOIDが見つかりました。", "署名方式の宣言を読み取った結果です。署名の正しさ、信頼チェーン、検証側の対応は別に確認してください。"),
        "CERT_ML_DSA_PUBLIC_KEY" => ("証明書の公開鍵方式にML-DSAのOIDが見つかりました。", "公開鍵方式と証明書の署名方式は別の情報です。署名と信頼チェーンも確認してください。"),
        "CERT_SLH_DSA_SIGNATURE" => ("証明書の署名方式にSLH-DSAのOIDが見つかりました。", "署名方式の宣言を読み取った結果です。署名の正しさ、信頼チェーン、検証側の対応は別に確認してください。"),
        "CERT_SLH_DSA_PUBLIC_KEY" => ("証明書の公開鍵方式にSLH-DSAのOIDが見つかりました。", "公開鍵方式と証明書の署名方式は別の情報です。署名と信頼チェーンも確認してください。"),
        "K8S_INGRESS_SSL_PROTOCOLS_LEGACY" | "K8S_GATEWAYAPI_TLSV1" => ("Kubernetesの入口の設定に古いTLS方式を許可する記述が見つかりました。", "Ingress・GatewayではTLS 1.3を優先してください。TLS 1.2は互換性が必要な接続に限り、安全な設定で残してください。"),
        "K8S_INGRESS_RSA_CIPHERS" => ("Kubernetes IngressにRSA依存の暗号スイート設定が見つかりました。", "コントローラーと接続元の対応を確認し、ハイブリッド鍵共有への移行を検討してください。"),
        "NGINX_SSL_PROTOCOLS_LEGACY" => ("nginxに古いTLS方式を許可する設定が見つかりました。", "ssl_protocolsではTLSv1.3を優先し、接続元がすべて対応していればTLSv1.3だけを許可してください。TLSv1.2を残す場合は互換性の理由を確認してください。"),
        "NGINX_PROXY_SSL_PROTOCOLS_LEGACY" => ("nginxの接続先向けTLS設定に古い方式が見つかりました。", "proxy_ssl_protocolsではTLS 1.3を優先してください。TLS 1.2は接続先との互換性が必要な場合に限り、安全な設定で残してください。"),
        "NGINX_SSL_CIPHERS_RSA" => ("nginxにRSA依存・弱い暗号スイートの設定が見つかりました。", "暗号スイートを見直し、ハイブリッド鍵共有へ移行できる構成を検討してください。"),
        "NGINX_SSL_PREFER_SERVER_CIPHERS_OFF" => ("nginxでサーバー側の暗号方式の優先設定が無効になっています。", "接続元への影響を確認し、サーバー側の優先設定を有効にしてください。"),
        "HTTPD_SSLPROTOCOL_LEGACY" => ("Apache httpdに古いTLS方式を許可する設定が見つかりました。", "SSLProtocolではTLS 1.3を優先してください。TLS 1.2を残す場合は、接続元との互換性の理由と安全な設定を確認してください。"),
        "HTTPD_SSLCIPHERSUITE_RSA" => ("Apache httpdにRSA依存・弱い暗号スイートの設定が見つかりました。", "SSLCipherSuiteを確認し、RSA鍵交換への依存を減らす計画を立ててください。"),
        "HTTPD_SSLHONORCIPHERORDER_OFF" => ("Apacheでサーバー側の暗号方式の優先設定が無効になっています。", "接続元への影響を確認し、SSLHonorCipherOrderをOnにしてください。"),
        "ENVOY_TLS_MIN_VERSION_LEGACY" => ("EnvoyのTLS最低バージョンが低い設定になっています。", "tls_minimum_protocol_versionはTLSv1_3を優先してください。TLSv1_2は接続相手との互換性が必要な場合に限って検討してください。"),
        "ENVOY_TLS_MAX_VERSION_LEGACY" => ("EnvoyのTLS最高バージョンが古い値に制限されています。", "tls_maximum_protocol_versionを確認し、TLS 1.3を優先して使えるようにしてください。TLS 1.2は互換性が必要な場合に限って検討してください。"),
        "ENVOY_CIPHER_SUITES_RSA" => ("EnvoyにRSA依存・弱い暗号スイートの設定が見つかりました。", "cipher_suitesの指定と接続先を確認し、RSA依存の方式を見直してください。"),
        "ENVOY_SIGNATURE_ALGORITHMS_RSA" => ("EnvoyにRSA署名方式の明示的な設定が見つかりました。", "署名方式の設定にPQC移行計画を反映し、証明書と接続先の対応を確認してください。"),
        "ISTIO_DESTINATIONRULE_TLSV1" | "ISTIO_GATEWAY_TLSV1" => ("IstioにTLS 1.0・1.1を許可する記述が見つかりました。", "DestinationRule・GatewayではTLS 1.3を優先してください。TLS 1.2は互換性が必要な接続に限って残し、TLS 1.0・1.1は無効にしてください。"),
        "ISTIO_DESTINATIONRULE_RSA_CIPHERS" => ("IstioにRSA依存・弱い暗号スイートの設定が見つかりました。", "暗号スイートの設定と接続先を確認し、RSAへの依存を減らす計画を立ててください。"),
        "HAPROXY_BIND_TLSV1" => ("HAProxyにTLS 1.0・1.1を有効にする記述が見つかりました。", "bindオプションからTLS 1.0・1.1を外し、TLS 1.3を優先してください。TLS 1.2を残す場合は互換性の理由を確認してください。"),
        "TRAEFIK_TLS_OPTIONS_LEGACY" => ("TraefikのTLS最低バージョンが低い設定になっています。", "TLSオプションのminVersionはTLS 1.3を優先してください。TLS 1.2は接続相手との互換性が必要な場合に限って検討してください。"),
        "DEP_LIBSSH" | "DEP_PARAMIKO" | "DEP_OPENSSL" | "DEP_BOUNCYCASTLE" | "DEP_BORINGSSL" | "DEP_PYCRYPTODOME" | "DEP_PYCA_CRYPTOGRAPHY" | "DEP_GO_JOSE" | "DEP_JAVA_BOUNCYCASTLE" | "DEP_RUST_OPENSSL" | "DEP_RUST_RSA_CRATE" | "DEP_RUBY_OPENSSL" | "DEP_RUBY_JWT" => ("暗号処理に関係するライブラリへの依存が見つかりました。", "表にあるライブラリ名と版を確認し、実際に使う暗号方式とPQC対応状況を調べてください。依存しているだけで、特定の方式を使っているとは判断できません。"),
        _ => return None,
    })
}

pub(crate) fn severity(value: &str) -> &str {
    match value {
        "critical" => "緊急",
        "high" => "高",
        "medium" => "中",
        "low" => "低",
        "info" => "参考情報",
        _ => value,
    }
}

pub(crate) fn risk(value: &str) -> &str {
    match value {
        "quantum-safe" => "PQC方式の参照あり（対応完了の保証ではありません）",
        "quantum-uncertain" => "要確認（実装や設定の確認が必要）",
        "quantum-vulnerable" => "量子計算によるリスクあり",
        "non-quantum-risk" => "量子計算とは別のセキュリティ上の注意点",
        _ => value,
    }
}

pub(crate) fn localize_finding(source: &Finding) -> Finding {
    let mut finding = source.clone();
    if let Some((description, hint)) = rule_text(&finding.rule_id) {
        finding.description = description.to_string();
        finding.migration_hint = hint.to_string();
    } else {
        finding.description = format!(
            "日本語の説明が未登録のルールです。原文：{}",
            finding.description
        );
        finding.migration_hint =
            format!("ルール作成者の確認事項（原文）：{}", finding.migration_hint);
    }
    for action in &mut finding.recommended_actions {
        localize_action(action, &finding.location.file);
    }
    finding
}

fn action_text(id: &str) -> Option<(&'static str, &'static str, &'static [&'static str])> {
    Some(match id {
        "tls.review-pqc-override" => ("PQCの上書き設定を確認する", "検出した設定はハイブリッド鍵共有を制限しています。RSAの利用や実際のTLSバージョンを確認した結果ではありません。", &["この箇所のGODEBUG、jdk.tls.namedGroups、groups=・ecdh_curves=のうち、検出対象の設定を確認してください。", "設定が必要な理由と接続相手の対応を確認してから、ハイブリッド方式や既定設定で通信を試してください。", "鍵共有、TLSバージョン、証明書の署名方式は別々に確認してください。TLS 1.3であるだけではPQC対応とは判断できません。"]),
        "incident.private-key-rotation" => ("露出した秘密鍵を失効し、更新する", "秘密鍵のヘッダーが検出されています。漏えいの範囲と利用先を速やかに確認してください。", &["対象の鍵を失効・再発行し、影響する認証情報も更新してください。", "その鍵で署名したトークンやセッションの扱いを確認してください。", "鍵を専用の保管先へ移し、リポジトリへの再混入を防いでください。"]),
        "plan.hybrid-migration" => ("既存方式とPQCを併用し、段階的に移行する", "検出した方式には、量子計算による将来のリスクがあります。接続相手との互換性を確かめながら移行を進めてください。", &["利用箇所と担当チームを洗い出してください。", "既存方式とPQCの併用を試し、一括切替の前に影響を確認してください。", "旧方式を廃止する条件と時期を決めてください。"]),
        "dependency.track-pqc-readiness" => ("依存ライブラリのPQC対応を確認する", "PQCへの移行方法は、利用しているライブラリの版や設定にも左右されます。", &["dependency-sbom.jsonで現在の版を確認し、更新候補を決めてください。", "リリース情報でPQC機能とAPIの変更を確認してください。", "本番導入の前にPQC・ハイブリッド方式の結合テストを行ってください。"]),
        "runtime.verify-pqc-capabilities" => ("実行環境と通信相手のPQC対応を確認する", "コードに方式名があるだけでは、その環境で動くか、接続相手が対応しているかは分かりません。", &["実際に配備している版、プロバイダー、機能設定を確認してください。", "本番設定を変える前に、接続相手との互換性を試してください。"]),
        "jwt.remove-rsa-algorithms" => ("JWTの署名方式と検証側の移行手順を決める", "RSA・ECDSAなどの署名には、量子計算による将来のリスクがあります。署名側だけを変えると認証に失敗するため、検証側と合わせて計画してください。", &["許可するalgを明示し、弱い方式や意図しない既定値を受け付けない設定にしてください。", "移行中はトークンの有効期間を短くすることを検討してください。", "JOSEの方式名・鍵形式とライブラリの対応を確認し、署名側より先に検証側を更新してください。"]),
        "node.webcrypto.pqc" => ("Node.jsの版とWebCryptoの対応方式を確認する", "Node.jsの版とAPIによって、利用できるPQC機能が異なります。", &["cryptoではML-DSAは24.6以降、ML-KEMは24.7以降、SLH-DSAは24.8以降です。WebCryptoのML-DSA・ML-KEMは24.7以降を確認してください。", "ブラウザの対応は別に確認してください。TypeScriptの型があっても実行環境の対応は保証されません。ML-KEMはJWTの署名の代わりにはなりません。"]),
        "go.hybrid-kem" => ("Goの鍵共有と署名を分けて移行する", "通信の鍵共有とJWTなどの署名では、必要な暗号方式が異なります。", &["Go 1.24以降のcrypto/mlkemとハイブリッドTLSの既定設定を確認してください。", "Go 1.27以降のcrypto/mldsaとML-DSA証明書・TLS署名の対応も確認してください。", "新旧方式を併用して検証し、接続相手との互換性を保ってください。"]),
        "python.signing-gateway" => ("Pythonの署名処理を切り替えられる構成にする", "署名処理をまとめると、ライブラリや方式を変更するときの影響を限定できます。", &["JWT署名の呼出しを共通の処理にまとめてください。", "cryptography 48以降とOpenSSL 3.5以降のML-KEM・ML-DSA対応を確認してください。47のAPIにはAWS-LC・BoringSSLの条件があります。非対応環境でのエラーも処理してください。"]),
        "rust.oqs-integration" => ("Rustの暗号実装を切り替えられる構成にする", "署名・鍵共有の呼出しを共通の型で扱うと、方式を段階的に変更しやすくなります。", &["RSAを直接呼ぶ箇所を署名用のトレイトにまとめてください。", "Cargoのfeaturesと結合テストを使い、PQC実装を段階的に追加してください。", "TLSはrustlsの版とプロバイダーを確認してください。0.23.22でaws-lc-rsのX25519MLKEM768に対応し、0.23.27で優先設定が既定になりました。ringや独自のグループ設定は別に確認してください。"]),
        "ruby.crypto-service-boundary" => ("Rubyの署名処理と暗号ライブラリの条件を整理する", "RubyのPQC機能は、OpenSSLのプロバイダーや外部の署名サービスにも依存します。", &["OpenSSL・JWTの呼出しを共通の処理にまとめてください。", "PQC対応の署名処理をプロバイダーや外部サービス経由で導入できるか確認してください。", "openssl gemとリンク先の版を確認してください。gem 4ではgroups=、sigalgs=、通信で選ばれた方式の確認APIが追加されています。"]),
        "java.jca-ml-dsa-ml-kem" => ("Java標準のPQC APIへの移行を検討する", "対応するJDKでは、JCAの標準方式名でML-DSA・ML-KEMを指定できます。", &["署名用途ではML-DSAへの切替を検討し、検証側の対応を確認してください。", "ML-KEMは鍵共有に使う方式です。TLSなどの対応は別に確認してください。", "移行中は旧方式の署名も検証できる期間を設けてください。"]),
        "java.runtime-upgrade-for-pqc" => ("Javaの実行環境をPQC APIに対応する版へ更新する", "現在のJavaの版では標準PQC APIを使えない可能性があります。版が不明な場合は実環境を確認してください。", &["ML-KEM・ML-DSAはJDK 24以降を更新候補にしてください。", "本番への適用前にCIで互換性を試してください。"]),
        "java.bouncycastle-pqc" => ("BouncyCastleによるPQC対応を検討する", "標準APIを使えない環境では、ライブラリによる対応が移行候補になります。", &["BouncyCastleの版とプロバイダーの登録方法を確認してください。", "ML-DSA・ML-KEMのパラメーターを選び、検証環境で試してください。", "プロバイダーの優先順位とFIPSなどの運用要件への影響を記録してください。"]),
        "tls.remove-rsa-kex" => ("TLS 1.3を優先し、鍵共有方式を確認する", "RSA鍵交換や古いTLS設定は移行時の確認対象です。この一般的な確認項目だけで、RSAの利用が確定するわけではありません。", &["TLS 1.3を優先し、全接続先が対応していれば1.3だけを許可してください。TLS 1.0・1.1は無効にしてください。", "TLS 1.2は互換性が必要な場合に限り、安全な設定で残してください。理由を記録し、鍵共有方式のPQC対応も別に確認してください。", "クライアントと負荷分散装置を含め、互換性を継続して確認してください。"]),
        "tls.nginx-hardening" => ("nginxのTLS設定を見直す", "プロトコルと暗号スイートを明示し、移行前後の動作を確認してください。", &["接続元がすべて対応していればssl_protocols TLSv1.3; を設定してください。TLSv1.2を残す場合は互換性の理由を確認してください。", "RSA鍵交換を使う暗号スイートを確認し、前方秘匿性を持つ方式へ見直してください。"]),
        "tls.httpd-hardening" => ("Apache httpdのTLS設定を見直す", "プロトコルと暗号スイートの変更は、接続元との互換性を確認して進めてください。", &["接続元がすべて対応していればSSLProtocol -all +TLSv1.3を設定してください。TLSv1.2を残す場合は互換性の理由を確認してください。", "SSLCipherSuiteを見直し、RSA鍵交換を使う方式を確認してください。"]),
        "tls.envoy-hardening" => ("EnvoyのTLS設定を見直す", "最低・最高バージョンと暗号スイートを設定で管理できます。", &["tls_minimum_protocol_versionはTLSv1_3を優先し、TLSv1_2は互換性が必要な場合に限って検討してください。", "cipher_suitesのRSA依存の方式を確認して見直してください。"]),
        "tls.istio-hardening" => ("IstioのTLS設定を移行計画に合わせる", "DestinationRuleとGatewayの設定を揃えて、接続への影響を確認してください。", &["サービス間通信とゲートウェイではTLS 1.3を優先し、TLS 1.2は互換性が必要な場合に限って残してください。", "設定されているRSA依存の暗号スイートを確認してください。"]),
        "tls.k8s-ingress-hardening" => ("Kubernetesの入口のTLS設定を更新する", "Ingressなどの入口では、複数サービスに共通のTLS設定が使われます。", &["IngressコントローラーではTLS 1.3を優先し、TLS 1.2は互換性が必要な接続に限って残してください。", "古い方式の再混入を設定検査で防いでください。"]),
        "pki.hybrid-certificate-roadmap" => ("証明書の発行と信頼設定の更新を計画する", "証明書の方式変更には、認証局、証明書チェーン、検証側の対応が必要です。", &["CBOMで証明書の署名方式と鍵情報を確認してください。", "本番以外の信頼設定でハイブリッド証明書チェーンを試してください。", "ルート・中間・サーバー証明書の再発行手順を決めてください。"]),
        "cryptoapi.abstraction-layer" => ("暗号方式を切り替えられる共通処理を設ける", "方式に依存する呼出しを分離すると、古典暗号からPQCへ段階的に移行しやすくなります。", &["署名・検証・鍵共有を共通の処理にまとめてください。", "設定で古典・ハイブリッド・PQCの各実装を選べる構成を検討してください。", "実装の変更を結合テストで確認してください。"]),
        "k8s.pqc-signature-readiness-note" | "nginx.pqc-signature-readiness-note" | "envoy.pqc-signature-readiness-note" | "istio.pqc-signature-readiness-note" | "haproxy.pqc-signature-readiness-note" | "traefik.pqc-signature-readiness-note" => ("接続経路全体でPQC証明書の署名を検証できるか確認する", "ハイブリッド鍵共有への対応と、PQC署名の証明書への対応は別です。TLSライブラリ、プロバイダー、各接続先の条件を確認してください。", &["元の参考資料と対象の製品・版を確認してください。", "入口、サービス間通信、認証局、クライアントを含めてML-DSA・SLH-DSA証明書の互換性を試してください。", "本番で必須にする前に、段階的な試験と切戻し手順を用意してください。"]),
        _ => return None,
    })
}

pub(crate) fn localize_action(action: &mut RecommendedAction, file: &str) {
    if let Some((title, rationale, steps)) = action_text(&action.action_id) {
        action.title = title.to_string();
        action.rationale = rationale.to_string();
        action.steps = steps.iter().map(|s| s.to_string()).collect();
        if action.action_id == "runtime.verify-pqc-capabilities" {
            let hint_id = match Path::new(file).extension().and_then(|s| s.to_str()) {
                Some("java") => "PQC_JAVA_API",
                Some("go") => "PQC_GO_API",
                Some("js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs") => "PQC_NODE_API",
                Some("py") => "PQC_PYTHON_API",
                Some("rs") => "PQC_RUST_API",
                Some("rb") => "PQC_RUBY_API",
                _ => "",
            };
            if let Some((_, hint)) = rule_text(hint_id) {
                action.steps.insert(0, hint.to_string());
            }
        }
    } else {
        action.title = format!("日本語未登録の対応案（原文）：{}", action.title);
    }
    action.priority = match action.priority.as_str() {
        "immediate" => "至急",
        "near-term" => "早めに対応",
        "planned" => "計画して対応",
        value => value,
    }
    .to_string();
}

pub(crate) fn html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub(crate) fn md(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('|', "\\|")
        .replace('`', "\\`")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\n', " ")
}

pub(crate) fn write_html(result: &ScanResult, path: &Path) -> Result<()> {
    crate::compact::html_report(result, path, crate::ReportLanguage::Ja)
}
pub(crate) fn write_markdown(result: &ScanResult, path: &Path) -> Result<()> {
    crate::compact::markdown_report(result, path, crate::ReportLanguage::Ja)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_rule_and_action_has_japanese_copy() {
        let rules = pqc_scan_rules::RuleSet::load_from_dir(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules/default"),
        )
        .unwrap();
        for rule in rules.all() {
            assert!(rule_text(&rule.id).is_some(), "missing rule {}", rule.id);
        }
        for line in include_str!("../../pqc_scan_core/src/recommendation.rs")
            .lines()
            .filter(|line| line.contains("action_id: \""))
        {
            let id = line.split('"').nth(1).unwrap();
            assert!(action_text(id).is_some(), "missing action {id}");
        }
    }
}
