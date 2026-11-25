//! Exact PHP API identities over the shared bounded Tree-sitter context.
mod names;
mod values;
use super::{
    builtin,
    context::{children, Context},
    Facts,
};
use names::{Names, CLASS, FUNCTION};
use pqc_scan_rules::RuleSet;
use tree_sitter::Node;
use values::{argument, Value, Values};
const API: &str = "PHP_CRYPTO_API";
const PQC: &str = "PHP_PQC_API";
const REFERENCE: &str = "PHP_CRYPTO_REFERENCE";
#[derive(Clone, Copy, PartialEq, Eq)]
enum Role {
    OpenSsl,
    Encap,
    Decap,
    Sign,
    Verify,
    Reference,
    XmlSignature,
}
#[derive(Clone, Copy)]
struct Key {
    algorithm: &'static str,
    role: Role,
}
#[derive(Clone, Copy)]
enum ClassInfo {
    Factory(&'static str),
    Key(Key),
    Compat,
    Reference(Option<&'static str>),
    XmlSignature,
}
fn class_info(name: &str) -> Option<ClassInfo> {
    match name {
        "robrichards\\xmlseclibs\\xmlsecuritydsig" => return Some(ClassInfo::XmlSignature),
        "phpseclib3\\crypt\\rsa" => return Some(ClassInfo::Reference(Some("RSA"))),
        "phpseclib3\\crypt\\ec" => return Some(ClassInfo::Reference(Some("EC"))),
        "phpseclib3\\crypt\\dh" => return Some(ClassInfo::Reference(Some("DH"))),
        "oqs_keyencapsulation" | "oqs_signature" | "oqs\\oqskem" | "oqs\\oqssig" => {
            return Some(ClassInfo::Reference(None))
        }
        _ => {}
    }
    let (rest, native) = if let Some(n) = name.strip_prefix("pqcrypto\\") {
        (n, true)
    } else {
        (name.strip_prefix("paragonie\\pqcrypto\\")?, false)
    };
    if rest == "compat" && !native {
        return Some(ClassInfo::Compat);
    }
    if rest == "slhdsa" && native {
        return Some(ClassInfo::Reference(Some("SLH-DSA")));
    }
    let (family, tail) = rest.split_once('\\').unwrap_or((rest, ""));
    let algorithm = match family {
        "mlkem512" => "ML-KEM-512",
        "mlkem768" => "ML-KEM-768",
        "mlkem1024" => "ML-KEM-1024",
        "mldsa44" => "ML-DSA-44",
        "mldsa65" => "ML-DSA-65",
        "mldsa87" => "ML-DSA-87",
        "xwing" => "X-Wing",
        _ => return None,
    };
    if tail.is_empty() {
        return Some(ClassInfo::Factory(algorithm));
    }
    let role = if algorithm.starts_with("ML-DSA") {
        match tail {
            "signingkey" => Role::Sign,
            "verifyingkey" if native => Role::Verify,
            "verificationkey" if !native => Role::Verify,
            _ => return None,
        }
    } else {
        match tail {
            "encapsulationkey" => Role::Encap,
            "decapsulationkey" => Role::Decap,
            _ => return None,
        }
    };
    Some(ClassInfo::Key(Key { algorithm, role }))
}
/// Reuse the detector's canonical namespace/import resolution for presentation
/// privacy. Only names/positions survive; no argument text or new crypto facts.
pub(crate) fn privacy_callees(
    root: Node<'_>,
    text: &str,
    max_nodes: usize,
) -> (std::collections::BTreeMap<usize, String>, bool) {
    let ctx = Context::new(
        root,
        text,
        "php",
        crate::DetectorBudget {
            max_ast_nodes: max_nodes,
            ..Default::default()
        },
    );
    let names = Names::new(&ctx);
    let mut out = std::collections::BTreeMap::new();
    for node in &ctx.nodes {
        match node.kind() {
            "function_call_expression" => {
                if let Some(function) = node.child_by_field_name("function") {
                    if let Some(name) = names.resolve(function, FUNCTION) {
                        out.insert(node.id(), name);
                    } else if names.function_import_alias(function) {
                        out.insert(node.id(), "?unresolved-function-import".into());
                    }
                }
            }
            "scoped_call_expression" => {
                if let (Some(class), Some(method)) = (
                    node.child_by_field_name("scope"),
                    node.child_by_field_name("name"),
                ) {
                    if let (Some(class), Some(method)) =
                        (names.resolve(class, CLASS), names.token(method, FUNCTION))
                    {
                        out.insert(node.id(), format!("{class}::{method}"));
                    }
                }
            }
            _ => {}
        }
    }
    (out, ctx.truncated.get())
}

fn compat_method(method: &str) -> Option<(&'static str, &str)> {
    let (algorithm, operation) = method.split_once('_')?;
    let algorithm = match algorithm {
        "mlkem512" => "ML-KEM-512",
        "mlkem768" => "ML-KEM-768",
        "mlkem1024" => "ML-KEM-1024",
        "mldsa44" => "ML-DSA-44",
        "mldsa65" => "ML-DSA-65",
        "mldsa87" => "ML-DSA-87",
        "xwing" => "X-Wing",
        _ => return None,
    };
    let valid = if algorithm.starts_with("ML-DSA") {
        matches!(operation, "keygen" | "sign" | "verify")
    } else {
        matches!(operation, "keygen" | "encaps" | "decaps")
    };
    valid.then_some((algorithm, operation))
}
fn sodium(name: &str) -> Option<(&'static str, bool)> {
    let suffix = name.strip_prefix("sodium_crypto_")?;
    if let Some(kem) = suffix.strip_prefix("kem_") {
        let (algorithm, operation) = kem
            .strip_prefix("mlkem768_")
            .map_or(("X-Wing", kem), |op| ("ML-KEM-768", op));
        return matches!(
            operation,
            "keypair" | "seed_keypair" | "secretkey" | "publickey" | "enc" | "dec"
        )
        .then_some((algorithm, true));
    }
    let algorithm = match suffix {
        "sign"
        | "sign_detached"
        | "sign_verify_detached"
        | "sign_keypair"
        | "sign_seed_keypair"
        | "sign_open"
        | "sign_publickey"
        | "sign_secretkey"
        | "sign_publickey_from_secretkey"
        | "sign_keypair_from_secretkey_and_publickey" => "Ed25519",
        "scalarmult"
        | "scalarmult_base"
        | "box"
        | "box_open"
        | "box_keypair"
        | "box_seed_keypair"
        | "box_keypair_from_secretkey_and_publickey"
        | "box_publickey"
        | "box_publickey_from_secretkey"
        | "box_secretkey"
        | "box_seal"
        | "box_seal_open"
        | "kx_keypair"
        | "kx_seed_keypair"
        | "kx_publickey"
        | "kx_secretkey"
        | "kx_client_session_keys"
        | "kx_server_session_keys" => "X25519",
        _ => return None,
    };
    Some((algorithm, false))
}
fn known_function(name: &str) -> bool {
    sodium(name).is_some()
        || matches!(
            name,
            "openssl_pkey_new"
                | "openssl_pkey_derive"
                | "openssl_public_encrypt"
                | "openssl_private_decrypt"
                | "openssl_private_encrypt"
                | "openssl_public_decrypt"
                | "openssl_sign"
                | "openssl_verify"
        )
}
fn emit(
    facts: &mut Facts,
    rules: &RuleSet,
    id: &'static str,
    node: Node<'_>,
    algorithm: Option<&'static str>,
    reason: &'static str,
    label: &'static str,
) {
    if !rules.get(id).is_some_and(builtin) {
        return;
    }
    let refs = algorithm.into_iter().collect::<Vec<_>>();
    facts.source_api(id, node, reason, &refs, label);
}
pub(super) fn inspect(ctx: &Context<'_, '_>, facts: &mut Facts, rules: &RuleSet) {
    let names = Names::new(ctx);
    if ctx.truncated.get() {
        return;
    }
    let values = Values::new(&names);
    if ctx.truncated.get() {
        return;
    }
    for (node, target) in &names.import_references {
        if let Some(ClassInfo::Reference(algorithm)) = class_info(target) {
            emit_reference(facts, rules, *node, target, algorithm);
        }
    }
    for n in &ctx.nodes {
        if !names.charge() {
            break;
        }
        if in_error(*n, ctx) {
            continue;
        }
        match n.kind() {
            "class_constant_access_expression" => {
                let parts = children(*n);
                if parts.len() == 2 && ctx.source(parts[1]).eq_ignore_ascii_case("class") {
                    if let Some(class) = names.resolve(parts[0], CLASS) {
                        if let Some(ClassInfo::Reference(algorithm)) = class_info(&class) {
                            emit_reference(facts, rules, *n, &class, algorithm);
                        }
                    }
                }
            }
            "function_call_expression" => {
                let Some(function) = n.child_by_field_name("function") else {
                    continue;
                };
                if !matches!(function.kind(), "name" | "qualified_name" | "relative_name") {
                    names.partial.set(true);
                    continue;
                }
                let Some(name) = names.resolve(function, FUNCTION) else {
                    continue;
                };
                if let Some((algorithm, pqc)) = sodium(&name) {
                    emit(
                        facts,
                        rules,
                        if pqc { PQC } else { API },
                        *n,
                        Some(algorithm),
                        "explicit_fixed_primitive_request",
                        "PHP Sodium API request",
                    );
                } else if name == "openssl_pkey_new" {
                    let key = values.value(*n, 0).and_then(|v| match v {
                        Value::Key(k) => Some(k),
                        _ => None,
                    });
                    if key.is_none() {
                        names.partial.set(true);
                    }
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        key.map(|k| k.algorithm),
                        if key.is_some() {
                            "explicit_key_type_request"
                        } else {
                            "unresolved_key_type"
                        },
                        "PHP OpenSSL key request",
                    );
                } else if matches!(
                    name.as_str(),
                    "openssl_public_encrypt"
                        | "openssl_private_decrypt"
                        | "openssl_private_encrypt"
                        | "openssl_public_decrypt"
                ) {
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        Some("RSA"),
                        "rsa_only_api_request",
                        "PHP OpenSSL RSA operation",
                    );
                } else if matches!(name.as_str(), "openssl_sign" | "openssl_verify") {
                    let label = if name == "openssl_sign" {
                        "private_key"
                    } else {
                        "public_key"
                    };
                    let key = argument(*n, 2, label, ctx.text)
                        .and_then(|n| values.key(n))
                        .filter(|k| k.role == Role::OpenSsl);
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        key.map(|k| k.algorithm),
                        if key.is_some() {
                            "locally_typed_signature_request"
                        } else {
                            "unknown_signature_key_type"
                        },
                        "PHP OpenSSL signature operation",
                    );
                } else if name == "openssl_pkey_derive" {
                    let a = argument(*n, 0, "public_key", ctx.text).and_then(|n| values.key(n));
                    let b = argument(*n, 1, "private_key", ctx.text).and_then(|n| values.key(n));
                    let algorithm = match (a, b) {
                        (Some(a), Some(b))
                            if a.role == Role::OpenSsl
                                && b.role == Role::OpenSsl
                                && a.algorithm == b.algorithm =>
                        {
                            match a.algorithm {
                                "DH" => Some("DH"),
                                "EC" => Some("ECDH"),
                                _ => None,
                            }
                        }
                        _ => None,
                    };
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        algorithm,
                        if algorithm.is_some() {
                            "locally_typed_key_agreement_request"
                        } else {
                            "unknown_key_agreement_type"
                        },
                        "PHP OpenSSL key agreement",
                    );
                } else if matches!(
                    name.as_str(),
                    "call_user_func"
                        | "call_user_func_array"
                        | "forward_static_call"
                        | "forward_static_call_array"
                ) {
                    names.partial.set(true);
                }
            }
            "scoped_call_expression" => {
                let (Some(scope), Some(method)) = (
                    n.child_by_field_name("scope"),
                    n.child_by_field_name("name"),
                ) else {
                    continue;
                };
                if !matches!(scope.kind(), "name" | "qualified_name" | "relative_name")
                    || method.kind() != "name"
                {
                    names.partial.set(true);
                    continue;
                }
                let (Some(class), Some(method)) =
                    (names.resolve(scope, CLASS), names.token(method, FUNCTION))
                else {
                    continue;
                };
                if class == "ffi" {
                    names.partial.set(true);
                    continue;
                }
                if class == "firebase\\jwt\\jwt" && matches!(method.as_str(), "decode" | "encode") {
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        None,
                        "canonical_jose_request_algorithm_unresolved",
                        "PHP JOSE library request",
                    );
                    continue;
                }
                match class_info(&class) {
                    Some(ClassInfo::Factory(algorithm)) if method == "generatekeypair" => emit(
                        facts,
                        rules,
                        PQC,
                        *n,
                        Some(algorithm),
                        "explicit_pqc_keypair_request",
                        "PHP ParagonIE key generation",
                    ),
                    Some(ClassInfo::Key(key)) if method == "frombytes" => emit(
                        facts,
                        rules,
                        PQC,
                        *n,
                        Some(key.algorithm),
                        "qualified_key_import_reference",
                        "PHP ParagonIE key import",
                    ),
                    Some(ClassInfo::Compat) => {
                        if let Some((algorithm, _)) = compat_method(&method) {
                            emit(
                                facts,
                                rules,
                                PQC,
                                *n,
                                Some(algorithm),
                                "qualified_compat_request",
                                "PHP ParagonIE Compat request",
                            );
                        }
                    }
                    Some(ClassInfo::Reference(algorithm)) => {
                        let id = if algorithm == Some("SLH-DSA") {
                            PQC
                        } else if algorithm.is_some() {
                            API
                        } else {
                            wrapper_id(&class)
                        };
                        emit(
                            facts,
                            rules,
                            id,
                            *n,
                            algorithm,
                            "qualified_library_reference",
                            if algorithm.is_none() {
                                wrapper_label(&class)
                            } else {
                                "PHP qualified library reference"
                            },
                        );
                    }
                    _ => {}
                }
            }
            "object_creation_expression" => {
                let class = children(*n)
                    .into_iter()
                    .find(|n| matches!(n.kind(), "name" | "qualified_name" | "relative_name"));
                let Some(class) = class else {
                    if children(*n).iter().any(|n| n.kind() == "variable_name") {
                        names.partial.set(true);
                    }
                    continue;
                };
                let Some(class) = names.resolve(class, CLASS) else {
                    continue;
                };
                if class.starts_with("reflection") {
                    names.partial.set(true);
                }
                if let Some(ClassInfo::Reference(algorithm)) = class_info(&class) {
                    if let Some(arguments) = n.child_by_field_name("arguments") {
                        facts.sensitive_ranges.push(arguments.byte_range());
                    }
                    let id = if algorithm == Some("SLH-DSA") {
                        PQC
                    } else if algorithm.is_some() {
                        API
                    } else {
                        wrapper_id(&class)
                    };
                    emit(
                        facts,
                        rules,
                        id,
                        if algorithm.is_none() {
                            children(*n)
                                .into_iter()
                                .find(|n| {
                                    matches!(n.kind(), "name" | "qualified_name" | "relative_name")
                                })
                                .expect("resolved class node")
                        } else {
                            *n
                        },
                        algorithm,
                        "qualified_library_reference",
                        if algorithm.is_none() {
                            wrapper_label(&class)
                        } else {
                            "PHP qualified library reference"
                        },
                    );
                }
            }
            "member_call_expression" | "nullsafe_member_call_expression" => {
                let (Some(object), Some(method)) = (
                    n.child_by_field_name("object"),
                    n.child_by_field_name("name"),
                ) else {
                    continue;
                };
                let Some(method) = names.token(method, FUNCTION) else {
                    names.partial.set(true);
                    continue;
                };
                if matches!(method.as_str(), "sign" | "validatereference" | "verify")
                    && values
                        .key(object)
                        .is_some_and(|k| k.role == Role::XmlSignature)
                {
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        None,
                        "canonical_xml_signature_request_algorithm_unresolved",
                        "PHP XML signature library request",
                    );
                    continue;
                }
                if method == "sign" && values.guarded_signer_reference(object, n.start_byte()) {
                    emit(
                        facts,
                        rules,
                        API,
                        *n,
                        None,
                        "local_key_type_check_receiver_relation_unresolved",
                        "PHP type-checked key signature reference",
                    );
                    names.partial.set(true);
                    continue;
                }
                if !matches!(
                    method.as_str(),
                    "sign" | "verify" | "encapsulate" | "decapsulate"
                ) {
                    continue;
                }
                let key = values.key(object);
                let expected = match method.as_str() {
                    "sign" => Role::Sign,
                    "verify" => Role::Verify,
                    "encapsulate" => Role::Encap,
                    _ => Role::Decap,
                };
                if let Some(key) = key.filter(|k| k.role == expected) {
                    emit(
                        facts,
                        rules,
                        PQC,
                        *n,
                        Some(key.algorithm),
                        "bounded_key_receiver_request",
                        "PHP ParagonIE key operation",
                    );
                } else {
                    names.partial.set(true);
                }
            }
            _ => {}
        }
    }
    if values.binding_limited {
        ctx.limit("php_same_name_bindings", 64, 64);
    }
    facts.partial |= names.partial.get();
}
fn in_error(mut node: Node<'_>, ctx: &Context<'_, '_>) -> bool {
    for _ in 0..64 {
        if node.kind() == "ERROR" {
            return true;
        }
        let Some(parent) = node.parent() else {
            return false;
        };
        node = parent;
    }
    ctx.limit("php_ancestor_depth", 64, 64);
    true
}

fn emit_reference(
    facts: &mut Facts,
    rules: &RuleSet,
    node: Node<'_>,
    class: &str,
    algorithm: Option<&'static str>,
) {
    let id = match algorithm {
        Some("SLH-DSA") => PQC,
        Some(_) => API,
        None => wrapper_id(class),
    };
    emit(
        facts,
        rules,
        id,
        node,
        algorithm,
        "qualified_library_reference",
        if algorithm.is_none() {
            wrapper_label(class)
        } else {
            "PHP qualified library reference"
        },
    );
}
fn wrapper_id(name: &str) -> &'static str {
    if matches!(name, "oqs_signature" | "oqs\\oqssig") {
        API
    } else {
        REFERENCE
    }
}
fn wrapper_label(name: &str) -> &'static str {
    if name.starts_with("oqs\\") {
        "mir-evgenii/liboqs-php FFI wrapper reference"
    } else {
        "Muzosh oqsphp wrapper reference"
    }
}
