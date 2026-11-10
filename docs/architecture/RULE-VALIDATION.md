# Rule loading, validation and compatible scopes

`RuleSet::load_from_dir` retains YAML single-map/sequence input, existing fields,
public f32 confidence, kind aliases, tolerated unknown fields, ID ordering and
regex/duplicate-ID validation. No new CLI flag, warning stream, output schema or
runtime metadata service is added by task028.

## Rejected input

- Empty or whitespace-only IDs are invalid. Valid custom identifiers are retained
  byte-for-byte, including surrounding whitespace; no trimming or new ID grammar.
- Confidence must be finite and in inclusive `[0,1]`. Validate the source f64 before
  narrowing to the existing f32 field: `1.00000001` and `-1e-50` cannot round into
  valid boundary values. `.nan`, `.inf`, negative and above-one values fail with a
  fixed validation message. This is floating-point validation, not arbitrary
  precision decimal validation. Missing/wrongly typed fields remain errors.
- Every actual walker error propagates. A missing/unreadable traversal root or late
  enumeration failure cannot produce an apparently successful empty/partial pack.
  Read/YAML/regex/duplicate failures still reject the whole load; no partial RuleSet
  is returned. CLI reports existing operational exit1 before writing reports.

Valid empty directories still yield an empty RuleSet. Readable YAML root-file
input remains supported by the historical WalkDir behavior. Only `.yml`/`.yaml`
regular-file entries are selected; uppercase/unrelated extensions and non-followed
nested symlinks are unchanged. This task does not add a link sandbox or promise
that ignored paths were traversed. Paths in existing errors remain operator metadata.

## Scope inventory: preserved semantics

| Kind | Existing behavior |
|---|---|
| regex | Runs against eligible text regardless of scope. Exact `config` changes native observation method; other scopes do not filter language/file. |
| tree_sitter | Lowercase comparison; language tokens split on comma, pipe, semicolon and ASCII space. Whole-string `*` or `any` accepts all supported languages. A JavaScript token also applies to TypeScript. Unknown/empty tokens do not match. Surrounding whitespace prevents whole-string wildcard recognition. |
| certificate | Exact `signature_oid`, `public_key_oid`, `rsa_modulus_bits`, `certificate_validity`, `legacy_not_after_2026` use typed parsed targets with no filename fallback. All other strings, including unknown/empty/case variants, retain historical composite/path behavior. |
| dependency | Scope is not a filter; applies to the detector's recognized dependency/SBOM inputs and normalized item targets. |
| key | Scope is not a filter; recognized header prefilter and rule regex determine matching. |

Custom scope strings are not rejected or rewritten. Scope is not a universal
security boundary or consistent DSL. Typed certificate scopes are case-sensitive;
AST language scopes are case-insensitive. This asymmetry is deliberate compatibility,
not an endorsement of a new scope contract. Future tightening needs explicit
migration/human review; task028 makes no such change and emits no new default warning.

[The built-in companion](../quality/RULE-QUALITY.md) separately records all119 rules'
classifications, source editions, method limits and pattern-only controls. The data
has no effect on rule loading or detection. Numeric task023 budgets remain unapproved.
