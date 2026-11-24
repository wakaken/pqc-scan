# Organization migration review

Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.

Targets: 2 · Evaluated at: <code>2026&#45;11&#45;19T00&#58;00&#58;00Z</code>

## Response overview

| System / location | Primitive / role | Urgency | Next response |
|---|---|---|---|
| [<code>f12</code>](#target-1) · <code>&#47;details&#47;offers&#47;kex&#95;algorithms&#47;0</code> | <code>mlkem768nistp256&#45;sha256</code> · Offered SSH key establishment | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-2) · <code>&#47;details&#47;offers&#47;server&#95;host&#95;key&#95;algorithms&#47;0</code> | <code>ECDSA</code> · Offered SSH host signature | Urgency undetermined | Investigate |

<a id="target-1"></a>

## <code>f12</code> · <code>mlkem768nistp256&#45;sha256</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>&#47;details&#47;offers&#47;kex&#95;algorithms&#47;0</code> (Offered SSH key establishment)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Selected negotiation or actual use
- Exact parameter support
- Peer support
- Protocol support
- Provider support
- Runtime support
- Actual or planned use

**Next checks**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

### Migration candidates

No candidate selected for this decision.

<a id="evidence-1"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>mlkem768nistp256&#45;sha256</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Unauthenticated offers</code> |
| Collection | <code>Partially collected</code> |

- These are unauthenticated offers, not a selected negotiation.
- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The hybrid combiner implementation has not been verified.

- [Required purpose](#claim-1)
- [Applicability](#claim-2)
- [<code>F09</code>](#source-1)
- [<code>F10</code>](#source-2)
- [<code>F12</code>](#source-3)
- [<code>F14</code>](#source-4)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;f3a0127271975e8bd5d73fd73668f6d3cb5e1a5da8900503a608a7517a70ea32</code>

Decision: <code>migration&#95;decision&#45;v1&#45;bb0657a62cf22232d3775d621ab09a1c9afcf3e1726ee14463adac6a21b5ba4f</code>

</details>

<a id="target-2"></a>

## <code>System not bound</code> · <code>ECDSA</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>&#47;details&#47;offers&#47;server&#95;host&#95;key&#95;algorithms&#47;0</code> (Offered SSH host signature)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
- Selected negotiation or actual use
- Exact parameter support
- Peer support
- Protocol support
- Provider support
- Runtime support
- Actual or planned use
- Required purpose

**Next checks**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

### Migration candidates

<a id="candidate-1"></a>

**<code>ML&#45;DSA</code> · Signing / authentication**

Integration is unverified. Select exact parameters and validate compatibility before adoption.

| Prerequisite | Declared support |
|---|---|
| Runtime support | Unknown |
| Provider support | Unknown |
| Exact parameter support | Unknown |
| Peer support | Unknown |
| Protocol support | Unknown |

<a id="evidence-2"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ECDSA</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Unauthenticated offers</code> |
| Collection | <code>Partially collected</code> |

- These are unauthenticated offers, not a selected negotiation.
- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F09</code>](#source-1)
- [<code>F12</code>](#source-3)
- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;465f39314d07735640800c00d8d768a7d9a32a79954572fdd72519297164eca0</code>

Decision: <code>migration&#95;decision&#45;v1&#45;14389a1b63a3e4baf49cb0360129b92ec74d737c1996100a832532999cd77f04</code>

</details>

## Declared context and provenance

These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.

<a id="claim-1"></a>

### Required purpose

<code>Key establishment</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### Applicability

<code>In scope</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

## Sources

<a id="source-1"></a>

**<code>F09</code> · <code>RFC4253 2006</code>**

<code>7&#46;1</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc4253&#46;html</code>

<a id="source-2"></a>

**<code>F10</code> · <code>RFC10042 August2026</code>**

<code>2&#46;3&#46;1&#45;2&#46;3&#46;3&#59; 2&#46;4&#59; 2&#46;5&#59; 5</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc10042&#46;html</code>

<a id="source-3"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-6"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-7"></a>

**<code>S203</code> · <code>FIPS203 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 3&#46;1&#59; 7&#59; 8 Table2</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;203&#47;final</code>

<a id="source-8"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## Assessment scope

- This is one saved attempt, not endpoint-wide coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.

Input SHA-256: <code>9234ed125428e7b12dec2d871f09ae629982bd19cffc7962a85e315a599a3e3c</code>

Collected at: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code>

[Machine-readable evidence and complete reference data (JSON)](migration.assessment.v1.json)

