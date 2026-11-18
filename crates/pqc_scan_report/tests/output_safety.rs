#![cfg(unix)]
use pqc_scan_core::probe::{ProbeProtocol, ProbeReport, ProbeTarget};
use pqc_scan_core::target_policy::{ProbeLimits, TargetRequest};
use pqc_scan_core::{scan_repository_native, NativeOptions, ScanConfig};
use pqc_scan_report::{
    write_assessment_json, write_cyclonedx_17, write_diagnostics, write_document_reports,
    write_inventory_json, write_native_json, write_probe_reports, ReportFormat, ReportLanguage,
};
use pqc_scan_rules::RuleSet;
use std::{
    fs,
    os::unix::fs::symlink,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "pqc-writers-safe-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(path.join("input")).unwrap();
        fs::create_dir_all(path.join("rules")).unwrap();
        fs::write(path.join("input/empty.txt"), "no cryptography").unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn every_static_export_rejects_a_final_link_and_preserves_the_external_file() {
    let scratch = Scratch::new();
    let document = scan_repository_native(
        &ScanConfig {
            root: scratch.0.join("input"),
            threads: 1,
            ..Default::default()
        },
        &RuleSet::load_from_dir(&scratch.0.join("rules")).unwrap(),
        &[],
        &NativeOptions::new(Some("test/output-safety")).unwrap(),
    )
    .unwrap();
    let assessment = pqc_scan_core::policy::evaluate(
        document.native().unwrap(),
        &Default::default(),
        pqc_scan_core::policy::PROFILE_ID,
    )
    .unwrap();
    for language in [ReportLanguage::En, ReportLanguage::Ja] {
        for filename in [
            "report.json",
            "report.html",
            "report.rules.html",
            "report.md",
            "report.sarif",
            "cbom.json",
            "dependency-sbom.json",
            "diagnostics.json",
            "scan.inventory.v1.json",
            "scan.native.v1.json",
            "scan.assessment.v1.json",
            "cbom.cdx-1.7.json",
            "dependency-sbom.cdx-1.7.json",
        ] {
            let root = scratch.0.join(format!("{language:?}-{filename}"));
            fs::create_dir(&root).unwrap();
            let victim = root.with_extension("canary");
            fs::write(&victim, b"EXTERNAL_CANARY").unwrap();
            symlink(&victim, root.join(filename)).unwrap();
            let result = match filename {
                "diagnostics.json" => write_diagnostics(&document, &root).map(|_| ()),
                "scan.inventory.v1.json" => write_inventory_json(&document, &root).map(|_| ()),
                "scan.native.v1.json" => write_native_json(&document, &root).map(|_| ()),
                "scan.assessment.v1.json" => write_assessment_json(&assessment, &root).map(|_| ()),
                "cbom.cdx-1.7.json" | "dependency-sbom.cdx-1.7.json" => {
                    write_cyclonedx_17(&document, &root).map(|_| ())
                }
                _ => write_document_reports(
                    &document,
                    Some(&assessment),
                    true,
                    &root,
                    ReportFormat::All,
                    language,
                )
                .map(|_| ()),
            };
            assert!(result.is_err(), "accepted {filename}/{language:?}");
            assert_eq!(
                fs::read(&victim).unwrap(),
                b"EXTERNAL_CANARY",
                "changed {filename}"
            );
            assert!(fs::symlink_metadata(root.join(filename))
                .unwrap()
                .file_type()
                .is_symlink());
        }
    }
    let legitimate = scratch.0.join("legitimate/nested");
    let first = write_document_reports(
        &document,
        Some(&assessment),
        true,
        &legitimate,
        ReportFormat::All,
        ReportLanguage::En,
    )
    .unwrap();
    let bytes: Vec<_> = first.iter().map(|p| fs::read(p).unwrap()).collect();
    let second = write_document_reports(
        &document,
        Some(&assessment),
        true,
        &legitimate,
        ReportFormat::All,
        ReportLanguage::En,
    )
    .unwrap();
    assert_eq!(
        bytes,
        second
            .iter()
            .map(|p| fs::read(p).unwrap())
            .collect::<Vec<_>>()
    );
}

#[test]
fn all_probe_protocol_exports_reject_links_without_networking() {
    let scratch = Scratch::new();
    for protocol in [
        ProbeProtocol::Tls,
        ProbeProtocol::SshOffers,
        ProbeProtocol::SshKex,
    ] {
        let report = ProbeReport::new(
            protocol,
            ProbeTarget::requested(&TargetRequest::new("localhost", 443).unwrap()),
            &ProbeLimits::default(),
            (),
        );
        let json = match protocol {
            ProbeProtocol::Tls => "probe.tls.v1.json",
            ProbeProtocol::SshOffers => "probe.ssh-offers.v1.json",
            ProbeProtocol::SshKex => "probe.ssh-kex.v1.json",
        };
        for filename in [json, "report.html", "report.md"] {
            let root = scratch.0.join(format!("{protocol:?}-{filename}"));
            fs::create_dir(&root).unwrap();
            let victim = root.with_extension("canary");
            fs::write(&victim, b"EXTERNAL_CANARY").unwrap();
            symlink(&victim, root.join(filename)).unwrap();
            assert!(write_probe_reports(&report, &root, ReportLanguage::En).is_err());
            assert_eq!(fs::read(victim).unwrap(), b"EXTERNAL_CANARY");
        }
        let normal = scratch.0.join(format!("normal-{protocol:?}"));
        assert_eq!(
            write_probe_reports(&report, &normal, ReportLanguage::En)
                .unwrap()
                .len(),
            3
        );
    }
}
