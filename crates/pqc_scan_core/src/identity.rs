//! Versioned identities for the additive native model, never legacy finding IDs.
use sha2::{Digest, Sha256};

/// Domain/version and component count are framed along with every tuple member.
pub fn native_id(domain: &str, parts: &[&str]) -> String {
    let mut hash = Sha256::new();
    let values = ["pqc-scan-native", "1", domain];
    for value in values {
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value.as_bytes());
    }
    hash.update((parts.len() as u64).to_be_bytes());
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part.as_bytes());
    }
    format!("{domain}-v1-{}", hex::encode(hash.finalize()))
}

pub(crate) fn safe_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
}

pub(crate) fn valid_scope(value: &str) -> bool {
    value.len() <= 128
        && value
            .split('/')
            .all(|s| safe_token(s) && s != "." && s != "..")
}

pub(crate) fn relative_path(root: &std::path::Path, value: &str) -> Option<String> {
    use std::path::Component;
    let path = std::path::Path::new(value);
    let relative = path
        .strip_prefix(root)
        .ok()
        .or_else(|| (!path.is_absolute()).then_some(path))?;
    let mut parts = Vec::new();
    for part in relative.components() {
        match part {
            Component::Normal(s) => {
                let s = s.to_str()?;
                if s.chars().any(char::is_control) || s.contains([':', '\\']) {
                    return None;
                }
                parts.push(s);
            }
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuples_domains_and_empty_fields_are_unambiguous() {
        assert_ne!(
            native_id("asset", &["ab", "c"]),
            native_id("asset", &["a", "bc"])
        );
        assert_ne!(native_id("asset", &["a", ""]), native_id("asset", &["a"]));
        assert_ne!(native_id("asset", &["x"]), native_id("observation", &["x"]));
        assert_eq!(
            native_id("asset", &["日本😀"]),
            native_id("asset", &["日本😀"])
        );
        assert_eq!(native_id("asset", &["x"]).len(), "asset-v1-".len() + 64);
    }
    #[test]
    fn paths_and_scope_do_not_admit_roots_credentials_or_traversal() {
        use std::path::Path;
        assert_eq!(
            relative_path(Path::new("/one"), "/one/日本/a.rs").as_deref(),
            Some("日本/a.rs")
        );
        assert_eq!(relative_path(Path::new("/two"), "/outside/a.rs"), None);
        assert_eq!(relative_path(Path::new("/one"), "../outside"), None);
        assert!(valid_scope("team/project"));
        for s in [
            "",
            "../project",
            "https://token@host/project",
            "/root",
            "a//b",
        ] {
            assert!(!valid_scope(s), "{s}");
        }
    }
}
