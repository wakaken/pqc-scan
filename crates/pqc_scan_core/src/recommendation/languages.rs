//! Guidance from pinned task076 APIs. References never establish deployed capability.
use super::*;
pub(super) fn append(
    actions: &mut Vec<RecommendedAction>,
    finding: &Finding,
    language: Option<&str>,
) {
    let ecosystem = finding
        .evidence
        .metadata
        .get("dep_ecosystem")
        .map(String::as_str);
    let language = match ecosystem {
        Some("composer" | "php-platform") => Some("php"),
        Some("nuget" | "dotnet-sdk" | "dotnet-framework") => Some("csharp"),
        Some("vcpkg" | "vcpkg-feature" | "vcpkg-baseline" | "conan" | "cmake") => Some("c"),
        _ => language,
    };
    let (id,title,steps,refs):(_,_,&[&str],&[&str]) = match language {
        Some("php") => ("php.verify-pqc-environment", "Verify PHP extensions and their native cryptographic backends", &[
            "Deployed PHP, extension and native library versions are unknown from a source or manifest reference. Composer config.platform is emulated dependency resolution, not the deployed PHP version.",
            "PHP 8.6 Sodium KEM APIs require libsodium 1.0.22 or later. Keep X-Wing separate from pure ML-KEM-768 and TLS hybrid groups; confirm the actual loaded extension and operation.",
            "Check ext-pqcrypto and paragonie/pqcrypto_compat independently; a Composer package is not a native extension. Verify each documented algorithm, version and maintenance/audit status.",
            "For PHP OpenSSL calls, verify the linked OpenSSL/provider and exposed PHP API. OpenSSL PQC primitives alone do not establish PHP KEM or TLS support. Framework encryption using AES does not call for blanket replacement with a public-key KEM.",
        ], &[
            "https://www.php.net/manual/en/migration86.new-functions.php", "https://github.com/paragonie/ext-pqcrypto", "https://github.com/paragonie/pqcrypto_compat", "https://www.php.net/manual/en/ref.openssl.php",
        ]),
        Some("csharp") => ("dotnet.verify-pqc-environment", "Verify .NET target, package and operating-system support", &[
            "Deployed runtime, OS, provider and native backend versions are unknown. Target frameworks, global.json SDK selection and NuGet package declarations are separate source claims, not the installed runtime.",
            ".NET 10 documents MLKem, MLDsa and SlhDsa. Check the selected parameter, OS/backend support and IsSupported or IsAlgorithmSupported in the deployed environment; Linux facilities may require OpenSSL 3.5 or later. Do not generalize platform support across Windows, Linux and macOS.",
            ".NET 11 CompositeMLKem and Hpke facilities require review of the exact API and experimental diagnostics. HPKE alone does not identify a PQC KEM. Check Bouncy Castle package/version independently of the .NET target.",
            "ASP.NET/Kestrel TLS depends on the OS/TLS backend and peer negotiation; cryptographic API availability does not enable PQC TLS. Data Protection AES payloads are not a reason to replace symmetric encryption with a KEM.",
        ], &[
            "https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10/libraries", "https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-11/libraries", "https://learn.microsoft.com/en-us/dotnet/standard/security/cross-platform-cryptography", "https://learn.microsoft.com/en-us/aspnet/core/fundamentals/servers/kestrel/endpoints?view=aspnetcore-10.0",
        ]),
        Some("c" | "cpp") => ("native.verify-pqc-environment", "Verify C/C++ library, provider and build capabilities", &[
            "The deployed native library, build options, provider configuration and peer capabilities are unknown. vcpkg/Conan declarations, baselines and CMake references are not installed libraries or a reconstructed build graph.",
            "OpenSSL 3.5 supplies ML-KEM, ML-DSA and SLH-DSA in its default provider; oqsprovider is not required for every OpenSSL PQC use. Check the exact OpenSSL build, enabled algorithms and provider/property selection.",
            "For liboqs, liboqs-cpp and oqsprovider, verify separate compatible versions, disabled/enabled algorithms, native build flags and maintenance status. OQS integrations are prototypes; a wrapper/include reference does not establish deployment suitability.",
            "TLS groups and provider loads are requested configuration only. Verify peers, negotiated groups, authentication signatures and protocol compatibility independently. Preserve appropriate symmetric encryption rather than replacing AES wholesale with a public-key KEM.",
        ], &[
            "https://docs.openssl.org/3.5/man7/EVP_KEM-ML-KEM/", "https://openquantumsafe.org/", "https://github.com/open-quantum-safe/oqs-provider", "https://github.com/open-quantum-safe/liboqs-cpp",
        ]),
        _ => return,
    };
    push_action(actions, RecommendedAction {
        action_id:id.into(), title:title.into(), priority:"planned".into(),
        rationale:"Static references and declarations require independent deployment and interoperability checks; missing versions remain unknown.".into(),
        steps:steps.iter().map(|s|(*s).into()).collect(), references:refs.iter().map(|s|(*s).into()).collect(), code_examples:vec![],
    });
    let name = finding
        .evidence
        .metadata
        .get("dep_name")
        .map(String::as_str)
        .unwrap_or("");
    let label = finding.evidence.r#match.as_str();
    let identity=match (name,label) {
        ("mir-evgenii/liboqs-php",_) | (_,"mir-evgenii/liboqs-php FFI wrapper reference") => Some(("oqs.php-ffi-wrapper","mir-evgenii/liboqs-php is an independent Composer/FFI wrapper; verify FFI configuration and the separately installed liboqs library.","https://github.com/mir-evgenii/liboqs-php")),
        ("ext-oqsphp",_) | (_,"Muzosh oqsphp wrapper reference") => Some(("oqs.php-native-wrapper","Muzosh/liboqs-php supplies the native oqsphp extension and OQS_* classes; it is distinct from the mir-evgenii Composer/FFI wrapper.","https://github.com/Muzosh/liboqs-php")),
        ("openforge.cryptography.liboqs" | "openforge.cryptography.liboqs.kem" | "openforge.cryptography.liboqs.sig",_) | (_,"C# OpenForge community OQS wrapper reference") => Some(("oqs.dotnet-community-wrapper","OpenForge liboqs-dotnet is an independent community wrapper. Check the exact modular NuGet/native packages, version and architecture; it is not the discontinued official OQS wrapper.","https://github.com/openforge-sh/liboqs-dotnet")),
        (_,"C# obsolete official dotnetOQS wrapper reference") => Some(("oqs.dotnet-obsolete-official","The official open-quantum-safe/liboqs-dotnet dotnetOQS wrapper is discontinued. Do not infer an official NuGet identity from a similar package name; evaluate maintained alternatives with their own ownership and native dependencies.","https://github.com/open-quantum-safe/liboqs-dotnet")),
        _=>None,
    };
    if let Some((id, step, reference)) = identity {
        push_action(actions,RecommendedAction{action_id:id.into(),title:"Verify the exact OQS wrapper project and maintenance identity".into(),priority:"planned".into(),rationale:"Similar wrapper names do not imply common ownership, packaging, maintenance or audit status.".into(),steps:vec![step.into(),"Verify the actual package, native library and runtime prerequisites independently; presence alone establishes no selected algorithm.".into()],references:vec![reference.into()],code_examples:vec![]});
    }
}
