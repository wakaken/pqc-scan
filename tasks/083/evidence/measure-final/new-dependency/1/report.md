# PQC Migration Scan Report

Generated: 2026-11-24 18:36:26 +09:00 (UTC+09:00)

Findings to review (items): 14 / Informational findings (items): 28 / Files inspected: 25

|Severity|Count (all findings)|
|---|---:|
|info|28|
|medium|14|

## Risk candidate index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[DEP_BOUNCYCASTLE](#rule-0) BouncyCastle dependency detected.|medium|quantum-uncertain|6|6|
|[DEP_OPENSSL](#rule-1) OpenSSL dependency detected.|medium|quantum-uncertain|8|5|

## Informational inventory index

|Rule / description|Severity|Risk|Hits|Files|
|---|---|---|---:|---:|
|[DEP_NATIVE_CRYPTO](#rule-2) C/C++ cryptographic dependency or literal build reference; effective configuration is unknown.|info|quantum-uncertain|11|5|
|[DEP_NUGET_CRYPTO](#rule-3) NuGet cryptographic package assertion; wrapper ownership and runtime support require verification.|info|quantum-uncertain|7|7|
|[DEP_PHP_CRYPTO](#rule-4) PHP cryptographic dependency or platform declaration; installed capability is unknown.|info|quantum-uncertain|10|5|

<a id="rule-0"></a>

## DEP_BOUNCYCASTLE

BouncyCastle dependency detected.

Check: Verify BouncyCastle PQC support status.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-0"></a>[nuget-central/Directory.Packages.props:1:1](#finding-0)|bouncyca***@2.7.0|Verify BouncyCastle PQC support status. / [Source](#source-0)|
|<a id="finding-1"></a>[nuget-conditional/sample.csproj:1:1](#finding-1)|bouncyca***nknown|Verify BouncyCastle PQC support status. / [Source](#source-1)|
|<a id="finding-2"></a>[nuget-legacy/packages.config:1:1](#finding-2)|bouncyca***@2.7.0|Verify BouncyCastle PQC support status. / [Source](#source-2)|
|<a id="finding-3"></a>[nuget-lock-1/packages.lock.json:5:1](#finding-3)|bouncyca***@2.7.0|Verify BouncyCastle PQC support status. / [Source](#source-3)|
|<a id="finding-4"></a>[nuget-lock-2/packages.lock.json:5:1](#finding-4)|bouncyca***@2.7.0|Verify BouncyCastle PQC support status. / [Source](#source-4)|
|<a id="finding-5"></a>[nuget-project/sample.csproj:1:1](#finding-5)|bouncyca***@2.7.0|Verify BouncyCastle PQC support status. / [Source](#source-5)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-0)
- [Track PQC readiness of dependent libraries](#action-1)

### nuget-central/Directory.Packages.props:1:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-0"></a>

### nuget-central/Directory.Packages.props:1:1

    > 1 [masked-sensitive-file-content]

### nuget-conditional/sample.csproj:1:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-1"></a>

### nuget-conditional/sample.csproj:1:1

    > 1 [masked-sensitive-file-content]

### nuget-legacy/packages.config:1:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-2"></a>

### nuget-legacy/packages.config:1:1

    > 1 [masked-sensitive-file-content]

### nuget-lock-1/packages.lock.json:5:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-3"></a>

### nuget-lock-1/packages.lock.json:5:1

    > 5 [masked-sensitive-file-content]

### nuget-lock-2/packages.lock.json:5:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-4"></a>

### nuget-lock-2/packages.lock.json:5:1

    > 5 [masked-sensitive-file-content]

### nuget-project/sample.csproj:1:1 — Matched rules

- `DEP_BOUNCYCASTLE` (medium): BouncyCastle dependency detected. — Verify BouncyCastle PQC support status.

<a id="source-5"></a>

### nuget-project/sample.csproj:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-1"></a>

## DEP_OPENSSL

OpenSSL dependency detected.

Check: Review the OpenSSL PQC roadmap.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-6"></a>[cmake-literals/CMakeLists.txt:1:1](#finding-6)|openssl@3.5|Review the OpenSSL PQC roadmap. / [Source](#source-6)|
|<a id="finding-7"></a>[cmake-literals/CMakeLists.txt:2:1](#finding-7)|openssl@unknown|Review the OpenSSL PQC roadmap. / [Source](#source-7)|
|<a id="finding-8"></a>[conan-lock/conan.lock:4:1](#finding-8)|openssl@***7890.1|Review the OpenSSL PQC roadmap. / [Source](#source-8)|
|<a id="finding-9"></a>[conan-text/conanfile.txt:2:1](#finding-9)|openssl@3.5.0|Review the OpenSSL PQC roadmap. / [Source](#source-9)|
|<a id="finding-10"></a>[vcpkg-config/vcpkg-configuration.json:12:1](#finding-10)|openssl@unknown|Review the OpenSSL PQC roadmap. / [Source](#source-10)|
|<a id="finding-11"></a>[vcpkg-manifest/vcpkg.json (line unavailable)](#finding-11)|openssl@unknown|Review the OpenSSL PQC roadmap. / [Source](#source-11)|
|<a id="finding-12"></a>[vcpkg-manifest/vcpkg.json (line unavailable)](#finding-12)|openssl@3.5.0|Review the OpenSSL PQC roadmap. / [Source](#source-12)|
|<a id="finding-13"></a>[vcpkg-manifest/vcpkg.json (line unavailable)](#finding-13)|liboqs/o***nknown|Review the OpenSSL PQC roadmap. / [Source](#source-13)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-2)
- [Track PQC readiness of dependent libraries](#action-1)

### cmake-literals/CMakeLists.txt:1:1 — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-6"></a>

### cmake-literals/CMakeLists.txt:1:1

    > 1 [masked-sensitive-file-content]

### cmake-literals/CMakeLists.txt:2:1 — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-7"></a>

### cmake-literals/CMakeLists.txt:2:1

    > 2 [masked-sensitive-file-content]

### conan-lock/conan.lock:4:1 — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-8"></a>

### conan-lock/conan.lock:4:1

    > 4 [masked-sensitive-file-content]

### conan-text/conanfile.txt:2:1 — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-9"></a>

### conan-text/conanfile.txt:2:1

    > 2 [masked-sensitive-file-content]

### vcpkg-config/vcpkg-configuration.json:12:1 — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-10"></a>

### vcpkg-config/vcpkg-configuration.json:12:1

    > 12 [masked-sensitive-file-content]

### vcpkg-manifest/vcpkg.json (line unavailable) — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-11"></a>

### vcpkg-manifest/vcpkg.json (line unavailable)

    > 0 [masked-sensitive-file-content]

### vcpkg-manifest/vcpkg.json (line unavailable) — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-12"></a>

### vcpkg-manifest/vcpkg.json (line unavailable)

    > 0 [masked-sensitive-file-content]

### vcpkg-manifest/vcpkg.json (line unavailable) — Matched rules

- `DEP_OPENSSL` (medium): OpenSSL dependency detected. — Review the OpenSSL PQC roadmap.

<a id="source-13"></a>

### vcpkg-manifest/vcpkg.json (line unavailable)

    > 0 [masked-sensitive-file-content]

<a id="rule-2"></a>

## DEP_NATIVE_CRYPTO

C/C++ cryptographic dependency or literal build reference; effective configuration is unknown.

Check: Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-14"></a>[cmake-literals/CMakeLists.txt:1:1](#finding-14)|openssl@3.5|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-14)|
|<a id="finding-15"></a>[cmake-literals/CMakeLists.txt:2:1](#finding-15)|openssl@unknown|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-15)|
|<a id="finding-16"></a>[cmake-literals/CMakeLists.txt:2:1](#finding-16)|liboqs@unknown|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-16)|
|<a id="finding-17"></a>[cmake-literals/CMakeLists.txt:3:1](#finding-17)|liboqs@unknown|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-17)|
|<a id="finding-18"></a>[conan-lock/conan.lock:4:1](#finding-18)|openssl@***7890.1|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-18)|
|<a id="finding-19"></a>[conan-text/conanfile.txt:2:1](#finding-19)|openssl@3.5.0|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-19)|
|<a id="finding-20"></a>[conan-text/conanfile.txt:3:1](#finding-20)|liboqs@0.16.0|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-20)|
|<a id="finding-21"></a>[vcpkg-config/vcpkg-configuration.json:12:1](#finding-21)|openssl@unknown|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-21)|
|<a id="finding-22"></a>[vcpkg-manifest/vcpkg.json (line unavailable)](#finding-22)|openssl@unknown|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-22)|
|<a id="finding-23"></a>[vcpkg-manifest/vcpkg.json (line unavailable)](#finding-23)|openssl@3.5.0|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-23)|
|<a id="finding-24"></a>[vcpkg-manifest/vcpkg.json:5:1](#finding-24)|liboqs@&gt;=0.16.0|Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS. / [Source](#source-24)|

Shared checks for these locations (not verified facts about each line):

- [Verify C/C++ library, provider and build capabilities](#action-2)
- [Track PQC readiness of dependent libraries](#action-1)

### cmake-literals/CMakeLists.txt:1:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-14"></a>

### cmake-literals/CMakeLists.txt:1:1

    > 1 [masked-sensitive-file-content]

### cmake-literals/CMakeLists.txt:2:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-15"></a>

### cmake-literals/CMakeLists.txt:2:1

    > 2 [masked-sensitive-file-content]

### cmake-literals/CMakeLists.txt:2:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-16"></a>

### cmake-literals/CMakeLists.txt:2:1

    > 2 [masked-sensitive-file-content]

### cmake-literals/CMakeLists.txt:3:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-17"></a>

### cmake-literals/CMakeLists.txt:3:1

    > 3 [masked-sensitive-file-content]

### conan-lock/conan.lock:4:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-18"></a>

### conan-lock/conan.lock:4:1

    > 4 [masked-sensitive-file-content]

### conan-text/conanfile.txt:2:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-19"></a>

### conan-text/conanfile.txt:2:1

    > 2 [masked-sensitive-file-content]

### conan-text/conanfile.txt:3:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-20"></a>

### conan-text/conanfile.txt:3:1

    > 3 [masked-sensitive-file-content]

### vcpkg-config/vcpkg-configuration.json:12:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-21"></a>

### vcpkg-config/vcpkg-configuration.json:12:1

    > 12 [masked-sensitive-file-content]

### vcpkg-manifest/vcpkg.json (line unavailable) — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-22"></a>

### vcpkg-manifest/vcpkg.json (line unavailable)

    > 0 [masked-sensitive-file-content]

### vcpkg-manifest/vcpkg.json (line unavailable) — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-23"></a>

### vcpkg-manifest/vcpkg.json (line unavailable)

    > 0 [masked-sensitive-file-content]

### vcpkg-manifest/vcpkg.json:5:1 — Matched rules

- `DEP_NATIVE_CRYPTO` (info): C/C++ cryptographic dependency or literal build reference; effective configuration is unknown. — Verify the native library, build flags and provider versions. vcpkg, Conan and CMake source declarations do not prove installation, selected algorithms or negotiated TLS.

<a id="source-24"></a>

### vcpkg-manifest/vcpkg.json:5:1

    > 5 [masked-sensitive-file-content]

<a id="rule-3"></a>

## DEP_NUGET_CRYPTO

NuGet cryptographic package assertion; wrapper ownership and runtime support require verification.

Check: Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-25"></a>[nuget-build/Directory.Build.props:1:1](#finding-25)|openforg***@1.0.0|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-3) / [Source](#source-25)|
|<a id="finding-26"></a>[nuget-central/Directory.Packages.props:1:1](#finding-26)|bouncyca***@2.7.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-26)|
|<a id="finding-27"></a>[nuget-conditional/sample.csproj:1:1](#finding-27)|bouncyca***nknown|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-27)|
|<a id="finding-28"></a>[nuget-legacy/packages.config:1:1](#finding-28)|bouncyca***@2.7.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-28)|
|<a id="finding-29"></a>[nuget-lock-1/packages.lock.json:5:1](#finding-29)|bouncyca***@2.7.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-29)|
|<a id="finding-30"></a>[nuget-lock-2/packages.lock.json:5:1](#finding-30)|bouncyca***@2.7.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-30)|
|<a id="finding-31"></a>[nuget-project/sample.csproj:1:1](#finding-31)|bouncyca***@2.7.0|Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider. / [Source](#source-31)|

Shared checks for these locations (not verified facts about each line):

- [Verify .NET target, package and operating-system support](#action-0)
- [Track PQC readiness of dependent libraries](#action-1)

### nuget-build/Directory.Build.props:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-25"></a>

### nuget-build/Directory.Build.props:1:1

    > 1 [masked-sensitive-file-content]

### nuget-central/Directory.Packages.props:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-26"></a>

### nuget-central/Directory.Packages.props:1:1

    > 1 [masked-sensitive-file-content]

### nuget-conditional/sample.csproj:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-27"></a>

### nuget-conditional/sample.csproj:1:1

    > 1 [masked-sensitive-file-content]

### nuget-legacy/packages.config:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-28"></a>

### nuget-legacy/packages.config:1:1

    > 1 [masked-sensitive-file-content]

### nuget-lock-1/packages.lock.json:5:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-29"></a>

### nuget-lock-1/packages.lock.json:5:1

    > 5 [masked-sensitive-file-content]

### nuget-lock-2/packages.lock.json:5:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-30"></a>

### nuget-lock-2/packages.lock.json:5:1

    > 5 [masked-sensitive-file-content]

### nuget-project/sample.csproj:1:1 — Matched rules

- `DEP_NUGET_CRYPTO` (info): NuGet cryptographic package assertion; wrapper ownership and runtime support require verification. — Check the exact NuGet package, target framework, runtime, OS and native backend. A lock record is not an installed runtime or enabled TLS provider.

<a id="source-31"></a>

### nuget-project/sample.csproj:1:1

    > 1 [masked-sensitive-file-content]

<a id="rule-4"></a>

## DEP_PHP_CRYPTO

PHP cryptographic dependency or platform declaration; installed capability is unknown.

Check: Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

|Location|Evidence|Next action / details|
|---|---|---|
|<a id="finding-32"></a>[composer-lock-git/composer.lock:4:1](#finding-32)|paragoni***v1.0.0|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-32)|
|<a id="finding-33"></a>[composer-lock-path/composer.lock:4:1](#finding-33)|paragoni***v1.0.0|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-33)|
|<a id="finding-34"></a>[composer-lock-zip/composer.lock:4:1](#finding-34)|paragoni***v1.0.0|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-34)|
|<a id="finding-35"></a>[composer-manifest/composer.json (line unavailable)](#finding-35)|ext-sodium@8.6.0|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-35)|
|<a id="finding-36"></a>[composer-manifest/composer.json (line unavailable)](#finding-36)|ext-sodium@*|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-36)|
|<a id="finding-37"></a>[composer-manifest/composer.json:5:1](#finding-37)|paragoni***at@1.0|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-37)|
|<a id="finding-38"></a>[composer-manifest/composer.json:8:1](#finding-38)|phpsecli***ib@3.0|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-38)|
|<a id="finding-39"></a>[composer-wrappers/composer.json:3:1](#finding-39)|mir-evge***hp@1.0|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-5) / [Source](#source-39)|
|<a id="finding-40"></a>[composer-wrappers/composer.json:4:1](#finding-40)|ext-oqsphp@*|Verify the exact OQS wrapper project and maintenance identity / [Verify the exact OQS wrapper project and maintenance identity](#action-6) / [Source](#source-40)|
|<a id="finding-41"></a>[composer-wrappers/composer.json:5:1](#finding-41)|ext-pqcrypto@*|Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence. / [Source](#source-41)|

Shared checks for these locations (not verified facts about each line):

- [Verify PHP extensions and their native cryptographic backends](#action-4)
- [Track PQC readiness of dependent libraries](#action-1)

### composer-lock-git/composer.lock:4:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-32"></a>

### composer-lock-git/composer.lock:4:1

    > 4 [masked-sensitive-file-content]

### composer-lock-path/composer.lock:4:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-33"></a>

### composer-lock-path/composer.lock:4:1

    > 4 [masked-sensitive-file-content]

### composer-lock-zip/composer.lock:4:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-34"></a>

### composer-lock-zip/composer.lock:4:1

    > 4 [masked-sensitive-file-content]

### composer-manifest/composer.json (line unavailable) — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-35"></a>

### composer-manifest/composer.json (line unavailable)

    > 0 [masked-sensitive-file-content]

### composer-manifest/composer.json (line unavailable) — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-36"></a>

### composer-manifest/composer.json (line unavailable)

    > 0 [masked-sensitive-file-content]

### composer-manifest/composer.json:5:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-37"></a>

### composer-manifest/composer.json:5:1

    > 5 [masked-sensitive-file-content]

### composer-manifest/composer.json:8:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-38"></a>

### composer-manifest/composer.json:8:1

    > 8 [masked-sensitive-file-content]

### composer-wrappers/composer.json:3:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-39"></a>

### composer-wrappers/composer.json:3:1

    > 3 [masked-sensitive-file-content]

### composer-wrappers/composer.json:4:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-40"></a>

### composer-wrappers/composer.json:4:1

    > 4 [masked-sensitive-file-content]

### composer-wrappers/composer.json:5:1 — Matched rules

- `DEP_PHP_CRYPTO` (info): PHP cryptographic dependency or platform declaration; installed capability is unknown. — Check the declared PHP package or platform requirement, loaded extension and linked native library separately; platform emulation is not deployment evidence.

<a id="source-41"></a>

### composer-wrappers/composer.json:5:1

    > 5 [masked-sensitive-file-content]

## Action details

Each location links only to its applicable action. Identical payloads are printed once.

<a id="action-0"></a>


#### Verify .NET target, package and operating-system support (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. Deployed runtime, OS, provider and native backend versions are unknown. Target frameworks, global.json SDK selection and NuGet package declarations are separate source claims, not the installed runtime.
2. .NET 10 documents MLKem, MLDsa and SlhDsa. Check the selected parameter, OS/backend support and IsSupported or IsAlgorithmSupported in the deployed environment; Linux facilities may require OpenSSL 3.5 or later. Do not generalize platform support across Windows, Linux and macOS.
3. .NET 11 CompositeMLKem and Hpke facilities require review of the exact API and experimental diagnostics. HPKE alone does not identify a PQC KEM. Check Bouncy Castle package/version independently of the .NET target.
4. ASP.NET/Kestrel TLS depends on the OS/TLS backend and peer negotiation; cryptographic API availability does not enable PQC TLS. Data Protection AES payloads are not a reason to replace symmetric encryption with a KEM.

References:

- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10/libraries
- https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-11/libraries
- https://learn.microsoft.com/en-us/dotnet/standard/security/cross-platform-cryptography
- https://learn.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel/endpoints?view=aspnetcore-10.0
<a id="action-1"></a>


#### Track PQC readiness of dependent libraries (planned)

Dependency-level crypto libraries strongly influence migration options and operational timelines.

1. Record current versions in dependency-sbom.json and define upgrade targets.
2. Subscribe to release notes for PQC feature availability and API changes.
3. Run integration tests for PQC or hybrid modes before promoting to production.

References:

- https://cyclonedx.org/
- https://spdx.dev/
<a id="action-2"></a>


#### Verify C/C++ library, provider and build capabilities (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. The deployed native library, build options, provider configuration and peer capabilities are unknown. vcpkg/Conan declarations, baselines and CMake references are not installed libraries or a reconstructed build graph.
2. OpenSSL 3.5 supplies ML-KEM, ML-DSA and SLH-DSA in its default provider; oqsprovider is not required for every OpenSSL PQC use. Check the exact OpenSSL build, enabled algorithms and provider/property selection.
3. For liboqs, liboqs-cpp and oqsprovider, verify separate compatible versions, disabled/enabled algorithms, native build flags and maintenance status. OQS integrations are prototypes; a wrapper/include reference does not establish deployment suitability.
4. TLS groups and provider loads are requested configuration only. Verify peers, negotiated groups, authentication signatures and protocol compatibility independently. Preserve appropriate symmetric encryption rather than replacing AES wholesale with a public-key KEM.

References:

- https://docs.openssl.org/3.5/man7/EVP_KEM-ML-KEM/
- https://openquantumsafe.org/
- https://github.com/open-quantum-safe/oqs-provider
- https://github.com/open-quantum-safe/liboqs-cpp
<a id="action-3"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. OpenForge liboqs-dotnet is an independent community wrapper. Check the exact modular NuGet/native packages, version and architecture; it is not the discontinued official OQS wrapper.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/openforge-sh/liboqs-dotnet
<a id="action-4"></a>


#### Verify PHP extensions and their native cryptographic backends (planned)

Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.

1. Deployed PHP, extension and native library versions are unknown from a source or manifest reference. Composer config.platform is emulated dependency resolution, not the deployed PHP version.
2. PHP 8.6 Sodium KEM APIs require libsodium 1.0.22 or later. Keep X-Wing separate from pure ML-KEM-768 and TLS hybrid groups; confirm the actual loaded extension and operation.
3. Check ext-pqcrypto and paragonie/pqcrypto_compat independently; a Composer package is not a native extension. Verify each documented algorithm, version and maintenance/audit status.
4. For PHP OpenSSL calls, verify the linked OpenSSL/provider and exposed PHP API. OpenSSL PQC primitives alone do not establish PHP KEM or TLS support. Framework encryption using AES does not call for blanket replacement with a public-key KEM.

References:

- https://www.php.net/manual/en/migration86.new-functions.php
- https://github.com/paragonie/ext-pqcrypto
- https://github.com/paragonie/pqcrypto_compat
- https://www.php.net/manual/en/ref.openssl.php
<a id="action-5"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. mir-evgenii/liboqs-php is an independent Composer/FFI wrapper; verify FFI configuration and the separately installed liboqs library.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/mir-evgenii/liboqs-php
<a id="action-6"></a>


#### Verify the exact OQS wrapper project and maintenance identity (planned)

Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.

1. Muzosh/liboqs-php supplies the native oqsphp extension and OQS_* classes; it is distinct from the mir-evgenii Composer/FFI wrapper.
2. Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.

References:

- https://github.com/Muzosh/liboqs-php

## Inventory exports

- [CBOM](cbom.json): 42
- [Dependency SBOM](dependency-sbom.json): 45


## Inspection coverage

Completeness covers supported inspection within the declared scope, after ignore settings, exclusions and size limits. It does not prove runtime use, certificate trust or security. Unenumerated files remain unknown.

|Item|Value|
|---|---|
|Status|Some inspection could not complete (partial)|
|Selected files|25|
|Read successfully|25|
|Read failed|0|
|Analyzed|20|
|Partial|5|
|Analysis failed|0|
|Unsupported|0|
|Not applicable|0|
|Unknown|0|
|Enumerated exclusions|0|
|Enumeration failures|0|
|Unenumerated population|Unknown|

## Inspection diagnostics

Counts and up to three example paths per reason; full records remain in diagnostics JSON.

|Item|Value|
|---|---|
|Inspection unsupported (detector_unsupported): A required inspection is unsupported by the detector.|2 records; conan-lock-future/conan.lock; nuget-lock-future/packages.lock.json|
|Only part of inspection completed (detector_partial): Detector completed only part of the inspection.|3 records; cmake-dynamic/dependencies.cmake; cmake-literals/CMakeLists.txt; nuget-conditional/sample.csproj|

## Observation assurance and usage

Assurance describes how evidence was collected; usage describes what was observed. Static references and parsed certificates do not prove runtime use or verified trust.

|Item|Value|
|---|---|
|observation-v1-de0213352a25ef52a676705da55f4c186ac7ff7eceea3fce75f1a8d6b3c795a3|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a007240a05ed29904278737853884a61db1b05ae99f2fdd5733dc48d888ad81f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-532aa628b85860abd617e0f7d07597578bf150c2bfdba4c0875aa670896a6068|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0e6c7a595022e98c13892e97c5276776af8eb2b22787881eeb35d75543470071|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8a76fc1a0bee2475751fe24933d8b78047d776e37d411a57a8743ea88203bfb0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ebc63ecfd6f56306611b876041992c7552201998d9b1bd3a35a997affcffd483|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-48d975fe6ce1d7f3bbb7a4b578f9c9e856642d8f21b07ebcc5e257187d3d42c6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5704b2b853ba7ae665ae791724b8455a3adb96923a71a2a676ada9d7964ed46e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6fa754807dc11cb2ba61363adf59bcf816940769db20301ff42496d6b12db444|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f198008794c59f288c3f562b8d7246874d72f22458047a8790f73416ba38ed2b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4b0b79e462c57ea190aab459ed7a93e31fced1f7d136b396dc3700bbc0536b3a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-21c93d3996aeac14103eb7f7b72a4731801b464f8a87416eb1c46e72387268ee|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3745e3846baaa981c07948757a06612009a41699fb4c7867ddca6979ac299219|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-dbf12854757b3a3399a1fc2f5f007ae2b32a7b8893a53f4fbb3db12ab8fda826|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-53358138badeb846fae0f223f1b28c2a2d8222dc98b86b84cf35147bb913147c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9e44037cb25d71a5901ed4c8e83e9968370e517a87f3cf1ae7281d95239cc921|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d9f0df5dfaa712cff8183fd72b0f568e1df26f576e56a6337254ef57cce90a62|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-901069c34b0df16cbfd96e18db5e8da7fca4df56816a802827389ce35fdc022a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1e303a530b03889c3586c0f08288fa38613ae92c2438fbc759f9adb96f7c9044|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4b5655db9d9af382e82b76fa3f6e87f14d3134648383d59adc0e1f6087c6da85|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d2d1f536ae660f0f02398c21a5bae523bbddfe5b2a83306b52dd6ade7144ab6b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-42fce2e80c9c885ff64556bf624a822abde6018fde62380c7297101a1e09a754|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ff89b6bf3b42a2052bda848016ada78a4e6eaf178d6dc177c1514be187de964c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1efc0905fe1f6a2d7d1161257f92b9a9578b19110966cf70c69e2ae4196f1a4b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2181b168c5dca7814b9df760cf229579350255e09373762e5a0d8483869e72e7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cd660c9970b4ddcd72aa8c92cc4ede7b2881214b6564f9a65a910276c18f2f17|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b30ab048ac78472613666614afe61f96b48021c75231a223fb4894b1420dce87|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-805897ffbbc506425666d9f88749d0d09e521e1d8fcc0dddb14d431c3bfc35ee|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ea212bf6054fa60f10f8a0de53a16fa542a57d84ef80b23bfa17717edbe7175b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-28cb2737888c871dbfd324597f7e6902d62dc8839bb7506463cbe46dbf86555f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-85fef3ec5f75a3a3f97e1a7798bcba7543cbe2f400898fef9e3557b5e307d273|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c6656289450be25538642298fa72356a8463709ee09f16c733a56812ee84c898|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a09dc544770dae3700d6c278fa41dbce9f68a52e97900593c6671462935755f1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c90a7f7568939a1e61c35fd2a843fe358c25d8905642773c3cf370be0393be63|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a6526b540ecefda87a77ae8be38db237bfa6e67e8d523407fd29d702e7c60086|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e782cdbb2191189b2f131ccead90f394b02c2695c793018c79c4d605c815d8f4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1c246a263643aec49567ea6e9d0abd2f8de3314f3edf5cfa4f68fe81e00b45df|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2d76e191b115d735b1e98d95a93acec6924366d5a2096e26d1c211984da1ad05|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-37c8fd7fe1d3e71ef696699f8e0f91b5c5574b05a26be037e0fa7997e182182a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f098288880be1c3c85c094e722f498392103c5c664c0df412dc6ec30586181f7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7c89448529e362f68704dcc67a3c6adb6a087af480bb3a78260b69a35eed05c4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-22f391a68658e71a11dccd13114520ecb648d8d25471b8af3166b5d3b0890784|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-62db869e7e17099a2fea72b60eef563ba449f7ff6c331de9410e992c32cbac54|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c1a6b435a64e85734851db10112ef5933d72bae96395d2f41724723974414afb|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-351477f70bfeac527177d8d9200f76e43ab75ba351009a09143f0a962e0e6edf|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-897f3ec10d9089c75cecd1066293fb2965f3ee40a9bf5fa0bad840580d88a9c9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6b032cf616ee4ca738ae576482b3595fe779baba2e1afb2599be3b21b10b002c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e2add5ef62dbf0c825adca4b9018bd3a16b792e9f8fbe1d50c121e9f5bb095c7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b6c226f4797f1d2d9365afa9266261160b82117327384f2403cb45deb4fbc6e2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-38973fb482dbfe131ca5a048d7eb7f0ed4369657ce155fe36e216669fdabc92f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-44a410f39531a94537167820b3d7184e3fb7ace68678912658350e7519da722a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-897b62d40f78bcd3791ffb43980cb43f332df7a4539add0c12336e244001e747|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8493584ac1911bb30f8129a6ad76b0c4ff440c80a461bc4096582bec30c4fad2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cf48004b1e8c309eb2665f991838899b96621b88e511fdad59632aea2d8f6ec2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6a1375fad8b54fe3645e238dab97d5ac4b64f2639b728bb2fd93fefa5e0b64a9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b763c42c3071961555b6182609b23c6ae21d2199da2efab27839fb21e1d3733d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-af0e0ce70e7d3d58c6e4dba7dadc5a6974d7c6df32352aebae2b154ea48d4457|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6311248fc71fee960a2c9f71baa4532d13d6c04aaff40f034c24969aa0cd11ed|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ffe497aa98a437b4ef4090e883794b2c0d07859357ae5d53e205629f6cf93c2b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4496f49db355d3d3d990e865ad5c290443885de200dc836dfed45919ae0058d7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-02f6e3358a942679a4ede20c3222488c1fd935d2352bc03dcb867ef5ccf6faa9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3f5eaa32e6ab8d554bf1757cdd730433a51102f8dbcdf6ee6e2976d6e2e729ba|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-887999bf4153297548531c1b72d553a1df721385db6e7409d9f39f31d2f9eb1e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f3864f5b804860b307959e905b2e32dde468e2b1741b7192233c79b06947d53d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3b9f66865265471ca5f6fbe1f154f7dcf7502f8d7f3b230b206e5d058e678165|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e0242238f4ca485129e4671197edc31629a349a837645b73bbc05026dfebc276|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-247841aa83ad53cd91599c637d722a13907252e5442e46e090a7da098f0bd572|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ebf1fb29a112a37447c96355af3cb21ca721289b898af3455a19f2adc2a305d2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b00175acd218f3b40831f676fcb8d7dd9378d00c4574401072cb8f8efbf44f76|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-34fb8de98c678a6d47a5a585a1a88a16266139f9d2751d7ca08307586ebc0ad9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-52965b4450422601a70e99e10bed9b6de0650615f8a17d4fcdeadc58d5129187|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-fb7633fe8c4179cd39a40657992aa2d766c0b5e003967feb00e4a50b21f6a15c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7f7e43d68445cf7dfd8fa35c74ead87d2a006e4ec16f224a12ae73509a52ec57|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2f0498a87c49376f4eec43a745783e407c0a58f20fff050a8db154885a0429eb|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-40a3ec81cbb119e9055e03edbf8cc9e756d6fe02f8d537690fbaa6c664332d58|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7bd2b42d95b7d59077136c7717114d9cf71e41f7fb67ef552a950848c6b704c2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c663bf35708acad811b7d9c81920a973ff09f2b80ecf919975bdb7618be3b321|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a30ad78d01b7c94cbbe49cb10f6d22492d929aa69703bb2862e5d8a41cadefda|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-26f8dce282a27a85e3992b0d0a6755f2537a3179e8b9bfef2d5e4f677dd2688e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3f66f801de910343a4416440df9fbbf22f0835f199ca89e9af6c1c47547110db|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2d955f2146f3c9b142232ec64ff63ea3c26c817e002398f43836f4b0059a25f0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4fb05c1ebc7f840e10c93d12706f7db7d51e352431f2b72fbfde61c6031c4059|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9444ef3a01a2c4eb723858e07a19adec07c28ffa091658cecdf9fac1df02ae07|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-96b283d5a016a1a531c2ce32cf91676c50580268786f433a68a2d0ea29569fec|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a62e5141f58de63c97c4a37b07e11947778aea426356ce62e5d53fb307eb2c8a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5dff2bb864e034ffd4f454b0f1370d9aff51fab1605561d62b791b88543d8fdf|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-97e0844c0a09167d9fa01285d919795bb8a9f58c20619667608ecc720f4dbd74|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-89a8034cac5b4359e10468d856e93942038b42d7eef8c79748903f0cf6891858|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
