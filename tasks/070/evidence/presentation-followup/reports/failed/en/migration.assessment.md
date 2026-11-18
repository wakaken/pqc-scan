# Organization migration review

Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.

Targets: 3 · Evaluated at: <code>2026&#45;11&#45;18T00&#58;00&#58;00Z</code>

## Response overview

| System / location | Primitive / role | Urgency | Next response |
|---|---|---|---|
| [<code>Not bound</code>](#target-1) · <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> | <code>unknown</code> · Connection key establishment | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-2) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> | <code>RSA&#45;MD5</code> · Certificate signature | Urgent | Investigate |
| [<code>Not bound</code>](#target-3) · <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;public&#95;key&#95;oid</code> | <code>RSA</code> · Certificate public key | Urgency undetermined | Investigate |

<a id="target-1"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>&#47;details&#47;negotiated&#47;key&#95;exchange&#95;group</code> (Connection key establishment)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
- Exact parameter support
- Provider support
- Runtime support
- Exact primitive identity
- Exact parameters
- Actual or planned use
- Required purpose

**Next checks**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

### Migration candidates

No candidate selected for this decision.

<a id="evidence-1"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Verification not completed or collection incomplete</code> |
| Collection | <code>Collection failed</code> |

- Collection is incomplete for this target.
- Verification was not completed.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;7dc4de55d949e725fee8cca5d0130e03814c88fddf71b8b5f099a30e993bb2d3</code>

Decision: <code>migration&#95;decision&#45;v1&#45;06dd1f8c8cd72f92d491fae6496d501cf5cea0a40444c0bb01a439b893f0dc60</code>

</details>

<a id="target-2"></a>

## <code>System not bound</code> · <code>RSA&#45;MD5</code>

**Urgent** · Information missing · Readiness unknown

Where: <code>&#47;details&#47;peer&#95;certificates&#47;0&#47;signature&#95;oid</code> (Certificate signature)

### Next response

Investigate

The saved facts show a relevant classical cryptographic concern.

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
| Primitive | <code>RSA&#45;MD5</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Parsed attributes</code> |
| Collection | <code>Collection failed</code> |

- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F08</code>](#source-2)
- [<code>F12</code>](#source-3)
- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)
- [<code>R6151</code>](#source-7)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;e872792348a59baca96dd26560dbbb7d0e9bb76943b882703904f1ee984c7604</code>

Decision: <code>migration&#95;decision&#45;v1&#45;57af0a250c2273ad6a6333ac09de78dde50df961c5d5f3640b743b42dfb84a96</code>

</details>

<a id="target-3"></a>

## <code>System not bound</code> · <code>RSA</code>

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
| Primitive | <code>RSA</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Parsed attributes</code> |
| Collection | <code>Collection failed</code> |

- Collection is incomplete for this target.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F08</code>](#source-2)
- [<code>F12</code>](#source-3)
- [<code>F14</code>](#source-4)
- [<code>LOCAL026</code>](#source-5)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-6)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;4696c757de0244243d537a84a174637883f59cf7f6c02ed30a8c991bee6cad83</code>

Decision: <code>migration&#95;decision&#45;v1&#45;fcda13859d2e6064f6a32e2bb3337103a31f4c0954c1f292f319db3aaaf83e8e</code>

</details>

## Declared context and provenance

These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.

## Sources

<a id="source-1"></a>

**<code>F07</code> · <code>RFC9846 July2026</code>**

<code>4&#46;2&#59; 4&#46;3&#46;7&#59; 4&#46;3&#46;8&#59; 4&#46;5&#46;1&#45;4&#46;5&#46;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc9846&#46;html</code>

<a id="source-2"></a>

**<code>F08</code> · <code>RFC5280 2008&#59; RFC4055 2005</code>**

<code>5280&#58;4&#46;1&#46;1&#46;2&#44;4&#46;1&#46;2&#46;5&#44;4&#46;1&#46;2&#46;7&#59; 4055&#58;1&#46;2&#44;3</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc5280&#46;html</code>

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

**<code>R6151</code> · <code>RFC6151 2011</code>**

<code>2</code> · <code>https&#58;&#47;&#47;www&#46;rfc&#45;editor&#46;org&#47;rfc&#47;rfc6151&#46;html</code>

<a id="source-8"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## Assessment scope

- This is one saved attempt, not endpoint-wide coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.

Input SHA-256: <code>538e5773018c4079f6209d78ccad54416ee6c61098bd2120f2d2fc583ab05b4c</code>

Collected at: <code>2026&#45;11&#45;17T00&#58;00&#58;00Z</code>

[Machine-readable evidence and complete reference data (JSON)](migration.assessment.v1.json)

