# PQC Migration Scan Report

Generated: 2026-11-24 18:36:27 +09:00 (UTC+09:00)

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
|observation-v1-6186864fdbda374877864202ee4b9764874c4e9238b9b551c89c7c79619eaefc|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1e8a7c3e2b9258b16e14f3707b23e07ff51bea99978ac5032390d7e9ecb5b386|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-19d871e6a9e0c45d41278b127164f6bb2a7c0f0ca27381eab6519dca386ce3ac|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4e13d9cf672f2d6c2c81c662117a1025424e6f7831ae7e344d959f3c289faa54|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-81e625c11fe1c337259d18dbaf009a4bedb82e24ac985ca7ca93e15aa3c6fbf9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e8ff9d3bd79860df21e7c2c78c61902f0cdcfc52ce7837d6b521221b59dd20c1|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7406e381e562786b6ed1e1b8b194fdb2a7abce9c0df7515d1e7d389d0a6e8b61|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-af379669e90f1df070f8237158cba977983b8db0fc16ae1b84c2941f2a1fb45c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1ce68f0da976439d0b4ad3017433797e06178d5142dd46912cde646d50ca8012|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5dc8ba2c0120047cad0bf50668499758cbb3c1fcc7a9a23647772a0fb0eeb2ae|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bd78e2dbe6ede7526dc7aaaab589f1568bdf899e30b34508a12a7bef00d76a15|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-25ac81a5b119c863459242d5b17cbc285fea8a81da375ea2ad920bd69efec3a0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3a6ca47890716c241f765b74d42cf46a60f32d79075b6eccf1316c352187afc4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ffa11e6d9a20c50b76aec60d762241d2179c91950bf7cefe0b2e416d1044ee40|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2c19c708e6f820a4b3c2958aad26e8d9ec5e8756236c570a9740b93c2968eb24|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-15fc6d7696465a5def52fd39085134bb2567201a92caa7cf88883c677b956ed2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e9dc0f90f3de51239c7f282c84c667b449b18e573f73cc177b3e77701a680b41|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-72b5f055e5dc51d0ad38ca5159f97b967ee997d673a2c74bf9d29343c1147a7a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-05acf310db64764ea922d9ec16c1a70893b333833dceb847f17a99c71c694826|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e31b8f6c2f1566a32ed8aec2f8229ed2c64c830b9b0aa6d74e0b15377fa97aa4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a95e92ba8d1c6d49b8b32d37dc9ceebb225fb3a81618e2fe3132bf23128b2d3f|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-61549c9dc76434f9f09bf719fa3edeffc6563c39f7791a2976b6e3d598d02eef|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cd9e7dc15c888ffd91192ab98efea9adcfeab47938594044460700289aa14d22|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-adcee3085ce3c3eb0a2ce30e5db2dc0c4e78823694a22302637806933916a301|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e214ad4ecc1a31db56666bc3fc4ba0edafe88a1dd526e3c0d5578619704d84c7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7a15a2fa8a09e8230cb39211f06a234c529da6bd3c111ca211ec11a2cc3d4f84|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ab757a18fc5da443cc0d2819e9c61b34a556971c506c152ffa13b2832ebdc587|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9c280644f270b7fd3932e0603056c139d623ec310a9b755ac480d6ed3ce2bf41|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4d2dad2c8f878ec0bf00434feeea44a27f5ce254632fd61e02c23c0949f8f507|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-007db5f2bc8a5cc3f426c6a827f7dab3f714b8c595a729c7def822f2dbc709fd|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4c53fdb9c5068f2f80aaaea540d97d4e92374113713cf776a4de2a591fa21de7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c46de2acdcdd9b615d84446be34df1c0ad10ff8fcc9f00802d1e05044d9a0226|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3cbe7cced1e2ab7407b92eefb6fd98f8d8b3c79eaebea7574a6591bd5d6d3d6c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1197c6edbc9eada14cbed047527a1fb5b17ef2de422fdaf2854ff9a38c873792|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-803e2b1f305a083312f309c73f1cdf2e7fd55bd184555f5c424e2be8e84313b4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bd18bf6c6e7032b9b2781d7b392b6dec8b8fea0e8b0948694f50b5ea190396d2|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-bb2cfb22d11b8142de42680ff84d08537e614c0852e17b9429bce8ee8c5a0533|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0580e0f49617e510a598e26cc539a4dc2159dd2825088529e061bae88cb26ac3|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c908277282c492d49d0cd64c586cf84ae23d3c037ac3e06ac2ec97aca359391b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2590d5ba502dd59619184f0eea6c5fde4cd93bbbc25744a966e6b2b71c505883|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d42c76cde32de4d90c0a71b2b3bdab8330d43a808d331c8f9ba7b692025c6f4b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-6ed3c1567f8c31bfdaecf729481177549f1efd17a09dabd6519e775ef23e0afc|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8e3725bfc41ba42751780d8fb11297f8f511adfdeba96d0c80881cd7d0be5aca|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b4ecb738642cbde9e3a19c8ec89572e1f2e7fdde2fb29aca889bf5386671e6ef|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cc07d5c89907dc5815b5926b83aabbafe832a09d7d79885d1f246d59f34b711e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-45cc4e975ab34508a757ea94225a902c6548b1b1aab4eb5cde82175dc4706595|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ef9b744f8d81df81bbd0eb3604ed64f5ee8fb3bdbb5e025c7420c695c68e9f8d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9c230c877fbd785738089256f8e88065ac05314f3ec430f7a802d9ccc812968c|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8dc8ffa2307289a1f3c530d2954e35e6c06a789b83fdc67664a7393d07116ca9|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-c9dd1bf74a4b1e84c7400d11595294581eac2c666bffe9a103d69ebf25b10e7e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-1447082676ea14e4b3156d3db270c44246ec9d0ae9d3d193277048230277d319|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-5a94110c19cd4e91dc86c5defa4a58899f740001de0dcde758986376682037ec|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-3b173896fc97388dba9d7526a3eb5819a3881b97478473ae18e41c059abb7719|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-47e2130d0cc00ed495864f16a79860350dbd7fa0d209b453024aef3b307374ad|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-cf276fc1463a9d1a9cc32858da24c8c69c914ab74a3889d291945640ade06a48|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-99bd17a1df99aecb5b8cb2e2161128c3e003c43d01c0e1c481dfe46bff717242|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-91bf7028fed819dbe1c63a0ac0e1839de092beeb2dcd5d76be8e796f9edb823e|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4fc7d292b33bc4144a1bfe09eeb6246c03b7c0f233fad6b08239013f614d0515|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f7b993d4c9de989c5485b3401e12ab0fa10f45dbc00f7ac4942fb01b3902aa09|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-e49904961a48f7407965e0985699195135a0a755bbf595cc435b5f87e9d823be|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8c9ad03d734754176ddce596b7f4ab7e081ee6eac083f6d3780e6e3a59dc7abb|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-4519a1be7198659774175c8398cf0657008de7898506a95df38e2a5f02f978ec|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-05ab26e2891b9cf85226315cec04006797db97bae6ae942fbd8569ba21a43b67|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-7db8873bfb25a2e53318e8779b68fc07f4ae7f6bac5528f532b9cb8761dedf69|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-b7b5d64cfac7ca456dd5cced1e45c321c32ca501564d6da9205e8e5ac88f2b15|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-99ce4d64d26125b5520feb0b33999a25cda04b896faa738c649825e0cdde0b14|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-8f6953f48bfed1b5f97870fd30a56ba9fe9f13510f9e3fa3a66177bd4b30c27d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-30ceac9a61883a49b6e74a1b8d137972186f5e5213781ca2f71b1737a9a24cec|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-9809d3488354a061a2836090ca817fa03faf18b7d83747ea15ab4d749dc65295|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-20c0c5c8992658565167921657dc95b77dd80cd2d6b53ba59507ff5ee93f076a|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-98706ffe58e3f05e1fa39a86281021d353b2e26df559a0a746b6b8b579309f34|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-26b29f41164a268ada16b8a3e245f2d6f7f29c9adfd37a0025c9ee52dfa0ba75|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-a36a12e8d0f74d0da527255fb5bf4b06624d2eb7b129153d6b5c895751326b74|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-d485d37ed6881cdfc8241769bb69890c8b564fb0a4f6400dfa392a33ff26522b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-185a085d8b025ea8419733cafd010b23985d050910b39cb44d80d118442e8c0d|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-40ea7b2394911688b626b955aa19b62c7bf8f44948c33cb7795e8bd9e5b2fa74|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-2dbdd82a9ca5cfc65b5f8d1ec7e4146593b34ea8a9ed89e46e875b9d9a10dee4|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-ce0ba996aaf6adc84770a15c264f624fc01a0e08bb3530871592252e03395c5b|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-40132b0a8a3968e4fdc8ab6b0f4e356fcc309d83086813c64f30c019c3d03ea0|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-76573b91397dce1d4bc0ce9068e141525b3193e9c20ccea6c7c47b055c093f93|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-729fdd134c37f80853661d4c02fbc59370d5440d369ef0060252f0451fd6b523|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-05c4190f601f7be32fb3dee5b96785f4cf4f0fda570a4bf32256e990abc30693|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-f9cca317dd1908bdb6d94ea57ed43e1426654c49f72e4594645979efbf960f13|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-034c4b018a31f742c0fee4174710b76ec4d27cc4a0b078c28a5facb40c826fd6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-afd03009a6744898d7c422cc1fd82e3a60344da3258aa9f240860fe50b32ea86|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-46246563826c81fdd6efc16bf773ca339b47ce6dcd927c2a0263ad1fcdbf14a6|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-026aac96dfcde2fd51e043d0b73d68ff66433f54e27025ae213cc241b8685ef8|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
|observation-v1-0642306473ab76d0629752696ad4ecd96713264c1635806d10c70cf9ea7c15b7|Assurance: Parsed public data (parsed); Usage: Referenced (referenced)|
