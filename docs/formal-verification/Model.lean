import Std

namespace PQC

inductive Decision where
  | pass | fail | indeterminate | notApplicable
  deriving DecidableEq, Repr

inductive Applicability where
  | applicable | unknown | notApplicable
  deriving DecidableEq

-- Declarative specification: positive evidence and the absence of blockers.
def PassAllowed (a : Applicability) (ds : List Decision) (c e : Bool) : Prop :=
  a = .applicable ∧ c = true ∧ e = true ∧
  .pass ∈ ds ∧ .fail ∉ ds ∧ .indeterminate ∉ ds

def PolicySpec (a : Applicability) (ds : List Decision) (c e : Bool)
    (result : Decision) : Prop :=
  (a = .notApplicable ∧ result = .notApplicable) ∨
  (a = .unknown ∧ result = .indeterminate) ∨
  (a = .applicable ∧
    ((.fail ∈ ds ∧ result = .fail) ∨
     (PassAllowed a ds c e ∧ result = .pass) ∨
     (.fail ∉ ds ∧ ¬ PassAllowed a ds c e ∧ result = .indeterminate)))

-- Executable abstraction of summarize's ordered branches, independent of PolicySpec.
def summarize (a : Applicability) (ds : List Decision) (c e : Bool) : Decision :=
  match a with
  | .notApplicable => .notApplicable
  | .unknown => .indeterminate
  | .applicable =>
    if .fail ∈ ds then .fail
    else if .pass ∉ ds ∨ .indeterminate ∈ ds ∨ c = false ∨ e = false
      then .indeterminate else .pass

theorem policy_correct (a : Applicability) (ds : List Decision) (c e : Bool) :
    PolicySpec a ds c e (summarize a ds c e) := by
  cases a <;> cases c <;> cases e <;>
    by_cases hf : Decision.fail ∈ ds <;>
    by_cases hp : Decision.pass ∈ ds <;>
    by_cases hi : Decision.indeterminate ∈ ds <;>
    simp_all [summarize, PolicySpec, PassAllowed]

theorem pass_iff (a : Applicability) (ds : List Decision) (c e : Bool) :
    summarize a ds c e = .pass ↔ PassAllowed a ds c e := by
  cases a <;> cases c <;> cases e <;>
    by_cases hf : Decision.fail ∈ ds <;>
    by_cases hp : Decision.pass ∈ ds <;>
    by_cases hi : Decision.indeterminate ∈ ds <;>
    simp_all [summarize, PassAllowed]

theorem incomplete_never_pass (a : Applicability) (ds : List Decision) (c e : Bool)
    (h : c = false ∨ e = false) : summarize a ds c e ≠ .pass := by
  intro hp
  have allowed := (pass_iff a ds c e).mp hp
  rcases allowed with ⟨_, hc, he, _⟩
  rcases h with h | h <;> simp_all

theorem indeterminate_never_pass (a : Applicability) (ds : List Decision) (c e : Bool)
    (h : Decision.indeterminate ∈ ds) : summarize a ds c e ≠ .pass := by
  intro hp
  exact ((pass_iff a ds c e).mp hp).2.2.2.2.2 h

theorem fail_dominates (ds : List Decision) (c e : Bool) (h : Decision.fail ∈ ds) :
    summarize .applicable ds c e = .fail := by
  simp [summarize, h]

structure Coverage where
  readOk : Nat
  readFailed : Nat
  analysisFailed : Nat
  enumerationFailed : Nat
  partialFiles : Nat
  unsupported : Nat
  unknown : Nat

inductive Status where
  | complete | incomplete | failed | unknown
  deriving DecidableEq

-- Natural-number model; Rust's representability assumptions are checked separately.
def finish (s : Coverage) : Status :=
  let failed := s.readFailed + s.analysisFailed + s.enumerationFailed
  let incomplete := s.partialFiles + s.unsupported
  if failed > 0 ∨ incomplete > 0 then
    if failed > 0 ∧ s.readOk = s.analysisFailed then .failed else .incomplete
  else if s.unknown > 0 then .unknown else .complete

-- The specification names each blocker separately rather than repeating the sums.
def CompleteAllowed (s : Coverage) : Prop :=
  s.readFailed = 0 ∧ s.analysisFailed = 0 ∧ s.enumerationFailed = 0 ∧
  s.partialFiles = 0 ∧ s.unsupported = 0 ∧ s.unknown = 0

theorem complete_iff (s : Coverage) : finish s = .complete ↔ CompleteAllowed s := by
  dsimp [finish, CompleteAllowed]
  split <;> split <;> simp_all <;> omega

theorem failure_never_complete (s : Coverage)
    (h : s.readFailed > 0 ∨ s.analysisFailed > 0 ∨ s.enumerationFailed > 0 ∨
         s.partialFiles > 0 ∨ s.unsupported > 0 ∨ s.unknown > 0) :
    finish s ≠ .complete := by
  intro hc
  have allowed := (complete_iff s).mp hc
  unfold CompleteAllowed at allowed
  omega

-- Kernel dependency audit consumed by the runner. No added axioms or native_decide.
#print axioms policy_correct
#print axioms pass_iff
#print axioms incomplete_never_pass
#print axioms indeterminate_never_pass
#print axioms fail_dominates
#print axioms complete_iff
#print axioms failure_never_complete

end PQC
