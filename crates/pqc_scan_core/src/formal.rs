//! Small-domain and long-input regression checks for production decisions.
use crate::diagnostic::{CoverageSummary, InspectionStatus};
use crate::policy::{summarize, AssessmentContext, Decision};

#[test]
fn long_decision_lists_preserve_uncertainty_and_order() {
    let context = AssessmentContext::default();
    for (last, expected) in [
        (Decision::Pass, Decision::Pass),
        (Decision::Fail, Decision::Fail),
        (Decision::Indeterminate, Decision::Indeterminate),
        (Decision::NotApplicable, Decision::Pass),
    ] {
        let mut decisions = vec![Decision::NotApplicable; 8193];
        decisions[0] = Decision::Pass;
        decisions[8192] = last;
        let forward = summarize(&decisions, true, true, &context);
        assert_eq!(forward.decision, expected);
        decisions.reverse();
        assert_eq!(forward, summarize(&decisions, true, true, &context));
    }
    let positive = vec![Decision::Pass; 8193];
    assert_eq!(
        summarize(&positive, false, true, &context).decision,
        Decision::Indeterminate
    );
    assert_eq!(
        summarize(&positive, true, false, &context).decision,
        Decision::Indeterminate
    );
    assert_eq!(
        summarize(&[], true, true, &context).decision,
        Decision::Indeterminate
    );
    assert_eq!(
        summarize(&vec![Decision::NotApplicable; 8193], true, true, &context).decision,
        Decision::Indeterminate
    );
}

fn coverage(counts: &[u64; 8]) -> CoverageSummary {
    let mut c = CoverageSummary::new();
    [
        c.analyzed_files,
        c.partial_files,
        c.analysis_failed_files,
        c.unsupported_files,
        c.not_applicable_files,
        c.unknown_files,
        c.read_failed_files,
        c.enumeration_failed_entries,
    ] = *counts;
    c.read_ok_files = counts[..6].iter().sum();
    c.selected_files = c.read_ok_files + c.read_failed_files;
    c
}

#[test]
fn small_domain_policy_and_coverage_regressions() {
    let values = [
        Decision::Pass,
        Decision::Fail,
        Decision::Indeterminate,
        Decision::NotApplicable,
    ];
    let mut policy_rows = 0;
    for len in 0..=4 {
        for encoded in 0..4_usize.pow(len) {
            let ds: Vec<_> = (0..len)
                .map(|i| values[(encoded / 4_usize.pow(i)) % 4])
                .collect();
            for (app, applicability) in ["applicable", "unknown", "not_applicable"]
                .iter()
                .enumerate()
            {
                let context = AssessmentContext::from_json(&format!(
                    "{{\"version\":1,\"applicability\":\"{applicability}\"}}"
                ))
                .unwrap();
                for complete in [false, true] {
                    for exhaustive in [false, true] {
                        let d = summarize(&ds, complete, exhaustive, &context).decision;
                        if !complete || !exhaustive || ds.contains(&Decision::Indeterminate) {
                            assert_ne!(d, Decision::Pass);
                        }
                        if app == 0 && ds.contains(&Decision::Fail) {
                            assert_eq!(d, Decision::Fail);
                        }
                        policy_rows += 1;
                    }
                }
            }
        }
    }
    assert_eq!(policy_rows, 4092);
    assert_eq!(
        summarize(&[Decision::Pass], true, true, &AssessmentContext::default()).decision,
        Decision::Pass
    );
    let mut coverage_cases = 0;
    for encoded in 0..3_u32.pow(8) {
        let counts = std::array::from_fn(|i| u64::from((encoded / 3_u32.pow(i as u32)) % 3));
        assert_coverage(&counts);
        coverage_cases += 1;
    }
    // Exercise large coherent counters while keeping every sum representable.
    for i in 0..8 {
        let mut counts = [0; 8];
        counts[i] = u64::MAX / 8;
        assert_coverage(&counts);
        coverage_cases += 1;
    }
    assert_coverage(&[u64::MAX / 8; 8]);
    coverage_cases += 1;
    assert_eq!(coverage_cases, 6570);
}

fn assert_coverage(counts: &[u64; 8]) {
    let mut c = coverage(counts);
    let before = serde_json::to_value(&c).unwrap();
    c.finish();
    let after = serde_json::to_value(&c).unwrap();
    for (key, value) in before.as_object().unwrap() {
        if key != "status" {
            assert_eq!(&after[key], value);
        }
    }
    assert_eq!(
        c.selected_files(),
        c.read_ok_files() + c.read_failed_files()
    );
    assert_eq!(c.read_ok_files(), counts[..6].iter().sum::<u64>());
    if [
        counts[1], counts[2], counts[3], counts[5], counts[6], counts[7],
    ]
    .iter()
    .any(|n| *n > 0)
    {
        assert_ne!(c.status(), InspectionStatus::CompleteWithinScope);
    }
}
