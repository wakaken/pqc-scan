use pqc_scan_core::probe::{
    ProbeDiagnostic, ProbeOutcome, ProbeProtocol, ProbeReport, ProbeTarget,
};
use pqc_scan_core::target_policy::{
    Cancellation, ProbeBudget, ProbeError, ProbeLimits, Resolver, TargetPolicy, TargetRequest,
};
use pqc_scan_report::{write_probe_reports, ReportLanguage};
use serde_json::{json, Value};
use std::{
    fs,
    net::IpAddr,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

struct Fixture(PathBuf);
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
impl Fixture {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
            "pqc-probe-report-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        )))
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct Local;
impl Resolver for Local {
    fn resolve(
        &mut self,
        _: &str,
        _: Instant,
        _: &Cancellation,
    ) -> Result<Vec<IpAddr>, ProbeError> {
        Ok(vec!["127.0.0.1".parse().unwrap()])
    }
}
fn report(protocol: ProbeProtocol, details: Value) -> ProbeReport<Value> {
    let limits = ProbeLimits::default();
    let budget = ProbeBudget::new(limits.clone(), Cancellation::new()).unwrap();
    let authorized = TargetPolicy::new(&["127.0.0.0/8".into()])
        .unwrap()
        .resolve(
            TargetRequest::new("localhost", 443).unwrap(),
            &mut Local,
            &budget,
        )
        .unwrap();
    let mut target = ProbeTarget::authorized(&authorized);
    target.connected_peer = target.pinned_address;
    let mut report = ProbeReport::new(protocol, target, &limits, details);
    report.set_outcome(ProbeOutcome::Completed, vec![]);
    report
}

#[test]
fn tls_fact_paths_and_unknown_fields_survive_both_languages_without_machine_drift() {
    let root = Fixture::new();
    let report = report(
        ProbeProtocol::Tls,
        json!({
            "verification":{"verification_name":"example.test","sent_sni":"example.test","certificate_chain_and_name":"verified","revocation":"not_checked","root_set_sha256":"a".repeat(64)},
            "handshake_complete":true,
            "negotiated":{"version":{"id":772,"name":"TLSv1_3"},"cipher_suite":{"id":4866,"name":"TLS13_AES_256_GCM_SHA384"},"key_exchange_group":{"id":4588,"name":"X25519MLKEM768"}},
            "peer_certificates":[{"certificate_der_sha256":"b".repeat(64)},{"certificate_der_sha256":"c".repeat(64)}],
            "future/<field>":{"values":["<script>alert(1)</script>","literal | [value] & text"]}
        }),
    );
    let original = serde_json::to_value(&report).unwrap();
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let dir = root.0.join(if language == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        let outputs = write_probe_reports(&report, &dir, language).unwrap();
        assert_eq!(outputs.len(), 3);
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 3);
        let value: Value =
            serde_json::from_slice(&fs::read(dir.join("probe.tls.v1.json")).unwrap()).unwrap();
        assert_eq!(value, original);
        let html = fs::read_to_string(dir.join("report.html")).unwrap();
        let md = fs::read_to_string(dir.join("report.md")).unwrap();
        for (field, en, ja, value) in [
            (
                "dns_timeout_ns",
                "DNS timeout (ns)",
                "名前解決の制限時間（ナノ秒）",
                "5000000000",
            ),
            (
                "tcp_timeout_ns",
                "TCP timeout (ns)",
                "TCP接続の制限時間（ナノ秒）",
                "3000000000",
            ),
            (
                "protocol_timeout_ns",
                "Protocol timeout (ns)",
                "プロトコル処理の制限時間（ナノ秒）",
                "5000000000",
            ),
            (
                "overall_timeout_ns",
                "Overall timeout (ns)",
                "全体の制限時間（ナノ秒）",
                "15000000000",
            ),
        ] {
            let path = format!("/limits/{field}");
            let label = if language == ReportLanguage::Ja {
                ja
            } else {
                en
            };
            let html_row = html
                .lines()
                .find(|line| line.contains(&format!("data-field=\"{path}\"")))
                .unwrap();
            assert!(html_row.contains(label));
            assert!(html_row.contains(value));
            let md_row = md.lines().find(|line| line.contains(&path)).unwrap();
            assert!(md_row.contains(label));
            assert!(md_row.contains(value));
            assert_eq!(original["limits"][field], value.parse::<u64>().unwrap());
        }
        for (field, value) in [
            ("/details/negotiated/version/id", "772"),
            ("/details/negotiated/key_exchange_group/id", "4588"),
            (
                "/details/peer_certificates/0/certificate_der_sha256",
                &"b".repeat(64),
            ),
            (
                "/details/peer_certificates/1/certificate_der_sha256",
                &"c".repeat(64),
            ),
        ] {
            let row = html
                .lines()
                .find(|row| row.contains(&format!("data-field=\"{field}\"")))
                .unwrap();
            assert!(row.contains(value));
            let row = md.lines().find(|row| row.contains(field)).unwrap();
            assert!(row.contains(value));
        }
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(!html.contains("<script>"));
        assert!(html.contains("/details/future~1&lt;field&gt;/values/0"));
        assert!(md.contains("literal \\| \\[value\\] &amp; text"));
        assert!(!html.contains("<script src="));
        assert!(!html.contains("<link"));
        assert!(html.contains(if language == ReportLanguage::Ja {
            "lang=\"ja\""
        } else {
            "lang=\"en\""
        }));
        assert!(html.contains(if language == ReportLanguage::Ja {
            "検証する名前"
        } else {
            "Verification name"
        }));
    }
    assert_eq!(
        fs::read(root.0.join("en/probe.tls.v1.json")).unwrap(),
        fs::read(root.0.join("ja/probe.tls.v1.json")).unwrap()
    );
    assert_eq!(serde_json::to_value(&report).unwrap(), original);
}

#[test]
fn ssh_preserves_every_ordered_offer_and_extension_marker_as_unauthenticated() {
    let root = Fixture::new();
    let lists = [
        "kex_algorithms",
        "server_host_key_algorithms",
        "encryption_algorithms_client_to_server",
        "encryption_algorithms_server_to_client",
        "mac_algorithms_client_to_server",
        "mac_algorithms_server_to_client",
        "compression_algorithms_client_to_server",
        "compression_algorithms_server_to_client",
        "languages_client_to_server",
        "languages_server_to_client",
    ];
    let mut offers = serde_json::Map::new();
    for (i, name) in lists.iter().enumerate() {
        offers.insert(
            (*name).into(),
            json!([format!("first-{i}"), format!("second-{i}")]),
        );
    }
    let report = report(
        ProbeProtocol::SshOffers,
        json!({"assurance":"unauthenticated_advertisement","server_identification":"SSH-2.0-fixture","offers":offers,"extension_markers":["ext-info-s","kex-strict-s-v00@openssh.com"]}),
    );
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let dir = root.0.join(if language == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_probe_reports(&report, &dir, language).unwrap();
        assert!(dir.join("probe.ssh-offers.v1.json").exists());
        assert!(!dir.join("probe.tls.v1.json").exists());
        let html = fs::read_to_string(dir.join("report.html")).unwrap();
        let md = fs::read_to_string(dir.join("report.md")).unwrap();
        for (i, name) in lists.iter().enumerate() {
            for (index, value) in [(0, format!("first-{i}")), (1, format!("second-{i}"))] {
                let path = format!("/details/offers/{name}/{index}");
                assert!(html
                    .lines()
                    .any(|line| line.contains(&path) && line.contains(&value)));
                assert!(md
                    .lines()
                    .any(|line| line.contains(&path) && line.contains(&value)));
            }
        }
        assert!(
            html.find("ext-info-s").unwrap() < html.find("kex-strict-s-v00@openssh.com").unwrap()
        );
        assert!(html.contains(if language == ReportLanguage::Ja {
            "認証前の申告"
        } else {
            "Unauthenticated advertisements"
        }));
    }
}

#[test]
fn recognized_secret_syntax_is_omitted_consistently_and_disclosed_as_export_loss() {
    let root = Fixture::new();
    let report = report(
        ProbeProtocol::SshOffers,
        json!({"server_identification":"SSH-2.0-fixture password=PROBE_PASSWORD_CANARY","unknown":"-----BEGIN PRIVATE KEY-----\nPROBE_PRIVATE_CANARY\n-----END PRIVATE KEY-----","offers":{"kex_algorithms":["curve25519-sha256"]}}),
    );
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let dir = root.0.join(if language == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_probe_reports(&report, &dir, language).unwrap();
        for name in ["probe.ssh-offers.v1.json", "report.html", "report.md"] {
            let content = fs::read_to_string(dir.join(name)).unwrap();
            assert!(!content.contains("PROBE_PASSWORD_CANARY"));
            assert!(!content.contains("PROBE_PRIVATE_CANARY"));
            assert!(!content.contains("BEGIN PRIVATE KEY"));
            assert!(content.contains(if name == "report.md" {
                "\\[masked-sensitive-content\\]"
            } else {
                "[masked-sensitive-content]"
            }));
        }
        let value: Value =
            serde_json::from_slice(&fs::read(dir.join("probe.ssh-offers.v1.json")).unwrap())
                .unwrap();
        assert_eq!(value["redaction_applied"], true);
        assert_eq!(
            value["details"]["server_identification"],
            "[masked-sensitive-content]"
        );
        let html = fs::read_to_string(dir.join("report.html")).unwrap();
        assert!(html.contains(if language == ReportLanguage::Ja {
            "該当箇所の原文はこの出力から復元できません"
        } else {
            "Original values cannot be recovered from this output"
        }));
    }
    assert_eq!(
        serde_json::to_value(&report).unwrap()["redaction_applied"],
        false
    );
}

#[test]
fn incomplete_outcomes_keep_raw_reason_codes_and_invalid_envelopes_write_nothing() {
    let root = Fixture::new();
    let mut report = report(ProbeProtocol::Tls, json!({"negotiated":null}));
    report.set_outcome(
        ProbeOutcome::Partial,
        vec![ProbeDiagnostic::CertificateCollectionLimit],
    );
    write_probe_reports(&report, &root.0.join("partial"), ReportLanguage::Ja).unwrap();
    let html = fs::read_to_string(root.0.join("partial/report.html")).unwrap();
    assert!(html.contains("certificate_collection_limit"));
    assert!(html.contains("partial"));
    assert!(html.contains("一部を収集できませんでした"));
    report.set_outcome(
        ProbeOutcome::Completed,
        vec![ProbeDiagnostic::CertificateCollectionLimit],
    );
    assert!(write_probe_reports(&report, &root.0.join("invalid"), ReportLanguage::En).is_err());
    assert!(!root.0.join("invalid").exists());
}

#[test]
fn diagnostic_client_proposals_are_not_labeled_as_remote_advertisements() {
    let root = Fixture::new();
    let report = report(
        ProbeProtocol::SshOffers,
        json!({
            "client_profile":{"purpose":"diagnostic_proposal_only","offers":{"kex_algorithms":["client-only"]},"first_kex_packet_follows":false},
            "offers":{"kex_algorithms":["server-only"]}
        }),
    );
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let dir = root.0.join(if language == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_probe_reports(&report, &dir, language).unwrap();
        let html = fs::read_to_string(dir.join("report.html")).unwrap();
        let client_row = html
            .lines()
            .find(|line| {
                line.contains("data-field=\"/details/client_profile/offers/kex_algorithms/0\"")
            })
            .unwrap();
        assert!(client_row.contains("client-only"));
        assert!(!client_row.contains("Advertised"));
        assert!(!client_row.contains("提示された"));
        assert!(html.contains(if language == ReportLanguage::Ja {
            "診断用の提案"
        } else {
            "diagnostic proposals"
        }));
    }
}

#[test]
fn failed_serialization_does_not_disclose_arbitrary_error_text_or_create_outputs() {
    struct CannotSerialize;
    impl serde::Serialize for CannotSerialize {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("SERIALIZER_ERROR_CANARY"))
        }
    }
    let root = Fixture::new();
    let base = report(ProbeProtocol::Tls, json!({}));
    let invalid = ProbeReport::new(
        base.protocol,
        base.target,
        &ProbeLimits::default(),
        CannotSerialize,
    );
    let error = write_probe_reports(&invalid, &root.0, ReportLanguage::En).unwrap_err();
    assert!(!error.to_string().contains("SERIALIZER_ERROR_CANARY"));
    assert!(!root.0.exists());
}

#[test]
fn outcome_is_visible_before_potentially_long_protocol_details() {
    let root = Fixture::new();
    let report = report(
        ProbeProtocol::SshOffers,
        json!({"offers":{"kex_algorithms":["detail-algorithm-marker"]}}),
    );
    write_probe_reports(&report, &root.0, ReportLanguage::En).unwrap();
    for filename in ["report.html", "report.md"] {
        let body = fs::read_to_string(root.0.join(filename)).unwrap();
        assert!(body.find("(completed)").unwrap() < body.find("detail-algorithm-marker").unwrap());
    }
}

#[test]
fn tls_initial_html_explains_verified_facts_partial_collection_and_next_checks() {
    let root = Fixture::new();
    let mut report = report(
        ProbeProtocol::Tls,
        json!({
            "verification":{"verification_name":"example.test","certificate_chain_and_name":"verified","trust_source":"bundled_mozilla","revocation":"not_checked","root_set_sha256":"a".repeat(64)},
            "handshake_complete":true,"handshake_kind":"full",
            "negotiated":{"version":{"id":772,"name":"TLSv1_3"},"cipher_suite":{"id":4866,"name":"TLS13_AES_256_GCM_SHA384"},"key_exchange_group":{"id":4588,"name":"X25519MLKEM768"},"group_classification":"hybrid"},
            "peer_certificates":[{"certificate_der_sha256":"b".repeat(64)}],
            "certificate_collection":{"status":"partial","presented_count":17,"collected_count":1}
        }),
    );
    report.set_outcome(
        ProbeOutcome::Partial,
        vec![ProbeDiagnostic::CertificateCollectionLimit],
    );
    let original = serde_json::to_value(&report).unwrap();
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let dir = root.0.join(if language == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_probe_reports(&report, &dir, language).unwrap();
        let html = fs::read_to_string(dir.join("report.html")).unwrap();
        let (initial, technical) = html
            .split_once("<details id=\"technical-details\">")
            .unwrap();
        assert!(!html.contains("<details open"));
        assert!(initial.contains("example.test"));
        assert!(initial.contains("TLS 1.3"));
        assert!(initial.contains("X25519MLKEM768"));
        assert!(initial.contains("TLS13_AES_256_GCM_SHA384"));
        assert!(initial.contains("Mozilla"));
        assert!(!initial.contains("/details/"));
        assert!(!initial.contains(&"b".repeat(64)));
        assert!(technical.contains("/details/peer_certificates/0/certificate_der_sha256"));
        assert!(technical.contains(&"b".repeat(64)));
        for text in if language == ReportLanguage::Ja {
            [
                "今回わかったこと",
                "証明書の収集上限",
                "失効確認は実施していません",
                "次に確認すること",
            ]
        } else {
            [
                "What was observed",
                "certificate collection limit",
                "Revocation was not checked",
                "What to check next",
            ]
        } {
            assert!(initial.contains(text), "{text}");
        }
        let machine: Value =
            serde_json::from_slice(&fs::read(dir.join("probe.tls.v1.json")).unwrap()).unwrap();
        assert_eq!(machine, original);
    }
}

#[test]
fn failed_tls_does_not_promote_unverified_negotiation_into_initial_html() {
    let root = Fixture::new();
    let mut report = report(
        ProbeProtocol::Tls,
        json!({
            "handshake_complete":false,"handshake_kind":null,
            "verification":{"certificate_chain_and_name":"failed","verification_name":"wrong.test"},
            "negotiated":{"version":{"name":"UNVERIFIED_TLS"},"key_exchange_group":{"name":"UNVERIFIED_GROUP"}}
        }),
    );
    report.set_outcome(
        ProbeOutcome::Failed,
        vec![ProbeDiagnostic::CertificateValidationFailed],
    );
    write_probe_reports(&report, &root.0, ReportLanguage::Ja).unwrap();
    let html = fs::read_to_string(root.0.join("report.html")).unwrap();
    let (initial, technical) = html
        .split_once("<details id=\"technical-details\">")
        .unwrap();
    assert!(!initial.contains("UNVERIFIED_GROUP"));
    assert!(!initial.contains("UNVERIFIED_TLS"));
    assert!(initial.contains("検証済みの通信結果は得られていません"));
    assert!(initial.contains("接続先名・有効期間・信頼するCA"));
    assert!(technical.contains("UNVERIFIED_GROUP"));
}

#[test]
fn ssh_initial_html_summarizes_remote_offers_and_keeps_all_facts_in_closed_details() {
    let root = Fixture::new();
    let report = report(
        ProbeProtocol::SshOffers,
        json!({
            "assurance":"unauthenticated_advertisement","server_identification":"SSH-2.0-fixture",
            "client_profile":{"offers":{"kex_algorithms":["CLIENT_PROPOSAL_ONLY"]}},
            "offers":{"kex_algorithms":["server-first","server-second","server-third","server-fourth","server-fifth","hidden-sixth-entry"],"server_host_key_algorithms":["ssh-ed25519"]}
        }),
    );
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        let dir = root.0.join(if language == ReportLanguage::En {
            "en"
        } else {
            "ja"
        });
        write_probe_reports(&report, &dir, language).unwrap();
        let html = fs::read_to_string(dir.join("report.html")).unwrap();
        let (initial, technical) = html
            .split_once("<details id=\"technical-details\">")
            .unwrap();
        assert!(initial.contains("server-first"));
        assert!(initial.contains("ssh-ed25519"));
        assert!(!initial.contains("CLIENT_PROPOSAL_ONLY"));
        assert!(!initial.contains("hidden-sixth-entry"));
        assert!(technical.contains("CLIENT_PROPOSAL_ONLY"));
        assert!(technical.contains("hidden-sixth-entry"));
        assert!(initial.contains(if language == ReportLanguage::Ja {
            "6件の記録（先頭5件）"
        } else {
            "6 records (first 5)"
        }));
        assert!(initial.contains(if language == ReportLanguage::Ja {
            "鍵交換・ホスト認証は実施していません"
        } else {
            "Key exchange and host verification were not performed"
        }));
    }
}

#[test]
fn unavailable_ssh_lists_are_unconfirmed_and_network_failure_has_actionable_guidance() {
    let root = Fixture::new();
    let mut report = report(
        ProbeProtocol::SshOffers,
        json!({"server_identification":"<script>alert('peer')</script>","offers":null}),
    );
    report.set_outcome(ProbeOutcome::Failed, vec![ProbeDiagnostic::DnsFailed]);
    write_probe_reports(&report, &root.0, ReportLanguage::Ja).unwrap();
    let html = fs::read_to_string(root.0.join("report.html")).unwrap();
    let (initial, _) = html
        .split_once("<details id=\"technical-details\">")
        .unwrap();
    assert!(initial.contains("未確認"));
    assert!(!initial.contains("0件の記録"));
    assert!(!initial.contains("<script>"));
    assert!(initial.contains("&lt;script&gt;"));
    assert!(initial.contains("名前解決の設定"));
}

#[test]
fn contradictory_tls_details_never_override_failed_or_fatal_envelope() {
    let root = Fixture::new();
    for (index, outcome, diagnostic) in [
        (
            0,
            ProbeOutcome::Failed,
            ProbeDiagnostic::CertificateValidationFailed,
        ),
        (
            1,
            ProbeOutcome::Partial,
            ProbeDiagnostic::CertificateValidationFailed,
        ),
        (2, ProbeOutcome::Partial, ProbeDiagnostic::TlsAlert),
        (
            3,
            ProbeOutcome::Failed,
            ProbeDiagnostic::CertificateCollectionLimit,
        ),
    ] {
        let mut report = report(
            ProbeProtocol::Tls,
            json!({
                "handshake_complete":true,"handshake_kind":"full",
                "verification":{"certificate_chain_and_name":"verified"},
                "negotiated":{"version":{"id":772,"name":"TLSv1_3"},"key_exchange_group":{"name":"CONTRADICTORY_GROUP"}}
            }),
        );
        report.set_outcome(outcome, vec![diagnostic]);
        for language in [ReportLanguage::En, ReportLanguage::Ja] {
            let dir = root.0.join(format!(
                "{index}-{}",
                if language == ReportLanguage::En {
                    "en"
                } else {
                    "ja"
                }
            ));
            write_probe_reports(&report, &dir, language).unwrap();
            let html = fs::read_to_string(dir.join("report.html")).unwrap();
            let (initial, technical) = html
                .split_once("<details id=\"technical-details\">")
                .unwrap();
            assert!(!initial.contains("CONTRADICTORY_GROUP"));
            assert!(!initial.contains("A fresh TLS handshake completed"));
            assert!(!initial.contains("証明書チェーンと名前を検証し、新しいTLS接続を確立しました"));
            assert!(initial.contains(if language == ReportLanguage::En {
                "No verified TLS connection result"
            } else {
                "検証済みの通信結果は得られていません"
            }));
            assert!(technical.contains("CONTRADICTORY_GROUP"));
            let machine: Value =
                serde_json::from_slice(&fs::read(dir.join("probe.tls.v1.json")).unwrap()).unwrap();
            assert_eq!(machine, serde_json::to_value(&report).unwrap());
        }
    }
}
