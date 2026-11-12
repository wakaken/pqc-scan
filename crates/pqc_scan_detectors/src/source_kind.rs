//! Content corroborates Dockerfile variant names before overriding an extension.
use std::path::Path;

pub(crate) fn is_dockerfile(path: &Path, text: &str) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let name = name.to_ascii_lowercase();
    if name != "dockerfile"
        && name
            .strip_prefix("dockerfile.")
            .is_none_or(|s| s.is_empty())
    {
        return false;
    }
    let Some(line) = text
        .trim_start_matches('\u{feff}')
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
    else {
        return false;
    };
    let Some((instruction, argument)) = line.split_once(char::is_whitespace) else {
        return false;
    };
    let argument = argument.trim();
    if instruction.eq_ignore_ascii_case("ARG") {
        let name = argument.split('=').next().unwrap_or("");
        return !name.is_empty()
            && name
                .bytes()
                .enumerate()
                .all(|(i, b)| b == b'_' || b.is_ascii_alphabetic() || i > 0 && b.is_ascii_digit());
    }
    if !instruction.eq_ignore_ascii_case("FROM") {
        return false;
    }
    let mut words = argument.split_whitespace();
    let Some(mut image) = words.next() else {
        return false;
    };
    if image.starts_with("--platform=") {
        let Some(next) = words.next() else {
            return false;
        };
        image = next;
    }
    if image.is_empty() || image.starts_with(['=', '-']) {
        return false;
    }
    match words.next() {
        None => true,
        Some(word) if word.eq_ignore_ascii_case("AS") => {
            words.next().is_some() && words.next().is_none()
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn python_from_import_is_source_even_with_a_dockerfile_name() {
        assert!(!super::is_dockerfile(
            std::path::Path::new("Dockerfile.py"),
            "from cryptography.hazmat.primitives import hashes\n"
        ));
        assert!(!super::is_dockerfile(
            std::path::Path::new("Dockerfile.py"),
            "from cryptography import (hashes, rsa)\n"
        ));
        for text in [
            "FROM alpine",
            "FROM --platform=$BUILDPLATFORM alpine AS build",
            "ARG BASE=alpine\nFROM ${BASE}",
        ] {
            assert!(super::is_dockerfile(
                std::path::Path::new("Dockerfile.py"),
                text
            ));
        }
    }
}
