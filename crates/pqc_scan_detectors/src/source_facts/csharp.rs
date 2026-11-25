//! Bounded C# API identities and operations; all captured facts remain references.
mod names;
mod values;
use super::{builtin, context::Context, Facts};
use names::{in_error, Names};
use pqc_scan_rules::RuleSet;
use tree_sitter::Node;
use values::Values;
const API: &str = "CSHARP_CRYPTO_API";
const PQC: &str = "CSHARP_PQC_API";
const REFERENCE: &str = "CSHARP_CRYPTO_REFERENCE";
const STD: &str = "System.Security.Cryptography.";
const BC: &str = "Org.BouncyCastle.Crypto.";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Classical(&'static str),
    Pqc(&'static str),
    Construction(&'static str),
    Generator(&'static str),
    Engine(&'static str),
    Encap,
    Decap,
    Signer(&'static str),
    OfficialWrapper,
    CommunityWrapper,
    BcTls,
    CertificateChain,
    XmlSignature,
    RsaFormatter,
    RsaDeformatter,
}
#[derive(Clone, Debug)]
struct Key {
    kind: Kind,
    algorithm: String,
}
impl Kind {
    fn family(self) -> Option<&'static str> {
        match self {
            Self::RsaFormatter | Self::RsaDeformatter => Some("RSA"),
            Self::Classical(a)
            | Self::Pqc(a)
            | Self::Construction(a)
            | Self::Generator(a)
            | Self::Engine(a)
            | Self::Signer(a) => Some(a),
            Self::Encap | Self::Decap => Some("ML-KEM"),
            Self::OfficialWrapper
            | Self::CommunityWrapper
            | Self::BcTls
            | Self::CertificateChain
            | Self::XmlSignature => None,
        }
    }
    fn id(self) -> &'static str {
        if matches!(
            self,
            Self::OfficialWrapper | Self::CommunityWrapper | Self::BcTls
        ) {
            REFERENCE
        } else if self.family().is_some_and(|s| {
            s.starts_with("ML-")
                || s.starts_with("SLH-")
                || matches!(s, "CompositeMLKem" | "CompositeMLDsa" | "HPKE")
        }) {
            PQC
        } else {
            API
        }
    }
    fn key(self) -> Option<Key> {
        Some(Key {
            kind: self,
            algorithm: if matches!(self, Self::CertificateChain | Self::XmlSignature) {
                String::new()
            } else {
                self.family()?.into()
            },
        })
    }
}
fn class_info(name: &str) -> Option<Kind> {
    if name == "dotnetOQS" || name.starts_with("dotnetOQS.") {
        return Some(Kind::OfficialWrapper);
    }
    if name == "OpenForge.Cryptography.LibOqs" || name.starts_with("OpenForge.Cryptography.LibOqs.")
    {
        return Some(Kind::CommunityWrapper);
    }
    if name == "Org.BouncyCastle.Tls" || name.starts_with("Org.BouncyCastle.Tls.") {
        return Some(Kind::BcTls);
    }
    if let Some(name) = name.strip_prefix(STD) {
        return Some(match name {
            "RSAPKCS1SignatureFormatter" => Kind::RsaFormatter,
            "RSAPKCS1SignatureDeformatter" => Kind::RsaDeformatter,
            "X509Certificates.X509Chain" => Kind::CertificateChain,
            "Xml.SignedXml" => Kind::XmlSignature,
            "RSA" | "RSACryptoServiceProvider" | "RSACng" | "RSAOpenSsl" => Kind::Classical("RSA"),
            "DSA" | "DSACryptoServiceProvider" | "DSACng" | "DSAOpenSsl" => Kind::Classical("DSA"),
            "ECDsa" | "ECDsaCng" | "ECDsaOpenSsl" => Kind::Classical("EC"),
            "ECDiffieHellman" | "ECDiffieHellmanCng" | "ECDiffieHellmanOpenSsl" => {
                Kind::Classical("ECDH")
            }
            "MLKem" => Kind::Pqc("ML-KEM"),
            "MLDsa" => Kind::Pqc("ML-DSA"),
            "SlhDsa" => Kind::Pqc("SLH-DSA"),
            "CompositeMLKem" => Kind::Construction("CompositeMLKem"),
            "CompositeMLDsa" => Kind::Construction("CompositeMLDsa"),
            "Hpke" => Kind::Construction("HPKE"),
            _ => return None,
        });
    }
    Some(match name.strip_prefix(BC)? {
        "Generators.RsaKeyPairGenerator" => Kind::Generator("RSA"),
        "Generators.DsaKeyPairGenerator" => Kind::Generator("DSA"),
        "Generators.ECKeyPairGenerator" => Kind::Generator("EC"),
        "Generators.MLKemKeyPairGenerator" => Kind::Generator("ML-KEM"),
        "Generators.MLDsaKeyPairGenerator" => Kind::Generator("ML-DSA"),
        "Generators.SlhDsaKeyPairGenerator" => Kind::Generator("SLH-DSA"),
        "Engines.RsaEngine" => Kind::Engine("RSA"),
        "Signers.DsaSigner" => Kind::Signer("DSA"),
        "Signers.RsaDigestSigner" => Kind::Signer("RSA"),
        "Signers.ECDsaSigner" => Kind::Signer("EC"),
        "Agreement.ECDHBasicAgreement" => Kind::Engine("ECDH"),
        "Kems.MLKemEncapsulator" => Kind::Encap,
        "Kems.MLKemDecapsulator" => Kind::Decap,
        "Signers.MLDsaSigner" => Kind::Signer("ML-DSA"),
        "Signers.SlhDsaSigner" => Kind::Signer("SLH-DSA"),
        _ => return None,
    })
}
fn parameter(name: &str) -> Option<String> {
    let (kind, value, bc) = if let Some(s) = name.strip_prefix(STD) {
        let (k, v) = s.split_once('.')?;
        (k, v, false)
    } else {
        let s = name.strip_prefix(BC)?.strip_prefix("Parameters.")?;
        let (k, v) = s.split_once('.')?;
        (k, v, true)
    };
    let family = match kind {
        "MLKemAlgorithm" if !bc => "ML-KEM",
        "MLKemParameters" if bc => "ML-KEM",
        "MLDsaAlgorithm" if !bc => "ML-DSA",
        "MLDsaParameters" if bc => "ML-DSA",
        "SlhDsaAlgorithm" if !bc => "SLH-DSA",
        "SlhDsaParameters" if bc => "SLH-DSA",
        _ => return None,
    };
    let set = if bc {
        match family {
            "ML-KEM" => value.strip_prefix("ml_kem_")?,
            "ML-DSA" => value.strip_prefix("ml_dsa_")?,
            _ => value.strip_prefix("slh_dsa_")?,
        }
    } else {
        match family {
            "ML-KEM" => value.strip_prefix("MLKem")?,
            "ML-DSA" => value.strip_prefix("MLDsa")?,
            _ => value.strip_prefix("SlhDsa")?,
        }
    };
    let name = match family {
        "ML-KEM" if matches!(set, "512" | "768" | "1024") => format!("ML-KEM-{set}"),
        "ML-DSA" if matches!(set, "44" | "65" | "87") => format!("ML-DSA-{set}"),
        "SLH-DSA" => {
            let (hash, set) = if bc {
                set.strip_prefix("sha2_")
                    .map(|s| ("SHA2", s))
                    .or_else(|| set.strip_prefix("shake_").map(|s| ("SHAKE", s)))?
            } else {
                set.strip_prefix("Sha2_")
                    .map(|s| ("SHA2", s))
                    .or_else(|| set.strip_prefix("Shake").map(|s| ("SHAKE", s)))?
            };
            if !matches!(set, "128s" | "128f" | "192s" | "192f" | "256s" | "256f") {
                return None;
            }
            format!("SLH-DSA-{hash}-{set}")
        }
        _ => return None,
    };
    Some(name)
}
fn parameter_family(name: &str) -> Option<&'static str> {
    let (typ, _) = name.rsplit_once('.')?;
    match typ {
        "System.Security.Cryptography.MLKemAlgorithm"
        | "Org.BouncyCastle.Crypto.Parameters.MLKemParameters" => Some("ML-KEM"),
        "System.Security.Cryptography.MLDsaAlgorithm"
        | "Org.BouncyCastle.Crypto.Parameters.MLDsaParameters" => Some("ML-DSA"),
        "System.Security.Cryptography.SlhDsaAlgorithm"
        | "Org.BouncyCastle.Crypto.Parameters.SlhDsaParameters" => Some("SLH-DSA"),
        _ => None,
    }
}
fn compatible(family: &str, selected: &str) -> bool {
    selected == family
        || selected
            .strip_prefix(family)
            .is_some_and(|s| s.starts_with('-'))
}
fn factory_method(kind: Kind, method: &str) -> bool {
    match kind {
        Kind::Classical(_) => matches!(
            method,
            "Create" | "CreateFromPem" | "CreateFromEncryptedPem"
        ),
        Kind::Pqc(_) => matches!(
            method,
            "GenerateKey"
                | "ImportFromPem"
                | "ImportFromEncryptedPem"
                | "ImportSubjectPublicKeyInfo"
                | "ImportPkcs8PrivateKey"
                | "ImportEncryptedPkcs8PrivateKey"
                | "ImportPrivateSeed"
                | "ImportDecapsulationKey"
                | "ImportEncapsulationKey"
                | "ImportMLDsaPrivateKey"
                | "ImportMLDsaPublicKey"
                | "ImportSlhDsaPrivateKey"
                | "ImportSlhDsaPublicKey"
        ),
        Kind::Construction(_) => true,
        _ => false,
    }
}
fn operation(kind: Kind, method: &str) -> bool {
    match kind {
        Kind::Classical("RSA") => matches!(
            method,
            "SignData" | "VerifyData" | "SignHash" | "VerifyHash" | "Encrypt" | "Decrypt"
        ),
        Kind::Classical("EC" | "DSA") => matches!(
            method,
            "SignData"
                | "VerifyData"
                | "SignHash"
                | "VerifyHash"
                | "CreateSignature"
                | "VerifySignature"
        ),
        Kind::Classical("ECDH") => matches!(
            method,
            "DeriveKeyMaterial"
                | "DeriveKeyFromHash"
                | "DeriveKeyFromHmac"
                | "DeriveRawSecretAgreement"
        ),
        Kind::Pqc("ML-KEM") => matches!(method, "Encapsulate" | "Decapsulate"),
        Kind::Pqc(_) => matches!(method, "SignData" | "VerifyData"),
        Kind::Generator(_) => method == "GenerateKeyPair",
        Kind::Signer(_) => matches!(method, "GenerateSignature" | "VerifySignature"),
        Kind::RsaFormatter => method == "CreateSignature",
        Kind::RsaDeformatter => method == "VerifySignature",
        Kind::CertificateChain => method == "Build",
        Kind::XmlSignature => matches!(method, "ComputeSignature" | "CheckSignature"),
        Kind::Encap => method == "Encapsulate",
        Kind::Decap => method == "Decapsulate",
        Kind::Engine("RSA") => method == "ProcessBlock",
        Kind::Engine("ECDH") => method == "CalculateAgreement",
        _ => false,
    }
}
fn crypto_operation_name(method: &str) -> bool {
    matches!(
        method,
        "SignData"
            | "VerifyData"
            | "SignHash"
            | "VerifyHash"
            | "Encrypt"
            | "Decrypt"
            | "Encapsulate"
            | "Decapsulate"
            | "GenerateSignature"
            | "VerifySignature"
            | "GenerateKeyPair"
            | "DeriveKeyMaterial"
            | "DeriveRawSecretAgreement"
            | "CalculateAgreement"
    )
}
fn emit(
    facts: &mut Facts,
    rules: &RuleSet,
    node: Node<'_>,
    kind: Kind,
    algorithm: Option<&str>,
    reason: &'static str,
) {
    if !rules.get(kind.id()).is_some_and(builtin) {
        return;
    }
    let names: Vec<_> = algorithm.into_iter().collect();
    facts.source_api(
        kind.id(),
        node,
        reason,
        &names,
        match kind {
            Kind::OfficialWrapper => "C# obsolete official dotnetOQS wrapper reference",
            Kind::CommunityWrapper => "C# OpenForge community OQS wrapper reference",
            Kind::BcTls => "C# Bouncy Castle TLS library reference",
            _ => "C# cryptographic source reference",
        },
    );
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
    for (node, target) in &names.references {
        if !names.external_path(target) {
            continue;
        }
        if let Some(kind) = class_info(target) {
            emit(
                facts,
                rules,
                *node,
                kind,
                kind.family(),
                "qualified_type_or_wrapper_reference",
            );
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
            "object_creation_expression" => {
                if let Some(key) = values.key(*n) {
                    emit(
                        facts,
                        rules,
                        *n,
                        key.kind,
                        Some(&key.algorithm),
                        "qualified_constructor_reference",
                    );
                } else if let Some(kind) = n
                    .child_by_field_name("type")
                    .and_then(|t| unique_kind(&names, t, true))
                    .filter(|k| k.family().is_none())
                {
                    emit(facts, rules, *n, kind, None, "qualified_library_reference");
                }
            }
            "invocation_expression" => {
                let Some(function) = n.child_by_field_name("function") else {
                    continue;
                };
                if names.path(function).as_deref() == Some("GenerateTraditionalKeyPair")
                    && names.namespace(*n, true)
                        == "Org.BouncyCastle.Crypto.Generators.CompositeMLDsaKeyPairGenerator."
                    && rules
                        .get("SOURCE_CONSTRUCTION_REFERENCE")
                        .is_some_and(builtin)
                {
                    facts.source_api(
                        "SOURCE_CONSTRUCTION_REFERENCE",
                        *n,
                        "composite_traditional_component_reference_not_whole_construction_safety",
                        &["CompositeMLDsa"],
                        "C# composite implementation component reference",
                    );
                }
                if let Some(key) = values.factory(*n, 0) {
                    emit(
                        facts,
                        rules,
                        *n,
                        key.kind,
                        Some(&key.algorithm),
                        "qualified_factory_or_import_reference",
                    );
                    continue;
                }
                if function.kind() == "member_access_expression" {
                    let (Some(object), Some(method)) = (
                        function.child_by_field_name("expression"),
                        function.child_by_field_name("name"),
                    ) else {
                        continue;
                    };
                    let Some(method) = names.path(method) else {
                        names.partial.set(true);
                        continue;
                    };
                    if let Some(key) = values.key(object) {
                        if operation(key.kind, &method) {
                            emit(
                                facts,
                                rules,
                                *n,
                                key.kind,
                                Some(&key.algorithm),
                                "bounded_local_receiver_reference",
                            );
                        } else if crypto_operation_name(&method) {
                            names.partial.set(true);
                        }
                    } else if crypto_operation_name(&method) {
                        names.partial.set(true);
                    }
                } else if !matches!(
                    function.kind(),
                    "identifier" | "qualified_name" | "alias_qualified_name"
                ) {
                    names.partial.set(true);
                }
            }
            "member_access_expression" => {
                if inside_call(*n, ctx) {
                    continue;
                }
                let paths = names.resolve(*n, false);
                let mut selected: Vec<_> = paths.iter().filter_map(|s| parameter(s)).collect();
                selected.sort();
                selected.dedup();
                let mut families: Vec<_> =
                    paths.iter().filter_map(|s| parameter_family(s)).collect();
                families.sort();
                families.dedup();
                if selected.len() == 1 {
                    emit(
                        facts,
                        rules,
                        *n,
                        Kind::Pqc(families.first().copied().unwrap_or("ML-KEM")),
                        Some(&selected[0]),
                        "qualified_parameter_reference",
                    );
                } else if families.len() == 1 {
                    names.partial.set(true);
                    emit(
                        facts,
                        rules,
                        *n,
                        Kind::Pqc(families[0]),
                        Some(families[0]),
                        "unresolved_parameter_reference",
                    );
                }
            }
            "parameter" | "variable_declaration" => {
                if let Some(typ) = n.child_by_field_name("type") {
                    if let Some(kind) = unique_kind(&names, typ, true) {
                        emit(
                            facts,
                            rules,
                            typ,
                            kind,
                            kind.family(),
                            "qualified_type_reference",
                        );
                    }
                }
            }
            _ => {}
        }
    }
    if values.binding_limited {
        ctx.limit("csharp_same_name_bindings", 64, 64);
    }
    facts.partial |= names.partial.get();
}
fn unique_kind(names: &Names<'_, '_, '_>, node: Node<'_>, ignore_vars: bool) -> Option<Kind> {
    let mut found: Vec<_> = names
        .resolve(node, ignore_vars)
        .iter()
        .filter_map(|s| class_info(s))
        .collect();
    found.dedup();
    (found.len() == 1).then(|| found[0])
}
fn inside_call(mut node: Node<'_>, ctx: &Context<'_, '_>) -> bool {
    for _ in 0..64 {
        let Some(p) = node.parent() else {
            return false;
        };
        if matches!(
            p.kind(),
            "invocation_expression" | "object_creation_expression"
        ) {
            return true;
        }
        if matches!(
            p.kind(),
            "expression_statement" | "variable_declarator" | "return_statement" | "block"
        ) {
            return false;
        }
        node = p;
    }
    ctx.limit("csharp_ancestor_depth", 64, 64);
    true
}
