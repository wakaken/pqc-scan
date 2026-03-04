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
