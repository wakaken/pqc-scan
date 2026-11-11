use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use rayon::prelude::*;

use pqc_scan_rules::{RuleSet, Severity};

use crate::model::{
    CbomEntry, DependencySbomEntry, Detection, Finding, ScanConfig, ScanResult, ScanSummary,
    ScannableFile, SourceSnippet, SourceSnippetLine,
};
use crate::{
    recommendation, risk, walker, CoverageSummary, Detector, DetectorStatus, DiagnosticCode,
    ScanDiagnostic, ScanDocument,
};

pub fn scan_repository(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
) -> Result<ScanResult> {
    Ok(scan_repository_detailed(config, rules, detectors)?.into_legacy())
}

pub fn scan_repository_detailed(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
) -> Result<ScanDocument> {
    scan_with_reader(config, rules, detectors, &|path| {
        crate::safe_read::read(&config.root, path, config.max_file_size_bytes)
    })
}

/// Capture native and legacy views during one detector execution, without changing defaults.
pub fn scan_repository_native(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
    options: &crate::NativeOptions,
) -> Result<ScanDocument> {
    scan_observed_with_reader(
        config,
        rules,
        detectors,
        &|path| crate::safe_read::read(&config.root, path, config.max_file_size_bytes),
        Some(options),
    )
}

fn scan_with_reader<F>(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
    read: &F,
) -> Result<ScanDocument>
where
    F: Fn(&std::path::Path) -> std::io::Result<Vec<u8>> + Sync,
{
    scan_observed_with_reader(config, rules, detectors, read, None)
}

fn scan_observed_with_reader<F>(
    config: &ScanConfig,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
    read: &F,
    native: Option<&crate::NativeOptions>,
) -> Result<ScanDocument>
where
    F: Fn(&std::path::Path) -> std::io::Result<Vec<u8>> + Sync,
{
    let walk = walker::walk_repository_detailed(&config.root, config.max_file_size_bytes)?;
    let files = walk.files;
    let mut diagnostics = walk.diagnostics;
    let mut coverage = CoverageSummary::new();
    coverage.selected_files = files.len() as u64;
    coverage.excluded_enumerated_files = walk.excluded;
    coverage.enumeration_failed_entries = walk.failed_entries;

    let thread_count = if config.threads == 0 {
        num_cpus::get()
    } else {
        config.threads
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()?;

    let outcomes: Vec<FileOutcome> = pool.install(|| {
        files
            .par_iter()
            .map(|path| {
                let relative = walker::relative_path(&config.root, path);
                let bytes = match read(path) {
                    Ok(v) => v,
                    Err(_) => {
                        return FileOutcome {
                            detections: Vec::new(),
                            observations: Vec::new(),
                            diagnostics: vec![ScanDiagnostic::new(
                                DiagnosticCode::ReadFailed,
                                Some(relative),
                                None,
                            )],
                            disposition: FileDisposition::ReadFailed,
                            privacy: None,
                        }
                    }
                };

                let file = ScannableFile::from_bytes(path.clone(), bytes);
                inspect_file(&file, &relative, rules, detectors, native.is_some())
            })
            .collect()
    });

    let mut detections = Vec::new();
    let mut observations = Vec::new();
    let mut privacy_by_file = HashMap::new();
    for outcome in outcomes {
        if let Some((path, privacy)) = outcome.privacy {
            privacy_by_file.insert(path, privacy);
        }
        use FileDisposition as D;
        if outcome.disposition != D::ReadFailed {
            coverage.read_ok_files += 1;
        }
        match outcome.disposition {
            D::ReadFailed => coverage.read_failed_files += 1,
            D::Analyzed => coverage.analyzed_files += 1,
            D::Partial => coverage.partial_files += 1,
            D::Failed => coverage.analysis_failed_files += 1,
            D::Unsupported => coverage.unsupported_files += 1,
            D::NotApplicable => coverage.not_applicable_files += 1,
            D::Unknown => coverage.unknown_files += 1,
        }
        observations.extend(outcome.observations);
        detections.extend(outcome.detections);
        diagnostics.extend(outcome.diagnostics);
    }

    let dependency_sbom = build_dependency_sbom(&config.root, &detections);
    let runtime_profile = recommendation::RuntimeProfile::from_repository_limited(
        &config.root,
        &files,
        &dependency_sbom,
        config.max_file_size_bytes,
    );
    let mut findings = normalize_findings(&config.root, detections, rules);
    recommendation::annotate_findings(&mut findings, &runtime_profile);
    attach_source_snippets(
        &config.root,
        config.max_file_size_bytes,
        &mut findings,
        &privacy_by_file,
    );
    findings.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then(a.location.file.cmp(&b.location.file))
    });

    let cbom = build_cbom(&findings);

    let mut summary = ScanSummary {
        total_findings: findings.len(),
        by_severity: BTreeMap::new(),
        by_risk: BTreeMap::new(),
        scanned_files: walk.stats.scanned,
        skipped_files: walk.stats.skipped,
    };

    for finding in &findings {
        *summary
            .by_severity
            .entry(finding.severity.as_str().to_string())
            .or_insert(0) += 1;
        *summary
            .by_risk
            .entry(finding.risk.as_str().to_string())
            .or_insert(0) += 1;
    }

    let sensitive_ids = findings
        .iter()
        .filter(|f| {
            privacy_by_file.get(&f.location.file).is_some_and(|p| {
                p.sensitive_match(
                    f.location.line,
                    f.location.column,
                    f.evidence.metadata.get("detector").is_none_or(|d| {
                        !matches!(
                            d.as_str(),
                            "regex_detector"
                                | "key_detector"
                                | "certificate_detector"
                                | "dependency_detector"
                        )
                    }),
                    &f.evidence.r#match,
                )
            })
        })
        .map(|f| f.finding_id.clone())
        .collect();
    // A detector may truncate a surrounding preview before we see it. Once
    // syntax is lost, protect the entire preview for files with known context.
    for f in &mut findings {
        if privacy_by_file
            .get(&f.location.file)
            .is_some_and(|p| p.protects_preview())
        {
            f.evidence.snippet_preview = "[masked-sensitive-content]".into();
        }
    }
    let legacy = ScanResult {
        generated_at: Utc::now(),
        findings,
        cbom,
        dependency_sbom,
        summary,
    };
    let mut document = ScanDocument::new(legacy, diagnostics, coverage, config);
    if let Some(options) = native {
        let native = crate::inventory::NativeScan::build(
            &config.root,
            rules,
            &document.legacy().findings,
            observations,
            options,
            serde_json::to_value(document.diagnostics_report())?,
            &document.legacy().generated_at.to_rfc3339(),
        );
        document = document.with_native(native);
    }
    crate::privacy::sanitize_with_context(document.legacy_mut(), &sensitive_ids);
    Ok(document)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileDisposition {
    ReadFailed,
    Analyzed,
    Partial,
    Failed,
    Unsupported,
    NotApplicable,
    Unknown,
}

struct FileOutcome {
    detections: Vec<Detection>,
    observations: Vec<crate::ObservationDraft>,
    diagnostics: Vec<ScanDiagnostic>,
    disposition: FileDisposition,
    privacy: Option<(String, crate::privacy::SourcePrivacy)>,
}

fn inspect_file(
    file: &ScannableFile,
    relative: &str,
    rules: &RuleSet,
    detectors: &[Arc<dyn Detector>],
    native: bool,
) -> FileOutcome {
    let privacy = file
        .text
        .as_deref()
        .map(crate::privacy::SourcePrivacy::from_text)
        .filter(|p| !p.is_empty())
        .map(|p| (relative.to_string(), p));
    let mut observations = Vec::new();
    let mut detections = Vec::new();
    let mut diagnostics = Vec::new();
    let (mut complete, mut failed, mut unsupported, mut partial, mut unknown) =
        (false, false, false, false, detectors.is_empty());
    if detectors.is_empty() {
        diagnostics.push(ScanDiagnostic::new(DiagnosticCode::NoDetectors, None, None));
    }
    for detector in detectors {
        let code = match if native {
            detector.detect_observed(file, rules)
        } else {
            detector.detect_detailed(file, rules)
        } {
            Err(_) => {
                failed = true;
                Some(DiagnosticCode::DetectorFailed)
            }
            Ok(outcome) => {
                let code = match outcome.status() {
                    DetectorStatus::Complete => {
                        complete = true;
                        None
                    }
                    DetectorStatus::NotApplicable => None,
                    DetectorStatus::Unsupported => {
                        unsupported = true;
                        Some(DiagnosticCode::DetectorUnsupported)
                    }
                    DetectorStatus::Partial => {
                        partial = true;
                        Some(DiagnosticCode::DetectorPartial)
                    }
                    DetectorStatus::Truncated => {
                        partial = true;
                        Some(DiagnosticCode::DetectorTruncated)
                    }
                    DetectorStatus::NotReported => {
                        unknown = true;
                        Some(DiagnosticCode::DetectorNotReported)
                    }
                };
                observations.extend(
                    outcome
                        .observations()
                        .iter()
                        .cloned()
                        .map(|draft| draft.bind_to_inspected_file(&file.path, relative)),
                );
                detections.extend(outcome.into_detections());
                code
            }
        };
        if let Some(code) = code {
            // Legacy coverage notices are detector-wide, avoiding one duplicate per file.
            let target =
                (code != DiagnosticCode::DetectorNotReported).then(|| relative.to_string());
            diagnostics.push(ScanDiagnostic::new(
                code,
                target,
                Some(detector.name().into()),
            ));
        }
    }
    let usable = complete || !detections.is_empty();
    let disposition = if partial || ((failed || unsupported) && usable) {
        FileDisposition::Partial
    } else if failed {
        FileDisposition::Failed
    } else if unsupported {
        FileDisposition::Unsupported
    } else if unknown {
        FileDisposition::Unknown
    } else if complete {
        FileDisposition::Analyzed
    } else {
        FileDisposition::NotApplicable
    };
    FileOutcome {
        observations,
        detections,
        diagnostics,
        disposition,
        privacy,
    }
}

fn normalize_findings(
    root: &std::path::Path,
    detections: Vec<Detection>,
    rules: &RuleSet,
) -> Vec<Finding> {
    let mut dedup = HashSet::new();
    let mut findings = Vec::new();

    for detection in detections {
        let rule = match rules.get(&detection.rule_id) {
            Some(v) => v,
            None => continue,
        };

        let dedup_key = format!(
            "{}:{}:{}:{}:{}",
            rule.id,
            detection.location.file,
            detection.location.line,
            detection.location.column,
            detection.evidence.r#match
        );
        if !dedup.insert(dedup_key) {
            continue;
        }

        let severity = risk::adjusted_severity(rule, &detection);
        let finding_id = make_finding_id(
            &rule.id,
            &detection.location.file,
            detection.location.line,
            detection.location.column,
            &detection.evidence.r#match,
        );

        let description = rule
            .description
            .clone()
            .unwrap_or_else(|| format!("Rule {} matched a quantum-relevant crypto usage", rule.id));

        findings.push(Finding {
            finding_id,
            rule_id: rule.id.clone(),
            category: rule.category.clone(),
            risk: rule.risk,
            severity,
            confidence: rule.confidence,
            description,
            migration_hint: rule.migration_hint.clone(),
            location: crate::model::Location {
                file: relativize_path(root, &detection.location.file),
                line: detection.location.line,
                column: detection.location.column,
            },
            evidence: detection.evidence,
            recommended_actions: Vec::new(),
            source_snippet: None,
        });
    }

    findings
}

fn make_finding_id(
    rule_id: &str,
    file: &str,
    line: usize,
    column: usize,
    evidence: &str,
) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(rule_id.as_bytes());
    hasher.update(file.as_bytes());
    hasher.update(line.to_string().as_bytes());
    hasher.update(column.to_string().as_bytes());
    hasher.update(evidence.as_bytes());
    let digest = hasher.finalize();
    hex::encode(&digest[..12])
}

fn relativize_path(root: &std::path::Path, path: &str) -> String {
    let path_obj = std::path::Path::new(path);
    path_obj
        .strip_prefix(root)
        .unwrap_or(path_obj)
        .to_string_lossy()
        .into_owned()
}

fn build_cbom(findings: &[Finding]) -> Vec<CbomEntry> {
    let mut seen = HashSet::new();
    let mut cbom = Vec::new();

    for finding in findings {
        let location = format!("{}:{}", finding.location.file, finding.location.line);
        let key = format!(
            "{}:{}:{}",
            finding.rule_id, location, finding.evidence.r#match
        );
        if !seen.insert(key) {
            continue;
        }

        cbom.push(CbomEntry {
            component: finding
                .location
                .file
                .split('/')
                .next()
                .unwrap_or("repo")
                .to_string(),
            algorithm: finding.evidence.r#match.clone(),
            usage_type: finding.category.clone(),
            location,
            quantum_risk: finding.risk,
            migration_hint: finding.migration_hint.clone(),
        });
    }

    cbom
}

fn build_dependency_sbom(
    root: &std::path::Path,
    detections: &[Detection],
) -> Vec<DependencySbomEntry> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for detection in detections {
        let meta = &detection.evidence.metadata;
        let marker = meta
            .get("inventory_type")
            .map(|x| x.as_str())
            .unwrap_or_default();
        if marker != "dependency" {
            continue;
        }

        let name = meta.get("dep_name").cloned().unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        let version = meta
            .get("dep_version")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let ecosystem = meta
            .get("dep_ecosystem")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let source_type = meta
            .get("dep_source_type")
            .cloned()
            .unwrap_or_else(|| "manifest".to_string());
        let purl = meta.get("dep_purl").cloned().unwrap_or_default();
        let source_file = relativize_path(root, &detection.location.file);

        let dedup_key = format!("{}:{}:{}:{}", ecosystem, name, version, source_file);
        if !seen.insert(dedup_key) {
            continue;
        }

        out.push(DependencySbomEntry {
            name,
            version,
            ecosystem,
            source_file,
            source_type,
            purl,
        });
    }

    out.sort_by(|a, b| {
        a.ecosystem
            .cmp(&b.ecosystem)
            .then(a.name.cmp(&b.name))
            .then(a.version.cmp(&b.version))
            .then(a.source_file.cmp(&b.source_file))
    });
    out
}

pub fn exceeds_fail_threshold(findings: &[Finding], threshold: Severity) -> bool {
    findings.iter().any(|f| f.severity >= threshold)
}

fn attach_source_snippets(
    root: &std::path::Path,
    max_bytes: usize,
    findings: &mut [Finding],
    privacy: &HashMap<String, crate::privacy::SourcePrivacy>,
) {
    let mut text_cache: HashMap<String, CachedSnippetSource> = HashMap::new();

    for finding in findings {
        if finding.evidence.r#type == "private_key" {
            finding.source_snippet = Some(SourceSnippet {
                lines: vec![SourceSnippetLine {
                    line: finding.location.line,
                    text: "[masked-private-key-material]".to_string(),
                    highlighted: true,
                }],
            });
            continue;
        }

        let rel_file = finding.location.file.clone();
        let source = text_cache.entry(rel_file.clone()).or_insert_with(|| {
            if privacy.get(&rel_file).is_some_and(|p| p.private_marker) {
                CachedSnippetSource::MaskedSensitiveFile
            } else {
                load_snippet_source(root, &root.join(&rel_file), max_bytes)
            }
        });

        finding.source_snippet = match source {
            CachedSnippetSource::MaskedSensitiveFile => {
                finding.evidence.snippet_preview = "[masked-sensitive-file-content]".to_string();
                Some(SourceSnippet {
                    lines: vec![SourceSnippetLine {
                        line: finding.location.line,
                        text: "[masked-sensitive-file-content]".to_string(),
                        highlighted: true,
                    }],
                })
            }
            CachedSnippetSource::TextLines(all_lines) => {
                build_source_snippet(all_lines, finding.location.line, 2, 2)
            }
            CachedSnippetSource::Unavailable => None,
        };
    }
}

enum CachedSnippetSource {
    MaskedSensitiveFile,
    TextLines(Vec<String>),
    Unavailable,
}

fn load_snippet_source(
    root: &std::path::Path,
    path: &std::path::Path,
    max_bytes: usize,
) -> CachedSnippetSource {
    if path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("p12") || e.eq_ignore_ascii_case("pfx"))
    {
        return CachedSnippetSource::Unavailable;
    }
    let bytes = match crate::safe_read::read(root, path, max_bytes) {
        Ok(v) => v,
        Err(_) => return CachedSnippetSource::Unavailable,
    };
    let sample_len = usize::min(bytes.len(), 1024);
    if bytes[..sample_len].contains(&0) {
        return CachedSnippetSource::Unavailable;
    }

    let raw = String::from_utf8_lossy(&bytes).into_owned();
    // Locator credentials can be escaped or live outside parsed dependency records.
    // Never copy source context from these containers into any report.
    if contains_private_key_marker(&raw)
        || crate::classifier::is_dependency_manifest(path)
        || crate::classifier::is_sbom_file(path)
    {
        return CachedSnippetSource::MaskedSensitiveFile;
    }

    let sanitized = mask_private_key_blocks(&raw);
    CachedSnippetSource::TextLines(sanitized.lines().map(sanitize_source_line).collect())
}

fn build_source_snippet(
    lines: &[String],
    target_line: usize,
    before: usize,
    after: usize,
) -> Option<SourceSnippet> {
    if lines.is_empty() || target_line == 0 {
        return None;
    }

    let target_idx = usize::min(target_line - 1, lines.len() - 1);
    let start = target_idx.saturating_sub(before);
    let end = usize::min(target_idx.saturating_add(after), lines.len() - 1);

    let mut snippet_lines = Vec::new();
    for (idx, text) in lines.iter().enumerate().take(end + 1).skip(start) {
        snippet_lines.push(SourceSnippetLine {
            line: idx + 1,
            text: sanitize_source_line(text),
            highlighted: idx == target_idx,
        });
    }

    if snippet_lines.is_empty() {
        None
    } else {
        Some(SourceSnippet {
            lines: snippet_lines,
        })
    }
}

fn sanitize_source_line(line: &str) -> String {
    if looks_like_private_key_line(line) {
        return "[masked-private-key-material]".to_string();
    }
    if looks_like_base64_secret(line) {
        return "[masked-sensitive-content]".to_string();
    }

    let mut out = crate::privacy::redact_text(line).replace('\r', "");
    if out.chars().count() > 240 {
        out = out.chars().take(240).collect::<String>();
        out.push_str("...");
    }
    out
}

fn looks_like_private_key_line(line: &str) -> bool {
    let upper = line.to_ascii_uppercase();
    upper.contains("BEGIN RSA PRIVATE KEY")
        || upper.contains("BEGIN PRIVATE KEY")
        || upper.contains("BEGIN OPENSSH PRIVATE KEY")
        || upper.contains("END RSA PRIVATE KEY")
        || upper.contains("END PRIVATE KEY")
        || upper.contains("END OPENSSH PRIVATE KEY")
}

fn contains_private_key_marker(text: &str) -> bool {
    let upper = text.to_ascii_uppercase();
    upper.contains("BEGIN RSA PRIVATE KEY")
        || upper.contains("BEGIN PRIVATE KEY")
        || upper.contains("BEGIN OPENSSH PRIVATE KEY")
        || upper.contains("BEGIN EC PRIVATE KEY")
        || upper.contains("BEGIN DSA PRIVATE KEY")
        || upper.contains("BEGIN ENCRYPTED PRIVATE KEY")
}

fn mask_private_key_blocks(text: &str) -> String {
    let mut out = String::new();
    let mut in_private_key_block = false;

    for line in text.lines() {
        let upper = line.to_ascii_uppercase();
        let is_begin = upper.contains("BEGIN RSA PRIVATE KEY")
            || upper.contains("BEGIN PRIVATE KEY")
            || upper.contains("BEGIN OPENSSH PRIVATE KEY");
        let is_end = upper.contains("END RSA PRIVATE KEY")
            || upper.contains("END PRIVATE KEY")
            || upper.contains("END OPENSSH PRIVATE KEY");

        if is_begin {
            in_private_key_block = true;
        }

        if in_private_key_block {
            out.push_str("[masked-private-key-material]\n");
        } else {
            out.push_str(line);
            out.push('\n');
        }

        if is_end {
            in_private_key_block = false;
        }
    }

    out
}

fn looks_like_base64_secret(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 64 || trimmed.chars().any(char::is_whitespace) {
        return false;
    }

    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=' | '-' | '_'))
    {
        return false;
    }

    trimmed.starts_with("MII") || trimmed.ends_with('=')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        dir.push(format!("pqc-scan-{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    struct CompleteDetector;

    impl Detector for CompleteDetector {
        fn name(&self) -> &'static str {
            "complete-test-detector"
        }

        fn detect(&self, file: &ScannableFile, _: &RuleSet) -> Result<Vec<Detection>> {
            if !file.text.as_deref().unwrap_or("").contains("RS256") {
                return Ok(Vec::new());
            }
            Ok(vec![Detection {
                rule_id: "JWT_RS256".into(),
                location: crate::Location {
                    file: file.path.to_string_lossy().into_owned(),
                    line: 1,
                    column: 1,
                },
                evidence: crate::Evidence {
                    r#type: "test".into(),
                    r#match: "RS256".into(),
                    snippet_preview: "RS256".into(),
                    metadata: BTreeMap::new(),
                },
            }])
        }

        fn detect_detailed(
            &self,
            file: &ScannableFile,
            rules: &RuleSet,
        ) -> Result<crate::DetectorOutcome> {
            Ok(crate::DetectorOutcome::complete(self.detect(file, rules)?))
        }
    }

    struct LegacyEmptyDetector;

    impl Detector for LegacyEmptyDetector {
        fn name(&self) -> &'static str {
            "legacy-empty"
        }

        fn detect(&self, _: &ScannableFile, _: &RuleSet) -> Result<Vec<Detection>> {
            Ok(Vec::new())
        }
    }

    struct FailingDetector;

    impl Detector for FailingDetector {
        fn name(&self) -> &'static str {
            "failing-test-detector"
        }

        fn detect(&self, _: &ScannableFile, _: &RuleSet) -> Result<Vec<Detection>> {
            anyhow::bail!("SYNTHETIC_ERROR_CONTENT_MUST_NOT_BE_EXPORTED")
        }
    }

    fn test_rules() -> RuleSet {
        RuleSet::load_from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rules"))
            .expect("checked-in rules")
    }

    fn test_config(root: PathBuf) -> ScanConfig {
        ScanConfig {
            root,
            threads: 1,
            ..ScanConfig::default()
        }
    }

    fn assert_coverage_partitions(c: &crate::CoverageSummary) {
        assert_eq!(
            c.selected_files(),
            c.read_ok_files() + c.read_failed_files()
        );
        assert_eq!(
            c.read_ok_files(),
            c.analyzed_files()
                + c.partial_files()
                + c.analysis_failed_files()
                + c.unsupported_files()
                + c.not_applicable_files()
                + c.unknown_files()
        );
    }

    #[test]
    fn detailed_scan_distinguishes_empty_success_from_legacy_unknown() {
        let root = temp_dir("inspection-empty");
        fs::write(root.join("empty.txt"), "no cryptography").unwrap();
        let config = test_config(root.clone());
        let rules = test_rules();
        let complete =
            scan_repository_detailed(&config, &rules, &[Arc::new(CompleteDetector)]).unwrap();
        assert!(complete.legacy().findings.is_empty());
        assert!(complete.coverage().is_complete());
        assert_eq!(complete.coverage().analyzed_files(), 1);
        assert_coverage_partitions(complete.coverage());

        let unknown =
            scan_repository_detailed(&config, &rules, &[Arc::new(LegacyEmptyDetector)]).unwrap();
        assert!(unknown.legacy().findings.is_empty());
        assert_eq!(
            unknown.coverage().status(),
            crate::InspectionStatus::Unknown
        );
        assert_eq!(unknown.coverage().unknown_files(), 1);
        assert!(unknown
            .diagnostics()
            .iter()
            .any(|d| { d.code() == crate::DiagnosticCode::DetectorNotReported }));
        assert_coverage_partitions(unknown.coverage());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn read_failure_retains_other_files_and_legacy_selected_count() {
        let root = temp_dir("inspection-read");
        fs::write(root.join("hit.js"), "RS256").unwrap();
        fs::write(root.join("empty.txt"), "nothing").unwrap();
        fs::write(root.join("unreadable.conf"), "unread").unwrap();
        let document = scan_with_reader(
            &test_config(root.clone()),
            &test_rules(),
            &[Arc::new(CompleteDetector)],
            &|path| {
                if path.ends_with("unreadable.conf") {
                    Err(std::io::Error::other("SYNTHETIC_READ_ERROR_CONTENT"))
                } else {
                    fs::read(path)
                }
            },
        )
        .unwrap();
        assert_eq!(document.legacy().findings.len(), 1);
        assert_eq!(document.legacy().summary.scanned_files, 3);
        assert_eq!(document.coverage().selected_files(), 3);
        assert_eq!(document.coverage().read_failed_files(), 1);
        assert_eq!(document.coverage().analyzed_files(), 2);
        assert_eq!(
            document.coverage().status(),
            crate::InspectionStatus::Partial
        );
        assert_coverage_partitions(document.coverage());
        let json = serde_json::to_value(document.diagnostics_report()).unwrap();
        assert!(json["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| { d["code"] == "read_failed" && d["file"] == "unreadable.conf" }));
        assert!(!json.to_string().contains("SYNTHETIC_READ_ERROR_CONTENT"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn detector_error_preserves_other_hits_and_does_not_export_error_content() {
        let root = temp_dir("inspection-detector");
        fs::write(root.join("hit.js"), "RS256").unwrap();
        let document = scan_repository_detailed(
            &test_config(root.clone()),
            &test_rules(),
            &[Arc::new(FailingDetector), Arc::new(CompleteDetector)],
        )
        .unwrap();
        assert_eq!(document.legacy().findings.len(), 1);
        assert_eq!(document.coverage().partial_files(), 1);
        assert_eq!(document.coverage().analysis_failed_files(), 0);
        assert_eq!(
            document.coverage().status(),
            crate::InspectionStatus::Partial
        );
        assert_coverage_partitions(document.coverage());
        let json = serde_json::to_value(document.diagnostics_report()).unwrap();
        assert_eq!(json["schema_version"], "1");
        assert!(json["diagnostics"].as_array().unwrap().iter().any(|d| {
            d["code"] == "detector_failed" && d["detector"] == "failing-test-detector"
        }));
        assert!(!json.to_string().contains("SYNTHETIC_ERROR_CONTENT"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn all_detector_errors_are_failed_not_empty_success() {
        let root = temp_dir("inspection-failed");
        fs::write(root.join("input.txt"), "anything").unwrap();
        let document = scan_repository_detailed(
            &test_config(root.clone()),
            &test_rules(),
            &[Arc::new(FailingDetector)],
        )
        .unwrap();
        assert!(document.legacy().findings.is_empty());
        assert_eq!(document.coverage().analysis_failed_files(), 1);
        assert_eq!(
            document.coverage().status(),
            crate::InspectionStatus::Failed
        );
        assert_coverage_partitions(document.coverage());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn exclusions_describe_declared_scope_and_do_not_become_successful_inspections() {
        let root = temp_dir("inspection-scope");
        fs::write(root.join("README.md"), "ignored by policy").unwrap();
        fs::write(root.join("big.txt"), "0123456789").unwrap();
        fs::write(root.join("ok.txt"), "ok").unwrap();
        let mut config = test_config(root.clone());
        config.max_file_size_bytes = 4;
        let document =
            scan_repository_detailed(&config, &test_rules(), &[Arc::new(CompleteDetector)])
                .unwrap();
        assert_eq!(document.coverage().selected_files(), 1);
        assert_eq!(document.coverage().excluded_enumerated_files(), 2);
        assert_eq!(document.legacy().summary.skipped_files, 2);
        assert!(document.coverage().is_complete());
        assert!(document.coverage().not_enumerated_population());
        let json = serde_json::to_value(document.diagnostics_report()).unwrap();
        assert_eq!(json["scope"]["max_file_size_bytes"], 4);
        assert_eq!(json["scope"]["git_ignore"], true);
        assert_eq!(json["diagnostics"].as_array().unwrap().len(), 2);
        assert_coverage_partitions(document.coverage());
        fs::remove_dir_all(root).unwrap();
    }

    struct StatusDetector(DetectorStatus, bool);

    impl Detector for StatusDetector {
        fn name(&self) -> &'static str {
            "status-test-detector"
        }

        fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
            if self.1 {
                CompleteDetector.detect(file, rules)
            } else {
                Ok(Vec::new())
            }
        }

        fn detect_detailed(
            &self,
            file: &ScannableFile,
            rules: &RuleSet,
        ) -> Result<crate::DetectorOutcome> {
            Ok(crate::DetectorOutcome::new(
                self.detect(file, rules)?,
                self.0,
            ))
        }
    }

    #[test]
    fn detailed_statuses_partition_files_without_claiming_false_completeness() {
        use crate::InspectionStatus as I;
        use DetectorStatus as S;
        let root = temp_dir("inspection-status");
        fs::write(root.join("hit.js"), "RS256").unwrap();
        for (status, emit_hit, count_key, overall) in [
            (S::Complete, false, "analyzed_files", I::CompleteWithinScope),
            (
                S::NotApplicable,
                false,
                "not_applicable_files",
                I::CompleteWithinScope,
            ),
            (S::Unsupported, false, "unsupported_files", I::Partial),
            (S::Partial, false, "partial_files", I::Partial),
            (S::Truncated, false, "partial_files", I::Partial),
            (S::NotReported, false, "unknown_files", I::Unknown),
            // A hit contradicts "not applicable"; preserve it without declaring completion.
            (S::NotApplicable, true, "partial_files", I::Partial),
        ] {
            let document = scan_repository_detailed(
                &test_config(root.clone()),
                &test_rules(),
                &[Arc::new(StatusDetector(status, emit_hit))],
            )
            .unwrap();
            assert_eq!(
                document.coverage().status(),
                overall,
                "{status:?}, {emit_hit}"
            );
            let c = serde_json::to_value(document.coverage()).unwrap();
            assert_eq!(c[count_key], 1, "{status:?}, {emit_hit}");
            assert_eq!(document.legacy().findings.len(), usize::from(emit_hit));
            assert_coverage_partitions(document.coverage());
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn selected_files_without_detectors_are_unknown_but_empty_scope_can_complete() {
        let root = temp_dir("inspection-no-detectors");
        let config = test_config(root.clone());
        let empty = scan_repository_detailed(&config, &test_rules(), &[]).unwrap();
        assert!(empty.coverage().is_complete());
        fs::write(root.join("input.txt"), "nothing").unwrap();
        let selected = scan_repository_detailed(&config, &test_rules(), &[]).unwrap();
        assert_eq!(selected.coverage().unknown_files(), 1);
        assert!(!selected.coverage().is_complete());
        assert_coverage_partitions(selected.coverage());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn legacy_adapter_runs_once_and_wrapper_keeps_the_same_result() {
        struct CountingLegacy(std::sync::atomic::AtomicUsize);
        impl Detector for CountingLegacy {
            fn name(&self) -> &'static str {
                "counting-legacy"
            }
            fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
                self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                CompleteDetector.detect(file, rules)
            }
        }
        let root = temp_dir("inspection-wrapper");
        fs::write(root.join("hit.js"), "RS256").unwrap();
        let detector = Arc::new(CountingLegacy(std::sync::atomic::AtomicUsize::new(0)));
        let detectors: Vec<Arc<dyn Detector>> = vec![detector.clone()];
        let config = test_config(root.clone());
        let rules = test_rules();
        let legacy = scan_repository(&config, &rules, &detectors).unwrap();
        assert_eq!(detector.0.load(std::sync::atomic::Ordering::SeqCst), 1);
        let detailed = scan_repository_detailed(&config, &rules, &detectors).unwrap();
        assert_eq!(detector.0.load(std::sync::atomic::Ordering::SeqCst), 2);
        let mut before = serde_json::to_value(legacy).unwrap();
        let mut after = serde_json::to_value(detailed.legacy()).unwrap();
        before.as_object_mut().unwrap().remove("generated_at");
        after.as_object_mut().unwrap().remove("generated_at");
        assert_eq!(before, after);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn masks_entire_private_key_block() {
        let text = "before\n-----BEGIN PRIVATE KEY-----\nSUPERSECRETBASE64PAYLOAD\n-----END PRIVATE KEY-----\nafter\n";
        let masked = mask_private_key_blocks(text);

        assert!(!masked.contains("SUPERSECRETBASE64PAYLOAD"));
        assert!(masked.contains("[masked-private-key-material]"));
        assert!(masked.contains("before"));
        assert!(masked.contains("after"));
    }

    #[test]
    fn marks_sensitive_file_for_snippet_masking() {
        let root = temp_dir("pipeline-mask");
        let file = root.join("leak.conf");
        fs::write(
            &file,
            "ssl_protocols TLSv1;\n-----BEGIN PRIVATE KEY-----\nMIIEV...SECRET...\n-----END PRIVATE KEY-----\n",
        )
        .expect("write file");

        match load_snippet_source(&root, &file, 2 * 1024 * 1024) {
            CachedSnippetSource::MaskedSensitiveFile => {}
            other => panic!(
                "expected masked sensitive file, got {:?}",
                type_name(&other)
            ),
        }
    }

    fn type_name(source: &CachedSnippetSource) -> &'static str {
        match source {
            CachedSnippetSource::MaskedSensitiveFile => "MaskedSensitiveFile",
            CachedSnippetSource::TextLines(_) => "TextLines",
            CachedSnippetSource::Unavailable => "Unavailable",
        }
    }
}

#[cfg(test)]
mod opaque_container_snippet_tests {
    use super::*;
    use std::fs;
    #[test]
    fn pkcs12_payloads_are_never_reread_as_source_snippets() {
        let root = std::env::temp_dir().join(format!("pqc-opaque-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        for ext in ["p12", "pfx", "P12", "PFX"] {
            let path = root.join(format!("container.{ext}"));
            fs::write(&path, b"opaque synthetic container payload without NUL").unwrap();
            assert!(matches!(
                load_snippet_source(&root, &path, 2 * 1024 * 1024),
                CachedSnippetSource::Unavailable
            ));
        }
        fs::remove_dir_all(root).unwrap();
    }
}
