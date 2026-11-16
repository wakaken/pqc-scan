# Product-only CI self-scan — 2026-11-16

The maintainer explicitly requested narrowing the scan input itself. Local
implementation and measurement are verified; hosted activation is not executed.
Task054 remains in-progress under its unchanged original73requirements.

CI prepares only root Cargo manifests/lock, optional root/crate build scripts,
crate manifests, crate src trees and shipped default rules in an external input.
It never traverses standalone test fixtures, examples, task/evidence archives,
documentation, CI scripts or workflow definitions as scan inputs. Colocated Rust
unit-test modules remain byte-identical. Detection defaults, rules, severities,
public result contracts and GitHub thresholds are unchanged.

The actual locked/offline default scanner build and confined run passed. The
known input contains105files/1253880bytes, all selected, read and analyzed with
zero failed, partial, unknown or enumerated exclusions. The verification gate
also checks both original/copied inventories and all byte hashes. An actual
scanner regression proves an ignore-hidden file cannot pass this gate even if
the scanner exits0. The scanner's broader unknown-population flag remains true;
these checks only establish the deliberately prepared finite population.

Compared with the prior wide scan:3531selected files become105;5255raw results
become486. All486are byte-canonical result/multiplicity matches to the old scan
restricted to the same paths. Existing exact conditions route475reviewed literal
references separately and leave11dependency observations for review
(5warning,6note,0error). All27scoped policy source bindings verify. This is not
an accuracy improvement, blanket false-positive verdict or vulnerability count.

Node50and Python50tests passed with no skips (19scope,2original CI,29evidence),
including actual scanner positive/negative cases. Syntax/workflow parsing and
combined-diff checks passed. Offline six-axis aggregation exits0 and retains
release decision Unknown. The original task053 artifact was preserved by moving
new workflow assertions into the new scope suite after an initial hash failure.
Implementation and test reviews reuse the two existing notification identities;
review records bind the final combined implementation. Logs and final-source
checks are retained alongside this record. Raw486-result SARIF and derived
policy/audit stay in private bounded artifacts with hashes, avoiding another
committed full-result/reference copy.

No push, PR, live alert dismissal, settings change or hosted execution occurred.
Historical check133176859198/new2135population remains historical and unresolved;
the smaller future scan does not retroactively adjudicate it. Task054all73final
applicability and original completion gates remain open. Assessment completion,
v0.3roadmap completion, product acceptance and publication authorization remain
separate decisions. Prior wide-scan notification evidence is preserved unchanged.
