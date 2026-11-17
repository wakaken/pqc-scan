# Initial case disagreements

The initial 166 actual checks had five positive disagreements, retained in
INITIAL-TEXT-CASES.json and the initial run log. Four snippets omitted imports
needed to establish the Node/Python library binding. Existing source_api and
source_facts binding guards deliberately reject unresolved receivers; existing
semantic_repairs controls verify those guards. New primary controls explicitly
import the declared library, preserving expected positive/negative counts rather
than changing a miss to success. The old snippets remain unresolved-reference
observations, not evidence that all such inputs are safe.

The initial RSA size example was a bare assignment: the detector selects
calls/literals, not arbitrary assignment ranges. Its positive control now uses
the actual Java RSAKeyGenParameterSpec constructor; the old assignment remains
a documented candidate-selection limit, not a full syntax-support claim.
Both case revisions precede the corrected run. This is implementation-exposed
validation and not an independent holdout or universal accuracy claim.
