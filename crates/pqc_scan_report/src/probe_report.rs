//! Human-readable views and unchanged-language JSON for explicit endpoint probes.
use crate::{japanese, ReportLanguage};
use anyhow::{anyhow, Result};
use pqc_scan_core::probe::{ProbeProtocol, ProbeReport};
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
        report.protocol,
        language,
        value["redaction_applied"] == true,
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
        ProbeProtocol::SshOffers => text(
            language,
            "SSH advertised algorithm observation",
            "SSH提示アルゴリズムの観測結果",
        ),
    }
}
fn scope(protocol: ProbeProtocol, language: ReportLanguage) -> &'static str {
    match protocol {
        ProbeProtocol::Tls=>text(language,"Verification state and any negotiated facts describe this single endpoint and client profile. Inspection limits and diagnostics qualify the result. This observation does not establish exhaustive server capabilities or completed PQC migration.","検証状態と通信で決まった事実は、この接続先とクライアント設定に対する1回の観測を示します。収集上限と診断情報を併せて確認してください。サーバーの全能力やPQC移行完了を保証する結果ではありません。"),
        ProbeProtocol::SshOffers=>text(language,"Unauthenticated advertisements describe the server's claims when lists are available. Lists retain their original order. Client lists are diagnostic proposals, not implemented cryptographic capabilities. Key exchange, host identity verification and authentication were not performed.","収集できた提示一覧は、サーバーによる認証前の申告として元の順に記録します。クライアント側の一覧は診断用の提案で、暗号方式の実装対応を示すものではありません。鍵交換・ホストの検証・認証は実施していません。"),
    }
}
fn loss(language: ReportLanguage) -> &'static str {
    text(language,"Strings resembling secrets were omitted. Original values cannot be recovered from this output. Observation completion describes collection; it does not remove this export loss.","秘密情報に見える文字列を省略しました。該当箇所の原文はこの出力から復元できません。観測完了は収集の状態を示すもので、この省略がなかったという意味ではありません。")
}
fn label(key: &str, language: ReportLanguage) -> &str {
    let labels = match key {
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
fn html(
    rows: &[Row],
    protocol: ProbeProtocol,
    language: ReportLanguage,
    redacted: bool,
) -> Result<String> {
    let mut out = String::new();
    writeln!(out,"<!doctype html><html lang=\"{}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><style>body{{font:16px/1.6 system-ui,sans-serif;color:#15243b;background:#f5f7fb;margin:0}}main{{max-width:1100px;margin:auto;padding:24px}}dt,dd{{overflow-wrap:anywhere}}table{{width:100%;table-layout:fixed;border-collapse:collapse;background:white}}th,td{{text-align:left;vertical-align:top;padding:12px;border:1px solid #c9d2df;overflow-wrap:anywhere;word-break:break-word}}th{{width:48%}}code{{display:block;font-size:.8em;overflow-wrap:anywhere}}summary{{cursor:pointer;padding:12px;font-weight:600}}summary:focus-visible{{outline:3px solid #1c60b9;outline-offset:2px}}.loss{{border-left:4px solid #a63d00;padding:12px;background:#fff4e8}}@media(max-width:600px){{main{{padding:12px}}th,td{{padding:8px}}}}</style></head><body><main><h1>{}</h1><p>{}</p>",text(language,"en","ja"),japanese::html(title(protocol,language)),japanese::html(title(protocol,language)),japanese::html(scope(protocol,language)))?;
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
    for row in overview(rows) {
        writeln!(
            out,
            "<dt>{}</dt><dd>{}</dd>",
            japanese::html(label(&row.key, language)),
            japanese::html(&display_value(row, language))
        )?;
    }
    writeln!(out, "</dl></section>")?;
    writeln!(out,"<details open><summary>{}</summary><table><caption>{}</caption><thead><tr><th scope=\"col\">{}</th><th scope=\"col\">{}</th></tr></thead><tbody>",text(language,"Recorded fields","記録した項目"),text(language,"Facts with their original machine-field paths","元の機械項目パスと対応する事実"),text(language,"Field","項目"),text(language,"Value","内容"))?;
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
    writeln!(out, "</tbody></table></details></main></body></html>")?;
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
