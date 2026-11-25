#[cfg(creusot)]
use creusot_std::prelude::*;

#[cfg_attr(creusot, derive(DeepModel))]
#[cfg_attr(
    not(creusot),
    derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)
)]
#[cfg_attr(not(creusot), serde(rename_all = "snake_case"))]
pub enum Decision {
    Pass,
    Fail,
    Indeterminate,
    NotApplicable,
}

#[cfg(creusot)]
#[logic]
fn seen(values: Seq<Decision>, end: Int, kind: Decision) -> bool {
    pearlite! { exists<i: Int> 0 <= i && i < end && i < values.len() && values[i] == kind }
}

#[cfg_attr(creusot, check(terminates))]
#[cfg_attr(creusot, ensures((result == Decision::Fail) == seen(decisions@, decisions@.len(), Decision::Fail)))]
#[cfg_attr(creusot, ensures((result == Decision::Pass) ==
    (inspection_complete && inventory_exhaustive
        && seen(decisions@, decisions@.len(), Decision::Pass)
        && !seen(decisions@, decisions@.len(), Decision::Fail)
        && !seen(decisions@, decisions@.len(), Decision::Indeterminate))))]
#[cfg_attr(creusot, ensures(result != Decision::NotApplicable))]
pub(crate) fn aggregate_decision(
    decisions: &[Decision],
    inspection_complete: bool,
    inventory_exhaustive: bool,
) -> Decision {
    let mut has_fail = false;
    let mut has_pass = false;
    let mut has_indeterminate = false;
    let mut i = 0;
    #[cfg_attr(creusot, invariant(i@ <= decisions@.len()))]
    #[cfg_attr(creusot, invariant(has_fail == seen(decisions@, i@, Decision::Fail)))]
    #[cfg_attr(creusot, invariant(has_pass == seen(decisions@, i@, Decision::Pass)))]
    #[cfg_attr(creusot, invariant(has_indeterminate == seen(decisions@, i@, Decision::Indeterminate)))]
    #[cfg_attr(creusot, variant(decisions@.len() - i@))]
    while i < decisions.len() {
        match decisions[i] {
            Decision::Fail => has_fail = true,
            Decision::Pass => has_pass = true,
            Decision::Indeterminate => has_indeterminate = true,
            Decision::NotApplicable => {}
        }
        i += 1;
    }
    if has_fail {
        Decision::Fail
    } else if !has_pass || has_indeterminate || !inspection_complete || !inventory_exhaustive {
        Decision::Indeterminate
    } else {
        Decision::Pass
    }
}
