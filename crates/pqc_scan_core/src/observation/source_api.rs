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
    pub(crate) fn public_source_label(&self) -> Option<String> {
        self.source_library
            .map(|r| r.label().to_owned())
            .or_else(|| self.source_configuration.as_ref().map(|r| r.label()))
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
