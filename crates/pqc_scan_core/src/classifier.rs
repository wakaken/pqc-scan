use std::path::Path;

pub fn is_dependency_manifest(path: &Path) -> bool {
    let name = match path.file_name().and_then(|x| x.to_str()) {
        Some(v) => v,
        None => return false,
    };
    if name.ends_with(".gemspec") {
        return true;
    }

    matches!(
        name,
        "Cargo.toml"
            | "Cargo.lock"
            | "Gemfile"
            | "Gemfile.lock"
            | "gems.rb"
            | "gems.locked"
            | "pom.xml"
            | "build.gradle"
            | "build.gradle.kts"
            | "go.mod"
            | "package.json"
            | "package-lock.json"
            | "pnpm-lock.yaml"
            | "requirements.txt"
            | "Pipfile.lock"
            | "poetry.lock"
            | "gradle.lockfile"
    )
}

pub fn is_certificate_file(path: &Path) -> bool {
    let ext = match path.extension().and_then(|x| x.to_str()) {
        Some(v) => v,
        None => return false,
    };

    matches!(
        ext.to_ascii_lowercase().as_str(),
        "pem" | "crt" | "cer" | "der" | "p12" | "pfx"
    )
}

pub fn is_sbom_file(path: &Path) -> bool {
    let name = match path.file_name().and_then(|x| x.to_str()) {
        Some(v) => v.to_ascii_lowercase(),
        None => return false,
    };

    if !(name.ends_with(".json") || name.ends_with(".cdx.json") || name.ends_with(".spdx.json")) {
        return false;
    }

    name.contains("sbom")
        || name.contains("bom")
        || name.contains("cyclonedx")
        || name.contains("spdx")
}
