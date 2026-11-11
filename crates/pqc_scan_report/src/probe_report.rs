//! Human-readable views and unchanged-language JSON for explicit endpoint probes.
use crate::{japanese, ReportLanguage};
use anyhow::{anyhow, Result};
use pqc_scan_core::probe::{ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport};
use serde::Serialize;
use serde_json::Value;
use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

/// Export only the selected runtime document and its two human views.
/// Known secret syntax is omitted from a cloned payload consistently in all views.
pub fn write_probe_reports<T: Serialize>(
    report: &ProbeReport<T>,
    out_dir: &Path,
    language: ReportLanguage,
) -> Result<Vec<PathBuf>> {
    report.validate()?;
    let mut value =
        serde_json::to_value(report).map_err(|_| anyhow!("probe report serialization failed"))?;
    let redacted = redact_values(&mut value);
    if redacted {
        value["redaction_applied"] = Value::Bool(true);
    }
    let json = serde_json::to_string_pretty(&value)
        .map_err(|_| anyhow!("probe report serialization failed"))?;
    let rows = rows(&value);
    let html = html(
        &rows,
        &value,
        report.protocol,
        language,
        value["redaction_applied"] == true,
        tls_envelope_allows_verified_summary(report),
    )?;
    let markdown = markdown(
        &rows,
        report.protocol,
        language,
        value["redaction_applied"] == true,
    )?;
    let filename = match report.protocol {
        ProbeProtocol::Tls => "probe.tls.v1.json",
        ProbeProtocol::SshOffers => "probe.ssh-offers.v1.json",
        ProbeProtocol::SshKex => "probe.ssh-kex.v1.json",
    };
    std::fs::create_dir_all(out_dir)?;
    let mut outputs = Vec::new();
    for (filename, content) in [
        (filename, json),
        ("report.html", html),
        ("report.md", markdown),
    ] {
        let path = out_dir.join(filename);
        std::fs::write(&path, content)?;
        outputs.push(path);
    }
    Ok(outputs)
}

fn redact_values(value: &mut Value) -> bool {
    match value {
        Value::String(text) if pqc_scan_core::privacy::redact_text(text) != *text => {
            *text = "[masked-sensitive-content]".into();
            true
        }
        Value::Array(values) => values
            .iter_mut()
            .fold(false, |changed, value| redact_values(value) | changed),
        Value::Object(values) => values
            .values_mut()
            .fold(false, |changed, value| redact_values(value) | changed),
        _ => false,
    }
}

struct Row {
    path: String,
    key: String,
    value: String,
}
fn rows(value: &Value) -> Vec<Row> {
    fn token(value: &str) -> String {
        value.replace('~', "~0").replace('/', "~1")
    }
    fn visit(value: &Value, path: String, key: &str, out: &mut Vec<Row>) {
        match value {
            Value::Object(values) if !values.is_empty() => {
                for (key, value) in values {
                    visit(value, format!("{path}/{}", token(key)), key, out);
                }
            }
            Value::Array(values) if !values.is_empty() => {
                for (index, value) in values.iter().enumerate() {
                    visit(value, format!("{path}/{index}"), key, out);
                }
            }
            _ => out.push(Row {
                path,
                key: key.into(),
                value: match value {
                    Value::String(s) if !s.is_empty() => s.clone(),
                    _ => value.to_string(),
                },
            }),
        }
    }
    let mut result = Vec::new();
    visit(value, String::new(), "", &mut result);
    result
}
fn text<'a>(language: ReportLanguage, en: &'a str, ja: &'a str) -> &'a str {
    if language == ReportLanguage::Ja {
        ja
    } else {
        en
    }
}
fn title(protocol: ProbeProtocol, language: ReportLanguage) -> &'static str {
    match protocol {
        ProbeProtocol::Tls => text(language, "TLS endpoint observation", "TLS接続先の観測結果"),
        ProbeProtocol::SshKex => text(
            language,
            "SSH hybrid key exchange verification",
            "SSHハイブリッド鍵交換の確認結果",
        ),
        ProbeProtocol::SshOffers => text(
            language,
            "SSH advertised algorithm observation",
            "SSH提示アルゴリズムの観測結果",
        ),
    }
}
fn scope(protocol: ProbeProtocol, language: ReportLanguage) -> &'static str {
    match protocol {
        ProbeProtocol::SshKex=>text(language,"An explicit prior public host-key pin and the server signature are required. This profile tests hybrid ML-KEM768+X25519 with classical Ed25519 host authentication, without user authentication or login. A failure does not establish global server non-support.","事前に信頼したホスト公開鍵との照合と、サーバー署名の検証を行います。ML-KEM768とX25519のハイブリッド鍵交換を調べ、ホスト認証には従来方式のEd25519を使います。ユーザー認証・ログインは行いません。失敗だけでサーバー全体の非対応を断定することはできません。"),
        ProbeProtocol::Tls=>text(language,"Verification state and any negotiated facts describe this single endpoint and client profile. Inspection limits and diagnostics qualify the result. This observation does not establish exhaustive server capabilities or completed PQC migration.","検証状態と通信で決まった事実は、この接続先とクライアント設定に対する1回の観測を示します。収集上限と診断情報を併せて確認してください。サーバーの全能力やPQC移行完了を保証する結果ではありません。"),
        ProbeProtocol::SshOffers=>text(language,"Unauthenticated advertisements describe the server's claims when lists are available. Lists retain their original order. Client lists are diagnostic proposals, not implemented cryptographic capabilities. Key exchange, host identity verification and authentication were not performed.","収集できた提示一覧は、サーバーによる認証前の申告として元の順に記録します。クライアント側の一覧は診断用の提案で、暗号方式の実装対応を示すものではありません。鍵交換・ホストの検証・認証は実施していません。"),
    }
}
fn loss(language: ReportLanguage) -> &'static str {
    text(language,"Strings resembling secrets were omitted. Original values cannot be recovered from this output. Observation completion describes collection; it does not remove this export loss.","秘密情報に見える文字列を省略しました。該当箇所の原文はこの出力から復元できません。観測完了は収集の状態を示すもので、この省略がなかったという意味ではありません。")
}
fn label(key: &str, language: ReportLanguage) -> &str {
    let labels = match key {
        "host_identity_verified" => (
            "Trusted host key and signature verified",
            "信頼済みホスト鍵と署名の検証",
        ),
        "negotiated_kex" => ("Verified selected key exchange", "検証済みの鍵交換方式"),
        "host_signature" => (
            "Verified host signature algorithm",
            "検証済みホスト署名方式",
        ),
        "host_authentication_pqc" => ("Post-quantum host authentication", "ホスト認証のPQC対応"),
        "user_authentication_attempted" => ("User authentication attempted", "ユーザー認証の実施"),
        "offered_kex" => ("Fixed offered key exchange", "提示した固定の鍵交換方式"),
        "offered_host_key" => ("Fixed offered host-key type", "提示した固定のホスト鍵方式"),
        "russh_version" => ("russh version", "russhの版"),
        "schema_version" => ("Schema version", "スキーマの版"),
        "scanner_version" => ("Scanner version", "スキャナーの版"),
        "generated_at" => ("Generated at", "作成日時"),
        "protocol" => ("Observation method", "観測方法"),
        "attempt_id" => ("Attempt within this invocation", "今回の実行内の試行番号"),
        "host" => ("Requested host", "要求したホスト"),
        "port" => ("Requested port", "要求したポート"),
        "resolved_addresses" => ("Checked DNS answers", "確認した名前解決結果"),
        "pinned_address" => ("Pinned destination", "固定した接続先"),
        "connected_peer" => ("Connected peer", "実際の接続相手"),
        "max_dns_answers" => ("Maximum DNS answers", "名前解決結果の上限"),
        "dns_timeout_ns" => ("DNS timeout (ns)", "名前解決の制限時間（ナノ秒）"),
        "tcp_timeout_ns" => ("TCP timeout (ns)", "TCP接続の制限時間（ナノ秒）"),
        "protocol_timeout_ns" => (
            "Protocol timeout (ns)",
            "プロトコル処理の制限時間（ナノ秒）",
        ),
        "overall_timeout_ns" => ("Overall timeout (ns)", "全体の制限時間（ナノ秒）"),
        "max_received_bytes" => ("Receive limit (bytes)", "受信上限（バイト）"),
        "max_sent_bytes" => ("Send limit (bytes)", "送信上限（バイト）"),
        "received_bytes" => ("Received socket bytes", "ソケットで受信したバイト数"),
        "sent_bytes" => ("Sent socket bytes", "ソケットで送信したバイト数"),
        "outcome" => ("Inspection outcome", "観測の完了状況"),
        "diagnostics" => ("Diagnostic code", "診断コード"),
        "redaction_applied" => ("Secret-like text omitted", "秘密情報に見える文字列の省略"),
        "name" => ("Name", "名前"),
        "id" => ("Identifier", "識別子"),
        "provider" => ("Provider", "プロバイダー"),
        "provider_version" => ("Provider version", "プロバイダーの版"),
        "rustls_version" => ("rustls version", "rustlsの版"),
        "protocol_versions" => ("Configured protocol version", "設定したプロトコルの版"),
        "cipher_suites" => ("Configured cipher suite", "設定した暗号スイート"),
        "key_exchange_groups" => ("Configured key exchange group", "設定した鍵共有グループ"),
        "verification_name" => ("Verification name", "検証する名前"),
        "sent_sni" => ("Sent SNI", "送信したSNI"),
        "trust_source" => ("Trust source", "信頼元"),
        "root_set_sha256" => ("Public root-set SHA-256", "公開ルート集合のSHA-256"),
        "certificate_chain_and_name" => (
            "Certificate chain and name validation",
            "証明書チェーンと名前の検証",
        ),
        "revocation" => ("Revocation checking", "失効確認"),
        "handshake_complete" => ("Handshake complete", "ハンドシェイク完了"),
        "handshake_kind" => ("Handshake kind", "ハンドシェイクの種類"),
        "group_classification" => ("Exact-group classification", "グループ識別子に基づく分類"),
        "certificate_der_sha256" => (
            "Full public certificate DER SHA-256",
            "公開証明書DER全体のSHA-256",
        ),
        "spki_sha256" => ("Public SPKI SHA-256", "公開SPKIのSHA-256"),
        "signature_oid" => ("Certificate signature OID", "証明書の署名OID"),
        "public_key_oid" => ("Public-key OID", "公開鍵OID"),
        "key_bits" => ("Key size (bits)", "鍵長（ビット）"),
        "named_curve_oid" => ("Named-curve OID", "名前付き曲線のOID"),
        "not_before_unix" => ("Validity start (Unix seconds)", "有効期間の開始（Unix秒）"),
        "not_after_unix" => ("Validity end (Unix seconds)", "有効期間の終了（Unix秒）"),
        "attribute_status" => ("Public-attribute inspection status", "公開属性の検査状態"),
        "status" => ("Status", "状態"),
        "presented_count" => ("Presented certificate count", "提示された証明書数"),
        "presented_bytes" => ("Presented certificate bytes", "提示された証明書のバイト数"),
        "collected_count" => ("Collected certificate count", "収集した証明書数"),
        "identification" => (
            "Diagnostic client identification",
            "診断クライアントの識別情報",
        ),
        "purpose" => ("Purpose", "用途"),
        "assurance" => ("Assurance", "確認根拠"),
        "server_identification" => ("Unverified server identification", "未検証のサーバー申告"),
        "first_kex_packet_follows" => (
            "Following guessed KEX packet declared",
            "後続の推測KEXパケットの申告",
        ),
        "reserved" => ("Reserved field", "予約フィールド"),
        "construction" => ("Construction classification", "構成の分類"),
        "namespace" => ("Identifier namespace", "識別子の名前空間"),
        "provenance" => ("Provenance", "情報の出所"),
        "reference" => ("Reference", "参考資料"),
        "extension_markers" => ("Extension marker", "拡張マーカー"),
        "pre_identification_lines" => ("Pre-identification lines", "識別前の行数"),
        "early_messages" => ("Early transport messages", "初期メッセージ数"),
        "kex_algorithms" => ("Key exchange candidates", "鍵交換方式の候補"),
        "server_host_key_algorithms" => ("Host-key candidates", "ホスト鍵方式の候補"),
        "encryption_algorithms_client_to_server" => (
            "Encryption candidates: client to server",
            "暗号化候補：クライアントからサーバー",
        ),
        "encryption_algorithms_server_to_client" => (
            "Encryption candidates: server to client",
            "暗号化候補：サーバーからクライアント",
        ),
        "mac_algorithms_client_to_server" => (
            "MAC candidates: client to server",
            "MAC候補：クライアントからサーバー",
        ),
        "mac_algorithms_server_to_client" => (
            "MAC candidates: server to client",
            "MAC候補：サーバーからクライアント",
        ),
        "compression_algorithms_client_to_server" => (
            "Compression candidates: client to server",
            "圧縮候補：クライアントからサーバー",
        ),
        "compression_algorithms_server_to_client" => (
            "Compression candidates: server to client",
            "圧縮候補：サーバーからクライアント",
        ),
        "languages_client_to_server" => (
            "Language candidates: client to server",
            "言語候補：クライアントからサーバー",
        ),
        "languages_server_to_client" => (
            "Language candidates: server to client",
            "言語候補：サーバーからクライアント",
        ),
        _ => return key,
    };
    text(language, labels.0, labels.1)
}
fn display_value(row: &Row, language: ReportLanguage) -> String {
    let translated = if row.path == "/outcome" {
        match row.value.as_str() {
            "completed" => Some(text(
                language,
                "Requested observation collected",
                "要求した観測を収集しました",
            )),
            "partial" => Some(text(
                language,
                "Some observation data could not be collected",
                "一部を収集できませんでした",
            )),
            "failed" => Some(text(
                language,
                "Requested observation could not complete",
                "要求した観測を完了できませんでした",
            )),
            _ => None,
        }
    } else {
        None
    };
    translated
        .map(|s| format!("{s} ({})", row.value))
        .unwrap_or_else(|| row.value.clone())
}
fn overview(rows: &[Row]) -> impl Iterator<Item = &Row> {
    rows.iter().filter(|row| {
        matches!(
            row.path.as_str(),
            "/outcome"
                | "/target/host"
                | "/target/port"
                | "/target/connected_peer"
                | "/traffic/received_bytes"
                | "/traffic/sent_bytes"
        ) || row.path.starts_with("/diagnostics/")
    })
}
// These are the only partial-result diagnostics emitted after the TLS runtime
// completes its verified handshake. Unknown or pre-handshake diagnostics must
// never promote generic serialized details into a verified connection claim.
fn tls_envelope_allows_verified_summary<T>(report: &ProbeReport<T>) -> bool {
    match report.outcome {
        ProbeOutcome::Completed => true,
        ProbeOutcome::Failed => false,
        ProbeOutcome::Partial => report.diagnostics.iter().all(|diagnostic| {
            matches!(
                diagnostic,
                ProbeDiagnostic::CertificateCollectionLimit
                    | ProbeDiagnostic::CertificateParseIncomplete
                    | ProbeDiagnostic::GroupUnavailable
            )
        }),
    }
}

// Only sanitized values reach the human summary. Unknown facts stay in the full table.
fn value_text<'a>(value: &'a Value, pointer: &str) -> Option<&'a str> {
    value.pointer(pointer).and_then(Value::as_str)
}
fn paragraph(out: &mut String, content: &str) -> Result<()> {
    writeln!(out, "<p>{}</p>", japanese::html(content))?;
    Ok(())
}
fn summary_item(out: &mut String, label: &str, content: &str) -> Result<()> {
    writeln!(
        out,
        "<dt>{}</dt><dd>{}</dd>",
        japanese::html(label),
        japanese::html(content)
    )?;
    Ok(())
}
fn protocol_summary(
    out: &mut String,
    value: &Value,
    protocol: ProbeProtocol,
    language: ReportLanguage,
    tls_envelope_allows_verified: bool,
) -> Result<()> {
    let unknown = text(language, "Not confirmed", "未確認");
    writeln!(
        out,
        "<section><h2>{}</h2>",
        text(language, "What was observed", "今回わかったこと")
    )?;
    match protocol {
        ProbeProtocol::Tls => {
            let verified = tls_envelope_allows_verified
                && value.pointer("/details/handshake_complete") == Some(&Value::Bool(true))
                && value_text(value, "/details/verification/certificate_chain_and_name")
                    == Some("verified")
                && matches!(
                    value_text(value, "/details/handshake_kind"),
                    Some("full" | "full_with_hello_retry_request")
                );
            if verified {
                paragraph(out,text(language,"A fresh TLS handshake completed with certificate-chain and name verification.","証明書チェーンと名前を検証し、新しいTLS接続を確立しました。"))?;
                writeln!(out, "<dl>")?;
                let version = match value
                    .pointer("/details/negotiated/version/id")
                    .and_then(Value::as_u64)
                {
                    Some(772) => "TLS 1.3",
                    Some(771) => "TLS 1.2",
                    _ => value_text(value, "/details/negotiated/version/name").unwrap_or(unknown),
                };
                summary_item(out, text(language, "TLS version", "TLSの版"), version)?;
                summary_item(
                    out,
                    text(language, "Cipher suite", "暗号スイート"),
                    value_text(value, "/details/negotiated/cipher_suite/name").unwrap_or(unknown),
                )?;
                summary_item(
                    out,
                    text(language, "Key exchange group", "鍵共有グループ"),
                    value_text(value, "/details/negotiated/key_exchange_group/name")
                        .unwrap_or(unknown),
                )?;
                writeln!(out, "</dl>")?;
            } else {
                paragraph(
                    out,
                    text(
                        language,
                        "No verified TLS connection result was obtained.",
                        "検証済みの通信結果は得られていません。",
                    ),
                )?;
            }
            if let Some(name) = value_text(value, "/details/verification/verification_name") {
                paragraph(
                    out,
                    &format!(
                        "{}: {name}",
                        text(
                            language,
                            "Certificate verification name",
                            "証明書の検証対象名"
                        )
                    ),
                )?;
            }
            let trust = match value_text(value, "/details/verification/trust_source") {
                Some("bundled_mozilla") => text(
                    language,
                    "Trust source: bundled Mozilla root certificates.",
                    "信頼元: 同梱のMozillaルート証明書。",
                ),
                Some("local_ca_file") => text(
                    language,
                    "Trust source: the specified local CA file.",
                    "信頼元: 指定したローカルCAファイル。",
                ),
                _ => text(language, "Trust source: not confirmed.", "信頼元: 未確認。"),
            };
            paragraph(out, trust)?;
            if value_text(value, "/details/verification/revocation") == Some("not_checked") {
                paragraph(
                    out,
                    text(
                        language,
                        "Revocation was not checked.",
                        "失効確認は実施していません。",
                    ),
                )?;
            }
            if let Some(count) = value
                .pointer("/details/certificate_collection/collected_count")
                .and_then(Value::as_u64)
            {
                paragraph(
                    out,
                    &format!(
                        "{}: {count}",
                        text(
                            language,
                            "Certificates collected for public-attribute inspection",
                            "公開属性の検査用に収集した証明書数"
                        )
                    ),
                )?;
            }
        }
        ProbeProtocol::SshKex => {
            let verified = value_text(value, "/outcome") == Some("completed")
                && value
                    .pointer("/details/handshake_complete")
                    .and_then(Value::as_bool)
                    == Some(true)
                && value
                    .pointer("/details/host_identity_verified")
                    .and_then(Value::as_bool)
                    == Some(true)
                && value
                    .pointer("/details/user_authentication_attempted")
                    .and_then(Value::as_bool)
                    == Some(false)
                && value_text(value, "/details/negotiated_kex") == Some("mlkem768x25519-sha256")
                && value_text(value, "/details/host_signature") == Some("ssh-ed25519");
            paragraph(
                out,
                if verified {
                    text(language,"Hybrid key exchange completed and the peer matched the trusted host key. No user authentication or login was attempted.","ハイブリッド鍵交換が成立し、接続先が信頼済みホスト鍵と一致しました。ユーザー認証・ログインは行っていません。")
                } else {
                    text(language,"Verified hybrid key exchange could not be established. Check the reason below; this does not prove global server non-support.","ハイブリッド鍵交換と接続先の検証を完了できませんでした。下の理由を確認してください。この結果だけでサーバー全体の非対応を示すものではありません。")
                },
            )?;
            if verified {
                writeln!(out, "<dl>")?;
                summary_item(
                    out,
                    text(language, "Key exchange", "成立した鍵交換"),
                    "ML-KEM768 + X25519 (mlkem768x25519-sha256)",
                )?;
                summary_item(
                    out,
                    text(language, "Host verification", "接続先の確認"),
                    text(
                        language,
                        "Signature verified; matches the separately provisioned public key",
                        "署名を検証し、別経路で入手した公開鍵との一致を確認",
                    ),
                )?;
                writeln!(out, "</dl>")?;
            }
            paragraph(out,text(language,"Host authentication uses classical Ed25519; hybrid key exchange does not make the host signature post-quantum.","ホスト認証は従来方式のEd25519です。ハイブリッド鍵交換の成立は、ホスト署名のPQC対応を意味しません。"))?;
        }
        ProbeProtocol::SshOffers => {
            paragraph(out,text(language,"Server lists are unauthenticated advertisements. Key exchange and host verification were not performed.","サーバーの一覧は認証前の申告です。鍵交換・ホスト認証は実施していません。"))?;
            if let Some(banner) = value_text(value, "/details/server_identification") {
                paragraph(
                    out,
                    &format!(
                        "{}: {banner}",
                        text(
                            language,
                            "Unverified server identification",
                            "未検証のサーバー申告"
                        )
                    ),
                )?;
            }
            writeln!(out, "<dl>")?;
            for (field, en, ja) in [
                (
                    "kex_algorithms",
                    "Advertised key exchange list",
                    "提示された鍵交換一覧",
                ),
                (
                    "server_host_key_algorithms",
                    "Advertised host-key list",
                    "提示されたホスト鍵一覧",
                ),
            ] {
                let content = match value
                    .pointer(&format!("/details/offers/{field}"))
                    .and_then(Value::as_array)
                {
                    Some(entries) => {
                        let count = if entries.len() > 5 {
                            format!(
                                "{} {}",
                                entries.len(),
                                text(language, "records (first 5)", "件の記録（先頭5件）")
                            )
                        } else {
                            format!(
                                "{} {}",
                                entries.len(),
                                text(language, "records", "件の記録")
                            )
                        };
                        // Japanese does not use a space between a count and its suffix.
                        let count = if language == ReportLanguage::Ja {
                            count.replace(" 件", "件")
                        } else {
                            count
                        };
                        let names = entries
                            .iter()
                            .take(5)
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{count}: {names}")
                    }
                    None => unknown.into(),
                };
                summary_item(out, text(language, en, ja), &content)?;
            }
            writeln!(out, "</dl>")?;
        }
    }
    writeln!(
        out,
        "</section><section><h2>{}</h2>",
        text(
            language,
            "Limits and incomplete observations",
            "確認範囲と未完了の理由"
        )
    )?;
    paragraph(out,text(language,"This is one endpoint observation with one client profile. It does not enumerate every server capability or establish completed PQC migration.","1つのクライアント設定による1接続先の観測です。サーバーの全対応方式やPQC移行完了を確認するものではありません。"))?;
    if let Some(diagnostics) = value.get("diagnostics").and_then(Value::as_array) {
        for code in diagnostics.iter().filter_map(Value::as_str) {
            let reason = match code {
                "certificate_collection_limit" => text(language,"Some certificate attributes were not collected because of the certificate collection limit.","証明書の収集上限に達したため、一部の公開属性を収集できませんでした。"),
                "host_key_untrusted" => text(language,"The peer did not match the trusted public host key.","接続先のホスト鍵が、指定した信頼済み公開鍵と一致しませんでした。"),
                "host_signature_invalid" => text(language,"The server's key exchange signature was invalid.","サーバーの鍵交換署名を検証できませんでした。"),
                "no_common_algorithm" => text(language,"The fixed hybrid/host-key profile had no compatible algorithm with the peer. No classical fallback was attempted.","固定したハイブリッド方式・ホスト鍵方式で互換性がありませんでした。従来方式への切り替えは行っていません。"),
                "rekey_rejected" => text(language,"Additional key exchange was rejected; this probe accepts one initial exchange only.","追加の鍵交換を拒否しました。この検査は初回の鍵交換だけを対象とします。"),
                "certificate_parse_incomplete" => text(language,"Some collected certificate attributes could not be parsed.","収集した証明書の一部の公開属性を解析できませんでした。"),
                "certificate_validation_failed" => text(language,"Certificate-chain or name verification failed.","証明書チェーンまたは名前の検証に失敗しました。"),
                "deadline_exceeded" => text(language,"The observation reached its time limit.","観測の制限時間に達しました。"),
                "address_denied" => text(language,"The destination was denied by the address policy.","接続先がアドレス許可方針の対象外でした。"),
                "dns_failed" => text(language,"Name resolution failed.","名前解決に失敗しました。"),
                "traffic_limit" => text(language,"The observation reached its traffic limit.","送受信量の上限に達しました。"),
                "transport_failed" | "peer_disconnected" => text(language,"The connection failed or the peer disconnected before collection completed.","収集の完了前に接続が失敗したか、相手から切断されました。"),
                "invalid_target" => text(language,"The requested host or port was invalid.","指定されたホストまたはポートが不正でした。"),
                "protocol_mismatch" | "malformed_peer_data" => text(language,"The response did not match the expected protocol or could not be parsed safely.","応答が期待するプロトコルと一致しないか、安全に解析できませんでした。"),
                "tls_alert" => text(language,"The TLS peer reported an alert before the observation completed.","TLSの相手からアラートが返され、観測を完了できませんでした。"),
                "cancelled" => text(language,"The observation was cancelled.","観測が取り消されました。"),
                "peer_mismatch" => text(language,"The connected peer did not match the pinned destination.","実際の接続相手が固定した接続先と一致しませんでした。"),
                "unsupported_local" | "local_resource_limit" | "attempt_limit" => text(language,"A local capability or resource limit prevented collection.","ローカルの対応機能またはリソースの制限により収集できませんでした。"),
                "inspection_not_completed" => text(language,"The requested observation has not completed.","要求した観測が完了していません。"),
                "group_unavailable" => text(language,"The negotiated key exchange group was not available.","通信で決まった鍵共有グループを取得できませんでした。"),
                _ => text(language,"An observation step could not complete; its diagnostic code is in the technical information.","完了できなかった観測処理があります。診断コードは技術情報で確認できます。"),
            };
            paragraph(out, reason)?;
        }
    }
    writeln!(
        out,
        "</section><section><h2>{}</h2>",
        text(language, "What to check next", "次に確認すること")
    )?;
    let next = match protocol {
        ProbeProtocol::SshKex => text(language,"Check the independently provisioned public key and the server's ML-KEM768+X25519/Ed25519 configuration. Do not accept an unknown key or disable verification to make the test pass.","別経路で入手した公開鍵と、サーバーのML-KEM768＋X25519／Ed25519設定を確認してください。成功させるために未知の鍵を受け入れたり、検証を無効化したりしないでください。"),
        ProbeProtocol::Tls if value_text(value,"/details/verification/certificate_chain_and_name")==Some("failed") => text(language,"Check the destination name, validity period and trusted CA. Correct the cause without disabling certificate verification.","接続先名・有効期間・信頼するCAを確認し、証明書検証を無効化せずに原因を解消してください。"),
        ProbeProtocol::Tls => text(language,"Use certificate-management records to check revocation and any missing certificate attributes. Review other client profiles and application usage separately.","証明書管理の記録で失効状態や未収集の公開属性を確認してください。他のクライアント設定やアプリケーションでの利用は別途確認が必要です。"),
        ProbeProtocol::SshOffers => text(language,"Check actual negotiated algorithms and the host identity with an authorized SSH client. Use server configuration to establish the full supported set.","許可されたSSHクライアントで実際に選ばれる方式とホストの本人性を確認してください。対応方式の全体はサーバー設定と照合してください。"),
    };
    let has = |code: &str| {
        value
            .get("diagnostics")
            .and_then(Value::as_array)
            .is_some_and(|values| values.iter().any(|value| value.as_str() == Some(code)))
    };
    let next = if has("dns_failed") {
        text(
            language,
            "Check the requested host name and name-resolution settings before retrying.",
            "指定したホスト名と名前解決の設定を確認してから再実行してください。",
        )
    } else if has("address_denied") || has("peer_mismatch") {
        text(language,"Check the resolved addresses and approved destination policy. Explicitly authorize an internal CIDR only when that destination is intended.","名前解決結果と接続先の許可方針を確認してください。内部CIDRの明示許可は、その接続先が意図した対象の場合に限って設定してください。")
    } else if has("invalid_target") || has("protocol_mismatch") || has("malformed_peer_data") {
        text(
            language,
            "Check the host, port and selected protocol against the endpoint configuration.",
            "ホスト・ポート・選択したプロトコルを接続先の設定と照合してください。",
        )
    } else if has("transport_failed") || has("peer_disconnected") || has("deadline_exceeded") {
        text(language,"Check service availability, the network path and server logs before a separate retry. This invocation does not retry automatically.","サービスの稼働状態・通信経路・サーバーログを確認してから別途再実行してください。この実行内では自動再試行しません。")
    } else {
        next
    };
    paragraph(out, next)?;
    writeln!(out, "</section>")?;
    Ok(())
}

fn html(
    rows: &[Row],
    value: &Value,
    protocol: ProbeProtocol,
    language: ReportLanguage,
    redacted: bool,
    tls_envelope_allows_verified: bool,
) -> Result<String> {
    let mut out = String::new();
    writeln!(out,"<!doctype html><html lang=\"{}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><style>body{{font:16px/1.6 system-ui,sans-serif;color:#15243b;background:#f5f7fb;margin:0}}main{{max-width:1100px;margin:auto;padding:24px}}dt,dd,p{{overflow-wrap:anywhere}}section{{background:white;padding:16px 20px;margin:16px 0;border-radius:8px}}dt{{font-weight:600}}dd{{margin:0 0 12px}}table{{width:100%;table-layout:fixed;border-collapse:collapse;background:white}}th,td{{text-align:left;vertical-align:top;padding:12px;border:1px solid #c9d2df;overflow-wrap:anywhere;word-break:break-word}}th{{width:48%}}code{{display:block;font-size:.8em;overflow-wrap:anywhere}}summary{{cursor:pointer;padding:12px;font-weight:600}}summary:focus-visible{{outline:3px solid #1c60b9;outline-offset:2px}}.loss{{border-left:4px solid #a63d00;padding:12px;background:#fff4e8}}@media(max-width:600px){{main{{padding:12px}}th,td{{padding:8px}}}}</style></head><body><main><h1>{}</h1><p>{}</p>",text(language,"en","ja"),japanese::html(title(protocol,language)),japanese::html(title(protocol,language)),japanese::html(text(language,"A single bounded endpoint observation. Expand technical information for every recorded field.","接続先に対する、上限を設けた1回の観測です。全記録は技術情報で確認できます。")))?;
    if redacted {
        writeln!(
            out,
            "<p class=\"loss\">{}</p>",
            japanese::html(loss(language))
        )?;
    }
    writeln!(
        out,
        "<section><h2>{}</h2><dl>",
        text(language, "Overview", "観測の概要")
    )?;
    for row in overview(rows).filter(|row| {
        matches!(
            row.path.as_str(),
            "/outcome" | "/target/host" | "/target/port" | "/target/connected_peer"
        )
    }) {
        writeln!(
            out,
            "<dt>{}</dt><dd>{}</dd>",
            japanese::html(label(&row.key, language)),
            japanese::html(&display_value(row, language))
        )?;
    }
    writeln!(out, "</dl></section>")?;
    protocol_summary(
        &mut out,
        value,
        protocol,
        language,
        tls_envelope_allows_verified,
    )?;
    writeln!(out,"<details id=\"technical-details\"><summary>{}</summary><table><caption>{}</caption><thead><tr><th scope=\"col\">{}</th><th scope=\"col\">{}</th></tr></thead><tbody>",text(language,"Technical information — all recorded fields","技術情報 — 記録した全項目"),text(language,"Facts with their original machine-field paths","元の機械項目パスと対応する事実"),text(language,"Field","項目"),text(language,"Value","内容"))?;
    for row in rows {
        writeln!(
            out,
            "<tr data-field=\"{}\"><th scope=\"row\">{}<code>{}</code></th><td>{}</td></tr>",
            japanese::html(&row.path),
            japanese::html(label(&row.key, language)),
            japanese::html(&row.path),
            japanese::html(&display_value(row, language))
        )?;
    }
    writeln!(
        out,
        "</tbody></table><p>{}</p></details></main></body></html>",
        japanese::html(scope(protocol, language))
    )?;
    Ok(out)
}
fn markdown(
    rows: &[Row],
    protocol: ProbeProtocol,
    language: ReportLanguage,
    redacted: bool,
) -> Result<String> {
    let mut out = format!(
        "# {}\n\n{}\n",
        title(protocol, language),
        scope(protocol, language)
    );
    if redacted {
        writeln!(out, "\n{}\n", loss(language))?;
    }
    let escape = |value: &str| japanese::md(&value.replace('&', "&amp;")).replace('\r', " ");
    writeln!(out, "\n## {}\n", text(language, "Overview", "観測の概要"))?;
    for row in overview(rows) {
        writeln!(
            out,
            "- {}: {}",
            escape(label(&row.key, language)),
            escape(&display_value(row, language))
        )?;
    }
    writeln!(
        out,
        "\n|{}|{}|{}|\n|---|---|---|",
        text(language, "Field path", "項目パス"),
        text(language, "Field", "項目"),
        text(language, "Value", "内容")
    )?;
    for row in rows {
        writeln!(
            out,
            "|{}|{}|{}|",
            escape(&row.path),
            escape(label(&row.key, language)),
            escape(&display_value(row, language))
        )?;
    }
    Ok(out)
}
