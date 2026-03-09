use aho_corasick::AhoCorasick;
use anyhow::Result;

use pqc_scan_core::model::{line_col_for_offset, mask_preview};
use pqc_scan_core::{Detection, Detector, Evidence, Location, ScannableFile};
use pqc_scan_rules::{RuleKind, RuleSet};

#[derive(Debug)]
pub struct KeyDetector {
    marker_matcher: AhoCorasick,
}

impl Default for KeyDetector {
    fn default() -> Self {
        let marker_matcher = AhoCorasick::new([
            "BEGIN RSA PRIVATE KEY",
            "BEGIN PRIVATE KEY",
            "BEGIN OPENSSH PRIVATE KEY",
        ])
        .expect("failed to compile key marker automaton");
        Self { marker_matcher }
    }
}

impl Detector for KeyDetector {
    fn name(&self) -> &'static str {
        "key_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        let text = match file.text.as_ref() {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        if !self.marker_matcher.is_match(text) {
            return Ok(Vec::new());
        }

        let mut out = Vec::new();

        for rule in rules.by_kind(RuleKind::Key) {
            let regex = match rule.compiled_pattern() {
                Some(v) => v,
                None => continue,
            };

            for found in regex.find_iter(text).take(8) {
                let (line, column) = line_col_for_offset(text, found.start());
                out.push(Detection {
                    rule_id: rule.id.clone(),
                    location: Location {
                        file: file.path.to_string_lossy().into_owned(),
                        line,
                        column,
                    },
                    evidence: Evidence {
                        r#type: "private_key".to_string(),
                        r#match: mask_preview(found.as_str()),
                        snippet_preview: "[masked-private-key-material]".to_string(),
                        metadata: std::collections::BTreeMap::from([
                            ("detector".to_string(), self.name().to_string()),
                            (
                                "file".to_string(),
                                file.file_name().unwrap_or("unknown").to_string(),
                            ),
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
    use std::path::PathBuf;

    fn rules_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../rules/default")
            .canonicalize()
            .expect("rules dir")
    }

    #[test]
    fn detects_private_key_header_and_masks_evidence() {
        let detector = KeyDetector::default();
        let rules = RuleSet::load_from_dir(&rules_dir()).expect("load rules");
        let file = ScannableFile::from_bytes(
            PathBuf::from("fixtures/id_rsa"),
            b"before\n-----BEGIN RSA PRIVATE KEY-----\nABCDEF\n-----END RSA PRIVATE KEY-----\n"
                .to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");

        assert_eq!(detections.len(), 1);
        let detection = &detections[0];
        assert_eq!(detection.rule_id, "PRIVATE_KEY_RSA_HEADER");
        assert_eq!(detection.location.line, 2);
        assert_eq!(detection.location.column, 1);
        assert_eq!(detection.evidence.r#type, "private_key");
        assert_eq!(detection.evidence.r#match, "[masked-private-key]");
        assert_eq!(
            detection.evidence.snippet_preview,
            "[masked-private-key-material]"
        );
        assert_eq!(
            detection.evidence.metadata.get("detector"),
            Some(&"key_detector".to_string())
        );
    }

    #[test]
    fn ignores_text_without_private_key_markers() {
        let detector = KeyDetector::default();
        let rules = RuleSet::load_from_dir(&rules_dir()).expect("load rules");
        let file = ScannableFile::from_bytes(
            PathBuf::from("fixtures/config.txt"),
            b"ssh-rsa is present but no private key header is here\n".to_vec(),
        );

        let detections = detector.detect(&file, &rules).expect("detect");

        assert!(detections.is_empty());
    }
}
