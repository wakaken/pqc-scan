import Model

open PQC

def decisionCode : Decision → String
  | .pass => "P" | .fail => "F" | .indeterminate => "I" | .notApplicable => "N"

def decisionAt (n : Nat) : Decision :=
  match n % 4 with
  | 0 => .pass | 1 => .fail | 2 => .indeterminate | _ => .notApplicable

def decisions (len encoded : Nat) : List Decision :=
  (List.range len).map fun i => decisionAt (encoded / 4 ^ i)

def statusCode : Status → String
  | .complete => "complete_within_scope" | .incomplete => "partial"
  | .failed => "failed" | .unknown => "unknown"

def coverageLine (counts : List Nat) : String := Id.run do
  let s : Coverage := {
    readOk := (counts.take 6).sum
    readFailed := counts[6]!
    analysisFailed := counts[2]!
    enumerationFailed := counts[7]!
    partialFiles := counts[1]!
    unsupported := counts[3]!
    unknown := counts[5]!
  }
  return s!"coverage\t{String.intercalate "," (counts.map toString)}\t{statusCode (finish s)}"

def main : IO Unit := do
  let out ← IO.getStdout
  for len in [:5] do
    for encoded in [:4 ^ len] do
      let ds := decisions len encoded
      let code := if ds.isEmpty then "-" else String.join (ds.map decisionCode)
      for app in [:3] do
        let a := if app == 0 then Applicability.applicable
          else if app == 1 then .unknown else .notApplicable
        for c in [false, true] do
          for e in [false, true] do
            out.putStrLn s!"policy\t{app}\t{if c then 1 else 0}\t{if e then 1 else 0}\t{code}\t{decisionCode (summarize a ds c e)}"
  for encoded in [:3 ^ 8] do
    out.putStrLn (coverageLine ((List.range 8).map fun i => encoded / 3 ^ i % 3))
  let bound := (2 ^ 64 - 1) / 8
  for i in [:8] do
    out.putStrLn (coverageLine ((List.range 8).map fun j => if i == j then bound else 0))
  out.putStrLn (coverageLine (List.replicate 8 bound))
