//! Counter coherence is a precondition, not something `finish` validates.
use super::*;

fn count() -> u64 {
    let n = kani::any();
    kani::assume(n <= u64::MAX / 8);
    n
}

#[kani::proof]
fn coverage_classification_contract() {
    let mut c = CoverageSummary::new();
    c.analyzed_files = count();
    c.partial_files = count();
    c.analysis_failed_files = count();
    c.unsupported_files = count();
    c.not_applicable_files = count();
    c.unknown_files = count();
    c.read_failed_files = count();
    c.enumeration_failed_entries = count();
    c.read_ok_files = c.analyzed_files
        + c.partial_files
        + c.analysis_failed_files
        + c.unsupported_files
        + c.not_applicable_files
        + c.unknown_files;
    c.selected_files = c.read_ok_files + c.read_failed_files;
    let before = c.clone();
    c.finish();
    let has_failure =
        c.read_failed_files > 0 || c.analysis_failed_files > 0 || c.enumeration_failed_entries > 0;
    let has_incomplete = c.partial_files > 0 || c.unsupported_files > 0;
    assert_eq!(
        c.is_complete(),
        !has_failure && !has_incomplete && c.unknown_files == 0
    );
    assert_eq!(
        c.status() == InspectionStatus::Failed,
        has_failure && c.read_ok_files == c.analysis_failed_files
    );
    assert_eq!(
        c.status() == InspectionStatus::Partial,
        (has_failure || has_incomplete)
            && !(has_failure && c.read_ok_files == c.analysis_failed_files)
    );
    assert_eq!(
        c.status() == InspectionStatus::Unknown,
        !has_failure && !has_incomplete && c.unknown_files > 0
    );
    assert_eq!(c.selected_files, before.selected_files);
    assert_eq!(c.read_ok_files, before.read_ok_files);
    assert_eq!(c.read_failed_files, before.read_failed_files);
    assert_eq!(c.analyzed_files, before.analyzed_files);
    assert_eq!(c.partial_files, before.partial_files);
    assert_eq!(c.analysis_failed_files, before.analysis_failed_files);
    assert_eq!(c.unsupported_files, before.unsupported_files);
    assert_eq!(c.not_applicable_files, before.not_applicable_files);
    assert_eq!(c.unknown_files, before.unknown_files);
    assert_eq!(
        c.enumeration_failed_entries,
        before.enumeration_failed_entries
    );
    assert_eq!(
        c.not_enumerated_population,
        before.not_enumerated_population
    );
}
