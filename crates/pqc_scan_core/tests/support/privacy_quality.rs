use super::*;

#[test]
fn basic_and_bearer_credentials_are_redacted_without_erasing_public_scheme() {
    for scheme in ["Basic", "basic", "Bearer"] {
        let input = format!("Authorization: {scheme} SYNTHETIC_CREDENTIAL");
        let out = redact_text(&input);
        assert!(!out.contains("SYNTHETIC_CREDENTIAL"));
        assert!(out.contains(scheme));
    }
    for input in [
        "password=\"SYNTHETIC_CREDENTIAL\"",
        "password='SYNTHETIC_CREDENTIAL'",
        "password=SYNTHETIC_CREDENTIAL",
    ] {
        assert!(!redact_text(input).contains("SYNTHETIC_CREDENTIAL"));
    }
}

#[test]
fn unknown_source_line_is_sensitive_if_any_private_or_credential_range_exists() {
    assert!(SourcePrivacy::from_text("-----BEGIN PRIVATE KEY-----").sensitive(0, 0, false));
    assert!(SourcePrivacy::from_text("token=synthetic").sensitive(0, 0, false));
    assert!(!SourcePrivacy::from_text("public text").sensitive(0, 0, false));
}
