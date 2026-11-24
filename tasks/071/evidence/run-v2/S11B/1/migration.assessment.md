# Organization migration review

Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.

Targets: 3 · Evaluated at: <code>2026&#45;11&#45;19T00&#58;00&#58;00Z</code>

## Response overview

| System / location | Primitive / role | Urgency | Next response |
|---|---|---|---|
| [<code>f11</code>](#target-1) · <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> | <code>X25519MLKEM768</code> · Connection key establishment | Monitor this role | Monitor changes to this role |
| [<code>Not bound</code>](#target-2) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> | <code>ECDSA</code> · Certificate signature | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-3) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> | <code>EC</code> · Certificate public key | Urgency undetermined | Investigate |

<a id="target-1"></a>

## <code>f11</code> · <code>X25519MLKEM768</code>

**Monitor this role** · Required information available · Migration readiness not applicable

Where: <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> (Connection key establishment)

### Next response

Monitor changes to this role

No known concern was established for this bounded role and its recognized facts.

**Next checks**

- Reassess when facts, scope or claims change.

### Migration candidates

No candidate selected for this decision.

<a id="evidence-1"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>X25519MLKEM768</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Collector reports a verified session</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The hybrid combiner implementation has not been verified.

- [Applicability](#claim-1)
- [Required purpose](#claim-2)
- [Actual or planned use](#claim-3)
- [<code>F06</code>](#source-1)
- [<code>F07</code>](#source-2)
- [<code>F12</code>](#source-4)
- [<code>F14</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;e5dcaff3d62ad2d32d37c921be44b5a3e9d069bf405efda90709c9af5d5597a6</code>

Decision: <code>migration&#95;decision&#45;v1&#45;b4ce29146b29311b1d3928e33845814c706609597f92b08178a678e2bd3b0a0f</code>

</details>

<a id="target-2"></a>

## <code>System not bound</code> · <code>ECDSA</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> (Certificate signature)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
- Exact parameter support
- Peer support
- PKI support
- Protocol support
- Provider support
- Runtime support
- Parameter encoding
- Exact parameters
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
| PKI support | Unknown |

<a id="evidence-2"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ECDSA</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Parsed attributes</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F08</code>](#source-3)
- [<code>F12</code>](#source-4)
- [<code>F14</code>](#source-5)
- [<code>LOCAL026</code>](#source-6)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;09ac1b7757eadbe1c3ab2158889936ceb1ebff30e84817f580f773e9c1b72be5</code>

Decision: <code>migration&#95;decision&#45;v1&#45;c886a216d80d06a21739e507eb534078543bf8836ad07f7884d1749f8a41fa66</code>

</details>

<a id="target-3"></a>

## <code>System not bound</code> · <code>EC</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> (Certificate public key)

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
- Actual or planned use
- Required purpose

**Next checks**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

### Migration candidates

No candidate selected for this decision.

<a id="evidence-3"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>EC</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Parsed attributes</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F08</code>](#source-3)
- [<code>F12</code>](#source-4)
- [<code>F14</code>](#source-5)
- [<code>LOCAL026</code>](#source-6)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-7)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;d6a2e1b251b4e8ff6b0c5eefa476b5b67a10318b25412b4228e21acf5975771b</code>

Decision: <code>migration&#95;decision&#45;v1&#45;7c15d636fe7b06e3f0ce7634546cf03ee64097821d0c962701e6c4ad0ec2c1dc</code>

</details>

## Declared context and provenance

These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.

<a id="claim-1"></a>

### Applicability

<code>In scope</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-2"></a>

### Required purpose

<code>Key establishment</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

<a id="claim-3"></a>

### Actual or planned use

<code>In use</code> · Active declared interval

Declared by: <code>synthetic&#45;owner</code> · Operator claim (unverified)

Asserted: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code> / Expires: <code>2027&#45;11&#45;01T00&#58;00&#58;00Z</code>

## Sources

<a id="source-1"></a>

**<code>F06</code> · <code>RFC10024 August2026&#59; RFC9954 July2026</code>**

<code>10024&#58;4&#46;1&#45;4&#46;3&#44;7&#46;1&#45;7&#46;4&#59; 9954&#58;3&#46;1&#45;3&#46;3&#44;6</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc10024&#46;html</code>

<a id="source-2"></a>

**<code>F07</code> · <code>RFC9846 July2026</code>**

<code>4&#46;2&#59; 4&#46;3&#46;7&#59; 4&#46;3&#46;8&#59; 4&#46;5&#46;1&#45;4&#46;5&#46;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc9846&#46;html</code>

<a id="source-3"></a>

**<code>F08</code> · <code>RFC5280 2008&#59; RFC4055 2005</code>**

<code>5280&#58;4&#46;1&#46;1&#46;2&#44;4&#46;1&#46;2&#46;5&#44;4&#46;1&#46;2&#46;7&#59; 4055&#58;1&#46;2&#44;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc5280&#46;html</code>

<a id="source-4"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-5"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-6"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-7"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-8"></a>

**<code>S203</code> · <code>FIPS203 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 3&#46;1&#59; 7&#59; 8 Table2</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;203&#47;final</code>

<a id="source-9"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## Assessment scope

- This is one saved attempt, not endpoint-wide coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.

Input SHA-256: <code>58af70d3e5fe71a209a43be647eedee6fb24d9239e68e499bc70047bbe5f156a</code>

Collected at: <code>2026&#45;11&#45;01T00&#58;00&#58;00Z</code>

[Machine-readable evidence and complete reference data (JSON)](migration.assessment.v1.json)

