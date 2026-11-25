//! Presentation-only secret spans. Never feeds source resolution or assets.
use pqc_scan_core::ScannableFile;
use std::{collections::BTreeSet, ops::Range};
use tree_sitter::{Node, Parser};

pub(super) fn typed_only(file: &ScannableFile, rules: &pqc_scan_rules::RuleSet) -> bool {
    file.text
        .as_deref()
        .and_then(|t| super::resolve_language(&file.path, t))
        .is_some_and(|(_, name)| {
            matches!(name, "php" | "csharp" | "c" | "cpp")
                && rules.all().iter().any(|r| {
                    super::scope_matches(&r.scope, name) && crate::source_facts::builtin(r)
                })
        })
}

pub(super) fn ranges(file: &ScannableFile, max_nodes: usize) -> Vec<Range<usize>> {
    let Some(text) = file.text.as_deref() else {
        return vec![];
    };
    let Some((language, name)) = super::resolve_language(&file.path, text) else {
        return vec![];
    };
    if !matches!(name, "php" | "csharp" | "c" | "cpp") {
        return vec![];
    }
    let mut parser = Parser::new();
    if parser.set_language(&language).is_err() {
        return std::iter::once(0..text.len()).collect();
    }
    let Some(tree) = parser.parse(text, None) else {
        return std::iter::once(0..text.len()).collect();
    };
    let (php_callees, name_limited) = if name == "php" {
        crate::source_facts::privacy_callees(tree.root_node(), text, max_nodes)
    } else {
        (Default::default(), false)
    };
    let mut out = Vec::new();
    let mut secret_names = BTreeSet::new();
    let mut assignments = Vec::new();
    let mut uncertain_scopes = BTreeSet::new();
    let mut cursor = tree.walk();
    let mut visited = 0;
    let mut node_limited = false;
    'walk: loop {
        let node = cursor.node();
        if visited >= max_nodes {
            out.push(node.start_byte()..text.len());
            node_limited = true;
            break;
        }
        visited += 1;
        if matches!(
            node.kind(),
            "init_declarator" | "variable_declarator" | "assignment_expression"
        ) {
            let name = node
                .child_by_field_name("name")
                .or_else(|| node.child_by_field_name("declarator"))
                .or_else(|| node.child_by_field_name("left"));
            let value = node
                .child_by_field_name("value")
                .or_else(|| node.child_by_field_name("right"))
                .or_else(|| {
                    if node.kind() != "variable_declarator" {
                        return None;
                    }
                    let mut children = node.walk();
                    let found = node
                        .named_children(&mut children)
                        .find(|child| name.is_none_or(|n| n.id() != child.id()));
                    found
                });
            if let (Some(name), Some(value)) = (name.and_then(|n| reference_name(n, text)), value) {
                if pqc_scan_core::privacy::is_sensitive_name(name) {
                    protect_named_initializer(value, &mut out);
                }
                assignments.push((scope(node), name.to_owned(), value));
            }
        }
        // Parent lookup on each root-level comment is quadratic in a large
        // translation unit. Only a declarator can use this constructor form.
        let constructor_type = (if node.kind() == "init_declarator" {
            node.parent()
        } else {
            None
        })
        .filter(|p| p.kind() == "declaration")
        .and_then(|p| p.child_by_field_name("type"))
        .filter(|t| {
            text.get(t.byte_range())
                .filter(|s| s.len() <= 512)
                .is_some_and(|s| {
                    matches!(
                        s.split_whitespace().collect::<String>().as_str(),
                        "oqs::Signature" | "oqs::KeyEncapsulation"
                    )
                })
        });
        let arguments = node.child_by_field_name("arguments").or_else(|| {
            constructor_type
                .and_then(|_| node.child_by_field_name("value"))
                .filter(|n| matches!(n.kind(), "argument_list" | "initializer_list"))
        });
        if let Some(arguments) = arguments {
            let callee = node
                .child_by_field_name("function")
                .or_else(|| node.child_by_field_name("name"))
                .or_else(|| node.child_by_field_name("type"))
                .or(constructor_type);
            if let Some(callee) = callee {
                let source_callee = &text[callee.byte_range()];
                let canonical = php_callees.get(&node.id());
                let raw = canonical.map(String::as_str).unwrap_or(source_callee);
                let oversized = raw.len() > 512;
                let leaf = (if oversized { "" } else { raw })
                    .rsplit([':', '.', '\\', '>'])
                    .next()
                    .unwrap_or(raw)
                    .to_ascii_lowercase();
                let mut args = arguments.walk();
                for (index, argument) in arguments
                    .named_children(&mut args)
                    .filter(|n| n.kind() != "comment")
                    .enumerate()
                {
                    let named = argument
                        .child_by_field_name("name")
                        .and_then(|n| text.get(n.byte_range()))
                        .map(|s| {
                            s.trim_start_matches('$')
                                .replace('_', "")
                                .to_ascii_lowercase()
                        });
                    let qualified = if let Some(canonical) = canonical {
                        canonical.as_str()
                    } else if node.start_byte() <= callee.end_byte() {
                        &text[node.start_byte()..callee.end_byte()]
                    } else {
                        raw
                    };
                    let secret = if oversized
                        || name_limited
                        || raw == "?unresolved-function-import"
                        || (name == "php" && leaf == "frombytes" && canonical.is_none())
                    {
                        true
                    } else if let Some(named) = named.as_deref() {
                        secret_named_argument(&leaf, named, qualified)
                    } else {
                        secret_argument(&leaf, index, qualified)
                    };
                    if secret {
                        // Keep variable references and key-loading calls readable.
                        // Literal data/byte-array/concatenation expressions in key
                        // positions are protected as one span before truncation.
                        let mut value = argument;
                        if matches!(value.kind(), "argument" | "named_argument") {
                            value = value
                                .child_by_field_name("value")
                                .or_else(|| {
                                    u32::try_from(value.named_child_count().saturating_sub(1))
                                        .ok()
                                        .and_then(|i| value.named_child(i))
                                })
                                .unwrap_or(value);
                        }
                        value = unwrap_value(value);
                        if literal_data(value) {
                            let raw = text[value.byte_range()].trim_start_matches(['\'', '"']);
                            if !(raw.starts_with("file://")
                                && file_key_input(&leaf, index, named.as_deref()))
                            {
                                out.push(value.byte_range());
                            }
                            if value.kind() == "binary_expression" {
                                uncertain_scopes.insert(scope(node));
                            }
                        } else if let Some(name) = reference_name(value, text) {
                            secret_names.insert((scope(node), name.to_owned()));
                        } else {
                            protect_nested_values(value, &mut out);
                            if !literal_only_call(value) {
                                uncertain_scopes.insert(scope(node));
                            }
                        }
                    }
                }
            }
        }
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                break 'walk;
            }
        }
    }
    // Follow only explicit same-function variable initializers, with a finite
    // alias depth. This does not resolve crypto facts or execute key loaders.
    let mut exhausted = true;
    for _ in 0..8 {
        let mut added = Vec::new();
        for (local_scope, name, value) in &assignments {
            if secret_names.contains(&(*local_scope, name.clone()))
                || (*local_scope == 0 && secret_names.iter().any(|(_, n)| n == name))
            {
                let value = unwrap_value(*value);
                if literal_data(value) {
                    out.push(value.byte_range());
                    if value.kind() == "binary_expression" {
                        uncertain_scopes.insert(*local_scope);
                    }
                } else if let Some(alias) = reference_name(value, text) {
                    added.push((*local_scope, alias.to_owned()));
                } else {
                    // A known key initializer whose expression cannot be
                    // resolved stays protected beyond the unwrap envelope.
                    protect_nested_values(value, &mut out);
                    if !literal_only_call(value) {
                        uncertain_scopes.insert(*local_scope);
                    }
                }
            }
        }
        let before = secret_names.len();
        secret_names.extend(added);
        if before == secret_names.len() {
            exhausted = false;
            break;
        }
    }
    if exhausted || node_limited || !uncertain_scopes.is_empty() {
        // The key's definition is unresolved beyond the finite alias envelope.
        // Protect literal initializers in the affected function, not unrelated
        // functions or their normal statements/variable references.
        let scopes: BTreeSet<_> = if exhausted || node_limited {
            secret_names
                .iter()
                .map(|(scope, _)| *scope)
                .chain(uncertain_scopes)
                .collect()
        } else {
            uncertain_scopes
        };
        for (local_scope, _, value) in &assignments {
            if (node_limited || scopes.contains(local_scope) || *local_scope == 0)
                && (literal_data(unwrap_value(*value))
                    || matches!(
                        unwrap_value(*value).kind(),
                        "parenthesized_expression"
                            | "cast_expression"
                            | "pointer_expression"
                            | "ref_expression"
                    ))
            {
                out.push(value.byte_range());
            }
        }
    }
    out
}

// Walk only a secret-valued expression, never the file. Preserve the public
// conversion/loader syntax while redacting nested literal material. Exhaustion
// masks this limited expression rather than publishing uninspected payloads.
fn literal_only_call(node: Node<'_>) -> bool {
    let Some(arguments) = node.child_by_field_name("arguments") else {
        return false;
    };
    if arguments.named_child_count() == 0 || arguments.named_child_count() > 256 {
        return false;
    }
    let mut cursor = arguments.walk();
    let all_literals = arguments.named_children(&mut cursor).all(|mut value| {
        if matches!(value.kind(), "argument" | "named_argument") {
            value = value
                .child_by_field_name("value")
                .or_else(|| {
                    u32::try_from(value.named_child_count().saturating_sub(1))
                        .ok()
                        .and_then(|i| value.named_child(i))
                })
                .unwrap_or(value);
        }
        matches!(
            unwrap_value(value).kind(),
            "string_literal"
                | "raw_string_literal"
                | "string"
                | "character_literal"
                | "integer_literal"
                | "number_literal"
        )
    });
    all_literals
}
// A known credential field may have more wrappers than normal API resolution.
// Inspect at most 256 wrapper levels; never descend a computed public call.
fn protect_named_initializer(value: Node<'_>, out: &mut Vec<Range<usize>>) {
    let mut current = value;
    for _ in 0..16 {
        current = unwrap_value(current);
        if !matches!(
            current.kind(),
            "parenthesized_expression"
                | "cast_expression"
                | "pointer_expression"
                | "ref_expression"
        ) {
            if literal_data(current) {
                protect_nested_values(current, out);
            }
            return;
        }
    }
    out.push(value.byte_range());
}

fn protect_nested_values(node: Node<'_>, out: &mut Vec<Range<usize>>) {
    let mut cursor = node.walk();
    for _ in 0..256 {
        let current = cursor.node();
        let literal = literal_data(current);
        if literal {
            out.push(current.byte_range());
        }
        if !literal && cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return;
            }
        }
    }
    out.push(node.byte_range());
}

fn scope(mut node: Node<'_>) -> usize {
    for _ in 0..128 {
        if matches!(
            node.kind(),
            "function_definition"
                | "function_declaration"
                | "method_declaration"
                | "anonymous_function"
                | "lambda_expression"
        ) {
            return node.start_byte() + 1;
        }
        let Some(parent) = node.parent() else {
            return 0;
        };
        node = parent;
    }
    0
}
fn unwrap_value(mut node: Node<'_>) -> Node<'_> {
    for _ in 0..16 {
        if !matches!(
            node.kind(),
            "parenthesized_expression"
                | "cast_expression"
                | "pointer_expression"
                | "ref_expression"
        ) {
            break;
        }
        let next = node
            .child_by_field_name("value")
            .or_else(|| node.child_by_field_name("argument"))
            .or_else(|| node.child_by_field_name("operand"))
            .or_else(|| {
                let mut cursor = node.walk();
                node.named_children(&mut cursor).last()
            });
        let Some(next) = next else {
            break;
        };
        node = next;
    }
    node
}
fn reference_name<'a>(node: Node<'_>, text: &'a str) -> Option<&'a str> {
    let mut node = unwrap_value(node);
    // A declaration such as material[32] binds material, not the bracketed
    // declarator spelling. Descend the parser-owned, finite declarator chain.
    while matches!(
        node.kind(),
        "array_declarator" | "pointer_declarator" | "parenthesized_declarator"
    ) {
        let Some(inner) = node.child_by_field_name("declarator") else {
            break;
        };
        node = inner;
    }
    if node.end_byte() - node.start_byte() > 256 {
        return None;
    }
    let raw = text
        .get(node.byte_range())?
        .trim()
        .trim_start_matches(['$', '*', '&', ' ']);
    let public = raw
        .strip_prefix("System.Security.Cryptography.")
        .unwrap_or(raw);
    if [
        "MLKemAlgorithm.",
        "MLDsaAlgorithm.",
        "SlhDsaAlgorithm.",
        "HashAlgorithmName.",
    ]
    .iter()
    .any(|prefix| public.starts_with(prefix))
    {
        return public.split('.').next();
    }
    for suffix in [
        ".data()",
        ".begin()",
        ".get()",
        ".as_bytes()",
        "->data()",
        "->begin()",
        "->get()",
    ] {
        if let Some(base) = raw.strip_suffix(suffix) {
            if !base.is_empty() && base.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_') {
                return Some(base);
            }
        }
    }
    (!raw.is_empty() && raw.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')).then_some(raw)
}

fn literal_data(node: Node<'_>) -> bool {
    let kind = node.kind();
    kind.contains("string")
        || kind.contains("array")
        || matches!(
            kind,
            "concatenation_expression"
                | "binary_expression"
                | "cast_expression"
                | "initializer_list"
                | "initializer_expression"
                | "collection_expression"
                | "heredoc"
                | "nowdoc"
        )
}

fn file_key_input(name: &str, index: usize, named: Option<&str>) -> bool {
    let key_slot = match name {
        "openssl_sign" | "openssl_private_encrypt" | "openssl_private_decrypt" => 2,
        "openssl_pkey_get_private" | "openssl_get_privatekey" => 0,
        _ => return false,
    };
    named.map_or(index == key_slot, |n| n == "privatekey")
}
fn secret_named_argument(name: &str, parameter: &str, qualified: &str) -> bool {
    // Public verification material and messages do not become secret merely
    // because the same API also accepts a key elsewhere.
    if name == "openssl_verify" {
        return false;
    }
    if name.starts_with("sodium_crypto_") {
        return matches!(
            parameter,
            "secretkey" | "keypair" | "clientkeypair" | "serverkeypair" | "decapsulationkey"
        ) || (parameter == "seed" && name.ends_with("_seed_keypair"))
            || (parameter == "n"
                && matches!(
                    name,
                    "sodium_crypto_scalarmult" | "sodium_crypto_scalarmult_base"
                ));
    }
    if matches!(name, "mldsa44_sign" | "mldsa65_sign" | "mldsa87_sign") {
        return parameter == "sk";
    }
    if matches!(
        name,
        "mlkem512_decaps" | "mlkem768_decaps" | "mlkem1024_decaps" | "xwing_decaps"
    ) {
        return parameter == "dk";
    }
    if matches!(
        parameter,
        "privatekey" | "secretkey" | "password" | "passphrase" | "privateseed" | "hmackey"
    ) {
        return true;
    }
    if name.starts_with("import")
        || matches!(
            name,
            "createfrompem" | "createfromencryptedpem" | "frombytes"
        )
    {
        return secret_argument(name, 0, qualified)
            && matches!(
                parameter,
                "source" | "input" | "data" | "bytes" | "key" | "seed"
            );
    }
    false
}

fn secret_argument(name: &str, index: usize, qualified: &str) -> bool {
    if qualified.len() > 512 {
        return true;
    }
    match name {
        "openssl_sign" => index == 2,
        "mldsa44_sign"
        | "mldsa65_sign"
        | "mldsa87_sign"
        | "mlkem512_decaps"
        | "mlkem768_decaps"
        | "mlkem1024_decaps"
        | "xwing_decaps"
        | "mlkem512_seed_keypair"
        | "mlkem768_seed_keypair"
        | "mlkem1024_seed_keypair"
        | "mldsa44_seed_keypair"
        | "mldsa65_seed_keypair"
        | "mldsa87_seed_keypair"
        | "xwing_seed_keypair" => index == 0,
        "openssl_private_encrypt" | "openssl_private_decrypt" => index == 2,
        "openssl_pkey_get_private" | "openssl_get_privatekey" => index <= 1,
        "openssl_encrypt" | "openssl_decrypt" => index == 2,
        "sodium_crypto_sign" | "sodium_crypto_sign_detached" => index == 1,
        "sodium_crypto_box"
        | "sodium_crypto_box_open"
        | "sodium_crypto_secretbox"
        | "sodium_crypto_secretbox_open" => index == 2,
        "sodium_crypto_box_seal_open" => index == 1,
        "sodium_crypto_sign_publickey_from_secretkey"
        | "sodium_crypto_sign_keypair_from_secretkey_and_publickey"
        | "sodium_crypto_box_publickey_from_secretkey"
        | "sodium_crypto_box_keypair_from_secretkey_and_publickey"
        | "sodium_crypto_scalarmult"
        | "sodium_crypto_scalarmult_base"
        | "sodium_crypto_sign_publickey"
        | "sodium_crypto_sign_secretkey"
        | "sodium_crypto_box_publickey"
        | "sodium_crypto_box_secretkey"
        | "sodium_crypto_kx_seed_keypair"
        | "sodium_crypto_kx_publickey"
        | "sodium_crypto_kx_secretkey"
        | "sodium_crypto_kx_client_session_keys"
        | "sodium_crypto_kx_server_session_keys" => index == 0,
        n if n.starts_with("sodium_crypto_kem_")
            && (n.ends_with("_seed_keypair")
                || n.ends_with("_secretkey")
                || n.ends_with("_publickey")) =>
        {
            index == 0
        }
        "sodium_crypto_sign_seed_keypair" | "sodium_crypto_box_seed_keypair" => index == 0,
        "evp_pkey_new_raw_private_key" => index == 2,
        "evp_pkey_new_raw_private_key_ex" => index == 3,
        "oqs_kem_decaps" => index == 1 || index == 3,
        "oqs_sig_sign" => index == 5,
        "secp256k1_ecdsa_sign" => index == 3,
        "rsa_sign" | "dsa_sign" | "ecdsa_sign" => index == 5,
        "wc_rsassl_sign" | "wc_rsaprivatedecrypt" => index == 4,
        "rsa_private_encrypt" | "rsa_private_decrypt" => index == 3,
        "dh_compute_key" => index == 2,
        "ecdh_compute_key" => index == 3,
        "evp_digestsigninit" => index == 4,
        "evp_digestsigninit_ex" => index == 5,
        "pk_signer" => index == 0,
        "sign" | "decap_secret" => index == 1,
        "signature" | "keyencapsulation"
            if qualified
                .split_whitespace()
                .collect::<String>()
                .contains("oqs::") =>
        {
            index == 1
        }
        "derivekeyfromhmac" => index >= 2,
        "createfrompem" | "createfromencryptedpem" => true,
        "createencryptor" | "createdecryptor" => index == 0,
        "frombytes" => {
            qualified.to_ascii_lowercase().contains("signingkey")
                || qualified.to_ascii_lowercase().contains("privatekey")
                || qualified.to_ascii_lowercase().contains("secretkey")
                || qualified.to_ascii_lowercase().contains("decapsulationkey")
        }
        n if n.starts_with("import")
            && (n.contains("private")
                || n.contains("seed")
                || n.contains("decapsulation")
                || n.contains("encrypted")
                || n == "importfrompem") =>
        {
            true
        }
        n if n.starts_with("sodium_crypto_kem") && n.ends_with("_dec") => index == 1,
        _ => false,
    }
}
