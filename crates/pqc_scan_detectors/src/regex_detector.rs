mod go_imports;
use anyhow::Result;

use crate::comment_filter::should_skip_comment_only_match;
use pqc_scan_core::model::{line_col_for_offset, mask_preview, snippet_around};
use pqc_scan_core::{
    Detection, Detector, DetectorOutcome, DetectorStatus, Evidence, Location, ScannableFile,
};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug, Default)]
pub struct RegexDetector;

impl Detector for RegexDetector {
    fn name(&self) -> &'static str {
        "regex_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(file, rules)?.into_detections())
    }

    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.detect_internal(file, rules, false, crate::DetectorBudget::default())
    }
    fn detect_observed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        self.detect_internal(file, rules, true, crate::DetectorBudget::default())
    }
}
impl RegexDetector {
    fn detect_internal(
        &self,
        file: &ScannableFile,
        rules: &RuleSet,
        observe: bool,
        budget: crate::DetectorBudget,
    ) -> Result<DetectorOutcome> {
        let text = match file.text.as_ref() {
            Some(v) => v,
            None => {
                return Ok(DetectorOutcome::new(
                    Vec::new(),
                    DetectorStatus::NotApplicable,
                ))
            }
        };

        let mut observations = Vec::new();
        let mut out = Vec::new();
        let is_go = file.ext().is_some_and(|e| e.eq_ignore_ascii_case("go"))
            && !crate::source_kind::is_dockerfile(&file.path, text);
        let imports = if is_go
            && rules
                .by_kind(RuleKind::Regex)
                .any(|r| matches!(r.id.as_str(), "GO_TLS_RSA_IMPORT" | "API_GO_CRYPTO_RSA"))
        {
            Some(go_imports::collect(text, budget.max_ast_nodes))
        } else {
            None
        };
        let mut status = imports
            .as_ref()
            .map_or(DetectorStatus::NotApplicable, |i| i.status);

        for rule in rules.by_kind(RuleKind::Regex) {
            // Go import indicators require a Go source file and an actual quoted
            // import path, never an OpenSSL build path in arbitrary text.
            if matches!(rule.id.as_str(), "GO_TLS_RSA_IMPORT" | "API_GO_CRYPTO_RSA") && !is_go {
                continue;
            }
            if status == DetectorStatus::NotApplicable {
                status = DetectorStatus::Complete;
            }
            let regex = match rule.compiled_pattern() {
                Some(v) => v,
                None => {
                    if status != DetectorStatus::Truncated {
                        status = DetectorStatus::Partial;
                    }
                    continue;
                }
            };

            let go_rule = matches!(rule.id.as_str(), "GO_TLS_RSA_IMPORT" | "API_GO_CRYPTO_RSA");
            let targets: Vec<_> = if go_rule {
                imports
                    .as_ref()
                    .map(|i| i.spans.iter().map(|(a, b)| (*a, &text[*a..*b])).collect())
                    .unwrap_or_default()
            } else {
                vec![(0, text.as_str())]
            };
            let mut accepted = 0;
            let mut examined = 0;
            'targets: for (base, target) in targets {
                for found in regex.find_iter(target) {
                    if examined >= budget.max_match_candidates {
                        status = DetectorStatus::Truncated;
                        break 'targets;
                    }
                    examined += 1;
                    if go_rule && (found.start() != 0 || found.end() != target.len()) {
                        continue;
                    }
                    let start = base + found.start();
                    let end = base + found.end();
                    if should_skip_comment_only_match(&file.path, text, start) {
                        continue;
                    }
                    if accepted == budget.max_hits_per_rule {
                        status = DetectorStatus::Truncated;
                        break 'targets;
                    }
                    let (line, column) = line_col_for_offset(text, start);
                    out.push(Detection {
                        rule_id: rule.id.clone(),
                        location: Location {
                            file: file.path.to_string_lossy().into_owned(),
                            line,
                            column,
                        },
                        evidence: Evidence {
                            r#type: "regex_match".into(),
                            r#match: mask_preview(found.as_str()),
                            snippet_preview: snippet_around(text, start, end, 64),
                            metadata: std::collections::BTreeMap::from([
                                ("detector".into(), self.name().into()),
                                ("scope".into(), rule.scope.clone()),
                            ]),
                        },
                    });
                    if observe {
                        observations.push(pqc_scan_core::ObservationDraft::source(
                            out.last().expect("accepted detection"),
                            if rule.scope == "config" {
                                pqc_scan_core::ObservationMethod::Configuration
                            } else {
                                pqc_scan_core::ObservationMethod::LexicalMatch
                            },
                            found.as_str(),
                            Some((start, end)),
                        ));
                    }
                    accepted += 1;
                }
            }
        }

        Ok(DetectorOutcome::new(out, status).with_observations(observations))
    }
}

pub struct BoundedRegexDetector(crate::DetectorBudget);
impl RegexDetector {
    pub fn with_budget(budget: crate::DetectorBudget) -> BoundedRegexDetector {
        BoundedRegexDetector(budget)
    }
}
impl Detector for BoundedRegexDetector {
    fn name(&self) -> &'static str {
        "regex_detector"
    }
    fn detect(&self, f: &ScannableFile, r: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(f, r)?.into_detections())
    }
    fn detect_detailed(&self, f: &ScannableFile, r: &RuleSet) -> Result<DetectorOutcome> {
        RegexDetector.detect_internal(f, r, false, self.0)
    }
    fn detect_observed(&self, f: &ScannableFile, r: &RuleSet) -> Result<DetectorOutcome> {
        RegexDetector.detect_internal(f, r, true, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_rules::RuleSet;
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

    fn regex_rules(pattern: &str) -> RuleSet {
        let dir = temp_dir("regex-rules");
        fs::write(
            dir.join("rules.yml"),
            format!(
                r#"
- id: TEST_REGEX
  kind: regex
  category: TLS
  severity: high
  risk: quantum-vulnerable
  confidence: 0.9
  migration_hint: Replace legacy crypto.
  pattern: "{pattern}"
  scope: code
"#
            ),
        )
        .expect("write rules");
        RuleSet::load_from_dir(&dir).expect("load rules")
    }

    #[test]
    fn detects_regex_match_with_location_and_metadata() {
        let detector = RegexDetector;
        let rules = regex_rules("RS256");
        let file = ScannableFile::from_bytes(
            PathBuf::from("src/app.js"),
            b"const alg = \"RS256\";\n".to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 1);
        let detection = &detections[0];
        assert_eq!(detection.rule_id, "TEST_REGEX");
        assert_eq!(detection.location.file, "src/app.js");
        assert_eq!(detection.location.line, 1);
        assert_eq!(detection.location.column, 14);
        assert_eq!(detection.evidence.r#type, "regex_match");
        assert_eq!(detection.evidence.r#match, "RS256");
        assert_eq!(
            detection
                .evidence
                .metadata
                .get("detector")
                .map(String::as_str),
            Some("regex_detector")
        );
        assert_eq!(
            detection.evidence.metadata.get("scope").map(String::as_str),
            Some("code")
        );
    }

    #[test]
    fn skips_comment_only_matches_for_javascript() {
        let detector = RegexDetector;
        let rules = regex_rules("RS256");
        let file = ScannableFile::from_bytes(
            PathBuf::from("src/app.js"),
            b"// RS256 should not count\nconst alg = \"RS256\";\n/* RS256 block */\n".to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].location.line, 2);
        assert_eq!(detections[0].location.column, 14);
        assert_eq!(detections[0].evidence.r#match, "RS256");
    }

    #[test]
    fn masks_private_key_like_matches_in_evidence_and_snippet() {
        let detector = RegexDetector;
        let rules = regex_rules("PRIVATE KEY");
        let file = ScannableFile::from_bytes(
            PathBuf::from("keys/sample.txt"),
            b"prefix -----BEGIN PRIVATE KEY----- suffix\n".to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].evidence.r#match, "[masked-private-key]");
        assert_eq!(
            detections[0].evidence.snippet_preview,
            "[masked-private-key]"
        );
    }

    #[test]
    fn explicit_small_budget_limits_matches_per_rule() {
        let detector = RegexDetector::with_budget(crate::DetectorBudget {
            max_hits_per_rule: 32,
            ..Default::default()
        });
        let rules = regex_rules("RS256");
        let text = std::iter::repeat_n("RS256\n", 40).collect::<String>();
        let file = ScannableFile::from_bytes(PathBuf::from("fixtures/plain.txt"), text.into());

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 32);
        assert_eq!(detections.first().map(|d| d.location.line), Some(1));
        assert_eq!(detections.last().map(|d| d.location.line), Some(32));
    }
}
