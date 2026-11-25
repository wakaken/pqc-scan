# Supported inputs and inspection limits

[日本語](COVERAGE.ja.md) · [README](../../README.md)

This guide describes the current source checkout, including PHP, C#, C/C++ and
related dependency support under v0.5 development. The v0.4.0 release scope is
narrower; use a matching scanner and rule bundle.
See the [input overview](../../README.md#targets) and [scanning guide](SCANNING.md).

## Contents

- [Dependency files](#dependencies)
- [SBOM input](#sbom)
- [Certificates and key markers](#certificates)
- [Configuration files and rules](#configuration)
- [Source code](#source-limits)
- [Privacy and excluded files](#privacy)

<a id="dependencies"></a>

## Dependency files

The following files provide relevant library and version information:

- Rust: `Cargo.toml`, `Cargo.lock`
- Ruby: `Gemfile`, `Gemfile.lock`, `gems.rb`, `gems.locked`, `*.gemspec`
- Java: `pom.xml`, `build.gradle`, `build.gradle.kts`, `gradle.lockfile`
- Go: `go.mod`
- JavaScript/TypeScript: `package.json`, `package-lock.json`, `pnpm-lock.yaml`
- Python: `requirements.txt`, `Pipfile.lock`, `poetry.lock`
- PHP: `composer.json`, `composer.lock`
- C#/.NET: `.csproj`, `Directory.Packages.props`, `Directory.Build.props`, `packages.config`, `packages.lock.json`, `global.json`
- C/C++: `vcpkg.json`, `vcpkg-configuration.json`, `conanfile.txt`, `conan.lock`, `CMakeLists.txt`, `*.cmake`

See [PHP, .NET and native dependency inputs](#native-dependencies) for the supported
subsets and distinctions between declarations, locks and build references.

Declared versions are distinguished from versions recorded in lockfiles.
Dynamically determined values are not treated as confirmed values. Finding a
library does not establish actual use of its cryptographic algorithms.

<a id="sbom"></a>

## SBOM input

Supported inputs are CycloneDX JSON (1.4–1.7, `*.cdx.json`/`*bom*.json`) and
SPDX JSON (2.2–2.3, `*.spdx.json`/`*sbom*.json`). These describe recorded
components and packages; they do not verify installation state.

<a id="certificates"></a>

## Certificates and key markers

Public certificates in `.pem`, `.crt`, `.cer` and `.der` provide signature and
public-key algorithms, supported key sizes/curves and validity periods.
Certificate signatures and public keys are shown separately. Expiry checks use
the scan time; they do not establish trust or use by a live endpoint.

For `.p12`/`.pfx`, readable public certificates are inspected. Password entry and
decryption are not supported. Check incomplete-inspection notices for unreadable contents.

RSA, PKCS#8 and OpenSSH private-key headers are also detected. This does not
decrypt private keys or verify their validity.

<a id="configuration"></a>

## Configuration files and rules

Supported configurations include Kubernetes/Ingress, nginx, Apache httpd, Envoy,
Istio, HAProxy and Traefik. Checks cover TLS, cipher suites and key-exchange groups.
These are file settings, not confirmed algorithms in a live connection.

There are 139 built-in rules. Use the matching bundled `rules/default` directory.

<a id="source-limits"></a>
<a id="source-facts"></a>

## Source-code inspection limits

Checks cover cryptographic API and algorithm references in Java, Go, JavaScript,
TypeScript/TSX, Python, Rust, Ruby, PHP, C# and C/C++. Code is not executed. Supported usage
patterns are limited, so not every cryptographic operation can be found.
An algorithm reference does not establish runtime use or completed PQC migration.

The development scanner also routes custom AST rules to PHP (`php`), C#
(`csharp`), C (`c`) and C++ (`cpp`). Built-in API coverage is limited to the
explicit subsets below; parser availability alone does not establish arbitrary
API support.
PHP includes `.php`, `.phtml` and `.inc` with an explicit PHP opening tag; mixed
HTML is inspected only for static PHP. Blade remains incomplete and is never
rendered. C# accepts `.cs`; Razor and cross-file semantic resolution are excluded.
C uses `.c`, while `.C`, `.cc`, `.cpp`, `.cxx`, `.hpp`, `.hh` and `.hxx` use C++.
A `.h` file has bounded C/C++ candidate views sharing work limits. Matching
source facts are retained once; differing syntax/bindings remain incomplete.
Neither preprocessing nor include/build execution selects an effective language.


PHP built-ins inspect explicit OpenSSL key types and RSA operations, local DH/ECDH
derivation and generic signatures without guessing an imported key type. Sodium
Ed25519/X25519 and ML-KEM-768 differ from X-Wing. ParagonIE native/polyfill/Compat
references preserve exact parameter families and local signing/KEM receiver roles;
SLH-DSA and phpseclib are qualified references. Native oqsphp and Composer/FFI OQS
wrappers remain distinct opaque references. These facts do not establish loaded
extensions, implementation equivalence, installation or active protection.

PHP namespaces, imports and function aliases use a bounded, source-ordered local
subset. Shadowing, reassignment, unknown writes and dynamic/reflected/FFI calls do
not inherit an older key identity. Findings show ordinary arguments and source
context, with recognized credential values, private-key blocks and supported
literal key inputs masked before clipping. Uninspected regions can retain broader
protection when analysis stops early. Opaque payload tokens do not become algorithm
assets; explicit API identities remain. Custom rules retain generic matching and
shared secret redaction. Arbitrary cross-file resolution is excluded. See
[source excerpts](REPORTS.md#source-excerpts).

When a variable reaches its tracking limit, later values of that variable remain
unknown. Independent API references are retained, and inspection still reports
truncation.

C# covers explicit standard RSA/DSA/EC/ECDH providers and bounded local operations,
.NET MLKem/MLDsa/SlhDsa generation/import and exact pure parameter symbols, and
supported Bouncy Castle generators/KEM/signers, including RSA `RsaDigestSigner`
references. The .NET PQC classes are handled
independently of AsymmetricAlgorithm. Imports/type references do not establish
successful operations. Named arguments and locally known receiver roles are
supported. Namespace aliases accept Alias.Type and Alias::Type; type parameters
and properties shadow imported names within their owning type/method scope.
Unknown writes and captured outer variables in deferred functions stop
older bindings. Cross-file globals, fields/inheritance, reflection, generators and
project-wide overload resolution are excluded; preprocessor selection is not run.
A saturated local variable keeps later values unresolved while independent API
references remain available; the inspection status stays Truncated.

CompositeMLKem/CompositeMLDsa/Hpke remain construction-unresolved references.
Generic HPKE is not necessarily post-quantum. Prehash/OID selectors outside the
finite table remain unresolved. BC TLS, obsolete official dotnetOQS and independent
OpenForge namespaces remain distinct opaque references. Platform/provider/version
availability is not inferred from source or target-framework declarations. Selected
C# built-ins use the same selective secret masking as PHP; ordinary arguments
and source context remain readable. Explicit finite API identities remain in
inventory. Opaque library occurrences retain their own fixed public project label
in findings/CBOM and a finite library marker in native inventory. Secret-valued
metadata remains protected. These labels do not establish installed libraries or
active use, and displayed code does not become a typed algorithm fact.

C/C++ covers the specified OpenSSL RSA/DSA/DH/EC low-level calls, explicit EVP
selectors and locally bound contexts/generated EVP keys, and OQS C requests and
qualified liboqs-cpp KEM/signature receivers. C++ parenthesized/list/multiple local
declarations, direct qualified construction and new-expression references are
supported. Pointer flow from new, arbitrary include/template/function-pointer
resolution, C++ namespace/type aliases, inherited/field/global mutable values and
compiler overload semantics remain outside this bounded subset. Macros and
conditionals are not evaluated; conditional source references remain incomplete.
Unknown calls, indirect writes, name shadows and conflicting roles invalidate
older bindings. Raw low-level/OQS objects do not become EVP keys.

Raw public/private EVP key imports retain Ed25519/X25519 numeric IDs or `_ex`
names, and exact ML-KEM/ML-DSA `_ex` parameter sets. Other raw-import selectors
remain unresolved; supplied key data is not validated.

Recognized includes remain library references. Finite provider/property values
and bare colon-separated TLS groups retain exact public labels/markers without
algorithm assets, loading or negotiation claims. Standalone MLKEM names, standard
hybrids, draft groups and aliases remain distinct. Other group grammar operators,
unknown/escaped/dynamic values and custom provider names remain unresolved.
New source rules intentionally overlap existing lexical OpenSSL rules; the older
IDs and occurrences remain. Ordinary source context is visible; recognized secret
values and secret-valued metadata are masked.
All native observations remain syntactic/referenced. No preprocessing, build,
linking, package manager or cryptographic execution takes place.

C/C++ distinguishes EVP numeric IDs from names and OpenSSL SLH-DSA names from
liboqs mechanism strings with exact uppercase S/F. Explicit EVP key/context outputs
update local facts; opaque output pointers discard older bindings. Known buffer
writes invalidate earlier string selections. C++ template parameters and using
declarations supply owner-scoped shadow guards, not instantiation or import
resolution.

Syntax errors, unsupported constructs and processing limits can leave inspection
incomplete while retaining available findings. Review the stated reasons and next
checks in [inspection status](SCANNING.md#diagnostics).

<a id="privacy"></a>

## Privacy and excluded files

Recognized private keys and credentials are masked in reports, but this is not a
universal secret scanner. Source snippets, paths and public certificate details
may remain. Review reports before sharing them.

Git ignore settings are respected. Default exclusions include `.git`,
`node_modules`, `target`, `dist`, `docs`, Markdown and known image, archive and
binary formats. The default file-size limit is 2 MiB; CLI and folder-picker
settings accept 1–64 MiB. See [file-size limits](SCANNING.md#file-size-limit) and
[file selection](SCANNING.md).

Tests are included by default. Use `--exclude-tests` or **Exclude test code** to
omit them, and `.pqcignore` for custom paths. Symbolic-link targets are not read.
Excluded and unsupported inputs mean that zero findings do not prove safety.

<a id="native-dependencies"></a>

## PHP, .NET and native dependency inputs

The following subsets are inspected offline. No PHP, dotnet/MSBuild, package manager, compiler, CMake or Conan Python runs. No dependency graph or installed inventory is inferred.

| Input | Retained information and limits |
|---|---|
| composer.json / composer.lock | Literal require/dev packages, PHP/extension/native platform requirements and config.platform or lock platform-overrides. Emulation (including false/disabled extensions) is separate from deployed versions. Lock Git/path/archive/custom records remain unverified, with no registry purl or resolved version. |
| .csproj / Directory.Packages.props / Directory.Build.props / packages.config | Literal package declarations, target frameworks and legacy package claims. Only the nearest selected in-root Directory.Packages.props can supply an unconditioned literal central constraint; the project assertion remains inherited, not an effective build. Build props are independently inventoried. Conditions/properties/imports are not evaluated. |
| global.json / packages.lock.json | SDK declarations are not runtimes. NuGet lock versions1/2 retain requested and resolved claims and separate target-framework/runtime-ID bucket labels; future editions are Unsupported. IDs compare case-insensitively while source assertions stay separate. |
| vcpkg.json / vcpkg-configuration.json | Dependencies, literal constraints/features, overrides and registry baselines are declarations. Feature and baseline records have distinct ecosystems/provenance and no installed version or registry purl. Custom registry URLs are omitted. |
| conanfile.txt / conan.lock | Literal requires/tool/build/test requirements and lock0.5 references. Safe revisions remain reported claims; user/channel sources and build requirements stay unresolved. Future lock editions are Unsupported. Python recipes are not evaluated. |
| CMakeLists.txt / *.cmake | Literal find_package, target_link_libraries and FetchContent_Declare names only, always incomplete lexical references. Comments/quoted command text and variables do not create package facts. URLs, script evaluation and downloads are excluded. |

The native dependency inventory keeps its existing six attributes. `composer`, `nuget`, `php-platform`, `dotnet-sdk`, `dotnet-framework`, `vcpkg`, `vcpkg-feature`, `vcpkg-baseline`, `conan` and `cmake` distinguish source assertions and environment/configuration references; the latter are not installed components. Feature identities use `port/feature`; baseline names are fixed registry-kind/ordinal labels rather than URLs. These records retain source file and ordinal, without a graph edge or inferred relationship between different assertions. Registry purls are generated only for representable Composer declarations and eligible NuGet records; native build references have none. Selected-file, no-symlink, in-root and resource bounds also apply to central context.

The PHP `PHP_PQC_API` rule uses the `quantum-safe` / `info` reference label;
other new source rules use `quantum-uncertain`. This label marks a recognized source
reference, not verified deployment or complete protection. Unknown constructions
remain unknown in migration assessment, and missing support cannot become ready.

New info/quantum-uncertain dependency rules add findings for these supported crypto packages. Existing broad package rules may also match new inputs; a historical risk label does not identify the selected algorithm. This can change finding counts and fail thresholds on newly covered files. Missing versions, active providers, peer support and runtime capabilities remain unknown. Guidance distinguishes PHP extensions/native libraries, .NET target/package/OS/backend and C/C++ build/provider prerequisites; exact OQS wrapper identities remain separate. API availability does not establish PQC TLS or require blanket replacement of symmetric AES.

## Additional source references

Finite wolfCrypt RSA and libsecp256k1 ECDSA calls, Botan PK_Signer/PK_Verifier, OpenSSL X509_verify_cert, .NET RSA signature formatters, SignedXml and X509Chain requests are recognized. PHP Firebase JWT and XMLSecLibs requests retain unknown algorithms where keys or selectors are unavailable. A same-function phpseclib private-key type check can retain an unresolved signature relationship, without asserting that the check dominates every path.

C++ nested direct initializers are recovered only when a reserved nullptr argument rules out a parameter declaration. Possible declarations remain unresolved. C# identical simple/type names require the identical canonical declared type, rather than merely the same algorithm family.

SOURCE_IMPLEMENTATION_REFERENCE records finite named entry definitions, including explicit .c.inc/.cc.inc/.cpp.inc fragments. SOURCE_CONSTRUCTION_REFERENCE records exact conditional hybrid guards or canonical composite-generator components. These records do not establish actual calls, compiled branches, conformance or combined resistance.
