//! Direct bounded verification of the allocation-free production decision kernel.
use super::*;

fn decision() -> Decision {
    match kani::any::<u8>() % 4 {
        0 => Decision::Pass,
        1 => Decision::Fail,
        2 => Decision::Indeterminate,
        _ => Decision::NotApplicable,
    }
}

#[kani::proof]
#[kani::unwind(5)]
fn policy_aggregation_contract() {
    let values = [decision(), decision(), decision(), decision()];
    let len: usize = kani::any();
    kani::assume(len <= values.len());
    let ds = &values[..len];
    let complete: bool = kani::any();
    let exhaustive: bool = kani::any();
    let actual = aggregate_decision(ds, complete, exhaustive);
    let pass_allowed = complete
        && exhaustive
        && ds.contains(&Decision::Pass)
        && !ds.contains(&Decision::Fail)
        && !ds.contains(&Decision::Indeterminate);
    assert_eq!(actual == Decision::Pass, pass_allowed);
    assert_eq!(actual == Decision::Fail, ds.contains(&Decision::Fail));
    assert_ne!(actual, Decision::NotApplicable);
    if !complete || !exhaustive || ds.contains(&Decision::Indeterminate) {
        assert_ne!(actual, Decision::Pass);
    }
}
