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
