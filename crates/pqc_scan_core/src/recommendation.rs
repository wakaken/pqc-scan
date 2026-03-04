use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;

use crate::model::{CodeExample, DependencySbomEntry, Finding, RecommendedAction};

#[derive(Debug, Clone, Default)]
pub struct RuntimeProfile {
    pub java_version: Option<String>,
    pub node_version: Option<String>,
    pub go_version: Option<String>,
    pub python_version: Option<String>,
    pub rust_version: Option<String>,
    pub ruby_version: Option<String>,
    pub middleware_targets: BTreeSet<String>,
    dependency_signals: BTreeSet<String>,
}

impl RuntimeProfile {
    pub fn from_repository(
        root: &Path,
        files: &[PathBuf],
        dependency_sbom: &[DependencySbomEntry],
    ) -> Self {
        let mut profile = Self::default();

        for path in files {
            let file_name = path
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or_default();
            let rel = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .to_ascii_lowercase();

            if rel.contains("k8s") || rel.contains("ingress") {
                profile.middleware_targets.insert("k8s".to_string());
            }
            if rel.contains("nginx") {
                profile.middleware_targets.insert("nginx".to_string());
            }
            if rel.contains("httpd") || rel.contains("apache") {
                profile.middleware_targets.insert("httpd".to_string());
            }
            if rel.contains("envoy") {
                profile.middleware_targets.insert("envoy".to_string());
            }
            if rel.contains("istio") {
                profile.middleware_targets.insert("istio".to_string());
            }
            if rel.contains("haproxy") {
                profile.middleware_targets.insert("haproxy".to_string());
            }
            if rel.contains("traefik") {
                profile.middleware_targets.insert("traefik".to_string());
            }

            match file_name {
                "pom.xml" if profile.java_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 256 * 1024) {
                        profile.java_version = parse_java_version_from_pom(&text);
                    }
                }
                "build.gradle" | "build.gradle.kts" if profile.java_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 256 * 1024) {
                        profile.java_version = parse_java_version_from_gradle(&text);
                    }
                }
                "package.json" if profile.node_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 512 * 1024) {
                        profile.node_version = parse_node_version_from_package_json(&text);
                    }
                }
                "go.mod" if profile.go_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 128 * 1024) {
                        profile.go_version = parse_go_version_from_go_mod(&text);
                    }
                }
                "Cargo.toml" if profile.rust_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 256 * 1024) {
                        profile.rust_version = parse_rust_version_from_cargo_toml(&text);
                    }
                }
                "Gemfile" if profile.ruby_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 128 * 1024) {
                        profile.ruby_version = parse_ruby_version_from_gemfile(&text);
                    }
                }
                ".ruby-version" if profile.ruby_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 16 * 1024) {
                        profile.ruby_version = parse_ruby_version_file(&text);
                    }
                }
                "pyproject.toml" if profile.python_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 256 * 1024) {
                        profile.python_version = parse_python_version_from_pyproject(&text);
                    }
                }
                "runtime.txt" if profile.python_version.is_none() => {
                    if let Some(text) = read_text_limited(path, 32 * 1024) {
                        profile.python_version = parse_python_version_from_runtime_txt(&text);
                    }
                }
                _ => {}
            }
        }

        for dep in dependency_sbom {
            let name = dep.name.to_ascii_lowercase();
            profile.dependency_signals.insert(name.clone());
            if dep.purl.starts_with("pkg:") {
                profile
                    .dependency_signals
                    .insert(dep.purl.to_ascii_lowercase());
            }
            if name.contains("bouncycastle") || name.contains("bcprov") || name.contains("bcpkix") {
                profile
                    .dependency_signals
                    .insert("java-bouncycastle".to_string());
            }
            if name.contains("jsonwebtoken") || name == "jose" {
                profile.dependency_signals.insert("jwt-lib".to_string());
            }
            if name.contains("openssl") {
                profile.dependency_signals.insert("openssl".to_string());
            }
        }

        profile
    }

    pub fn has_dependency_signal(&self, token: &str) -> bool {
        self.dependency_signals
            .contains(&token.to_ascii_lowercase())
    }
}

pub fn annotate_findings(findings: &mut [Finding], profile: &RuntimeProfile) {
    for finding in findings {
        finding.recommended_actions = recommend_for_finding(finding, profile);
    }
}

fn recommend_for_finding(finding: &Finding, profile: &RuntimeProfile) -> Vec<RecommendedAction> {
    let mut actions = Vec::new();
    let language = detect_language(finding);
    let category = finding.category.to_ascii_lowercase();

    add_middleware_pqc_signature_notes(&mut actions, finding);

    if finding.evidence.r#type == "private_key" {
        push_action(
            &mut actions,
            RecommendedAction {
                action_id: "incident.private-key-rotation".to_string(),
                title: "Rotate exposed private keys and revoke affected credentials".to_string(),
                priority: "immediate".to_string(),
                rationale: "Private key material was detected. This is an active key-management incident and must be remediated immediately.".to_string(),
                steps: vec![
                    "Revoke/disable the exposed key and issue a replacement key pair.".to_string(),
                    "Invalidate sessions/tokens signed by the compromised key if applicable.".to_string(),
                    "Move keys to a secret manager or HSM and enforce commit scanning in CI.".to_string(),
                ],
                references: vec![
                    "https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html".to_string(),
                ],
                code_examples: Vec::new(),
            },
        );
    }

    if finding.risk.as_str() == "quantum-vulnerable" {
        push_action(
            &mut actions,
            RecommendedAction {
                action_id: "plan.hybrid-migration".to_string(),
                title: "Adopt a phased hybrid PQC migration plan".to_string(),
                priority: "near-term".to_string(),
                rationale: "The detected primitive is vulnerable to cryptographically relevant quantum attacks. Hybrid rollout minimizes compatibility risk.".to_string(),
                steps: vec![
                    "Inventory all call sites using this algorithm and owner teams.".to_string(),
                    "Introduce dual-stack or hybrid mode (classical + PQC) before hard cut-over.".to_string(),
                    "Define retirement dates for legacy classical algorithms.".to_string(),
                ],
                references: vec![
                    "https://www.nist.gov/itl/post-quantum-cryptography".to_string(),
                    "https://csrc.nist.gov/pubs/fips/203/final".to_string(),
                    "https://csrc.nist.gov/pubs/fips/204/final".to_string(),
                ],
                code_examples: Vec::new(),
            },
        );
    }

    match category.as_str() {
        "jwt" => add_jwt_actions(&mut actions, finding, language, profile),
        "tls" => add_tls_actions(&mut actions, finding, language, profile),
        "pki" => add_pki_actions(&mut actions, finding, language, profile),
        "cryptoapi" => add_crypto_api_actions(&mut actions, finding, language, profile),
        _ => {}
    }

    if finding.rule_id.starts_with("DEP_") {
        push_action(
            &mut actions,
            RecommendedAction {
                action_id: "dependency.track-pqc-readiness".to_string(),
                title: "Track PQC readiness of dependent libraries".to_string(),
                priority: "planned".to_string(),
                rationale: "Dependency-level crypto libraries strongly influence migration options and operational timelines.".to_string(),
                steps: vec![
                    "Record current versions in dependency-sbom.json and define upgrade targets.".to_string(),
                    "Subscribe to release notes for PQC feature availability and API changes.".to_string(),
                    "Run integration tests for PQC or hybrid modes before promoting to production.".to_string(),
                ],
                references: vec![
                    "https://cyclonedx.org/".to_string(),
                    "https://spdx.dev/".to_string(),
                ],
                code_examples: Vec::new(),
            },
        );
    }

    actions
}

fn detect_middleware_targets_for_finding(finding: &Finding) -> Vec<&'static str> {
    let category = finding.category.to_ascii_lowercase();
    if category != "middleware" && category != "tls" {
        return Vec::new();
    }

    let mut targets = Vec::new();
    let rule = finding.rule_id.to_ascii_lowercase();
    let location = finding.location.file.to_ascii_lowercase();

    let mut push_if = |target: &'static str, matched: bool| {
        if matched && !targets.contains(&target) {
            targets.push(target);
        }
    };

    push_if(
        "k8s",
        rule.starts_with("k8s_")
            || location.contains("k8s")
            || location.contains("kubernetes")
            || location.contains("ingress")
            || location.contains("gatewayapi"),
    );
    push_if(
        "nginx",
        rule.starts_with("nginx_") || location.contains("nginx"),
    );
    push_if(
        "envoy",
        rule.starts_with("envoy_") || location.contains("envoy"),
    );
    push_if(
        "istio",
        rule.starts_with("istio_") || location.contains("istio"),
    );
    push_if(
        "haproxy",
        rule.starts_with("haproxy_") || location.contains("haproxy"),
    );
    push_if(
        "traefik",
        rule.starts_with("traefik_") || location.contains("traefik"),
    );

    targets
}

fn add_middleware_pqc_signature_notes(actions: &mut Vec<RecommendedAction>, finding: &Finding) {
    for target in detect_middleware_targets_for_finding(finding) {
        let action = match target {
            "k8s" => RecommendedAction {
                action_id: "k8s.pqc-signature-readiness-note".to_string(),
                title: "Validate Kubernetes PQC signature readiness before production rollout"
                    .to_string(),
                priority: "advisory".to_string(),
                rationale: "As described in Kubernetes guidance dated July 18, 2025, ecosystem readiness is currently stronger for hybrid key exchange than for end-to-end PQC certificate signatures.".to_string(),
                steps: vec![
                    "Treat TLS modernization (TLS 1.2+/1.3) and hybrid KEM pilots as the immediate baseline.".to_string(),
                    "Do not assume ML-DSA/SLH-DSA certificate signatures are natively interoperable across ingress/controller/mesh/CA/client paths.".to_string(),
                    "Run staged compatibility tests before enforcing PQC signatures in production.".to_string(),
                ],
                references: vec![
                    "https://kubernetes.io/blog/2025/07/18/pqc-in-k8s/".to_string(),
                    "https://go.dev/doc/go1.26".to_string(),
                ],
                code_examples: Vec::new(),
            },
            "nginx" => RecommendedAction {
                action_id: "nginx.pqc-signature-readiness-note".to_string(),
                title: "Validate nginx/OpenSSL PQC signature interoperability before rollout"
                    .to_string(),
                priority: "advisory".to_string(),
                rationale: "nginx relies on TLS library behavior; hybrid KEM support and PQC certificate-signature interoperability can differ by OpenSSL/provider build and client ecosystem.".to_string(),
                steps: vec![
                    "Keep TLS 1.2+/1.3 hardening as baseline and pilot hybrid KEM first.".to_string(),
                    "Test ML-DSA/SLH-DSA certificate-chain interoperability across nginx, upstream TLS endpoints, and client stacks.".to_string(),
                    "Roll out behind canary and keep fallback policy until interoperability evidence is complete.".to_string(),
                ],
                references: vec![
                    "https://nginx.org/en/docs/http/ngx_http_ssl_module.html".to_string(),
                    "https://openssl-library.org/post/2025-04-08-openssl-3.5-final/".to_string(),
                ],
                code_examples: Vec::new(),
            },
            "envoy" => RecommendedAction {
                action_id: "envoy.pqc-signature-readiness-note".to_string(),
                title: "Validate Envoy PQC signature compatibility in control/data planes"
                    .to_string(),
                priority: "advisory".to_string(),
                rationale: "Envoy TLS behavior depends on configured providers and peer capabilities; PQC signature readiness can vary between mesh edges, sidecars, and external clients.".to_string(),
                steps: vec![
                    "Harden min/max TLS versions and remove legacy RSA suites first.".to_string(),
                    "Verify PQC certificate-signature compatibility in both downstream and upstream clusters.".to_string(),
                    "Gate production rollout with staged canary tests and explicit fallback paths.".to_string(),
                ],
                references: vec![
                    "https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/security/ssl.html".to_string(),
                    "https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/tls.proto".to_string(),
                ],
                code_examples: Vec::new(),
            },
            "istio" => RecommendedAction {
                action_id: "istio.pqc-signature-readiness-note".to_string(),
                title: "Validate Istio mesh-wide PQC signature interoperability before enforcement"
                    .to_string(),
                priority: "advisory".to_string(),
                rationale: "Istio rollout spans gateways, sidecars, and service-to-service traffic; mesh-wide certificate-signature interoperability must be validated end-to-end.".to_string(),
                steps: vec![
                    "Standardize TLS modernization policy in DestinationRule/Gateway resources.".to_string(),
                    "Test PQC signature compatibility across ingress gateway, sidecars, and external clients.".to_string(),
                    "Roll out with progressive policies and keep compatibility fallbacks during migration.".to_string(),
                ],
                references: vec![
                    "https://istio.io/latest/docs/reference/config/networking/destination-rule/".to_string(),
                    "https://istio.io/latest/docs/reference/config/networking/gateway/".to_string(),
                ],
                code_examples: Vec::new(),
            },
            "haproxy" => RecommendedAction {
                action_id: "haproxy.pqc-signature-readiness-note".to_string(),
                title: "Validate HAProxy TLS stack PQC signature readiness before production"
                    .to_string(),
                priority: "advisory".to_string(),
                rationale: "HAProxy TLS capabilities are tied to the linked TLS stack and runtime policy; PQC signature behavior may differ by build and client compatibility.".to_string(),
                steps: vec![
                    "Keep TLS 1.2+/1.3 policy and remove legacy protocol/cipher options first.".to_string(),
                    "Run interoperability tests for PQC certificate signatures with all major client types.".to_string(),
                    "Use phased rollout with rollback/fallback controls while collecting compatibility evidence.".to_string(),
                ],
                references: vec![
                    "https://www.haproxy.com/documentation/haproxy-configuration-tutorials/security/ssl-tls/".to_string(),
                ],
                code_examples: Vec::new(),
            },
            "traefik" => RecommendedAction {
                action_id: "traefik.pqc-signature-readiness-note".to_string(),
                title: "Validate Traefik PQC signature interoperability with all entrypoints"
                    .to_string(),
                priority: "advisory".to_string(),
                rationale: "Traefik TLS options and provider integrations can produce environment-specific PQC signature behavior that must be validated before strict enforcement.".to_string(),
                steps: vec![
                    "Raise minimum TLS version and remove legacy suites/options as baseline.".to_string(),
                    "Test PQC certificate-signature compatibility for edge traffic, internal services, and automation clients.".to_string(),
                    "Apply staged rollout with explicit compatibility fallback configuration.".to_string(),
                ],
                references: vec![
                    "https://doc.traefik.io/traefik/https/tls/".to_string(),
                    "https://doc.traefik.io/traefik/routing/routers/#tls".to_string(),
                ],
                code_examples: Vec::new(),
            },
            _ => continue,
        };
        push_action(actions, action);
    }
}

fn add_jwt_actions(
    actions: &mut Vec<RecommendedAction>,
    finding: &Finding,
    language: Option<&'static str>,
    profile: &RuntimeProfile,
) {
    push_action(
        actions,
        RecommendedAction {
            action_id: "jwt.remove-rsa-algorithms".to_string(),
            title: "Stop issuing new JWTs with RSA/PKCS#1/ECDSA-only algorithms".to_string(),
            priority: "near-term".to_string(),
            rationale: "JWT signatures using classical public-key algorithms become long-term high risk under quantum adversaries.".to_string(),
            steps: vec![
                "Restrict accepted alg values and explicitly deny weak/legacy defaults.".to_string(),
                "Introduce short-lived tokens during the migration phase.".to_string(),
                "Prepare verifier rollout before signer rollout to avoid compatibility outages.".to_string(),
            ],
            references: vec!["https://www.rfc-editor.org/rfc/rfc7519".to_string()],
            code_examples: Vec::new(),
        },
    );

    match language {
        Some("java") => add_java_actions(actions, finding, profile),
        Some("javascript") | Some("typescript") => {
            let node24 = version_at_least(profile.node_version.as_deref(), 24);
            let title = if node24 {
                "Use Node.js WebCrypto PQC algorithms (ML-DSA / ML-KEM)"
            } else {
                "Upgrade Node.js runtime for built-in WebCrypto PQC support"
            };
            push_action(
                actions,
                RecommendedAction {
                    action_id: "node.webcrypto.pqc".to_string(),
                    title: title.to_string(),
                    priority: if node24 {
                        "near-term".to_string()
                    } else {
                        "planned".to_string()
                    },
                    rationale: "Node.js runtime level determines whether built-in PQC APIs are available for direct migration.".to_string(),
                    steps: vec![
                        "Validate runtime support for ML-DSA/ML-KEM in your Node.js LTS baseline.".to_string(),
                        "Migrate signing and key exchange to WebCrypto and phase out classical-only JWT usage.".to_string(),
                    ],
                    references: vec!["https://nodejs.org/api/webcrypto.html".to_string()],
                    code_examples: vec![CodeExample {
                        language: language.unwrap_or("javascript").to_string(),
                        before: "import jwt from \"jsonwebtoken\";\nconst token = jwt.sign(payload, privateKey, { algorithm: \"RS256\" });".to_string(),
                        after: "// Replace RS256 token-signing path with a PQC-capable signing service\n// or WebCrypto ML-DSA workflow once runtime support is enabled.\nconst token = await signWithPqc(payload);".to_string(),
                    }],
                },
            );
        }
        Some("go") => {
            push_action(
                actions,
                RecommendedAction {
                    action_id: "go.hybrid-kem".to_string(),
                    title: "Use hybrid key establishment in Go services".to_string(),
                    priority: "near-term".to_string(),
                    rationale: "Go migration is typically introduced via hybrid KEM for transport/session keys while JWT signing transitions separately.".to_string(),
                    steps: vec![
                        "Use `crypto/mlkem` for key encapsulation where available.".to_string(),
                        "Keep compatibility by running classical + PQC in parallel during rollout.".to_string(),
                    ],
                    references: vec!["https://pkg.go.dev/crypto/mlkem".to_string()],
                    code_examples: Vec::new(),
                },
            );
        }
        Some("python") => {
            push_action(
                actions,
                RecommendedAction {
                    action_id: "python.signing-gateway".to_string(),
                    title: "Introduce a PQC signing gateway for Python JWT workloads".to_string(),
                    priority: "planned".to_string(),
                    rationale: "Python ecosystem PQC support is evolving, so isolating signing behind an internal API reduces lock-in.".to_string(),
                    steps: vec![
                        "Abstract JWT signing behind an interface to swap implementation safely.".to_string(),
                        "Evaluate `liboqs-python` or provider-backed OpenSSL paths for pilot environments.".to_string(),
                    ],
                    references: vec!["https://github.com/open-quantum-safe/liboqs-python".to_string()],
                    code_examples: Vec::new(),
                },
            );
        }
        Some("rust") => {
            push_action(
                actions,
                RecommendedAction {
                    action_id: "rust.oqs-integration".to_string(),
                    title: "Refactor Rust crypto calls to trait-based signer/KEM abstraction".to_string(),
                    priority: "near-term".to_string(),
                    rationale: "Rust migrations are safer when algorithm choices are hidden behind typed traits and feature flags.".to_string(),
                    steps: vec![
                        "Replace direct RSA calls with trait-based signer interfaces.".to_string(),
                        "Add PQC/hybrid implementations behind cargo features and integration tests.".to_string(),
                    ],
                    references: vec!["https://github.com/open-quantum-safe/liboqs-rust".to_string()],
                    code_examples: Vec::new(),
                },
            );
        }
        Some("ruby") => {
            push_action(
                actions,
                RecommendedAction {
                    action_id: "ruby.crypto-service-boundary".to_string(),
                    title: "Move Ruby JWT signing to a provider-backed crypto boundary".to_string(),
                    priority: "planned".to_string(),
                    rationale: "Ruby runtime support for PQC primitives is commonly mediated through OpenSSL providers or external services.".to_string(),
                    steps: vec![
                        "Isolate OpenSSL/JWT calls in one service object.".to_string(),
                        "Integrate a PQC-capable signer via provider or sidecar.".to_string(),
                    ],
                    references: vec!["https://docs.ruby-lang.org/en/master/OpenSSL/PKey.html".to_string()],
                    code_examples: Vec::new(),
                },
            );
        }
        _ => {}
    }
}

fn add_java_actions(
    actions: &mut Vec<RecommendedAction>,
    _finding: &Finding,
    profile: &RuntimeProfile,
) {
    let java25_or_newer = version_at_least(profile.java_version.as_deref(), 25);
    let java24_or_newer = version_at_least(profile.java_version.as_deref(), 24);

    if java25_or_newer || java24_or_newer {
        push_action(
            actions,
            RecommendedAction {
                action_id: "java.jca-ml-dsa-ml-kem".to_string(),
                title: "Use Java standard JCA names for ML-DSA / ML-KEM".to_string(),
                priority: "near-term".to_string(),
                rationale: "Modern JDKs expose standardized PQC algorithm names through JCA, reducing vendor lock-in.".to_string(),
                steps: vec![
                    "Replace RSA-only `Signature.getInstance(...)` calls with `ML-DSA` where policy allows.".to_string(),
                    "Adopt ML-KEM for key establishment in hybrid TLS/key exchange designs.".to_string(),
                    "Keep dual verification during migration to preserve compatibility.".to_string(),
                ],
                references: vec![
                    "https://openjdk.org/jeps/496".to_string(),
                    "https://openjdk.org/jeps/497".to_string(),
                ],
                code_examples: vec![CodeExample {
                    language: "java".to_string(),
                    before: "Signature sig = Signature.getInstance(\"SHA256withRSA\");\nsig.initSign(privateKey);\nsig.update(payload);\nbyte[] signature = sig.sign();".to_string(),
                    after: "Signature sig = Signature.getInstance(\"ML-DSA\");\nsig.initSign(privateKey);\nsig.update(payload);\nbyte[] signature = sig.sign();\n// Keep legacy verification path during staged rollout.".to_string(),
                }],
            },
        );
    } else {
        push_action(
            actions,
            RecommendedAction {
                action_id: "java.runtime-upgrade-for-pqc".to_string(),
                title: "Upgrade Java runtime baseline for standard PQC APIs".to_string(),
                priority: "planned".to_string(),
                rationale: "Detected Java version appears older than JDK releases with standardized PQC APIs.".to_string(),
                steps: vec![
                    "Target JDK 24+ for ML-KEM and ML-DSA support planning.".to_string(),
                    "Pilot compatibility testing in CI before production migration.".to_string(),
                ],
                references: vec![
                    "https://openjdk.org/jeps/496".to_string(),
                    "https://openjdk.org/jeps/497".to_string(),
                ],
                code_examples: Vec::new(),
            },
        );
    }

    if profile.has_dependency_signal("java-bouncycastle") || !java24_or_newer {
        push_action(
            actions,
            RecommendedAction {
                action_id: "java.bouncycastle-pqc".to_string(),
                title: "Provide BouncyCastle PQC fallback path".to_string(),
                priority: "near-term".to_string(),
                rationale: "BouncyCastle can be used as a pragmatic transition option when standard runtime support is unavailable or incomplete.".to_string(),
                steps: vec![
                    "Add BC provider initialization at bootstrap.".to_string(),
                    "Use BC ML-DSA / ML-KEM parameter specs for migration pilot implementations.".to_string(),
                    "Document provider ordering and FIPS/compliance impact.".to_string(),
                ],
                references: vec![
                    "https://www.bouncycastle.org/java.html".to_string(),
                    "https://downloads.bouncycastle.org/java/docs/bcprov-jdk18on-javadoc/".to_string(),
                ],
                code_examples: vec![CodeExample {
                    language: "java".to_string(),
                    before: "Security.addProvider(new BouncyCastleProvider());\nSignature sig = Signature.getInstance(\"SHA256withRSA\", \"BC\");".to_string(),
                    after: "Security.addProvider(new BouncyCastlePQCProvider());\nSignature sig = Signature.getInstance(\"ML-DSA\", \"BCPQC\");\n// Configure parameter set (example): MLDSAParameterSpec.ml_dsa_65".to_string(),
                }],
            },
        );
    }
}

fn add_tls_actions(
    actions: &mut Vec<RecommendedAction>,
    finding: &Finding,
    _language: Option<&'static str>,
    profile: &RuntimeProfile,
) {
    push_action(
        actions,
        RecommendedAction {
            action_id: "tls.remove-rsa-kex".to_string(),
            title: "Eliminate RSA key exchange and enforce TLS 1.2+".to_string(),
            priority: "near-term".to_string(),
            rationale: "RSA key exchange and legacy protocol versions are high-priority migration blockers for PQC readiness.".to_string(),
            steps: vec![
                "Set minimum protocol version to TLS 1.2 or 1.3.".to_string(),
                "Prefer ECDHE today and plan hybrid KEM suites for next phase.".to_string(),
                "Continuously test interoperability across clients and load balancers.".to_string(),
            ],
            references: vec!["https://www.rfc-editor.org/rfc/rfc8446".to_string()],
            code_examples: Vec::new(),
        },
    );

    let location = finding.location.file.to_ascii_lowercase();
    if location.contains("nginx") || profile.middleware_targets.contains("nginx") {
        push_action(
            actions,
            RecommendedAction {
                action_id: "tls.nginx-hardening".to_string(),
                title: "Harden nginx TLS policy for migration".to_string(),
                priority: "near-term".to_string(),
                rationale: "nginx config was detected and can be remediated with explicit protocol and cipher policies.".to_string(),
                steps: vec![
                    "Set `ssl_protocols TLSv1.2 TLSv1.3;`.".to_string(),
                    "Remove RSA key-exchange suites and keep forward-secret suites only.".to_string(),
                ],
                references: vec!["https://nginx.org/en/docs/http/ngx_http_ssl_module.html".to_string()],
                code_examples: Vec::new(),
            },
        );
    }

    if location.contains("httpd") || profile.middleware_targets.contains("httpd") {
        push_action(
            actions,
            RecommendedAction {
                action_id: "tls.httpd-hardening".to_string(),
                title: "Harden Apache httpd SSL/TLS directives".to_string(),
                priority: "near-term".to_string(),
                rationale: "Apache TLS settings can be migrated safely via explicit protocol and cipher controls.".to_string(),
                steps: vec![
                    "Set `SSLProtocol -all +TLSv1.2 +TLSv1.3`.".to_string(),
                    "Review `SSLCipherSuite` and remove RSA key-exchange suites.".to_string(),
                ],
                references: vec!["https://httpd.apache.org/docs/trunk/mod/mod_ssl.html".to_string()],
                code_examples: Vec::new(),
            },
        );
    }

    if location.contains("envoy") || profile.middleware_targets.contains("envoy") {
        push_action(
            actions,
            RecommendedAction {
                action_id: "tls.envoy-hardening".to_string(),
                title: "Set modern TLS policy in Envoy".to_string(),
                priority: "near-term".to_string(),
                rationale: "Envoy control-plane config supports strict min/max protocol and cipher suite policy.".to_string(),
                steps: vec![
                    "Set `tls_minimum_protocol_version: TLSv1_2` or higher.".to_string(),
                    "Remove RSA-dependent suites from `cipher_suites`.".to_string(),
                ],
                references: vec!["https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/tls.proto".to_string()],
                code_examples: Vec::new(),
            },
        );
    }

    if location.contains("istio") || profile.middleware_targets.contains("istio") {
        push_action(
            actions,
            RecommendedAction {
                action_id: "tls.istio-hardening".to_string(),
                title: "Align Istio TLS settings with PQC migration policy".to_string(),
                priority: "near-term".to_string(),
                rationale: "Istio DestinationRule/Gateway policies should mirror TLS modernization and hybrid-readiness goals.".to_string(),
                steps: vec![
                    "Set minimum TLS version to 1.2+ in mesh and gateway resources.".to_string(),
                    "Reduce classical RSA dependency in configured cipher sets.".to_string(),
                ],
                references: vec!["https://istio.io/latest/docs/reference/config/networking/destination-rule/".to_string()],
                code_examples: Vec::new(),
            },
        );
    }

    if location.contains("ingress") || profile.middleware_targets.contains("k8s") {
        push_action(
            actions,
            RecommendedAction {
                action_id: "tls.k8s-ingress-hardening".to_string(),
                title: "Update Kubernetes ingress/gateway TLS policy".to_string(),
                priority: "near-term".to_string(),
                rationale: "Kubernetes edge resources are frequent concentration points for legacy TLS policy.".to_string(),
                steps: vec![
                    "Set explicit TLS 1.2+ minimum version in ingress controller settings.".to_string(),
                    "Use policy-as-code checks to block reintroduction of legacy suites.".to_string(),
                ],
                references: vec!["https://kubernetes.io/docs/concepts/services-networking/ingress/".to_string()],
                code_examples: Vec::new(),
            },
        );
    }
}

fn add_pki_actions(
    actions: &mut Vec<RecommendedAction>,
    _finding: &Finding,
    _language: Option<&'static str>,
    _profile: &RuntimeProfile,
) {
    push_action(
        actions,
        RecommendedAction {
            action_id: "pki.hybrid-certificate-roadmap".to_string(),
            title: "Prepare hybrid certificate issuance and trust-store rollout".to_string(),
            priority: "near-term".to_string(),
            rationale: "PKI updates require coordinated CA, chain, and relying-party compatibility planning.".to_string(),
            steps: vec![
                "Inventory certificate signature algorithms and key sizes from CBOM/report output.".to_string(),
                "Pilot hybrid chain issuance in non-production trust stores.".to_string(),
                "Define reissuance plan for quantum-vulnerable roots/intermediates/end-entity certs.".to_string(),
            ],
            references: vec![
                "https://csrc.nist.gov/pubs/fips/204/final".to_string(),
                "https://csrc.nist.gov/pubs/fips/205/final".to_string(),
            ],
            code_examples: Vec::new(),
        },
    );
}

fn add_crypto_api_actions(
    actions: &mut Vec<RecommendedAction>,
    finding: &Finding,
    language: Option<&'static str>,
    profile: &RuntimeProfile,
) {
    if language == Some("java") {
        add_java_actions(actions, finding, profile);
        return;
    }

    push_action(
        actions,
        RecommendedAction {
            action_id: "cryptoapi.abstraction-layer".to_string(),
            title: "Introduce crypto abstraction boundary for algorithm agility".to_string(),
            priority: "near-term".to_string(),
            rationale: "Direct algorithm calls create high migration cost. A boundary layer enables staged classical-to-PQC transitions.".to_string(),
            steps: vec![
                "Wrap signing, verification, keygen, and key exchange behind stable interfaces.".to_string(),
                "Add policy controls to select classical, hybrid, or PQC paths at runtime.".to_string(),
            ],
            references: vec!["https://www.nist.gov/itl/post-quantum-cryptography".to_string()],
            code_examples: Vec::new(),
        },
    );
}

fn detect_language(finding: &Finding) -> Option<&'static str> {
    if let Some(lang) = finding.evidence.metadata.get("language") {
        return canonical_language_name(lang);
    }

    let path = finding.location.file.to_ascii_lowercase();
    let ext = Path::new(&path)
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or_default();

    match ext {
        "java" => Some("java"),
        "js" | "mjs" | "cjs" | "jsx" => Some("javascript"),
        "ts" | "tsx" => Some("typescript"),
        "py" => Some("python"),
        "go" => Some("go"),
        "rs" => Some("rust"),
        "rb" => Some("ruby"),
        _ => None,
    }
}

fn canonical_language_name(raw: &str) -> Option<&'static str> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "java" => Some("java"),
        "javascript" | "js" => Some("javascript"),
        "typescript" | "ts" => Some("typescript"),
        "python" | "py" => Some("python"),
        "go" | "golang" => Some("go"),
        "rust" | "rs" => Some("rust"),
        "ruby" | "rb" => Some("ruby"),
        _ => None,
    }
}

fn push_action(actions: &mut Vec<RecommendedAction>, action: RecommendedAction) {
    if actions.iter().any(|v| v.action_id == action.action_id) {
        return;
    }
    actions.push(action);
}

fn read_text_limited(path: &Path, max_bytes: usize) -> Option<String> {
    let meta = fs::metadata(path).ok()?;
    if meta.len() as usize > max_bytes {
        return None;
    }
    fs::read_to_string(path).ok()
}

fn parse_java_version_from_pom(text: &str) -> Option<String> {
    extract_xml_tag(text, "maven.compiler.release")
        .or_else(|| extract_xml_tag(text, "java.version"))
        .or_else(|| extract_xml_tag(text, "maven.compiler.source"))
        .map(|v| normalize_version_hint(&v))
}

fn parse_java_version_from_gradle(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.contains("JavaLanguageVersion.of(") {
            if let Some(start) = trimmed.find("JavaLanguageVersion.of(") {
                let chunk = &trimmed[start + "JavaLanguageVersion.of(".len()..];
                if let Some(end) = chunk.find(')') {
                    return Some(normalize_version_hint(&chunk[..end]));
                }
            }
        }
        if trimmed.starts_with("sourceCompatibility") || trimmed.starts_with("targetCompatibility")
        {
            if let Some(version) = trimmed.split('=').nth(1) {
                return Some(normalize_version_hint(version));
            }
        }
    }
    None
}

fn parse_node_version_from_package_json(text: &str) -> Option<String> {
    let value = serde_json::from_str::<JsonValue>(text).ok()?;
    let engines = value.get("engines")?.as_object()?;
    let raw = engines.get("node")?.as_str()?;
    Some(normalize_version_hint(raw))
}

fn parse_go_version_from_go_mod(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("go ") {
            return Some(normalize_version_hint(rest));
        }
    }
    None
}

fn parse_rust_version_from_cargo_toml(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("rust-version") {
            if let Some((_, rhs)) = rest.split_once('=') {
                return Some(normalize_version_hint(rhs));
            }
        }
    }
    None
}

fn parse_ruby_version_from_gemfile(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("ruby ") {
            return Some(normalize_version_hint(rest));
        }
    }
    None
}

fn parse_ruby_version_file(text: &str) -> Option<String> {
    text.lines()
        .map(|line| line.trim())
        .find(|line| !line.is_empty())
        .map(normalize_version_hint)
}

fn parse_python_version_from_pyproject(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("requires-python") {
            if let Some((_, rhs)) = rest.split_once('=') {
                return Some(normalize_version_hint(rhs));
            }
        }
    }
    None
}

fn parse_python_version_from_runtime_txt(text: &str) -> Option<String> {
    text.lines().next().map(normalize_version_hint)
}

fn extract_xml_tag(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = text.find(&open)? + open.len();
    let end = text[start..].find(&close)? + start;
    Some(text[start..end].trim().to_string())
}

fn normalize_version_hint(raw: &str) -> String {
    raw.trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_string()
}

fn version_at_least(raw: Option<&str>, expected_major: u32) -> bool {
    let value = match raw {
        Some(v) => v,
        None => return false,
    };
    let major = first_number(value).unwrap_or(0);
    major >= expected_major
}

fn first_number(raw: &str) -> Option<u32> {
    let mut started = false;
    let mut digits = String::new();

    for ch in raw.chars() {
        if ch.is_ascii_digit() {
            started = true;
            digits.push(ch);
        } else if started {
            break;
        }
    }

    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Evidence, Finding, Location};
    use pqc_scan_rules::{Risk, Severity};
    use std::collections::BTreeMap;

    fn make_finding(rule_id: &str, category: &str, file: &str) -> Finding {
        Finding {
            finding_id: "test-finding".to_string(),
            rule_id: rule_id.to_string(),
            category: category.to_string(),
            risk: Risk::NonQuantumRisk,
            severity: Severity::Medium,
            confidence: 0.8,
            description: "test".to_string(),
            migration_hint: "hint".to_string(),
            location: Location {
                file: file.to_string(),
                line: 1,
                column: 1,
            },
            evidence: Evidence {
                r#type: "regex_match".to_string(),
                r#match: "test".to_string(),
                snippet_preview: "test".to_string(),
                metadata: BTreeMap::new(),
            },
            recommended_actions: Vec::new(),
            source_snippet: None,
        }
    }

    fn assert_has_action(rule_id: &str, file: &str, expected_action_id: &str) {
        let mut findings = vec![make_finding(rule_id, "Middleware", file)];
        annotate_findings(&mut findings, &RuntimeProfile::default());
        assert!(
            findings[0]
                .recommended_actions
                .iter()
                .any(|a| a.action_id == expected_action_id),
            "expected action {expected_action_id} to be present for {rule_id}:{file}"
        );
    }

    #[test]
    fn adds_middleware_readiness_notes_for_all_supported_targets() {
        assert_has_action(
            "K8S_INGRESS_SSL_PROTOCOLS_LEGACY",
            "manifests/ingress.yaml",
            "k8s.pqc-signature-readiness-note",
        );
        assert_has_action(
            "NGINX_SSL_PROTOCOLS_LEGACY",
            "infra/nginx.conf",
            "nginx.pqc-signature-readiness-note",
        );
        assert_has_action(
            "ENVOY_TLS_MIN_VERSION_LEGACY",
            "mesh/envoy.yaml",
            "envoy.pqc-signature-readiness-note",
        );
        assert_has_action(
            "ISTIO_DESTINATIONRULE_TLSV1",
            "mesh/istio/dr.yaml",
            "istio.pqc-signature-readiness-note",
        );
        assert_has_action(
            "HAPROXY_BIND_TLSV1",
            "edge/haproxy.cfg",
            "haproxy.pqc-signature-readiness-note",
        );
        assert_has_action(
            "TRAEFIK_TLS_OPTIONS_LEGACY",
            "edge/traefik.yaml",
            "traefik.pqc-signature-readiness-note",
        );
    }

    #[test]
    fn does_not_add_middleware_note_for_non_middleware_category() {
        let mut findings = vec![make_finding("JWT_RS256", "JWT", "src/token_service.go")];
        annotate_findings(&mut findings, &RuntimeProfile::default());
        assert!(
            findings[0]
                .recommended_actions
                .iter()
                .all(|a| !a.action_id.ends_with("pqc-signature-readiness-note")),
            "middleware readiness note should not be added to JWT finding"
        );
    }
}
