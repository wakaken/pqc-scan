# Organization migration review

Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.

Targets: 1 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## Response overview

| System / location | Primitive / role | Urgency | Next response |
|---|---|---|---|
| [<code>system&#45;a</code>](#target-1) · <code>app&#46;txt&#58;1</code> | <code>RSA</code> · Algorithm reference | Urgent | Investigate |

<a id="target-1"></a>

## <code>system&#45;a</code> · <code>RSA</code>

**Urgent** · Information missing · Readiness unknown

Where: <code>app&#46;txt&#58;1</code> (Algorithm reference)

### Next response

Investigate

A stale due-deadline claim requires urgent investigation and refresh.

**Obtain this information**

- Exact parameter support
- Peer support
- Protocol support
- Provider support
- Runtime support

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

<a id="evidence-1"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>RSA</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- A used claim is not currently active; refresh it.

- [Actual or planned use](#claim-1)
- [Applicability](#claim-2)
- [Required purpose](#claim-3)
- [Scoped deadline](#claim-4)
- [<code>F12</code>](#source-1)
- [<code>F13</code>](#source-2)
- [<code>F14</code>](#source-3)
- [<code>LOCAL026</code>](#source-4)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)
- [<code>context&#58;org&#45;deadline</code>](#source-7)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad</code>

Decision: <code>migration&#95;decision&#45;v1&#45;0b1785e6769b6c6a39742cff0e5f114b58b26928bf8367236aca7c712f916271</code>

</details>

## Declared context and provenance

These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.

<a id="claim-1"></a>

### Actual or planned use

<code>In use</code> · Active declared interval

Declared by: <code>owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### Applicability

<code>In scope</code> · Active declared interval

Declared by: <code>owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-3"></a>

### Required purpose

<code>Signing &#47; authentication</code> · Active declared interval

Declared by: <code>owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;17T00&#58;00&#58;00Z</code>

<a id="claim-4"></a>

### Scoped deadline

<code>2026&#45;11&#45;16T00&#58;00&#58;00Z · Reference&#58; org&#45;deadline</code> · Expired declaration

Declared by: <code>owner</code> · External-review claim (unverified)

Asserted: <code>2026&#45;11&#45;16T00&#58;00&#58;00Z</code> / Expires: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

## Sources

<a id="source-1"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-2"></a>

**<code>F13</code> · <code>task066 adopted time contract 2026&#45;11&#45;18</code>**

<code>time and policy semantics&#59; FACT&#45;SOURCE&#45;MAPPING F13</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-3"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-6"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

<a id="source-7"></a>

**<code>context&#58;org&#45;deadline</code> · <code>v1</code>**

<code>3&#46;1</code> · <code>https&#58;&#47;&#47;synthetic&#46;invalid&#47;never&#45;fetch</code>

## Assessment scope

- Inventory does not establish exhaustive coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.

Input SHA-256: <code>ea072032ff581bb287e010de83d5c9be0f72debdce0962925dfa3c5dbf29f81f</code>

Collected at: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[Machine-readable evidence and complete reference data (JSON)](migration.assessment.v1.json)

