# Organization migration review

Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.

Targets: 2 · Evaluated at: <code>2026&#45;11&#45;19T00&#58;00&#58;00Z</code>

## Response overview

| System / location | Primitive / role | Urgency | Next response |
|---|---|---|---|
| [<code>f03</code>](#target-1) · <code>S03B&#47;captured&#45;record&#46;txt&#58;1</code> | <code>RSA</code> · Certificate signature | Plan a response | Investigate |
| [<code>Not bound</code>](#target-2) · <code>S03B&#47;captured&#45;record&#46;txt&#58;1</code> | <code>unknown</code> · Certificate public key | Urgency undetermined | Investigate |

<a id="target-1"></a>

## <code>f03</code> · <code>RSA</code>

**Plan a response** · Information missing · Readiness unknown

Where: <code>S03B&#47;captured&#45;record&#46;txt&#58;1</code> (Certificate signature)

### Next response

Investigate

Active scoped facts support planning a response to the known concern.

**Obtain this information**

- Exact parameter support
- Peer support
- PKI support
- Protocol support
- Provider support
- Runtime support
- Parameter encoding
- Exact parameters
- Actual or planned use

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
| PKI support | Unknown |

<a id="evidence-1"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>RSA</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Parsed attributes</code> |
| Collection | <code>Partially collected</code> |

- Collection is incomplete for this target.
- The exact parameter encoding is unknown.

- [Applicability](#claim-1)
- [Business criticality](#claim-2)
- [Signature-verification lifetime](#claim-3)
- [Required purpose](#claim-4)
- [Scoped deadline](#claim-5)
- [<code>F12</code>](#source-1)
- [<code>F13</code>](#source-2)
- [<code>F14</code>](#source-3)
- [<code>LOCAL026</code>](#source-4)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;ec8b74d1e1444a0c6b5012b932eae031940ec6773cf31ba12b9338032ec3138e</code>

Decision: <code>migration&#95;decision&#45;v1&#45;df942b372ded93f99e1a801a23a64d41c10ba592f7e3d67708149bde3361e223</code>

</details>

<a id="target-2"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>S03B&#47;captured&#45;record&#46;txt&#58;1</code> (Certificate public key)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
- Exact parameter support
- Provider support
- Runtime support
- Parameter encoding
- Exact primitive identity
- Exact parameters
- Actual or planned use
- Required purpose

**Next checks**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

### Migration candidates

No candidate selected for this decision.

<a id="evidence-2"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Parsed attributes</code> |
| Collection | <code>Partially collected</code> |

- Collection is incomplete for this target.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-3)
- [<code>LOCAL026</code>](#source-4)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;7407316a2563230d598904064d7ec4074ccb1acb60fb70fc3c6682f2841fb578</code>

Decision: <code>migration&#95;decision&#45;v1&#45;d55097248d02cb3316a0f1581f15f50c3b114b5ced92c410a1072c2026cf09f8</code>

</details>

## Declared context and provenance

These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.

<a id="claim-1"></a>

### Applicability

<code>In scope</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### Business criticality

<code>No</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-3"></a>

### Signature-verification lifetime

<code>2031&#45;11&#45;01T00&#58;00&#58;00Z</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-4"></a>

### Required purpose

<code>Signing &#47; authentication</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-5"></a>

### Scoped deadline

<code>No applicable deadline declared</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

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

## Assessment scope

- Inventory does not establish exhaustive coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.

Input SHA-256: <code>2543aabfa5917087affa00dcf74b8e198c4cd9d5ee92537d014d32f4174c249c</code>

Collected at: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code>

[Machine-readable evidence and complete reference data (JSON)](migration.assessment.v1.json)

