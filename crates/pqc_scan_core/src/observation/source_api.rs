//! Finite source-API facts. This does not change legacy lexical normalization.
use super::{AlgorithmSpec, ObservationDraft};
use std::collections::BTreeMap;
/// Finite project references, never an installation or runtime-capability claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLibraryReference {
    PhpOqsNative,
    PhpOqsFfi,
    CSharpOqsOfficial,
    CSharpOqsCommunity,
    CSharpBcTls,
    COpenSslInclude,
    CLibOqsInclude,
    CppLibOqsInclude,
}
impl SourceLibraryReference {
    pub fn label(self) -> &'static str {
        match self {
            Self::PhpOqsNative => "Muzosh oqsphp wrapper reference",
            Self::PhpOqsFfi => "mir-evgenii/liboqs-php FFI wrapper reference",
            Self::CSharpOqsOfficial => "C# obsolete official dotnetOQS wrapper reference",
            Self::CSharpOqsCommunity => "C# OpenForge community OQS wrapper reference",
            Self::CSharpBcTls => "C# Bouncy Castle TLS library reference",
            Self::COpenSslInclude => "C/C++ OpenSSL include reference",
            Self::CLibOqsInclude => "C/C++ liboqs include reference",
            Self::CppLibOqsInclude => "C/C++ liboqs-cpp include reference",
        }
    }
    pub fn marker(self) -> &'static str {
        match self {
            Self::PhpOqsNative => "library_ref_php_oqsphp_native",
            Self::PhpOqsFfi => "library_ref_php_liboqs_ffi",
            Self::CSharpOqsOfficial => "library_ref_csharp_dotnetoqs_legacy",
            Self::CSharpOqsCommunity => "library_ref_csharp_openforge_community",
            Self::CSharpBcTls => "library_ref_csharp_bouncycastle_tls",
            Self::COpenSslInclude => "library_ref_c_openssl_include",
            Self::CLibOqsInclude => "library_ref_c_liboqs_include",
            Self::CppLibOqsInclude => "library_ref_cpp_liboqs_include",
        }
    }
    /// Matches code-owned fixed labels only; no scanned value is retained.
    pub fn from_label(label: &str) -> Option<Self> {
        [
            Self::PhpOqsNative,
            Self::PhpOqsFfi,
            Self::CSharpOqsOfficial,
            Self::CSharpOqsCommunity,
            Self::CSharpBcTls,
            Self::COpenSslInclude,
            Self::CLibOqsInclude,
            Self::CppLibOqsInclude,
        ]
        .into_iter()
        .find(|v| v.label() == label)
    }
}
impl AlgorithmSpec {
    /// A canonical source-selected family and explicit public parameters only.
    /// No backend, execution success, active usage or signing purpose is inferred.
    pub fn source_api(name: &str, parameters: &BTreeMap<String, String>) -> Option<Self> {
        if name.len() > 64 || parameters.len() > 2 || parameters.values().any(|v| v.len() > 64) {
            return None;
        }
        let extra = matches!(
            name,
            "DSA"
                | "DH"
                | "EC"
                | "ECDH"
                | "ECDSA"
                | "X25519"
                | "X-Wing"
                | "CompositeMLKem"
                | "CompositeMLDsa"
                | "HPKE"
                | "sntrup761x25519-sha512"
        ) || exact_slh(name);
        let mut value = if extra {
            Self {
                namespace: "named".into(),
                identifier: name.into(),
                purpose: "reference".into(),
                parameters: BTreeMap::new(),
            }
        } else {
            Self::recognized(name).filter(|v| v.identifier == name)?
        };
        if parameters.is_empty() {
            return Some(value);
        }
        if !matches!(name, "RSA" | "DSA" | "DH" | "EC" | "ECDH" | "ECDSA") {
            return None;
        }
        if parameters
            .keys()
            .any(|k| !matches!(k.as_str(), "key_bits" | "named_curve_oid"))
        {
            return None;
        }
        if let Some(bits) = parameters.get("key_bits") {
            let n = bits.parse::<u32>().ok()?;
            if n == 0 || n > 32768 || n.to_string() != *bits {
                return None;
            }
        }
        if let Some(oid) = parameters.get("named_curve_oid") {
            if !matches!(name, "EC" | "ECDH" | "ECDSA") {
                return None;
            }
            let expected = match oid.as_str() {
                "1.2.840.10045.3.1.7" => "256",
                "1.3.132.0.34" => "384",
                "1.3.132.0.35" => "521",
                _ => return None,
            };
            if parameters.get("key_bits").is_some_and(|b| b != expected) {
                return None;
            }
        }
        value.parameters = parameters.clone();
        Some(value)
    }
}
fn exact_slh(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("SLH-DSA-") else {
        return false;
    };
    let Some((hash, set)) = rest.split_once('-') else {
        return false;
    };
    matches!(hash, "SHA2" | "SHAKE")
        && matches!(set, "128s" | "128f" | "192s" | "192f" | "256s" | "256f")
}
impl ObservationDraft {
    /// Source byte interval without source text or arguments. Coordinates are
    /// decoded UTF-8 before file binding and original file bytes after binding.
    pub fn source_span(&self) -> Option<(usize, usize)> {
        if self.method != "ast_pattern" {
            return None;
        }
        self.target
            .byte_start
            .zip(self.target.byte_end)
            .filter(|(start, end)| start < end)
    }
    /// Replace token-derived guesses with only the bounded handler's exact API
    /// facts. Raw arguments must not introduce additional algorithm identities.
    pub fn with_source_api_references(mut self, names: &[String]) -> Self {
        if self.method != "ast_pattern"
            || self.assurance != "syntactic"
            || self.usage != "referenced"
        {
            return self;
        }
        self.source_api_typed = true;
        self.algorithms = names
            .iter()
            .take(16)
            .filter_map(|n| AlgorithmSpec::source_api(n, &BTreeMap::new()))
            .collect();
        self.algorithms.sort_by_key(AlgorithmSpec::key);
        self.algorithms.dedup();
        self
    }
    /// Retain only an opaque, finite project identity in the existing marker
    /// field. It carries no version, primitive, backend or active-use assertion.
    pub fn with_source_library_reference(mut self, reference: SourceLibraryReference) -> Self {
        if self.source_api_typed
            && self.method == "ast_pattern"
            && self.assurance == "syntactic"
            && self.usage == "referenced"
            && self.algorithms.is_empty()
        {
            self.source_configuration = None;
            self.source_library = Some(reference);
            self.evidence.marker_kind = Some(reference.marker().into());
        }
        self
    }
    /// Preserve an exact finite public setting, without an algorithm asset or
    /// any claim that the provider/group was enabled, available or negotiated.
    pub fn with_source_configuration_reference(
        mut self,
        reference: super::SourceConfigurationReference,
    ) -> Self {
        if self.source_api_typed
            && self.method == "ast_pattern"
            && self.assurance == "syntactic"
            && self.usage == "referenced"
            && self.algorithms.is_empty()
        {
            self.source_library = None;
            self.evidence.marker_kind = Some(reference.marker());
            self.source_configuration = Some(reference);
        }
        self
    }
    /// A finite public API token only; never accepts source arguments, receivers
    /// or arbitrary metadata. This transient admission token is not serialized or
    /// included in observation/finding identities. Core may replace it only with
    /// an inspected, span-bound and redacted source excerpt after all detectors.
    pub fn with_source_api_display_name(mut self, name: &str) -> Self {
        if self.source_api_typed
            && self.method == "ast_pattern"
            && self.assurance == "syntactic"
            && self.usage == "referenced"
        {
            self.source_api_display =
                public_api_name(name).map(|name| format!("{name}(/* arguments omitted */)"));
        }
        self
    }
    pub(crate) fn public_source_label(&self) -> Option<String> {
        self.source_library
            .map(|r| r.label().to_owned())
            .or_else(|| self.source_configuration.as_ref().map(|r| r.label()))
            .or_else(|| self.source_api_display.clone())
    }
    /// The source producer computed these columns with line_col_for_offset.
    /// Legacy producers retain their existing mixed-column annotation.
    pub fn with_code_point_columns(mut self) -> Self {
        if self.method == "ast_pattern" {
            self.target.column_kind = "unicode_code_points".into();
        }
        self
    }
    /// Compare opaque source facts across ambiguous grammar candidates without
    /// exposing raw arguments or merging different primitive/parameter bindings.
    pub fn equivalent_source_reference(&self, other: &Self) -> bool {
        self.method == "ast_pattern"
            && other.method == "ast_pattern"
            && self.source_span().is_some()
            && self.producer == other.producer
            && self.assurance == other.assurance
            && self.usage == other.usage
            && self.raw_path == other.raw_path
            && self.rule_id == other.rule_id
            && self.target.byte_start == other.target.byte_start
            && self.target.byte_end == other.target.byte_end
            && self.target.line == other.target.line
            && self.target.column == other.target.column
            && self.target.column_kind == other.target.column_kind
            && self.algorithms == other.algorithms
            && self.source_library == other.source_library
            && self.source_configuration == other.source_configuration
    }
}

// Finite public tokens, canonicalized only after language-aware acceptance.
// No prefix/pattern allowlist. In particular, a scanned
// receiver, alias, credential or string literal can never extend this set.
fn public_api_name(name: &str) -> Option<&'static str> {
    const NAMES: &[&str] = &[
        "RSA_new",
        "RSA_generate_key_ex",
        "RSA_public_encrypt",
        "RSA_private_decrypt",
        "RSA_private_encrypt",
        "RSA_public_decrypt",
        "RSA_sign",
        "RSA_verify",
        "DSA_new",
        "DSA_generate_key",
        "DSA_generate_parameters_ex",
        "DSA_sign",
        "DSA_verify",
        "DH_new",
        "DH_generate_key",
        "DH_generate_parameters_ex",
        "DH_compute_key",
        "EC_KEY_new",
        "EC_KEY_new_by_curve_name",
        "EC_KEY_generate_key",
        "ECDSA_sign",
        "ECDSA_verify",
        "ECDH_compute_key",
        "secp256k1_ecdsa_sign",
        "secp256k1_ecdsa_verify",
        "wc_RsaSSL_Sign",
        "wc_RsaSSL_Verify",
        "wc_RsaPublicEncrypt",
        "wc_RsaPrivateDecrypt",
        "EVP_PKEY_CTX_new_id",
        "EVP_PKEY_CTX_new_from_name",
        "EVP_PKEY_CTX_new",
        "EVP_PKEY_CTX_new_from_pkey",
        "EVP_PKEY_CTX_dup",
        "EVP_PKEY_new_raw_private_key",
        "EVP_PKEY_new_raw_private_key_ex",
        "EVP_PKEY_new_raw_public_key",
        "EVP_PKEY_new_raw_public_key_ex",
        "EVP_PKEY_keygen_init",
        "EVP_PKEY_keygen",
        "EVP_PKEY_generate",
        "EVP_PKEY_paramgen_init",
        "EVP_PKEY_paramgen",
        "EVP_PKEY_encapsulate_init",
        "EVP_PKEY_encapsulate",
        "EVP_PKEY_decapsulate_init",
        "EVP_PKEY_decapsulate",
        "EVP_PKEY_sign_init",
        "EVP_PKEY_sign",
        "EVP_PKEY_verify_init",
        "EVP_PKEY_verify",
        "EVP_PKEY_encrypt_init",
        "EVP_PKEY_encrypt",
        "EVP_PKEY_decrypt_init",
        "EVP_PKEY_decrypt",
        "EVP_PKEY_derive_init",
        "EVP_PKEY_derive",
        "EVP_DigestSignInit",
        "EVP_DigestVerifyInit",
        "EVP_DigestSignInit_ex",
        "EVP_DigestVerifyInit_ex",
        "OSSL_PROVIDER_load",
        "OSSL_PROVIDER_load_ex",
        "EVP_set_default_properties",
        "SSL_CTX_set1_groups_list",
        "SSL_set1_groups_list",
        "X509_verify_cert",
        "OQS_KEM_new",
        "OQS_KEM_keypair",
        "OQS_KEM_encaps",
        "OQS_KEM_decaps",
        "OQS_SIG_new",
        "OQS_SIG_keypair",
        "OQS_SIG_sign",
        "OQS_SIG_verify",
        "KeyEncapsulation",
        "Signature",
        "PK_Signer",
        "PK_Verifier",
        "encap_secret",
        "decap_secret",
        "generate_keypair",
        "generate_keypair_seed",
        "export_secret_key",
        "sign",
        "verify",
        "Create",
        "CreateFromPem",
        "CreateFromEncryptedPem",
        "GenerateKey",
        "GenerateKeyPair",
        "Import",
        "ImportPrivateSeed",
        "ImportPrivateKey",
        "ImportPublicKey",
        "ImportSubjectPublicKeyInfo",
        "SignData",
        "VerifyData",
        "SignHash",
        "VerifyHash",
        "Encrypt",
        "Decrypt",
        "DeriveKeyMaterial",
        "DeriveRawSecretAgreement",
        "DeriveKeyFromHash",
        "GenerateSignature",
        "VerifySignature",
        "CreateSignature",
        "Encapsulate",
        "Decapsulate",
        "Build",
        "ComputeSignature",
        "CheckSignature",
        "RSAPKCS1SignatureFormatter",
        "RSAPKCS1SignatureDeformatter",
        "RSAPKCS1KeyExchangeFormatter",
        "RSAPKCS1KeyExchangeDeformatter",
        "RsaKeyPairGenerator",
        "ECKeyPairGenerator",
        "DHKeyPairGenerator",
        "DsaKeyPairGenerator",
        "MLKemKeyPairGenerator",
        "MLDsaKeyPairGenerator",
        "SlhDsaKeyPairGenerator",
        "RsaEngine",
        "RsaDigestSigner",
        "openssl_pkey_new",
        "openssl_pkey_get_private",
        "openssl_pkey_get_public",
        "openssl_pkey_derive",
        "openssl_sign",
        "openssl_verify",
        "openssl_public_encrypt",
        "openssl_private_decrypt",
        "openssl_private_encrypt",
        "openssl_public_decrypt",
        "sodium_crypto_sign_keypair",
        "sodium_crypto_sign_seed_keypair",
        "sodium_crypto_sign_detached",
        "sodium_crypto_sign_verify_detached",
        "sodium_crypto_box_keypair",
        "sodium_crypto_box_seed_keypair",
        "sodium_crypto_scalarmult",
        "sodium_crypto_kx_keypair",
        "sodium_crypto_kem_keypair",
        "sodium_crypto_kem_encapsulate",
        "sodium_crypto_kem_decapsulate",
        "signData",
        "verifySignature",
        "encode",
        "decode",
        "generateKeys",
        "generateKeyPair",
        "encapsulate",
        "decapsulate",
        "keygen",
        "verifyDetached",
        "createKey",
        "CalculateAgreement",
        "CompositeMLDsa",
        "CompositeMLKem",
        "DSA",
        "DSACng",
        "DSACryptoServiceProvider",
        "DSAOpenSsl",
        "DeriveKeyFromHmac",
        "DsaSigner",
        "EC",
        "ECDH",
        "ECDHBasicAgreement",
        "ECDiffieHellman",
        "ECDiffieHellmanCng",
        "ECDiffieHellmanOpenSsl",
        "ECDsa",
        "ECDsaCng",
        "ECDsaOpenSsl",
        "ECDsaSigner",
        "HPKE",
        "Hpke",
        "ImportDecapsulationKey",
        "ImportEncapsulationKey",
        "ImportEncryptedPkcs8PrivateKey",
        "ImportFromEncryptedPem",
        "ImportFromPem",
        "ImportMLDsaPrivateKey",
        "ImportMLDsaPublicKey",
        "ImportPkcs8PrivateKey",
        "ImportSlhDsaPrivateKey",
        "ImportSlhDsaPublicKey",
        "LibOqs",
        "MLDsa",
        "MLDsaSigner",
        "MLKem",
        "MLKemDecapsulator",
        "MLKemEncapsulator",
        "ProcessBlock",
        "RSA",
        "RSACng",
        "RSACryptoServiceProvider",
        "RSAOpenSsl",
        "SignedXml",
        "SlhDsa",
        "SlhDsaSigner",
        "Tls",
        "X509Chain",
        "dotnetOQS",
        "fromBytes",
        "mldsa44_keygen",
        "mldsa44_sign",
        "mldsa44_verify",
        "mldsa65_keygen",
        "mldsa65_sign",
        "mldsa65_verify",
        "mldsa87_keygen",
        "mldsa87_sign",
        "mldsa87_verify",
        "mlkem1024_decaps",
        "mlkem1024_encaps",
        "mlkem1024_keygen",
        "mlkem512_decaps",
        "mlkem512_encaps",
        "mlkem512_keygen",
        "mlkem768_decaps",
        "mlkem768_encaps",
        "mlkem768_keygen",
        "signMessage",
        "sodium_crypto_box",
        "sodium_crypto_box_keypair_from_secretkey_and_publickey",
        "sodium_crypto_box_open",
        "sodium_crypto_box_publickey",
        "sodium_crypto_box_publickey_from_secretkey",
        "sodium_crypto_box_seal",
        "sodium_crypto_box_seal_open",
        "sodium_crypto_box_secretkey",
        "sodium_crypto_kem_dec",
        "sodium_crypto_kem_enc",
        "sodium_crypto_kem_mlkem768_dec",
        "sodium_crypto_kem_mlkem768_enc",
        "sodium_crypto_kem_mlkem768_keypair",
        "sodium_crypto_kem_mlkem768_publickey",
        "sodium_crypto_kem_mlkem768_secretkey",
        "sodium_crypto_kem_mlkem768_seed_keypair",
        "sodium_crypto_kem_publickey",
        "sodium_crypto_kem_secretkey",
        "sodium_crypto_kem_seed_keypair",
        "sodium_crypto_kx_client_session_keys",
        "sodium_crypto_kx_publickey",
        "sodium_crypto_kx_secretkey",
        "sodium_crypto_kx_seed_keypair",
        "sodium_crypto_kx_server_session_keys",
        "sodium_crypto_scalarmult_base",
        "sodium_crypto_sign",
        "sodium_crypto_sign_keypair_from_secretkey_and_publickey",
        "sodium_crypto_sign_open",
        "sodium_crypto_sign_publickey",
        "sodium_crypto_sign_publickey_from_secretkey",
        "sodium_crypto_sign_secretkey",
        "validateReference",
        "verifyMessage",
        "xwing_decaps",
        "xwing_encaps",
        "xwing_keygen",
    ];
    NAMES
        .iter()
        .copied()
        .find(|n| *n == name)
        .or_else(|| NAMES.iter().copied().find(|n| n.eq_ignore_ascii_case(name)))
}

#[cfg(test)]
mod display_names_tests {
    use super::public_api_name;
    #[test]
    fn only_exact_public_api_tokens_can_be_displayed() {
        assert_eq!(
            public_api_name("secp256k1_ecdsa_sign"),
            Some("secp256k1_ecdsa_sign")
        );
        assert_eq!(public_api_name("OPENSSL_SIGN"), Some("openssl_sign"));
        for name in [
            "secp256k1_ecdsa_sign_PRIVATE086",
            "RSA_new(secret)",
            "PRIVATE086",
            "x.sign",
            "signData\nPRIVATE086",
            "",
            "RS256",
        ] {
            assert!(public_api_name(name).is_none());
        }
    }
}
