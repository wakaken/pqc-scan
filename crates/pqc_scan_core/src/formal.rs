//! Execute production decisions; Lean independently generates the same input domain.
use crate::diagnostic::{CoverageSummary, InspectionStatus};
use crate::policy::{summarize, AssessmentContext, Decision};
use std::fmt::Write;

fn decision_code(d: Decision) -> char {
    match d {
        Decision::Pass => 'P',
        Decision::Fail => 'F',
        Decision::Indeterminate => 'I',
        Decision::NotApplicable => 'N',
    }
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
fn write_lean_vectors() {
    let mut output = String::new();
    let mut records = String::new();
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
            let code: String = if ds.is_empty() {
                "-".into()
            } else {
                ds.iter().copied().map(decision_code).collect()
            };
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
                        let record = summarize(&ds, complete, exhaustive, &context);
                        writeln!(records, "{}", serde_json::to_string(&record).unwrap()).unwrap();
                        let d = record.decision;
                        if !complete || !exhaustive || ds.contains(&Decision::Indeterminate) {
                            assert_ne!(d, Decision::Pass);
                        }
                        if app == 0 && ds.contains(&Decision::Fail) {
                            assert_eq!(d, Decision::Fail);
                        }
                        writeln!(
                            output,
                            "policy\t{app}\t{}\t{}\t{code}\t{}",
                            u8::from(complete),
                            u8::from(exhaustive),
                            decision_code(d)
                        )
                        .unwrap();
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
    for encoded in 0..3_u32.pow(8) {
        let counts = std::array::from_fn(|i| u64::from((encoded / 3_u32.pow(i as u32)) % 3));
        append_coverage(&mut output, &counts);
    }
    // Large representable values bridge Nat semantics and the Rust u64 boundary.
    for i in 0..8 {
        let mut counts = [0; 8];
        counts[i] = u64::MAX / 8;
        append_coverage(&mut output, &counts);
    }
    append_coverage(&mut output, &[u64::MAX / 8; 8]);
    assert_eq!(output.lines().count(), 10662);
    if let Some(directory) = std::env::var_os("PQC_FORMAL_VECTORS_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("rust.tsv"), output).unwrap();
        std::fs::write(directory.join("rust-policy-records.jsonl"), records).unwrap();
    }
}

fn append_coverage(output: &mut String, counts: &[u64; 8]) {
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
    let fields = counts
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    writeln!(output, "coverage\t{fields}\t{}", c.status().as_str()).unwrap();
}
