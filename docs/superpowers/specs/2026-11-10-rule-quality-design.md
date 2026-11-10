# Task028 rule validation and companion quality provenance

Base b21330034d33ef30ec002c8e0d8222effd4b9d01, latest origin/develop, clean checkout.
Branch codex/scanner-quality-improvements intentionally has no task number so later
independent task commits can continue here. User authorizes implementation, tests,
docs, independent review and local commit only; no push/PR.

## Approach and constraints

Use existing Rule/RuleSet loading, add finite [0,1] confidence and nonblank ID
validation without trimming/rewriting valid values, and propagate every WalkDir
error instead of silently succeeding with a partial pack. Validate source f64
before narrowing to public f32, so 1.00000001 cannot round into a valid bound.
Keep inclusive endpoints, YAML map/sequence forms, aliases/unknown fields, valid
custom IDs/scopes, missing/default field behavior and regex/duplicate checks.
No new CLI flags, default warnings, report fields, DSL or rule-pattern change.
Unknown/custom scope semantics must stay compatible; inventory them in docs/tests.
No breaking scope rejection is authorized by the task's decision gate.

Alternatives: rejecting unknown scopes or emitting new default CLI warnings would
change existing custom behavior/output, so neither is chosen. The minimal existing
loader correction and development-only companion satisfy the explicit task scope.
This is an implementation choice within the user's compatibility requirement,
not a new claim that they approved changes to custom scope semantics.

All-or-nothing load failure is deliberate: no usable RuleSet is returned when any
selected YAML or traversal fails. Preserve readable YAML root-file input, empty
directories and non-followed nested symlink behavior; do not invent a new rule-pack
filesystem policy. A private iterator seam enables portable late traversal-error
injection rather than relying on chmod behavior as root.

## Companion review

Current built-ins number 119, not the task's historical117. Review all119 using a
separate original JSON companion under docs/quality (outside the loader's recursive
YAML population). Bind source file hashes and complete serialized rule snapshots.
Each row records purpose, algorithm family, quantum and classical axes, reference
editions, method limits, positive/negative target examples and known gaps. Rule
risk/severity/pattern/scope remain authoritative and unchanged. Record disagreements
rather than silently modifying rules or claiming detection is policy compliance.

Examples exercise compiled-pattern targets, not runtime/AST semantic resolution,
certificate trust or installed dependency state. Clearly separate desired labels
from observed expected matches; known misses/false positives remain explicit.
Existing end-to-end source/certificate/dependency and frozen corpus tests remain
required evidence. Companion validation runs in development tests only and does
not add a runtime metadata dependency or a new public output schema.

Existing scope behavior: regex applies to eligible text regardless of scope, with
exact config only changing observation method; key/dependency scopes are not
filters. Tree-sitter uses case-insensitive seven-language token lists split on
comma/pipe/semicolon/space, exact '*'/'any', JS scope also applies to TS, unknown
language tokens simply do not match. Certificate five exact typed scopes use
parsed fields; all other strings retain legacy composite/path fallback. Empty or
odd custom scope remains accepted. No claim that scope is a universal sandbox.

## Validation and delivery

Red loader boundary/traversal/compatibility tests, then minimal production change.
Test frozen119 rows, source binding, references and pattern controls; actual custom
scope detector/CLI errors and preserved outputs. Pinned Rust1.93.1, build/fmt/test/
clippy, Node/Python, old/new source scan shadow and frozen corpus measurement;
rebind only reviewed CI source changes, no new broad omissions. Independent
source/classification review and Evaluator acceptance before one local commit.
Task023 numeric budgets remain unapproved; do not mark it done or tune its labels.
