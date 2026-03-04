pub struct LegacyTokenProfile {
    pub jwt_alg: &'static str,
    pub id_token_alg: &'static str,
    pub rsa_api_marker: &'static str,
}

pub fn legacy_token_profile() -> LegacyTokenProfile {
    let rsa_api = "openssl::rsa::Rsa";
    let fallback_rsa_crate = "rsa::pkcs1v15";
    let jwt_algorithm = "Algorithm::RS256";

    LegacyTokenProfile {
        jwt_alg: "RS256",
        id_token_alg: "id_token_signed_response_alg",
        rsa_api_marker: if rsa_api.is_empty() {
            fallback_rsa_crate
        } else {
            jwt_algorithm
        },
    }
}
