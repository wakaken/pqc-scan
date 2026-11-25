//! Ambiguous headers have two candidate views and one shared work allocation.
use super::{scope_matches, TreeSitterDetector};
use crate::DetectorBudget;
use pqc_scan_core::{DetectorOutcome, DetectorStatus, RepositoryContext, ScannableFile};
use pqc_scan_rules::RuleSet;
pub(super) fn c_kind_known(kind: &str) -> bool {
    static KINDS: std::sync::LazyLock<std::collections::HashSet<&'static str>> =
        std::sync::LazyLock::new(|| {
            let c: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
            (0..c.node_kind_count())
                .filter_map(|id| c.node_kind_for_id(id as u16))
                .collect()
        });
    KINDS.contains(kind)
}
fn half(b: DetectorBudget, second: bool) -> DetectorBudget {
    let split = |n: usize| if second { n - n / 2 } else { n / 2 };
    DetectorBudget {
        max_hits_per_rule: split(b.max_hits_per_rule),
        max_match_candidates: split(b.max_match_candidates),
        max_ast_candidates: split(b.max_ast_candidates),
        max_ast_nodes: split(b.max_ast_nodes),
        max_ast_match_bytes: split(b.max_ast_match_bytes),
    }
}
pub(super) fn inspect(
    detector: &TreeSitterDetector,
    file: &ScannableFile,
    rules: &RuleSet,
    observe: bool,
    budget: DetectorBudget,
    repository: Option<&RepositoryContext>,
) -> anyhow::Result<DetectorOutcome> {
    // Drafts are retained internally even for legacy callers so comparison uses
    // exact source spans and typed identities, rather than masked preview text.
    let c = detector.detect_candidate(
        file,
        rules,
        true,
        half(budget, false),
        repository,
        Some((tree_sitter_c::LANGUAGE.into(), "c")),
    )?;
    let cpp = detector.detect_candidate(
        file,
        rules,
        true,
        half(budget, true),
        repository,
        Some((tree_sitter_cpp::LANGUAGE.into(), "cpp")),
    )?;
    Ok(consolidate(&c, &cpp, rules, observe))
}
fn consolidate(
    c: &DetectorOutcome,
    cpp: &DetectorOutcome,
    rules: &RuleSet,
    observe: bool,
) -> DetectorOutcome {
    if c.status() == DetectorStatus::NotApplicable && cpp.status() == DetectorStatus::NotApplicable
    {
        return DetectorOutcome::new(Vec::new(), DetectorStatus::NotApplicable);
    }
    let mut detections = Vec::new();
    let mut observations = Vec::new();
    let mut disagreement = false;
    let mut used = std::collections::HashSet::new();
    // Index exact occurrences once; hostile headers must not cause a quadratic
    // all-pairs comparison of the two bounded candidate populations.
    let mut index = std::collections::HashMap::new();
    for (j, d) in cpp.detections().iter().enumerate() {
        if let Some(span) = cpp.observations().get(j).and_then(|o| o.source_span()) {
            index
                .entry((d.rule_id.as_str(), span))
                .and_modify(|v| *v = None)
                .or_insert(Some(j));
        }
    }
    for (i, d) in c.detections().iter().enumerate() {
        let same = c
            .observations()
            .get(i)
            .and_then(|o| o.source_span())
            .and_then(|span| index.get(&(d.rule_id.as_str(), span)))
            .copied()
            .flatten()
            .filter(|j| {
                !used.contains(j)
                    && c.observations()
                        .get(i)
                        .zip(cpp.observations().get(*j))
                        .is_some_and(|(a, b)| a.equivalent_source_reference(b))
            });
        if let Some(j) = same {
            used.insert(j);
            let mut hit = d.clone();
            hit.evidence
                .metadata
                .insert("language".into(), "c,cpp".into());
            detections.push(hit);
            if observe {
                observations.push(c.observations()[i].clone());
            }
        } else {
            disagreement = true;
            // A rule explicitly scoped to only one language still uses its
            // selected grammar; no cross-candidate primitive binding is merged.
            let only_c = rules
                .get(&d.rule_id)
                .is_some_and(|r| !scope_matches(&r.scope, "cpp"));
            if only_c {
                let mut hit = d.clone();
                hit.evidence
                    .metadata
                    .insert("language".into(), "c,cpp".into());
                detections.push(hit);
                if observe {
                    observations.push(c.observations()[i].clone());
                }
            }
        }
    }
    for (j, d) in cpp.detections().iter().enumerate() {
        if used.contains(&j) {
            continue;
        }
        disagreement = true;
        let only_cpp = rules
            .get(&d.rule_id)
            .is_some_and(|r| !scope_matches(&r.scope, "c"));
        if only_cpp {
            let mut hit = d.clone();
            hit.evidence
                .metadata
                .insert("language".into(), "c,cpp".into());
            detections.push(hit);
            if observe {
                observations.push(cpp.observations()[j].clone());
            }
        }
    }
    let status = if [c.status(), cpp.status()].contains(&DetectorStatus::Truncated) {
        DetectorStatus::Truncated
    } else if disagreement
        || c.status() != DetectorStatus::Complete
        || cpp.status() != DetectorStatus::Complete
    {
        DetectorStatus::Partial
    } else {
        DetectorStatus::Complete
    };
    let outcome = DetectorOutcome::new(detections, status)
        .with_observations(observations)
        .with_sensitive_ranges(
            c.sensitive_ranges()
                .iter()
                .chain(cpp.sensitive_ranges())
                .cloned()
                .collect(),
        )
        .with_sensitive_source_context(
            c.sensitive_source_context() || cpp.sensitive_source_context(),
        );
    match status {
        DetectorStatus::Truncated=>outcome.with_explanation("Ambiguous C/C++ header: shared inspection budget exhausted; omitted or conflicting facts remain unknown."),
        DetectorStatus::Partial=>outcome.with_explanation("Ambiguous C/C++ header: candidate syntax or source bindings differ; only agreeing facts and explicitly scoped references were retained. Configured build language is unknown."),
        _=>outcome,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::{Detection, Evidence, Location, ObservationDraft, ObservationMethod};
    #[test]
    fn conflicting_typed_candidate_bindings_are_not_merged() {
        // Same rule and source expression can resolve to different selected
        // primitives under different candidate scopes. Preview equality is not proof.
        let root = std::env::temp_dir().join(format!("pqc-v05-header-unit-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("rule.yml"),serde_json::json!([{"id":"HEADER_CONTROL","kind":"tree_sitter","scope":"c,cpp","pattern":"CONTROL","category":"test","severity":"info","risk":"quantum-uncertain","confidence":0.5,"migration_hint":"Review"}]).to_string()).unwrap();
        let rules = RuleSet::load_from_dir(&root).unwrap();
        let candidate = |name: &str| {
            let d = Detection {
                rule_id: "HEADER_CONTROL".into(),
                location: Location {
                    file: "test.h".into(),
                    line: 1,
                    column: 1,
                },
                evidence: Evidence {
                    r#type: "tree_sitter_match".into(),
                    r#match: "same masked preview".into(),
                    snippet_preview: "same masked preview".into(),
                    metadata: Default::default(),
                },
            };
            let o = ObservationDraft::source(&d, ObservationMethod::AstPattern, "", Some((0, 20)))
                .with_source_api_references(&[name.into()]);
            DetectorOutcome::complete(vec![d]).with_observations(vec![o])
        };
        let a = candidate("ML-KEM-768");
        let b = candidate("ML-KEM-1024");
        let combined = consolidate(&a, &b, &rules, true);
        assert_eq!(combined.status(), DetectorStatus::Partial);
        assert!(combined.detections().is_empty());
        assert!(combined.observations().is_empty());
        let combined = consolidate(&a, &candidate("ML-KEM-768"), &rules, true);
        assert_eq!(combined.status(), DetectorStatus::Complete);
        assert_eq!(combined.detections().len(), 1);
        assert_eq!(combined.observations().len(), 1);
        std::fs::remove_dir_all(root).unwrap();
    }
}
