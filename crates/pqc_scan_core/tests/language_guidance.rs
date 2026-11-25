use pqc_scan_core::{
    recommendation::{annotate_findings, RuntimeProfile},
    Finding,
};
use serde_json::{json, Value};
fn actions(file: &str, rule: &str, kind: &str, metadata: Value, reference: &str) -> String {
    let mut f:Vec<Finding>=serde_json::from_value(json!([{
        "finding_id":"guidance081","rule_id":rule,"category":"CryptoAPI","risk":"quantum-uncertain","severity":"info","confidence":0.6,
        "description":"Reference","migration_hint":"Review","location":{"file":file,"line":1,"column":1},
        "evidence":{"type":kind,"match":reference,"snippet_preview":"[masked-sensitive-content]","metadata":metadata}
    }])).unwrap();
    annotate_findings(&mut f, &RuntimeProfile::default());
    serde_json::to_string(&f[0].recommended_actions).unwrap()
}
#[test]
fn language_source_and_dependency_guidance_names_prerequisites_without_claiming_runtime_support() {
    for (file, rule, ecosystem, terms) in [
        (
            "a.php",
            "PHP_PQC_API",
            "composer",
            vec![
                "php.verify-pqc-environment",
                "8.6",
                "1.0.22",
                "X-Wing",
                "unknown",
                "OpenSSL",
            ],
        ),
        (
            "a.cs",
            "CSHARP_PQC_API",
            "nuget",
            vec![
                "dotnet.verify-pqc-environment",
                ".NET 10",
                ".NET 11",
                "OS",
                "IsSupported",
                "Kestrel",
                "unknown",
            ],
        ),
        (
            "a.c",
            "C_FAMILY_PQC_API",
            "vcpkg",
            vec![
                "native.verify-pqc-environment",
                "OpenSSL 3.5",
                "oqsprovider",
                "build",
                "unknown",
            ],
        ),
    ] {
        for text in [
            actions(file, rule, "source_api", json!({}), "opaque"),
            actions(
                "manifest.json",
                "DEP_LANGUAGE_CRYPTO",
                "dependency",
                json!({"dep_ecosystem":ecosystem}),
                "opaque",
            ),
        ] {
            for term in &terms {
                assert!(text.contains(term), "{file} missing {term}: {text}");
            }
            assert!(!text.contains("java.runtime-upgrade"));
            assert!(!text.contains("Replace AES"));
            assert!(!text.contains("PQC-ready"));
        }
    }
}
#[test]
fn wrapper_maintenance_identities_are_exact_and_never_inferred_from_a_similar_package_name() {
    for (name, id) in [
        ("mir-evgenii/liboqs-php", "oqs.php-ffi-wrapper"),
        ("ext-oqsphp", "oqs.php-native-wrapper"),
        (
            "openforge.cryptography.liboqs",
            "oqs.dotnet-community-wrapper",
        ),
        (
            "openforge.cryptography.liboqs.kem",
            "oqs.dotnet-community-wrapper",
        ),
    ] {
        let text = actions(
            "manifest.json",
            "DEP_LANGUAGE_CRYPTO",
            "dependency",
            json!({"dep_ecosystem":if name.starts_with("openforge"){"nuget"}else{"composer"},"dep_name":name}),
            "opaque",
        );
        assert!(text.contains(id), "{name}: {text}");
        assert!(!text.contains("oqs.dotnet-obsolete-official"));
    }
    let official = actions(
        "a.cs",
        "CSHARP_CRYPTO_API",
        "source_api",
        json!({"language":"csharp"}),
        "C# obsolete official dotnetOQS wrapper reference",
    );
    assert!(official.contains("oqs.dotnet-obsolete-official"));
    let ambiguous = actions(
        "manifest.json",
        "DEP_LANGUAGE_CRYPTO",
        "dependency",
        json!({"dep_ecosystem":"nuget","dep_name":"liboqs-dotnet"}),
        "opaque",
    );
    assert!(!ambiguous.contains("oqs.dotnet-obsolete-official"));
    assert!(!ambiguous.contains("oqs.dotnet-community-wrapper"));
}
