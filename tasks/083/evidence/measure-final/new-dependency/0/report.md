# PQC Migration Scan Report

Generated: 2026-11-24 18:36:25 +09:00 (UTC+09:00)

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
|observation-v1-b4e99e5be5b85de3b010a7b3be4fdc3eb63a96e3fb6c7d24969806d466ad5788|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bc50913b84b697b8309441baac7d3fe4c6793addedf4cc68a5c11160bf137a3c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-641102ae1132cbbe3d9c2129c13c3a92dd1e584a18d31f28f77081cce7b05e44|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-53c624758ebe7194e6d5460272415ac82f5db3fee178cbf400c1b5d488d5219c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c3b4784bead7d1899bb33afea81fc23dc5543dee19f4a24c043b426c147992f7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a2ceb56fb7610dbf31d91c812f04464de7bc1da817fe5c53658396fa72bef30c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b5280895455fa7580d2529ad496ad23d9b5a044fdb4b08b247d93390e95de8ba|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8cb3ac220187ff7e93bdc6ca6def09828f87eafdaadc259d1fb0fe3a526b08b5|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-79893a7ec0ffb8f4bc10a2066ed272297f56212936d5458711a5870d465564f3|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f2597889e715a9dcd6b1aa79d8a9bf69bfb93ff53a90096bf5b2af1df3b1c92f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4c41bf40fff6da7207519ccb7d9b4313ca1307726547c2cfa4314051d8543876|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0ee69cbf51316c567a2dce4946ba27d0fc4389af2eac343d3d66479681f8ae3a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-721a16c2927d1a24b4f49cfdd0abfd1e16c0b1b2213e2b9d88466aa989a10cb1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-228e6c756068058a211203769629947cbf5b32f3ff71e5cd33c3deaa6af9cd02|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-736c9c38aafd4f378e44380b2d0fbc79127054729b0b551a70eb1738dd2230bd|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8b5aed6d47ed9f580e0da5a2e8405a80ccbdda55143e8ad96cb14ca45ecb5476|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1abf4d3a0c95ac9c1656486c6f6f3e0427754114b5947d5a2a3d60489ac62842|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c90da5286f95c0dde9153fdae6a4c740aec8ce06c76620c9735d415511d4323a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-933f8ed18e3b28b8d9028ea7ff0967c8471c959e31793cb202392b35f759e283|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5b750379db77ba226409815e1a1267c0d3429feb48bd5832fa31351b4548c15a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7a83cc2185fc16b778cc7ca582572c5bef41d2b44e9f52b997390e7c094dd3ab|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5ce74f60c75426fbb33da90e1ec6ddce962645fec5a3fbdfc74337f9dce5257c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-57bedad3a4a3c749a1029983926f9e9d1756aef55b9d243680a4c51f097b90e9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-31127546653817a7b3c234c011c0c76ac11b5c965c9ed45c8aec0f169a3723e4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-dbf8a70c751d5a7b654bc11364844d443daf36ac18ca7341e90c5c9fd46d156f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-471ab39000b639685fcdb856ad1bed2494f96279f4b2890e64d19f50ec09a588|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ceef3206873e3138e4f6fc356c8606dbe82fb2b01dc4b193f048825258df1a48|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-46dcdbac303ab5d1f12c313d746242885e176453bfba7c5910a48c86de804892|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4e70ce95761aaa22ef1dc31e2e74900e86ade1f8ee512418eadc901ee2250aae|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7eb8643e9f55ed4f185ba7a875cadea44437fb0509c4e39d47c573a588916b71|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-761b98db0c4b2accbbdff20907ad14cd939d3c1895dcd4d87c208d4cde5173a1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-83ed4af1b9c36c0d5c0f9d963c48400b482c122c3521d7e1ae3aab8129f084b2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6184ea185f09775273b1da623849bc465e22e87a0100d8af2cddb4e4239caa05|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-707935eba4c6fc64aecd5dafa06089115e0cbde1edac649298765bd3af90c09e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cf1565584ecb50a5653e7da394986ed7fe9ca85c81fbef33b4f05baa974daaa4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1768d25eb96b351ea6568f4cf9be37c6ec4c30bc95f4b09b4a6be7977ab39371|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-68304c65fd5c572ff449956f6fd1f79f4b962acaa7a61bf5caf7e729faadb012|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b3adbd2c5860ff8793ca6f976d3e0eff38d9349c9ae5461ca3a1c56503ec2133|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bb0930d835526b347b2dbe4556183be0a4215186491a0c21cb46ab56da2695ac|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-517ecbd99af55851331dae23c82b897b10b01296c6c05166340f4728e331214c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7747be2c884242481d72ba60c0f020e759bef44db7ea8877c1c2df1db0982794|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1b6d18bd16fb50fbfc3b9799a0573fb530c47a9072768debde396cbab9c83bf4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1d94accb910f22649af543a40faa8cf28a7cfdbf44484ccf22742c4b711113d3|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c8f6a7b29032f9a26fe2552bfd00c4792926f66e5b0c34483c631fc5408063fe|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d82c8e62dc2c0ea73c56499e510e4662fca6263eb75a0853faf12b775f9b93d6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d86fe944b671258c2befd962e4c96645f099d2d8fb6ce769ef1084ae14bb8045|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c6181135d900a9ddf8fe8d9ccc47ea083bf3e71666f69d2c5c2780ae0821e60d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9f14a12854348d7ee2ce313b843bc949140c2791234cf4e20c4f1c023d3f87e1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f7d527a65b08c9a44a84156bc482fcc5e5899b5e4422290994ff00ce4df9ef69|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a8a25fa08c526c949e7daa705cb4ec5fcfcecb3eb4a961caba38340e42a1055d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e6ae8ba81827589b54248817b26202d49037c003130fb291245bc9b61fef08b8|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c7c01b2f40ee5a4fe95546db602727d1e4641d3d516355c03be1e1db513f825a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-33e75e4a1ec0f9bcfa516441c192e92ff46357b4623e17ffc4228ab97f51c646|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-14de3b5f7e82dec70accfb1b28ad5c07ad7add9f49241dddcfcd82c9f0d69080|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e6f0937763549daee4870cb96ac833fc81220a9ef092cc50f8428d3ee4a5b483|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a348e4086df13178739bb713b6d3370de0b574f537106d783b3242662d4fc004|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-94272ea825561fbe30e422b60bff8e67d05c14da7993c87cd2738379c0d7c50f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7c1e7447a4841765489937338430be15752d1f2f8faf962e27c212eaf043f729|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bff20efc0c15b2136c9bf3c26bff966824496a0429c4d63cf55e34a3bb007d52|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-eff465efceba76b8cba9a12c2a4ddd6a5a245d75f8a5c2343a6f93288ef10fef|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d544fb299a22c2c8a5c49bb714aa2c604273e84b43dbfa9a1d0315350228e17b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8d57d8f2bf29130e7bd0e7c3de80f2dc7ffac0a16c551151e26e73647f82342c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6fe1bbf3a601967432cb71513a13da29a31ca0bd10a29097fa5ebfcec4b2d6ee|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8b1e6c4edd795f64a424847847653c986f3c2c2a033a37b6d6d74a0c3b9031f7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-fbf1032627e33392d40f3b6a736d90e59f7d80205ce4b488a14b0b4eaca25458|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-73341621b7f379c5b9c20bc646db5a94ffc36125bb8d0b427907f0773c8ba627|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-05840f6e3b277f853aa436eba81154d6ba0cd0847890cf00897b30d7873e8e07|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5dcec53c186b428a4fb71b812fb382590af31b22d3980d20622dece882e5c154|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cf20e9fd30e12ea3ec7cb229a54654c34f25241a5a959bdbf201fe101c047914|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2dc6aa6c2acab5d458d46e1a6e16d9210f2fa497a7b3ae990973f94d330abf16|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0ff668b836013baf5945a17cb97cc04d54025e17260fc4dd6f8d1d1f1115a1de|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6cc34bb6e1814a302a0db8ff9ff3a8dd3963cdea7258170e857e906bc74c9afc|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-80a78ddd0f40012e126c5876897d6543b714139e8b8bcd85ea59b8236e2efe41|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0777b0fea00d2164a6dec3d05cd7c88dfe59e2db65bf6433c43ee9369f527ea0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0b4727dba37dba44aa94d74b7a39a162730ef567bc4ecb4d19c9f11505cf67e2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-49a7e5a7b4a4600bc718e1d09b0adad4696e8dfac26f35d37f5b8211106ef5cb|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-02d8d2642ea95b2f8217ca32335b9a0271861ca7c26371d0ef9b7a0160d6e243|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d0cce27b4d636976450198e87ac0361ea4f9b484e67b16f52a95f74c86b9d16c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2a9725f02c39d024aec59da25ff26b2cee9837dcf805a26aa6b6e2be445b4cb4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1e94179c2a6bdbe97eb36dfc18a0e4371018287a112fd7b50303b932b3ce1d4c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-14322c1a6883068afd4106389ba07d97728773ad9d5e1566e30b251db8e67958|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-de4adab5c38fc969f2f57c16a4a6f494903b728975a84589518093964a2a3240|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e5827c9d3b9b07fec3590516a60b88b9eb87ed06eedb5053aca03727055a65f5|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e73b0bd5212b05790210c1ae8f208282c8b629d9123a68056654b8c77dc30469|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-869369b79ef68b75bb53b9185d0dab84a9663ddf1735ce9d394dda43c8cf3f65|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3892c8c757aa0292aca726b1bc2e5a7372c6d582a1f7844529c6eff36527c99e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ec693490c370fc5d2c97d29c57233459e16dfdc967d113df72f28b029594e9a1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-44689c4a8635e31f8e7d07d1f9902939cf591943a372bed6a812baa38cebd590|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
