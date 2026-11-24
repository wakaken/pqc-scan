//! Declared pre-inspection policy; cryptographic containers are never lumped in
//! with media/archives. The opt-out permits explicit forensic/custom-detector use.
use std::path::Path;
#[derive(Debug, Clone, Copy, Default)]
pub struct FileSelection {
    pub include_non_scannable: bool,
}
pub(crate) const POLICY_LABEL: &str = "known non-scannable media, archives and binary data";
pub(crate) fn known_extension(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(
        ext.as_str(),
        "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "bmp"
            | "ico"
            | "webp"
            | "tif"
            | "tiff"
            | "heic"
            | "heif"
            | "avif"
            | "jxl"
            | "jp2"
            | "icns"
            | "psd"
            | "dds"
            | "cur"
            | "tga"
            | "exr"
            | "ktx"
            | "j2c"
            | "pfm"
            | "glb"
            | "mp3"
            | "mp4"
            | "wav"
            | "webm"
            | "mov"
            | "avi"
            | "ogg"
            | "oga"
            | "flac"
            | "woff"
            | "woff2"
            | "ttf"
            | "otf"
            | "eot"
            | "zip"
            | "gz"
            | "tgz"
            | "tar"
            | "xz"
            | "lzma"
            | "bz2"
            | "zst"
            | "z"
            | "br"
            | "lz4"
            | "7z"
            | "rar"
            | "jar"
            | "war"
            | "ear"
            | "egg"
            | "whl"
            | "wasm"
            | "exe"
            | "dll"
            | "so"
            | "dylib"
            | "o"
            | "obj"
            | "class"
            | "pyc"
            | "pyc-tpl"
            | "mo"
            | "pb"
            | "binpb"
            | "desc"
            | "pdf"
    )
}
pub(crate) fn protects_crypto(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(
        ext.as_str(),
        "pem"
            | "crt"
            | "cer"
            | "der"
            | "p12"
            | "pfx"
            | "pkcs12"
            | "jks"
            | "jceks"
            | "bcfks"
            | "keystore"
            | "truststore"
            | "keytab"
            | "kbx"
            | "key"
            | "pub"
            | "asc"
    )
}
pub(crate) fn non_text(path: &Path, bytes: &[u8]) -> bool {
    !protects_crypto(path)
        && !crate::text_encoding::has_wide_bom(bytes)
        && crate::text_encoding::decode(bytes).is_none()
}

/// Additive scan scope; existing FileSelection callers keep their defaults.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScanSelection {
    pub files: FileSelection,
    pub exclude_tests: bool,
}
impl From<FileSelection> for ScanSelection {
    fn from(files: FileSelection) -> Self {
        Self {
            files,
            exclude_tests: false,
        }
    }
}
pub(crate) const TEST_POLICY_LABEL: &str =
    "test directories and test filename conventions (explicit opt-out)";

/// Conservative path convention matching, relative to the selected root.
/// Inline tests are handled separately by syntax-proven detector ranges.
pub fn is_test_path(path: &Path) -> bool {
    test_path(Path::new(""), path)
}

pub(crate) fn test_path(root: &Path, path: &Path) -> bool {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let test_dir = |s: &str| {
        matches!(
            s,
            "test"
                | "tests"
                | "__tests__"
                | "__test__"
                | "spec"
                | "specs"
                | "testsuite"
                | "test-suite"
                | "test-framework"
        )
    };
    if relative.parent().is_some_and(|parent| {
        parent
            .components()
            .any(|c| c.as_os_str().to_str().is_some_and(test_dir))
    }) {
        return true;
    }
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    matches!(name, "tests.py" | "tests.rs")
        || name.starts_with("test_") && name.ends_with(".py")
        || name.ends_with("_test.py")
        || name.ends_with("_test.go")
        || name.ends_with("_spec.rb")
        || name.ends_with("_test.rb")
        || ["js", "jsx", "ts", "tsx", "mjs", "cjs", "mts", "cts"]
            .iter()
            .any(|ext| {
                name.ends_with(&format!(".test.{ext}")) || name.ends_with(&format!(".spec.{ext}"))
            })
        || name.ends_with("Test.java")
        || name.ends_with("Tests.java")
}

#[cfg(test)]
mod test_selection_tests {
    use super::*;
    #[test]
    fn matches_conventions_without_matching_ancestors_or_similar_names() {
        let root = Path::new("/tests/customer-app");
        for relative in [
            "tests/token.py",
            "testsuite/keys/example.pem",
            "test-suite/data/key.pem",
            "test-framework/oauth/src/main/java/Key.java",
            "src/test/java/Key.java",
            "__tests__/jwt.ts",
            "spec/key_spec.rb",
            "test_key.py",
            "key_test.go",
            "key.test.ts",
            "key.spec.js",
            "KeyTest.java",
            "key_test.rb",
            "src/tests.rs",
            "app/tests.py",
            "key.test.mts",
            "key.spec.cts",
        ] {
            assert!(test_path(root, &root.join(relative)), "{relative}");
        }
        for relative in [
            "src/main.py",
            "contest.py",
            "src/testsuite-client/key.py",
            "src/testing/key.py",
            "testdata/key.py",
            "src/lib.rs",
            "latest.js",
            "Key.java",
        ] {
            assert!(!test_path(root, &root.join(relative)), "{relative}");
        }
        assert!(test_path(
            Path::new("/app/test_key.py"),
            Path::new("/app/test_key.py")
        ));
    }
}
