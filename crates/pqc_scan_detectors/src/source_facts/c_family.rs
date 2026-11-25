//! Bounded C/C++ source references. No preprocessing, linking or crypto executes.
mod values;
use super::{
    builtin,
    context::{children, Context},
    Facts,
};
use pqc_scan_rules::RuleSet;
use tree_sitter::Node;
use values::{Role, Values};
const API: &str = "C_FAMILY_CRYPTO_API";
const PQC: &str = "C_FAMILY_PQC_API";
const TLS: &str = "C_FAMILY_TLS_REFERENCE";
fn rule_id(algorithm: Option<&str>) -> &'static str {
    if algorithm.is_some_and(|a| a.starts_with("ML-") || a.starts_with("SLH-")) {
        PQC
    } else {
        API
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectorKind {
    EvpId,
    EvpName,
    OqsKem,
    OqsSig,
}
fn exact_algorithm(value: &str) -> Option<String> {
    let value = match value {
        "ED25519" => "Ed25519",
        value => value,
    };
    if !(matches!(value, "RSA" | "DSA" | "DH" | "EC")
        || matches!(value, "Ed25519" | "X25519")
        || value.starts_with("ML-KEM-")
        || value.starts_with("ML-DSA-")
        || value.starts_with("SLH-DSA-")
        || value.starts_with("Kyber")
        || value.starts_with("Dilithium"))
    {
        return None;
    }
    pqc_scan_core::observation::AlgorithmSpec::source_api(value, &Default::default())
        .map(|_| value.into())
}
fn selected_literal(value: &str, kind: SelectorKind) -> Option<String> {
    if kind == SelectorKind::EvpId {
        return None;
    }
    let mapped = if kind == SelectorKind::OqsSig && value.starts_with("SLH_DSA_PURE_") {
        let (hash, set) = value.strip_prefix("SLH_DSA_PURE_")?.split_once('_')?;
        if !matches!(hash, "SHA2" | "SHAKE")
            || !matches!(set, "128S" | "128F" | "192S" | "192F" | "256S" | "256F")
        {
            return None;
        }
        format!("SLH-DSA-{hash}-{}", set.to_ascii_lowercase())
    } else {
        if kind == SelectorKind::OqsSig && value.starts_with("SLH-DSA-") {
            return None;
        }
        value.into()
    };
    let algorithm = exact_algorithm(&mapped)?;
    match kind {
        SelectorKind::OqsKem
            if !algorithm.starts_with("ML-KEM-") && !algorithm.starts_with("Kyber") =>
        {
            None
        }
        SelectorKind::OqsSig
            if !algorithm.starts_with("ML-DSA-")
                && !algorithm.starts_with("SLH-DSA-")
                && !algorithm.starts_with("Dilithium") =>
        {
            None
        }
        _ => Some(algorithm),
    }
}
fn low_level(name: &str) -> Option<&'static str> {
    Some(match name {
        "RSA_new"
        | "RSA_generate_key_ex"
        | "RSA_public_encrypt"
        | "RSA_private_decrypt"
        | "RSA_private_encrypt"
        | "RSA_public_decrypt"
        | "RSA_sign"
        | "RSA_verify" => "RSA",
        "DSA_new"
        | "DSA_generate_key"
        | "DSA_generate_parameters_ex"
        | "DSA_sign"
        | "DSA_verify" => "DSA",
        "DH_new" | "DH_generate_key" | "DH_generate_parameters_ex" | "DH_compute_key" => "DH",
        "EC_KEY_new" | "EC_KEY_new_by_curve_name" | "EC_KEY_generate_key" => "EC",
        "ECDSA_sign" | "ECDSA_verify" => "EC",
        "ECDH_compute_key" => "ECDH",
        "wc_RsaSSL_Sign" | "wc_RsaSSL_Verify" | "wc_RsaPublicEncrypt" | "wc_RsaPrivateDecrypt" => {
            "RSA"
        }
        "secp256k1_ecdsa_sign" | "secp256k1_ecdsa_verify" => "ECDSA",
        _ => return None,
    })
}
fn context_constructor(name: &str) -> bool {
    matches!(
        name,
        "EVP_PKEY_CTX_new_id"
            | "EVP_PKEY_CTX_new_from_name"
            | "EVP_PKEY_CTX_new"
            | "EVP_PKEY_CTX_new_from_pkey"
            | "EVP_PKEY_CTX_dup"
    )
}
fn raw_key_constructor(name: &str) -> bool {
    matches!(
        name,
        "EVP_PKEY_new_raw_private_key"
            | "EVP_PKEY_new_raw_public_key"
            | "EVP_PKEY_new_raw_private_key_ex"
            | "EVP_PKEY_new_raw_public_key_ex"
    )
}
fn context_operation(name: &str) -> bool {
    matches!(
        name,
        "EVP_PKEY_keygen_init"
            | "EVP_PKEY_keygen"
            | "EVP_PKEY_generate"
            | "EVP_PKEY_paramgen_init"
            | "EVP_PKEY_paramgen"
            | "EVP_PKEY_encapsulate_init"
            | "EVP_PKEY_encapsulate"
            | "EVP_PKEY_decapsulate_init"
            | "EVP_PKEY_decapsulate"
            | "EVP_PKEY_sign_init"
            | "EVP_PKEY_sign"
            | "EVP_PKEY_verify_init"
            | "EVP_PKEY_verify"
            | "EVP_PKEY_encrypt_init"
            | "EVP_PKEY_encrypt"
            | "EVP_PKEY_decrypt_init"
            | "EVP_PKEY_decrypt"
            | "EVP_PKEY_derive_init"
            | "EVP_PKEY_derive"
    )
}
fn digest_operation(name: &str) -> bool {
    matches!(
        name,
        "EVP_DigestSignInit"
            | "EVP_DigestVerifyInit"
            | "EVP_DigestSignInit_ex"
            | "EVP_DigestVerifyInit_ex"
    )
}
fn configuration(name: &str) -> bool {
    matches!(
        name,
        "OSSL_PROVIDER_load"
            | "OSSL_PROVIDER_load_ex"
            | "EVP_set_default_properties"
            | "SSL_CTX_set1_groups_list"
            | "SSL_set1_groups_list"
    )
}
fn known(name: &str) -> bool {
    low_level(name).is_some()
        || context_constructor(name)
        || raw_key_constructor(name)
        || context_operation(name)
        || digest_operation(name)
        || configuration(name)
        || matches!(
            name,
            "X509_verify_cert" | "Botan::PK_Signer" | "Botan::PK_Verifier"
        )
        || matches!(
            name,
            "OQS_KEM_new"
                | "OQS_SIG_new"
                | "OQS_KEM_keypair"
                | "OQS_KEM_encaps"
                | "OQS_KEM_decaps"
                | "OQS_SIG_keypair"
                | "OQS_SIG_sign"
                | "OQS_SIG_verify"
                | "oqs::KeyEncapsulation"
                | "oqs::Signature"
        )
}
fn function_name(ctx: &Context<'_, '_>, node: Node<'_>) -> Option<String> {
    let raw = ctx.source(node);
    if raw.len() > 512 {
        ctx.limit("c_family_api_identifier_bytes", 512, 512);
        return None;
    }
    Some(raw.split_whitespace().collect())
}
fn args(node: Node<'_>) -> Vec<Node<'_>> {
    if node.kind() == "parameter_declaration" {
        return node
            .child_by_field_name("declarator")
            .and_then(|d| d.child_by_field_name("parameters"))
            .map(children)
            .unwrap_or_default()
            .into_iter()
            .map(|n| {
                if n.child_by_field_name("declarator").is_none() {
                    n.child_by_field_name("type").unwrap_or(n)
                } else {
                    n
                }
            })
            .collect();
    }
    let arg = if matches!(node.kind(), "argument_list" | "initializer_list") {
        Some(node)
    } else {
        node.child_by_field_name("arguments")
    };
    arg.map(children)
        .unwrap_or_default()
        .into_iter()
        .filter(|n| !n.kind().contains("comment"))
        .collect()
}
// The C++ grammar can view an initializer's nested factory as a parameter.
// A reserved nullptr token cannot be a parameter type, so this bounded case
// is an expression. Possible declarations (including NULL/type names) stay out.
fn recovered_cpp_call<'a>(ctx: &Context<'a, '_>, declaration: Node<'a>) -> Option<Node<'a>> {
    if ctx.language != "cpp"
        || declaration.kind() != "declaration"
        || ctx.function_scope(ctx.scope(declaration)) == 0
    {
        return None;
    }
    let outer = declaration.child_by_field_name("declarator")?;
    if outer.kind() != "function_declarator" {
        return None;
    }
    let parameters = children(outer.child_by_field_name("parameters")?);
    if parameters.len() != 1 {
        return None;
    }
    let call = parameters[0];
    if call.kind() != "parameter_declaration" {
        return None;
    }
    let function = call.child_by_field_name("type")?;
    if !context_constructor(ctx.bounded_source(function)) {
        return None;
    }
    let declarator = call.child_by_field_name("declarator")?;
    if declarator.kind() != "abstract_function_declarator" {
        return None;
    }
    args(call)
        .iter()
        .any(|n| ctx.bounded_source(*n) == "nullptr")
        .then_some(call)
}
fn conditional(mut node: Node<'_>) -> bool {
    for _ in 0..64 {
        if matches!(
            node.kind(),
            "preproc_if"
                | "preproc_ifdef"
                | "preproc_elif"
                | "preproc_else"
                | "if_statement"
                | "conditional_expression"
                | "switch_statement"
                | "while_statement"
                | "for_statement"
                | "template_declaration"
        ) {
            return true;
        }
        let Some(parent) = node.parent() else {
            return false;
        };
        node = parent;
    }
    true
}
fn emit(
    facts: &mut Facts,
    rules: &RuleSet,
    node: Node<'_>,
    id: &'static str,
    algorithm: Option<&str>,
    reason: &'static str,
) {
    if rules.get(id).is_some_and(builtin) {
        let names: Vec<_> = algorithm.into_iter().collect();
        facts.source_api(
            id,
            node,
            reason,
            &names,
            if id == TLS {
                "C/C++ TLS or cryptographic include reference"
            } else {
                "C/C++ cryptographic source reference"
            },
        );
    }
}
pub(super) fn inspect(ctx: &Context<'_, '_>, facts: &mut Facts, rules: &RuleSet) {
    let values = Values::new(ctx);
    for node in &ctx.nodes {
        if !values.charge() {
            break;
        }
        if node.kind() == "function_definition" {
            if let Some(name) = node
                .child_by_field_name("declarator")
                .and_then(|d| d.child_by_field_name("declarator"))
                .and_then(|d| {
                    if d.kind() == "qualified_identifier" {
                        let scope = d.child_by_field_name("scope")?;
                        let name = d.child_by_field_name("name")?;
                        (ctx.bounded_source(scope) == "bssl"
                            && ctx.bounded_source(name).starts_with("BCM_"))
                        .then_some(name)
                    } else {
                        Some(d)
                    }
                })
                .filter(|n| n.kind() == "identifier")
            {
                let algorithm = match ctx.bounded_source(name) {
                    "ECDSA_sign" => Some("ECDSA"),
                    "BCM_mlkem512_generate_key" => Some("ML-KEM-512"),
                    "BCM_mlkem768_generate_key" => Some("ML-KEM-768"),
                    "BCM_mlkem1024_generate_key" => Some("ML-KEM-1024"),
                    "BCM_mldsa44_generate_key" => Some("ML-DSA-44"),
                    "BCM_mldsa65_generate_key" => Some("ML-DSA-65"),
                    "BCM_mldsa87_generate_key" => Some("ML-DSA-87"),
                    _ => None,
                };
                if let Some(algorithm) = algorithm {
                    emit(
                        facts,
                        rules,
                        name,
                        "SOURCE_IMPLEMENTATION_REFERENCE",
                        Some(algorithm),
                        "implementation_definition_not_api_call",
                    );
                }
            }
        }
        if node.kind() == "preproc_ifdef" {
            if let Some(name) = node
                .child_by_field_name("name")
                .filter(|n| ctx.bounded_source(*n) == "USE_SNTRUP761X25519")
            {
                emit(
                    facts,
                    rules,
                    name,
                    "SOURCE_CONSTRUCTION_REFERENCE",
                    Some("sntrup761x25519-sha512"),
                    "conditional_hybrid_reference_not_enabled_or_negotiated",
                );
            }
        }
        if node.kind().starts_with("preproc_")
            && !matches!(node.kind(), "preproc_include" | "preproc_arg")
        {
            facts.partial = true;
        }
        if node.kind() == "preproc_include" {
            if let Some(path) = node.child_by_field_name("path") {
                let raw = ctx.bounded_source(path);
                if matches!(
                    raw,
                    "<openssl/evp.h>"
                        | "<openssl/rsa.h>"
                        | "<openssl/dsa.h>"
                        | "<openssl/dh.h>"
                        | "<openssl/ec.h>"
                        | "<openssl/ssl.h>"
                        | "<openssl/provider.h>"
                        | "<oqs/oqs.h>"
                        | "<oqs_cpp.hpp>"
                        | "\"oqs_cpp.hpp\""
                ) {
                    let before = facts.hits.get(TLS).map_or(0, Vec::len);
                    emit(
                        facts,
                        rules,
                        *node,
                        TLS,
                        None,
                        "include_reference_not_linked_or_enabled",
                    );
                    if let Some(hit) = facts.hits.get_mut(TLS).and_then(|h| h.get_mut(before)) {
                        let label = if raw.contains("oqs_cpp") {
                            "C/C++ liboqs-cpp include reference"
                        } else if raw.contains("oqs/oqs") {
                            "C/C++ liboqs include reference"
                        } else {
                            "C/C++ OpenSSL include reference"
                        };
                        hit.safe_preview = Some(label.into());
                        hit.source_library =
                            pqc_scan_core::observation::SourceLibraryReference::from_label(label);
                    }
                }
            }
            continue;
        }
        if node.kind() == "declaration" {
            if let Some(call) = recovered_cpp_call(ctx, *node) {
                let function = call.child_by_field_name("type").unwrap();
                let name = ctx.bounded_source(function);
                if values.api_available(name, function) {
                    let key = values.constructor(call, 0);
                    let algorithm = key.as_ref().map(|k| k.algorithm.as_str());
                    emit(
                        facts,
                        rules,
                        call,
                        rule_id(algorithm),
                        algorithm,
                        "cpp_unambiguous_nested_initializer_reference",
                    );
                    facts.sensitive_ranges.push(call.byte_range());
                    facts.partial = true;
                }
            }
            if let Some(ty) = node.child_by_field_name("type") {
                let name = ctx.bounded_source(ty).replace(' ', "");
                if matches!(name.as_str(), "oqs::KeyEncapsulation" | "oqs::Signature")
                    && !values.oqs_shadowed()
                {
                    if !values.api_available(&name, ty) {
                        facts.partial = true;
                        continue;
                    }
                    let mut cursor = node.walk();
                    for (i, decl) in node
                        .children_by_field_name("declarator", &mut cursor)
                        .enumerate()
                    {
                        if let Some(arguments) = decl
                            .child_by_field_name("value")
                            .filter(|n| matches!(n.kind(), "argument_list" | "initializer_list"))
                        {
                            let role = if name == "oqs::KeyEncapsulation" {
                                Role::CppKem
                            } else {
                                Role::CppSig
                            };
                            let algorithm =
                                values.cpp_constructor(arguments, role).map(|k| k.algorithm);
                            if algorithm.is_none() || conditional(*node) {
                                facts.partial = true;
                            }
                            let id = rule_id(algorithm.as_deref());
                            if rules.get(id).is_some_and(builtin) {
                                let start = if i == 0 {
                                    ty.start_byte()
                                } else {
                                    decl.start_byte()
                                };
                                let names: Vec<_> = algorithm.as_deref().into_iter().collect();
                                facts.source_api_range(
                                    id,
                                    start..decl.end_byte(),
                                    "qualified_cpp_constructor",
                                    &names,
                                    "C/C++ cryptographic source reference",
                                );
                                facts.sensitive_ranges.push(arguments.byte_range());
                            }
                        }
                    }
                }
            }
            continue;
        }
        if node.kind() == "new_expression" {
            if node.child_by_field_name("type").is_some_and(|t| {
                matches!(
                    ctx.bounded_source(t).replace(' ', "").as_str(),
                    "oqs::KeyEncapsulation" | "oqs::Signature"
                )
            }) {
                if values.oqs_shadowed() {
                    facts.partial = true;
                    continue;
                }
                let key = values.constructor(*node, 0);
                if key.is_none() || conditional(*node) {
                    facts.partial = true;
                }
                let algorithm = key.as_ref().map(|k| k.algorithm.as_str());
                emit(
                    facts,
                    rules,
                    *node,
                    rule_id(algorithm),
                    algorithm,
                    "qualified_cpp_new_reference_pointer_flow_unresolved",
                );
            }
            continue;
        }
        if node.kind() != "call_expression" {
            continue;
        }
        let Some(function) = node.child_by_field_name("function") else {
            continue;
        };
        let arguments = args(*node);
        let Some(name) = function_name(ctx, function) else {
            continue;
        };
        let name = name.strip_prefix("::").unwrap_or(&name);
        if let Some((receiver, method)) = values.receiver(function) {
            if let Some(key) = values.key(receiver, node.start_byte(), 0) {
                let allowed = match key.role {
                    Role::CppKem => {
                        matches!(method, "generate_keypair" | "encap_secret" | "decap_secret")
                    }
                    Role::CppSig => matches!(method, "generate_keypair" | "sign" | "verify"),
                    _ => false,
                };
                if allowed {
                    emit(
                        facts,
                        rules,
                        *node,
                        rule_id(Some(&key.algorithm)),
                        Some(&key.algorithm),
                        "bounded_cpp_receiver",
                    );
                }
            } else if values.was_bound(receiver) {
                facts.partial = true;
            }
            continue;
        }
        if !known(name) {
            continue;
        }
        if !values.api_available(name, function) {
            facts.partial = true;
            continue;
        }
        if conditional(*node) {
            facts.partial = true
        }
        if matches!(
            name,
            "X509_verify_cert" | "Botan::PK_Signer" | "Botan::PK_Verifier"
        ) {
            emit(
                facts,
                rules,
                *node,
                API,
                None,
                "canonical_request_key_or_chain_algorithm_unresolved",
            );
            continue;
        }
        if let Some(algorithm) = low_level(name) {
            emit(
                facts,
                rules,
                *node,
                API,
                Some(algorithm),
                "explicit_low_level_api",
            );
            continue;
        }
        if configuration(name) {
            let before = facts.hits.get(API).map_or(0, Vec::len);
            emit(
                facts,
                rules,
                *node,
                API,
                None,
                "configuration_reference_not_activation_or_negotiation",
            );
            let reference = arguments
                .get(1)
                .and_then(|n| values.literal(*n, node.start_byte(), 0))
                .and_then(|v| {
                    use pqc_scan_core::observation::SourceConfigurationReference as C;
                    if name.starts_with("OSSL_PROVIDER_") {
                        C::provider(&v)
                    } else if name == "EVP_set_default_properties" {
                        C::property(&v)
                    } else {
                        C::tls_groups(&v)
                    }
                });
            if reference.is_none() {
                facts.partial = true;
            }
            if let Some(hit) = facts.hits.get_mut(API).and_then(|h| h.get_mut(before)) {
                if let Some(reference) = &reference {
                    hit.safe_preview = Some(reference.label());
                }
                hit.source_configuration = reference;
            }
            continue;
        }
        if context_constructor(name)
            || raw_key_constructor(name)
            || matches!(
                name,
                "OQS_KEM_new" | "OQS_SIG_new" | "oqs::KeyEncapsulation" | "oqs::Signature"
            )
        {
            let key = values.constructor(*node, 0);
            if key.is_none() {
                facts.partial = true
            }
            let algorithm = key.as_ref().map(|k| k.algorithm.as_str());
            emit(
                facts,
                rules,
                *node,
                rule_id(algorithm),
                algorithm,
                "explicit_algorithm_or_unresolved_selector",
            );
            continue;
        }
        let (key_node, role) = if digest_operation(name) {
            (
                arguments
                    .get(if name.ends_with("_ex") { 5 } else { 4 })
                    .copied(),
                Role::Key,
            )
        } else if context_operation(name) {
            (arguments.first().copied(), Role::Context)
        } else {
            (
                arguments.first().copied(),
                if name.starts_with("OQS_KEM_") {
                    Role::OqsKem
                } else {
                    Role::OqsSig
                },
            )
        };
        let mut key = key_node
            .and_then(|n| values.key(n, node.start_byte(), 0))
            .filter(|k| k.role == role);
        if let Some(key) = &mut key {
            if name.starts_with("EVP_PKEY_derive") && key.algorithm == "EC" {
                key.algorithm = "ECDH".into();
            }
        }
        if key.is_none() && key_node.is_some_and(|n| values.was_bound(n)) {
            facts.partial = true;
        }
        let algorithm = key.as_ref().map(|k| k.algorithm.as_str());
        let id = if name.starts_with("OQS_") && algorithm.is_none() {
            PQC
        } else {
            rule_id(algorithm)
        };
        emit(
            facts,
            rules,
            *node,
            id,
            algorithm,
            "bounded_key_or_context_operation",
        );
    }
}
