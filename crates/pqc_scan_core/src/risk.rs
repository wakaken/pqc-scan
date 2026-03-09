use pqc_scan_rules::{Risk, Rule, Severity};

use crate::model::Detection;

pub fn adjusted_severity(rule: &Rule, detection: &Detection) -> Severity {
    let sensitive_category = matches!(
        rule.category.to_ascii_lowercase().as_str(),
        "tls" | "pki" | "jwt" | "auth" | "authentication"
    );

    if !sensitive_category {
        return rule.severity;
    }

    if rule.risk != Risk::QuantumVulnerable {
        return rule.severity;
    }

    // 誤検知の爆発を防ぐため、高信頼のみ昇格する。
    if rule.confidence < 0.45 {
        return rule.severity;
    }

    if detection.evidence.r#type == "private_key" {
        return Severity::Critical;
    }

    bump_severity(rule.severity)
}

fn bump_severity(severity: Severity) -> Severity {
    match severity {
        Severity::Info => Severity::Low,
        Severity::Low => Severity::Medium,
        Severity::Medium => Severity::High,
        Severity::High | Severity::Critical => Severity::Critical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn sample_rule(category: &str, risk: Risk, severity: Severity, confidence: f32) -> Rule {
        let yaml = format!(
            "id: TEST_RULE\nkind: regex\ncategory: {category}\nseverity: {}\nrisk: {}\nconfidence: {confidence}\nmigration_hint: hint\npattern: TEST\nscope: code\ndescription: desc\n",
            severity.as_str(),
            risk.as_str()
        );
        serde_yaml::from_str(&yaml).expect("parse rule")
    }

    fn sample_detection(evidence_type: &str) -> Detection {
        Detection {
            rule_id: "TEST_RULE".to_string(),
            location: crate::model::Location {
                file: "src/main.rs".to_string(),
                line: 1,
                column: 1,
            },
            evidence: crate::model::Evidence {
                r#type: evidence_type.to_string(),
                r#match: "match".to_string(),
                snippet_preview: "snippet".to_string(),
                metadata: BTreeMap::new(),
            },
        }
    }

    #[test]
    fn keeps_non_sensitive_categories_at_original_severity() {
        let rule = sample_rule("cryptoapi", Risk::QuantumVulnerable, Severity::Medium, 0.9);
        let detection = sample_detection("code");
        assert_eq!(adjusted_severity(&rule, &detection), Severity::Medium);
    }

    #[test]
    fn keeps_low_confidence_sensitive_findings_unpromoted() {
        let rule = sample_rule("tls", Risk::QuantumVulnerable, Severity::Medium, 0.4);
        let detection = sample_detection("code");
        assert_eq!(adjusted_severity(&rule, &detection), Severity::Medium);
    }

    #[test]
    fn promotes_sensitive_quantum_vulnerable_findings() {
        let rule = sample_rule("pki", Risk::QuantumVulnerable, Severity::Medium, 0.8);
        let detection = sample_detection("code");
        assert_eq!(adjusted_severity(&rule, &detection), Severity::High);
    }

    #[test]
    fn escalates_private_key_evidence_to_critical() {
        let rule = sample_rule("auth", Risk::QuantumVulnerable, Severity::Low, 0.8);
        let detection = sample_detection("private_key");
        assert_eq!(adjusted_severity(&rule, &detection), Severity::Critical);
    }
}
