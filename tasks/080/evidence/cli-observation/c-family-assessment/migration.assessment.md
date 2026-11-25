# Organization migration review

Review which systems need attention, what prevents migration and what to check next, using saved evidence and declared business context.

Targets: 18 · Evaluated at: <code>2026&#45;11&#45;24T07&#58;30&#58;08Z</code>

## Response overview

| System / location | Primitive / role | Urgency | Next response |
|---|---|---|---|
| [<code>Not bound</code>](#target-1) · <code>common&#46;h&#58;1</code> | <code>ML&#45;KEM&#45;1024</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-2) · <code>api&#46;cpp&#58;2</code> | <code>ML&#45;KEM&#45;512</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-3) · <code>api&#46;cpp&#58;1</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-4) · <code>api&#46;c&#58;3</code> | <code>RSA</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-5) · <code>api&#46;c&#58;3</code> | <code>ML&#45;KEM&#45;768</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-6) · <code>api&#46;c&#58;3</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-7) · <code>api&#46;c&#58;3</code> | <code>ML&#45;KEM&#45;768</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-8) · <code>api&#46;c&#58;3</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-9) · <code>api&#46;cpp&#58;2</code> | <code>ML&#45;DSA&#45;65</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-10) · <code>api&#46;c&#58;3</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-11) · <code>api&#46;c&#58;3</code> | <code>ML&#45;KEM&#45;768</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-12) · <code>common&#46;h&#58;1</code> | <code>ML&#45;KEM&#45;1024</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-13) · <code>api&#46;c&#58;3</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-14) · <code>api&#46;cpp&#58;2</code> | <code>ML&#45;KEM&#45;512</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-15) · <code>api&#46;c&#58;1</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-16) · <code>api&#46;cpp&#58;2</code> | <code>ML&#45;DSA&#45;65</code> · Algorithm reference | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-17) · <code>api&#46;c&#58;3</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |
| [<code>Not bound</code>](#target-18) · <code>api&#46;c&#58;2</code> | <code>unknown</code> · Reference only | Urgency undetermined | Investigate |

<a id="target-1"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;1024</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>common&#46;h&#58;1</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-1"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;1024</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;075f848f9a534ea5f984874a668a393469f93270e1afeed7888404bf3e83764e</code>

Decision: <code>migration&#95;decision&#45;v1&#45;c1c2dd91aad84524126c646e9ed25531b98e1a1e7a2f9724bb2b6b2a06feae77</code>

</details>

<a id="target-2"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;512</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;cpp&#58;2</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-2"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;512</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;90f03080617b72d42a698b177536b92b19fc47c9cb11116714d9c327edeb97b2</code>

Decision: <code>migration&#95;decision&#45;v1&#45;85bf59e5f31da9c54b5f6d2341bedd5dd77bf558aa739d1ac0a85d3641a27879</code>

</details>

<a id="target-3"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;cpp&#58;1</code> (Reference only)

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

<a id="evidence-3"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;5e1aecfe52d3a274f5fad35d5a9f126862b1702e3c6dc88239cdd140d2aed42d</code>

Decision: <code>migration&#95;decision&#45;v1&#45;c2e46fec3e9d77ae2ca5ba508d406449b04eb0fafce808679a66d83f40af69ed</code>

</details>

<a id="target-4"></a>

## <code>System not bound</code> · <code>RSA</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
- Exact parameter support
- Provider support
- Runtime support
- Exact parameters
- Actual or planned use
- Required purpose

**Next checks**

- Confirm actual use and the required cryptographic purpose.
- Obtain or refresh the missing, unknown or stale scoped claims.

### Migration candidates

No candidate selected for this decision.

<a id="evidence-4"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>RSA</code> |
| Quantum exposure | <code>Quantum exposure identified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>NIST&#45;PQ&#45;FAQ</code>](#source-4)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;9ef18fc3dd28b79db9a553cdc7ca700f0c49e8d2d41b544bf578df5eb3436443</code>

Decision: <code>migration&#95;decision&#45;v1&#45;ecc5a2993b3d1b84cd7af283df747f62d714e5c7d47f31b623d8efc553b2eac6</code>

</details>

<a id="target-5"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;768</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-5"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;768</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;fdedc50f41054caf563bebef61078a5b6bb96c9cbad3ef07375314d850908abc</code>

Decision: <code>migration&#95;decision&#45;v1&#45;519f8b95c1219d171662b2a50b45fbb4699037a06e71f07218781a64701255ef</code>

</details>

<a id="target-6"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Reference only)

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

<a id="evidence-6"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;4c1ef6bab3f433c0e6ced0f58354a96ae87c7fce56bb84168dd93eab6a018be2</code>

Decision: <code>migration&#95;decision&#45;v1&#45;72c942dc9e4525df735b8c74f59f12e696d8ba72fc8b7efcde8c56262184dcee</code>

</details>

<a id="target-7"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;768</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-7"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;768</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;a13b2cf075822e9c045601b770d654a20b3ac2cc903663e6d706a4ef1128b742</code>

Decision: <code>migration&#95;decision&#45;v1&#45;47bba4051508d64df206ba25cb6a56c2fe09f1c425789b971e96f26088db8ccb</code>

</details>

<a id="target-8"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Reference only)

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

<a id="evidence-8"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Text match</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;f66eabbf4c3580539781c7d6dd32ca17e7dc43894d67648fcc956f4f197fea5d</code>

Decision: <code>migration&#95;decision&#45;v1&#45;f0abeb84258e2701ebe3cd3fbae2ee0bfe9af759a029fcb71dc34143a6bd3687</code>

</details>

<a id="target-9"></a>

## <code>System not bound</code> · <code>ML&#45;DSA&#45;65</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;cpp&#58;2</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-9"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;DSA&#45;65</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S204</code>](#source-6)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;6aaacf1bf01cdb87fd172bdfdef0a61f078f7d054cd2547c7e8195e15cd36bf7</code>

Decision: <code>migration&#95;decision&#45;v1&#45;846c313b134c97e01d796fa3c01200d8a09d7eb62ef70d820adc47cd69233f21</code>

</details>

<a id="target-10"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Reference only)

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

<a id="evidence-10"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;3900bec34586594f79901b03b1cb30b034d73ef54351a709e2dbe0527284637e</code>

Decision: <code>migration&#95;decision&#45;v1&#45;912d2116e44b16c0ff9c32a7546d38c1333d6326924d5728da8199a6d4e04e94</code>

</details>

<a id="target-11"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;768</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-11"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;768</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;6c49e4efd21aa3f71bf4483b342a5b172eb11250ed86efad9835f240e23c74b9</code>

Decision: <code>migration&#95;decision&#45;v1&#45;10ca642af0e79e519d30093949362d4d85fb4187f2f737c888e06834d7be3cdc</code>

</details>

<a id="target-12"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;1024</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>common&#46;h&#58;1</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-12"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;1024</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;a7cad6554834de176879eab4433e834ecc24ab907c1bc6729ab57eb391d1928b</code>

Decision: <code>migration&#95;decision&#45;v1&#45;b5f9a59a1a8311408febb5b9ef8ef2351f96d154412e82730dd9cf07d9c7da67</code>

</details>

<a id="target-13"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Reference only)

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

<a id="evidence-13"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;75da385017bd1c1dc59691ed2da14a408849df6314780b11531ec3d7d4cae303</code>

Decision: <code>migration&#95;decision&#45;v1&#45;8e374716d8dfa14140dad7b229bb80d03ed9930a2ed15ed76de8dd3eaf371a0a</code>

</details>

<a id="target-14"></a>

## <code>System not bound</code> · <code>ML&#45;KEM&#45;512</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;cpp&#58;2</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-14"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;KEM&#45;512</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S203</code>](#source-5)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;6eee1a0be5aa3f74e1256ae328162738eeb9b01ccb074bfeea7ee5edf84cda3a</code>

Decision: <code>migration&#95;decision&#45;v1&#45;d8f207839f8fc39992eba926234792bb0eb7958a542381ce523b85f7e6a3426d</code>

</details>

<a id="target-15"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;1</code> (Reference only)

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

<a id="evidence-15"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;075a2b54ff848013e92dc77ff85637e8c202690c7b08372e6aafb030173f16fa</code>

Decision: <code>migration&#95;decision&#45;v1&#45;c65e62e566e298cd720d8323b0aa4025fd555ad0c3bec9da4e841de56f87af91</code>

</details>

<a id="target-16"></a>

## <code>System not bound</code> · <code>ML&#45;DSA&#45;65</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;cpp&#58;2</code> (Algorithm reference)

### Next response

Investigate

The available facts do not establish a supported urgency branch.

**Obtain this information**

- Applicability
- Explicit system binding
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

No candidate selected for this decision.

<a id="evidence-16"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>ML&#45;DSA&#45;65</code> |
| Quantum exposure | <code>Designed for quantum resistance&#59; deployment unverified</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F12</code>](#source-1)
- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)
- [<code>S204</code>](#source-6)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;960a565772e5e49558febfea43a3862f13096150910d2a35dba2ec3faa0c3135</code>

Decision: <code>migration&#95;decision&#45;v1&#45;050b57b9d269e5d9abc2b5a8a8de61c4ab75ae6af114db7bcadcbc37e0a78c36</code>

</details>

<a id="target-17"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;3</code> (Reference only)

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

<a id="evidence-17"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;5b3b3075d6b59cf1c382fb128879b7386297340a26241eb070b3863f6ef26c62</code>

Decision: <code>migration&#95;decision&#45;v1&#45;94cf1506326df62dd6b283442f88e180fd096343f0166d6697d9f1f191394394</code>

</details>

<a id="target-18"></a>

## <code>System not bound</code> · <code>unknown</code>

**Urgency undetermined** · Information missing · Readiness unknown

Where: <code>api&#46;c&#58;2</code> (Reference only)

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

<a id="evidence-18"></a>

### Evidence for this response

| Fact | Recorded value |
|---|---|
| Primitive | <code>unknown</code> |
| Quantum exposure | <code>Unknown</code> |
| Evidence assurance | <code>Syntactic reference</code> |
| Collection | <code>Completed within recorded scope</code> |

- A reference does not establish deployed use.
- The primitive is unidentified.
- The exact parameter encoding is unknown.
- The business-system binding is unknown.

- [<code>F14</code>](#source-2)
- [<code>LOCAL026</code>](#source-3)

<details><summary>Reference IDs</summary>

Target: <code>migration&#95;target&#45;v1&#45;c01f4600251a084b3f578bce11c3c22c8d95af5aa7f3233b913c8665007bea3d</code>

Decision: <code>migration&#95;decision&#45;v1&#45;f5acf79c1f460fbc29f390fddb59c2c0d60459e1a352eb9008e4c460badb237b</code>

</details>

## Declared context and provenance

These are unverified declarations, including those labeled external review. Readiness describes active declarations, not verified deployment.

## Sources

<a id="source-1"></a>

**<code>F12</code> · <code>task066 adopted contract 2026&#45;11&#45;18</code>**

<code>technical dispatch and ingestion invariants&#59; FACT&#45;SOURCE&#45;MAPPING F12</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-2"></a>

**<code>F14</code> · <code>migration&#45;decision&#45;v1 1&#46;0&#46;0&#59; adopted 2026&#45;11&#45;18</code>**

<code>P1&#45;P8&#59; readiness and action precedence</code> · <code>docs&#47;architecture&#47;MIGRATION&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-3"></a>

**<code>LOCAL026</code> · <code>local&#45;assessment&#45;1</code>**

<code>finite vocabulary and evidence limits</code> · <code>docs&#47;architecture&#47;LOCAL&#45;ASSESSMENT&#45;V1&#46;md</code>

<a id="source-4"></a>

**<code>NIST&#45;PQ&#45;FAQ</code> · <code>task056 pinned FAQ snapshot</code>**

<code>quantum public&#45;key and generic&#45;search models</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;projects&#47;post&#45;quantum&#45;cryptography&#47;faqs</code>

<a id="source-5"></a>

**<code>S203</code> · <code>FIPS203 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 3&#46;1&#59; 7&#59; 8 Table2</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;203&#47;final</code>

<a id="source-6"></a>

**<code>S204</code> · <code>FIPS204 Final 2024&#45;08&#45;13</code>**

<code>1&#46;1&#59; 4 Table1</code> · <code>https&#58;&#47;&#47;csrc&#46;nist&#46;gov&#47;pubs&#47;fips&#47;204&#47;final</code>

## Assessment scope

- Inventory does not establish exhaustive coverage.
- The digest binds saved content, not collector authenticity.
- Offline assessment does not repeat cryptographic verification.
- This local advisory policy has not established practitioner usefulness.

Input SHA-256: <code>11a003b7159f2cab33238178fbedbaf56fa443460621098a6c161137aa8a004f</code>

Collected at: <code>2026&#45;11&#45;24T07&#58;30&#58;07&#46;078123&#43;00&#58;00</code>

[Machine-readable evidence and complete reference data (JSON)](migration.assessment.v1.json)

