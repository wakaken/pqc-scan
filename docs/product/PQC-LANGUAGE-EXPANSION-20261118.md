# PHP, C#, C and C++ expansion research

Research date: **2026-11-18 (Asia/Tokyo)**. Status: research and implementation proposal, **not an approved implementation specification**.

Baseline: fetched `origin/develop`, `3f2bc6e6209e1f2813caac8077c8d7a7198832cc`; branch `codex/pqc-language-research`. The original checkout's uncommitted work was not copied or modified. Only this research document is added. No task creation, product source/rule/dependency changes, upstream library installation, cryptographic execution or protocol interoperability tests were performed.

## Scope and method

The requested three priorities are PHP, C#, and the C/C++ family. Kotlin and Swift are explicitly excluded. C and C++ need separate grammars: adding these families would ordinarily take seven source languages to eleven, rather than ten.

This is a broad inventory of relevant public language APIs, established crypto libraries, OQS bindings, selected emerging implementations, and application/framework integration points. Official documentation, release notes, public headers and maintainer repositories were inspected. It is not proof that every package, commercial SDK, downstream patch or private framework has been discovered. Unconfirmed support is recorded as unconfirmed, never as absence of support. Version claims below describe documented capability, not a recommendation to deploy a particular release or proof of a deployment's capability. Mutable upstream branches should be pinned when implementation fixtures are prepared.

Evidence levels must remain distinct:

1. A dependency, header, type or algorithm name is referenced.
2. A recognized operation requests a specific algorithm and parameter set.
3. The deployment has a compatible library/provider/OS and build configuration.
4. An operation succeeds, or a protocol actually negotiates the algorithm.

Repository inspection can support the first two, and some static prerequisites for the third. It cannot establish the fourth. KEM, signatures, certificate signatures and TLS authentication/key establishment are separate capabilities. ML-KEM is not a drop-in arbitrary-message encryption API. Likewise, an AES/HMAC/password-hashing call is not an RSA/ECC migration finding.

## Principal findings

- All three target families have real PQC implementation paths. The question is how to recognize them accurately, rather than whether PQC exists at all.
- PHP is at a release boundary: PHP 8.6 is scheduled for **2026-11-19**, after this research date. Its new Sodium KEM functions must be labelled pre-release here. Independently installable PHP PQC extensions and libraries already exist.
- .NET 10 documents ML-KEM, ML-DSA and SLH-DSA. .NET 11 adds further APIs, including explicitly experimental composite KEM/HPKE facilities. Runtime/API availability must not be confused with backend availability or ASP.NET TLS configuration.
- OpenSSL 3.5 already supplies standardized PQC in its default provider. OQS is important for discovery and research but is not required for all OpenSSL PQC use. OQS explicitly describes its integrations as prototypes.
- Old comparison tables are unreliable: libsodium 1.0.22 adds KEMs; LibreSSL has PQC additions; the official OQS .NET wrapper is deprecated; PQClean is archived.

## PHP inventory

| Component | Documented status at research date | Proposed scanner treatment |
|---|---|---|
| PHP OpenSSL extension | Existing public-key/signature/TLS interface; linked OpenSSL/provider matters. The inspected PHP 8.5 public stub has no KEM encapsulation/decapsulation API. | Detect actual calls and explicit key/algorithm arguments. Treat PQC key import/signature interoperability as unverified until tested against exact PHP/OpenSSL versions. Never infer PHP KEM support from OpenSSL version alone. |
| PHP 8.6 Sodium extension | Scheduled release; X-Wing via `sodium_crypto_kem_*`, ML-KEM-768 via `sodium_crypto_kem_mlkem768_*`; requires libsodium >=1.0.22. | Detect both families with separate hybrid/pure identities. Record PHP release status and native-library prerequisite. |
| ParagonIE `ext-pqcrypto` | RustCrypto-backed extension exposing ML-KEM, ML-DSA, SLH-DSA and X-Wing; README requires PHP >=8.1. | Recognize `PQCrypto` qualified types, static key generation, bounded key receivers and operation calls; extension declarations are prerequisites, not use. |
| `paragonie/pqcrypto_compat` | Pure-PHP implementation with delegation to the extension; README documents ML-KEM, ML-DSA and X-Wing and says it has not been independently audited. | Recognize `ParagonIE\PQCrypto\Compat`, aliases and methods such as `mlkem768_keygen`, `mldsa44_sign`, `xwing_encaps`. Do not assume SLH-DSA parity merely because the native extension supports it. |
| `Muzosh/liboqs-php` | Third-party OQS-listed PHP wrapper. | Inventory and explicit wrapper/API references; pin wrapper version and native ABI before adding detailed operation semantics. |
| `mir-evgenii/liboqs-php` | Third-party FFI wrapper; ML-KEM/ML-DSA and other liboqs algorithms; explicitly experimental/educational. | Recognize Composer identity, `OqsKem`/`OqsSig`, literal algorithm selection and FFI dependency. Its custom hybrid-encryption construction is not an independently validated protocol. |
| phpseclib | Established PHP crypto/SSH library. Inspected current README does not establish standardized PQC support. | Detect documented classical RSA/EC/DH uses and inventory dependency. PQC support remains unconfirmed; do not report the library categorically incapable. |
| Laravel | Documented encrypter uses AES-128/AES-256 plus MAC. | Framework identification/context, not automatic quantum-vulnerable classification. Public-key operations in application/packages need their own evidence. |
| Symfony | Security configuration documents password hashing; an announced 8.2 KeyManagement component concerns envelope encryption/backends. | Inspect actual algorithms/backend selection. Neither a password hasher nor a KMS/envelope abstraction proves PQC. Release/version and PQC integration remain separate questions. |
| WordPress and plugins | No core PQC capability was established in this survey; plugin-specific implementations are an open ecosystem. | Scan PHP/plugin code and manifests using the same bounded rules; no blanket “WordPress PQC supported/unsupported” rule. |

Primary sources: [PHP 8.6 release status](https://www.php.net/releases/8.6/en.php), [8.6 new functions and prerequisites](https://www.php.net/manual/en/migration86.new-functions.php), [PHP 8.5 OpenSSL public API](https://github.com/php/php-src/blob/PHP-8.5/ext/openssl/openssl.stub.php), [OpenSSL function index](https://www.php.net/manual/en/ref.openssl.php), [ext-pqcrypto](https://github.com/paragonie/ext-pqcrypto), [pqcrypto_compat](https://github.com/paragonie/pqcrypto_compat), [ParagonIE release context](https://www.paragonie.com/blog/2026/04/post-quantum-cryptography-for-php-community), [OQS wrapper catalogue](https://openquantumsafe.org/liboqs/wrappers.html), [Muzosh wrapper](https://github.com/Muzosh/liboqs-php), [mir-evgenii wrapper](https://github.com/mir-evgenii/liboqs-php), [phpseclib](https://github.com/phpseclib/phpseclib), [Laravel encryption](https://laravel.com/framework/docs/encryption), [Symfony security](https://symfony.com/doc/current/reference/configuration/security.html), [Symfony KeyManagement announcement](https://symfony.com/blog/new-in-symfony-8-2-keymanagement-component).

### PHP proposed initial coverage

- Source: `.php`, including ordinary mixed HTML/PHP; separately evaluate historical `.phtml`/`.inc` routing. Do not advertise arbitrary Blade/Twig template execution or generated PHP analysis.
- Classical operations: OpenSSL RSA/EC key generation, signing/verification, RSA encryption/decryption, DH/ECDH derivation; Sodium Ed25519 signatures, X25519/key-exchange/public-key box operations; phpseclib RSA/EC/DH calls with recognized imports/types.
- PQC references and operations: both planned Sodium KEM families, ext-pqcrypto, pqcrypto_compat; OQS wrappers initially as dependency/reference observations unless exact APIs are frozen.
- Dependencies: `composer.json` and `composer.lock`; separate declared constraints, lock versions, development requirements, platform packages (`php`, `ext-openssl`, `ext-sodium`, `ext-pqcrypto`) and `config.platform` emulation. A platform constraint is not the loaded extension version. PHP/native requirements must not be presented as installed-state proof.
- Resolution: bounded `use`/`use function` aliases, fully qualified calls, namespace fallback and shadowing. User-defined namesakes, comments, documentation strings, variable function names, reflection, FFI and arbitrary containers are not resolved by naive API-name matching.
- Runtime guidance: PHP version, extension availability, linked OpenSSL/libsodium, loaded provider and TLS termination location are independent checks. `openssl_pkey_derive` is DH/ECDH, not a generic KEM substitute.

Composer semantics: [basic usage and lock files](https://getcomposer.org/doc/01-basic-usage.md). Derivation API: [PHP manual](https://www.php.net/openssl-pkey-derive).

## C# inventory

| Component | Documented PQC capability/status | Proposed scanner treatment |
|---|---|---|
| .NET 10 `System.Security.Cryptography` | `MLKem`, `MLDsa`, `SlhDsa`; composite signature facilities also documented. These new core types do not derive from `AsymmetricAlgorithm`. | Recognize qualified types, static generation/import and matching receiver operations; do not restrict discovery to `AsymmetricAlgorithm` descendants. |
| .NET 11 | GA documentation adds `CompositeMLKem` and `Hpke` marked experimental (`SYSLIB5006`, `SYSLIB5009`); platform/combination support checks required. | Inventory explicit references, preserve experimental status. An HPKE reference alone does not prove that its selected KEM is PQC. |
| `Microsoft.Bcl.Cryptography` | Provides PQC facilities for older target frameworks as documented by Microsoft; native backend prerequisites remain. | Inspect package identity/version and target framework separately. `net48` alone cannot establish absence of PQC. |
| Bouncy Castle C# | Standardized PQC primitives; 2.7.0 adds hybrid ML-KEM TLS 1.3 key exchange and ML-DSA/SLH-DSA authentication. Some protocol definitions are drafts. | Recognize `Org.BouncyCastle` types and selected parameter sets, plus package version; distinguish BC's own TLS stack from OS-backed `SslStream`. |
| Official OQS `liboqs-dotnet` | Deprecated; repository archived 2025-01-06; OQS recommends against its use. | Detect legacy dependency/API references with maintenance guidance. Do not recommend it as the default migration route. |
| OpenForge `liboqs-dotnet` | Independent community wrapper (`OpenForge.Cryptography.LibOqs`), with modular native packages. | Separate its package identity from the discontinued official wrapper; inspect native package/version/architecture. Existence is established, audit and deployment suitability are not. |
| Other community wrappers | `filipw/maybe-liboqs-dotnet` also exists. | Catalogue/reference recognition; full wrapper-by-wrapper API and native ABI certification is outside this source survey. |
| NSec | Public API list inspected documents X25519 and Ed25519, not a PQC KEM/signature API. | Detect relevant classical references; libsodium's newer capability does not automatically flow through an older/different .NET wrapper API. |
| ASP.NET Core / Kestrel / `HttpClient` / `SslStream` | Runtime/native TLS integration rather than a separate application-level PQC primitive implementation. | Backend/OS/certificate/group context, with actual handshake verification separate. TLS 1.3 and `net10.0` are insufficient PQC evidence. |
| ASP.NET Core Data Protection | Default payload protection documented as AES-256-CBC + HMACSHA256. Key wrapping may involve asymmetric certificates or external services. | Do not classify ordinary Data Protection calls as RSA use. Inspect certificate wrapping separately. |
| IdentityModel/JWT and other framework packages | General-purpose signing abstractions may use classical or extensible providers; this survey did not establish universal built-in PQC JOSE support. | Detect explicit RSA/ECDSA key types and selected algorithms. Do not infer ML-DSA interoperability from .NET's primitive API alone. |

Primary sources: [.NET 10 libraries](https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10/libraries), [.NET 11 libraries](https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-11/libraries), [platform/backend matrix](https://learn.microsoft.com/en-us/dotnet/standard/security/cross-platform-cryptography), [BC C# release notes](https://www.bouncycastle.org/download/bouncy-castle-c/), [BC 2.7 announcement](https://www.bouncycastle.org/resources/new-release-bouncy-castle-c-2-7-0/), [deprecated OQS wrapper](https://github.com/open-quantum-safe/liboqs-dotnet), [OpenForge wrapper](https://github.com/openforge-sh/liboqs-dotnet), [additional wrapper](https://github.com/filipw/maybe-liboqs-dotnet), [NSec APIs](https://nsec.rocks/), [Kestrel endpoints](https://learn.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel/endpoints?view=aspnetcore-10.0), [Data Protection payload format](https://learn.microsoft.com/en-us/aspnet/core/security/data-protection/implementation/authenticated-encryption-details?view=aspnetcore-10.0).

Microsoft's cross-platform table inspected during research still lists Windows Insider restrictions for several native PQC operations, OpenSSL >=3.5 for Linux facilities, and unavailable combinations on other platforms. Preserve that documented limitation; do not infer that every Windows release or every .NET 11 installation supports them. `IsSupported`/`IsAlgorithmSupported` is a runtime check, not something a repository scan can execute or assume true.

### C# proposed initial coverage

- Source: `.cs`; explicitly exclude C# embedded in Razor `.cshtml`/`.razor`, generated assemblies and arbitrary preprocessor evaluation from the first syntax contract.
- Classical: `RSA`, `RSACryptoServiceProvider`, `RSACng`, `ECDsa`, `ECDiffieHellman`, `DSA`, and corresponding Bouncy Castle types/operations. Distinguish key construction, signing, verification, encryption and key agreement when evidence permits.
- PQC: standard .NET types/parameter selections and selected BC primitives; obsolete OQS and community wrappers as separately identified dependency/reference signals.
- Resolve bounded `using`, `global using`, aliases, fully qualified names, local type/variable shadowing and receiver types. `MLDsa` as a user-defined class must not become a built-in PQC observation. Partial classes, reflection, source generators and cross-file type inference stay unresolved unless implemented explicitly.
- Dependencies: literal `PackageReference` and target frameworks in `.csproj`, `Directory.Packages.props`/`Directory.Build.props` with bounded local context, legacy `packages.config`, `packages.lock.json`, SDK context from `global.json`. MSBuild conditions, imports, property expansion and multi-targeting must retain uncertainty; never run MSBuild or NuGet during offline scanning.
- Reuse safe XML parsing limits and repository-bound relative reads; report declared/package lock data independently of the actually loaded assembly/native library.

NuGet semantics: [PackageReference and lock files](https://learn.microsoft.com/en-us/nuget/consume-packages/package-references-in-project-files).

## C/C++ inventory

C/C++ language facilities do not supply a general standard PQC API. Actual capability comes from libraries, OS APIs and protocol stacks. The following inventory covers both mainstream integration targets and recognizable lower-level implementations.

| Component | PQC evidence/status | Detection proposal and qualification |
|---|---|---|
| OpenSSL >=3.5 | ML-KEM, ML-DSA, SLH-DSA in default provider; hybrid TLS groups including `X25519MLKEM768`. | Highest priority: explicit EVP algorithm selection, KEM/signature operations and TLS group/provider configuration. Primitive support does not mean every signature family is supported in every TLS version/path. |
| liboqs | C KEM/signature API; broad algorithm catalogue; official site lists 0.16.0 as current. | `OQS_KEM_new`, `OQS_SIG_new`, parameter constants, keypair/encaps/decaps/sign/verify operations and build gates. Algorithm availability is build-dependent. |
| liboqs-cpp | Official C++ bindings to liboqs. | Qualified `oqs` wrappers and method calls; separate wrapper from native version. |
| oqs-provider | OpenSSL provider; TLS/X.509/CMS integrations and experimental/nonstandard combinations. | Provider load/configuration and algorithm selection are separate evidence. On OpenSSL >=3.5, algorithms supplied natively are disabled in oqs-provider; provider absence does not prove PQC absence. |
| Botan 3 | ML-KEM, ML-DSA, SLH-DSA plus additional PQ and stateful hash-based families; release history distinguishes old Kyber/Dilithium variants. | Qualified Botan key types, modes/parameters, `PK_KEM_*`, signer/verifier and selected TLS policy. C++ priority after OpenSSL/OQS. |
| wolfSSL / wolfCrypt | Native PQC implementations; 2026 announcement documents retirement of its liboqs integration in favour of native coverage. | `wc_*` operations, selected algorithms and build configuration; do not require liboqs for positive discovery. wolfProvider is an additional OpenSSL-facing route. |
| AWS-LC | Public EVP header documents KEM operations and ML-DSA; OpenSSL-like C API. | Distinguish library provenance even when function names resemble OpenSSL. Recognize selected EVP key types and operation paths. |
| BoringSSL | Public ML-KEM and ML-DSA headers; direct APIs such as `MLKEM768_generate_key` and corresponding encaps/decaps operations. | Header/provenance plus exact API families; no stable universal OpenSSL ABI assumption. |
| LibreSSL | ChangeLog places the ML-KEM public API under 4.2.0 and preferred `X25519MLKEM768` under 4.4.0. The experimental ML-DSA API is under **4.5.0 in development**, not a released capability. | Do not reuse an outdated “LibreSSL has no PQC” assumption, or promote its unreleased ML-DSA entry to stable support. |
| libsodium 1.0.22 | ML-KEM-768 and X-Wing via `crypto_kem_mlkem768_*` and `crypto_kem_*`. | Direct C API detection; Ed25519/box/key-exchange calls remain distinct classical families. KEM support does not establish a PQC signature API. |
| GnuTLS | 3.8.9 release describes ML-KEM hybrids and experimental ML-DSA with leancrypto/liboqs build prerequisites; later 3.8.x releases exist. | Recognize selected public-key/group constants and priority strings; pin version/backend matrix rather than treating all builds equivalently. |
| Mbed TLS / TF-PSA-Crypto | Maintainer describes ML-DSA-87 at internal driver-dispatch level in TF-PSA-Crypto 1.2.0, not yet general-purpose support. ML-KEM/general API work is on roadmap. | Record internal/experimental/backend scope. Do not claim complete public PSA or TLS PQC support from this evidence. Downstream drivers need separate attribution. |
| Oberon / Nordic PSA drivers | Official downstream changelog describes ML-KEM/ML-DSA and PSA PQC extension support. | Inventory downstream configuration separately from upstream Mbed TLS capability. |
| SymCrypt / Windows CNG | SymCrypt public header contains ML-KEM and ML-DSA APIs; Microsoft documents native platform restrictions. | Recognize `SymCryptMlKem*`/`SymCryptMlDsa*`; OS CNG calls require algorithm/provider identification. Source availability does not certify Windows API availability. |
| libgcrypt | 1.11 announcement describes Kyber against an initial public draft; 1.12 release notes include subsequent work/fixes. | Preserve draft/standard distinction. Do not automatically normalize every historical Kyber encoding to FIPS 203 ML-KEM. |
| Mozilla NSS | TLS header contains PQ hybrid group identifiers. | Dependency/group references first; public primitive and release-version coverage need pinning before stronger rules. |
| PQ Code Package | `mlkem-native`, `mldsa-native`, `slhdsa-c` offer native algorithm implementations. | Important for vendored source with no package manifest; match namespaced headers/symbols and parameters, not arbitrary `crypto_kem_*` names alone. |
| PQClean | Portable implementations; repository archived 2026-08-04. | Detect existing vendored `PQCLEAN_*` implementations and maintenance context. Do not present as a currently maintained default dependency. |
| leancrypto | ML-KEM, ML-DSA, SLH-DSA and additional/hybrid constructions. | Detect explicit API/algorithm identities and preserve construction provenance. |
| PQMagic | Native PQ implementations including ML-KEM in project documentation. | Secondary inventory/reference target; exact algorithms, versions and API contract require fixture pinning before detailed rules. |
| Crypto++ upstream | Inspected upstream README does not establish ML-KEM/ML-DSA/SLH-DSA support. Independent forks exist and must not be conflated with upstream. | Classical crypto detection and dependency identity; upstream PQC status unconfirmed by this survey, not a universal absence verdict. |

Primary sources:

- [OpenSSL 3.5 release history](https://github.com/openssl/openssl/blob/openssl-3.5/NEWS.md), [TLS groups](https://docs.openssl.org/3.5/man3/SSL_CTX_set1_curves/), [SSL_CONF group defaults](https://docs.openssl.org/3.5/man3/SSL_CONF_cmd/).
- [liboqs overview](https://openquantumsafe.org/liboqs/), [security/prototype limitations](https://openquantumsafe.org/liboqs/security.html), [C++ bindings](https://github.com/open-quantum-safe/liboqs-cpp), [oqs-provider native-algorithm exclusions](https://github.com/open-quantum-safe/oqs-provider).
- [Botan public-key APIs](https://botan.randombit.net/handbook/api_ref/pubkey.html), [Botan releases](https://botan.randombit.net/news.html), [wolfCrypt native implementation transition](https://www.wolfssl.com/wolfcrypt-retires-liboqs-as-native-post-quantum-support-expands/), [wolfProvider PQC](https://www.wolfssl.com/documentation/manuals/wolfprovider/chapter11.html).
- [AWS-LC EVP header](https://github.com/aws/aws-lc/blob/main/include/openssl/evp.h), [BoringSSL ML-KEM header](https://github.com/google/boringssl/blob/main/include/openssl/mlkem.h), [BoringSSL ML-DSA header](https://github.com/google/boringssl/blob/main/include/openssl/mldsa.h), [LibreSSL ChangeLog](https://github.com/libressl/portable/blob/master/ChangeLog), [LibreSSL released versions](https://www.libressl.org/releases.html), [libsodium 1.0.22 release](https://github.com/jedisct1/libsodium/releases/tag/1.0.22-RELEASE).
- [GnuTLS 3.8.9 announcement](https://lists.gnutls.org/pipermail/gnutls-help/2025-February/004875.html), [current release list](https://www.gnutls.org/news.html), [Mbed TLS maintainer statement](https://github.com/Mbed-TLS/mbedtls/issues/10834), [Mbed TLS roadmap](https://mbed-tls.readthedocs.io/en/latest/project/roadmap/), [Oberon downstream changelog](https://github.com/nrfconnect/sdk-oberon-psa-crypto/blob/main/CHANGELOG.md).
- [SymCrypt header](https://github.com/microsoft/SymCrypt/blob/main/inc/symcrypt.h), [libgcrypt 1.11](https://lists.gnupg.org/pipermail/gnupg-announce/2024q2/000483.html), [libgcrypt 1.12](https://lists.gnupg.org/pipermail/gnupg-announce/2026q1/000502.html), [NSS TLS header](https://github.com/mozilla/nss/blob/master/lib/ssl/sslt.h).
- [mlkem-native](https://github.com/pq-code-package/mlkem-native), [mldsa-native](https://github.com/pq-code-package/mldsa-native), [slhdsa-c](https://github.com/pq-code-package/slhdsa-c), [PQClean archive](https://github.com/PQClean/PQClean), [leancrypto](https://github.com/smuellerDD/leancrypto), [PQMagic](https://github.com/pqcrypto-cn/PQMagic/blob/main/README_EN.md), [Crypto++ upstream](https://github.com/weidai11/cryptopp).

### C/C++ framework and transport boundaries

- Boost.Asio/Beast: SSL support uses OpenSSL; `native_handle()` exposes the underlying context. Detect native OpenSSL configuration when visible. A Boost dependency is not PQC evidence. [Boost.Asio SSL documentation](https://www.boost.org/doc/libs/develop/doc/html/boost_asio/overview/ssl.html).
- Qt: SSL support is backend-dependent. Inspect selected backend/native configuration; do not imply that every Qt deployment has OpenSSL's capability. [Qt SSL documentation](https://doc.qt.io/qt-6/ssl.html).
- libcurl, including PHP curl bindings: TLS backend, groups and cipher suites are different dimensions. `CURLOPT_TLS13_CIPHERS` does not select the TLS key-exchange group. Detect relevant group/native-context configuration and preserve backend uncertainty. [curl cipher configuration](https://curl.se/docs/ssl-ciphers.html).
- nginx/Apache/Envoy/HAProxy and similar servers may terminate TLS outside the application. Reuse existing configuration inspection where applicable; adding C++ source support does not automatically prove an application's externally negotiated TLS capability.
- Cloud/HSM services are another route available through these languages' SDKs. AWS KMS documents ML-DSA and ML-KEM keys. Explicit key-spec/operation arguments are useful future language rules; an SDK dependency or remote key identifier is insufficient. This study does not certify all cloud SDK versions, regional availability or private HSM modules. [AWS KMS PQ keys](https://docs.aws.amazon.com/kms/latest/developerguide/pqkeys.html).

### C/C++ proposed initial coverage

- Use separate pinned C and C++ grammars for `.c`, `.cc`, `.cpp`, `.cxx`, `.hpp`, `.hh`, `.hxx`; define a safe policy for ambiguous `.h` and uppercase `.C`. Existing extension lowercasing must not silently route C++ `.C` as C. Report ambiguity/partial inspection rather than claiming every header was parsed correctly.
- OpenSSL: low-level RSA/DSA/DH/EC operations, explicit `EVP_PKEY_CTX_new_from_name`/algorithm selection, `EVP_PKEY_encapsulate`/decapsulate, signature APIs, `OSSL_PROVIDER_load`, property queries and `SSL_CTX_set1_groups_list` equivalents. Generic EVP calls without a known key/algorithm remain unresolved.
- OQS: C and C++ wrapper construction, literal algorithm/parameter selection and bounded receiver operations. Separate standardized ML names, pre-standard Kyber/Dilithium/SPHINCS+ names, experimental families, and hybrid combinations.
- Next library packs: Botan, wolfCrypt, libsodium; then AWS-LC/BoringSSL/LibreSSL and GnuTLS/PSA/native implementations. This ordering proposes implementation sequencing only; it does not declare the other surveyed libraries supported by the initial release.
- Dependencies/build inputs: first parse literal `vcpkg.json`, `vcpkg-configuration.json`, `conanfile.txt`, and supported `conan.lock` data; inspect bounded literal `find_package`/link/fetch declarations in CMake as declaration references only. `conanfile.py`, arbitrary CMake, configure scripts and build generators must not be executed. Version ranges/baselines, Git/path sources and resolved records must remain distinct.
- Vendored code: identify known headers/namespaced symbols with bounded work; absence of a package manifest does not imply absence of cryptography. Do not follow include paths outside the selected root, expand arbitrary macros, run a compiler or assume a preprocessor branch is active.

## Fit with the current scanner

The inspected baseline has seven explicit language routes in `tree_sitter_detector::resolve_language`; a shared semantic-node selector and scope matcher; per-language local facts under `source_facts`; dependency parsing modules; runtime/dependency profiling and recommendation dispatch in core; and generic YAML rule loading. Existing regex rules already recognize selected OpenSSL RSA references. Thus C/C++ currently has limited lexical observations, not dedicated language-level semantic support.

Relevant local sources: [AST routing and node selection](../../crates/pqc_scan_detectors/src/tree_sitter_detector.rs), [local source facts](../../crates/pqc_scan_detectors/src/source_facts.rs), [dependency detector](../../crates/pqc_scan_detectors/src/dependency_detector.rs), [dependency parsers](../../crates/pqc_scan_detectors/src/dependency_detector/parse.rs), [runtime and recommendations](../../crates/pqc_scan_core/src/recommendation.rs), [existing OpenSSL rules](../../rules/default/crypto_api.yml), [coverage contract](../user/COVERAGE.md), [architecture](../architecture.md).

Implementation surfaces, to be changed only after authorization:

1. Pinned grammar dependencies, extension routing and language-appropriate semantic nodes. Audit parser/Rust/toolchain compatibility, provenance, licenses, build/packaging size and syntax-recovery behavior.
2. Bounded language-specific import/name/receiver facts, following existing budget/status conventions. Generic matching remains a fallback for custom rules; new default-rule semantics must not silently attach to arbitrary custom patterns.
3. New language/library-scoped rules with unique IDs. Define precise positive and negative examples and evidence strength before deciding severity/confidence. Existing regex and AST overlap requires identity/deduplication analysis; do not remove legacy IDs or silently change historical findings.
4. Bounded manifest/lock parsing with provenance and conditionality. Do not add a general build/package-manager evaluator.
5. Language/runtime recommendation routing. The public `RuntimeProfile` currently has explicit fields for existing runtimes: adding mandatory fields would affect external struct literals. Assess public Rust API compatibility; prefer internal supplementary context or an explicitly agreed additive API rather than casually changing the struct contract.
6. English/Japanese user coverage and rule reference updates; stable report/CLI schema and risk-enum preservation. New detector coverage must not turn an unresolved/unsupported inspection into a successful complete verdict.

## Approaches considered

| Approach | Benefit | Cost/limitation | Recommendation |
|---|---|---|---|
| Add regex/API names only | Fast initial reference discovery | Weak namespace/alias/receiver handling; comments and namesakes can match; does not justify full language support | Supplement only |
| Extend Tree-sitter + bounded local facts + structured dependency subsets | Matches current architecture; offline; explainable source evidence | Requires four grammars and language-specific tests; cannot resolve arbitrary programs | **Recommended** |
| Add Roslyn/Clang/PHP runtime analyzers | Potentially deeper typing/build context | Runtime/toolchain dependencies, project execution risks, packaging and compatibility costs | Separate future feasibility study, not initial scope |

The recommended approach changes detection, not cryptographic implementation. No PQC library needs to be linked into pqc-scan merely to identify its use.

Upstream grammars exist for all four languages: [PHP](https://github.com/tree-sitter/tree-sitter-php), [C#](https://github.com/tree-sitter/tree-sitter-c-sharp), [C](https://github.com/tree-sitter/tree-sitter-c), [C++](https://github.com/tree-sitter/tree-sitter-cpp). Their existence establishes feasibility of the parsing approach, not compatibility with this repository's pinned Tree-sitter ABI or complete support for every source dialect. Compatibility/build checks are an implementation prerequisite, not a result of this research.

## Proposed delivery sequence and acceptance gates

This sequence is a proposal within the three requested families, not new tasks or agreed acceptance criteria. No calendar estimate is justified until grammar compatibility and corpus size are measured.

| Sequence | Deliverable | Acceptance evidence required before implementation can be called complete |
|---|---|---|
| 1. Detection contract | Per-language file policy, initial library/API matrix, parameter/operation semantics and unresolved cases | Approved bounded scope; no blanket “PQC ready” verdict; standard/draft/hybrid/experimental distinctions |
| 2. Parsing and facts | PHP, C#, C, C++ grammar adapters and local name/receiver resolution | Positive and negative syntax corpus; mixed PHP; modern C#; `.h`/`.C`; parse errors, Unicode/wide text, budgets and exact positions |
| 3. Initial library packs | PHP OpenSSL/Sodium/ParagonIE; C# standard .NET/BC; C/C++ OpenSSL/OQS | Per API family tests for call/reference, aliases, shadowing, parameters, signatures vs KEM, same-line occurrences and false namesakes |
| 4. Dependencies and guidance | Composer, NuGet and bounded C/C++ input subsets; migration guidance | Declared vs locked vs conditional/unresolved evidence; no build execution; backend/platform/version prerequisites preserved |
| 5. Integration and documentation | End-to-end reports and language documentation | Existing seven-language regression corpus, stable output contracts, masking canaries, rule validation and required reviews/checks |

Required test dimensions include: algorithm-name strings used as help text; unrelated `RSA`/`MLDsa` classes; namespace aliases and shadowing; dynamic/unresolved selections; composite/HPKE selections with classical-only KEMs; disabled preprocessor branches; legacy-vs-standard parameter names; provider overrides; missing/ambiguous native dependencies; malformed manifests/XML and bounded relative imports; exact source spans/deduplication; synthetic private-key and credential masking; multi-target and multi-language repositories.

Use existing risk/category vocabulary where it fits. Treat dependency presence, algorithm reference and operation reference as different evidence without changing serialized enums as part of this proposal. Policy for deprecated wrapper observations and experimental constructions must be specified before coding; do not invent numeric severity/confidence merely from a package name.

Before an actual implementation is finished, repository rules require relevant targeted checks, workspace tests, formatting, Clippy, applicable builds/features, rule validation, an independent implementation review followed by the required test review, final-source verification, and paired user documentation. Those checks have **not** been run for this research-only change and no implementation quality or performance result is claimed.

## Remaining decisions and research limits

- Decide whether first-release support includes full ParagonIE local receiver analysis or reference-only recognition; whether OQS PHP/C# community wrappers are reference-only; and which C/C++ package/build subsets are acceptance requirements.
- Freeze source versions and exact public symbols for each initial library and grammar. The inventory establishes availability; it does not certify all ABI/encoding combinations or source-level binding accuracy.
- Test PHP OpenSSL PQC key import/signature behavior before recommending that path. The planned Sodium API is more directly documented but still pre-release on the research date.
- Reconcile released LibreSSL/GnuTLS/PSA versions and experimental branches when authoring version-specific capability rules. Mutable headers and ChangeLogs are discovery evidence, not universal released-version guarantees.
- Audit/validation/FIPS claims are separate from implementing a FIPS-named algorithm. This study does not grant security certification to any package.
- Commercial libraries, all HSM/cloud vendors, every framework plugin and custom/vendored forks are not exhaustively certified. No missing search result is treated as proof of unsupported cryptography.

## Research verification

The baseline was fetched and a clean isolated worktree/branch created from the recorded SHA. Repository reads confirmed the integration points and the existing limited OpenSSL lexical rules. External official pages and selected public headers were inspected; failed initial URLs were replaced with canonical repositories/headers where possible. The research document's relative file links and whitespace are checked separately. There are no new executable examples or changes to product behavior, and no Rust/library interoperability tests are claimed.
