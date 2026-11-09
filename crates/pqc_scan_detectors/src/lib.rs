pub mod certificate_detector;
mod comment_filter;
pub mod dependency_detector;
pub mod key_detector;
pub mod regex_detector;
pub mod tree_sitter_detector;

use std::sync::Arc;

use pqc_scan_core::Detector;

// Accepted-hit caps are separate. One additional iterator look-ahead may establish
// remaining candidates, but is never filtered, emitted or counted as an omitted hit.
const MAX_RULE_MATCH_CANDIDATES: usize = 4096;

pub fn default_detectors() -> Vec<Arc<dyn Detector>> {
    vec![
        Arc::new(tree_sitter_detector::TreeSitterDetector),
        Arc::new(regex_detector::RegexDetector),
        Arc::new(dependency_detector::DependencyDetector),
        Arc::new(certificate_detector::CertificateDetector),
        Arc::new(key_detector::KeyDetector::default()),
    ]
}
