use anyhow::Result;

use crate::comment_filter::should_skip_comment_only_match;
use pqc_scan_core::model::{line_col_for_offset, mask_preview, snippet_around};
use pqc_scan_core::{Detection, Detector, Evidence, Location, ScannableFile};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug, Default)]
pub struct RegexDetector;

impl Detector for RegexDetector {
    fn name(&self) -> &'static str {
        "regex_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        let text = match file.text.as_ref() {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        let mut out = Vec::new();

        for rule in rules.by_kind(RuleKind::Regex) {
            let regex = match rule.compiled_pattern() {
                Some(v) => v,
                None => continue,
            };

            for found in regex.find_iter(text).take(32) {
                if should_skip_comment_only_match(&file.path, text, found.start()) {
                    continue;
                }
                let (line, column) = line_col_for_offset(text, found.start());
                out.push(Detection {
                    rule_id: rule.id.clone(),
                    location: Location {
                        file: file.path.to_string_lossy().into_owned(),
                        line,
                        column,
                    },
                    evidence: Evidence {
                        r#type: "regex_match".to_string(),
                        r#match: mask_preview(found.as_str()),
                        snippet_preview: snippet_around(text, found.start(), found.end(), 64),
                        metadata: std::collections::BTreeMap::from([
                            ("detector".to_string(), self.name().to_string()),
                            ("scope".to_string(), rule.scope.clone()),
                        ]),
                    },
                });
            }
        }

        Ok(out)
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
    fn limits_matches_per_rule_to_thirty_two() {
        let detector = RegexDetector;
        let rules = regex_rules("RS256");
        let text = std::iter::repeat_n("RS256\n", 40).collect::<String>();
        let file = ScannableFile::from_bytes(PathBuf::from("fixtures/plain.txt"), text.into());

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 32);
        assert_eq!(detections.first().map(|d| d.location.line), Some(1));
        assert_eq!(detections.last().map(|d| d.location.line), Some(32));
    }
}
