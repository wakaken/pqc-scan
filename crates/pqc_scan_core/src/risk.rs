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
