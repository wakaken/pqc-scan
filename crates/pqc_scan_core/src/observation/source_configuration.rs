//! Finite public configuration references. These are not algorithm/usage facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConfigurationReference {
    kind: &'static str,
    values: Vec<&'static str>,
}
impl SourceConfigurationReference {
    pub fn provider(value: &str) -> Option<Self> {
        Some(Self {
            kind: "provider",
            values: vec![match value {
                "oqsprovider" => "oqsprovider",
                "default" => "default",
                "legacy" => "legacy",
                _ => return None,
            }],
        })
    }
    pub fn property(value: &str) -> Option<Self> {
        Some(Self {
            kind: "property",
            values: vec![match value {
                "provider=oqsprovider" => "provider=oqsprovider",
                "provider=default" => "provider=default",
                "provider=legacy" => "provider=legacy",
                _ => return None,
            }],
        })
    }
    /// Exact bare colon-list subset only. Provider extensions, grammar operators
    /// and unknown names are not evaluated or silently dropped from the list.
    pub fn tls_groups(value: &str) -> Option<Self> {
        if value.len() > 512 {
            return None;
        }
        let mut values = Vec::new();
        for item in value.split(':') {
            if values.len() == 16 {
                return None;
            }
            values.push(match item {
                "X25519MLKEM768" => "X25519MLKEM768",
                "SecP256r1MLKEM768" => "SecP256r1MLKEM768",
                "SecP384r1MLKEM1024" => "SecP384r1MLKEM1024",
                "MLKEM512" => "MLKEM512",
                "MLKEM768" => "MLKEM768",
                "MLKEM1024" => "MLKEM1024",
                "X25519Kyber768Draft00" => "X25519Kyber768Draft00",
                "X25519" => "X25519",
                "X448" => "X448",
                "P-256" => "P-256",
                "P-384" => "P-384",
                "P-521" => "P-521",
                "secp256r1" => "secp256r1",
                "secp384r1" => "secp384r1",
                "secp521r1" => "secp521r1",
                "ffdhe2048" => "ffdhe2048",
                "ffdhe3072" => "ffdhe3072",
                "ffdhe4096" => "ffdhe4096",
                _ => return None,
            });
        }
        Some(Self {
            kind: "tls_group",
            values,
        })
    }
    pub fn label(&self) -> String {
        let label = if self.kind == "tls_group" {
            "TLS group"
        } else {
            self.kind
        };
        format!(
            "C/C++ {label} configuration reference: {}",
            self.values.join(":")
        )
    }
    pub fn marker(&self) -> String {
        format!("source_{}_ref:{}", self.kind, self.values.join(":"))
    }
}
