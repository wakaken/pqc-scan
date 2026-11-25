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
|observation-v1-4e5e315e03f16a20243149b7df1dbc071209495b37ddc06fe54ad20e67997b13|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1a3de79f78c913985c33d52bd6caabc6a573647fd821eb0a4750a088b2a27616|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c52e05449a36a5762b67d72b7527dba076d31714d53b0124c129b0edcef27ce3|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cf29f3013ca7a13f6b532c6679dc998933b1e7b23b300180704fb7ca89ee952b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bcbab0bfab30f8c3ab3ad18c3cd38921a218b8b968a0c2430a3deb5eab5e6980|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b40098e1338af77b0d0543082becfc149690d33ce6527e35ab1d522d8b00348e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0cae243c9f034201b7f6251bd5cdce7c9ebb2c53494c929f1cd373e77beffc95|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f4d6f7981c10cb6e865129a6b979d2a1ce9b919274be236fcefd79798dcfece6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c6835e3158bf1c00843ce1768fc815e1c3f5bae5555dd9a978f137116d5e3e34|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ebb6a8d64ad806a81613e6c5588ff8751966b3e9d832a16b98d2e72b3d9c18e7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-77b1eb22e60b51e614b36d6ff990d7b7475db760bd9b4500d3b9b031b1ca7d9e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2a5b4941f8821b91a4ed31186b35de763dac20a2ffc5417912b694ea1ecc1e8f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8fe6a2a37dbef2e9d72466241a8f506fd35b3c78fdf2eb72271de614113408a2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-39a0e3716aa42e8cc0e20f3b6ab7a92faf30ae639b159e23e816164179311672|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b22a9114a62eb1fabaa683d3844964ff3dd9d9db7fcd36f5bb2893c662824945|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-978c5ed0785f401a55ddfc5946fe6dcfb7023eaf784b5c4047cbbabdfcba43d1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4ce4fc14b0fef4826789a3f7fea1662edb111edcdd59886d4c7d3747d794df8e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c7606d36586ffa38125bfdb674580e7cfc26339f96a46d2bf895adc899a0d7f7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1e872ea07552300010ccbfcab23519e93cd2fd62011557687fdf559925e2d110|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-eca56166f56212e9e17f3dc605befc94f8491fce0c91fb2a9ecb55385c453799|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-93f2e287a12814bb4dab1d622119a7c4fbdb1029d5b3290c431f2ea518ebe20e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-157fd54d9c29cd5795ba42269ce1f91c43ebf9762a10207eecc28e07f2d49c14|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7b13778df440c866627110152ce225441764089603c197daf12abc9f9b3df2a2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-08daadf8a73e708a32c00683e2b64eaac99c0cf6fa248c26134246f8c71d0e82|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-987f543e413a7e0938b5625d3addcdf6060c538ee4cb5f4a968330f822a2f59b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-649743fbb9a849b069918052c15cbca40dfcade783ecf47c271598b25dea6ced|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f12f188473124fa0f5db78fc39be25407d25fe2ef2f2e8482c17738904f2c6b9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-14542dde0501f7c6a5205c1a9a59dc9333599589d549a5ef9963fa58947d0010|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ba075c0794558dd97e3fa989840c7ae932b550cc05469aac520f252776848c7a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-738de478bbdd250483d01d019d7c355f35c93bc49644a0c476e7248a0c8af433|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a5c5b33b78bf3f75d825bfb0f54e8339f3ecd1f7fbc44a6d86faad6d94869fd0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-18d4f827110204327acdbfed3fe83136a3392b51f5b223afb63d5a43e68f6adb|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3cd5d0b8016a354d4bc3e7e767d4ee0abe8ac0efc071caa2bc206a6f58325328|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-881fc4d8833dc85232e98cfad5228e9dbd8d5a0868a7e643ca7c57df0b8f9f6c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7d71fda3437fbf7e5a2d9222e65cd100ec26c62ca66f623f7ffd26a83ba57ce2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-98b5501c5a122e4c72c30dd6be7237c644b363ea517030783cb61d10e7497074|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-da25851407ddc58d144d84114483163e29542b8f261850b9f9bc7ac1364af591|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b614e68eb51da25e2bbe58cbe4154180e165f06305895a31bda4f7dd7df238cc|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-74d46091f94eaa9e30dfe737107e55667e751986ec663ba40a922214f9870917|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4c878e992b9a2a6fb27de11fb5889cff7a791ea531275b6b4cd44f9c8e447014|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-482f88ef0821c44f1d18f420017d751f433e521227e10318595263719090ee91|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a941a0645c93b3d00b1b4716219f835f4dc700ea65cc3919cad82d64825c17cc|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5df8218b7bac1291af3923b104b07e0357bc408f284eb67ea7bc39dd930d5b09|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-fe2b22872c7fd2dc53145670d4236dcbbd97d45f9b7dc1cadce2d9b8d8363804|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e5f36f59feb79a400d5ba87967447d23bbdd5e9e078d7de344c0a0b0e88907ef|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4faa274561b35569d5e621fb64fdbc6025c7b6dee2f7bfff2922e05a5f0c98ee|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4725b27df26476c304b977c7ba15f48b2738e4b70e7db38db1996f4c64c3fc85|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-95323c038de77c3ad073e550c2d5741d5983665566194b0b9f31a6706f0a3963|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8a9f126ef7713a0fd01fa948eeb8710fedb98e0ce3b0a89d386b45b0766e5782|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-aa242c6d069fe23ed59fecc2d7278e719107b42cc3855b50f59b61f47fc9a147|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f283eb417f59ec545d525c4059ce17f9809a7005eb32cbfcee5d85b029a87016|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3a32a47c631c877a591312844dfc8fc4412076ba153ab44974b4a9f9bb29ae21|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5973d94a076eca1bf632baab76fcd3ce71b1934e68c6e4e0defd7f0d3d91d001|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7355007b570cb055664b56c5f3c885d2f4161534280693194e2f6dceca3086b6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6127555cdba964f3a13dd9f7c91854dbacbe58acf56f0d21911454593f02ca62|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5233e32ddfe14b89ef00af7f97ebb8b717371cc92807e75bb729789b1e07edab|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-98b4252e5b8f9a2634d9f6da1a8d685e6ee29f9e6d325ad9232ea76ff9fadba0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2d734d58d71b113ee5b57b4756039a2f1307012ca28917babe96d3b5182e6777|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-854fa65b91ef8de28e405dcbc2e505f93f06cd99a8914b0d412bfa30a9c75cfa|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c49c59a03936970f272367d2b3e7d42ef2bdaa9755ce94884cf5cec434bb2d73|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b4196e25c7dc35679a501cb2ca472c7be54b71a73a5d221c22b09bfd57b9b425|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-13c654f05b26297d2557a97ad709f7f7d9270047abd2c419d62b4948c6238cbc|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-208231eebc9ec7feed1814503c0ded8620e4e7e51859c99c5fd18033b75b2531|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-48140db9db944e08317bfc0304ee3ca5e3286080b1b971556dd17b493d741357|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-141f45ecf4eb8a7ae845d4733c7b9b10356d05ac9df6b68c3d838db3d509888c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-49391ebdcddafc075bd6033c3174351c3df487645b017b21719e2db50f61b229|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3256135859d1f194ffb34de063f76ea722c8c08a240e2a16b88b212e83ff9c4a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e34cf1723a9f481f9cde326fd5f594c4c2e736e8d720430f7d0c34fb5b92c12b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-eaccc59057c261112a87e863074e610d6ebd1e6d58d066ddafec64baa69dfc80|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1f139b072d0efa4d2418370569b7347f764a9bc0cb57641013c00148b023fec6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-71b759c744ead9d15dc607c9a4a8860a371e62df3f9323f188e5e5ef759fcd64|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c8adf52261c858b4ca7942db906abc630781013c943676c9e0989414ffd0bca1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-98e10ae14b411b1b709e172c28b5d5944c8ccda0183ee1e2770b1e436a771216|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e7fe24f37ea31c7bf12948f6eb9b0cd8b84bfd023bbf99aa95cf632d824b134b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e7ef50cee14a6ed34de0044c91386b4b68e830836f87fc34d35fb66c9c793a67|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cf2dab56ffe8561897ca9c89167ded2439b73a51905e0d70928b75bc545dc74a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c59678a24f2867c9823a32a25cae64b87ac725245d6a7037e3f6799720f65403|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-84ca1dffd9d405626e282e282f0f5a4ba8e527773bbb60f5af12b4c7318760cf|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b8fe757580b8a6f49a85abfa2ccf58cee91a63cf4d88c7b8078494fe5220a132|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e6c7e4d99ce2ea88c4eeff133523d2defc5f721afbdee74e93fc3ef083ddbe06|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8ec83528a8aaa886a851b540ff7f6f99c7687f64ff8b6e3d251fc04a18e088f5|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-14f6d731568f9461426f30ddc03d8cb48006f25984fafe9b98a40493dfa63b0b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5843d5439965210f4c2ba89d7226843a029b64e94417e0eb4da6a1ae158b3f09|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-665fd8ace2a2c6039795d80412f979623741ffbff0fd3e5c4b92f279e8762551|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-76dab842f0553bb847e8b2eef1141b7143b42527c182fd19f08f79b4054b2309|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e970c1de73b67d41b3bc444e37c2c043ad64f808c3972792cf871fae64ccd415|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-86b42aa5858bb907f15b22f9f6baef5f96bf77531b860ac77e69b32ab8a2065e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-312e90537c9c3bf5e07fbebf9349f41c669938503e3c01a3bfa18b711b58c470|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
